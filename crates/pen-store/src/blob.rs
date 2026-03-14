use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::checksum::{FileChecksum, verify_checksum_sidecar, write_checksum_sidecar};

pub fn write_blob(
    frontier_dir: &Path,
    file_name: &str,
    bytes: &[u8],
) -> Result<Option<FileChecksum>> {
    if bytes.is_empty() {
        return Ok(None);
    }

    fs::create_dir_all(frontier_dir)
        .with_context(|| format!("create {}", frontier_dir.display()))?;
    let path = frontier_dir.join(file_name);
    fs::write(&path, bytes).with_context(|| format!("write {}", path.display()))?;
    write_checksum_sidecar(&path, bytes).map(Some)
}

pub fn read_blob(path: &Path) -> Result<Vec<u8>> {
    let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    verify_checksum_sidecar(path, &bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::{read_blob, write_blob};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-store-blob-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn blobs_round_trip_with_sidecar_checksums() {
        let root = temp_dir("round-trip");
        let checksum = write_blob(&root, "cache.json", br#"{"state":"green"}"#)
            .expect("blob should write")
            .expect("blob should exist");
        assert_eq!(checksum.file_name, "cache.json");
        let bytes = read_blob(&root.join("cache.json")).expect("blob should read");
        assert_eq!(bytes, br#"{"state":"green"}"#);

        fs::remove_dir_all(root).ok();
    }
}
