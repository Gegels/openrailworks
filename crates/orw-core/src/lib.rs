mod config;
mod install;
mod route;
mod route_properties;
pub mod settings;
pub mod tracks;

pub use config::{
    load_user_config, save_user_config, GraphicsProfile, UpdateChannel, UserConfig,
};
pub use settings::{
    graphics_profile_label, CameraMode, CameraSettings, DisplaySettings, GraphicsSettings,
};
pub use tracks::{probe_tracks_bin, TracksBinProbe};
pub use install::{InstallError, InstallLayout, RailWorksInstall};
pub use route::{
    catalog_matches_benchmarks, scan_routes, RouteCatalog, RouteEntry, RouteScanOptions,
};
pub use route_properties::{parse_route_properties_xml, read_display_name_from_file};
