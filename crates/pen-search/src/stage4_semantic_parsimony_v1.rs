//! Verdict-blind Stage-4 semantic parsimony audit.
//!
//! The complete nonempty geometry is enumerated live from the guarded Stage-4
//! cone; no expected root count, expected kappa, or R-T2 outcome-shaped
//! artifact contributes to that enumeration.  Each retained strict candidate
//! then earns registry-erased B1/B2 and prefix-generic B3 evidence.
//! Structural `(kappa, nu) = (3, 5)` testimony, the frozen R-T2 successor
//! comparison, and the enacted-history digest are read strictly after the
//! semantic parsimony result is sealed.  The theorem issuer has no
//! branch-continuation or artifact-write capability; this module deliberately
//! exposes a separate outer create-new serializer.

use crate::act_local_semantic_provenance_v5::{
    issue_act_local_semantic_sequence_v5, replay_act_local_semantic_sequence_v5,
};
use crate::bi0_semantic_register_v5::{
    BI0_SEMANTIC_REGISTER_V5_SCHEMA, Bi0SemanticRegisterV5Certificate,
    replay_bi0_semantic_register_v5_certificate,
};
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::naturality_orbit_transport::{
    issue_stage4_r_t1_orbit_audit, replay_stage4_r_t1_orbit_audit,
};
use crate::phase5b_reselection_v2::{PHASE5B_RESELECTION_BURN_SCHEMA, Phase5bReselectionBurn};
use crate::r_t2_future_hole_confluence_v2::{
    R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA, Rt2FutureHoleConfluenceV2Certificate,
    Rt2V2PairwiseSchemeSetComparison, replay_archived_stage4_fork_projection,
};
use crate::t_bi_intrinsic_isolation_v3::issue_replayed_t_bi_intrinsic_isolation_v3_context;
use pen_core::canonical::canonical_key_telescope;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_type::admissibility::{
    AdmissibilityMode, passes_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA: &str = "stage4-semantic-parsimony-verdict-blind-v1";
pub const STAGE4_SEMANTIC_PARSIMONY_V1_DATE: &str = "2026-07-22";
pub const STAGE4_SEMANTIC_PARSIMONY_V1_THEOREM_ID: &str =
    "T-SP4-v1-prefix-local-strict-cone-semantic-parsimony";

const STAGE4_PRESEAL_OPENING_AUTHORITY_SCHEMA: &str =
    "stage4-bi0-opening-exact-prefix-capability-v1";
const STAGE4_PRESEAL_PROCEDURE_AUTHORITY_SCHEMA: &str = "stage4-parsimony-procedure-authority-v1";

const BI0_V5_ARTIFACT_BLAKE3: &str =
    "blake3:9709451e36838b3dd7974b18cff159e203be6ce27039390c45b739673ae58304";
const NU_REGISTER_ADJUDICATION_BLAKE3: &str =
    "blake3:bd59a0e10cdf9fe25e2b36b8e9c19427af21c1b07e577fdc11385a90f207de7d";
const TIE_PROTOCOL_BLAKE3: &str =
    "blake3:92b8cab2c0a87e575bb73840f047678bfd6a0ec94f5173907bb90000100f3d30";
const R_T3_ADJUDICATION_BLAKE3: &str =
    "blake3:b46c6db7fe642822ae5b472617f0bab009be521ece0867df2249791c53bfb804";
const R_T2_ARTIFACT_BLAKE3: &str =
    "blake3:664ac42b93d065b7dc965296d49cee6b9fafbb147ff6f3394d6769c5071acafa";
const PHASE5B_BURN_BLAKE3: &str =
    "blake3:e18da4d5902ba2bf540473da07ab067536d64a28953730437dff8e619bc3167e";

const R_T2_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json");
const PHASE5B_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v2.json");
const BI0_V5_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/bi0_semantic_register_v5.json");
const NU_REGISTER_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/nu_register_adjudication.md");
const TIE_PROTOCOL_BYTES: &[u8] = include_bytes!("../../../docs/tie_resolution_protocol.md");
const R_T3_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/r_t3_stage4_adjudication.md");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA, domain, value))
        .expect("Stage-4 semantic parsimony evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn phase5b_burn_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(PHASE5B_RESELECTION_BURN_SCHEMA, domain, value))
        .expect("Phase-5b burn projection serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn phase5b_burn_digest_valid(burn: &Phase5bReselectionBurn) -> bool {
    let mut projection = burn.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == phase5b_burn_hash("reselection-burn", &projection)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticSourceBindingV1 {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage4PresealOperationV1 {
    BindExactEnactedPrefix,
    EnumerateStrictCone,
    IssuePrefixLocalSemanticEvidence,
    CompareLexicographicParsimony,
    SealPretestimonyResult,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage4ForbiddenPresealInputV1 {
    FullBi0Certificate,
    HistoricalRegistry,
    LegacyV5Comparison,
    StructuralNu,
    ArchivedR2Comparison,
    EnactedWinner,
    DesiredRootCount,
    DesiredKappa,
    DesiredMinimum,
    DesiredTieVerdict,
    Bar,
    FutureBranch,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4PrefixEntryV1 {
    pub stage: u32,
    pub telescope: Telescope,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4PresealProcedureAuthorityV1 {
    pub schema: String,
    pub operations: Vec<Stage4PresealOperationV1>,
    pub parsimony_order: String,
    pub tie_behavior: String,
    pub forbidden_inputs: Vec<Stage4ForbiddenPresealInputV1>,
    pub authority_scope: String,
    pub closed_operation_surface: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4PresealOpeningTokenV1 {
    pub schema: String,
    pub granting_gate_schema: String,
    pub authority_scope: String,
    pub procedure_authority_derivation_hash: String,
    pub common_prefix_entries: Vec<Stage4PrefixEntryV1>,
    pub common_prefix_steps: Vec<u32>,
    pub common_prefix_candidate_hashes: Vec<String>,
    pub common_prefix_signature_digest: String,
    pub permitted_operations: Vec<Stage4PresealOperationV1>,
    pub forbidden_inputs: Vec<Stage4ForbiddenPresealInputV1>,
    pub full_bi0_scalar_or_archive_digest_present: bool,
    pub branch_continuation_capability_present: bool,
    pub exact_prefix_and_no_branch_opening: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4StrictConeRootGeometryV1 {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub telescope: Telescope,
    pub kappa: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4StrictConeGeometryV1 {
    pub opening_token_derivation_hash: String,
    pub common_prefix_signature_digest: String,
    pub stage: u32,
    pub window_depth: u16,
    pub enumerated_kappa_min: u16,
    pub enumerated_kappa_max: u16,
    pub raw_enumerated_count: usize,
    pub strict_admitted_count: usize,
    pub retained_strict_candidate_count: usize,
    pub all_strict_candidates_retained_without_canonical_deduplication: bool,
    pub least_kappa: u16,
    pub roots: Vec<Stage4StrictConeRootGeometryV1>,
    pub complete_nonempty_strict_cone: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticRootAuditV1 {
    pub candidate_hash: String,
    pub geometry_telescope_hash: String,
    pub kappa: u16,
    pub prefix_signature_digest: String,
    pub authoritative_prefix_local_v5_sequence_derivation_hash: String,
    pub authoritative_prefix_local_v5_sequence_seal: String,
    pub authoritative_prefix_local_v5_package_derivation_hashes: Vec<String>,
    pub authoritative_prefix_local_v5_stage4_package_hash: String,
    pub authoritative_prefix_semantic_v3_seal: String,
    pub prefix_semantic_nu_vector: Vec<u32>,
    pub stage4_semantic_nu: u32,
    pub prefix_local_v5_t_bi_b1_proved: bool,
    pub prefix_local_v5_t_bi_b2_proved: bool,
    pub prefix_local_v5_named_role_residual_count: usize,
    pub prefix_local_v5_named_quotient_residual_count: usize,
    pub prefix_local_v5_named_a3_residual_count: usize,
    pub prefix_local_v5_silent_residue_count: usize,
    pub b3_v3_derivation_hash: String,
    pub b3_v3_prefix_generic_isolation_proved: bool,
    pub b3_v3_prefix_local_sequence_replayed: bool,
    pub b3_v3_every_local_package_proved_b1_b2: bool,
    pub b3_v3_registry_extension_invariance_proved: bool,
    pub b3_v3_no_historical_registry_or_legacy_v5_authority: bool,
    pub b3_v3_no_forbidden_or_future_semantic_input: bool,
    pub exact_candidate_prefix_and_package_bindings: bool,
    pub root_semantic_audit_proved: bool,
    pub derivation_hash: String,
}

/// The exact root projection permitted to influence semantic parsimony.
/// Legacy v5 commitments and the full B3 token digest are intentionally
/// absent: they remain replayable compatibility testimony but cannot enter
/// the pre-testimony selection seal.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticSelectionRootV1 {
    pub candidate_hash: String,
    pub geometry_telescope_hash: String,
    pub kappa: u16,
    pub prefix_signature_digest: String,
    pub authoritative_prefix_semantic_v3_seal: String,
    pub prefix_semantic_nu_vector: Vec<u32>,
    pub stage4_semantic_nu: u32,
    pub registry_extension_invariance_proved: bool,
    pub prefix_generic_transitive_isolation_proved: bool,
    pub no_forbidden_or_future_semantic_input: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4LegacyV5RootComparisonV1 {
    pub candidate_hash: String,
    pub legacy_sequence_derivation_hash: String,
    pub legacy_sequence_seal: String,
    pub legacy_package_derivation_hashes: Vec<String>,
    pub legacy_stage4_package_hash: String,
    pub legacy_semantic_nu_vector: Vec<u32>,
    pub legacy_stage4_semantic_nu: u32,
    pub legacy_t_bi_b1_comparison: bool,
    pub legacy_t_bi_b2_comparison: bool,
    pub legacy_named_role_residual_comparison: usize,
    pub legacy_named_quotient_residual_comparison: usize,
    pub legacy_named_a3_residual_comparison: usize,
    pub legacy_silent_residue_comparison: usize,
    pub legacy_self_reported_forbidden_or_future_input_used: bool,
    pub legacy_sequence_replay_valid: bool,
    pub authoritative_nu_matches_legacy_comparison: bool,
    pub used_as_semantic_selector: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4Rt2SurvivorJoinV1 {
    pub unordered_candidate_pair: Vec<String>,
    pub left_complete_scheme_count: usize,
    pub right_complete_scheme_count: usize,
    pub matched_scheme_count: usize,
    pub full_scheme_sets_equivalent: bool,
    pub comparison_well_formed: bool,
    pub relative_coverage_and_zero_gap_gate_passed: bool,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub pairwise_row_derivation_hash_valid: bool,
    pub exactly_one_row_for_surviving_pair: bool,
    pub six_unordered_comparisons_exact: bool,
    pub all_six_pairwise_row_derivation_hashes_valid: bool,
    pub exact_five_five_three_inequivalence_signature: bool,
    pub exact_surviving_pair_join: bool,
    pub frozen_r_t2_comparison_derivation_hash: String,
    pub derivation_hash: String,
}

/// Postseal-only join between the blind live cone and the structural R-T1
/// audit.  The scalar minimum is testimony; this projection additionally
/// binds every candidate and every derivation used to obtain it.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4StructuralRt1PackageBindingV1 {
    pub candidate_hash: String,
    pub telescope_hash: String,
    pub package_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4StructuralRt1PairBindingV1 {
    pub unordered_candidate_pair: Vec<String>,
    pub comparison_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4StructuralRt1ClassBindingV1 {
    pub class_id: String,
    pub member_candidate_hashes: Vec<String>,
    pub class_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4StructuralRt1JoinV1 {
    pub live_candidate_hashes: Vec<String>,
    pub package_bindings: Vec<Stage4StructuralRt1PackageBindingV1>,
    pub pairwise_bindings: Vec<Stage4StructuralRt1PairBindingV1>,
    pub orbit_class_bindings: Vec<Stage4StructuralRt1ClassBindingV1>,
    pub structural_audit_derivation_hash: String,
    pub exact_candidate_multiset_join: bool,
    pub every_structural_derivation_bound: bool,
    pub replay_valid: bool,
    pub used_as_semantic_selector: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage4SemanticParsimonyOutcomeV1 {
    TwoSemanticMinimizersRt2InequivalentEnactedRootNonminimalAdjudicationRequired,
    UniqueSemanticMinimumRecordedNoBranchExecuted,
    OtherSemanticConeRecordedAdjudicationRequired,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticPresealV1 {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub procedure_authority: Stage4PresealProcedureAuthorityV1,
    pub opening_token: Stage4PresealOpeningTokenV1,
    pub live_strict_cone_geometry: Stage4StrictConeGeometryV1,
    pub common_prefix_steps: Vec<u32>,
    pub common_prefix_candidate_hashes: Vec<String>,
    pub common_prefix_signature_digest: String,
    pub roots: Vec<Stage4SemanticRootAuditV1>,
    pub root_count: usize,
    pub every_root_has_authoritative_prefix_local_b1_b2_b3_v3: bool,
    pub exact_stage1_through3_binding_from_local_b3_packages: bool,
    pub semantic_selection_roots: Vec<Stage4SemanticSelectionRootV1>,
    pub shared_stage1_through3_semantic_nu: Vec<u32>,
    pub exact_shared_semantic_prefix_across_all_roots: bool,
    pub semantic_parsimony_order: String,
    pub minimum_kappa: u16,
    pub minimum_semantic_nu: u32,
    pub semantic_minimizer_hashes: Vec<String>,
    pub semantic_minimizer_count: usize,
    pub semantic_parsimony_seal: String,
    pub typed_preseal_surface_excludes_declared_forbidden_inputs: bool,
    pub desired_numeric_vector_verdict_or_history_used_as_premise: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticParsimonyV1Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub preseal: Stage4SemanticPresealV1,
    pub source_bindings: Vec<Stage4SemanticSourceBindingV1>,
    pub postseal_attachment_consumed_exact_preseal_digest: String,
    pub full_bi0_read_or_replayed_before_semantic_seal: bool,
    pub sealed_bi0_v5_schema: String,
    pub sealed_bi0_v5_digest: String,
    pub sealed_bi0_v5_replay_valid: bool,
    pub sealed_bi0_v5_replay_errors: Vec<String>,
    pub sealed_bi0_v5_reissuance_drift_bound_postseal_only: bool,
    pub sealed_bi0_v5_pinned_logical_projection_valid: bool,
    pub sealed_bi0_v5_passed: bool,
    pub sealed_bi0_v5_executed_no_branch_work: bool,
    pub nu_register_adjudication_hash: String,
    pub tie_resolution_protocol_hash: String,
    pub r_t3_stage4_adjudication_hash: String,
    pub postseal_nu_register_and_tie_markers_replayed: bool,
    pub postseal_r_t3_branch_boundary_replayed: bool,
    pub postseal_live_geometry_root_count_is_four: bool,
    pub postseal_live_geometry_every_root_kappa_three: bool,
    pub postseal_live_geometry_candidate_hashes_distinct: bool,
    pub postseal_exact_four_distinct_kappa_three_regression: bool,
    pub exact_shared_semantic_prefix_is_1_0_1_postseal_regression: bool,
    pub postseal_legacy_v5_root_comparisons: Vec<Stage4LegacyV5RootComparisonV1>,
    pub postseal_legacy_v5_comparison_count: usize,
    pub legacy_v5_issued_only_after_semantic_seal: bool,
    pub legacy_v5_used_as_semantic_selector: bool,
    pub frozen_r_t2_certificate_digest: String,
    pub postseal_full_r_t2_projection_replay_valid: bool,
    pub postseal_archived_exact_four_way_r_t1_class_join: bool,
    pub postseal_live_root_hashes_join_archived_r_t2_geometry: bool,
    pub structural_testimony_read_after_semantic_seal: bool,
    pub structural_testimony_minimum_kappa: u16,
    pub structural_testimony_minimum_nu: u32,
    pub structural_testimony_minimizer_count: usize,
    pub structural_testimony_used_as_selector: bool,
    pub structural_r_t1_join: Stage4StructuralRt1JoinV1,
    pub survivor_r_t2_join: Option<Stage4Rt2SurvivorJoinV1>,
    pub enacted_history_burn_digest: String,
    pub enacted_root_hash: String,
    pub enacted_root_kappa: u16,
    pub enacted_root_semantic_nu: u32,
    pub enacted_root_identified_only_after_semantic_seal: bool,
    pub enacted_root_is_semantic_minimizer: bool,
    pub exact_two_minimizers: bool,
    pub surviving_pair_frozen_r_t2_inequivalent: bool,
    pub enacted_root_nonminimal: bool,
    pub exact_two_minimizer_enacted_nonminimal_divergence: bool,
    pub no_branch_executed: bool,
    pub theorem_issuer_has_no_artifact_write_capability: bool,
    pub desired_verdict_count_score_or_bar_used_as_semantic_premise: bool,
    pub outcome: Stage4SemanticParsimonyOutcomeV1,
    pub divergence_statement: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Stage4SemanticParsimonyV1Error {
    #[error("Stage-4 semantic parsimony prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("Stage-4 semantic parsimony invariant failed: {0}")]
    Invariant(String),
    #[error("Stage-4 semantic parsimony JSON failed: {0}")]
    Json(String),
    #[error("Stage-4 semantic parsimony I/O failed: {0}")]
    Io(String),
    #[error("Stage-4 semantic parsimony emitted replay failed: {0}")]
    EmittedReplay(String),
}

fn root_hash(root: &Stage4SemanticRootAuditV1) -> String {
    let mut projection = root.clone();
    projection.derivation_hash.clear();
    tagged_hash("semantic-root-audit", &projection)
}

fn prefix_entry_hash(entry: &Stage4PrefixEntryV1) -> String {
    let mut projection = entry.clone();
    projection.derivation_hash.clear();
    tagged_hash("exact-prefix-entry", &projection)
}

fn procedure_authority_hash(authority: &Stage4PresealProcedureAuthorityV1) -> String {
    let mut projection = authority.clone();
    projection.derivation_hash.clear();
    tagged_hash("preseal-procedure-authority", &projection)
}

fn opening_token_hash(token: &Stage4PresealOpeningTokenV1) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("preseal-opening-token", &projection)
}

fn preseal_hash(preseal: &Stage4SemanticPresealV1) -> String {
    let mut projection = preseal.clone();
    projection.result_digest.clear();
    tagged_hash("stage4-semantic-preseal", &projection)
}

fn legacy_comparison_hash(comparison: &Stage4LegacyV5RootComparisonV1) -> String {
    let mut projection = comparison.clone();
    projection.derivation_hash.clear();
    tagged_hash("postseal-legacy-v5-root-comparison", &projection)
}

fn semantic_selection_root(root: &Stage4SemanticRootAuditV1) -> Stage4SemanticSelectionRootV1 {
    Stage4SemanticSelectionRootV1 {
        candidate_hash: root.candidate_hash.clone(),
        geometry_telescope_hash: root.geometry_telescope_hash.clone(),
        kappa: root.kappa,
        prefix_signature_digest: root.prefix_signature_digest.clone(),
        authoritative_prefix_semantic_v3_seal: root.authoritative_prefix_semantic_v3_seal.clone(),
        prefix_semantic_nu_vector: root.prefix_semantic_nu_vector.clone(),
        stage4_semantic_nu: root.stage4_semantic_nu,
        registry_extension_invariance_proved: root.b3_v3_registry_extension_invariance_proved,
        prefix_generic_transitive_isolation_proved: root.b3_v3_prefix_generic_isolation_proved,
        no_forbidden_or_future_semantic_input: root.b3_v3_no_forbidden_or_future_semantic_input,
    }
}

fn semantic_parsimony_seal(
    roots: &[Stage4SemanticSelectionRootV1],
    minimum_pair: (u16, u32),
    minimizer_hashes: &[String],
) -> String {
    tagged_hash(
        "semantic-parsimony-pretestimony-seal",
        &(
            roots,
            minimum_pair,
            minimizer_hashes,
            "lexicographic least kappa then least authoritative registry-erased semantic-family nu",
        ),
    )
}

fn survivor_join_hash(join: &Stage4Rt2SurvivorJoinV1) -> String {
    let mut projection = join.clone();
    projection.derivation_hash.clear();
    tagged_hash("surviving-pair-r-t2-join", &projection)
}

fn structural_r_t1_join_hash(join: &Stage4StructuralRt1JoinV1) -> String {
    let mut projection = join.clone();
    projection.derivation_hash.clear();
    tagged_hash("postseal-structural-r-t1-join", &projection)
}

fn certificate_hash(certificate: &Stage4SemanticParsimonyV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("stage4-semantic-parsimony-certificate", &projection)
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> Stage4SemanticSourceBindingV1 {
    Stage4SemanticSourceBindingV1 {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    }
}

fn strict_cone_geometry_hash(geometry: &Stage4StrictConeGeometryV1) -> String {
    let mut projection = geometry.clone();
    projection.derivation_hash.clear();
    tagged_hash("live-stage4-strict-cone-geometry", &projection)
}

fn issue_preseal_procedure_authority() -> Stage4PresealProcedureAuthorityV1 {
    let operations = vec![
        Stage4PresealOperationV1::BindExactEnactedPrefix,
        Stage4PresealOperationV1::EnumerateStrictCone,
        Stage4PresealOperationV1::IssuePrefixLocalSemanticEvidence,
        Stage4PresealOperationV1::CompareLexicographicParsimony,
        Stage4PresealOperationV1::SealPretestimonyResult,
    ];
    let forbidden_inputs = vec![
        Stage4ForbiddenPresealInputV1::FullBi0Certificate,
        Stage4ForbiddenPresealInputV1::HistoricalRegistry,
        Stage4ForbiddenPresealInputV1::LegacyV5Comparison,
        Stage4ForbiddenPresealInputV1::StructuralNu,
        Stage4ForbiddenPresealInputV1::ArchivedR2Comparison,
        Stage4ForbiddenPresealInputV1::EnactedWinner,
        Stage4ForbiddenPresealInputV1::DesiredRootCount,
        Stage4ForbiddenPresealInputV1::DesiredKappa,
        Stage4ForbiddenPresealInputV1::DesiredMinimum,
        Stage4ForbiddenPresealInputV1::DesiredTieVerdict,
        Stage4ForbiddenPresealInputV1::Bar,
        Stage4ForbiddenPresealInputV1::FutureBranch,
    ];
    let mut authority = Stage4PresealProcedureAuthorityV1 {
        schema: STAGE4_PRESEAL_PROCEDURE_AUTHORITY_SCHEMA.to_owned(),
        operations,
        parsimony_order:
            "lexicographic: least kappa, then least certified prefix-local semantic-family nu"
                .to_owned(),
        tie_behavior:
            "report every equal minimum; do not select a branch or consult R-T1/R-T2 preseal"
                .to_owned(),
        forbidden_inputs,
        authority_scope: "This token authorizes only the closed Stage-4 preseal operation surface; adopted-document identity and all historical comparisons are postseal testimony."
            .to_owned(),
        closed_operation_surface: true,
        derivation_hash: String::new(),
    };
    authority.derivation_hash = procedure_authority_hash(&authority);
    authority
}

fn replay_preseal_procedure_authority(authority: &Stage4PresealProcedureAuthorityV1) -> bool {
    authority == &issue_preseal_procedure_authority()
        && authority.derivation_hash == procedure_authority_hash(authority)
}

fn prefix_entries(stem: &[(u32, Telescope)]) -> Vec<Stage4PrefixEntryV1> {
    stem.iter()
        .enumerate()
        .map(|(index, (stage, telescope))| {
            let mut entry = Stage4PrefixEntryV1 {
                stage: *stage,
                telescope: telescope.clone(),
                candidate_hash: candidate_hash(telescope),
                predecessor_signature_digest: SealedSignature::from_telescopes(
                    stem[..index].to_vec(),
                )
                .digest()
                .to_owned(),
                derivation_hash: String::new(),
            };
            entry.derivation_hash = prefix_entry_hash(&entry);
            entry
        })
        .collect()
}

/// Mint a self-digested opening capability over the exact enacted prefix
/// payload.  No BI-0 scalar digest or archive digest is accepted by this
/// surface.  The full BI-0 certificate is replayed only by the postseal
/// attachment.
fn issue_preseal_opening_token(
    stem: &[(u32, Telescope)],
    procedure_authority: &Stage4PresealProcedureAuthorityV1,
) -> Result<Stage4PresealOpeningTokenV1, Stage4SemanticParsimonyV1Error> {
    if !replay_preseal_procedure_authority(procedure_authority) {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "opening token received an invalid typed procedure authority".to_owned(),
        ));
    }
    let common_prefix_steps = stem.iter().map(|(stage, _)| *stage).collect::<Vec<_>>();
    let common_prefix_candidate_hashes = stem
        .iter()
        .map(|(_, candidate)| candidate_hash(candidate))
        .collect::<Vec<_>>();
    let common_prefix_signature_digest = SealedSignature::from_telescopes(stem.to_vec())
        .digest()
        .to_owned();
    let common_prefix_entries = prefix_entries(stem);
    let exact_prefix_and_no_branch_opening = common_prefix_steps == [1, 2, 3]
        && stem
            .iter()
            .all(|(stage, telescope)| *telescope == Telescope::reference(*stage))
        && common_prefix_entries.len() == stem.len()
        && common_prefix_entries.iter().all(|entry| {
            entry.derivation_hash == prefix_entry_hash(entry)
                && entry.candidate_hash == candidate_hash(&entry.telescope)
        })
        && common_prefix_candidate_hashes
            == common_prefix_entries
                .iter()
                .map(|entry| entry.candidate_hash.clone())
                .collect::<Vec<_>>()
        && !common_prefix_signature_digest.is_empty()
        && procedure_authority.closed_operation_surface;
    if !exact_prefix_and_no_branch_opening {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "opening token did not bind the exact Stage-1-through-3 payload and no-branch capability"
                .to_owned(),
        ));
    }
    let mut token = Stage4PresealOpeningTokenV1 {
        schema: STAGE4_PRESEAL_OPENING_AUTHORITY_SCHEMA.to_owned(),
        granting_gate_schema: BI0_SEMANTIC_REGISTER_V5_SCHEMA.to_owned(),
        authority_scope: "Exact Stage-1-through-3 payload plus the closed preseal operation surface; this token is not a BI-0 certificate digest and carries no historical semantic register."
            .to_owned(),
        procedure_authority_derivation_hash: procedure_authority.derivation_hash.clone(),
        common_prefix_entries,
        common_prefix_steps,
        common_prefix_candidate_hashes,
        common_prefix_signature_digest,
        permitted_operations: procedure_authority.operations.clone(),
        forbidden_inputs: procedure_authority.forbidden_inputs.clone(),
        full_bi0_scalar_or_archive_digest_present: false,
        branch_continuation_capability_present: false,
        exact_prefix_and_no_branch_opening,
        derivation_hash: String::new(),
    };
    token.derivation_hash = opening_token_hash(&token);
    Ok(token)
}

fn replay_preseal_opening_token(
    stem: &[(u32, Telescope)],
    procedure_authority: &Stage4PresealProcedureAuthorityV1,
    token: &Stage4PresealOpeningTokenV1,
) -> bool {
    token.derivation_hash == opening_token_hash(token)
        && issue_preseal_opening_token(stem, procedure_authority)
            .is_ok_and(|expected| expected == *token)
}

/// Public typed seam for a BI-0 successor: the returned capability contains
/// the exact prefix payload and no BI-0 certificate or archive digest.
pub fn issue_stage4_preseal_opening_token_v1()
-> Result<Stage4PresealOpeningTokenV1, Stage4SemanticParsimonyV1Error> {
    let stem = common_stem()?;
    let authority = issue_preseal_procedure_authority();
    issue_preseal_opening_token(&stem, &authority)
}

pub fn replay_stage4_preseal_opening_token_v1(
    claimed: &Stage4PresealOpeningTokenV1,
) -> Vec<String> {
    let Ok(stem) = common_stem() else {
        return vec!["canonical Stage-1-through-3 prefix is unavailable".to_owned()];
    };
    let authority = issue_preseal_procedure_authority();
    if replay_preseal_opening_token(&stem, &authority, claimed) {
        Vec::new()
    } else {
        vec!["Stage-4 preseal opening token differs from exact-prefix reissuance".to_owned()]
    }
}

/// Enumerate the complete guarded Stage-4 strict cone without constructing or
/// storing structural nu.  This is the only preseal source of root geometry.
pub fn issue_stage4_strict_cone_geometry_v1()
-> Result<Stage4StrictConeGeometryV1, Stage4SemanticParsimonyV1Error> {
    let stem = common_stem()?;
    let authority = issue_preseal_procedure_authority();
    let opening = issue_preseal_opening_token(&stem, &authority)?;
    issue_stage4_strict_cone_geometry_from_opening(&stem, &opening)
}

fn issue_stage4_strict_cone_geometry_from_opening(
    stem: &[(u32, Telescope)],
    opening: &Stage4PresealOpeningTokenV1,
) -> Result<Stage4StrictConeGeometryV1, Stage4SemanticParsimonyV1Error> {
    let procedure_authority = issue_preseal_procedure_authority();
    if !replay_preseal_opening_token(stem, &procedure_authority, opening) {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "live cone did not receive an exact replayable opening token".to_owned(),
        ));
    }
    let mut library: Library = Vec::new();
    for (_, telescope) in stem {
        library.push(LibraryEntry::from_telescope(telescope, &library));
    }
    let admissibility = strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::Guarded);
    let context = EnumerationContext::from_admissibility(&library, admissibility);
    let mut raw = Vec::new();
    for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        raw.extend(enumerate_telescopes(&library, context, kappa));
    }
    let raw_enumerated_count = raw.len();
    let strict = raw
        .into_iter()
        .filter(|telescope| passes_strict_admissibility(4, &library, telescope, admissibility))
        .collect::<Vec<_>>();
    let strict_admitted_count = strict.len();
    let mut roots = strict
        .into_iter()
        .map(|telescope| {
            let canonical_key = canonical_key_telescope(&telescope).0;
            let kappa = u16::try_from(telescope.kappa()).expect("admissible kappa fits u16");
            Stage4StrictConeRootGeometryV1 {
                candidate_hash: candidate_hash(&telescope),
                canonical_key,
                telescope,
                kappa,
            }
        })
        .collect::<Vec<_>>();
    roots.sort_by(|left, right| {
        left.candidate_hash
            .cmp(&right.candidate_hash)
            .then_with(|| left.canonical_key.cmp(&right.canonical_key))
    });
    let least_kappa = roots.iter().map(|root| root.kappa).min().ok_or_else(|| {
        Stage4SemanticParsimonyV1Error::Prerequisite("live Stage-4 strict cone is empty".to_owned())
    })?;
    let all_strict_candidates_retained_without_canonical_deduplication =
        strict_admitted_count == roots.len();
    let complete_nonempty_strict_cone = !roots.is_empty()
        && all_strict_candidates_retained_without_canonical_deduplication
        && roots.iter().all(|root| {
            passes_strict_admissibility(4, &library, &root.telescope, admissibility)
                && candidate_hash(&root.telescope) == root.candidate_hash
                && canonical_key_telescope(&root.telescope).0 == root.canonical_key
        });
    if !complete_nonempty_strict_cone {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "live Stage-4 strict cone is empty, incomplete, or dropped an enumerated strict candidate (raw={raw_enumerated_count}, strict={strict_admitted_count}, retained={})",
            roots.len()
        )));
    }
    let mut geometry = Stage4StrictConeGeometryV1 {
        opening_token_derivation_hash: opening.derivation_hash.clone(),
        common_prefix_signature_digest: opening.common_prefix_signature_digest.clone(),
        stage: 4,
        window_depth: 2,
        enumerated_kappa_min: admissibility.min_clause_kappa,
        enumerated_kappa_max: admissibility.max_clause_kappa,
        raw_enumerated_count,
        strict_admitted_count,
        retained_strict_candidate_count: roots.len(),
        all_strict_candidates_retained_without_canonical_deduplication,
        least_kappa,
        roots,
        complete_nonempty_strict_cone,
        derivation_hash: String::new(),
    };
    geometry.derivation_hash = strict_cone_geometry_hash(&geometry);
    Ok(geometry)
}

pub fn replay_stage4_strict_cone_geometry_v1(claimed: &Stage4StrictConeGeometryV1) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != strict_cone_geometry_hash(claimed) {
        errors.push("Stage-4 strict-cone geometry digest mismatch".to_owned());
    }
    match issue_stage4_strict_cone_geometry_v1() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => {
            errors.push("Stage-4 strict-cone geometry differs from live reissuance".to_owned())
        }
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn r_t2_pairwise_row_hash(row: &Rt2V2PairwiseSchemeSetComparison) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    let bytes = serde_json::to_vec(&(
        R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA,
        "pairwise-complete-stage5-scheme-sets",
        projection,
    ))
    .expect("R-T2 pairwise projection serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn unordered_pair(left: &str, right: &str) -> Vec<String> {
    let mut pair = vec![left.to_owned(), right.to_owned()];
    pair.sort();
    pair
}

fn common_stem() -> Result<Vec<(u32, Telescope)>, Stage4SemanticParsimonyV1Error> {
    Ok((1..=3)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect())
}

fn root_audit(
    telescope: &Telescope,
    stem: &[(u32, Telescope)],
) -> Result<Stage4SemanticRootAuditV1, Stage4SemanticParsimonyV1Error> {
    let mut entries = stem.to_vec();
    entries.push((4, telescope.clone()));
    let prefix_signature_digest = SealedSignature::from_telescopes(entries.clone())
        .digest()
        .to_owned();
    let b3_context = issue_replayed_t_bi_intrinsic_isolation_v3_context(&entries)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Invariant(error.to_string()))?;
    let b3_independently_replayed = b3_context.proved
        && b3_context.replay_errors.is_empty()
        && b3_context.sequence_evidence_equal
        && b3_context.token_evidence_equal;
    if !b3_independently_replayed {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(format!(
            "B3 v3 replay failed: {}",
            b3_context.replay_errors.join("; ")
        )));
    }
    let b3 = b3_context.token;
    let sequence = b3_context.sequence;
    let package = sequence.packages.last().ok_or_else(|| {
        Stage4SemanticParsimonyV1Error::Invariant(
            "source-first Stage-4 package is absent".to_owned(),
        )
    })?;
    let authoritative_semantic_nu_vector = sequence.semantic_nu_vector.clone();
    let stage4_semantic_nu = authoritative_semantic_nu_vector
        .last()
        .copied()
        .ok_or_else(|| {
            Stage4SemanticParsimonyV1Error::Invariant(
                "B3 v3 registry-erased semantic vector is empty".to_owned(),
            )
        })?;
    let exact_candidate_prefix_and_package_bindings = entries
        .iter()
        .enumerate()
        .zip(&sequence.packages)
        .zip(&b3.prefix_bindings)
        .all(|(((index, (stage, candidate)), package), binding)| {
            let predecessor = SealedSignature::from_telescopes(entries[..index].to_vec());
            package.stage == *stage
                && package.candidate_hash == candidate_hash(candidate)
                && package.predecessor_signature_digest == predecessor.digest()
                && binding.stage == *stage
                && binding.candidate_hash == candidate_hash(candidate)
                && binding.predecessor_signature_digest == predecessor.digest()
                && binding.semantic_package_derivation_hash == package.derivation_hash
                && binding.exact_candidate_and_prefix_binding
        })
        && sequence.packages.len() == entries.len()
        && b3.prefix_bindings.len() == entries.len();
    let root_semantic_audit_proved = b3_independently_replayed
        && b3.prefix_generic_transitive_isolation_proved
        && b3.prefix_local_sequence_replayed
        && b3.every_local_package_proved_b1_b2
        && b3.every_registry_extension_projection_equal
        && b3.no_historical_registry_or_legacy_v5_authority
        && !b3.authoritative_prefix_semantic_seal.is_empty()
        && sequence.no_historical_registry_or_future_input
        && sequence.t_bi_b1_proved_on_sequence
        && sequence.t_bi_b2_proved_on_sequence
        && exact_candidate_prefix_and_package_bindings
        && b3.no_archive_structural_bar_verdict_or_future_input;
    if !root_semantic_audit_proved {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(format!(
            "root {} did not close B1/B2/B3 without residuals",
            package.candidate_hash
        )));
    }
    let kappa = u16::try_from(telescope.kappa()).map_err(|_| {
        Stage4SemanticParsimonyV1Error::Invariant("Stage-4 kappa exceeds u16".to_owned())
    })?;
    let mut root = Stage4SemanticRootAuditV1 {
        candidate_hash: candidate_hash(telescope),
        geometry_telescope_hash: tagged_hash("certified-root-geometry", telescope),
        kappa,
        prefix_signature_digest,
        authoritative_prefix_local_v5_sequence_derivation_hash: sequence.derivation_hash.clone(),
        authoritative_prefix_local_v5_sequence_seal: sequence.authoritative_sequence_seal.clone(),
        authoritative_prefix_local_v5_package_derivation_hashes: sequence
            .packages
            .iter()
            .map(|package| package.derivation_hash.clone())
            .collect(),
        authoritative_prefix_local_v5_stage4_package_hash: package.derivation_hash.clone(),
        authoritative_prefix_semantic_v3_seal: b3.authoritative_prefix_semantic_seal.clone(),
        prefix_semantic_nu_vector: authoritative_semantic_nu_vector,
        stage4_semantic_nu,
        prefix_local_v5_t_bi_b1_proved: package.t_bi_b1_proved,
        prefix_local_v5_t_bi_b2_proved: package.t_bi_b2_proved,
        prefix_local_v5_named_role_residual_count: package.named_role_residual_count,
        prefix_local_v5_named_quotient_residual_count: package.named_quotient_residual_count,
        prefix_local_v5_named_a3_residual_count: package.named_a3_residual_count,
        prefix_local_v5_silent_residue_count: package.silent_residue_count,
        b3_v3_derivation_hash: b3.derivation_hash,
        b3_v3_prefix_generic_isolation_proved: b3.prefix_generic_transitive_isolation_proved,
        b3_v3_prefix_local_sequence_replayed: b3.prefix_local_sequence_replayed,
        b3_v3_every_local_package_proved_b1_b2: b3.every_local_package_proved_b1_b2,
        b3_v3_registry_extension_invariance_proved: b3.every_registry_extension_projection_equal,
        b3_v3_no_historical_registry_or_legacy_v5_authority: b3
            .no_historical_registry_or_legacy_v5_authority,
        b3_v3_no_forbidden_or_future_semantic_input: b3
            .no_archive_structural_bar_verdict_or_future_input,
        exact_candidate_prefix_and_package_bindings,
        root_semantic_audit_proved,
        derivation_hash: String::new(),
    };
    root.derivation_hash = root_hash(&root);
    Ok(root)
}

fn postseal_legacy_v5_comparison(
    telescope: &Telescope,
    stem: &[(u32, Telescope)],
    authoritative: &Stage4SemanticRootAuditV1,
) -> Result<Stage4LegacyV5RootComparisonV1, Stage4SemanticParsimonyV1Error> {
    let mut entries = stem.to_vec();
    entries.push((4, telescope.clone()));
    let sequence = issue_act_local_semantic_sequence_v5(&entries)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Invariant(error.to_string()))?;
    let replay_errors = replay_act_local_semantic_sequence_v5(&entries, &sequence);
    if !replay_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(format!(
            "postseal legacy-v5 replay failed: {}",
            replay_errors.join("; ")
        )));
    }
    let package = sequence.packages.last().ok_or_else(|| {
        Stage4SemanticParsimonyV1Error::Invariant(
            "postseal legacy-v5 Stage-4 package is absent".to_owned(),
        )
    })?;
    if package.candidate_hash != authoritative.candidate_hash {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(
            "postseal legacy-v5 comparison is not bound to the authoritative root".to_owned(),
        ));
    }
    let legacy_semantic_nu_vector = sequence
        .packages
        .iter()
        .map(|package| package.semantic_family_nu)
        .collect::<Vec<_>>();
    let legacy_stage4_semantic_nu = package.semantic_family_nu;
    let legacy_self_reported_forbidden_or_future_input_used =
        sequence.packages.iter().any(|package| {
            package.archive_read
                || package.structural_nu_read
                || package.bar_read
                || package.verdict_read
                || package.enacted_future_read
        });
    let authoritative_nu_matches_legacy_comparison =
        authoritative.prefix_semantic_nu_vector == legacy_semantic_nu_vector;
    let mut comparison = Stage4LegacyV5RootComparisonV1 {
        candidate_hash: authoritative.candidate_hash.clone(),
        legacy_sequence_derivation_hash: sequence.derivation_hash.clone(),
        legacy_sequence_seal: sequence.intrinsic_sequence_seal.clone(),
        legacy_package_derivation_hashes: sequence.exact_package_derivation_hashes.clone(),
        legacy_stage4_package_hash: package.derivation_hash.clone(),
        legacy_semantic_nu_vector,
        legacy_stage4_semantic_nu,
        legacy_t_bi_b1_comparison: package.t_bi_b1_proved,
        legacy_t_bi_b2_comparison: package.t_bi_b2_proved,
        legacy_named_role_residual_comparison: package.named_role_residual_count,
        legacy_named_quotient_residual_comparison: package.named_quotient_residual_count,
        legacy_named_a3_residual_comparison: package.named_a3_residual_count,
        legacy_silent_residue_comparison: package.silent_residue_count,
        legacy_self_reported_forbidden_or_future_input_used,
        legacy_sequence_replay_valid: true,
        authoritative_nu_matches_legacy_comparison,
        used_as_semantic_selector: false,
        derivation_hash: String::new(),
    };
    comparison.derivation_hash = legacy_comparison_hash(&comparison);
    Ok(comparison)
}

