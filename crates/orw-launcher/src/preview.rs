//! Live **GameCam** preview (openOMSI-style bus preview → track / cab placeholder).

use std::sync::{Arc, Mutex};

use bytemuck::{Pod, Zeroable};
use eframe::egui;
use eframe::wgpu;
use wgpu::util::DeviceExt;
use egui_wgpu::CallbackTrait;
use glam::{Mat4, Vec3};
use orw_core::{CameraMode, GraphicsProfile, UserConfig};

const SHADER: &str = r#"
struct Uniforms {
    view_proj: mat4x4<f32>,
    params0: vec4<f32>, // fog_distance, enhanced, shadows, time_norm
    params1: vec4<f32>, // zenith rgb + pad
    params2: vec4<f32>, // horizon rgb + pad
};

@group(0) @binding(0) var<uniform> u: Uniforms;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) n: vec3<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec3<f32>, @location(1) n: vec3<f32>) -> VsOut {
    var o: VsOut;
    o.world = pos;
    o.n = n;
    o.pos = u.view_proj * vec4(pos, 1.0);
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let fog_d = u.params0.x;
    let enhanced = u.params0.y;
    let shadows = u.params0.z;
    let dist = length(in.world.xz);
    let fog = clamp(dist / fog_d, 0.0, 1.0);

    var base = vec3(0.22, 0.28, 0.2);
    if (in.n.y > 0.5) {
        base = vec3(0.35, 0.38, 0.32);
    } else if (abs(in.n.x) > 0.5) {
        base = vec3(0.55, 0.52, 0.48); // rail highlight
    }

    let sun = max(0.0, dot(normalize(in.n), normalize(vec3(0.4, 0.9, 0.2))));
    if (enhanced > 0.5) {
        base = base * (0.65 + 0.35 * sun);
        if (shadows > 0.5) {
            base *= 0.85 + 0.15 * sun;
        }
    } else if (enhanced > 0.0) {
        base = mix(base, base * (0.8 + 0.2 * sun), 0.45);
    }

    let horizon = u.params2.rgb;
    let zenith = u.params1.rgb;
    let sky = mix(horizon, zenith, clamp(in.pos.y * 0.5 + 0.5, 0.0, 1.0));
    let col = mix(base, sky, fog * 0.92);
    return vec4(col, 1.0);
}
"#;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuUniforms {
    view_proj: [[f32; 4]; 4],
    params0: [f32; 4],
    params1: [f32; 4],
    params2: [f32; 4],
}

#[derive(Clone)]
pub struct PreviewShared {
    pub config: Arc<Mutex<UserConfig>>,
}

pub struct GameCamGpu {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl GameCamGpu {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gamecam"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gamecam_uniform"),
            size: std::mem::size_of::<GpuUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gamecam_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gamecam_bg"),
            layout: &bind_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let (vertices, indices) = build_scene_mesh();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("gamecam_vb"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("gamecam_ib"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gamecam_pl"),
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("gamecam_pipe"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: 12,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn update_uniforms(&self, queue: &wgpu::Queue, cfg: &UserConfig, aspect: f32) {
        let u = build_uniforms(cfg, aspect);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&u));
    }

}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    pos: [f32; 3],
    n: [f32; 3],
}

fn build_scene_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let push_tri = |vertices: &mut Vec<Vertex>, indices: &mut Vec<u16>, a: [f32; 3], b: [f32; 3], c: [f32; 3], n: [f32; 3]| {
        let base = vertices.len() as u16;
        vertices.push(Vertex { pos: a, n });
        vertices.push(Vertex { pos: b, n });
        vertices.push(Vertex { pos: c, n });
        indices.extend([base, base + 1, base + 2]);
    };

    // Ground tiles
    for i in -4..4 {
        for j in -2..2 {
            let x0 = i as f32 * 4.0;
            let z0 = j as f32 * 8.0;
            let x1 = x0 + 4.0;
            let z1 = z0 + 8.0;
            push_tri(
                &mut vertices,
                &mut indices,
                [x0, 0.0, z0],
                [x1, 0.0, z0],
                [x1, 0.0, z1],
                [0.0, 1.0, 0.0],
            );
            push_tri(
                &mut vertices,
                &mut indices,
                [x0, 0.0, z0],
                [x1, 0.0, z1],
                [x0, 0.0, z1],
                [0.0, 1.0, 0.0],
            );
        }
    }

    // Two rails
    for segment in 0..20 {
        let z0 = segment as f32 * -2.0;
        let z1 = z0 - 2.0;
        for (x, nx) in [(-0.72f32, -1.0), (0.72, 1.0)] {
            push_tri(
                &mut vertices,
                &mut indices,
                [x - 0.04, 0.05, z0],
                [x + 0.04, 0.05, z0],
                [x + 0.04, 0.05, z1],
                [nx, 0.0, 0.0],
            );
            push_tri(
                &mut vertices,
                &mut indices,
                [x - 0.04, 0.05, z0],
                [x + 0.04, 0.05, z1],
                [x - 0.04, 0.05, z1],
                [nx, 0.0, 0.0],
            );
        }
    }

  // Cab nose box (placeholder)
    push_tri(
        &mut vertices,
        &mut indices,
        [-1.0, 0.0, 2.0],
        [1.0, 0.0, 2.0],
        [1.0, 1.2, 1.0],
        [0.0, 0.0, 1.0],
    );
    push_tri(
        &mut vertices,
        &mut indices,
        [-1.0, 0.0, 2.0],
        [1.0, 1.2, 1.0],
        [-1.0, 1.2, 1.0],
        [0.0, 0.0, 1.0],
    );

    (vertices, indices)
}

