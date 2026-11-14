//! Read-only VFS: loose files on disk win over the same path inside `.ap` (ZIP) archives.

mod ap;
mod error;
mod vfs;

pub use ap::{ApArchive, ApEntry};
pub use error::VfsError;
pub use vfs::{RailWorksVfs, VfsEntryMeta, VfsRead};
