//! Fail-closed, two-register T-BI-NU1 unified-registry audit.
//!
//! This module has deliberately no history-certificate capability.  It sees
//! an act, its exact sealed prefix, the frozen v3 declaration ledger, and the
//! term/type/equality kernels.  Structural nu, bars, verdicts, and enacted
//! futures are not inputs.  Every v3 role-schema gap is resolved either by a
//! replayable term-level family proof or by a finite-registry impossibility
//! theorem.  A resolved declaration is not automatically a credit: marginal
//! families still have to survive the global per-act anchor injection.

use crate::act_local_provenance_v3::{
    ACT_LOCAL_PROVENANCE_V3_SCHEMA, ActLocalProvenanceV3Certificate, ActLocalV3NaturalFamilyRow,
    ActLocalV3RoleOccurrence, issue_act_local_provenance_v3, issue_act_local_sequence_v3,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::generate_a3_window_for_exact_prefix_unbounded;
use pen_eval::a3_rule_inventory_exhaustiveness::prove_a3_window_inventory_for_exact_prefix_unbounded;
use pen_eval::semantic_provenance::{CreditMechanism, LocalRole};
use pen_eval::typed_families::{
    CandidateExtractionOutcome, ExtractedFamily, InstanceKind, MarginalityDisposition,
    clause_presentation, extract_candidate_families, predecessor_closure,
};
use pen_schema::context::{
    BinderId, Declaration, FormedSchemaContext, SubstitutionImage, TermExpr, TypeExpr,
    TypedSubstitutionToken, form_schema_context, issue_typed_substitution,
};
use pen_schema::e3_normalization::{
    issue_ordinary_schema_normalization, replay_ordinary_schema_normalization,
};
use pen_schema::e4_generator_basis::{
    M1GeneratedSubject, issue_step8_cell_action_m1_generated, replay_m1_generated_membership,
};
use pen_schema::e34_class_induction::{
    issue_dependent_cubical_action_induction, issue_ordinary_constructor_naturality,
    replay_ordinary_constructor_naturality,
};
use pen_schema::grammar::{
    ClauseAnchor, DerivationRef, FamilyPresentation, OrdinaryInterpretation, OrdinarySchemaKind,
    SemanticLocalRole, SupportWindow, UnitOrientation, form_ordinary_schema,
    ordinary_constructor_descriptor, ordinary_constructor_registry_digest,
    replay_ordinary_constructor_registry,
};
use pen_schema::ordinary::{issue_ordinary_typed_realizer, replay_ordinary_typed_realizer};
use pen_schema::step8_r2::{
    issue_step8_r2_typed_signatures_token, replay_step8_r2_typed_signatures_token,
};
use pen_type::cubical::{
    CubicalContext, CubicalTerm, PathRealizationToken, beta_realizer_term,
    decide_historical_path_family_equality, diagonal_realizer_term, normalize_typed_term,
    off_diagonal_realizer_term, project_historical_path_family, realize_path_basis,
};
use pen_type::elaborate::{KernelTy, SealedSignature, candidate_hash, elaborate_telescope};
use pen_type::tdc1::{PathSchemaKey, elaborate_formed_path};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const ACT_LOCAL_SEMANTIC_PROVENANCE_V4_SCHEMA: &str = "act-local-semantic-family-provenance-v4";
pub const ACT_LOCAL_SEMANTIC_PROVENANCE_V4_DATE: &str = "2026-07-22";
pub const T_BI_NU1_V4_THEOREM_ID: &str =
    "T-BI-NU1-v4-total-proof-bearing-semantic-family-extraction";
pub const V4_PREFIX_LOCAL_SOURCE_SURFACE_SCHEMA: &str =
    "act-local-semantic-prefix-local-source-surface-v1";
pub const V4_PREFIX_LOCAL_SOURCE_SURFACE_THEOREM_ID: &str =
    "T-BI-B1-v4-prefix-local-pre-anchor-source-surface-v1";
pub const ROLE_TERM_REGISTRY_V1: &str = "t-bi-nu1-v4-finite-role-term-registry-exhaustiveness";
pub const UNIFIED_TERM_REGISTRY_V2: &str =
    "t-bi-nu1-v4-core-ordinary-cubical-a3-unified-registry-v2";
pub const GENERIC_R1_V2: &str = "generic-prefix-local-formation-completion-R1-v2";
pub const PATH_QUOTIENT_V1: &str = "typed-cubical-path-family-quotient-v1";
pub const ANCHOR_INJECTION_V1: &str = "exact-family-anchor-injection-v1";
pub const R2_OCCURRENCE_MEMBERSHIP_NAMESPACE_V1: &str = "act-local-role-occurrence-membership-v1";
pub const R2_PARENT_SLOT_NAMESPACE_V1: &str = "r2-parent-role-slot-v1";

const NU_REGISTER_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/nu_register_adjudication.md");
const R1_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_quotient_adjudications.md");

/// This is the complete finite kind surface observed among the 250 v3
/// ROLE_SCHEMA_EXTRACTION_GAP declarations.  A new kind fails issuance; it is
/// never assigned a default interpretation.
pub const HISTORICAL_ROLE_KINDS: [&str; 27] = [
    "axiomatic_inherited_family",
    "axiomatic_introduction_head",
    "axiomatic_local_and_bridge_face",
    "axiomatic_support_bridge",
    "former_adjoint",
    "former_eliminator",
    "former_introduction",
    "foundation_completion",
    "foundation_formation",
    "hit_formation_package",
    "hit_kan_coherence",
    "hit_parametric_formation_action",
    "hit_path_beta",
    "hit_post_path_face",
    "hit_pre_path_declaration",
    "map_postcomposition",
    "map_precomposition",
    "map_reference_coherence",
    "map_single_action",
    "map_single_head",
    "modal_local_declaration",
    "modal_pairwise_coherence",
    "modal_uniform_legacy_action",
    "synthesis_distributive_transport",
    "synthesis_infinitesimal_shift",
    "synthesis_local_declaration",
    "synthesis_uniform_temporal_action",
];

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(ACT_LOCAL_SEMANTIC_PROVENANCE_V4_SCHEMA, domain, value))
        .expect("v4 semantic evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn prefix_local_source_tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(V4_PREFIX_LOCAL_SOURCE_SURFACE_SCHEMA, domain, value))
        .expect("v4 prefix-local source evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn value<T: Serialize + ?Sized>(input: &T) -> Value {
    serde_json::to_value(input).expect("proof projection serializes")
}

fn value_hash(domain: &str, input: &Value) -> String {
    tagged_hash(domain, input)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4TermFamilyEvidence {
    pub language: String,
    pub source_term: Value,
    pub normal_form: Value,
    pub inferred_type: Value,
    pub typing_derivation: Value,
    pub typing_derivation_hash: String,
    pub naturality_derivation: Value,
    pub naturality_derivation_hash: String,
    pub typing_replayed: bool,
    pub normalization_replayed: bool,
    pub naturality_replayed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "disposition")]
pub enum V4MarginalityDisposition {
    MarginalNoPreimage { proof: Value, proof_hash: String },
    InternalIdentical { proof: Value, proof_hash: String },
    InternalDerivable { proof: Value, proof_hash: String },
    R1CarrierPackageProvenance { generic_r1_derivation_hash: String },
}

impl V4MarginalityDisposition {
    fn is_marginal(&self) -> bool {
        matches!(self, Self::MarginalNoPreimage { .. })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "anchor_disposition")]
pub enum V4AnchorDisposition {
    NotMarginalInternal,
    PackageProvenanceNotSeparateCredit,
    CreditedLocalRole {
        clause: u16,
        role: LocalRole,
        exact_relation_hash: String,
        injection_hash: String,
    },
    ImpossibleCollision {
        theorem_id: String,
        clause: u16,
        role: LocalRole,
        competing_family_ids: Vec<String>,
        no_constructed_exported_a3_fallback: bool,
        proof_hash: String,
    },
    UncreditedAnchorCollision {
        gap_id: String,
        clause: u16,
        role: LocalRole,
        competing_family_ids: Vec<String>,
        no_adopted_canonical_matching_rule: bool,
        a3_fallback_gap: String,
        global_noninjective_matching_witness_hash: String,
    },
    NamedResidual {
        gap_id: String,
        reason: String,
        registry_query_hash: String,
        proof_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "source")]
pub enum V4FamilySource {
    Clause {
        generator_clause: u16,
        kernel_role: ClauseRole,
    },
    R1CompletedPackage {
        carrier_clause: u16,
        completion_clause: u16,
        generic_r1_derivation_hash: String,
    },
    CubicalPath {
        path_clause: u16,
        key: Value,
        path_quotient_derivation_hash: String,
    },
}

/// One direct query against a member of the adopted nine-constructor
/// ordinary registry.  `schema_issued` is deliberately not enough to bridge
/// a `pen_core::Expr`: the E-2 realizer itself says whether C1 has retired.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4OrdinaryConstructorQuery {
    pub constructor: String,
    pub descriptor: Value,
    pub exact_shape_instances: Vec<Value>,
    pub exact_shape_instance_count: usize,
    pub typed_realizer_derivation_hashes: Vec<String>,
    pub normalization_derivation_hashes: Vec<String>,
    pub naturality_derivation_hashes: Vec<String>,
    pub every_applicable_schema_issued: bool,
    pub every_typed_realizer_replayed: bool,
    pub every_normalization_replayed: bool,
    pub every_naturality_replayed: bool,
    pub global_c1_bridge_retired: bool,
    pub exact_candidate_family_bridge_proved: bool,
    pub blocker: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4OrdinaryRegistryAudit {
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub registry_digest: String,
    pub registry_replayed: bool,
    pub constructor_queries: Vec<V4OrdinaryConstructorQuery>,
    pub all_nine_constructors_queried_once: bool,
    pub applicable_schema_count: usize,
    pub proof_bearing_schema_count: usize,
    pub exact_candidate_bridge_count: usize,
    pub generated_cell_instance_count: usize,
    pub archive_or_scalar_input_used: bool,
    pub complete_as_registry_query: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4UnifiedSemanticFamily {
    pub family_id: String,
    pub stage: u32,
    pub source: V4FamilySource,
    pub term: V4TermFamilyEvidence,
    pub marginality: V4MarginalityDisposition,
    pub marginal: bool,
    pub desired_mechanism: CreditMechanism,
    pub desired_clause: u16,
    pub desired_role: LocalRole,
    /// Candidate-local role-occurrence identities that admit this family into
    /// the unified ledger. These use the same namespace as
    /// `r2_removed_child_occurrence_hashes`; R2-removed children are never
    /// surviving members.
    pub surviving_parent_membership_ids: Vec<String>,
    pub r2_removed_child_occurrence_hashes: Vec<String>,
    pub parent_membership_replayed: bool,
    pub anchor: V4AnchorDisposition,
    pub credited: bool,
    pub role_declaration_ids: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4GenericR1Proof {
    pub theorem_id: String,
    pub adoption_source_hash: String,
    pub adopted_package_clause_replayed: bool,
    pub candidate_hash: String,
    pub prefix_signature_digest: String,
    pub carrier_clause: u16,
    pub completion_clause: u16,
    pub carrier_term: Value,
    pub completion_term: Value,
    pub carrier_typing: Value,
    pub completion_typing: Value,
    pub completion_dependency_level: u32,
    pub carrier_is_kernel_formation_type: bool,
    pub completion_is_kernel_formation_type: bool,
    pub completion_is_exact_app_univ_carrier: bool,
    pub dependency_resolves_to_carrier: bool,
    pub completed_action_covers_carrier_by_adopted_r1: bool,
    pub all_four_carrier_exception_roles_enumerated: bool,
    pub local_role_case_proofs: Vec<Value>,
    pub every_carrier_role_decided_term_locally: bool,
    pub carrier_exception_decided_without_v3_absence: bool,
    pub carrier_role_surface_gap: String,
    pub carrier_owned_role_declarations: Vec<String>,
    pub no_separate_carrier_role_declaration: bool,
    pub archive_or_count_input_used: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4A3OrbitEvidence {
    pub orbit_id: String,
    pub scheme_id: String,
    pub representative_instance_id: String,
    pub member_instance_count: usize,
    pub independently_exported: bool,
    pub required_output: Value,
    pub required_output_hash: String,
    pub output_term_constructed: bool,
    pub output_term_kernel_typed: bool,
    pub usable_credit_position: bool,
    pub orbit_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4ExactA3Inventory {
    pub stage: u32,
    pub prefix_signature_digest: String,
    pub window_derivation_hash: String,
    pub inventory_exhaustiveness_derivation_hash: String,
    pub scheme_count: usize,
    pub instance_count: usize,
    pub orbit_count: usize,
    pub exported_orbit_count: usize,
    pub orbits: Vec<V4A3OrbitEvidence>,
    pub every_instance_typed: bool,
    pub exact_prefix_bound: bool,
    pub relative_rule_inventory_exhaustive: bool,
    pub generator_constructs_no_output_terms: bool,
    pub usable_credit_position_count: usize,
    pub complete: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4PathFamilyEvidence {
    pub family_id: String,
    pub key: Value,
    pub key_hash: String,
    pub path_clause: u16,
    pub term: V4TermFamilyEvidence,
    pub realization_token: Value,
    pub realization_derivation_hash: String,
    pub projection: Value,
    pub projection_derivation_hash: String,
    pub typed_projection_derivation_hash: String,
    pub predecessor_equality_decisions: Vec<Value>,
    pub predecessor_comparison_complete: bool,
    pub no_predecessor_preimage: bool,
    pub marginal: bool,
    pub desired_role: LocalRole,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4PathQuotientProof {
    pub theorem_id: String,
    pub stage: u32,
    pub dimension: u32,
    pub expected_family_count: usize,
    pub expected_key_hashes: Vec<String>,
    pub observed_key_hashes: Vec<String>,
    pub exact_key_coverage: bool,
    pub every_key_unique: bool,
    pub family_ids: Vec<String>,
    pub pairwise_decisions: Vec<Value>,
    pub pair_count_exact: bool,
    pub every_pair_decided: bool,
    pub every_ordered_key_distinct: bool,
    pub no_uniform_coordinate_multiplied: bool,
    pub complete: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4ImpossibilityProof {
    pub theorem_id: String,
    pub stage: u32,
    pub declaration_id: String,
    pub role_kind: String,
    pub owner_clause: u16,
    pub exact_coordinate: Value,
    pub finite_registry_version: String,
    pub registry_entry: Value,
    pub registry_derivation_hash: String,
    pub registry_kind_count: usize,
    pub registry_kinds_unique: bool,
    pub registry_exactly_covers_historical_surface: bool,
    pub classified_route: String,
    pub route_classification_derivation_hash: String,
    pub role_kind_in_exhaustive_registry: bool,
    pub candidate_clause_surface: Value,
    pub candidate_clause_surface_derivation_hash: String,
    pub owner_clause_enumerated: bool,
    pub candidate_clause_term_registry_exhausted: bool,
    pub cubical_path_registry_exhausted: bool,
    pub exact_a3_inventory_exhausted: bool,
    pub no_registered_term_constructor: bool,
    pub no_constructed_exported_a3_output: bool,
    pub conclusion: String,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "resolution")]
pub enum V4RoleResolution {
    ProvedFamily {
        family_id: String,
        term_derivation_hash: String,
        quotient_relation_hash: String,
        additional_credit_minted: bool,
    },
    TheoremBackedImpossibility {
        proof: V4ImpossibilityProof,
    },
    NamedRegistryResidual {
        gap_id: String,
        reason: String,
        core_match_family_ids: Vec<String>,
        cubical_match_family_ids: Vec<String>,
        ordinary_constructor_matches: Vec<String>,
        ordinary_registry_derivation_hash: String,
        generic_r1_four_role_coverage_proved: bool,
        candidate_prefix_closure_exhaustive: bool,
        derivation_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4RoleDeclarationResolution {
    pub stage: u32,
    pub declaration_id: String,
    pub v3_gap_id: String,
    pub v3_family_row_hash: String,
    pub occurrence: ActLocalV3RoleOccurrence,
    pub role_registry_entry: Value,
    pub role_route_classification_hash: String,
    pub resolution: V4RoleResolution,
    pub resolved: bool,
    pub silent_residue: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4SpecialCaseAudit {
    pub stage: u32,
    pub generic_r1: Option<V4GenericR1Proof>,
    pub stage2_constitutive_question_applicable: bool,
    pub stage2_family_internal_under_typed_predecessor_closure: bool,
    pub stage2_constitutive_exception_adopted: bool,
    pub stage2_decision: String,
    pub stage9_boundary_case_applicable: bool,
    pub stage9_map_role_declaration_count: usize,
    pub stage9_all_map_roles_resolved: bool,
    pub r2_generated_instance_removed_count: usize,
    pub r2_removed_occurrence_hashes: Vec<String>,
    pub r2_parent_slot_label_join_holds: bool,
    pub r2_exact_v3_occurrence_surface_holds: bool,
    pub r2_step8_typed_signature_derivation_hash: Option<String>,
    pub r2_m1_generated_membership_derivation_hash: Option<String>,
    pub r2_m1_generated_membership_replayed: bool,
    pub r2_parent_membership_replayed_before_anchoring: bool,
    pub r2_removed_occurrences_absent_from_unified_membership: bool,
    pub r2_removed_occurrences_emitted_as_families: usize,
    pub r2_generated_instance_multiplied: bool,
    pub derivation_hash: String,
}

/// Candidate-and-prefix-local family data before declaration resolution or
/// anchor assignment.  This is deliberately a distinct type from
/// `V4UnifiedSemanticFamily`: no role-registry route, resolution, anchor,
/// credit flag, or role-declaration identifier can be represented here.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4PrefixLocalUnifiedFamilySource {
    pub family_id: String,
    pub stage: u32,
    pub source: V4FamilySource,
    pub term: V4TermFamilyEvidence,
    pub marginality: V4MarginalityDisposition,
    pub marginal: bool,
    pub surviving_parent_membership_ids: Vec<String>,
    pub r2_removed_child_occurrence_hashes: Vec<String>,
    pub parent_membership_replayed: bool,
    pub derivation_hash: String,
}

/// The generic-R1 package premises and the adopted R2/M1 membership facts
/// available before any family-to-role relation is attempted.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4PrefixLocalR1R2Premises {
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub generic_r1: Option<V4GenericR1Proof>,
    pub generic_r1_typed_package_premises_replayed: bool,
    pub r2_generated_instance_removed_count: usize,
    pub r2_removed_occurrence_hashes: Vec<String>,
    pub r2_parent_slot_label_join_holds: bool,
    pub r2_exact_v3_occurrence_surface_holds: bool,
    pub r2_step8_typed_signature_derivation_hash: Option<String>,
    pub r2_m1_generated_membership_derivation_hash: Option<String>,
    pub r2_m1_generated_membership_replayed: bool,
    pub r2_parent_membership_replayed: bool,
    pub r2_removed_occurrences_absent_from_unified_membership: bool,
    pub r2_removed_occurrences_emitted_as_families: usize,
    pub r2_generated_instance_multiplied: bool,
    pub r2_local_premises_replayed: bool,
    pub derivation_hash: String,
}

/// Additive source-only v4 surface for prefix-generic consumers.  Issuance
/// stops before the historical role-kind registry, declaration resolution,
/// or anchor injection.  The old public v4 certificate and all of its hashes
/// remain unchanged.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4PrefixLocalSourceSurface {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub kappa: u32,
    pub candidate_elaboration_hash: String,
    pub candidate_family_extraction_hash: String,
    pub frozen_v3_schema: String,
    pub frozen_v3_package_hash: String,
    pub v3_package_replayed: bool,
    pub exact_a3: V4ExactA3Inventory,
    pub ordinary_registry: V4OrdinaryRegistryAudit,
    pub path_quotient: Option<V4PathQuotientProof>,
    pub unified_families: Vec<V4PrefixLocalUnifiedFamilySource>,
    pub r1_r2_premises: V4PrefixLocalR1R2Premises,
    pub candidate_extraction_covers_every_clause: bool,
    pub unified_family_ids_unique: bool,
    pub every_family_is_pre_anchor: bool,
    pub term_evidence_complete: bool,
    pub path_quotient_complete: bool,
    pub ordinary_constructor_surface_complete: bool,
    pub exact_a3_complete: bool,
    pub archive_read: bool,
    pub structural_nu_read: bool,
    pub bar_read: bool,
    pub verdict_read: bool,
    pub enacted_future_read: bool,
    pub historical_kind_surface_read: bool,
    pub declaration_resolution_read: bool,
    pub anchor_assignment_read: bool,
    pub source_surface_complete: bool,
    pub source_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalSemanticProvenanceV4Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub kappa: u32,
    pub candidate_elaboration_hash: String,
    pub candidate_family_extraction_hash: String,
    pub frozen_v3_schema: String,
    pub frozen_v3_package_hash: String,
    pub exact_a3: V4ExactA3Inventory,
    pub ordinary_registry: V4OrdinaryRegistryAudit,
    pub path_quotient: Option<V4PathQuotientProof>,
    pub unified_families: Vec<V4UnifiedSemanticFamily>,
    pub role_resolutions: Vec<V4RoleDeclarationResolution>,
    pub v3_role_schema_gap_count: usize,
    pub proved_role_declaration_count: usize,
    pub impossible_role_declaration_count: usize,
    pub named_role_residual_count: usize,
    pub resolved_role_declaration_count: usize,
    pub silent_role_residue_count: usize,
    pub marginal_unified_family_count: usize,
    pub credited_semantic_family_count: usize,
    pub theorem_anchor_impossibility_count: usize,
    pub uncredited_anchor_collision_count: usize,
    pub named_anchor_residual_count: usize,
    pub semantic_family_nu: u32,
    pub local_anchor_nonreuse_holds: bool,
    pub a3_output_nonreuse_holds: bool,
    pub uniform_instances_not_multiplied: bool,
    pub r2_parent_membership_replayed: bool,
    pub role_registry_exhaustive: bool,
    pub every_role_gap_resolved: bool,
    pub every_marginal_family_credited_or_theorem_impossible: bool,
    pub special_cases: V4SpecialCaseAudit,
    pub archive_read: bool,
    pub structural_nu_read: bool,
    pub bar_read: bool,
    pub verdict_read: bool,
    pub enacted_future_read: bool,
    pub authoritative_semantic_extraction: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ActLocalSemanticProvenanceV4Error {
    #[error("invalid v4 input: {0}")]
    Input(String),
    #[error("v4 kernel family extraction failed: {0}")]
    Family(String),
    #[error("v4 cubical family proof failed: {0}")]
    Cubical(String),
    #[error("v4 declaration resolution failed: {0}")]
    Resolution(String),
    #[error("v4 invariant failed: {0}")]
    Invariant(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RoleTermRoute {
    GenericR1,
    DirectClause,
    CubicalBeta,
    CubicalKan,
    SearchUnifiedRegistry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RoleTermRegistryRule {
    GenericR1,
    DirectClause,
    DirectClauseOnlyForPackageRoleZero,
    CubicalBeta,
    CubicalKan,
    NoRegisteredTerm,
}

const ROLE_TERM_REGISTRY: [(&str, RoleTermRegistryRule); 27] = [
    (
        "axiomatic_inherited_family",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    (
        "axiomatic_introduction_head",
        RoleTermRegistryRule::DirectClause,
    ),
    (
        "axiomatic_local_and_bridge_face",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    (
        "axiomatic_support_bridge",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    ("former_adjoint", RoleTermRegistryRule::NoRegisteredTerm),
    ("former_eliminator", RoleTermRegistryRule::DirectClause),
    ("former_introduction", RoleTermRegistryRule::DirectClause),
    ("foundation_completion", RoleTermRegistryRule::GenericR1),
    ("foundation_formation", RoleTermRegistryRule::DirectClause),
    (
        "hit_formation_package",
        RoleTermRegistryRule::DirectClauseOnlyForPackageRoleZero,
    ),
    ("hit_kan_coherence", RoleTermRegistryRule::CubicalKan),
    (
        "hit_parametric_formation_action",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    ("hit_path_beta", RoleTermRegistryRule::CubicalBeta),
    ("hit_post_path_face", RoleTermRegistryRule::DirectClause),
    (
        "hit_pre_path_declaration",
        RoleTermRegistryRule::DirectClause,
    ),
    (
        "map_postcomposition",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    ("map_precomposition", RoleTermRegistryRule::NoRegisteredTerm),
    (
        "map_reference_coherence",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    ("map_single_action", RoleTermRegistryRule::NoRegisteredTerm),
    ("map_single_head", RoleTermRegistryRule::DirectClause),
    (
        "modal_local_declaration",
        RoleTermRegistryRule::DirectClause,
    ),
    (
        "modal_pairwise_coherence",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    (
        "modal_uniform_legacy_action",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    (
        "synthesis_distributive_transport",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    (
        "synthesis_infinitesimal_shift",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
    (
        "synthesis_local_declaration",
        RoleTermRegistryRule::DirectClause,
    ),
    (
        "synthesis_uniform_temporal_action",
        RoleTermRegistryRule::NoRegisteredTerm,
    ),
];

fn package_role(row: &ActLocalV3NaturalFamilyRow) -> Option<u64> {
    row.representative_role
        .coordinate
        .get("coordinates")
        .and_then(|coordinates| coordinates.get("package_role"))
        .and_then(Value::as_u64)
}

fn classify_role_term_route(
    row: &ActLocalV3NaturalFamilyRow,
) -> Option<(RoleTermRoute, Value, String)> {
    let matching = ROLE_TERM_REGISTRY
        .iter()
        .enumerate()
        .filter(|(_, (kind, _))| *kind == row.representative_role.kind)
        .collect::<Vec<_>>();
    if matching.len() != 1 {
        return None;
    }
    let (registry_index, (kind, rule)) = matching[0];
    let (route, coordinate_predicate) = match rule {
        RoleTermRegistryRule::GenericR1 => (RoleTermRoute::GenericR1, json!(true)),
        RoleTermRegistryRule::DirectClause => (RoleTermRoute::DirectClause, json!(true)),
        RoleTermRegistryRule::DirectClauseOnlyForPackageRoleZero => {
            let observed = package_role(row);
            (
                if observed == Some(0) {
                    RoleTermRoute::DirectClause
                } else {
                    RoleTermRoute::SearchUnifiedRegistry
                },
                json!({
                    "predicate": "package_role == 0",
                    "observed_package_role": observed,
                    "satisfied": observed == Some(0),
                }),
            )
        }
        RoleTermRegistryRule::CubicalBeta => (RoleTermRoute::CubicalBeta, json!(true)),
        RoleTermRegistryRule::CubicalKan => (RoleTermRoute::CubicalKan, json!(true)),
        RoleTermRegistryRule::NoRegisteredTerm => (
            RoleTermRoute::SearchUnifiedRegistry,
            json!({
                "legacy_hint_only": true,
                "query_all_unified_registries": true,
                "not_an_impossibility_claim": true,
            }),
        ),
    };
    let entry = json!({
        "registry_version": ROLE_TERM_REGISTRY_V1,
        "registry_index": registry_index,
        "kind": kind,
        "rule": rule,
        "legacy_rule_is_only_a_query_hint": true,
        "exact_coordinate": row.representative_role.coordinate,
        "coordinate_predicate": coordinate_predicate,
        "classified_route": route,
    });
    let classification_hash = tagged_hash(
        "finite-role-term-route-classification",
        &(
            &entry,
            &row.representative_role.derivation_hash,
            &row.derivation_hash,
        ),
    );
    Some((route, entry, classification_hash))
}

fn generator_clause(family: &ExtractedFamily) -> Option<u16> {
    family.instances.iter().find_map(|instance| {
        matches!(&instance.kind, InstanceKind::Generator).then_some(instance.clause_index)
    })
}

fn mechanism_for_clause_role(role: ClauseRole) -> (CreditMechanism, LocalRole) {
    match role {
        ClauseRole::Formation | ClauseRole::Introduction => {
            (CreditMechanism::IntrinsicKernel, LocalRole::KernelHead)
        }
        ClauseRole::Elimination => (CreditMechanism::AdjointCompletion, LocalRole::AdjointMate),
        ClauseRole::PathAttach => (CreditMechanism::DimensionSquared, LocalRole::Coherence),
        ClauseRole::Computation => (
            CreditMechanism::P6UniformSpecialization,
            LocalRole::SupportAction,
        ),
    }
}

fn marginality_projection(disposition: &MarginalityDisposition) -> V4MarginalityDisposition {
    let proof = value(disposition);
    let proof_hash = value_hash("typed-predecessor-marginality", &proof);
    match disposition {
        MarginalityDisposition::MarginalNoClosurePreimage { .. } => {
            V4MarginalityDisposition::MarginalNoPreimage { proof, proof_hash }
        }
        MarginalityDisposition::InternalIdentical { .. } => {
            V4MarginalityDisposition::InternalIdentical { proof, proof_hash }
        }
        MarginalityDisposition::InternalDerivable { .. } => {
            V4MarginalityDisposition::InternalDerivable { proof, proof_hash }
        }
    }
}

fn clause_term_evidence(
    candidate: &Telescope,
    elaboration: &pen_type::elaborate::TelescopeElaboration,
    family: &ExtractedFamily,
) -> Result<V4TermFamilyEvidence, ActLocalSemanticProvenanceV4Error> {
    let clause = generator_clause(family).ok_or_else(|| {
        ActLocalSemanticProvenanceV4Error::Family(
            "extracted family has no generator occurrence".to_owned(),
        )
    })?;
    let candidate_clause = candidate.clauses.get(usize::from(clause)).ok_or_else(|| {
        ActLocalSemanticProvenanceV4Error::Family(format!(
            "family generator clause {clause} is outside candidate"
        ))
    })?;
    let clause_elaboration = elaboration
        .clauses
        .get(usize::from(clause))
        .ok_or_else(|| {
            ActLocalSemanticProvenanceV4Error::Family(format!(
                "elaboration lacks family generator clause {clause}"
            ))
        })?;
    let source_term = value(&candidate_clause.expr);
    let normal_form = value(&family.presentation.canonical_normal_form);
    let inferred_type = value(&family.generator_kernel_ty);
    let prior_roles = elaboration
        .clauses
        .iter()
        .map(|clause| clause.kernel_role)
        .collect::<Vec<_>>();
    let free_scope_len = elaboration.ambient_parameters + u32::from(clause);
    let replayed_presentation = clause_presentation(
        &clause_elaboration.normal_form,
        free_scope_len,
        &prior_roles[..usize::from(clause)],
        elaboration.ambient_parameters,
    );
    let normalization_replayed = replayed_presentation == family.presentation;
    let typing_derivation = json!({
        "clause_elaboration": value(clause_elaboration),
        "canonicalization_replay": value(&replayed_presentation),
        "canonicalization_matches_extracted_family": normalization_replayed,
    });
    let typing_derivation_hash = tagged_hash(
        "clause-family-typing",
        &(
            family.id.as_str(),
            clause,
            &source_term,
            &normal_form,
            &inferred_type,
            &typing_derivation,
            &elaboration.derivation_hash,
        ),
    );
    let naturality_derivation = value(&family.naturality);
    let naturality_derivation_hash = tagged_hash(
        "clause-family-naturality",
        &(
            family.id.as_str(),
            &naturality_derivation,
            &family.presentation.renaming,
        ),
    );
    let mut evidence = V4TermFamilyEvidence {
        language: "pen-core-expr/kernel-v1".to_owned(),
        source_term,
        normal_form,
        inferred_type,
        typing_derivation,
        typing_derivation_hash,
        naturality_derivation,
        naturality_derivation_hash,
        typing_replayed: clause_elaboration.clause_index == clause
            && clause_elaboration.kernel_ty == family.generator_kernel_ty,
        normalization_replayed,
        naturality_replayed: family.naturality.square.equal,
        derivation_hash: String::new(),
    };
    evidence.derivation_hash = tagged_hash("term-family-evidence", &evidence);
    Ok(evidence)
}

fn prove_generic_r1(
    prefix: &SealedSignature,
    candidate: &Telescope,
    elaboration: &pen_type::elaborate::TelescopeElaboration,
    v3: &ActLocalProvenanceV3Certificate,
) -> Result<Option<V4GenericR1Proof>, ActLocalSemanticProvenanceV4Error> {
    let adoption = std::str::from_utf8(R1_ADJUDICATION_BYTES)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Input(error.to_string()))?;
    let adopted_package_clause_replayed = adoption.contains("formation-completion-package")
        && adoption.contains("R1")
        && adoption.contains("**one** natural family")
        && adoption.contains("completion covers carrier");
    if !adopted_package_clause_replayed {
        return Err(ActLocalSemanticProvenanceV4Error::Input(
            "adopted R1 package source did not replay".to_owned(),
        ));
    }
    let mut match_pair = None;
    for (completion_index, clause) in candidate.clauses.iter().enumerate() {
        let Expr::App(function, argument) = &clause.expr else {
            continue;
        };
        if function.as_ref() != &Expr::Univ {
            continue;
        }
        let Expr::Var(level) = argument.as_ref() else {
            continue;
        };
        let ambient = elaboration.ambient_parameters;
        let priors = completion_index as u32;
        if *level <= ambient || *level > ambient + priors {
            continue;
        }
        let carrier_index = (*level - ambient - 1) as usize;
        let Some(carrier) = elaboration.clauses.get(carrier_index) else {
            continue;
        };
        let Some(completion) = elaboration.clauses.get(completion_index) else {
            continue;
        };
        if carrier.kernel_ty == KernelTy::Type
            && carrier.kernel_role == ClauseRole::Formation
            && completion.kernel_ty == KernelTy::Type
            && completion.kernel_role == ClauseRole::Formation
        {
            if match_pair
                .replace((carrier_index, completion_index, *level))
                .is_some()
            {
                return Err(ActLocalSemanticProvenanceV4Error::Family(
                    "generic R1 found multiple formation/completion packages".to_owned(),
                ));
            }
        }
    }
    let Some((carrier_index, completion_index, dependency_level)) = match_pair else {
        return Ok(None);
    };
    let carrier_clause = u16::try_from(carrier_index).map_err(|_| {
        ActLocalSemanticProvenanceV4Error::Family("R1 carrier index exceeds u16".to_owned())
    })?;
    let completion_clause = u16::try_from(completion_index).map_err(|_| {
        ActLocalSemanticProvenanceV4Error::Family("R1 completion index exceeds u16".to_owned())
    })?;
    let carrier_owned_role_declarations = v3
        .natural_family_rows
        .iter()
        .filter(|row| row.representative_role.owner_clause == carrier_clause)
        .map(|row| row.semantic_family_id.clone())
        .collect::<Vec<_>>();
    let dependency_resolves_to_carrier =
        dependency_level == elaboration.ambient_parameters + u32::from(carrier_clause) + 1;
    let carrier_is_kernel_formation_type = elaboration.clauses[carrier_index].kernel_ty
        == KernelTy::Type
        && elaboration.clauses[carrier_index].kernel_role == ClauseRole::Formation;
    let completion_is_kernel_formation_type = elaboration.clauses[completion_index].kernel_ty
        == KernelTy::Type
        && elaboration.clauses[completion_index].kernel_role == ClauseRole::Formation;
    let completion_is_exact_app_univ_carrier = matches!(
        &candidate.clauses[completion_index].expr,
        Expr::App(function, argument)
            if function.as_ref() == &Expr::Univ
                && matches!(argument.as_ref(), Expr::Var(level) if *level == dependency_level)
    );
    let completed_action_covers_carrier_by_adopted_r1 = adopted_package_clause_replayed
        && carrier_is_kernel_formation_type
        && completion_is_kernel_formation_type
        && completion_is_exact_app_univ_carrier
        && dependency_resolves_to_carrier;
    let all_four_carrier_exception_roles_enumerated = LocalRole::ALL.len() == 4;
    let formation_descriptor = ordinary_constructor_descriptor(OrdinarySchemaKind::FreshFormation);
    let formation_registry_role_is_kernel_head =
        formation_descriptor.role == SemanticLocalRole::KernelHead;
    let local_role_case_proofs = LocalRole::ALL
        .iter()
        .map(|role| {
            let generated_by_completed_package = *role == LocalRole::KernelHead
                && completed_action_covers_carrier_by_adopted_r1
                && formation_registry_role_is_kernel_head;
            // The schema descriptor names the role of a formed ordinary
            // schema, but its realizer explicitly leaves C1 open.  Therefore
            // it cannot prove non-carriage for raw candidate terms.
            let typed_role_not_carried_by_formation = false;
            json!({
                "role": role,
                "formation_descriptor": value(formation_descriptor),
                "carrier_typing_derivation": value(&elaboration.clauses[carrier_index]),
                "completion_typing_derivation": value(&elaboration.clauses[completion_index]),
                "generated_by_completed_package": generated_by_completed_package,
                "typed_role_not_carried_by_formation": typed_role_not_carried_by_formation,
                "decided": generated_by_completed_package || typed_role_not_carried_by_formation,
            })
        })
        .collect::<Vec<_>>();
    let every_carrier_role_decided_term_locally = local_role_case_proofs
        .iter()
        .all(|case| case.get("decided").and_then(Value::as_bool) == Some(true));
    let carrier_exception_decided_without_v3_absence = false;
    let no_separate_carrier_role_declaration = carrier_owned_role_declarations.is_empty();
    let proved = dependency_resolves_to_carrier
        && completed_action_covers_carrier_by_adopted_r1
        && all_four_carrier_exception_roles_enumerated
        && every_carrier_role_decided_term_locally
        && carrier_exception_decided_without_v3_absence;
    let mut proof = V4GenericR1Proof {
        theorem_id: GENERIC_R1_V2.to_owned(),
        adoption_source_hash: format!("blake3:{}", blake3_hex(R1_ADJUDICATION_BYTES)),
        adopted_package_clause_replayed,
        candidate_hash: candidate_hash(candidate),
        prefix_signature_digest: prefix.digest().to_owned(),
        carrier_clause,
        completion_clause,
        carrier_term: value(&candidate.clauses[carrier_index].expr),
        completion_term: value(&candidate.clauses[completion_index].expr),
        carrier_typing: value(&elaboration.clauses[carrier_index]),
        completion_typing: value(&elaboration.clauses[completion_index]),
        completion_dependency_level: dependency_level,
        carrier_is_kernel_formation_type,
        completion_is_kernel_formation_type,
        completion_is_exact_app_univ_carrier,
        dependency_resolves_to_carrier,
        completed_action_covers_carrier_by_adopted_r1,
        all_four_carrier_exception_roles_enumerated,
        local_role_case_proofs,
        every_carrier_role_decided_term_locally,
        carrier_exception_decided_without_v3_absence,
        carrier_role_surface_gap: "T_BI_NU1_R1_CARRIER_ROLE_SURFACE_INCOMPLETE_C1".to_owned(),
        carrier_owned_role_declarations,
        no_separate_carrier_role_declaration,
        archive_or_count_input_used: false,
        proved,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("generic-R1-proof", &proof);
    Ok(Some(proof))
}

fn exact_a3_inventory(
    prefix: &SealedSignature,
    stage: u32,
) -> Result<V4ExactA3Inventory, ActLocalSemanticProvenanceV4Error> {
    let window = generate_a3_window_for_exact_prefix_unbounded(prefix, stage)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(prefix, stage)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let scheme_by_id = window
        .schemes
        .iter()
        .map(|scheme| (scheme.scheme_id.as_str(), scheme))
        .collect::<BTreeMap<_, _>>();
    let mut orbits = Vec::new();
    for orbit in &window.orbits {
        let scheme = scheme_by_id.get(orbit.scheme_id.as_str()).ok_or_else(|| {
            ActLocalSemanticProvenanceV4Error::Family(format!(
                "A3 orbit {} has no typed scheme",
                orbit.orbit_id
            ))
        })?;
        let required_output = value(&scheme.required_output);
        let required_output_hash = tagged_hash(
            "exact-prefix-A3-required-output",
            &(
                &orbit.orbit_id,
                &orbit.scheme_id,
                &required_output,
                &scheme.formation_derivation_hash,
            ),
        );
        let mut row = V4A3OrbitEvidence {
            orbit_id: orbit.orbit_id.clone(),
            scheme_id: orbit.scheme_id.clone(),
            representative_instance_id: orbit.representative_instance_id.clone(),
            member_instance_count: orbit.member_instance_ids.len(),
            independently_exported: orbit.independently_exported_demand_orbit,
            required_output,
            required_output_hash,
            output_term_constructed: false,
            output_term_kernel_typed: false,
            usable_credit_position: false,
            orbit_derivation_hash: orbit.orbit_derivation_hash.clone(),
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("exact-A3-orbit", &row);
        orbits.push(row);
    }
    let exported_orbit_count = orbits
        .iter()
        .filter(|orbit| orbit.independently_exported)
        .count();
    let exact_prefix_bound = proof.exact_prefix_signature_digest == prefix.digest();
    let relative_rule_inventory_exhaustive = proof.relative_rule_inventory_exhaustive_for_window
        && proof.operational_instance_count == window.instances.len()
        && proof.every_operational_instance_has_exactly_one_independent_preimage
        && proof.every_operational_scheme_and_orbit_is_consumed;
    let generator_constructs_no_output_terms = orbits.iter().all(|orbit| {
        !orbit.output_term_constructed
            && !orbit.output_term_kernel_typed
            && !orbit.usable_credit_position
    });
    let usable_credit_position_count = orbits
        .iter()
        .filter(|orbit| orbit.usable_credit_position)
        .count();
    let complete = window.finite_by_construction
        && window.every_instance_typed
        && exact_prefix_bound
        && relative_rule_inventory_exhaustive
        && orbits.len() == window.orbits.len()
        && generator_constructs_no_output_terms;
    let mut inventory = V4ExactA3Inventory {
        stage,
        prefix_signature_digest: prefix.digest().to_owned(),
        window_derivation_hash: window.window_derivation_hash.clone(),
        inventory_exhaustiveness_derivation_hash: proof.derivation_hash.clone(),
        scheme_count: window.schemes.len(),
        instance_count: window.instances.len(),
        orbit_count: window.orbits.len(),
        exported_orbit_count,
        orbits,
        every_instance_typed: window.every_instance_typed,
        exact_prefix_bound,
        relative_rule_inventory_exhaustive,
        generator_constructs_no_output_terms,
        usable_credit_position_count,
        complete,
        derivation_hash: String::new(),
    };
    inventory.derivation_hash = tagged_hash("exact-A3-inventory", &inventory);
    Ok(inventory)
}

#[derive(Clone)]
struct OrdinarySpecV4 {
    kind: OrdinarySchemaKind,
    source_clauses: Vec<u16>,
    interpretation: OrdinaryInterpretation,
    generated_instance: bool,
}

fn ordinary_context_v4() -> Result<FormedSchemaContext, ActLocalSemanticProvenanceV4Error> {
    form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(0),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::TypeParameter {
            binder: BinderId(1),
            name: "B".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(2),
            name: "a".to_owned(),
            ty: TypeExpr::parameter(0),
        },
    ])
    .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))
}

fn ordinary_substitution_v4(
    source: &FormedSchemaContext,
) -> Result<TypedSubstitutionToken, ActLocalSemanticProvenanceV4Error> {
    let target = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(10),
            name: "B".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(11),
            name: "b".to_owned(),
            ty: TypeExpr::parameter(10),
        },
    ])
    .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    issue_typed_substitution(
        source,
        &target,
        vec![
            SubstitutionImage::Type {
                source: BinderId(0),
                image: TypeExpr::trunc(TypeExpr::parameter(10)),
            },
            SubstitutionImage::Type {
                source: BinderId(1),
                image: TypeExpr::parameter(10),
            },
            SubstitutionImage::Term {
                source: BinderId(2),
                image: TermExpr::TruncPoint {
                    carrier: Box::new(TypeExpr::parameter(10)),
                    point: Box::new(TermExpr::variable(11)),
                },
            },
        ],
    )
    .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))
}

fn ordinary_kind_name(kind: OrdinarySchemaKind) -> String {
    serde_json::to_value(kind)
        .expect("ordinary kind serializes")
        .as_str()
        .expect("ordinary kind is a string")
        .to_owned()
}

fn is_exact_hit_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ))
        || matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(_)))
}

fn ordinary_specs_v4(
    stage: u32,
    candidate: &Telescope,
) -> Result<Vec<OrdinarySpecV4>, ActLocalSemanticProvenanceV4Error> {
    if !(5..=8).contains(&stage) {
        return Ok(Vec::new());
    }
    let path_clauses = candidate
        .clauses
        .iter()
        .enumerate()
        .filter_map(|(index, clause)| matches!(clause.expr, Expr::PathCon(_)).then_some(index))
        .collect::<Vec<_>>();
    if path_clauses.len() != 1 {
        return Err(ActLocalSemanticProvenanceV4Error::Family(format!(
            "Stage {stage} ordinary registry requires exactly one path clause"
        )));
    }
    let path_index = path_clauses[0];
    let dimension = match candidate.clauses[path_index].expr {
        Expr::PathCon(dimension) if (1..=3).contains(&dimension) => dimension,
        _ => {
            return Err(ActLocalSemanticProvenanceV4Error::Family(format!(
                "Stage {stage} ordinary registry path dimension is outside d<=3"
            )));
        }
    };
    let formations = candidate
        .clauses
        .iter()
        .enumerate()
        .filter_map(|(index, clause)| is_exact_hit_formation(&clause.expr).then_some(index))
        .collect::<Vec<_>>();
    if formations.len() != 1 || formations[0] >= path_index {
        return Err(ActLocalSemanticProvenanceV4Error::Family(format!(
            "Stage {stage} ordinary registry has no unique pre-path formation"
        )));
    }
    let formation = u16::try_from(formations[0]).map_err(|_| {
        ActLocalSemanticProvenanceV4Error::Family("formation index exceeds u16".to_owned())
    })?;
    let path = u16::try_from(path_index).map_err(|_| {
        ActLocalSemanticProvenanceV4Error::Family("path index exceeds u16".to_owned())
    })?;
    let carrier = TypeExpr::parameter(0);
    let point = TermExpr::variable(2);
    let operation_ref = DerivationRef::parse(tagged_hash(
        "ordinary-exact-post-path-operation-ref",
        &(stage, candidate_hash(candidate), path),
    ))
    .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let boundary_ref = DerivationRef::parse(tagged_hash(
        "ordinary-exact-cell-boundary-ref",
        &(stage, candidate_hash(candidate), path, dimension),
    ))
    .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let mut specs = vec![OrdinarySpecV4 {
        kind: OrdinarySchemaKind::FreshFormation,
        source_clauses: vec![formation],
        interpretation: OrdinaryInterpretation::FreshFormation {
            carrier: carrier.clone(),
        },
        generated_instance: false,
    }];
    for index in 0..path_index {
        if index == formations[0] {
            continue;
        }
        specs.push(OrdinarySpecV4 {
            kind: OrdinarySchemaKind::PointOrUnitIntro,
            source_clauses: vec![u16::try_from(index).map_err(|_| {
                ActLocalSemanticProvenanceV4Error::Family("point index exceeds u16".to_owned())
            })?],
            interpretation: OrdinaryInterpretation::PointOrUnitIntro {
                carrier: carrier.clone(),
                point: point.clone(),
            },
            generated_instance: false,
        });
    }
    specs.extend([
        OrdinarySpecV4 {
            kind: OrdinarySchemaKind::PathConstructorIntro,
            source_clauses: vec![path],
            interpretation: OrdinaryInterpretation::PathConstructorIntro {
                carrier: carrier.clone(),
                left: point.clone(),
                right: point.clone(),
                cubical_dimension: dimension,
            },
            generated_instance: false,
        },
        OrdinarySpecV4 {
            kind: OrdinarySchemaKind::Recursor,
            source_clauses: vec![path],
            interpretation: OrdinaryInterpretation::Recursor {
                carrier: carrier.clone(),
                codomain: TypeExpr::parameter(1),
            },
            generated_instance: false,
        },
        OrdinarySpecV4 {
            kind: OrdinarySchemaKind::Inductor,
            source_clauses: vec![path],
            interpretation: OrdinaryInterpretation::Inductor {
                carrier: carrier.clone(),
                motive_fiber: TypeExpr::parameter(1),
            },
            generated_instance: false,
        },
    ]);
    if matches!(
        candidate.clauses[usize::from(formation)].expr,
        Expr::Trunc(_)
    ) {
        specs.push(OrdinarySpecV4 {
            kind: OrdinarySchemaKind::TruncParametricAction,
            source_clauses: vec![formation],
            interpretation: OrdinaryInterpretation::TruncParametricAction {
                source_carrier: carrier.clone(),
                target_carrier: TypeExpr::parameter(1),
            },
            generated_instance: false,
        });
    }
    for (offset, index) in (path_index + 1..candidate.clauses.len()).enumerate() {
        let clause = u16::try_from(index).map_err(|_| {
            ActLocalSemanticProvenanceV4Error::Family("post-path index exceeds u16".to_owned())
        })?;
        if offset % 2 == 0 {
            specs.push(OrdinarySpecV4 {
                kind: OrdinarySchemaKind::PostPathOperation,
                source_clauses: vec![clause],
                interpretation: OrdinaryInterpretation::PostPathOperation {
                    carrier: carrier.clone(),
                    arity: 2,
                },
                generated_instance: false,
            });
            specs.push(OrdinarySpecV4 {
                kind: OrdinarySchemaKind::CellAction,
                source_clauses: vec![clause, path],
                interpretation: OrdinaryInterpretation::CellAction {
                    carrier: carrier.clone(),
                    operation: operation_ref.clone(),
                    cell_dimension: dimension,
                    registered_boundary_bundle: boundary_ref.clone(),
                },
                generated_instance: true,
            });
        } else {
            specs.push(OrdinarySpecV4 {
                kind: OrdinarySchemaKind::PostPathCoherence,
                source_clauses: vec![clause],
                interpretation: OrdinaryInterpretation::PostPathCoherence {
                    carrier: carrier.clone(),
                    operation: operation_ref.clone(),
                    unit: point.clone(),
                    variable: point.clone(),
                    orientation: UnitOrientation::Left,
                },
                generated_instance: false,
            });
        }
    }
    Ok(specs)
}

fn ordinary_registry_audit_v4(
    stage: u32,
    candidate: &Telescope,
) -> Result<V4OrdinaryRegistryAudit, ActLocalSemanticProvenanceV4Error> {
    let context = ordinary_context_v4()?;
    let substitution = ordinary_substitution_v4(&context)?;
    let specs = ordinary_specs_v4(stage, candidate)?;
    let registry_digest = ordinary_constructor_registry_digest();
    let registry_replayed = replay_ordinary_constructor_registry(&registry_digest);
    let mut constructor_queries = Vec::new();
    for kind in OrdinarySchemaKind::ALL {
        let descriptor = ordinary_constructor_descriptor(kind);
        let matching = specs
            .iter()
            .filter(|spec| spec.kind == kind)
            .collect::<Vec<_>>();
        let mut exact_shape_instances = Vec::new();
        let mut typed_hashes = Vec::new();
        let mut normalization_hashes = Vec::new();
        let mut naturality_hashes = Vec::new();
        let mut all_issued = true;
        let mut all_typed = true;
        let mut all_normalized = true;
        let mut all_natural = true;
        for spec in &matching {
            let anchors = spec
                .source_clauses
                .iter()
                .map(|clause| ClauseAnchor {
                    step: stage,
                    clause: u32::from(*clause),
                })
                .collect::<Vec<_>>();
            let schema = form_ordinary_schema(
                context.clone(),
                kind,
                spec.interpretation.clone(),
                FamilyPresentation::CanonicalFamily,
                SupportWindow::new(stage.saturating_sub(1), stage).map_err(|error| {
                    ActLocalSemanticProvenanceV4Error::Family(error.to_string())
                })?,
                anchors,
            )
            .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
            let typed = issue_ordinary_typed_realizer(schema.clone())
                .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
            let typed_replayed = replay_ordinary_typed_realizer(&typed).is_ok();
            let normalization = issue_ordinary_schema_normalization(schema.clone())
                .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
            let normalization_replayed = replay_ordinary_schema_normalization(&normalization)
                .is_ok()
                && normalization.typed_before_and_after()
                && normalization.replay_stable()
                && normalization.provenance_retained();
            let naturality =
                issue_ordinary_constructor_naturality(schema.clone(), substitution.clone())
                    .map_err(|error| {
                        ActLocalSemanticProvenanceV4Error::Family(error.to_string())
                    })?;
            let naturality_replayed =
                replay_ordinary_constructor_naturality(&naturality).is_ok() && naturality.holds();
            all_typed &= typed_replayed;
            all_normalized &= normalization_replayed;
            all_natural &= naturality_replayed;
            all_issued &= true;
            typed_hashes.push(typed.derivation_hash().to_owned());
            normalization_hashes.push(normalization.derivation_hash().to_owned());
            naturality_hashes.push(naturality.derivation_hash().to_owned());
            exact_shape_instances.push(json!({
                "source_clauses": spec.source_clauses,
                "generated_instance": spec.generated_instance,
                "schema": value(&schema),
                "typed_realizer": value(&typed),
                "normalization": value(&normalization),
                "naturality": value(&naturality),
                "typed_replayed": typed_replayed,
                "normalization_replayed": normalization_replayed,
                "naturality_replayed": naturality_replayed,
            }));
        }
        // The E-2 realizer exposes GLOBAL_C1_BRIDGE_GAP unconditionally.
        // An empty applicability set is not a proof that the bridge retired.
        let global_c1_bridge_retired = false;
        let exact_candidate_family_bridge_proved = false;
        let blocker = if matching.is_empty() {
            "exact candidate shape has no instance of this ordinary constructor"
        } else {
            "the count-blind ordinary tokens replay, but OrdinaryTypedRealizerToken explicitly leaves the pen_core::Expr-to-schema C1 bridge and marginality unproved"
        }
        .to_owned();
        let mut query = V4OrdinaryConstructorQuery {
            constructor: ordinary_kind_name(kind),
            descriptor: value(descriptor),
            exact_shape_instance_count: exact_shape_instances.len(),
            exact_shape_instances,
            typed_realizer_derivation_hashes: typed_hashes,
            normalization_derivation_hashes: normalization_hashes,
            naturality_derivation_hashes: naturality_hashes,
            every_applicable_schema_issued: all_issued,
            every_typed_realizer_replayed: all_typed,
            every_normalization_replayed: all_normalized,
            every_naturality_replayed: all_natural,
            global_c1_bridge_retired,
            exact_candidate_family_bridge_proved,
            blocker,
            derivation_hash: String::new(),
        };
        query.derivation_hash = tagged_hash("ordinary-constructor-query", &query);
        constructor_queries.push(query);
    }
    let all_nine_constructors_queried_once = constructor_queries.len() == 9
        && constructor_queries
            .iter()
            .map(|query| query.constructor.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            == 9;
    let applicable_schema_count = constructor_queries
        .iter()
        .map(|query| query.exact_shape_instance_count)
        .sum();
    let proof_bearing_schema_count = constructor_queries
        .iter()
        .filter(|query| {
            query.every_applicable_schema_issued
                && query.every_typed_realizer_replayed
                && query.every_normalization_replayed
                && query.every_naturality_replayed
        })
        .map(|query| query.exact_shape_instance_count)
        .sum();
    let exact_candidate_bridge_count = constructor_queries
        .iter()
        .filter(|query| query.exact_candidate_family_bridge_proved)
        .map(|query| query.exact_shape_instance_count)
        .sum();
    let generated_cell_instance_count = specs.iter().filter(|spec| spec.generated_instance).count();
    let complete_as_registry_query = registry_replayed
        && all_nine_constructors_queried_once
        && proof_bearing_schema_count == applicable_schema_count;
    let mut audit = V4OrdinaryRegistryAudit {
        theorem_id: UNIFIED_TERM_REGISTRY_V2.to_owned(),
        stage,
        candidate_hash: candidate_hash(candidate),
        registry_digest,
        registry_replayed,
        constructor_queries,
        all_nine_constructors_queried_once,
        applicable_schema_count,
        proof_bearing_schema_count,
        exact_candidate_bridge_count,
        generated_cell_instance_count,
        archive_or_scalar_input_used: false,
        complete_as_registry_query,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("ordinary-registry-audit", &audit);
    Ok(audit)
}

#[derive(Clone)]
struct BuiltPathFamily {
    key: PathSchemaKey,
    evidence: V4PathFamilyEvidence,
}

fn path_terms(typing: &pen_type::tdc1::FormedPathTyping) -> Vec<(PathSchemaKey, CubicalTerm)> {
    let mut terms = Vec::with_capacity(1 + (typing.dimension * typing.dimension) as usize);
    terms.push((PathSchemaKey::Beta, beta_realizer_term(typing)));
    for principal in 0..typing.dimension {
        for probe in 0..typing.dimension {
            let term = if principal == probe {
                diagonal_realizer_term(typing, principal)
            } else {
                off_diagonal_realizer_term(typing, principal, probe)
            };
            terms.push((PathSchemaKey::Kan { principal, probe }, term));
        }
    }
    terms
}

fn predecessor_path_tokens(
    prefix: &SealedSignature,
) -> Result<Vec<PathRealizationToken>, ActLocalSemanticProvenanceV4Error> {
    let mut tokens = Vec::new();
    for entry in prefix.entries() {
        if entry.telescope.path_dimensions().len() != 1 {
            continue;
        }
        let (typing, _) =
            elaborate_formed_path(prefix, &entry.telescope, entry.step.saturating_sub(1))
                .map_err(|error| ActLocalSemanticProvenanceV4Error::Cubical(error.to_string()))?;
        if !(1..=3).contains(&typing.dimension) {
            continue;
        }
        let basis = realize_path_basis(
            prefix,
            &entry.telescope,
            entry.step.saturating_sub(1),
            &typing,
        )
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Cubical(error.to_string()))?;
        tokens.extend(basis.tokens().iter().cloned());
    }
    Ok(tokens)
}

fn build_path_families(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
) -> Result<(Vec<BuiltPathFamily>, Option<V4PathQuotientProof>), ActLocalSemanticProvenanceV4Error>
{
    if candidate.path_dimensions().is_empty() {
        return Ok((Vec::new(), None));
    }
    let (typing, _) = elaborate_formed_path(prefix, candidate, stage.saturating_sub(1))
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Cubical(error.to_string()))?;
    if !(1..=3).contains(&typing.dimension) {
        return Err(ActLocalSemanticProvenanceV4Error::Cubical(format!(
            "historical path dimension {} is outside the proved path-family fragment",
            typing.dimension
        )));
    }
    let terms = path_terms(&typing);
    let basis = realize_path_basis(prefix, candidate, stage.saturating_sub(1), &typing)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Cubical(error.to_string()))?;
    if basis.tokens().len() != terms.len() {
        return Err(ActLocalSemanticProvenanceV4Error::Cubical(
            "path basis token/term arity mismatch".to_owned(),
        ));
    }
    let predecessor_tokens = predecessor_path_tokens(prefix)?;
    let context = CubicalContext::total(typing.dimension as u16);
    let mut built = Vec::new();
    for ((expected_key, source_term), token) in terms.into_iter().zip(basis.tokens()) {
        if token.key() != &expected_key {
            return Err(ActLocalSemanticProvenanceV4Error::Cubical(
                "path basis order/key mismatch".to_owned(),
            ));
        }
        let (inferred_type, normal_form) = normalize_typed_term(&context, &source_term)
            .map_err(|error| ActLocalSemanticProvenanceV4Error::Cubical(error.to_string()))?;
        let induction =
            issue_dependent_cubical_action_induction(context.clone(), source_term.clone())
                .map_err(|error| ActLocalSemanticProvenanceV4Error::Cubical(error.to_string()))?;
        let projection = project_historical_path_family(token)
            .map_err(|error| ActLocalSemanticProvenanceV4Error::Cubical(error.to_string()))?;
        let mut predecessor_equality_decisions = Vec::new();
        let mut predecessor_comparison_complete = true;
        let mut no_predecessor_preimage = true;
        for predecessor in &predecessor_tokens {
            let decision = decide_historical_path_family_equality(prefix, token, predecessor);
            if decision.is_undefined() {
                predecessor_comparison_complete = false;
            }
            if decision.is_equal() {
                no_predecessor_preimage = false;
            }
            predecessor_equality_decisions.push(value(&decision));
        }
        if !predecessor_comparison_complete {
            return Err(ActLocalSemanticProvenanceV4Error::Cubical(format!(
                "Stage {stage} path marginality has an undefined predecessor comparison"
            )));
        }
        let source_value = value(&source_term);
        let normal_value = value(&normal_form);
        let type_value = value(&inferred_type);
        let realization_value = value(token);
        let projection_value = value(&projection);
        let induction_value = value(&induction);
        let typing_derivation = json!({
            "formed_path": value(&typing),
            "realization": realization_value,
            "projection": projection_value,
        });
        let typing_derivation_hash = tagged_hash(
            "cubical-path-family-typing",
            &(
                projection.family_hash(),
                token.derivation_hash(),
                projection.typed_derivation_hash(),
                &typing_derivation,
            ),
        );
        let naturality_derivation_hash = tagged_hash(
            "cubical-path-family-naturality",
            &(
                projection.family_hash(),
                induction.derivation_hash(),
                &induction_value,
            ),
        );
        let mut term = V4TermFamilyEvidence {
            language: "pen-type-dependent-cubical-v1".to_owned(),
            source_term: source_value,
            normal_form: normal_value,
            inferred_type: type_value,
            typing_derivation,
            typing_derivation_hash,
            naturality_derivation: induction_value,
            naturality_derivation_hash,
            typing_replayed: !token.derivation_hash().is_empty()
                && !projection.typed_derivation_hash().is_empty(),
            normalization_replayed: induction.normalization_stable(),
            naturality_replayed: induction.all_dimension_substitutions_natural()
                && induction.typing_preserved()
                && induction.structural_match_exhaustive_over_enum(),
            derivation_hash: String::new(),
        };
        term.derivation_hash = tagged_hash("term-family-evidence", &term);
        let desired_role = match expected_key {
            PathSchemaKey::Beta => LocalRole::KernelHead,
            PathSchemaKey::Kan { .. } => LocalRole::Coherence,
        };
        let mut evidence = V4PathFamilyEvidence {
            family_id: projection.family_hash().to_owned(),
            key: value(&expected_key),
            key_hash: tagged_hash("path-key", &expected_key),
            path_clause: typing.path_clause,
            term,
            realization_token: value(token),
            realization_derivation_hash: token.derivation_hash().to_owned(),
            projection: value(&projection),
            projection_derivation_hash: projection.projection_derivation_hash().to_owned(),
            typed_projection_derivation_hash: projection.typed_derivation_hash().to_owned(),
            predecessor_equality_decisions,
            predecessor_comparison_complete,
            no_predecessor_preimage,
            marginal: no_predecessor_preimage,
            desired_role,
            derivation_hash: String::new(),
        };
        evidence.derivation_hash = tagged_hash("path-family-evidence", &evidence);
        built.push(BuiltPathFamily {
            key: expected_key,
            evidence,
        });
    }

    let mut pairwise_decisions = Vec::new();
    let mut every_pair_decided = true;
    let mut every_ordered_key_distinct = true;
    for left in 0..basis.tokens().len() {
        for right in left + 1..basis.tokens().len() {
            let decision = decide_historical_path_family_equality(
                prefix,
                &basis.tokens()[left],
                &basis.tokens()[right],
            );
            every_pair_decided &= !decision.is_undefined();
            every_ordered_key_distinct &= decision.is_distinct();
            pairwise_decisions.push(value(&decision));
        }
    }
    let expected_keys = path_terms(&typing)
        .into_iter()
        .map(|(key, _)| path_key_hash(&key))
        .collect::<Vec<_>>();
    let observed_key_hashes = built
        .iter()
        .map(|family| path_key_hash(&family.key))
        .collect::<Vec<_>>();
    let expected_family_count = 1 + (typing.dimension * typing.dimension) as usize;
    let expected_pair_count = expected_family_count.saturating_mul(expected_family_count - 1) / 2;
    let pair_count_exact = pairwise_decisions.len() == expected_pair_count;
    let exact_key_coverage = observed_key_hashes == expected_keys;
    let every_key_unique =
        observed_key_hashes.iter().collect::<BTreeSet<_>>().len() == observed_key_hashes.len();
    let no_uniform_coordinate_multiplied = exact_key_coverage
        && every_key_unique
        && pair_count_exact
        && every_pair_decided
        && every_ordered_key_distinct;
    let complete = built.len() == expected_family_count
        && pair_count_exact
        && every_pair_decided
        && every_ordered_key_distinct
        && no_uniform_coordinate_multiplied
        && built.iter().all(|family| {
            family.evidence.term.typing_replayed
                && family.evidence.term.normalization_replayed
                && family.evidence.term.naturality_replayed
                && family.evidence.predecessor_comparison_complete
        });
    let mut quotient = V4PathQuotientProof {
        theorem_id: PATH_QUOTIENT_V1.to_owned(),
        stage,
        dimension: typing.dimension,
        expected_family_count,
        expected_key_hashes: expected_keys,
        observed_key_hashes,
        exact_key_coverage,
        every_key_unique,
        family_ids: built
            .iter()
            .map(|family| family.evidence.family_id.clone())
            .collect(),
        pairwise_decisions,
        pair_count_exact,
        every_pair_decided,
        every_ordered_key_distinct,
        no_uniform_coordinate_multiplied,
        complete,
        derivation_hash: String::new(),
    };
    quotient.derivation_hash = tagged_hash("path-quotient-proof", &quotient);
    let quotient_hash = quotient.derivation_hash.clone();
    for family in &mut built {
        family.evidence.derivation_hash = tagged_hash(
            "path-family-evidence-with-quotient",
            &(&family.evidence, &quotient_hash),
        );
    }
    Ok((built, Some(quotient)))
}

#[derive(Clone)]
struct UnifiedBuild {
    families: Vec<V4UnifiedSemanticFamily>,
    clause_family_by_clause: BTreeMap<u16, String>,
    path_family_by_key_hash: BTreeMap<String, String>,
    generic_r1: Option<V4GenericR1Proof>,
    path_quotient: Option<V4PathQuotientProof>,
    ordinary_registry: V4OrdinaryRegistryAudit,
    r2_removed_occurrence_hashes: Vec<String>,
    r2_parent_slot_label_join_holds: bool,
    r2_exact_v3_occurrence_surface_holds: bool,
    r2_step8_typed_signature_derivation_hash: Option<String>,
    r2_m1_generated_membership_derivation_hash: Option<String>,
    r2_m1_generated_membership_replayed: bool,
    r2_parent_membership_replayed: bool,
}

fn family_instance_clauses(family: &ExtractedFamily) -> BTreeSet<u16> {
    family
        .instances
        .iter()
        .map(|instance| instance.clause_index)
        .collect()
}

/// Stable identity for membership comparisons. The adopted R2 disposition is
/// deliberately excluded: changing a disposition must not change which
/// occurrence is being discussed.
fn role_occurrence_membership_id(stage: u32, occurrence: &ActLocalV3RoleOccurrence) -> String {
    tagged_hash(
        R2_OCCURRENCE_MEMBERSHIP_NAMESPACE_V1,
        &(
            stage,
            &occurrence.kind,
            occurrence.owner_clause,
            occurrence.mechanism,
            occurrence.local_role,
            &occurrence.coordinate,
            &occurrence.natural_family_shape,
            occurrence.uniform_specialization,
        ),
    )
}

/// R2 relates a generated child to a surviving semantic parent at one local
/// role slot. This is a relation key, not an occurrence identity: parent and
/// child must have distinct membership IDs even when this key agrees.
fn r2_parent_slot_id(stage: u32, occurrence: &ActLocalV3RoleOccurrence) -> String {
    tagged_hash(
        R2_PARENT_SLOT_NAMESPACE_V1,
        &(
            stage,
            occurrence.owner_clause,
            occurrence.mechanism,
            occurrence.local_role,
        ),
    )
}

fn direct_memberships(
    stage: u32,
    family: &ExtractedFamily,
    v3: &ActLocalProvenanceV3Certificate,
) -> (Vec<String>, Vec<String>, bool) {
    let clauses = family_instance_clauses(family);
    let surviving = v3
        .natural_family_rows
        .iter()
        .filter(|row| clauses.contains(&row.representative_role.owner_clause))
        .map(|row| role_occurrence_membership_id(stage, &row.representative_role))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let removed = v3
        .role_occurrences_before_quotient
        .iter()
        .filter(|occurrence| {
            occurrence.removed_by_adopted_r2 && clauses.contains(&occurrence.owner_clause)
        })
        .map(|occurrence| role_occurrence_membership_id(stage, occurrence))
        .collect::<Vec<_>>();
    let parent_replayed = v3
        .role_occurrences_before_quotient
        .iter()
        .filter(|occurrence| {
            occurrence.removed_by_adopted_r2 && clauses.contains(&occurrence.owner_clause)
        })
        .all(|removed| {
            let removed_id = role_occurrence_membership_id(stage, removed);
            let removed_slot = r2_parent_slot_id(stage, removed);
            v3.natural_family_rows
                .iter()
                .filter(|parent| {
                    !parent.removed_by_r2
                        && r2_parent_slot_id(stage, &parent.representative_role) == removed_slot
                        && role_occurrence_membership_id(stage, &parent.representative_role)
                            != removed_id
                })
                .count()
                == 1
        });
    (surviving, removed, parent_replayed)
}

#[derive(Clone, Debug)]
struct R2M1ReplayEvidence {
    exact_v3_occurrence_surface_holds: bool,
    step8_typed_signature_derivation_hash: Option<String>,
    m1_generated_membership_derivation_hash: Option<String>,
    m1_generated_membership_replayed: bool,
}

/// Join the one historical R2 removal to the term-level E-4 M1 theorem.
///
/// The v3 disposition bit identifies the declaration under audit; it is not
/// itself membership evidence.  Authority comes from replaying the exact
/// Step-8 typed-signature token and its map-cube M1 derivation, after checking
/// that their act and prefix are the inputs of this intrinsic package.
fn replay_exact_r2_m1_membership(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
) -> Result<R2M1ReplayEvidence, ActLocalSemanticProvenanceV4Error> {
    let removed = v3
        .role_occurrences_before_quotient
        .iter()
        .filter(|occurrence| occurrence.removed_by_adopted_r2)
        .collect::<Vec<_>>();
    if removed.is_empty() {
        return Ok(R2M1ReplayEvidence {
            exact_v3_occurrence_surface_holds: true,
            step8_typed_signature_derivation_hash: None,
            m1_generated_membership_derivation_hash: None,
            m1_generated_membership_replayed: true,
        });
    }

    let exact_v3_occurrence_surface_holds = stage == 8
        && removed.len() == 1
        && removed[0].kind == "hit_canonical_operation_action"
        && removed[0].owner_clause == 3
        && removed[0].mechanism == CreditMechanism::P6UniformSpecialization
        && removed[0].local_role == LocalRole::SupportAction
        && v3
            .natural_family_rows
            .iter()
            .filter(|row| {
                !row.removed_by_r2
                    && row.representative_role.kind == "hit_post_path_face"
                    && row.representative_role.owner_clause == 3
                    && row.representative_role.mechanism == CreditMechanism::P6UniformSpecialization
                    && row.representative_role.local_role == LocalRole::SupportAction
            })
            .count()
            == 1;
    if !exact_v3_occurrence_surface_holds {
        return Ok(R2M1ReplayEvidence {
            exact_v3_occurrence_surface_holds: false,
            step8_typed_signature_derivation_hash: None,
            m1_generated_membership_derivation_hash: None,
            m1_generated_membership_replayed: false,
        });
    }

    let step8 = issue_step8_r2_typed_signatures_token()
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    replay_step8_r2_typed_signatures_token(&step8)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let exact_intrinsic_inputs = step8.source_step == stage
        && step8.predecessor_signature_digest == prefix.digest()
        && step8.source_telescope_hash == candidate_hash(candidate)
        && !step8.archived_count_used_as_input
        && !step8.acceptance_bar_used_as_input;
    let generated = issue_step8_cell_action_m1_generated(&step8)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    replay_m1_generated_membership(&generated)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let subject_exact = matches!(
        generated.subject(),
        M1GeneratedSubject::Step8CellAction {
            step8_derivation_hash,
            schema_signature_derivation_hash,
        } if step8_derivation_hash == &step8.derivation_hash
            && schema_signature_derivation_hash == &step8.schema_signature_derivation_hash
    );
    let m1_generated_membership_replayed = exact_intrinsic_inputs
        && subject_exact
        && generated.final_by_basis_monotonicity()
        && !generated.full_e4_completeness_used();
    Ok(R2M1ReplayEvidence {
        exact_v3_occurrence_surface_holds,
        step8_typed_signature_derivation_hash: Some(step8.derivation_hash),
        m1_generated_membership_derivation_hash: Some(generated.derivation_hash().to_owned()),
        m1_generated_membership_replayed,
    })
}

fn path_key_hash(key: &PathSchemaKey) -> String {
    tagged_hash("path-key", key)
}

fn row_path_key(row: &ActLocalV3NaturalFamilyRow) -> Option<PathSchemaKey> {
    match row.representative_role.kind.as_str() {
        "hit_path_beta" => Some(PathSchemaKey::Beta),
        "hit_kan_coherence" => {
            let coordinates = row.representative_role.coordinate.get("coordinates")?;
            let principal = u32::try_from(coordinates.get("left_axis")?.as_u64()?).ok()?;
            let probe = u32::try_from(coordinates.get("right_axis")?.as_u64()?).ok()?;
            Some(PathSchemaKey::Kan { principal, probe })
        }
        _ => None,
    }
}

fn placeholder_anchor(marginality: &V4MarginalityDisposition) -> V4AnchorDisposition {
    if marginality.is_marginal() {
        // Replaced by `anchor_unified_families` before the package can issue.
        V4AnchorDisposition::ImpossibleCollision {
            theorem_id: "uninitialized-anchor-placeholder".to_owned(),
            clause: 0,
            role: LocalRole::KernelHead,
            competing_family_ids: Vec::new(),
            no_constructed_exported_a3_fallback: false,
            proof_hash: String::new(),
        }
    } else if matches!(
        marginality,
        V4MarginalityDisposition::R1CarrierPackageProvenance { .. }
    ) {
        V4AnchorDisposition::PackageProvenanceNotSeparateCredit
    } else {
        V4AnchorDisposition::NotMarginalInternal
    }
}

fn build_unified_families(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    elaboration: &pen_type::elaborate::TelescopeElaboration,
    extraction: &pen_eval::typed_families::CandidateFamilyExtraction,
    v3: &ActLocalProvenanceV3Certificate,
) -> Result<UnifiedBuild, ActLocalSemanticProvenanceV4Error> {
    let ordinary_registry = ordinary_registry_audit_v4(stage, candidate)?;
    let r1_requested = v3.theorem_gaps.iter().any(|gap| {
        gap.kind == "ROLE_SCHEMA_EXTRACTION_GAP"
            && gap
                .family_id
                .as_ref()
                .and_then(|id| {
                    v3.natural_family_rows
                        .iter()
                        .find(|row| &row.semantic_family_id == id)
                })
                .is_some_and(|row| row.representative_role.kind == "foundation_completion")
    });
    let generic_r1 = if r1_requested {
        let proof = prove_generic_r1(prefix, candidate, elaboration, v3)?.ok_or_else(|| {
            ActLocalSemanticProvenanceV4Error::Family(
                "foundation-completion declaration has no generic R1 package".to_owned(),
            )
        })?;
        Some(proof)
    } else {
        None
    };

    let mut families = Vec::new();
    let mut clause_family_by_clause = BTreeMap::new();
    let r2_m1 = replay_exact_r2_m1_membership(prefix, stage, candidate, v3)?;
    let mut r2_parent_slot_label_join_holds = true;
    let mut r2_parent_membership_replayed = true;
    let mut r2_removed_occurrence_hashes = v3
        .role_occurrences_before_quotient
        .iter()
        .filter(|occurrence| occurrence.removed_by_adopted_r2)
        .map(|occurrence| role_occurrence_membership_id(stage, occurrence))
        .collect::<Vec<_>>();
    r2_removed_occurrence_hashes.sort();

    for family in &extraction.families {
        let generator = generator_clause(family).ok_or_else(|| {
            ActLocalSemanticProvenanceV4Error::Family(
                "candidate family lacks a generator".to_owned(),
            )
        })?;
        let (surviving_parent_membership_ids, removed_children, parent_slot_label_join_holds) =
            direct_memberships(stage, family, v3);
        r2_parent_slot_label_join_holds &= parent_slot_label_join_holds;
        let parent_membership_replayed = removed_children.is_empty()
            || (parent_slot_label_join_holds
                && r2_m1.exact_v3_occurrence_surface_holds
                && r2_m1.m1_generated_membership_replayed);
        r2_parent_membership_replayed &= parent_membership_replayed;

        // R2 operates at semantic parent membership.  A clause presentation
        // whose sole proposed member was the removed generated action is not
        // allowed to re-enter the unified ledger through syntax alone.
        if !removed_children.is_empty()
            && surviving_parent_membership_ids.is_empty()
            && generic_r1
                .as_ref()
                .map_or(true, |proof| proof.carrier_clause != generator)
        {
            continue;
        }

        let term = clause_term_evidence(candidate, elaboration, family)?;
        if !term.typing_replayed || !term.normalization_replayed || !term.naturality_replayed {
            return Err(ActLocalSemanticProvenanceV4Error::Family(format!(
                "clause family {} did not replay typing/normalization/naturality",
                family.id.as_str()
            )));
        }
        let mut family_id = family.id.as_str().to_owned();
        let mut marginality = marginality_projection(&family.marginality);
        let mut source = V4FamilySource::Clause {
            generator_clause: generator,
            kernel_role: family.generator_role,
        };
        let (mut desired_mechanism, mut desired_role) =
            mechanism_for_clause_role(family.generator_role);
        let mut desired_clause = generator;
        if let Some(proof) = &generic_r1 {
            if generator == proof.carrier_clause {
                marginality = V4MarginalityDisposition::R1CarrierPackageProvenance {
                    generic_r1_derivation_hash: proof.derivation_hash.clone(),
                };
            } else if generator == proof.completion_clause {
                family_id = tagged_hash(
                    "generic-R1-completed-package-family",
                    &(
                        family.id.as_str(),
                        proof.carrier_clause,
                        proof.completion_clause,
                        &proof.derivation_hash,
                    ),
                );
                source = V4FamilySource::R1CompletedPackage {
                    carrier_clause: proof.carrier_clause,
                    completion_clause: proof.completion_clause,
                    generic_r1_derivation_hash: proof.derivation_hash.clone(),
                };
                desired_mechanism = CreditMechanism::IntrinsicKernel;
                desired_role = LocalRole::KernelHead;
                desired_clause = proof.completion_clause;
            }
        }
        let anchor = placeholder_anchor(&marginality);
        let mut row = V4UnifiedSemanticFamily {
            family_id: family_id.clone(),
            stage,
            source,
            term,
            marginal: marginality.is_marginal(),
            marginality,
            desired_mechanism,
            desired_clause,
            desired_role,
            surviving_parent_membership_ids,
            r2_removed_child_occurrence_hashes: removed_children,
            parent_membership_replayed,
            anchor,
            credited: false,
            role_declaration_ids: Vec::new(),
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("unified-semantic-family", &row);
        for instance in &family.instances {
            if clause_family_by_clause
                .insert(instance.clause_index, family_id.clone())
                .is_some()
            {
                return Err(ActLocalSemanticProvenanceV4Error::Family(format!(
                    "clause {} was admitted by two unified clause families",
                    instance.clause_index
                )));
            }
        }
        families.push(row);
    }

    let (path_families, path_quotient) = build_path_families(prefix, stage, candidate)?;
    let quotient_hash = path_quotient
        .as_ref()
        .map(|proof| proof.derivation_hash.clone())
        .unwrap_or_default();
    let mut path_family_by_key_hash = BTreeMap::new();
    for built in path_families {
        let key_hash = path_key_hash(&built.key);
        let memberships = v3
            .natural_family_rows
            .iter()
            .filter(|row| row_path_key(row).as_ref() == Some(&built.key))
            .map(|row| role_occurrence_membership_id(stage, &row.representative_role))
            .collect::<Vec<_>>();
        let parent_membership_replayed = memberships.len() == 1;
        let marginality = if built.evidence.marginal {
            let proof = json!({
                "procedure": "typed-cubical-predecessor-family-equality-v1",
                "decisions": built.evidence.predecessor_equality_decisions,
                "comparison_complete": built.evidence.predecessor_comparison_complete,
                "no_predecessor_preimage": built.evidence.no_predecessor_preimage,
            });
            let proof_hash = value_hash("cubical-path-marginality", &proof);
            V4MarginalityDisposition::MarginalNoPreimage { proof, proof_hash }
        } else {
            let proof = json!({
                "procedure": "typed-cubical-predecessor-family-equality-v1",
                "decisions": built.evidence.predecessor_equality_decisions,
                "comparison_complete": built.evidence.predecessor_comparison_complete,
                "no_predecessor_preimage": false,
            });
            let proof_hash = value_hash("cubical-path-marginality", &proof);
            V4MarginalityDisposition::InternalIdentical { proof, proof_hash }
        };
        let mechanism = match built.key {
            PathSchemaKey::Beta => CreditMechanism::IntrinsicKernel,
            PathSchemaKey::Kan { .. } => CreditMechanism::DimensionSquared,
        };
        let mut row = V4UnifiedSemanticFamily {
            family_id: built.evidence.family_id.clone(),
            stage,
            source: V4FamilySource::CubicalPath {
                path_clause: built.evidence.path_clause,
                key: built.evidence.key.clone(),
                path_quotient_derivation_hash: quotient_hash.clone(),
            },
            term: built.evidence.term.clone(),
            marginal: marginality.is_marginal(),
            anchor: placeholder_anchor(&marginality),
            marginality,
            desired_mechanism: mechanism,
            desired_clause: built.evidence.path_clause,
            desired_role: built.evidence.desired_role,
            surviving_parent_membership_ids: memberships,
            r2_removed_child_occurrence_hashes: Vec::new(),
            parent_membership_replayed,
            credited: false,
            role_declaration_ids: Vec::new(),
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("unified-semantic-family", &row);
        if path_family_by_key_hash
            .insert(key_hash, row.family_id.clone())
            .is_some()
        {
            return Err(ActLocalSemanticProvenanceV4Error::Cubical(
                "duplicate cubical path key".to_owned(),
            ));
        }
        families.push(row);
    }

    let ids = families
        .iter()
        .map(|family| family.family_id.clone())
        .collect::<BTreeSet<_>>();
    if ids.len() != families.len() {
        return Err(ActLocalSemanticProvenanceV4Error::Family(
            "unified clause/path quotient emitted duplicate family IDs".to_owned(),
        ));
    }
    Ok(UnifiedBuild {
        families,
        clause_family_by_clause,
        path_family_by_key_hash,
        generic_r1,
        path_quotient,
        ordinary_registry,
        r2_removed_occurrence_hashes,
        r2_parent_slot_label_join_holds,
        r2_exact_v3_occurrence_surface_holds: r2_m1.exact_v3_occurrence_surface_holds,
        r2_step8_typed_signature_derivation_hash: r2_m1.step8_typed_signature_derivation_hash,
        r2_m1_generated_membership_derivation_hash: r2_m1.m1_generated_membership_derivation_hash,
        r2_m1_generated_membership_replayed: r2_m1.m1_generated_membership_replayed,
        r2_parent_membership_replayed,
    })
}

fn anchor_unified_families(
    stage: u32,
    candidate_digest: &str,
    a3: &V4ExactA3Inventory,
    families: &mut [V4UnifiedSemanticFamily],
    resolutions: &[V4RoleDeclarationResolution],
) -> Result<(), ActLocalSemanticProvenanceV4Error> {
    let mut claims = BTreeMap::<(u16, LocalRole), Vec<(usize, String)>>::new();
    for (index, family) in families.iter_mut().enumerate() {
        if matches!(
            family.marginality,
            V4MarginalityDisposition::R1CarrierPackageProvenance { .. }
        ) {
            family.anchor = V4AnchorDisposition::PackageProvenanceNotSeparateCredit;
        } else if family.marginal {
            let family_relations = resolutions
                .iter()
                .filter_map(|resolution| match &resolution.resolution {
                    V4RoleResolution::ProvedFamily { family_id, .. }
                        if family_id == &family.family_id =>
                    {
                        Some((
                            resolution.occurrence.owner_clause,
                            resolution.occurrence.local_role,
                            resolution.occurrence.mechanism,
                            resolution.derivation_hash.clone(),
                        ))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            let distinct_relations = family_relations
                .iter()
                .map(|(clause, role, mechanism, _)| (*clause, *role, *mechanism))
                .collect::<BTreeSet<_>>();
            if distinct_relations.len() != 1 {
                let reason = if distinct_relations.is_empty() {
                    "no replayed role declaration proves an exact family-to-semantic-role relation; T_BI_NU1_A3_EXPORTED_OUTPUT_TO_ORDINARY_FAMILY_RELATION_UNPROVED records that no pre-candidate exported, constructed, typed A3 output has exact equality to this family"
                } else {
                    "the family has multiple replayed semantic-role relations and no adopted canonical relation theorem"
                };
                let registry_query_hash = tagged_hash(
                    "family-role-relation-query",
                    &(
                        stage,
                        candidate_digest,
                        &family.family_id,
                        &family_relations,
                        &a3.derivation_hash,
                    ),
                );
                let proof_hash = tagged_hash(
                    "named-family-anchor-residual",
                    &(&family.family_id, reason, &registry_query_hash),
                );
                family.anchor = V4AnchorDisposition::NamedResidual {
                    gap_id: format!(
                        "T_BI_NU1_V4_FAMILY_ROLE_RELATION_RESIDUAL_S{stage}_{}",
                        family.family_id
                    ),
                    reason: reason.to_owned(),
                    registry_query_hash,
                    proof_hash,
                };
                continue;
            }
            let (clause, role, mechanism) =
                *distinct_relations.iter().next().expect("singleton checked");
            if family.desired_clause != clause
                || family.desired_role != role
                || family.desired_mechanism != mechanism
            {
                return Err(ActLocalSemanticProvenanceV4Error::Invariant(format!(
                    "family {} role proof disagrees with its typed source projection",
                    family.family_id
                )));
            }
            let relation_hash = tagged_hash(
                "replayed-family-to-semantic-role-relation",
                &(
                    stage,
                    candidate_digest,
                    &family.family_id,
                    clause,
                    role,
                    mechanism,
                    &family_relations,
                ),
            );
            claims
                .entry((clause, role))
                .or_default()
                .push((index, relation_hash));
        } else {
            family.anchor = V4AnchorDisposition::NotMarginalInternal;
        }
    }
    for ((clause, role), entries) in claims {
        if entries.len() == 1 {
            let (index, exact_relation_hash) = &entries[0];
            let injection_hash = tagged_hash(
                "global-act-local-role-injection",
                &(
                    ANCHOR_INJECTION_V1,
                    stage,
                    candidate_digest,
                    &families[*index].family_id,
                    clause,
                    role,
                    exact_relation_hash,
                ),
            );
            families[*index].anchor = V4AnchorDisposition::CreditedLocalRole {
                clause,
                role,
                exact_relation_hash: exact_relation_hash.clone(),
                injection_hash,
            };
            families[*index].credited = true;
        } else {
            let mut competing_family_ids = entries
                .iter()
                .map(|(index, _)| families[*index].family_id.clone())
                .collect::<Vec<_>>();
            competing_family_ids.sort();
            for (index, relation_hash) in entries {
                let witness_hash = tagged_hash(
                    "global-noninjective-anchor-matching-witness",
                    &(
                        ANCHOR_INJECTION_V1,
                        stage,
                        candidate_digest,
                        clause,
                        role,
                        &families[index].family_id,
                        &competing_family_ids,
                        &relation_hash,
                        &a3.derivation_hash,
                    ),
                );
                families[index].anchor = V4AnchorDisposition::UncreditedAnchorCollision {
                    gap_id: "T_BI_NU1_ANCHOR_MATCHING_UNDERDETERMINED".to_owned(),
                    clause,
                    role,
                    competing_family_ids: competing_family_ids.clone(),
                    no_adopted_canonical_matching_rule: true,
                    a3_fallback_gap:
                        "T_BI_NU1_A3_EXPORTED_OUTPUT_TO_ORDINARY_FAMILY_RELATION_UNPROVED"
                            .to_owned(),
                    global_noninjective_matching_witness_hash: witness_hash,
                };
            }
        }
    }
    if families.iter().any(|family| {
        family.marginal
            && matches!(
                &family.anchor,
                V4AnchorDisposition::ImpossibleCollision { theorem_id, .. }
                    if theorem_id == "uninitialized-anchor-placeholder"
            )
    }) {
        return Err(ActLocalSemanticProvenanceV4Error::Invariant(
            "a marginal family retained the anchor placeholder".to_owned(),
        ));
    }
    Ok(())
}

fn role_gap_rows<'a>(
    v3: &'a ActLocalProvenanceV3Certificate,
) -> Result<
    Vec<(
        &'a crate::act_local_provenance_v3::ActLocalV3Gap,
        &'a ActLocalV3NaturalFamilyRow,
    )>,
    ActLocalSemanticProvenanceV4Error,
> {
    let rows = v3
        .theorem_gaps
        .iter()
        .filter(|gap| gap.kind == "ROLE_SCHEMA_EXTRACTION_GAP")
        .map(|gap| {
            let id = gap.family_id.as_ref().ok_or_else(|| {
                ActLocalSemanticProvenanceV4Error::Resolution(format!(
                    "role gap {} has no v3 family ID",
                    gap.id
                ))
            })?;
            let row = v3
                .natural_family_rows
                .iter()
                .find(|row| &row.semantic_family_id == id)
                .ok_or_else(|| {
                    ActLocalSemanticProvenanceV4Error::Resolution(format!(
                        "role gap {} has no v3 natural-family row",
                        gap.id
                    ))
                })?;
            if row.gap_id.as_deref() != Some(gap.id.as_str()) {
                return Err(ActLocalSemanticProvenanceV4Error::Resolution(format!(
                    "v3 role gap {} is not the row's exact gap",
                    gap.id
                )));
            }
            Ok((gap, row))
        })
        .collect::<Result<Vec<_>, ActLocalSemanticProvenanceV4Error>>()?;
    let ids = rows
        .iter()
        .map(|(gap, _)| gap.id.as_str())
        .collect::<BTreeSet<_>>();
    if ids.len() != rows.len() {
        return Err(ActLocalSemanticProvenanceV4Error::Resolution(
            "v3 role-schema gaps are not uniquely named".to_owned(),
        ));
    }
    Ok(rows)
}

fn family_index_by_id(families: &[V4UnifiedSemanticFamily], family_id: &str) -> Option<usize> {
    families
        .iter()
        .position(|family| family.family_id == family_id)
}

fn ordinary_relation(kind: &str) -> Option<(CreditMechanism, LocalRole)> {
    match kind {
        "fresh_formation" | "point_or_unit_intro" | "path_constructor_intro" => {
            Some((CreditMechanism::IntrinsicKernel, LocalRole::KernelHead))
        }
        "recursor" | "inductor" => {
            Some((CreditMechanism::AdjointCompletion, LocalRole::KernelHead))
        }
        "trunc_parametric_action" => Some((
            CreditMechanism::P6UniformSpecialization,
            LocalRole::SupportAction,
        )),
        "post_path_operation" => Some((
            CreditMechanism::P6UniformSpecialization,
            LocalRole::KernelHead,
        )),
        "post_path_coherence" => Some((
            CreditMechanism::P6UniformSpecialization,
            LocalRole::Coherence,
        )),
        "cell_action" => Some((
            CreditMechanism::P6UniformSpecialization,
            LocalRole::SupportAction,
        )),
        _ => None,
    }
}

fn ordinary_matches_for_occurrence(
    audit: &V4OrdinaryRegistryAudit,
    occurrence: &ActLocalV3RoleOccurrence,
) -> Vec<String> {
    let mut matches = Vec::new();
    for query in &audit.constructor_queries {
        let Some((mechanism, role)) = ordinary_relation(&query.constructor) else {
            continue;
        };
        if mechanism != occurrence.mechanism || role != occurrence.local_role {
            continue;
        }
        for (index, instance) in query.exact_shape_instances.iter().enumerate() {
            let owns_clause = instance
                .get("source_clauses")
                .and_then(Value::as_array)
                .is_some_and(|clauses| {
                    clauses
                        .iter()
                        .any(|clause| clause.as_u64() == Some(u64::from(occurrence.owner_clause)))
                });
            if owns_clause {
                matches.push(format!("{}#{index}", query.constructor));
            }
        }
    }
    matches.sort();
    matches.dedup();
    matches
}

fn resolve_role_declarations(
    stage: u32,
    v3: &ActLocalProvenanceV3Certificate,
    a3: &V4ExactA3Inventory,
    extraction: &pen_eval::typed_families::CandidateFamilyExtraction,
    build: &mut UnifiedBuild,
) -> Result<Vec<V4RoleDeclarationResolution>, ActLocalSemanticProvenanceV4Error> {
    let registry = HISTORICAL_ROLE_KINDS.into_iter().collect::<BTreeSet<_>>();
    let registry_kinds = ROLE_TERM_REGISTRY
        .iter()
        .map(|(kind, _)| *kind)
        .collect::<Vec<_>>();
    let registry_kind_set = registry_kinds.iter().copied().collect::<BTreeSet<_>>();
    let registry_kinds_unique = registry_kind_set.len() == registry_kinds.len();
    let registry_exactly_covers_historical_surface = registry_kind_set == registry;
    let registry_derivation_hash = tagged_hash(
        "finite-role-term-registry",
        &(
            ROLE_TERM_REGISTRY_V1,
            ROLE_TERM_REGISTRY,
            registry_kinds_unique,
            registry_exactly_covers_historical_surface,
        ),
    );
    let candidate_clause_surface = json!({
        "extraction_derivation_hash": extraction.derivation_hash,
        "families": extraction.families.iter().map(|family| json!({
            "family_id": family.id.as_str(),
            "generator_role": family.generator_role,
            "instances": family.instances,
        })).collect::<Vec<_>>(),
    });
    let candidate_clause_surface_derivation_hash = tagged_hash(
        "exact-candidate-clause-term-surface",
        &candidate_clause_surface,
    );
    let enumerated_clauses = extraction
        .families
        .iter()
        .flat_map(|family| {
            family
                .instances
                .iter()
                .map(|instance| instance.clause_index)
        })
        .collect::<BTreeSet<_>>();
    let expected_clauses = (0..v3.kappa)
        .filter_map(|clause| u16::try_from(clause).ok())
        .collect::<BTreeSet<_>>();
    let candidate_clause_surface_exact = enumerated_clauses == expected_clauses
        && build
            .clause_family_by_clause
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            == expected_clauses;
    let rows = role_gap_rows(v3)?;
    let mut resolutions = Vec::new();
    for (gap, row) in rows {
        let occurrence = row.representative_role.clone();
        let (route, mut registry_entry, role_route_classification_hash) =
            classify_role_term_route(row).ok_or_else(|| {
                ActLocalSemanticProvenanceV4Error::Resolution(format!(
                    "role {} has no unique finite-registry classification",
                    gap.id
                ))
            })?;
        // These are registry candidates only.  Equality of clause/role labels
        // does not prove that the semantic role declaration is realized by a
        // candidate term family.
        let mut core_matches = Vec::new();
        let mut exact_term_relations = Vec::<(String, String)>::new();
        if route == RoleTermRoute::DirectClause {
            if let Some(family_id) = build
                .clause_family_by_clause
                .get(&occurrence.owner_clause)
                .cloned()
            {
                if let Some(family) = build
                    .families
                    .iter()
                    .find(|family| family.family_id == family_id)
                {
                    if family.desired_clause == occurrence.owner_clause
                        && family.desired_mechanism == occurrence.mechanism
                        && family.desired_role == occurrence.local_role
                    {
                        core_matches.push(family_id);
                    }
                }
            }
        }
        if route == RoleTermRoute::GenericR1 {
            if let Some(proof) = &build.generic_r1 {
                if proof.proved
                    && proof.every_carrier_role_decided_term_locally
                    && proof.carrier_exception_decided_without_v3_absence
                    && occurrence.owner_clause == proof.completion_clause
                    && occurrence.mechanism == CreditMechanism::IntrinsicKernel
                    && occurrence.local_role == LocalRole::KernelHead
                {
                    if let Some(family_id) = build
                        .clause_family_by_clause
                        .get(&proof.completion_clause)
                        .cloned()
                    {
                        core_matches.push(family_id.clone());
                        // Unlike the clause-label join above, generic R1 is an
                        // explicit term-level package theorem.  It may resolve
                        // the role only after all of its proof flags hold.
                        exact_term_relations.push((family_id, proof.derivation_hash.clone()));
                    }
                }
            }
        }
        let mut cubical_matches = Vec::new();
        if matches!(
            route,
            RoleTermRoute::CubicalBeta | RoleTermRoute::CubicalKan
        ) {
            if let Some(family_id) = row_path_key(row).and_then(|key| {
                build
                    .path_family_by_key_hash
                    .get(&path_key_hash(&key))
                    .cloned()
            }) {
                if let Some(family) = build
                    .families
                    .iter()
                    .find(|family| family.family_id == family_id)
                {
                    if family.desired_clause == occurrence.owner_clause
                        && family.desired_mechanism == occurrence.mechanism
                        && family.desired_role == occurrence.local_role
                    {
                        cubical_matches.push(family_id);
                    }
                }
            }
        }
        let ordinary_constructor_matches =
            ordinary_matches_for_occurrence(&build.ordinary_registry, &occurrence);
        core_matches.sort();
        core_matches.dedup();
        cubical_matches.sort();
        cubical_matches.dedup();
        exact_term_relations.sort();
        exact_term_relations.dedup();
        registry_entry["unified_registry_version"] = json!(UNIFIED_TERM_REGISTRY_V2);
        registry_entry["label_only_core_candidates"] = json!(core_matches);
        registry_entry["label_only_cubical_candidates"] = json!(cubical_matches);
        registry_entry["exact_term_relation_theorems"] = json!(exact_term_relations);
        registry_entry["ordinary_constructor_matches"] = json!(ordinary_constructor_matches);
        registry_entry["ordinary_registry_derivation_hash"] =
            json!(build.ordinary_registry.derivation_hash);
        registry_entry["candidate_prefix_closure_exhaustive"] = json!(false);
        let proved_relation =
            (exact_term_relations.len() == 1).then(|| exact_term_relations[0].clone());
        let resolution = if let Some((family_id, exact_term_relation_hash)) = proved_relation {
            let index = family_index_by_id(&build.families, &family_id).ok_or_else(|| {
                ActLocalSemanticProvenanceV4Error::Resolution(format!(
                    "role {} resolved to missing unified family {family_id}",
                    gap.id
                ))
            })?;
            let family = &mut build.families[index];
            if !family.term.typing_replayed
                || !family.term.normalization_replayed
                || !family.term.naturality_replayed
            {
                return Err(ActLocalSemanticProvenanceV4Error::Resolution(format!(
                    "role {} points at a family without complete term evidence",
                    gap.id
                )));
            }
            family.role_declaration_ids.push(gap.id.clone());
            V4RoleResolution::ProvedFamily {
                family_id,
                term_derivation_hash: family.term.derivation_hash.clone(),
                quotient_relation_hash: exact_term_relation_hash,
                additional_credit_minted: false,
            }
        } else {
            let reason = if exact_term_relations.len() > 1 {
                "the unified registry returned multiple exact term-level relation theorems and no adopted canonical theorem selection rule"
            } else if !core_matches.is_empty() || !cubical_matches.is_empty() {
                "candidate families match the declaration only by clause, mechanism, local-role, or cubical-key labels; no exact term-level relation theorem connects the role schema to those families"
            } else if !ordinary_constructor_matches.is_empty() {
                "an exact ordinary constructor schema is typed, normalized, and natural, but its own token leaves the pen_core::Expr C1 bridge and marginality unproved"
            } else {
                "the core, ordinary, cubical, and exact-A3 queries returned no bridged family, but the adopted candidate-plus-prefix grammar closure has not been proved exhaustive"
            }
            .to_owned();
            let generic_r1_four_role_coverage_proved =
                build.generic_r1.as_ref().is_some_and(|proof| {
                    proof.every_carrier_role_decided_term_locally
                        && proof.carrier_exception_decided_without_v3_absence
                });
            let residual_hash = tagged_hash(
                "named-unified-registry-residual",
                &(
                    &gap.id,
                    &reason,
                    &core_matches,
                    &cubical_matches,
                    &exact_term_relations,
                    &ordinary_constructor_matches,
                    &build.ordinary_registry.derivation_hash,
                    generic_r1_four_role_coverage_proved,
                    false,
                    &candidate_clause_surface_derivation_hash,
                    candidate_clause_surface_exact,
                    &registry_derivation_hash,
                    a3.complete,
                ),
            );
            V4RoleResolution::NamedRegistryResidual {
                gap_id: format!("T_BI_NU1_V4_UNIFIED_REGISTRY_RESIDUAL_{}", gap.id),
                reason,
                core_match_family_ids: core_matches,
                cubical_match_family_ids: cubical_matches,
                ordinary_constructor_matches,
                ordinary_registry_derivation_hash: build.ordinary_registry.derivation_hash.clone(),
                generic_r1_four_role_coverage_proved,
                candidate_prefix_closure_exhaustive: false,
                derivation_hash: residual_hash,
            }
        };
        let resolved = !matches!(resolution, V4RoleResolution::NamedRegistryResidual { .. });
        let mut row_resolution = V4RoleDeclarationResolution {
            stage,
            declaration_id: gap.id.clone(),
            v3_gap_id: gap.id.clone(),
            v3_family_row_hash: row.derivation_hash.clone(),
            occurrence,
            role_registry_entry: registry_entry,
            role_route_classification_hash,
            resolution,
            resolved,
            silent_residue: false,
            derivation_hash: String::new(),
        };
        row_resolution.derivation_hash =
            tagged_hash("role-declaration-resolution", &row_resolution);
        resolutions.push(row_resolution);
    }
    for family in &mut build.families {
        family.role_declaration_ids.sort();
        family.role_declaration_ids.dedup();
        family.derivation_hash = tagged_hash("unified-semantic-family", family);
    }
    resolutions.sort_by(|left, right| left.declaration_id.cmp(&right.declaration_id));
    Ok(resolutions)
}

fn replay_nu_register_adjudication() -> Result<(), ActLocalSemanticProvenanceV4Error> {
    let text = std::str::from_utf8(NU_REGISTER_ADJUDICATION_BYTES)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Input(error.to_string()))?;
    if !text.contains("nu-register-split-v1")
        || !text.contains("F-AL1")
        || !text.contains("zero silent residue")
        || !text.contains("Scalar equality with the archive is")
        || !text.contains("## ADOPTION")
    {
        return Err(ActLocalSemanticProvenanceV4Error::Input(
            "nu-register-split-v1 adoption did not replay".to_owned(),
        ));
    }
    Ok(())
}

fn certificate_digest(certificate: &ActLocalSemanticProvenanceV4Certificate) -> String {
    let mut projection = certificate.clone();
    projection.derivation_hash.clear();
    tagged_hash("act-local-semantic-provenance-certificate", &projection)
}

fn prefix_local_unified_family_source_hash(family: &V4PrefixLocalUnifiedFamilySource) -> String {
    let mut projection = family.clone();
    projection.derivation_hash.clear();
    prefix_local_source_tagged_hash("pre-anchor-unified-family", &projection)
}

fn prefix_local_r1_r2_premises_hash(premises: &V4PrefixLocalR1R2Premises) -> String {
    let mut projection = premises.clone();
    projection.derivation_hash.clear();
    prefix_local_source_tagged_hash("generic-r1-r2-local-premises", &projection)
}

fn prefix_local_source_surface_hash(surface: &V4PrefixLocalSourceSurface) -> String {
    let mut projection = surface.clone();
    projection.source_hash.clear();
    prefix_local_source_tagged_hash("prefix-local-source-surface", &projection)
}

fn project_prefix_local_unified_family(
    family: &V4UnifiedSemanticFamily,
) -> V4PrefixLocalUnifiedFamilySource {
    let mut projected = V4PrefixLocalUnifiedFamilySource {
        family_id: family.family_id.clone(),
        stage: family.stage,
        source: family.source.clone(),
        term: family.term.clone(),
        marginality: family.marginality.clone(),
        marginal: family.marginal,
        surviving_parent_membership_ids: family.surviving_parent_membership_ids.clone(),
        r2_removed_child_occurrence_hashes: family.r2_removed_child_occurrence_hashes.clone(),
        parent_membership_replayed: family.parent_membership_replayed,
        derivation_hash: String::new(),
    };
    projected.derivation_hash = prefix_local_unified_family_source_hash(&projected);
    projected
}

fn build_prefix_local_r1_r2_premises(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    build: &UnifiedBuild,
) -> V4PrefixLocalR1R2Premises {
    let candidate_digest = candidate_hash(candidate);
    let generic_r1_typed_package_premises_replayed = build.generic_r1.as_ref().map_or(true, |r1| {
        r1.candidate_hash == candidate_digest
            && r1.prefix_signature_digest == prefix.digest()
            && r1.adopted_package_clause_replayed
            && r1.carrier_is_kernel_formation_type
            && r1.completion_is_kernel_formation_type
            && r1.completion_is_exact_app_univ_carrier
            && r1.dependency_resolves_to_carrier
            && r1.completed_action_covers_carrier_by_adopted_r1
            && r1.all_four_carrier_exception_roles_enumerated
            && !r1.archive_or_count_input_used
    });
    let surviving_memberships = build
        .families
        .iter()
        .flat_map(|family| family.surviving_parent_membership_ids.iter().cloned())
        .collect::<BTreeSet<_>>();
    let removed_memberships = build
        .r2_removed_occurrence_hashes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let r2_removed_occurrences_emitted_as_families = surviving_memberships
        .intersection(&removed_memberships)
        .count();
    let r2_removed_occurrences_absent_from_unified_membership =
        r2_removed_occurrences_emitted_as_families == 0;
    let r2_generated_instance_multiplied = !r2_removed_occurrences_absent_from_unified_membership
        || !build.r2_parent_membership_replayed;
    let r2_hash_surface_exact = if build.r2_removed_occurrence_hashes.is_empty() {
        build.r2_step8_typed_signature_derivation_hash.is_none()
            && build.r2_m1_generated_membership_derivation_hash.is_none()
    } else {
        build.r2_step8_typed_signature_derivation_hash.is_some()
            && build.r2_m1_generated_membership_derivation_hash.is_some()
    };
    let r2_local_premises_replayed = build.r2_parent_slot_label_join_holds
        && build.r2_exact_v3_occurrence_surface_holds
        && build.r2_m1_generated_membership_replayed
        && build.r2_parent_membership_replayed
        && r2_removed_occurrences_absent_from_unified_membership
        && r2_removed_occurrences_emitted_as_families == 0
        && !r2_generated_instance_multiplied
        && r2_hash_surface_exact;
    let mut premises = V4PrefixLocalR1R2Premises {
        stage,
        candidate_hash: candidate_digest,
        predecessor_signature_digest: prefix.digest().to_owned(),
        generic_r1: build.generic_r1.clone(),
        generic_r1_typed_package_premises_replayed,
        r2_generated_instance_removed_count: build.r2_removed_occurrence_hashes.len(),
        r2_removed_occurrence_hashes: build.r2_removed_occurrence_hashes.clone(),
        r2_parent_slot_label_join_holds: build.r2_parent_slot_label_join_holds,
        r2_exact_v3_occurrence_surface_holds: build.r2_exact_v3_occurrence_surface_holds,
        r2_step8_typed_signature_derivation_hash: build
            .r2_step8_typed_signature_derivation_hash
            .clone(),
        r2_m1_generated_membership_derivation_hash: build
            .r2_m1_generated_membership_derivation_hash
            .clone(),
        r2_m1_generated_membership_replayed: build.r2_m1_generated_membership_replayed,
        r2_parent_membership_replayed: build.r2_parent_membership_replayed,
        r2_removed_occurrences_absent_from_unified_membership,
        r2_removed_occurrences_emitted_as_families,
        r2_generated_instance_multiplied,
        r2_local_premises_replayed,
        derivation_hash: String::new(),
    };
    premises.derivation_hash = prefix_local_r1_r2_premises_hash(&premises);
    premises
}

/// Issue only the candidate-local semantic constructor surface.  This path
/// intentionally ends before `resolve_role_declarations` and
/// `anchor_unified_families`; it also has no dependency on either historical
/// role-kind constant.
pub fn issue_v4_prefix_local_source_surface(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
) -> Result<V4PrefixLocalSourceSurface, ActLocalSemanticProvenanceV4Error> {
    let expected_v3 = issue_act_local_provenance_v3(prefix, stage, candidate)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Input(error.to_string()))?;
    let v3_package_replayed = &expected_v3 == v3
        && v3.schema == ACT_LOCAL_PROVENANCE_V3_SCHEMA
        && v3.stage == stage
        && v3.candidate_hash == candidate_hash(candidate)
        && v3.predecessor_signature_digest == prefix.digest();
    if !v3_package_replayed {
        return Err(ActLocalSemanticProvenanceV4Error::Input(
            "prefix-local source v3 package did not replay exactly".to_owned(),
        ));
    }

    let elaboration = elaborate_telescope(prefix, candidate, stage.saturating_sub(1))
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let closure = predecessor_closure(prefix)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let extraction =
        match extract_candidate_families(prefix, &closure, candidate, stage.saturating_sub(1)) {
            CandidateExtractionOutcome::Extracted(extraction) => extraction,
            CandidateExtractionOutcome::KernelInvalid { failure } => {
                return Err(ActLocalSemanticProvenanceV4Error::Family(
                    failure.to_string(),
                ));
            }
        };
    let exact_a3 = exact_a3_inventory(prefix, stage)?;
    if !exact_a3.complete {
        return Err(ActLocalSemanticProvenanceV4Error::Family(
            "prefix-local exact-A3 inventory did not close".to_owned(),
        ));
    }
    let build = build_unified_families(prefix, stage, candidate, &elaboration, &extraction, v3)?;

    let every_family_is_pre_anchor = build.families.iter().all(|family| {
        family.anchor == placeholder_anchor(&family.marginality)
            && !family.credited
            && family.role_declaration_ids.is_empty()
    });
    if !every_family_is_pre_anchor {
        return Err(ActLocalSemanticProvenanceV4Error::Invariant(
            "prefix-local source construction crossed the anchor boundary".to_owned(),
        ));
    }
    let unified_families = build
        .families
        .iter()
        .map(project_prefix_local_unified_family)
        .collect::<Vec<_>>();
    let unified_family_ids_unique = unified_families
        .iter()
        .map(|family| family.family_id.as_str())
        .collect::<BTreeSet<_>>()
        .len()
        == unified_families.len();
    let extracted_clauses = extraction
        .families
        .iter()
        .flat_map(|family| {
            family
                .instances
                .iter()
                .map(|instance| instance.clause_index)
        })
        .collect::<BTreeSet<_>>();
    let expected_clauses = (0..candidate.clauses.len())
        .filter_map(|index| u16::try_from(index).ok())
        .collect::<BTreeSet<_>>();
    let candidate_extraction_covers_every_clause = extracted_clauses == expected_clauses;
    let term_evidence_complete = unified_families.iter().all(|family| {
        family.term.typing_replayed
            && family.term.normalization_replayed
            && family.term.naturality_replayed
            && family.parent_membership_replayed
            && family.derivation_hash == prefix_local_unified_family_source_hash(family)
    });
    let path_quotient_complete = build
        .path_quotient
        .as_ref()
        .map_or(true, |proof| proof.complete);
    let ordinary_constructor_surface_complete = build.ordinary_registry.complete_as_registry_query
        && !build.ordinary_registry.archive_or_scalar_input_used;
    let exact_a3_complete = exact_a3.complete;
    let r1_r2_premises = build_prefix_local_r1_r2_premises(prefix, stage, candidate, &build);

    let archive_read = false;
    let structural_nu_read = false;
    let bar_read = false;
    let verdict_read = false;
    let enacted_future_read = false;
    let historical_kind_surface_read = false;
    let declaration_resolution_read = false;
    let anchor_assignment_read = false;
    let source_surface_complete = v3_package_replayed
        && candidate_extraction_covers_every_clause
        && unified_family_ids_unique
        && every_family_is_pre_anchor
        && term_evidence_complete
        && path_quotient_complete
        && ordinary_constructor_surface_complete
        && exact_a3_complete
        && r1_r2_premises.generic_r1_typed_package_premises_replayed
        && r1_r2_premises.r2_local_premises_replayed
        && !archive_read
        && !structural_nu_read
        && !bar_read
        && !verdict_read
        && !enacted_future_read
        && !historical_kind_surface_read
        && !declaration_resolution_read
        && !anchor_assignment_read;
    if !source_surface_complete {
        return Err(ActLocalSemanticProvenanceV4Error::Invariant(
            "prefix-local source surface did not close".to_owned(),
        ));
    }

    let mut surface = V4PrefixLocalSourceSurface {
        schema: V4_PREFIX_LOCAL_SOURCE_SURFACE_SCHEMA.to_owned(),
        date: ACT_LOCAL_SEMANTIC_PROVENANCE_V4_DATE.to_owned(),
        theorem_id: V4_PREFIX_LOCAL_SOURCE_SURFACE_THEOREM_ID.to_owned(),
        stage,
        candidate_hash: candidate_hash(candidate),
        predecessor_signature_digest: prefix.digest().to_owned(),
        kappa: candidate.kappa() as u32,
        candidate_elaboration_hash: elaboration.derivation_hash,
        candidate_family_extraction_hash: extraction.derivation_hash,
        frozen_v3_schema: v3.schema.clone(),
        frozen_v3_package_hash: v3.derivation_hash.clone(),
        v3_package_replayed,
        exact_a3,
        ordinary_registry: build.ordinary_registry,
        path_quotient: build.path_quotient,
        unified_families,
        r1_r2_premises,
        candidate_extraction_covers_every_clause,
        unified_family_ids_unique,
        every_family_is_pre_anchor,
        term_evidence_complete,
        path_quotient_complete,
        ordinary_constructor_surface_complete,
        exact_a3_complete,
        archive_read,
        structural_nu_read,
        bar_read,
        verdict_read,
        enacted_future_read,
        historical_kind_surface_read,
        declaration_resolution_read,
        anchor_assignment_read,
        source_surface_complete,
        source_hash: String::new(),
    };
    surface.source_hash = prefix_local_source_surface_hash(&surface);
    Ok(surface)
}

pub fn replay_v4_prefix_local_source_surface(
    prefix: &SealedSignature,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
    claimed: &V4PrefixLocalSourceSurface,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.source_hash != prefix_local_source_surface_hash(claimed) {
        errors.push("v4 prefix-local source surface digest mismatch".to_owned());
    }
    match issue_v4_prefix_local_source_surface(prefix, claimed.stage, candidate, v3) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push(
            "v4 prefix-local source surface differs from deterministic reissuance".to_owned(),
        ),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn special_case_audit(
    stage: u32,
    build: &UnifiedBuild,
    resolutions: &[V4RoleDeclarationResolution],
) -> V4SpecialCaseAudit {
    let stage2_constitutive_question_applicable = stage == 2;
    let stage2_family_internal_under_typed_predecessor_closure = stage != 2
        || build.families.iter().all(|family| {
            !family.marginal
                && matches!(
                    family.marginality,
                    V4MarginalityDisposition::InternalIdentical { .. }
                        | V4MarginalityDisposition::InternalDerivable { .. }
                )
        });
    let stage2_constitutive_exception_adopted = false;
    let stage2_decision = if stage == 2 {
        "The sole typed family is predecessor-internal. No prefix-local constitutive theorem is adopted, so it is resolved as nonmarginal and mints zero ordinary semantic credit."
    } else {
        "not_applicable"
    }
    .to_owned();
    let stage9_boundary_case_applicable = stage == 9;
    let stage9_map_role_declaration_count = resolutions
        .iter()
        .filter(|resolution| resolution.occurrence.kind.starts_with("map_"))
        .count();
    let stage9_all_map_roles_resolved = stage != 9
        || (stage9_map_role_declaration_count > 0
            && resolutions
                .iter()
                .filter(|resolution| resolution.occurrence.kind.starts_with("map_"))
                .all(|resolution| resolution.resolved && !resolution.silent_residue));
    let surviving_occurrence_memberships = build
        .families
        .iter()
        .flat_map(|family| family.surviving_parent_membership_ids.iter().cloned())
        .collect::<BTreeSet<_>>();
    let removed_occurrence_memberships = build
        .r2_removed_occurrence_hashes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    // Both sides are occurrence-membership IDs.  Comparing a removed
    // occurrence hash with a family hash would be vacuous because those are
    // different namespaces.
    let r2_removed_occurrences_emitted_as_families = surviving_occurrence_memberships
        .intersection(&removed_occurrence_memberships)
        .count();
    let r2_removed_occurrences_absent_from_unified_membership =
        r2_removed_occurrences_emitted_as_families == 0;
    let r2_generated_instance_multiplied = r2_removed_occurrences_emitted_as_families != 0
        || !r2_removed_occurrences_absent_from_unified_membership
        || !build.r2_parent_membership_replayed;
    let mut audit = V4SpecialCaseAudit {
        stage,
        generic_r1: build.generic_r1.clone(),
        stage2_constitutive_question_applicable,
        stage2_family_internal_under_typed_predecessor_closure,
        stage2_constitutive_exception_adopted,
        stage2_decision,
        stage9_boundary_case_applicable,
        stage9_map_role_declaration_count,
        stage9_all_map_roles_resolved,
        r2_generated_instance_removed_count: build.r2_removed_occurrence_hashes.len(),
        r2_removed_occurrence_hashes: build.r2_removed_occurrence_hashes.clone(),
        r2_parent_slot_label_join_holds: build.r2_parent_slot_label_join_holds,
        r2_exact_v3_occurrence_surface_holds: build.r2_exact_v3_occurrence_surface_holds,
        r2_step8_typed_signature_derivation_hash: build
            .r2_step8_typed_signature_derivation_hash
            .clone(),
        r2_m1_generated_membership_derivation_hash: build
            .r2_m1_generated_membership_derivation_hash
            .clone(),
        r2_m1_generated_membership_replayed: build.r2_m1_generated_membership_replayed,
        r2_parent_membership_replayed_before_anchoring: build.r2_parent_membership_replayed,
        r2_removed_occurrences_absent_from_unified_membership,
        r2_removed_occurrences_emitted_as_families,
        r2_generated_instance_multiplied,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("special-case-audit", &audit);
    audit
}

fn issue_one(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
) -> Result<ActLocalSemanticProvenanceV4Certificate, ActLocalSemanticProvenanceV4Error> {
    replay_nu_register_adjudication()?;
    let expected_v3 = issue_act_local_provenance_v3(prefix, stage, candidate)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Input(error.to_string()))?;
    if &expected_v3 != v3
        || v3.schema != ACT_LOCAL_PROVENANCE_V3_SCHEMA
        || v3.stage != stage
        || v3.candidate_hash != candidate_hash(candidate)
        || v3.predecessor_signature_digest != prefix.digest()
    {
        return Err(ActLocalSemanticProvenanceV4Error::Input(
            "frozen v3 declaration ledger did not replay exactly".to_owned(),
        ));
    }
    let elaboration = elaborate_telescope(prefix, candidate, stage.saturating_sub(1))
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let closure = predecessor_closure(prefix)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Family(error.to_string()))?;
    let extraction =
        match extract_candidate_families(prefix, &closure, candidate, stage.saturating_sub(1)) {
            CandidateExtractionOutcome::Extracted(extraction) => extraction,
            CandidateExtractionOutcome::KernelInvalid { failure } => {
                return Err(ActLocalSemanticProvenanceV4Error::Family(
                    failure.to_string(),
                ));
            }
        };
    let exact_a3 = exact_a3_inventory(prefix, stage)?;
    if !exact_a3.complete {
        return Err(ActLocalSemanticProvenanceV4Error::Family(
            "exact-prefix A3 inventory did not close".to_owned(),
        ));
    }
    let mut build =
        build_unified_families(prefix, stage, candidate, &elaboration, &extraction, v3)?;
    let role_resolutions =
        resolve_role_declarations(stage, v3, &exact_a3, &extraction, &mut build)?;
    anchor_unified_families(
        stage,
        &candidate_hash(candidate),
        &exact_a3,
        &mut build.families,
        &role_resolutions,
    )?;
    for family in &mut build.families {
        family.derivation_hash = tagged_hash("unified-semantic-family", family);
    }
    let special_cases = special_case_audit(stage, &build, &role_resolutions);

    let role_gap_count = v3
        .theorem_gaps
        .iter()
        .filter(|gap| gap.kind == "ROLE_SCHEMA_EXTRACTION_GAP")
        .count();
    let proved_role_declaration_count = role_resolutions
        .iter()
        .filter(|resolution| matches!(resolution.resolution, V4RoleResolution::ProvedFamily { .. }))
        .count();
    let impossible_role_declaration_count = role_resolutions
        .iter()
        .filter(|resolution| {
            matches!(
                resolution.resolution,
                V4RoleResolution::TheoremBackedImpossibility { .. }
            )
        })
        .count();
    let named_role_residual_count = role_resolutions
        .iter()
        .filter(|resolution| {
            matches!(
                resolution.resolution,
                V4RoleResolution::NamedRegistryResidual { .. }
            )
        })
        .count();
    let resolved_role_declaration_count = role_resolutions
        .iter()
        .filter(|resolution| resolution.resolved)
        .count();
    let silent_role_residue_count = role_resolutions
        .iter()
        .filter(|resolution| resolution.silent_residue)
        .count()
        + role_gap_count.saturating_sub(role_resolutions.len());
    let marginal_unified_family_count = build
        .families
        .iter()
        .filter(|family| family.marginal)
        .count();
    let credited_semantic_family_count = build
        .families
        .iter()
        .filter(|family| family.credited)
        .count();
    let theorem_anchor_impossibility_count = build
        .families
        .iter()
        .filter(|family| {
            matches!(
                family.anchor,
                V4AnchorDisposition::ImpossibleCollision { .. }
            )
        })
        .count();
    let uncredited_anchor_collision_count = build
        .families
        .iter()
        .filter(|family| {
            matches!(
                family.anchor,
                V4AnchorDisposition::UncreditedAnchorCollision { .. }
            )
        })
        .count();
    let named_anchor_residual_count = build
        .families
        .iter()
        .filter(|family| matches!(family.anchor, V4AnchorDisposition::NamedResidual { .. }))
        .count();
    let credited_slots = build
        .families
        .iter()
        .filter_map(|family| match family.anchor {
            V4AnchorDisposition::CreditedLocalRole { clause, role, .. } => Some((clause, role)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let local_anchor_nonreuse_holds = credited_slots
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .len()
        == credited_slots.len();
    let a3_output_nonreuse_holds = exact_a3.usable_credit_position_count == 0;
    let uniform_instances_not_multiplied = build
        .path_quotient
        .as_ref()
        .map_or(true, |proof| proof.no_uniform_coordinate_multiplied)
        && v3.uniform_specializations_not_multiplied;
    let role_registry = HISTORICAL_ROLE_KINDS.into_iter().collect::<BTreeSet<_>>();
    let role_registry_exhaustive = role_resolutions
        .iter()
        .all(|resolution| role_registry.contains(resolution.occurrence.kind.as_str()));
    let every_role_gap_resolved = role_resolutions.len() == role_gap_count
        && resolved_role_declaration_count == role_gap_count
        && silent_role_residue_count == 0;
    let every_marginal_family_credited_or_theorem_impossible =
        build.families.iter().all(|family| {
            !family.marginal
                || matches!(
                    family.anchor,
                    V4AnchorDisposition::CreditedLocalRole { .. }
                        | V4AnchorDisposition::ImpossibleCollision { .. }
                )
        });
    let r2_parent_membership_replayed = special_cases
        .r2_parent_membership_replayed_before_anchoring
        && special_cases.r2_parent_slot_label_join_holds
        && special_cases.r2_exact_v3_occurrence_surface_holds
        && special_cases.r2_m1_generated_membership_replayed
        && special_cases.r2_removed_occurrences_absent_from_unified_membership
        && special_cases.r2_removed_occurrences_emitted_as_families == 0
        && !special_cases.r2_generated_instance_multiplied;
    let term_evidence_complete = build.families.iter().all(|family| {
        family.term.typing_replayed
            && family.term.normalization_replayed
            && family.term.naturality_replayed
            && family.parent_membership_replayed
    });
    let path_complete = build
        .path_quotient
        .as_ref()
        .map_or(true, |proof| proof.complete);
    let generic_r1_complete = special_cases
        .generic_r1
        .as_ref()
        .map_or(true, |proof| proof.proved);
    let ordinary_registry_complete = build.ordinary_registry.complete_as_registry_query;
    let every_applicable_ordinary_family_bridged = build.ordinary_registry.applicable_schema_count
        == build.ordinary_registry.exact_candidate_bridge_count;
    let stage2_complete = !special_cases.stage2_constitutive_question_applicable
        || (special_cases.stage2_family_internal_under_typed_predecessor_closure
            && !special_cases.stage2_constitutive_exception_adopted
            && credited_semantic_family_count == 0);
    let stage9_complete = !special_cases.stage9_boundary_case_applicable
        || special_cases.stage9_all_map_roles_resolved;
    let archive_read = false;
    let structural_nu_read = false;
    let bar_read = false;
    let verdict_read = false;
    let enacted_future_read = false;
    let authoritative_semantic_extraction = exact_a3.complete
        && term_evidence_complete
        && path_complete
        && generic_r1_complete
        && ordinary_registry_complete
        && every_applicable_ordinary_family_bridged
        && role_registry_exhaustive
        && every_role_gap_resolved
        && every_marginal_family_credited_or_theorem_impossible
        && local_anchor_nonreuse_holds
        && a3_output_nonreuse_holds
        && uniform_instances_not_multiplied
        && r2_parent_membership_replayed
        && stage2_complete
        && stage9_complete
        && !archive_read
        && !structural_nu_read
        && !bar_read
        && !verdict_read
        && !enacted_future_read;
    let mut certificate = ActLocalSemanticProvenanceV4Certificate {
        schema: ACT_LOCAL_SEMANTIC_PROVENANCE_V4_SCHEMA.to_owned(),
        date: ACT_LOCAL_SEMANTIC_PROVENANCE_V4_DATE.to_owned(),
        theorem_id: T_BI_NU1_V4_THEOREM_ID.to_owned(),
        stage,
        candidate_hash: candidate_hash(candidate),
        predecessor_signature_digest: prefix.digest().to_owned(),
        kappa: candidate.kappa() as u32,
        candidate_elaboration_hash: elaboration.derivation_hash,
        candidate_family_extraction_hash: extraction.derivation_hash,
        frozen_v3_schema: v3.schema.clone(),
        frozen_v3_package_hash: v3.derivation_hash.clone(),
        exact_a3,
        ordinary_registry: build.ordinary_registry,
        path_quotient: build.path_quotient,
        unified_families: build.families,
        role_resolutions,
        v3_role_schema_gap_count: role_gap_count,
        proved_role_declaration_count,
        impossible_role_declaration_count,
        named_role_residual_count,
        resolved_role_declaration_count,
        silent_role_residue_count,
        marginal_unified_family_count,
        credited_semantic_family_count,
        theorem_anchor_impossibility_count,
        uncredited_anchor_collision_count,
        named_anchor_residual_count,
        semantic_family_nu: credited_semantic_family_count as u32,
        local_anchor_nonreuse_holds,
        a3_output_nonreuse_holds,
        uniform_instances_not_multiplied,
        r2_parent_membership_replayed,
        role_registry_exhaustive,
        every_role_gap_resolved,
        every_marginal_family_credited_or_theorem_impossible,
        special_cases,
        archive_read,
        structural_nu_read,
        bar_read,
        verdict_read,
        enacted_future_read,
        authoritative_semantic_extraction,
        derivation_hash: String::new(),
    };
    certificate.derivation_hash = certificate_digest(&certificate);
    Ok(certificate)
}

/// Issue the semantic registry package for one act.  The supplied v3 value is
/// only a declaration ledger and must replay byte-for-byte from the same act
/// and exact prefix before it can be consumed.
pub fn issue_act_local_semantic_provenance_v4(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
) -> Result<ActLocalSemanticProvenanceV4Certificate, ActLocalSemanticProvenanceV4Error> {
    issue_one(prefix, stage, candidate, v3)
}

/// Issue a contiguous semantic history.  For the historical fifteen-act
/// surface, the global gate is exact: all 250 v3 declarations and all 27
/// frozen role kinds must be classified without silent residue, and Step 8's
/// sole R2 child must remain absent from the credited-family floor.  Named
/// residuals are returned as a negative theorem result rather than converted
/// into impossibilities.
pub fn issue_act_local_semantic_sequence_v4(
    entries: &[(u32, Telescope)],
) -> Result<Vec<ActLocalSemanticProvenanceV4Certificate>, ActLocalSemanticProvenanceV4Error> {
    if entries.is_empty()
        || !entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=entries.len() as u32)
    {
        return Err(ActLocalSemanticProvenanceV4Error::Input(
            "v4 semantic sequence must be contiguous from Stage 1".to_owned(),
        ));
    }
    let v3_packages = issue_act_local_sequence_v3(entries)
        .map_err(|error| ActLocalSemanticProvenanceV4Error::Input(error.to_string()))?;
    let mut accepted = Vec::<(u32, Telescope)>::new();
    let mut packages = Vec::new();
    for ((stage, candidate), v3) in entries.iter().zip(&v3_packages) {
        let prefix = SealedSignature::from_telescopes(accepted.clone());
        let package = issue_one(&prefix, *stage, candidate, v3)?;
        accepted.push((*stage, candidate.clone()));
        packages.push(package);
    }
    if entries.len() == 15 {
        let gap_total = packages
            .iter()
            .map(|package| package.v3_role_schema_gap_count)
            .sum::<usize>();
        let classified_total = packages
            .iter()
            .map(|package| package.role_resolutions.len())
            .sum::<usize>();
        let observed_kinds = packages
            .iter()
            .flat_map(|package| {
                package
                    .role_resolutions
                    .iter()
                    .map(|resolution| resolution.occurrence.kind.as_str())
            })
            .collect::<BTreeSet<_>>();
        let expected_kinds = HISTORICAL_ROLE_KINDS.into_iter().collect::<BTreeSet<_>>();
        let r2_removed_total = packages
            .iter()
            .map(|package| package.special_cases.r2_generated_instance_removed_count)
            .sum::<usize>();
        if gap_total != 250
            || classified_total != 250
            || observed_kinds != expected_kinds
            || r2_removed_total != 1
            || packages.iter().any(|package| {
                package.silent_role_residue_count != 0 || !package.r2_parent_membership_replayed
            })
        {
            return Err(ActLocalSemanticProvenanceV4Error::Invariant(format!(
                "historical registry enumeration failed: gaps={gap_total}, classified={classified_total}, kinds={}/{}, R2-removed={r2_removed_total}",
                observed_kinds.len(),
                expected_kinds.len(),
            )));
        }
    }
    Ok(packages)
}

pub fn replay_act_local_semantic_provenance_v4(
    prefix: &SealedSignature,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
    claimed: &ActLocalSemanticProvenanceV4Certificate,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != certificate_digest(claimed) {
        errors.push("v4 semantic package digest mismatch".to_owned());
    }
    match issue_one(prefix, claimed.stage, candidate, v3) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("v4 semantic package differs from create-new reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

pub fn issue_reference_act_local_semantic_sequence_v4()
-> Result<Vec<ActLocalSemanticProvenanceV4Certificate>, ActLocalSemanticProvenanceV4Error> {
    let entries = (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    issue_act_local_semantic_sequence_v4(&entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prefix_local_inputs(
        stage: u32,
    ) -> (SealedSignature, Telescope, ActLocalProvenanceV3Certificate) {
        let prefix = SealedSignature::from_telescopes(
            (1..stage)
                .map(|prior| (prior, Telescope::reference(prior)))
                .collect(),
        );
        let candidate = Telescope::reference(stage);
        let v3 = issue_act_local_provenance_v3(&prefix, stage, &candidate)
            .expect("prefix-local v3 package");
        (prefix, candidate, v3)
    }

    fn historical_inputs(
        stage: u32,
    ) -> (
        SealedSignature,
        Telescope,
        ActLocalProvenanceV3Certificate,
        ActLocalSemanticProvenanceV4Certificate,
    ) {
        let entries = (1..=15)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let v3 = issue_act_local_sequence_v3(&entries).expect("v3 declaration ledgers");
        let packages = issue_act_local_semantic_sequence_v4(&entries).expect("v4 packages");
        let prefix = SealedSignature::from_telescopes(
            entries
                .iter()
                .take(stage.saturating_sub(1) as usize)
                .cloned()
                .collect(),
        );
        (
            prefix,
            entries[(stage - 1) as usize].1.clone(),
            v3[(stage - 1) as usize].clone(),
            packages[(stage - 1) as usize].clone(),
        )
    }

    #[test]
    fn prefix_local_source_surface_reissues_without_role_registry_or_anchor_fields() {
        let (prefix, candidate, v3) = prefix_local_inputs(4);
        let source = issue_v4_prefix_local_source_surface(&prefix, 4, &candidate, &v3)
            .expect("prefix-local v4 source");
        assert!(source.source_surface_complete);
        assert!(source.v3_package_replayed);
        assert!(source.candidate_extraction_covers_every_clause);
        assert!(source.unified_family_ids_unique);
        assert!(source.every_family_is_pre_anchor);
        assert!(!source.historical_kind_surface_read);
        assert!(!source.declaration_resolution_read);
        assert!(!source.anchor_assignment_read);
        assert!(
            replay_v4_prefix_local_source_surface(&prefix, &candidate, &v3, &source).is_empty()
        );

        let json = serde_json::to_value(&source).expect("source serializes");
        let encoded = serde_json::to_string(&json).expect("source JSON serializes");
        for forbidden in [
            "role_resolutions",
            "role_registry_entry",
            "role_route_classification_hash",
            "registry_index",
            "registry_size",
            "historical_role_kinds",
            "role_term_registry",
        ] {
            assert!(!encoded.contains(forbidden), "forbidden field {forbidden}");
        }
        for family in json["unified_families"].as_array().expect("family array") {
            assert!(family.get("anchor").is_none());
            assert!(family.get("credited").is_none());
            assert!(family.get("role_declaration_ids").is_none());
        }
    }

    #[test]
    fn prefix_local_source_carries_generic_r1_and_r2_premises_at_their_own_stages() {
        let (prefix1, candidate1, v3_1) = prefix_local_inputs(1);
        let stage1 = issue_v4_prefix_local_source_surface(&prefix1, 1, &candidate1, &v3_1)
            .expect("Stage-1 source");
        let r1 = stage1
            .r1_r2_premises
            .generic_r1
            .as_ref()
            .expect("generic R1 premises");
        assert!(
            stage1
                .r1_r2_premises
                .generic_r1_typed_package_premises_replayed
        );
        assert!(r1.adopted_package_clause_replayed);
        assert!(r1.completed_action_covers_carrier_by_adopted_r1);

        let (prefix8, candidate8, v3_8) = prefix_local_inputs(8);
        let stage8 = issue_v4_prefix_local_source_surface(&prefix8, 8, &candidate8, &v3_8)
            .expect("Stage-8 source");
        assert_eq!(stage8.r1_r2_premises.r2_generated_instance_removed_count, 1);
        assert!(stage8.r1_r2_premises.r2_local_premises_replayed);
        assert!(
            stage8
                .r1_r2_premises
                .r2_step8_typed_signature_derivation_hash
                .is_some()
        );
        assert!(
            stage8
                .r1_r2_premises
                .r2_m1_generated_membership_derivation_hash
                .is_some()
        );
        assert_eq!(
            stage8
                .r1_r2_premises
                .r2_removed_occurrences_emitted_as_families,
            0
        );
        assert!(!stage8.r1_r2_premises.r2_generated_instance_multiplied);
    }

    #[test]
    fn prefix_local_source_is_additive_and_rejects_a_fully_rehashed_mutation() {
        let (prefix, candidate, v3) = prefix_local_inputs(4);
        let legacy_before = issue_act_local_semantic_provenance_v4(&prefix, 4, &candidate, &v3)
            .expect("legacy v4 before source issuance");
        let source = issue_v4_prefix_local_source_surface(&prefix, 4, &candidate, &v3)
            .expect("prefix-local source");
        let legacy_after = issue_act_local_semantic_provenance_v4(&prefix, 4, &candidate, &v3)
            .expect("legacy v4 after source issuance");
        assert_eq!(legacy_before, legacy_after);

        let mut forged = source.clone();
        forged.candidate_family_extraction_hash.push_str("-forged");
        forged.source_hash = prefix_local_source_surface_hash(&forged);
        let errors = replay_v4_prefix_local_source_surface(&prefix, &candidate, &v3, &forged);
        assert!(errors.iter().any(|error| error.contains("reissuance")));
        assert!(!errors.iter().any(|error| error.contains("digest mismatch")));
    }

    #[test]
    fn unified_registry_names_all_residuals_without_silent_residue() {
        let packages = issue_reference_act_local_semantic_sequence_v4().expect("v4 sequence");
        assert_eq!(packages.len(), 15);
        assert_eq!(
            packages
                .iter()
                .map(|package| package.v3_role_schema_gap_count)
                .sum::<usize>(),
            250
        );
        assert_eq!(
            packages
                .iter()
                .map(|package| package.role_resolutions.len())
                .sum::<usize>(),
            250
        );
        assert_eq!(
            packages
                .iter()
                .map(|package| package.named_role_residual_count)
                .sum::<usize>(),
            250
        );
        assert_eq!(
            packages
                .iter()
                .map(|package| package.proved_role_declaration_count)
                .sum::<usize>(),
            0
        );
        assert!(packages.iter().all(|package| {
            package.silent_role_residue_count == 0 && package.r2_parent_membership_replayed
        }));
        assert!(
            packages
                .iter()
                .any(|package| !package.authoritative_semantic_extraction)
        );
        assert_eq!(
            packages
                .iter()
                .map(|package| package.semantic_family_nu)
                .collect::<Vec<_>>(),
            vec![0; 15]
        );
        assert_eq!(
            packages
                .iter()
                .map(|package| package.theorem_anchor_impossibility_count)
                .sum::<usize>(),
            0
        );
        assert!(
            packages
                .iter()
                .any(|package| package.named_anchor_residual_count > 0)
        );
        assert!(packages.iter().all(|package| {
            package.marginal_unified_family_count == 0
                || !package.every_marginal_family_credited_or_theorem_impossible
        }));
        assert!(
            packages[0]
                .special_cases
                .generic_r1
                .as_ref()
                .is_some_and(|proof| {
                    !proof.proved
                        && proof.carrier_role_surface_gap
                            == "T_BI_NU1_R1_CARRIER_ROLE_SURFACE_INCOMPLETE_C1"
                })
        );
        assert!(packages[4..=7].iter().all(|package| {
            package.ordinary_registry.all_nine_constructors_queried_once
                && package.ordinary_registry.applicable_schema_count > 0
                && package.ordinary_registry.exact_candidate_bridge_count == 0
        }));
    }

    #[test]
    fn label_only_family_candidates_never_issue_proved_family() {
        let packages = issue_reference_act_local_semantic_sequence_v4().expect("v4 sequence");
        let label_candidate_resolutions = packages
            .iter()
            .flat_map(|package| &package.role_resolutions)
            .filter(|resolution| {
                resolution
                    .role_registry_entry
                    .get("label_only_core_candidates")
                    .and_then(Value::as_array)
                    .is_some_and(|values| !values.is_empty())
                    || resolution
                        .role_registry_entry
                        .get("label_only_cubical_candidates")
                        .and_then(Value::as_array)
                        .is_some_and(|values| !values.is_empty())
            })
            .collect::<Vec<_>>();
        assert!(!label_candidate_resolutions.is_empty());
        assert!(label_candidate_resolutions.iter().all(|resolution| {
            matches!(
                resolution.resolution,
                V4RoleResolution::NamedRegistryResidual { .. }
            ) && resolution
                .role_registry_entry
                .get("exact_term_relation_theorems")
                .and_then(Value::as_array)
                .is_some_and(Vec::is_empty)
        }));
    }

    #[test]
    fn r2_occurrence_memberships_share_a_namespace_without_identifying_parent_and_child() {
        let (_prefix, _candidate, v3, package) = historical_inputs(8);
        let removed = v3
            .role_occurrences_before_quotient
            .iter()
            .find(|occurrence| occurrence.removed_by_adopted_r2)
            .expect("Stage 8 removed child");
        let removed_id = role_occurrence_membership_id(8, removed);
        let removed_slot = r2_parent_slot_id(8, removed);
        let parents = v3
            .natural_family_rows
            .iter()
            .filter(|row| r2_parent_slot_id(8, &row.representative_role) == removed_slot)
            .collect::<Vec<_>>();
        assert_eq!(parents.len(), 1);
        let parent_id = role_occurrence_membership_id(8, &parents[0].representative_role);
        assert_ne!(parent_id, removed_id);
        assert_eq!(
            package.special_cases.r2_removed_occurrence_hashes,
            vec![removed_id.clone()]
        );
        assert!(package.unified_families.iter().any(|family| {
            family.surviving_parent_membership_ids.contains(&parent_id)
                && family
                    .r2_removed_child_occurrence_hashes
                    .contains(&removed_id)
        }));
        assert!(
            package
                .special_cases
                .r2_removed_occurrences_absent_from_unified_membership
        );
        assert_eq!(
            package
                .special_cases
                .r2_removed_occurrences_emitted_as_families,
            0
        );
        assert!(package.special_cases.r2_parent_slot_label_join_holds);
        assert!(package.special_cases.r2_exact_v3_occurrence_surface_holds);
        assert!(
            package
                .special_cases
                .r2_step8_typed_signature_derivation_hash
                .is_some()
        );
        assert!(
            package
                .special_cases
                .r2_m1_generated_membership_derivation_hash
                .is_some()
        );
        assert!(package.special_cases.r2_m1_generated_membership_replayed);
        assert!(package.r2_parent_membership_replayed);
    }

    #[test]
    fn resigning_an_r2_child_membership_restoration_fails_reissue() {
        let (prefix, candidate, v3, package) = historical_inputs(8);
        assert_eq!(package.special_cases.r2_generated_instance_removed_count, 1);
        let mut forged = package.clone();
        let removed_ids = forged.special_cases.r2_removed_occurrence_hashes.clone();
        let restored = forged
            .unified_families
            .iter_mut()
            .find(|family| !family.r2_removed_child_occurrence_hashes.is_empty())
            .expect("Stage 8 has a family carrying the R2 removal evidence");
        restored.surviving_parent_membership_ids.extend(removed_ids);
        restored.surviving_parent_membership_ids.sort();
        restored.surviving_parent_membership_ids.dedup();
        restored.r2_removed_child_occurrence_hashes.clear();
        restored.derivation_hash = tagged_hash("unified-semantic-family", restored);
        forged.derivation_hash = certificate_digest(&forged);
        assert!(
            replay_act_local_semantic_provenance_v4(&prefix, &candidate, &v3, &forged)
                .iter()
                .any(|error| error.contains("reissuance"))
        );
    }

    #[test]
    fn resigned_path_key_and_equality_mutations_fail_reissue() {
        let (prefix, candidate, v3, package) = historical_inputs(6);
        let mut forged = package.clone();
        let quotient = forged
            .path_quotient
            .as_mut()
            .expect("Stage 6 path quotient");
        quotient.observed_key_hashes[0].push_str("-mutated");
        quotient.pairwise_decisions[0] = json!({"decision": "forged_equal"});
        quotient.derivation_hash = tagged_hash("path-quotient-proof", quotient);
        forged.derivation_hash = certificate_digest(&forged);
        assert!(
            !replay_act_local_semantic_provenance_v4(&prefix, &candidate, &v3, &forged).is_empty()
        );
    }

    #[test]
    fn resigned_r1_dependency_mutation_fails_reissue() {
        let (prefix, candidate, v3, package) = historical_inputs(1);
        let mut forged = package.clone();
        let proof = forged
            .special_cases
            .generic_r1
            .as_mut()
            .expect("Stage 1 generic R1 proof");
        proof.completion_dependency_level += 1;
        proof.dependency_resolves_to_carrier = false;
        proof.proved = false;
        proof.derivation_hash = tagged_hash("generic-R1-proof", proof);
        forged.special_cases.derivation_hash =
            tagged_hash("special-case-audit", &forged.special_cases);
        forged.derivation_hash = certificate_digest(&forged);
        assert!(
            !replay_act_local_semantic_provenance_v4(&prefix, &candidate, &v3, &forged).is_empty()
        );
    }

    #[test]
    fn resigned_role_kind_and_route_mutation_fails_reissue() {
        let (prefix, candidate, v3, package) = historical_inputs(9);
        let mut forged = package.clone();
        let resolution = forged
            .role_resolutions
            .first_mut()
            .expect("Stage 9 role resolution");
        resolution.occurrence.kind = "foundation_formation".to_owned();
        resolution.role_registry_entry["classified_route"] = json!("direct_clause");
        resolution.role_route_classification_hash = tagged_hash(
            "forged-role-route-classification",
            &resolution.role_registry_entry,
        );
        resolution.derivation_hash = tagged_hash("role-declaration-resolution", resolution);
        forged.derivation_hash = certificate_digest(&forged);
        assert!(
            !replay_act_local_semantic_provenance_v4(&prefix, &candidate, &v3, &forged).is_empty()
        );
    }
}
