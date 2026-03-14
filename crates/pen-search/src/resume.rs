use pen_store::manifest::{CheckpointCompat, FrontierManifestV1, ResumeCompatible};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentCompat {
    pub ast_schema_hash: String,
    pub type_rules_hash: String,
    pub evaluator_hash: String,
    pub search_semantics_hash: String,
    pub record_layout_id: String,
}

impl CurrentCompat {
    pub fn checkpoint_compat(&self) -> CheckpointCompat {
        CheckpointCompat {
            ast_schema_hash: self.ast_schema_hash.clone(),
            type_rules_hash: self.type_rules_hash.clone(),
            evaluator_hash: self.evaluator_hash.clone(),
            search_semantics_hash: self.search_semantics_hash.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResumeDecision {
    FrontierCheckpoint,
    StepCheckpoint,
    StepCheckpointReevaluate,
    MigrationRequired,
}

pub fn decide_resume(current: &CurrentCompat, frontier: &FrontierManifestV1) -> ResumeDecision {
    let resume = &frontier.resume_compatible;

    if same_frontier_compat(current, resume) {
        ResumeDecision::FrontierCheckpoint
    } else {
        decide_step_resume(
            &current.checkpoint_compat(),
            &checkpoint_compat_from_resume(&frontier.resume_compatible),
        )
    }
}

pub fn decide_step_resume(current: &CheckpointCompat, stored: &CheckpointCompat) -> ResumeDecision {
    if same_ast_type_eval_checkpoint(current, stored) {
        ResumeDecision::StepCheckpoint
    } else if same_ast_and_type_checkpoint(current, stored) {
        ResumeDecision::StepCheckpointReevaluate
    } else {
        ResumeDecision::MigrationRequired
    }
}

fn same_frontier_compat(current: &CurrentCompat, resume: &ResumeCompatible) -> bool {
    current.ast_schema_hash == resume.ast_schema_hash
        && current.type_rules_hash == resume.type_rules_hash
        && current.evaluator_hash == resume.evaluator_hash
        && current.search_semantics_hash == resume.search_semantics_hash
        && current.record_layout_id == resume.record_layout_id
}

fn same_ast_type_eval_checkpoint(current: &CheckpointCompat, stored: &CheckpointCompat) -> bool {
    current.ast_schema_hash == stored.ast_schema_hash
        && current.type_rules_hash == stored.type_rules_hash
        && current.evaluator_hash == stored.evaluator_hash
}

fn same_ast_and_type_checkpoint(current: &CheckpointCompat, stored: &CheckpointCompat) -> bool {
    current.ast_schema_hash == stored.ast_schema_hash
        && current.type_rules_hash == stored.type_rules_hash
}

pub fn checkpoint_compat_from_resume(value: &ResumeCompatible) -> CheckpointCompat {
    CheckpointCompat {
        ast_schema_hash: value.ast_schema_hash.clone(),
        type_rules_hash: value.type_rules_hash.clone(),
        evaluator_hash: value.evaluator_hash.clone(),
        search_semantics_hash: value.search_semantics_hash.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrentCompat, ResumeDecision, decide_resume, decide_step_resume};
    use pen_store::layout::{FRONTIER_RECORD_LAYOUT_ID, SCHEMA_VERSION_V1};
    use pen_store::manifest::{
        CheckpointCompat, FrontierCounts, FrontierFiles, FrontierManifestV1, FrontierScheduler,
        MemorySnapshot, ResumeCompatible,
    };

    fn current() -> CurrentCompat {
        CurrentCompat {
            ast_schema_hash: "blake3:ast".to_owned(),
            type_rules_hash: "blake3:type".to_owned(),
            evaluator_hash: "blake3:eval".to_owned(),
            search_semantics_hash: "blake3:search".to_owned(),
            record_layout_id: FRONTIER_RECORD_LAYOUT_ID.to_owned(),
        }
    }

    fn frontier_manifest(resume_compatible: ResumeCompatible) -> FrontierManifestV1 {
        FrontierManifestV1 {
            schema_version: SCHEMA_VERSION_V1,
            run_id: "run-1".to_owned(),
            step_index: 10,
            band_index: 4,
            frontier_epoch: 17,
            base_step_checkpoint: "../../../steps/step-09.cbor.zst".to_owned(),
            resume_compatible,
            counts: FrontierCounts::default(),
            files: FrontierFiles::default(),
            memory_snapshot: MemorySnapshot::default(),
            scheduler: FrontierScheduler::default(),
        }
    }

    fn matching_resume() -> ResumeCompatible {
        ResumeCompatible {
            ast_schema_hash: "blake3:ast".to_owned(),
            type_rules_hash: "blake3:type".to_owned(),
            evaluator_hash: "blake3:eval".to_owned(),
            search_semantics_hash: "blake3:search".to_owned(),
            record_layout_id: FRONTIER_RECORD_LAYOUT_ID.to_owned(),
        }
    }

    #[test]
    fn resume_policy_prefers_frontier_only_on_full_match() {
        let current = current();
        let frontier = frontier_manifest(matching_resume());

        assert_eq!(
            decide_resume(&current, &frontier),
            ResumeDecision::FrontierCheckpoint
        );
    }

    #[test]
    fn resume_policy_drops_frontier_when_search_changes() {
        let current = current();
        let frontier = frontier_manifest(ResumeCompatible {
            search_semantics_hash: "blake3:other-search".to_owned(),
            ..matching_resume()
        });

        assert_eq!(
            decide_resume(&current, &frontier),
            ResumeDecision::StepCheckpoint
        );
    }

    #[test]
    fn resume_policy_requires_reevaluation_when_evaluator_changes() {
        let current = current();
        let frontier = frontier_manifest(ResumeCompatible {
            ast_schema_hash: "blake3:ast".to_owned(),
            type_rules_hash: "blake3:type".to_owned(),
            evaluator_hash: "blake3:old-eval".to_owned(),
            search_semantics_hash: "blake3:search".to_owned(),
            record_layout_id: FRONTIER_RECORD_LAYOUT_ID.to_owned(),
        });

        assert_eq!(
            decide_resume(&current, &frontier),
            ResumeDecision::StepCheckpointReevaluate
        );
    }

    #[test]
    fn resume_policy_stops_on_ast_changes() {
        let current = current();
        let frontier = frontier_manifest(ResumeCompatible {
            ast_schema_hash: "blake3:old-ast".to_owned(),
            type_rules_hash: "blake3:type".to_owned(),
            evaluator_hash: "blake3:eval".to_owned(),
            search_semantics_hash: "blake3:search".to_owned(),
            record_layout_id: FRONTIER_RECORD_LAYOUT_ID.to_owned(),
        });

        assert_eq!(
            decide_resume(&current, &frontier),
            ResumeDecision::MigrationRequired
        );
    }

    #[test]
    fn checkpoint_resume_policy_uses_the_same_ast_type_eval_ladder() {
        let current = current().checkpoint_compat();
        let matching = CheckpointCompat {
            ast_schema_hash: "blake3:ast".to_owned(),
            type_rules_hash: "blake3:type".to_owned(),
            evaluator_hash: "blake3:eval".to_owned(),
            search_semantics_hash: "blake3:search".to_owned(),
        };

        assert_eq!(
            decide_step_resume(&current, &matching),
            ResumeDecision::StepCheckpoint
        );
        assert_eq!(
            decide_step_resume(
                &current,
                &CheckpointCompat {
                    evaluator_hash: "blake3:other-eval".to_owned(),
                    ..matching.clone()
                }
            ),
            ResumeDecision::StepCheckpointReevaluate
        );
        assert_eq!(
            decide_step_resume(
                &current,
                &CheckpointCompat {
                    ast_schema_hash: "blake3:other-ast".to_owned(),
                    ..matching
                }
            ),
            ResumeDecision::MigrationRequired
        );
    }
}
