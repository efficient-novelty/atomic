//! Create-new orchestration and replay certificates for BI-0 through BI-4.
//!
//! The isolation boundary in this module is intentional.  A branch certificate
//! is issued using only the adopted cone, the branch-parametric continuation,
//! and the branch-parametric E-5 finale. The sealed v2 artifact is read once
//! before issuance solely to identify the enacted Stage-4 root, as authorized
//! by BI-0. Historical outcome comparators (the remaining v2/v3 rows,
//! full-history totals, and enacted E-5 result) enter only the separately typed
//! BI-0 comparator after that root's branch certificate has replayed.
//! Cross-branch comparisons enter only BI-4, after all four branch
//! certificates have replayed.

use crate::branch_invariance::{
    BranchContinuation, BranchContinuationLimits, BranchContinuationOutcome, BranchInvarianceError,
    BranchNuProvenanceDisposition, BranchNuProvenanceToken, CertifiedStage4BranchCone,
    execute_branch_continuation, issue_certified_stage4_branch_cone, replay_branch_continuation,
    replay_branch_nu_provenance_token, replay_certified_stage4_branch_cone,
};
use crate::branch_invariance_finale::{
    BranchFinaleAudit, execute_certified_branch_finale, replay_certified_branch_finale,
};
use crate::chronological_slot_map::{
    CHRONOLOGICAL_SLOT_MAP_CERTIFICATE_SCHEMA, ChronologicalSlotMapCertificate,
    ChronologicalSlotMapReplay, replay_chronological_slot_map_certificate,
};
use crate::chronological_slot_map_v3::{
    CHRONOLOGICAL_SLOT_MAP_V3_SCHEMA, ChronologicalSlotMapAuditV3,
    replay_chronological_slot_map_audit_v3,
};
use crate::e5_future_hole_finale_v2::{
    E5_FUTURE_HOLE_FINALE_V2_SCHEMA, E5FutureHoleFinaleV2Certificate,
    replay_e5_future_hole_finale_v2_certificate,
};
use crate::phase5b_history_certification::{
    PHASE5B_HISTORY_CERT_SCHEMA, Phase5bHistoryCertificate,
};
use crate::phase5b_reselection_v2::{PHASE5B_RESELECTION_BURN_SCHEMA, Phase5bReselectionBurn};
use crate::phase5b_reselection_v3::{
    PHASE5B_RESELECTION_V3_BURN_SCHEMA, Phase5bReselectionV3Burn, Phase5bReselectionV3Program,
    replay_phase5b_reselection_v3_burn,
};
use crate::r_t2_future_hole_confluence_v2::{
    R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA, Rt2FutureHoleConfluenceV2Certificate,
    Rt2FutureHoleConfluenceV2Outcome,
};
use crate::t_bi_nu1_regression::{
    T_BI_NU1_REGRESSION_SCHEMA, TBiNu1RegressionCertificate, TBiNu1RegressionReplay,
    replay_t_bi_nu1_regression_certificate,
};
use crate::t_bi_nu1_regression_v3::{
    T_BI_NU1_REGRESSION_V3_SCHEMA, TBiNu1RegressionV3Certificate,
    replay_t_bi_nu1_regression_v3_certificate,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const BI_PROGRAM_SCHEMA: &str = "branch-invariance-program-v2";
pub const BI_BRANCH_CERTIFICATE_SCHEMA: &str = "branch-invariance-branch-certificate-v2";
pub const BI_REGRESSION_CERTIFICATE_SCHEMA: &str = "branch-invariance-regression-certificate-v2";
pub const BI_CONE_CERTIFICATE_SCHEMA: &str = "branch-invariance-cone-certificate-v2";
pub const BI_PROGRAM_DATE: &str = "2026-07-22";
pub const BI_MAX_INSPECTED_STAGE: u32 = 64;
pub const BI_MAX_ENUMERATED_CANDIDATES_PER_STAGE: usize = 10_000_000;
pub const BI_REGRESSION_V2_CERTIFICATE_NAME: &str = "BI_REGRESSION_V2_CERTIFICATE.json";
pub const BI_REGRESSION_V2_REPORT_NAME: &str = "BI_REGRESSION_V2_RESULT.md";
pub const BI_CONE_V2_CERTIFICATE_NAME: &str = "BI_CONE_V2_CERTIFICATE.json";
pub const BI_CONE_V2_REPORT_NAME: &str = "BI_CONE_V2_RESULT.md";

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/branch_invariance_program_plan.md");
const R_T3_BYTES: &[u8] = include_bytes!("../../../docs/r_t3_stage4_adjudication.md");
const BI0_PREREQUISITES_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/bi0_prerequisites_adjudication.md");
const BI0_PREREQUISITES_V1_AUDIT_BURN_BYTES: &[u8] =
    include_bytes!("../../../docs/bi0_prerequisites_v1_audit_burn.md");
const CHRONOLOGICAL_SLOT_MAP_V3_BYTES: &[u8] =
    include_bytes!("../../../docs/chronological_interface_slot_map_v3.json");
const T_BI_NU1_V3_BYTES: &[u8] =
    include_bytes!("../../../docs/t_bi_nu1_act_local_provenance_v3.json");
const CHRONOLOGICAL_SLOT_MAP_BYTES: &[u8] =
    include_bytes!("../../../docs/chronological_interface_slot_map_v1.json");
const T_BI_NU1_BYTES: &[u8] = include_bytes!("../../../docs/t_bi_nu1_act_local_provenance_v1.json");
const R_T2_BYTES: &[u8] =
    include_bytes!("../../../docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json");
const V2_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v2.json");
const V3_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v3.json");
const V3_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v3.json");
const E5_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_future_hole_finale_v2_dependent_context.json");
const FULL_HISTORY_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const CORE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance.rs");
const FINALE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_finale.rs");
const ACT_LOCAL_PROVENANCE_SOURCE_BYTES: &[u8] = include_bytes!("act_local_provenance.rs");
const T_BI_NU1_SOURCE_BYTES: &[u8] = include_bytes!("t_bi_nu1_regression.rs");
const CHRONOLOGICAL_SLOT_MAP_SOURCE_BYTES: &[u8] = include_bytes!("chronological_slot_map.rs");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const A3_EXHAUSTIVENESS_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/a3_rule_inventory_exhaustiveness.rs");
const FUTURE_V2_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/future_hole_hypothesis_v2.rs");
const TRANSPORT_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_program.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiReplay {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiObligationProfileRow {
    pub stage: u32,
    pub required_packages: Vec<String>,
    pub structural_constructors: Vec<String>,
    pub live_obligation_count: usize,
    pub total_typed_discharge: bool,
    pub debt_free: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiNumericLedgerRow {
    pub stage: u32,
    pub winner_hash: String,
    pub kappa: u16,
    pub diagnostic_nu: u32,
    pub nu_provenance: BranchNuProvenanceToken,
    pub diagnostic_bar: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiBranchCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<BiSourceBinding>,
    pub cone_digest: String,
    pub branch_root_hash: String,
    pub branch_digest_prefix: String,
    pub r_t1_class_id: String,
    pub economy_probe_class_key: String,
    pub continuation: BranchContinuation,
    pub obligation_profile: Vec<BiObligationProfileRow>,
    pub numeric_ledger_stage5_onward: Vec<BiNumericLedgerRow>,
    pub complete_sum_kappa: u32,
    pub complete_diagnostic_sum_nu: u32,
    pub finale: Option<BranchFinaleAudit>,
    pub finale_not_run_reason: Option<String>,
    pub candidate_assessment_count: usize,
    pub semantic_evidence_gap_count: usize,
    pub exact_nu_provenance_failure_count: usize,
    pub every_selected_winner_exact_nu_certified: bool,
    pub complete_ledger_exact_nu_provenance_count: usize,
    pub every_complete_ledger_entry_exact_nu_certified: bool,
    pub complete_enumerated_cone_semantic_evidence_closed: bool,
    pub one_live_demand_per_reached_guarded_stage: bool,
    pub every_reached_live_demand_totally_discharged: bool,
    pub local_debt_free_halt: bool,
    pub local_halt_stage: Option<u32>,
    pub local_successor_stage: Option<u32>,
    pub local_semantic_successor_o_empty: bool,
    pub local_g4_debt_free_halt_at_15: bool,
    pub local_semantic_o16_empty: bool,
    pub local_f1_excluded: bool,
    pub local_e5_class_complete: bool,
    pub enacted_reference_artifacts_consumed_by_branch_issuer: bool,
    pub enacted_outcomes_consumed_for_acceptance_selection_or_steering: bool,
    pub claim_scope_before_bi4: String,
    pub mutation_falsifiers: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiE5RegressionProjection {
    pub historical_window_count: usize,
    pub final_a3_inventory_count: usize,
    pub final_unary_count: usize,
    pub final_direct_chronological_count: usize,
    pub final_pointwise_chronological_count: usize,
    pub final_higher_count: usize,
    pub final_structural_count: usize,
    pub structural_registration_count: usize,
    pub structural_realization_count: usize,
    pub exact_d_partition: bool,
    pub derivable_instance_count: usize,
    pub underdetermined_instance_count: usize,
    pub stage3_wrinkle_reproduced: bool,
    pub focus_projection_reproduces_ladder: bool,
    pub semantic_o16_empty: bool,
    pub f1_executed: bool,
    pub f1_excluded: bool,
    pub theorem12_full_instance_granularity_proved: bool,
    pub e5_class_complete: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiRegressionCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<BiSourceBinding>,
    pub sealed_branch_certificate_digest: String,
    pub enacted_branch_root_hash: String,
    pub enacted_identified_by_sealed_stage4_digest_only: bool,
    pub only_historical_read_by_branch_issuer_was_stage4_identity: bool,
    pub enacted_branch_replayed_before_bi0_outcome_comparisons: bool,
    pub non_enacted_branches_issued_before_bi0: usize,
    pub bi0_is_hard_gate_before_bi1: bool,
    pub prerequisites_adjudication_digest: String,
    pub prerequisites_adoption_replayed: bool,
    pub prerequisites_replayed_before_enacted_branch_issuance: bool,
    pub chronological_slot_map_schema: String,
    pub chronological_slot_map_certificate_digest: String,
    pub chronological_slot_map_replay_valid: bool,
    pub chronological_slot_map_replay_errors: Vec<String>,
    pub chronological_slot_map_f_sm1_passed: bool,
    pub chronological_slot_map_f_sm2_passed: bool,
    pub chronological_slot_map_every_declaration_zero_charge: bool,
    pub chronological_slot_map_former_nine_exact_and_derived: bool,
    pub chronological_slot_map_stage16_89_id_surface_exact: bool,
    pub chronological_slot_map_non_enacted_branch_count: usize,
    pub t_bi_nu1_schema: String,
    pub t_bi_nu1_certificate_digest: String,
    pub t_bi_nu1_replay_valid: bool,
    pub t_bi_nu1_replay_errors: Vec<String>,
    pub t_bi_nu1_all_fifteen_acts_issued: bool,
    pub t_bi_nu1_all_fifteen_packages_authoritative: bool,
    pub t_bi_nu1_package_count: usize,
    pub t_bi_nu1_authoritative_token_hashes_globally_unique: bool,
    pub t_bi_nu1_all_thirteen_structural_filler_classes_covered: bool,
    pub t_bi_nu1_f_al1_passed: bool,
    pub t_bi_nu1_non_enacted_branch_work_executed: bool,
    pub enacted_complete_ledger_entry_count: usize,
    pub enacted_complete_ledger_exact_provenance_count: usize,
    pub enacted_complete_ledger_stage_surface_exact: bool,
    pub t_bi_nu1_authoritative_token_count: usize,
    pub enacted_complete_ledger_authoritative_token_count: usize,
    pub enacted_complete_ledger_authoritative_token_count_exact: bool,
    pub enacted_complete_ledger_provenance_join_exact: bool,
    pub v2_archive_digest_valid: bool,
    pub v3_archive_digest_valid: bool,
    pub e5_archive_digest_valid: bool,
    pub enacted_full_history_archive_digest: String,
    pub enacted_full_history_archive_digest_valid: bool,
    pub enacted_archived_candidate_hash_total_join_valid: bool,
    pub enacted_archived_certified_vector_numerically_matched_post_seal: bool,
    pub enacted_winner_nu_provenance_exact: bool,
    pub enacted_nu_provenance_gap_count: usize,
    pub v3_live_replay_valid: bool,
    pub v3_live_replay_errors: Vec<String>,
    pub e5_live_replay_valid: bool,
    pub e5_live_replay_errors: Vec<String>,
    pub frozen_archive_self_digest_comparators_are_bi0_regression_references: bool,
    pub live_replay_diagnostics_are_not_bi0_acceptance_inputs: bool,
    pub expected_complete_vector: Vec<(u32, u32, u32)>,
    pub observed_complete_vector: Vec<(u32, u32, u32)>,
    pub complete_vector_exact: bool,
    pub expected_winner_hashes: Vec<(u32, String)>,
    pub observed_winner_hashes: Vec<(u32, String)>,
    pub winner_hashes_exact: bool,
    pub expected_v3_discharger_census: Vec<(u32, usize)>,
    pub observed_v3_discharger_census: Vec<(u32, usize)>,
    pub v3_discharger_census_exact: bool,
    pub expected_e5_projection: BiE5RegressionProjection,
    pub observed_e5_projection: BiE5RegressionProjection,
    pub e5_projection_exact: bool,
    pub e5_membership_instance_ids_exact: bool,
    pub enacted_finale_inventory_scope: String,
    pub enacted_finale_absolute_semantic_exhaustiveness_claimed: bool,
    pub enacted_finale_e5_class_complete: bool,
    pub enacted_finale_expressivity_gaps: Vec<String>,
    pub enacted_finale_final_a3_inventory_count: usize,
    pub enacted_finale_derivable_instance_count: usize,
    pub enacted_finale_underdetermined_instance_ids: Vec<String>,
    pub enacted_finale_issuer_gap_instance_ids: Vec<String>,
    pub enacted_finale_realization_gap_ids: Vec<String>,
    pub bi0_passed: bool,
    pub f_bi5_triggered: bool,
    pub historical_outcome_comparators_used_only_after_independent_branch_seal: bool,
    pub mutation_falsifiers: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BiGranularityVerdict {
    Passed,
    Refuted,
    NotReached,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiGranularityAudit {
    pub level: String,
    pub verdict: BiGranularityVerdict,
    pub passed: bool,
    pub exact_basis: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiG1PairVerdict {
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub left_scheme_count: usize,
    pub right_scheme_count: usize,
    pub matched_scheme_count: usize,
    pub equivalent: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiPairComparison {
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub equal: bool,
    pub first_divergence_stage: Option<u32>,
    pub exact_context: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiConeBranchSummary {
    pub branch_root_hash: String,
    pub branch_certificate_digest: String,
    pub r_t1_class_id: String,
    pub economy_probe_class_key: String,
    pub debt_free_halt: bool,
    pub halt_stage: Option<u32>,
    pub successor_stage: Option<u32>,
    pub semantic_successor_o_empty: bool,
    pub halt_at_15: bool,
    pub semantic_o16_empty: bool,
    pub f1_excluded: bool,
    pub sum_kappa: u32,
    pub diagnostic_sum_nu: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiBranchIndexDisposition {
    pub enacted_branch_identification_status: String,
    pub remains_branch_indexed: Vec<String>,
    pub promoted_to_cone_level: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiConeCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<BiSourceBinding>,
    pub option_b_replayed: bool,
    pub all_four_branch_certificates_replayed_before_comparison: bool,
    pub branch_count: usize,
    pub enacted_branch_root_hash: String,
    pub bi0_certificate_digest: String,
    pub bi0_passed: bool,
    pub branches: Vec<BiConeBranchSummary>,
    pub g1_pair_verdicts: Vec<BiG1PairVerdict>,
    pub g2_pair_comparisons: Vec<BiPairComparison>,
    pub g3_pair_comparisons: Vec<BiPairComparison>,
    pub g1: BiGranularityAudit,
    pub g2: BiGranularityAudit,
    pub g3: BiGranularityAudit,
    pub g4: BiGranularityAudit,
    pub outcome_zone: String,
    pub first_divergence_published_verbatim: Option<String>,
    pub branch_index_disposition: BiBranchIndexDisposition,
    pub bridge_prerequisite_bi4_satisfied: bool,
    pub bridge_executed: bool,
    pub no_divergence_suppressed_averaged_or_repaired: bool,
    pub class_representative_substitution_used: bool,
    pub enacted_branch_privileged_beyond_indexical_testimony: bool,
    pub mutation_falsifiers: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug)]
pub struct BiProgramBundle {
    pub branches: Vec<BiBranchCertificate>,
    pub regression: BiRegressionCertificate,
    pub cone: BiConeCertificate,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum BiProgramError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted artifact did not replay: {0}")]
    EmittedReplay(String),
}

impl From<BranchInvarianceError> for BiProgramError {
    fn from(value: BranchInvarianceError) -> Self {
        Self::Prerequisite(value.to_string())
    }
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("BI evidence serializes");
    bytes_hash(&bytes)
}

fn binding(path: &str, role: &str, bytes: &[u8]) -> BiSourceBinding {
    BiSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    }
}

fn common_bindings() -> Vec<BiSourceBinding> {
    vec![
        binding(
            "docs/branch_invariance_program_plan.md",
            "frozen_pre_registered_BI_brief",
            PLAN_BYTES,
        ),
        binding(
            "docs/r_t3_stage4_adjudication.md",
            "adopted_Option_B_cone_and_F_R3_B1",
            R_T3_BYTES,
        ),
        binding(
            "crates/pen-search/src/branch_invariance.rs",
            "reference_isolated_candidate_level_continuation",
            CORE_SOURCE_BYTES,
        ),
        binding(
            "crates/pen-search/src/branch_invariance_finale.rs",
            "branch_parametric_E5_class_finale",
            FINALE_SOURCE_BYTES,
        ),
        binding(
            "crates/pen-search/src/act_local_provenance.rs",
            "act_local_nu_and_authoritative_ordinary_family_token_issuer",
            ACT_LOCAL_PROVENANCE_SOURCE_BYTES,
        ),
        binding(
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "adopted_depth_two_A3_instance_grammar",
            A3_SOURCE_BYTES,
        ),
        binding(
            "crates/pen-eval/src/a3_rule_inventory_exhaustiveness.rs",
            "exact_prefix_A3_exhaustiveness",
            A3_EXHAUSTIVENESS_SOURCE_BYTES,
        ),
        binding(
            "crates/pen-eval/src/future_hole_hypothesis_v2.rs",
            "dependent_registration_and_realization",
            FUTURE_V2_SOURCE_BYTES,
        ),
        binding(
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "certified_Stage4_R_T1_orbit_distinctness_input",
            TRANSPORT_SOURCE_BYTES,
        ),
        binding(
            "crates/pen-search/src/branch_invariance_program.rs",
            "BI_create_new_replay_and_comparator_isolation",
            THIS_SOURCE_BYTES,
        ),
    ]
}

fn regression_bindings() -> Vec<BiSourceBinding> {
    let mut rows = common_bindings();
    rows.extend([
        binding(
            "docs/bi0_prerequisites_adjudication.md",
            "adopted_chronological_slot_map_and_act_local_provenance_rules",
            BI0_PREREQUISITES_ADJUDICATION_BYTES,
        ),
        binding(
            "docs/chronological_interface_slot_map_v1.json",
            "replayed_F_SM1_chronological_interface_slot_map_prerequisite",
            CHRONOLOGICAL_SLOT_MAP_BYTES,
        ),
        binding(
            "docs/t_bi_nu1_act_local_provenance_v1.json",
            "replayed_T_BI_NU1_act_local_provenance_prerequisite",
            T_BI_NU1_BYTES,
        ),
        binding(
            "crates/pen-search/src/chronological_slot_map.rs",
            "F_SM1_F_SM2_prerequisite_replay_implementation",
            CHRONOLOGICAL_SLOT_MAP_SOURCE_BYTES,
        ),
        binding(
            "crates/pen-search/src/t_bi_nu1_regression.rs",
            "T_BI_NU1_F_AL1_prerequisite_replay_implementation",
            T_BI_NU1_SOURCE_BYTES,
        ),
        binding(
            "docs/phase5b_reselection_burn_v2.json",
            "sealed_stage4_identity_and_stages1_to7_comparator_only",
            V2_BURN_BYTES,
        ),
        binding(
            "docs/phase5b_reselection_program_v3.json",
            "frozen_v3_live_replay_diagnostic_only",
            V3_PROGRAM_BYTES,
        ),
        binding(
            "docs/phase5b_reselection_burn_v3.json",
            "certified_vector_winners_and_discharger_comparator_only",
            V3_BURN_BYTES,
        ),
        binding(
            "docs/schema2_e5_future_hole_finale_v2_dependent_context.json",
            "E5_dependent_context_regression_comparator_only",
            E5_BYTES,
        ),
        binding(
            "docs/phase5b_full_history_v1.json",
            "enacted_candidate_dependent_exact_nu_join_post_seal_only",
            FULL_HISTORY_BYTES,
        ),
    ]);
    rows
}

#[derive(Clone, Debug)]
struct Bi0PrerequisiteEvidence {
    adoption_replayed: bool,
    slot_map: ChronologicalSlotMapCertificate,
    slot_map_replay: ChronologicalSlotMapReplay,
    t_bi_nu1: TBiNu1RegressionCertificate,
    t_bi_nu1_replay: TBiNu1RegressionReplay,
}

/// Enforce the versioned prerequisite authority before the legacy BI-0
/// consumer can read either burned v1 certificate.  The present v3 artifacts
/// are replay-valid negative results, so this function must fail closed.  If
/// both falsifiers pass in a future successor, the downstream provenance join
/// must first be migrated to the v3 package shape; silently falling through to
/// the v1 join would reintroduce the exact authority disconnect recorded by
/// the audit burn.
fn enforce_v3_bi0_prerequisite_gate() -> Result<(), BiProgramError> {
    let burn = std::str::from_utf8(BI0_PREREQUISITES_V1_AUDIT_BURN_BYTES)
        .map_err(|error| BiProgramError::Prerequisite(error.to_string()))?;
    if !burn.contains("BURNED AS BI-0 AUTHORITY")
        || !burn.contains("v1 prerequisite artifacts are forbidden BI-0 inputs")
    {
        return Err(BiProgramError::Prerequisite(
            "BI-0 v1 prerequisite audit burn did not replay".to_owned(),
        ));
    }

    let slot_map: ChronologicalSlotMapAuditV3 =
        serde_json::from_slice(CHRONOLOGICAL_SLOT_MAP_V3_BYTES)
            .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let t_bi_nu1: TBiNu1RegressionV3Certificate = serde_json::from_slice(T_BI_NU1_V3_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let slot_replay = replay_chronological_slot_map_audit_v3(&slot_map);
    let t_bi_replay = replay_t_bi_nu1_regression_v3_certificate(&t_bi_nu1);

    let slot_passed = slot_map.schema == CHRONOLOGICAL_SLOT_MAP_V3_SCHEMA
        && slot_replay.valid
        && slot_map.f_sm1_passed
        && slot_map.bi0_prerequisite_side_reopened;
    let t_bi_passed = t_bi_nu1.schema == T_BI_NU1_REGRESSION_V3_SCHEMA
        && t_bi_replay.valid
        && t_bi_nu1.t_bi_nu1_proved_on_enacted_branch
        && t_bi_nu1.bi0_rerun_authorized_by_t_bi_nu1_side;

    if !slot_passed || !t_bi_passed {
        return Err(BiProgramError::Prerequisite(format!(
            "versioned BI-0 prerequisites deny authority: F-SM1={} (replay={}, derived={}/{}, named_gaps={}), F-AL1={} (replay={}, authoritative={}/15, first divergence={:?}, first capacity impossibility={:?}); burned v1/v2 artifacts were not read as authority",
            slot_map.f_sm1_passed,
            slot_replay.valid,
            slot_map.sealed_discharge_derived_count,
            slot_map.sealed_discharge_count,
            slot_map.sealed_discharge_named_gap_count,
            t_bi_nu1.f_al1.f_al1_passed,
            t_bi_replay.valid,
            t_bi_nu1.authoritative_package_count,
            t_bi_nu1.f_al1.first_exact_divergence,
            t_bi_nu1.first_capacity_impossibility_stage,
        )));
    }

    Err(BiProgramError::Prerequisite(
        "both v3 prerequisites pass, but the BI-0 ledger consumer still has the burned v1 package shape; implement and replay the candidate-level v3 provenance join before BI-0 create-new"
            .to_owned(),
    ))
}

fn load_and_replay_bi0_prerequisites() -> Result<Bi0PrerequisiteEvidence, BiProgramError> {
    let adjudication = std::str::from_utf8(BI0_PREREQUISITES_ADJUDICATION_BYTES)
        .map_err(|error| BiProgramError::Prerequisite(error.to_string()))?;
    let adoption_replayed = adjudication.contains("**Status:** **ADOPTED** 2026-07-22")
        && adjudication.contains("`chronological-interface-slot-map-v1`")
        && adjudication.contains("`act-local-provenance-v1`")
        && adjudication.contains("BI-0 must rerun and pass before any BI-1")
        && adjudication.contains("Halvor Lande, 22 July 2026");
    if !adoption_replayed {
        return Err(BiProgramError::Prerequisite(
            "BI-0 prerequisite adjudication did not replay its adopted rule names, standing hard gate, and adoption block"
                .to_owned(),
        ));
    }
    enforce_v3_bi0_prerequisite_gate()?;
    let slot_map: ChronologicalSlotMapCertificate =
        serde_json::from_slice(CHRONOLOGICAL_SLOT_MAP_BYTES)
            .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let t_bi_nu1: TBiNu1RegressionCertificate = serde_json::from_slice(T_BI_NU1_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let slot_map_replay = replay_chronological_slot_map_certificate(&slot_map);
    let t_bi_nu1_replay = replay_t_bi_nu1_regression_certificate(&t_bi_nu1);

    let slot_map_complete = slot_map.schema == CHRONOLOGICAL_SLOT_MAP_CERTIFICATE_SCHEMA
        && slot_map_replay.valid
        && slot_map.f_sm1_passed
        && slot_map.f_sm2_no_inference_or_override
        && slot_map.every_declaration_order_preserving_identity
        && slot_map.every_declaration_zero_charge
        && slot_map.every_membership_derived
        && slot_map.historical_chronological_instance_count
            == slot_map.historical_chronological_replayed_count
        && slot_map.sealed_chronological_discharge_count
            == slot_map.sealed_chronological_discharge_replayed_count
        && slot_map.formerly_gapped_nine_ids_exact
        && slot_map.formerly_gapped_nine_all_derived
        && slot_map.formerly_gapped_expected_instance_ids.len() == 9
        && slot_map.formerly_gapped_observed_instance_ids.len() == 9
        && slot_map.frozen_stage16_89_id_surface_exact
        && slot_map.frozen_stage16_inventory_count == 89
        && slot_map.live_stage16_inventory_count == 89
        && slot_map.alternative_slot_map_permutation_trial_count == 0
        && slot_map.instance_override_count == 0
        && !slot_map.outcome_or_verdict_used_as_definition_input
        && slot_map.non_enacted_branch_continuation_count == 0
        && slot_map.bi0_rerun_required
        && !slot_map.bi1_or_cone_authorized;
    if !slot_map_complete {
        return Err(BiProgramError::Prerequisite(format!(
            "chronological slot-map prerequisite did not replay as the exact F-SM1/F-SM2 gate: {:?}",
            slot_map_replay.errors
        )));
    }

    let t_bi_nu1_complete = t_bi_nu1.schema == T_BI_NU1_REGRESSION_SCHEMA
        && t_bi_nu1_replay.valid
        && t_bi_nu1.all_fifteen_acts_issued
        && t_bi_nu1.intrinsic_packages.len() == 15
        && t_bi_nu1.closure_rows.len() == 15
        && t_bi_nu1.all_fifteen_packages_authoritative
        && t_bi_nu1.authoritative_token_hashes_globally_unique
        && t_bi_nu1.structural_filler_class_count == 13
        && t_bi_nu1.all_thirteen_structural_filler_classes_covered
        && t_bi_nu1.f_al1.archive_self_digest_valid
        && t_bi_nu1.f_al1.archive_used_only_as_post_issuance_comparator
        && t_bi_nu1.f_al1.candidate_hash_vector_exact
        && t_bi_nu1.f_al1.predecessor_signature_vector_exact
        && t_bi_nu1.f_al1.nu_vector_exact
        && t_bi_nu1.f_al1.divergences.is_empty()
        && !t_bi_nu1.f_al1.f_al1_triggered
        && t_bi_nu1.f_al1.f_al1_passed
        && t_bi_nu1.t_bi_nu1_proved_on_enacted_branch
        && t_bi_nu1.bi0_rerun_authorized_by_t_bi_nu1_side
        && !t_bi_nu1.non_enacted_branch_work_executed;
    if !t_bi_nu1_complete {
        return Err(BiProgramError::Prerequisite(format!(
            "T-BI-NU1 prerequisite did not replay as a total 15-act/13-filler F-AL1 gate: {:?}",
            t_bi_nu1_replay.errors
        )));
    }

    Ok(Bi0PrerequisiteEvidence {
        adoption_replayed,
        slot_map,
        slot_map_replay,
        t_bi_nu1,
        t_bi_nu1_replay,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CompleteLedgerProvenanceAudit {
    ledger_entry_count: usize,
    exact_provenance_count: usize,
    prerequisite_package_count: usize,
    prerequisite_authoritative_token_count: usize,
    ledger_authoritative_token_count: usize,
    authoritative_token_count_exact: bool,
    stage_surface_exact: bool,
    exact_join: bool,
}

fn complete_ledger_provenance_audit(
    branch: &BiBranchCertificate,
    t_bi_nu1: &TBiNu1RegressionCertificate,
) -> CompleteLedgerProvenanceAudit {
    let packages = t_bi_nu1
        .intrinsic_packages
        .iter()
        .map(|package| (package.stage, package))
        .collect::<BTreeMap<_, _>>();
    let prerequisite_authoritative_token_count = t_bi_nu1
        .intrinsic_packages
        .iter()
        .map(|package| package.ordinary_family_tokens.len())
        .sum::<usize>();
    let mut exact_provenance_count = 0usize;
    let mut ledger_authoritative_token_count = 0usize;
    let mut every_join_exact = true;

    for row in &branch.continuation.complete_ledger {
        let Some(package) = packages.get(&row.stage).copied() else {
            every_join_exact = false;
            continue;
        };
        let token_replays = replay_branch_nu_provenance_token(&row.nu_provenance).is_empty();
        let common_fields_exact = row.nu_provenance.stage == row.stage
            && row.nu_provenance.candidate_hash == row.candidate_hash
            && row.nu_provenance.predecessor_signature_digest
                == package.predecessor_signature_digest
            && row.nu_provenance.structural_formula_total == package.structural_formula_total
            && row.nu_provenance.r2_generated_instance_adjustment
                == package.generated_instance_adjustment
            && row.nu_provenance.diagnostic_nu == row.semantic_nu
            && row.nu_provenance.diagnostic_nu == package.exact_certified_nu
            && package.candidate_hash == row.candidate_hash
            && package.authoritative;
        let disposition_exact = match &row.nu_provenance.disposition {
            BranchNuProvenanceDisposition::ExactCertified {
                theorem_id,
                certificate_hash,
                counted_family_ids,
                authoritative_family_token_hashes,
                generated_instance_family_ids_removed_by_quotient,
            } => {
                ledger_authoritative_token_count += authoritative_family_token_hashes.len();
                theorem_id == &package.theorem_id
                    && certificate_hash == &package.derivation_hash
                    && counted_family_ids == &package.counted_family_ids()
                    && authoritative_family_token_hashes == &package.authoritative_token_hashes()
                    && authoritative_family_token_hashes.len()
                        == package.ordinary_family_tokens.len()
                    && generated_instance_family_ids_removed_by_quotient
                        == &package.generated_instance_family_ids_removed_by_quotient
            }
            BranchNuProvenanceDisposition::DiagnosticFormulaOnly { .. } => false,
        };
        let joined = token_replays && common_fields_exact && disposition_exact;
        exact_provenance_count += usize::from(joined);
        every_join_exact &= joined;
    }
    let ledger_entry_count = branch.continuation.complete_ledger.len();
    let prerequisite_package_count = t_bi_nu1.intrinsic_packages.len();
    let expected_stages = (1..=15).collect::<BTreeSet<_>>();
    let ledger_stages = branch
        .continuation
        .complete_ledger
        .iter()
        .map(|row| row.stage)
        .collect::<BTreeSet<_>>();
    let package_stages = t_bi_nu1
        .intrinsic_packages
        .iter()
        .map(|package| package.stage)
        .collect::<BTreeSet<_>>();
    let stage_surface_exact = ledger_stages == expected_stages && package_stages == expected_stages;
    let authoritative_token_count_exact =
        ledger_authoritative_token_count == prerequisite_authoritative_token_count;
    let exact_join = every_join_exact
        && ledger_entry_count == 15
        && prerequisite_package_count == 15
        && packages.len() == 15
        && stage_surface_exact
        && exact_provenance_count == 15
        && authoritative_token_count_exact;
    CompleteLedgerProvenanceAudit {
        ledger_entry_count,
        exact_provenance_count,
        prerequisite_package_count,
        prerequisite_authoritative_token_count,
        ledger_authoritative_token_count,
        authoritative_token_count_exact,
        stage_surface_exact,
        exact_join,
    }
}

fn cone_bindings() -> Vec<BiSourceBinding> {
    vec![
        binding(
            "docs/branch_invariance_program_plan.md",
            "pre_registered_G1_to_G4_and_outcome_zones",
            PLAN_BYTES,
        ),
        binding(
            "docs/r_t3_stage4_adjudication.md",
            "adopted_Option_B_branch_index_discipline",
            R_T3_BYTES,
        ),
        binding(
            "docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json",
            "authoritative_G1_refutation_only",
            R_T2_BYTES,
        ),
        binding(
            "crates/pen-search/src/branch_invariance_program.rs",
            "BI4_post_seal_comparator",
            THIS_SOURCE_BYTES,
        ),
    ]
}

fn branch_digest(certificate: &BiBranchCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash(
        BI_BRANCH_CERTIFICATE_SCHEMA,
        "branch-certificate",
        &projection,
    )
}

fn regression_digest(certificate: &BiRegressionCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash(
        BI_REGRESSION_CERTIFICATE_SCHEMA,
        "regression-certificate",
        &projection,
    )
}

fn cone_digest(certificate: &BiConeCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash(BI_CONE_CERTIFICATE_SCHEMA, "cone-certificate", &projection)
}

fn digest_prefix(hash: &str) -> String {
    hash.strip_prefix("blake3:")
        .unwrap_or(hash)
        .chars()
        .take(8)
        .collect()
}

fn obligation_profile(continuation: &BranchContinuation) -> Vec<BiObligationProfileRow> {
    let mut rows = continuation
        .stages
        .iter()
        .map(|stage| BiObligationProfileRow {
            stage: stage.stage,
            required_packages: stage.demand.a3_required_packages.clone(),
            structural_constructors: stage.demand.structural_constructors.clone(),
            live_obligation_count: stage.demand.structural_instance_ids.len(),
            total_typed_discharge: stage.discharger_count == 1 && stage.winner.is_some(),
            debt_free: stage.demand.debt_free,
        })
        .collect::<Vec<_>>();
    rows.push(BiObligationProfileRow {
        stage: continuation.terminal_demand.stage,
        required_packages: continuation.terminal_demand.a3_required_packages.clone(),
        structural_constructors: continuation.terminal_demand.structural_constructors.clone(),
        live_obligation_count: continuation.terminal_demand.structural_instance_ids.len(),
        total_typed_discharge: false,
        debt_free: continuation.terminal_demand.debt_free,
    });
    rows
}

fn numeric_ledger(continuation: &BranchContinuation) -> Vec<BiNumericLedgerRow> {
    let stages = continuation
        .stages
        .iter()
        .map(|row| (row.stage, row))
        .collect::<BTreeMap<_, _>>();
    continuation
        .complete_ledger
        .iter()
        .filter(|row| row.stage >= 5)
        .map(|row| BiNumericLedgerRow {
            stage: row.stage,
            winner_hash: row.candidate_hash.clone(),
            kappa: row.kappa,
            diagnostic_nu: row.semantic_nu,
            nu_provenance: row.nu_provenance.clone(),
            diagnostic_bar: stages
                .get(&row.stage)
                .map(|stage| stage.diagnostic_bar.clone()),
        })
        .collect()
}

fn issue_branch_from_cone(
    cone: &CertifiedStage4BranchCone,
    branch_root_hash: &str,
) -> Result<BiBranchCertificate, BiProgramError> {
    if !replay_certified_stage4_branch_cone(cone).is_empty() {
        return Err(BiProgramError::Prerequisite(
            "Stage-4 cone did not replay before branch execution".to_owned(),
        ));
    }
    let continuation = execute_branch_continuation(
        cone,
        branch_root_hash,
        &BranchContinuationLimits {
            max_inspected_stage: BI_MAX_INSPECTED_STAGE,
            max_enumerated_candidates_per_stage: BI_MAX_ENUMERATED_CANDIDATES_PER_STAGE,
        },
    )?;
    let continuation_errors = replay_branch_continuation(cone, &continuation);
    if !continuation_errors.is_empty() {
        return Err(BiProgramError::Invariant(format!(
            "branch continuation did not replay: {continuation_errors:?}"
        )));
    }
    let (local_halt_stage, local_successor_stage) = match &continuation.outcome {
        BranchContinuationOutcome::DebtFreeHalt {
            halt_stage,
            next_stage,
        } => (Some(*halt_stage), Some(*next_stage)),
        _ => (None, None),
    };
    let local_debt_free_halt = local_halt_stage.is_some();
    let (finale, finale_not_run_reason) = if local_debt_free_halt {
        let finale = execute_certified_branch_finale(cone, &continuation)
            .map_err(BiProgramError::Invariant)?;
        let finale_errors = replay_certified_branch_finale(cone, &continuation, &finale);
        if !finale_errors.is_empty() {
            return Err(BiProgramError::Invariant(format!(
                "branch finale did not replay: {finale_errors:?}"
            )));
        }
        (Some(finale), None)
    } else {
        (
            None,
            Some(format!(
                "lawful continuation terminus: {:?}",
                continuation.outcome
            )),
        )
    };
    let profile = obligation_profile(&continuation);
    let candidate_assessment_count = continuation
        .stages
        .iter()
        .map(|stage| stage.assessments.len())
        .sum();
    let semantic_evidence_gap_count = continuation
        .stages
        .iter()
        .flat_map(|stage| &stage.assessments)
        .filter(|assessment| assessment.semantic_evidence_gap)
        .count();
    let exact_nu_provenance_failure_count = continuation
        .stages
        .iter()
        .flat_map(|stage| &stage.assessments)
        .filter(|assessment| !assessment.nu_provenance.is_exact_certified())
        .count();
    let selected_winner_count = continuation
        .stages
        .iter()
        .filter(|stage| stage.winner.is_some())
        .count();
    let every_selected_winner_exact_nu_certified = selected_winner_count > 0
        && continuation
            .stages
            .iter()
            .filter_map(|stage| {
                stage.winner.as_ref().map(|winner| {
                    stage.assessments.iter().any(|assessment| {
                        assessment.candidate_hash == winner.candidate_hash
                            && assessment.nu_provenance.is_exact_certified()
                            && !assessment.semantic_evidence_gap
                    })
                })
            })
            .all(|supported| supported);
    let complete_ledger_exact_nu_provenance_count = continuation
        .complete_ledger
        .iter()
        .filter(|row| {
            row.nu_provenance.is_exact_certified()
                && replay_branch_nu_provenance_token(&row.nu_provenance).is_empty()
        })
        .count();
    let every_complete_ledger_entry_exact_nu_certified =
        continuation.complete_ledger.len() == 15 && complete_ledger_exact_nu_provenance_count == 15;
    // The exact-nu decomposition gap is a G3 measurement stop, not a
    // candidate-selection gap. Guarded uniqueness depends only on the exact
    // A3 discharge evidence, and therefore remains executable.
    let complete_enumerated_cone_semantic_evidence_closed = semantic_evidence_gap_count == 0;
    if !complete_enumerated_cone_semantic_evidence_closed {
        return Err(BiProgramError::Invariant(format!(
            "branch uniqueness is underdetermined: assessments={candidate_assessment_count}, semantic_gaps={semantic_evidence_gap_count}"
        )));
    }
    let one_live_demand_per_reached_guarded_stage = profile
        .iter()
        .filter(|row| !row.debt_free)
        .all(|row| row.live_obligation_count == 1);
    let every_reached_live_demand_totally_discharged = profile
        .iter()
        .filter(|row| !row.debt_free)
        .all(|row| row.total_typed_discharge);
    let local_semantic_successor_o_empty = finale
        .as_ref()
        .is_some_and(|value| value.semantic_successor_o_empty);
    let local_semantic_o16_empty = local_halt_stage == Some(15)
        && local_successor_stage == Some(16)
        && local_semantic_successor_o_empty;
    let local_f1_excluded = finale.as_ref().is_some_and(|value| value.f1_excluded);
    let local_e5_class_complete = finale.as_ref().is_some_and(|value| value.e5_class_complete);
    let complete_sum_kappa = continuation
        .complete_ledger
        .iter()
        .map(|row| u32::from(row.kappa))
        .sum();
    let complete_diagnostic_sum_nu = continuation
        .complete_ledger
        .iter()
        .map(|row| row.semantic_nu)
        .sum();
    let branch = &continuation.branch;
    let mut certificate = BiBranchCertificate {
        schema: BI_BRANCH_CERTIFICATE_SCHEMA.to_owned(),
        date: BI_PROGRAM_DATE.to_owned(),
        source_bindings: common_bindings(),
        cone_digest: cone.derivation_hash.clone(),
        branch_root_hash: branch.candidate_hash.clone(),
        branch_digest_prefix: digest_prefix(&branch.candidate_hash),
        r_t1_class_id: branch.r_t1_class_id.clone(),
        economy_probe_class_key: branch.economy_probe_class_key.clone(),
        numeric_ledger_stage5_onward: numeric_ledger(&continuation),
        continuation,
        obligation_profile: profile,
        complete_sum_kappa,
        complete_diagnostic_sum_nu,
        finale,
        finale_not_run_reason,
        candidate_assessment_count,
        semantic_evidence_gap_count,
        exact_nu_provenance_failure_count,
        every_selected_winner_exact_nu_certified,
        complete_ledger_exact_nu_provenance_count,
        every_complete_ledger_entry_exact_nu_certified,
        complete_enumerated_cone_semantic_evidence_closed,
        one_live_demand_per_reached_guarded_stage,
        every_reached_live_demand_totally_discharged,
        local_debt_free_halt,
        local_halt_stage,
        local_successor_stage,
        local_semantic_successor_o_empty,
        local_g4_debt_free_halt_at_15: false,
        local_semantic_o16_empty,
        local_f1_excluded,
        local_e5_class_complete,
        enacted_reference_artifacts_consumed_by_branch_issuer: false,
        enacted_outcomes_consumed_for_acceptance_selection_or_steering: false,
        claim_scope_before_bi4: "branch_indexed_only_pending_BI4".to_owned(),
        mutation_falsifiers: vec![
            "flip_any_verdict_or_G4_flag_then_replay_must_fail".to_owned(),
            "flip_any_count_or_kappa_nu_row_then_replay_must_fail".to_owned(),
            "flip_any_winner_or_branch_root_digest_then_replay_must_fail".to_owned(),
            "flip_any_finale_membership_O16_or_F1_field_then_replay_must_fail".to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.local_g4_debt_free_halt_at_15 = certificate.local_debt_free_halt
        && certificate.local_halt_stage == Some(15)
        && certificate.local_successor_stage == Some(16)
        && certificate.local_semantic_o16_empty
        && certificate.local_f1_excluded
        && certificate.local_e5_class_complete;
    certificate.result_digest = branch_digest(&certificate);
    Ok(certificate)
}

/// Ungated branch issuance is deliberately private. The only public issuance
/// surface for BI-1 is `issue_bi_program_after_bi0`, which requires a sealed,
/// replayed, passing BI-0 v2 certificate.
fn issue_bi_branch_certificate(
    branch_root_hash: &str,
) -> Result<BiBranchCertificate, BiProgramError> {
    let cone = issue_certified_stage4_branch_cone()?;
    issue_branch_from_cone(&cone, branch_root_hash)
}

/// Replay an already materialized branch certificate.  Replay performs an
/// independent branch reissuance, so it is crate-private as well: otherwise a
/// forged claim could use the verifier as a pre-BI0 continuation oracle.
pub(crate) fn replay_bi_branch_certificate(certificate: &BiBranchCertificate) -> BiReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != branch_digest(certificate) {
        errors.push("branch certificate digest mismatch".to_owned());
        return BiReplay {
            valid: false,
            errors,
        };
    }
    match issue_bi_branch_certificate(&certificate.branch_root_hash) {
        Ok(expected) if &expected == certificate => {}
        Ok(_) => errors.push("branch certificate differs from independent reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    BiReplay {
        valid: errors.is_empty(),
        errors,
    }
}

fn archived_v2_digest_valid(certificate: &Phase5bReselectionBurn) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == tagged_hash(
            PHASE5B_RESELECTION_BURN_SCHEMA,
            "reselection-burn",
            &projection,
        )
}

fn archived_v3_digest_valid(certificate: &Phase5bReselectionV3Burn) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == tagged_hash(PHASE5B_RESELECTION_V3_BURN_SCHEMA, "v3-burn", &projection)
}

fn archived_e5_digest_valid(certificate: &E5FutureHoleFinaleV2Certificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == tagged_hash(E5_FUTURE_HOLE_FINALE_V2_SCHEMA, "certificate", &projection)
}

fn archived_full_history_digest_valid(certificate: &Phase5bHistoryCertificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == tagged_hash(
            PHASE5B_HISTORY_CERT_SCHEMA,
            "phase5b-history-certificate",
            &projection,
        )
        && certificate.all_fifteen_steps_certified
        && certificate.all_amplification_provenanced
        && certificate.ledger.every_entry_certified
        && certificate.steps.len() == 15
        && certificate.steps.iter().all(|step| {
            step.certified
                && step.every_counted_family_provenanced
                && step.local_anchor_injection_holds
                && step.demand_output_nonreuse_holds
        })
}

fn expected_e5_projection(
    certificate: &E5FutureHoleFinaleV2Certificate,
) -> BiE5RegressionProjection {
    BiE5RegressionProjection {
        historical_window_count: certificate.a3_coverage.historical_window_count,
        final_a3_inventory_count: certificate.stage16.exact_a3_inventory_count,
        final_unary_count: certificate.stage16.unary_registration_count,
        final_direct_chronological_count: certificate.stage16.direct_chronological_instance_count,
        final_pointwise_chronological_count: certificate
            .stage16
            .pointwise_chronological_instance_count,
        final_higher_count: certificate.stage16.higher_instance_count,
        final_structural_count: certificate.stage16.structural_instance_count,
        structural_registration_count: certificate.historical.structural_registration_count,
        structural_realization_count: certificate.historical.structural_realization_count,
        exact_d_partition: certificate.stage16.d_partition_complete,
        derivable_instance_count: certificate.stage16.derivable_instance_ids.len(),
        underdetermined_instance_count: certificate.stage16.underdetermined_instance_ids.len(),
        stage3_wrinkle_reproduced: certificate.historical.stage_three_wrinkle_reproduced,
        focus_projection_reproduces_ladder: certificate
            .historical
            .focus_projection_reproduces_entire_coarse_ladder,
        semantic_o16_empty: certificate.stage16.semantic_o16_empty == Some(true),
        f1_executed: certificate.f1.f1_executed,
        f1_excluded: certificate.f1.f1_excluded == Some(true),
        theorem12_full_instance_granularity_proved: certificate
            .f1
            .theorem12_full_instance_granularity_proved
            == Some(true),
        e5_class_complete: certificate.e5_complete,
    }
}

fn observed_e5_projection(finale: &BranchFinaleAudit) -> BiE5RegressionProjection {
    BiE5RegressionProjection {
        historical_window_count: finale.windows.len(),
        final_a3_inventory_count: finale.final_a3_inventory_count,
        final_unary_count: finale.final_unary_count,
        final_direct_chronological_count: finale.final_direct_chronological_count,
        final_pointwise_chronological_count: finale.final_pointwise_chronological_count,
        final_higher_count: finale.final_higher_count,
        final_structural_count: finale.final_structural_count,
        structural_registration_count: finale.structural_registration_count,
        structural_realization_count: finale.structural_realization_count,
        exact_d_partition: finale.exact_d_partition,
        derivable_instance_count: finale.derivable_instance_count,
        underdetermined_instance_count: finale.underdetermined_instance_ids.len(),
        stage3_wrinkle_reproduced: finale.stage3_wrinkle_reproduced,
        focus_projection_reproduces_ladder: finale.focus_projection_reproduces_branch_ladder,
        semantic_o16_empty: finale.semantic_successor_o_empty,
        f1_executed: finale.f1_executed,
        f1_excluded: finale.f1_excluded,
        theorem12_full_instance_granularity_proved: finale
            .theorem12_full_instance_granularity_proved,
        e5_class_complete: finale.e5_class_complete,
    }
}

fn expected_winners(
    v2: &Phase5bReselectionBurn,
    v3: &Phase5bReselectionV3Burn,
) -> Result<Vec<(u32, String)>, BiProgramError> {
    let mut rows = Vec::new();
    for stage in &v2.stages {
        if stage.stage > 7 {
            continue;
        }
        let winner = stage.winner.as_ref().ok_or_else(|| {
            BiProgramError::Prerequisite(format!(
                "frozen v2 certificate omits its Stage {} winner",
                stage.stage
            ))
        })?;
        rows.push((stage.stage, winner.candidate_hash.clone()));
    }
    for stage in &v3.stages {
        let winner = stage.winner.as_ref().ok_or_else(|| {
            BiProgramError::Prerequisite(format!(
                "frozen v3 certificate omits its Stage {} winner",
                stage.stage
            ))
        })?;
        rows.push((stage.stage, winner.candidate_hash.clone()));
    }
    rows.sort_by_key(|row| row.0);
    Ok(rows)
}

fn observed_vector(certificate: &BiBranchCertificate) -> Vec<(u32, u32, u32)> {
    certificate
        .continuation
        .complete_ledger
        .iter()
        .map(|row| (row.stage, row.semantic_nu, u32::from(row.kappa)))
        .collect()
}

fn observed_winners(certificate: &BiBranchCertificate) -> Vec<(u32, String)> {
    certificate
        .continuation
        .complete_ledger
        .iter()
        .map(|row| (row.stage, row.candidate_hash.clone()))
        .collect()
}

fn issue_regression_from_sealed_branch(
    branch: &BiBranchCertificate,
    prerequisites: &Bi0PrerequisiteEvidence,
) -> Result<BiRegressionCertificate, BiProgramError> {
    let branch_replay = replay_bi_branch_certificate(branch);
    if !branch_replay.valid {
        return Err(BiProgramError::Prerequisite(format!(
            "BI-0 comparator received an unsealed branch certificate: {:?}",
            branch_replay.errors
        )));
    }
    let v2: Phase5bReselectionBurn = serde_json::from_slice(V2_BURN_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let v3_program: Phase5bReselectionV3Program = serde_json::from_slice(V3_PROGRAM_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let v3: Phase5bReselectionV3Burn = serde_json::from_slice(V3_BURN_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let e5: E5FutureHoleFinaleV2Certificate = serde_json::from_slice(E5_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let full_history: Phase5bHistoryCertificate = serde_json::from_slice(FULL_HISTORY_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;

    let stage4 = v2
        .stages
        .iter()
        .find(|row| row.stage == 4)
        .and_then(|row| row.winner.as_ref())
        .ok_or_else(|| {
            BiProgramError::Prerequisite(
                "frozen v2 certificate has no sealed Stage-4 winner".to_owned(),
            )
        })?;
    if stage4.candidate_hash != branch.branch_root_hash {
        return Err(BiProgramError::Prerequisite(format!(
            "BI-0 requires sealed Stage-4 root {}, received {}",
            stage4.candidate_hash, branch.branch_root_hash
        )));
    }
    let v3_live = replay_phase5b_reselection_v3_burn(&v3_program, &v3);
    let e5_live = replay_e5_future_hole_finale_v2_certificate(&e5);
    let expected_complete_vector = v3.complete_revised_prefix.clone();
    let observed_complete_vector = observed_vector(branch);
    let complete_vector_exact = expected_complete_vector == observed_complete_vector;
    let expected_winner_hashes = expected_winners(&v2, &v3)?;
    let observed_winner_hashes = observed_winners(branch);
    let winner_hashes_exact = expected_winner_hashes == observed_winner_hashes;
    let expected_v3_discharger_census = v3.discharger_counts.clone();
    let observed_v3_discharger_census = branch
        .continuation
        .stages
        .iter()
        .filter(|row| row.stage >= 8)
        .map(|row| (row.stage, row.discharger_count))
        .collect::<Vec<_>>();
    let v3_discharger_census_exact = expected_v3_discharger_census == observed_v3_discharger_census;
    let finale = branch.finale.as_ref().ok_or_else(|| {
        BiProgramError::Invariant("BI-0 branch has no independent E-5-class finale".to_owned())
    })?;
    let expected_e5_projection = expected_e5_projection(&e5);
    let observed_e5_projection = observed_e5_projection(finale);
    let e5_projection_exact = expected_e5_projection == observed_e5_projection;
    let expected_membership_ids = e5
        .stage16
        .membership_rows
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let observed_membership_ids = finale
        .membership
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let e5_membership_instance_ids_exact = expected_membership_ids == observed_membership_ids;
    let v2_archive_digest_valid = archived_v2_digest_valid(&v2);
    let v3_archive_digest_valid =
        archived_v3_digest_valid(&v3) && v3.imported_v2_burn_digest == v2.result_digest;
    let e5_archive_digest_valid = archived_e5_digest_valid(&e5);
    let enacted_full_history_archive_digest_valid =
        archived_full_history_digest_valid(&full_history);
    let enacted_archived_candidate_hash_total_join_valid = enacted_full_history_archive_digest_valid
        && full_history.steps.len() == branch.continuation.complete_ledger.len()
        && full_history
            .steps
            .iter()
            .zip(&branch.continuation.complete_ledger)
            .all(|(certified, observed)| {
                certified.step == observed.stage
                    && certified.candidate_hash == observed.candidate_hash
                    && certified.kappa == u32::from(observed.kappa)
                    && certified.certified_semantic_total == observed.semantic_nu
            });
    let enacted_archived_certified_vector_numerically_matched_post_seal =
        enacted_archived_candidate_hash_total_join_valid;
    let provenance = complete_ledger_provenance_audit(branch, &prerequisites.t_bi_nu1);
    let chronological_slot_map_former_nine_exact_and_derived =
        prerequisites.slot_map.formerly_gapped_nine_ids_exact
            && prerequisites.slot_map.formerly_gapped_nine_all_derived
            && prerequisites
                .slot_map
                .formerly_gapped_observed_instance_ids
                .len()
                == 9;
    let prerequisite_gate_passed = prerequisites.adoption_replayed
        && prerequisites.slot_map_replay.valid
        && prerequisites.slot_map.f_sm1_passed
        && prerequisites.slot_map.f_sm2_no_inference_or_override
        && prerequisites.slot_map.every_declaration_zero_charge
        && chronological_slot_map_former_nine_exact_and_derived
        && prerequisites.slot_map.frozen_stage16_89_id_surface_exact
        && prerequisites.slot_map.non_enacted_branch_continuation_count == 0
        && prerequisites.t_bi_nu1_replay.valid
        && prerequisites.t_bi_nu1.all_fifteen_acts_issued
        && prerequisites.t_bi_nu1.all_fifteen_packages_authoritative
        && prerequisites
            .t_bi_nu1
            .authoritative_token_hashes_globally_unique
        && prerequisites
            .t_bi_nu1
            .all_thirteen_structural_filler_classes_covered
        && prerequisites.t_bi_nu1.f_al1.f_al1_passed
        && !prerequisites.t_bi_nu1.non_enacted_branch_work_executed;
    let enacted_nu_provenance_gap_count = provenance
        .ledger_entry_count
        .saturating_sub(provenance.exact_provenance_count);
    let enacted_winner_nu_provenance_exact = provenance.exact_join
        && branch.every_selected_winner_exact_nu_certified
        && branch.every_complete_ledger_entry_exact_nu_certified;
    let bi0_passed = prerequisite_gate_passed
        && provenance.exact_join
        && enacted_nu_provenance_gap_count == 0
        && enacted_winner_nu_provenance_exact
        && branch.semantic_evidence_gap_count == 0
        && branch.complete_enumerated_cone_semantic_evidence_closed
        && v2_archive_digest_valid
        && v3_archive_digest_valid
        && e5_archive_digest_valid
        && enacted_archived_certified_vector_numerically_matched_post_seal
        && complete_vector_exact
        && winner_hashes_exact
        && v3_discharger_census_exact
        && e5_projection_exact
        && e5_membership_instance_ids_exact
        && finale.e5_class_complete
        && finale.expressivity_gaps.is_empty()
        && finale.underdetermined_instance_ids.is_empty()
        && finale.issuer_gap_instance_ids.is_empty()
        && finale.realization_gap_ids.is_empty();
    let mut certificate = BiRegressionCertificate {
        schema: BI_REGRESSION_CERTIFICATE_SCHEMA.to_owned(),
        date: BI_PROGRAM_DATE.to_owned(),
        source_bindings: regression_bindings(),
        sealed_branch_certificate_digest: branch.result_digest.clone(),
        enacted_branch_root_hash: branch.branch_root_hash.clone(),
        enacted_identified_by_sealed_stage4_digest_only: true,
        only_historical_read_by_branch_issuer_was_stage4_identity: true,
        enacted_branch_replayed_before_bi0_outcome_comparisons: true,
        non_enacted_branches_issued_before_bi0: 0,
        bi0_is_hard_gate_before_bi1: true,
        prerequisites_adjudication_digest: bytes_hash(BI0_PREREQUISITES_ADJUDICATION_BYTES),
        prerequisites_adoption_replayed: prerequisites.adoption_replayed,
        prerequisites_replayed_before_enacted_branch_issuance: true,
        chronological_slot_map_schema: prerequisites.slot_map.schema.clone(),
        chronological_slot_map_certificate_digest: prerequisites.slot_map.result_digest.clone(),
        chronological_slot_map_replay_valid: prerequisites.slot_map_replay.valid,
        chronological_slot_map_replay_errors: prerequisites.slot_map_replay.errors.clone(),
        chronological_slot_map_f_sm1_passed: prerequisites.slot_map.f_sm1_passed,
        chronological_slot_map_f_sm2_passed: prerequisites.slot_map.f_sm2_no_inference_or_override,
        chronological_slot_map_every_declaration_zero_charge: prerequisites
            .slot_map
            .every_declaration_zero_charge,
        chronological_slot_map_former_nine_exact_and_derived,
        chronological_slot_map_stage16_89_id_surface_exact: prerequisites
            .slot_map
            .frozen_stage16_89_id_surface_exact,
        chronological_slot_map_non_enacted_branch_count: prerequisites
            .slot_map
            .non_enacted_branch_continuation_count,
        t_bi_nu1_schema: prerequisites.t_bi_nu1.schema.clone(),
        t_bi_nu1_certificate_digest: prerequisites.t_bi_nu1.result_digest.clone(),
        t_bi_nu1_replay_valid: prerequisites.t_bi_nu1_replay.valid,
        t_bi_nu1_replay_errors: prerequisites.t_bi_nu1_replay.errors.clone(),
        t_bi_nu1_all_fifteen_acts_issued: prerequisites.t_bi_nu1.all_fifteen_acts_issued,
        t_bi_nu1_all_fifteen_packages_authoritative: prerequisites
            .t_bi_nu1
            .all_fifteen_packages_authoritative,
        t_bi_nu1_package_count: provenance.prerequisite_package_count,
        t_bi_nu1_authoritative_token_hashes_globally_unique: prerequisites
            .t_bi_nu1
            .authoritative_token_hashes_globally_unique,
        t_bi_nu1_all_thirteen_structural_filler_classes_covered: prerequisites
            .t_bi_nu1
            .all_thirteen_structural_filler_classes_covered,
        t_bi_nu1_f_al1_passed: prerequisites.t_bi_nu1.f_al1.f_al1_passed,
        t_bi_nu1_non_enacted_branch_work_executed: prerequisites
            .t_bi_nu1
            .non_enacted_branch_work_executed,
        enacted_complete_ledger_entry_count: provenance.ledger_entry_count,
        enacted_complete_ledger_exact_provenance_count: provenance.exact_provenance_count,
        enacted_complete_ledger_stage_surface_exact: provenance.stage_surface_exact,
        t_bi_nu1_authoritative_token_count: provenance.prerequisite_authoritative_token_count,
        enacted_complete_ledger_authoritative_token_count: provenance
            .ledger_authoritative_token_count,
        enacted_complete_ledger_authoritative_token_count_exact: provenance
            .authoritative_token_count_exact,
        enacted_complete_ledger_provenance_join_exact: provenance.exact_join,
        v2_archive_digest_valid,
        v3_archive_digest_valid,
        e5_archive_digest_valid,
        enacted_full_history_archive_digest: full_history.result_digest.clone(),
        enacted_full_history_archive_digest_valid,
        enacted_archived_candidate_hash_total_join_valid,
        enacted_archived_certified_vector_numerically_matched_post_seal,
        enacted_winner_nu_provenance_exact,
        enacted_nu_provenance_gap_count,
        v3_live_replay_valid: v3_live.valid,
        v3_live_replay_errors: v3_live.errors,
        e5_live_replay_valid: e5_live.valid,
        e5_live_replay_errors: e5_live.errors,
        frozen_archive_self_digest_comparators_are_bi0_regression_references: true,
        live_replay_diagnostics_are_not_bi0_acceptance_inputs: true,
        expected_complete_vector,
        observed_complete_vector,
        complete_vector_exact,
        expected_winner_hashes,
        observed_winner_hashes,
        winner_hashes_exact,
        expected_v3_discharger_census,
        observed_v3_discharger_census,
        v3_discharger_census_exact,
        expected_e5_projection,
        observed_e5_projection,
        e5_projection_exact,
        e5_membership_instance_ids_exact,
        enacted_finale_inventory_scope: finale.inventory_scope.clone(),
        enacted_finale_absolute_semantic_exhaustiveness_claimed: finale
            .absolute_semantic_exhaustiveness_claimed,
        enacted_finale_e5_class_complete: finale.e5_class_complete,
        enacted_finale_expressivity_gaps: finale.expressivity_gaps.clone(),
        enacted_finale_final_a3_inventory_count: finale.final_a3_inventory_count,
        enacted_finale_derivable_instance_count: finale.derivable_instance_count,
        enacted_finale_underdetermined_instance_ids: finale.underdetermined_instance_ids.clone(),
        enacted_finale_issuer_gap_instance_ids: finale.issuer_gap_instance_ids.clone(),
        enacted_finale_realization_gap_ids: finale.realization_gap_ids.clone(),
        bi0_passed,
        f_bi5_triggered: !bi0_passed,
        historical_outcome_comparators_used_only_after_independent_branch_seal: true,
        mutation_falsifiers: vec![
            "flip_any_prerequisite_replay_or_totality_field_then_replay_must_fail".to_owned(),
            "flip_any_complete_ledger_provenance_join_or_token_count_then_replay_must_fail"
                .to_owned(),
            "flip_BI0_or_any_exactness_verdict_then_replay_must_fail".to_owned(),
            "flip_any_vector_count_or_winner_digest_then_replay_must_fail".to_owned(),
            "flip_any_E5_projection_O16_or_F1_field_then_replay_must_fail".to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = regression_digest(&certificate);
    Ok(certificate)
}

/// Execute only the enacted BI-0 gate. This is the lawful diagnostic surface
/// when F-BI5 fires: it issues and seals the enacted branch, compares it with
/// the frozen record, and never starts a non-enacted branch.
pub fn issue_bi0_regression_gate() -> Result<BiRegressionCertificate, BiProgramError> {
    // Both adopted prerequisites replay before even the enacted branch is
    // issued. Thus failure cannot be converted into a partial BI-0 result and
    // no branch continuation can be used to infer either prerequisite.
    let prerequisites = load_and_replay_bi0_prerequisites()?;
    let cone = issue_certified_stage4_branch_cone()?;
    let v2: Phase5bReselectionBurn = serde_json::from_slice(V2_BURN_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let enacted_root = v2
        .stages
        .iter()
        .find(|row| row.stage == 4)
        .and_then(|row| row.winner.as_ref())
        .map(|winner| winner.candidate_hash.as_str())
        .ok_or_else(|| BiProgramError::Prerequisite("sealed Stage-4 root is absent".to_owned()))?;
    if !cone
        .branches
        .iter()
        .any(|branch| branch.candidate_hash == enacted_root)
    {
        return Err(BiProgramError::Prerequisite(
            "sealed enacted Stage-4 root is outside cone".to_owned(),
        ));
    }
    let enacted = issue_branch_from_cone(&cone, enacted_root)?;
    issue_regression_from_sealed_branch(&enacted, &prerequisites)
}

pub fn replay_bi_regression_certificate(certificate: &BiRegressionCertificate) -> BiReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != regression_digest(certificate) {
        errors.push("BI-0 regression certificate digest mismatch".to_owned());
        return BiReplay {
            valid: false,
            errors,
        };
    }
    let prerequisites = match load_and_replay_bi0_prerequisites() {
        Ok(value) => value,
        Err(error) => {
            errors.push(error.to_string());
            return BiReplay {
                valid: false,
                errors,
            };
        }
    };
    match issue_bi_branch_certificate(&certificate.enacted_branch_root_hash)
        .and_then(|branch| issue_regression_from_sealed_branch(&branch, &prerequisites))
    {
        Ok(expected) if &expected == certificate => {}
        Ok(_) => errors.push("BI-0 certificate differs from independent reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    BiReplay {
        valid: errors.is_empty(),
        errors,
    }
}

pub fn replay_bi_regression_json(bytes: &[u8]) -> BiReplay {
    match serde_json::from_slice::<BiRegressionCertificate>(bytes) {
        Ok(certificate) => replay_bi_regression_certificate(&certificate),
        Err(error) => BiReplay {
            valid: false,
            errors: vec![error.to_string()],
        },
    }
}

fn granularity(level: &str, passed: bool, exact_basis: impl Into<String>) -> BiGranularityAudit {
    granularity_with_verdict(
        level,
        if passed {
            BiGranularityVerdict::Passed
        } else {
            BiGranularityVerdict::Refuted
        },
        exact_basis,
    )
}

fn granularity_with_verdict(
    level: &str,
    verdict: BiGranularityVerdict,
    exact_basis: impl Into<String>,
) -> BiGranularityAudit {
    let passed = verdict == BiGranularityVerdict::Passed;
    let mut row = BiGranularityAudit {
        level: level.to_owned(),
        verdict,
        passed,
        exact_basis: exact_basis.into(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = granularity_digest(&row);
    row
}

fn granularity_digest(row: &BiGranularityAudit) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash(BI_CONE_CERTIFICATE_SCHEMA, "granularity-audit", &projection)
}

fn obligation_rows_equal(left: &BiObligationProfileRow, right: &BiObligationProfileRow) -> bool {
    left.stage == right.stage
        && left.required_packages == right.required_packages
        && left.structural_constructors == right.structural_constructors
        && left.live_obligation_count == right.live_obligation_count
        && left.total_typed_discharge == right.total_typed_discharge
        && left.debt_free == right.debt_free
}

fn compare_g2(left: &BiBranchCertificate, right: &BiBranchCertificate) -> BiPairComparison {
    let first_divergence = left
        .obligation_profile
        .iter()
        .zip(&right.obligation_profile)
        .find(|(a, b)| !obligation_rows_equal(a, b));
    let same_len = left.obligation_profile.len() == right.obligation_profile.len();
    let equal = same_len && first_divergence.is_none();
    let (first_divergence_stage, exact_context) = if let Some((a, b)) = first_divergence {
        (
            Some(a.stage.min(b.stage)),
            format!(
                "G2 Stage {}: left packages={:?}, constructors={:?}, live={}, total={}; right packages={:?}, constructors={:?}, live={}, total={}",
                a.stage,
                a.required_packages,
                a.structural_constructors,
                a.live_obligation_count,
                a.total_typed_discharge,
                b.required_packages,
                b.structural_constructors,
                b.live_obligation_count,
                b.total_typed_discharge,
            ),
        )
    } else if !same_len {
        (
            None,
            format!(
                "G2 profile length differs: left={}, right={}",
                left.obligation_profile.len(),
                right.obligation_profile.len()
            ),
        )
    } else {
        (
            None,
            "G2 exact obligation profiles agree at every reached post-fork window through each branch's inspected successor"
                .to_owned(),
        )
    };
    BiPairComparison {
        left_branch_root: left.branch_root_hash.clone(),
        right_branch_root: right.branch_root_hash.clone(),
        equal,
        first_divergence_stage,
        exact_context,
    }
}

fn numeric_fields_equal(left: &BiNumericLedgerRow, right: &BiNumericLedgerRow) -> bool {
    left.stage == right.stage
        && left.kappa == right.kappa
        && left.diagnostic_nu == right.diagnostic_nu
        && left.diagnostic_bar == right.diagnostic_bar
}

fn compare_g3(left: &BiBranchCertificate, right: &BiBranchCertificate) -> BiPairComparison {
    let first_divergence = left
        .numeric_ledger_stage5_onward
        .iter()
        .zip(&right.numeric_ledger_stage5_onward)
        .find(|(a, b)| !numeric_fields_equal(a, b));
    let same_len =
        left.numeric_ledger_stage5_onward.len() == right.numeric_ledger_stage5_onward.len();
    let equal = same_len && first_divergence.is_none();
    let (first_divergence_stage, exact_context) = if let Some((a, b)) = first_divergence {
        (
            Some(a.stage.min(b.stage)),
            format!(
                "G3 exact-certified Stage {}: left (kappa,nu,diagnostic_bar)=({},{},{:?}); right=({},{},{:?})",
                a.stage,
                a.kappa,
                a.diagnostic_nu,
                a.diagnostic_bar,
                b.kappa,
                b.diagnostic_nu,
                b.diagnostic_bar,
            ),
        )
    } else if !same_len {
        (
            None,
            format!(
                "G3 ledger length differs: left={}, right={}",
                left.numeric_ledger_stage5_onward.len(),
                right.numeric_ledger_stage5_onward.len()
            ),
        )
    } else {
        (
            None,
            "G3 exact-certified (kappa,nu) vectors and descriptive bar trajectories agree from Stage 5; every selected ledger row separately carries replayable act-local T-BI-NU1 provenance"
                .to_owned(),
        )
    };
    BiPairComparison {
        left_branch_root: left.branch_root_hash.clone(),
        right_branch_root: right.branch_root_hash.clone(),
        equal,
        first_divergence_stage,
        exact_context,
    }
}

fn all_pairs<T>(rows: &[T], compare: impl Fn(&T, &T) -> BiPairComparison) -> Vec<BiPairComparison> {
    let mut result = Vec::new();
    for left in 0..rows.len() {
        for right in (left + 1)..rows.len() {
            result.push(compare(&rows[left], &rows[right]));
        }
    }
    result
}

fn g1_pair_verdicts() -> Result<Vec<BiG1PairVerdict>, BiProgramError> {
    let r_t2: Rt2FutureHoleConfluenceV2Certificate = serde_json::from_slice(R_T2_BYTES)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    let mut projection = r_t2.clone();
    projection.result_digest.clear();
    let archive_digest_valid = r_t2.result_digest
        == tagged_hash(
            R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA,
            "certificate",
            &projection,
        );
    if !archive_digest_valid
        || r_t2.outcome != Rt2FutureHoleConfluenceV2Outcome::LawLevelConfluenceRefutedRungRt3Opened
        || !r_t2.r_t2_confluence_refuted
        || r_t2.all_stage5_scheme_sets_equivalent
        || !r_t2.order_reversal_invariant
        || r_t2.selected_candidate_hash.is_some()
    {
        return Err(BiProgramError::Prerequisite(format!(
            "authoritative frozen R-T2 G1 refutation failed its self-digest/internal audit (digest_valid={archive_digest_valid})"
        )));
    }
    let rows = r_t2
        .order_reversal_audit
        .forward_normalized_pair_verdicts
        .iter()
        .map(|row| BiG1PairVerdict {
            left_branch_root: row.unordered_candidate_pair.0.clone(),
            right_branch_root: row.unordered_candidate_pair.1.clone(),
            left_scheme_count: row.left_scheme_count,
            right_scheme_count: row.right_scheme_count,
            matched_scheme_count: row.matched_count,
            equivalent: row.equivalent,
        })
        .collect::<Vec<_>>();
    if rows.len() != 6
        || rows
            .iter()
            .any(|row| row.left_scheme_count != 5 || row.right_scheme_count != 5 || row.equivalent)
    {
        return Err(BiProgramError::Invariant(
            "R-T2 does not contain the exact six inequivalent 5-scheme pair verdicts".to_owned(),
        ));
    }
    Ok(rows)
}

fn contains_tree_stop(branches: &[BiBranchCertificate]) -> bool {
    branches.iter().any(|branch| {
        matches!(
            branch.continuation.outcome,
            BranchContinuationOutcome::HaltedMultipleGuardedDischargers { .. }
        )
    })
}

fn contains_expressivity_stop(branches: &[BiBranchCertificate]) -> bool {
    branches.iter().any(|branch| {
        matches!(
            branch.continuation.outcome,
            BranchContinuationOutcome::ExpressivityGap { .. }
                | BranchContinuationOutcome::ResourceLimitReached { .. }
        ) || branch
            .finale
            .as_ref()
            .is_some_and(|finale| !finale.expressivity_gaps.is_empty())
    })
}

fn assemble_cone(
    branches: &[BiBranchCertificate],
    regression: &BiRegressionCertificate,
) -> Result<BiConeCertificate, BiProgramError> {
    // Check the BI-0 capability before replaying even one branch.  Branch
    // replay performs independent continuation issuance and must not be
    // usable as a pre-gate oracle through the BI-4 assembler.
    let regression_replay = replay_bi_regression_certificate(regression);
    if !regression_replay.valid {
        return Err(BiProgramError::Prerequisite(format!(
            "BI-0 did not replay before BI-4: {:?}",
            regression_replay.errors
        )));
    }
    if !regression.bi0_passed {
        return Err(BiProgramError::Prerequisite(
            "F-BI5: BI-4 cannot compare branches under a replayable but failing BI-0 v2 gate"
                .to_owned(),
        ));
    }
    if branches.len() != 4 {
        return Err(BiProgramError::Prerequisite(format!(
            "BI-4 requires four sealed branches, received {}",
            branches.len()
        )));
    }
    let mut sealed = branches.to_vec();
    sealed.sort_by(|left, right| left.branch_root_hash.cmp(&right.branch_root_hash));
    let distinct = sealed
        .iter()
        .map(|row| row.branch_root_hash.as_str())
        .collect::<BTreeSet<_>>();
    if distinct.len() != 4 {
        return Err(BiProgramError::Prerequisite(
            "BI-4 received duplicate branch roots".to_owned(),
        ));
    }
    for branch in &sealed {
        let replay = replay_bi_branch_certificate(branch);
        if !replay.valid {
            return Err(BiProgramError::Prerequisite(format!(
                "branch {} was compared before it sealed: {:?}",
                branch.branch_root_hash, replay.errors
            )));
        }
    }
    if !sealed
        .iter()
        .any(|branch| branch.branch_root_hash == regression.enacted_branch_root_hash)
    {
        return Err(BiProgramError::Prerequisite(
            "BI-0 enacted root is outside the sealed cone".to_owned(),
        ));
    }

    let g1_pair_verdicts = g1_pair_verdicts()?;
    let g2_pair_comparisons = all_pairs(&sealed, compare_g2);
    let g3_pair_comparisons = all_pairs(&sealed, compare_g3);
    let continuation_measurement_stop = sealed.iter().any(|branch| {
        matches!(
            branch.continuation.outcome,
            BranchContinuationOutcome::HaltedMultipleGuardedDischargers { .. }
                | BranchContinuationOutcome::ExpressivityGap { .. }
                | BranchContinuationOutcome::ResourceLimitReached { .. }
        )
    });
    let g2_passed = g2_pair_comparisons.iter().all(|row| row.equal)
        && sealed.iter().all(|branch| {
            branch.one_live_demand_per_reached_guarded_stage
                && branch.every_reached_live_demand_totally_discharged
        });
    let every_selected_winner_exact_nu_certified = sealed
        .iter()
        .all(|branch| branch.every_selected_winner_exact_nu_certified);
    let g3_diagnostic_vectors_equal = g3_pair_comparisons.iter().all(|row| row.equal);
    let g3_passed = every_selected_winner_exact_nu_certified && g3_diagnostic_vectors_equal;
    let g4_passed = sealed.iter().all(|branch| {
        branch.local_g4_debt_free_halt_at_15
            && branch.local_semantic_o16_empty
            && branch.local_f1_excluded
            && branch.local_e5_class_complete
    });
    let g4_measurement_gap = continuation_measurement_stop
        || sealed.iter().any(|branch| {
            branch.local_debt_free_halt
                && branch.finale.as_ref().is_none_or(|finale| {
                    !finale.expressivity_gaps.is_empty() || !finale.e5_class_complete
                })
        });
    let g1 = granularity(
        "G1_scheme_identity",
        false,
        "R-T2 authoritative six-pair family-quotient comparison: matched counts 3/5,2/5,3/5,3/5,2/5,3/5; every pair inequivalent",
    );
    let g2 = if continuation_measurement_stop {
        granularity_with_verdict(
            "G2_obligation_profile",
            BiGranularityVerdict::NotReached,
            "at least one branch stopped at multiplicity, expressivity, or a resource boundary, so its truncated obligation profile is recorded but not promoted to G2",
        )
    } else {
        granularity(
            "G2_obligation_profile",
            g2_passed,
            "all six post-seal pairwise comparisons of exact reached post-fork A3 package/constructor/count/total-discharge profiles, including each inspected successor",
        )
    };
    let g3 = if continuation_measurement_stop {
        granularity_with_verdict(
            "G3_numeric_ledger",
            BiGranularityVerdict::NotReached,
            "at least one branch stopped before a complete ledger existed; truncated diagnostic vectors are never promoted to G3",
        )
    } else if every_selected_winner_exact_nu_certified {
        granularity(
            "G3_numeric_ledger",
            g3_passed,
            "all six post-seal pairwise comparisons of Stage 5-15 exact-certified (kappa,nu) and diagnostic bars; winner identity excluded from G3",
        )
    } else {
        granularity_with_verdict(
            "G3_numeric_ledger",
            BiGranularityVerdict::NotReached,
            format!(
                "the numeric vectors were compared (pairwise_equal={g3_diagnostic_vectors_equal}), but at least one selected branch ledger row failed replayable act-local T-BI-NU1 certification; no merely numeric equality is promoted to G3"
            ),
        )
    };
    let g4 = if g4_measurement_gap {
        granularity_with_verdict(
            "G4_debt_free_halt",
            BiGranularityVerdict::NotReached,
            "at least one continuation stopped at multiplicity/expressivity/resource or has a named branch-finale certification gap, so semantic successor-O/F1 cannot be promoted to G4",
        )
    } else {
        granularity(
            "G4_debt_free_halt",
            g4_passed,
            "four branch-local E-5-class certificates: halt=15, exact D partition, semantic O(16)=empty, F1 excluded",
        )
    };
    let outcome_zone = if contains_tree_stop(&sealed) {
        "Z-TREE"
    } else if contains_expressivity_stop(&sealed) || g4_measurement_gap {
        "Z-STOP"
    } else if !every_selected_winner_exact_nu_certified {
        "Z-STOP"
    } else if !g4_passed || !g3_passed {
        "Z-SPLIT"
    } else if !g2_passed {
        "Z-ISO"
    } else {
        "Z-CONE"
    }
    .to_owned();
    let first_g1 = g1_pair_verdicts
        .iter()
        .min_by(|left, right| {
            (&left.left_branch_root, &left.right_branch_root)
                .cmp(&(&right.left_branch_root, &right.right_branch_root))
        })
        .expect("six pairs");
    let first_divergence_published_verbatim = Some(format!(
        "G1 first divergence at the Stage-5 successor surface: {} versus {}; certified family-quotient matching {}/{} (right set size {}), equivalent=false. This R-T2 divergence is retained without repair or re-quotienting.",
        first_g1.left_branch_root,
        first_g1.right_branch_root,
        first_g1.matched_scheme_count,
        first_g1.left_scheme_count,
        first_g1.right_scheme_count,
    ));
    let branches_summary = sealed
        .iter()
        .map(|branch| BiConeBranchSummary {
            branch_root_hash: branch.branch_root_hash.clone(),
            branch_certificate_digest: branch.result_digest.clone(),
            r_t1_class_id: branch.r_t1_class_id.clone(),
            economy_probe_class_key: branch.economy_probe_class_key.clone(),
            debt_free_halt: branch.local_debt_free_halt,
            halt_stage: branch.local_halt_stage,
            successor_stage: branch.local_successor_stage,
            semantic_successor_o_empty: branch.local_semantic_successor_o_empty,
            halt_at_15: branch.local_g4_debt_free_halt_at_15,
            semantic_o16_empty: branch.local_semantic_o16_empty,
            f1_excluded: branch.local_f1_excluded,
            sum_kappa: branch.complete_sum_kappa,
            diagnostic_sum_nu: branch.complete_diagnostic_sum_nu,
        })
        .collect::<Vec<_>>();
    let mut promoted_to_cone_level = Vec::new();
    if g2.passed {
        promoted_to_cone_level.push(
            "G2_obligation_profile_and_one-demand-per-stage_shape_from_Stage4_fork_to_halt"
                .to_owned(),
        );
    }
    if g3.passed {
        promoted_to_cone_level.push(
            "G3_stage5_to15_kappa_nu_vector_complete_sums_and_diagnostic_bar_trajectory".to_owned(),
        );
    }
    if g4.passed {
        promoted_to_cone_level.extend([
            "G4_debt_free_halt_at_15".to_owned(),
            "semantic_O16_empty_at_full_instance_granularity_relative_to_the_adopted_A3_inventory"
                .to_owned(),
            "Guard_Rail_F1_excluded_on_every_branch".to_owned(),
            "Theorem12_full_instance_granularity_on_the_cone_relative_to_the_adopted_A3_inventory"
                .to_owned(),
        ]);
    }
    let bridge_prerequisite_bi4_satisfied = regression.bi0_passed
        && g3.passed
        && matches!(outcome_zone.as_str(), "Z-CONE" | "Z-ISO" | "Z-SPLIT");
    let mut certificate = BiConeCertificate {
        schema: BI_CONE_CERTIFICATE_SCHEMA.to_owned(),
        date: BI_PROGRAM_DATE.to_owned(),
        source_bindings: cone_bindings(),
        option_b_replayed: true,
        all_four_branch_certificates_replayed_before_comparison: true,
        branch_count: sealed.len(),
        enacted_branch_root_hash: regression.enacted_branch_root_hash.clone(),
        bi0_certificate_digest: regression.result_digest.clone(),
        bi0_passed: regression.bi0_passed,
        branches: branches_summary,
        g1_pair_verdicts,
        g2_pair_comparisons,
        g3_pair_comparisons,
        g1,
        g2,
        g3,
        g4,
        outcome_zone,
        first_divergence_published_verbatim,
        branch_index_disposition: BiBranchIndexDisposition {
            enacted_branch_identification_status:
                "indexical_testimony_only_not_law_not_selector_not_comparison_baseline".to_owned(),
            remains_branch_indexed: vec![
                "Stage4_root_act_identity".to_owned(),
                "G1_Stage5_successor_scheme_sets_and_exact_derivations".to_owned(),
                "act_local_provenance_certificate_identity_for_each_branch_specific_act".to_owned(),
                "branch_local_winner_digests_where_they_differ_as_terms".to_owned(),
            ],
            promoted_to_cone_level,
        },
        bridge_prerequisite_bi4_satisfied,
        bridge_executed: false,
        no_divergence_suppressed_averaged_or_repaired: true,
        class_representative_substitution_used: false,
        enacted_branch_privileged_beyond_indexical_testimony: false,
        mutation_falsifiers: vec![
            "flip_any_G1_to_G4_verdict_or_outcome_zone_then_replay_must_fail".to_owned(),
            "flip_any_branch_count_winner_digest_kappa_nu_or_bar_then_replay_must_fail".to_owned(),
            "flip_any_O16_F1_or_branch_index_disposition_then_replay_must_fail".to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = cone_digest(&certificate);
    Ok(certificate)
}

/// BI-4 is the only cross-branch comparator.  Every input certificate and BI-0
/// replays before the first G2/G3/G4 comparison is formed.
pub fn issue_bi_cone_certificate(
    sealed_branches: &[BiBranchCertificate],
    regression: &BiRegressionCertificate,
) -> Result<BiConeCertificate, BiProgramError> {
    assemble_cone(sealed_branches, regression)
}

pub fn replay_bi_cone_certificate(
    certificate: &BiConeCertificate,
    sealed_regression: &BiRegressionCertificate,
) -> BiReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != cone_digest(certificate) {
        errors.push("BI-4 cone certificate digest mismatch".to_owned());
        return BiReplay {
            valid: false,
            errors,
        };
    }
    let regression_replay = replay_bi_regression_certificate(sealed_regression);
    if !regression_replay.valid
        || !sealed_regression.bi0_passed
        || certificate.bi0_certificate_digest != sealed_regression.result_digest
    {
        errors.extend(regression_replay.errors);
        errors.push(
            "BI-4 replay requires the separately sealed, passing BI-0 v2 certificate".to_owned(),
        );
        return BiReplay {
            valid: false,
            errors,
        };
    }
    let mut branches = Vec::new();
    for summary in &certificate.branches {
        match issue_bi_branch_certificate(&summary.branch_root_hash) {
            Ok(branch) => branches.push(branch),
            Err(error) => {
                errors.push(error.to_string());
                return BiReplay {
                    valid: false,
                    errors,
                };
            }
        }
    }
    match assemble_cone(&branches, sealed_regression) {
        Ok(expected) if &expected == certificate => {}
        Ok(_) => errors.push("BI-4 certificate differs from independent reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    BiReplay {
        valid: errors.is_empty(),
        errors,
    }
}

pub fn replay_bi_cone_json(bytes: &[u8], sealed_regression: &BiRegressionCertificate) -> BiReplay {
    match serde_json::from_slice::<BiConeCertificate>(bytes) {
        Ok(certificate) => replay_bi_cone_certificate(&certificate, sealed_regression),
        Err(error) => BiReplay {
            valid: false,
            errors: vec![error.to_string()],
        },
    }
}

/// Issue BI-1 and BI-4 only after the caller supplies a separately sealed,
/// replayable, passing BI-0 v2 certificate. This function cannot create or
/// rewrite BI-0 and has no fallback combined workflow.
pub fn issue_bi_program_after_bi0(
    sealed_regression: &BiRegressionCertificate,
) -> Result<BiProgramBundle, BiProgramError> {
    let regression_replay = replay_bi_regression_certificate(sealed_regression);
    if !regression_replay.valid {
        return Err(BiProgramError::Prerequisite(format!(
            "sealed BI-0 v2 did not replay before BI-1: {:?}",
            regression_replay.errors
        )));
    }
    if !sealed_regression.bi0_passed {
        return Err(BiProgramError::Prerequisite(
            "F-BI5: sealed BI-0 v2 is not passing; BI-1/BI-4 remain closed".to_owned(),
        ));
    }
    let cone_seed = issue_certified_stage4_branch_cone()?;
    if !cone_seed
        .branches
        .iter()
        .any(|branch| branch.candidate_hash == sealed_regression.enacted_branch_root_hash)
    {
        return Err(BiProgramError::Prerequisite(
            "sealed BI-0 enacted Stage-4 root is outside the current certified cone".to_owned(),
        ));
    }
    let mut branches = Vec::with_capacity(cone_seed.branches.len());
    for seed in &cone_seed.branches {
        branches.push(issue_branch_from_cone(&cone_seed, &seed.candidate_hash)?);
    }
    branches.sort_by(|left, right| left.branch_root_hash.cmp(&right.branch_root_hash));
    let cone = assemble_cone(&branches, sealed_regression)?;
    Ok(BiProgramBundle {
        branches,
        regression: sealed_regression.clone(),
        cone,
    })
}

pub fn render_bi_branch_markdown(certificate: &BiBranchCertificate) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# BI branch {} result\n\n",
        certificate.branch_digest_prefix
    ));
    out.push_str(&format!(
        "**Date:** {}. **Scope:** branch-indexed until BI-4. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str(&format!(
        "Root `{}` was continued without consuming enacted v2/v3 or E-5 outcomes. The terminus is `{:?}`. Its generic debt-free finale reports successor O empty = **{}**, relative to the finale's serialized inventory scope. Registered local G4 (halt 15, semantic O(16) empty, F1 excluded, E-5 complete) is **{}**; semantic O(16) empty is **{}** and F1 excluded is **{}**.\n\n",
        certificate.branch_root_hash,
        certificate.continuation.outcome,
        certificate.local_semantic_successor_o_empty,
        certificate.local_g4_debt_free_halt_at_15,
        certificate.local_semantic_o16_empty,
        certificate.local_f1_excluded,
    ));
    out.push_str("| Stage | A3 demand | Cone admitted | Dischargers | Winner | kappa | Diagnostic formula nu | Diagnostic bar |\n");
    out.push_str("|---:|---|---:|---:|---|---:|---:|---|\n");
    for stage in &certificate.continuation.stages {
        let winner = stage.winner.as_ref();
        out.push_str(&format!(
            "| {} | `{}` | {} | {} | `{}` | {} | {} | `{}` |\n",
            stage.stage,
            stage.demand.a3_required_packages.join(","),
            stage.cone_admitted,
            stage.discharger_count,
            winner
                .map(|row| digest_prefix(&row.candidate_hash))
                .unwrap_or_else(|| "none".to_owned()),
            winner.map(|row| row.kappa).unwrap_or(0),
            winner.map(|row| row.semantic_nu).unwrap_or(0),
            stage.diagnostic_bar,
        ));
    }
    let terminal = &certificate.continuation.terminal_demand;
    out.push_str(&format!(
        "| {} | `{}` | 0 | 0 | `debt inspection` | 0 | 0 | `n/a` |\n\n",
        terminal.stage,
        terminal.a3_required_packages.join(",")
    ));
    if let Some(finale) = &certificate.finale {
        out.push_str("## Branch-local E-5 finale\n\n");
        out.push_str(&format!(
            "The adopted-A3-relative inventory has {} instances: {} unary, {} direct chronological, {} pointwise chronological, {} higher, and {} structural. The historical ledger has {} structural registrations and {} realizations. D derives {}/{} instances; the underdetermined set is empty: **{}**. The Stage-3 wrinkle replayed: **{}**. Inventory scope: `{}`. Absolute semantic exhaustiveness claimed: **{}**. Theorem-12 scope: `{}`; broader absolute Theorem-12 claimed: **{}**.\n\n",
            finale.final_a3_inventory_count,
            finale.final_unary_count,
            finale.final_direct_chronological_count,
            finale.final_pointwise_chronological_count,
            finale.final_higher_count,
            finale.final_structural_count,
            finale.structural_registration_count,
            finale.structural_realization_count,
            finale.derivable_instance_count,
            finale.final_a3_inventory_count,
            finale.underdetermined_instance_ids.is_empty(),
            finale.stage3_wrinkle_reproduced,
            finale.inventory_scope,
            finale.absolute_semantic_exhaustiveness_claimed,
            finale.theorem12_scope,
            finale.broader_absolute_theorem12_claimed,
        ));
    } else if let Some(reason) = &certificate.finale_not_run_reason {
        out.push_str(&format!("## Finale not run\n\n{reason}\n\n"));
    }
    out.push_str("## Isolation and falsifiers\n\n");
    out.push_str(&format!(
        "Enacted outcomes consumed for acceptance, selection, or steering: **{}**. Bars and diagnostic formulas used as selectors: **{}**. Candidate assessments: **{}**; discharge-semantic gaps: **{}**; diagnostic-only candidate-assessment provenance rows: **{}**; every selected winner exact-nu certified: **{}**. The complete historical ledger carries exact replayable provenance on **{}/15** entries.\n\n",
        certificate.enacted_outcomes_consumed_for_acceptance_selection_or_steering,
        !certificate.continuation.bar_never_used_as_gate_or_selector
            || !certificate.continuation.score_never_used_as_selector,
        certificate.candidate_assessment_count,
        certificate.semantic_evidence_gap_count,
        certificate.exact_nu_provenance_failure_count,
        certificate.every_selected_winner_exact_nu_certified,
        certificate.complete_ledger_exact_nu_provenance_count,
    ));
    for falsifier in &certificate.mutation_falsifiers {
        out.push_str(&format!("- `{falsifier}`\n"));
    }
    out
}

pub fn render_bi_regression_markdown(certificate: &BiRegressionCertificate) -> String {
    let mut out = String::new();
    out.push_str("# BI regression result\n\n");
    out.push_str(&format!(
        "**Date:** {}. **BI-0:** **{}**. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.bi0_passed, certificate.result_digest
    ));
    out.push_str(&format!(
        "The adopted slot-map and T-BI-NU1 prerequisites replayed before the enacted branch was issued: **{}**. The sealed v2 artifact was then read only to identify the authorized Stage-4 branch root `{}`. That root's independently issued branch certificate was replayed before any historical outcome comparator (remaining v2/v3 rows, E-5 result, or full-history totals) was consumed. Non-enacted branches issued before BI-0: **{}**; BI-0 is a hard gate before BI-1: **{}**. The diagnostic formula (stage, nu, kappa) vector is exact: **{}**; all fifteen winner digests are exact: **{}**; the v3 Stage 8-15 discharger census is exact: **{}**; the E-5 projection and membership IDs are exact: **{} / {}**. After that seal, the archived full-history certificate matched every candidate hash, kappa, and certified total: **{}**; that archive remains corroborating regression evidence, never provenance authority.\n\n",
        certificate.prerequisites_replayed_before_enacted_branch_issuance,
        certificate.enacted_branch_root_hash,
        certificate.non_enacted_branches_issued_before_bi0,
        certificate.bi0_is_hard_gate_before_bi1,
        certificate.complete_vector_exact,
        certificate.winner_hashes_exact,
        certificate.v3_discharger_census_exact,
        certificate.e5_projection_exact,
        certificate.e5_membership_instance_ids_exact,
        certificate.enacted_archived_certified_vector_numerically_matched_post_seal,
    ));
    out.push_str(&format!(
        "Exact act-local winner provenance is now a BI-0 premise: **{}** (complete-ledger gaps **{}**). The branch ledger joined T-BI-NU1 on **{}/{}** acts over the exact Stage-1-through-15 surface **{}**; the full package join is **{}**, with **{} / {}** authoritative ordinary-family tokens and exact count/vector equality **{}**.\n\n",
        certificate.enacted_winner_nu_provenance_exact,
        certificate.enacted_nu_provenance_gap_count,
        certificate.enacted_complete_ledger_exact_provenance_count,
        certificate.enacted_complete_ledger_entry_count,
        certificate.enacted_complete_ledger_stage_surface_exact,
        certificate.enacted_complete_ledger_provenance_join_exact,
        certificate.enacted_complete_ledger_authoritative_token_count,
        certificate.t_bi_nu1_authoritative_token_count,
        certificate.enacted_complete_ledger_authoritative_token_count_exact,
    ));
    out.push_str("| BI-0 prerequisite | Replay | Required result |\n");
    out.push_str("|---|---:|---|\n");
    out.push_str(&format!(
        "| chronological slot map | {} | F-SM1={}, F-SM2={}, zero-charge={}, former 9={}, Stage-16 89 IDs={} |\n",
        certificate.chronological_slot_map_replay_valid,
        certificate.chronological_slot_map_f_sm1_passed,
        certificate.chronological_slot_map_f_sm2_passed,
        certificate.chronological_slot_map_every_declaration_zero_charge,
        certificate.chronological_slot_map_former_nine_exact_and_derived,
        certificate.chronological_slot_map_stage16_89_id_surface_exact,
    ));
    out.push_str(&format!(
        "| T-BI-NU1 act-local provenance | {} | acts={}/15, packages={}/15 authoritative={}, token hashes unique={}, fillers=13 covered={}, F-AL1={} |\n\n",
        certificate.t_bi_nu1_replay_valid,
        certificate.enacted_complete_ledger_entry_count,
        certificate.t_bi_nu1_package_count,
        certificate.t_bi_nu1_all_fifteen_packages_authoritative,
        certificate.t_bi_nu1_authoritative_token_hashes_globally_unique,
        certificate.t_bi_nu1_all_thirteen_structural_filler_classes_covered,
        certificate.t_bi_nu1_f_al1_passed,
    ));
    out.push_str(&format!(
        "The plan authorizes byte-stable, self-digested historical artifacts as BI-0 regression references: **{}**. Live replay diagnostics are recorded but excluded from BI-0 acceptance: **{}**.\n\n",
        certificate.frozen_archive_self_digest_comparators_are_bi0_regression_references,
        certificate.live_replay_diagnostics_are_not_bi0_acceptance_inputs,
    ));
    out.push_str("| Archive | Self-digest | Live replay | Status in BI-0 |\n");
    out.push_str("|---|---:|---:|---|\n");
    out.push_str(&format!(
        "| v2 sealed prefix | {} | n/a | Stage-4 identity and Stages 1-7 comparator |\n",
        certificate.v2_archive_digest_valid
    ));
    out.push_str(&format!(
        "| v3 history | {} | {} | Frozen logical comparator; live drift is disclosed, not a steering input |\n",
        certificate.v3_archive_digest_valid, certificate.v3_live_replay_valid
    ));
    out.push_str(&format!(
        "| E-5 dependent-context | {} | {} | Frozen logical comparator; live drift is disclosed, not a branch premise |\n",
        certificate.e5_archive_digest_valid, certificate.e5_live_replay_valid
    ));
    out.push_str(&format!(
        "| Full-history exact nu | {} | n/a | Enacted candidate-hash/total join only, loaded post-seal |\n\n",
        certificate.enacted_full_history_archive_digest_valid
    ));
    if !certificate.v3_live_replay_errors.is_empty() {
        out.push_str(&format!(
            "v3 live replay diagnostics: `{}`\n\n",
            certificate.v3_live_replay_errors.join(" | ")
        ));
    }
    if !certificate.e5_live_replay_errors.is_empty() {
        out.push_str(&format!(
            "E-5 live replay diagnostics: `{}`\n\n",
            certificate.e5_live_replay_errors.join(" | ")
        ));
    }
    out.push_str("## Enacted finale gate\n\n");
    out.push_str(&format!(
        "The independently rebuilt finale is E-5-class complete: **{}**. Its inventory scope is `{}`; absolute semantic exhaustiveness claimed: **{}**. Its D membership partition is **{} / {} Derivable**, **{} Underdetermined**, and **{} IssuerGap**. It contains **{}** named expressivity gaps, including **{}** structural-realization gaps.\n\n",
        certificate.enacted_finale_e5_class_complete,
        certificate.enacted_finale_inventory_scope,
        certificate.enacted_finale_absolute_semantic_exhaustiveness_claimed,
        certificate.enacted_finale_derivable_instance_count,
        certificate.enacted_finale_final_a3_inventory_count,
        certificate.enacted_finale_underdetermined_instance_ids.len(),
        certificate.enacted_finale_issuer_gap_instance_ids.len(),
        certificate.enacted_finale_expressivity_gaps.len(),
        certificate.enacted_finale_realization_gap_ids.len(),
    ));
    for gap in &certificate.enacted_finale_expressivity_gaps {
        out.push_str(&format!("- `{gap}`\n"));
    }
    if !certificate
        .enacted_finale_underdetermined_instance_ids
        .is_empty()
    {
        out.push_str("\nDemand instances classified `Underdetermined`:\n\n");
        for instance in &certificate.enacted_finale_underdetermined_instance_ids {
            out.push_str(&format!("- `{instance}`\n"));
        }
    }
    if !certificate
        .enacted_finale_issuer_gap_instance_ids
        .is_empty()
    {
        out.push_str(
            "\nDemand instances left as issuer gaps (not semantic `Underdetermined` verdicts):\n\n",
        );
        for instance in &certificate.enacted_finale_issuer_gap_instance_ids {
            out.push_str(&format!("- `{instance}`\n"));
        }
    }
    if !certificate.bi0_passed {
        out.push_str(
            "\n**F-BI5 fired.** This certificate records the regression failure only. No non-enacted branch, cone verdict, branch-index promotion, bridge, or final certificate is authorized.\n",
        );
    }
    out
}

pub fn render_bi_cone_markdown(certificate: &BiConeCertificate) -> String {
    let mut out = String::new();
    out.push_str("# BI cone report\n\n");
    out.push_str(&format!(
        "**Date:** {}. **Outcome zone:** **{}**. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.outcome_zone, certificate.result_digest
    ));
    out.push_str("All four branch certificates replayed before comparison. The enacted branch is retained only as indexical testimony; it was not a law-level baseline or selector.\n\n");
    out.push_str("| Granularity | Verdict | Exact basis |\n");
    out.push_str("|---|---|---|\n");
    for row in [
        &certificate.g1,
        &certificate.g2,
        &certificate.g3,
        &certificate.g4,
    ] {
        out.push_str(&format!(
            "| {} | `{:?}` | {} |\n",
            row.level, row.verdict, row.exact_basis
        ));
    }
    out.push_str(
        "\n| Stage-4 root | Debt-free halt | Halt / successor | Successor O empty | Local halt 15 | O(16) empty | F1 excluded | sum kappa | diagnostic formula sum nu |\n",
    );
    out.push_str("|---|---:|---|---:|---:|---:|---:|---:|---:|\n");
    for branch in &certificate.branches {
        out.push_str(&format!(
            "| `{}` | {} | `{:?} / {:?}` | {} | {} | {} | {} | {} | {} |\n",
            digest_prefix(&branch.branch_root_hash),
            branch.debt_free_halt,
            branch.halt_stage,
            branch.successor_stage,
            branch.semantic_successor_o_empty,
            branch.halt_at_15,
            branch.semantic_o16_empty,
            branch.f1_excluded,
            branch.sum_kappa,
            branch.diagnostic_sum_nu,
        ));
    }
    out.push_str("\n## Published divergence\n\n");
    out.push_str(
        certificate
            .first_divergence_published_verbatim
            .as_deref()
            .unwrap_or("No registered divergence."),
    );
    out.push_str("\n\n## Branch-index disposition\n\n");
    out.push_str(&format!(
        "Enacted identification: `{}`.\n\nPromoted to cone level:\n\n",
        certificate
            .branch_index_disposition
            .enacted_branch_identification_status
    ));
    for claim in &certificate.branch_index_disposition.promoted_to_cone_level {
        out.push_str(&format!("- `{claim}`\n"));
    }
    out.push_str("\nRemains branch-indexed:\n\n");
    for claim in &certificate.branch_index_disposition.remains_branch_indexed {
        out.push_str(&format!("- `{claim}`\n"));
    }
    out.push_str(&format!(
        "\nBI-4 bridge prerequisite satisfied: **{}**. The bridge was not executed by this artifact: **{}**.\n",
        certificate.bridge_prerequisite_bi4_satisfied, certificate.bridge_executed
    ));
    out
}

fn branch_json_name(prefix: &str) -> String {
    format!("BI_BRANCH_V2_{prefix}_CERTIFICATE.json")
}

fn branch_md_name(prefix: &str) -> String {
    format!("BI_BRANCH_V2_{prefix}_RESULT.md")
}

fn write_create_new(path: &Path, bytes: &[u8]) -> Result<(), BiProgramError> {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| BiProgramError::Io(format!("{}: {error}", path.display())))?;
    output
        .write_all(bytes)
        .map_err(|error| BiProgramError::Io(format!("{}: {error}", path.display())))
}

fn json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, BiProgramError> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| BiProgramError::Json(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn bundle_paths(output_dir: &Path, bundle: &BiProgramBundle) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for branch in &bundle.branches {
        paths.push(output_dir.join(branch_json_name(&branch.branch_digest_prefix)));
        paths.push(output_dir.join(branch_md_name(&branch.branch_digest_prefix)));
    }
    paths.extend([
        output_dir.join(BI_CONE_V2_CERTIFICATE_NAME),
        output_dir.join(BI_CONE_V2_REPORT_NAME),
    ]);
    paths
}

fn read_and_replay_sealed_bi0_v2(
    output_dir: &Path,
) -> Result<BiRegressionCertificate, BiProgramError> {
    let path = output_dir.join(BI_REGRESSION_V2_CERTIFICATE_NAME);
    let bytes = std::fs::read(&path)
        .map_err(|error| BiProgramError::Io(format!("{}: {error}", path.display())))?;
    let regression: BiRegressionCertificate =
        serde_json::from_slice(&bytes).map_err(|error| BiProgramError::Json(error.to_string()))?;
    let replay = replay_bi_regression_certificate(&regression);
    if !replay.valid {
        return Err(BiProgramError::EmittedReplay(format!(
            "sealed BI-0 v2: {:?}",
            replay.errors
        )));
    }
    if !regression.bi0_passed {
        return Err(BiProgramError::Prerequisite(
            "F-BI5: sealed BI-0 v2 is replayable but failed; BI-1/BI-4 remain closed".to_owned(),
        ));
    }
    Ok(regression)
}

/// Runtime-read and replay the separately emitted BI-0 v2 gate, then issue
/// BI-1 and BI-4. The gate file is never created, overwritten, or rewritten by
/// this function.
pub fn emit_bi_program_after_bi0_create_new(
    output_dir: &Path,
) -> Result<BiProgramBundle, BiProgramError> {
    let regression = read_and_replay_sealed_bi0_v2(output_dir)?;
    let bundle = issue_bi_program_after_bi0(&regression)?;
    for branch in &bundle.branches {
        let replay = replay_bi_branch_certificate(branch);
        if !replay.valid {
            return Err(BiProgramError::EmittedReplay(format!(
                "branch {}: {:?}",
                branch.branch_root_hash, replay.errors
            )));
        }
    }
    let cone_replay = replay_bi_cone_certificate(&bundle.cone, &bundle.regression);
    if !cone_replay.valid {
        return Err(BiProgramError::EmittedReplay(format!(
            "BI-4: {:?}",
            cone_replay.errors
        )));
    }
    std::fs::create_dir_all(output_dir).map_err(|error| BiProgramError::Io(error.to_string()))?;
    let paths = bundle_paths(output_dir, &bundle);
    if let Some(existing) = paths.iter().find(|path| path.exists()) {
        return Err(BiProgramError::Io(format!(
            "create-new target already exists: {}",
            existing.display()
        )));
    }
    for branch in &bundle.branches {
        write_create_new(
            &output_dir.join(branch_json_name(&branch.branch_digest_prefix)),
            &json_bytes(branch)?,
        )?;
        let mut report = render_bi_branch_markdown(branch).into_bytes();
        report.push(b'\n');
        write_create_new(
            &output_dir.join(branch_md_name(&branch.branch_digest_prefix)),
            &report,
        )?;
    }
    write_create_new(
        &output_dir.join(BI_CONE_V2_CERTIFICATE_NAME),
        &json_bytes(&bundle.cone)?,
    )?;
    let mut cone_report = render_bi_cone_markdown(&bundle.cone).into_bytes();
    cone_report.push(b'\n');
    write_create_new(&output_dir.join(BI_CONE_V2_REPORT_NAME), &cone_report)?;
    Ok(bundle)
}

/// Emit only the replayable BI-0 result with create-new semantics. Unlike the
/// complete emitter, this remains available when the gate fails and writes no
/// branch or cone artifact.
pub fn emit_bi0_regression_create_new(
    output_dir: &Path,
) -> Result<BiRegressionCertificate, BiProgramError> {
    let regression = issue_bi0_regression_gate()?;
    let replay = replay_bi_regression_certificate(&regression);
    if !replay.valid {
        return Err(BiProgramError::EmittedReplay(format!(
            "BI-0: {:?}",
            replay.errors
        )));
    }
    std::fs::create_dir_all(output_dir).map_err(|error| BiProgramError::Io(error.to_string()))?;
    let certificate_path = output_dir.join(BI_REGRESSION_V2_CERTIFICATE_NAME);
    let report_path = output_dir.join(BI_REGRESSION_V2_REPORT_NAME);
    for path in [&certificate_path, &report_path] {
        if path.exists() {
            return Err(BiProgramError::Io(format!(
                "create-new target already exists: {}",
                path.display()
            )));
        }
    }
    write_create_new(&certificate_path, &json_bytes(&regression)?)?;
    let mut report = render_bi_regression_markdown(&regression).into_bytes();
    report.push(b'\n');
    write_create_new(&report_path, &report)?;
    Ok(regression)
}

pub fn replay_bi0_regression_directory(output_dir: &Path) -> BiReplay {
    match std::fs::read(output_dir.join(BI_REGRESSION_V2_CERTIFICATE_NAME)) {
        Ok(bytes) => replay_bi_regression_json(&bytes),
        Err(error) => BiReplay {
            valid: false,
            errors: vec![error.to_string()],
        },
    }
}

pub fn replay_bi_program_directory(output_dir: &Path) -> BiReplay {
    let mut errors = Vec::new();
    let regression = match read_and_replay_sealed_bi0_v2(output_dir) {
        Ok(value) => value,
        Err(error) => {
            return BiReplay {
                valid: false,
                errors: vec![error.to_string()],
            };
        }
    };
    let cone_bytes = match std::fs::read(output_dir.join(BI_CONE_V2_CERTIFICATE_NAME)) {
        Ok(bytes) => bytes,
        Err(error) => {
            return BiReplay {
                valid: false,
                errors: vec![error.to_string()],
            };
        }
    };
    let cone: BiConeCertificate = match serde_json::from_slice(&cone_bytes) {
        Ok(value) => value,
        Err(error) => {
            return BiReplay {
                valid: false,
                errors: vec![error.to_string()],
            };
        }
    };
    let cone_replay = replay_bi_cone_certificate(&cone, &regression);
    errors.extend(cone_replay.errors);
    for summary in &cone.branches {
        let path = output_dir.join(branch_json_name(&digest_prefix(&summary.branch_root_hash)));
        match std::fs::read(&path) {
            Ok(bytes) => match serde_json::from_slice::<BiBranchCertificate>(&bytes) {
                Ok(branch) => {
                    if branch.result_digest != summary.branch_certificate_digest {
                        errors.push(format!("{} does not join the cone summary", path.display()));
                    }
                    errors.extend(replay_bi_branch_certificate(&branch).errors);
                }
                Err(error) => errors.push(error.to_string()),
            },
            Err(error) => errors.push(error.to_string()),
        }
    }
    BiReplay {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v2_paths_cannot_alias_preserved_v1_outputs() {
        assert_eq!(
            branch_json_name("deadbeef"),
            "BI_BRANCH_V2_deadbeef_CERTIFICATE.json"
        );
        assert_eq!(
            branch_md_name("deadbeef"),
            "BI_BRANCH_V2_deadbeef_RESULT.md"
        );
        assert_eq!(
            BI_REGRESSION_V2_CERTIFICATE_NAME,
            "BI_REGRESSION_V2_CERTIFICATE.json"
        );
        assert_eq!(BI_REGRESSION_V2_REPORT_NAME, "BI_REGRESSION_V2_RESULT.md");
        assert_eq!(BI_CONE_V2_CERTIFICATE_NAME, "BI_CONE_V2_CERTIFICATE.json");
        assert_eq!(BI_CONE_V2_REPORT_NAME, "BI_CONE_V2_RESULT.md");
        assert!(!BI_REGRESSION_V2_CERTIFICATE_NAME.eq("BI_REGRESSION_CERTIFICATE.json"));
    }

    #[test]
    fn v3_prerequisite_failures_close_bi0_before_burned_authority() {
        let error = load_and_replay_bi0_prerequisites()
            .expect_err("the replay-valid negative v3 prerequisites must close BI-0");
        let message = error.to_string();
        assert!(message.contains("versioned BI-0 prerequisites deny authority"));
        assert!(message.contains("F-SM1=false"));
        assert!(message.contains("F-AL1=false"));
        assert!(message.contains("burned v1/v2 artifacts were not read as authority"));
    }
}
