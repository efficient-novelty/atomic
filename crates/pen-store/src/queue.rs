use anyhow::Result;
use std::path::Path;

use crate::checksum::FileChecksum;
use crate::shard::{read_record_shards, write_record_shards};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueSegments {
    pub hot_shards: Vec<FileChecksum>,
    pub cold_shards: Vec<FileChecksum>,
}

pub fn persist_frontier_queue<const N: usize>(
    frontier_dir: &Path,
    hot_records: &[[u8; N]],
    cold_records: &[[u8; N]],
    max_records_per_shard: usize,
) -> Result<QueueSegments> {
    Ok(QueueSegments {
        hot_shards: write_record_shards(frontier_dir, "hot", hot_records, max_records_per_shard)?,
        cold_shards: write_record_shards(
            frontier_dir,
            "cold",
            cold_records,
            max_records_per_shard,
        )?,
    })
}

pub fn load_frontier_queue<const N: usize>(
    frontier_dir: &Path,
    hot_files: &[String],
    cold_files: &[String],
) -> Result<(Vec<[u8; N]>, Vec<[u8; N]>)> {
    Ok((
        read_record_shards(frontier_dir, hot_files)?,
        read_record_shards(frontier_dir, cold_files)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::{load_frontier_queue, persist_frontier_queue};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-store-queue-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn queue_segments_preserve_hot_and_cold_order() {
        let root = temp_dir("round-trip");
        let hot = [[1_u8; 4], [2_u8; 4], [3_u8; 4]];
        let cold = [[9_u8; 4], [8_u8; 4]];

        let persisted = persist_frontier_queue(&root, &hot, &cold, 2).expect("persist queue");
        let (loaded_hot, loaded_cold) = load_frontier_queue::<4>(
            &root,
            &persisted
                .hot_shards
                .iter()
                .map(|entry| entry.file_name.clone())
                .collect::<Vec<_>>(),
            &persisted
                .cold_shards
                .iter()
                .map(|entry| entry.file_name.clone())
                .collect::<Vec<_>>(),
        )
        .expect("load queue");

        assert_eq!(loaded_hot, hot);
        assert_eq!(loaded_cold, cold);

        fs::remove_dir_all(root).ok();
    }
}