fn build_uniforms(cfg: &UserConfig, aspect: f32) -> GpuUniforms {
    let cam = &cfg.camera;
    let yaw = cam.orbit_yaw_deg.to_radians();
    let pitch = cam.orbit_pitch_deg.to_radians();
    let dist = match cam.mode {
        CameraMode::Cab => 3.5,
        CameraMode::Chase => cam.chase_distance_m,
        CameraMode::Aerial => cam.chase_distance_m * 1.8,
    };
    let eye = Vec3::new(
        dist * pitch.cos() * yaw.sin(),
        dist * pitch.sin().abs() + match cam.mode {
            CameraMode::Cab => 1.1,
            CameraMode::Chase => 2.0,
            CameraMode::Aerial => 8.0,
        },
        dist * pitch.cos() * yaw.cos(),
    );
    let target = match cam.mode {
        CameraMode::Cab => Vec3::new(0.0, 1.0, -8.0),
        _ => Vec3::new(0.0, 0.5, -12.0),
    };
    let up = Vec3::Y;
    let view = Mat4::look_at_rh(eye, target, up);
    let fov = cam.fov_vertical_deg.to_radians();
    let proj = Mat4::perspective_rh(fov, aspect.max(0.1), 0.1, cfg.graphics.view_distance_m);
    let view_proj = proj * view;

    let t = cfg.graphics.time_of_day_hours / 24.0;
    let (zenith, horizon) = sky_colors(t);

    let enhanced = match cfg.graphics_profile {
        GraphicsProfile::Vanilla => 0.0,
        GraphicsProfile::VanillaPlus => 0.5,
        GraphicsProfile::Enhanced => 1.0,
    };

    GpuUniforms {
        view_proj: view_proj.to_cols_array_2d(),
        params0: [
            cfg.graphics.view_distance_m,
            enhanced,
            if cfg.graphics.shadows { 1.0 } else { 0.0 },
            t,
        ],
        params1: [zenith.x, zenith.y, zenith.z, 0.0],
        params2: [horizon.x, horizon.y, horizon.z, 0.0],
    }
}

fn sky_colors(t: f32) -> (Vec3, Vec3) {
    let day_zenith = Vec3::new(0.35, 0.55, 0.95);
    let day_horizon = Vec3::new(0.65, 0.75, 0.9);
    let night_zenith = Vec3::new(0.02, 0.03, 0.08);
    let night_horizon = Vec3::new(0.05, 0.06, 0.12);
    let sunset = (1.0 - (t - 0.7).abs() * 5.0).clamp(0.0, 1.0);
    let mut zenith = day_zenith.lerp(night_zenith, night_factor(t));
    let mut horizon = day_horizon.lerp(night_horizon, night_factor(t));
    horizon += Vec3::new(0.4, 0.15, 0.02) * sunset;
    zenith += Vec3::new(0.15, 0.05, 0.0) * sunset * 0.5;
    (zenith, horizon)
}

fn night_factor(t: f32) -> f32 {
    let h = t * 24.0;
    if (6.0..18.0).contains(&h) {
        0.0
    } else if h < 6.0 {
        1.0 - h / 6.0
    } else {
        (h - 18.0) / 6.0
    }
}

pub struct GameCamCallback {
    pub shared: PreviewShared,
    pub aspect: Arc<Mutex<f32>>,
}

impl CallbackTrait for GameCamCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let gpu = callback_resources.get::<GameCamGpu>().expect("GameCamGpu");
        let cfg = self.shared.config.lock().expect("config lock");
        let aspect = *self.aspect.lock().expect("aspect lock");
        gpu.update_uniforms(queue, &cfg, aspect);
        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        let gpu = callback_resources.get::<GameCamGpu>().expect("GameCamGpu");
        render_pass.set_pipeline(&gpu.pipeline);
        render_pass.set_bind_group(0, &gpu.bind_group, &[]);
        render_pass.set_vertex_buffer(0, gpu.vertex_buffer.slice(..));
        render_pass.set_index_buffer(gpu.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..gpu.num_indices, 0, 0..1);
    }
}

pub fn show_gamecam_preview(
    ui: &mut egui::Ui,
    shared: &PreviewShared,
    aspect_store: &Arc<Mutex<f32>>,
    render_available: bool,
) -> egui::Response {
    let cfg = shared.config.lock().expect("config lock");
    let w = cfg.display_width.max(320).min(3840) as f32;
    let h = cfg.display_height.max(180).min(2160) as f32;
    let scale = cfg.graphics.render_scale.clamp(0.25, 1.0);
    drop(cfg);

    let max_w = ui.available_width().max(280.0);
    let preview_w = max_w * scale;
    let preview_h = preview_w * (h / w);
    let size = egui::vec2(preview_w, preview_h.clamp(120.0, 520.0));

    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());

    if render_available {
        if let Ok(mut asp) = aspect_store.lock() {
            *asp = (rect.width() / rect.height().max(1.0)).max(0.1);
        }
        let cb = GameCamCallback {
            shared: PreviewShared {
                config: Arc::clone(&shared.config),
            },
            aspect: Arc::clone(aspect_store),
        };
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(rect, cb));
    } else {
        ui.painter().rect_filled(rect, 4.0, egui::Color32::from_gray(28));
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "wgpu-Vorschau nicht verfügbar",
            egui::FontId::proportional(14.0),
            egui::Color32::GRAY,
        );
    }

    if response.dragged() {
        if let Ok(mut cfg) = shared.config.lock() {
            cfg.camera.orbit_yaw_deg += response.drag_delta().x * 0.25;
            cfg.camera.orbit_pitch_deg =
                (cfg.camera.orbit_pitch_deg + response.drag_delta().y * 0.15).clamp(-45.0, 35.0);
        }
    }

    response
}
