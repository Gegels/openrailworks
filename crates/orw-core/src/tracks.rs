//! Minimal `Tracks.bin` probe until a full parser lands (see docs/FORMATS.md).

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::Serialize;

const HEADER_SAMPLE: usize = 16;

#[derive(Debug, Clone, Serialize)]
pub struct TracksBinProbe {
    pub path: String,
    pub byte_length: u64,
    pub header_hex: String,
    /// True when the file is non-empty (parser not implemented yet).
    pub looks_present: bool,
}

pub fn probe_tracks_bin(path: &Path) -> io::Result<TracksBinProbe> {
    let meta = std::fs::metadata(path)?;
    let byte_length = meta.len();
    let mut header = [0u8; HEADER_SAMPLE];
    let mut file = File::open(path)?;
    let read = file.read(&mut header)?;
    let header_hex: String = header[..read]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    Ok(TracksBinProbe {
        path: path.display().to_string(),
        byte_length,
        header_hex,
        looks_present: byte_length > 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn probes_small_file() {
        let tmp = NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), &[0x01, 0x02, 0x03, 0x00]).unwrap();
        let probe = probe_tracks_bin(tmp.path()).unwrap();
        assert_eq!(probe.byte_length, 4);
        assert!(probe.looks_present);
        assert!(probe.header_hex.starts_with("010203"));
    }
}
