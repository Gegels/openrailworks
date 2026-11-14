use std::collections::HashMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Serialize;
use tracing::debug;

use orw_core::RailWorksInstall;

use crate::ap::{discover_ap_files, ApArchive};
use crate::error::VfsError;

#[derive(Debug, Clone, Serialize)]
pub struct VfsEntryMeta {
    pub relative_path: String,
    pub source: VfsSource,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum VfsSource {
    Loose,
    Ap { archive: PathBuf, inner_path: String },
}

#[derive(Debug, Clone)]
pub struct VfsRead {
    pub relative_path: String,
    pub source: VfsSource,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
struct ApHit {
    archive: PathBuf,
    inner_path: String,
}

#[derive(Debug, Default)]
struct ApIndex {
    by_full_path: HashMap<String, ApHit>,
}

/// Read-only unified access to a TSC tree. Loose files always beat `.ap` entries.
#[derive(Debug)]
pub struct RailWorksVfs {
    install: RailWorksInstall,
    ap_index: Option<ApIndex>,
}

impl RailWorksVfs {
    pub fn open(install: &RailWorksInstall) -> Result<Self, VfsError> {
        Ok(Self {
            install: install.clone(),
            ap_index: None,
        })
    }

  /// Index every `.ap` under Assets/Packages (can take a while on large installs).
    pub fn with_ap_index(mut self) -> Result<Self, VfsError> {
        self.ap_index = Some(build_ap_index(&self.install)?);
        Ok(self)
    }

    pub fn install(&self) -> &RailWorksInstall {
        &self.install
    }

    pub fn list_ap_archives(&self) -> Result<Vec<PathBuf>, VfsError> {
        discover_ap_files(&self.install.root, 8).map_err(VfsError::from)
    }

    pub fn exists(&self, relative: &str) -> bool {
        self.resolve_loose(relative).is_some()
            || self
                .ap_index
                .as_ref()
                .and_then(|idx| idx.by_full_path.get(&normalize_rel(relative)))
                .is_some()
    }

    pub fn read(&self, relative: &str) -> Result<VfsRead, VfsError> {
        let rel = normalize_rel(relative);
        if let Some(abs) = self.resolve_loose(&rel) {
            let bytes = fs::read(&abs)?;
            return Ok(VfsRead {
                relative_path: rel,
                source: VfsSource::Loose,
                bytes,
            });
        }
        if let Some(index) = &self.ap_index {
            if let Some(hit) = index.by_full_path.get(&rel) {
                let archive = ApArchive::open(&hit.archive)?;
                let bytes = archive.read_entry(&hit.inner_path)?;
                return Ok(VfsRead {
                    relative_path: rel.clone(),
                    source: VfsSource::Ap {
                        archive: hit.archive.clone(),
                        inner_path: hit.inner_path.clone(),
                    },
                    bytes,
                });
            }
        }
        Err(VfsError::NotFound(PathBuf::from(rel)))
    }

    pub fn read_route_file(&self, route_id: &str, file_name: &str) -> Result<VfsRead, VfsError> {
        let rel = format!("Content/Routes/{route_id}/{file_name}");
        self.read(&rel).or_else(|_| {
            // Legacy layout
            self.read(&format!("Routes/{route_id}/{file_name}"))
        })
    }

    fn resolve_loose(&self, relative: &str) -> Option<PathBuf> {
        let joined = safe_join(&self.install.root, relative)?;
        joined.is_file().then_some(joined)
    }
}

fn normalize_rel(relative: &str) -> String {
    relative.replace('\\', "/").trim_start_matches('/').to_string()
}

fn safe_join(root: &Path, relative: &str) -> Option<PathBuf> {
    let mut out = root.to_path_buf();
    for component in Path::new(relative).components() {
        match component {
            Component::Normal(part) => out.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if !out.starts_with(root) {
        return None;
    }
    Some(out)
}

fn build_ap_index(install: &RailWorksInstall) -> Result<ApIndex, VfsError> {
    let archives = discover_ap_files(&install.root, 8)?;
    let mut by_full_path = HashMap::new();
    for ap_path in archives {
        let archive = match ApArchive::open(&ap_path) {
            Ok(a) => a,
            Err(e) => {
                debug!(?ap_path, error = %e, "skip invalid .ap");
                continue;
            }
        };
        let parent = ap_path
            .parent()
            .and_then(|p| p.strip_prefix(&install.root).ok())
            .map(|p| normalize_rel(&p.to_string_lossy()))
            .unwrap_or_default();
        for entry in archive.list_entries()? {
            let key = if parent.is_empty() {
                entry.inner_path.clone()
            } else {
                format!("{}/{}", parent, entry.inner_path)
            };
            by_full_path
                .entry(key)
                .or_insert(ApHit {
                    archive: ap_path.clone(),
                    inner_path: entry.inner_path.clone(),
                });
        }
    }
    Ok(ApIndex { by_full_path })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ap::write_test_ap;
    use orw_core::{InstallLayout, RailWorksInstall};
    use tempfile::TempDir;

    #[test]
    fn loose_beats_ap() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let product = root.join("Assets/RSC/TestRoute");
        std::fs::create_dir_all(&product).unwrap();
        let ap = product.join("MainContent.ap");
        write_test_ap(
            &ap,
            &[("Scenery/tree.bin", b"from-ap")],
        )
        .unwrap();
        std::fs::create_dir_all(product.join("Scenery")).unwrap();
        std::fs::write(product.join("Scenery/tree.bin"), b"from-loose").unwrap();

        let install = RailWorksInstall {
            root: root.clone(),
            layout: InstallLayout::ContentRoutes,
            routes_dir: root.join("Content/Routes"),
            packages_dir: None,
        };
        let vfs = RailWorksVfs::open(&install).unwrap().with_ap_index().unwrap();
        let read = vfs
            .read("Assets/RSC/TestRoute/Scenery/tree.bin")
            .unwrap();
        assert!(matches!(read.source, VfsSource::Loose));
        assert_eq!(read.bytes, b"from-loose");
    }

    #[test]
    fn reads_from_ap_when_no_loose() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let product = root.join("Assets/DTG/LocoPack");
        std::fs::create_dir_all(&product).unwrap();
        write_test_ap(
            &product.join("MainContent.ap"),
            &[("RailVehicles/Engine/geo.bin", b"mesh")],
        )
        .unwrap();

        let install = RailWorksInstall {
            root: root.clone(),
            layout: InstallLayout::ContentRoutes,
            routes_dir: root.join("Content/Routes"),
            packages_dir: None,
        };
        let vfs = RailWorksVfs::open(&install).unwrap().with_ap_index().unwrap();
        let read = vfs
            .read("Assets/DTG/LocoPack/RailVehicles/Engine/geo.bin")
            .unwrap();
        assert!(matches!(read.source, VfsSource::Ap { .. }));
        assert_eq!(read.bytes, b"mesh");
    }
}
