use anyhow::{Result, bail};
use std::path::Path;

use crate::blob::{read_blob, write_blob};
use crate::checksum::FileChecksum;

pub fn write_record_shards<const N: usize>(
    frontier_dir: &Path,
    prefix: &str,
    records: &[[u8; N]],
    max_records_per_shard: usize,
) -> Result<Vec<FileChecksum>> {
    let max_records_per_shard = max_records_per_shard.max(1);
    let mut shards = Vec::new();

    for (index, chunk) in records.chunks(max_records_per_shard).enumerate() {
        let mut bytes = Vec::with_capacity(chunk.len() * N);
        for record in chunk {
            bytes.extend_from_slice(record);
        }
        let file_name = format!("{prefix}-{index:03}.bin");
        if let Some(checksum) = write_blob(frontier_dir, &file_name, &bytes)? {
            shards.push(checksum);
        }
    }

    Ok(shards)
}

pub fn read_record_shards<const N: usize>(
    frontier_dir: &Path,
    file_names: &[String],
) -> Result<Vec<[u8; N]>> {
    let mut records = Vec::new();
    for file_name in file_names {
        let path = frontier_dir.join(file_name);
        let bytes = read_blob(&path)?;
        if bytes.len() % N != 0 {
            bail!(
                "record shard {} had {} bytes, not a multiple of record size {}",
                path.display(),
                bytes.len(),
                N
            );
        }
        for chunk in bytes.chunks_exact(N) {
            let mut record = [0_u8; N];
            record.copy_from_slice(chunk);
            records.push(record);
        }
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::{read_record_shards, write_record_shards};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-store-shard-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn record_shards_split_and_round_trip() {
        let root = temp_dir("round-trip");
        let records = [[1_u8; 8], [2_u8; 8], [3_u8; 8], [4_u8; 8], [5_u8; 8]];
        let shards = write_record_shards(&root, "cold", &records, 2).expect("write shards");
        assert_eq!(
            shards
                .iter()
                .map(|entry| entry.file_name.as_str())
                .collect::<Vec<_>>(),
            vec!["cold-000.bin", "cold-001.bin", "cold-002.bin"]
        );
        let round_trip = read_record_shards::<8>(
            &root,
            &shards
                .iter()
                .map(|entry| entry.file_name.clone())
                .collect::<Vec<_>>(),
        )
        .expect("read shards");
        assert_eq!(round_trip, records);

        fs::remove_dir_all(root).ok();
    }
}
