use serde::{Deserialize, Serialize};

use orw_core::GraphicsProfile;

use crate::PerformanceSlo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriveMode {
    FreeRoam,
    Scenario,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchSession {
    pub route_id: String,
    pub mode: DriveMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scenario_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vehicle_id: Option<String>,
    pub graphics: GraphicsProfile,
    pub performance: PerformanceSlo,
}

impl LaunchSession {
    pub fn free_roam(route_id: impl Into<String>, graphics: GraphicsProfile) -> Self {
        Self {
            route_id: route_id.into(),
            mode: DriveMode::FreeRoam,
            scenario_id: None,
            vehicle_id: None,
            graphics,
            performance: PerformanceSlo::default(),
        }
    }
}
