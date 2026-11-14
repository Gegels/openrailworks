//! Fullscreen/windowed **game view** opened from the launcher (placeholder world until route load).

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::Context;
use eframe::egui;
use orw_engine::LaunchBundle;

use crate::preview::{show_gamecam_preview, GameCamGpu, PreviewShared};

pub fn run(session_file: PathBuf) -> anyhow::Result<()> {
    let raw = std::fs::read_to_string(&session_file)?;
    let bundle: LaunchBundle = serde_json::from_str(&raw).context("parse launch bundle")?;
    let title = bundle
        .session
        .route_display_name
        .clone()
        .unwrap_or_else(|| bundle.session.route_id.clone());

    let preview_shared = PreviewShared {
        config: Arc::new(Mutex::new(bundle.config.clone())),
    };
    let aspect = Arc::new(Mutex::new(
        bundle.config.display_width as f32 / bundle.config.display_height.max(1) as f32,
    ));

    let vsync = bundle.config.graphics.vsync;
    let fullscreen = bundle.config.display.fullscreen;
    let size = [
        bundle.config.display_width as f32,
        bundle.config.display_height as f32,
    ];

    let mut viewport = egui::ViewportBuilder::default()
        .with_title(format!("OpenRailWorks — {title}"))
        .with_inner_size(size);
    if fullscreen {
        viewport = viewport.with_fullscreen(true);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        vsync,
        ..Default::default()
    };

    let shared = preview_shared.clone();
    let aspect_store = aspect.clone();
    let window_id = title.clone();
    eframe::run_native(
        &window_id,
        native_options,
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            if let Some(rs) = cc.wgpu_render_state.as_ref() {
                let gpu = GameCamGpu::new(&rs.device, rs.target_format);
                rs.renderer.write().callback_resources.insert(gpu);
            }
            Ok(Box::new(GameApp {
                preview_shared: shared,
                aspect_store,
                route_title: title,
                route_id: bundle.session.route_id.clone(),
                config_summary: format!(
                    "{:?} · {:.0} h · FOV {:.0}°",
                    bundle.config.graphics_profile,
                    bundle.config.graphics.time_of_day_hours,
                    bundle.config.camera.fov_vertical_deg,
                ),
            }))
        }),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(())
}

struct GameApp {
    preview_shared: PreviewShared,
    aspect_store: Arc<Mutex<f32>>,
    route_title: String,
    route_id: String,
    config_summary: String,
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("bar").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.heading(&self.route_title);
                    ui.separator();
                    ui.label(&self.config_summary);
                    if ui.button("Beenden").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.colored_label(
                    egui::Color32::from_rgb(255, 190, 100),
                    format!(
                        "Strecke ist gewählt (Ordner: {}). OpenRailWorks lädt noch keine Gleise aus TSC \
                         — gleiche Platzhalter-Welt für jede Route. Echte Ringbahn kommt mit dem Strecken-Loader. \
                         Für echte Fahrt weiter Train Simulator Classic nutzen.",
                        self.route_id
                    ),
                );
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = show_gamecam_preview(ui, &self.preview_shared, &self.aspect_store, true);
        });
    }
}