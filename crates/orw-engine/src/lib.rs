//! Graphics and simulation engine — Phase 1: session model + wgpu prep; runtime window next.

mod launch;
mod session;

pub use launch::LaunchBundle;
pub use session::{DriveMode, LaunchSession};

use std::path::PathBuf;

use thiserror::Error;

pub const ENGINE_NAME: &str = "OpenRailWorks";
pub const GRAPHICS_API: &str = "wgpu (Vulkan / DX12 / Metal)";

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PerformanceSlo {
    pub target_fps: u32,
    pub width: u32,
    pub height: u32,
}

impl Default for PerformanceSlo {
    fn default() -> Self {
        Self {
            target_fps: 60,
            width: 1920,
            height: 1080,
        }
    }
}

impl From<&orw_core::UserConfig> for PerformanceSlo {
    fn from(cfg: &orw_core::UserConfig) -> Self {
        Self {
            target_fps: cfg.target_fps,
            width: cfg.display_width,
            height: cfg.display_height,
        }
    }
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("orw-game nicht gefunden (bitte cargo build --release -p orw-launcher): {}", .0.display())]
    GameBinaryMissing(PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub use launch::launch_session;

/// Returns whether the linked wgpu stack can be built on this platform (compile-time smoke).
pub fn platform_capabilities() -> &'static str {
    GRAPHICS_API
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_slo_matches_plan() {
        let slo = PerformanceSlo::default();
        assert_eq!(slo.target_fps, 60);
        assert_eq!(slo.width, 1920);
    }

}
