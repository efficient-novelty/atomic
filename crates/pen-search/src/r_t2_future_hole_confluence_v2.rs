//! Versioned, blocker-aware R-T2 successor-scheme confluence rerun.
//!
//! V1 compared the five schemes emitted by the then-current Stage-5 A3
//! generator, but the generator still named rule-inventory exhaustiveness as
//! a premise.  This successor keeps that scoped comparison immutable and
//! permits a law-level R-T2/R-T3 conclusion only after an independently
//! replayed A3 inventory theorem proves full relative seed/constructor
//! coverage with no named gap.  Family equality is scoped by the declared
//! canonical-presentation arity; a maximum free-variable reference is never
//! used as a substitute for that declaration.

use crate::naturality_orbit_transport::FrozenEqualityRecord;
use crate::r_t2_future_hole_confluence::{
    BranchStage5Formation, R_T2_FUTURE_HOLE_CONFLUENCE_SCHEMA, Rt2FutureHoleConfluenceCertificate,
    SchemeFamilyPresentation, Stage5SchemeFormation, issue_r_t2_future_hole_confluence_certificate,
    replay_r_t2_future_hole_confluence_certificate,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3DemandSchemeOrigin, A3HistoricalWindow, A3RuleConstructor, A3TypedDemandInstance,
    A3TypedDemandScheme, generate_a3_window_for_prefix,
};
use pen_eval::a3_rule_inventory_exhaustiveness::{
    A3RuleInventoryExhaustivenessCertificate, A3WindowRuleInventoryProof,
    issue_historical_a3_rule_inventory_exhaustiveness, prove_a3_window_inventory_for_exact_prefix,
    replay_historical_a3_rule_inventory_exhaustiveness,
};
use pen_eval::future_hole_hypothesis_v2::{
    FUTURE_HOLE_HYPOTHESIS_V2_SCHEMA, FutureHoleBodyV2, FutureHoleOutputContractV2,
    FutureHoleRegistrationDispositionV2, RegisteredFutureHoleV2, StructuralJurisdictionV2,
    attach_external_exhaustiveness_evidence_v2, register_structural_future_hole_v2,
    register_unary_action_v2, replay_future_hole_registration_v2,
};
use pen_eval::typed_families::ParamSort;
use pen_type::contextual_internality::{
    ContextualInternalityError, ContextualMotive, issue_ambient_context_declaration_token,
};
use pen_type::dependent_context::{
    DEPENDENT_AMBIENT_CONTEXT_VERSION, DEPENDENT_TOTALITY_VERSION, DependentContextMotive,
};
use pen_type::elaborate::{KernelTy, SealedSignature, candidate_hash, elaborate_telescope};
use pen_type::equality::{KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use pen_type::motive_parametric_coherence_v2::CLOSURE_RULE_INVENTORY_V2;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA: &str = "r-t2-future-hole-confluence-v2";
pub const R_T2_FUTURE_HOLE_CONFLUENCE_V2_DATE: &str = "2026-07-22";

const TIE_PROTOCOL_BYTES: &[u8] = include_bytes!("../../../docs/tie_resolution_protocol.md");
const FUTURE_HOLE_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/future_hole_definition_adjudication.md");
const DEPENDENT_CONTEXT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/dependent_context_adjudication.md");
const DEPENDENT_CONTEXT_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/dependent_context.rs");
const V1_POST_AUDIT_BYTES: &[u8] =
    include_bytes!("../../../docs/R_T2_FUTURE_HOLE_CONFLUENCE_RESULT.md");
const V1_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/r_t2_future_hole_confluence_v1.json");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const A3_EXHAUSTIVENESS_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/a3_rule_inventory_exhaustiveness.rs");
const A3_EXHAUSTIVENESS_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/a3_rule_inventory_exhaustiveness_v2.json");
const FUTURE_HOLE_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/future_hole_hypothesis.rs");
const FUTURE_HOLE_V2_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/future_hole_hypothesis_v2.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const ADMISSIBILITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");
const AMBIENT_FORMER_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/ambient_former_internality.rs");
const CONTEXTUAL_INTERNALITY_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/contextual_internality.rs");
const MOTIVE_PARAMETRIC_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence.rs");
const MOTIVE_PARAMETRIC_V2_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence_v2.rs");
const SUBSTITUTION_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/substitution.rs");
const TRANSPORT_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const V1_SOURCE_BYTES: &[u8] = include_bytes!("r_t2_future_hole_confluence.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("r_t2_future_hole_confluence_v2.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2CoverageGate {
    pub theorem_schema: String,
    pub theorem_result_digest: String,
    pub theorem_replay_valid: bool,
    pub exact_prefix_inventory_proof_hash: String,
    pub relative_domain: String,
    pub expected_seed_keys: Vec<String>,
    pub covered_seed_keys: Vec<String>,
    pub missing_seed_keys: Vec<String>,
    pub unexpected_seed_keys: Vec<String>,
    pub expected_constructor_keys: Vec<String>,
    pub covered_constructor_keys: Vec<String>,
    pub missing_constructor_keys: Vec<String>,
    pub unexpected_constructor_keys: Vec<String>,
    pub full_relative_seed_coverage: bool,
    pub full_relative_constructor_coverage: bool,
    pub all_generated_schemes_accounted_for: bool,
    pub certified_promoted_scheme_ids: Vec<String>,
    pub compared_scheme_ids: Vec<String>,
    pub exact_compared_scheme_set_equals_inventory_promotions: bool,
    pub named_gaps: Vec<String>,
    pub zero_named_gaps: bool,
    pub full_relative_a3_seed_constructor_coverage: bool,
    pub full_adopted_frozen_semantic_scope_exhaustive: bool,
    pub unresolved_in_domain_semantic_scope_premises: Vec<String>,
    pub disclosed_nonblocking_scope_and_archive_limits: Vec<String>,
    pub law_level_promotion_authorized: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2FamilyPresentation {
    pub source_anchor_audit_only: String,
    pub canonical_normal_form: Expr,
    pub declared_parameter_sorts_json: String,
    pub declared_parameter_arity: u32,
    pub equality_scope_len: u32,
    pub equality_scope_source: String,
    pub maximum_free_variable_reference_used_as_scope: bool,
    pub kernel_type_json: String,
    pub presentation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2SchemeFormation {
    pub local_scheme_id: String,
    pub local_instance_id: String,
    pub rule_constructor: String,
    pub origin_kind: String,
    pub support_depth: u8,
    pub source_families: Vec<Rt2V2FamilyPresentation>,
    pub declared_source_family_count: usize,
    pub output_shape_json: String,
    pub registered_future_semantics_key: String,
    pub registered_future_semantics_formation_hash: String,
    pub registered_future_semantics_joined: bool,
    pub registered_future_semantics_family_slot: Option<usize>,
    pub registered_future_semantics_body_compared_by_family_quotient: bool,
    pub branch_local_family_id_used_in_semantic_key: bool,
    pub formation_derivation_hash: String,
    pub semantic_formation_class_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2OpaqueMotiveReconstruction {
    pub source_anchor_id: String,
    pub source_step: u32,
    pub source_clause_index: u16,
    pub source_candidate_hash: String,
    pub source_telescope_elaboration_hash: String,
    pub canonical_parameter_index: u32,
    pub canonical_parameter_sort_json: String,
    pub canonical_renaming_forward: Vec<(u32, u32)>,
    pub canonical_renaming_is_total_bijection: bool,
    pub canonical_index_has_unique_old_level_preimage: bool,
    pub old_level: Option<u32>,
    pub source_ambient_parameters: u32,
    pub maps_to_prior_clause: bool,
    pub prior_clause_index: Option<u16>,
    pub prior_clause_kernel_type_json: Option<String>,
    pub attempted_contextual_motive: Option<ContextualMotive>,
    pub attempted_motive_contains_forbidden_neutral_or_path: bool,
    pub exact_source_telescope_join: bool,
    pub exact_source_elaboration_join: bool,
    pub declaration_carrier: Telescope,
    pub declaration_motives: Vec<ContextualMotive>,
    pub motive_declaration_error_json: Option<String>,
    pub motive_not_formable_parameter: Option<u32>,
    pub full_declaration_ambient_limit_replayed: bool,
    pub isolated_motive_declaration_carrier: Telescope,
    pub isolated_motive_declaration_error_json: Option<String>,
    pub isolated_motive_not_formable_parameter: Option<u32>,
    pub motive_not_formable_canonical_parameter_index: Option<u32>,
    pub isolated_projection_is_diagnostic_not_original_full_context_judgment: bool,
    pub exact_motive_not_formable_replayed: bool,
    pub failure_disposition: String,
    pub current_contextual_api_expressivity_gap_f_fh4: bool,
    pub intrinsic_semantic_non_formability_claimed: bool,
    pub future_scheme_nonexistence_claimed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2FutureHoleGate {
    pub registered_open_judgment_schema: String,
    pub registration_replay_valid: bool,
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub rule_constructor: String,
    pub registration_kind: String,
    pub natural_family_id: String,
    pub occurrence_id: String,
    pub stage: u32,
    pub visible_library_at_registration: u32,
    pub ambient_registration_prefix_last_step: u32,
    pub ambient_registration_prefix_signature_digest: String,
    pub ambient_declaration_bound_to_registration_prefix: bool,
    pub exact_registration_prefix_bound: bool,
    pub constructor: Option<String>,
    pub constructor_side_condition_json: Option<String>,
    pub open_body: Option<Expr>,
    pub declared_motive_count: Option<usize>,
    pub dependent_context_version: Option<String>,
    pub dependent_context_declared_arity: Option<u32>,
    pub dependent_totality_theorem_replayed: bool,
    pub opaque_prior_clause_reference_count: usize,
    pub opaque_prior_clause_references_replayed_whole: bool,
    pub no_outcome_filtering_used: bool,
    pub output_contract_json: Option<String>,
    pub output_type_preserved: bool,
    pub declaration_projection_replayed: bool,
    pub parameter_projection_replayed: bool,
    pub hypothetical_derivation_replayable: bool,
    pub motive_parametric_theorem_version: String,
    pub motive_parametric_v1_success_row_provisional: bool,
    pub law_level_semantic_scope_satisfied: bool,
    pub upstream_soundness_blockers: Vec<String>,
    pub zero_registration_marginal_charge: bool,
    pub no_reflexivity_fallback: bool,
    pub semantic_scope_premises_satisfied: bool,
    pub external_exhaustiveness_evidence_hash: Option<String>,
    pub historical_realization_required_for_r_t2_formation: bool,
    pub named_gaps: Vec<String>,
    pub zero_named_gaps: bool,
    pub branch_independent_semantic_key: String,
    pub semantic_family_slot: Option<usize>,
    pub semantic_body_compared_by_family_quotient: bool,
    pub branch_local_family_id_used_in_semantic_key: bool,
    pub opaque_motive_reconstructions: Vec<Rt2V2OpaqueMotiveReconstruction>,
    pub formation_derivation_hash: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2FutureHoleAggregateGate {
    pub expected_future_semantics_scheme_ids: Vec<String>,
    pub expected_future_semantics_instance_ids: Vec<String>,
    pub registered_scheme_ids: Vec<String>,
    pub registered_instance_ids: Vec<String>,
    pub gap_scheme_ids: Vec<String>,
    pub unary_registration_count: usize,
    pub structural_registration_count: usize,
    pub registration_count: usize,
    pub exact_scheme_and_instance_coverage: bool,
    pub f_dc5_same_issuer_scope_preserved: bool,
    pub registrations: Vec<Rt2V2FutureHoleGate>,
    pub every_registration_replay_valid: bool,
    pub every_registration_hypothetically_derivable: bool,
    pub every_registration_zero_marginal_charge: bool,
    pub every_registration_no_reflexivity_fallback: bool,
    pub every_registration_semantic_scope_satisfied: bool,
    pub every_registration_inventory_bound: bool,
    pub every_registration_exact_prefix_bound: bool,
    pub successful_rows_provisional_due_motive_parametric_v1: bool,
    pub successful_rows_law_level_authoritative: bool,
    pub upstream_soundness_blockers: Vec<String>,
    pub historical_realization_required_for_r_t2_formation: bool,
    pub named_gaps: Vec<String>,
    pub zero_named_gaps: bool,
    pub aggregate_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2BranchFormation {
    pub r_t1_class_id: String,
    pub stage4_candidate_hash: String,
    pub stage4_telescope: Telescope,
    pub prefix_signature_digest: String,
    pub prefix_steps: Vec<u32>,
    pub prefix_steps_exactly_one_through_four: bool,
    pub independently_generated_stage5_window: bool,
    pub stage5_window_derivation_hash: String,
    pub stage5_newest_step: u32,
    pub stage5_older_step: u32,
    pub coverage_gate: Rt2V2CoverageGate,
    pub future_hole_aggregate: Rt2V2FutureHoleAggregateGate,
    pub every_instance_typed: bool,
    pub every_scheme_has_formation_evidence: bool,
    pub schemes: Vec<Rt2V2SchemeFormation>,
    pub complete_certified_scheme_count: usize,
    pub scheme_formation_class_ids: Vec<String>,
    pub complete_formation_set_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2FamilyQuotientComparison {
    pub left_source_anchor_audit_only: String,
    pub right_source_anchor_audit_only: String,
    pub declared_parameter_arities_equal: bool,
    pub equality_scope_len: Option<u32>,
    pub equality_scope_derived_only_from_declared_parameter_arity: bool,
    pub maximum_free_variable_reference_used_as_scope: bool,
    pub parameter_sorts_equal: bool,
    pub kernel_types_equal: bool,
    pub frozen_equality: Option<FrozenEqualityRecord>,
    pub equivalent: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2SchemeFormationComparison {
    pub left_scheme_id: String,
    pub right_scheme_id: String,
    pub rule_constructor_equal: bool,
    pub origin_kind_equal: bool,
    pub support_depth_equal: bool,
    pub declared_source_family_counts_equal: bool,
    pub source_family_comparisons: Vec<Rt2V2FamilyQuotientComparison>,
    pub output_shape_equal: bool,
    pub registered_future_semantics_joined_both: bool,
    pub registered_future_semantics_bound_to_family_quotient: bool,
    pub registered_future_semantic_keys_equal: bool,
    pub equivalent_under_certified_family_quotient: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2PairwiseSchemeSetComparison {
    pub left_candidate_hash: String,
    pub right_candidate_hash: String,
    pub left_complete_scheme_count: usize,
    pub right_complete_scheme_count: usize,
    pub comparisons: Vec<Rt2V2SchemeFormationComparison>,
    pub perfect_matching: Vec<(String, String)>,
    pub unmatched_left_formation_classes: Vec<String>,
    pub unmatched_right_formation_classes: Vec<String>,
    pub full_scheme_sets_equivalent: bool,
    pub comparison_well_formed: bool,
    pub relative_coverage_and_zero_gap_gate_passed: bool,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2OrderIndependentPairVerdict {
    pub unordered_candidate_pair: (String, String),
    pub left_scheme_count: usize,
    pub right_scheme_count: usize,
    pub matched_count: usize,
    pub equivalent: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2V2OrderReversalAudit {
    pub forward_branch_order: Vec<String>,
    pub reversed_branch_order: Vec<String>,
    pub forward_normalized_pair_verdicts: Vec<Rt2V2OrderIndependentPairVerdict>,
    pub reversed_normalized_pair_verdicts: Vec<Rt2V2OrderIndependentPairVerdict>,
    pub forward_complete_set_hashes: Vec<String>,
    pub reversed_complete_set_hashes: Vec<String>,
    pub exact_pair_verdict_invariance: bool,
    pub exact_complete_set_invariance: bool,
    pub selection_invariant: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Rt2FutureHoleConfluenceV2Outcome {
    RelativeCoverageOrNamedGapBlocker,
    ScopedInequivalenceAdoptedFrozenScopePremisePending,
    LawLevelConfluenceRefutedRungRt3Opened,
    ImmediateSchemeSetsEquivalentFutureIsomorphismPending,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2FutureHoleConfluenceV2Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Rt2V2SourceBinding>,
    pub tie_protocol_adoption_replayed: bool,
    pub future_hole_adoption_replayed: bool,
    pub v1_post_audit_blocker_replayed: bool,
    pub equality_procedure: String,
    pub archived_v1_artifact_digest_verified: bool,
    pub v1_certificate_digest: String,
    pub v1_replay_valid: bool,
    pub a3_inventory_exhaustiveness_certificate_digest: String,
    pub a3_inventory_exhaustiveness_replay_valid: bool,
    pub a3_inventory_exhaustiveness_archived_artifact_replay_valid: bool,
    pub registered_future_hole_aggregate_digest: String,
    pub all_registered_future_holes_replay_valid: bool,
    pub naturality_orbit_transport_certificate_digest: String,
    pub naturality_orbit_transport_replay_valid: bool,
    pub exact_four_way_r_t1_class_join: bool,
    pub branch_count: usize,
    pub branches: Vec<Rt2V2BranchFormation>,
    pub four_exact_stage5_prefixes_independently_generated: bool,
    pub full_relative_a3_seed_constructor_coverage_every_branch: bool,
    pub zero_named_gaps_every_branch: bool,
    pub full_adopted_frozen_semantic_scope_exhaustive_every_branch: bool,
    pub unresolved_in_domain_semantic_scope_premises: Vec<String>,
    pub disclosed_nonblocking_scope_and_archive_limits: Vec<String>,
    pub all_family_equality_scopes_from_declared_parameter_arity: bool,
    pub maximum_free_variable_reference_used_as_scope: bool,
    pub pairwise_scheme_set_comparisons: Vec<Rt2V2PairwiseSchemeSetComparison>,
    pub every_pairwise_comparison_well_formed: bool,
    pub all_stage5_scheme_sets_equivalent: bool,
    pub order_reversal_audit: Rt2V2OrderReversalAudit,
    pub order_reversal_invariant: bool,
    pub immediate_inequivalent_successor_on_certified_relative_surface: bool,
    pub law_level_promotion_gate_passed: bool,
    pub deterministic_future_isomorphism_replayed: bool,
    pub r_t2_confluence_proved: bool,
    pub r_t2_confluence_refuted: bool,
    pub r_t3_user_adjudication_opened: bool,
    pub selected_candidate_hash: Option<String>,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub desired_history_count_score_or_bar_used_as_premise: bool,
    pub theorem_t_bf1_proved: bool,
    pub theorem_t_bf3_proved: bool,
    pub bar_free_adoption_authorized: bool,
    pub bridge_authorized: bool,
    pub halt_claim_issued: bool,
    pub outcome: Rt2FutureHoleConfluenceV2Outcome,
    pub serialized_blockers: Vec<String>,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rt2FutureHoleConfluenceV2Replay {
    pub valid: bool,
    pub outcome: Option<Rt2FutureHoleConfluenceV2Outcome>,
    pub branch_count: usize,
    pub relative_coverage_gate_passed: bool,
    pub zero_named_gaps: bool,
    pub full_adopted_frozen_semantic_scope_gate_passed: bool,
    pub order_reversal_invariant: bool,
    pub all_stage5_scheme_sets_equivalent: bool,
    pub r_t2_confluence_proved: bool,
    pub r_t2_confluence_refuted: bool,
    pub r_t3_user_adjudication_opened: bool,
    pub serialized_blockers: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Rt2FutureHoleConfluenceV2Error {
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

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA, domain, value))
        .expect("R-T2 v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn v1_certificate_digest(certificate: &Rt2FutureHoleConfluenceCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(
        R_T2_FUTURE_HOLE_CONFLUENCE_SCHEMA,
        "certificate",
        projection,
    ))
    .expect("R-T2 v1 certificate projection serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<Rt2V2SourceBinding> {
    [
        (
            "docs/tie_resolution_protocol.md",
            "adopted_quotient_confluence_adjudication_ladder",
            TIE_PROTOCOL_BYTES,
        ),
        (
            "docs/future_hole_definition_adjudication.md",
            "adopted_registered_open_judgment_semantics",
            FUTURE_HOLE_ADJUDICATION_BYTES,
        ),
        (
            "docs/dependent_context_adjudication.md",
            "adopted_dependent_telescope_arity_and_opaque_reference_extension",
            DEPENDENT_CONTEXT_ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-type/src/dependent_context.rs",
            "dependent_context_declaration_and_total_specialization_kernel",
            DEPENDENT_CONTEXT_SOURCE_BYTES,
        ),
        (
            "docs/R_T2_FUTURE_HOLE_CONFLUENCE_RESULT.md",
            "immutable_v1_scoped_counterexample_and_post_audit_blocker",
            V1_POST_AUDIT_BYTES,
        ),
        (
            "docs/r_t2_future_hole_confluence_v1.json",
            "immutable_v1_scoped_counterexample_certificate",
            V1_ARTIFACT_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "count_blind_exact_prefix_stage5_generator",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_rule_inventory_exhaustiveness.rs",
            "independent_relative_a3_seed_constructor_inventory_theorem",
            A3_EXHAUSTIVENESS_SOURCE_BYTES,
        ),
        (
            "docs/a3_rule_inventory_exhaustiveness_v2.json",
            "create_new_relative_a3_seed_constructor_inventory_certificate",
            A3_EXHAUSTIVENESS_ARTIFACT_BYTES,
        ),
        (
            "crates/pen-eval/src/future_hole_hypothesis.rs",
            "immutable_future_hole_v1_predecessor",
            FUTURE_HOLE_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/future_hole_hypothesis_v2.rs",
            "registered_body_motive_open_judgment_v2_issuer_and_replay",
            FUTURE_HOLE_V2_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/equality.rs",
            "frozen_univalent_equality_procedure",
            EQUALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/elaborate.rs",
            "explicit_context_body_typing_and_prefix_sealing",
            ELABORATE_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/admissibility.rs",
            "guarded_structural_provider_contract",
            ADMISSIBILITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/ambient_former_internality.rs",
            "registered_transparent_former_closure",
            AMBIENT_FORMER_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/contextual_internality.rs",
            "ambient_motive_declaration_and_prefix_locality",
            CONTEXTUAL_INTERNALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence.rs",
            "immutable_v1_generic_substitution_predecessor",
            MOTIVE_PARAMETRIC_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence_v2.rs",
            "proof_strength_exact_source_eliminator_and_open_universality_blocker",
            MOTIVE_PARAMETRIC_V2_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/substitution.rs",
            "typed_structural_substitution_replay",
            SUBSTITUTION_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "four_r_t1_classes_and_frozen_family_quotient",
            TRANSPORT_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/r_t2_future_hole_confluence.rs",
            "immutable_v1_scoped_scheme_set_comparison",
            V1_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/r_t2_future_hole_confluence_v2.rs",
            "blocker_aware_declared_arity_law_level_rerun",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Rt2V2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn archived_v1_certificate()
-> Result<Rt2FutureHoleConfluenceCertificate, Rt2FutureHoleConfluenceV2Error> {
    serde_json::from_slice(V1_ARTIFACT_BYTES)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))
}

fn archived_a3_exhaustiveness_certificate()
-> Result<A3RuleInventoryExhaustivenessCertificate, Rt2FutureHoleConfluenceV2Error> {
    serde_json::from_slice(A3_EXHAUSTIVENESS_ARTIFACT_BYTES)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))
}

fn replay_adoptions() -> Result<(), Rt2FutureHoleConfluenceV2Error> {
    let protocol = std::str::from_utf8(TIE_PROTOCOL_BYTES)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
    for clause in [
        "R-T1",
        "R-T2",
        "Confluence second",
        "R-T3",
        "Enumeration order is not an admissible option",
        "I adopt the tie-resolution protocol",
    ] {
        if !protocol.contains(clause) {
            return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(format!(
                "tie protocol omits {clause:?}"
            )));
        }
    }
    let future = std::str::from_utf8(FUTURE_HOLE_ADJUDICATION_BYTES)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
    for clause in [
        "future-hole-hypothesis-definition-v1",
        "Stage successors (R-T2's instrument)",
        "holes and fillings mint nothing",
        "I adopt `future-hole-hypothesis-definition-v1`",
    ] {
        if !future.contains(clause) {
            return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(format!(
                "future-hole adjudication omits {clause:?}"
            )));
        }
    }
    let dependent = std::str::from_utf8(DEPENDENT_CONTEXT_ADJUDICATION_BYTES)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
    for clause in [
        "dependent-ambient-context-v1",
        "Sequential typed substitution",
        "Declared arity",
        "Opaque prior-clause motives",
        "F-DC5 (rerun identity)",
        "I adopt `dependent-ambient-context-v1`",
    ] {
        if !dependent.contains(clause) {
            return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(format!(
                "dependent-context adjudication omits {clause:?}"
            )));
        }
    }
    let post_audit = std::str::from_utf8(V1_POST_AUDIT_BYTES)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
    for clause in [
        "V1 SCOPED COUNTEREXAMPLE; LAW-LEVEL R-T2 STILL OPEN",
        "full-A3",
        "exhaustiveness theorem",
        "R-T3 remains closed",
    ] {
        if !post_audit.contains(clause) {
            return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(format!(
                "v1 post-audit record omits {clause:?}"
            )));
        }
    }
    Ok(())
}

fn prefix_signature(stage4: &Telescope) -> SealedSignature {
    let mut telescopes = (1..=3)
        .map(|step| (step, Telescope::reference(step)))
        .collect::<Vec<_>>();
    telescopes.push((4, stage4.clone()));
    SealedSignature::from_telescopes(telescopes)
}

fn coverage_gate(
    global: &A3RuleInventoryExhaustivenessCertificate,
    global_replay_valid: bool,
    proof: &A3WindowRuleInventoryProof,
    compared_scheme_ids: &BTreeSet<String>,
) -> Rt2V2CoverageGate {
    let mut expected_seed_keys = proof
        .base_seed_dispositions
        .iter()
        .map(|row| format!("base::{}", row.independent_seed_id))
        .chain(
            proof
                .structural_seed_dispositions
                .iter()
                .map(|row| format!("structural::{}", row.structural_family)),
        )
        .collect::<Vec<_>>();
    expected_seed_keys.sort();
    let mut covered_seed_keys = proof
        .base_seed_dispositions
        .iter()
        .filter(|row| row.exactly_one_promotion_or_rejection)
        .map(|row| format!("base::{}", row.independent_seed_id))
        .chain(
            proof
                .structural_seed_dispositions
                .iter()
                .filter(|row| row.exactly_one_promotion_or_rejection)
                .map(|row| format!("structural::{}", row.structural_family)),
        )
        .collect::<Vec<_>>();
    covered_seed_keys.sort();
    let expected_seed_set = expected_seed_keys.iter().cloned().collect::<BTreeSet<_>>();
    let covered_seed_set = covered_seed_keys.iter().cloned().collect::<BTreeSet<_>>();
    let missing_seed_keys = expected_seed_set
        .difference(&covered_seed_set)
        .cloned()
        .collect::<Vec<_>>();
    let mut unexpected_seed_keys = covered_seed_set
        .difference(&expected_seed_set)
        .cloned()
        .collect::<Vec<_>>();
    if !proof.every_operational_instance_has_exactly_one_independent_preimage {
        unexpected_seed_keys
            .push("UNIDENTIFIED_OPERATIONAL_INSTANCE_WITHOUT_INDEPENDENT_PREIMAGE".to_owned());
    }

    let expected_constructor_keys = global
        .shape_mappings
        .iter()
        .map(|row| row.operational_rule_constructor.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut covered_constructor_set = proof
        .base_kind_aggregates
        .iter()
        .filter(|row| row.operational_disposition_exactly_replayed)
        .map(|row| row.mapped_rule_constructor.clone())
        .collect::<BTreeSet<_>>();
    if proof.structural_future_hole_clause_coverage_complete {
        covered_constructor_set.extend(
            proof
                .structural_seed_dispositions
                .iter()
                .map(|row| row.mapped_rule_constructor.clone()),
        );
    }
    let covered_constructor_keys = covered_constructor_set.iter().cloned().collect::<Vec<_>>();
    let expected_constructor_set = expected_constructor_keys
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let missing_constructor_keys = expected_constructor_set
        .difference(&covered_constructor_set)
        .cloned()
        .collect::<Vec<_>>();
    let unexpected_constructor_keys = covered_constructor_set
        .difference(&expected_constructor_set)
        .cloned()
        .collect::<Vec<_>>();

    let full_relative_seed_coverage = missing_seed_keys.is_empty()
        && unexpected_seed_keys.is_empty()
        && proof.every_independent_seed_has_exactly_one_disposition
        && proof.every_operational_instance_has_exactly_one_independent_preimage;
    let full_relative_constructor_coverage = missing_constructor_keys.is_empty()
        && unexpected_constructor_keys.is_empty()
        && proof.operational_seed_dispositions_replayed
        && proof.structural_future_hole_clause_coverage_complete;
    let certified_promoted_scheme_ids = proof
        .base_seed_dispositions
        .iter()
        .filter_map(|row| row.promoted_scheme_id.clone())
        .chain(
            proof
                .structural_seed_dispositions
                .iter()
                .filter_map(|row| row.structural_scheme_id.clone()),
        )
        .collect::<BTreeSet<_>>();
    let exact_compared_scheme_set_equals_inventory_promotions =
        &certified_promoted_scheme_ids == compared_scheme_ids;
    let all_generated_schemes_accounted_for = proof.every_operational_scheme_and_orbit_is_consumed
        && exact_compared_scheme_set_equals_inventory_promotions;
    let mut named_gaps = Vec::new();
    if !global_replay_valid {
        named_gaps.push("A3_RELATIVE_INVENTORY_CERTIFICATE_REPLAY_FAILED".to_owned());
    }
    if !global.relative_rule_constructor_inventory_exhaustiveness_proved {
        named_gaps.push("A3_GLOBAL_RELATIVE_RULE_CONSTRUCTOR_EXHAUSTIVENESS_FAILED".to_owned());
    }
    if !proof.relative_rule_inventory_exhaustive_for_window {
        named_gaps.push("A3_EXACT_PREFIX_RELATIVE_RULE_INVENTORY_EXHAUSTIVENESS_FAILED".to_owned());
    }
    if !full_relative_seed_coverage {
        named_gaps.push("A3_EXACT_PREFIX_SEED_COVERAGE_FAILED".to_owned());
    }
    if !full_relative_constructor_coverage {
        named_gaps.push("A3_EXACT_PREFIX_CONSTRUCTOR_COVERAGE_FAILED".to_owned());
    }
    if !all_generated_schemes_accounted_for {
        named_gaps.push("A3_EXACT_PREFIX_OPERATIONAL_SCHEME_REVERSE_JOIN_FAILED".to_owned());
    }
    if !global.completed_schema2_artifact_digest_verified
        || !global.every_completed_schema_source_is_an_a3_typed_parameter
    {
        named_gaps.push("A3_FROZEN_SCHEMA2_SOURCE_PARAMETER_JOIN_FAILED".to_owned());
    }
    if !global.global_e4_wrapped_domain_class_exhaustion_bound
        || !global.global_e4_no_unknown_survives_on_frozen_wrapped_surface
    {
        named_gaps.push("A3_FROZEN_WRAPPED_SURFACE_EXHAUSTION_FAILED".to_owned());
    }
    named_gaps.sort();
    named_gaps.dedup();
    let zero_named_gaps = named_gaps.is_empty();
    let full_relative_a3_seed_constructor_coverage = full_relative_seed_coverage
        && full_relative_constructor_coverage
        && all_generated_schemes_accounted_for;
    let full_adopted_frozen_semantic_scope_exhaustive = global_replay_valid
        && global.relative_rule_constructor_inventory_exhaustiveness_proved
        && proof.relative_rule_inventory_exhaustive_for_window
        && full_relative_a3_seed_constructor_coverage
        && global.completed_schema2_artifact_digest_verified
        && global.every_completed_schema_source_is_an_a3_typed_parameter
        && global.global_e4_wrapped_domain_class_exhaustion_bound
        && global.global_e4_no_unknown_survives_on_frozen_wrapped_surface;
    let unresolved_in_domain_semantic_scope_premises = named_gaps.clone();
    let disclosed_nonblocking_scope_and_archive_limits = global.named_scope_limits.clone();
    let law_level_promotion_authorized = full_adopted_frozen_semantic_scope_exhaustive
        && zero_named_gaps
        && unresolved_in_domain_semantic_scope_premises.is_empty();
    let mut gate = Rt2V2CoverageGate {
        theorem_schema: global.schema.clone(),
        theorem_result_digest: global.result_digest.clone(),
        theorem_replay_valid: global_replay_valid,
        exact_prefix_inventory_proof_hash: proof.derivation_hash.clone(),
        relative_domain: "adopted_depth_two_a3_grammar_over_frozen_wrapped_global_e4_surface"
            .to_owned(),
        expected_seed_keys,
        covered_seed_keys,
        missing_seed_keys,
        unexpected_seed_keys,
        expected_constructor_keys,
        covered_constructor_keys,
        missing_constructor_keys,
        unexpected_constructor_keys,
        full_relative_seed_coverage,
        full_relative_constructor_coverage,
        all_generated_schemes_accounted_for,
        certified_promoted_scheme_ids: certified_promoted_scheme_ids.into_iter().collect(),
        compared_scheme_ids: compared_scheme_ids.iter().cloned().collect(),
        exact_compared_scheme_set_equals_inventory_promotions,
        named_gaps,
        zero_named_gaps,
        full_relative_a3_seed_constructor_coverage,
        full_adopted_frozen_semantic_scope_exhaustive,
        unresolved_in_domain_semantic_scope_premises,
        disclosed_nonblocking_scope_and_archive_limits,
        law_level_promotion_authorized,
        derivation_hash: String::new(),
    };
    gate.derivation_hash = tagged_hash("branch-relative-a3-coverage-gate", &gate);
    gate
}

fn declared_parameter_arity(
    parameter_sorts_json: &str,
) -> Result<u32, Rt2FutureHoleConfluenceV2Error> {
    let parameters = serde_json::from_str::<Vec<serde_json::Value>>(parameter_sorts_json)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?;
    u32::try_from(parameters.len()).map_err(|_| {
        Rt2FutureHoleConfluenceV2Error::Invariant(
            "declared canonical-parameter arity exceeds u32".to_owned(),
        )
    })
}

fn family_from_v1(
    family: &SchemeFamilyPresentation,
) -> Result<Rt2V2FamilyPresentation, Rt2FutureHoleConfluenceV2Error> {
    let declared_parameter_arity = declared_parameter_arity(&family.parameter_sorts_json)?;
    let presentation_hash = tagged_hash(
        "declared-arity-family-presentation",
        &(
            &family.canonical_normal_form,
            &family.parameter_sorts_json,
            declared_parameter_arity,
            &family.kernel_type_json,
        ),
    );
    Ok(Rt2V2FamilyPresentation {
        source_anchor_audit_only: family.source_anchor_audit_only.clone(),
        canonical_normal_form: family.canonical_normal_form.clone(),
        declared_parameter_sorts_json: family.parameter_sorts_json.clone(),
        declared_parameter_arity,
        equality_scope_len: declared_parameter_arity,
        equality_scope_source: "A3TypedClauseSource.canonical_presentation.parameters.len"
            .to_owned(),
        maximum_free_variable_reference_used_as_scope: false,
        kernel_type_json: family.kernel_type_json.clone(),
        presentation_hash,
    })
}

fn scheme_from_v1(
    scheme: &Stage5SchemeFormation,
    registration: &Rt2V2FutureHoleGate,
) -> Result<Rt2V2SchemeFormation, Rt2FutureHoleConfluenceV2Error> {
    if registration.a3_scheme_id != scheme.local_scheme_id
        || registration.a3_instance_id != scheme.local_instance_id
    {
        return Err(Rt2FutureHoleConfluenceV2Error::Invariant(format!(
            "registered future semantics does not join scheme/instance {} / {}",
            scheme.local_scheme_id, scheme.local_instance_id
        )));
    }
    let source_families = scheme
        .source_families
        .iter()
        .map(family_from_v1)
        .collect::<Result<Vec<_>, _>>()?;
    let declared_source_family_count = source_families.len();
    let semantic_formation_class_id = tagged_hash(
        "declared-arity-stage5-scheme-formation-class",
        &(
            &scheme.rule_constructor,
            &scheme.origin_kind,
            scheme.support_depth,
            &source_families
                .iter()
                .map(|family| {
                    (
                        &family.canonical_normal_form,
                        &family.declared_parameter_sorts_json,
                        family.declared_parameter_arity,
                        &family.kernel_type_json,
                    )
                })
                .collect::<Vec<_>>(),
            &scheme.output_shape_json,
            &registration.branch_independent_semantic_key,
        ),
    );
    Ok(Rt2V2SchemeFormation {
        local_scheme_id: scheme.local_scheme_id.clone(),
        local_instance_id: scheme.local_instance_id.clone(),
        rule_constructor: scheme.rule_constructor.clone(),
        origin_kind: scheme.origin_kind.clone(),
        support_depth: scheme.support_depth,
        source_families,
        declared_source_family_count,
        output_shape_json: scheme.output_shape_json.clone(),
        registered_future_semantics_key: registration.branch_independent_semantic_key.clone(),
        registered_future_semantics_formation_hash: registration
            .formation_derivation_hash
            .clone()
            .unwrap_or_default(),
        registered_future_semantics_joined: registration.zero_named_gaps
            && registration.registration_replay_valid
            && registration.hypothetical_derivation_replayable
            && registration.law_level_semantic_scope_satisfied
            && registration.formation_derivation_hash.is_some(),
        registered_future_semantics_family_slot: registration.semantic_family_slot,
        registered_future_semantics_body_compared_by_family_quotient: registration
            .semantic_body_compared_by_family_quotient,
        branch_local_family_id_used_in_semantic_key: registration
            .branch_local_family_id_used_in_semantic_key,
        formation_derivation_hash: scheme.formation_derivation_hash.clone(),
        semantic_formation_class_id,
    })
}

fn branch_from_v1(
    branch: &BranchStage5Formation,
    coverage_gate: Rt2V2CoverageGate,
    future_hole_aggregate: Rt2V2FutureHoleAggregateGate,
) -> Result<Rt2V2BranchFormation, Rt2FutureHoleConfluenceV2Error> {
    let mut schemes = branch
        .schemes
        .iter()
        .map(|scheme| {
            let registrations = future_hole_aggregate
                .registrations
                .iter()
                .filter(|registration| registration.a3_scheme_id == scheme.local_scheme_id)
                .collect::<Vec<_>>();
            if registrations.len() != 1 {
                return Err(Rt2FutureHoleConfluenceV2Error::Invariant(format!(
                    "scheme {} has {} registered future-semantics rows instead of one",
                    scheme.local_scheme_id,
                    registrations.len()
                )));
            }
            scheme_from_v1(scheme, registrations[0])
        })
        .collect::<Result<Vec<_>, _>>()?;
    schemes.sort_by(|left, right| {
        left.semantic_formation_class_id
            .cmp(&right.semantic_formation_class_id)
            .then(left.local_scheme_id.cmp(&right.local_scheme_id))
    });
    let scheme_formation_class_ids = schemes
        .iter()
        .map(|scheme| scheme.semantic_formation_class_id.clone())
        .collect::<Vec<_>>();
    let complete_formation_set_hash = tagged_hash(
        "complete-relative-stage5-formation-set",
        &scheme_formation_class_ids,
    );
    let prefix_steps = vec![1, 2, 3, 4];
    Ok(Rt2V2BranchFormation {
        r_t1_class_id: branch.r_t1_class_id.clone(),
        stage4_candidate_hash: branch.stage4_candidate_hash.clone(),
        stage4_telescope: branch.stage4_telescope.clone(),
        prefix_signature_digest: branch.prefix_signature_digest.clone(),
        prefix_steps,
        prefix_steps_exactly_one_through_four: branch.prefix_steps_exactly_one_through_four,
        independently_generated_stage5_window: true,
        stage5_window_derivation_hash: branch.stage5_window_derivation_hash.clone(),
        stage5_newest_step: branch.stage5_newest_step,
        stage5_older_step: branch.stage5_older_step,
        coverage_gate,
        future_hole_aggregate,
        every_instance_typed: branch.every_instance_typed,
        every_scheme_has_formation_evidence: branch.every_scheme_has_formation_evidence,
        complete_certified_scheme_count: schemes.len(),
        schemes,
        scheme_formation_class_ids,
        complete_formation_set_hash,
    })
}

fn registered_future_hole_gate(
    scheme: &A3TypedDemandScheme,
    registered: &RegisteredFutureHoleV2,
    replay_valid: bool,
    expected_prefix_signature_digest: &str,
) -> Result<Rt2V2FutureHoleGate, Rt2FutureHoleConfluenceV2Error> {
    let rule_constructor = serde_json::to_string(&scheme.rule_constructor)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?;
    let output_contract_json = serde_json::to_string(&registered.output_contract)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?;
    let (
        registration_kind,
        constructor,
        constructor_side_condition_json,
        open_body,
        contract_no_reflexivity,
        semantic_contract,
        semantic_family_slot,
        semantic_body_compared_by_family_quotient,
    ) = match (&registered.body, &registered.output_contract) {
        (
            FutureHoleBodyV2::UnaryAction {
                sealed_operator_family_key: _,
                operator_template: _,
                fresh_hole_arguments,
                instantiated_body,
                ..
            },
            FutureHoleOutputContractV2::UnaryActionAt {
                source_family_key: _,
                expected_kernel_type,
                no_reflexivity_rule,
            },
        ) if scheme.rule_constructor == A3RuleConstructor::UnaryAction => (
            "unary_action".to_owned(),
            None,
            None,
            Some(instantiated_body.clone()),
            *no_reflexivity_rule,
            serde_json::json!({
                "kind": "unary_action",
                "source_family_slot": 0,
                "operator_template_compared_by_source_family_quotient": true,
                "fresh_hole_argument_shape": fresh_hole_arguments
                    .iter()
                    .map(|argument| (
                        argument.source_parameter,
                        argument.fresh_parameter,
                        argument.occurs_live_in_body,
                    ))
                    .collect::<Vec<_>>(),
                "expected_kernel_type": expected_kernel_type,
                "no_reflexivity_rule": no_reflexivity_rule,
            }),
            Some(0),
            true,
        ),
        (
            FutureHoleBodyV2::StructuralHypothesis { body, .. },
            FutureHoleOutputContractV2::StructuralProvides(contract),
        ) if scheme.rule_constructor == A3RuleConstructor::StructuralCompletionHole => {
            let constructor = serde_json::to_string(&contract.constructor)
                .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?;
            let side_condition = serde_json::to_string(contract)
                .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?;
            let jurisdiction_semantics = match &contract.jurisdiction {
                StructuralJurisdictionV2::Direct { stage } => {
                    serde_json::json!({"kind": "direct", "stage": stage})
                }
                StructuralJurisdictionV2::Stage3To4 {
                    registration_stage,
                    jurisdiction_stage,
                    ..
                } => serde_json::json!({
                    "kind": "stage3_to_4",
                    "registration_stage": registration_stage,
                    "jurisdiction_stage": jurisdiction_stage,
                }),
            };
            (
                "structural_completion".to_owned(),
                Some(constructor),
                Some(side_condition),
                Some(body.clone()),
                true,
                serde_json::json!({
                    "kind": "structural_completion",
                    "open_body": body,
                    "provider_family": contract.provider_family,
                    "mode": contract.mode,
                    "required_policy": contract.required_policy,
                    "provider_registry_version": contract.provider_registry_version,
                    "provider_registry_source_hash": contract.provider_registry_source_hash,
                    "jurisdiction": jurisdiction_semantics,
                }),
                None,
                false,
            )
        }
        _ => {
            return Err(Rt2FutureHoleConfluenceV2Error::Invariant(format!(
                "registered body/output contract does not match scheme {} constructor",
                scheme.scheme_id
            )));
        }
    };
    let declaration_projection_replayed = registered
        .ambient_declaration
        .every_motive_formable_over_predecessors
        && registered.ambient_declaration.declared_arity
            == u32::try_from(registered.declared_motives.len()).unwrap_or(u32::MAX)
        && registered.declared_ambient_arity == registered.ambient_declaration.declared_arity
        && registered.ambient_declaration.body_telescope == registered.body_telescope
        && registered.ambient_declaration.candidate_hash
            == candidate_hash(&registered.body_telescope)
        && registered.ambient_declaration.declared_role
            == registered.body_telescope.clauses[0].role
        && registered.ambient_declaration.expression == registered.body_telescope.clauses[0].expr
        && registered.parametric_internality.explicit_ambient_arity
            == registered.declared_ambient_arity
        && registered.ambient_declaration.no_outcome_filtering_used
        && registered.ambient_declaration.opaque_references_used_whole
        && !registered.ambient_declaration.declaration_hash.is_empty();
    let exact_source_projection_replayed = registered
        .parametric_internality
        .source_bound_to_exact_body
        && registered.parametric_internality.exact_body_candidate_hash
            == candidate_hash(&registered.body_telescope)
        && registered
            .parametric_internality
            .historical_prefix_declaration_hash
            == registered.ambient_declaration.declaration_hash
        && registered.parametric_internality.dependent_declaration_hash
            == registered.ambient_declaration.declaration_hash
        && registered.ambient_declaration.expression == registered.body_telescope.clauses[0].expr
        && registered.ambient_declaration.signature_digest
            == registered.ambient_registration_prefix_signature_digest
        && registered.ambient_declaration.visible_library
            == registered.visible_library_at_registration
        && registered
            .parametric_internality
            .dependent_totality_theorem_replayed
        && registered
            .parametric_internality
            .total_specialization_authoritative
        && registered
            .parametric_internality
            .dependent_totality_theorem
            .total_specialization_theorem_issued
        && registered
            .parametric_internality
            .dependent_totality_theorem
            .declaration_hash
            == registered.ambient_declaration.declaration_hash
        && registered
            .parametric_internality
            .closure_rule_inventory
            .len()
            == CLOSURE_RULE_INVENTORY_V2.len();
    let parameter_projection_replayed = registered
        .parametric_internality
        .every_declared_parameter_live
        && registered
            .parametric_internality
            .every_body_former_registered
        && exact_source_projection_replayed
        && registered
            .parametric_internality
            .hypothetical_internality_issued
        && registered.parametric_internality.marginal_nu == 0
        && !registered.parametric_internality.evidence_hash.is_empty();
    let motive_parametric_theorem_version = registered
        .parametric_internality
        .dependent_totality_theorem
        .version
        .clone();
    let motive_parametric_v1_success_row_provisional =
        motive_parametric_theorem_version == "motive-parametric-instantiation-coherence-v1";
    let mut upstream_soundness_blockers = Vec::new();
    if motive_parametric_v1_success_row_provisional {
        upstream_soundness_blockers
            .push("R_T2_MOTIVE_PARAMETRIC_V1_SUCCESS_ROW_PROVISIONAL".to_owned());
    }
    if !registered
        .parametric_internality
        .total_specialization_authoritative
    {
        upstream_soundness_blockers.push("R_T2_DEPENDENT_TOTAL_SPECIALIZATION_UNPROVED".to_owned());
    } else if motive_parametric_theorem_version
        != "dependent-ambient-context-total-specialization-induction-v1"
        && !motive_parametric_v1_success_row_provisional
    {
        upstream_soundness_blockers
            .push("R_T2_UNKNOWN_MOTIVE_PARAMETRIC_ELIMINATOR_VERSION".to_owned());
    }
    if !exact_source_projection_replayed {
        upstream_soundness_blockers.push("R_T2_EXACT_SOURCE_DERIVATION_REPLAY_FAILED".to_owned());
    }
    upstream_soundness_blockers.sort();
    upstream_soundness_blockers.dedup();
    let hypothetical_derivation_replayable = replay_valid
        && registered.every_hole_live
        && declaration_projection_replayed
        && parameter_projection_replayed
        && upstream_soundness_blockers.is_empty();
    let zero_registration_marginal_charge = registered.hole_marginal_charge.replays_as_zero();
    let output_type_preserved = registered.explicit_body_elaboration.kernel_ty
        == registered.expected_kernel_type
        && hypothetical_derivation_replayable;
    let external_exhaustiveness_evidence_hash =
        registered.external_exhaustiveness_evidence_hash.clone();
    let exact_registration_prefix_bound = registered.ambient_registration_prefix_last_step
        == registered.visible_library_at_registration
        && registered.ambient_registration_prefix_signature_digest
            == expected_prefix_signature_digest
        && registered.ambient_declaration_bound_to_registration_prefix;
    let semantic_scope_premises_satisfied = registered.local_semantic_scope_premises_satisfied
        && external_exhaustiveness_evidence_hash.is_some()
        && exact_registration_prefix_bound;
    let law_level_semantic_scope_satisfied = semantic_scope_premises_satisfied
        && !motive_parametric_v1_success_row_provisional
        && upstream_soundness_blockers.is_empty();
    let opaque_prior_clause_reference_count = registered
        .declared_motives
        .iter()
        .filter(|motive| matches!(motive, DependentContextMotive::OpaquePriorClause { .. }))
        .count();
    // Exact sealed-reference digests remain in the formation token.  The
    // semantic key projects only their quotient-relevant content, so a
    // branch-local digest cannot manufacture inequivalence between otherwise
    // identical opaque prior-clause motives.
    let dependent_motive_semantic_shapes = registered
        .declared_motives
        .iter()
        .map(|motive| match motive {
            DependentContextMotive::Independent { motive } => serde_json::json!({
                "kind": "independent",
                "motive": motive,
            }),
            DependentContextMotive::ElementOfApplicationHead { head } => serde_json::json!({
                "kind": "element_of_application_head",
                "head": head,
            }),
            DependentContextMotive::OpaquePriorClause { reference } => serde_json::json!({
                "kind": "opaque_prior_clause",
                "prior_clause_index": reference.prior_clause_index,
                "prior_clause_kernel_type": reference.prior_clause_kernel_type,
            }),
        })
        .collect::<Vec<_>>();
    let branch_independent_semantic_key = tagged_hash(
        "registered-future-semantics-key",
        &(
            &registered.schema,
            &rule_constructor,
            &semantic_contract,
            semantic_family_slot,
            semantic_body_compared_by_family_quotient,
            &dependent_motive_semantic_shapes,
            &registered.expected_kernel_type,
            registered.no_reflexivity_fallback,
            registered.local_semantic_scope_premises_satisfied,
            zero_registration_marginal_charge,
        ),
    );
    let mut gate = Rt2V2FutureHoleGate {
        registered_open_judgment_schema: registered.schema.clone(),
        registration_replay_valid: replay_valid,
        a3_instance_id: registered.a3_instance_id.clone(),
        a3_scheme_id: registered.a3_scheme_id.clone(),
        rule_constructor,
        registration_kind,
        natural_family_id: registered.natural_family_id.clone(),
        occurrence_id: registered.occurrence_id.clone(),
        stage: registered.registration_stage,
        visible_library_at_registration: registered.visible_library_at_registration,
        ambient_registration_prefix_last_step: registered.ambient_registration_prefix_last_step,
        ambient_registration_prefix_signature_digest: registered
            .ambient_registration_prefix_signature_digest
            .clone(),
        ambient_declaration_bound_to_registration_prefix: registered
            .ambient_declaration_bound_to_registration_prefix,
        exact_registration_prefix_bound,
        constructor,
        constructor_side_condition_json,
        open_body,
        declared_motive_count: Some(registered.declared_motives.len()),
        dependent_context_version: Some(DEPENDENT_AMBIENT_CONTEXT_VERSION.to_owned()),
        dependent_context_declared_arity: Some(registered.declared_ambient_arity),
        dependent_totality_theorem_replayed: registered
            .parametric_internality
            .dependent_totality_theorem_replayed
            && registered
                .parametric_internality
                .dependent_totality_theorem
                .version
                == DEPENDENT_TOTALITY_VERSION,
        opaque_prior_clause_reference_count,
        opaque_prior_clause_references_replayed_whole: registered
            .ambient_declaration
            .opaque_references_used_whole,
        no_outcome_filtering_used: registered.ambient_declaration.no_outcome_filtering_used
            && registered
                .parametric_internality
                .dependent_totality_theorem
                .no_assignment_outcome_filtering,
        output_contract_json: Some(output_contract_json),
        output_type_preserved,
        declaration_projection_replayed,
        parameter_projection_replayed,
        hypothetical_derivation_replayable,
        motive_parametric_theorem_version,
        motive_parametric_v1_success_row_provisional,
        law_level_semantic_scope_satisfied,
        upstream_soundness_blockers,
        zero_registration_marginal_charge,
        no_reflexivity_fallback: registered.no_reflexivity_fallback && contract_no_reflexivity,
        semantic_scope_premises_satisfied,
        external_exhaustiveness_evidence_hash,
        historical_realization_required_for_r_t2_formation: false,
        named_gaps: Vec::new(),
        zero_named_gaps: true,
        branch_independent_semantic_key,
        semantic_family_slot,
        semantic_body_compared_by_family_quotient,
        branch_local_family_id_used_in_semantic_key: false,
        opaque_motive_reconstructions: Vec::new(),
        formation_derivation_hash: Some(registered.formation_hash.clone()),
        derivation_hash: String::new(),
    };
    gate.derivation_hash = tagged_hash("registered-future-hole-gate", &gate);
    Ok(gate)
}

fn attempted_contextual_motive(kernel_ty: &KernelTy) -> (ContextualMotive, bool) {
    match kernel_ty {
        KernelTy::Type => (ContextualMotive::Type, false),
        KernelTy::El(expression) => (ContextualMotive::Element(expression.clone()), false),
        KernelTy::Fun(domain, codomain) => {
            let (domain, domain_forbidden) = attempted_contextual_motive(domain);
            let (codomain, codomain_forbidden) = attempted_contextual_motive(codomain);
            (
                ContextualMotive::Function {
                    domain: Box::new(domain),
                    codomain: Box::new(codomain),
                },
                domain_forbidden || codomain_forbidden,
            )
        }
        KernelTy::Neutral | KernelTy::PathDecl { .. } => (ContextualMotive::Neutral, true),
    }
}

fn opaque_motive_reconstructions(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
    instance: &A3TypedDemandInstance,
) -> Result<Vec<Rt2V2OpaqueMotiveReconstruction>, Rt2FutureHoleConfluenceV2Error> {
    let mut result = Vec::new();
    for source_anchor_id in &instance.source_anchor_ids {
        let source = window
            .typed_sources
            .iter()
            .find(|source| &source.anchor_id == source_anchor_id)
            .ok_or_else(|| {
                Rt2FutureHoleConfluenceV2Error::Invariant(format!(
                    "future-hole gap source anchor {source_anchor_id} is absent from its A3 window"
                ))
            })?;
        if !source
            .canonical_presentation
            .parameters
            .iter()
            .any(|sort| *sort == ParamSort::Opaque)
        {
            continue;
        }
        let entry = signature.entry(source.step).ok_or_else(|| {
            Rt2FutureHoleConfluenceV2Error::Invariant(format!(
                "future-hole gap source step {} is absent from its exact signature",
                source.step
            ))
        })?;
        let elaboration =
            elaborate_telescope(signature, &entry.telescope, source.step.saturating_sub(1))
                .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
        let canonical_arity = u32::try_from(source.canonical_presentation.parameters.len())
            .map_err(|_| {
                Rt2FutureHoleConfluenceV2Error::Invariant(
                    "canonical parameter arity exceeds u32".to_owned(),
                )
            })?;
        let forward = &source.canonical_presentation.renaming.forward;
        let old_levels = forward
            .iter()
            .map(|(old_level, _)| *old_level)
            .collect::<BTreeSet<_>>();
        let canonical_indices = forward
            .iter()
            .map(|(_, canonical_index)| *canonical_index)
            .collect::<BTreeSet<_>>();
        let canonical_renaming_is_total_bijection = forward.len()
            == source.canonical_presentation.parameters.len()
            && old_levels.len() == forward.len()
            && canonical_indices == (1..=canonical_arity).collect::<BTreeSet<_>>()
            && old_levels.iter().all(|old_level| {
                *old_level <= source.canonical_presentation.renaming.free_scope_len
            });

        let mut reconstructed = Vec::with_capacity(source.canonical_presentation.parameters.len());
        for (zero_based_index, sort) in source.canonical_presentation.parameters.iter().enumerate()
        {
            let canonical_index = zero_based_index as u32 + 1;
            let preimages = forward
                .iter()
                .filter(|(_, index)| *index == canonical_index)
                .map(|(old_level, _)| *old_level)
                .collect::<Vec<_>>();
            let motive = match sort {
                ParamSort::Type => ContextualMotive::Type,
                ParamSort::Opaque if preimages.len() == 1 => {
                    let old_level = preimages[0];
                    let Some(prior_offset) =
                        old_level.checked_sub(elaboration.ambient_parameters + 1)
                    else {
                        reconstructed.push(ContextualMotive::Neutral);
                        continue;
                    };
                    elaboration
                        .clauses
                        .get(prior_offset as usize)
                        .map(|clause| attempted_contextual_motive(&clause.kernel_ty).0)
                        .unwrap_or(ContextualMotive::Neutral)
                }
                ParamSort::Opaque => ContextualMotive::Neutral,
            };
            reconstructed.push(motive);
        }
        let declaration_carrier = Telescope::new(vec![ClauseRec::new(
            ClauseRole::Formation,
            if canonical_arity == 0 {
                Expr::Univ
            } else {
                Expr::Var(canonical_arity)
            },
        )]);
        let declaration_attempt = issue_ambient_context_declaration_token(
            signature,
            &declaration_carrier,
            window.stage.saturating_sub(1),
            reconstructed.clone(),
        );
        let (motive_declaration_error_json, motive_not_formable_parameter) =
            match &declaration_attempt {
                Ok(_) => (None, None),
                Err(error) => (
                    Some(serde_json::to_string(error).map_err(|json_error| {
                        Rt2FutureHoleConfluenceV2Error::Json(json_error.to_string())
                    })?),
                    match error {
                        ContextualInternalityError::MotiveNotFormable { parameter } => {
                            Some(*parameter)
                        }
                        _ => None,
                    },
                ),
            };
        let full_declaration_ambient_limit_replayed = matches!(
            &declaration_attempt,
            Err(ContextualInternalityError::Elaboration(reason))
                if reason.contains("required ambient context 3 exceeds kernel maximum 2")
        );

        for (zero_based_index, sort) in source
            .canonical_presentation
            .parameters
            .iter()
            .enumerate()
            .filter(|(_, sort)| **sort == ParamSort::Opaque)
        {
            let canonical_parameter_index = zero_based_index as u32 + 1;
            let preimages = forward
                .iter()
                .filter(|(_, index)| *index == canonical_parameter_index)
                .map(|(old_level, _)| *old_level)
                .collect::<Vec<_>>();
            let canonical_index_has_unique_old_level_preimage = preimages.len() == 1;
            let old_level = if canonical_index_has_unique_old_level_preimage {
                preimages.first().copied()
            } else {
                None
            };
            let prior_clause_index = old_level
                .and_then(|level| level.checked_sub(elaboration.ambient_parameters + 1))
                .and_then(|index| u16::try_from(index).ok())
                .filter(|index| *index < source.clause_index);
            let prior_clause =
                prior_clause_index.and_then(|index| elaboration.clauses.get(usize::from(index)));
            let prior_clause_kernel_type_json = prior_clause
                .map(|clause| serde_json::to_string(&clause.kernel_ty))
                .transpose()
                .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?;
            let (attempted_contextual_motive, attempted_forbidden) = prior_clause
                .map(|clause| attempted_contextual_motive(&clause.kernel_ty))
                .map_or((None, false), |(motive, forbidden)| {
                    (Some(motive), forbidden)
                });
            let isolated_motive_declaration_carrier =
                Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Var(1))]);
            let isolated_motive_attempt = attempted_contextual_motive.as_ref().map(|motive| {
                issue_ambient_context_declaration_token(
                    signature,
                    &isolated_motive_declaration_carrier,
                    window.stage.saturating_sub(1),
                    vec![motive.clone()],
                )
            });
            let (isolated_motive_declaration_error_json, isolated_motive_not_formable_parameter) =
                match &isolated_motive_attempt {
                    None | Some(Ok(_)) => (None, None),
                    Some(Err(error)) => (
                        Some(serde_json::to_string(error).map_err(|json_error| {
                            Rt2FutureHoleConfluenceV2Error::Json(json_error.to_string())
                        })?),
                        match error {
                            ContextualInternalityError::MotiveNotFormable { parameter } => {
                                Some(*parameter)
                            }
                            _ => None,
                        },
                    ),
                };
            let exact_source_telescope_join = entry.candidate_hash == source.candidate_hash;
            let exact_source_elaboration_join = elaboration.derivation_hash
                == source.telescope_elaboration_hash
                && elaboration
                    .clauses
                    .get(usize::from(source.clause_index))
                    .is_some_and(|clause| {
                        clause.kernel_ty == source.kernel_type
                            && clause.normal_form == source.normal_form
                    });
            let exact_motive_not_formable_replayed = canonical_renaming_is_total_bijection
                && canonical_index_has_unique_old_level_preimage
                && prior_clause_index.is_some()
                && attempted_forbidden
                && exact_source_telescope_join
                && exact_source_elaboration_join
                && full_declaration_ambient_limit_replayed
                && motive_not_formable_parameter.is_none()
                && isolated_motive_not_formable_parameter == Some(1);
            let mut evidence = Rt2V2OpaqueMotiveReconstruction {
                source_anchor_id: source.anchor_id.clone(),
                source_step: source.step,
                source_clause_index: source.clause_index,
                source_candidate_hash: source.candidate_hash.clone(),
                source_telescope_elaboration_hash: source.telescope_elaboration_hash.clone(),
                canonical_parameter_index,
                canonical_parameter_sort_json: serde_json::to_string(sort)
                    .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?,
                canonical_renaming_forward: forward.clone(),
                canonical_renaming_is_total_bijection,
                canonical_index_has_unique_old_level_preimage,
                old_level,
                source_ambient_parameters: elaboration.ambient_parameters,
                maps_to_prior_clause: prior_clause_index.is_some(),
                prior_clause_index,
                prior_clause_kernel_type_json,
                attempted_contextual_motive,
                attempted_motive_contains_forbidden_neutral_or_path: attempted_forbidden,
                exact_source_telescope_join,
                exact_source_elaboration_join,
                declaration_carrier: declaration_carrier.clone(),
                declaration_motives: reconstructed.clone(),
                motive_declaration_error_json: motive_declaration_error_json.clone(),
                motive_not_formable_parameter,
                full_declaration_ambient_limit_replayed,
                isolated_motive_declaration_carrier,
                isolated_motive_declaration_error_json,
                isolated_motive_not_formable_parameter,
                motive_not_formable_canonical_parameter_index: Some(
                    canonical_parameter_index,
                ),
                isolated_projection_is_diagnostic_not_original_full_context_judgment: true,
                exact_motive_not_formable_replayed,
                failure_disposition: "candidate-relative dependent/prior-field motive is not expressible by the current non-dependent ambient declaration API (F-FH4): the original full context hits the ambient-arity limit, while the isolated MotiveNotFormable projection is only a diagnostic preserving the exact reconstructed motive, not the original full-context judgment; neither failure is a semantic nonexistence theorem".to_owned(),
                current_contextual_api_expressivity_gap_f_fh4: true,
                intrinsic_semantic_non_formability_claimed: false,
                future_scheme_nonexistence_claimed: false,
                derivation_hash: String::new(),
            };
            evidence.derivation_hash = tagged_hash("opaque-motive-reconstruction", &evidence);
            result.push(evidence);
        }
    }
    result.sort_by(|left, right| {
        (&left.source_anchor_id, left.canonical_parameter_index)
            .cmp(&(&right.source_anchor_id, right.canonical_parameter_index))
    });
    Ok(result)
}

fn gap_future_hole_gate(
    signature: &SealedSignature,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    window: &A3HistoricalWindow,
    gap: &pen_eval::future_hole_hypothesis_v2::NamedFutureHoleGapV2,
    replay_valid: bool,
) -> Result<Rt2V2FutureHoleGate, Rt2FutureHoleConfluenceV2Error> {
    let rule_constructor = serde_json::to_string(&scheme.rule_constructor)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?;
    let constructor = match &scheme.origin {
        A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } => Some(
            serde_json::to_string(constructor)
                .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?,
        ),
        A3DemandSchemeOrigin::BaseRule { .. } => None,
    };
    let named_gap = format!(
        "{}::{}::{}::{}",
        gap.id, gap.exact_error, gap.input_digest, gap.gap_hash
    );
    let opaque_motive_reconstructions = opaque_motive_reconstructions(signature, window, instance)?;
    if gap.id == "a3_v2_unary_motive_not_declarable"
        && (opaque_motive_reconstructions.is_empty()
            || !opaque_motive_reconstructions
                .iter()
                .all(|evidence| evidence.exact_motive_not_formable_replayed))
    {
        return Err(Rt2FutureHoleConfluenceV2Error::Invariant(format!(
            "opaque-motive gap {} lacks exact reconstruction evidence: {:#?}",
            scheme.scheme_id, opaque_motive_reconstructions
        )));
    }
    let mut gate = Rt2V2FutureHoleGate {
        registered_open_judgment_schema: FUTURE_HOLE_HYPOTHESIS_V2_SCHEMA.to_owned(),
        registration_replay_valid: replay_valid,
        a3_instance_id: instance.instance_id.clone(),
        a3_scheme_id: scheme.scheme_id.clone(),
        rule_constructor,
        registration_kind: "gap".to_owned(),
        natural_family_id: String::new(),
        occurrence_id: gap.occurrence_id.clone().unwrap_or_default(),
        stage: window.stage,
        visible_library_at_registration: window.stage.saturating_sub(1),
        ambient_registration_prefix_last_step: window.stage.saturating_sub(1),
        ambient_registration_prefix_signature_digest: String::new(),
        ambient_declaration_bound_to_registration_prefix: false,
        exact_registration_prefix_bound: false,
        constructor,
        constructor_side_condition_json: None,
        open_body: None,
        declared_motive_count: None,
        dependent_context_version: None,
        dependent_context_declared_arity: None,
        dependent_totality_theorem_replayed: false,
        opaque_prior_clause_reference_count: 0,
        opaque_prior_clause_references_replayed_whole: false,
        no_outcome_filtering_used: false,
        output_contract_json: None,
        output_type_preserved: false,
        declaration_projection_replayed: false,
        parameter_projection_replayed: false,
        hypothetical_derivation_replayable: false,
        motive_parametric_theorem_version: String::new(),
        motive_parametric_v1_success_row_provisional: false,
        law_level_semantic_scope_satisfied: false,
        upstream_soundness_blockers: Vec::new(),
        zero_registration_marginal_charge: false,
        no_reflexivity_fallback: false,
        semantic_scope_premises_satisfied: false,
        external_exhaustiveness_evidence_hash: None,
        historical_realization_required_for_r_t2_formation: false,
        named_gaps: vec![named_gap],
        zero_named_gaps: false,
        branch_independent_semantic_key: String::new(),
        semantic_family_slot: None,
        semantic_body_compared_by_family_quotient: false,
        branch_local_family_id_used_in_semantic_key: false,
        opaque_motive_reconstructions,
        formation_derivation_hash: None,
        derivation_hash: String::new(),
    };
    gate.derivation_hash = tagged_hash("registered-future-hole-gap-gate", &gate);
    Ok(gate)
}

fn future_hole_aggregate(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
    inventory_proof: &A3WindowRuleInventoryProof,
) -> Result<Rt2V2FutureHoleAggregateGate, Rt2FutureHoleConfluenceV2Error> {
    let mut expected = window
        .schemes
        .iter()
        .filter(|scheme| {
            matches!(
                scheme.rule_constructor,
                A3RuleConstructor::UnaryAction | A3RuleConstructor::StructuralCompletionHole
            )
        })
        .map(|scheme| {
            let instances = window
                .instances
                .iter()
                .filter(|instance| instance.scheme_id == scheme.scheme_id)
                .collect::<Vec<_>>();
            if instances.len() != 1 {
                return Err(Rt2FutureHoleConfluenceV2Error::Invariant(format!(
                    "future-semantics scheme {} has {} instances instead of one",
                    scheme.scheme_id,
                    instances.len()
                )));
            }
            Ok((scheme, instances[0]))
        })
        .collect::<Result<Vec<_>, _>>()?;
    expected.sort_by(|left, right| left.0.scheme_id.cmp(&right.0.scheme_id));
    let expected_future_semantics_scheme_ids = expected
        .iter()
        .map(|(scheme, _)| scheme.scheme_id.clone())
        .collect::<Vec<_>>();
    let expected_future_semantics_instance_ids = expected
        .iter()
        .map(|(_, instance)| instance.instance_id.clone())
        .collect::<Vec<_>>();
    let all_window_scheme_ids = window
        .schemes
        .iter()
        .map(|scheme| scheme.scheme_id.clone())
        .collect::<BTreeSet<_>>();
    let expected_scheme_set = expected_future_semantics_scheme_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let certified_scheme_set = inventory_proof
        .base_seed_dispositions
        .iter()
        .filter_map(|row| row.promoted_scheme_id.clone())
        .chain(
            inventory_proof
                .structural_seed_dispositions
                .iter()
                .filter_map(|row| row.structural_scheme_id.clone()),
        )
        .collect::<BTreeSet<_>>();
    let certified_instance_set = inventory_proof
        .base_seed_dispositions
        .iter()
        .filter_map(|row| row.promoted_instance_id.clone())
        .chain(
            inventory_proof
                .structural_seed_dispositions
                .iter()
                .filter_map(|row| row.structural_instance_id.clone()),
        )
        .collect::<BTreeSet<_>>();

    let mut registrations = Vec::new();
    for (scheme, instance) in &expected {
        let disposition = match scheme.rule_constructor {
            A3RuleConstructor::UnaryAction => {
                register_unary_action_v2(signature, window, scheme, instance)
            }
            A3RuleConstructor::StructuralCompletionHole => {
                register_structural_future_hole_v2(signature, window, scheme, instance)
            }
            _ => unreachable!("expected future-semantics filter is exhaustive"),
        }
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
        let gate = match disposition {
            FutureHoleRegistrationDispositionV2::Registered(registered) => {
                let joined = attach_external_exhaustiveness_evidence_v2(
                    &registered,
                    inventory_proof.derivation_hash.clone(),
                )
                .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
                let claimed = FutureHoleRegistrationDispositionV2::Registered(joined.clone());
                let replay = replay_future_hole_registration_v2(
                    signature, window, scheme, instance, &claimed,
                );
                registered_future_hole_gate(scheme, &joined, replay.valid, signature.digest())?
            }
            FutureHoleRegistrationDispositionV2::Gap(gap) => {
                let claimed = FutureHoleRegistrationDispositionV2::Gap(gap.clone());
                let replay = replay_future_hole_registration_v2(
                    signature, window, scheme, instance, &claimed,
                );
                gap_future_hole_gate(signature, scheme, instance, window, &gap, replay.valid)?
            }
        };
        registrations.push(gate);
    }

    registrations.sort_by(|left, right| left.a3_scheme_id.cmp(&right.a3_scheme_id));
    let registered_scheme_ids = registrations
        .iter()
        .filter(|row| row.zero_named_gaps && row.formation_derivation_hash.is_some())
        .map(|row| row.a3_scheme_id.clone())
        .collect::<Vec<_>>();
    let registered_instance_ids = registrations
        .iter()
        .filter(|row| row.zero_named_gaps && row.formation_derivation_hash.is_some())
        .map(|row| row.a3_instance_id.clone())
        .collect::<Vec<_>>();
    let gap_scheme_ids = registrations
        .iter()
        .filter(|row| !row.zero_named_gaps)
        .map(|row| row.a3_scheme_id.clone())
        .collect::<Vec<_>>();
    let registered_scheme_set = registered_scheme_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let registered_instance_set = registered_instance_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected_instance_set = expected_future_semantics_instance_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let unary_registration_count = registrations
        .iter()
        .filter(|row| row.registration_kind == "unary_action" && row.zero_named_gaps)
        .count();
    let structural_registration_count = registrations
        .iter()
        .filter(|row| row.registration_kind == "structural_completion" && row.zero_named_gaps)
        .count();
    let exact_scheme_and_instance_coverage = expected.len() == 5
        && unary_registration_count == 4
        && structural_registration_count == 1
        && expected_scheme_set == all_window_scheme_ids
        && expected_scheme_set == certified_scheme_set
        && expected_scheme_set == registered_scheme_set
        && expected_instance_set == certified_instance_set
        && expected_instance_set == registered_instance_set;
    let f_dc5_same_issuer_scope_preserved = expected.len() == 5
        && registrations.len() == 5
        && expected_scheme_set == all_window_scheme_ids
        && expected_scheme_set == certified_scheme_set;
    let every_registration_replay_valid = registrations
        .iter()
        .all(|row| row.registration_replay_valid);
    let every_registration_hypothetically_derivable = registrations
        .iter()
        .all(|row| row.hypothetical_derivation_replayable && row.output_type_preserved);
    let every_registration_zero_marginal_charge = registrations
        .iter()
        .all(|row| row.zero_registration_marginal_charge);
    let every_registration_no_reflexivity_fallback =
        registrations.iter().all(|row| row.no_reflexivity_fallback);
    let every_registration_semantic_scope_satisfied = registrations.iter().all(|row| {
        row.law_level_semantic_scope_satisfied
            && row.dependent_context_version.as_deref() == Some(DEPENDENT_AMBIENT_CONTEXT_VERSION)
            && row.dependent_totality_theorem_replayed
            && row.opaque_prior_clause_references_replayed_whole
            && row.no_outcome_filtering_used
    });
    let every_registration_inventory_bound = registrations.iter().all(|row| {
        row.external_exhaustiveness_evidence_hash.as_deref()
            == Some(inventory_proof.derivation_hash.as_str())
    });
    let every_registration_exact_prefix_bound = registrations
        .iter()
        .all(|row| row.exact_registration_prefix_bound);
    let successful_rows_provisional_due_motive_parametric_v1 = registrations
        .iter()
        .filter(|row| row.zero_named_gaps)
        .any(|row| row.motive_parametric_v1_success_row_provisional);
    let successful_rows_law_level_authoritative = registrations
        .iter()
        .filter(|row| row.zero_named_gaps)
        .all(|row| row.law_level_semantic_scope_satisfied)
        && !successful_rows_provisional_due_motive_parametric_v1;
    let mut upstream_soundness_blockers = registrations
        .iter()
        .flat_map(|row| row.upstream_soundness_blockers.iter().cloned())
        .collect::<Vec<_>>();
    upstream_soundness_blockers.sort();
    upstream_soundness_blockers.dedup();
    let historical_realization_required_for_r_t2_formation = registrations
        .iter()
        .any(|row| row.historical_realization_required_for_r_t2_formation);
    let mut named_gaps = registrations
        .iter()
        .flat_map(|row| row.named_gaps.iter().cloned())
        .collect::<Vec<_>>();
    named_gaps.extend(upstream_soundness_blockers.iter().cloned());
    if !exact_scheme_and_instance_coverage {
        named_gaps.push("R_T2_STAGE5_FUTURE_SEMANTICS_EXACT_COVERAGE_FAILED".to_owned());
    }
    if !every_registration_exact_prefix_bound {
        named_gaps.push("R_T2_STAGE5_FUTURE_SEMANTICS_PREFIX_LOCALITY_FAILED".to_owned());
    }
    named_gaps.sort();
    named_gaps.dedup();
    let zero_named_gaps = named_gaps.is_empty();
    let mut aggregate = Rt2V2FutureHoleAggregateGate {
        expected_future_semantics_scheme_ids,
        expected_future_semantics_instance_ids,
        registered_scheme_ids,
        registered_instance_ids,
        gap_scheme_ids,
        unary_registration_count,
        structural_registration_count,
        registration_count: registrations.len(),
        exact_scheme_and_instance_coverage,
        f_dc5_same_issuer_scope_preserved,
        registrations,
        every_registration_replay_valid,
        every_registration_hypothetically_derivable,
        every_registration_zero_marginal_charge,
        every_registration_no_reflexivity_fallback,
        every_registration_semantic_scope_satisfied,
        every_registration_inventory_bound,
        every_registration_exact_prefix_bound,
        successful_rows_provisional_due_motive_parametric_v1,
        successful_rows_law_level_authoritative,
        upstream_soundness_blockers,
        historical_realization_required_for_r_t2_formation,
        named_gaps,
        zero_named_gaps,
        aggregate_derivation_hash: String::new(),
    };
    aggregate.aggregate_derivation_hash = tagged_hash("future-hole-aggregate-gate", &aggregate);
    Ok(aggregate)
}

fn build_branches(
    v1: &Rt2FutureHoleConfluenceCertificate,
    inventory: &A3RuleInventoryExhaustivenessCertificate,
    inventory_replay_valid: bool,
) -> Result<Vec<Rt2V2BranchFormation>, Rt2FutureHoleConfluenceV2Error> {
    let mut branches = Vec::new();
    for v1_branch in &v1.branches {
        let signature = prefix_signature(&v1_branch.stage4_telescope);
        let window = generate_a3_window_for_prefix(&signature, 5)
            .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
        if signature.digest() != v1_branch.prefix_signature_digest
            || window.window_derivation_hash != v1_branch.stage5_window_derivation_hash
            || window.stage != 5
            || window.newest_step != Some(4)
            || window.older_step != Some(3)
        {
            return Err(Rt2FutureHoleConfluenceV2Error::Invariant(format!(
                "live exact-prefix regeneration drifted for branch {}",
                v1_branch.stage4_candidate_hash
            )));
        }
        let proof = prove_a3_window_inventory_for_exact_prefix(&signature, 5)
            .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
        if proof.exact_prefix_signature_digest != v1_branch.prefix_signature_digest
            || proof.stage != 5
            || proof.newest_step != Some(4)
            || proof.older_step != Some(3)
        {
            return Err(Rt2FutureHoleConfluenceV2Error::Invariant(format!(
                "exact-prefix inventory proof drifted for branch {}",
                v1_branch.stage4_candidate_hash
            )));
        }
        let compared_scheme_ids = v1_branch
            .schemes
            .iter()
            .map(|scheme| scheme.local_scheme_id.clone())
            .collect::<BTreeSet<_>>();
        let coverage = coverage_gate(
            inventory,
            inventory_replay_valid,
            &proof,
            &compared_scheme_ids,
        );
        let future = future_hole_aggregate(&signature, &window, &proof)?;
        branches.push(branch_from_v1(v1_branch, coverage, future)?);
    }
    branches.sort_by(|left, right| left.stage4_candidate_hash.cmp(&right.stage4_candidate_hash));
    Ok(branches)
}

fn compare_family(
    left: &Rt2V2FamilyPresentation,
    right: &Rt2V2FamilyPresentation,
) -> Result<Rt2V2FamilyQuotientComparison, Rt2FutureHoleConfluenceV2Error> {
    let declared_parameter_arities_equal =
        left.declared_parameter_arity == right.declared_parameter_arity;
    let equality_scope_len =
        declared_parameter_arities_equal.then_some(left.declared_parameter_arity);
    let parameter_sorts_equal =
        left.declared_parameter_sorts_json == right.declared_parameter_sorts_json;
    let kernel_types_equal = left.kernel_type_json == right.kernel_type_json;
    let frozen_equality = equality_scope_len
        .map(|scope_len| {
            univalent_equality(
                &left.canonical_normal_form,
                &right.canonical_normal_form,
                scope_len,
                128,
            )
            .map(FrozenEqualityRecord::from)
            .map_err(|error| Rt2FutureHoleConfluenceV2Error::Invariant(error.to_string()))
        })
        .transpose()?;
    let equality_scope_derived_only_from_declared_parameter_arity =
        equality_scope_len.is_none_or(|scope_len| {
            scope_len == left.declared_parameter_arity
                && scope_len == right.declared_parameter_arity
                && left.equality_scope_len == left.declared_parameter_arity
                && right.equality_scope_len == right.declared_parameter_arity
        });
    let equivalent = declared_parameter_arities_equal
        && equality_scope_derived_only_from_declared_parameter_arity
        && parameter_sorts_equal
        && kernel_types_equal
        && frozen_equality
            .as_ref()
            .is_some_and(|witness| witness.equal);
    let mut result = Rt2V2FamilyQuotientComparison {
        left_source_anchor_audit_only: left.source_anchor_audit_only.clone(),
        right_source_anchor_audit_only: right.source_anchor_audit_only.clone(),
        declared_parameter_arities_equal,
        equality_scope_len,
        equality_scope_derived_only_from_declared_parameter_arity,
        maximum_free_variable_reference_used_as_scope: false,
        parameter_sorts_equal,
        kernel_types_equal,
        frozen_equality,
        equivalent,
        derivation_hash: String::new(),
    };
    result.derivation_hash = tagged_hash("declared-arity-family-comparison", &result);
    Ok(result)
}

fn compare_scheme(
    left: &Rt2V2SchemeFormation,
    right: &Rt2V2SchemeFormation,
) -> Result<Rt2V2SchemeFormationComparison, Rt2FutureHoleConfluenceV2Error> {
    let rule_constructor_equal = left.rule_constructor == right.rule_constructor;
    let origin_kind_equal = left.origin_kind == right.origin_kind;
    let support_depth_equal = left.support_depth == right.support_depth;
    let declared_source_family_counts_equal =
        left.declared_source_family_count == right.declared_source_family_count;
    let source_family_comparisons = if declared_source_family_counts_equal {
        left.source_families
            .iter()
            .zip(&right.source_families)
            .map(|(left, right)| compare_family(left, right))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    let output_shape_equal = left.output_shape_json == right.output_shape_json;
    let registered_future_semantics_joined_both = left.registered_future_semantics_joined
        && right.registered_future_semantics_joined
        && !left.registered_future_semantics_formation_hash.is_empty()
        && !right.registered_future_semantics_formation_hash.is_empty();
    let registered_future_semantics_bound_to_family_quotient = !left
        .branch_local_family_id_used_in_semantic_key
        && !right.branch_local_family_id_used_in_semantic_key
        && left.registered_future_semantics_family_slot
            == right.registered_future_semantics_family_slot
        && left.registered_future_semantics_body_compared_by_family_quotient
            == right.registered_future_semantics_body_compared_by_family_quotient
        && match left.registered_future_semantics_family_slot {
            Some(slot) => {
                left.registered_future_semantics_body_compared_by_family_quotient
                    && source_family_comparisons
                        .get(slot)
                        .is_some_and(|comparison| comparison.equivalent)
            }
            None => !left.registered_future_semantics_body_compared_by_family_quotient,
        };
    let registered_future_semantic_keys_equal = left.registered_future_semantics_key
        == right.registered_future_semantics_key
        && !left.registered_future_semantics_key.is_empty();
    let equivalent_under_certified_family_quotient = rule_constructor_equal
        && origin_kind_equal
        && support_depth_equal
        && declared_source_family_counts_equal
        && source_family_comparisons.len() == left.declared_source_family_count
        && source_family_comparisons
            .iter()
            .all(|comparison| comparison.equivalent)
        && output_shape_equal
        && registered_future_semantics_joined_both
        && registered_future_semantics_bound_to_family_quotient
        && registered_future_semantic_keys_equal;
    let mut result = Rt2V2SchemeFormationComparison {
        left_scheme_id: left.local_scheme_id.clone(),
        right_scheme_id: right.local_scheme_id.clone(),
        rule_constructor_equal,
        origin_kind_equal,
        support_depth_equal,
        declared_source_family_counts_equal,
        source_family_comparisons,
        output_shape_equal,
        registered_future_semantics_joined_both,
        registered_future_semantics_bound_to_family_quotient,
        registered_future_semantic_keys_equal,
        equivalent_under_certified_family_quotient,
        derivation_hash: String::new(),
    };
    result.derivation_hash = tagged_hash("declared-arity-scheme-comparison", &result);
    Ok(result)
}

fn augment_matching(
    left: usize,
    edges: &[Vec<usize>],
    seen_right: &mut [bool],
    right_to_left: &mut [Option<usize>],
) -> bool {
    for &right in &edges[left] {
        if seen_right[right] {
            continue;
        }
        seen_right[right] = true;
        if right_to_left[right].is_none()
            || augment_matching(
                right_to_left[right].expect("checked present"),
                edges,
                seen_right,
                right_to_left,
            )
        {
            right_to_left[right] = Some(left);
            return true;
        }
    }
    false
}

fn compare_scheme_sets(
    left: &Rt2V2BranchFormation,
    right: &Rt2V2BranchFormation,
) -> Result<Rt2V2PairwiseSchemeSetComparison, Rt2FutureHoleConfluenceV2Error> {
    let mut comparisons = Vec::new();
    let mut edges = vec![Vec::new(); left.schemes.len()];
    for (left_index, left_scheme) in left.schemes.iter().enumerate() {
        for (right_index, right_scheme) in right.schemes.iter().enumerate() {
            let comparison = compare_scheme(left_scheme, right_scheme)?;
            if comparison.equivalent_under_certified_family_quotient {
                edges[left_index].push(right_index);
            }
            comparisons.push(comparison);
        }
    }
    let mut right_to_left = vec![None; right.schemes.len()];
    let mut matched = 0usize;
    for left_index in 0..left.schemes.len() {
        let mut seen_right = vec![false; right.schemes.len()];
        if augment_matching(left_index, &edges, &mut seen_right, &mut right_to_left) {
            matched += 1;
        }
    }
    let mut perfect_matching = right_to_left
        .iter()
        .enumerate()
        .filter_map(|(right_index, left_index)| {
            left_index.map(|left_index| {
                (
                    left.schemes[left_index].local_scheme_id.clone(),
                    right.schemes[right_index].local_scheme_id.clone(),
                )
            })
        })
        .collect::<Vec<_>>();
    perfect_matching.sort();
    let full_scheme_sets_equivalent = left.schemes.len() == right.schemes.len()
        && matched == left.schemes.len()
        && matched == right.schemes.len();
    let left_classes = left
        .scheme_formation_class_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let right_classes = right
        .scheme_formation_class_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let future_aggregate_ready = |aggregate: &Rt2V2FutureHoleAggregateGate| {
        aggregate.registration_count == 5
            && aggregate.unary_registration_count == 4
            && aggregate.structural_registration_count == 1
            && aggregate.exact_scheme_and_instance_coverage
            && aggregate.f_dc5_same_issuer_scope_preserved
            && aggregate.every_registration_replay_valid
            && aggregate.every_registration_hypothetically_derivable
            && aggregate.every_registration_zero_marginal_charge
            && aggregate.every_registration_no_reflexivity_fallback
            && aggregate.every_registration_semantic_scope_satisfied
            && aggregate.every_registration_inventory_bound
            && aggregate.every_registration_exact_prefix_bound
            && !aggregate.successful_rows_provisional_due_motive_parametric_v1
            && aggregate.successful_rows_law_level_authoritative
            && aggregate.upstream_soundness_blockers.is_empty()
            && !aggregate.historical_realization_required_for_r_t2_formation
            && aggregate.zero_named_gaps
    };
    let relative_coverage_and_zero_gap_gate_passed = left
        .coverage_gate
        .full_relative_a3_seed_constructor_coverage
        && right
            .coverage_gate
            .full_relative_a3_seed_constructor_coverage
        && left.coverage_gate.zero_named_gaps
        && right.coverage_gate.zero_named_gaps
        && future_aggregate_ready(&left.future_hole_aggregate)
        && future_aggregate_ready(&right.future_hole_aggregate);
    let comparison_well_formed = relative_coverage_and_zero_gap_gate_passed
        && left.every_instance_typed
        && right.every_instance_typed
        && left.every_scheme_has_formation_evidence
        && right.every_scheme_has_formation_evidence
        && comparisons.iter().all(|comparison| {
            !comparison.derivation_hash.is_empty()
                && comparison.registered_future_semantics_joined_both
                && comparison.source_family_comparisons.iter().all(|family| {
                    family.equality_scope_derived_only_from_declared_parameter_arity
                        && !family.maximum_free_variable_reference_used_as_scope
                })
        });
    let mut result = Rt2V2PairwiseSchemeSetComparison {
        left_candidate_hash: left.stage4_candidate_hash.clone(),
        right_candidate_hash: right.stage4_candidate_hash.clone(),
        left_complete_scheme_count: left.schemes.len(),
        right_complete_scheme_count: right.schemes.len(),
        comparisons,
        perfect_matching,
        unmatched_left_formation_classes: left_classes
            .difference(&right_classes)
            .cloned()
            .collect(),
        unmatched_right_formation_classes: right_classes
            .difference(&left_classes)
            .cloned()
            .collect(),
        full_scheme_sets_equivalent,
        comparison_well_formed,
        relative_coverage_and_zero_gap_gate_passed,
        hash_or_enumeration_order_used_as_selector: false,
        derivation_hash: String::new(),
    };
    result.derivation_hash = tagged_hash("pairwise-complete-stage5-scheme-sets", &result);
    Ok(result)
}

fn all_pairwise_comparisons(
    branches: &[Rt2V2BranchFormation],
) -> Result<Vec<Rt2V2PairwiseSchemeSetComparison>, Rt2FutureHoleConfluenceV2Error> {
    let mut comparisons = Vec::new();
    for left in 0..branches.len() {
        for right in (left + 1)..branches.len() {
            comparisons.push(compare_scheme_sets(&branches[left], &branches[right])?);
        }
    }
    Ok(comparisons)
}

fn normalized_pair_verdicts(
    comparisons: &[Rt2V2PairwiseSchemeSetComparison],
) -> Vec<Rt2V2OrderIndependentPairVerdict> {
    let mut rows = comparisons
        .iter()
        .map(|comparison| {
            let (unordered_candidate_pair, left_scheme_count, right_scheme_count) =
                if comparison.left_candidate_hash <= comparison.right_candidate_hash {
                    (
                        (
                            comparison.left_candidate_hash.clone(),
                            comparison.right_candidate_hash.clone(),
                        ),
                        comparison.left_complete_scheme_count,
                        comparison.right_complete_scheme_count,
                    )
                } else {
                    (
                        (
                            comparison.right_candidate_hash.clone(),
                            comparison.left_candidate_hash.clone(),
                        ),
                        comparison.right_complete_scheme_count,
                        comparison.left_complete_scheme_count,
                    )
                };
            Rt2V2OrderIndependentPairVerdict {
                unordered_candidate_pair,
                left_scheme_count,
                right_scheme_count,
                matched_count: comparison.perfect_matching.len(),
                equivalent: comparison.full_scheme_sets_equivalent,
            }
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn order_reversal_audit(
    branches: &[Rt2V2BranchFormation],
    forward: &[Rt2V2PairwiseSchemeSetComparison],
) -> Result<Rt2V2OrderReversalAudit, Rt2FutureHoleConfluenceV2Error> {
    let mut reversed_branches = branches.to_vec();
    reversed_branches.reverse();
    let reversed = all_pairwise_comparisons(&reversed_branches)?;
    let forward_normalized_pair_verdicts = normalized_pair_verdicts(forward);
    let reversed_normalized_pair_verdicts = normalized_pair_verdicts(&reversed);
    let mut forward_complete_set_hashes = branches
        .iter()
        .map(|branch| branch.complete_formation_set_hash.clone())
        .collect::<Vec<_>>();
    let mut reversed_complete_set_hashes = reversed_branches
        .iter()
        .map(|branch| branch.complete_formation_set_hash.clone())
        .collect::<Vec<_>>();
    forward_complete_set_hashes.sort();
    reversed_complete_set_hashes.sort();
    let exact_pair_verdict_invariance =
        forward_normalized_pair_verdicts == reversed_normalized_pair_verdicts;
    let exact_complete_set_invariance = forward_complete_set_hashes == reversed_complete_set_hashes;
    let mut audit = Rt2V2OrderReversalAudit {
        forward_branch_order: branches
            .iter()
            .map(|branch| branch.stage4_candidate_hash.clone())
            .collect(),
        reversed_branch_order: reversed_branches
            .iter()
            .map(|branch| branch.stage4_candidate_hash.clone())
            .collect(),
        forward_normalized_pair_verdicts,
        reversed_normalized_pair_verdicts,
        forward_complete_set_hashes,
        reversed_complete_set_hashes,
        exact_pair_verdict_invariance,
        exact_complete_set_invariance,
        selection_invariant: true,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("order-reversal-audit", &audit);
    Ok(audit)
}

fn blocked_order_reversal_audit(branches: &[Rt2V2BranchFormation]) -> Rt2V2OrderReversalAudit {
    let forward_branch_order = branches
        .iter()
        .map(|branch| branch.stage4_candidate_hash.clone())
        .collect::<Vec<_>>();
    let reversed_branch_order = forward_branch_order
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<_>>();
    let mut forward_complete_set_hashes = branches
        .iter()
        .map(|branch| branch.complete_formation_set_hash.clone())
        .collect::<Vec<_>>();
    forward_complete_set_hashes.sort();
    let reversed_complete_set_hashes = forward_complete_set_hashes.clone();
    let mut audit = Rt2V2OrderReversalAudit {
        forward_branch_order,
        reversed_branch_order,
        forward_normalized_pair_verdicts: Vec::new(),
        reversed_normalized_pair_verdicts: Vec::new(),
        forward_complete_set_hashes,
        reversed_complete_set_hashes,
        exact_pair_verdict_invariance: false,
        exact_complete_set_invariance: true,
        selection_invariant: true,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("blocked-order-reversal-audit", &audit);
    audit
}

fn finalize_certificate(
    v1: &Rt2FutureHoleConfluenceCertificate,
    archived_v1_artifact_digest_verified: bool,
    inventory: &A3RuleInventoryExhaustivenessCertificate,
    inventory_replay_valid: bool,
    archived_inventory_replay_valid: bool,
    branches: Vec<Rt2V2BranchFormation>,
) -> Result<Rt2FutureHoleConfluenceV2Certificate, Rt2FutureHoleConfluenceV2Error> {
    let branch_count = branches.len();
    let class_ids = branches
        .iter()
        .map(|branch| branch.r_t1_class_id.clone())
        .collect::<BTreeSet<_>>();
    let branch_candidate_hashes = branches
        .iter()
        .map(|branch| branch.stage4_candidate_hash.clone())
        .collect::<BTreeSet<_>>();
    let v1_candidate_hashes = v1
        .branches
        .iter()
        .map(|branch| branch.stage4_candidate_hash.clone())
        .collect::<BTreeSet<_>>();
    let exact_four_way_r_t1_class_join = branch_count == 4
        && class_ids.len() == 4
        && branch_candidate_hashes == v1_candidate_hashes
        && v1.exact_four_way_stage4_minimizer_join
        && v1.r_t1_semantic_class_count == 4;
    if !exact_four_way_r_t1_class_join {
        return Err(Rt2FutureHoleConfluenceV2Error::Invariant(
            "v2 branches do not join exactly to the four certified R-T1 classes".to_owned(),
        ));
    }
    let distinct_prefixes = branches
        .iter()
        .map(|branch| branch.prefix_signature_digest.clone())
        .collect::<BTreeSet<_>>();
    let distinct_windows = branches
        .iter()
        .map(|branch| branch.stage5_window_derivation_hash.clone())
        .collect::<BTreeSet<_>>();
    let four_exact_stage5_prefixes_independently_generated = branch_count == 4
        && distinct_prefixes.len() == 4
        && distinct_windows.len() == 4
        && branches.iter().all(|branch| {
            branch.prefix_steps == vec![1, 2, 3, 4]
                && branch.prefix_steps_exactly_one_through_four
                && branch.independently_generated_stage5_window
                && branch.stage5_newest_step == 4
                && branch.stage5_older_step == 3
                && branch.complete_certified_scheme_count == branch.schemes.len()
                && branch
                    .coverage_gate
                    .exact_compared_scheme_set_equals_inventory_promotions
        });
    if !four_exact_stage5_prefixes_independently_generated {
        return Err(Rt2FutureHoleConfluenceV2Error::Invariant(
            "four independent exact Stage-5 prefix generations were not certified".to_owned(),
        ));
    }

    let full_relative_a3_seed_constructor_coverage_every_branch = branches.iter().all(|branch| {
        branch
            .coverage_gate
            .full_relative_a3_seed_constructor_coverage
    });
    let branch_future_aggregate_ready = |branch: &Rt2V2BranchFormation| {
        let aggregate = &branch.future_hole_aggregate;
        aggregate.registration_count == 5
            && aggregate.unary_registration_count == 4
            && aggregate.structural_registration_count == 1
            && aggregate.exact_scheme_and_instance_coverage
            && aggregate.every_registration_replay_valid
            && aggregate.every_registration_hypothetically_derivable
            && aggregate.every_registration_zero_marginal_charge
            && aggregate.every_registration_no_reflexivity_fallback
            && aggregate.every_registration_semantic_scope_satisfied
            && aggregate.every_registration_inventory_bound
            && aggregate.every_registration_exact_prefix_bound
            && !aggregate.successful_rows_provisional_due_motive_parametric_v1
            && aggregate.successful_rows_law_level_authoritative
            && aggregate.upstream_soundness_blockers.is_empty()
            && !aggregate.historical_realization_required_for_r_t2_formation
            && aggregate.zero_named_gaps
            && branch.schemes.iter().all(|scheme| {
                scheme.registered_future_semantics_joined
                    && !scheme.registered_future_semantics_key.is_empty()
                    && !scheme.registered_future_semantics_formation_hash.is_empty()
            })
    };
    let zero_named_gaps_every_branch = branches.iter().all(|branch| {
        branch.coverage_gate.zero_named_gaps && branch_future_aggregate_ready(branch)
    });
    let full_adopted_frozen_semantic_scope_exhaustive_every_branch =
        branches.iter().all(|branch| {
            branch
                .coverage_gate
                .full_adopted_frozen_semantic_scope_exhaustive
                && branch
                    .coverage_gate
                    .unresolved_in_domain_semantic_scope_premises
                    .is_empty()
                && branch
                    .future_hole_aggregate
                    .upstream_soundness_blockers
                    .is_empty()
                && branch
                    .future_hole_aggregate
                    .successful_rows_law_level_authoritative
        });
    let mut unresolved_in_domain_semantic_scope_premises = branches
        .iter()
        .flat_map(|branch| {
            branch
                .coverage_gate
                .unresolved_in_domain_semantic_scope_premises
                .iter()
                .cloned()
                .chain(branch.future_hole_aggregate.named_gaps.iter().cloned())
        })
        .collect::<Vec<_>>();
    unresolved_in_domain_semantic_scope_premises.sort();
    unresolved_in_domain_semantic_scope_premises.dedup();
    let mut disclosed_nonblocking_scope_and_archive_limits = branches
        .iter()
        .flat_map(|branch| {
            branch
                .coverage_gate
                .disclosed_nonblocking_scope_and_archive_limits
                .iter()
                .cloned()
        })
        .collect::<Vec<_>>();
    disclosed_nonblocking_scope_and_archive_limits.sort();
    disclosed_nonblocking_scope_and_archive_limits.dedup();
    let all_family_equality_scopes_from_declared_parameter_arity = branches
        .iter()
        .flat_map(|branch| &branch.schemes)
        .flat_map(|scheme| &scheme.source_families)
        .all(|family| {
            family.equality_scope_len == family.declared_parameter_arity
                && family.equality_scope_source
                    == "A3TypedClauseSource.canonical_presentation.parameters.len"
                && !family.maximum_free_variable_reference_used_as_scope
        });
    let maximum_free_variable_reference_used_as_scope = false;

    let law_level_promotion_gate_passed = inventory_replay_valid
        && archived_inventory_replay_valid
        && full_relative_a3_seed_constructor_coverage_every_branch
        && zero_named_gaps_every_branch
        && full_adopted_frozen_semantic_scope_exhaustive_every_branch
        && unresolved_in_domain_semantic_scope_premises.is_empty()
        && all_family_equality_scopes_from_declared_parameter_arity
        && !maximum_free_variable_reference_used_as_scope;
    let pairwise_scheme_set_comparisons = if law_level_promotion_gate_passed {
        all_pairwise_comparisons(&branches)?
    } else {
        Vec::new()
    };
    let every_pairwise_comparison_well_formed = law_level_promotion_gate_passed
        && pairwise_scheme_set_comparisons.len() == 6
        && pairwise_scheme_set_comparisons
            .iter()
            .all(|comparison| comparison.comparison_well_formed);
    let all_stage5_scheme_sets_equivalent = every_pairwise_comparison_well_formed
        && pairwise_scheme_set_comparisons
            .iter()
            .all(|comparison| comparison.full_scheme_sets_equivalent);
    let order_reversal_audit = if law_level_promotion_gate_passed {
        order_reversal_audit(&branches, &pairwise_scheme_set_comparisons)?
    } else {
        blocked_order_reversal_audit(&branches)
    };
    let order_reversal_invariant = law_level_promotion_gate_passed
        && order_reversal_audit.exact_pair_verdict_invariance
        && order_reversal_audit.exact_complete_set_invariance
        && order_reversal_audit.selection_invariant;
    let immediate_inequivalent_successor_on_certified_relative_surface =
        every_pairwise_comparison_well_formed && !all_stage5_scheme_sets_equivalent;
    let deterministic_future_isomorphism_replayed = false;
    let r_t2_confluence_proved = all_stage5_scheme_sets_equivalent
        && deterministic_future_isomorphism_replayed
        && order_reversal_invariant;
    let r_t2_confluence_refuted = law_level_promotion_gate_passed
        && order_reversal_invariant
        && immediate_inequivalent_successor_on_certified_relative_surface;
    let r_t3_user_adjudication_opened = r_t2_confluence_refuted;

    let mut serialized_blockers = unresolved_in_domain_semantic_scope_premises.clone();
    if !full_relative_a3_seed_constructor_coverage_every_branch {
        serialized_blockers.push("R_T2_RELATIVE_A3_SEED_CONSTRUCTOR_COVERAGE_FAILED".to_owned());
    }
    if !zero_named_gaps_every_branch {
        serialized_blockers.push("R_T2_REGISTERED_FUTURE_HOLE_GAP_SURVIVES".to_owned());
    }
    if !full_adopted_frozen_semantic_scope_exhaustive_every_branch {
        serialized_blockers.push("R_T2_ADOPTED_FROZEN_SEMANTIC_SCOPE_PREMISE_SURVIVES".to_owned());
    }
    if !all_family_equality_scopes_from_declared_parameter_arity {
        serialized_blockers.push("R_T2_DECLARED_PARAMETER_ARITY_SCOPE_GATE_FAILED".to_owned());
    }
    if law_level_promotion_gate_passed && !order_reversal_invariant {
        serialized_blockers.push("R_T2_ORDER_REVERSAL_INVARIANCE_FAILED".to_owned());
    }
    serialized_blockers.sort();
    serialized_blockers.dedup();

    let outcome = if !full_relative_a3_seed_constructor_coverage_every_branch
        || !zero_named_gaps_every_branch
    {
        Rt2FutureHoleConfluenceV2Outcome::RelativeCoverageOrNamedGapBlocker
    } else if !full_adopted_frozen_semantic_scope_exhaustive_every_branch
        || !unresolved_in_domain_semantic_scope_premises.is_empty()
    {
        Rt2FutureHoleConfluenceV2Outcome::ScopedInequivalenceAdoptedFrozenScopePremisePending
    } else if r_t2_confluence_refuted {
        Rt2FutureHoleConfluenceV2Outcome::LawLevelConfluenceRefutedRungRt3Opened
    } else {
        Rt2FutureHoleConfluenceV2Outcome::ImmediateSchemeSetsEquivalentFutureIsomorphismPending
    };
    let (permitted_conclusion, required_successor_action) = match outcome {
        Rt2FutureHoleConfluenceV2Outcome::RelativeCoverageOrNamedGapBlocker => (
            "No law-level R-T2 verdict is issued: a relative A3 coverage or registered future-hole semantic gap remains. The four branch surfaces are retained only as typed evidence; R-T3 remains closed.".to_owned(),
            "Discharge the serialized in-domain coverage/gap blockers and rerun this v2 issuer create-new. Do not select a branch, run the bridge, or issue a halt claim.".to_owned(),
        ),
        Rt2FutureHoleConfluenceV2Outcome::ScopedInequivalenceAdoptedFrozenScopePremisePending => (
            "No law-level R-T2 verdict is issued: the immediate comparison may be scoped evidence, but a premise internal to the adopted frozen semantic domain survives. R-T3 remains closed.".to_owned(),
            "Prove the serialized adopted-domain semantic-scope premise and rerun v2. The separately disclosed absolute-language and archive-live-replay limits do not themselves select or block within the frozen domain.".to_owned(),
        ),
        Rt2FutureHoleConfluenceV2Outcome::LawLevelConfluenceRefutedRungRt3Opened => (
            "R-T2 confluence is refuted on the complete adopted frozen A3 domain: all four exact Stage-5 successor surfaces pass independent seed/constructor reverse joins and registered-hole replay, yet at least one complete scheme-set pair has no perfect matching under the declared-arity frozen family quotient. Order reversal preserves every verdict. R-T3 opens without selecting a branch.".to_owned(),
            "Proceed only to the adopted R-T3 user adjudication. Enumeration/hash order, downstream history, counts, scores, and the bar remain inadmissible selectors; the bridge and halt certificate remain closed.".to_owned(),
        ),
        Rt2FutureHoleConfluenceV2Outcome::ImmediateSchemeSetsEquivalentFutureIsomorphismPending => (
            "The complete immediate Stage-5 scheme sets are equivalent under the declared-arity frozen family quotient, but deterministic future-isomorphism and continuation preservation have not been replayed. R-T2 is not yet proved and R-T3 remains closed.".to_owned(),
            "Construct the deterministic future-isomorphism/continuation replay required by R-T2. Do not select a branch or run the bridge from immediate set equivalence alone.".to_owned(),
        ),
    };
    let registered_future_hole_aggregate_digest = tagged_hash(
        "registered-future-hole-aggregate",
        &branches
            .iter()
            .map(|branch| &branch.future_hole_aggregate.aggregate_derivation_hash)
            .collect::<Vec<_>>(),
    );
    let all_registered_future_holes_replay_valid = branches
        .iter()
        .all(|branch| branch.future_hole_aggregate.every_registration_replay_valid);

    let mut certificate = Rt2FutureHoleConfluenceV2Certificate {
        schema: R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA.to_owned(),
        date: R_T2_FUTURE_HOLE_CONFLUENCE_V2_DATE.to_owned(),
        source_bindings: source_bindings(),
        tie_protocol_adoption_replayed: true,
        future_hole_adoption_replayed: true,
        v1_post_audit_blocker_replayed: true,
        equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
        archived_v1_artifact_digest_verified,
        v1_certificate_digest: v1.result_digest.clone(),
        v1_replay_valid: true,
        a3_inventory_exhaustiveness_certificate_digest: inventory.result_digest.clone(),
        a3_inventory_exhaustiveness_replay_valid: inventory_replay_valid,
        a3_inventory_exhaustiveness_archived_artifact_replay_valid: archived_inventory_replay_valid,
        registered_future_hole_aggregate_digest,
        all_registered_future_holes_replay_valid,
        naturality_orbit_transport_certificate_digest: v1
            .naturality_orbit_transport_certificate_digest
            .clone(),
        naturality_orbit_transport_replay_valid: v1.naturality_orbit_transport_replay_valid,
        exact_four_way_r_t1_class_join,
        branch_count,
        branches,
        four_exact_stage5_prefixes_independently_generated,
        full_relative_a3_seed_constructor_coverage_every_branch,
        zero_named_gaps_every_branch,
        full_adopted_frozen_semantic_scope_exhaustive_every_branch,
        unresolved_in_domain_semantic_scope_premises,
        disclosed_nonblocking_scope_and_archive_limits,
        all_family_equality_scopes_from_declared_parameter_arity,
        maximum_free_variable_reference_used_as_scope,
        pairwise_scheme_set_comparisons,
        every_pairwise_comparison_well_formed,
        all_stage5_scheme_sets_equivalent,
        order_reversal_audit,
        order_reversal_invariant,
        immediate_inequivalent_successor_on_certified_relative_surface,
        law_level_promotion_gate_passed,
        deterministic_future_isomorphism_replayed,
        r_t2_confluence_proved,
        r_t2_confluence_refuted,
        r_t3_user_adjudication_opened,
        selected_candidate_hash: None,
        hash_or_enumeration_order_used_as_selector: false,
        desired_history_count_score_or_bar_used_as_premise: false,
        theorem_t_bf1_proved: false,
        theorem_t_bf3_proved: false,
        bar_free_adoption_authorized: false,
        bridge_authorized: false,
        halt_claim_issued: false,
        outcome,
        serialized_blockers,
        permitted_conclusion,
        required_successor_action,
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn certificate_digest(certificate: &Rt2FutureHoleConfluenceV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

pub fn issue_r_t2_future_hole_confluence_v2_certificate()
-> Result<Rt2FutureHoleConfluenceV2Certificate, Rt2FutureHoleConfluenceV2Error> {
    replay_adoptions()?;

    let archived_v1 = archived_v1_certificate()?;
    let archived_v1_artifact_digest_verified = archived_v1.schema
        == R_T2_FUTURE_HOLE_CONFLUENCE_SCHEMA
        && archived_v1.result_digest == v1_certificate_digest(&archived_v1)
        && archived_v1.branch_count == 4
        && archived_v1.pairwise_scheme_set_comparisons.len() == 6;
    if !archived_v1_artifact_digest_verified {
        return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(
            "immutable R-T2 v1 artifact failed its frozen digest/shape check".to_owned(),
        ));
    }

    let v1 = issue_r_t2_future_hole_confluence_certificate()
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
    let v1_replay = replay_r_t2_future_hole_confluence_certificate(&v1);
    if !v1_replay.valid {
        return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(format!(
            "live v1 scoped comparison replay failed: {}",
            v1_replay.errors.join("; ")
        )));
    }

    let inventory = issue_historical_a3_rule_inventory_exhaustiveness()
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Prerequisite(error.to_string()))?;
    let inventory_replay = replay_historical_a3_rule_inventory_exhaustiveness(&inventory);
    if !inventory_replay.valid {
        return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(format!(
            "live A3 inventory theorem replay failed: {}",
            inventory_replay.errors.join("; ")
        )));
    }
    let archived_inventory = archived_a3_exhaustiveness_certificate()?;
    let archived_inventory_replay =
        replay_historical_a3_rule_inventory_exhaustiveness(&archived_inventory);
    if !archived_inventory_replay.valid {
        return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(format!(
            "create-new A3 inventory artifact replay failed: {}",
            archived_inventory_replay.errors.join("; ")
        )));
    }
    if archived_inventory.result_digest != inventory.result_digest {
        return Err(Rt2FutureHoleConfluenceV2Error::Prerequisite(
            "live and create-new A3 inventory theorem digests differ".to_owned(),
        ));
    }

    let branches = build_branches(&v1, &inventory, inventory_replay.valid)?;
    finalize_certificate(
        &v1,
        archived_v1_artifact_digest_verified,
        &inventory,
        inventory_replay.valid,
        archived_inventory_replay.valid,
        branches,
    )
}

fn failed_replay(error: impl Into<String>) -> Rt2FutureHoleConfluenceV2Replay {
    Rt2FutureHoleConfluenceV2Replay {
        valid: false,
        outcome: None,
        branch_count: 0,
        relative_coverage_gate_passed: false,
        zero_named_gaps: false,
        full_adopted_frozen_semantic_scope_gate_passed: false,
        order_reversal_invariant: false,
        all_stage5_scheme_sets_equivalent: false,
        r_t2_confluence_proved: false,
        r_t2_confluence_refuted: false,
        r_t3_user_adjudication_opened: false,
        serialized_blockers: vec![error.into()],
        errors: vec!["live R-T2 v2 issuance failed".to_owned()],
    }
}

pub fn replay_r_t2_future_hole_confluence_v2_certificate(
    certificate: &Rt2FutureHoleConfluenceV2Certificate,
) -> Rt2FutureHoleConfluenceV2Replay {
    let expected = match issue_r_t2_future_hole_confluence_v2_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

/// Verify the immutable Stage-4 fork projection without reissuing the later
/// Stage-5/A3 formation surface.  Branch-invariance continuations consume
/// exactly this projection: the four unselected roots and their exact prefix
/// commitments.  Later source drift therefore cannot smuggle an enacted
/// suffix dependency into a non-enacted branch.
pub fn replay_archived_stage4_fork_projection(
    certificate: &Rt2FutureHoleConfluenceV2Certificate,
) -> Vec<String> {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("archived R-T2 v2 certificate digest mismatch".to_owned());
    }
    if certificate.schema != R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA
        || certificate.outcome
            != Rt2FutureHoleConfluenceV2Outcome::LawLevelConfluenceRefutedRungRt3Opened
        || certificate.branch_count != 4
        || certificate.branches.len() != 4
        || !certificate.exact_four_way_r_t1_class_join
        || !certificate.r_t2_confluence_refuted
        || certificate.r_t2_confluence_proved
        || certificate.selected_candidate_hash.is_some()
        || certificate.hash_or_enumeration_order_used_as_selector
        || certificate.desired_history_count_score_or_bar_used_as_premise
    {
        errors.push("archived R-T2 v2 Stage-4 fork projection drifted".to_owned());
    }
    let branch_hashes = certificate
        .branches
        .iter()
        .map(|branch| branch.stage4_candidate_hash.as_str())
        .collect::<BTreeSet<_>>();
    let class_ids = certificate
        .branches
        .iter()
        .map(|branch| branch.r_t1_class_id.as_str())
        .collect::<BTreeSet<_>>();
    if branch_hashes.len() != 4
        || class_ids.len() != 4
        || certificate.branches.iter().any(|branch| {
            candidate_hash(&branch.stage4_telescope) != branch.stage4_candidate_hash
                || branch.prefix_steps != vec![1, 2, 3, 4]
                || !branch.prefix_steps_exactly_one_through_four
                || branch.prefix_signature_digest.is_empty()
        })
    {
        errors.push("archived R-T2 v2 branch-root projection is not exact".to_owned());
    }
    errors
}

fn replay_against_expected(
    certificate: &Rt2FutureHoleConfluenceV2Certificate,
    expected: &Rt2FutureHoleConfluenceV2Certificate,
) -> Rt2FutureHoleConfluenceV2Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("certificate digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from independent live R-T2 v2 rerun".to_owned());
    }
    Rt2FutureHoleConfluenceV2Replay {
        valid: errors.is_empty(),
        outcome: Some(certificate.outcome.clone()),
        branch_count: certificate.branch_count,
        relative_coverage_gate_passed: certificate
            .full_relative_a3_seed_constructor_coverage_every_branch,
        zero_named_gaps: certificate.zero_named_gaps_every_branch,
        full_adopted_frozen_semantic_scope_gate_passed: certificate
            .full_adopted_frozen_semantic_scope_exhaustive_every_branch,
        order_reversal_invariant: certificate.order_reversal_invariant,
        all_stage5_scheme_sets_equivalent: certificate.all_stage5_scheme_sets_equivalent,
        r_t2_confluence_proved: certificate.r_t2_confluence_proved,
        r_t2_confluence_refuted: certificate.r_t2_confluence_refuted,
        r_t3_user_adjudication_opened: certificate.r_t3_user_adjudication_opened,
        serialized_blockers: certificate.serialized_blockers.clone(),
        errors,
    }
}

pub fn emit_r_t2_future_hole_confluence_v2_create_new(
    path: &Path,
) -> Result<Rt2FutureHoleConfluenceV2Replay, Rt2FutureHoleConfluenceV2Error> {
    let certificate = issue_r_t2_future_hole_confluence_v2_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Rt2FutureHoleConfluenceV2Error::Io(error.to_string()))?;
    let replay = replay_r_t2_future_hole_confluence_v2_certificate(&certificate);
    if !replay.valid {
        return Err(Rt2FutureHoleConfluenceV2Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn certificate() -> Rt2FutureHoleConfluenceV2Certificate {
        static CERTIFICATE: OnceLock<Rt2FutureHoleConfluenceV2Certificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_r_t2_future_hole_confluence_v2_certificate().unwrap())
            .clone()
    }

    #[test]
    fn four_complete_exact_prefix_surfaces_are_compared_only_after_the_gates() {
        let certificate = certificate();
        assert_eq!(certificate.branch_count, 4);
        assert!(certificate.exact_four_way_r_t1_class_join);
        assert!(certificate.four_exact_stage5_prefixes_independently_generated);
        assert!(certificate.full_relative_a3_seed_constructor_coverage_every_branch);
        assert!(certificate.zero_named_gaps_every_branch);
        assert!(certificate.full_adopted_frozen_semantic_scope_exhaustive_every_branch);
        assert!(certificate.law_level_promotion_gate_passed);
        assert!(certificate.branches.iter().all(|branch| {
            let aggregate = &branch.future_hole_aggregate;
            let arity_three_rows = aggregate
                .registrations
                .iter()
                .filter(|row| row.dependent_context_declared_arity == Some(3))
                .collect::<Vec<_>>();
            branch.complete_certified_scheme_count == 5
                && branch
                    .coverage_gate
                    .exact_compared_scheme_set_equals_inventory_promotions
                && aggregate.registration_count == 5
                && aggregate.unary_registration_count == 4
                && aggregate.structural_registration_count == 1
                && aggregate.exact_scheme_and_instance_coverage
                && aggregate.f_dc5_same_issuer_scope_preserved
                && aggregate.every_registration_replay_valid
                && aggregate.every_registration_hypothetically_derivable
                && aggregate.every_registration_exact_prefix_bound
                && !aggregate.successful_rows_provisional_due_motive_parametric_v1
                && aggregate.successful_rows_law_level_authoritative
                && aggregate.upstream_soundness_blockers.is_empty()
                && !aggregate.historical_realization_required_for_r_t2_formation
                && aggregate.zero_named_gaps
                && arity_three_rows.len() == 1
                && arity_three_rows[0].opaque_prior_clause_reference_count == 1
                && aggregate.registrations.iter().all(|row| {
                    row.exact_registration_prefix_bound
                        && row.dependent_context_version.as_deref()
                            == Some(DEPENDENT_AMBIENT_CONTEXT_VERSION)
                        && row.dependent_totality_theorem_replayed
                        && row.opaque_prior_clause_references_replayed_whole
                        && row.no_outcome_filtering_used
                        && !row.branch_local_family_id_used_in_semantic_key
                        && row.semantic_scope_premises_satisfied
                        && !row.motive_parametric_v1_success_row_provisional
                        && row.motive_parametric_theorem_version == DEPENDENT_TOTALITY_VERSION
                        && row.hypothetical_derivation_replayable
                        && row.law_level_semantic_scope_satisfied
                        && row.upstream_soundness_blockers.is_empty()
                        && row.zero_named_gaps
                })
        }));
        assert_eq!(certificate.pairwise_scheme_set_comparisons.len(), 6);
        assert!(certificate.every_pairwise_comparison_well_formed);
        assert!(certificate.order_reversal_invariant);
        assert!(certificate.all_family_equality_scopes_from_declared_parameter_arity);
        assert!(!certificate.maximum_free_variable_reference_used_as_scope);
        assert!(!certificate.r_t2_confluence_proved);
        assert!(certificate.r_t2_confluence_refuted);
        assert!(certificate.r_t3_user_adjudication_opened);
        assert!(certificate.selected_candidate_hash.is_none());
        assert!(!certificate.bridge_authorized);
        assert!(!certificate.halt_claim_issued);
    }

    #[test]
    fn order_reversal_and_law_level_promotion_are_verdict_invariant() {
        let certificate = certificate();
        assert_eq!(
            certificate
                .order_reversal_audit
                .forward_normalized_pair_verdicts,
            certificate
                .order_reversal_audit
                .reversed_normalized_pair_verdicts
        );
        assert_eq!(
            certificate.order_reversal_audit.forward_complete_set_hashes,
            certificate
                .order_reversal_audit
                .reversed_complete_set_hashes
        );
        assert_eq!(
            certificate.outcome,
            Rt2FutureHoleConfluenceV2Outcome::LawLevelConfluenceRefutedRungRt3Opened
        );
        assert!(!certificate.all_stage5_scheme_sets_equivalent);
        assert!(!certificate.r_t2_confluence_proved);
        assert!(certificate.r_t2_confluence_refuted);
        assert!(certificate.r_t3_user_adjudication_opened);
        assert!(certificate.serialized_blockers.is_empty());
    }

    #[test]
    fn replay_rejects_redigested_arity_coverage_and_gap_mutations() {
        let certificate = certificate();

        let mut arity_forgery = certificate.clone();
        let family = &mut arity_forgery.branches[0].schemes[0].source_families[0];
        family.declared_parameter_arity += 1;
        family.equality_scope_len = family.declared_parameter_arity;
        family.presentation_hash = tagged_hash("declared-arity-family-presentation", family);
        arity_forgery.result_digest = certificate_digest(&arity_forgery);
        assert!(!replay_against_expected(&arity_forgery, &certificate).valid);

        let mut coverage_forgery = certificate.clone();
        coverage_forgery.branches[0]
            .coverage_gate
            .full_relative_a3_seed_constructor_coverage = false;
        coverage_forgery.branches[0]
            .coverage_gate
            .named_gaps
            .push("FORGED_RELATIVE_COVERAGE_GAP".to_owned());
        coverage_forgery.branches[0].coverage_gate.zero_named_gaps = false;
        coverage_forgery.branches[0].coverage_gate.derivation_hash = tagged_hash(
            "branch-relative-a3-coverage-gate",
            &coverage_forgery.branches[0].coverage_gate,
        );
        coverage_forgery.result_digest = certificate_digest(&coverage_forgery);
        assert!(!replay_against_expected(&coverage_forgery, &certificate).valid);

        let mut gap_forgery = certificate.clone();
        gap_forgery.branches[0].future_hole_aggregate.registrations[0]
            .named_gaps
            .push("FORGED_REGISTERED_FUTURE_HOLE_GAP".to_owned());
        gap_forgery.branches[0].future_hole_aggregate.registrations[0].zero_named_gaps = false;
        gap_forgery.branches[0].future_hole_aggregate.registrations[0].derivation_hash =
            tagged_hash(
                "registered-future-hole-gate",
                &gap_forgery.branches[0].future_hole_aggregate.registrations[0],
            );
        gap_forgery.branches[0]
            .future_hole_aggregate
            .named_gaps
            .push("FORGED_REGISTERED_FUTURE_HOLE_GAP".to_owned());
        gap_forgery.branches[0]
            .future_hole_aggregate
            .zero_named_gaps = false;
        gap_forgery.branches[0]
            .future_hole_aggregate
            .aggregate_derivation_hash = tagged_hash(
            "future-hole-aggregate-gate",
            &gap_forgery.branches[0].future_hole_aggregate,
        );
        gap_forgery.result_digest = certificate_digest(&gap_forgery);
        assert!(!replay_against_expected(&gap_forgery, &certificate).valid);

        let mut dependent_reference_forgery = certificate.clone();
        let opaque_row = dependent_reference_forgery.branches[0]
            .future_hole_aggregate
            .registrations
            .iter_mut()
            .find(|row| row.opaque_prior_clause_reference_count == 1)
            .expect("one registered opaque-motive row");
        opaque_row.opaque_prior_clause_references_replayed_whole = false;
        opaque_row.derivation_hash = tagged_hash("registered-future-hole-gate", opaque_row);
        dependent_reference_forgery.branches[0]
            .future_hole_aggregate
            .aggregate_derivation_hash = tagged_hash(
            "future-hole-aggregate-gate",
            &dependent_reference_forgery.branches[0].future_hole_aggregate,
        );
        dependent_reference_forgery.result_digest =
            certificate_digest(&dependent_reference_forgery);
        assert!(!replay_against_expected(&dependent_reference_forgery, &certificate).valid);
    }
}
