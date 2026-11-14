use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::settings::{CameraSettings, DisplaySettings, GraphicsSettings};

const ENV_RAILWORKS_PATH: &str = "RAILWORKS_PATH";
const ENV_OPENRAILWORKS_CONFIG: &str = "OPENRAILWORKS_CONFIG";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UpdateChannel {
    #[default]
    Stable,
    Beta,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphicsProfile {
    #[default]
    Vanilla,
    VanillaPlus,
    Enhanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub railworks_path: Option<PathBuf>,
    #[serde(default)]
    pub update_channel: UpdateChannel,
    #[serde(default)]
    pub graphics_profile: GraphicsProfile,
    #[serde(default = "default_target_fps")]
    pub target_fps: u32,
    #[serde(default = "default_width")]
    pub display_width: u32,
    #[serde(default = "default_height")]
    pub display_height: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub favorite_routes: Vec<String>,
    #[serde(default)]
    pub camera: CameraSettings,
    #[serde(default)]
    pub graphics: GraphicsSettings,
    #[serde(default)]
    pub display: DisplaySettings,
}

fn default_target_fps() -> u32 {
    60
}

fn default_width() -> u32 {
    1920
}

fn default_height() -> u32 {
    1080
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            railworks_path: None,
            update_channel: UpdateChannel::default(),
            graphics_profile: GraphicsProfile::default(),
            target_fps: default_target_fps(),
            display_width: default_width(),
            display_height: default_height(),
            favorite_routes: Vec::new(),
            camera: CameraSettings::default(),
            graphics: GraphicsSettings::default(),
            display: DisplaySettings::default(),
        }
    }
}

impl UserConfig {
    pub fn config_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        if let Ok(explicit) = env::var(ENV_OPENRAILWORKS_CONFIG) {
            paths.push(PathBuf::from(explicit));
        }
        if let Some(home) = home_dir() {
            paths.push(home.join(".config/openrailworks/config.toml"));
            #[cfg(windows)]
            paths.push(home.join("AppData/Roaming/openrailworks/config.toml"));
        }
        paths
    }

    pub fn primary_config_path() -> Option<PathBuf> {
        for path in Self::config_paths() {
            if path.is_file() {
                return Some(path);
            }
        }
        home_dir().map(|h| h.join(".config/openrailworks/config.toml"))
    }
}

pub fn load_user_config() -> UserConfig {
    for path in UserConfig::config_paths() {
        if !path.is_file() {
            continue;
        }
        if let Ok(contents) = std::fs::read_to_string(&path) {
            if let Ok(cfg) = toml::from_str::<UserConfig>(&contents) {
                return cfg;
            }
        }
    }
    UserConfig::default()
}

/// Persist config to the primary path (creates parent directories).
pub fn save_user_config(cfg: &UserConfig) -> std::io::Result<PathBuf> {
    let path = UserConfig::primary_config_path().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "no config path")
    })?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = toml::to_string_pretty(cfg)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    fs::write(&path, text)?;
    Ok(path)
}

/// Resolve the user-provided or discovered RailWorks root directory.
pub fn resolve_install_root(cli_path: Option<&Path>) -> Option<PathBuf> {
    if let Some(p) = cli_path {
        return Some(p.to_path_buf());
    }
    if let Ok(p) = env::var(ENV_RAILWORKS_PATH) {
        if !p.is_empty() {
            return Some(PathBuf::from(p));
        }
    }
    let cfg = load_user_config();
    if let Some(p) = cfg.railworks_path {
        return Some(p);
    }
    default_steam_railworks_path()
}

#[cfg(windows)]
fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE").map(PathBuf::from)
}

#[cfg(not(windows))]
fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

#[cfg(windows)]
fn default_steam_railworks_path() -> Option<PathBuf> {
    let mut bases = Vec::new();
    if let Some(program_files) = env::var_os("ProgramFiles(x86)") {
        bases.push(PathBuf::from(program_files));
    }
    if let Some(program_files) = env::var_os("ProgramFiles") {
        bases.push(PathBuf::from(program_files));
    }
    for base in bases {
        let candidate = base
            .join("Steam")
            .join("steamapps")
            .join("common")
            .join("RailWorks");
        if candidate.is_dir() {
            return Some(candidate);
        }
    }
    None
}

#[cfg(not(windows))]
fn default_steam_railworks_path() -> Option<PathBuf> {
    let home = home_dir()?;
    let candidates = [
        home.join(".steam/steam/steamapps/common/RailWorks"),
        home.join(".local/share/Steam/steamapps/common/RailWorks"),
    ];
    for candidate in candidates {
        if candidate.is_dir() {
            return Some(candidate);
        }
    }
    None
}
