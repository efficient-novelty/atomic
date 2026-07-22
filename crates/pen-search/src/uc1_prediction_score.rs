//! Read-only, post-BI-4 scoring of the frozen UC-1 predictions.
//!
//! This module never issues a branch, selects a winner, or changes a BI
//! artifact.  It consumes the sealed BI v2 JSON certificates, requires their
//! ordinary replay gates, and applies only comparison rules that are sound on
//! the serialized surface:
//!
//! * P1 compares exact-certified `(kappa, nu)` rows.
//! * P2 may pass by literal winner-telescope identity.  A syntactic difference
//!   is `Unknown` until a separately frozen fork-shadow relabeling exists.
//! * P3 may pass by literal live-scheme-id identity.  A differing id is
//!   `Unknown` until an exhaustive pairwise family-quotient relation is sealed.

use crate::branch_invariance::{BranchNuProvenanceDisposition, replay_branch_nu_provenance_token};
use crate::branch_invariance_program::{
    BI_BRANCH_CERTIFICATE_SCHEMA, BI_CONE_CERTIFICATE_SCHEMA, BI_CONE_V2_CERTIFICATE_NAME,
    BI_REGRESSION_CERTIFICATE_SCHEMA, BI_REGRESSION_V2_CERTIFICATE_NAME, BiBranchCertificate,
    BiConeCertificate, BiRegressionCertificate, replay_bi_branch_certificate,
    replay_bi_cone_certificate, replay_bi_regression_certificate,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const UC1_SCORE_SCHEMA: &str = "uc1-p1-p3-score-v1";
pub const UC1_SCORE_DATE: &str = "2026-07-22";
pub const UC1_SCORE_CERTIFICATE_NAME: &str = "UC1_P1_P3_SCORE_V1_CERTIFICATE.json";
pub const UC1_SCORE_REPORT_NAME: &str = "UC1_P1_P3_SCORE_V1_RESULT.md";

const HYPOTHESIS_BYTES: &[u8] = include_bytes!("../../../docs/univalent_collapse_hypothesis.md");
const SCORER_SOURCE_BYTES: &[u8] = include_bytes!("uc1_prediction_score.rs");
const BI_PROGRAM_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_program.rs");
const BI_CORE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Uc1PredictionStatus {
    Passed,
    Refuted,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1InputBinding {
    pub file_name: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
    pub parsed_schema: String,
    pub parsed_result_digest: String,
    pub replay_valid: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1LedgerRow {
    pub stage: u32,
    pub kappa: u16,
    pub nu: u32,
    pub winner_hash: String,
    pub provenance_derivation_hash: String,
    pub exact_certified: bool,
    pub provenance_replay_valid: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1P1BranchAudit {
    pub branch_root_hash: String,
    pub rows: Vec<Uc1LedgerRow>,
    pub exact_stage_5_through_15_coverage: bool,
    pub every_row_exact_certified_and_replayed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1P1PairAudit {
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub comparable_complete_vectors: bool,
    pub vectors_equal: bool,
    pub first_divergence_stage: Option<u32>,
    pub exact_context: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1P2StageAudit {
    pub stage: u32,
    pub left_winner_hash: Option<String>,
    pub right_winner_hash: Option<String>,
    pub left_telescope_hash: Option<String>,
    pub right_telescope_hash: Option<String>,
    pub exact_telescope_syntax_equal: bool,
    pub comparison_complete: bool,
    pub exact_identity_is_sufficient_without_relabeling: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1P2PairAudit {
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub stage_audits: Vec<Uc1P2StageAudit>,
    pub exact_stage_5_through_15_coverage: bool,
    pub all_winner_telescopes_exactly_identical: bool,
    pub first_nonidentical_or_missing_stage: Option<u32>,
    pub fork_shadow_relabeling_evidence_present: bool,
    pub nonidentity_is_not_treated_as_refutation: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1P3StageAudit {
    pub stage: u32,
    pub left_live_scheme_ids: Vec<String>,
    pub right_live_scheme_ids: Vec<String>,
    pub both_live_scheme_sets_nonempty: bool,
    pub exact_live_scheme_id_sets_equal: bool,
    pub exact_identity_is_certified_matching: bool,
    pub exhaustive_nonidentity_matching_evidence_present: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1P3PairAudit {
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub stage_audits: Vec<Uc1P3StageAudit>,
    pub exact_stage_5_through_15_coverage: bool,
    pub every_live_demand_exactly_identity_matched: bool,
    pub first_nonidentity_or_missing_stage: Option<u32>,
    pub matched_scheme_counts_were_not_used_as_membership_evidence: bool,
    pub nonidentity_is_not_treated_as_refutation: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1PredictionAudit {
    pub prediction: String,
    pub status: Uc1PredictionStatus,
    pub exact_basis: String,
    pub first_counterexample_stage: Option<u32>,
    pub missing_evidence: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1OutcomeDisposition {
    pub predictions_all_passed: bool,
    pub construction_tasks_u_t1_through_u_t4_authorized: bool,
    pub uc1c_healing_status: String,
    pub uc1d_full_collapse_status: String,
    pub registered_outcome_zone: Option<String>,
    pub exact_reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1PredictionScoreCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Uc1SourceBinding>,
    pub input_bindings: Vec<Uc1InputBinding>,
    pub scoring_scope: String,
    pub hypothesis_document_was_not_a_bi_input: bool,
    pub bi_artifacts_read_only: bool,
    pub no_bi_artifact_emitter_or_selector_called_by_scorer: bool,
    pub bi0_replay_valid_and_passed: bool,
    pub bi4_replay_valid: bool,
    pub all_four_branch_replays_valid: bool,
    pub cone_to_branch_digest_join_exact: bool,
    pub branch_count: usize,
    pub p1_branch_audits: Vec<Uc1P1BranchAudit>,
    pub p1_pair_audits: Vec<Uc1P1PairAudit>,
    pub p2_pair_audits: Vec<Uc1P2PairAudit>,
    pub p3_pair_audits: Vec<Uc1P3PairAudit>,
    pub p1: Uc1PredictionAudit,
    pub p2: Uc1PredictionAudit,
    pub p3: Uc1PredictionAudit,
    pub outcome: Uc1OutcomeDisposition,
    pub f_uc1_fired: bool,
    pub f_uc2_fired: bool,
    pub no_post_hoc_equivalence_or_relabeling_inferred: bool,
    pub unknown_is_never_coerced_to_pass_or_refutation: bool,
    pub mutation_falsifiers: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1Replay {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Error)]
pub enum Uc1ScoreError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("sealed BI v2 prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("create-new target already exists: {0}")]
    CreateNew(String),
    #[error("emitted replay failed: {0}")]
    EmittedReplay(String),
}

#[derive(Clone)]
struct LoadedInput<T> {
    file_name: String,
    bytes: Vec<u8>,
    value: T,
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(UC1_SCORE_SCHEMA, domain, value))
        .expect("UC-1 scoring evidence serializes");
    bytes_hash(&bytes)
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> Uc1SourceBinding {
    Uc1SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    }
}

fn source_bindings() -> Vec<Uc1SourceBinding> {
    vec![
        source_binding(
            "docs/univalent_collapse_hypothesis.md",
            "frozen_UC_1_hypothesis_and_P1_to_P3_predictions",
            HYPOTHESIS_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/uc1_prediction_score.rs",
            "read_only_tri_state_scoring_implementation",
            SCORER_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/branch_invariance_program.rs",
            "sealed_BI_v2_certificate_schemas_and_replay",
            BI_PROGRAM_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/branch_invariance.rs",
            "winner_telescope_demand_and_exact_nu_evidence_schemas",
            BI_CORE_SOURCE_BYTES,
        ),
    ]
}

fn certificate_digest(certificate: &Uc1PredictionScoreCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn audit_digest<T: Serialize>(domain: &str, value: &T) -> String {
    tagged_hash(domain, value)
}

fn digest_prefix(hash: &str) -> String {
    hash.strip_prefix("blake3:")
        .unwrap_or(hash)
        .chars()
        .take(8)
        .collect()
}

fn branch_certificate_name(root_hash: &str) -> String {
    format!("BI_BRANCH_V2_{}_CERTIFICATE.json", digest_prefix(root_hash))
}

fn read_json<T: for<'de> Deserialize<'de>>(
    directory: &Path,
    file_name: &str,
) -> Result<LoadedInput<T>, Uc1ScoreError> {
    let path = directory.join(file_name);
    let bytes = std::fs::read(&path)
        .map_err(|error| Uc1ScoreError::Io(format!("{}: {error}", path.display())))?;
    let value = serde_json::from_slice(&bytes)
        .map_err(|error| Uc1ScoreError::Json(format!("{}: {error}", path.display())))?;
    Ok(LoadedInput {
        file_name: file_name.to_owned(),
        bytes,
        value,
    })
}

fn input_binding(
    file_name: &str,
    role: &str,
    bytes: &[u8],
    schema: &str,
    result_digest: &str,
    replay_valid: bool,
) -> Uc1InputBinding {
    Uc1InputBinding {
        file_name: file_name.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
        parsed_schema: schema.to_owned(),
        parsed_result_digest: result_digest.to_owned(),
        replay_valid,
    }
}

fn exact_stage_range(stages: impl Iterator<Item = u32>) -> bool {
    stages.eq(5..=15)
}

fn issue_p1_branch_audit(branch: &BiBranchCertificate) -> Uc1P1BranchAudit {
    let mut rows = branch
        .numeric_ledger_stage5_onward
        .iter()
        .map(|row| {
            let disposition_exact = matches!(
                row.nu_provenance.disposition,
                BranchNuProvenanceDisposition::ExactCertified { .. }
            );
            let replay_valid = replay_branch_nu_provenance_token(&row.nu_provenance).is_empty()
                && row.nu_provenance.stage == row.stage
                && row.nu_provenance.candidate_hash == row.winner_hash
                && row.nu_provenance.diagnostic_nu == row.diagnostic_nu;
            Uc1LedgerRow {
                stage: row.stage,
                kappa: row.kappa,
                nu: row.diagnostic_nu,
                winner_hash: row.winner_hash.clone(),
                provenance_derivation_hash: row.nu_provenance.derivation_hash.clone(),
                exact_certified: disposition_exact,
                provenance_replay_valid: replay_valid,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| row.stage);
    let exact_stage_5_through_15_coverage = exact_stage_range(rows.iter().map(|row| row.stage));
    let every_row_exact_certified_and_replayed = branch.every_selected_winner_exact_nu_certified
        && rows
            .iter()
            .all(|row| row.exact_certified && row.provenance_replay_valid);
    let mut audit = Uc1P1BranchAudit {
        branch_root_hash: branch.branch_root_hash.clone(),
        rows,
        exact_stage_5_through_15_coverage,
        every_row_exact_certified_and_replayed,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = audit_digest("p1-branch-audit", &audit);
    audit
}

fn issue_p1_pair_audit(left: &Uc1P1BranchAudit, right: &Uc1P1BranchAudit) -> Uc1P1PairAudit {
    let comparable_complete_vectors = left.exact_stage_5_through_15_coverage
        && right.exact_stage_5_through_15_coverage
        && left.every_row_exact_certified_and_replayed
        && right.every_row_exact_certified_and_replayed;
    let left_by_stage = left
        .rows
        .iter()
        .map(|row| (row.stage, row))
        .collect::<BTreeMap<_, _>>();
    let right_by_stage = right
        .rows
        .iter()
        .map(|row| (row.stage, row))
        .collect::<BTreeMap<_, _>>();
    let first_divergence = (5..=15).find_map(|stage| {
        let left = left_by_stage.get(&stage)?;
        let right = right_by_stage.get(&stage)?;
        (left.exact_certified
            && left.provenance_replay_valid
            && right.exact_certified
            && right.provenance_replay_valid
            && (left.kappa != right.kappa || left.nu != right.nu))
            .then_some((*left, *right))
    });
    let vectors_equal = comparable_complete_vectors
        && first_divergence.is_none()
        && (5..=15).all(|stage| {
            let left = left_by_stage[&stage];
            let right = right_by_stage[&stage];
            left.kappa == right.kappa && left.nu == right.nu
        });
    let (first_divergence_stage, exact_context) = if let Some((left, right)) = first_divergence {
        (
            Some(left.stage.min(right.stage)),
            format!(
                "Stage {}: left (kappa,nu)=({},{}), right=({},{})",
                left.stage, left.kappa, left.nu, right.kappa, right.nu
            ),
        )
    } else if !comparable_complete_vectors {
        (
            None,
            "one or both exact-certified Stage-5-through-15 vectors are incomplete".to_owned(),
        )
    } else {
        (
            None,
            "all eleven exact-certified (kappa,nu) rows agree".to_owned(),
        )
    };
    let mut audit = Uc1P1PairAudit {
        left_branch_root: left.branch_root_hash.clone(),
        right_branch_root: right.branch_root_hash.clone(),
        comparable_complete_vectors,
        vectors_equal,
        first_divergence_stage,
        exact_context,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = audit_digest("p1-pair-audit", &audit);
    audit
}

fn winner_by_stage(branch: &BiBranchCertificate) -> BTreeMap<u32, (&str, &Telescope)> {
    branch
        .continuation
        .stages
        .iter()
        .filter_map(|stage| {
            stage.winner.as_ref().map(|winner| {
                (
                    stage.stage,
                    (winner.candidate_hash.as_str(), &winner.telescope),
                )
            })
        })
        .collect()
}

fn issue_p2_pair_audit(left: &BiBranchCertificate, right: &BiBranchCertificate) -> Uc1P2PairAudit {
    let left_winners = winner_by_stage(left);
    let right_winners = winner_by_stage(right);
    let stage_audits = (5..=15)
        .map(|stage| {
            let left_winner = left_winners.get(&stage);
            let right_winner = right_winners.get(&stage);
            let comparison_complete = left_winner.is_some() && right_winner.is_some();
            let exact_telescope_syntax_equal = matches!(
                (left_winner, right_winner),
                (Some((_, left)), Some((_, right))) if left == right
            );
            Uc1P2StageAudit {
                stage,
                left_winner_hash: left_winner.map(|(hash, _)| (*hash).to_owned()),
                right_winner_hash: right_winner.map(|(hash, _)| (*hash).to_owned()),
                left_telescope_hash: left_winner
                    .map(|(_, telescope)| audit_digest("winner-telescope", *telescope)),
                right_telescope_hash: right_winner
                    .map(|(_, telescope)| audit_digest("winner-telescope", *telescope)),
                exact_telescope_syntax_equal,
                comparison_complete,
                exact_identity_is_sufficient_without_relabeling: comparison_complete
                    && exact_telescope_syntax_equal,
            }
        })
        .collect::<Vec<_>>();
    let exact_stage_5_through_15_coverage = stage_audits.iter().all(|row| row.comparison_complete);
    let all_winner_telescopes_exactly_identical = exact_stage_5_through_15_coverage
        && stage_audits
            .iter()
            .all(|row| row.exact_telescope_syntax_equal);
    let first_nonidentical_or_missing_stage = stage_audits
        .iter()
        .find(|row| !row.comparison_complete || !row.exact_telescope_syntax_equal)
        .map(|row| row.stage);
    let mut audit = Uc1P2PairAudit {
        left_branch_root: left.branch_root_hash.clone(),
        right_branch_root: right.branch_root_hash.clone(),
        stage_audits,
        exact_stage_5_through_15_coverage,
        all_winner_telescopes_exactly_identical,
        first_nonidentical_or_missing_stage,
        fork_shadow_relabeling_evidence_present: false,
        nonidentity_is_not_treated_as_refutation: true,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = audit_digest("p2-pair-audit", &audit);
    audit
}

fn live_schemes_by_stage(branch: &BiBranchCertificate) -> BTreeMap<u32, Vec<String>> {
    branch
        .continuation
        .stages
        .iter()
        .map(|stage| {
            let mut ids = stage.demand.structural_scheme_ids.clone();
            ids.sort();
            ids.dedup();
            (stage.stage, ids)
        })
        .collect()
}

fn issue_p3_pair_audit(left: &BiBranchCertificate, right: &BiBranchCertificate) -> Uc1P3PairAudit {
    let left_schemes = live_schemes_by_stage(left);
    let right_schemes = live_schemes_by_stage(right);
    let stage_audits = (5..=15)
        .map(|stage| {
            let left_live_scheme_ids = left_schemes.get(&stage).cloned().unwrap_or_default();
            let right_live_scheme_ids = right_schemes.get(&stage).cloned().unwrap_or_default();
            let both_live_scheme_sets_nonempty =
                !left_live_scheme_ids.is_empty() && !right_live_scheme_ids.is_empty();
            let exact_live_scheme_id_sets_equal =
                both_live_scheme_sets_nonempty && left_live_scheme_ids == right_live_scheme_ids;
            Uc1P3StageAudit {
                stage,
                left_live_scheme_ids,
                right_live_scheme_ids,
                both_live_scheme_sets_nonempty,
                exact_live_scheme_id_sets_equal,
                exact_identity_is_certified_matching: exact_live_scheme_id_sets_equal,
                exhaustive_nonidentity_matching_evidence_present: false,
            }
        })
        .collect::<Vec<_>>();
    let exact_stage_5_through_15_coverage = (5..=15)
        .all(|stage| left_schemes.contains_key(&stage) && right_schemes.contains_key(&stage));
    let every_live_demand_exactly_identity_matched = exact_stage_5_through_15_coverage
        && stage_audits
            .iter()
            .all(|row| row.both_live_scheme_sets_nonempty && row.exact_live_scheme_id_sets_equal);
    let first_nonidentity_or_missing_stage = stage_audits
        .iter()
        .find(|row| !row.both_live_scheme_sets_nonempty || !row.exact_live_scheme_id_sets_equal)
        .map(|row| row.stage);
    let mut audit = Uc1P3PairAudit {
        left_branch_root: left.branch_root_hash.clone(),
        right_branch_root: right.branch_root_hash.clone(),
        stage_audits,
        exact_stage_5_through_15_coverage,
        every_live_demand_exactly_identity_matched,
        first_nonidentity_or_missing_stage,
        matched_scheme_counts_were_not_used_as_membership_evidence: true,
        nonidentity_is_not_treated_as_refutation: true,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = audit_digest("p3-pair-audit", &audit);
    audit
}

fn all_pairs<T, U>(rows: &[T], mut compare: impl FnMut(&T, &T) -> U) -> Vec<U> {
    let mut result = Vec::new();
    for left in 0..rows.len() {
        for right in (left + 1)..rows.len() {
            result.push(compare(&rows[left], &rows[right]));
        }
    }
    result
}

fn prediction_audit(
    prediction: &str,
    status: Uc1PredictionStatus,
    exact_basis: String,
    first_counterexample_stage: Option<u32>,
    missing_evidence: Vec<String>,
) -> Uc1PredictionAudit {
    let mut audit = Uc1PredictionAudit {
        prediction: prediction.to_owned(),
        status,
        exact_basis,
        first_counterexample_stage,
        missing_evidence,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = audit_digest("prediction-audit", &audit);
    audit
}

fn outcome_disposition(
    p1: &Uc1PredictionAudit,
    p2: &Uc1PredictionAudit,
    p3: &Uc1PredictionAudit,
) -> Uc1OutcomeDisposition {
    use Uc1PredictionStatus::{Passed, Refuted};
    let predictions_all_passed =
        matches!(p1.status, Passed) && matches!(p2.status, Passed) && matches!(p3.status, Passed);
    if matches!(p1.status, Refuted) {
        Uc1OutcomeDisposition {
            predictions_all_passed,
            construction_tasks_u_t1_through_u_t4_authorized: false,
            uc1c_healing_status: "refuted_by_F_UC1".to_owned(),
            uc1d_full_collapse_status: "refuted_by_F_UC1".to_owned(),
            registered_outcome_zone: Some("Z-REFUTED".to_owned()),
            exact_reason: "P1 contains an exact-certified per-stage ledger divergence".to_owned(),
        }
    } else if matches!(p1.status, Passed)
        && (matches!(p2.status, Refuted) || matches!(p3.status, Refuted))
    {
        Uc1OutcomeDisposition {
            predictions_all_passed,
            construction_tasks_u_t1_through_u_t4_authorized: false,
            uc1c_healing_status: "refuted_by_P2_or_P3".to_owned(),
            uc1d_full_collapse_status: "refuted_by_P2_or_P3".to_owned(),
            registered_outcome_zone: Some("Z-LEDGER".to_owned()),
            exact_reason: "P1 passed while a structural continuation prediction was refuted"
                .to_owned(),
        }
    } else if predictions_all_passed {
        Uc1OutcomeDisposition {
            predictions_all_passed,
            construction_tasks_u_t1_through_u_t4_authorized: true,
            uc1c_healing_status: "predictions_survived_construction_theorems_pending".to_owned(),
            uc1d_full_collapse_status: "not_yet_proved_U_T1_through_U_T4_pending".to_owned(),
            registered_outcome_zone: None,
            exact_reason: "P1-P3 pass, but Z-COLLAPSE and Z-HEAL require the registered U-T construction results"
                .to_owned(),
        }
    } else {
        Uc1OutcomeDisposition {
            predictions_all_passed,
            construction_tasks_u_t1_through_u_t4_authorized: false,
            uc1c_healing_status: "unresolved_due_to_unknown_prediction".to_owned(),
            uc1d_full_collapse_status: "unresolved_due_to_unknown_prediction".to_owned(),
            registered_outcome_zone: None,
            exact_reason:
                "the frozen outcome zones do not assign a zone while any prediction is Unknown"
                    .to_owned(),
        }
    }
}

fn hypothesis_file_gate(directory: &Path) -> Result<(), Uc1ScoreError> {
    let path = directory.join("univalent_collapse_hypothesis.md");
    let observed = std::fs::read(&path)
        .map_err(|error| Uc1ScoreError::Io(format!("{}: {error}", path.display())))?;
    if observed != HYPOTHESIS_BYTES {
        return Err(Uc1ScoreError::Prerequisite(format!(
            "{} differs from the scorer's frozen hypothesis binding",
            path.display()
        )));
    }
    Ok(())
}

/// Score P1-P3 solely from already-emitted BI v2 certificates.
///
/// Ordinary certificate replay is a prerequisite check.  This function never
/// emits or selects a BI result and never writes to `directory`.  The standard
/// BI replay functions may reconstruct their own evidence in memory.
pub fn issue_uc1_prediction_score_from_directory(
    directory: &Path,
) -> Result<Uc1PredictionScoreCertificate, Uc1ScoreError> {
    hypothesis_file_gate(directory)?;
    let regression: LoadedInput<BiRegressionCertificate> =
        read_json(directory, BI_REGRESSION_V2_CERTIFICATE_NAME)?;
    let cone: LoadedInput<BiConeCertificate> = read_json(directory, BI_CONE_V2_CERTIFICATE_NAME)?;

    if regression.value.schema != BI_REGRESSION_CERTIFICATE_SCHEMA
        || cone.value.schema != BI_CONE_CERTIFICATE_SCHEMA
    {
        return Err(Uc1ScoreError::Prerequisite(
            "BI0 or BI4 is not a v2 certificate".to_owned(),
        ));
    }
    let regression_replay = replay_bi_regression_certificate(&regression.value);
    let cone_replay = replay_bi_cone_certificate(&cone.value, &regression.value);
    if !regression_replay.valid || !regression.value.bi0_passed {
        return Err(Uc1ScoreError::Prerequisite(format!(
            "BI0 must replay and pass before UC-1 scoring: {:?}",
            regression_replay.errors
        )));
    }
    if !cone_replay.valid {
        return Err(Uc1ScoreError::Prerequisite(format!(
            "BI4 must replay before UC-1 scoring: {:?}",
            cone_replay.errors
        )));
    }
    if cone.value.branch_count != 4
        || cone.value.branches.len() != 4
        || !cone
            .value
            .all_four_branch_certificates_replayed_before_comparison
        || !cone.value.bi0_passed
        || cone.value.bi0_certificate_digest != regression.value.result_digest
    {
        return Err(Uc1ScoreError::Prerequisite(
            "BI4 does not join a passing BI0 and exactly four sealed branches".to_owned(),
        ));
    }

    let hypothesis_hash = bytes_hash(HYPOTHESIS_BYTES);
    let is_hypothesis_binding = |path: &str, blake3: &str| {
        path.contains("univalent_collapse_hypothesis") || blake3 == hypothesis_hash
    };
    let hypothesis_document_was_not_a_bi_input = regression
        .value
        .source_bindings
        .iter()
        .chain(cone.value.source_bindings.iter())
        .all(|binding| !is_hypothesis_binding(&binding.path, &binding.blake3));
    if !hypothesis_document_was_not_a_bi_input {
        return Err(Uc1ScoreError::Prerequisite(
            "UC-1 hypothesis leaked into a BI source binding".to_owned(),
        ));
    }

    let mut input_bindings = vec![
        input_binding(
            &regression.file_name,
            "sealed_BI0_v2_gate",
            &regression.bytes,
            &regression.value.schema,
            &regression.value.result_digest,
            regression_replay.valid,
        ),
        input_binding(
            &cone.file_name,
            "sealed_BI4_v2_cone",
            &cone.bytes,
            &cone.value.schema,
            &cone.value.result_digest,
            cone_replay.valid,
        ),
    ];
    let mut branches = Vec::new();
    let mut branch_roots = BTreeSet::new();
    for summary in &cone.value.branches {
        let file_name = branch_certificate_name(&summary.branch_root_hash);
        let branch: LoadedInput<BiBranchCertificate> = read_json(directory, &file_name)?;
        let replay = replay_bi_branch_certificate(&branch.value);
        if branch.value.schema != BI_BRANCH_CERTIFICATE_SCHEMA
            || !replay.valid
            || branch.value.branch_root_hash != summary.branch_root_hash
            || branch.value.result_digest != summary.branch_certificate_digest
            || !branch_roots.insert(branch.value.branch_root_hash.clone())
        {
            return Err(Uc1ScoreError::Prerequisite(format!(
                "branch {} failed schema/replay/cone/dedup join: {:?}",
                summary.branch_root_hash, replay.errors
            )));
        }
        if branch
            .value
            .source_bindings
            .iter()
            .any(|binding| is_hypothesis_binding(&binding.path, &binding.blake3))
        {
            return Err(Uc1ScoreError::Prerequisite(format!(
                "UC-1 hypothesis leaked into branch {} source bindings",
                branch.value.branch_root_hash
            )));
        }
        input_bindings.push(input_binding(
            &branch.file_name,
            "sealed_BI1_BI2_branch",
            &branch.bytes,
            &branch.value.schema,
            &branch.value.result_digest,
            replay.valid,
        ));
        branches.push(branch.value);
    }
    branches.sort_by(|left, right| left.branch_root_hash.cmp(&right.branch_root_hash));
    input_bindings.sort_by(|left, right| left.file_name.cmp(&right.file_name));

    let p1_branch_audits = branches
        .iter()
        .map(issue_p1_branch_audit)
        .collect::<Vec<_>>();
    let p1_pair_audits = all_pairs(&p1_branch_audits, issue_p1_pair_audit);
    let p2_pair_audits = all_pairs(&branches, issue_p2_pair_audit);
    let p3_pair_audits = all_pairs(&branches, issue_p3_pair_audit);

    let p1_first_divergence = p1_pair_audits
        .iter()
        .filter_map(|row| row.first_divergence_stage)
        .min();
    let p1_all_comparable = p1_pair_audits
        .iter()
        .all(|row| row.comparable_complete_vectors);
    let p1_all_equal = p1_pair_audits.iter().all(|row| row.vectors_equal);
    let p1 = if p1_first_divergence.is_some() {
        prediction_audit(
            "P1_ledger_identity",
            Uc1PredictionStatus::Refuted,
            "an exact-certified (kappa,nu) divergence is a direct F-UC1 witness".to_owned(),
            p1_first_divergence,
            Vec::new(),
        )
    } else if p1_all_comparable && p1_all_equal {
        prediction_audit(
            "P1_ledger_identity",
            Uc1PredictionStatus::Passed,
            "all six pairwise comparisons of exact-certified Stage-5-through-15 (kappa,nu) vectors are equal"
                .to_owned(),
            None,
            Vec::new(),
        )
    } else {
        prediction_audit(
            "P1_ledger_identity",
            Uc1PredictionStatus::Unknown,
            "no exact divergence was found, but at least one required exact-certified row is absent"
                .to_owned(),
            None,
            vec!["complete_exact_certified_stage_5_through_15_vector".to_owned()],
        )
    };

    let p2_all_identity = p2_pair_audits
        .iter()
        .all(|row| row.all_winner_telescopes_exactly_identical);
    let p2_first_unknown = p2_pair_audits
        .iter()
        .filter_map(|row| row.first_nonidentical_or_missing_stage)
        .min();
    let p2 = if p2_all_identity {
        prediction_audit(
            "P2_winner_identity_modulo_fork_shadow",
            Uc1PredictionStatus::Passed,
            "all Stage-5-through-15 winner telescopes are literally identical; the identity relabeling is therefore sufficient"
                .to_owned(),
            None,
            Vec::new(),
        )
    } else {
        prediction_audit(
            "P2_winner_identity_modulo_fork_shadow",
            Uc1PredictionStatus::Unknown,
            "literal identity did not settle every row; syntactic nonidentity is not a refutation of equality modulo fork shadow"
                .to_owned(),
            None,
            vec![format!(
                "typed_Stage4_reference_origin_map_and_frozen_fork_shadow_relabeling_from_stage_{}",
                p2_first_unknown.unwrap_or(5)
            )],
        )
    };

    let p3_all_identity = p3_pair_audits
        .iter()
        .all(|row| row.every_live_demand_exactly_identity_matched);
    let p3_first_unknown = p3_pair_audits
        .iter()
        .filter_map(|row| row.first_nonidentity_or_missing_stage)
        .min();
    let p3 = if p3_all_identity {
        prediction_audit(
            "P3_margin_confinement",
            Uc1PredictionStatus::Passed,
            "every pairwise live-demand scheme-id set is literally identical at Stages 5 through 15"
                .to_owned(),
            None,
            Vec::new(),
        )
    } else {
        prediction_audit(
            "P3_margin_confinement",
            Uc1PredictionStatus::Unknown,
            "literal scheme identity did not settle every live demand; aggregate matched counts do not prove membership in the matched subset"
                .to_owned(),
            None,
            vec![format!(
                "exhaustive_pairwise_scheme_equivalence_edges_and_live_matching_from_stage_{}",
                p3_first_unknown.unwrap_or(5)
            )],
        )
    };
    let outcome = outcome_disposition(&p1, &p2, &p3);
    let f_uc1_fired = matches!(p1.status, Uc1PredictionStatus::Refuted);
    let f_uc2_fired = matches!(p3.status, Uc1PredictionStatus::Refuted);
    let cone_to_branch_digest_join_exact = branches.iter().all(|branch| {
        cone.value.branches.iter().any(|summary| {
            summary.branch_root_hash == branch.branch_root_hash
                && summary.branch_certificate_digest == branch.result_digest
        })
    });
    let mut certificate = Uc1PredictionScoreCertificate {
        schema: UC1_SCORE_SCHEMA.to_owned(),
        date: UC1_SCORE_DATE.to_owned(),
        source_bindings: source_bindings(),
        input_bindings,
        scoring_scope: "sealed_BI_v2_certificates_after_BI4_only".to_owned(),
        hypothesis_document_was_not_a_bi_input,
        bi_artifacts_read_only: true,
        no_bi_artifact_emitter_or_selector_called_by_scorer: true,
        bi0_replay_valid_and_passed: regression_replay.valid && regression.value.bi0_passed,
        bi4_replay_valid: cone_replay.valid,
        all_four_branch_replays_valid: true,
        cone_to_branch_digest_join_exact,
        branch_count: branches.len(),
        p1_branch_audits,
        p1_pair_audits,
        p2_pair_audits,
        p3_pair_audits,
        p1,
        p2,
        p3,
        outcome,
        f_uc1_fired,
        f_uc2_fired,
        no_post_hoc_equivalence_or_relabeling_inferred: true,
        unknown_is_never_coerced_to_pass_or_refutation: true,
        mutation_falsifiers: vec![
            "flip_any_P1_P2_or_P3_status_basis_or_missing_evidence_then_replay_must_fail"
                .to_owned(),
            "flip_any_ledger_telescope_or_live_scheme_audit_then_replay_must_fail".to_owned(),
            "flip_any_BI_input_byte_binding_result_digest_or_replay_gate_then_replay_must_fail"
                .to_owned(),
            "replace_Unknown_with_Passed_or_Refuted_without_new_evidence_then_replay_must_fail"
                .to_owned(),
            "assign_Z_COLLAPSE_or_Z_HEAL_before_U_T_constructions_then_replay_must_fail".to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn replay_uc1_prediction_score_certificate(
    directory: &Path,
    certificate: &Uc1PredictionScoreCertificate,
) -> Uc1Replay {
    let mut errors = Vec::new();
    if certificate.schema != UC1_SCORE_SCHEMA {
        errors.push("UC-1 score schema mismatch".to_owned());
    }
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("UC-1 score digest mismatch".to_owned());
    }
    match issue_uc1_prediction_score_from_directory(directory) {
        Ok(expected) if &expected == certificate => {}
        Ok(_) => errors.push("UC-1 score differs from read-only reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    Uc1Replay {
        valid: errors.is_empty(),
        errors,
    }
}

pub fn replay_uc1_prediction_score_json(directory: &Path, bytes: &[u8]) -> Uc1Replay {
    match serde_json::from_slice::<Uc1PredictionScoreCertificate>(bytes) {
        Ok(certificate) => replay_uc1_prediction_score_certificate(directory, &certificate),
        Err(error) => Uc1Replay {
            valid: false,
            errors: vec![error.to_string()],
        },
    }
}

pub fn replay_uc1_prediction_score_directory(directory: &Path) -> Uc1Replay {
    let path = directory.join(UC1_SCORE_CERTIFICATE_NAME);
    match std::fs::read(&path) {
        Ok(bytes) => replay_uc1_prediction_score_json(directory, &bytes),
        Err(error) => Uc1Replay {
            valid: false,
            errors: vec![format!("{}: {error}", path.display())],
        },
    }
}

pub fn render_uc1_prediction_score_markdown(certificate: &Uc1PredictionScoreCertificate) -> String {
    let mut out = String::new();
    out.push_str("# UC-1 P1-P3 mechanical score\n\n");
    out.push_str(&format!(
        "**Date:** {}. **Certificate:** `{}`. **Scope:** sealed BI v2 artifacts after BI-4.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("| Prediction | Verdict | Exact basis |\n");
    out.push_str("|---|---|---|\n");
    for row in [&certificate.p1, &certificate.p2, &certificate.p3] {
        out.push_str(&format!(
            "| `{}` | `{:?}` | {} |\n",
            row.prediction, row.status, row.exact_basis
        ));
    }
    out.push_str("\nP2 uses literal winner-telescope identity only. P3 uses literal live-scheme-id identity only. A non-identical serialized value is `Unknown`, not a refutation, because the frozen BI v2 surface carries neither a typed fork-shadow relabeling nor exhaustive per-stage scheme-equivalence edges. Aggregate G1 matched counts were not used as membership evidence.\n\n");
    for row in [&certificate.p2, &certificate.p3] {
        if !row.missing_evidence.is_empty() {
            out.push_str(&format!(
                "`{}` missing evidence: `{}`.\n\n",
                row.prediction,
                row.missing_evidence.join("`, `")
            ));
        }
    }
    out.push_str(&format!(
        "Registered outcome zone: `{}`. U-T1 through U-T4 authorized: **{}**. {}\n\n",
        certificate
            .outcome
            .registered_outcome_zone
            .as_deref()
            .unwrap_or("none"),
        certificate
            .outcome
            .construction_tasks_u_t1_through_u_t4_authorized,
        certificate.outcome.exact_reason,
    ));
    out.push_str("The scorer wrote no BI input and invoked no BI artifact emitter or selector. Standard certificate replay, including its in-memory evidence reconstruction, was required before scoring.\n");
    out
}

fn write_create_new(path: &Path, bytes: &[u8]) -> Result<(), Uc1ScoreError> {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Uc1ScoreError::CreateNew(format!("{}: {error}", path.display())))?;
    output
        .write_all(bytes)
        .map_err(|error| Uc1ScoreError::Io(format!("{}: {error}", path.display())))
}

pub fn emit_uc1_prediction_score_create_new(
    directory: &Path,
) -> Result<Uc1PredictionScoreCertificate, Uc1ScoreError> {
    let certificate = issue_uc1_prediction_score_from_directory(directory)?;
    let replay = replay_uc1_prediction_score_certificate(directory, &certificate);
    if !replay.valid {
        return Err(Uc1ScoreError::EmittedReplay(format!("{:?}", replay.errors)));
    }
    let certificate_path = directory.join(UC1_SCORE_CERTIFICATE_NAME);
    let report_path = directory.join(UC1_SCORE_REPORT_NAME);
    for path in [&certificate_path, &report_path] {
        if path.exists() {
            return Err(Uc1ScoreError::CreateNew(path.display().to_string()));
        }
    }
    let mut json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Uc1ScoreError::Json(error.to_string()))?;
    json.push(b'\n');
    let mut report = render_uc1_prediction_score_markdown(&certificate).into_bytes();
    report.push(b'\n');
    write_create_new(&certificate_path, &json)?;
    if let Err(error) = write_create_new(&report_path, &report) {
        return Err(error);
    }
    Ok(certificate)
}

pub fn uc1_score_paths(directory: &Path) -> [PathBuf; 2] {
    [
        directory.join(UC1_SCORE_CERTIFICATE_NAME),
        directory.join(UC1_SCORE_REPORT_NAME),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_versioned_and_distinct() {
        let paths = uc1_score_paths(Path::new("docs"));
        assert!(paths[0].ends_with(UC1_SCORE_CERTIFICATE_NAME));
        assert!(paths[1].ends_with(UC1_SCORE_REPORT_NAME));
        assert_ne!(paths[0], paths[1]);
    }

    #[test]
    fn exact_stage_range_rejects_holes_and_extensions() {
        assert!(exact_stage_range((5..=15).into_iter()));
        assert!(!exact_stage_range((5..=14).into_iter()));
        assert!(!exact_stage_range((4..=15).into_iter()));
    }

    #[test]
    fn outcome_with_unknown_has_no_registered_zone() {
        let passed = prediction_audit(
            "P1",
            Uc1PredictionStatus::Passed,
            "test".to_owned(),
            None,
            Vec::new(),
        );
        let unknown = prediction_audit(
            "P2",
            Uc1PredictionStatus::Unknown,
            "test".to_owned(),
            None,
            vec!["evidence".to_owned()],
        );
        let outcome = outcome_disposition(&passed, &unknown, &unknown);
        assert!(outcome.registered_outcome_zone.is_none());
        assert!(!outcome.construction_tasks_u_t1_through_u_t4_authorized);
    }

    /// This is intentionally ignored until the BI v2 create-new artifacts
    /// exist.  It copies those read-only inputs to a unique temporary
    /// directory, exercises UC-1 create-new and replay there, and checks a
    /// self-consistent verdict mutation against independent reissuance.
    #[test]
    #[ignore = "requires sealed BI v2 artifacts in docs"]
    fn sealed_v2_create_new_replays_and_mutated_verdict_fails() {
        struct TempDirectory(PathBuf);
        impl Drop for TempDirectory {
            fn drop(&mut self) {
                let is_expected = self
                    .0
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("atomic-uc1-score-test-"));
                if is_expected {
                    let _ = std::fs::remove_dir_all(&self.0);
                }
            }
        }

        let source = Path::new("docs");
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let directory = TempDirectory(std::env::temp_dir().join(format!(
            "atomic-uc1-score-test-{}-{nonce}",
            std::process::id()
        )));
        std::fs::create_dir(&directory.0).expect("unique temporary directory");

        for file_name in [
            "univalent_collapse_hypothesis.md",
            BI_REGRESSION_V2_CERTIFICATE_NAME,
            BI_CONE_V2_CERTIFICATE_NAME,
        ] {
            std::fs::copy(source.join(file_name), directory.0.join(file_name))
                .expect("copy sealed input");
        }
        let cone_bytes =
            std::fs::read(source.join(BI_CONE_V2_CERTIFICATE_NAME)).expect("sealed cone bytes");
        let cone: BiConeCertificate = serde_json::from_slice(&cone_bytes).expect("sealed cone");
        for summary in &cone.branches {
            let file_name = branch_certificate_name(&summary.branch_root_hash);
            std::fs::copy(source.join(&file_name), directory.0.join(&file_name))
                .expect("copy sealed branch input");
        }

        let certificate =
            emit_uc1_prediction_score_create_new(&directory.0).expect("UC-1 create-new score");
        assert!(replay_uc1_prediction_score_directory(&directory.0).valid);

        let mut mutation = certificate;
        mutation.p1.status = match mutation.p1.status {
            Uc1PredictionStatus::Passed => Uc1PredictionStatus::Unknown,
            _ => Uc1PredictionStatus::Passed,
        };
        mutation.p1.derivation_hash = audit_digest("prediction-audit", &mutation.p1);
        mutation.result_digest = certificate_digest(&mutation);
        assert!(!replay_uc1_prediction_score_certificate(&directory.0, &mutation).valid);
    }
}
