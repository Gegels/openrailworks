use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use walkdir::WalkDir;

use crate::install::RailWorksInstall;

#[derive(Debug, Clone, Default)]
pub struct RouteScanOptions {
    /// Include routes that look like non-route folders (no Tracks.bin / minimal files).
    pub include_incomplete: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteEntry {
    pub id: String,
    pub path: PathBuf,
    pub has_tracks_bin: bool,
    pub has_route_properties: bool,
    pub scenario_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteCatalog {
    pub install_root: PathBuf,
    pub routes: Vec<RouteEntry>,
    pub package_count: usize,
}

pub fn scan_routes(
    install: &RailWorksInstall,
    options: &RouteScanOptions,
) -> std::io::Result<RouteCatalog> {
    let mut routes = Vec::new();

    for entry in fs::read_dir(&install.routes_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }
        let path = entry.path();
        let id = entry
            .file_name()
            .to_string_lossy()
            .into_owned();

        let tracks = path.join("Tracks.bin");
        let has_tracks_bin = tracks.is_file();
        let route_props = path.join("RouteProperties.xml");
        let has_route_properties = route_props.is_file();

        let scenario_dir = path.join("Scenarios");
        let scenario_count = if scenario_dir.is_dir() {
            fs::read_dir(&scenario_dir)?
                .filter(|e| e.as_ref().map(|x| x.path().is_dir()).unwrap_or(false))
                .count()
        } else {
            0
        };

        let looks_like_route = has_tracks_bin || has_route_properties || scenario_count > 0;
        if !options.include_incomplete && !looks_like_route {
            continue;
        }

        routes.push(RouteEntry {
            id,
            path,
            has_tracks_bin,
            has_route_properties,
            scenario_count,
        });
    }

    routes.sort_by(|a, b| a.id.to_lowercase().cmp(&b.id.to_lowercase()));

    let package_count = install
        .packages_dir
        .as_ref()
        .map(count_ap_archives)
        .unwrap_or(0);

    Ok(RouteCatalog {
        install_root: install.root.clone(),
        routes,
        package_count,
    })
}

fn count_ap_archives(packages_dir: &PathBuf) -> usize {
    WalkDir::new(packages_dir)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("ap"))
        })
        .count()
}

/// Well-known benchmark route identifiers (user-defined; match by folder name substring).
pub fn benchmark_route_ids() -> &'static [&'static str] {
    &[
        "Ringbahn",
        "ringbahn",
        "Berlin",
        "Südast",
        "Suedast",
        "South",
    ]
}

pub fn catalog_matches_benchmarks(catalog: &RouteCatalog) -> Vec<String> {
    let needles = benchmark_route_ids();
    catalog
        .routes
        .iter()
        .filter(|r| {
            let id = r.id.to_lowercase();
            needles
                .iter()
                .any(|n| id.contains(&n.to_lowercase()))
        })
        .map(|r| r.id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::install::{InstallLayout, RailWorksInstall};
    use tempfile::TempDir;

    #[test]
    fn scans_routes_under_content() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let route_a = root.join("Content/Routes/TestRoute");
        std::fs::create_dir_all(route_a.join("Scenarios/S1")).unwrap();
        std::fs::write(route_a.join("Tracks.bin"), b"\0").unwrap();

        let install = RailWorksInstall {
            root: root.clone(),
            layout: InstallLayout::ContentRoutes,
            routes_dir: root.join("Content/Routes"),
            packages_dir: None,
        };

        let catalog = scan_routes(&install, &RouteScanOptions::default()).unwrap();
        assert_eq!(catalog.routes.len(), 1);
        assert!(catalog.routes[0].has_tracks_bin);
        assert_eq!(catalog.routes[0].id, "TestRoute");
    }
}
