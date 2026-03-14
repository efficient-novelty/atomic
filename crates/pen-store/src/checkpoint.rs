use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::manifest::StepCheckpointV1;

pub fn step_checkpoint_path(run_dir: &Path, step_index: u32) -> PathBuf {
    run_dir
        .join("checkpoints")
        .join("steps")
        .join(format!("step-{step_index:02}.json"))
}

pub fn read_step_checkpoint(path: &Path) -> Result<StepCheckpointV1> {
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&text).context("parse step checkpoint")
}

#[cfg(test)]
mod tests {
    use super::{read_step_checkpoint, step_checkpoint_path};
    use crate::layout::SCHEMA_VERSION_V1;
    use crate::manifest::{
        AcceptedCandidate, BitBand, CheckpointCompat, StepCheckpointV1, StepObjective, StepStats,
    };
    use pen_core::library::{LibrarySnapshot, LibrarySnapshotEntry};
    use pen_core::rational::Rational;
    use pen_core::telescope::Telescope;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-store-checkpoint-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn step_checkpoint_helpers_round_trip() {
        let root = temp_dir("round-trip");
        let path = step_checkpoint_path(&root, 7);
        let checkpoint = StepCheckpointV1 {
            schema_version: SCHEMA_VERSION_V1,
            run_id: "run-1".to_owned(),
            step_index: 7,
            accepted_utc: "2026-03-14T09:10:11Z".to_owned(),
            compat: CheckpointCompat {
                ast_schema_hash: "blake3:ast".to_owned(),
                type_rules_hash: "blake3:type".to_owned(),
                evaluator_hash: "blake3:eval".to_owned(),
                search_semantics_hash: "blake3:search".to_owned(),
            },
            objective: StepObjective {
                bar: Rational::new(5, 2),
                exact_clause_kappa: 3,
                bit_band: BitBand { min: 10, max: 10 },
            },
            accepted: AcceptedCandidate {
                candidate_hash: "blake3:candidate".to_owned(),
                canonical_hash: "blake3:canonical".to_owned(),
                bit_kappa: 10,
                clause_kappa: 3,
                nu: 5,
                rho: Rational::new(5, 3),
                overshoot: Rational::new(5, 6),
                shape_fingerprint: "0xshape".to_owned(),
                support_fingerprint: "0xsupport".to_owned(),
            },
            library_snapshot: LibrarySnapshot {
                window_depth: 2,
                entries: vec![LibrarySnapshotEntry {
                    step: 6,
                    candidate_hash: "blake3:step-6".to_owned(),
                    telescope: Telescope::reference(6),
                }],
            },
            near_misses: Vec::new(),
            stats: StepStats::default(),
        };
        fs::create_dir_all(path.parent().expect("checkpoint parent")).expect("create parent");
        fs::write(
            &path,
            format!(
                "{}\n",
                serde_json::to_string_pretty(&checkpoint).expect("serialize checkpoint")
            ),
        )
        .expect("write checkpoint");

        let round_trip = read_step_checkpoint(&path).expect("checkpoint should read");
        assert_eq!(round_trip, checkpoint);

        fs::remove_dir_all(root).ok();
    }
}
