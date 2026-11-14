//! Launcher graphics, camera, and display settings (openOMSI-style; apply to preview + game).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraMode {
    #[default]
    Cab,
    Chase,
    Aerial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraSettings {
    /// Vertical field of view in degrees (TSC cab / chase).
    #[serde(default = "default_fov")]
    pub fov_vertical_deg: f32,
    #[serde(default)]
    pub mode: CameraMode,
    /// Orbit yaw for the launcher preview (drag on viewport).
    #[serde(default)]
    pub orbit_yaw_deg: f32,
    #[serde(default = "default_pitch")]
    pub orbit_pitch_deg: f32,
    /// Chase / aerial distance in metres (preview scale).
    #[serde(default = "default_cam_distance")]
    pub chase_distance_m: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphicsSettings {
    #[serde(default = "default_msaa")]
    pub msaa_samples: u32,
    #[serde(default = "default_true")]
    pub shadows: bool,
    #[serde(default = "default_view_distance")]
    pub view_distance_m: f32,
    /// Internal render resolution scale (openOMSI `render_scale`).
    #[serde(default = "one_f32")]
    pub render_scale: f32,
    /// 0–24 h for preview sky and lighting.
    #[serde(default = "default_time_of_day")]
    pub time_of_day_hours: f32,
    #[serde(default = "default_true")]
    pub vsync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisplaySettings {
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default = "default_ui_opacity")]
    pub ui_opacity: f32,
}

fn default_fov() -> f32 {
    72.0
}

fn default_pitch() -> f32 {
    -12.0
}

fn default_cam_distance() -> f32 {
    18.0
}

fn default_msaa() -> u32 {
    4
}

fn default_view_distance() -> f32 {
    2500.0
}

fn one_f32() -> f32 {
    1.0
}

fn default_time_of_day() -> f32 {
    14.0
}

fn default_true() -> bool {
    true
}

fn default_ui_opacity() -> f32 {
    0.85
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            fov_vertical_deg: default_fov(),
            mode: CameraMode::default(),
            orbit_yaw_deg: 0.0,
            orbit_pitch_deg: default_pitch(),
            chase_distance_m: default_cam_distance(),
        }
    }
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            msaa_samples: default_msaa(),
            shadows: default_true(),
            view_distance_m: default_view_distance(),
            render_scale: one_f32(),
            time_of_day_hours: default_time_of_day(),
            vsync: default_true(),
        }
    }
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            fullscreen: false,
            ui_opacity: default_ui_opacity(),
        }
    }
}

/// Quality preset labels matching openOMSI-style tiers.
pub fn graphics_profile_label(profile: super::GraphicsProfile) -> &'static str {
    match profile {
        super::GraphicsProfile::Vanilla => "Vanilla (wie TSC)",
        super::GraphicsProfile::VanillaPlus => "Vanilla+",
        super::GraphicsProfile::Enhanced => "Enhanced",
    }
}

#[cfg(test)]
mod tests {
    use crate::UserConfig;

    #[test]
    fn config_deserialize_fills_settings_defaults() {
        let raw = r#"railworks_path = "/tmp/rw""#;
        let cfg: UserConfig = toml::from_str(raw).unwrap();
        assert_eq!(cfg.camera.fov_vertical_deg, 72.0);
        assert_eq!(cfg.graphics.msaa_samples, 4);
        assert!((cfg.display.ui_opacity - 0.85).abs() < f32::EPSILON);
    }
}
