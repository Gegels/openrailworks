use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use walkdir::WalkDir;

use crate::install::RailWorksInstall;
use crate::route_properties::{parse_route_properties_xml, read_display_name_from_file};

#[derive(Debug, Clone, Default)]
pub struct RouteScanOptions {
    /// Include routes that look like non-route folders (no Tracks.bin / minimal files).
    pub include_incomplete: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteEntry {
    /// Folder name under `Content/Routes` (often a GUID).
    pub id: String,
    /// Title from `RouteProperties.xml` / TSC route list (when available).
    pub display_name: String,
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

        let display_name =
            resolve_display_name(&path, has_route_properties, &route_props).unwrap_or_else(|| id.clone());

        routes.push(RouteEntry {
            id,
            display_name,
            path,
            has_tracks_bin,
            has_route_properties,
            scenario_count,
        });
    }

    routes.sort_by(|a, b| {
        a.display_name
            .to_lowercase()
            .cmp(&b.display_name.to_lowercase())
    });

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

fn resolve_display_name(
    route_dir: &PathBuf,
    has_route_properties: bool,
    route_props: &PathBuf,
) -> Option<String> {
    if has_route_properties {
        if let Some(name) = read_display_name_from_file(route_props) {
            return Some(name);
        }
    }
    let ap = route_dir.join("MainContent.ap");
    if ap.is_file() {
        if let Ok(data) = read_route_properties_from_ap(&ap) {
            return parse_route_properties_xml(&data);
        }
    }
    None
}

fn read_route_properties_from_ap(ap_path: &PathBuf) -> std::io::Result<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;
    use zip::read::ZipArchive;

    let file = File::open(ap_path)?;
    let mut archive = ZipArchive::new(file).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().replace('\\', "/");
        if name.eq_ignore_ascii_case("RouteProperties.xml")
            || name.ends_with("/RouteProperties.xml")
        {
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)?;
            return Ok(buf);
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "RouteProperties.xml not in .ap",
    ))
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
            let name = r.display_name.to_lowercase();
            needles.iter().any(|n| {
                let n = n.to_lowercase();
                id.contains(&n) || name.contains(&n)
            })
        })
        .map(|r| r.display_name.clone())
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
        std::fs::write(
            route_a.join("RouteProperties.xml"),
            br#"<DisplayName>My Test Route</DisplayName>"#,
        )
        .unwrap();

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
        assert_eq!(catalog.routes[0].display_name, "My Test Route");
    }
}
