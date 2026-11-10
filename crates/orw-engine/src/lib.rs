//! Graphics and simulation engine — Phase 1: session model + wgpu prep; runtime window next.

mod session;

pub use session::{DriveMode, LaunchSession};

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
    #[error("engine runtime not ready (Phase 1 — route display in progress)")]
    NotReady,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Accept a launch request from the launcher; returns when a windowed runtime exists.
pub fn launch_session(session: &LaunchSession) -> Result<(), EngineError> {
    let _ = session;
    Err(EngineError::NotReady)
}

/// Returns whether the linked wgpu stack can be built on this platform (compile-time smoke).
pub fn platform_capabilities() -> &'static str {
    GRAPHICS_API
}

#[cfg(test)]
mod tests {
    use super::*;
    use orw_core::GraphicsProfile;

    #[test]
    fn default_slo_matches_plan() {
        let slo = PerformanceSlo::default();
        assert_eq!(slo.target_fps, 60);
        assert_eq!(slo.width, 1920);
    }

    #[test]
    fn launch_not_ready_yet() {
        let session = LaunchSession::free_roam("TestRoute", GraphicsProfile::Vanilla);
        assert!(matches!(launch_session(&session), Err(EngineError::NotReady)));
    }
}
