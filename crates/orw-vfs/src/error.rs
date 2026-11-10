use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VfsError {
    #[error("path escapes installation root: {}", .0.display())]
    PathEscape(PathBuf),
    #[error("not found: {}", .0.display())]
    NotFound(PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid .ap archive {}: {}", .0.display(), .1)]
    ApInvalid(PathBuf, String),
    #[error(transparent)]
    Install(#[from] orw_core::InstallError),
}
