use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

use crate::blob::{read_blob, write_blob};
use crate::manifest::FrontierManifestV1;
use crate::shard::read_record_shards;

pub const FRONTIER_MANIFEST_FILE: &str = "frontier.manifest.json";

pub fn frontier_checkpoint_dir(run_dir: &Path, step_index: u32, band_index: u32) -> PathBuf {
    run_dir
        .join("checkpoints")
        .join("frontier")
        .join(format!("step-{step_index:02}"))
        .join(format!("band-{band_index:02}"))
}

pub fn manifest_path(frontier_dir: &Path) -> PathBuf {
    frontier_dir.join(FRONTIER_MANIFEST_FILE)
}

pub fn write_frontier_manifest(frontier_dir: &Path, manifest: &FrontierManifestV1) -> Result<()> {
    fs::create_dir_all(frontier_dir)
        .with_context(|| format!("create {}", frontier_dir.display()))?;
    let json = serde_json::to_string_pretty(manifest)?;
    fs::write(manifest_path(frontier_dir), format!("{json}\n"))
        .with_context(|| format!("write {}", manifest_path(frontier_dir).display()))?;
    Ok(())
}

pub fn read_frontier_manifest(path: &Path) -> Result<FrontierManifestV1> {
    let manifest_path = if path.is_dir() {
        manifest_path(path)
    } else {
        path.to_path_buf()
    };
    let text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;
    serde_json::from_str(&text).context("parse frontier manifest")
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierGeneration<const N: usize> {
    pub frontier_dir: PathBuf,
    pub manifest: FrontierManifestV1,
    pub hot_records: Vec<[u8; N]>,
    pub cold_records: Vec<[u8; N]>,
}

impl<const N: usize> FrontierGeneration<N> {
    pub fn total_records(&self) -> usize {
        self.hot_records.len() + self.cold_records.len()
    }
}

pub fn load_frontier_generation<const N: usize>(path: &Path) -> Result<FrontierGeneration<N>> {
    let frontier_dir = frontier_dir(path)?;
    let manifest = read_frontier_manifest(&frontier_dir)?;
    let hot_records = read_named_shards::<N>(&frontier_dir, &manifest.files.hot_shards)?;
    let cold_records = read_named_shards::<N>(&frontier_dir, &manifest.files.cold_shards)?;

    if hot_records.len() as u64 != manifest.counts.hot_states {
        bail!(
            "frontier manifest {} expected {} hot states, found {}",
            manifest_path(&frontier_dir).display(),
            manifest.counts.hot_states,
            hot_records.len()
        );
    }
    if cold_records.len() as u64 != manifest.counts.cold_states {
        bail!(
            "frontier manifest {} expected {} cold states, found {}",
            manifest_path(&frontier_dir).display(),
            manifest.counts.cold_states,
            cold_records.len()
        );
    }

    Ok(FrontierGeneration {
        frontier_dir,
        manifest,
        hot_records,
        cold_records,
    })
}

pub fn load_latest_frontier_generation<const N: usize>(
    run_dir: &Path,
    step_index: u32,
    band_index: u32,
) -> Result<Option<FrontierGeneration<N>>> {
    let frontier_dir = frontier_checkpoint_dir(run_dir, step_index, band_index);
    if !manifest_path(&frontier_dir).exists() {
        return Ok(None);
    }

    load_frontier_generation(&frontier_dir).map(Some)
}

pub fn base_step_checkpoint_path(frontier_dir: &Path, manifest: &FrontierManifestV1) -> PathBuf {
    frontier_dir.join(&manifest.base_step_checkpoint)
}

pub fn write_record_shard<const N: usize>(
    frontier_dir: &Path,
    file_name: &str,
    records: &[[u8; N]],
) -> Result<Option<String>> {
    if records.is_empty() {
        return Ok(None);
    }

    let mut bytes = Vec::with_capacity(records.len() * N);
    for record in records {
        bytes.extend_from_slice(record);
    }
    write_blob(frontier_dir, file_name, &bytes)?;
    Ok(Some(file_name.to_owned()))
}

pub fn read_record_shard<const N: usize>(path: &Path) -> Result<Vec<[u8; N]>> {
    let bytes = read_blob(path)?;
    if bytes.len() % N != 0 {
        bail!(
            "record shard {} had {} bytes, not a multiple of record size {}",
            path.display(),
            bytes.len(),
            N
        );
    }

    Ok(bytes
        .chunks_exact(N)
        .map(|chunk| {
            let mut record = [0_u8; N];
            record.copy_from_slice(chunk);
            record
        })
        .collect())
}

fn frontier_dir(path: &Path) -> Result<PathBuf> {
    if path.is_dir() {
        return Ok(path.to_path_buf());
    }

    path.parent()
        .map(Path::to_path_buf)
        .context("frontier manifest must have a parent directory")
}

fn read_named_shards<const N: usize>(
    frontier_dir: &Path,
    file_names: &[String],
) -> Result<Vec<[u8; N]>> {
    read_record_shards(frontier_dir, file_names)
}

#[cfg(test)]
mod tests {
    use super::{
        FRONTIER_MANIFEST_FILE, base_step_checkpoint_path, frontier_checkpoint_dir,
        load_frontier_generation, load_latest_frontier_generation, manifest_path,
        read_frontier_manifest, read_record_shard, write_frontier_manifest, write_record_shard,
    };
    use crate::layout::{FRONTIER_RECORD_LAYOUT_ID, SCHEMA_VERSION_V1};
    use crate::manifest::{
        FrontierCounts, FrontierFiles, FrontierManifestV1, FrontierScheduler, MemorySnapshot,
        ResumeCompatible,
    };
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-store-frontier-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    fn frontier_manifest() -> FrontierManifestV1 {
        FrontierManifestV1 {
            schema_version: SCHEMA_VERSION_V1,
            run_id: "run-1".to_owned(),
            step_index: 15,
            band_index: 8,
            frontier_epoch: 12,
            base_step_checkpoint: "../../../steps/step-15.json".to_owned(),
            resume_compatible: ResumeCompatible {
                ast_schema_hash: "blake3:ast".to_owned(),
                type_rules_hash: "blake3:type".to_owned(),
                evaluator_hash: "blake3:eval".to_owned(),
                search_semantics_hash: "blake3:search".to_owned(),
                record_layout_id: FRONTIER_RECORD_LAYOUT_ID.to_owned(),
            },
            counts: FrontierCounts {
                prefixes_created: 11,
                prefix_states_explored: 8,
                prefix_states_merged_by_signature: 5,
                prefix_states_exact_pruned: 2,
                prefix_states_heuristic_dropped: 1,
                incremental_legality_cache_hits: 13,
                incremental_connectivity_shortcuts: 3,
                incremental_connectivity_fallbacks: 1,
                incremental_connectivity_prunes: 2,
                incremental_clause_family_filter_hits: 4,
                incremental_clause_family_prunes: 1,
                incremental_active_window_clause_filter_hits: 3,
                incremental_active_window_clause_filter_prunes: 2,
                incremental_terminal_clause_filter_hits: 2,
                incremental_terminal_clause_filter_prunes: 1,
                incremental_trivial_derivability_hits: 5,
                incremental_trivial_derivability_prunes: 1,
                incremental_terminal_admissibility_hits: 3,
                incremental_terminal_admissibility_rejections: 1,
                incremental_terminal_prefix_completion_hits: 2,
                incremental_terminal_rank_prunes: 1,
                incremental_partial_prefix_bound_hits: 1,
                incremental_partial_prefix_bound_checks: 2,
                incremental_partial_prefix_bound_prunes: 1,
                incremental_terminal_prefix_bar_prunes: 2,
                hot_states: 2,
                cold_states: 3,
                dedupe_keys: 5,
            },
            files: FrontierFiles {
                hot_shards: vec!["hot-000.bin".to_owned()],
                cold_shards: vec!["cold-000.bin".to_owned()],
                dedupe_segments: Vec::new(),
                cache_blob: String::new(),
            },
            memory_snapshot: MemorySnapshot {
                rss_bytes: 0,
                hot_frontier_bytes: 128,
                interner_bytes: 0,
                dedupe_bytes: 80,
                cache_bytes: 0,
            },
            scheduler: FrontierScheduler {
                worker_count: 1,
                priority_heads: vec![1, 2, 3],
                spill_generation: 0,
            },
        }
    }

    #[test]
    fn frontier_manifest_helpers_round_trip() {
        let root = temp_dir("manifest");
        let dir = frontier_checkpoint_dir(&root, 15, 8);
        let manifest = frontier_manifest();

        write_frontier_manifest(&dir, &manifest).expect("frontier manifest should write");
        assert!(manifest_path(&dir).ends_with(FRONTIER_MANIFEST_FILE));

        let round_trip = read_frontier_manifest(&dir).expect("frontier manifest should read");
        assert_eq!(round_trip, manifest);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn record_shards_round_trip_fixed_width_bytes() {
        let root = temp_dir("records");
        let dir = frontier_checkpoint_dir(&root, 15, 8);
        let records = [[1_u8; 64], [2_u8; 64], [3_u8; 64]];

        let file_name = write_record_shard(&dir, "hot-000.bin", &records)
            .expect("record shard should write")
            .expect("record shard should be present");
        let round_trip =
            read_record_shard::<64>(&dir.join(file_name)).expect("record shard should read");
        assert_eq!(round_trip, records);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn load_frontier_generation_reads_manifest_and_all_record_shards() {
        let root = temp_dir("generation");
        let dir = frontier_checkpoint_dir(&root, 15, 8);
        let mut manifest = frontier_manifest();
        let hot_records = [[1_u8; 64], [2_u8; 64]];
        let cold_records = [[3_u8; 64]];
        write_record_shard(&dir, "hot-000.bin", &hot_records).expect("write hot shard");
        write_record_shard(&dir, "cold-000.bin", &cold_records).expect("write cold shard");
        manifest.counts.hot_states = hot_records.len() as u64;
        manifest.counts.cold_states = cold_records.len() as u64;
        write_frontier_manifest(&dir, &manifest).expect("write manifest");

        let generation = load_frontier_generation::<64>(&dir).expect("generation should read");
        assert_eq!(generation.frontier_dir, dir);
        assert_eq!(generation.manifest, manifest);
        assert_eq!(generation.hot_records, hot_records);
        assert_eq!(generation.cold_records, cold_records);
        assert_eq!(
            base_step_checkpoint_path(&generation.frontier_dir, &generation.manifest),
            generation.frontier_dir.join("../../../steps/step-15.json")
        );

        let latest = load_latest_frontier_generation::<64>(&root, 15, 8)
            .expect("latest frontier should read")
            .expect("latest frontier should exist");
        assert_eq!(latest.total_records(), 3);

        fs::remove_dir_all(root).ok();
    }
}
