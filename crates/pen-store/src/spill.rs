use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

use crate::blob::{read_blob, write_blob};
use crate::checksum::FileChecksum;
use crate::manifest::{FrontierCounts, FrontierFiles, FrontierScheduler, MemorySnapshot};
use crate::memory::{
    GovernorConfig, GovernorState, MemoryUsage, PressureAction, evaluate_governor,
};
use crate::queue::persist_frontier_queue;

pub const FRONTIER_CACHE_BLOB_FILE: &str = "frontier-runtime.json";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpillConfig {
    pub max_records_per_shard: usize,
    pub max_dedupe_keys_per_segment: usize,
    pub resident_cold_records: usize,
}

impl Default for SpillConfig {
    fn default() -> Self {
        Self {
            max_records_per_shard: 1024,
            max_dedupe_keys_per_segment: 4096,
            resident_cold_records: 1024,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierRuntimeInput<const N: usize> {
    pub frontier_epoch: u32,
    pub hot_records: Vec<[u8; N]>,
    pub cold_records: Vec<[u8; N]>,
    pub dedupe_keys: Vec<String>,
    pub prefixes_created: u64,
    pub prefix_states_explored: u64,
    pub prefix_states_merged_by_signature: u64,
    pub prefix_states_exact_pruned: u64,
    pub prefix_states_heuristic_dropped: u64,
    pub incremental_legality_cache_hits: u64,
    pub incremental_connectivity_shortcuts: u64,
    pub incremental_connectivity_fallbacks: u64,
    pub incremental_connectivity_prunes: u64,
    pub worker_count: u16,
    pub priority_heads: Vec<u32>,
    pub interner_bytes: u64,
    pub worker_scratch_bytes: u64,
    pub checkpoint_bytes: u64,
    pub spill_buffer_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierRuntimeArtifacts {
    pub counts: FrontierCounts,
    pub files: FrontierFiles,
    pub memory_snapshot: MemorySnapshot,
    pub scheduler: FrontierScheduler,
    pub governor_state: GovernorState,
    pub pressure_action: PressureAction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FrontierCacheBlob {
    pub schema_version: u32,
    pub governor_state: GovernorState,
    pub pressure_action: PressureAction,
    pub rss_bytes: u64,
    pub cold_frontier_bytes: u64,
    pub worker_scratch_bytes: u64,
    pub checkpoint_bytes: u64,
    pub spill_buffer_bytes: u64,
    pub resident_cold_records: u64,
    pub spilled_cold_records: u64,
    pub dedupe_segment_count: u64,
    pub checksums: Vec<FileChecksum>,
}

pub fn persist_frontier_runtime<const N: usize>(
    frontier_dir: &Path,
    input: &FrontierRuntimeInput<N>,
    governor_config: GovernorConfig,
    spill_config: SpillConfig,
) -> Result<FrontierRuntimeArtifacts> {
    let queue = persist_frontier_queue(
        frontier_dir,
        &input.hot_records,
        &input.cold_records,
        spill_config.max_records_per_shard,
    )?;
    let dedupe_segments = write_dedupe_segments(
        frontier_dir,
        &input.dedupe_keys,
        spill_config.max_dedupe_keys_per_segment,
    )?;
    let dedupe_bytes = dedupe_segments.iter().map(|entry| entry.bytes).sum::<u64>();
    let hot_frontier_bytes = (input.hot_records.len() * N) as u64;
    let cold_frontier_bytes = (input.cold_records.len() * N) as u64;

    let initial_usage = MemoryUsage {
        hot_frontier_bytes,
        cold_frontier_bytes,
        interner_bytes: input.interner_bytes,
        dedupe_bytes,
        cache_bytes: 0,
        worker_scratch_bytes: input.worker_scratch_bytes,
        checkpoint_bytes: input.checkpoint_bytes,
        spill_buffer_bytes: input.spill_buffer_bytes,
    };
    let initial_decision = evaluate_governor(governor_config, initial_usage);
    let (resident_cold_records, spilled_cold_records) = cold_residency(
        input.cold_records.len(),
        spill_config,
        initial_decision.action,
    );
    let checksums = queue
        .hot_shards
        .iter()
        .chain(queue.cold_shards.iter())
        .chain(dedupe_segments.iter())
        .cloned()
        .collect::<Vec<_>>();
    let provisional_cache = FrontierCacheBlob {
        schema_version: 1,
        governor_state: initial_decision.state,
        pressure_action: initial_decision.action,
        rss_bytes: initial_decision.rss_bytes,
        cold_frontier_bytes,
        worker_scratch_bytes: input.worker_scratch_bytes,
        checkpoint_bytes: input.checkpoint_bytes,
        spill_buffer_bytes: input.spill_buffer_bytes,
        resident_cold_records: resident_cold_records as u64,
        spilled_cold_records: spilled_cold_records as u64,
        dedupe_segment_count: dedupe_segments.len() as u64,
        checksums: checksums.clone(),
    };
    let cache_bytes = serde_json::to_vec_pretty(&provisional_cache)
        .context("serialize provisional frontier cache blob")?
        .len() as u64;
    let final_usage = MemoryUsage {
        cache_bytes,
        ..initial_usage
    };
    let final_decision = evaluate_governor(governor_config, final_usage);
    let (resident_cold_records, spilled_cold_records) = cold_residency(
        input.cold_records.len(),
        spill_config,
        final_decision.action,
    );
    let cache_blob = FrontierCacheBlob {
        schema_version: 1,
        governor_state: final_decision.state,
        pressure_action: final_decision.action,
        rss_bytes: final_decision.rss_bytes,
        cold_frontier_bytes,
        worker_scratch_bytes: input.worker_scratch_bytes,
        checkpoint_bytes: input.checkpoint_bytes,
        spill_buffer_bytes: input.spill_buffer_bytes,
        resident_cold_records: resident_cold_records as u64,
        spilled_cold_records: spilled_cold_records as u64,
        dedupe_segment_count: dedupe_segments.len() as u64,
        checksums,
    };
    let cache_blob_bytes =
        serde_json::to_vec_pretty(&cache_blob).context("serialize frontier cache blob")?;
    let cache_blob_file = write_blob(frontier_dir, FRONTIER_CACHE_BLOB_FILE, &cache_blob_bytes)?
        .expect("frontier cache blob should be written");
    let spill_generation = if spilled_cold_records > 0 || final_decision.action.spills() {
        input.frontier_epoch
    } else {
        0
    };

    Ok(FrontierRuntimeArtifacts {
        counts: FrontierCounts {
            prefixes_created: input.prefixes_created,
            prefix_states_explored: input.prefix_states_explored,
            prefix_states_merged_by_signature: input.prefix_states_merged_by_signature,
            prefix_states_exact_pruned: input.prefix_states_exact_pruned,
            prefix_states_heuristic_dropped: input.prefix_states_heuristic_dropped,
            incremental_legality_cache_hits: input.incremental_legality_cache_hits,
            incremental_connectivity_shortcuts: input.incremental_connectivity_shortcuts,
            incremental_connectivity_fallbacks: input.incremental_connectivity_fallbacks,
            incremental_connectivity_prunes: input.incremental_connectivity_prunes,
            hot_states: input.hot_records.len() as u64,
            cold_states: input.cold_records.len() as u64,
            dedupe_keys: BTreeSet::<String>::from_iter(input.dedupe_keys.iter().cloned()).len()
                as u64,
        },
        files: FrontierFiles {
            hot_shards: queue
                .hot_shards
                .iter()
                .map(|entry| entry.file_name.clone())
                .collect(),
            cold_shards: queue
                .cold_shards
                .iter()
                .map(|entry| entry.file_name.clone())
                .collect(),
            dedupe_segments: dedupe_segments
                .iter()
                .map(|entry| entry.file_name.clone())
                .collect(),
            cache_blob: cache_blob_file.file_name,
        },
        memory_snapshot: MemorySnapshot {
            rss_bytes: final_decision.rss_bytes,
            hot_frontier_bytes,
            interner_bytes: input.interner_bytes,
            dedupe_bytes,
            cache_bytes,
        },
        scheduler: FrontierScheduler {
            worker_count: input.worker_count.max(1),
            priority_heads: input.priority_heads.clone(),
            spill_generation,
        },
        governor_state: final_decision.state,
        pressure_action: final_decision.action,
    })
}

pub fn read_frontier_cache_blob(
    frontier_dir: &Path,
    file_name: &str,
) -> Result<Option<FrontierCacheBlob>> {
    if file_name.is_empty() {
        return Ok(None);
    }

    let bytes = read_blob(&frontier_dir.join(file_name))?;
    serde_json::from_slice(&bytes)
        .context("parse frontier cache blob")
        .map(Some)
}

fn cold_residency(
    cold_records: usize,
    spill_config: SpillConfig,
    pressure_action: PressureAction,
) -> (usize, usize) {
    if !pressure_action.spills() {
        return (cold_records, 0);
    }

    let resident = cold_records.min(spill_config.resident_cold_records.max(1));
    (resident, cold_records.saturating_sub(resident))
}

fn write_dedupe_segments(
    frontier_dir: &Path,
    dedupe_keys: &[String],
    max_keys_per_segment: usize,
) -> Result<Vec<FileChecksum>> {
    let max_keys_per_segment = max_keys_per_segment.max(1);
    let dedupe_keys = BTreeSet::<String>::from_iter(dedupe_keys.iter().cloned())
        .into_iter()
        .collect::<Vec<_>>();
    let mut files = Vec::new();
    for (index, chunk) in dedupe_keys.chunks(max_keys_per_segment).enumerate() {
        let file_name = format!("dedupe-{index:03}.txt");
        let bytes = format!("{}\n", chunk.join("\n")).into_bytes();
        if let Some(file) = write_blob(frontier_dir, &file_name, &bytes)? {
            files.push(file);
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::{
        FRONTIER_CACHE_BLOB_FILE, FrontierRuntimeInput, SpillConfig, persist_frontier_runtime,
        read_frontier_cache_blob,
    };
    use crate::memory::GovernorConfig;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-store-spill-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn spill_runtime_writes_shards_cache_blob_and_dedupe_segments() {
        let root = temp_dir("runtime");
        let input = FrontierRuntimeInput {
            frontier_epoch: 7,
            hot_records: vec![[1_u8; 8], [2_u8; 8]],
            cold_records: vec![[9_u8; 8], [8_u8; 8], [7_u8; 8], [6_u8; 8]],
            dedupe_keys: vec![
                "dedupe-b".to_owned(),
                "dedupe-a".to_owned(),
                "dedupe-a".to_owned(),
            ],
            prefixes_created: 9,
            prefix_states_explored: 6,
            prefix_states_merged_by_signature: 3,
            prefix_states_exact_pruned: 1,
            prefix_states_heuristic_dropped: 1,
            incremental_legality_cache_hits: 12,
            incremental_connectivity_shortcuts: 4,
            incremental_connectivity_fallbacks: 2,
            incremental_connectivity_prunes: 3,
            worker_count: 2,
            priority_heads: vec![11, 22],
            interner_bytes: 16,
            worker_scratch_bytes: 24,
            checkpoint_bytes: 32,
            spill_buffer_bytes: 8,
        };
        let artifacts = persist_frontier_runtime(
            &root,
            &input,
            GovernorConfig {
                green_limit_bytes: 16,
                yellow_limit_bytes: 32,
                orange_limit_bytes: 64,
                red_limit_bytes: 96,
                hard_limit_bytes: 128,
            },
            SpillConfig {
                max_records_per_shard: 2,
                max_dedupe_keys_per_segment: 1,
                resident_cold_records: 1,
            },
        )
        .expect("persist runtime");

        assert_eq!(artifacts.files.hot_shards, vec!["hot-000.bin".to_owned()]);
        assert_eq!(
            artifacts.files.cold_shards,
            vec!["cold-000.bin".to_owned(), "cold-001.bin".to_owned()]
        );
        assert_eq!(
            artifacts.files.dedupe_segments,
            vec!["dedupe-000.txt".to_owned(), "dedupe-001.txt".to_owned()]
        );
        assert_eq!(artifacts.files.cache_blob, FRONTIER_CACHE_BLOB_FILE);
        assert_eq!(artifacts.scheduler.spill_generation, 7);
        assert_eq!(artifacts.counts.prefixes_created, 9);
        assert_eq!(artifacts.counts.prefix_states_explored, 6);
        assert_eq!(artifacts.counts.prefix_states_merged_by_signature, 3);
        assert_eq!(artifacts.counts.prefix_states_exact_pruned, 1);
        assert_eq!(artifacts.counts.prefix_states_heuristic_dropped, 1);
        assert_eq!(artifacts.counts.incremental_legality_cache_hits, 12);
        assert_eq!(artifacts.counts.incremental_connectivity_shortcuts, 4);
        assert_eq!(artifacts.counts.incremental_connectivity_fallbacks, 2);
        assert_eq!(artifacts.counts.incremental_connectivity_prunes, 3);
        assert!(root.join(FRONTIER_CACHE_BLOB_FILE).exists());

        let cache_blob = read_frontier_cache_blob(&root, &artifacts.files.cache_blob)
            .expect("read cache blob")
            .expect("cache blob should exist");
        assert_eq!(cache_blob.spilled_cold_records, 3);
        assert_eq!(cache_blob.dedupe_segment_count, 2);

        fs::remove_dir_all(root).ok();
    }
}
