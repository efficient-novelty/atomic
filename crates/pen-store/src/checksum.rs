use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FileChecksum {
    pub file_name: String,
    pub sha256: String,
    pub bytes: u64,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn checksum_sidecar_path(path: &Path) -> Result<PathBuf> {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .context("checksum sidecar target must have a file name")?;
    Ok(path.with_file_name(format!("{file_name}.sha256")))
}

pub fn write_checksum_sidecar(path: &Path, bytes: &[u8]) -> Result<FileChecksum> {
    let sha256 = sha256_hex(bytes);
    let sidecar = checksum_sidecar_path(path)?;
    fs::write(&sidecar, format!("{sha256}\n"))
        .with_context(|| format!("write {}", sidecar.display()))?;
    Ok(FileChecksum {
        file_name: path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .context("checksummed path must have a file name")?,
        sha256,
        bytes: bytes.len() as u64,
    })
}

pub fn verify_checksum_sidecar(path: &Path, bytes: &[u8]) -> Result<()> {
    let sidecar = checksum_sidecar_path(path)?;
    if !sidecar.exists() {
        return Ok(());
    }

    let expected = fs::read_to_string(&sidecar)
        .with_context(|| format!("read {}", sidecar.display()))?
        .trim()
        .to_owned();
    let actual = sha256_hex(bytes);
    if expected != actual {
        bail!(
            "checksum mismatch for {}: expected {}, found {}",
            path.display(),
            expected,
            actual
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{checksum_sidecar_path, verify_checksum_sidecar, write_checksum_sidecar};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-store-checksum-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn checksum_sidecars_round_trip() {
        let root = temp_dir("round-trip");
        let path = root.join("frontier.bin");
        let bytes = b"frontier-runtime".to_vec();
        fs::write(&path, &bytes).expect("write payload");

        let checksum = write_checksum_sidecar(&path, &bytes).expect("write checksum");
        assert_eq!(checksum.file_name, "frontier.bin");
        assert!(checksum_sidecar_path(&path).expect("sidecar path").exists());
        verify_checksum_sidecar(&path, &bytes).expect("checksum should verify");

        fs::remove_dir_all(root).ok();
    }
}
