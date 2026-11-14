use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;
use zip::read::ZipArchive;

use crate::error::VfsError;

#[derive(Debug, Clone, Serialize)]
pub struct ApEntry {
    pub archive_path: PathBuf,
    pub inner_path: String,
    pub uncompressed_size: u64,
}

#[derive(Debug)]
pub struct ApArchive {
    path: PathBuf,
}

impl ApArchive {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, VfsError> {
        let path = path.as_ref().to_path_buf();
        if !path.is_file() {
            return Err(VfsError::NotFound(path));
        }
        // Validate header by opening once.
        let file = File::open(&path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| VfsError::ApInvalid(path.clone(), e.to_string()))?;
        archive.by_index(0).ok(); // touch first entry if any
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn list_entries(&self) -> Result<Vec<ApEntry>, VfsError> {
        let file = File::open(&self.path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| VfsError::ApInvalid(self.path.clone(), e.to_string()))?;
        let mut out = Vec::with_capacity(archive.len());
        for i in 0..archive.len() {
            let entry = archive
                .by_index(i)
                .map_err(|e| VfsError::ApInvalid(self.path.clone(), e.to_string()))?;
            if entry.is_dir() {
                continue;
            }
            let name = entry.name().replace('\\', "/");
            out.push(ApEntry {
                archive_path: self.path.clone(),
                inner_path: name,
                uncompressed_size: entry.size(),
            });
        }
        out.sort_by(|a, b| a.inner_path.cmp(&b.inner_path));
        Ok(out)
    }

    pub fn read_entry(&self, inner_path: &str) -> Result<Vec<u8>, VfsError> {
        let normalized = inner_path.replace('\\', "/");
        let file = File::open(&self.path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| VfsError::ApInvalid(self.path.clone(), e.to_string()))?;
        let mut entry = archive
            .by_name(&normalized)
            .map_err(|_| VfsError::NotFound(PathBuf::from(&normalized)))?;
        let mut buf = Vec::new();
        entry
            .read_to_end(&mut buf)
            .map_err(|e| VfsError::ApInvalid(self.path.clone(), e.to_string()))?;
        Ok(buf)
    }
}

/// Collect every `.ap` file under `install_root` (Assets tree and Packages).
pub fn discover_ap_files(install_root: &Path, max_depth: usize) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let roots = [
        install_root.join("Assets"),
        install_root.join("Content").join("Packages"),
        install_root.join("Packages"),
    ];
    for root in roots {
        if !root.is_dir() {
            continue;
        }
        for entry in WalkDir::new(&root)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("ap"))
            {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

/// Build a fake `.ap` ZIP for unit tests (stored, no compression).
#[cfg(test)]
pub(crate) fn write_test_ap(path: &Path, entries: &[(&str, &[u8])]) -> io::Result<()> {
    use std::io::Write;
    let file = File::create(path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    for (name, data) in entries {
        zip.start_file(*name, options)?;
        zip.write_all(data)?;
    }
    zip.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn roundtrip_ap_read() {
        let tmp = TempDir::new().unwrap();
        let ap = tmp.path().join("MainContent.ap");
        write_test_ap(&ap, &[("RailVehicles/foo.bin", b"hello")]).unwrap();
        let archive = ApArchive::open(&ap).unwrap();
        let list = archive.list_entries().unwrap();
        assert_eq!(list.len(), 1);
        let data = archive.read_entry("RailVehicles/foo.bin").unwrap();
        assert_eq!(data, b"hello");
    }
}