/// Issue the authoritative semantic comparison.  This function reads no
/// adjudication prose, BI-0 certificate, archive, structural score, expected
/// root count, expected kappa, or expected tie result.  Procedure authority is
/// a closed typed token; all historical comparison is deferred to postseal.
pub fn issue_stage4_semantic_preseal_v1()
-> Result<Stage4SemanticPresealV1, Stage4SemanticParsimonyV1Error> {
    let procedure_authority = issue_preseal_procedure_authority();
    if !replay_preseal_procedure_authority(&procedure_authority) {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "typed preseal procedure authority failed replay".to_owned(),
        ));
    }
    let stem = common_stem()?;
    let opening_token = issue_preseal_opening_token(&stem, &procedure_authority)?;
    let common_prefix_steps = stem.iter().map(|(stage, _)| *stage).collect::<Vec<_>>();
    let common_prefix_candidate_hashes = stem
        .iter()
        .map(|(_, candidate)| candidate_hash(candidate))
        .collect::<Vec<_>>();
    let common_prefix_signature_digest = SealedSignature::from_telescopes(stem.clone())
        .digest()
        .to_owned();
    let geometry = issue_stage4_strict_cone_geometry_from_opening(&stem, &opening_token)?;
    if geometry.derivation_hash != strict_cone_geometry_hash(&geometry)
        || !geometry.complete_nonempty_strict_cone
        || !geometry.all_strict_candidates_retained_without_canonical_deduplication
    {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "live Stage-4 strict-cone geometry failed its typed opening-bound replay".to_owned(),
        ));
    }
    let mut roots = geometry
        .roots
        .iter()
        .map(|root| root_audit(&root.telescope, &stem))
        .collect::<Result<Vec<_>, _>>()?;
    roots.sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));
    let expected_prefix_length = stem.len() + 1;
    if roots.is_empty()
        || roots.len() != geometry.roots.len()
        || roots.iter().any(|root| {
            root.prefix_semantic_nu_vector.len() != expected_prefix_length
                || root
                    .authoritative_prefix_local_v5_package_derivation_hashes
                    .len()
                    != expected_prefix_length
        })
    {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(
            "source-first geometry did not produce one exact prefix-local semantic audit per retained strict candidate"
                .to_owned(),
        ));
    }
    let first_prefix_packages =
        roots[0].authoritative_prefix_local_v5_package_derivation_hashes[..stem.len()].to_vec();
    let exact_stage1_through3_binding_from_local_b3_packages = roots.iter().all(|root| {
        root.exact_candidate_prefix_and_package_bindings
            && root
                .authoritative_prefix_local_v5_package_derivation_hashes
                .len()
                == expected_prefix_length
            && root.authoritative_prefix_local_v5_package_derivation_hashes[..stem.len()]
                == first_prefix_packages
    });
    if !exact_stage1_through3_binding_from_local_b3_packages {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(
            "the source-first B3 roots do not all bind the same exact Stage-1-through-3 package prefix"
                .to_owned(),
        ));
    }
    let shared_stage1_through3_semantic_nu =
        roots[0].prefix_semantic_nu_vector[..stem.len()].to_vec();
    let exact_shared_semantic_prefix_across_all_roots = roots.iter().all(|root| {
        root.prefix_semantic_nu_vector[..stem.len()] == shared_stage1_through3_semantic_nu
    });
    if !exact_shared_semantic_prefix_across_all_roots {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(
            "the source-first roots do not all share a semantic Stage-1-through-3 prefix"
                .to_owned(),
        ));
    }
    let minimum_pair = roots
        .iter()
        .map(|root| (root.kappa, root.stage4_semantic_nu))
        .min()
        .ok_or_else(|| Stage4SemanticParsimonyV1Error::Invariant("empty root cone".to_owned()))?;
    let semantic_minimizer_hashes = roots
        .iter()
        .filter(|root| (root.kappa, root.stage4_semantic_nu) == minimum_pair)
        .map(|root| root.candidate_hash.clone())
        .collect::<Vec<_>>();
    let semantic_selection_roots = roots
        .iter()
        .map(semantic_selection_root)
        .collect::<Vec<_>>();
    let semantic_parsimony_seal = semantic_parsimony_seal(
        &semantic_selection_roots,
        minimum_pair,
        &semantic_minimizer_hashes,
    );
    let every_root_has_authoritative_prefix_local_b1_b2_b3_v3 =
        roots.iter().all(|root| root.root_semantic_audit_proved);
    let typed_preseal_surface_excludes_declared_forbidden_inputs =
        replay_preseal_procedure_authority(&procedure_authority)
            && replay_preseal_opening_token(&stem, &procedure_authority, &opening_token)
            && roots.iter().all(|root| {
                root.b3_v3_no_historical_registry_or_legacy_v5_authority
                    && root.b3_v3_no_forbidden_or_future_semantic_input
            });
    if !every_root_has_authoritative_prefix_local_b1_b2_b3_v3
        || !typed_preseal_surface_excludes_declared_forbidden_inputs
    {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(
            "a preseal root lacks source-first B1/B2/B3 isolation".to_owned(),
        ));
    }
    let mut preseal = Stage4SemanticPresealV1 {
        schema: STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA.to_owned(),
        date: STAGE4_SEMANTIC_PARSIMONY_V1_DATE.to_owned(),
        theorem_id: STAGE4_SEMANTIC_PARSIMONY_V1_THEOREM_ID.to_owned(),
        procedure_authority,
        opening_token,
        live_strict_cone_geometry: geometry,
        common_prefix_steps,
        common_prefix_candidate_hashes,
        common_prefix_signature_digest,
        root_count: roots.len(),
        every_root_has_authoritative_prefix_local_b1_b2_b3_v3,
        exact_stage1_through3_binding_from_local_b3_packages,
        semantic_selection_roots,
        shared_stage1_through3_semantic_nu,
        exact_shared_semantic_prefix_across_all_roots,
        semantic_parsimony_order: "lexicographic: least kappa, then least source-first prefix-local semantic-family nu from B3 v3"
            .to_owned(),
        minimum_kappa: minimum_pair.0,
        minimum_semantic_nu: minimum_pair.1,
        semantic_minimizer_count: semantic_minimizer_hashes.len(),
        semantic_minimizer_hashes,
        semantic_parsimony_seal,
        typed_preseal_surface_excludes_declared_forbidden_inputs,
        desired_numeric_vector_verdict_or_history_used_as_premise: false,
        roots,
        result_digest: String::new(),
    };
    preseal.result_digest = preseal_hash(&preseal);
    Ok(preseal)
}

