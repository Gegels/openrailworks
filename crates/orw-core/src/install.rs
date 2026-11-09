use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::config::resolve_install_root;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallLayout {
    /// `Content/Routes` (modern TSC / RailWorks)
    ContentRoutes,
    /// Legacy top-level `Routes`
    Routes,
}

#[derive(Debug, Clone)]
pub struct RailWorksInstall {
    pub root: PathBuf,
    pub layout: InstallLayout,
    pub routes_dir: PathBuf,
    pub packages_dir: Option<PathBuf>,
}

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("no Train Simulator Classic / RailWorks installation path was configured")]
    PathNotConfigured,
    #[error("path does not exist: {}", .0.display())]
    NotFound(PathBuf),
    #[error("not a RailWorks/TSC installation (missing Content/Routes or Routes): {}", .0.display())]
    InvalidLayout(PathBuf),
}

impl RailWorksInstall {
    pub fn discover(cli_path: Option<&Path>) -> Result<Self, InstallError> {
        let root = resolve_install_root(cli_path).ok_or(InstallError::PathNotConfigured)?;
        Self::from_root(root)
    }

    pub fn from_root(root: PathBuf) -> Result<Self, InstallError> {
        if !root.is_dir() {
            return Err(InstallError::NotFound(root));
        }

        let content_routes = root.join("Content").join("Routes");
        let top_routes = root.join("Routes");

        let (layout, routes_dir) = if content_routes.is_dir() {
            (InstallLayout::ContentRoutes, content_routes)
        } else if top_routes.is_dir() {
            (InstallLayout::Routes, top_routes)
        } else {
            return Err(InstallError::InvalidLayout(root));
        };

        let packages = {
            let p = root.join("Content").join("Packages");
            if p.is_dir() {
                Some(p)
            } else {
                let legacy = root.join("Packages");
                if legacy.is_dir() {
                    Some(legacy)
                } else {
                    None
                }
            }
        };

        Ok(Self {
            root,
            layout,
            routes_dir,
            packages_dir: packages,
        })
    }

    pub fn is_valid_marker(&self) -> bool {
        self.routes_dir.is_dir()
    }
}
