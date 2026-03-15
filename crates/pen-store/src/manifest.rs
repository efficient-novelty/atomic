use pen_core::library::LibrarySnapshot;
use pen_core::rational::Rational;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::layout::{FRONTIER_RECORD_LAYOUT_ID, SCHEMA_VERSION_V1};

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct RunManifestV1 {
    pub schema_version: u32,
    pub run_id: String,
    pub status: RunStatus,
    pub created_utc: String,
    pub updated_utc: String,
    pub workspace_version: String,
    pub compat: RunCompat,
    pub host: HostInfo,
    pub config: ConfigFingerprint,
    pub position: RunPosition,
    pub artifacts: RunArtifacts,
}

impl Default for RunManifestV1 {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION_V1,
            run_id: String::new(),
            status: RunStatus::Running,
            created_utc: String::new(),
            updated_utc: String::new(),
            workspace_version: String::new(),
            compat: RunCompat::default(),
            host: HostInfo::default(),
            config: ConfigFingerprint::default(),
            position: RunPosition::default(),
            artifacts: RunArtifacts::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct StepCheckpointV1 {
    pub schema_version: u32,
    pub run_id: String,
    pub step_index: u32,
    pub accepted_utc: String,
    pub compat: CheckpointCompat,
    pub objective: StepObjective,
    pub accepted: AcceptedCandidate,
    pub library_snapshot: LibrarySnapshot,
    pub near_misses: Vec<NearMiss>,
    pub stats: StepStats,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct FrontierManifestV1 {
    pub schema_version: u32,
    pub run_id: String,
    pub step_index: u32,
    pub band_index: u32,
    pub frontier_epoch: u32,
    pub base_step_checkpoint: String,
    pub resume_compatible: ResumeCompatible,
    pub counts: FrontierCounts,
    pub files: FrontierFiles,
    pub memory_snapshot: MemorySnapshot,
    pub scheduler: FrontierScheduler,
}

impl Default for FrontierManifestV1 {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION_V1,
            run_id: String::new(),
            step_index: 0,
            band_index: 0,
            frontier_epoch: 0,
            base_step_checkpoint: String::new(),
            resume_compatible: ResumeCompatible::default(),
            counts: FrontierCounts::default(),
            files: FrontierFiles::default(),
            memory_snapshot: MemorySnapshot::default(),
            scheduler: FrontierScheduler::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    #[default]
    Running,
    Paused,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct RunCompat {
    pub ast_schema_hash: String,
    pub type_rules_hash: String,
    pub evaluator_hash: String,
    pub search_semantics_hash: String,
    pub store_schema_hash: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct CheckpointCompat {
    pub ast_schema_hash: String,
    pub type_rules_hash: String,
    pub evaluator_hash: String,
    pub search_semantics_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct ResumeCompatible {
    pub ast_schema_hash: String,
    pub type_rules_hash: String,
    pub evaluator_hash: String,
    pub search_semantics_hash: String,
    pub record_layout_id: String,
}

impl Default for ResumeCompatible {
    fn default() -> Self {
        Self {
            ast_schema_hash: String::new(),
            type_rules_hash: String::new(),
            evaluator_hash: String::new(),
            search_semantics_hash: String::new(),
            record_layout_id: FRONTIER_RECORD_LAYOUT_ID.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct HostInfo {
    pub os: String,
    pub arch: String,
    pub logical_cpus: u32,
    pub ram_bytes: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct ConfigFingerprint {
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct RunPosition {
    pub completed_step: u32,
    pub active_step: u32,
    pub active_band: u32,
    pub frontier_epoch: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct RunArtifacts {
    pub telemetry: String,
    pub reports_dir: String,
    pub checkpoints_dir: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct StepObjective {
    pub bar: Rational,
    pub exact_clause_kappa: u16,
    pub bit_band: BitBand,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct BitBand {
    pub min: u16,
    pub max: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct AcceptedCandidate {
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub bit_kappa: u16,
    pub clause_kappa: u16,
    pub nu: u16,
    pub rho: Rational,
    pub overshoot: Rational,
    pub shape_fingerprint: String,
    pub support_fingerprint: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct NearMiss {
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub bit_kappa: u16,
    pub clause_kappa: u16,
    pub nu: u16,
    pub status: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct StepStats {
    pub frontier_scanned: u64,
    pub typed_prefixes: u64,
    pub sound_prunes: u64,
    pub heuristic_drops: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct FrontierCounts {
    #[serde(default)]
    pub prefixes_created: u64,
    pub prefix_states_explored: u64,
    pub prefix_states_merged_by_signature: u64,
    pub prefix_states_exact_pruned: u64,
    pub prefix_states_heuristic_dropped: u64,
    #[serde(default)]
    pub incremental_legality_cache_hits: u64,
    #[serde(default)]
    pub incremental_connectivity_shortcuts: u64,
    #[serde(default)]
    pub incremental_connectivity_fallbacks: u64,
    #[serde(default)]
    pub incremental_connectivity_prunes: u64,
    #[serde(default)]
    pub incremental_clause_family_filter_hits: u64,
    #[serde(default)]
    pub incremental_clause_family_prunes: u64,
    pub hot_states: u64,
    pub cold_states: u64,
    pub dedupe_keys: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct FrontierFiles {
    pub hot_shards: Vec<String>,
    pub cold_shards: Vec<String>,
    pub dedupe_segments: Vec<String>,
    pub cache_blob: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct MemorySnapshot {
    pub rss_bytes: u64,
    pub hot_frontier_bytes: u64,
    pub interner_bytes: u64,
    pub dedupe_bytes: u64,
    pub cache_bytes: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct FrontierScheduler {
    pub worker_count: u16,
    pub priority_heads: Vec<u32>,
    pub spill_generation: u32,
}

#[cfg(test)]
mod tests {
    use super::{
        AcceptedCandidate, BitBand, CheckpointCompat, ConfigFingerprint, FrontierCounts,
        FrontierFiles, FrontierManifestV1, FrontierScheduler, HostInfo, MemorySnapshot, NearMiss,
        ResumeCompatible, RunArtifacts, RunCompat, RunManifestV1, RunPosition, RunStatus,
        StepCheckpointV1, StepObjective, StepStats,
    };
    use crate::layout::{FRONTIER_RECORD_LAYOUT_ID, SCHEMA_VERSION_V1};
    use pen_core::library::{LibrarySnapshot, LibrarySnapshotEntry};
    use pen_core::rational::Rational;
    use pen_core::telescope::Telescope;

    #[test]
    fn run_manifest_round_trip_preserves_frozen_keys() {
        let manifest = RunManifestV1 {
            schema_version: SCHEMA_VERSION_V1,
            run_id: "2026-03-13T14-32-21Z-desktop16".to_owned(),
            status: RunStatus::Running,
            created_utc: "2026-03-13T14:32:21Z".to_owned(),
            updated_utc: "2026-03-13T17:05:09Z".to_owned(),
            workspace_version: "0.1.0".to_owned(),
            compat: RunCompat {
                ast_schema_hash: "blake3:ast".to_owned(),
                type_rules_hash: "blake3:type".to_owned(),
                evaluator_hash: "blake3:eval".to_owned(),
                search_semantics_hash: "blake3:search".to_owned(),
                store_schema_hash: "blake3:store".to_owned(),
            },
            host: HostInfo {
                os: "windows".to_owned(),
                arch: "x86_64".to_owned(),
                logical_cpus: 16,
                ram_bytes: 17_179_869_184,
            },
            config: ConfigFingerprint {
                path: "config.toml".to_owned(),
                sha256: "sha".to_owned(),
            },
            position: RunPosition {
                completed_step: 9,
                active_step: 10,
                active_band: 4,
                frontier_epoch: 17,
            },
            artifacts: RunArtifacts {
                telemetry: "telemetry.ndjson".to_owned(),
                reports_dir: "reports".to_owned(),
                checkpoints_dir: "checkpoints".to_owned(),
            },
        };

        let json = serde_json::to_string_pretty(&manifest).expect("serialize run manifest");
        assert!(json.contains("\"schema_version\": 1"));
        assert!(json.contains("\"store_schema_hash\": \"blake3:store\""));

        let round_trip: RunManifestV1 =
            serde_json::from_str(&json).expect("deserialize run manifest");
        assert_eq!(round_trip, manifest);
    }

    #[test]
    fn step_checkpoint_round_trip_preserves_exact_rationals() {
        let checkpoint = StepCheckpointV1 {
            schema_version: SCHEMA_VERSION_V1,
            run_id: "run-1".to_owned(),
            step_index: 9,
            accepted_utc: "2026-03-13T16:58:11Z".to_owned(),
            compat: CheckpointCompat {
                ast_schema_hash: "blake3:ast".to_owned(),
                type_rules_hash: "blake3:type".to_owned(),
                evaluator_hash: "blake3:eval".to_owned(),
                search_semantics_hash: "blake3:search".to_owned(),
            },
            objective: StepObjective {
                bar: Rational::new(401, 100),
                exact_clause_kappa: 4,
                bit_band: BitBand { min: 76, max: 84 },
            },
            accepted: AcceptedCandidate {
                candidate_hash: "blake3:candidate".to_owned(),
                canonical_hash: "blake3:canonical".to_owned(),
                bit_kappa: 78,
                clause_kappa: 4,
                nu: 17,
                rho: Rational::new(17, 4),
                overshoot: Rational::new(66, 100),
                shape_fingerprint: "0xshape".to_owned(),
                support_fingerprint: "0xsupport".to_owned(),
            },
            library_snapshot: LibrarySnapshot {
                window_depth: 2,
                entries: vec![LibrarySnapshotEntry {
                    step: 1,
                    candidate_hash: "blake3:step1".to_owned(),
                    telescope: Telescope::reference(1),
                }],
            },
            near_misses: vec![NearMiss {
                candidate_hash: "blake3:near".to_owned(),
                canonical_hash: "blake3:near-canonical".to_owned(),
                bit_kappa: 79,
                clause_kappa: 4,
                nu: 18,
                status: "bar_clear_higher_overshoot".to_owned(),
            }],
            stats: StepStats {
                frontier_scanned: 4_123_312,
                typed_prefixes: 991_281,
                sound_prunes: 3_400_012,
                heuristic_drops: 120_398,
            },
        };

        let json = serde_json::to_string_pretty(&checkpoint).expect("serialize step checkpoint");
        assert!(json.contains("\"num\": 401"));
        assert!(json.contains("\"den\": 100"));

        let round_trip: StepCheckpointV1 =
            serde_json::from_str(&json).expect("deserialize step checkpoint");
        assert_eq!(round_trip, checkpoint);
    }

    #[test]
    fn frontier_manifest_round_trip_preserves_resume_layout_id() {
        let manifest = FrontierManifestV1 {
            schema_version: SCHEMA_VERSION_V1,
            run_id: "run-1".to_owned(),
            step_index: 10,
            band_index: 4,
            frontier_epoch: 17,
            base_step_checkpoint: "../../../steps/step-09-hopf.cbor.zst".to_owned(),
            resume_compatible: ResumeCompatible {
                ast_schema_hash: "blake3:ast".to_owned(),
                type_rules_hash: "blake3:type".to_owned(),
                evaluator_hash: "blake3:eval".to_owned(),
                search_semantics_hash: "blake3:search".to_owned(),
                record_layout_id: FRONTIER_RECORD_LAYOUT_ID.to_owned(),
            },
            counts: FrontierCounts {
                prefixes_created: 26_114_992,
                prefix_states_explored: 17_449_876,
                prefix_states_merged_by_signature: 4_110_551,
                prefix_states_exact_pruned: 1_220_014,
                prefix_states_heuristic_dropped: 342_118,
                incremental_legality_cache_hits: 8_604_229,
                incremental_connectivity_shortcuts: 91_220,
                incremental_connectivity_fallbacks: 14_331,
                incremental_connectivity_prunes: 203_114,
                incremental_clause_family_filter_hits: 5_882_441,
                incremental_clause_family_prunes: 118_204,
                hot_states: 2_241_132,
                cold_states: 15_208_744,
                dedupe_keys: 10_677_731,
            },
            files: FrontierFiles {
                hot_shards: vec!["hot-000.bin.zst".to_owned(), "hot-001.bin.zst".to_owned()],
                cold_shards: vec!["cold-000.bin.zst".to_owned(), "cold-001.bin.zst".to_owned()],
                dedupe_segments: vec!["dedupe-000.bin.zst".to_owned()],
                cache_blob: "cache.bin.zst".to_owned(),
            },
            memory_snapshot: MemorySnapshot {
                rss_bytes: 8_642_146_304,
                hot_frontier_bytes: 2_147_483_648,
                interner_bytes: 1_744_830_464,
                dedupe_bytes: 1_107_296_256,
                cache_bytes: 671_088_640,
            },
            scheduler: FrontierScheduler {
                worker_count: 12,
                priority_heads: vec![101, 117, 128, 144],
                spill_generation: 23,
            },
        };

        let json = serde_json::to_string_pretty(&manifest).expect("serialize frontier manifest");
        assert!(json.contains(FRONTIER_RECORD_LAYOUT_ID));

        let round_trip: FrontierManifestV1 =
            serde_json::from_str(&json).expect("deserialize frontier manifest");
        assert_eq!(round_trip, manifest);
    }
}