pub fn replay_stage4_semantic_preseal_v1(claimed: &Stage4SemanticPresealV1) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.result_digest != preseal_hash(claimed) {
        errors.push("Stage-4 semantic preseal digest mismatch".to_owned());
    }
    match issue_stage4_semantic_preseal_v1() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("Stage-4 semantic preseal differs from blind reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn attach_stage4_postseal_testimony(
    preseal: Stage4SemanticPresealV1,
) -> Result<Stage4SemanticParsimonyV1Certificate, Stage4SemanticParsimonyV1Error> {
    if preseal.result_digest != preseal_hash(&preseal) {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(
            "postseal attachment received an invalid semantic preseal".to_owned(),
        ));
    }
    let frozen_sources_pinned = bytes_hash(BI0_V5_ARTIFACT_BYTES) == BI0_V5_ARTIFACT_BLAKE3
        && bytes_hash(NU_REGISTER_ADJUDICATION_BYTES) == NU_REGISTER_ADJUDICATION_BLAKE3
        && bytes_hash(TIE_PROTOCOL_BYTES) == TIE_PROTOCOL_BLAKE3
        && bytes_hash(R_T3_ADJUDICATION_BYTES) == R_T3_ADJUDICATION_BLAKE3
        && bytes_hash(R_T2_ARTIFACT_BYTES) == R_T2_ARTIFACT_BLAKE3
        && bytes_hash(PHASE5B_BURN_BYTES) == PHASE5B_BURN_BLAKE3;
    if !frozen_sources_pinned {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "a frozen postseal testimony source differs from its pinned byte digest".to_owned(),
        ));
    }
    let semantic_minimizer_hashes = preseal.semantic_minimizer_hashes.clone();
    let minimum_pair = (preseal.minimum_kappa, preseal.minimum_semantic_nu);
    let roots = &preseal.roots;
    let geometry = &preseal.live_strict_cone_geometry;
    let stem = common_stem()?;
    let postseal_live_geometry_root_count_is_four = geometry.roots.len() == 4;
    let postseal_live_geometry_every_root_kappa_three =
        geometry.roots.iter().all(|root| root.kappa == 3);
    let postseal_live_geometry_candidate_hashes_distinct = geometry
        .roots
        .iter()
        .map(|root| root.candidate_hash.as_str())
        .collect::<BTreeSet<_>>()
        .len()
        == geometry.roots.len();
    let postseal_exact_four_distinct_kappa_three_regression =
        postseal_live_geometry_root_count_is_four
            && postseal_live_geometry_every_root_kappa_three
            && postseal_live_geometry_candidate_hashes_distinct;

    let nu_adjudication = std::str::from_utf8(NU_REGISTER_ADJUDICATION_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Prerequisite(error.to_string()))?;
    let tie_protocol = std::str::from_utf8(TIE_PROTOCOL_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Prerequisite(error.to_string()))?;
    let postseal_nu_register_and_tie_markers_replayed = nu_adjudication
        .contains("Status:** **ADOPTED")
        && nu_adjudication.contains("F-AL1")
        && tie_protocol.contains("Status:** **ADOPTED")
        && tie_protocol.contains("R-T1")
        && tie_protocol.contains("R-T2");
    if !postseal_nu_register_and_tie_markers_replayed {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "postseal adopted semantic-register or tie-protocol markers are absent".to_owned(),
        ));
    }

    let mut postseal_legacy_v5_root_comparisons = geometry
        .roots
        .iter()
        .map(|geometry_root| {
            let authoritative = roots
                .iter()
                .find(|root| root.candidate_hash == geometry_root.candidate_hash)
                .ok_or_else(|| {
                    Stage4SemanticParsimonyV1Error::Invariant(
                        "live geometry root has no authoritative preseal audit".to_owned(),
                    )
                })?;
            postseal_legacy_v5_comparison(&geometry_root.telescope, &stem, authoritative)
        })
        .collect::<Result<Vec<_>, _>>()?;
    postseal_legacy_v5_root_comparisons
        .sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));

    let bi0: Bi0SemanticRegisterV5Certificate = serde_json::from_slice(BI0_V5_ARTIFACT_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    let bi0_replay = replay_bi0_semantic_register_v5_certificate(&bi0);
    let sealed_bi0_v5_pinned_logical_projection_valid = bi0.schema
        == BI0_SEMANTIC_REGISTER_V5_SCHEMA
        && bi0.bi0_attempt_executed
        && bi0.bi0_passed
        && !bi0.bi1_invoked
        && !bi0.non_enacted_cone_invoked
        && !bi0.non_enacted_branch_work_executed;
    let reissuance_drift_marker = "BI-0 v5 certificate differs from create-new reissuance";
    let bi0_replay_errors_are_only_deterministic_reissuance_drift = bi0_replay
        .errors
        .iter()
        .all(|error| error == reissuance_drift_marker)
        && bi0_replay.errors.len() <= 1;
    if !sealed_bi0_v5_pinned_logical_projection_valid
        || !bi0_replay_errors_are_only_deterministic_reissuance_drift
        || !bi0_replay.bi0_passed
        || bi0_replay.bi1_invoked
        || bi0_replay.non_enacted_cone_invoked
    {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "postseal pinned BI-0 v5 logical projection failed its passed/no-branch gate: {:?}",
            bi0_replay.errors
        )));
    }
    // BI-0 is testimony only at this point.  A deterministic reissuance drift
    // caused by later issuer hardening is published in the certificate and
    // cannot retroactively alter the already sealed semantic comparison.
    let sealed_bi0_v5_reissuance_drift_bound_postseal_only = !bi0_replay.valid;
    let r_t3_adjudication = std::str::from_utf8(R_T3_ADJUDICATION_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Prerequisite(error.to_string()))?;
    let postseal_r_t3_branch_boundary_replayed = r_t3_adjudication.contains("Option B adopted")
        && r_t3_adjudication.contains("F-R3-B1")
        && r_t3_adjudication.contains("F-R3-B2");
    if !postseal_r_t3_branch_boundary_replayed {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "postseal R-T3 branch-boundary adoption markers are absent".to_owned(),
        ));
    }
    let r_t2: Rt2FutureHoleConfluenceV2Certificate = serde_json::from_slice(R_T2_ARTIFACT_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    let r_t2_projection_errors = replay_archived_stage4_fork_projection(&r_t2);
    if !r_t2_projection_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "postseal frozen R-T2 full projection failed: {}",
            r_t2_projection_errors.join("; ")
        )));
    }
    let archived_candidate_hashes = r_t2
        .branches
        .iter()
        .map(|branch| branch.stage4_candidate_hash.as_str())
        .collect::<BTreeSet<_>>();
    let live_candidate_hashes = roots
        .iter()
        .map(|root| root.candidate_hash.as_str())
        .collect::<BTreeSet<_>>();
    let archived_class_ids = r_t2
        .branches
        .iter()
        .map(|branch| branch.r_t1_class_id.as_str())
        .collect::<BTreeSet<_>>();
    let postseal_archived_exact_four_way_r_t1_class_join =
        r_t2.exact_four_way_r_t1_class_join && archived_class_ids.len() == 4;
    let postseal_live_root_hashes_join_archived_r_t2_geometry = archived_candidate_hashes
        == live_candidate_hashes
        && r_t2.branches.iter().all(|branch| {
            roots.iter().any(|root| {
                root.candidate_hash == branch.stage4_candidate_hash
                    && candidate_hash(&branch.stage4_telescope) == root.candidate_hash
                    && branch.stage4_telescope
                        == geometry
                            .roots
                            .iter()
                            .find(|geometry_root| {
                                geometry_root.candidate_hash == root.candidate_hash
                            })
                            .expect("live root is in live geometry")
                            .telescope
                    && branch.prefix_steps == vec![1, 2, 3, 4]
                    && branch.prefix_steps_exactly_one_through_four
                    && branch.prefix_signature_digest == root.prefix_signature_digest
            })
        });
    if !postseal_archived_exact_four_way_r_t1_class_join
        || !postseal_live_root_hashes_join_archived_r_t2_geometry
        || !postseal_exact_four_distinct_kappa_three_regression
    {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "postseal four-root/kappa-three regression or archived R-T2 geometry join failed"
                .to_owned(),
        ));
    }
    let structural = issue_stage4_r_t1_orbit_audit()
        .map_err(|error| Stage4SemanticParsimonyV1Error::Prerequisite(error.to_string()))?;
    let structural_errors = replay_stage4_r_t1_orbit_audit(&structural);
    if !structural_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "structural R-T1 replay failed: {}",
            structural_errors.join("; ")
        )));
    }
    let mut live_candidate_hashes = roots
        .iter()
        .map(|root| root.candidate_hash.clone())
        .collect::<Vec<_>>();
    live_candidate_hashes.sort();
    let mut package_bindings = structural
        .packages
        .iter()
        .map(|package| Stage4StructuralRt1PackageBindingV1 {
            candidate_hash: package.candidate_hash.clone(),
            telescope_hash: tagged_hash("postseal-r-t1-package-telescope", &package.telescope),
            package_derivation_hash: package.derivation_hash.clone(),
        })
        .collect::<Vec<_>>();
    package_bindings.sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));
    let mut pairwise_bindings = structural
        .pairwise_comparisons
        .iter()
        .map(|comparison| Stage4StructuralRt1PairBindingV1 {
            unordered_candidate_pair: unordered_pair(
                &comparison.left_candidate_hash,
                &comparison.right_candidate_hash,
            ),
            comparison_derivation_hash: comparison.derivation_hash.clone(),
        })
        .collect::<Vec<_>>();
    pairwise_bindings.sort_by(|left, right| {
        left.unordered_candidate_pair
            .cmp(&right.unordered_candidate_pair)
    });
    let mut orbit_class_bindings = structural
        .orbit_classes
        .iter()
        .map(|class| {
            let mut member_candidate_hashes = class.member_candidate_hashes.clone();
            member_candidate_hashes.sort();
            Stage4StructuralRt1ClassBindingV1 {
                class_id: class.class_id.clone(),
                member_candidate_hashes,
                class_derivation_hash: class.derivation_hash.clone(),
            }
        })
        .collect::<Vec<_>>();
    orbit_class_bindings.sort_by(|left, right| left.class_id.cmp(&right.class_id));
    let structural_candidate_hashes = package_bindings
        .iter()
        .map(|binding| binding.candidate_hash.clone())
        .collect::<Vec<_>>();
    let exact_candidate_multiset_join = live_candidate_hashes == structural_candidate_hashes
        && package_bindings.iter().all(|binding| {
            geometry.roots.iter().any(|root| {
                root.candidate_hash == binding.candidate_hash
                    && tagged_hash("postseal-r-t1-package-telescope", &root.telescope)
                        == binding.telescope_hash
            })
        });
    let expected_pair_count = live_candidate_hashes
        .len()
        .saturating_mul(live_candidate_hashes.len().saturating_sub(1))
        / 2;
    let every_structural_derivation_bound = !structural.derivation_hash.is_empty()
        && package_bindings
            .iter()
            .all(|binding| !binding.package_derivation_hash.is_empty())
        && pairwise_bindings.len() == expected_pair_count
        && pairwise_bindings
            .iter()
            .all(|binding| !binding.comparison_derivation_hash.is_empty())
        && orbit_class_bindings
            .iter()
            .all(|binding| !binding.class_derivation_hash.is_empty());
    let mut structural_r_t1_join = Stage4StructuralRt1JoinV1 {
        live_candidate_hashes,
        package_bindings,
        pairwise_bindings,
        orbit_class_bindings,
        structural_audit_derivation_hash: structural.derivation_hash.clone(),
        exact_candidate_multiset_join,
        every_structural_derivation_bound,
        replay_valid: true,
        used_as_semantic_selector: false,
        derivation_hash: String::new(),
    };
    structural_r_t1_join.derivation_hash = structural_r_t1_join_hash(&structural_r_t1_join);
    if !structural_r_t1_join.exact_candidate_multiset_join
        || !structural_r_t1_join.every_structural_derivation_bound
    {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "postseal structural R-T1 testimony did not bind the exact live candidate multiset and derivations"
                .to_owned(),
        ));
    }

    let survivor_r_t2_join = if semantic_minimizer_hashes.len() == 2 {
        let live_hashes = roots
            .iter()
            .map(|root| root.candidate_hash.clone())
            .collect::<Vec<_>>();
        let mut expected_unordered_pairs = BTreeSet::new();
        for left in 0..live_hashes.len() {
            for right in (left + 1)..live_hashes.len() {
                expected_unordered_pairs
                    .insert(unordered_pair(&live_hashes[left], &live_hashes[right]));
            }
        }
        let all_unordered_pairs = r_t2
            .pairwise_scheme_set_comparisons
            .iter()
            .map(|comparison| {
                unordered_pair(
                    &comparison.left_candidate_hash,
                    &comparison.right_candidate_hash,
                )
            })
            .collect::<Vec<_>>();
        let observed_unordered_pairs = all_unordered_pairs.iter().cloned().collect::<BTreeSet<_>>();
        let all_six_pairwise_row_derivation_hashes_valid = r_t2
            .pairwise_scheme_set_comparisons
            .iter()
            .all(|comparison| comparison.derivation_hash == r_t2_pairwise_row_hash(comparison));
        let six_unordered_comparisons_exact = expected_unordered_pairs.len() == 6
            && all_unordered_pairs.len() == 6
            && observed_unordered_pairs == expected_unordered_pairs;
        let matching_rows = r_t2
            .pairwise_scheme_set_comparisons
            .iter()
            .filter(|comparison| {
                unordered_pair(
                    &comparison.left_candidate_hash,
                    &comparison.right_candidate_hash,
                ) == semantic_minimizer_hashes
            })
            .collect::<Vec<_>>();
        if matching_rows.len() != 1 {
            return Err(Stage4SemanticParsimonyV1Error::Invariant(format!(
                "frozen R-T2 has {} rows for the exact semantic survivor pair",
                matching_rows.len()
            )));
        }
        let comparison = matching_rows[0];
        let pairwise_row_derivation_hash_valid =
            comparison.derivation_hash == r_t2_pairwise_row_hash(comparison);
        let exact_five_five_three_inequivalence_signature = comparison.left_complete_scheme_count
            == 5
            && comparison.right_complete_scheme_count == 5
            && comparison.perfect_matching.len() == 3
            && !comparison.full_scheme_sets_equivalent
            && comparison.comparison_well_formed
            && comparison.relative_coverage_and_zero_gap_gate_passed
            && !comparison.hash_or_enumeration_order_used_as_selector;
        if !six_unordered_comparisons_exact
            || !all_six_pairwise_row_derivation_hashes_valid
            || !pairwise_row_derivation_hash_valid
            || !comparison.comparison_well_formed
            || !comparison.relative_coverage_and_zero_gap_gate_passed
            || comparison.hash_or_enumeration_order_used_as_selector
        {
            return Err(Stage4SemanticParsimonyV1Error::Invariant(
                "frozen survivor row failed its exact six-row/hash/well-formed integrity gate"
                    .to_owned(),
            ));
        }
        let mut join = Stage4Rt2SurvivorJoinV1 {
            unordered_candidate_pair: semantic_minimizer_hashes.clone(),
            left_complete_scheme_count: comparison.left_complete_scheme_count,
            right_complete_scheme_count: comparison.right_complete_scheme_count,
            matched_scheme_count: comparison.perfect_matching.len(),
            full_scheme_sets_equivalent: comparison.full_scheme_sets_equivalent,
            comparison_well_formed: comparison.comparison_well_formed,
            relative_coverage_and_zero_gap_gate_passed: comparison
                .relative_coverage_and_zero_gap_gate_passed,
            hash_or_enumeration_order_used_as_selector: comparison
                .hash_or_enumeration_order_used_as_selector,
            pairwise_row_derivation_hash_valid,
            exactly_one_row_for_surviving_pair: true,
            six_unordered_comparisons_exact,
            all_six_pairwise_row_derivation_hashes_valid,
            exact_five_five_three_inequivalence_signature,
            exact_surviving_pair_join: pairwise_row_derivation_hash_valid
                && all_six_pairwise_row_derivation_hashes_valid
                && six_unordered_comparisons_exact
                && comparison.comparison_well_formed
                && comparison.relative_coverage_and_zero_gap_gate_passed
                && !comparison.hash_or_enumeration_order_used_as_selector,
            frozen_r_t2_comparison_derivation_hash: comparison.derivation_hash.clone(),
            derivation_hash: String::new(),
        };
        join.derivation_hash = survivor_join_hash(&join);
        Some(join)
    } else {
        None
    };

    let burn: Phase5bReselectionBurn = serde_json::from_slice(PHASE5B_BURN_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    if burn.schema != PHASE5B_RESELECTION_BURN_SCHEMA || !phase5b_burn_digest_valid(&burn) {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "sealed Phase-5b burn digest is invalid".to_owned(),
        ));
    }
    let enacted_stage4_rows = burn
        .stages
        .iter()
        .filter(|stage| stage.stage == 4)
        .collect::<Vec<_>>();
    if enacted_stage4_rows.len() != 1 {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "sealed Phase-5b burn has {} Stage-4 rows instead of exactly one",
            enacted_stage4_rows.len()
        )));
    }
    let enacted_stage4 = enacted_stage4_rows[0].winner.as_ref().ok_or_else(|| {
        Stage4SemanticParsimonyV1Error::Prerequisite(
            "sealed Phase-5b burn has no enacted Stage-4 root".to_owned(),
        )
    })?;
    let enacted_root = roots
        .iter()
        .find(|root| root.candidate_hash == enacted_stage4.candidate_hash)
        .ok_or_else(|| {
            Stage4SemanticParsimonyV1Error::Invariant(
                "sealed enacted Stage-4 digest is outside the certified four-root geometry"
                    .to_owned(),
            )
        })?;

    let exact_two_minimizers = semantic_minimizer_hashes.len() == 2;
    let surviving_pair_frozen_r_t2_inequivalent = survivor_r_t2_join
        .as_ref()
        .is_some_and(|join| join.exact_surviving_pair_join && !join.full_scheme_sets_equivalent);
    let enacted_root_is_semantic_minimizer = semantic_minimizer_hashes
        .iter()
        .any(|hash| hash == &enacted_root.candidate_hash);
    let enacted_root_nonminimal = !enacted_root_is_semantic_minimizer;
    let enacted_root_hash = enacted_root.candidate_hash.clone();
    let enacted_root_kappa = enacted_root.kappa;
    let enacted_root_semantic_nu = enacted_root.stage4_semantic_nu;
    let exact_two_minimizer_enacted_nonminimal_divergence =
        exact_two_minimizers && surviving_pair_frozen_r_t2_inequivalent && enacted_root_nonminimal;
    let outcome = if exact_two_minimizer_enacted_nonminimal_divergence {
        Stage4SemanticParsimonyOutcomeV1::TwoSemanticMinimizersRt2InequivalentEnactedRootNonminimalAdjudicationRequired
    } else if semantic_minimizer_hashes.len() == 1 {
        Stage4SemanticParsimonyOutcomeV1::UniqueSemanticMinimumRecordedNoBranchExecuted
    } else {
        Stage4SemanticParsimonyOutcomeV1::OtherSemanticConeRecordedAdjudicationRequired
    };
    let divergence_statement = format!(
        "Intrinsic semantic parsimony (least kappa, then least semantic nu) yields {} minimizer(s) {:?} at ({},{}). The sealed enacted root {} reissues at ({},{}), so enacted_nonminimal={}. The frozen R-T2 survivor comparison is inequivalent={}, matched_scheme_count={}. No branch was selected or executed.",
        semantic_minimizer_hashes.len(),
        semantic_minimizer_hashes,
        minimum_pair.0,
        minimum_pair.1,
        enacted_root_hash,
        enacted_root_kappa,
        enacted_root_semantic_nu,
        enacted_root_nonminimal,
        survivor_r_t2_join
            .as_ref()
            .is_some_and(|join| !join.full_scheme_sets_equivalent),
        survivor_r_t2_join
            .as_ref()
            .map_or(0, |join| join.matched_scheme_count),
    );
    let postseal_attachment_consumed_exact_preseal_digest = preseal.result_digest.clone();
    let exact_shared_semantic_prefix_is_1_0_1_postseal_regression =
        preseal.shared_stage1_through3_semantic_nu == [1, 0, 1];
    let mut certificate = Stage4SemanticParsimonyV1Certificate {
        schema: STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA.to_owned(),
        date: STAGE4_SEMANTIC_PARSIMONY_V1_DATE.to_owned(),
        theorem_id: STAGE4_SEMANTIC_PARSIMONY_V1_THEOREM_ID.to_owned(),
        postseal_attachment_consumed_exact_preseal_digest,
        preseal,
        source_bindings: vec![
            source_binding(
                "docs/bi0_semantic_register_v5.json",
                "postseal_full_replay_only; preseal opening is a self-digested exact-prefix token",
                BI0_V5_ARTIFACT_BYTES,
            ),
            source_binding(
                "docs/nu_register_adjudication.md",
                "postseal_adoption-marker_testimony_only",
                NU_REGISTER_ADJUDICATION_BYTES,
            ),
            source_binding(
                "docs/tie_resolution_protocol.md",
                "postseal_adoption-marker_testimony_only",
                TIE_PROTOCOL_BYTES,
            ),
            source_binding(
                "docs/r_t3_stage4_adjudication.md",
                "postseal_adopted_option_b_branch_invariance_boundary",
                R_T3_ADJUDICATION_BYTES,
            ),
            source_binding(
                "docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json",
                "postseal_archived_geometry_and_exact_pair_comparison_only",
                R_T2_ARTIFACT_BYTES,
            ),
            source_binding(
                "docs/phase5b_reselection_burn_v2.json",
                "postseal_enacted_root_digest_only",
                PHASE5B_BURN_BYTES,
            ),
        ],
        full_bi0_read_or_replayed_before_semantic_seal: false,
        sealed_bi0_v5_schema: bi0.schema.clone(),
        sealed_bi0_v5_digest: bi0.result_digest.clone(),
        sealed_bi0_v5_replay_valid: bi0_replay.valid,
        sealed_bi0_v5_replay_errors: bi0_replay.errors.clone(),
        sealed_bi0_v5_reissuance_drift_bound_postseal_only,
        sealed_bi0_v5_pinned_logical_projection_valid,
        sealed_bi0_v5_passed: bi0.bi0_passed,
        sealed_bi0_v5_executed_no_branch_work: bi0.bi0_attempt_executed
            && !bi0.bi1_invoked
            && !bi0.non_enacted_cone_invoked
            && !bi0.non_enacted_branch_work_executed,
        nu_register_adjudication_hash: bytes_hash(NU_REGISTER_ADJUDICATION_BYTES),
        tie_resolution_protocol_hash: bytes_hash(TIE_PROTOCOL_BYTES),
        r_t3_stage4_adjudication_hash: bytes_hash(R_T3_ADJUDICATION_BYTES),
        postseal_nu_register_and_tie_markers_replayed,
        postseal_r_t3_branch_boundary_replayed,
        postseal_live_geometry_root_count_is_four,
        postseal_live_geometry_every_root_kappa_three,
        postseal_live_geometry_candidate_hashes_distinct,
        postseal_exact_four_distinct_kappa_three_regression,
        exact_shared_semantic_prefix_is_1_0_1_postseal_regression,
        postseal_legacy_v5_comparison_count: postseal_legacy_v5_root_comparisons.len(),
        postseal_legacy_v5_root_comparisons,
        legacy_v5_issued_only_after_semantic_seal: true,
        legacy_v5_used_as_semantic_selector: false,
        frozen_r_t2_certificate_digest: r_t2.result_digest,
        postseal_full_r_t2_projection_replay_valid: true,
        postseal_archived_exact_four_way_r_t1_class_join,
        postseal_live_root_hashes_join_archived_r_t2_geometry,
        structural_testimony_read_after_semantic_seal: true,
        structural_testimony_minimum_kappa: structural.minimum_kappa,
        structural_testimony_minimum_nu: structural.minimum_certified_nu,
        structural_testimony_minimizer_count: structural.minimizer_count,
        structural_testimony_used_as_selector: false,
        structural_r_t1_join,
        survivor_r_t2_join,
        enacted_history_burn_digest: burn.result_digest,
        enacted_root_hash,
        enacted_root_kappa,
        enacted_root_semantic_nu,
        enacted_root_identified_only_after_semantic_seal: true,
        enacted_root_is_semantic_minimizer,
        exact_two_minimizers,
        surviving_pair_frozen_r_t2_inequivalent,
        enacted_root_nonminimal,
        exact_two_minimizer_enacted_nonminimal_divergence,
        no_branch_executed: true,
        theorem_issuer_has_no_artifact_write_capability: true,
        desired_verdict_count_score_or_bar_used_as_semantic_premise: false,
        outcome,
        divergence_statement,
        required_successor_action: "Versioned adjudication is required before any branch continuation; the theorem issuer authorizes no branch and has no artifact-write capability."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_hash(&certificate);
    Ok(certificate)
}

