//! Corrected create-new E-5 orchestration over the versioned future-hole API.
//!
//! This versioned module leaves the frozen v1 theorem surface untouched.
//! Registration is joined to the independently proved exact-window A3
//! inventory. Structural realization is separately joined to the ordinary
//! filler charge already certified by Phase 5b; neither a hole nor its
//! discharge can erase, duplicate, or mint that charge.

use crate::naturality_orbit_transport::{
    NaturalityOrbitTransportCertificate, issue_naturality_orbit_transport_certificate,
    replay_naturality_orbit_transport_certificate,
};
use crate::phase5b_history_certification::{
    FullHistoryStepCertification, PHASE5B_HISTORY_CERT_SCHEMA, Phase5bHistoryCertificate,
    replay_phase5b_history_certificate,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3DemandSchemeOrigin, A3HistoricalWindow, A3RuleConstructor, A3SeedRejectionReason,
    A3TypedDemandInstance, A3TypedDemandScheme, generate_a3_window_for_prefix,
};
use pen_eval::a3_rule_inventory_exhaustiveness::{
    A3RuleInventoryExhaustivenessCertificate, A3WindowRuleInventoryProof,
    issue_historical_a3_rule_inventory_exhaustiveness,
    replay_historical_a3_rule_inventory_exhaustiveness,
};
use pen_eval::debt_guard::directive_debt_timeline;
use pen_eval::future_hole_hypothesis_v2 as future_v2;
use pen_type::dependent_context::DependentContextMotive;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use pen_type::motive_parametric_coherence_v2::{
    CLOSURE_RULE_INVENTORY_V2, MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const E5_FUTURE_HOLE_FINALE_V2_SCHEMA: &str = "schema2-e5-future-hole-finale-v2";
pub const E5_FUTURE_HOLE_FINALE_V2_DATE: &str = "2026-07-22";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/future_hole_definition_adjudication.md");
const DEPENDENT_CONTEXT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/dependent_context_adjudication.md");
const DEPENDENT_CONTEXT_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/dependent_context.rs");
const FUTURE_V2_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/future_hole_hypothesis_v2.rs");
const CONTEXTUAL_INTERNALITY_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/contextual_internality.rs");
const AMBIENT_FORMER_INTERNALITY_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/ambient_former_internality.rs");
const MOTIVE_PARAMETRIC_COHERENCE_V2_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence_v2.rs");
const SUBSTITUTION_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/substitution.rs");
const ADMISSIBILITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const A3_EXHAUSTIVENESS_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/a3_rule_inventory_exhaustiveness.rs");
const A3_EXHAUSTIVENESS_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/a3_rule_inventory_exhaustiveness_v2.json");
const TRANSPORT_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const PHASE5B_SOURCE_BYTES: &[u8] = include_bytes!("phase5b_history_certification.rs");
const PHASE5B_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const DEBT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/debt_guard.rs");
const THEOREM_BYTES: &[u8] = include_bytes!("../../../docs/t1_result.md");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("e5_future_hole_finale_v2.rs");

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(E5_FUTURE_HOLE_FINALE_V2_SCHEMA, domain, value))
        .expect("E-5 v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2A3CoverageAudit {
    pub certificate_schema: String,
    pub certificate_digest: String,
    pub certificate_replayed: bool,
    pub historical_window_count: usize,
    pub stage16_exact_prefix_signature_digest: String,
    pub stage16_window_derivation_hash: String,
    pub stage16_exhaustiveness_derivation_hash: String,
    pub stage16_promoted_instance_count: usize,
    pub stage16_operational_instance_count: usize,
    pub stage16_promoted_instances_join_operational_inventory_bijectively: bool,
    pub every_historical_window_covered: bool,
    pub every_raw_seed_promoted_or_rejected: bool,
    pub every_operational_occurrence_has_an_independent_preimage: bool,
    pub every_operational_rule_constructor_has_an_independent_preimage: bool,
    pub structural_future_hole_clauses_included: bool,
    pub completed_schema2_artifact_digest_verified: bool,
    pub every_completed_schema_source_is_an_a3_typed_parameter: bool,
    pub global_e4_wrapped_domain_class_exhaustion_bound: bool,
    pub global_e4_no_unknown_survives_on_frozen_wrapped_surface: bool,
    pub relative_rule_constructor_inventory_exhaustiveness_proved: bool,
    pub broader_absolute_semantic_exhaustiveness_claimed: bool,
    pub named_scope_limits: Vec<String>,
    pub desired_count_score_bar_winner_or_halt_forbidden_as_input: bool,
    pub adopted_and_frozen_relative_a3_coverage_complete: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2RegistrationSummary {
    pub stage: u32,
    pub kind: String,
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub natural_family_id: String,
    pub occurrence_id: String,
    pub structural_constructor: Option<String>,
    pub jurisdiction_stage: Option<u32>,
    pub stage3_to_stage4_jurisdiction_transport: bool,
    pub registration_input_prefix_last_step: u32,
    pub registration_input_signature_digest: String,
    pub ambient_registration_prefix_last_step: u32,
    pub ambient_registration_prefix_signature_digest: String,
    pub ambient_declaration_signature_digest: String,
    pub declaration_and_jurisdiction_locality_proved: bool,
    pub stage3_declaration_jurisdiction_separation_proved: bool,
    pub external_exhaustiveness_evidence_hash: String,
    pub external_exhaustiveness_join_exact: bool,
    pub every_hole_live: bool,
    pub no_reflexivity_fallback: bool,
    pub local_semantic_scope_premises_satisfied: bool,
    pub hole_marginal_charge_zero: bool,
    pub formation_hash: String,
    pub deterministic_replay_valid: bool,
    pub gap_free: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2NamedGapAudit {
    pub falsifier: String,
    pub id: String,
    pub phase: String,
    pub a3_scheme_id: String,
    pub a3_instance_id: String,
    pub occurrence_id: Option<String>,
    pub exact_error: String,
    pub input_digest: String,
    pub gap_hash: String,
    pub deterministic_replay_valid: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2MembershipRow {
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub rule: String,
    pub source_step: Option<u32>,
    pub source_clause: Option<u16>,
    pub natural_family_id: Option<String>,
    pub orbit_id: Option<String>,
    pub d_membership_derivation_hash: String,
    pub proof_strength_exact_source_replayed: Option<bool>,
    pub proof_strength_exact_body_binding_replayed: Option<bool>,
    pub stable_specialization_totality_proved: Option<bool>,
    pub hypothetical_derivation_replayed: bool,
    pub output_kernel_typed: bool,
    pub uniform_specialization_not_new_family: bool,
    pub independently_exported_output_orbit: bool,
    pub hole_marginal_charge_zero: Option<bool>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2Stage16Audit {
    pub exact_a3_inventory_count: usize,
    pub exhaustiveness_promoted_count: usize,
    pub unary_registration_count: usize,
    pub unary_gap_count: usize,
    pub unary_named_gaps: Vec<E5V2NamedGapAudit>,
    pub accepted_unary_exact_source_replay_count: usize,
    pub accepted_unary_exact_body_binding_count: usize,
    pub accepted_unary_totality_provisional_instance_ids: Vec<String>,
    pub stable_source_guard_audit_replayed: bool,
    pub universal_unary_specialization_totality_proved: bool,
    pub dependent_context_registration_count: usize,
    pub published_dependent_redeclaration_count: usize,
    pub total_specialization_theorem_replay_count: usize,
    pub f_dc1_stage16_rows_rederived: bool,
    pub f_dc2_no_outcome_filtering: bool,
    pub f_dc3_no_partial_credit: bool,
    pub f_dc4_opaque_references_sealed_and_whole: bool,
    pub f_dc5_same_issuer_scope_preserved: bool,
    pub direct_chronological_instance_count: usize,
    pub pointwise_chronological_instance_count: usize,
    pub higher_instance_count: usize,
    pub structural_instance_count: usize,
    pub higher_seed_rejected_for_no_typed_path_witness: bool,
    pub chronological_transport_bijection: bool,
    pub direct_regression_64_to_8: bool,
    pub pointwise_joined_to_same_eight_families: bool,
    pub unary_ids_join_exactly: bool,
    pub every_unary_registration_prefix_local: bool,
    pub every_unary_registration_evidence_replayed: bool,
    pub three_membership_parts_pairwise_disjoint: bool,
    pub exact_inventory_union_bijection: bool,
    pub registrations: Vec<E5V2RegistrationSummary>,
    pub membership_rows: Vec<E5V2MembershipRow>,
    pub derivable_instance_ids: Vec<String>,
    pub underdetermined_instance_ids: Vec<String>,
    pub every_instance_d_decided: bool,
    pub d_partition_complete: bool,
    pub semantic_o16_empty: Option<bool>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2OrdinaryFillerChargeSummary {
    pub filler_step: u32,
    pub filler_candidate_hash: String,
    pub phase5b_step_derivation_hash: String,
    pub ordinary_kappa: u32,
    pub certified_nu: u32,
    pub bit_length: u32,
    pub ordinary_family_token_hashes: Vec<String>,
    pub phase5b_candidate_hash_join_exact: bool,
    pub phase5b_kappa_join_exact: bool,
    pub phase5b_nu_join_exact: bool,
    pub phase5b_family_tokens_join_exact: bool,
    pub provenance_replays: bool,
    pub provenance_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2RealizationSummary {
    pub registration_stage: u32,
    pub jurisdiction_stage: u32,
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub occurrence_id: String,
    pub structural_constructor: String,
    pub provider_family: String,
    pub registration_formation_hash: String,
    pub filler_step: u32,
    pub filler_candidate_hash: String,
    pub filler_is_whole_sealed_entry: bool,
    pub provider_relation_satisfied: bool,
    pub constructor_live_before_filler: bool,
    pub constructor_absent_after_filler: bool,
    pub specialized_expression_is_filler_reference: bool,
    pub clause4_prime_instantiation_replayed: bool,
    pub ordinary_filler_charge: E5V2OrdinaryFillerChargeSummary,
    pub discharge_marginal_charge_zero: bool,
    pub ordinary_charge_preserved_not_zeroed_or_reminted: bool,
    pub realization_hash: String,
    pub deterministic_replay_valid: bool,
    pub gap_free: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2ProviderControlAudit {
    pub authoritative_phase5b_provider_count: usize,
    pub correct_provider_join_count: usize,
    pub wrong_provider_trial_count: usize,
    pub wrong_provider_rejection_count: usize,
    pub charge_swap_trial_count: usize,
    pub charge_swap_rejection_count: usize,
    pub self_hashed_charge_swap_would_replay_without_authoritative_join: bool,
    pub authoritative_history_join_mandatory: bool,
    pub every_wrong_provider_rejected: bool,
    pub every_charge_swap_rejected: bool,
    pub no_hole_or_discharge_minted_filler_credit: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2Stage3To4Transport {
    pub stage3_a3_instance_id: String,
    pub stage4_a3_instance_id: String,
    pub occurrence_ids_intentionally_distinct: bool,
    pub same_natural_family: bool,
    pub same_structural_constructor: bool,
    pub stage3_jurisdiction_transport_present: bool,
    pub declaration_jurisdiction_prefix_separation_proved: bool,
    pub both_jurisdiction_stage_four: bool,
    pub same_filler_step_four: bool,
    pub same_phase5b_provider: bool,
    pub both_clause4_prime_instantiations_replayed: bool,
    pub semantic_scheme_transport_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2HistoricalFocusRow {
    pub stage: u32,
    pub coarse_required_packages: Vec<String>,
    pub exact_a3_required_packages: Vec<String>,
    pub a3_constructor_evidence: Vec<String>,
    pub registered_structural_constructors: Vec<String>,
    pub realized_structural_constructors: Vec<String>,
    pub projected_focus: Option<String>,
    pub demand_precedes_jurisdiction: bool,
    pub stage_three_wrinkle_replayed: bool,
    pub focus_projection_reproduces_coarse_record: bool,
    pub every_standing_hole_realized_by_specialization: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2HistoricalLedgerAudit {
    pub phase5b_certificate_digest: String,
    pub phase5b_archive_digest_valid: bool,
    pub phase5b_live_replay_valid: bool,
    pub phase5b_live_replay_errors: Vec<String>,
    pub frozen_phase5b_surface_drift_bound_explicitly: bool,
    pub historical_artifact_reinterpreted_after_drift: bool,
    pub all_fifteen_fillers_certified: bool,
    pub all_amplification_provenanced: bool,
    pub structural_registration_count: usize,
    pub structural_registration_gap_count: usize,
    pub structural_realization_count: usize,
    pub structural_realization_gap_count: usize,
    pub every_structural_registration_prefix_local: bool,
    pub every_structural_registration_evidence_replayed: bool,
    pub stage3_declaration_jurisdiction_separation_proved: bool,
    pub registrations: Vec<E5V2RegistrationSummary>,
    pub realizations: Vec<E5V2RealizationSummary>,
    pub provider_controls: E5V2ProviderControlAudit,
    pub stage3_to_stage4_transport: E5V2Stage3To4Transport,
    pub focus_rows: Vec<E5V2HistoricalFocusRow>,
    pub exact_structural_a3_registration_bijection: bool,
    pub f_fh1_every_historical_discharge_rederived: bool,
    pub focus_projection_reproduces_entire_coarse_ladder: bool,
    pub stage_three_wrinkle_reproduced: bool,
    pub coarse_o16_empty: bool,
    pub semantic_o16_agrees_with_coarse_projection: Option<bool>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5V2F1Audit {
    pub adopted_and_frozen_a3_coverage_complete: bool,
    pub d_partition_complete: bool,
    pub historical_realization_complete: bool,
    pub f1_preconditions_complete: bool,
    pub f1_executed: bool,
    pub demanded_instance_count: usize,
    pub derivable_instance_count: usize,
    pub demanded_but_underdetermined_instance: Option<String>,
    pub f1_triggered: Option<bool>,
    pub f1_excluded: Option<bool>,
    pub theorem12_full_instance_granularity_proved: Option<bool>,
    pub theorem12_refuted: Option<bool>,
    pub semantic_o16_empty: Option<bool>,
    pub disposition: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FutureHoleFinaleV2Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<E5V2SourceBinding>,
    pub desired_count_bar_winner_halt_or_f1_verdict_used_as_definition_input: bool,
    pub future_v2_evidence_digest: String,
    pub a3_coverage: E5V2A3CoverageAudit,
    pub stage16: E5V2Stage16Audit,
    pub historical: E5V2HistoricalLedgerAudit,
    pub f1: E5V2F1Audit,
    pub f_fh4_triggered: bool,
    pub blocking_obligations: Vec<String>,
    pub e5_complete: bool,
    pub semantic_o16_certificate_issued: bool,
    pub t_bf2_authorized: bool,
    pub e5_side_of_bridge_satisfied: bool,
    pub r_t2_confluence_consumed: bool,
    pub bridge_authorized: bool,
    pub bridge_executed: bool,
    pub halt_claim_issued: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FutureHoleFinaleV2Replay {
    pub valid: bool,
    pub stage16_membership_count: usize,
    pub stage16_underdetermined_count: usize,
    pub unary_registration_count: usize,
    pub structural_registration_count: usize,
    pub structural_realization_count: usize,
    pub semantic_o16_empty: Option<bool>,
    pub f1_executed: bool,
    pub f1_excluded: Option<bool>,
    pub f_fh4_triggered: bool,
    pub e5_complete: bool,
    pub t_bf2_authorized: bool,
    pub bridge_authorized: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum E5FutureHoleFinaleV2Error {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("E-5 v2 invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn source_bindings() -> Vec<E5V2SourceBinding> {
    [
        (
            "docs/future_hole_definition_adjudication.md",
            "adopted_future_hole_definition",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/dependent_context_adjudication.md",
            "adopted_dependent_ambient_context_extension",
            DEPENDENT_CONTEXT_ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-type/src/dependent_context.rs",
            "dependent_telescope_sequential_substitution_and_totality_kernel",
            DEPENDENT_CONTEXT_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/future_hole_hypothesis_v2.rs",
            "versioned_body_motive_registration_and_realization_API",
            FUTURE_V2_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/contextual_internality.rs",
            "motive_typed_hypothesis_and_closed_assignment_kernel",
            CONTEXTUAL_INTERNALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/ambient_former_internality.rs",
            "transparent_former_internality_kernel",
            AMBIENT_FORMER_INTERNALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence_v2.rs",
            "exact_source_bound_specialization_eliminator_v2",
            MOTIVE_PARAMETRIC_COHERENCE_V2_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/substitution.rs",
            "typed_structural_substitution_kernel",
            SUBSTITUTION_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/admissibility.rs",
            "guarded_provider_decision_kernel",
            ADMISSIBILITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "count_blind_exact_window_A3_generator",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_rule_inventory_exhaustiveness.rs",
            "relative_A3_rule_inventory_exhaustiveness_theorem",
            A3_EXHAUSTIVENESS_SOURCE_BYTES,
        ),
        (
            "docs/a3_rule_inventory_exhaustiveness_v2.json",
            "stable_create_new_A3_exhaustiveness_certificate",
            A3_EXHAUSTIVENESS_ARTIFACT_BYTES,
        ),
        (
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "Stage16_chronological_transport_and_64_to_8_regression",
            TRANSPORT_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/phase5b_history_certification.rs",
            "ordinary_filler_charge_provenance_issuer",
            PHASE5B_SOURCE_BYTES,
        ),
        (
            "docs/phase5b_full_history_v1.json",
            "frozen_certified_history_charge_ledger",
            PHASE5B_ARTIFACT_BYTES,
        ),
        (
            "crates/pen-eval/src/debt_guard.rs",
            "coarse_O_ladder_regression_corpus",
            DEBT_SOURCE_BYTES,
        ),
        (
            "docs/t1_result.md",
            "Theorem_12_and_F1_statement",
            THEOREM_BYTES,
        ),
        (
            "crates/pen-search/src/e5_future_hole_finale_v2.rs",
            "this_corrected_create_new_orchestration",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| E5V2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn certificate_digest(certificate: &E5FutureHoleFinaleV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn phase5b_archive_digest(certificate: &Phase5bHistoryCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(
        PHASE5B_HISTORY_CERT_SCHEMA,
        "phase5b-history-certificate",
        &projection,
    ))
    .expect("Phase 5b certificate serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn exact_prefix(stage: u32) -> SealedSignature {
    SealedSignature::from_telescopes(
        (1..stage)
            .map(|step| (step, Telescope::reference(step)))
            .collect(),
    )
}

fn scheme_for_instance<'a>(
    window: &'a A3HistoricalWindow,
    instance: &A3TypedDemandInstance,
) -> Result<&'a A3TypedDemandScheme, E5FutureHoleFinaleV2Error> {
    window
        .schemes
        .iter()
        .find(|scheme| scheme.scheme_id == instance.scheme_id)
        .ok_or_else(|| {
            E5FutureHoleFinaleV2Error::Invariant(format!(
                "Stage {} instance {} has no scheme",
                window.stage, instance.instance_id
            ))
        })
}

fn proof_for_stage<'a>(
    certificate: &'a A3RuleInventoryExhaustivenessCertificate,
    stage: u32,
) -> Result<&'a A3WindowRuleInventoryProof, E5FutureHoleFinaleV2Error> {
    certificate
        .historical_windows
        .iter()
        .find(|proof| proof.stage == stage)
        .ok_or_else(|| {
            E5FutureHoleFinaleV2Error::Invariant(format!(
                "A3 exhaustiveness certificate omits Stage {stage}"
            ))
        })
}

fn phase5b_step<'a>(
    history: &'a Phase5bHistoryCertificate,
    step: u32,
) -> Result<&'a FullHistoryStepCertification, E5FutureHoleFinaleV2Error> {
    history
        .steps
        .iter()
        .find(|row| row.step == step)
        .ok_or_else(|| {
            E5FutureHoleFinaleV2Error::Invariant(format!(
                "Phase 5b history omits filler Step {step}"
            ))
        })
}

fn row_hash(row: &E5V2MembershipRow) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("D-membership-row", &projection)
}

fn registration_summary_hash(row: &E5V2RegistrationSummary) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("registration-summary", &projection)
}

fn realization_summary_hash(row: &E5V2RealizationSummary) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("realization-summary", &projection)
}

fn promoted_instance_ids(proof: &A3WindowRuleInventoryProof) -> BTreeSet<String> {
    proof
        .base_seed_dispositions
        .iter()
        .filter_map(|row| row.promoted_instance_id.clone())
        .chain(
            proof
                .structural_seed_dispositions
                .iter()
                .filter_map(|row| row.structural_instance_id.clone()),
        )
        .collect()
}

fn structural_constructor_name(scheme: &A3TypedDemandScheme) -> Option<String> {
    match &scheme.origin {
        A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } => {
            Some(constructor.slug().to_owned())
        }
        A3DemandSchemeOrigin::BaseRule { .. } => None,
    }
}

fn registered_summary(
    stage: u32,
    registration_input: &SealedSignature,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    proof: &A3WindowRuleInventoryProof,
    registration: &future_v2::RegisteredFutureHoleV2,
    replay_valid: bool,
) -> E5V2RegistrationSummary {
    let (kind, structural_constructor, jurisdiction_stage, stage3_transport) =
        match &registration.output_contract {
            future_v2::FutureHoleOutputContractV2::UnaryActionAt { .. } => {
                ("unary_action", None, None, false)
            }
            future_v2::FutureHoleOutputContractV2::StructuralProvides(contract) => (
                "structural_completion_hole",
                Some(contract.constructor.slug().to_owned()),
                Some(contract.jurisdiction.jurisdiction_stage()),
                matches!(
                    contract.jurisdiction,
                    future_v2::StructuralJurisdictionV2::Stage3To4 { .. }
                ),
            ),
        };
    let external_hash = registration
        .external_exhaustiveness_evidence_hash
        .clone()
        .unwrap_or_default();
    let expected_registration_input = exact_prefix(if stage == 3 { 4 } else { stage });
    let expected_ambient_prefix = exact_prefix(stage);
    let registration_input_prefix_last_step = if stage == 3 {
        3
    } else {
        stage.saturating_sub(1)
    };
    let declaration_and_jurisdiction_locality_proved = registration_input.digest()
        == expected_registration_input.digest()
        && registration.ambient_registration_prefix_last_step == stage.saturating_sub(1)
        && registration.ambient_registration_prefix_signature_digest
            == expected_ambient_prefix.digest()
        && registration.ambient_declaration.signature_digest == expected_ambient_prefix.digest()
        && registration.ambient_declaration_bound_to_registration_prefix;
    let stage3_declaration_jurisdiction_separation_proved = stage == 3
        && registration_input_prefix_last_step == 3
        && registration.ambient_registration_prefix_last_step == 2
        && registration_input.digest() != registration.ambient_declaration.signature_digest;
    let mut summary = E5V2RegistrationSummary {
        stage,
        kind: kind.to_owned(),
        a3_instance_id: instance.instance_id.clone(),
        a3_scheme_id: scheme.scheme_id.clone(),
        natural_family_id: registration.natural_family_id.clone(),
        occurrence_id: registration.occurrence_id.clone(),
        structural_constructor,
        jurisdiction_stage,
        stage3_to_stage4_jurisdiction_transport: stage3_transport,
        registration_input_prefix_last_step,
        registration_input_signature_digest: registration_input.digest().to_owned(),
        ambient_registration_prefix_last_step: registration.ambient_registration_prefix_last_step,
        ambient_registration_prefix_signature_digest: registration
            .ambient_registration_prefix_signature_digest
            .clone(),
        ambient_declaration_signature_digest: registration
            .ambient_declaration
            .signature_digest
            .clone(),
        declaration_and_jurisdiction_locality_proved,
        stage3_declaration_jurisdiction_separation_proved,
        external_exhaustiveness_evidence_hash: external_hash.clone(),
        external_exhaustiveness_join_exact: external_hash == proof.derivation_hash,
        every_hole_live: registration.every_hole_live,
        no_reflexivity_fallback: registration.no_reflexivity_fallback,
        local_semantic_scope_premises_satisfied: registration
            .local_semantic_scope_premises_satisfied,
        hole_marginal_charge_zero: registration.hole_marginal_charge.replays_as_zero(),
        formation_hash: registration.formation_hash.clone(),
        deterministic_replay_valid: replay_valid,
        gap_free: true,
        derivation_hash: String::new(),
    };
    summary.derivation_hash = registration_summary_hash(&summary);
    summary
}

fn gap_registration_summary(
    stage: u32,
    registration_input: &SealedSignature,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    proof: &A3WindowRuleInventoryProof,
    gap: &future_v2::NamedFutureHoleGapV2,
    replay_valid: bool,
) -> E5V2RegistrationSummary {
    let mut summary = E5V2RegistrationSummary {
        stage,
        kind: match scheme.rule_constructor {
            A3RuleConstructor::UnaryAction => "unary_action",
            A3RuleConstructor::StructuralCompletionHole => "structural_completion_hole",
            _ => "outside_future_surface",
        }
        .to_owned(),
        a3_instance_id: instance.instance_id.clone(),
        a3_scheme_id: scheme.scheme_id.clone(),
        natural_family_id: String::new(),
        occurrence_id: gap.occurrence_id.clone().unwrap_or_default(),
        structural_constructor: structural_constructor_name(scheme),
        jurisdiction_stage: None,
        stage3_to_stage4_jurisdiction_transport: false,
        registration_input_prefix_last_step: if stage == 3 {
            3
        } else {
            stage.saturating_sub(1)
        },
        registration_input_signature_digest: registration_input.digest().to_owned(),
        ambient_registration_prefix_last_step: 0,
        ambient_registration_prefix_signature_digest: String::new(),
        ambient_declaration_signature_digest: String::new(),
        declaration_and_jurisdiction_locality_proved: false,
        stage3_declaration_jurisdiction_separation_proved: false,
        external_exhaustiveness_evidence_hash: proof.derivation_hash.clone(),
        external_exhaustiveness_join_exact: false,
        every_hole_live: false,
        no_reflexivity_fallback: false,
        local_semantic_scope_premises_satisfied: false,
        hole_marginal_charge_zero: false,
        formation_hash: gap.gap_hash.clone(),
        deterministic_replay_valid: replay_valid,
        gap_free: false,
        derivation_hash: String::new(),
    };
    summary.derivation_hash = registration_summary_hash(&summary);
    summary
}

fn named_gap_audit(
    gap: &future_v2::NamedFutureHoleGapV2,
    deterministic_replay_valid: bool,
) -> E5V2NamedGapAudit {
    E5V2NamedGapAudit {
        falsifier: "F-FH4".to_owned(),
        id: gap.id.clone(),
        phase: format!("{:?}", gap.phase),
        a3_scheme_id: gap.a3_scheme_id.clone(),
        a3_instance_id: gap.a3_instance_id.clone(),
        occurrence_id: gap.occurrence_id.clone(),
        exact_error: gap.exact_error.clone(),
        input_digest: gap.input_digest.clone(),
        gap_hash: gap.gap_hash.clone(),
        deterministic_replay_valid,
    }
}

fn proof_strength_exact_source_replays(registration: &future_v2::RegisteredFutureHoleV2) -> bool {
    let evidence = &registration.parametric_internality;
    let expected_rule_inventory = CLOSURE_RULE_INVENTORY_V2
        .iter()
        .map(|rule| format!("{rule:?}"))
        .collect::<Vec<_>>();
    evidence.hypothetical_internality_issued
        && evidence.eliminator_version == MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION
        && evidence.closure_rule_inventory == expected_rule_inventory
        && evidence.explicit_ambient_arity == registration.declared_ambient_arity
        && evidence.every_declared_parameter_live
        && evidence.every_body_former_registered
        && evidence
            .closure_formers
            .iter()
            .all(|former| evidence.registered_former_inventory.contains(former))
        && evidence.historical_prefix_declaration_hash
            == registration.ambient_declaration.declaration_hash
        && evidence.dependent_declaration_hash == registration.ambient_declaration.declaration_hash
        && evidence.dependent_totality_theorem_replayed
        && evidence.total_specialization_authoritative
        && evidence
            .dependent_totality_theorem
            .total_specialization_theorem_issued
        && evidence.dependent_totality_theorem.declaration_hash
            == registration.ambient_declaration.declaration_hash
        && !evidence.typed_body_elaboration_hash.is_empty()
        && !evidence.evidence_hash.is_empty()
        && evidence.marginal_nu == 0
}

fn proof_strength_exact_body_binding_replays(
    registration: &future_v2::RegisteredFutureHoleV2,
) -> bool {
    let evidence = &registration.parametric_internality;
    evidence.source_bound_to_exact_body
        && evidence.exact_body_candidate_hash == candidate_hash(&registration.body_telescope)
        && registration.ambient_declaration.body_telescope == registration.body_telescope
        && registration.ambient_declaration.candidate_hash == evidence.exact_body_candidate_hash
        && registration
            .body_telescope
            .clauses
            .first()
            .is_some_and(|clause| registration.ambient_declaration.expression == clause.expr)
        && registration.ambient_declaration.signature_digest
            == registration.ambient_registration_prefix_signature_digest
        && registration.ambient_declaration.visible_library
            == registration.visible_library_at_registration
}

fn ordinary_family_token_hashes(step: &FullHistoryStepCertification) -> Vec<String> {
    step.newly_issued_units
        .iter()
        .map(|unit| unit.derivation_hash.clone())
        .collect()
}

fn issue_phase5b_filler_charge(
    step: &FullHistoryStepCertification,
    filler: &Telescope,
) -> future_v2::FillerOrdinaryChargeProvenanceV2 {
    future_v2::issue_filler_ordinary_charge_provenance_v2(
        step.step,
        filler,
        step.kappa,
        step.certified_semantic_total,
        filler.bit_cost(),
        step.derivation_hash.clone(),
        ordinary_family_token_hashes(step),
    )
}

fn filler_charge_joins_phase5b_exactly(
    charge: &future_v2::FillerOrdinaryChargeProvenanceV2,
    step: &FullHistoryStepCertification,
    filler: &Telescope,
) -> bool {
    future_v2::replay_filler_ordinary_charge_provenance_v2(charge)
        && charge.filler_step == step.step
        && charge.filler_candidate_hash == step.candidate_hash
        && charge.filler_telescope_digest == step.candidate_hash
        && charge.ordinary_kappa == step.kappa
        && charge.ordinary_kappa == filler.kappa() as u32
        && charge.certified_nu == step.certified_semantic_total
        && charge.bit_length == filler.bit_cost()
        && charge.upstream_certificate_hash == step.derivation_hash
        && charge.ordinary_family_token_hashes == ordinary_family_token_hashes(step)
        && step.certified
        && step.every_counted_family_provenanced
}

fn filler_charge_summary(
    charge: &future_v2::FillerOrdinaryChargeProvenanceV2,
    step: &FullHistoryStepCertification,
    filler: &Telescope,
) -> E5V2OrdinaryFillerChargeSummary {
    E5V2OrdinaryFillerChargeSummary {
        filler_step: charge.filler_step,
        filler_candidate_hash: charge.filler_candidate_hash.clone(),
        phase5b_step_derivation_hash: step.derivation_hash.clone(),
        ordinary_kappa: charge.ordinary_kappa,
        certified_nu: charge.certified_nu,
        bit_length: charge.bit_length,
        ordinary_family_token_hashes: charge.ordinary_family_token_hashes.clone(),
        phase5b_candidate_hash_join_exact: charge.filler_candidate_hash == step.candidate_hash
            && charge.filler_telescope_digest == step.candidate_hash,
        phase5b_kappa_join_exact: charge.ordinary_kappa == step.kappa
            && charge.ordinary_kappa == filler.kappa() as u32,
        phase5b_nu_join_exact: charge.certified_nu == step.certified_semantic_total,
        phase5b_family_tokens_join_exact: charge.ordinary_family_token_hashes
            == ordinary_family_token_hashes(step),
        provenance_replays: future_v2::replay_filler_ordinary_charge_provenance_v2(charge),
        provenance_hash: charge.provenance_hash.clone(),
    }
}

fn transport_prerequisite() -> Result<NaturalityOrbitTransportCertificate, E5FutureHoleFinaleV2Error>
{
    let transport = issue_naturality_orbit_transport_certificate()
        .map_err(|error| E5FutureHoleFinaleV2Error::Prerequisite(error.to_string()))?;
    let replay = replay_naturality_orbit_transport_certificate(&transport);
    if !replay.valid {
        return Err(E5FutureHoleFinaleV2Error::Prerequisite(
            replay.errors.join("; "),
        ));
    }
    if !transport.a3.stage16_inventory_partition_exact
        || !transport.a3.chronological_instance_transport_bijection
        || !transport.a3.j3_regression_64_to_8
        || !transport.a3.pointwise_instances_join_existing_j3_families
        || !transport
            .a3
            .chronological_outputs_constructed_and_kernel_typed
        || transport.a3.unary_pending_outputs.len() != 17
        || transport.a3.structural_completion_pending_outputs.len() != 13
        || !transport.a3.frozen_surface_drift_bound_explicitly
    {
        return Err(E5FutureHoleFinaleV2Error::Prerequisite(
            "naturality transport omitted the exact 17+64+8 Stage-16 partition, the historical 13 structural occurrences, or its frozen-drift boundary"
                .to_owned(),
        ));
    }
    Ok(transport)
}

pub fn issue_e5_future_hole_finale_v2_certificate()
-> Result<E5FutureHoleFinaleV2Certificate, E5FutureHoleFinaleV2Error> {
    let archived_a3: A3RuleInventoryExhaustivenessCertificate =
        serde_json::from_slice(A3_EXHAUSTIVENESS_ARTIFACT_BYTES)
            .map_err(|error| E5FutureHoleFinaleV2Error::Json(error.to_string()))?;
    let archived_a3_replay = replay_historical_a3_rule_inventory_exhaustiveness(&archived_a3);
    if !archived_a3_replay.valid {
        return Err(E5FutureHoleFinaleV2Error::Prerequisite(format!(
            "stable A3 exhaustiveness certificate did not replay: {}",
            archived_a3_replay.errors.join("; ")
        )));
    }
    let live_a3 = issue_historical_a3_rule_inventory_exhaustiveness()
        .map_err(|error| E5FutureHoleFinaleV2Error::Prerequisite(error.to_string()))?;
    if live_a3 != archived_a3 {
        return Err(E5FutureHoleFinaleV2Error::Prerequisite(
            "live A3 exhaustiveness issuance differs from its stable create-new artifact"
                .to_owned(),
        ));
    }

    let mut windows = BTreeMap::<u32, A3HistoricalWindow>::new();
    let mut every_window_operational_join = true;
    for stage in 1..=16_u32 {
        let prefix = exact_prefix(stage);
        let window = generate_a3_window_for_prefix(&prefix, stage)
            .map_err(|error| E5FutureHoleFinaleV2Error::Prerequisite(error.to_string()))?;
        let proof = proof_for_stage(&archived_a3, stage)?;
        let operational_ids = window
            .instances
            .iter()
            .map(|instance| instance.instance_id.clone())
            .collect::<BTreeSet<_>>();
        every_window_operational_join &= proof.exact_prefix_signature_digest == prefix.digest()
            && proof.relative_rule_inventory_exhaustive_for_window
            && promoted_instance_ids(proof) == operational_ids;
        windows.insert(stage, window);
    }
    if !every_window_operational_join {
        return Err(E5FutureHoleFinaleV2Error::Invariant(
            "an exact historical A3 window does not join its exhaustiveness proof bijectively"
                .to_owned(),
        ));
    }
    let stage16_window = windows
        .get(&16)
        .ok_or_else(|| E5FutureHoleFinaleV2Error::Invariant("A3 omits Stage 16".to_owned()))?;
    let stage16_proof = proof_for_stage(&archived_a3, 16)?;
    let stage16_operational_ids = stage16_window
        .instances
        .iter()
        .map(|instance| instance.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let stage16_promoted_ids = promoted_instance_ids(stage16_proof);
    let stage16_promoted_join = stage16_promoted_ids == stage16_operational_ids;

    let adopted_and_frozen_relative_a3_coverage_complete = archived_a3_replay.valid
        && archived_a3.future_hole_adoption_replayed
        && archived_a3.every_historical_window_covered
        && archived_a3.every_raw_seed_promoted_or_rejected
        && archived_a3.every_operational_occurrence_has_an_independent_preimage
        && archived_a3.every_operational_rule_constructor_has_an_independent_preimage
        && archived_a3.structural_future_hole_clauses_included
        && archived_a3.completed_schema2_artifact_digest_verified
        && archived_a3.every_completed_schema_source_is_an_a3_typed_parameter
        && archived_a3.global_e4_wrapped_domain_class_exhaustion_bound
        && archived_a3.global_e4_no_unknown_survives_on_frozen_wrapped_surface
        && archived_a3.relative_rule_constructor_inventory_exhaustiveness_proved
        && !archived_a3.broader_absolute_semantic_exhaustiveness_claimed
        && archived_a3.forbidden_count_score_bar_winner_or_halt_input
        && every_window_operational_join
        && stage16_promoted_join;
    let a3_coverage_hash = tagged_hash(
        "A3-relative-coverage",
        &(
            &archived_a3.result_digest,
            stage16_window.window_derivation_hash.as_str(),
            stage16_proof.derivation_hash.as_str(),
            stage16_promoted_join,
            every_window_operational_join,
            adopted_and_frozen_relative_a3_coverage_complete,
        ),
    );
    let a3_coverage = E5V2A3CoverageAudit {
        certificate_schema: archived_a3.schema.clone(),
        certificate_digest: archived_a3.result_digest.clone(),
        certificate_replayed: archived_a3_replay.valid,
        historical_window_count: archived_a3.historical_windows.len(),
        stage16_exact_prefix_signature_digest: stage16_proof.exact_prefix_signature_digest.clone(),
        stage16_window_derivation_hash: stage16_window.window_derivation_hash.clone(),
        stage16_exhaustiveness_derivation_hash: stage16_proof.derivation_hash.clone(),
        stage16_promoted_instance_count: stage16_promoted_ids.len(),
        stage16_operational_instance_count: stage16_operational_ids.len(),
        stage16_promoted_instances_join_operational_inventory_bijectively: stage16_promoted_join,
        every_historical_window_covered: archived_a3.every_historical_window_covered,
        every_raw_seed_promoted_or_rejected: archived_a3.every_raw_seed_promoted_or_rejected,
        every_operational_occurrence_has_an_independent_preimage: archived_a3
            .every_operational_occurrence_has_an_independent_preimage,
        every_operational_rule_constructor_has_an_independent_preimage: archived_a3
            .every_operational_rule_constructor_has_an_independent_preimage,
        structural_future_hole_clauses_included: archived_a3
            .structural_future_hole_clauses_included,
        completed_schema2_artifact_digest_verified: archived_a3
            .completed_schema2_artifact_digest_verified,
        every_completed_schema_source_is_an_a3_typed_parameter: archived_a3
            .every_completed_schema_source_is_an_a3_typed_parameter,
        global_e4_wrapped_domain_class_exhaustion_bound: archived_a3
            .global_e4_wrapped_domain_class_exhaustion_bound,
        global_e4_no_unknown_survives_on_frozen_wrapped_surface: archived_a3
            .global_e4_no_unknown_survives_on_frozen_wrapped_surface,
        relative_rule_constructor_inventory_exhaustiveness_proved: archived_a3
            .relative_rule_constructor_inventory_exhaustiveness_proved,
        broader_absolute_semantic_exhaustiveness_claimed: archived_a3
            .broader_absolute_semantic_exhaustiveness_claimed,
        named_scope_limits: archived_a3.named_scope_limits.clone(),
        desired_count_score_bar_winner_or_halt_forbidden_as_input: archived_a3
            .forbidden_count_score_bar_winner_or_halt_input,
        adopted_and_frozen_relative_a3_coverage_complete,
        derivation_hash: a3_coverage_hash,
    };

    let transport = transport_prerequisite()?;
    let mut unary_summaries = Vec::new();
    let mut unary_named_gaps = Vec::new();
    let mut structural_summaries = Vec::new();
    let mut unary_registered = BTreeMap::<String, future_v2::RegisteredFutureHoleV2>::new();
    let mut structural_registered =
        BTreeMap::<(u32, String), future_v2::RegisteredFutureHoleV2>::new();
    let mut unary_gap_count = 0_usize;
    let mut structural_gap_count = 0_usize;
    let mut attempted_structural_ids = BTreeSet::<(u32, String)>::new();

    for stage in 1..=16_u32 {
        let window = windows.get(&stage).expect("all exact windows inserted");
        let proof = proof_for_stage(&archived_a3, stage)?;
        let registration_prefix = exact_prefix(if stage == 3 { 4 } else { stage });
        for instance in &window.instances {
            let scheme = scheme_for_instance(window, instance)?;
            let relevant = (stage == 16
                && scheme.rule_constructor == A3RuleConstructor::UnaryAction)
                || scheme.rule_constructor == A3RuleConstructor::StructuralCompletionHole;
            if !relevant {
                continue;
            }
            if scheme.rule_constructor == A3RuleConstructor::StructuralCompletionHole {
                attempted_structural_ids.insert((stage, instance.instance_id.clone()));
            }
            let disposition = match scheme.rule_constructor {
                A3RuleConstructor::UnaryAction => future_v2::register_unary_action_v2(
                    &registration_prefix,
                    window,
                    scheme,
                    instance,
                ),
                A3RuleConstructor::StructuralCompletionHole => {
                    future_v2::register_structural_future_hole_v2(
                        &registration_prefix,
                        window,
                        scheme,
                        instance,
                    )
                }
                _ => unreachable!("relevance restricts the future-hole surface"),
            }
            .map_err(|error| E5FutureHoleFinaleV2Error::Prerequisite(error.to_string()))?;
            match disposition {
                future_v2::FutureHoleRegistrationDispositionV2::Registered(raw) => {
                    let joined = future_v2::attach_external_exhaustiveness_evidence_v2(
                        &raw,
                        proof.derivation_hash.clone(),
                    )
                    .map_err(|error| E5FutureHoleFinaleV2Error::Prerequisite(error.to_string()))?;
                    let claimed =
                        future_v2::FutureHoleRegistrationDispositionV2::Registered(joined.clone());
                    let replay = future_v2::replay_future_hole_registration_v2(
                        &registration_prefix,
                        window,
                        scheme,
                        instance,
                        &claimed,
                    );
                    let summary = registered_summary(
                        stage,
                        &registration_prefix,
                        scheme,
                        instance,
                        proof,
                        &joined,
                        replay.valid,
                    );
                    if scheme.rule_constructor == A3RuleConstructor::UnaryAction {
                        unary_registered.insert(instance.instance_id.clone(), joined);
                        unary_summaries.push(summary);
                    } else {
                        structural_registered.insert((stage, instance.instance_id.clone()), joined);
                        structural_summaries.push(summary);
                    }
                }
                future_v2::FutureHoleRegistrationDispositionV2::Gap(gap) => {
                    let claimed = future_v2::FutureHoleRegistrationDispositionV2::Gap(gap.clone());
                    let replay = future_v2::replay_future_hole_registration_v2(
                        &registration_prefix,
                        window,
                        scheme,
                        instance,
                        &claimed,
                    );
                    let summary = gap_registration_summary(
                        stage,
                        &registration_prefix,
                        scheme,
                        instance,
                        proof,
                        &gap,
                        replay.valid,
                    );
                    if scheme.rule_constructor == A3RuleConstructor::UnaryAction {
                        unary_gap_count += 1;
                        unary_named_gaps.push(named_gap_audit(&gap, replay.valid));
                        unary_summaries.push(summary);
                    } else {
                        structural_gap_count += 1;
                        structural_summaries.push(summary);
                    }
                }
            }
        }
    }
    unary_summaries.sort_by(|left, right| left.a3_instance_id.cmp(&right.a3_instance_id));
    unary_named_gaps.sort_by(|left, right| left.a3_instance_id.cmp(&right.a3_instance_id));
    structural_summaries.sort_by(|left, right| {
        (left.stage, left.a3_instance_id.as_str())
            .cmp(&(right.stage, right.a3_instance_id.as_str()))
    });
    let every_unary_registration_prefix_local = unary_summaries.len() == 17
        && unary_summaries
            .iter()
            .all(|row| row.declaration_and_jurisdiction_locality_proved);
    let every_unary_registration_evidence_replayed = unary_summaries.len() == 17
        && unary_summaries.iter().all(|row| {
            row.gap_free
                && row.deterministic_replay_valid
                && row.external_exhaustiveness_join_exact
                && row.every_hole_live
                && row.no_reflexivity_fallback
                && row.local_semantic_scope_premises_satisfied
                && row.hole_marginal_charge_zero
        });
    let every_structural_registration_prefix_local = structural_summaries.len() == 13
        && structural_summaries
            .iter()
            .all(|row| row.declaration_and_jurisdiction_locality_proved);
    let every_structural_registration_evidence_replayed = structural_summaries.len() == 13
        && structural_summaries.iter().all(|row| {
            row.gap_free
                && row.deterministic_replay_valid
                && row.external_exhaustiveness_join_exact
                && row.every_hole_live
                && row.no_reflexivity_fallback
                && row.local_semantic_scope_premises_satisfied
                && row.hole_marginal_charge_zero
        });
    let stage3_rows = structural_summaries
        .iter()
        .filter(|row| row.stage == 3)
        .collect::<Vec<_>>();
    let stage3_declaration_jurisdiction_separation_proved =
        stage3_rows.len() == 1 && stage3_rows[0].stage3_declaration_jurisdiction_separation_proved;

    let accepted_unary_exact_source_replay_count = unary_registered
        .iter()
        .filter(|(instance_id, registration)| {
            proof_strength_exact_source_replays(registration)
                && unary_summaries.iter().any(|summary| {
                    summary.a3_instance_id == **instance_id
                        && summary.gap_free
                        && summary.deterministic_replay_valid
                        && summary.external_exhaustiveness_join_exact
                })
        })
        .count();
    let accepted_unary_exact_body_binding_count = unary_registered
        .iter()
        .filter(|(instance_id, registration)| {
            proof_strength_exact_body_binding_replays(registration)
                && unary_summaries.iter().any(|summary| {
                    summary.a3_instance_id == **instance_id
                        && summary.gap_free
                        && summary.deterministic_replay_valid
                        && summary.external_exhaustiveness_join_exact
                })
        })
        .count();
    // F-DC5 re-runs the same issuer after replacing the old independent-motive
    // approximation with the adopted dependent telescope.  Totality is now a
    // theorem attached to each declaration, not an empirical guard over a
    // selected assignment corpus.  `stable_source_guard_audit_replayed` stays
    // false intentionally: using such a guard would violate F-DC2.
    let stable_source_guard_audit_replayed = false;
    let dependent_context_registration_count = unary_registered.len();
    let published_dependent_redeclaration_count = unary_registered
        .values()
        .filter(|registration| {
            registration.declared_motives.iter().any(|motive| {
                matches!(
                    motive,
                    DependentContextMotive::ElementOfApplicationHead { .. }
                )
            })
        })
        .count();
    let total_specialization_theorem_replay_count = unary_registered
        .values()
        .filter(|registration| {
            let evidence = &registration.parametric_internality;
            evidence.dependent_totality_theorem_replayed
                && evidence.total_specialization_authoritative
                && evidence
                    .dependent_totality_theorem
                    .total_specialization_theorem_issued
                && evidence.dependent_totality_theorem.declaration_hash
                    == registration.ambient_declaration.declaration_hash
        })
        .count();
    let universal_unary_specialization_totality_proved = unary_registered.len() == 17
        && unary_gap_count == 0
        && total_specialization_theorem_replay_count == 17;
    let accepted_unary_totality_provisional_instance_ids = unary_registered
        .iter()
        .filter(|(_, registration)| {
            !registration
                .parametric_internality
                .total_specialization_authoritative
        })
        .map(|(instance_id, _)| instance_id.clone())
        .collect::<Vec<_>>();
    let f_dc1_stage16_rows_rederived = unary_registered.len() == 17
        && unary_gap_count == 0
        && unary_summaries.len() == 17
        && every_unary_registration_evidence_replayed;
    let f_dc2_no_outcome_filtering = unary_registered.values().all(|registration| {
        registration.ambient_declaration.no_outcome_filtering_used
            && registration
                .parametric_internality
                .dependent_totality_theorem
                .no_assignment_outcome_filtering
            && !stable_source_guard_audit_replayed
    });
    let f_dc3_no_partial_credit = unary_gap_count == 0
        && accepted_unary_totality_provisional_instance_ids.is_empty()
        && unary_registered.values().all(|registration| {
            registration
                .parametric_internality
                .total_specialization_authoritative
        });
    let f_dc4_opaque_references_sealed_and_whole = unary_registered.values().all(|registration| {
        registration
            .ambient_declaration
            .opaque_references_used_whole
    });
    let f_dc5_same_issuer_scope_preserved = unary_registered.len() + unary_gap_count == 17
        && unary_summaries.len() == 17
        && unary_summaries.iter().all(|row| row.stage == 16);

    let mut membership_rows = Vec::new();
    for (instance_id, registration) in &unary_registered {
        let instance = stage16_window
            .instances
            .iter()
            .find(|row| &row.instance_id == instance_id)
            .ok_or_else(|| {
                E5FutureHoleFinaleV2Error::Invariant(format!(
                    "unary registration {instance_id} is absent from Stage 16"
                ))
            })?;
        let scheme = scheme_for_instance(stage16_window, instance)?;
        let source = instance
            .source_anchor_ids
            .first()
            .and_then(|anchor| {
                stage16_window
                    .typed_sources
                    .iter()
                    .find(|source| &source.anchor_id == anchor)
            })
            .ok_or_else(|| {
                E5FutureHoleFinaleV2Error::Invariant(format!(
                    "unary registration {instance_id} has no exact typed source"
                ))
            })?;
        let registration_summary = unary_summaries
            .iter()
            .find(|row| row.a3_instance_id == *instance_id)
            .ok_or_else(|| {
                E5FutureHoleFinaleV2Error::Invariant(format!(
                    "unary registration {instance_id} has no replay summary"
                ))
            })?;
        let registration_evidence_replayed = registration_summary.gap_free
            && registration_summary.deterministic_replay_valid
            && registration_summary.external_exhaustiveness_join_exact
            && registration_summary.declaration_and_jurisdiction_locality_proved
            && registration_summary.hole_marginal_charge_zero
            && registration.no_reflexivity_fallback
            && registration.every_hole_live
            && registration.local_semantic_scope_premises_satisfied
            && registration
                .external_exhaustiveness_evidence_hash
                .as_deref()
                == Some(stage16_proof.derivation_hash.as_str());
        let exact_source_replayed =
            registration_evidence_replayed && proof_strength_exact_source_replays(registration);
        let exact_body_binding_replayed = registration_evidence_replayed
            && proof_strength_exact_body_binding_replays(registration);
        let stable_specialization_totality_proved = registration_evidence_replayed
            && registration
                .parametric_internality
                .total_specialization_authoritative
            && registration
                .parametric_internality
                .dependent_totality_theorem_replayed;
        let hypothetical_derivation_replayed = exact_source_replayed
            && exact_body_binding_replayed
            && stable_specialization_totality_proved;
        let output_kernel_typed = hypothetical_derivation_replayed
            && registration.explicit_body_elaboration.kernel_ty
                == registration.expected_kernel_type
            && registration
                .ambient_declaration
                .typed_body_elaboration
                .kernel_ty
                == registration.expected_kernel_type;
        let mut row = E5V2MembershipRow {
            a3_instance_id: instance_id.clone(),
            a3_scheme_id: scheme.scheme_id.clone(),
            rule: "unary_action_by_registered_future_hypothesis_v2".to_owned(),
            source_step: Some(source.step),
            source_clause: Some(source.clause_index),
            natural_family_id: Some(registration.natural_family_id.clone()),
            orbit_id: stage16_window
                .orbits
                .iter()
                .find(|orbit| orbit.member_instance_ids.contains(instance_id))
                .map(|orbit| orbit.orbit_id.clone()),
            d_membership_derivation_hash: registration.formation_hash.clone(),
            proof_strength_exact_source_replayed: Some(exact_source_replayed),
            proof_strength_exact_body_binding_replayed: Some(exact_body_binding_replayed),
            stable_specialization_totality_proved: Some(stable_specialization_totality_proved),
            hypothetical_derivation_replayed,
            output_kernel_typed,
            uniform_specialization_not_new_family: true,
            independently_exported_output_orbit: false,
            hole_marginal_charge_zero: Some(registration.hole_marginal_charge.replays_as_zero()),
            derivation_hash: String::new(),
        };
        row.derivation_hash = row_hash(&row);
        membership_rows.push(row);
    }
    for transported in transport
        .a3
        .direct_instances
        .iter()
        .chain(&transport.a3.pointwise_instances)
    {
        let instance = stage16_window
            .instances
            .iter()
            .find(|instance| instance.instance_id == transported.a3_instance_id)
            .ok_or_else(|| {
                E5FutureHoleFinaleV2Error::Invariant(format!(
                    "transported chronological instance {} is absent from Stage 16",
                    transported.a3_instance_id
                ))
            })?;
        let scheme = scheme_for_instance(stage16_window, instance)?;
        if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison
            || scheme.scheme_id != transported.a3_scheme_id
        {
            return Err(E5FutureHoleFinaleV2Error::Invariant(format!(
                "transported instance {} does not join its exact chronological scheme",
                transported.a3_instance_id
            )));
        }
        let hypothetical_derivation_replayed = match transported.mode.as_str() {
            "direct_type" => transported.exact_predecessor_instance_join,
            "pointwise_type" => {
                transport.a3.pointwise_instances_join_existing_j3_families
                    && transported.normalization_equality.equal
                    && !transported.structural_substitution_hash.is_empty()
                    && !transported.typed_image_naturality_hash.is_empty()
                    && !transported.d_membership_derivation_hash.is_empty()
            }
            _ => false,
        };
        let mut row = E5V2MembershipRow {
            a3_instance_id: transported.a3_instance_id.clone(),
            a3_scheme_id: transported.a3_scheme_id.clone(),
            rule: format!("chronological_comparison::{}", transported.mode),
            source_step: Some(transported.newest_step),
            source_clause: Some(transported.newest_clause),
            natural_family_id: Some(transported.family_id.clone()),
            orbit_id: Some(transported.orbit_id.clone()),
            d_membership_derivation_hash: transported.d_membership_derivation_hash.clone(),
            proof_strength_exact_source_replayed: None,
            proof_strength_exact_body_binding_replayed: None,
            stable_specialization_totality_proved: None,
            hypothetical_derivation_replayed,
            output_kernel_typed: transported.normalization_equality.equal
                && !transported.required_output_kernel_type_json.is_empty(),
            uniform_specialization_not_new_family: transported
                .uniform_specialization_not_new_family,
            independently_exported_output_orbit: transported.independently_exported_output_orbit,
            hole_marginal_charge_zero: None,
            derivation_hash: String::new(),
        };
        row.derivation_hash = row_hash(&row);
        membership_rows.push(row);
    }
    membership_rows.sort_by(|left, right| left.a3_instance_id.cmp(&right.a3_instance_id));

    let unary_expected_ids = stage16_window
        .instances
        .iter()
        .filter_map(|instance| {
            scheme_for_instance(stage16_window, instance)
                .ok()
                .filter(|scheme| scheme.rule_constructor == A3RuleConstructor::UnaryAction)
                .map(|_| instance.instance_id.clone())
        })
        .collect::<BTreeSet<_>>();
    let unary_registered_ids = unary_registered.keys().cloned().collect::<BTreeSet<_>>();
    let direct_ids = transport
        .a3
        .direct_instances
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let pointwise_ids = transport
        .a3
        .pointwise_instances
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let three_membership_parts_pairwise_disjoint = unary_registered_ids.is_disjoint(&direct_ids)
        && unary_registered_ids.is_disjoint(&pointwise_ids)
        && direct_ids.is_disjoint(&pointwise_ids);
    let membership_ids = membership_rows
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let exact_inventory_union_bijection = three_membership_parts_pairwise_disjoint
        && membership_rows.len() == membership_ids.len()
        && membership_ids == stage16_operational_ids;

    let mut higher_instance_count = 0_usize;
    let mut structural_instance_count = 0_usize;
    for instance in &stage16_window.instances {
        match scheme_for_instance(stage16_window, instance)?.rule_constructor {
            A3RuleConstructor::HigherOpenBoxReduction => higher_instance_count += 1,
            A3RuleConstructor::StructuralCompletionHole => structural_instance_count += 1,
            _ => {}
        }
    }
    let higher_seed_rejected_for_no_typed_path_witness = stage16_window
        .seed_dispositions
        .iter()
        .find(|row| row.rule_constructor == A3RuleConstructor::HigherOpenBoxReduction)
        .is_some_and(|row| {
            row.promoted_seed_count == 0
                && row.generated_typed_instance_count == 0
                && row.rejection_reason == Some(A3SeedRejectionReason::NoTypedPathWitness)
        });
    let derivable_ids = membership_rows
        .iter()
        .filter(|row| {
            row.hypothetical_derivation_replayed
                && row.output_kernel_typed
                && row.uniform_specialization_not_new_family
                && !row.independently_exported_output_orbit
                && row.hole_marginal_charge_zero != Some(false)
        })
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let underdetermined_ids = stage16_operational_ids
        .difference(&derivable_ids)
        .cloned()
        .collect::<Vec<_>>();
    let every_instance_d_decided = exact_inventory_union_bijection;
    let d_partition_complete = adopted_and_frozen_relative_a3_coverage_complete
        && every_instance_d_decided
        && every_unary_registration_prefix_local
        && every_unary_registration_evidence_replayed
        && transport
            .a3
            .chronological_outputs_constructed_and_kernel_typed
        && membership_rows.iter().all(|row| {
            !row.d_membership_derivation_hash.is_empty() && row.derivation_hash == row_hash(row)
        });
    let semantic_o16_empty =
        (adopted_and_frozen_relative_a3_coverage_complete && d_partition_complete).then_some(
            underdetermined_ids.is_empty()
                && structural_instance_count == 0
                && stage16_window.constructor_evidence.is_empty()
                && stage16_proof.required_packages_from_raw_debt.is_empty(),
        );
    let mut stage16 = E5V2Stage16Audit {
        exact_a3_inventory_count: stage16_operational_ids.len(),
        exhaustiveness_promoted_count: stage16_promoted_ids.len(),
        unary_registration_count: unary_registered.len(),
        unary_gap_count,
        unary_named_gaps,
        accepted_unary_exact_source_replay_count,
        accepted_unary_exact_body_binding_count,
        accepted_unary_totality_provisional_instance_ids,
        stable_source_guard_audit_replayed,
        universal_unary_specialization_totality_proved,
        dependent_context_registration_count,
        published_dependent_redeclaration_count,
        total_specialization_theorem_replay_count,
        f_dc1_stage16_rows_rederived,
        f_dc2_no_outcome_filtering,
        f_dc3_no_partial_credit,
        f_dc4_opaque_references_sealed_and_whole,
        f_dc5_same_issuer_scope_preserved,
        direct_chronological_instance_count: transport.a3.direct_instances.len(),
        pointwise_chronological_instance_count: transport.a3.pointwise_instances.len(),
        higher_instance_count,
        structural_instance_count,
        higher_seed_rejected_for_no_typed_path_witness,
        chronological_transport_bijection: transport.a3.chronological_instance_transport_bijection,
        direct_regression_64_to_8: transport.a3.j3_regression_64_to_8,
        pointwise_joined_to_same_eight_families: transport
            .a3
            .pointwise_instances_join_existing_j3_families,
        unary_ids_join_exactly: unary_registered_ids == unary_expected_ids,
        every_unary_registration_prefix_local,
        every_unary_registration_evidence_replayed,
        three_membership_parts_pairwise_disjoint,
        exact_inventory_union_bijection,
        registrations: unary_summaries,
        membership_rows,
        derivable_instance_ids: derivable_ids.iter().cloned().collect(),
        underdetermined_instance_ids: underdetermined_ids,
        every_instance_d_decided,
        d_partition_complete,
        semantic_o16_empty,
        derivation_hash: String::new(),
    };
    stage16.derivation_hash = tagged_hash("Stage-16-full-A3-D-audit-v2", &stage16);

    let archived_history: Phase5bHistoryCertificate =
        serde_json::from_slice(PHASE5B_ARTIFACT_BYTES)
            .map_err(|error| E5FutureHoleFinaleV2Error::Json(error.to_string()))?;
    let phase5b_archive_digest_valid = archived_history.schema == PHASE5B_HISTORY_CERT_SCHEMA
        && archived_history.result_digest == phase5b_archive_digest(&archived_history);
    let phase5b_live_replay = replay_phase5b_history_certificate(&archived_history);
    let all_fifteen_fillers_certified = phase5b_archive_digest_valid
        && archived_history.all_fifteen_steps_certified
        && archived_history.ledger.every_entry_certified
        && archived_history.steps.len() == 15
        && archived_history
            .steps
            .iter()
            .map(|step| step.step)
            .eq(1..=15)
        && archived_history.steps.iter().all(|step| {
            let filler = Telescope::reference(step.step);
            step.certified
                && step.every_counted_family_provenanced
                && step.candidate_hash
                    == future_v2::issue_filler_ordinary_charge_provenance_v2(
                        step.step,
                        &filler,
                        step.kappa,
                        step.certified_semantic_total,
                        filler.bit_cost(),
                        step.derivation_hash.clone(),
                        ordinary_family_token_hashes(step),
                    )
                    .filler_candidate_hash
                && step.kappa == filler.kappa() as u32
                && step.newly_issued_units.iter().all(|unit| {
                    unit.anchor_valid
                        && unit.family_marginal
                        && unit.family_or_instance_decided
                        && !unit.uniform_specialization_multiplied_without_exported_orbit
                        && !unit.historical_count_used_as_input
                        && !unit.acceptance_bar_used_as_input
                        && !unit.derivation_hash.is_empty()
                })
        });
    if !all_fifteen_fillers_certified || !archived_history.all_amplification_provenanced {
        return Err(E5FutureHoleFinaleV2Error::Prerequisite(
            "the frozen Phase 5b artifact does not provide an internally intact ordinary-charge row for every sealed filler"
                .to_owned(),
        ));
    }

    let mut realization_summaries = Vec::<E5V2RealizationSummary>::new();
    let mut structural_realization_gap_count = 0_usize;
    let mut correct_provider_join_count = 0_usize;
    let mut wrong_provider_trial_count = 0_usize;
    let mut wrong_provider_rejection_count = 0_usize;
    let mut charge_swap_trial_count = 0_usize;
    let mut charge_swap_rejection_count = 0_usize;
    let mut every_charge_swap_self_replays = true;
    let mut no_hole_or_discharge_minted_filler_credit = true;
    let mut consumed_provider_steps = BTreeSet::<u32>::new();

    for ((registration_stage, instance_id), registration) in &structural_registered {
        let contract = match &registration.output_contract {
            future_v2::FutureHoleOutputContractV2::StructuralProvides(contract) => contract,
            future_v2::FutureHoleOutputContractV2::UnaryActionAt { .. } => {
                return Err(E5FutureHoleFinaleV2Error::Invariant(format!(
                    "structural registration {instance_id} carries a unary contract"
                )));
            }
        };
        let filler_step = contract.jurisdiction.jurisdiction_stage();
        let filler = Telescope::reference(filler_step);
        let history_step = phase5b_step(&archived_history, filler_step)?;
        let charge = issue_phase5b_filler_charge(history_step, &filler);
        let correct_join = filler_charge_joins_phase5b_exactly(&charge, history_step, &filler);
        if correct_join {
            correct_provider_join_count += 1;
        }
        consumed_provider_steps.insert(filler_step);

        // A wrong sealed entry is supplied at the correct jurisdiction.
        // The v2 typed provider relation must reject it before any clause-4'
        // realization can be issued.
        // Use an already sealed predecessor at the same jurisdiction.  It is
        // well scoped in the prefix, so rejection reaches the semantic
        // provider check instead of failing earlier on future references.
        let wrong_step = filler_step - 1;
        let wrong_filler = Telescope::reference(wrong_step);
        let wrong_history_step = phase5b_step(&archived_history, wrong_step)?;
        let wrong_subject_charge = future_v2::issue_filler_ordinary_charge_provenance_v2(
            filler_step,
            &wrong_filler,
            wrong_filler.kappa() as u32,
            wrong_history_step.certified_semantic_total,
            wrong_filler.bit_cost(),
            wrong_history_step.derivation_hash.clone(),
            ordinary_family_token_hashes(wrong_history_step),
        );
        let wrong_provider = future_v2::realize_structural_future_hole_v2(
            &exact_prefix(filler_step),
            registration,
            filler_step,
            &wrong_filler,
            &wrong_subject_charge,
        )
        .map_err(|error| E5FutureHoleFinaleV2Error::Prerequisite(error.to_string()))?;
        wrong_provider_trial_count += 1;
        if matches!(
            &wrong_provider,
            future_v2::FutureHoleRealizationDispositionV2::Gap(gap)
                if gap.id == "a3_v2_wrong_structural_provider" && gap.replays()
        ) {
            wrong_provider_rejection_count += 1;
        }

        // This token is internally self-consistent but transfers the
        // certified charge fields from a different Phase 5b row onto the
        // correct filler subject.  Only the authoritative history join can
        // reject that swap; self-hashing alone deliberately cannot.
        let swapped_charge = future_v2::issue_filler_ordinary_charge_provenance_v2(
            filler_step,
            &filler,
            wrong_history_step.kappa,
            wrong_history_step.certified_semantic_total,
            wrong_filler.bit_cost(),
            wrong_history_step.derivation_hash.clone(),
            ordinary_family_token_hashes(wrong_history_step),
        );
        charge_swap_trial_count += 1;
        every_charge_swap_self_replays &=
            future_v2::replay_filler_ordinary_charge_provenance_v2(&swapped_charge);
        if !filler_charge_joins_phase5b_exactly(&swapped_charge, history_step, &filler) {
            charge_swap_rejection_count += 1;
        }

        let disposition = if correct_join {
            future_v2::realize_structural_future_hole_v2(
                &exact_prefix(filler_step),
                registration,
                filler_step,
                &filler,
                &charge,
            )
            .map_err(|error| E5FutureHoleFinaleV2Error::Prerequisite(error.to_string()))?
        } else {
            return Err(E5FutureHoleFinaleV2Error::Invariant(format!(
                "Stage {registration_stage} structural hole did not join the exact Phase 5b filler charge"
            )));
        };
        let replay = future_v2::replay_structural_realization_v2(
            &exact_prefix(filler_step),
            registration,
            filler_step,
            &filler,
            &charge,
            &disposition,
        );
        let charge_summary = filler_charge_summary(&charge, history_step, &filler);
        let (
            realization_hash,
            filler_candidate_hash,
            filler_is_whole_sealed_entry,
            provider_relation_satisfied,
            constructor_live_before_filler,
            constructor_absent_after_filler,
            specialized_expression_is_filler_reference,
            clause4_prime_instantiation_replayed,
            discharge_marginal_charge_zero,
            ordinary_charge_preserved_not_zeroed_or_reminted,
            gap_free,
        ) = match &disposition {
            future_v2::FutureHoleRealizationDispositionV2::Realized(value) => {
                let discharge_zero = value.discharge_marginal_charge.replays_as_zero();
                let preserved = value.filler_ordinary_charge == charge
                    && filler_charge_joins_phase5b_exactly(
                        &value.filler_ordinary_charge,
                        history_step,
                        &filler,
                    )
                    && discharge_zero;
                no_hole_or_discharge_minted_filler_credit &=
                    registration.hole_marginal_charge.replays_as_zero()
                        && discharge_zero
                        && preserved;
                (
                    value.realization_hash.clone(),
                    value.filler_candidate_hash.clone(),
                    value.filler_is_whole_sealed_entry,
                    value.provider_relation_satisfied,
                    value.constructor_live_before_filler,
                    value.constructor_absent_after_filler,
                    value.specialized_expression_is_filler_reference,
                    value.clause4_prime_instantiation_replayed,
                    discharge_zero,
                    preserved,
                    true,
                )
            }
            future_v2::FutureHoleRealizationDispositionV2::Gap(gap) => {
                structural_realization_gap_count += 1;
                no_hole_or_discharge_minted_filler_credit = false;
                (
                    gap.gap_hash.clone(),
                    charge.filler_candidate_hash.clone(),
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                )
            }
        };
        let scheme_id = registration.a3_scheme_id.clone();
        let mut summary = E5V2RealizationSummary {
            registration_stage: *registration_stage,
            jurisdiction_stage: filler_step,
            a3_instance_id: instance_id.clone(),
            a3_scheme_id: scheme_id,
            occurrence_id: registration.occurrence_id.clone(),
            structural_constructor: contract.constructor.slug().to_owned(),
            provider_family: contract.provider_family.slug().to_owned(),
            registration_formation_hash: registration.formation_hash.clone(),
            filler_step,
            filler_candidate_hash,
            filler_is_whole_sealed_entry,
            provider_relation_satisfied,
            constructor_live_before_filler,
            constructor_absent_after_filler,
            specialized_expression_is_filler_reference,
            clause4_prime_instantiation_replayed,
            ordinary_filler_charge: charge_summary,
            discharge_marginal_charge_zero,
            ordinary_charge_preserved_not_zeroed_or_reminted,
            realization_hash,
            deterministic_replay_valid: replay.valid,
            gap_free,
            derivation_hash: String::new(),
        };
        summary.derivation_hash = realization_summary_hash(&summary);
        realization_summaries.push(summary);
    }
    realization_summaries.sort_by(|left, right| {
        (left.registration_stage, left.a3_instance_id.as_str())
            .cmp(&(right.registration_stage, right.a3_instance_id.as_str()))
    });

    let every_wrong_provider_rejected = wrong_provider_trial_count == 13
        && wrong_provider_rejection_count == wrong_provider_trial_count;
    let every_charge_swap_rejected =
        charge_swap_trial_count == 13 && charge_swap_rejection_count == charge_swap_trial_count;
    let provider_control_hash = tagged_hash(
        "provider-negative-controls",
        &(
            &consumed_provider_steps,
            correct_provider_join_count,
            wrong_provider_trial_count,
            wrong_provider_rejection_count,
            charge_swap_trial_count,
            charge_swap_rejection_count,
            every_charge_swap_self_replays,
            no_hole_or_discharge_minted_filler_credit,
        ),
    );
    let provider_controls = E5V2ProviderControlAudit {
        authoritative_phase5b_provider_count: consumed_provider_steps.len(),
        correct_provider_join_count,
        wrong_provider_trial_count,
        wrong_provider_rejection_count,
        charge_swap_trial_count,
        charge_swap_rejection_count,
        self_hashed_charge_swap_would_replay_without_authoritative_join:
            every_charge_swap_self_replays,
        authoritative_history_join_mandatory: true,
        every_wrong_provider_rejected,
        every_charge_swap_rejected,
        no_hole_or_discharge_minted_filler_credit,
        derivation_hash: provider_control_hash,
    };

    let stage3_registration = structural_registered
        .iter()
        .find(|((stage, _), _)| *stage == 3)
        .map(|(_, value)| value)
        .ok_or_else(|| {
            E5FutureHoleFinaleV2Error::Invariant(
                "Stage-3 former-eliminator registration is absent".to_owned(),
            )
        })?;
    let stage4_registration = structural_registered
        .iter()
        .find(|((stage, _), _)| *stage == 4)
        .map(|(_, value)| value)
        .ok_or_else(|| {
            E5FutureHoleFinaleV2Error::Invariant(
                "Stage-4 former-eliminator registration is absent".to_owned(),
            )
        })?;
    let stage3_realization = realization_summaries
        .iter()
        .find(|row| row.registration_stage == 3)
        .ok_or_else(|| {
            E5FutureHoleFinaleV2Error::Invariant("Stage-3 realization is absent".to_owned())
        })?;
    let stage4_realization = realization_summaries
        .iter()
        .find(|row| row.registration_stage == 4)
        .ok_or_else(|| {
            E5FutureHoleFinaleV2Error::Invariant("Stage-4 realization is absent".to_owned())
        })?;
    let stage3_contract = match &stage3_registration.output_contract {
        future_v2::FutureHoleOutputContractV2::StructuralProvides(contract) => contract,
        _ => unreachable!("Stage 3 structural map contains only structural registrations"),
    };
    let occurrence_ids_intentionally_distinct =
        stage3_registration.occurrence_id != stage4_registration.occurrence_id;
    let same_natural_family =
        stage3_registration.natural_family_id == stage4_registration.natural_family_id;
    let same_structural_constructor = stage3_realization.structural_constructor
        == stage4_realization.structural_constructor
        && stage3_realization.structural_constructor == "former_eliminator";
    let stage3_jurisdiction_transport_present = matches!(
        &stage3_contract.jurisdiction,
        future_v2::StructuralJurisdictionV2::Stage3To4 {
            registration_stage: 3,
            jurisdiction_stage: 4,
            transport_hash,
            ..
        } if !transport_hash.is_empty()
    );
    let both_jurisdiction_stage_four =
        stage3_realization.jurisdiction_stage == 4 && stage4_realization.jurisdiction_stage == 4;
    let same_filler_step_four =
        stage3_realization.filler_step == 4 && stage4_realization.filler_step == 4;
    let same_phase5b_provider = stage3_realization.filler_candidate_hash
        == stage4_realization.filler_candidate_hash
        && stage3_realization
            .ordinary_filler_charge
            .phase5b_step_derivation_hash
            == stage4_realization
                .ordinary_filler_charge
                .phase5b_step_derivation_hash;
    let both_clause4_prime_instantiations_replayed = stage3_realization
        .clause4_prime_instantiation_replayed
        && stage4_realization.clause4_prime_instantiation_replayed;
    let semantic_scheme_transport_proved = occurrence_ids_intentionally_distinct
        && same_natural_family
        && same_structural_constructor
        && stage3_jurisdiction_transport_present
        && stage3_declaration_jurisdiction_separation_proved
        && both_jurisdiction_stage_four
        && same_filler_step_four
        && same_phase5b_provider
        && both_clause4_prime_instantiations_replayed;
    let stage3_transport_hash = tagged_hash(
        "Stage-3-to-Stage-4-future-scheme-transport-v2",
        &(
            &stage3_registration.a3_instance_id,
            &stage4_registration.a3_instance_id,
            occurrence_ids_intentionally_distinct,
            same_natural_family,
            same_structural_constructor,
            stage3_jurisdiction_transport_present,
            stage3_declaration_jurisdiction_separation_proved,
            both_jurisdiction_stage_four,
            same_filler_step_four,
            same_phase5b_provider,
            both_clause4_prime_instantiations_replayed,
            semantic_scheme_transport_proved,
        ),
    );
    let stage3_to_stage4_transport = E5V2Stage3To4Transport {
        stage3_a3_instance_id: stage3_registration.a3_instance_id.clone(),
        stage4_a3_instance_id: stage4_registration.a3_instance_id.clone(),
        occurrence_ids_intentionally_distinct,
        same_natural_family,
        same_structural_constructor,
        stage3_jurisdiction_transport_present,
        declaration_jurisdiction_prefix_separation_proved:
            stage3_declaration_jurisdiction_separation_proved,
        both_jurisdiction_stage_four,
        same_filler_step_four,
        same_phase5b_provider,
        both_clause4_prime_instantiations_replayed,
        semantic_scheme_transport_proved,
        derivation_hash: stage3_transport_hash,
    };

    let timeline = directive_debt_timeline();
    let mut focus_rows = Vec::new();
    for coarse in &timeline {
        let window = windows
            .get(&coarse.stage)
            .expect("every coarse stage has an exact A3 window");
        let proof = proof_for_stage(&archived_a3, coarse.stage)?;
        let coarse_required_packages = coarse
            .required_packages
            .iter()
            .map(|value| (*value).to_owned())
            .collect::<Vec<_>>();
        let exact_a3_required_packages = proof.required_packages_from_raw_debt.clone();
        let a3_constructor_evidence = window
            .constructor_evidence
            .iter()
            .map(|evidence| evidence.constructor.slug().to_owned())
            .collect::<Vec<_>>();
        let registered_structural_constructors = structural_summaries
            .iter()
            .filter(|row| row.stage == coarse.stage && row.gap_free)
            .filter_map(|row| row.structural_constructor.clone())
            .collect::<Vec<_>>();
        let realized_structural_constructors = realization_summaries
            .iter()
            .filter(|row| row.registration_stage == coarse.stage && row.gap_free)
            .map(|row| row.structural_constructor.clone())
            .collect::<Vec<_>>();
        let projected_focus = window
            .focus_projection
            .projected_focus
            .map(|constructor| constructor.slug().to_owned());
        let stage_three_wrinkle_replayed = coarse.stage == 3
            && coarse_required_packages == ["former_eliminator"]
            && exact_a3_required_packages == ["former_eliminator"]
            && a3_constructor_evidence == ["former_eliminator"]
            && projected_focus.is_none()
            && window.focus_projection.demand_precedes_jurisdiction
            && registered_structural_constructors == ["former_eliminator"]
            && realized_structural_constructors == ["former_eliminator"]
            && semantic_scheme_transport_proved;
        let jurisdiction_projection_matches = if coarse.stage == 3 {
            stage_three_wrinkle_replayed
        } else if coarse_required_packages.is_empty() {
            projected_focus.is_none()
        } else {
            projected_focus.as_ref() == coarse_required_packages.first()
        };
        let focus_projection_reproduces_coarse_record = coarse_required_packages
            == exact_a3_required_packages
            && exact_a3_required_packages == a3_constructor_evidence
            && registered_structural_constructors == a3_constructor_evidence
            && realized_structural_constructors == a3_constructor_evidence
            && jurisdiction_projection_matches;
        let every_standing_hole_realized_by_specialization = realized_structural_constructors
            == a3_constructor_evidence
            && realization_summaries
                .iter()
                .filter(|row| row.registration_stage == coarse.stage)
                .all(|row| {
                    row.gap_free
                        && row.deterministic_replay_valid
                        && row.clause4_prime_instantiation_replayed
                        && row.ordinary_charge_preserved_not_zeroed_or_reminted
                });
        let derivation_hash = tagged_hash(
            "historical-focus-row-v2",
            &(
                coarse.stage,
                &coarse_required_packages,
                &exact_a3_required_packages,
                &a3_constructor_evidence,
                &registered_structural_constructors,
                &realized_structural_constructors,
                &projected_focus,
                window.focus_projection.demand_precedes_jurisdiction,
                stage_three_wrinkle_replayed,
                focus_projection_reproduces_coarse_record,
                every_standing_hole_realized_by_specialization,
            ),
        );
        focus_rows.push(E5V2HistoricalFocusRow {
            stage: coarse.stage,
            coarse_required_packages,
            exact_a3_required_packages,
            a3_constructor_evidence,
            registered_structural_constructors,
            realized_structural_constructors,
            projected_focus,
            demand_precedes_jurisdiction: window.focus_projection.demand_precedes_jurisdiction,
            stage_three_wrinkle_replayed,
            focus_projection_reproduces_coarse_record,
            every_standing_hole_realized_by_specialization,
            derivation_hash,
        });
    }

    let structural_summary_ids = structural_summaries
        .iter()
        .map(|row| (row.stage, row.a3_instance_id.clone()))
        .collect::<BTreeSet<_>>();
    let exact_structural_a3_registration_bijection = structural_summaries.len() == 13
        && structural_registered.len() == 13
        && structural_gap_count == 0
        && structural_summary_ids == attempted_structural_ids;
    let f_fh1_every_historical_discharge_rederived = exact_structural_a3_registration_bijection
        && every_structural_registration_prefix_local
        && every_structural_registration_evidence_replayed
        && stage3_declaration_jurisdiction_separation_proved
        && realization_summaries.len() == 13
        && structural_realization_gap_count == 0
        && realization_summaries.iter().all(|row| {
            row.gap_free
                && row.deterministic_replay_valid
                && row.clause4_prime_instantiation_replayed
                && row.ordinary_charge_preserved_not_zeroed_or_reminted
        })
        && provider_controls.every_wrong_provider_rejected
        && provider_controls.every_charge_swap_rejected
        && stage3_to_stage4_transport.semantic_scheme_transport_proved;
    let focus_projection_reproduces_entire_coarse_ladder = focus_rows
        .iter()
        .all(|row| row.focus_projection_reproduces_coarse_record);
    let stage_three_wrinkle_reproduced = focus_rows
        .iter()
        .find(|row| row.stage == 3)
        .is_some_and(|row| row.stage_three_wrinkle_replayed);
    let coarse_o16_empty = focus_rows
        .iter()
        .find(|row| row.stage == 16)
        .is_some_and(|row| row.coarse_required_packages.is_empty());
    let semantic_o16_agrees_with_coarse_projection = stage16
        .semantic_o16_empty
        .map(|semantic| semantic && coarse_o16_empty);
    let historical_hash = tagged_hash(
        "historical-ledger-v2",
        &(
            (
                &archived_history.result_digest,
                phase5b_archive_digest_valid,
                phase5b_live_replay.valid,
                &structural_summaries,
                &realization_summaries,
                &provider_controls,
                &stage3_to_stage4_transport,
                &focus_rows,
            ),
            (
                exact_structural_a3_registration_bijection,
                every_structural_registration_prefix_local,
                every_structural_registration_evidence_replayed,
                stage3_declaration_jurisdiction_separation_proved,
                f_fh1_every_historical_discharge_rederived,
                focus_projection_reproduces_entire_coarse_ladder,
                stage_three_wrinkle_reproduced,
                coarse_o16_empty,
                semantic_o16_agrees_with_coarse_projection,
            ),
        ),
    );
    let historical = E5V2HistoricalLedgerAudit {
        phase5b_certificate_digest: archived_history.result_digest.clone(),
        phase5b_archive_digest_valid,
        phase5b_live_replay_valid: phase5b_live_replay.valid,
        phase5b_live_replay_errors: phase5b_live_replay.errors.clone(),
        frozen_phase5b_surface_drift_bound_explicitly: !phase5b_live_replay.valid,
        historical_artifact_reinterpreted_after_drift: false,
        all_fifteen_fillers_certified,
        all_amplification_provenanced: archived_history.all_amplification_provenanced,
        structural_registration_count: structural_registered.len(),
        structural_registration_gap_count: structural_gap_count,
        structural_realization_count: realization_summaries
            .iter()
            .filter(|row| row.gap_free)
            .count(),
        structural_realization_gap_count,
        every_structural_registration_prefix_local,
        every_structural_registration_evidence_replayed,
        stage3_declaration_jurisdiction_separation_proved,
        registrations: structural_summaries,
        realizations: realization_summaries,
        provider_controls,
        stage3_to_stage4_transport,
        focus_rows,
        exact_structural_a3_registration_bijection,
        f_fh1_every_historical_discharge_rederived,
        focus_projection_reproduces_entire_coarse_ladder,
        stage_three_wrinkle_reproduced,
        coarse_o16_empty,
        semantic_o16_agrees_with_coarse_projection,
        derivation_hash: historical_hash,
    };

    let historical_realization_complete = historical.f_fh1_every_historical_discharge_rederived
        && historical.focus_projection_reproduces_entire_coarse_ladder
        && historical.stage_three_wrinkle_reproduced;
    let f1_preconditions_complete = a3_coverage.adopted_and_frozen_relative_a3_coverage_complete
        && stage16.d_partition_complete
        && historical_realization_complete;
    let f1_executed = f1_preconditions_complete;
    let demanded_but_underdetermined_instance = f1_executed
        .then(|| stage16.underdetermined_instance_ids.first().cloned())
        .flatten();
    let f1_triggered = f1_executed.then_some(demanded_but_underdetermined_instance.is_some());
    let f1_excluded = f1_executed.then_some(demanded_but_underdetermined_instance.is_none());
    let theorem12_full_instance_granularity_proved = f1_executed.then_some(
        f1_excluded == Some(true)
            && stage16.semantic_o16_empty == Some(true)
            && historical.semantic_o16_agrees_with_coarse_projection == Some(true),
    );
    let theorem12_refuted = f1_triggered;
    let f1_disposition = if f1_triggered == Some(true) {
        "F1_TRIGGERED_DEMANDED_BUT_UNDERDETERMINED_INSTANCE"
    } else if f1_excluded == Some(true) {
        "F1_EXCLUDED_FULL_A3_SEMANTIC_O16_EMPTY"
    } else {
        "F1_NOT_EXECUTED_PRECONDITIONS_INCOMPLETE"
    };
    let f1_hash = tagged_hash(
        "F1-final-disposition-v2",
        &(
            a3_coverage.adopted_and_frozen_relative_a3_coverage_complete,
            stage16.d_partition_complete,
            historical_realization_complete,
            f1_preconditions_complete,
            f1_executed,
            stage16.exact_a3_inventory_count,
            stage16.derivable_instance_ids.len(),
            &demanded_but_underdetermined_instance,
            f1_triggered,
            f1_excluded,
            theorem12_full_instance_granularity_proved,
            theorem12_refuted,
            stage16.semantic_o16_empty,
        ),
    );
    let f1 = E5V2F1Audit {
        adopted_and_frozen_a3_coverage_complete: a3_coverage
            .adopted_and_frozen_relative_a3_coverage_complete,
        d_partition_complete: stage16.d_partition_complete,
        historical_realization_complete,
        f1_preconditions_complete,
        f1_executed,
        demanded_instance_count: stage16.exact_a3_inventory_count,
        derivable_instance_count: stage16.derivable_instance_ids.len(),
        demanded_but_underdetermined_instance,
        f1_triggered,
        f1_excluded,
        theorem12_full_instance_granularity_proved,
        theorem12_refuted,
        semantic_o16_empty: stage16.semantic_o16_empty,
        disposition: f1_disposition.to_owned(),
        derivation_hash: f1_hash,
    };

    let future_v2_evidence_digest = tagged_hash(
        "joined-future-v2-evidence",
        &(
            stage16
                .registrations
                .iter()
                .map(|row| row.formation_hash.as_str())
                .collect::<Vec<_>>(),
            historical
                .registrations
                .iter()
                .map(|row| row.formation_hash.as_str())
                .collect::<Vec<_>>(),
            historical
                .realizations
                .iter()
                .map(|row| row.realization_hash.as_str())
                .collect::<Vec<_>>(),
        ),
    );
    let f_fh4_triggered = !stage16.unary_named_gaps.is_empty()
        || !stage16.universal_unary_specialization_totality_proved
        || historical.structural_registration_gap_count != 0
        || historical.structural_realization_gap_count != 0;
    let mut blocking_obligations = stage16
        .unary_named_gaps
        .iter()
        .map(|gap| {
            format!(
                "F-FH4:{}:{}:{}",
                gap.id, gap.a3_instance_id, gap.exact_error
            )
        })
        .collect::<Vec<_>>();
    if !stage16.universal_unary_specialization_totality_proved {
        blocking_obligations.push(
            "F-FH4:unary_specialization_totality_unproved:release and replay a versioned stable-source guard over every accepted unary registration; exact source replay alone is insufficient"
                .to_owned(),
        );
    }
    if historical.structural_registration_gap_count != 0
        || historical.structural_realization_gap_count != 0
    {
        blocking_obligations
            .push("F-FH4:historical_structural_registration_or_realization_gap".to_owned());
    }
    let e5_complete = !f_fh4_triggered
        && f1.f1_executed
        && stage16.unary_gap_count == 0
        && stage16.f_dc1_stage16_rows_rederived
        && stage16.f_dc2_no_outcome_filtering
        && stage16.f_dc3_no_partial_credit
        && stage16.f_dc4_opaque_references_sealed_and_whole
        && stage16.f_dc5_same_issuer_scope_preserved
        && stage16.every_unary_registration_prefix_local
        && stage16.every_unary_registration_evidence_replayed
        && historical.structural_registration_gap_count == 0
        && historical.every_structural_registration_prefix_local
        && historical.every_structural_registration_evidence_replayed
        && historical.stage3_declaration_jurisdiction_separation_proved
        && historical.structural_realization_gap_count == 0;
    let semantic_o16_certificate_issued = e5_complete && f1.semantic_o16_empty == Some(true);
    let t_bf2_authorized =
        e5_complete && f1.theorem12_full_instance_granularity_proved == Some(true);
    let e5_side_of_bridge_satisfied = t_bf2_authorized;
    let outcome = if f_fh4_triggered {
        "E5_V2_BLOCKED_F_FH4_UNARY_SPECIALIZATION_TOTALITY"
    } else if f1.f1_triggered == Some(true) {
        "E5_V2_COMPLETE_F1_TRIGGERED_THEOREM12_REFUTED"
    } else if f1.f1_excluded == Some(true) {
        "E5_V2_COMPLETE_F1_EXCLUDED_SEMANTIC_O16_EMPTY"
    } else {
        "E5_V2_INCOMPLETE_F1_NOT_EXECUTED"
    };
    let permitted_conclusion = if f_fh4_triggered {
        "At least one Stage-16 unary row failed dependent-context registration or its universal total-specialization theorem. F-FH4 therefore fires with the exact named gap or theorem failure. No partial D-membership, semantic O(16), F1 disposition, Theorem 12 result, T-BF2 authorization, bridge authorization, or halt claim is issued."
    } else if f1.f1_triggered == Some(true) {
        "The adopted/frozen full-A3 audit found a demanded-but-underdetermined Stage-16 instance. F1 fires and Theorem 12 is refuted at instance granularity; T-BF2 and the bridge remain closed."
    } else if f1.f1_excluded == Some(true) {
        "Every Stage-16 A3 instance is classified derivable after exact future-hole registration and history-provenanced realization. Semantic O(16)=empty and F1 is excluded. This authorizes T-BF2 and satisfies only the E-5 side of the bridge."
    } else {
        "Adopted/frozen A3 coverage, the total D partition, or historical realization is incomplete. Semantic O(16), F1, T-BF2, and the bridge remain unissued."
    };
    let required_successor_action = if f_fh4_triggered {
        "Publish the exact dependent-context registration or total-specialization obstruction under F-DC1/F-DC3 and keep E-5, T-BF2, and the bridge closed."
    } else if t_bf2_authorized {
        "Run T-BF2 and consume the independently create-new R-T2 confluence result; keep the bridge false until both replay."
    } else {
        "Publish the exact E-5 v2 obstruction and keep T-BF2 and the bridge closed."
    };
    let mut certificate = E5FutureHoleFinaleV2Certificate {
        schema: E5_FUTURE_HOLE_FINALE_V2_SCHEMA.to_owned(),
        date: E5_FUTURE_HOLE_FINALE_V2_DATE.to_owned(),
        source_bindings: source_bindings(),
        desired_count_bar_winner_halt_or_f1_verdict_used_as_definition_input: false,
        future_v2_evidence_digest,
        a3_coverage,
        stage16,
        historical,
        f1,
        f_fh4_triggered,
        blocking_obligations,
        e5_complete,
        semantic_o16_certificate_issued,
        t_bf2_authorized,
        e5_side_of_bridge_satisfied,
        r_t2_confluence_consumed: false,
        bridge_authorized: false,
        bridge_executed: false,
        halt_claim_issued: false,
        outcome: outcome.to_owned(),
        permitted_conclusion: permitted_conclusion.to_owned(),
        required_successor_action: required_successor_action.to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> E5FutureHoleFinaleV2Replay {
    E5FutureHoleFinaleV2Replay {
        valid: false,
        stage16_membership_count: 0,
        stage16_underdetermined_count: 0,
        unary_registration_count: 0,
        structural_registration_count: 0,
        structural_realization_count: 0,
        semantic_o16_empty: None,
        f1_executed: false,
        f1_excluded: None,
        f_fh4_triggered: false,
        e5_complete: false,
        t_bf2_authorized: false,
        bridge_authorized: false,
        outcome: "REPLAY_FAILED".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_e5_future_hole_finale_v2_certificate(
    certificate: &E5FutureHoleFinaleV2Certificate,
) -> E5FutureHoleFinaleV2Replay {
    let expected = match issue_e5_future_hole_finale_v2_certificate() {
        Ok(value) => value,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != &expected {
        errors.push("certificate differs from independent E-5 v2 reissuance".to_owned());
    }
    E5FutureHoleFinaleV2Replay {
        valid: errors.is_empty(),
        stage16_membership_count: certificate.stage16.membership_rows.len(),
        stage16_underdetermined_count: certificate.stage16.underdetermined_instance_ids.len(),
        unary_registration_count: certificate.stage16.unary_registration_count,
        structural_registration_count: certificate.historical.structural_registration_count,
        structural_realization_count: certificate.historical.structural_realization_count,
        semantic_o16_empty: certificate.stage16.semantic_o16_empty,
        f1_executed: certificate.f1.f1_executed,
        f1_excluded: certificate.f1.f1_excluded,
        f_fh4_triggered: certificate.f_fh4_triggered,
        e5_complete: certificate.e5_complete,
        t_bf2_authorized: certificate.t_bf2_authorized,
        bridge_authorized: certificate.bridge_authorized,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn emit_e5_future_hole_finale_v2_create_new(
    path: &Path,
) -> Result<E5FutureHoleFinaleV2Replay, E5FutureHoleFinaleV2Error> {
    let certificate = issue_e5_future_hole_finale_v2_certificate()?;
    let replay = replay_e5_future_hole_finale_v2_certificate(&certificate);
    if !replay.valid {
        return Err(E5FutureHoleFinaleV2Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| E5FutureHoleFinaleV2Error::Json(error.to_string()))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| E5FutureHoleFinaleV2Error::Io(error.to_string()))?;
    file.write_all(&bytes)
        .and_then(|_| file.write_all(b"\n"))
        .map_err(|error| E5FutureHoleFinaleV2Error::Io(error.to_string()))?;
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::{
        issue_e5_future_hole_finale_v2_certificate, replay_e5_future_hole_finale_v2_certificate,
    };

    #[test]
    fn dependent_context_finale_decides_all_rows_and_replays() {
        let certificate = issue_e5_future_hole_finale_v2_certificate().expect("E-5 v2 issues");
        let replay = replay_e5_future_hole_finale_v2_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(certificate.stage16.exact_a3_inventory_count, 89);
        assert_eq!(certificate.stage16.unary_registration_count, 17);
        assert_eq!(certificate.stage16.unary_gap_count, 0);
        assert!(certificate.stage16.unary_named_gaps.is_empty());
        assert_eq!(
            certificate.stage16.accepted_unary_exact_source_replay_count,
            17
        );
        assert_eq!(
            certificate.stage16.accepted_unary_exact_body_binding_count,
            17
        );
        assert_eq!(
            certificate
                .stage16
                .accepted_unary_totality_provisional_instance_ids
                .len(),
            0
        );
        assert!(!certificate.stage16.stable_source_guard_audit_replayed);
        assert!(
            certificate
                .stage16
                .universal_unary_specialization_totality_proved
        );
        assert_eq!(certificate.stage16.dependent_context_registration_count, 17);
        assert!(certificate.stage16.published_dependent_redeclaration_count > 0);
        assert_eq!(
            certificate
                .stage16
                .total_specialization_theorem_replay_count,
            17
        );
        assert!(certificate.stage16.f_dc1_stage16_rows_rederived);
        assert!(certificate.stage16.f_dc2_no_outcome_filtering);
        assert!(certificate.stage16.f_dc3_no_partial_credit);
        assert!(certificate.stage16.f_dc4_opaque_references_sealed_and_whole);
        assert!(certificate.stage16.f_dc5_same_issuer_scope_preserved);
        assert_eq!(certificate.stage16.direct_chronological_instance_count, 64);
        assert_eq!(
            certificate.stage16.pointwise_chronological_instance_count,
            8
        );
        assert_eq!(certificate.stage16.membership_rows.len(), 89);
        assert!(certificate.stage16.underdetermined_instance_ids.is_empty());
        assert!(certificate.stage16.unary_ids_join_exactly);
        assert!(certificate.stage16.exact_inventory_union_bijection);
        assert!(certificate.stage16.every_unary_registration_prefix_local);
        assert!(
            certificate
                .stage16
                .every_unary_registration_evidence_replayed
        );
        assert!(
            certificate
                .stage16
                .membership_rows
                .iter()
                .filter(|row| row.proof_strength_exact_source_replayed == Some(true))
                .all(|row| {
                    row.stable_specialization_totality_proved == Some(true)
                        && row.hypothetical_derivation_replayed
                        && row.output_kernel_typed
                })
        );
        assert_eq!(certificate.historical.structural_registration_count, 13);
        assert_eq!(certificate.historical.structural_realization_count, 13);
        assert!(
            certificate
                .historical
                .every_structural_registration_prefix_local
        );
        assert!(
            certificate
                .historical
                .every_structural_registration_evidence_replayed
        );
        assert!(
            certificate
                .historical
                .stage3_declaration_jurisdiction_separation_proved
        );
        assert_eq!(
            certificate
                .historical
                .provider_controls
                .wrong_provider_rejection_count,
            13
        );
        assert_eq!(
            certificate
                .historical
                .provider_controls
                .charge_swap_rejection_count,
            13
        );
        assert!(
            certificate
                .historical
                .f_fh1_every_historical_discharge_rederived
        );
        assert!(
            certificate
                .historical
                .focus_projection_reproduces_entire_coarse_ladder
        );
        assert_eq!(certificate.stage16.semantic_o16_empty, Some(true));
        assert!(certificate.f1.f1_executed);
        assert_eq!(certificate.f1.f1_excluded, Some(true));
        assert!(!certificate.f_fh4_triggered);
        assert!(certificate.e5_complete);
        assert!(certificate.semantic_o16_certificate_issued);
        assert!(certificate.t_bf2_authorized);
        assert!(certificate.e5_side_of_bridge_satisfied);
        assert!(!certificate.bridge_authorized);
        assert!(!certificate.halt_claim_issued);
        assert_eq!(
            certificate.outcome,
            "E5_V2_COMPLETE_F1_EXCLUDED_SEMANTIC_O16_EMPTY"
        );
    }

    #[test]
    fn mutation_of_charge_join_or_inventory_is_rejected() {
        let certificate = issue_e5_future_hole_finale_v2_certificate().expect("E-5 v2 issues");

        let mut charge_mutation = certificate.clone();
        charge_mutation.historical.realizations[0]
            .ordinary_filler_charge
            .certified_nu += 1;
        assert!(!replay_e5_future_hole_finale_v2_certificate(&charge_mutation).valid);

        let mut inventory_mutation = certificate.clone();
        inventory_mutation.stage16.exact_inventory_union_bijection = false;
        assert!(!replay_e5_future_hole_finale_v2_certificate(&inventory_mutation).valid);

        let mut locality_mutation = certificate.clone();
        locality_mutation
            .stage16
            .registrations
            .iter_mut()
            .find(|row| row.declaration_and_jurisdiction_locality_proved)
            .expect("one registered unary row is prefix-local")
            .declaration_and_jurisdiction_locality_proved = false;
        assert!(!replay_e5_future_hole_finale_v2_certificate(&locality_mutation).valid);

        let mut totality_mutation = certificate.clone();
        totality_mutation
            .stage16
            .universal_unary_specialization_totality_proved = false;
        assert!(!replay_e5_future_hole_finale_v2_certificate(&totality_mutation).valid);
    }

    #[test]
    fn bridge_cannot_be_minted_before_r_t2_is_consumed() {
        let mut certificate = issue_e5_future_hole_finale_v2_certificate().expect("E-5 v2 issues");
        assert!(!certificate.r_t2_confluence_consumed);
        assert!(!certificate.bridge_authorized);
        certificate.bridge_authorized = true;
        assert!(!replay_e5_future_hole_finale_v2_certificate(&certificate).valid);
    }
}
