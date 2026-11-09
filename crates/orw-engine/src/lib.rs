//! Graphics and simulation engine — placeholder until route loading and wgpu runtime land.
//!
//! Phase 1 (roadmap): headless instance backend for CI; interactive window on developer machines.

use thiserror::Error;

pub const ENGINE_NAME: &str = "OpenRailWorks";
pub const GRAPHICS_API: &str = "wgpu (Vulkan / DX12 / Metal)";

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("engine not initialized — use orw-catalog until Phase 1 ships")]
    NotReady,
}

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