/// Run both typed phases.  The theorem issuer returns data only; no branch is
/// selected or executed and this function has no filesystem parameter.
pub fn issue_stage4_semantic_parsimony_v1()
-> Result<Stage4SemanticParsimonyV1Certificate, Stage4SemanticParsimonyV1Error> {
    attach_stage4_postseal_testimony(issue_stage4_semantic_preseal_v1()?)
}

fn validate_against_expected(
    claimed: &Stage4SemanticParsimonyV1Certificate,
    expected: &Stage4SemanticParsimonyV1Certificate,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_hash(claimed) {
        errors.push("Stage-4 semantic parsimony certificate digest mismatch".to_owned());
    }
    if claimed != expected {
        errors.push("certificate differs from deterministic verdict-blind reissuance".to_owned());
    }
    errors
}

pub fn replay_stage4_semantic_parsimony_v1(
    claimed: &Stage4SemanticParsimonyV1Certificate,
) -> Vec<String> {
    match issue_stage4_semantic_parsimony_v1() {
        Ok(expected) => validate_against_expected(claimed, &expected),
        Err(error) => vec![error.to_string()],
    }
}

pub fn replay_stage4_semantic_parsimony_v1_json(json: &str) -> Vec<String> {
    match serde_json::from_str::<Stage4SemanticParsimonyV1Certificate>(json) {
        Ok(certificate) => replay_stage4_semantic_parsimony_v1(&certificate),
        Err(error) => vec![format!(
            "Stage-4 semantic parsimony JSON did not deserialize: {error}"
        )],
    }
}

