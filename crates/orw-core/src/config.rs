use std::env;
use std::path::{Path, PathBuf};

use serde::Deserialize;

const ENV_RAILWORKS_PATH: &str = "RAILWORKS_PATH";
const ENV_OPENRAILWORKS_CONFIG: &str = "OPENRAILWORKS_CONFIG";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserConfig {
    #[serde(default)]
    pub railworks_path: Option<PathBuf>,
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