/// Create a replay-checked artifact.  This outer serialization helper is
/// intentionally separate from the theorem issuer and uses `create_new`.
pub fn emit_stage4_semantic_parsimony_v1_create_new(
    path: &Path,
) -> Result<Stage4SemanticParsimonyV1Certificate, Stage4SemanticParsimonyV1Error> {
    let certificate = issue_stage4_semantic_parsimony_v1()?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    let replay_errors = replay_stage4_semantic_parsimony_v1_json(&json);
    if !replay_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::EmittedReplay(
            replay_errors.join("; "),
        ));
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Io(error.to_string()))?;
    output
        .write_all(json.as_bytes())
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Stage4SemanticParsimonyV1Error::Io(error.to_string()))?;
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn cached_certificate() -> Stage4SemanticParsimonyV1Certificate {
        static CERTIFICATE: OnceLock<Stage4SemanticParsimonyV1Certificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_stage4_semantic_parsimony_v1().expect("semantic audit"))
            .clone()
    }

    #[test]
    fn live_strict_cone_retains_every_strict_candidate_and_contains_no_testimony() {
        let geometry = issue_stage4_strict_cone_geometry_v1().expect("strict cone");
        assert!(!geometry.roots.is_empty());
        assert!(geometry.complete_nonempty_strict_cone);
        assert!(geometry.all_strict_candidates_retained_without_canonical_deduplication);
        assert_eq!(geometry.roots.len(), geometry.strict_admitted_count);
        assert_eq!(
            geometry.roots.len(),
            geometry.retained_strict_candidate_count
        );
        assert!(
            !serde_json::to_string(&geometry)
                .expect("geometry serializes")
                .contains("structural_nu")
        );
        assert!(replay_stage4_strict_cone_geometry_v1(&geometry).is_empty());
    }

    #[test]
    fn preseal_opening_is_exact_prefix_bound_and_contains_no_archive_digest() {
        let stem = common_stem().expect("common stem");
        let procedure = issue_preseal_procedure_authority();
        let opening_token = issue_preseal_opening_token(&stem, &procedure).expect("opening token");
        assert!(opening_token.exact_prefix_and_no_branch_opening);
        assert_eq!(opening_token.common_prefix_steps, vec![1, 2, 3]);
        assert_eq!(opening_token.common_prefix_candidate_hashes.len(), 3);
        assert_eq!(opening_token.common_prefix_entries.len(), 3);
        assert_eq!(
            opening_token.procedure_authority_derivation_hash,
            procedure.derivation_hash
        );
        assert_eq!(
            opening_token.granting_gate_schema,
            BI0_SEMANTIC_REGISTER_V5_SCHEMA
        );
        assert!(!opening_token.full_bi0_scalar_or_archive_digest_present);
        assert!(!opening_token.branch_continuation_capability_present);
        assert!(replay_preseal_opening_token(
            &stem,
            &procedure,
            &opening_token
        ));
        assert_eq!(
            opening_token.derivation_hash,
            opening_token_hash(&opening_token)
        );
        let serialized = serde_json::to_string(&opening_token).expect("token serializes");
        assert!(!serialized.contains(BI0_V5_ARTIFACT_BLAKE3));
        assert!(!serialized.contains("t_bi_result_digest"));
        assert!(!serialized.contains("authoritative_semantic_register"));

        // Re-hash every affected layer of a wrong-prefix forgery.  The token
        // is internally content-hashed, but replay still rejects it because
        // the payload is not the authorized enacted prefix.
        let mut forged = opening_token.clone();
        forged.common_prefix_entries[0].telescope = Telescope::reference(2);
        let forged_stem = forged
            .common_prefix_entries
            .iter()
            .map(|entry| (entry.stage, entry.telescope.clone()))
            .collect::<Vec<_>>();
        for (index, entry) in forged.common_prefix_entries.iter_mut().enumerate() {
            entry.candidate_hash = candidate_hash(&entry.telescope);
            entry.predecessor_signature_digest =
                SealedSignature::from_telescopes(forged_stem[..index].to_vec())
                    .digest()
                    .to_owned();
            entry.derivation_hash = prefix_entry_hash(entry);
        }
        forged.common_prefix_candidate_hashes = forged
            .common_prefix_entries
            .iter()
            .map(|entry| entry.candidate_hash.clone())
            .collect();
        forged.common_prefix_signature_digest = SealedSignature::from_telescopes(forged_stem)
            .digest()
            .to_owned();
        forged.derivation_hash = opening_token_hash(&forged);
        assert_eq!(forged.derivation_hash, opening_token_hash(&forged));
        assert!(
            forged
                .common_prefix_entries
                .iter()
                .all(|entry| entry.derivation_hash == prefix_entry_hash(entry))
        );
        assert!(!replay_preseal_opening_token(&stem, &procedure, &forged));
        assert!(issue_stage4_strict_cone_geometry_from_opening(&stem, &forged).is_err());
    }

    #[test]
    fn preseal_issuer_source_has_no_document_or_outcome_testimony_read() {
        let source = include_str!("stage4_semantic_parsimony_v1.rs");
        let start = source
            .find("/// Issue the authoritative semantic comparison.")
            .expect("preseal issuer source boundary");
        let end = source[start..]
            .find("fn attach_stage4_postseal_testimony(")
            .map(|offset| start + offset)
            .expect("postseal attachment source boundary");
        let issuer = &source[start..end];
        for forbidden in [
            "include_bytes!",
            "from_slice",
            "from_utf8",
            "BI0_V5_ARTIFACT",
            "NU_REGISTER_ADJUDICATION",
            "TIE_PROTOCOL_BYTES",
            "R_T2_ARTIFACT",
            "PHASE5B_BURN",
            "exact_four",
            "len() != 4",
            "len() == 4",
        ] {
            assert!(
                !issuer.contains(forbidden),
                "preseal issuer contains forbidden postseal/outcome marker {forbidden}"
            );
        }
        assert!(issuer.contains("complete_nonempty_strict_cone"));
        assert!(issuer.contains("all_strict_candidates_retained_without_canonical_deduplication"));
    }

    #[test]
    fn semantic_preseal_replays_before_any_postseal_testimony() {
        let preseal = issue_stage4_semantic_preseal_v1().expect("blind semantic preseal");
        assert!(replay_stage4_semantic_preseal_v1(&preseal).is_empty());
        assert!(
            preseal
                .live_strict_cone_geometry
                .complete_nonempty_strict_cone
        );
        assert_eq!(
            preseal.root_count,
            preseal
                .live_strict_cone_geometry
                .retained_strict_candidate_count
        );
        assert!(preseal.typed_preseal_surface_excludes_declared_forbidden_inputs);
    }

    #[test]
    fn blind_semantic_audit_publishes_exact_stage4_divergence_without_execution() {
        let certificate = cached_certificate();
        let preseal = &certificate.preseal;
        assert_eq!(preseal.result_digest, preseal_hash(preseal));
        assert_eq!(preseal.root_count, 4);
        assert_eq!(preseal.minimum_kappa, 3);
        assert_eq!(preseal.minimum_semantic_nu, 2);
        assert_eq!(preseal.semantic_minimizer_count, 2);
        assert!(replay_preseal_procedure_authority(
            &preseal.procedure_authority
        ));
        assert!(preseal.opening_token.exact_prefix_and_no_branch_opening);
        assert!(
            preseal
                .live_strict_cone_geometry
                .complete_nonempty_strict_cone
        );
        assert!(
            preseal
                .live_strict_cone_geometry
                .all_strict_candidates_retained_without_canonical_deduplication
        );
        assert!(preseal.every_root_has_authoritative_prefix_local_b1_b2_b3_v3);
        assert!(preseal.exact_stage1_through3_binding_from_local_b3_packages);
        assert_eq!(preseal.semantic_selection_roots.len(), 4);
        assert!(preseal.roots.iter().all(|root| {
            root.b3_v3_prefix_generic_isolation_proved
                && root.b3_v3_prefix_local_sequence_replayed
                && root.b3_v3_every_local_package_proved_b1_b2
                && root.b3_v3_registry_extension_invariance_proved
                && root.b3_v3_no_historical_registry_or_legacy_v5_authority
                && root.b3_v3_no_forbidden_or_future_semantic_input
                && root.exact_candidate_prefix_and_package_bindings
        }));
        assert_eq!(preseal.shared_stage1_through3_semantic_nu, vec![1, 0, 1]);
        assert!(preseal.exact_shared_semantic_prefix_across_all_roots);
        assert_eq!(certificate.structural_testimony_minimum_kappa, 3);
        assert_eq!(certificate.structural_testimony_minimum_nu, 5);
        assert_eq!(certificate.structural_testimony_minimizer_count, 4);
        assert!(certificate.exact_two_minimizer_enacted_nonminimal_divergence);
        assert!(certificate.surviving_pair_frozen_r_t2_inequivalent);
        assert!(certificate.enacted_root_nonminimal);
        assert!(!certificate.full_bi0_read_or_replayed_before_semantic_seal);
        assert!(certificate.sealed_bi0_v5_pinned_logical_projection_valid);
        assert_eq!(
            certificate.sealed_bi0_v5_reissuance_drift_bound_postseal_only,
            !certificate.sealed_bi0_v5_replay_valid
        );
        assert!(
            certificate
                .sealed_bi0_v5_replay_errors
                .iter()
                .all(|error| { error == "BI-0 v5 certificate differs from create-new reissuance" })
        );
        assert!(certificate.sealed_bi0_v5_passed);
        assert!(certificate.sealed_bi0_v5_executed_no_branch_work);
        assert!(certificate.postseal_full_r_t2_projection_replay_valid);
        assert!(certificate.postseal_live_geometry_root_count_is_four);
        assert!(certificate.postseal_live_geometry_every_root_kappa_three);
        assert!(certificate.postseal_live_geometry_candidate_hashes_distinct);
        assert!(certificate.postseal_exact_four_distinct_kappa_three_regression);
        assert!(certificate.postseal_archived_exact_four_way_r_t1_class_join);
        assert!(certificate.postseal_live_root_hashes_join_archived_r_t2_geometry);
        assert!(certificate.exact_shared_semantic_prefix_is_1_0_1_postseal_regression);
        assert_eq!(certificate.postseal_legacy_v5_comparison_count, 4);
        assert!(certificate.legacy_v5_issued_only_after_semantic_seal);
        assert!(!certificate.legacy_v5_used_as_semantic_selector);
        assert!(
            certificate
                .structural_r_t1_join
                .exact_candidate_multiset_join
        );
        assert!(
            certificate
                .structural_r_t1_join
                .every_structural_derivation_bound
        );
        assert_eq!(
            certificate.structural_r_t1_join.derivation_hash,
            structural_r_t1_join_hash(&certificate.structural_r_t1_join)
        );
        assert!(
            certificate
                .postseal_legacy_v5_root_comparisons
                .iter()
                .all(|comparison| comparison.legacy_sequence_replay_valid
                    && comparison.authoritative_nu_matches_legacy_comparison
                    && !comparison.used_as_semantic_selector)
        );
        let join = certificate
            .survivor_r_t2_join
            .as_ref()
            .expect("exact survivor join");
        assert_eq!(join.left_complete_scheme_count, 5);
        assert_eq!(join.right_complete_scheme_count, 5);
        assert_eq!(join.matched_scheme_count, 3);
        assert!(join.pairwise_row_derivation_hash_valid);
        assert!(join.exactly_one_row_for_surviving_pair);
        assert!(join.six_unordered_comparisons_exact);
        assert!(join.all_six_pairwise_row_derivation_hashes_valid);
        assert!(join.exact_five_five_three_inequivalence_signature);
        assert!(certificate.no_branch_executed);
        assert!(certificate.theorem_issuer_has_no_artifact_write_capability);
        assert_eq!(
            certificate.outcome,
            Stage4SemanticParsimonyOutcomeV1::TwoSemanticMinimizersRt2InequivalentEnactedRootNonminimalAdjudicationRequired
        );

        let mut testimony_only_mutation = certificate.postseal_legacy_v5_root_comparisons.clone();
        testimony_only_mutation[0].legacy_sequence_derivation_hash =
            "blake3:non-authoritative-testimony-mutation".to_owned();
        testimony_only_mutation[0].authoritative_nu_matches_legacy_comparison = false;
        testimony_only_mutation[0].derivation_hash =
            legacy_comparison_hash(&testimony_only_mutation[0]);
        assert_ne!(
            testimony_only_mutation,
            certificate.postseal_legacy_v5_root_comparisons
        );
        assert_eq!(
            semantic_parsimony_seal(
                &preseal.semantic_selection_roots,
                (preseal.minimum_kappa, preseal.minimum_semantic_nu),
                &preseal.semantic_minimizer_hashes,
            ),
            preseal.semantic_parsimony_seal
        );
    }

    #[test]
    fn fully_rehashed_mutations_cannot_rewrite_the_semantic_result() {
        let expected = cached_certificate();
        let mut forgeries = Vec::new();

        let mut forged = expected.clone();
        forged.preseal.minimum_semantic_nu += 1;
        forged.preseal.result_digest = preseal_hash(&forged.preseal);
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.preseal.semantic_minimizer_hashes = vec![expected.enacted_root_hash.clone()];
        forged.preseal.semantic_minimizer_count = 1;
        forged.preseal.result_digest = preseal_hash(&forged.preseal);
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.enacted_root_nonminimal = false;
        forged.enacted_root_is_semantic_minimizer = true;
        forgeries.push(forged);

        let mut forged = expected.clone();
        let join = forged
            .survivor_r_t2_join
            .as_mut()
            .expect("two-survivor join");
        join.full_scheme_sets_equivalent = true;
        join.derivation_hash = survivor_join_hash(join);
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.postseal_legacy_v5_root_comparisons[0].used_as_semantic_selector = true;
        forged.postseal_legacy_v5_root_comparisons[0].derivation_hash =
            legacy_comparison_hash(&forged.postseal_legacy_v5_root_comparisons[0]);
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.preseal.live_strict_cone_geometry.roots[0].kappa = 4;
        forged.preseal.live_strict_cone_geometry.derivation_hash =
            strict_cone_geometry_hash(&forged.preseal.live_strict_cone_geometry);
        forged.preseal.result_digest = preseal_hash(&forged.preseal);
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.preseal.procedure_authority.tie_behavior =
            "select the first equal minimum".to_owned();
        forged.preseal.procedure_authority.derivation_hash =
            procedure_authority_hash(&forged.preseal.procedure_authority);
        forged.preseal.result_digest = preseal_hash(&forged.preseal);
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.structural_r_t1_join.package_bindings[0].package_derivation_hash =
            "blake3:fully-rehashed-substitution".to_owned();
        forged.structural_r_t1_join.derivation_hash =
            structural_r_t1_join_hash(&forged.structural_r_t1_join);
        forgeries.push(forged);

        for forged in &mut forgeries {
            forged.result_digest = certificate_hash(forged);
            let errors = validate_against_expected(forged, &expected);
            assert!(errors.iter().any(|error| error.contains("reissuance")));
            assert!(!errors.iter().any(|error| error.contains("digest mismatch")));
        }
    }
}
