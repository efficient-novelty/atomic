//! Additive T-BI-B1/B2 successor to the frozen v4 semantic audit.
//!
//! V4 remains a replayed negative predecessor.  This module does not change
//! any of its C1 flags.  Instead it proves an external, candidate-and-prefix
//! local bridge from kernel-normal `pen_core::Expr` families to the already
//! typed ordinary Schema2 instances, and then performs a finite constructor
//! induction for the exact family-to-local-role relation.  Serialized role
//! labels are query keys only: a positive result additionally carries the
//! reconstructed term/schema bridge or cubical projection that proves it.

use crate::act_local_provenance_v3::{
    ActLocalProvenanceV3Certificate, ActLocalV3RoleOccurrence, issue_act_local_sequence_v3,
};
use crate::act_local_semantic_provenance_v4::{
    ActLocalSemanticProvenanceV4Certificate, HISTORICAL_ROLE_KINDS, V4FamilySource,
    V4GenericR1Proof, V4MarginalityDisposition, V4OrdinaryRegistryAudit,
    V4PathQuotientProof, V4PrefixLocalSourceSurface, issue_act_local_semantic_sequence_v4,
    issue_v4_prefix_local_source_surface, replay_act_local_semantic_provenance_v4,
    replay_v4_prefix_local_source_surface,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3DemandOutputType, A3RuleConstructor, generate_a3_window_for_exact_prefix_unbounded,
};
use pen_eval::a3_rule_inventory_exhaustiveness::prove_a3_window_inventory_for_exact_prefix_unbounded;
use pen_eval::future_hole_hypothesis_v2::{
    FutureHoleBodyV2, FutureHoleOutputContractV2, FutureHoleRegistrationDispositionV2,
    register_structural_future_hole_v2, replay_future_hole_registration_v2,
};
use pen_eval::semantic_provenance::{CreditMechanism, LocalRole};
use pen_eval::typed_families::{
    CandidateExtractionOutcome, ExtractedFamily, InstanceKind,
    MarginalityDisposition as TypedMarginalityDisposition, PredecessorClosure,
    extract_candidate_families, predecessor_closure,
};
use pen_type::elaborate::{KernelTy, SealedSignature, candidate_hash, elaborate_telescope};
use pen_type::equality::univalent_equality;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use thiserror::Error;

pub const ACT_LOCAL_SEMANTIC_PROVENANCE_V5_SCHEMA: &str = "act-local-semantic-family-provenance-v5";
pub const ACT_LOCAL_SEMANTIC_PROVENANCE_V5_DATE: &str = "2026-07-22";
pub const T_BI_B1_THEOREM_ID: &str = "T-BI-B1-expr-to-ordinary-schema2-bridge-v1";
pub const T_BI_B2_THEOREM_ID: &str = "T-BI-B2-exact-family-local-role-relation-v1";
pub const T_BI_B2_CLOSURE_ID: &str =
    "T-BI-B2-core-ordinary-cubical-A3-finite-constructor-induction-v1";
pub const T_BI_B2_ROLE_KIND_ID: &str = "T-BI-B2-27-kind-no-default-constructor-search-v2";
pub const T_BI_B2_A3_CAPABILITY_ID: &str = "T-BI-B2-exact-prefix-A3-output-capability-v2";
pub const T_BI_B2_UNIFIED_QUOTIENT_ID: &str =
    "T-BI-B2-core-ordinary-cubical-unified-equality-quotient-v2";
pub const T_BI_B2_EQUIVALENCE_CLOSURE_ID: &str =
    "T-BI-B2-closed-unified-equivalence-rule-induction-v1";
pub const T_BI_B2_STAGE1_R1_ID: &str = "T-BI-B2-stage1-formation-completion-four-role-cases-v2";
pub const T_BI_B2_PREFIX_LOCAL_REGISTRY_ERASURE_ID: &str =
    "T-BI-B2-prefix-local-role-registry-extension-erasure-v1";
pub const V5_PREFIX_LOCAL_SEMANTIC_SEQUENCE_SCHEMA: &str =
    "act-local-semantic-family-provenance-prefix-local-v1";
pub const T_BI_B1_B2_PREFIX_LOCAL_THEOREM_ID: &str =
    "T-BI-B1-B2-prefix-local-source-first-registry-extension-invariant-v1";

const SUPPORT_COMPREHENSION_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/support_comprehension_adjudication.md");
const NU_REGISTER_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/nu_register_adjudication.md");

/// The semantic fields consumed by v5, separated from v4's historical role
/// registry.  Historical issuance creates this view from a full v4
/// certificate; the prefix-local issuer creates it from the source-first v4
/// surface that never constructs a role registry.
#[derive(Clone)]
struct V5SemanticSourceFamily {
    family_id: String,
    stage: u32,
    source: V4FamilySource,
    term: crate::act_local_semantic_provenance_v4::V4TermFamilyEvidence,
    marginality: V4MarginalityDisposition,
    marginal: bool,
    surviving_parent_membership_ids: Vec<String>,
    r2_removed_child_occurrence_hashes: Vec<String>,
    parent_membership_replayed: bool,
}

#[derive(Clone)]
struct V5SemanticSourceSpecialCases {
    generic_r1: Option<V4GenericR1Proof>,
    r2_generated_instance_removed_count: usize,
    r2_removed_occurrence_hashes: Vec<String>,
    r2_parent_slot_label_join_holds: bool,
    r2_exact_v3_occurrence_surface_holds: bool,
    r2_step8_typed_signature_derivation_hash: Option<String>,
    r2_m1_generated_membership_derivation_hash: Option<String>,
    r2_m1_generated_membership_replayed: bool,
    r2_parent_membership_replayed_before_anchoring: bool,
    r2_removed_occurrences_absent_from_unified_membership: bool,
    r2_removed_occurrences_emitted_as_families: usize,
    r2_generated_instance_multiplied: bool,
}

#[derive(Clone)]
struct V5SemanticSourceView {
    stage: u32,
    candidate_hash: String,
    predecessor_signature_digest: String,
    candidate_elaboration_hash: String,
    candidate_family_extraction_hash: String,
    source_derivation_hash: String,
    ordinary_registry: V4OrdinaryRegistryAudit,
    path_quotient: Option<V4PathQuotientProof>,
    unified_families: Vec<V5SemanticSourceFamily>,
    special_cases: V5SemanticSourceSpecialCases,
    role_registry_exhaustive: bool,
    role_schema_gap_count: usize,
    historical_registry_consulted: bool,
    archive_structural_bar_verdict_or_future_read: bool,
}

impl V5SemanticSourceView {
    fn historical(v4: &ActLocalSemanticProvenanceV4Certificate) -> Self {
        Self {
            stage: v4.stage,
            candidate_hash: v4.candidate_hash.clone(),
            predecessor_signature_digest: v4.predecessor_signature_digest.clone(),
            candidate_elaboration_hash: v4.candidate_elaboration_hash.clone(),
            candidate_family_extraction_hash: v4.candidate_family_extraction_hash.clone(),
            source_derivation_hash: v4.derivation_hash.clone(),
            ordinary_registry: v4.ordinary_registry.clone(),
            path_quotient: v4.path_quotient.clone(),
            unified_families: v4
                .unified_families
                .iter()
                .map(|family| V5SemanticSourceFamily {
                    family_id: family.family_id.clone(),
                    stage: family.stage,
                    source: family.source.clone(),
                    term: family.term.clone(),
                    marginality: family.marginality.clone(),
                    marginal: family.marginal,
                    surviving_parent_membership_ids: family
                        .surviving_parent_membership_ids
                        .clone(),
                    r2_removed_child_occurrence_hashes: family
                        .r2_removed_child_occurrence_hashes
                        .clone(),
                    parent_membership_replayed: family.parent_membership_replayed,
                })
                .collect(),
            special_cases: V5SemanticSourceSpecialCases {
                generic_r1: v4.special_cases.generic_r1.clone(),
                r2_generated_instance_removed_count: v4
                    .special_cases
                    .r2_generated_instance_removed_count,
                r2_removed_occurrence_hashes: v4
                    .special_cases
                    .r2_removed_occurrence_hashes
                    .clone(),
                r2_parent_slot_label_join_holds: v4
                    .special_cases
                    .r2_parent_slot_label_join_holds,
                r2_exact_v3_occurrence_surface_holds: v4
                    .special_cases
                    .r2_exact_v3_occurrence_surface_holds,
                r2_step8_typed_signature_derivation_hash: v4
                    .special_cases
                    .r2_step8_typed_signature_derivation_hash
                    .clone(),
                r2_m1_generated_membership_derivation_hash: v4
                    .special_cases
                    .r2_m1_generated_membership_derivation_hash
                    .clone(),
                r2_m1_generated_membership_replayed: v4
                    .special_cases
                    .r2_m1_generated_membership_replayed,
                r2_parent_membership_replayed_before_anchoring: v4
                    .special_cases
                    .r2_parent_membership_replayed_before_anchoring,
                r2_removed_occurrences_absent_from_unified_membership: v4
                    .special_cases
                    .r2_removed_occurrences_absent_from_unified_membership,
                r2_removed_occurrences_emitted_as_families: v4
                    .special_cases
                    .r2_removed_occurrences_emitted_as_families,
                r2_generated_instance_multiplied: v4
                    .special_cases
                    .r2_generated_instance_multiplied,
            },
            role_registry_exhaustive: v4.role_registry_exhaustive,
            role_schema_gap_count: v4.v3_role_schema_gap_count,
            historical_registry_consulted: true,
            archive_structural_bar_verdict_or_future_read: v4.archive_read
                || v4.structural_nu_read
                || v4.bar_read
                || v4.verdict_read
                || v4.enacted_future_read,
        }
    }

    fn prefix_local(source: &V4PrefixLocalSourceSurface, role_schema_gap_count: usize) -> Self {
        Self {
            stage: source.stage,
            candidate_hash: source.candidate_hash.clone(),
            predecessor_signature_digest: source.predecessor_signature_digest.clone(),
            candidate_elaboration_hash: source.candidate_elaboration_hash.clone(),
            candidate_family_extraction_hash: source.candidate_family_extraction_hash.clone(),
            source_derivation_hash: source.source_hash.clone(),
            ordinary_registry: source.ordinary_registry.clone(),
            path_quotient: source.path_quotient.clone(),
            unified_families: source
                .unified_families
                .iter()
                .map(|family| V5SemanticSourceFamily {
                    family_id: family.family_id.clone(),
                    stage: family.stage,
                    source: family.source.clone(),
                    term: family.term.clone(),
                    marginality: family.marginality.clone(),
                    marginal: family.marginal,
                    surviving_parent_membership_ids: family
                        .surviving_parent_membership_ids
                        .clone(),
                    r2_removed_child_occurrence_hashes: family
                        .r2_removed_child_occurrence_hashes
                        .clone(),
                    parent_membership_replayed: family.parent_membership_replayed,
                })
                .collect(),
            special_cases: V5SemanticSourceSpecialCases {
                generic_r1: source.r1_r2_premises.generic_r1.clone(),
                r2_generated_instance_removed_count: source
                    .r1_r2_premises
                    .r2_generated_instance_removed_count,
                r2_removed_occurrence_hashes: source
                    .r1_r2_premises
                    .r2_removed_occurrence_hashes
                    .clone(),
                r2_parent_slot_label_join_holds: source
                    .r1_r2_premises
                    .r2_parent_slot_label_join_holds,
                r2_exact_v3_occurrence_surface_holds: source
                    .r1_r2_premises
                    .r2_exact_v3_occurrence_surface_holds,
                r2_step8_typed_signature_derivation_hash: source
                    .r1_r2_premises
                    .r2_step8_typed_signature_derivation_hash
                    .clone(),
                r2_m1_generated_membership_derivation_hash: source
                    .r1_r2_premises
                    .r2_m1_generated_membership_derivation_hash
                    .clone(),
                r2_m1_generated_membership_replayed: source
                    .r1_r2_premises
                    .r2_m1_generated_membership_replayed,
                r2_parent_membership_replayed_before_anchoring: source
                    .r1_r2_premises
                    .r2_parent_membership_replayed,
                r2_removed_occurrences_absent_from_unified_membership: source
                    .r1_r2_premises
                    .r2_removed_occurrences_absent_from_unified_membership,
                r2_removed_occurrences_emitted_as_families: source
                    .r1_r2_premises
                    .r2_removed_occurrences_emitted_as_families,
                r2_generated_instance_multiplied: source
                    .r1_r2_premises
                    .r2_generated_instance_multiplied,
            },
            role_registry_exhaustive: true,
            role_schema_gap_count,
            historical_registry_consulted: source.historical_kind_surface_read
                || source.declaration_resolution_read
                || source.anchor_assignment_read,
            archive_structural_bar_verdict_or_future_read: source.archive_read
                || source.structural_nu_read
                || source.bar_read
                || source.verdict_read
                || source.enacted_future_read,
        }
    }
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(ACT_LOCAL_SEMANTIC_PROVENANCE_V5_SCHEMA, domain, value))
        .expect("v5 semantic proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V5OrdinaryKind {
    FreshFormation,
    PointOrUnitIntro,
    PathConstructorIntro,
    Recursor,
    Inductor,
    TruncParametricAction,
    PostPathOperation,
    PostPathCoherence,
    CellAction,
}

impl V5OrdinaryKind {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "fresh_formation" => Self::FreshFormation,
            "point_or_unit_intro" => Self::PointOrUnitIntro,
            "path_constructor_intro" => Self::PathConstructorIntro,
            "recursor" => Self::Recursor,
            "inductor" => Self::Inductor,
            "trunc_parametric_action" => Self::TruncParametricAction,
            "post_path_operation" => Self::PostPathOperation,
            "post_path_coherence" => Self::PostPathCoherence,
            "cell_action" => Self::CellAction,
            _ => return None,
        })
    }

    fn generated_instance(self) -> bool {
        matches!(self, Self::CellAction)
    }

    fn uses_operation_shape(self) -> bool {
        matches!(
            self,
            Self::PostPathOperation | Self::PostPathCoherence | Self::CellAction
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V5ExprSchemaBridgeRule {
    FormationFromKernelType,
    IntroductionFromKernelIntroduction,
    PathConstructorFromTypedPathDeclaration,
    RecursorFromTypedPathDeclaration,
    InductorFromTypedPathDeclaration,
    TruncActionFromTypedTruncFormation,
    PostPathOperationFromTypedPostClause,
    PostPathCoherenceFromTypedPostClause,
    CellActionFromTypedOperationAndPath,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5ExprSchemaBridgeProof {
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub kind: V5OrdinaryKind,
    pub bridge_rule: V5ExprSchemaBridgeRule,
    pub source_clauses: Vec<u16>,
    pub carrier_clause: u16,
    pub source_exprs: Vec<Expr>,
    pub source_normal_forms: Vec<Expr>,
    pub carrier_normal_form: Expr,
    pub source_core_family_ids: Vec<String>,
    pub source_typing_derivation_hashes: Vec<String>,
    pub ordinary_schema_derivation_hash: String,
    pub ordinary_typed_realizer_derivation_hash: String,
    pub ordinary_normalization_derivation_hash: String,
    pub ordinary_naturality_derivation_hash: String,
    pub ordinary_shape_key: String,
    pub semantic_family_id: String,
    pub legacy_v4_c1_flag_remains_false: bool,
    pub support_bound_to_exact_candidate: bool,
    pub source_family_membership_replayed: bool,
    pub constructor_shape_induction_holds: bool,
    pub typed_normalized_natural_replayed: bool,
    pub generated_instance: bool,
    pub zero_charge_support_only: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "source")]
pub enum V5SemanticFamilySource {
    FrozenV4 {
        v4_family_id: String,
    },
    OrdinaryBridge {
        bridge_derivation_hash: String,
        kind: V5OrdinaryKind,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V5UnifiedSurface {
    CoreExpr,
    OrdinarySchema2,
    CubicalPath,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "presentation")]
pub enum V5UnifiedFamilyPresentation {
    CoreExpr {
        extracted_family_id: String,
        canonical_normal_form: Expr,
        parameter_sorts: Value,
        generator_kernel_type: Value,
        generator_role: ClauseRole,
    },
    OrdinarySchema2 {
        kind: V5OrdinaryKind,
        ordinary_shape_key: String,
        carrier_normal_form: Expr,
        operation_normal_forms: Vec<Expr>,
        source_core_family_ids: Vec<String>,
    },
    CubicalPath {
        path_clause: u16,
        key: Value,
        normal_form: Value,
        inferred_type: Value,
    },
}

impl V5UnifiedFamilyPresentation {
    fn surface(&self) -> V5UnifiedSurface {
        match self {
            Self::CoreExpr { .. } => V5UnifiedSurface::CoreExpr,
            Self::OrdinarySchema2 { .. } => V5UnifiedSurface::OrdinarySchema2,
            Self::CubicalPath { .. } => V5UnifiedSurface::CubicalPath,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "ledger_disposition")]
pub enum V5UnifiedMemberDisposition {
    Active,
    CoreReplacedByOrdinaryBridge { bridge_family_ids: Vec<String> },
    GeneratedUniformInstance,
    EqualWithinCandidate { representative_family_id: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5UnifiedSurfaceMember {
    pub family_id: String,
    pub stage: u32,
    pub source_clauses: Vec<u16>,
    pub presentation: V5UnifiedFamilyPresentation,
    pub presentation_hash: String,
    pub typing_normalization_naturality_hash: String,
    pub typed_normalized_natural: bool,
    pub disposition: V5UnifiedMemberDisposition,
    pub included_in_semantic_ledger: bool,
    pub visible_to_later_predecessor_closure: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "relation")]
pub enum V5UnifiedEqualityRelation {
    Equal,
    Distinct,
    NamedResidual {
        gap_id: String,
        exact_reason: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5UnifiedEqualityDecision {
    pub left_stage: u32,
    pub left_family_id: String,
    pub right_stage: u32,
    pub right_family_id: String,
    pub left_surface: V5UnifiedSurface,
    pub right_surface: V5UnifiedSurface,
    pub relation: V5UnifiedEqualityRelation,
    pub procedure: String,
    pub evidence: Value,
    pub v4_marginality_label_used_as_proof: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

/// The complete set of *base* equality constructors used by the unified
/// quotient.  Reflexivity, symmetry, and transitivity are supplied only by
/// the finite equivalence closure below.  Consequently the exhaustive match
/// in `V5UnifiedEquivalenceBaseRule::endpoint_surfaces` is also the promised
/// induction over every way an equality edge can enter the closure.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V5UnifiedEquivalenceBaseRule {
    CoreUnivalentEquality,
    OrdinaryNormalizedShapeEquality,
    CubicalTypedEquality,
    ExactExprSchema2Bridge,
}

impl V5UnifiedEquivalenceBaseRule {
    fn closed_registry() -> Vec<Self> {
        vec![
            Self::CoreUnivalentEquality,
            Self::OrdinaryNormalizedShapeEquality,
            Self::CubicalTypedEquality,
            Self::ExactExprSchema2Bridge,
        ]
    }

    fn endpoint_surfaces(self) -> (V5UnifiedSurface, V5UnifiedSurface) {
        match self {
            Self::CoreUnivalentEquality => (V5UnifiedSurface::CoreExpr, V5UnifiedSurface::CoreExpr),
            Self::OrdinaryNormalizedShapeEquality => (
                V5UnifiedSurface::OrdinarySchema2,
                V5UnifiedSurface::OrdinarySchema2,
            ),
            Self::CubicalTypedEquality => {
                (V5UnifiedSurface::CubicalPath, V5UnifiedSurface::CubicalPath)
            }
            Self::ExactExprSchema2Bridge => (
                V5UnifiedSurface::CoreExpr,
                V5UnifiedSurface::OrdinarySchema2,
            ),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "base_relation")]
pub enum V5UnifiedBaseRelation {
    Equal {
        rule: V5UnifiedEquivalenceBaseRule,
    },
    Distinct,
    NamedResidual {
        gap_id: String,
        exact_reason: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5UnifiedBaseEqualityDecision {
    pub left_node_id: String,
    pub right_node_id: String,
    pub left_surface: V5UnifiedSurface,
    pub right_surface: V5UnifiedSurface,
    pub relation: V5UnifiedBaseRelation,
    pub evidence: Value,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5UnifiedEquivalenceClosureProof {
    pub theorem_id: String,
    pub member_universe_node_ids: Vec<String>,
    pub member_universe_digest: String,
    pub base_rule_registry: Vec<V5UnifiedEquivalenceBaseRule>,
    pub base_rule_registry_digest: String,
    pub base_decisions: Vec<V5UnifiedBaseEqualityDecision>,
    pub expected_base_pair_count: usize,
    pub component_by_node: BTreeMap<String, String>,
    pub exact_rule_registry_replayed: bool,
    pub only_cross_surface_constructor_is_exact_expr_schema2_bridge: bool,
    pub no_base_equality_edge_touches_cubical_and_noncubical: bool,
    pub every_base_pair_decided: bool,
    pub reflexive_symmetric_transitive_closure_computed: bool,
    pub named_residual_count: usize,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "marginality")]
pub enum V5FreshMarginalityDisposition {
    InternalIdentical {
        predecessor_step: u32,
        predecessor_clause: u16,
        equality_derivation_hash: String,
    },
    InternalDerivable {
        predecessor_step: u32,
        predecessor_clause: u16,
        substitution: Value,
        instance_derivation_hash: String,
    },
    InternalUnifiedPreimage {
        predecessor_stage: u32,
        predecessor_family_id: String,
        equality_derivation_hash: String,
    },
    InternalR1FormationCompletionPackage {
        carrier_clause: u16,
        completion_clause: u16,
        package_derivation_hash: String,
    },
    MarginalNoPreimage {
        predecessor_comparison_digest: String,
    },
    NamedResidual {
        gap_id: String,
        exact_reason: String,
        evidence_hash: String,
    },
}

impl V5FreshMarginalityDisposition {
    fn is_marginal(&self) -> bool {
        matches!(self, Self::MarginalNoPreimage { .. })
    }

    fn is_internal(&self) -> bool {
        matches!(
            self,
            Self::InternalIdentical { .. }
                | Self::InternalDerivable { .. }
                | Self::InternalUnifiedPreimage { .. }
                | Self::InternalR1FormationCompletionPackage { .. }
        )
    }

    fn is_residual(&self) -> bool {
        matches!(self, Self::NamedResidual { .. })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5UnifiedQuotientProof {
    pub theorem_id: String,
    pub stage: u32,
    pub predecessor_signature_digest: String,
    pub predecessor_closure_digest: String,
    pub predecessor_surface_digest: String,
    pub current_surface_digest: String,
    pub current_members: Vec<V5UnifiedSurfaceMember>,
    pub fresh_core_extraction_family_ids: Vec<String>,
    pub r2_quotiented_core_family_ids: Vec<String>,
    pub fresh_core_inventory_digest: String,
    pub fresh_ordinary_bridge_inventory_digest: String,
    pub fresh_cubical_inventory_digest: String,
    pub core_family_ids: Vec<String>,
    pub ordinary_family_ids: Vec<String>,
    pub cubical_family_ids: Vec<String>,
    pub core_surface_digest: String,
    pub ordinary_surface_digest: String,
    pub cubical_surface_digest: String,
    pub equivalence_closure: V5UnifiedEquivalenceClosureProof,
    pub within_candidate_decisions: Vec<V5UnifiedEqualityDecision>,
    pub predecessor_decisions: Vec<V5UnifiedEqualityDecision>,
    pub fresh_marginality: BTreeMap<String, V5FreshMarginalityDisposition>,
    pub expected_within_candidate_pair_count: usize,
    pub expected_predecessor_pair_count: usize,
    pub every_current_member_enumerated_once: bool,
    pub fresh_core_inventory_exact: bool,
    pub fresh_ordinary_inventory_exact: bool,
    pub fresh_cubical_inventory_exact: bool,
    pub every_within_candidate_pair_decided: bool,
    pub every_predecessor_pair_decided: bool,
    pub every_active_member_has_fresh_marginality: bool,
    pub generated_instances_mint_no_class: bool,
    pub v4_marginality_labels_used_as_proof: bool,
    pub named_residual_count: usize,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "anchor")]
pub enum V5AnchorDisposition {
    Internal,
    CreditedLocalRole {
        clause: u16,
        role: LocalRole,
        relation_derivation_hashes: Vec<String>,
        injection_hash: String,
    },
    TheoremImpossibleNoRelation {
        theorem_id: String,
        closure_derivation_hash: String,
        no_constructed_exported_a3_fallback: bool,
        proof_hash: String,
    },
    TheoremImpossibleRelationCollision {
        theorem_id: String,
        clause: u16,
        role: LocalRole,
        competing_family_ids: Vec<String>,
        no_constructed_exported_a3_fallback: bool,
        proof_hash: String,
    },
    TheoremImpossibleNonFunctionalRelation {
        theorem_id: String,
        exact_slots: Vec<(u16, LocalRole)>,
        no_constructed_exported_a3_fallback: bool,
        proof_hash: String,
    },
    NamedResidual {
        gap_id: String,
        exact_reason: String,
        exact_a3_capability_derivation_hash: String,
        proof_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5SemanticFamilyWitness {
    pub family_id: String,
    pub stage: u32,
    pub source: V5SemanticFamilySource,
    pub shape_key: String,
    pub term_or_bridge_derivation_hash: String,
    pub typed_normalized_natural: bool,
    pub fresh_marginality: V5FreshMarginalityDisposition,
    pub marginal: bool,
    pub marginality_proof_hash: String,
    pub anchor: V5AnchorDisposition,
    pub credited: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V5RelationRule {
    GenericR1Completion,
    DirectKernelFamily,
    HitFormation,
    HitPoint,
    HitPathConstructor,
    HitRecursor,
    HitInductor,
    HitTruncAction,
    HitPostPathFace,
    HitPostPathOperation,
    HitPostPathCoherence,
    CubicalBeta,
    CubicalKan,
    AxiomaticInheritedFamily,
    AxiomaticLocalAndBridgeFace,
    AxiomaticSupportBridge,
    FormerAdjoint,
    MapPostcomposition,
    MapPrecomposition,
    MapReferenceCoherence,
    MapSingleAction,
    ModalPairwiseCoherence,
    ModalUniformLegacyAction,
    SynthesisDistributiveTransport,
    SynthesisInfinitesimalShift,
    SynthesisUniformTemporalAction,
    /// Serialization-only predecessor case.  V5 issuance never selects it:
    /// an unknown kind is an issuance error, not an absence theorem.
    NoFamilyConstructor,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V5ConstructorSurface {
    CoreExpr,
    OrdinarySchema2,
    CubicalPath,
    ExactA3,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "disposition")]
pub enum V5ExactA3OrbitDisposition {
    NonExportedInstanceOrbit {
        quotient_equality_hashes: Vec<String>,
        proof_hash: String,
    },
    ZeroChargeOpenHypothesis {
        registration_formation_hash: String,
        registration_replay_digest: String,
        hole_charge_hash: String,
        no_candidate_term_constructed: bool,
        no_credit_anchor_or_orbit_minted: bool,
        proof_hash: String,
    },
    ConstructedExportedOutput {
        family_id: String,
        term_relation_derivation_hash: String,
        proof_hash: String,
    },
    NamedResidual {
        gap_id: String,
        exact_reason: String,
        evidence_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5ExactA3OrbitCapability {
    pub orbit_id: String,
    pub scheme_id: String,
    pub representative_instance_id: String,
    pub member_instance_ids: Vec<String>,
    pub rule_constructor: String,
    pub required_output: Value,
    pub independently_exported: bool,
    pub scheme_and_instance_join_replayed: bool,
    pub disposition: V5ExactA3OrbitDisposition,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5ExactA3CapabilityProof {
    pub theorem_id: String,
    pub stage: u32,
    pub prefix_signature_digest: String,
    pub window_derivation_hash: String,
    pub inventory_exhaustiveness_derivation_hash: String,
    pub fresh_scheme_count: usize,
    pub fresh_instance_count: usize,
    pub fresh_orbit_count: usize,
    pub fresh_exported_orbit_count: usize,
    pub orbit_capabilities: Vec<V5ExactA3OrbitCapability>,
    pub exact_prefix_bound: bool,
    pub relative_inventory_exhaustive: bool,
    pub every_fresh_orbit_classified: bool,
    pub constructed_exported_output_count: usize,
    pub named_live_export_residual_count: usize,
    pub no_constructed_exported_a3_fallback: bool,
    pub cached_v4_zero_used_as_evidence: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5RoleConstructorSearchProof {
    pub theorem_id: String,
    pub declaration_id: String,
    pub role_kind: String,
    pub registry_index: usize,
    pub registry_size: usize,
    pub registry_exactly_historical_27: bool,
    pub rule: V5RelationRule,
    pub exact_coordinate: Value,
    pub owner_expr: Expr,
    pub owner_normal_form: Expr,
    pub owner_kernel_role: ClauseRole,
    pub owner_typing_derivation_hash: String,
    pub queried_surfaces: Vec<V5ConstructorSurface>,
    pub core_family_ids_checked: Vec<String>,
    pub core_surface_derivation_hash: String,
    pub ordinary_bridge_ids_checked: Vec<String>,
    pub ordinary_surface_derivation_hash: String,
    pub cubical_family_ids_checked: Vec<String>,
    pub cubical_surface_derivation_hash: String,
    pub a3_orbit_ids_checked: Vec<String>,
    pub a3_surface_derivation_hash: String,
    pub matching_family_ids: Vec<String>,
    pub core_constructor_search_complete: bool,
    pub ordinary_constructor_search_complete: bool,
    pub cubical_constructor_search_complete: bool,
    pub a3_capability_search_complete: bool,
    pub exact_coordinate_predicate_replayed: bool,
    pub exact_mechanism_predicate_replayed: bool,
    pub exact_local_role_predicate_replayed: bool,
    pub no_wildcard_or_default_absence_rule: bool,
    pub fresh_term_relation_derivation_hashes: Vec<String>,
    pub used_v4_desired_label_equality: bool,
    pub exhaustive_empty_search_proved: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5FamilyRoleRelationProof {
    pub theorem_id: String,
    pub declaration_id: String,
    pub occurrence: ActLocalV3RoleOccurrence,
    pub rule: V5RelationRule,
    pub family_id: String,
    pub family_term_or_bridge_derivation_hash: String,
    pub fresh_term_relation_derivation_hash: String,
    pub declaration_reissued_from_candidate_and_prefix: bool,
    pub exact_constructor_coordinate_replayed: bool,
    pub exact_owner_clause_replayed: bool,
    pub exact_mechanism_replayed: bool,
    pub exact_local_role_replayed: bool,
    pub label_candidate_used_as_proof: bool,
    pub v4_desired_label_equality_used_as_proof: bool,
    pub additional_credit_minted: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5RoleImpossibilityProof {
    pub theorem_id: String,
    pub declaration_id: String,
    pub occurrence: ActLocalV3RoleOccurrence,
    pub constructor_induction_rule: V5RelationRule,
    pub finite_closure_derivation_hash: String,
    pub candidate_family_ids_checked: Vec<String>,
    pub exact_a3_usable_output_count: usize,
    pub exact_a3_capability_derivation_hash: String,
    pub every_relation_constructor_checked: bool,
    pub matching_constructed_family_ids: Vec<String>,
    pub no_registered_family_relation: bool,
    pub no_unique_family_relation: bool,
    pub no_constructed_exported_a3_fallback: bool,
    pub conclusion: String,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5NamedRoleResidual {
    pub gap_id: String,
    pub declaration_id: String,
    pub occurrence: ActLocalV3RoleOccurrence,
    pub constructor_search_derivation_hash: String,
    pub exact_reason: String,
    pub missing_theorem: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "resolution")]
pub enum V5RoleResolution {
    ProvedFamily { proof: V5FamilyRoleRelationProof },
    TheoremBackedImpossibility { proof: V5RoleImpossibilityProof },
    NamedResidual { proof: V5NamedRoleResidual },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5RoleDeclarationResolution {
    pub stage: u32,
    pub declaration_id: String,
    pub frozen_v4_resolution_hash: String,
    pub constructor_search: V5RoleConstructorSearchProof,
    pub resolution: V5RoleResolution,
    pub resolved: bool,
    pub silent_residue: bool,
    pub derivation_hash: String,
}

/// Registry-erased semantic projection used by prefix-generic B3.  No field
/// contains the historical registry index/size or the old constructor-search
/// digest; those commitments remain non-authoritative compatibility evidence.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V5PrefixLocalResolutionClass {
    ProvedFamily,
    TheoremBackedImpossibility,
    NamedResidual,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5PrefixLocalRoleErasureRow {
    pub declaration_id: String,
    pub role_kind: String,
    pub occurrence_hash: String,
    pub occurrence_rederived_from_v3_prefix: bool,
    pub observed_kind_has_unique_grammar_entry: bool,
    pub direct_v5_rule: V5RelationRule,
    pub direct_rule_reclassification_exact: bool,
    pub exact_coordinate_predicate_replayed: bool,
    pub exact_mechanism_predicate_replayed: bool,
    pub exact_local_role_predicate_replayed: bool,
    pub queried_surfaces: Vec<V5ConstructorSurface>,
    pub core_surface_digest: String,
    pub ordinary_surface_digest: String,
    pub cubical_surface_digest: String,
    pub a3_surface_digest: String,
    pub target_family_ids: Vec<String>,
    pub target_relation_hashes: Vec<String>,
    pub resolution_class: V5PrefixLocalResolutionClass,
    pub resolved_family_id: Option<String>,
    pub local_role_clause: u16,
    pub local_role: LocalRole,
    pub resolution_exact_for_targets: bool,
    pub legacy_resolution_projection_matches: bool,
    pub unused_registry_extension_projection_equal: bool,
    pub no_default_rule: bool,
    pub v4_registry_route_hash_in_authoritative_projection: bool,
    pub registry_index_or_size_in_authoritative_projection: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5PrefixLocalRegistryErasureProof {
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub v3_occurrence_surface_replayed: bool,
    pub v4_declaration_surface_joined_only_as_legacy_witness: bool,
    pub sorted_unique_observed_kinds: Vec<String>,
    pub rows: Vec<V5PrefixLocalRoleErasureRow>,
    pub every_occurrence_has_one_direct_grammar_entry: bool,
    pub direct_closed_rule_induction_exhaustive: bool,
    pub target_computation_registry_extension_invariant: bool,
    pub resolution_registry_extension_invariant: bool,
    pub prefix_local_finite_closure_proved: bool,
    pub prefix_local_b1_proved: bool,
    pub prefix_local_b2_proved: bool,
    pub every_marginal_family_locally_credited_or_theorem_impossible: bool,
    pub credited_family_ids: Vec<String>,
    pub legacy_package_credited_family_ids: Vec<String>,
    pub credited_family_projection_exact: bool,
    pub recomputed_semantic_nu: u32,
    pub package_semantic_nu: u32,
    pub semantic_nu_registry_extension_invariant: bool,
    pub named_role_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub silent_residue_count: usize,
    pub t_bi_b1_proved: bool,
    pub t_bi_b2_proved: bool,
    pub special_case_projection_rebuilt_without_registry_hashes: bool,
    pub package_projection_exact: bool,
    pub historical_registry_suffix_used_as_semantic_premise: bool,
    pub old_v4_v5_full_hashes_authoritative_for_prefix_theorem: bool,
    pub stage9_historical_boundary_hash_in_authoritative_projection: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

/// Authoritative role row for the prefix-local issuer.  It deliberately
/// omits constructor-search registry indices, registry sizes, route hashes,
/// full-package hashes, and all legacy comparison fields.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5PrefixLocalAuthoritativeRoleRow {
    pub declaration_id: String,
    pub occurrence: ActLocalV3RoleOccurrence,
    pub direct_rule: V5RelationRule,
    pub queried_surfaces: Vec<V5ConstructorSurface>,
    pub exact_coordinate_predicate_replayed: bool,
    pub exact_mechanism_predicate_replayed: bool,
    pub exact_local_role_predicate_replayed: bool,
    pub no_default_rule: bool,
    pub target_family_ids: Vec<String>,
    pub target_relation_hashes: Vec<String>,
    pub resolution_class: V5PrefixLocalResolutionClass,
    pub resolved_family_id: Option<String>,
    pub resolved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5PrefixLocalAuthoritativeFamilyRow {
    pub family_id: String,
    pub stage: u32,
    pub source: V5SemanticFamilySource,
    pub shape_key: String,
    pub typed_normalized_natural: bool,
    pub fresh_marginality: V5FreshMarginalityDisposition,
    pub marginal: bool,
    pub anchor: V5AnchorDisposition,
    pub credited: bool,
    pub derivation_hash: String,
}

/// Registry-free authority projection for one locally built package.  The
/// source-surface hash is issued before any role registry is consulted; all
/// other hashes below are produced by the prefix-local v5 construction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5PrefixLocalSemanticPackageProof {
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub v3_declaration_hash: String,
    pub v4_candidate_local_source_hash: String,
    pub observed_role_kinds: Vec<String>,
    pub role_rows: Vec<V5PrefixLocalAuthoritativeRoleRow>,
    pub family_rows: Vec<V5PrefixLocalAuthoritativeFamilyRow>,
    pub bridge_derivation_hashes: Vec<String>,
    pub unified_quotient_derivation_hash: String,
    pub exact_a3_capability_derivation_hash: String,
    pub finite_closure_derivation_hash: String,
    pub credited_family_ids: Vec<String>,
    pub semantic_nu: u32,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub named_role_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub silent_residue_count: usize,
    pub t_bi_b1_proved: bool,
    pub t_bi_b2_proved: bool,
    pub every_role_declaration_resolved: bool,
    pub every_marginal_family_credited_or_theorem_impossible: bool,
    pub local_anchor_nonreuse_holds: bool,
    pub special_cases_proved: bool,
    pub unused_registry_extension_projection_equal: bool,
    pub historical_registry_consulted: bool,
    pub archive_structural_bar_verdict_or_future_read: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5PrefixLocalSemanticSequence {
    pub schema: String,
    pub theorem_id: String,
    pub packages: Vec<V5PrefixLocalSemanticPackageProof>,
    pub semantic_nu_vector: Vec<u32>,
    pub sorted_unique_observed_role_kinds: Vec<String>,
    pub every_package_registry_extension_invariant: bool,
    pub no_historical_registry_or_future_input: bool,
    pub t_bi_b1_proved_on_sequence: bool,
    pub t_bi_b2_proved_on_sequence: bool,
    pub authoritative_sequence_seal: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5Stage1CarrierRoleCaseProof {
    pub theorem_id: String,
    pub carrier_clause: u16,
    pub completion_clause: u16,
    pub role: LocalRole,
    pub carrier_expr: Expr,
    pub completion_expr: Expr,
    pub carrier_normal_form: Expr,
    pub completion_normal_form: Expr,
    pub candidate_clause_roles: Vec<ClauseRole>,
    pub ordinary_bridge_count: usize,
    pub cubical_family_count: usize,
    pub exact_a3_orbit_count: usize,
    pub completion_relation_derivation_hash: Option<String>,
    pub exact_formation_completion_package: bool,
    pub role_generated_by_completion: bool,
    pub other_role_constructor_surface_empty: bool,
    pub archive_or_desired_label_used: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5FiniteClosureProof {
    pub theorem_id: String,
    pub candidate_clause_count: usize,
    pub extracted_clause_family_count: usize,
    pub frozen_v4_unified_family_count: usize,
    pub ordinary_applicable_schema_count: usize,
    pub ordinary_bridge_count: usize,
    pub cubical_family_count: usize,
    pub unified_quotient_derivation_hash: String,
    pub core_surface_digest: String,
    pub ordinary_surface_digest: String,
    pub cubical_surface_digest: String,
    pub exact_a3_orbit_count: usize,
    pub exact_a3_usable_output_count: usize,
    pub exact_a3_capability_derivation_hash: String,
    pub exact_a3_fallback_exclusion_proved: bool,
    pub every_candidate_clause_in_extraction: bool,
    pub every_ordinary_schema_bridged: bool,
    pub path_quotient_complete: bool,
    pub unified_quotient_complete: bool,
    pub exact_a3_complete: bool,
    pub role_kind_registry_exhaustive: bool,
    pub archive_or_scalar_input_used: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V5PhaseReceipt {
    pub ordinal: u8,
    pub phase_id: String,
    pub evidence_hashes: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalSemanticProvenanceV5Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_ids: Vec<String>,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub frozen_v3_package_hash: String,
    pub frozen_v4_package_hash: String,
    pub bridges: Vec<V5ExprSchemaBridgeProof>,
    pub unified_quotient: V5UnifiedQuotientProof,
    pub semantic_families: Vec<V5SemanticFamilyWitness>,
    pub role_resolutions: Vec<V5RoleDeclarationResolution>,
    pub exact_a3_capability: V5ExactA3CapabilityProof,
    pub finite_closure: V5FiniteClosureProof,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub named_role_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub total_named_residual_count: usize,
    pub silent_residue_count: usize,
    pub marginal_family_count: usize,
    pub credited_semantic_family_count: usize,
    pub theorem_anchor_impossibility_count: usize,
    pub semantic_family_nu: u32,
    pub t_bi_b1_proved: bool,
    pub t_bi_b2_proved: bool,
    pub every_role_declaration_resolved: bool,
    pub every_marginal_family_credited_or_theorem_impossible: bool,
    pub local_anchor_nonreuse_holds: bool,
    pub stage1_r1_preserved: bool,
    pub stage1_carrier_role_case_proofs: Vec<V5Stage1CarrierRoleCaseProof>,
    pub stage1_carrier_role_case_derivation_hashes: Vec<String>,
    pub stage1_exact_completion_equality_derivation_hash: String,
    pub stage2_constitutive_question_not_assumed: bool,
    pub stage2_exact_family_surface_ids: Vec<String>,
    pub stage2_internality_derivation_hashes: Vec<String>,
    pub stage2_exact_predecessor_closure_hash: String,
    pub stage2_no_ordinary_duplicate: bool,
    pub stage9_boundary_decided_by_relation_theorem: bool,
    pub stage9_map_declaration_ids: Vec<String>,
    pub stage9_boundary_theorem_hash: String,
    pub r2_generated_instance_not_exported: bool,
    pub archive_read: bool,
    pub structural_nu_read: bool,
    pub bar_read: bool,
    pub verdict_read: bool,
    pub enacted_future_read: bool,
    pub phase_receipts: Vec<V5PhaseReceipt>,
    pub preseal_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalSemanticSequenceV5 {
    pub schema: String,
    pub date: String,
    pub packages: Vec<ActLocalSemanticProvenanceV5Certificate>,
    pub exact_package_derivation_hashes: Vec<String>,
    pub intrinsic_sequence_seal: String,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub named_role_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub total_named_residual_count: usize,
    pub silent_residue_count: usize,
    pub t_bi_b1_proved_on_sequence: bool,
    pub t_bi_b2_proved_on_sequence: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ActLocalSemanticProvenanceV5Error {
    #[error("invalid v5 input: {0}")]
    Input(String),
    #[error("T-BI-B1 failed: {0}")]
    Bridge(String),
    #[error("T-BI-B2 failed: {0}")]
    Relation(String),
    #[error("v5 invariant failed: {0}")]
    Invariant(String),
}

fn replay_adoptions() -> Result<(), ActLocalSemanticProvenanceV5Error> {
    let sc = std::str::from_utf8(SUPPORT_COMPREHENSION_ADJUDICATION_BYTES)
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Input(error.to_string()))?;
    let nr = std::str::from_utf8(NU_REGISTER_ADJUDICATION_BYTES)
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Input(error.to_string()))?;
    if !sc.contains("T-BI-B1")
        || !sc.contains("T-BI-B2")
        || !sc.contains("F-SC5")
        || !sc.contains("## ADOPTION")
        || !nr.contains("nu-register-split-v1")
        || !nr.contains("zero silent residue")
        || !nr.contains("## ADOPTION")
    {
        return Err(ActLocalSemanticProvenanceV5Error::Input(
            "adopted support-comprehension / nu-register sources did not replay".to_owned(),
        ));
    }
    Ok(())
}

fn as_u16_vec(value: &Value, field: &str) -> Option<Vec<u16>> {
    value
        .get(field)?
        .as_array()?
        .iter()
        .map(|entry| u16::try_from(entry.as_u64()?).ok())
        .collect()
}

fn extracted_family_for_clause<'a>(
    families: &'a [ExtractedFamily],
    clause: u16,
) -> Option<&'a ExtractedFamily> {
    families.iter().find(|family| {
        family
            .instances
            .iter()
            .any(|instance| instance.clause_index == clause)
    })
}

fn bridge_rule(kind: V5OrdinaryKind) -> V5ExprSchemaBridgeRule {
    match kind {
        V5OrdinaryKind::FreshFormation => V5ExprSchemaBridgeRule::FormationFromKernelType,
        V5OrdinaryKind::PointOrUnitIntro => {
            V5ExprSchemaBridgeRule::IntroductionFromKernelIntroduction
        }
        V5OrdinaryKind::PathConstructorIntro => {
            V5ExprSchemaBridgeRule::PathConstructorFromTypedPathDeclaration
        }
        V5OrdinaryKind::Recursor => V5ExprSchemaBridgeRule::RecursorFromTypedPathDeclaration,
        V5OrdinaryKind::Inductor => V5ExprSchemaBridgeRule::InductorFromTypedPathDeclaration,
        V5OrdinaryKind::TruncParametricAction => {
            V5ExprSchemaBridgeRule::TruncActionFromTypedTruncFormation
        }
        V5OrdinaryKind::PostPathOperation => {
            V5ExprSchemaBridgeRule::PostPathOperationFromTypedPostClause
        }
        V5OrdinaryKind::PostPathCoherence => {
            V5ExprSchemaBridgeRule::PostPathCoherenceFromTypedPostClause
        }
        V5OrdinaryKind::CellAction => V5ExprSchemaBridgeRule::CellActionFromTypedOperationAndPath,
    }
}

fn schema_hash(instance: &Value, field: &str, nested: &str) -> Option<String> {
    instance
        .get(field)?
        .get(nested)?
        .as_str()
        .map(str::to_owned)
}

fn source_shape_holds(
    kind: V5OrdinaryKind,
    source_clauses: &[u16],
    candidate: &Telescope,
    roles: &[ClauseRole],
    schema: &Value,
) -> bool {
    let exprs = source_clauses
        .iter()
        .filter_map(|clause| candidate.clauses.get(usize::from(*clause)))
        .map(|clause| &clause.expr)
        .collect::<Vec<_>>();
    if exprs.len() != source_clauses.len() {
        return false;
    }
    match kind {
        V5OrdinaryKind::FreshFormation => {
            source_clauses.len() == 1
                && roles.get(usize::from(source_clauses[0])) == Some(&ClauseRole::Formation)
        }
        V5OrdinaryKind::PointOrUnitIntro => {
            source_clauses.len() == 1
                && roles.get(usize::from(source_clauses[0])) == Some(&ClauseRole::Introduction)
        }
        V5OrdinaryKind::PathConstructorIntro => {
            source_clauses.len() == 1
                && matches!(exprs[0], Expr::PathCon(dimension) if *dimension > 0
                    && schema.get("interpretation")
                        .and_then(|value| value.get("cubical_dimension"))
                        .and_then(Value::as_u64) == Some(u64::from(*dimension)))
        }
        V5OrdinaryKind::Recursor | V5OrdinaryKind::Inductor => {
            source_clauses.len() == 1
                && matches!(exprs[0], Expr::PathCon(dimension) if *dimension > 0)
        }
        V5OrdinaryKind::TruncParametricAction => {
            source_clauses.len() == 1 && matches!(exprs[0], Expr::Trunc(_))
        }
        V5OrdinaryKind::PostPathOperation => {
            source_clauses.len() == 1
                && matches!(
                    roles.get(usize::from(source_clauses[0])),
                    Some(ClauseRole::Introduction | ClauseRole::Computation)
                )
        }
        V5OrdinaryKind::PostPathCoherence => source_clauses.len() == 1,
        V5OrdinaryKind::CellAction => {
            source_clauses.len() == 2 && exprs.iter().any(|expr| matches!(expr, Expr::PathCon(_)))
        }
    }
}

fn build_bridges(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    source: &V5SemanticSourceView,
    extraction: &pen_eval::typed_families::CandidateFamilyExtraction,
    elaboration: &pen_type::elaborate::TelescopeElaboration,
) -> Result<Vec<V5ExprSchemaBridgeProof>, ActLocalSemanticProvenanceV5Error> {
    let candidate_digest = candidate_hash(candidate);
    let carrier_clause = elaboration
        .clauses
        .iter()
        .find(|clause| {
            clause.kernel_ty == KernelTy::Type && clause.kernel_role == ClauseRole::Formation
        })
        .map(|clause| clause.clause_index)
        .or_else(|| {
            elaboration
                .clauses
                .first()
                .map(|clause| clause.clause_index)
        })
        .ok_or_else(|| {
            ActLocalSemanticProvenanceV5Error::Bridge("candidate has no clause".to_owned())
        })?;
    let carrier_normal_form = elaboration.clauses[usize::from(carrier_clause)]
        .normal_form
        .clone();
    let roles = elaboration
        .clauses
        .iter()
        .map(|clause| clause.kernel_role)
        .collect::<Vec<_>>();
    let mut bridges = Vec::new();
    for query in &source.ordinary_registry.constructor_queries {
        let kind = V5OrdinaryKind::parse(&query.constructor).ok_or_else(|| {
            ActLocalSemanticProvenanceV5Error::Bridge(format!(
                "unknown ordinary constructor {}",
                query.constructor
            ))
        })?;
        for instance in &query.exact_shape_instances {
            let source_clauses = as_u16_vec(instance, "source_clauses").ok_or_else(|| {
                ActLocalSemanticProvenanceV5Error::Bridge(format!(
                    "{} instance has no exact source clauses",
                    query.constructor
                ))
            })?;
            let source_exprs = source_clauses
                .iter()
                .map(|clause| {
                    candidate
                        .clauses
                        .get(usize::from(*clause))
                        .map(|clause| clause.expr.clone())
                        .ok_or_else(|| {
                            ActLocalSemanticProvenanceV5Error::Bridge(format!(
                                "ordinary source clause {clause} is out of bounds"
                            ))
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let source_normal_forms = source_clauses
                .iter()
                .map(|clause| {
                    elaboration.clauses[usize::from(*clause)]
                        .normal_form
                        .clone()
                })
                .collect::<Vec<_>>();
            let source_core_family_ids = source_clauses
                .iter()
                .map(|clause| {
                    extracted_family_for_clause(&extraction.families, *clause)
                        .map(|family| family.id.as_str().to_owned())
                        .ok_or_else(|| {
                            ActLocalSemanticProvenanceV5Error::Bridge(format!(
                                "ordinary source clause {clause} has no extracted family"
                            ))
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let source_typing_derivation_hashes = source_clauses
                .iter()
                .map(|clause| {
                    tagged_hash(
                        "source-clause-typing-derivation",
                        &elaboration.clauses[usize::from(*clause)],
                    )
                })
                .collect::<Vec<_>>();
            let schema = instance.get("schema").ok_or_else(|| {
                ActLocalSemanticProvenanceV5Error::Bridge(
                    "ordinary instance lacks schema".to_owned(),
                )
            })?;
            let support = schema.get("support_proof");
            let support_candidate_hashes = support
                .and_then(|value| value.get("source_telescopes"))
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|source| source.get("candidate_hash").and_then(Value::as_str))
                .collect::<BTreeSet<_>>();
            let support_current = support
                .and_then(|value| value.get("window"))
                .and_then(|value| value.get("current_step"))
                .and_then(Value::as_u64);
            let support_anchors = support
                .and_then(|value| value.get("anchors"))
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|anchor| {
                    Some((
                        anchor.get("step")?.as_u64()?,
                        u16::try_from(anchor.get("clause")?.as_u64()?).ok()?,
                    ))
                })
                .collect::<BTreeSet<_>>();
            let expected_anchors = source_clauses
                .iter()
                .map(|clause| (u64::from(stage), *clause))
                .collect::<BTreeSet<_>>();
            let support_bound_to_exact_candidate = support_current == Some(u64::from(stage))
                && support_candidate_hashes == BTreeSet::from([candidate_digest.as_str()])
                && support_anchors == expected_anchors;
            let source_family_membership_replayed = source_clauses.iter().all(|clause| {
                extracted_family_for_clause(&extraction.families, *clause).is_some_and(|family| {
                    family.instances.iter().any(|instance| {
                        instance.clause_index == *clause
                            && matches!(
                                instance.kind,
                                InstanceKind::Generator
                                    | InstanceKind::RenamingInstance
                                    | InstanceKind::Specialization { .. }
                            )
                    })
                })
            });
            let constructor_shape_induction_holds =
                source_shape_holds(kind, &source_clauses, candidate, &roles, schema);
            let typed_normalized_natural_replayed = query.every_typed_realizer_replayed
                && query.every_normalization_replayed
                && query.every_naturality_replayed
                && instance.get("typed_replayed").and_then(Value::as_bool) == Some(true)
                && instance
                    .get("normalization_replayed")
                    .and_then(Value::as_bool)
                    == Some(true)
                && instance.get("naturality_replayed").and_then(Value::as_bool) == Some(true);
            let generated_instance = instance
                .get("generated_instance")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let operation_shape = kind
                .uses_operation_shape()
                .then(|| source_normal_forms.clone());
            // Path dimension is deliberately absent from this key: the
            // adopted family/instance quotient treats uniform dimension
            // specialization as an instance.  Carrier and operation shapes
            // remain, so Trunc and post-path families do not collapse.
            let ordinary_shape_key = tagged_hash(
                "ordinary-natural-family-shape",
                &(kind, &carrier_normal_form, &operation_shape),
            );
            let ordinary_schema_derivation_hash = schema
                .get("formation_derivation_hash")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned();
            let ordinary_typed_realizer_derivation_hash =
                schema_hash(instance, "typed_realizer", "derivation_hash").unwrap_or_default();
            let ordinary_normalization_derivation_hash =
                schema_hash(instance, "normalization", "derivation_hash").unwrap_or_default();
            let ordinary_naturality_derivation_hash =
                schema_hash(instance, "naturality", "derivation_hash").unwrap_or_default();
            let legacy_v4_c1_flag_remains_false = !query.global_c1_bridge_retired
                && !query.exact_candidate_family_bridge_proved
                && schema
                    .get("global_c1_bridge_retired")
                    .and_then(Value::as_bool)
                    == Some(false);
            let semantic_family_id = tagged_hash(
                "ordinary-bridged-semantic-family",
                &(
                    stage,
                    &candidate_digest,
                    &ordinary_shape_key,
                    &ordinary_schema_derivation_hash,
                ),
            );
            let proved = support_bound_to_exact_candidate
                && source_family_membership_replayed
                && constructor_shape_induction_holds
                && typed_normalized_natural_replayed
                && legacy_v4_c1_flag_remains_false
                && generated_instance == kind.generated_instance()
                && !ordinary_schema_derivation_hash.is_empty()
                && !ordinary_typed_realizer_derivation_hash.is_empty()
                && !ordinary_normalization_derivation_hash.is_empty()
                && !ordinary_naturality_derivation_hash.is_empty();
            let mut proof = V5ExprSchemaBridgeProof {
                theorem_id: T_BI_B1_THEOREM_ID.to_owned(),
                stage,
                candidate_hash: candidate_digest.clone(),
                predecessor_signature_digest: prefix.digest().to_owned(),
                kind,
                bridge_rule: bridge_rule(kind),
                source_clauses,
                carrier_clause,
                source_exprs,
                source_normal_forms,
                carrier_normal_form: carrier_normal_form.clone(),
                source_core_family_ids,
                source_typing_derivation_hashes,
                ordinary_schema_derivation_hash,
                ordinary_typed_realizer_derivation_hash,
                ordinary_normalization_derivation_hash,
                ordinary_naturality_derivation_hash,
                ordinary_shape_key,
                semantic_family_id,
                legacy_v4_c1_flag_remains_false,
                support_bound_to_exact_candidate,
                source_family_membership_replayed,
                constructor_shape_induction_holds,
                typed_normalized_natural_replayed,
                generated_instance,
                zero_charge_support_only: true,
                proved,
                derivation_hash: String::new(),
            };
            proof.derivation_hash = tagged_hash("expr-schema2-bridge-proof", &proof);
            bridges.push(proof);
        }
    }
    bridges.sort_by(|left, right| {
        (left.kind, &left.source_clauses, &left.semantic_family_id).cmp(&(
            right.kind,
            &right.source_clauses,
            &right.semantic_family_id,
        ))
    });
    Ok(bridges)
}

fn extracted_family_for_v4_source<'a>(
    extraction: &'a pen_eval::typed_families::CandidateFamilyExtraction,
    source: &V4FamilySource,
) -> Option<&'a ExtractedFamily> {
    match source {
        V4FamilySource::Clause {
            generator_clause, ..
        } => extracted_family_for_clause(&extraction.families, *generator_clause),
        V4FamilySource::R1CompletedPackage {
            completion_clause, ..
        } => extracted_family_for_clause(&extraction.families, *completion_clause),
        V4FamilySource::CubicalPath { .. } => None,
    }
}

fn cubical_decision_values(source: &V5SemanticSourceView) -> Vec<Value> {
    let mut decisions = source
        .path_quotient
        .as_ref()
        .map(|proof| proof.pairwise_decisions.clone())
        .unwrap_or_default();
    for family in &source.unified_families {
        if !matches!(family.source, V4FamilySource::CubicalPath { .. }) {
            continue;
        }
        let proof = match &family.marginality {
            V4MarginalityDisposition::MarginalNoPreimage { proof, .. }
            | V4MarginalityDisposition::InternalIdentical { proof, .. } => proof,
            _ => continue,
        };
        decisions.extend(
            proof
                .get("decisions")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
        );
    }
    decisions
}

fn cubical_decision_for_pair<'a>(
    decisions: &'a [Value],
    left: &str,
    right: &str,
) -> Option<&'a Value> {
    decisions.iter().find(|decision| {
        let observed_left = decision.get("left_family_hash").and_then(Value::as_str);
        let observed_right = decision.get("right_family_hash").and_then(Value::as_str);
        (observed_left == Some(left) && observed_right == Some(right))
            || (observed_left == Some(right) && observed_right == Some(left))
    })
}

fn unified_equivalence_node_id(member: &V5UnifiedSurfaceMember) -> String {
    tagged_hash(
        "unified-equivalence-node",
        &(
            member.stage,
            &member.family_id,
            &member.presentation_hash,
            &member.typing_normalization_naturality_hash,
        ),
    )
}

fn decide_unified_base_equality(
    left: &V5UnifiedSurfaceMember,
    right: &V5UnifiedSurfaceMember,
    cubical_decisions: &[Value],
) -> V5UnifiedBaseEqualityDecision {
    let left_node_id = unified_equivalence_node_id(left);
    let right_node_id = unified_equivalence_node_id(right);
    let left_surface = left.presentation.surface();
    let right_surface = right.presentation.surface();
    let (relation, evidence) = match (&left.presentation, &right.presentation) {
        (
            V5UnifiedFamilyPresentation::CoreExpr {
                canonical_normal_form: left_nf,
                parameter_sorts: left_params,
                generator_kernel_type: left_kernel_type,
                generator_role: left_role,
                ..
            },
            V5UnifiedFamilyPresentation::CoreExpr {
                canonical_normal_form: right_nf,
                parameter_sorts: right_params,
                generator_kernel_type: right_kernel_type,
                generator_role: right_role,
                ..
            },
        ) => {
            if left_params != right_params
                || left_kernel_type != right_kernel_type
                || left_role != right_role
            {
                (
                    V5UnifiedBaseRelation::Distinct,
                    serde_json::json!({
                        "procedure": "typed-core-judgement-boundary-separation",
                        "left_parameters": left_params,
                        "right_parameters": right_params,
                        "left_kernel_type": left_kernel_type,
                        "right_kernel_type": right_kernel_type,
                        "left_generator_role": left_role,
                        "right_generator_role": right_role,
                    }),
                )
            } else {
                let scope = left_params
                    .as_array()
                    .and_then(|params| u32::try_from(params.len()).ok())
                    .unwrap_or(u32::MAX);
                match univalent_equality(left_nf, right_nf, scope, 4096) {
                    Ok(witness) => (
                        if witness.equal {
                            V5UnifiedBaseRelation::Equal {
                                rule: V5UnifiedEquivalenceBaseRule::CoreUnivalentEquality,
                            }
                        } else {
                            V5UnifiedBaseRelation::Distinct
                        },
                        serde_json::json!({
                            "procedure": "fresh-core-univalent-equality",
                            "term_equality": witness,
                            "kernel_type_compatible": true,
                            "generator_role_identical": true,
                        }),
                    ),
                    Err(error) => (
                        V5UnifiedBaseRelation::NamedResidual {
                            gap_id: "T_BI_B2_CORE_EQUALITY_UNDECIDED".to_owned(),
                            exact_reason: error.to_string(),
                        },
                        serde_json::json!({
                            "procedure": "fresh-core-univalent-equality",
                            "normalization_error": error.to_string(),
                        }),
                    ),
                }
            }
        }
        (
            V5UnifiedFamilyPresentation::OrdinarySchema2 {
                ordinary_shape_key: left_key,
                ..
            },
            V5UnifiedFamilyPresentation::OrdinarySchema2 {
                ordinary_shape_key: right_key,
                ..
            },
        ) => (
            if left_key == right_key {
                V5UnifiedBaseRelation::Equal {
                    rule: V5UnifiedEquivalenceBaseRule::OrdinaryNormalizedShapeEquality,
                }
            } else {
                V5UnifiedBaseRelation::Distinct
            },
            serde_json::json!({
                "procedure": "ordinary-normalized-family-shape-equality",
                "left_shape_key": left_key,
                "right_shape_key": right_key,
            }),
        ),
        (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        ) => {
            if left.family_id == right.family_id
                && left.presentation_hash == right.presentation_hash
            {
                (
                    V5UnifiedBaseRelation::Equal {
                        rule: V5UnifiedEquivalenceBaseRule::CubicalTypedEquality,
                    },
                    serde_json::json!({
                        "procedure": "identical-typed-cubical-presentation-reflexivity",
                        "family_id": left.family_id,
                        "presentation_hash": left.presentation_hash,
                    }),
                )
            } else {
                match cubical_decision_for_pair(
                    cubical_decisions,
                    &left.family_id,
                    &right.family_id,
                ) {
                    Some(decision) => match decision.get("decision").and_then(Value::as_str) {
                        Some("equal") => (
                            V5UnifiedBaseRelation::Equal {
                                rule: V5UnifiedEquivalenceBaseRule::CubicalTypedEquality,
                            },
                            decision.clone(),
                        ),
                        Some("distinct_within_fragment") => {
                            (V5UnifiedBaseRelation::Distinct, decision.clone())
                        }
                        Some("undefined") | None | Some(_) => (
                            V5UnifiedBaseRelation::NamedResidual {
                                gap_id: "T_BI_B2_CUBICAL_EQUALITY_UNDECIDED".to_owned(),
                                exact_reason: decision
                                    .get("obligation")
                                    .and_then(Value::as_str)
                                    .unwrap_or("cubical decision has no decided constructor")
                                    .to_owned(),
                            },
                            decision.clone(),
                        ),
                    },
                    None => (
                        V5UnifiedBaseRelation::NamedResidual {
                            gap_id: "T_BI_B2_CUBICAL_PAIR_NOT_ENUMERATED".to_owned(),
                            exact_reason: format!(
                                "no exact cubical equality row for {} and {}",
                                left.family_id, right.family_id
                            ),
                        },
                        serde_json::json!({
                            "left_family_id": left.family_id,
                            "right_family_id": right.family_id,
                        }),
                    ),
                }
            }
        }
        (
            V5UnifiedFamilyPresentation::CoreExpr {
                extracted_family_id,
                ..
            },
            V5UnifiedFamilyPresentation::OrdinarySchema2 {
                source_core_family_ids,
                ..
            },
        )
        | (
            V5UnifiedFamilyPresentation::OrdinarySchema2 {
                source_core_family_ids,
                ..
            },
            V5UnifiedFamilyPresentation::CoreExpr {
                extracted_family_id,
                ..
            },
        ) => {
            let typed = left.typed_normalized_natural
                && right.typed_normalized_natural
                && !left.typing_normalization_naturality_hash.is_empty()
                && !right.typing_normalization_naturality_hash.is_empty();
            let bridge_membership = source_core_family_ids.contains(extracted_family_id);
            if typed {
                (
                    if bridge_membership {
                        V5UnifiedBaseRelation::Equal {
                            rule: V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge,
                        }
                    } else {
                        V5UnifiedBaseRelation::Distinct
                    },
                    serde_json::json!({
                        "procedure": "exact-expr-schema2-bridge-constructor",
                        "core_family_id": extracted_family_id,
                        "ordinary_source_core_family_ids": source_core_family_ids,
                        "bridge_membership": bridge_membership,
                    }),
                )
            } else {
                (
                    V5UnifiedBaseRelation::NamedResidual {
                        gap_id: "T_BI_B2_EXPR_SCHEMA2_BRIDGE_UNTYPED".to_owned(),
                        exact_reason:
                            "a bridge endpoint lacked fresh typed/normal/natural evidence"
                                .to_owned(),
                    },
                    serde_json::json!({
                        "procedure": "exact-expr-schema2-bridge-constructor",
                        "typed_endpoints": typed,
                    }),
                )
            }
        }
        (
            V5UnifiedFamilyPresentation::CoreExpr { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        )
        | (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::CoreExpr { .. },
        )
        | (
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        )
        | (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
        ) => {
            let typed = left.typed_normalized_natural
                && right.typed_normalized_natural
                && !left.typing_normalization_naturality_hash.is_empty()
                && !right.typing_normalization_naturality_hash.is_empty();
            if typed {
                (
                    V5UnifiedBaseRelation::Distinct,
                    serde_json::json!({
                        "procedure": "closed-base-rule-endpoint-induction",
                        "left_surface": left_surface,
                        "right_surface": right_surface,
                        "applicable_cross_surface_base_constructors": [],
                    }),
                )
            } else {
                (
                    V5UnifiedBaseRelation::NamedResidual {
                        gap_id: "T_BI_B2_CUBICAL_CROSS_ENDPOINT_UNTYPED".to_owned(),
                        exact_reason:
                            "a cross-surface endpoint lacked typed/normal/natural evidence"
                                .to_owned(),
                    },
                    serde_json::json!({
                        "procedure": "closed-base-rule-endpoint-induction",
                        "typed_endpoints": typed,
                    }),
                )
            }
        }
    };
    let proved = !matches!(relation, V5UnifiedBaseRelation::NamedResidual { .. });
    let mut decision = V5UnifiedBaseEqualityDecision {
        left_node_id,
        right_node_id,
        left_surface,
        right_surface,
        relation,
        evidence,
        proved,
        derivation_hash: String::new(),
    };
    decision.derivation_hash = tagged_hash("unified-base-equality-decision", &decision);
    decision
}

fn build_unified_equivalence_closure(
    members: &[&V5UnifiedSurfaceMember],
    cubical_decisions: &[Value],
) -> V5UnifiedEquivalenceClosureProof {
    let mut member_universe_node_ids = members
        .iter()
        .map(|member| unified_equivalence_node_id(member))
        .collect::<Vec<_>>();
    member_universe_node_ids.sort();
    let unique_node_count = member_universe_node_ids
        .iter()
        .collect::<BTreeSet<_>>()
        .len();
    let base_rule_registry = V5UnifiedEquivalenceBaseRule::closed_registry();
    let expected_registry = vec![
        V5UnifiedEquivalenceBaseRule::CoreUnivalentEquality,
        V5UnifiedEquivalenceBaseRule::OrdinaryNormalizedShapeEquality,
        V5UnifiedEquivalenceBaseRule::CubicalTypedEquality,
        V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge,
    ];
    let exact_rule_registry_replayed = base_rule_registry == expected_registry;
    let cross_surface_rules = base_rule_registry
        .iter()
        .copied()
        .filter(|rule| {
            let (left, right) = rule.endpoint_surfaces();
            left != right
        })
        .collect::<Vec<_>>();
    let only_cross_surface_constructor_is_exact_expr_schema2_bridge =
        cross_surface_rules == vec![V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge];
    let base_rule_registry_digest = tagged_hash(
        "closed-unified-equivalence-base-rule-registry",
        &base_rule_registry,
    );
    let member_universe_digest = tagged_hash(
        "closed-unified-equivalence-member-universe",
        &member_universe_node_ids,
    );

    let mut base_decisions = Vec::new();
    let mut adjacency = member_universe_node_ids
        .iter()
        .cloned()
        .map(|node| (node, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for left in 0..members.len() {
        for right in left + 1..members.len() {
            let decision =
                decide_unified_base_equality(members[left], members[right], cubical_decisions);
            if matches!(decision.relation, V5UnifiedBaseRelation::Equal { .. }) {
                adjacency
                    .get_mut(&decision.left_node_id)
                    .expect("left equivalence node was registered")
                    .insert(decision.right_node_id.clone());
                adjacency
                    .get_mut(&decision.right_node_id)
                    .expect("right equivalence node was registered")
                    .insert(decision.left_node_id.clone());
            }
            base_decisions.push(decision);
        }
    }
    let expected_base_pair_count = members
        .len()
        .saturating_mul(members.len().saturating_sub(1))
        / 2;
    let named_residual_count = base_decisions
        .iter()
        .filter(|decision| !decision.proved)
        .count();
    let every_base_pair_decided = base_decisions.len() == expected_base_pair_count
        && base_decisions.iter().all(|decision| decision.proved);
    let no_base_equality_edge_touches_cubical_and_noncubical =
        base_decisions.iter().all(|decision| {
            !matches!(decision.relation, V5UnifiedBaseRelation::Equal { .. })
                || decision.left_surface == decision.right_surface
                || (decision.left_surface != V5UnifiedSurface::CubicalPath
                    && decision.right_surface != V5UnifiedSurface::CubicalPath)
        });

    let mut component_by_node = BTreeMap::new();
    for start in &member_universe_node_ids {
        if component_by_node.contains_key(start) {
            continue;
        }
        let component_id = start.clone();
        let mut queue = VecDeque::from([start.clone()]);
        while let Some(node) = queue.pop_front() {
            if component_by_node
                .insert(node.clone(), component_id.clone())
                .is_some()
            {
                continue;
            }
            for neighbour in adjacency
                .get(&node)
                .into_iter()
                .flat_map(|neighbours| neighbours.iter())
            {
                if !component_by_node.contains_key(neighbour) {
                    queue.push_back(neighbour.clone());
                }
            }
        }
    }
    let reflexive_symmetric_transitive_closure_computed = unique_node_count == members.len()
        && component_by_node.len() == members.len()
        && component_by_node.keys().eq(member_universe_node_ids.iter());
    let proved = exact_rule_registry_replayed
        && only_cross_surface_constructor_is_exact_expr_schema2_bridge
        && no_base_equality_edge_touches_cubical_and_noncubical
        && every_base_pair_decided
        && reflexive_symmetric_transitive_closure_computed
        && named_residual_count == 0;
    let mut proof = V5UnifiedEquivalenceClosureProof {
        theorem_id: T_BI_B2_EQUIVALENCE_CLOSURE_ID.to_owned(),
        member_universe_node_ids,
        member_universe_digest,
        base_rule_registry,
        base_rule_registry_digest,
        base_decisions,
        expected_base_pair_count,
        component_by_node,
        exact_rule_registry_replayed,
        only_cross_surface_constructor_is_exact_expr_schema2_bridge,
        no_base_equality_edge_touches_cubical_and_noncubical,
        every_base_pair_decided,
        reflexive_symmetric_transitive_closure_computed,
        named_residual_count,
        proved,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("closed-unified-equivalence-closure", &proof);
    proof
}

/// Replay the closed-rule and finite-graph part of T-BI-B2 without trusting
/// any cached conclusion bit.  The full certificate replay additionally
/// regenerates every base decision from the candidate and prefix.
pub fn replay_unified_equivalence_closure_v5(
    claimed: &V5UnifiedEquivalenceClosureProof,
) -> Vec<String> {
    let mut errors = Vec::new();
    let expected_registry = V5UnifiedEquivalenceBaseRule::closed_registry();
    let exact_rule_registry_replayed = claimed.base_rule_registry == expected_registry;
    if claimed.theorem_id != T_BI_B2_EQUIVALENCE_CLOSURE_ID
        || !exact_rule_registry_replayed
        || claimed.base_rule_registry_digest
            != tagged_hash(
                "closed-unified-equivalence-base-rule-registry",
                &claimed.base_rule_registry,
            )
    {
        errors.push("closed equivalence base-rule registry mismatch".to_owned());
    }
    let cross_surface_rules = claimed
        .base_rule_registry
        .iter()
        .copied()
        .filter(|rule| {
            let (left, right) = rule.endpoint_surfaces();
            left != right
        })
        .collect::<Vec<_>>();
    let only_cross_surface_constructor_is_exact_expr_schema2_bridge =
        cross_surface_rules == vec![V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge];

    let mut nodes = claimed.member_universe_node_ids.clone();
    nodes.sort();
    let node_set = nodes.iter().cloned().collect::<BTreeSet<_>>();
    let nodes_exact = nodes == claimed.member_universe_node_ids && node_set.len() == nodes.len();
    if !nodes_exact
        || claimed.member_universe_digest
            != tagged_hash("closed-unified-equivalence-member-universe", &nodes)
    {
        errors.push("closed equivalence member universe mismatch".to_owned());
    }

    let expected_pairs = (0..nodes.len())
        .flat_map(|left| {
            let nodes = &nodes;
            (left + 1..nodes.len()).map(move |right| (nodes[left].clone(), nodes[right].clone()))
        })
        .collect::<BTreeSet<_>>();
    let mut observed_pairs = BTreeSet::new();
    let mut adjacency = nodes
        .iter()
        .cloned()
        .map(|node| (node, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    let mut named_residual_count = 0usize;
    let mut base_rows_valid = true;
    for decision in &claimed.base_decisions {
        let pair = if decision.left_node_id < decision.right_node_id {
            (
                decision.left_node_id.clone(),
                decision.right_node_id.clone(),
            )
        } else {
            (
                decision.right_node_id.clone(),
                decision.left_node_id.clone(),
            )
        };
        base_rows_valid &= pair.0 != pair.1
            && node_set.contains(&pair.0)
            && node_set.contains(&pair.1)
            && observed_pairs.insert(pair);
        let mut projection = decision.clone();
        projection.derivation_hash.clear();
        let row_hash_valid =
            decision.derivation_hash == tagged_hash("unified-base-equality-decision", &projection);
        let row_proved = !matches!(
            decision.relation,
            V5UnifiedBaseRelation::NamedResidual { .. }
        );
        base_rows_valid &= row_hash_valid && decision.proved == row_proved;
        match &decision.relation {
            V5UnifiedBaseRelation::Equal { rule } => {
                let (rule_left, rule_right) = rule.endpoint_surfaces();
                let endpoints_match = (decision.left_surface == rule_left
                    && decision.right_surface == rule_right)
                    || (decision.left_surface == rule_right && decision.right_surface == rule_left);
                let evidence_supports_rule = match rule {
                    V5UnifiedEquivalenceBaseRule::CoreUnivalentEquality => {
                        decision
                            .evidence
                            .pointer("/term_equality/equal")
                            .and_then(Value::as_bool)
                            == Some(true)
                    }
                    V5UnifiedEquivalenceBaseRule::OrdinaryNormalizedShapeEquality => {
                        let left = decision.evidence.get("left_shape_key");
                        let right = decision.evidence.get("right_shape_key");
                        left.is_some() && left == right
                    }
                    V5UnifiedEquivalenceBaseRule::CubicalTypedEquality => {
                        decision.evidence.get("decision").and_then(Value::as_str) == Some("equal")
                            || decision.evidence.get("procedure").and_then(Value::as_str)
                                == Some("identical-typed-cubical-presentation-reflexivity")
                    }
                    V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge => {
                        decision
                            .evidence
                            .get("bridge_membership")
                            .and_then(Value::as_bool)
                            == Some(true)
                    }
                };
                base_rows_valid &= endpoints_match && evidence_supports_rule;
                if let (Some(left), Some(right)) = (
                    adjacency.get_mut(&decision.left_node_id),
                    Some(decision.right_node_id.clone()),
                ) {
                    left.insert(right);
                }
                if let (Some(right), Some(left)) = (
                    adjacency.get_mut(&decision.right_node_id),
                    Some(decision.left_node_id.clone()),
                ) {
                    right.insert(left);
                }
            }
            V5UnifiedBaseRelation::Distinct => {}
            V5UnifiedBaseRelation::NamedResidual { .. } => {
                named_residual_count += 1;
            }
        }
    }
    let every_base_pair_decided = claimed.base_decisions.len() == expected_pairs.len()
        && observed_pairs == expected_pairs
        && base_rows_valid
        && named_residual_count == 0;
    if !every_base_pair_decided {
        errors.push("closed equivalence base-pair sweep mismatch".to_owned());
    }
    let no_base_equality_edge_touches_cubical_and_noncubical =
        claimed.base_decisions.iter().all(|decision| {
            !matches!(decision.relation, V5UnifiedBaseRelation::Equal { .. })
                || decision.left_surface == decision.right_surface
                || (decision.left_surface != V5UnifiedSurface::CubicalPath
                    && decision.right_surface != V5UnifiedSurface::CubicalPath)
        });

    let mut component_by_node = BTreeMap::new();
    for start in &nodes {
        if component_by_node.contains_key(start) {
            continue;
        }
        let component_id = start.clone();
        let mut queue = VecDeque::from([start.clone()]);
        while let Some(node) = queue.pop_front() {
            if component_by_node
                .insert(node.clone(), component_id.clone())
                .is_some()
            {
                continue;
            }
            for neighbour in adjacency
                .get(&node)
                .into_iter()
                .flat_map(|neighbours| neighbours.iter())
            {
                if !component_by_node.contains_key(neighbour) {
                    queue.push_back(neighbour.clone());
                }
            }
        }
    }
    let reflexive_symmetric_transitive_closure_computed = nodes_exact
        && component_by_node.len() == nodes.len()
        && component_by_node == claimed.component_by_node;
    if !reflexive_symmetric_transitive_closure_computed {
        errors.push("closed equivalence component closure mismatch".to_owned());
    }
    let expected_proved = exact_rule_registry_replayed
        && only_cross_surface_constructor_is_exact_expr_schema2_bridge
        && no_base_equality_edge_touches_cubical_and_noncubical
        && every_base_pair_decided
        && reflexive_symmetric_transitive_closure_computed
        && named_residual_count == 0;
    if claimed.expected_base_pair_count != expected_pairs.len()
        || claimed.exact_rule_registry_replayed != exact_rule_registry_replayed
        || claimed.only_cross_surface_constructor_is_exact_expr_schema2_bridge
            != only_cross_surface_constructor_is_exact_expr_schema2_bridge
        || claimed.no_base_equality_edge_touches_cubical_and_noncubical
            != no_base_equality_edge_touches_cubical_and_noncubical
        || claimed.every_base_pair_decided != every_base_pair_decided
        || claimed.reflexive_symmetric_transitive_closure_computed
            != reflexive_symmetric_transitive_closure_computed
        || claimed.named_residual_count != named_residual_count
        || claimed.proved != expected_proved
    {
        errors.push("closed equivalence cached conclusion mismatch".to_owned());
    }
    let mut projection = claimed.clone();
    projection.derivation_hash.clear();
    if claimed.derivation_hash != tagged_hash("closed-unified-equivalence-closure", &projection) {
        errors.push("closed equivalence proof digest mismatch".to_owned());
    }
    errors
}

fn equivalence_closure_relation(
    left: &V5UnifiedSurfaceMember,
    right: &V5UnifiedSurfaceMember,
    closure: &V5UnifiedEquivalenceClosureProof,
) -> (V5UnifiedEqualityRelation, String, Value) {
    let left_node_id = unified_equivalence_node_id(left);
    let right_node_id = unified_equivalence_node_id(right);
    let left_component = closure.component_by_node.get(&left_node_id);
    let right_component = closure.component_by_node.get(&right_node_id);
    let evidence = serde_json::json!({
        "closure_theorem_id": closure.theorem_id,
        "closure_derivation_hash": closure.derivation_hash,
        "member_universe_digest": closure.member_universe_digest,
        "base_rule_registry_digest": closure.base_rule_registry_digest,
        "left_node_id": left_node_id,
        "right_node_id": right_node_id,
        "left_component": left_component,
        "right_component": right_component,
        "only_cross_surface_constructor_is_exact_expr_schema2_bridge": closure.only_cross_surface_constructor_is_exact_expr_schema2_bridge,
        "no_base_equality_edge_touches_cubical_and_noncubical": closure.no_base_equality_edge_touches_cubical_and_noncubical,
    });
    if !closure.proved || left_component.is_none() || right_component.is_none() {
        (
            V5UnifiedEqualityRelation::NamedResidual {
                gap_id: "T_BI_B2_EQUIVALENCE_CLOSURE_UNPROVED".to_owned(),
                exact_reason: "the closed base-rule induction or its finite equivalence closure did not replay"
                    .to_owned(),
            },
            "closed-unified-equivalence-rule-induction-v1".to_owned(),
            evidence,
        )
    } else if left_component == right_component {
        (
            V5UnifiedEqualityRelation::Equal,
            "closed-unified-equivalence-rule-induction-v1".to_owned(),
            evidence,
        )
    } else {
        (
            V5UnifiedEqualityRelation::Distinct,
            "closed-unified-equivalence-rule-induction-v1".to_owned(),
            evidence,
        )
    }
}

fn compare_unified_members(
    left: &V5UnifiedSurfaceMember,
    right: &V5UnifiedSurfaceMember,
    equivalence_closure: &V5UnifiedEquivalenceClosureProof,
) -> V5UnifiedEqualityDecision {
    let left_surface = left.presentation.surface();
    let right_surface = right.presentation.surface();
    let (relation, procedure, evidence) = match (&left.presentation, &right.presentation) {
        (
            V5UnifiedFamilyPresentation::CoreExpr { .. },
            V5UnifiedFamilyPresentation::CoreExpr { .. },
        ) => equivalence_closure_relation(left, right, equivalence_closure),
        (
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
        ) => equivalence_closure_relation(left, right, equivalence_closure),
        (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        ) => equivalence_closure_relation(left, right, equivalence_closure),
        (
            V5UnifiedFamilyPresentation::CoreExpr { .. },
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
        )
        | (
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
            V5UnifiedFamilyPresentation::CoreExpr { .. },
        )
        | (
            V5UnifiedFamilyPresentation::CoreExpr { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        )
        | (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::CoreExpr { .. },
        )
        | (
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        )
        | (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
        ) => equivalence_closure_relation(left, right, equivalence_closure),
    };
    let proved = !matches!(relation, V5UnifiedEqualityRelation::NamedResidual { .. });
    let mut decision = V5UnifiedEqualityDecision {
        left_stage: left.stage,
        left_family_id: left.family_id.clone(),
        right_stage: right.stage,
        right_family_id: right.family_id.clone(),
        left_surface,
        right_surface,
        relation,
        procedure,
        evidence,
        v4_marginality_label_used_as_proof: false,
        proved,
        derivation_hash: String::new(),
    };
    decision.derivation_hash = tagged_hash("unified-family-equality-decision", &decision);
    decision
}

fn build_unified_members(
    stage: u32,
    source: &V5SemanticSourceView,
    extraction: &pen_eval::typed_families::CandidateFamilyExtraction,
    bridges: &[V5ExprSchemaBridgeProof],
) -> Result<Vec<V5UnifiedSurfaceMember>, ActLocalSemanticProvenanceV5Error> {
    let covered_core_ids = bridges
        .iter()
        .filter(|bridge| !bridge.generated_instance)
        .flat_map(|bridge| bridge.source_core_family_ids.iter().cloned())
        .collect::<BTreeSet<_>>();
    let mut members = Vec::new();
    for family in &source.unified_families {
        let (source_clauses, presentation) = match &family.source {
            V4FamilySource::Clause {
                generator_clause, ..
            }
            | V4FamilySource::R1CompletedPackage {
                completion_clause: generator_clause,
                ..
            } => {
                let extracted = extracted_family_for_v4_source(extraction, &family.source)
                    .ok_or_else(|| {
                        ActLocalSemanticProvenanceV5Error::Invariant(format!(
                            "core family {} has no fresh extracted presentation",
                            family.family_id
                        ))
                    })?;
                (
                    vec![*generator_clause],
                    V5UnifiedFamilyPresentation::CoreExpr {
                        extracted_family_id: extracted.id.as_str().to_owned(),
                        canonical_normal_form: extracted.presentation.canonical_normal_form.clone(),
                        parameter_sorts: serde_json::to_value(&extracted.presentation.parameters)
                            .expect("parameter sorts serialize"),
                        generator_kernel_type: serde_json::to_value(&extracted.generator_kernel_ty)
                            .expect("kernel type serializes"),
                        generator_role: extracted.generator_role,
                    },
                )
            }
            V4FamilySource::CubicalPath {
                path_clause, key, ..
            } => (
                vec![*path_clause],
                V5UnifiedFamilyPresentation::CubicalPath {
                    path_clause: *path_clause,
                    key: key.clone(),
                    normal_form: family.term.normal_form.clone(),
                    inferred_type: family.term.inferred_type.clone(),
                },
            ),
        };
        let replaced = matches!(family.source, V4FamilySource::Clause { .. })
            && covered_core_ids.contains(&family.family_id);
        let bridge_family_ids = bridges
            .iter()
            .filter(|bridge| bridge.source_core_family_ids.contains(&family.family_id))
            .map(|bridge| bridge.semantic_family_id.clone())
            .collect::<Vec<_>>();
        let disposition = if replaced {
            V5UnifiedMemberDisposition::CoreReplacedByOrdinaryBridge { bridge_family_ids }
        } else {
            V5UnifiedMemberDisposition::Active
        };
        let presentation_hash = tagged_hash("unified-family-presentation", &presentation);
        let mut member = V5UnifiedSurfaceMember {
            family_id: family.family_id.clone(),
            stage,
            source_clauses,
            presentation,
            presentation_hash,
            typing_normalization_naturality_hash: family.term.derivation_hash.clone(),
            typed_normalized_natural: family.term.typing_replayed
                && family.term.normalization_replayed
                && family.term.naturality_replayed,
            included_in_semantic_ledger: !replaced,
            visible_to_later_predecessor_closure: true,
            disposition,
            derivation_hash: String::new(),
        };
        member.derivation_hash = tagged_hash("unified-surface-member", &member);
        members.push(member);
    }
    for bridge in bridges {
        let operation_normal_forms = bridge
            .kind
            .uses_operation_shape()
            .then(|| bridge.source_normal_forms.clone())
            .unwrap_or_default();
        let presentation = V5UnifiedFamilyPresentation::OrdinarySchema2 {
            kind: bridge.kind,
            ordinary_shape_key: bridge.ordinary_shape_key.clone(),
            carrier_normal_form: bridge.carrier_normal_form.clone(),
            operation_normal_forms,
            source_core_family_ids: bridge.source_core_family_ids.clone(),
        };
        let presentation_hash = tagged_hash("unified-family-presentation", &presentation);
        let disposition = if bridge.generated_instance {
            V5UnifiedMemberDisposition::GeneratedUniformInstance
        } else {
            V5UnifiedMemberDisposition::Active
        };
        let mut member = V5UnifiedSurfaceMember {
            family_id: bridge.semantic_family_id.clone(),
            stage,
            source_clauses: bridge.source_clauses.clone(),
            presentation,
            presentation_hash,
            typing_normalization_naturality_hash: bridge.derivation_hash.clone(),
            typed_normalized_natural: bridge.proved,
            included_in_semantic_ledger: !bridge.generated_instance,
            visible_to_later_predecessor_closure: !bridge.generated_instance,
            disposition,
            derivation_hash: String::new(),
        };
        member.derivation_hash = tagged_hash("unified-surface-member", &member);
        members.push(member);
    }
    members.sort_by(|left, right| {
        (left.presentation.surface(), &left.family_id)
            .cmp(&(right.presentation.surface(), &right.family_id))
    });
    let mut ordinary_representatives = BTreeMap::<String, String>::new();
    for member in &mut members {
        let V5UnifiedFamilyPresentation::OrdinarySchema2 {
            ordinary_shape_key, ..
        } = &member.presentation
        else {
            continue;
        };
        if !member.included_in_semantic_ledger {
            continue;
        }
        if let Some(representative) = ordinary_representatives.get(ordinary_shape_key) {
            member.disposition = V5UnifiedMemberDisposition::EqualWithinCandidate {
                representative_family_id: representative.clone(),
            };
            member.included_in_semantic_ledger = false;
            member.visible_to_later_predecessor_closure = false;
            member.derivation_hash = tagged_hash("unified-surface-member", member);
        } else {
            ordinary_representatives.insert(ordinary_shape_key.clone(), member.family_id.clone());
        }
    }
    Ok(members)
}

fn fresh_core_marginality(
    member: &V5UnifiedSurfaceMember,
    extraction: &pen_eval::typed_families::CandidateFamilyExtraction,
    closure: &PredecessorClosure,
    source: &V5SemanticSourceView,
    predecessor_decisions: &[V5UnifiedEqualityDecision],
) -> V5FreshMarginalityDisposition {
    let V5UnifiedFamilyPresentation::CoreExpr {
        extracted_family_id,
        ..
    } = &member.presentation
    else {
        unreachable!("fresh_core_marginality called on non-core member")
    };
    if let Some(r1) = &source.special_cases.generic_r1 {
        let is_carrier = member.source_clauses == vec![r1.carrier_clause];
        let exact_package = r1.adopted_package_clause_replayed
            && r1.carrier_is_kernel_formation_type
            && r1.completion_is_kernel_formation_type
            && r1.completion_is_exact_app_univ_carrier
            && r1.dependency_resolves_to_carrier
            && r1.completed_action_covers_carrier_by_adopted_r1
            && !r1.archive_or_count_input_used;
        if is_carrier && exact_package {
            return V5FreshMarginalityDisposition::InternalR1FormationCompletionPackage {
                carrier_clause: r1.carrier_clause,
                completion_clause: r1.completion_clause,
                package_derivation_hash: tagged_hash(
                    "fresh-R1-carrier-package-internality",
                    &(
                        &member.presentation_hash,
                        &r1.carrier_term,
                        &r1.completion_term,
                        &r1.carrier_typing,
                        &r1.completion_typing,
                    ),
                ),
            };
        }
    }
    let Some(extracted) = extraction
        .families
        .iter()
        .find(|family| family.id.as_str() == extracted_family_id)
    else {
        return V5FreshMarginalityDisposition::NamedResidual {
            gap_id: "T_BI_B2_CORE_PRESENTATION_NOT_IN_FRESH_EXTRACTION".to_owned(),
            exact_reason: format!("fresh extraction lacks {extracted_family_id}"),
            evidence_hash: member.derivation_hash.clone(),
        };
    };
    match &extracted.marginality {
        TypedMarginalityDisposition::InternalIdentical {
            predecessor_step,
            predecessor_clause,
            equality,
        } => {
            let closure_member = closure.families.iter().any(|family| {
                family.step == *predecessor_step
                    && family.clause_index == *predecessor_clause
                    && family.presentation.parameters == extracted.presentation.parameters
            });
            let unified_equal_preimage = predecessor_decisions.iter().any(|decision| {
                decision.left_family_id == member.family_id
                    && decision.right_stage == *predecessor_step
                    && matches!(decision.relation, V5UnifiedEqualityRelation::Equal)
            });
            if equality.equal && closure_member && unified_equal_preimage {
                V5FreshMarginalityDisposition::InternalIdentical {
                    predecessor_step: *predecessor_step,
                    predecessor_clause: *predecessor_clause,
                    equality_derivation_hash: tagged_hash(
                        "fresh-core-predecessor-equality",
                        &(
                            &member.presentation_hash,
                            equality,
                            &closure.digest,
                            predecessor_step,
                            predecessor_clause,
                            unified_equal_preimage,
                        ),
                    ),
                }
            } else {
                V5FreshMarginalityDisposition::NamedResidual {
                    gap_id: "T_BI_B2_CORE_IDENTICAL_REPLAY_FAILED".to_owned(),
                    exact_reason: "fresh typed equality did not identify an exact closure member"
                        .to_owned(),
                    evidence_hash: tagged_hash(
                        "failed-core-identical-replay",
                        &(equality, closure_member, unified_equal_preimage),
                    ),
                }
            }
        }
        TypedMarginalityDisposition::InternalDerivable {
            predecessor_step,
            predecessor_clause,
            substitution,
        } => {
            let closure_member = closure.families.iter().any(|family| {
                family.step == *predecessor_step && family.clause_index == *predecessor_clause
            });
            if closure_member {
                let substitution_value =
                    serde_json::to_value(substitution).expect("substitution serializes");
                V5FreshMarginalityDisposition::InternalDerivable {
                    predecessor_step: *predecessor_step,
                    predecessor_clause: *predecessor_clause,
                    substitution: substitution_value.clone(),
                    instance_derivation_hash: tagged_hash(
                        "fresh-core-instance-derivation",
                        &(
                            &member.presentation_hash,
                            predecessor_step,
                            predecessor_clause,
                            &substitution_value,
                            &closure.digest,
                        ),
                    ),
                }
            } else {
                V5FreshMarginalityDisposition::NamedResidual {
                    gap_id: "T_BI_B2_CORE_INSTANCE_PREIMAGE_MISSING".to_owned(),
                    exact_reason: "typed instance points outside the fresh predecessor closure"
                        .to_owned(),
                    evidence_hash: tagged_hash(
                        "failed-core-instance-replay",
                        &(predecessor_step, predecessor_clause, substitution),
                    ),
                }
            }
        }
        TypedMarginalityDisposition::MarginalNoClosurePreimage { closure_digest } => {
            let comparisons = predecessor_decisions
                .iter()
                .filter(|decision| decision.left_family_id == member.family_id)
                .collect::<Vec<_>>();
            let no_equal = comparisons
                .iter()
                .all(|decision| matches!(decision.relation, V5UnifiedEqualityRelation::Distinct));
            if closure_digest == &closure.digest && no_equal {
                V5FreshMarginalityDisposition::MarginalNoPreimage {
                    predecessor_comparison_digest: tagged_hash(
                        "fresh-core-no-preimage",
                        &(
                            &member.presentation_hash,
                            closure_digest,
                            comparisons
                                .iter()
                                .map(|decision| decision.derivation_hash.as_str())
                                .collect::<Vec<_>>(),
                        ),
                    ),
                }
            } else {
                V5FreshMarginalityDisposition::NamedResidual {
                    gap_id: "T_BI_B2_CORE_NO_PREIMAGE_REPLAY_FAILED".to_owned(),
                    exact_reason: "fresh extraction and unified predecessor sweep disagree"
                        .to_owned(),
                    evidence_hash: tagged_hash(
                        "failed-core-no-preimage-replay",
                        &(closure_digest, &closure.digest, no_equal),
                    ),
                }
            }
        }
    }
}

fn build_unified_quotient_and_families(
    prefix: &SealedSignature,
    stage: u32,
    source: &V5SemanticSourceView,
    extraction: &pen_eval::typed_families::CandidateFamilyExtraction,
    closure: &PredecessorClosure,
    bridges: &[V5ExprSchemaBridgeProof],
    predecessor_members: &[V5UnifiedSurfaceMember],
    historical_cubical_decisions: &[Value],
) -> Result<(V5UnifiedQuotientProof, Vec<V5SemanticFamilyWitness>), ActLocalSemanticProvenanceV5Error>
{
    let current_members = build_unified_members(stage, source, extraction, bridges)?;
    let visible_predecessors = predecessor_members
        .iter()
        .filter(|member| member.visible_to_later_predecessor_closure)
        .collect::<Vec<_>>();
    let equivalence_member_universe = current_members
        .iter()
        .chain(visible_predecessors.iter().copied())
        .collect::<Vec<_>>();
    let equivalence_closure = build_unified_equivalence_closure(
        &equivalence_member_universe,
        historical_cubical_decisions,
    );
    let mut within_candidate_decisions = Vec::new();
    for left in 0..current_members.len() {
        for right in left + 1..current_members.len() {
            within_candidate_decisions.push(compare_unified_members(
                &current_members[left],
                &current_members[right],
                &equivalence_closure,
            ));
        }
    }
    let mut predecessor_decisions = Vec::new();
    for current in &current_members {
        for predecessor in &visible_predecessors {
            predecessor_decisions.push(compare_unified_members(
                current,
                predecessor,
                &equivalence_closure,
            ));
        }
    }
    let expected_within_candidate_pair_count = current_members
        .len()
        .saturating_mul(current_members.len().saturating_sub(1))
        / 2;
    let expected_predecessor_pair_count = current_members.len() * visible_predecessors.len();
    let every_within_candidate_pair_decided = within_candidate_decisions.len()
        == expected_within_candidate_pair_count
        && within_candidate_decisions
            .iter()
            .all(|decision| decision.proved);
    let every_predecessor_pair_decided = predecessor_decisions.len()
        == expected_predecessor_pair_count
        && predecessor_decisions.iter().all(|decision| decision.proved);

    let mut fresh_marginality = BTreeMap::new();
    for member in current_members
        .iter()
        .filter(|member| member.included_in_semantic_ledger)
    {
        let member_decisions = predecessor_decisions
            .iter()
            .filter(|decision| decision.left_family_id == member.family_id)
            .collect::<Vec<_>>();
        let residual = member_decisions.iter().find(|decision| !decision.proved);
        let disposition = if let Some(residual) = residual {
            V5FreshMarginalityDisposition::NamedResidual {
                gap_id: "T_BI_B2_UNIFIED_PREDECESSOR_COMPARISON_UNDECIDED".to_owned(),
                exact_reason: format!(
                    "comparison {} to {} is undecided",
                    residual.left_family_id, residual.right_family_id
                ),
                evidence_hash: residual.derivation_hash.clone(),
            }
        } else {
            match member.presentation.surface() {
                V5UnifiedSurface::CoreExpr => {
                    fresh_core_marginality(
                        member,
                        extraction,
                        closure,
                        source,
                        &predecessor_decisions,
                    )
                }
                V5UnifiedSurface::OrdinarySchema2 | V5UnifiedSurface::CubicalPath => {
                    if let Some(equal) = member_decisions.iter().find(|decision| {
                        matches!(decision.relation, V5UnifiedEqualityRelation::Equal)
                    }) {
                        V5FreshMarginalityDisposition::InternalUnifiedPreimage {
                            predecessor_stage: equal.right_stage,
                            predecessor_family_id: equal.right_family_id.clone(),
                            equality_derivation_hash: equal.derivation_hash.clone(),
                        }
                    } else {
                        V5FreshMarginalityDisposition::MarginalNoPreimage {
                            predecessor_comparison_digest: tagged_hash(
                                "fresh-unified-no-preimage",
                                &(
                                    &member.presentation_hash,
                                    member_decisions
                                        .iter()
                                        .map(|decision| decision.derivation_hash.as_str())
                                        .collect::<Vec<_>>(),
                                ),
                            ),
                        }
                    }
                }
            }
        };
        fresh_marginality.insert(member.family_id.clone(), disposition);
    }
    let core_family_ids = current_members
        .iter()
        .filter(|member| {
            member.included_in_semantic_ledger
                && member.presentation.surface() == V5UnifiedSurface::CoreExpr
        })
        .map(|member| member.family_id.clone())
        .collect::<Vec<_>>();
    let ordinary_family_ids = current_members
        .iter()
        .filter(|member| {
            member.included_in_semantic_ledger
                && member.presentation.surface() == V5UnifiedSurface::OrdinarySchema2
        })
        .map(|member| member.family_id.clone())
        .collect::<Vec<_>>();
    let cubical_family_ids = current_members
        .iter()
        .filter(|member| {
            member.included_in_semantic_ledger
                && member.presentation.surface() == V5UnifiedSurface::CubicalPath
        })
        .map(|member| member.family_id.clone())
        .collect::<Vec<_>>();
    let mut fresh_core_extraction_family_ids = extraction
        .families
        .iter()
        .map(|family| family.id.as_str().to_owned())
        .collect::<Vec<_>>();
    fresh_core_extraction_family_ids.sort();
    let observed_core_extraction_ids = current_members
        .iter()
        .filter_map(|member| match &member.presentation {
            V5UnifiedFamilyPresentation::CoreExpr {
                extracted_family_id,
                ..
            } => Some(extracted_family_id.clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let fresh_core_extraction_set = fresh_core_extraction_family_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut r2_quotiented_core_family_ids = fresh_core_extraction_set
        .difference(&observed_core_extraction_ids)
        .cloned()
        .collect::<Vec<_>>();
    r2_quotiented_core_family_ids.sort();
    let r2_generated_source_ids = bridges
        .iter()
        .filter(|bridge| bridge.generated_instance)
        .flat_map(|bridge| bridge.source_core_family_ids.iter().cloned())
        .collect::<BTreeSet<_>>();
    let r2_replay_complete = source.special_cases.r2_parent_slot_label_join_holds
        && source.special_cases.r2_exact_v3_occurrence_surface_holds
        && source.special_cases.r2_m1_generated_membership_replayed
        && source
            .special_cases
            .r2_parent_membership_replayed_before_anchoring
        && source
            .special_cases
            .r2_removed_occurrences_absent_from_unified_membership
        && source.special_cases.r2_removed_occurrences_emitted_as_families == 0
        && !source.special_cases.r2_generated_instance_multiplied;
    let fresh_core_inventory_exact = observed_core_extraction_ids
        .is_subset(&fresh_core_extraction_set)
        && r2_quotiented_core_family_ids
            .iter()
            .all(|family_id| r2_replay_complete && r2_generated_source_ids.contains(family_id))
        && observed_core_extraction_ids.len() + r2_quotiented_core_family_ids.len()
            == fresh_core_extraction_set.len();
    let mut expected_ordinary_ids = bridges
        .iter()
        .map(|bridge| bridge.semantic_family_id.clone())
        .collect::<Vec<_>>();
    expected_ordinary_ids.sort();
    let mut observed_ordinary_ids = current_members
        .iter()
        .filter(|member| member.presentation.surface() == V5UnifiedSurface::OrdinarySchema2)
        .map(|member| member.family_id.clone())
        .collect::<Vec<_>>();
    observed_ordinary_ids.sort();
    let fresh_ordinary_inventory_exact = expected_ordinary_ids == observed_ordinary_ids
        && bridges.iter().all(|bridge| bridge.proved)
        && bridges
            .iter()
            .filter(|bridge| bridge.generated_instance)
            .count()
            == current_members
                .iter()
                .filter(|member| {
                    matches!(
                        member.disposition,
                        V5UnifiedMemberDisposition::GeneratedUniformInstance
                    )
                })
                .count();
    let mut expected_cubical_ids = source
        .path_quotient
        .as_ref()
        .map(|proof| proof.family_ids.clone())
        .unwrap_or_default();
    expected_cubical_ids.sort();
    let mut observed_cubical_ids = current_members
        .iter()
        .filter(|member| member.presentation.surface() == V5UnifiedSurface::CubicalPath)
        .map(|member| member.family_id.clone())
        .collect::<Vec<_>>();
    observed_cubical_ids.sort();
    let fresh_cubical_inventory_exact = expected_cubical_ids == observed_cubical_ids
        && source
            .path_quotient
            .as_ref()
            .map_or(observed_cubical_ids.is_empty(), |proof| {
                proof.complete
                    && proof.exact_key_coverage
                    && proof.every_key_unique
                    && proof.pair_count_exact
                    && proof.every_pair_decided
                    && proof.no_uniform_coordinate_multiplied
                    && proof.expected_family_count == observed_cubical_ids.len()
            });
    let fresh_core_inventory_digest = tagged_hash(
        "fresh-core-extraction-R1-R2-inventory",
        &(
            &extraction.derivation_hash,
            &extraction.families,
            &fresh_core_extraction_family_ids,
            &observed_core_extraction_ids,
            &r2_quotiented_core_family_ids,
            &source
                .special_cases
                .r2_m1_generated_membership_derivation_hash,
            fresh_core_inventory_exact,
        ),
    );
    let fresh_ordinary_bridge_inventory_digest = tagged_hash(
        "fresh-ordinary-bridge-inventory",
        &(
            &bridges,
            &expected_ordinary_ids,
            fresh_ordinary_inventory_exact,
        ),
    );
    let fresh_cubical_inventory_digest = tagged_hash(
        "fresh-cubical-quotient-inventory",
        &(
            &source.path_quotient,
            &expected_cubical_ids,
            fresh_cubical_inventory_exact,
        ),
    );
    let core_surface_digest = tagged_hash("unified-core-surface", &core_family_ids);
    let ordinary_surface_digest = tagged_hash("unified-ordinary-surface", &ordinary_family_ids);
    let cubical_surface_digest = tagged_hash("unified-cubical-surface", &cubical_family_ids);
    let predecessor_surface_digest =
        tagged_hash("unified-predecessor-surface", predecessor_members);
    let current_surface_digest = tagged_hash("unified-current-surface", &current_members);
    let every_current_member_enumerated_once = current_members
        .iter()
        .map(|member| member.family_id.as_str())
        .collect::<BTreeSet<_>>()
        .len()
        == current_members.len()
        && fresh_core_inventory_exact
        && fresh_ordinary_inventory_exact
        && fresh_cubical_inventory_exact;
    let every_active_member_has_fresh_marginality = current_members
        .iter()
        .filter(|member| member.included_in_semantic_ledger)
        .all(|member| fresh_marginality.contains_key(&member.family_id));
    let generated_instances_mint_no_class = current_members.iter().all(|member| {
        !matches!(
            member.disposition,
            V5UnifiedMemberDisposition::GeneratedUniformInstance
        ) || (!member.included_in_semantic_ledger
            && !member.visible_to_later_predecessor_closure
            && !fresh_marginality.contains_key(&member.family_id))
    });
    let named_residual_count = within_candidate_decisions
        .iter()
        .chain(&predecessor_decisions)
        .filter(|decision| !decision.proved)
        .count()
        + equivalence_closure.named_residual_count
        + fresh_marginality
            .values()
            .filter(|disposition| disposition.is_residual())
            .count();
    let proved = every_current_member_enumerated_once
        && every_within_candidate_pair_decided
        && every_predecessor_pair_decided
        && every_active_member_has_fresh_marginality
        && generated_instances_mint_no_class
        && fresh_core_inventory_exact
        && fresh_ordinary_inventory_exact
        && fresh_cubical_inventory_exact
        && equivalence_closure.proved
        && named_residual_count == 0
        && current_members
            .iter()
            .all(|member| member.typed_normalized_natural)
        && !source.ordinary_registry.archive_or_scalar_input_used;
    let mut quotient = V5UnifiedQuotientProof {
        theorem_id: T_BI_B2_UNIFIED_QUOTIENT_ID.to_owned(),
        stage,
        predecessor_signature_digest: prefix.digest().to_owned(),
        predecessor_closure_digest: closure.digest.clone(),
        predecessor_surface_digest,
        current_surface_digest,
        current_members,
        fresh_core_extraction_family_ids,
        r2_quotiented_core_family_ids,
        fresh_core_inventory_digest,
        fresh_ordinary_bridge_inventory_digest,
        fresh_cubical_inventory_digest,
        core_family_ids,
        ordinary_family_ids,
        cubical_family_ids,
        core_surface_digest,
        ordinary_surface_digest,
        cubical_surface_digest,
        equivalence_closure,
        within_candidate_decisions,
        predecessor_decisions,
        fresh_marginality,
        expected_within_candidate_pair_count,
        expected_predecessor_pair_count,
        every_current_member_enumerated_once,
        fresh_core_inventory_exact,
        fresh_ordinary_inventory_exact,
        fresh_cubical_inventory_exact,
        every_within_candidate_pair_decided,
        every_predecessor_pair_decided,
        every_active_member_has_fresh_marginality,
        generated_instances_mint_no_class,
        v4_marginality_labels_used_as_proof: false,
        named_residual_count,
        proved,
        derivation_hash: String::new(),
    };
    quotient.derivation_hash = tagged_hash("unified-equality-quotient", &quotient);

    let bridge_by_family = bridges
        .iter()
        .map(|bridge| (bridge.semantic_family_id.as_str(), bridge))
        .collect::<BTreeMap<_, _>>();
    let v4_by_family = source
        .unified_families
        .iter()
        .map(|family| (family.family_id.as_str(), family))
        .collect::<BTreeMap<_, _>>();
    let mut families = Vec::new();
    for member in quotient
        .current_members
        .iter()
        .filter(|member| member.included_in_semantic_ledger)
    {
        let marginality = quotient
            .fresh_marginality
            .get(&member.family_id)
            .expect("active member has fresh marginality")
            .clone();
        let source = match &member.presentation {
            V5UnifiedFamilyPresentation::OrdinarySchema2 { kind, .. } => {
                let bridge = bridge_by_family
                    .get(member.family_id.as_str())
                    .ok_or_else(|| {
                        ActLocalSemanticProvenanceV5Error::Invariant(format!(
                            "ordinary quotient member {} has no bridge",
                            member.family_id
                        ))
                    })?;
                V5SemanticFamilySource::OrdinaryBridge {
                    bridge_derivation_hash: bridge.derivation_hash.clone(),
                    kind: *kind,
                }
            }
            _ => {
                if !v4_by_family.contains_key(member.family_id.as_str()) {
                    return Err(ActLocalSemanticProvenanceV5Error::Invariant(format!(
                        "core/cubical quotient member {} has no fresh v4 term evidence",
                        member.family_id
                    )));
                }
                V5SemanticFamilySource::FrozenV4 {
                    v4_family_id: member.family_id.clone(),
                }
            }
        };
        let marginal = marginality.is_marginal();
        let marginality_proof_hash = tagged_hash(
            "fresh-unified-marginality",
            &(
                &member.derivation_hash,
                &marginality,
                &quotient.derivation_hash,
            ),
        );
        let anchor = if marginal {
            V5AnchorDisposition::TheoremImpossibleNoRelation {
                theorem_id: "uninitialized-v5-anchor".to_owned(),
                closure_derivation_hash: String::new(),
                no_constructed_exported_a3_fallback: false,
                proof_hash: String::new(),
            }
        } else if marginality.is_internal() {
            V5AnchorDisposition::Internal
        } else {
            V5AnchorDisposition::NamedResidual {
                gap_id: "T_BI_B2_UNIFIED_MARGINALITY_RESIDUAL".to_owned(),
                exact_reason: "unified marginality did not decide internal versus marginal"
                    .to_owned(),
                exact_a3_capability_derivation_hash: String::new(),
                proof_hash: marginality_proof_hash.clone(),
            }
        };
        let mut witness = V5SemanticFamilyWitness {
            family_id: member.family_id.clone(),
            stage,
            source,
            shape_key: member.presentation_hash.clone(),
            term_or_bridge_derivation_hash: member.typing_normalization_naturality_hash.clone(),
            typed_normalized_natural: member.typed_normalized_natural,
            fresh_marginality: marginality,
            marginal,
            marginality_proof_hash,
            anchor,
            credited: false,
            derivation_hash: String::new(),
        };
        witness.derivation_hash = tagged_hash("semantic-family-witness", &witness);
        families.push(witness);
    }
    families.sort_by(|left, right| left.family_id.cmp(&right.family_id));
    Ok((quotient, families))
}

fn build_exact_a3_capability(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
) -> Result<V5ExactA3CapabilityProof, ActLocalSemanticProvenanceV5Error> {
    let window = generate_a3_window_for_exact_prefix_unbounded(prefix, stage)
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Relation(error.to_string()))?;
    let inventory = prove_a3_window_inventory_for_exact_prefix_unbounded(prefix, stage)
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Relation(error.to_string()))?;
    let schemes = window
        .schemes
        .iter()
        .map(|scheme| (scheme.scheme_id.as_str(), scheme))
        .collect::<BTreeMap<_, _>>();
    let instances = window
        .instances
        .iter()
        .map(|instance| (instance.instance_id.as_str(), instance))
        .collect::<BTreeMap<_, _>>();
    let mut orbit_capabilities = Vec::new();
    for orbit in &window.orbits {
        let scheme = schemes.get(orbit.scheme_id.as_str()).ok_or_else(|| {
            ActLocalSemanticProvenanceV5Error::Relation(format!(
                "fresh A3 orbit {} has no scheme",
                orbit.orbit_id
            ))
        })?;
        let representative = instances
            .get(orbit.representative_instance_id.as_str())
            .ok_or_else(|| {
                ActLocalSemanticProvenanceV5Error::Relation(format!(
                    "fresh A3 orbit {} has no representative instance",
                    orbit.orbit_id
                ))
            })?;
        let scheme_and_instance_join_replayed = representative.scheme_id == scheme.scheme_id
            && orbit.member_instance_ids.iter().all(|instance_id| {
                instances
                    .get(instance_id.as_str())
                    .is_some_and(|instance| instance.scheme_id == scheme.scheme_id)
            })
            && orbit
                .quotient_equalities
                .iter()
                .all(|equality| equality.same_typed_scheme_output);
        let disposition = if !orbit.independently_exported_demand_orbit {
            let quotient_equality_hashes = orbit
                .quotient_equalities
                .iter()
                .map(|equality| equality.equality_derivation_hash.clone())
                .collect::<Vec<_>>();
            V5ExactA3OrbitDisposition::NonExportedInstanceOrbit {
                proof_hash: tagged_hash(
                    "non-exported-A3-instance-orbit",
                    &(
                        &orbit.orbit_id,
                        &orbit.orbit_derivation_hash,
                        &quotient_equality_hashes,
                    ),
                ),
                quotient_equality_hashes,
            }
        } else if scheme.rule_constructor == A3RuleConstructor::StructuralCompletionHole
            && matches!(
                scheme.required_output,
                A3DemandOutputType::StructuralCompletion { .. }
            )
        {
            // Stage 3's demand is registered under the adopted demand-before-
            // jurisdiction transport into Stage 4.  The provider query is
            // therefore made against the post-Stage-3 signature, while the
            // A3 window itself remains bound to the exact predecessor prefix.
            // This is the same typed boundary used by the v2 structural
            // filler coverage theorem; it does not expose a future winner.
            let registration_signature = if stage == 3 {
                let mut entries = prefix
                    .entries()
                    .iter()
                    .map(|entry| (entry.step, entry.telescope.clone()))
                    .collect::<Vec<_>>();
                entries.push((stage, candidate.clone()));
                SealedSignature::from_telescopes(entries)
            } else {
                prefix.clone()
            };
            let registration = register_structural_future_hole_v2(
                &registration_signature,
                &window,
                scheme,
                representative,
            )
            .map_err(|error| ActLocalSemanticProvenanceV5Error::Relation(error.to_string()))?;
            let replay = replay_future_hole_registration_v2(
                &registration_signature,
                &window,
                scheme,
                representative,
                &registration,
            );
            match registration {
                FutureHoleRegistrationDispositionV2::Registered(registration) => {
                    let open_hypothesis = matches!(
                        registration.body,
                        FutureHoleBodyV2::StructuralHypothesis {
                            parameter: 1,
                            body: Expr::Var(1)
                        }
                    );
                    let structural_contract = matches!(
                        registration.output_contract,
                        FutureHoleOutputContractV2::StructuralProvides(_)
                    );
                    let zero_charge = registration.hole_marginal_charge.replays_as_zero();
                    let no_candidate_term_constructed = open_hypothesis
                        && structural_contract
                        && registration.no_reflexivity_fallback
                        && registration.local_semantic_scope_premises_satisfied;
                    let no_credit_anchor_or_orbit_minted = zero_charge
                        && registration.hole_marginal_charge.anchors_minted == 0
                        && registration.hole_marginal_charge.demand_orbits_minted == 0
                        && !registration.hole_marginal_charge.credit_minted;
                    let proof_hash = tagged_hash(
                        "zero-charge-open-A3-hypothesis",
                        &(
                            &registration.formation_hash,
                            &replay.expected_digest,
                            &registration.hole_marginal_charge.charge_hash,
                            no_candidate_term_constructed,
                            no_credit_anchor_or_orbit_minted,
                        ),
                    );
                    V5ExactA3OrbitDisposition::ZeroChargeOpenHypothesis {
                        registration_formation_hash: registration.formation_hash,
                        registration_replay_digest: replay.expected_digest,
                        hole_charge_hash: registration.hole_marginal_charge.charge_hash,
                        no_candidate_term_constructed: replay.valid
                            && no_candidate_term_constructed,
                        no_credit_anchor_or_orbit_minted,
                        proof_hash,
                    }
                }
                FutureHoleRegistrationDispositionV2::Gap(gap) => {
                    V5ExactA3OrbitDisposition::NamedResidual {
                        gap_id: format!("T_BI_B2_A3_CAPABILITY_{}", gap.id),
                        exact_reason: gap.exact_error,
                        evidence_hash: gap.gap_hash,
                    }
                }
            }
        } else {
            V5ExactA3OrbitDisposition::NamedResidual {
                gap_id: "T_BI_B2_LIVE_EXPORTED_A3_OUTPUT_NOT_CLASSIFIED".to_owned(),
                exact_reason: format!(
                    "exported A3 rule {:?} has no adopted output-to-family capability theorem",
                    scheme.rule_constructor
                ),
                evidence_hash: tagged_hash(
                    "unclassified-live-exported-A3-output",
                    &(
                        &orbit.orbit_id,
                        &scheme.scheme_id,
                        &scheme.formation_derivation_hash,
                    ),
                ),
            }
        };
        let mut row = V5ExactA3OrbitCapability {
            orbit_id: orbit.orbit_id.clone(),
            scheme_id: scheme.scheme_id.clone(),
            representative_instance_id: representative.instance_id.clone(),
            member_instance_ids: orbit.member_instance_ids.clone(),
            rule_constructor: format!("{:?}", scheme.rule_constructor),
            required_output: serde_json::to_value(&scheme.required_output)
                .expect("A3 required output serializes"),
            independently_exported: orbit.independently_exported_demand_orbit,
            scheme_and_instance_join_replayed,
            disposition,
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("exact-A3-orbit-capability", &row);
        orbit_capabilities.push(row);
    }
    orbit_capabilities.sort_by(|left, right| left.orbit_id.cmp(&right.orbit_id));
    let constructed_exported_output_count = orbit_capabilities
        .iter()
        .filter(|row| {
            row.independently_exported
                && matches!(
                    row.disposition,
                    V5ExactA3OrbitDisposition::ConstructedExportedOutput { .. }
                )
        })
        .count();
    let named_live_export_residual_count = orbit_capabilities
        .iter()
        .filter(|row| {
            row.independently_exported
                && matches!(
                    row.disposition,
                    V5ExactA3OrbitDisposition::NamedResidual { .. }
                )
        })
        .count();
    let no_constructed_exported_a3_fallback = orbit_capabilities.iter().all(|row| {
        !row.independently_exported
            || matches!(
                row.disposition,
                V5ExactA3OrbitDisposition::ZeroChargeOpenHypothesis {
                    no_candidate_term_constructed: true,
                    no_credit_anchor_or_orbit_minted: true,
                    ..
                }
            )
    });
    let exact_prefix_bound = inventory.exact_prefix_signature_digest == prefix.digest()
        && window.stage == stage
        && inventory.stage == stage;
    let relative_inventory_exhaustive = inventory.relative_rule_inventory_exhaustive_for_window
        && inventory.every_operational_instance_has_exactly_one_independent_preimage
        && inventory.every_operational_scheme_and_orbit_is_consumed
        && inventory.operational_instance_count == window.instances.len();
    let every_fresh_orbit_classified = orbit_capabilities.len() == window.orbits.len()
        && orbit_capabilities
            .iter()
            .all(|row| row.scheme_and_instance_join_replayed);
    let proved = window.finite_by_construction
        && window.every_instance_typed
        && exact_prefix_bound
        && relative_inventory_exhaustive
        && every_fresh_orbit_classified
        && named_live_export_residual_count == 0
        && no_constructed_exported_a3_fallback;
    let mut proof = V5ExactA3CapabilityProof {
        theorem_id: T_BI_B2_A3_CAPABILITY_ID.to_owned(),
        stage,
        prefix_signature_digest: prefix.digest().to_owned(),
        window_derivation_hash: window.window_derivation_hash,
        inventory_exhaustiveness_derivation_hash: inventory.derivation_hash,
        fresh_scheme_count: window.schemes.len(),
        fresh_instance_count: window.instances.len(),
        fresh_orbit_count: window.orbits.len(),
        fresh_exported_orbit_count: orbit_capabilities
            .iter()
            .filter(|row| row.independently_exported)
            .count(),
        orbit_capabilities,
        exact_prefix_bound,
        relative_inventory_exhaustive,
        every_fresh_orbit_classified,
        constructed_exported_output_count,
        named_live_export_residual_count,
        no_constructed_exported_a3_fallback,
        cached_v4_zero_used_as_evidence: false,
        proved,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("exact-prefix-A3-capability-proof", &proof);
    Ok(proof)
}

fn coordinate_u64(occurrence: &ActLocalV3RoleOccurrence, field: &str) -> Option<u64> {
    occurrence
        .coordinate
        .get("coordinates")?
        .get(field)?
        .as_u64()
}

fn is_type_formation_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Trunc(_))
        || matches!(expr, Expr::App(head, _) if matches!(head.as_ref(), Expr::Univ))
}

fn classify_role_rule(
    occurrence: &ActLocalV3RoleOccurrence,
    candidate: &Telescope,
) -> Result<V5RelationRule, ActLocalSemanticProvenanceV5Error> {
    let owner = candidate
        .clauses
        .get(usize::from(occurrence.owner_clause))
        .ok_or_else(|| {
            ActLocalSemanticProvenanceV5Error::Relation(format!(
                "role {} has out-of-bounds owner clause {}",
                occurrence.kind, occurrence.owner_clause
            ))
        })?;
    let rule = match occurrence.kind.as_str() {
        "axiomatic_inherited_family" => V5RelationRule::AxiomaticInheritedFamily,
        "axiomatic_introduction_head" => V5RelationRule::DirectKernelFamily,
        "axiomatic_local_and_bridge_face" => V5RelationRule::AxiomaticLocalAndBridgeFace,
        "axiomatic_support_bridge" => V5RelationRule::AxiomaticSupportBridge,
        "former_adjoint" => V5RelationRule::FormerAdjoint,
        "former_eliminator" | "former_introduction" => V5RelationRule::DirectKernelFamily,
        "foundation_completion" => V5RelationRule::GenericR1Completion,
        "foundation_formation" => V5RelationRule::DirectKernelFamily,
        "hit_formation_package" => match coordinate_u64(occurrence, "package_role") {
            Some(0) => V5RelationRule::HitPathConstructor,
            Some(1) => V5RelationRule::HitRecursor,
            Some(2) => V5RelationRule::HitInductor,
            _ => {
                return Err(ActLocalSemanticProvenanceV5Error::Relation(
                    "HIT formation package has no exact package_role in 0..=2".to_owned(),
                ));
            }
        },
        "hit_kan_coherence" => V5RelationRule::CubicalKan,
        "hit_parametric_formation_action" => V5RelationRule::HitTruncAction,
        "hit_path_beta" => V5RelationRule::CubicalBeta,
        "hit_post_path_face" => V5RelationRule::HitPostPathFace,
        "hit_pre_path_declaration" => {
            if is_type_formation_expr(&owner.expr) {
                V5RelationRule::HitFormation
            } else {
                V5RelationRule::HitPoint
            }
        }
        "map_postcomposition" => V5RelationRule::MapPostcomposition,
        "map_precomposition" => V5RelationRule::MapPrecomposition,
        "map_reference_coherence" => V5RelationRule::MapReferenceCoherence,
        "map_single_action" => V5RelationRule::MapSingleAction,
        "map_single_head" => V5RelationRule::DirectKernelFamily,
        "modal_local_declaration" => V5RelationRule::DirectKernelFamily,
        "modal_pairwise_coherence" => V5RelationRule::ModalPairwiseCoherence,
        "modal_uniform_legacy_action" => V5RelationRule::ModalUniformLegacyAction,
        "synthesis_distributive_transport" => V5RelationRule::SynthesisDistributiveTransport,
        "synthesis_infinitesimal_shift" => V5RelationRule::SynthesisInfinitesimalShift,
        "synthesis_local_declaration" => V5RelationRule::DirectKernelFamily,
        "synthesis_uniform_temporal_action" => V5RelationRule::SynthesisUniformTemporalAction,
        other => {
            return Err(ActLocalSemanticProvenanceV5Error::Relation(format!(
                "historical role kind {other} is outside the explicit 27-kind induction"
            )));
        }
    };
    Ok(rule)
}

fn expected_relation_slot(
    rule: V5RelationRule,
    occurrence: &ActLocalV3RoleOccurrence,
) -> Option<(u16, CreditMechanism, LocalRole)> {
    use CreditMechanism as M;
    use LocalRole as L;
    Some(match rule {
        V5RelationRule::GenericR1Completion => {
            (occurrence.owner_clause, M::IntrinsicKernel, L::KernelHead)
        }
        V5RelationRule::DirectKernelFamily => match occurrence.kind.as_str() {
            "former_eliminator" => (
                occurrence.owner_clause,
                M::AdjointCompletion,
                L::AdjointMate,
            ),
            "axiomatic_introduction_head"
            | "former_introduction"
            | "foundation_formation"
            | "map_single_head"
            | "modal_local_declaration"
            | "synthesis_local_declaration" => {
                (occurrence.owner_clause, M::IntrinsicKernel, L::KernelHead)
            }
            _ => return None,
        },
        V5RelationRule::HitFormation
        | V5RelationRule::HitPoint
        | V5RelationRule::HitPathConstructor => {
            (occurrence.owner_clause, M::IntrinsicKernel, L::KernelHead)
        }
        V5RelationRule::HitRecursor => (
            occurrence.owner_clause,
            M::AdjointCompletion,
            L::AdjointMate,
        ),
        V5RelationRule::HitInductor => (
            occurrence.owner_clause,
            M::IntrinsicKernel,
            L::SupportAction,
        ),
        V5RelationRule::HitTruncAction
        | V5RelationRule::HitPostPathFace
        | V5RelationRule::HitPostPathOperation
        | V5RelationRule::HitPostPathCoherence => (
            occurrence.owner_clause,
            M::P6UniformSpecialization,
            L::SupportAction,
        ),
        V5RelationRule::CubicalBeta => (occurrence.owner_clause, M::IntrinsicKernel, L::KernelHead),
        V5RelationRule::CubicalKan => (occurrence.owner_clause, M::DimensionSquared, L::Coherence),
        V5RelationRule::AxiomaticInheritedFamily => (
            occurrence.owner_clause,
            M::P5InheritedSurface,
            L::SupportAction,
        ),
        V5RelationRule::AxiomaticLocalAndBridgeFace | V5RelationRule::AxiomaticSupportBridge => (
            occurrence.owner_clause,
            M::P5LocalAndBridge,
            L::SupportAction,
        ),
        V5RelationRule::FormerAdjoint
        | V5RelationRule::MapPostcomposition
        | V5RelationRule::MapPrecomposition
        | V5RelationRule::MapSingleAction => (
            occurrence.owner_clause,
            M::AdjointCompletion,
            L::AdjointMate,
        ),
        V5RelationRule::MapReferenceCoherence => {
            (occurrence.owner_clause, M::ReferenceSquared, L::Coherence)
        }
        V5RelationRule::ModalPairwiseCoherence => (
            occurrence.owner_clause,
            M::ModalPairwiseCoherence,
            L::Coherence,
        ),
        V5RelationRule::ModalUniformLegacyAction
        | V5RelationRule::SynthesisUniformTemporalAction => (
            occurrence.owner_clause,
            M::P6UniformSpecialization,
            L::SupportAction,
        ),
        V5RelationRule::SynthesisDistributiveTransport => (
            occurrence.owner_clause,
            M::DistributiveInheritance,
            L::Coherence,
        ),
        V5RelationRule::SynthesisInfinitesimalShift => (
            occurrence.owner_clause,
            M::CombinatorialSynthesis,
            L::SupportAction,
        ),
        V5RelationRule::NoFamilyConstructor => return None,
    })
}

fn coordinate_predicate_replayed(occurrence: &ActLocalV3RoleOccurrence) -> bool {
    let coordinates = occurrence.coordinate.get("coordinates");
    match occurrence.kind.as_str() {
        "hit_formation_package" => {
            matches!(coordinate_u64(occurrence, "package_role"), Some(0..=2))
        }
        "hit_kan_coherence" => {
            coordinate_u64(occurrence, "left_axis").is_some()
                && coordinate_u64(occurrence, "right_axis").is_some()
        }
        "hit_path_beta" => coordinate_u64(occurrence, "dimension").is_some(),
        "map_reference_coherence" => {
            coordinate_u64(occurrence, "left_step").is_some()
                && coordinate_u64(occurrence, "right_step").is_some()
        }
        "modal_pairwise_coherence" => coordinates.is_some_and(|value| {
            value.get("left_kind").and_then(Value::as_str).is_some()
                && value.get("right_kind").and_then(Value::as_str).is_some()
        }),
        "axiomatic_inherited_family" | "synthesis_distributive_transport" => coordinates
            .is_some_and(|value| {
                value.get("source_step").and_then(Value::as_u64).is_some()
                    && value
                        .get("source_family_id")
                        .and_then(Value::as_str)
                        .is_some()
                    && value
                        .get("source_family_evidence")
                        .and_then(Value::as_str)
                        .is_some()
            }),
        "axiomatic_support_bridge" => {
            coordinate_u64(occurrence, "left_step").is_some()
                && coordinate_u64(occurrence, "right_step").is_some()
        }
        "synthesis_infinitesimal_shift" => coordinates.is_some_and(|value| {
            value.get("source_step").and_then(Value::as_u64).is_some()
                && value
                    .get("source_path_clause")
                    .and_then(Value::as_u64)
                    .is_some()
                && value.get("left_axis").and_then(Value::as_u64).is_some()
                && value.get("right_axis").and_then(Value::as_u64).is_some()
        }),
        "modal_uniform_legacy_action" | "synthesis_uniform_temporal_action" => occurrence
            .coordinate
            .get("target")
            .and_then(|target| target.get("step"))
            .and_then(Value::as_u64)
            .is_some(),
        "axiomatic_introduction_head"
        | "axiomatic_local_and_bridge_face"
        | "former_adjoint"
        | "former_eliminator"
        | "former_introduction"
        | "foundation_completion"
        | "foundation_formation"
        | "hit_parametric_formation_action"
        | "hit_post_path_face"
        | "hit_pre_path_declaration"
        | "map_postcomposition"
        | "map_precomposition"
        | "map_single_action"
        | "map_single_head"
        | "modal_local_declaration"
        | "synthesis_local_declaration" => coordinates.is_some(),
        _ => false,
    }
}

fn ordinary_kind_for_rule(rule: V5RelationRule) -> Option<V5OrdinaryKind> {
    match rule {
        V5RelationRule::HitFormation => Some(V5OrdinaryKind::FreshFormation),
        V5RelationRule::HitPoint => Some(V5OrdinaryKind::PointOrUnitIntro),
        V5RelationRule::HitPathConstructor => Some(V5OrdinaryKind::PathConstructorIntro),
        V5RelationRule::HitRecursor => Some(V5OrdinaryKind::Recursor),
        V5RelationRule::HitInductor => Some(V5OrdinaryKind::Inductor),
        V5RelationRule::HitTruncAction => Some(V5OrdinaryKind::TruncParametricAction),
        _ => None,
    }
}

fn mechanism_for_kernel_role(role: ClauseRole) -> (CreditMechanism, LocalRole) {
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

#[derive(Clone, Copy)]
enum V5RoleRegistrySurface<'a> {
    Historical27,
    PrefixLocal(&'a BTreeSet<String>),
}

fn relation_targets_with_registry(
    declaration_id: &str,
    occurrence: &ActLocalV3RoleOccurrence,
    candidate: &Telescope,
    elaboration: &pen_type::elaborate::TelescopeElaboration,
    source: &V5SemanticSourceView,
    bridges: &[V5ExprSchemaBridgeProof],
    families: &[V5SemanticFamilyWitness],
    quotient: &V5UnifiedQuotientProof,
    a3: &V5ExactA3CapabilityProof,
    registry_surface: V5RoleRegistrySurface<'_>,
) -> Result<(V5RoleConstructorSearchProof, Vec<(String, String)>), ActLocalSemanticProvenanceV5Error>
{
    let rule = classify_role_rule(occurrence, candidate)?;
    let registry = match registry_surface {
        V5RoleRegistrySurface::Historical27 => HISTORICAL_ROLE_KINDS
            .iter()
            .map(|kind| (*kind).to_owned())
            .collect::<BTreeSet<_>>(),
        V5RoleRegistrySurface::PrefixLocal(observed) => observed.clone(),
    };
    let registry_index = registry
        .iter()
        .position(|kind| *kind == occurrence.kind)
        .ok_or_else(|| {
            ActLocalSemanticProvenanceV5Error::Relation(format!(
                "{} is absent from the selected closed role grammar surface",
                occurrence.kind
            ))
        })?;
    let owner_index = usize::from(occurrence.owner_clause);
    let owner_clause = candidate.clauses.get(owner_index).ok_or_else(|| {
        ActLocalSemanticProvenanceV5Error::Relation("role owner is out of bounds".to_owned())
    })?;
    let owner_elaboration = elaboration.clauses.get(owner_index).ok_or_else(|| {
        ActLocalSemanticProvenanceV5Error::Relation(
            "role owner has no exact elaboration".to_owned(),
        )
    })?;
    let family_ids = families
        .iter()
        .map(|family| family.family_id.as_str())
        .collect::<BTreeSet<_>>();
    let core_family_ids_checked = quotient.core_family_ids.clone();
    let ordinary_bridge_ids_checked = quotient.ordinary_family_ids.clone();
    let cubical_family_ids_checked = quotient.cubical_family_ids.clone();
    let a3_orbit_ids_checked = a3
        .orbit_capabilities
        .iter()
        .map(|row| row.orbit_id.clone())
        .collect::<Vec<_>>();
    let core_surface_derivation_hash = tagged_hash(
        "role-core-surface-query",
        &(
            declaration_id,
            &core_family_ids_checked,
            &quotient.core_surface_digest,
        ),
    );
    let ordinary_surface_derivation_hash = tagged_hash(
        "role-ordinary-surface-query",
        &(
            declaration_id,
            &ordinary_bridge_ids_checked,
            &quotient.ordinary_surface_digest,
        ),
    );
    let cubical_surface_derivation_hash = tagged_hash(
        "role-cubical-surface-query",
        &(
            declaration_id,
            &cubical_family_ids_checked,
            &quotient.cubical_surface_digest,
        ),
    );
    let a3_surface_derivation_hash = tagged_hash(
        "role-exact-A3-surface-query",
        &(declaration_id, &a3_orbit_ids_checked, &a3.derivation_hash),
    );
    let expected_slot = expected_relation_slot(rule, occurrence);
    let exact_mechanism =
        expected_slot.is_some_and(|(_, mechanism, _)| mechanism == occurrence.mechanism);
    let exact_role = expected_slot.is_some_and(|(_, _, role)| role == occurrence.local_role);
    let exact_coordinate = coordinate_predicate_replayed(occurrence);
    let mut targets = Vec::<(String, String)>::new();
    if matches!(
        rule,
        V5RelationRule::GenericR1Completion | V5RelationRule::DirectKernelFamily
    ) {
        targets.extend(
            source.unified_families
                .iter()
                .filter_map(|family| {
                    let source_exact = match (&rule, &family.source) {
                        (
                            V5RelationRule::GenericR1Completion,
                            V4FamilySource::R1CompletedPackage {
                                completion_clause,
                                generic_r1_derivation_hash,
                                ..
                            },
                        ) => {
                            *completion_clause == occurrence.owner_clause
                                && source
                                    .special_cases
                                    .generic_r1
                                    .as_ref()
                                    .is_some_and(|proof| {
                                    // V4 deliberately left the four carrier
                                    // role cases open, so `proof.proved` is
                                    // false there.  Reuse only its replayable
                                    // typed package premises; V5 discharges
                                    // the four cases below from the live
                                    // constructor surfaces.
                                    proof.adopted_package_clause_replayed
                                        && proof.completion_clause == *completion_clause
                                        && proof.derivation_hash == *generic_r1_derivation_hash
                                        && proof.carrier_is_kernel_formation_type
                                        && proof.completion_is_kernel_formation_type
                                        && proof.completion_is_exact_app_univ_carrier
                                        && proof.dependency_resolves_to_carrier
                                        && proof.completed_action_covers_carrier_by_adopted_r1
                                        && !proof.archive_or_count_input_used
                                })
                        }
                        (
                            V5RelationRule::DirectKernelFamily,
                            V4FamilySource::Clause {
                                generator_clause,
                                kernel_role,
                            },
                        ) => {
                            *generator_clause == occurrence.owner_clause
                                && *kernel_role == owner_elaboration.kernel_role
                                && mechanism_for_kernel_role(*kernel_role)
                                    == (occurrence.mechanism, occurrence.local_role)
                        }
                        _ => false,
                    };
                    let term_exact = family.term.source_term
                        == serde_json::to_value(&owner_clause.expr).expect("Expr serializes")
                        && family.term.normal_form
                            == serde_json::to_value(&owner_elaboration.normal_form)
                                .expect("Expr serializes")
                        && family.term.typing_replayed
                        && family.term.normalization_replayed
                        && family.term.naturality_replayed;
                    (source_exact
                        && term_exact
                        && family_ids.contains(family.family_id.as_str())
                        && exact_coordinate
                        && exact_mechanism
                        && exact_role)
                        .then(|| {
                            let relation_hash = tagged_hash(
                                "fresh-core-term-role-relation",
                                &(
                                    declaration_id,
                                    rule,
                                    &occurrence.coordinate,
                                    &owner_clause.expr,
                                    &owner_elaboration.normal_form,
                                    &owner_elaboration.kernel_ty,
                                    &family.source,
                                    &family.term.derivation_hash,
                                ),
                            );
                            (family.family_id.clone(), relation_hash)
                        })
                })
                .collect::<Vec<_>>(),
        );
    }
    if matches!(
        rule,
        V5RelationRule::CubicalBeta | V5RelationRule::CubicalKan
    ) {
        let principal = coordinate_u64(occurrence, "left_axis");
        let probe = coordinate_u64(occurrence, "right_axis");
        targets.extend(
            source.unified_families
                .iter()
                .filter_map(|family| {
                    let V4FamilySource::CubicalPath {
                        path_clause,
                        key,
                        path_quotient_derivation_hash,
                    } = &family.source
                    else {
                        return None;
                    };
                    let key_holds = match rule {
                        V5RelationRule::CubicalBeta => {
                            key.get("kind").and_then(Value::as_str) == Some("beta")
                        }
                        V5RelationRule::CubicalKan => {
                            key.get("kind").and_then(Value::as_str) == Some("kan")
                                && key.get("principal").and_then(Value::as_u64) == principal
                                && key.get("probe").and_then(Value::as_u64) == probe
                        }
                        _ => false,
                    };
                    (*path_clause == occurrence.owner_clause
                        && key_holds
                        && family_ids.contains(family.family_id.as_str())
                        && family.term.typing_replayed
                        && family.term.normalization_replayed
                        && family.term.naturality_replayed
                        && exact_coordinate
                        && exact_mechanism
                        && exact_role)
                        .then(|| {
                            let relation_hash = tagged_hash(
                                "fresh-cubical-key-role-transport",
                                &(
                                    declaration_id,
                                    rule,
                                    path_clause,
                                    key,
                                    path_quotient_derivation_hash,
                                    &family.term.derivation_hash,
                                    &occurrence.coordinate,
                                ),
                            );
                            (family.family_id.clone(), relation_hash)
                        })
                })
                .collect::<Vec<_>>(),
        );
    }
    if rule == V5RelationRule::HitPostPathFace {
        targets.extend(
            bridges
                .iter()
                .filter(|bridge| {
                    matches!(
                        bridge.kind,
                        V5OrdinaryKind::PostPathOperation | V5OrdinaryKind::PostPathCoherence
                    ) && bridge.source_clauses.contains(&occurrence.owner_clause)
                        && bridge.proved
                        && family_ids.contains(bridge.semantic_family_id.as_str())
                        && exact_coordinate
                        && exact_mechanism
                        && exact_role
                })
                .map(|bridge| {
                    let relation_hash = tagged_hash(
                        "fresh-ordinary-post-path-role-transport",
                        &(
                            declaration_id,
                            bridge.kind,
                            &bridge.source_exprs,
                            &bridge.source_normal_forms,
                            &bridge.derivation_hash,
                            &occurrence.coordinate,
                        ),
                    );
                    (bridge.semantic_family_id.clone(), relation_hash)
                })
                .collect::<Vec<_>>(),
        );
    }
    if let Some(kind) = ordinary_kind_for_rule(rule) {
        targets.extend(
            bridges
                .iter()
                .filter(|bridge| {
                    bridge.kind == kind
                        && bridge.proved
                        && family_ids.contains(bridge.semantic_family_id.as_str())
                        && exact_coordinate
                        && exact_mechanism
                        && exact_role
                        && match rule {
                            V5RelationRule::HitFormation | V5RelationRule::HitPoint => {
                                bridge.source_clauses.contains(&occurrence.owner_clause)
                            }
                            V5RelationRule::HitPathConstructor
                            | V5RelationRule::HitRecursor
                            | V5RelationRule::HitInductor
                            | V5RelationRule::HitTruncAction => {
                                bridge.carrier_clause == occurrence.owner_clause
                            }
                            _ => false,
                        }
                })
                .map(|bridge| {
                    let relation_hash = tagged_hash(
                        "fresh-ordinary-schema-role-transport",
                        &(
                            declaration_id,
                            rule,
                            bridge.kind,
                            &bridge.source_exprs,
                            &bridge.source_normal_forms,
                            &bridge.ordinary_schema_derivation_hash,
                            &bridge.ordinary_typed_realizer_derivation_hash,
                            &bridge.ordinary_normalization_derivation_hash,
                            &bridge.ordinary_naturality_derivation_hash,
                            &occurrence.coordinate,
                        ),
                    );
                    (bridge.semantic_family_id.clone(), relation_hash)
                })
                .collect::<Vec<_>>(),
        );
    }
    for row in &a3.orbit_capabilities {
        if let V5ExactA3OrbitDisposition::ConstructedExportedOutput {
            family_id,
            term_relation_derivation_hash,
            ..
        } = &row.disposition
        {
            if family_ids.contains(family_id.as_str()) {
                targets.push((family_id.clone(), term_relation_derivation_hash.clone()));
            }
        }
    }
    targets.sort();
    targets.dedup();
    let observed_core_ids = families
        .iter()
        .filter(|family| quotient.core_family_ids.contains(&family.family_id))
        .map(|family| family.family_id.clone())
        .collect::<BTreeSet<_>>();
    let observed_ordinary_ids = families
        .iter()
        .filter(|family| quotient.ordinary_family_ids.contains(&family.family_id))
        .map(|family| family.family_id.clone())
        .collect::<BTreeSet<_>>();
    let observed_cubical_ids = families
        .iter()
        .filter(|family| quotient.cubical_family_ids.contains(&family.family_id))
        .map(|family| family.family_id.clone())
        .collect::<BTreeSet<_>>();
    let core_constructor_search_complete = quotient.proved
        && tagged_hash("unified-core-surface", &core_family_ids_checked)
            == quotient.core_surface_digest
        && observed_core_ids
            == core_family_ids_checked
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
        && !core_surface_derivation_hash.is_empty();
    let ordinary_constructor_search_complete = quotient.proved
        && tagged_hash("unified-ordinary-surface", &ordinary_bridge_ids_checked)
            == quotient.ordinary_surface_digest
        && observed_ordinary_ids
            == ordinary_bridge_ids_checked
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
        && !ordinary_surface_derivation_hash.is_empty();
    let cubical_constructor_search_complete = quotient.proved
        && tagged_hash("unified-cubical-surface", &cubical_family_ids_checked)
            == quotient.cubical_surface_digest
        && observed_cubical_ids
            == cubical_family_ids_checked
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
        && !cubical_surface_derivation_hash.is_empty();
    let a3_capability_search_complete = a3.proved
        && a3.no_constructed_exported_a3_fallback
        && !a3.cached_v4_zero_used_as_evidence
        && a3_orbit_ids_checked.len() == a3.fresh_orbit_count
        && a3_orbit_ids_checked.iter().collect::<BTreeSet<_>>().len() == a3_orbit_ids_checked.len()
        && a3
            .orbit_capabilities
            .iter()
            .all(|row| row.scheme_and_instance_join_replayed)
        && !a3_surface_derivation_hash.is_empty();
    let fresh_term_relation_derivation_hashes = targets
        .iter()
        .map(|(_, hash)| hash.clone())
        .collect::<Vec<_>>();
    let exhaustive_empty_search_proved = targets.is_empty()
        && exact_coordinate
        && exact_mechanism
        && exact_role
        && core_constructor_search_complete
        && ordinary_constructor_search_complete
        && cubical_constructor_search_complete
        && a3_capability_search_complete;
    let no_wildcard_or_default_absence_rule = true;
    let registry_surface_proved = match registry_surface {
        V5RoleRegistrySurface::Historical27 => registry.len() == HISTORICAL_ROLE_KINDS.len(),
        V5RoleRegistrySurface::PrefixLocal(observed) => {
            &registry == observed && registry.contains(&occurrence.kind)
        }
    };
    let proved = registry_surface_proved
        && registry_index < registry.len()
        && exact_coordinate
        && exact_mechanism
        && exact_role
        && core_constructor_search_complete
        && ordinary_constructor_search_complete
        && cubical_constructor_search_complete
        && a3_capability_search_complete
        && no_wildcard_or_default_absence_rule
        && !matches!(rule, V5RelationRule::NoFamilyConstructor)
        && (exhaustive_empty_search_proved || !targets.is_empty());
    let mut search = V5RoleConstructorSearchProof {
        theorem_id: T_BI_B2_ROLE_KIND_ID.to_owned(),
        declaration_id: declaration_id.to_owned(),
        role_kind: occurrence.kind.clone(),
        registry_index,
        registry_size: registry.len(),
        registry_exactly_historical_27: matches!(
            registry_surface,
            V5RoleRegistrySurface::Historical27
        ) && registry.len() == HISTORICAL_ROLE_KINDS.len(),
        rule,
        exact_coordinate: occurrence.coordinate.clone(),
        owner_expr: owner_clause.expr.clone(),
        owner_normal_form: owner_elaboration.normal_form.clone(),
        owner_kernel_role: owner_elaboration.kernel_role,
        owner_typing_derivation_hash: tagged_hash("owner-clause-typing", owner_elaboration),
        queried_surfaces: vec![
            V5ConstructorSurface::CoreExpr,
            V5ConstructorSurface::OrdinarySchema2,
            V5ConstructorSurface::CubicalPath,
            V5ConstructorSurface::ExactA3,
        ],
        core_family_ids_checked,
        core_surface_derivation_hash,
        ordinary_bridge_ids_checked,
        ordinary_surface_derivation_hash,
        cubical_family_ids_checked,
        cubical_surface_derivation_hash,
        a3_orbit_ids_checked,
        a3_surface_derivation_hash,
        matching_family_ids: targets.iter().map(|(family, _)| family.clone()).collect(),
        core_constructor_search_complete,
        ordinary_constructor_search_complete,
        cubical_constructor_search_complete,
        a3_capability_search_complete,
        exact_coordinate_predicate_replayed: exact_coordinate,
        exact_mechanism_predicate_replayed: exact_mechanism,
        exact_local_role_predicate_replayed: exact_role,
        no_wildcard_or_default_absence_rule,
        fresh_term_relation_derivation_hashes,
        used_v4_desired_label_equality: false,
        exhaustive_empty_search_proved,
        proved,
        derivation_hash: String::new(),
    };
    search.derivation_hash = tagged_hash("four-surface-role-constructor-search", &search);
    Ok((search, targets))
}

fn relation_targets(
    declaration_id: &str,
    occurrence: &ActLocalV3RoleOccurrence,
    candidate: &Telescope,
    elaboration: &pen_type::elaborate::TelescopeElaboration,
    v4: &ActLocalSemanticProvenanceV4Certificate,
    bridges: &[V5ExprSchemaBridgeProof],
    families: &[V5SemanticFamilyWitness],
    quotient: &V5UnifiedQuotientProof,
    a3: &V5ExactA3CapabilityProof,
) -> Result<(V5RoleConstructorSearchProof, Vec<(String, String)>), ActLocalSemanticProvenanceV5Error>
{
    let source = V5SemanticSourceView::historical(v4);
    relation_targets_with_registry(
        declaration_id,
        occurrence,
        candidate,
        elaboration,
        &source,
        bridges,
        families,
        quotient,
        a3,
        V5RoleRegistrySurface::Historical27,
    )
}

pub fn replay_role_constructor_search_v5(claimed: &V5RoleConstructorSearchProof) -> Vec<String> {
    let mut errors = Vec::new();
    let exact_surfaces = claimed.queried_surfaces
        == vec![
            V5ConstructorSurface::CoreExpr,
            V5ConstructorSurface::OrdinarySchema2,
            V5ConstructorSurface::CubicalPath,
            V5ConstructorSurface::ExactA3,
        ];
    let expected_empty = claimed.matching_family_ids.is_empty()
        && claimed.exact_coordinate_predicate_replayed
        && claimed.exact_mechanism_predicate_replayed
        && claimed.exact_local_role_predicate_replayed
        && claimed.core_constructor_search_complete
        && claimed.ordinary_constructor_search_complete
        && claimed.cubical_constructor_search_complete
        && claimed.a3_capability_search_complete;
    let relation_hashes_exact = claimed.fresh_term_relation_derivation_hashes.len()
        == claimed.matching_family_ids.len()
        && claimed
            .fresh_term_relation_derivation_hashes
            .iter()
            .all(|hash| !hash.is_empty());
    let expected_proved = claimed.theorem_id == T_BI_B2_ROLE_KIND_ID
        && claimed.registry_size == 27
        && claimed.registry_exactly_historical_27
        && claimed.registry_index < 27
        && exact_surfaces
        && claimed.exact_coordinate_predicate_replayed
        && claimed.exact_mechanism_predicate_replayed
        && claimed.exact_local_role_predicate_replayed
        && claimed.core_constructor_search_complete
        && claimed.ordinary_constructor_search_complete
        && claimed.cubical_constructor_search_complete
        && claimed.a3_capability_search_complete
        && claimed.no_wildcard_or_default_absence_rule
        && !claimed.used_v4_desired_label_equality
        && !matches!(claimed.rule, V5RelationRule::NoFamilyConstructor)
        && claimed.exhaustive_empty_search_proved == expected_empty
        && (expected_empty || (!claimed.matching_family_ids.is_empty() && relation_hashes_exact));
    if claimed.proved != expected_proved || !expected_proved {
        errors.push("four-surface constructor-search conclusion mismatch".to_owned());
    }
    let mut projection = claimed.clone();
    projection.derivation_hash.clear();
    if claimed.derivation_hash != tagged_hash("four-surface-role-constructor-search", &projection) {
        errors.push("four-surface constructor-search digest mismatch".to_owned());
    }
    errors
}

fn prefix_local_erasure_row_hash(row: &V5PrefixLocalRoleErasureRow) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-registry-erasure-row", &projection)
}

#[cfg(test)]
pub(crate) fn test_only_prefix_local_erasure_row_hash(
    row: &V5PrefixLocalRoleErasureRow,
) -> String {
    prefix_local_erasure_row_hash(row)
}

fn prefix_local_erasure_proof_hash(proof: &V5PrefixLocalRegistryErasureProof) -> String {
    let mut projection = proof.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-registry-erasure-proof", &projection)
}

#[cfg(test)]
pub(crate) fn test_only_prefix_local_erasure_proof_hash(
    proof: &V5PrefixLocalRegistryErasureProof,
) -> String {
    prefix_local_erasure_proof_hash(proof)
}

/// Project a v5 package onto the semantics that remain after erasing the
/// historical registry's index, size, route hash, and old search digest.
/// Occurrences are rejoined to the candidate-and-prefix v3 declaration
/// surface.  Targets are then recomputed through the registry-free core using
/// only the observed prefix grammar.  The local role injection is rebuilt
/// from those targets instead of trusting the package's credited flags.  The
/// theorem is relative to replayed candidate-local v4 term, typing, ordinary,
/// cubical, quotient, and A3 source fields; it proves that extending the role
/// grammar cannot change the erased semantic projection, not that every v4
/// byte is irrelevant.
pub fn prove_prefix_local_role_registry_erasure_v5(
    prefix: &SealedSignature,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
    v4: &ActLocalSemanticProvenanceV4Certificate,
    package: &ActLocalSemanticProvenanceV5Certificate,
) -> Result<V5PrefixLocalRegistryErasureProof, ActLocalSemanticProvenanceV5Error> {
    let source = V5SemanticSourceView::historical(v4);
    let candidate_digest = candidate_hash(candidate);
    if candidate_digest != package.candidate_hash
        || v3.candidate_hash != candidate_digest
        || v4.candidate_hash != candidate_digest
        || v3.stage != package.stage
        || v4.stage != package.stage
        || v3.predecessor_signature_digest != prefix.digest()
        || v4.predecessor_signature_digest != prefix.digest()
        || package.predecessor_signature_digest != prefix.digest()
        || package.frozen_v3_package_hash != v3.derivation_hash
        || package.frozen_v4_package_hash != v4.derivation_hash
    {
        return Err(ActLocalSemanticProvenanceV5Error::Input(
            "prefix-local erasure candidate/prefix/v3/v4/v5 binding mismatch".to_owned(),
        ));
    }
    let elaboration = elaborate_telescope(prefix, candidate, package.stage.saturating_sub(1))
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Relation(error.to_string()))?;
    let v3_rows = v3
        .natural_family_rows
        .iter()
        .map(|row| (row.derivation_hash.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let v4_by_declaration = v4
        .role_resolutions
        .iter()
        .map(|row| (row.declaration_id.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let package_by_declaration = package
        .role_resolutions
        .iter()
        .map(|row| (row.declaration_id.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let declaration_sets_exact = v4_by_declaration.len() == v4.role_resolutions.len()
        && package_by_declaration.len() == package.role_resolutions.len()
        && v4_by_declaration.keys().eq(package_by_declaration.keys());
    if !declaration_sets_exact {
        return Err(ActLocalSemanticProvenanceV5Error::Relation(
            "prefix-local erasure declaration surfaces do not join exactly".to_owned(),
        ));
    }
    let occurrences = v4
        .role_resolutions
        .iter()
        .map(|frozen| {
            let row = v3_rows.get(frozen.v3_family_row_hash.as_str()).ok_or_else(|| {
                ActLocalSemanticProvenanceV5Error::Relation(format!(
                    "{} has no exact v3 family-row preimage",
                    frozen.declaration_id
                ))
            })?;
            if row.representative_role != frozen.occurrence
                || row.gap_id.as_deref() != Some(frozen.v3_gap_id.as_str())
                || frozen.v3_gap_id != frozen.declaration_id
            {
                return Err(ActLocalSemanticProvenanceV5Error::Relation(format!(
                    "{} does not rederive its occurrence from the v3 prefix surface",
                    frozen.declaration_id
                )));
            }
            Ok(frozen.occurrence.clone())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let sorted_unique_observed_kinds = occurrences
        .iter()
        .map(|occurrence| occurrence.kind.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let prefix_local_grammar = sorted_unique_observed_kinds
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let exact_surfaces = vec![
        V5ConstructorSurface::CoreExpr,
        V5ConstructorSurface::OrdinarySchema2,
        V5ConstructorSurface::CubicalPath,
        V5ConstructorSurface::ExactA3,
    ];
    let mut rows = Vec::new();
    for (frozen, occurrence) in v4.role_resolutions.iter().zip(&occurrences) {
        let resolution = package_by_declaration
            .get(frozen.declaration_id.as_str())
            .copied()
            .expect("declaration sets proved equal");
        let direct_v5_rule = classify_role_rule(occurrence, candidate)?;
        let (local_search, mut targets) = relation_targets_with_registry(
            &frozen.declaration_id,
            occurrence,
            candidate,
            &elaboration,
            &source,
            &package.bridges,
            &package.semantic_families,
            &package.unified_quotient,
            &package.exact_a3_capability,
            V5RoleRegistrySurface::PrefixLocal(&prefix_local_grammar),
        )?;
        targets.sort();
        targets.dedup();
        const UNUSED_EXTENSION_SENTINEL: &str =
            "__t_bi_prefix_local_unused_registry_extension_sentinel__";
        if prefix_local_grammar.contains(UNUSED_EXTENSION_SENTINEL) {
            return Err(ActLocalSemanticProvenanceV5Error::Invariant(
                "prefix role grammar collides with the extension sentinel".to_owned(),
            ));
        }
        let mut extended_grammar = prefix_local_grammar.clone();
        extended_grammar.insert(UNUSED_EXTENSION_SENTINEL.to_owned());
        let (extended_search, mut extended_targets) = relation_targets_with_registry(
            &frozen.declaration_id,
            occurrence,
            candidate,
            &elaboration,
            &source,
            &package.bridges,
            &package.semantic_families,
            &package.unified_quotient,
            &package.exact_a3_capability,
            V5RoleRegistrySurface::PrefixLocal(&extended_grammar),
        )?;
        extended_targets.sort();
        extended_targets.dedup();
        let erased_search_projection = |search: &V5RoleConstructorSearchProof| {
            serde_json::json!({
                "declaration_id": search.declaration_id,
                "role_kind": search.role_kind,
                "rule": search.rule,
                "exact_coordinate": search.exact_coordinate,
                "owner_expr": search.owner_expr,
                "owner_normal_form": search.owner_normal_form,
                "owner_kernel_role": search.owner_kernel_role,
                "owner_typing_derivation_hash": search.owner_typing_derivation_hash,
                "queried_surfaces": search.queried_surfaces,
                "core_family_ids_checked": search.core_family_ids_checked,
                "core_surface_derivation_hash": search.core_surface_derivation_hash,
                "ordinary_bridge_ids_checked": search.ordinary_bridge_ids_checked,
                "ordinary_surface_derivation_hash": search.ordinary_surface_derivation_hash,
                "cubical_family_ids_checked": search.cubical_family_ids_checked,
                "cubical_surface_derivation_hash": search.cubical_surface_derivation_hash,
                "a3_orbit_ids_checked": search.a3_orbit_ids_checked,
                "a3_surface_derivation_hash": search.a3_surface_derivation_hash,
                "core_complete": search.core_constructor_search_complete,
                "ordinary_complete": search.ordinary_constructor_search_complete,
                "cubical_complete": search.cubical_constructor_search_complete,
                "a3_complete": search.a3_capability_search_complete,
                "coordinate_replayed": search.exact_coordinate_predicate_replayed,
                "mechanism_replayed": search.exact_mechanism_predicate_replayed,
                "local_role_replayed": search.exact_local_role_predicate_replayed,
                "no_default": search.no_wildcard_or_default_absence_rule,
                "desired_label_used": search.used_v4_desired_label_equality,
                "empty_search_proved": search.exhaustive_empty_search_proved,
                "proved": search.proved,
            })
        };
        let unused_registry_extension_projection_equal = extended_grammar.len()
            == prefix_local_grammar.len() + 1
            && erased_search_projection(&local_search)
                == erased_search_projection(&extended_search)
            && targets == extended_targets;
        let target_family_ids = targets
            .iter()
            .map(|(family_id, _)| family_id.clone())
            .collect::<Vec<_>>();
        let target_relation_hashes = targets
            .iter()
            .map(|(_, relation_hash)| relation_hash.clone())
            .collect::<Vec<_>>();
        let local_resolution_is_family = target_family_ids.len() == 1;
        let resolution_class = if local_resolution_is_family {
            V5PrefixLocalResolutionClass::ProvedFamily
        } else {
            V5PrefixLocalResolutionClass::TheoremBackedImpossibility
        };
        let resolved_family_id = local_resolution_is_family.then(|| target_family_ids[0].clone());
        let resolution_exact_for_targets = local_search.proved
            && (local_resolution_is_family
                || (target_family_ids.len() != 1
                    && package.exact_a3_capability.no_constructed_exported_a3_fallback));
        let legacy_resolution_projection_matches = match &resolution.resolution {
            V5RoleResolution::ProvedFamily { proof } => {
                local_resolution_is_family
                    && resolution.resolved
                    && proof.proved
                    && proof.rule == direct_v5_rule
                    && proof.occurrence == *occurrence
                    && target_family_ids == vec![proof.family_id.clone()]
            }
            V5RoleResolution::TheoremBackedImpossibility { proof } => {
                !local_resolution_is_family
                    && resolution.resolved
                    && proof.proved
                    && proof.constructor_induction_rule == direct_v5_rule
                    && proof.occurrence == *occurrence
                    && proof.matching_constructed_family_ids == target_family_ids
                    && proof.no_unique_family_relation
                    && proof.no_registered_family_relation == target_family_ids.is_empty()
            }
            V5RoleResolution::NamedResidual { .. } => false,
        };
        let direct_rule_reclassification_exact = local_search.declaration_id
            == resolution.declaration_id
            && local_search.role_kind == occurrence.kind
            && local_search.rule == direct_v5_rule;
        let observed_kind_has_unique_grammar_entry =
            prefix_local_grammar.contains(&occurrence.kind)
                && !matches!(direct_v5_rule, V5RelationRule::NoFamilyConstructor);
        let no_default_rule = !matches!(direct_v5_rule, V5RelationRule::NoFamilyConstructor)
            && local_search.no_wildcard_or_default_absence_rule;
        let mut row = V5PrefixLocalRoleErasureRow {
            declaration_id: resolution.declaration_id.clone(),
            role_kind: occurrence.kind.clone(),
            occurrence_hash: tagged_hash("prefix-local-role-occurrence", occurrence),
            occurrence_rederived_from_v3_prefix: true,
            observed_kind_has_unique_grammar_entry,
            direct_v5_rule,
            direct_rule_reclassification_exact,
            exact_coordinate_predicate_replayed: local_search
                .exact_coordinate_predicate_replayed,
            exact_mechanism_predicate_replayed: local_search
                .exact_mechanism_predicate_replayed,
            exact_local_role_predicate_replayed: local_search
                .exact_local_role_predicate_replayed,
            queried_surfaces: local_search.queried_surfaces.clone(),
            core_surface_digest: local_search.core_surface_derivation_hash.clone(),
            ordinary_surface_digest: local_search.ordinary_surface_derivation_hash.clone(),
            cubical_surface_digest: local_search.cubical_surface_derivation_hash.clone(),
            a3_surface_digest: local_search.a3_surface_derivation_hash.clone(),
            target_family_ids,
            target_relation_hashes,
            resolution_class,
            resolved_family_id,
            local_role_clause: occurrence.owner_clause,
            local_role: occurrence.local_role,
            resolution_exact_for_targets,
            legacy_resolution_projection_matches,
            unused_registry_extension_projection_equal,
            no_default_rule,
            v4_registry_route_hash_in_authoritative_projection: false,
            registry_index_or_size_in_authoritative_projection: false,
            derivation_hash: String::new(),
        };
        row.derivation_hash = prefix_local_erasure_row_hash(&row);
        rows.push(row);
    }
    rows.sort_by(|left, right| left.declaration_id.cmp(&right.declaration_id));
    let v3_occurrence_surface_replayed = rows.len() == v4.role_resolutions.len()
        && rows
            .iter()
            .all(|row| row.occurrence_rederived_from_v3_prefix);
    let every_occurrence_has_one_direct_grammar_entry = rows.iter().all(|row| {
        row.observed_kind_has_unique_grammar_entry
            && row.direct_rule_reclassification_exact
            && row.no_default_rule
    });
    let direct_closed_rule_induction_exhaustive = rows.iter().all(|row| {
        row.direct_rule_reclassification_exact
            && row.no_default_rule
            && row.queried_surfaces == exact_surfaces
            && row.exact_coordinate_predicate_replayed
            && row.exact_mechanism_predicate_replayed
            && row.exact_local_role_predicate_replayed
            && !row.core_surface_digest.is_empty()
            && !row.ordinary_surface_digest.is_empty()
            && !row.cubical_surface_digest.is_empty()
            && !row.a3_surface_digest.is_empty()
            && !row.v4_registry_route_hash_in_authoritative_projection
            && !row.registry_index_or_size_in_authoritative_projection
    });
    let target_computation_registry_extension_invariant =
        direct_closed_rule_induction_exhaustive
            && rows.iter().all(|row| {
                row.unused_registry_extension_projection_equal
                    && row.target_family_ids.len() == row.target_relation_hashes.len()
                    && row.target_relation_hashes.iter().all(|hash| !hash.is_empty())
            });
    let prefix_local_b1_proved = package.bridges.len()
        == v4.ordinary_registry.applicable_schema_count
        && package.bridges.iter().all(|bridge| bridge.proved)
        && !v4.ordinary_registry.archive_or_scalar_input_used;
    let prefix_local_finite_closure_proved = package
        .finite_closure
        .every_candidate_clause_in_extraction
        && package.finite_closure.every_ordinary_schema_bridged
        && package.finite_closure.path_quotient_complete
        && package.finite_closure.unified_quotient_complete
        && package.finite_closure.exact_a3_complete
        && package.finite_closure.exact_a3_fallback_exclusion_proved
        && package.unified_quotient.proved
        && package.exact_a3_capability.proved
        && package.exact_a3_capability.no_constructed_exported_a3_fallback
        && prefix_local_b1_proved
        && every_occurrence_has_one_direct_grammar_entry;
    let resolution_registry_extension_invariant = prefix_local_finite_closure_proved
        && target_computation_registry_extension_invariant
        && rows.iter().all(|row| row.resolution_exact_for_targets);

    let mut relations = BTreeMap::<String, Vec<(u16, LocalRole)>>::new();
    for row in &rows {
        if row.resolution_class == V5PrefixLocalResolutionClass::ProvedFamily {
            let family_id = row
                .resolved_family_id
                .as_ref()
                .expect("proved-family erasure row has a family");
            relations
                .entry(family_id.clone())
                .or_default()
                .push((row.local_role_clause, row.local_role));
        }
    }
    for slots in relations.values_mut() {
        slots.sort();
        slots.dedup();
    }
    let mut slot_claims = BTreeMap::<(u16, LocalRole), BTreeSet<String>>::new();
    for family in package.semantic_families.iter().filter(|family| family.marginal) {
        let slots = relations
            .get(&family.family_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect::<BTreeSet<_>>();
        if slots.len() == 1 {
            slot_claims
                .entry(*slots.iter().next().expect("singleton"))
                .or_default()
                .insert(family.family_id.clone());
        }
    }
    let mut credited_family_ids = package
        .semantic_families
        .iter()
        .filter(|family| {
            if !family.marginal {
                return false;
            }
            let slots = relations
                .get(&family.family_id)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .collect::<BTreeSet<_>>();
            slots.len() == 1
                && slot_claims
                    .get(slots.iter().next().expect("singleton"))
                    .is_some_and(|claimants| claimants.len() == 1)
        })
        .map(|family| family.family_id.clone())
        .collect::<Vec<_>>();
    credited_family_ids.sort();
    credited_family_ids.dedup();
    let mut legacy_package_credited_family_ids = package
        .semantic_families
        .iter()
        .filter(|family| family.credited)
        .map(|family| family.family_id.clone())
        .collect::<Vec<_>>();
    legacy_package_credited_family_ids.sort();
    legacy_package_credited_family_ids.dedup();
    let credited_family_projection_exact =
        credited_family_ids == legacy_package_credited_family_ids;
    let recomputed_semantic_nu = u32::try_from(credited_family_ids.len()).map_err(|_| {
        ActLocalSemanticProvenanceV5Error::Invariant(
            "prefix-local credited-family count exceeds u32".to_owned(),
        )
    })?;
    let semantic_nu_registry_extension_invariant = resolution_registry_extension_invariant
        && credited_family_projection_exact
        && recomputed_semantic_nu == package.semantic_family_nu
        && credited_family_ids.len() == package.credited_semantic_family_count;
    let stage1_r1_rows = rows
        .iter()
        .filter(|row| {
            row.direct_v5_rule == V5RelationRule::GenericR1Completion
                && row.resolution_class == V5PrefixLocalResolutionClass::ProvedFamily
                && row.resolution_exact_for_targets
        })
        .collect::<Vec<_>>();
    let stage1_local = package.stage != 1
        || (candidate.clauses.len() == 2
            && candidate.clauses[0].expr == Expr::Univ
            && candidate.clauses[1].expr
                == Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))
            && stage1_r1_rows.len() == 1
            && stage1_r1_rows[0]
                .resolved_family_id
                .as_ref()
                .is_some_and(|family_id| {
                    credited_family_ids == vec![family_id.clone()]
                        && v4.unified_families.iter().any(|family| {
                            family.family_id == *family_id
                                && matches!(
                                    family.source,
                                    V4FamilySource::R1CompletedPackage { .. }
                                )
                        })
                })
            && recomputed_semantic_nu == 1);
    let stage2_local = package.stage != 2
        || (recomputed_semantic_nu == 0
            && package
                .semantic_families
                .iter()
                .all(|family| !family.marginal && !credited_family_ids.contains(&family.family_id)));
    let stage9_local = package.stage != 9
        || (rows.len() == 17
            && rows.iter().all(|row| {
                row.role_kind.starts_with("map_") && row.resolution_exact_for_targets
            }));
    let r2_local = package.bridges.iter().filter(|bridge| bridge.generated_instance).all(
        |bridge| {
            !package
                .semantic_families
                .iter()
                .any(|family| family.family_id == bridge.semantic_family_id)
        },
    );
    let special_case_projection_rebuilt_without_registry_hashes =
        stage1_local && stage2_local && stage9_local && r2_local;
    let local_anchor_nonreuse = {
        let mut credited_slots = Vec::new();
        for family_id in &credited_family_ids {
            let slots = relations
                .get(family_id)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .collect::<BTreeSet<_>>();
            if slots.len() != 1 {
                return Err(ActLocalSemanticProvenanceV5Error::Invariant(
                    "credited prefix-local family lacks one exact local role".to_owned(),
                ));
            }
            credited_slots.push(*slots.iter().next().expect("singleton"));
        }
        credited_slots.iter().copied().collect::<BTreeSet<_>>().len()
            == credited_slots.len()
    };
    let every_marginal_family_locally_credited_or_theorem_impossible = package
        .semantic_families
        .iter()
        .filter(|family| family.marginal)
        .all(|family| {
            let slots = relations
                .get(&family.family_id)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .collect::<BTreeSet<_>>();
            let credited = credited_family_ids.contains(&family.family_id);
            let theorem_impossible = slots.is_empty()
                || slots.len() > 1
                || (slots.len() == 1
                    && slot_claims
                        .get(slots.iter().next().expect("singleton"))
                        .is_none_or(|claimants| claimants.len() != 1));
            credited || theorem_impossible
        });
    let prefix_local_b2_proved = prefix_local_finite_closure_proved
        && resolution_registry_extension_invariant
        && local_anchor_nonreuse
        && every_marginal_family_locally_credited_or_theorem_impossible
        && special_case_projection_rebuilt_without_registry_hashes
        && package.named_quotient_residual_count == 0
        && package.named_a3_residual_count == 0;
    let package_projection_exact = semantic_nu_registry_extension_invariant
        && rows
            .iter()
            .all(|row| row.legacy_resolution_projection_matches)
        && package.named_quotient_residual_count == 0
        && package.named_a3_residual_count == 0
        && package.named_role_residual_count == 0
        && package.silent_residue_count == 0
        && package.t_bi_b1_proved == prefix_local_b1_proved
        && package.t_bi_b2_proved == prefix_local_b2_proved;
    let historical_registry_suffix_used_as_semantic_premise = false;
    let old_v4_v5_full_hashes_authoritative_for_prefix_theorem = false;
    let stage9_historical_boundary_hash_in_authoritative_projection = false;
    let proved = v3_occurrence_surface_replayed
        && every_occurrence_has_one_direct_grammar_entry
        && direct_closed_rule_induction_exhaustive
        && target_computation_registry_extension_invariant
        && resolution_registry_extension_invariant
        && prefix_local_b1_proved
        && prefix_local_b2_proved
        && semantic_nu_registry_extension_invariant
        && special_case_projection_rebuilt_without_registry_hashes
        && package_projection_exact
        && !historical_registry_suffix_used_as_semantic_premise
        && !old_v4_v5_full_hashes_authoritative_for_prefix_theorem
        && !stage9_historical_boundary_hash_in_authoritative_projection;
    let mut proof = V5PrefixLocalRegistryErasureProof {
        theorem_id: T_BI_B2_PREFIX_LOCAL_REGISTRY_ERASURE_ID.to_owned(),
        stage: package.stage,
        candidate_hash: package.candidate_hash.clone(),
        predecessor_signature_digest: package.predecessor_signature_digest.clone(),
        v3_occurrence_surface_replayed,
        v4_declaration_surface_joined_only_as_legacy_witness: true,
        sorted_unique_observed_kinds,
        rows,
        every_occurrence_has_one_direct_grammar_entry,
        direct_closed_rule_induction_exhaustive,
        target_computation_registry_extension_invariant,
        resolution_registry_extension_invariant,
        prefix_local_finite_closure_proved,
        prefix_local_b1_proved,
        prefix_local_b2_proved,
        every_marginal_family_locally_credited_or_theorem_impossible,
        credited_family_ids,
        legacy_package_credited_family_ids,
        credited_family_projection_exact,
        recomputed_semantic_nu,
        package_semantic_nu: package.semantic_family_nu,
        semantic_nu_registry_extension_invariant,
        named_role_residual_count: package.named_role_residual_count,
        named_quotient_residual_count: package.named_quotient_residual_count,
        named_a3_residual_count: package.named_a3_residual_count,
        silent_residue_count: package.silent_residue_count,
        t_bi_b1_proved: package.t_bi_b1_proved,
        t_bi_b2_proved: package.t_bi_b2_proved,
        special_case_projection_rebuilt_without_registry_hashes,
        package_projection_exact,
        historical_registry_suffix_used_as_semantic_premise,
        old_v4_v5_full_hashes_authoritative_for_prefix_theorem,
        stage9_historical_boundary_hash_in_authoritative_projection,
        proved,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = prefix_local_erasure_proof_hash(&proof);
    Ok(proof)
}

pub fn replay_prefix_local_role_registry_erasure_v5(
    prefix: &SealedSignature,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
    v4: &ActLocalSemanticProvenanceV4Certificate,
    package: &ActLocalSemanticProvenanceV5Certificate,
    claimed: &V5PrefixLocalRegistryErasureProof,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != prefix_local_erasure_proof_hash(claimed) {
        errors.push("prefix-local registry-erasure proof digest mismatch".to_owned());
    }
    match prove_prefix_local_role_registry_erasure_v5(prefix, candidate, v3, v4, package) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push(
            "prefix-local registry-erasure proof differs from direct reissuance".to_owned(),
        ),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn build_closure_proof(
    candidate: &Telescope,
    extraction: &pen_eval::typed_families::CandidateFamilyExtraction,
    source: &V5SemanticSourceView,
    bridges: &[V5ExprSchemaBridgeProof],
    quotient: &V5UnifiedQuotientProof,
    a3: &V5ExactA3CapabilityProof,
    role_kind_registry_exhaustive: bool,
) -> V5FiniteClosureProof {
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
    let path_quotient_complete = source
        .path_quotient
        .as_ref()
        .map_or(true, |proof| proof.complete);
    let every_ordinary_schema_bridged = bridges.len()
        == source.ordinary_registry.applicable_schema_count
        && bridges.iter().all(|bridge| bridge.proved);
    let proved = extracted_clauses == expected_clauses
        && every_ordinary_schema_bridged
        && path_quotient_complete
        && quotient.proved
        && a3.proved
        && role_kind_registry_exhaustive
        && !source.ordinary_registry.archive_or_scalar_input_used;
    let mut proof = V5FiniteClosureProof {
        theorem_id: T_BI_B2_CLOSURE_ID.to_owned(),
        candidate_clause_count: candidate.clauses.len(),
        extracted_clause_family_count: extraction.families.len(),
        frozen_v4_unified_family_count: source.unified_families.len(),
        ordinary_applicable_schema_count: source.ordinary_registry.applicable_schema_count,
        ordinary_bridge_count: bridges.len(),
        cubical_family_count: source
            .unified_families
            .iter()
            .filter(|family| matches!(family.source, V4FamilySource::CubicalPath { .. }))
            .count(),
        unified_quotient_derivation_hash: quotient.derivation_hash.clone(),
        core_surface_digest: quotient.core_surface_digest.clone(),
        ordinary_surface_digest: quotient.ordinary_surface_digest.clone(),
        cubical_surface_digest: quotient.cubical_surface_digest.clone(),
        exact_a3_orbit_count: a3.fresh_orbit_count,
        exact_a3_usable_output_count: a3.constructed_exported_output_count,
        exact_a3_capability_derivation_hash: a3.derivation_hash.clone(),
        exact_a3_fallback_exclusion_proved: a3.no_constructed_exported_a3_fallback,
        every_candidate_clause_in_extraction: extracted_clauses == expected_clauses,
        every_ordinary_schema_bridged,
        path_quotient_complete,
        unified_quotient_complete: quotient.proved,
        exact_a3_complete: a3.proved,
        role_kind_registry_exhaustive,
        archive_or_scalar_input_used: false,
        proved,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("finite-semantic-closure-proof", &proof);
    proof
}

#[derive(Clone, Debug)]
struct V5RoleDeclarationInput {
    declaration_id: String,
    occurrence: ActLocalV3RoleOccurrence,
    source_resolution_hash: String,
}

fn historical_role_declaration_inputs(
    v4: &ActLocalSemanticProvenanceV4Certificate,
) -> Vec<V5RoleDeclarationInput> {
    v4.role_resolutions
        .iter()
        .map(|frozen| V5RoleDeclarationInput {
            declaration_id: frozen.declaration_id.clone(),
            occurrence: frozen.occurrence.clone(),
            source_resolution_hash: frozen.derivation_hash.clone(),
        })
        .collect()
}

fn prefix_local_role_declaration_inputs(
    v3: &ActLocalProvenanceV3Certificate,
) -> Result<Vec<V5RoleDeclarationInput>, ActLocalSemanticProvenanceV5Error> {
    let mut inputs = v3
        .theorem_gaps
        .iter()
        .filter(|gap| gap.kind == "ROLE_SCHEMA_EXTRACTION_GAP")
        .map(|gap| {
            let family_id = gap.family_id.as_ref().ok_or_else(|| {
                ActLocalSemanticProvenanceV5Error::Relation(format!(
                    "prefix-local role gap {} has no family ID",
                    gap.id
                ))
            })?;
            let row = v3
                .natural_family_rows
                .iter()
                .find(|row| &row.semantic_family_id == family_id)
                .ok_or_else(|| {
                    ActLocalSemanticProvenanceV5Error::Relation(format!(
                        "prefix-local role gap {} has no natural-family row",
                        gap.id
                    ))
                })?;
            if row.gap_id.as_deref() != Some(gap.id.as_str()) {
                return Err(ActLocalSemanticProvenanceV5Error::Relation(format!(
                    "prefix-local role gap {} is not the row's exact gap",
                    gap.id
                )));
            }
            Ok(V5RoleDeclarationInput {
                declaration_id: gap.id.clone(),
                occurrence: row.representative_role.clone(),
                source_resolution_hash: tagged_hash(
                    "prefix-local-v3-role-declaration-source",
                    &(
                        gap.id.as_str(),
                        row.semantic_family_id.as_str(),
                        row.derivation_hash.as_str(),
                    ),
                ),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    inputs.sort_by(|left, right| left.declaration_id.cmp(&right.declaration_id));
    if inputs
        .iter()
        .map(|input| input.declaration_id.as_str())
        .collect::<BTreeSet<_>>()
        .len()
        != inputs.len()
    {
        return Err(ActLocalSemanticProvenanceV5Error::Relation(
            "prefix-local role declarations are not uniquely named".to_owned(),
        ));
    }
    Ok(inputs)
}

fn resolve_roles(
    stage: u32,
    candidate: &Telescope,
    elaboration: &pen_type::elaborate::TelescopeElaboration,
    source: &V5SemanticSourceView,
    bridges: &[V5ExprSchemaBridgeProof],
    families: &[V5SemanticFamilyWitness],
    quotient: &V5UnifiedQuotientProof,
    closure: &V5FiniteClosureProof,
    a3: &V5ExactA3CapabilityProof,
    declarations: &[V5RoleDeclarationInput],
    registry_surface: V5RoleRegistrySurface<'_>,
) -> Result<Vec<V5RoleDeclarationResolution>, ActLocalSemanticProvenanceV5Error> {
    let all_family_ids = families
        .iter()
        .map(|family| family.family_id.clone())
        .collect::<Vec<_>>();
    let mut resolutions = Vec::new();
    for declaration in declarations {
        let occurrence = &declaration.occurrence;
        let (constructor_search, mut targets) = relation_targets_with_registry(
            &declaration.declaration_id,
            occurrence,
            candidate,
            elaboration,
            source,
            bridges,
            families,
            quotient,
            a3,
            registry_surface,
        )?;
        targets.sort();
        targets.dedup();
        let resolution = if closure.proved && constructor_search.proved && targets.len() == 1 {
            let (family_id, evidence_hash) = targets[0].clone();
            let mut proof = V5FamilyRoleRelationProof {
                theorem_id: T_BI_B2_THEOREM_ID.to_owned(),
                declaration_id: declaration.declaration_id.clone(),
                occurrence: occurrence.clone(),
                rule: constructor_search.rule,
                family_id,
                family_term_or_bridge_derivation_hash: families
                    .iter()
                    .find(|family| family.family_id == targets[0].0)
                    .map(|family| family.term_or_bridge_derivation_hash.clone())
                    .unwrap_or_default(),
                fresh_term_relation_derivation_hash: evidence_hash,
                declaration_reissued_from_candidate_and_prefix: true,
                exact_constructor_coordinate_replayed: constructor_search
                    .exact_coordinate_predicate_replayed,
                exact_owner_clause_replayed: true,
                exact_mechanism_replayed: constructor_search.exact_mechanism_predicate_replayed,
                exact_local_role_replayed: constructor_search.exact_local_role_predicate_replayed,
                label_candidate_used_as_proof: false,
                v4_desired_label_equality_used_as_proof: false,
                additional_credit_minted: false,
                proved: true,
                derivation_hash: String::new(),
            };
            proof.derivation_hash = tagged_hash("exact-family-role-relation", &proof);
            V5RoleResolution::ProvedFamily { proof }
        } else if closure.proved
            && constructor_search.proved
            && (constructor_search.exhaustive_empty_search_proved || targets.len() > 1)
        {
            let no_relation = targets.is_empty();
            let no_unique_relation = targets.len() != 1;
            let mut proof = V5RoleImpossibilityProof {
                theorem_id: T_BI_B2_THEOREM_ID.to_owned(),
                declaration_id: declaration.declaration_id.clone(),
                occurrence: occurrence.clone(),
                constructor_induction_rule: constructor_search.rule,
                finite_closure_derivation_hash: closure.derivation_hash.clone(),
                candidate_family_ids_checked: all_family_ids.clone(),
                exact_a3_usable_output_count: a3.constructed_exported_output_count,
                exact_a3_capability_derivation_hash: a3.derivation_hash.clone(),
                every_relation_constructor_checked: closure.proved
                    && constructor_search.proved,
                matching_constructed_family_ids: targets
                    .iter()
                    .map(|(family_id, _)| family_id)
                    .cloned()
                    .collect(),
                no_registered_family_relation: no_relation,
                no_unique_family_relation: no_unique_relation,
                no_constructed_exported_a3_fallback: a3
                    .no_constructed_exported_a3_fallback,
                conclusion: "the declaration has no unique proof-bearing family relation in the exact core/ordinary/cubical/A3 closure".to_owned(),
                proved: closure.proved
                    && constructor_search.proved
                    && no_unique_relation
                    && a3.no_constructed_exported_a3_fallback,
                derivation_hash: String::new(),
            };
            proof.derivation_hash = tagged_hash("role-relation-impossibility", &proof);
            V5RoleResolution::TheoremBackedImpossibility { proof }
        } else {
            let exact_reason = if !a3.proved {
                "the fresh exact-prefix A3 capability classification has a live residual"
            } else if !constructor_search.proved {
                "the explicit four-surface constructor search did not prove"
            } else if !closure.proved {
                "the finite candidate/ordinary/cubical/A3 closure did not prove"
            } else {
                "the relation has neither a unique target nor an exhaustive emptiness/collision proof"
            };
            let mut proof = V5NamedRoleResidual {
                gap_id: format!(
                    "T_BI_B2_ROLE_RELATION_RESIDUAL_S{stage}_{}",
                    declaration.declaration_id
                ),
                declaration_id: declaration.declaration_id.clone(),
                occurrence: occurrence.clone(),
                constructor_search_derivation_hash: constructor_search.derivation_hash.clone(),
                exact_reason: exact_reason.to_owned(),
                missing_theorem: if !a3.proved {
                    T_BI_B2_A3_CAPABILITY_ID.to_owned()
                } else {
                    T_BI_B2_ROLE_KIND_ID.to_owned()
                },
                derivation_hash: String::new(),
            };
            proof.derivation_hash = tagged_hash("named-role-relation-residual", &proof);
            V5RoleResolution::NamedResidual { proof }
        };
        let resolved = match &resolution {
            V5RoleResolution::ProvedFamily { proof } => proof.proved,
            V5RoleResolution::TheoremBackedImpossibility { proof } => proof.proved,
            V5RoleResolution::NamedResidual { .. } => false,
        };
        let mut row = V5RoleDeclarationResolution {
            stage,
            declaration_id: declaration.declaration_id.clone(),
            frozen_v4_resolution_hash: declaration.source_resolution_hash.clone(),
            constructor_search,
            resolution,
            resolved,
            silent_residue: false,
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("role-declaration-resolution", &row);
        resolutions.push(row);
    }
    resolutions.sort_by(|left, right| left.declaration_id.cmp(&right.declaration_id));
    Ok(resolutions)
}

fn anchor_families(
    stage: u32,
    candidate_digest: &str,
    closure: &V5FiniteClosureProof,
    a3: &V5ExactA3CapabilityProof,
    families: &mut [V5SemanticFamilyWitness],
    resolutions: &[V5RoleDeclarationResolution],
) {
    let mut relations = BTreeMap::<String, Vec<(u16, LocalRole, String)>>::new();
    for resolution in resolutions {
        if let V5RoleResolution::ProvedFamily { proof } = &resolution.resolution {
            relations.entry(proof.family_id.clone()).or_default().push((
                proof.occurrence.owner_clause,
                proof.occurrence.local_role,
                proof.derivation_hash.clone(),
            ));
        }
    }
    for values in relations.values_mut() {
        values.sort();
        values.dedup();
    }
    let mut slot_claims = BTreeMap::<(u16, LocalRole), BTreeSet<String>>::new();
    for family in families.iter() {
        if !family.marginal {
            continue;
        }
        if let Some(values) = relations.get(&family.family_id) {
            let slots = values
                .iter()
                .map(|(clause, role, _)| (*clause, *role))
                .collect::<BTreeSet<_>>();
            if slots.len() == 1 {
                slot_claims
                    .entry(*slots.iter().next().expect("singleton"))
                    .or_default()
                    .insert(family.family_id.clone());
            }
        }
    }
    for family in families.iter_mut() {
        if !family.marginal {
            family.anchor = V5AnchorDisposition::Internal;
            family.credited = false;
        } else {
            let values = relations
                .get(&family.family_id)
                .cloned()
                .unwrap_or_default();
            let slots = values
                .iter()
                .map(|(clause, role, _)| (*clause, *role))
                .collect::<BTreeSet<_>>();
            if !a3.proved || !a3.no_constructed_exported_a3_fallback {
                let proof_hash = tagged_hash(
                    "unresolved-exact-A3-fallback",
                    &(
                        stage,
                        candidate_digest,
                        &family.family_id,
                        &a3.derivation_hash,
                        a3.constructed_exported_output_count,
                        a3.named_live_export_residual_count,
                        &closure.derivation_hash,
                    ),
                );
                family.anchor = V5AnchorDisposition::NamedResidual {
                    gap_id: "T_BI_B2_LIVE_EXPORTED_A3_FALLBACK_NOT_DECIDED".to_owned(),
                    exact_reason: "the fresh exact-prefix A3 capability theorem did not exclude every exported constructed output".to_owned(),
                    exact_a3_capability_derivation_hash: a3.derivation_hash.clone(),
                    proof_hash,
                };
                family.credited = false;
            } else if slots.is_empty() {
                let proof_hash = tagged_hash(
                    "no-family-role-relation",
                    &(
                        stage,
                        candidate_digest,
                        &family.family_id,
                        &closure.derivation_hash,
                        a3.constructed_exported_output_count,
                    ),
                );
                family.anchor = V5AnchorDisposition::TheoremImpossibleNoRelation {
                    theorem_id: T_BI_B2_THEOREM_ID.to_owned(),
                    closure_derivation_hash: closure.derivation_hash.clone(),
                    no_constructed_exported_a3_fallback: a3.no_constructed_exported_a3_fallback,
                    proof_hash,
                };
                family.credited = false;
            } else if slots.len() != 1 {
                let exact_slots = slots.into_iter().collect::<Vec<_>>();
                let proof_hash = tagged_hash(
                    "non-functional-family-role-relation",
                    &(&family.family_id, &exact_slots, &values),
                );
                family.anchor = V5AnchorDisposition::TheoremImpossibleNonFunctionalRelation {
                    theorem_id: T_BI_B2_THEOREM_ID.to_owned(),
                    exact_slots,
                    no_constructed_exported_a3_fallback: a3.no_constructed_exported_a3_fallback,
                    proof_hash,
                };
                family.credited = false;
            } else {
                let slot = *slots.iter().next().expect("singleton");
                let competitors = slot_claims
                    .get(&slot)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .collect::<Vec<_>>();
                if competitors.len() == 1 {
                    let relation_derivation_hashes = values
                        .iter()
                        .map(|(_, _, hash)| hash.clone())
                        .collect::<Vec<_>>();
                    let injection_hash = tagged_hash(
                        "exact-local-role-injection",
                        &(
                            stage,
                            candidate_digest,
                            &family.family_id,
                            slot,
                            &relation_derivation_hashes,
                        ),
                    );
                    family.anchor = V5AnchorDisposition::CreditedLocalRole {
                        clause: slot.0,
                        role: slot.1,
                        relation_derivation_hashes,
                        injection_hash,
                    };
                    family.credited = true;
                } else {
                    let proof_hash = tagged_hash(
                        "local-role-relation-collision",
                        &(
                            stage,
                            candidate_digest,
                            slot,
                            &family.family_id,
                            &competitors,
                            a3.constructed_exported_output_count,
                            &closure.derivation_hash,
                        ),
                    );
                    family.anchor = V5AnchorDisposition::TheoremImpossibleRelationCollision {
                        theorem_id: T_BI_B2_THEOREM_ID.to_owned(),
                        clause: slot.0,
                        role: slot.1,
                        competing_family_ids: competitors,
                        no_constructed_exported_a3_fallback: a3.no_constructed_exported_a3_fallback,
                        proof_hash,
                    };
                    family.credited = false;
                }
            }
        }
        family.derivation_hash = tagged_hash("semantic-family-witness", family);
    }
}

fn receipt(ordinal: u8, phase_id: &str, evidence_hashes: Vec<String>) -> V5PhaseReceipt {
    let mut value = V5PhaseReceipt {
        ordinal,
        phase_id: phase_id.to_owned(),
        evidence_hashes,
        derivation_hash: String::new(),
    };
    value.derivation_hash = tagged_hash("phase-receipt", &value);
    value
}

fn certificate_digest(certificate: &ActLocalSemanticProvenanceV5Certificate) -> String {
    let mut projection = certificate.clone();
    projection.derivation_hash.clear();
    tagged_hash("act-local-semantic-v5-certificate", &projection)
}

fn issue_one_from_source(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
    source: &V5SemanticSourceView,
    declarations: &[V5RoleDeclarationInput],
    registry_surface: V5RoleRegistrySurface<'_>,
    predecessor_members: &[V5UnifiedSurfaceMember],
    historical_cubical_decisions: &[Value],
) -> Result<ActLocalSemanticProvenanceV5Certificate, ActLocalSemanticProvenanceV5Error> {
    replay_adoptions()?;
    let candidate_digest = candidate_hash(candidate);
    if source.stage != stage
        || source.candidate_hash != candidate_digest
        || source.predecessor_signature_digest != prefix.digest()
    {
        return Err(ActLocalSemanticProvenanceV5Error::Input(
            "semantic source surface is not bound to the exact act and prefix".to_owned(),
        ));
    }
    let elaboration = elaborate_telescope(prefix, candidate, stage.saturating_sub(1))
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Bridge(error.to_string()))?;
    let closure = predecessor_closure(prefix)
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Bridge(error.to_string()))?;
    let CandidateExtractionOutcome::Extracted(extraction) =
        extract_candidate_families(prefix, &closure, candidate, stage.saturating_sub(1))
    else {
        return Err(ActLocalSemanticProvenanceV5Error::Bridge(
            "candidate extraction was kernel-invalid after elaboration".to_owned(),
        ));
    };
    if extraction.derivation_hash != source.candidate_family_extraction_hash
        || elaboration.derivation_hash != source.candidate_elaboration_hash
    {
        return Err(ActLocalSemanticProvenanceV5Error::Bridge(
            "candidate elaboration/extraction does not match frozen v4".to_owned(),
        ));
    }
    let bridges = build_bridges(prefix, stage, candidate, source, &extraction, &elaboration)?;
    let t_bi_b1_proved = bridges.len() == source.ordinary_registry.applicable_schema_count
        && bridges.iter().all(|bridge| bridge.proved);
    if !t_bi_b1_proved {
        return Err(ActLocalSemanticProvenanceV5Error::Bridge(format!(
            "only {}/{} ordinary schemas obtained an exact bridge",
            bridges.iter().filter(|bridge| bridge.proved).count(),
            source.ordinary_registry.applicable_schema_count
        )));
    }
    let (unified_quotient, mut semantic_families) = build_unified_quotient_and_families(
        prefix,
        stage,
        source,
        &extraction,
        &closure,
        &bridges,
        predecessor_members,
        historical_cubical_decisions,
    )?;
    let exact_a3_capability = build_exact_a3_capability(prefix, stage, candidate)?;
    let finite_closure = build_closure_proof(
        candidate,
        &extraction,
        source,
        &bridges,
        &unified_quotient,
        &exact_a3_capability,
        source.role_registry_exhaustive,
    );
    let role_resolutions = resolve_roles(
        stage,
        candidate,
        &elaboration,
        source,
        &bridges,
        &semantic_families,
        &unified_quotient,
        &finite_closure,
        &exact_a3_capability,
        declarations,
        registry_surface,
    )?;
    anchor_families(
        stage,
        &candidate_digest,
        &finite_closure,
        &exact_a3_capability,
        &mut semantic_families,
        &role_resolutions,
    );
    let role_declaration_count = role_resolutions.len();
    let proved_family_declaration_count = role_resolutions
        .iter()
        .filter(|resolution| matches!(resolution.resolution, V5RoleResolution::ProvedFamily { .. }))
        .count();
    let theorem_impossibility_declaration_count = role_resolutions
        .iter()
        .filter(|resolution| {
            matches!(
                resolution.resolution,
                V5RoleResolution::TheoremBackedImpossibility { .. }
            )
        })
        .count();
    let named_role_residual_count = role_resolutions
        .iter()
        .filter(|resolution| {
            matches!(
                resolution.resolution,
                V5RoleResolution::NamedResidual { .. }
            )
        })
        .count();
    let named_quotient_residual_count = unified_quotient.named_residual_count;
    let named_a3_residual_count = exact_a3_capability.named_live_export_residual_count;
    let total_named_residual_count =
        named_role_residual_count + named_quotient_residual_count + named_a3_residual_count;
    let silent_residue_count = role_resolutions
        .iter()
        .filter(|resolution| resolution.silent_residue)
        .count()
        + source.role_schema_gap_count
            .saturating_sub(role_resolutions.len());
    let every_role_declaration_resolved = role_declaration_count == source.role_schema_gap_count
        && role_resolutions
            .iter()
            .all(|resolution| resolution.resolved)
        && silent_residue_count == 0;
    let marginal_family_count = semantic_families
        .iter()
        .filter(|family| family.marginal)
        .count();
    let credited_semantic_family_count = semantic_families
        .iter()
        .filter(|family| family.credited)
        .count();
    let proved_anchor_impossibility = |family: &V5SemanticFamilyWitness| match &family.anchor {
        V5AnchorDisposition::TheoremImpossibleNoRelation {
            theorem_id,
            closure_derivation_hash,
            no_constructed_exported_a3_fallback,
            proof_hash,
        } => {
            theorem_id == T_BI_B2_THEOREM_ID
                && closure_derivation_hash == &finite_closure.derivation_hash
                && *no_constructed_exported_a3_fallback
                && exact_a3_capability.proved
                && exact_a3_capability.no_constructed_exported_a3_fallback
                && exact_a3_capability.constructed_exported_output_count == 0
                && !proof_hash.is_empty()
        }
        V5AnchorDisposition::TheoremImpossibleRelationCollision {
            theorem_id,
            competing_family_ids,
            no_constructed_exported_a3_fallback,
            proof_hash,
            ..
        } => {
            theorem_id == T_BI_B2_THEOREM_ID
                && competing_family_ids.len() > 1
                && *no_constructed_exported_a3_fallback
                && exact_a3_capability.proved
                && exact_a3_capability.no_constructed_exported_a3_fallback
                && exact_a3_capability.constructed_exported_output_count == 0
                && !proof_hash.is_empty()
        }
        V5AnchorDisposition::TheoremImpossibleNonFunctionalRelation {
            theorem_id,
            exact_slots,
            no_constructed_exported_a3_fallback,
            proof_hash,
        } => {
            theorem_id == T_BI_B2_THEOREM_ID
                && exact_slots.len() > 1
                && *no_constructed_exported_a3_fallback
                && exact_a3_capability.proved
                && exact_a3_capability.no_constructed_exported_a3_fallback
                && exact_a3_capability.constructed_exported_output_count == 0
                && !proof_hash.is_empty()
        }
        _ => false,
    };
    let theorem_anchor_impossibility_count = semantic_families
        .iter()
        .filter(|family| family.marginal && proved_anchor_impossibility(family))
        .count();
    let every_marginal_family_credited_or_theorem_impossible =
        semantic_families.iter().all(|family| {
            !family.marginal
                || (family.credited
                    && matches!(
                        &family.anchor,
                        V5AnchorDisposition::CreditedLocalRole {
                            relation_derivation_hashes,
                            injection_hash,
                            ..
                        } if !relation_derivation_hashes.is_empty() && !injection_hash.is_empty()
                    ))
                || (!family.credited && proved_anchor_impossibility(family))
        });
    let credited_slots = semantic_families
        .iter()
        .filter_map(|family| match family.anchor {
            V5AnchorDisposition::CreditedLocalRole { clause, role, .. } => Some((clause, role)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let local_anchor_nonreuse_holds = credited_slots
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .len()
        == credited_slots.len();
    let mut t_bi_b2_proved = finite_closure.proved
        && unified_quotient.proved
        && total_named_residual_count == 0
        && every_role_declaration_resolved
        && every_marginal_family_credited_or_theorem_impossible
        && local_anchor_nonreuse_holds;
    let generic_r1_relations = role_resolutions
        .iter()
        .filter_map(|resolution| match &resolution.resolution {
            V5RoleResolution::ProvedFamily { proof }
                if proof.rule == V5RelationRule::GenericR1Completion && proof.proved =>
            {
                Some(proof)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let stage1_exact_completion_equality_derivation_hash = if stage == 1
        && candidate.clauses.len() == 2
        && candidate.clauses[0].expr == Expr::Univ
        && candidate.clauses[1].expr == Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))
        && elaboration.clauses.len() == 2
        && elaboration.clauses[1].normal_form
            == Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))
    {
        tagged_hash(
            "stage1-exact-formation-completion-equality",
            &(
                &candidate.clauses[0].expr,
                &candidate.clauses[1].expr,
                &elaboration.clauses[0],
                &elaboration.clauses[1],
                generic_r1_relations
                    .first()
                    .map(|proof| proof.fresh_term_relation_derivation_hash.as_str()),
            ),
        )
    } else {
        String::new()
    };
    let stage1_carrier_role_case_proofs = if stage == 1 {
        source.special_cases
            .generic_r1
            .as_ref()
            .and_then(|r1| {
                let carrier = candidate.clauses.get(usize::from(r1.carrier_clause))?;
                let completion = candidate.clauses.get(usize::from(r1.completion_clause))?;
                let carrier_elaboration =
                    elaboration.clauses.get(usize::from(r1.carrier_clause))?;
                let completion_elaboration =
                    elaboration.clauses.get(usize::from(r1.completion_clause))?;
                let candidate_clause_roles = elaboration
                    .clauses
                    .iter()
                    .map(|clause| clause.kernel_role)
                    .collect::<Vec<_>>();
                let cubical_family_count = source
                    .unified_families
                    .iter()
                    .filter(|family| matches!(family.source, V4FamilySource::CubicalPath { .. }))
                    .count();
                let exact_formation_completion_package = r1.adopted_package_clause_replayed
                    && r1.carrier_is_kernel_formation_type
                    && r1.completion_is_kernel_formation_type
                    && r1.completion_is_exact_app_univ_carrier
                    && r1.dependency_resolves_to_carrier
                    && r1.completed_action_covers_carrier_by_adopted_r1
                    && carrier.expr == Expr::Univ
                    && completion.expr == Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))
                    && !stage1_exact_completion_equality_derivation_hash.is_empty()
                    && !r1.archive_or_count_input_used;
                let completion_relation = generic_r1_relations.first();
                Some(
                    LocalRole::ALL
                        .into_iter()
                        .map(|role| {
                            let role_generated_by_completion = role == LocalRole::KernelHead
                                && exact_formation_completion_package
                                && generic_r1_relations.len() == 1
                                && completion_relation.is_some_and(|proof| {
                                    proof.proved
                                        && proof.occurrence.owner_clause == r1.completion_clause
                                        && proof.occurrence.mechanism
                                            == CreditMechanism::IntrinsicKernel
                                        && proof.occurrence.local_role == LocalRole::KernelHead
                                        && !proof.fresh_term_relation_derivation_hash.is_empty()
                                        && !proof.v4_desired_label_equality_used_as_proof
                                });
                            let other_role_constructor_surface_empty = role
                                != LocalRole::KernelHead
                                && candidate_clause_roles
                                    .iter()
                                    .all(|candidate_role| *candidate_role == ClauseRole::Formation)
                                && bridges.is_empty()
                                && cubical_family_count == 0
                                && exact_a3_capability.fresh_orbit_count == 0;
                            let mut proof = V5Stage1CarrierRoleCaseProof {
                                theorem_id: T_BI_B2_STAGE1_R1_ID.to_owned(),
                                carrier_clause: r1.carrier_clause,
                                completion_clause: r1.completion_clause,
                                role,
                                carrier_expr: carrier.expr.clone(),
                                completion_expr: completion.expr.clone(),
                                carrier_normal_form: carrier_elaboration.normal_form.clone(),
                                completion_normal_form: completion_elaboration.normal_form.clone(),
                                candidate_clause_roles: candidate_clause_roles.clone(),
                                ordinary_bridge_count: bridges.len(),
                                cubical_family_count,
                                exact_a3_orbit_count: exact_a3_capability.fresh_orbit_count,
                                completion_relation_derivation_hash: completion_relation
                                    .map(|relation| relation.derivation_hash.clone()),
                                exact_formation_completion_package,
                                role_generated_by_completion,
                                other_role_constructor_surface_empty,
                                archive_or_desired_label_used: false,
                                proved: exact_formation_completion_package
                                    && (role_generated_by_completion
                                        || other_role_constructor_surface_empty),
                                derivation_hash: String::new(),
                            };
                            proof.derivation_hash =
                                tagged_hash("stage1-v5-carrier-role-case", &proof);
                            proof
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let stage1_carrier_role_case_derivation_hashes = stage1_carrier_role_case_proofs
        .iter()
        .map(|proof| proof.derivation_hash.clone())
        .collect::<Vec<_>>();
    let stage1_r1_preserved = stage != 1
        || source.special_cases.generic_r1.as_ref().is_some_and(|r1| {
            let credited_ids = semantic_families
                .iter()
                .filter(|family| family.credited)
                .map(|family| family.family_id.as_str())
                .collect::<Vec<_>>();
            let completion_id = generic_r1_relations
                .first()
                .map(|proof| proof.family_id.as_str());
            let completion_is_exact_r1 = completion_id.is_some_and(|family_id| {
                source.unified_families.iter().any(|family| {
                    family.family_id == family_id
                        && matches!(family.source, V4FamilySource::R1CompletedPackage { .. })
                })
            });
            let carrier_separately_credited = semantic_families.iter().any(|family| {
                family.credited
                    && source.unified_families.iter().any(|v4_family| {
                        v4_family.family_id == family.family_id
                            && matches!(
                                v4_family.source,
                                V4FamilySource::Clause {
                                    generator_clause,
                                    ..
                                } if generator_clause == r1.carrier_clause
                            )
                    })
            });
            let exact_single_completion_credit =
                completion_id.is_some_and(|family_id| credited_ids == vec![family_id]);
            generic_r1_relations.len() == 1
                && !generic_r1_relations[0]
                    .fresh_term_relation_derivation_hash
                    .is_empty()
                && exact_single_completion_credit
                && completion_is_exact_r1
                && !carrier_separately_credited
                && stage1_carrier_role_case_derivation_hashes.len() == 4
                && stage1_carrier_role_case_derivation_hashes
                    .iter()
                    .all(|hash| !hash.is_empty())
                && stage1_carrier_role_case_proofs.len() == 4
                && stage1_carrier_role_case_proofs
                    .iter()
                    .all(|proof| proof.proved && !proof.archive_or_desired_label_used)
                && !stage1_exact_completion_equality_derivation_hash.is_empty()
                && r1.all_four_carrier_exception_roles_enumerated
                && r1.completed_action_covers_carrier_by_adopted_r1
                && r1.completion_is_exact_app_univ_carrier
                && r1.dependency_resolves_to_carrier
                && !r1.archive_or_count_input_used
        });
    let stage2_exact_family_surface_ids = if stage == 2 {
        semantic_families
            .iter()
            .map(|family| family.family_id.clone())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let stage2_internality_derivation_hashes = if stage == 2 {
        semantic_families
            .iter()
            .filter_map(|family| {
                family
                    .fresh_marginality
                    .is_internal()
                    .then(|| family.marginality_proof_hash.clone())
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let stage2_exact_predecessor_closure_hash = if stage == 2 {
        tagged_hash(
            "stage2-fresh-exact-predecessor-closure",
            &(
                prefix.digest(),
                &closure.digest,
                &closure.families,
                &extraction.derivation_hash,
                &unified_quotient.derivation_hash,
            ),
        )
    } else {
        String::new()
    };
    let stage2_no_ordinary_duplicate = stage != 2
        || (bridges.is_empty()
            && semantic_families
                .iter()
                .all(|family| matches!(family.source, V5SemanticFamilySource::FrozenV4 { .. })));
    let stage2_constitutive_question_not_assumed = stage != 2
        || (unified_quotient.proved
            && unified_quotient.named_residual_count == 0
            && !stage2_exact_family_surface_ids.is_empty()
            && stage2_internality_derivation_hashes.len() == stage2_exact_family_surface_ids.len()
            && stage2_internality_derivation_hashes
                .iter()
                .all(|hash| !hash.is_empty())
            && stage2_no_ordinary_duplicate
            && !stage2_exact_predecessor_closure_hash.is_empty()
            && extraction.signature_digest == prefix.digest()
            && extraction.closure_digest == closure.digest
            && semantic_families.iter().all(|family| {
                !family.marginal
                    && !family.credited
                    && family.fresh_marginality.is_internal()
                    && matches!(family.anchor, V5AnchorDisposition::Internal)
            })
            && role_resolutions
                .iter()
                .all(|resolution| match &resolution.resolution {
                    V5RoleResolution::ProvedFamily { proof } => proof.proved,
                    V5RoleResolution::TheoremBackedImpossibility { proof } => proof.proved,
                    V5RoleResolution::NamedResidual { .. } => false,
                }));
    let stage9_map_declaration_ids = if stage == 9 {
        role_resolutions
            .iter()
            .filter(|resolution| resolution.constructor_search.role_kind.starts_with("map_"))
            .map(|resolution| resolution.declaration_id.clone())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let stage9_boundary_theorem_hash = if stage == 9
        && stage9_map_declaration_ids.len() == role_resolutions.len()
        && role_resolutions
            .iter()
            .all(|resolution| resolution.constructor_search.proved)
    {
        tagged_hash(
            "stage9-exact-map-boundary-theorem",
            &(
                &candidate_digest,
                &stage9_map_declaration_ids,
                role_resolutions
                    .iter()
                    .map(|resolution| resolution.constructor_search.derivation_hash.as_str())
                    .collect::<Vec<_>>(),
                &finite_closure.derivation_hash,
                &exact_a3_capability.derivation_hash,
            ),
        )
    } else {
        String::new()
    };
    let stage9_boundary_decided_by_relation_theorem = stage != 9
        || (role_declaration_count == 17
            && stage9_map_declaration_ids.len() == 17
            && stage9_map_declaration_ids
                .iter()
                .collect::<BTreeSet<_>>()
                .len()
                == 17
            && !stage9_boundary_theorem_hash.is_empty()
            && every_role_declaration_resolved
            && role_resolutions
                .iter()
                .all(|resolution| match &resolution.resolution {
                    V5RoleResolution::ProvedFamily { proof } => proof.proved,
                    V5RoleResolution::TheoremBackedImpossibility { proof } => proof.proved,
                    V5RoleResolution::NamedResidual { .. } => false,
                }));
    let r2_generated_instance_not_exported = bridges
        .iter()
        .filter(|bridge| bridge.generated_instance)
        .all(|bridge| {
            !semantic_families
                .iter()
                .any(|family| family.family_id == bridge.semantic_family_id)
        })
        && !source.special_cases.r2_generated_instance_multiplied;
    t_bi_b2_proved &= stage1_r1_preserved
        && stage2_constitutive_question_not_assumed
        && stage9_boundary_decided_by_relation_theorem
        && r2_generated_instance_not_exported;
    let archive_read = false;
    let structural_nu_read = false;
    let bar_read = false;
    let verdict_read = false;
    let enacted_future_read = false;
    let mut phase_receipts = vec![
        receipt(
            1,
            "candidate-prefix",
            vec![candidate_digest.clone(), prefix.digest().to_owned()],
        ),
        receipt(
            2,
            "frozen-declaration-and-v4-reissue",
            vec![v3.derivation_hash.clone(), source.source_derivation_hash.to_owned()],
        ),
        receipt(
            3,
            "elaboration-and-extraction",
            vec![
                elaboration.derivation_hash.clone(),
                extraction.derivation_hash.clone(),
            ],
        ),
        receipt(
            4,
            "expr-to-schema2",
            bridges
                .iter()
                .map(|bridge| bridge.derivation_hash.clone())
                .collect(),
        ),
        receipt(
            5,
            "exact-a3",
            vec![exact_a3_capability.derivation_hash.clone()],
        ),
        receipt(
            6,
            "ordinary-and-cubical-theorem",
            vec![
                source.ordinary_registry.derivation_hash.clone(),
                source.path_quotient
                    .as_ref()
                    .map(|proof| proof.derivation_hash.clone())
                    .unwrap_or_else(|| tagged_hash("empty-path-quotient", &stage)),
                unified_quotient.derivation_hash.clone(),
                finite_closure.derivation_hash.clone(),
            ],
        ),
        receipt(
            7,
            "family-role-relation",
            role_resolutions
                .iter()
                .map(|resolution| resolution.derivation_hash.clone())
                .collect(),
        ),
    ];
    let preseal_derivation_hash = tagged_hash(
        "v5-package-preseal",
        &(
            stage,
            &candidate_digest,
            prefix.digest(),
            &bridges,
            &unified_quotient,
            &semantic_families,
            &role_resolutions,
            &exact_a3_capability,
            &finite_closure,
            &stage1_carrier_role_case_proofs,
            t_bi_b1_proved,
            t_bi_b2_proved,
        ),
    );
    phase_receipts.push(receipt(
        8,
        "package-seal",
        vec![preseal_derivation_hash.clone()],
    ));
    let mut certificate = ActLocalSemanticProvenanceV5Certificate {
        schema: ACT_LOCAL_SEMANTIC_PROVENANCE_V5_SCHEMA.to_owned(),
        date: ACT_LOCAL_SEMANTIC_PROVENANCE_V5_DATE.to_owned(),
        theorem_ids: vec![
            T_BI_B1_THEOREM_ID.to_owned(),
            T_BI_B2_THEOREM_ID.to_owned(),
            T_BI_B2_ROLE_KIND_ID.to_owned(),
            T_BI_B2_A3_CAPABILITY_ID.to_owned(),
            T_BI_B2_UNIFIED_QUOTIENT_ID.to_owned(),
            T_BI_B2_EQUIVALENCE_CLOSURE_ID.to_owned(),
            T_BI_B2_STAGE1_R1_ID.to_owned(),
        ],
        stage,
        candidate_hash: candidate_digest,
        predecessor_signature_digest: prefix.digest().to_owned(),
        frozen_v3_package_hash: v3.derivation_hash.clone(),
        frozen_v4_package_hash: source.source_derivation_hash.to_owned(),
        bridges,
        unified_quotient,
        semantic_families,
        role_resolutions,
        exact_a3_capability,
        finite_closure,
        role_declaration_count,
        proved_family_declaration_count,
        theorem_impossibility_declaration_count,
        named_role_residual_count,
        named_quotient_residual_count,
        named_a3_residual_count,
        total_named_residual_count,
        silent_residue_count,
        marginal_family_count,
        credited_semantic_family_count,
        theorem_anchor_impossibility_count,
        semantic_family_nu: credited_semantic_family_count as u32,
        t_bi_b1_proved,
        t_bi_b2_proved,
        every_role_declaration_resolved,
        every_marginal_family_credited_or_theorem_impossible,
        local_anchor_nonreuse_holds,
        stage1_r1_preserved,
        stage1_carrier_role_case_proofs,
        stage1_carrier_role_case_derivation_hashes,
        stage1_exact_completion_equality_derivation_hash,
        stage2_constitutive_question_not_assumed,
        stage2_exact_family_surface_ids,
        stage2_internality_derivation_hashes,
        stage2_exact_predecessor_closure_hash,
        stage2_no_ordinary_duplicate,
        stage9_boundary_decided_by_relation_theorem,
        stage9_map_declaration_ids,
        stage9_boundary_theorem_hash,
        r2_generated_instance_not_exported,
        archive_read,
        structural_nu_read,
        bar_read,
        verdict_read,
        enacted_future_read,
        phase_receipts,
        preseal_derivation_hash,
        derivation_hash: String::new(),
    };
    certificate.derivation_hash = certificate_digest(&certificate);
    Ok(certificate)
}

fn issue_one(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
    v4: &ActLocalSemanticProvenanceV4Certificate,
    predecessor_members: &[V5UnifiedSurfaceMember],
    historical_cubical_decisions: &[Value],
) -> Result<ActLocalSemanticProvenanceV5Certificate, ActLocalSemanticProvenanceV5Error> {
    let candidate_digest = candidate_hash(candidate);
    if v4.stage != stage
        || v4.candidate_hash != candidate_digest
        || v4.predecessor_signature_digest != prefix.digest()
    {
        return Err(ActLocalSemanticProvenanceV5Error::Input(
            "frozen v4 package is not bound to the exact act and prefix".to_owned(),
        ));
    }
    if !replay_act_local_semantic_provenance_v4(prefix, candidate, v3, v4).is_empty() {
        return Err(ActLocalSemanticProvenanceV5Error::Input(
            "frozen v4 predecessor did not replay".to_owned(),
        ));
    }
    let source = V5SemanticSourceView::historical(v4);
    let declarations = historical_role_declaration_inputs(v4);
    issue_one_from_source(
        prefix,
        stage,
        candidate,
        v3,
        &source,
        &declarations,
        V5RoleRegistrySurface::Historical27,
        predecessor_members,
        historical_cubical_decisions,
    )
}

fn authoritative_role_row_hash(row: &V5PrefixLocalAuthoritativeRoleRow) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-authoritative-role-row", &projection)
}

fn authoritative_family_row_hash(row: &V5PrefixLocalAuthoritativeFamilyRow) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-authoritative-family-row", &projection)
}

fn prefix_local_semantic_package_hash(proof: &V5PrefixLocalSemanticPackageProof) -> String {
    let mut projection = proof.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-authoritative-semantic-package", &projection)
}

fn prefix_local_semantic_sequence_hash(sequence: &V5PrefixLocalSemanticSequence) -> String {
    let mut projection = sequence.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-authoritative-semantic-sequence", &projection)
}

fn project_prefix_local_semantic_package(
    v3: &ActLocalProvenanceV3Certificate,
    source: &V5SemanticSourceView,
    observed_role_kinds: Vec<String>,
    package: &ActLocalSemanticProvenanceV5Certificate,
) -> V5PrefixLocalSemanticPackageProof {
    let mut role_rows = package
        .role_resolutions
        .iter()
        .map(|resolution| {
            let (resolution_class, resolved_family_id) = match &resolution.resolution {
                V5RoleResolution::ProvedFamily { proof } => (
                    V5PrefixLocalResolutionClass::ProvedFamily,
                    Some(proof.family_id.clone()),
                ),
                V5RoleResolution::TheoremBackedImpossibility { .. } => (
                    V5PrefixLocalResolutionClass::TheoremBackedImpossibility,
                    None,
                ),
                V5RoleResolution::NamedResidual { .. } => {
                    (V5PrefixLocalResolutionClass::NamedResidual, None)
                }
            };
            let mut target_family_ids = resolution.constructor_search.matching_family_ids.clone();
            target_family_ids.sort();
            target_family_ids.dedup();
            let mut target_relation_hashes = resolution
                .constructor_search
                .fresh_term_relation_derivation_hashes
                .clone();
            target_relation_hashes.sort();
            target_relation_hashes.dedup();
            let mut row = V5PrefixLocalAuthoritativeRoleRow {
                declaration_id: resolution.declaration_id.clone(),
                occurrence: match &resolution.resolution {
                    V5RoleResolution::ProvedFamily { proof } => proof.occurrence.clone(),
                    V5RoleResolution::TheoremBackedImpossibility { proof } => {
                        proof.occurrence.clone()
                    }
                    V5RoleResolution::NamedResidual { proof } => proof.occurrence.clone(),
                },
                direct_rule: resolution.constructor_search.rule,
                queried_surfaces: resolution.constructor_search.queried_surfaces.clone(),
                exact_coordinate_predicate_replayed: resolution
                    .constructor_search
                    .exact_coordinate_predicate_replayed,
                exact_mechanism_predicate_replayed: resolution
                    .constructor_search
                    .exact_mechanism_predicate_replayed,
                exact_local_role_predicate_replayed: resolution
                    .constructor_search
                    .exact_local_role_predicate_replayed,
                no_default_rule: resolution
                    .constructor_search
                    .no_wildcard_or_default_absence_rule,
                target_family_ids,
                target_relation_hashes,
                resolution_class,
                resolved_family_id,
                resolved: resolution.resolved,
                derivation_hash: String::new(),
            };
            row.derivation_hash = authoritative_role_row_hash(&row);
            row
        })
        .collect::<Vec<_>>();
    role_rows.sort_by(|left, right| left.declaration_id.cmp(&right.declaration_id));

    let mut family_rows = package
        .semantic_families
        .iter()
        .map(|family| {
            let mut row = V5PrefixLocalAuthoritativeFamilyRow {
                family_id: family.family_id.clone(),
                stage: family.stage,
                source: family.source.clone(),
                shape_key: family.shape_key.clone(),
                typed_normalized_natural: family.typed_normalized_natural,
                fresh_marginality: family.fresh_marginality.clone(),
                marginal: family.marginal,
                anchor: family.anchor.clone(),
                credited: family.credited,
                derivation_hash: String::new(),
            };
            row.derivation_hash = authoritative_family_row_hash(&row);
            row
        })
        .collect::<Vec<_>>();
    family_rows.sort_by(|left, right| left.family_id.cmp(&right.family_id));
    let mut bridge_derivation_hashes = package
        .bridges
        .iter()
        .map(|bridge| bridge.derivation_hash.clone())
        .collect::<Vec<_>>();
    bridge_derivation_hashes.sort();
    let mut credited_family_ids = family_rows
        .iter()
        .filter(|family| family.credited)
        .map(|family| family.family_id.clone())
        .collect::<Vec<_>>();
    credited_family_ids.sort();
    credited_family_ids.dedup();
    let special_cases_proved = package.stage1_r1_preserved
        && package.stage2_constitutive_question_not_assumed
        && package.stage9_boundary_decided_by_relation_theorem
        && package.r2_generated_instance_not_exported;
    let local_completeness = package.t_bi_b1_proved
        && package.t_bi_b2_proved
        && package.every_role_declaration_resolved
        && package.every_marginal_family_credited_or_theorem_impossible
        && package.local_anchor_nonreuse_holds
        && package.named_role_residual_count == 0
        && package.named_quotient_residual_count == 0
        && package.named_a3_residual_count == 0
        && package.silent_residue_count == 0
        && special_cases_proved
        && credited_family_ids.len() == package.credited_semantic_family_count
        && u32::try_from(credited_family_ids.len()).ok() == Some(package.semantic_family_nu)
        && role_rows.iter().all(|row| {
            row.resolved
                && row.no_default_rule
                && row.queried_surfaces
                    == vec![
                        V5ConstructorSurface::CoreExpr,
                        V5ConstructorSurface::OrdinarySchema2,
                        V5ConstructorSurface::CubicalPath,
                        V5ConstructorSurface::ExactA3,
                    ]
                && row.exact_coordinate_predicate_replayed
                && row.exact_mechanism_predicate_replayed
                && row.exact_local_role_predicate_replayed
                && row.resolution_class != V5PrefixLocalResolutionClass::NamedResidual
        });
    let mut proof = V5PrefixLocalSemanticPackageProof {
        theorem_id: T_BI_B1_B2_PREFIX_LOCAL_THEOREM_ID.to_owned(),
        stage: package.stage,
        candidate_hash: package.candidate_hash.clone(),
        predecessor_signature_digest: package.predecessor_signature_digest.clone(),
        v3_declaration_hash: v3.derivation_hash.clone(),
        v4_candidate_local_source_hash: source.source_derivation_hash.clone(),
        observed_role_kinds,
        role_rows,
        family_rows,
        bridge_derivation_hashes,
        unified_quotient_derivation_hash: package.unified_quotient.derivation_hash.clone(),
        exact_a3_capability_derivation_hash: package.exact_a3_capability.derivation_hash.clone(),
        finite_closure_derivation_hash: package.finite_closure.derivation_hash.clone(),
        credited_family_ids,
        semantic_nu: package.semantic_family_nu,
        role_declaration_count: package.role_declaration_count,
        proved_family_declaration_count: package.proved_family_declaration_count,
        theorem_impossibility_declaration_count: package
            .theorem_impossibility_declaration_count,
        named_role_residual_count: package.named_role_residual_count,
        named_quotient_residual_count: package.named_quotient_residual_count,
        named_a3_residual_count: package.named_a3_residual_count,
        silent_residue_count: package.silent_residue_count,
        t_bi_b1_proved: package.t_bi_b1_proved,
        t_bi_b2_proved: package.t_bi_b2_proved,
        every_role_declaration_resolved: package.every_role_declaration_resolved,
        every_marginal_family_credited_or_theorem_impossible: package
            .every_marginal_family_credited_or_theorem_impossible,
        local_anchor_nonreuse_holds: package.local_anchor_nonreuse_holds,
        special_cases_proved,
        unused_registry_extension_projection_equal: false,
        historical_registry_consulted: source.historical_registry_consulted,
        archive_structural_bar_verdict_or_future_read: source
            .archive_structural_bar_verdict_or_future_read
            || package.archive_read
            || package.structural_nu_read
            || package.bar_read
            || package.verdict_read
            || package.enacted_future_read,
        proved: local_completeness
            && !source.historical_registry_consulted
            && !source.archive_structural_bar_verdict_or_future_read,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = prefix_local_semantic_package_hash(&proof);
    proof
}

fn extension_comparison_projection(
    proof: &V5PrefixLocalSemanticPackageProof,
) -> V5PrefixLocalSemanticPackageProof {
    let mut projection = proof.clone();
    projection.unused_registry_extension_projection_equal = false;
    projection.proved = false;
    projection.derivation_hash.clear();
    projection
}

fn issue_one_prefix_local(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
    v3: &ActLocalProvenanceV3Certificate,
    source_surface: &V4PrefixLocalSourceSurface,
    registry: &BTreeSet<String>,
    predecessor_members: &[V5UnifiedSurfaceMember],
    cubical_decisions: &[Value],
) -> Result<ActLocalSemanticProvenanceV5Certificate, ActLocalSemanticProvenanceV5Error> {
    let replay_errors =
        replay_v4_prefix_local_source_surface(prefix, candidate, v3, source_surface);
    if !replay_errors.is_empty() {
        return Err(ActLocalSemanticProvenanceV5Error::Input(format!(
            "v4 prefix-local source surface did not replay: {}",
            replay_errors.join("; ")
        )));
    }
    let declarations = prefix_local_role_declaration_inputs(v3)?;
    let observed = declarations
        .iter()
        .map(|declaration| declaration.occurrence.kind.clone())
        .collect::<BTreeSet<_>>();
    if !observed.is_subset(registry) {
        return Err(ActLocalSemanticProvenanceV5Error::Relation(
            "prefix-local registry omits an observed role kind".to_owned(),
        ));
    }
    let source = V5SemanticSourceView::prefix_local(source_surface, declarations.len());
    issue_one_from_source(
        prefix,
        stage,
        candidate,
        v3,
        &source,
        &declarations,
        V5RoleRegistrySurface::PrefixLocal(registry),
        predecessor_members,
        cubical_decisions,
    )
}

/// Build the B1/B2 semantic sequence without constructing or replaying the
/// historical 27-kind registry.  Each package is built twice, once over the
/// exact observed grammar and once after adjoining an unused sentinel; only
/// the registry-erased projections may agree and become authority.
pub fn issue_prefix_local_semantic_sequence_v5(
    entries: &[(u32, Telescope)],
) -> Result<V5PrefixLocalSemanticSequence, ActLocalSemanticProvenanceV5Error> {
    if entries.is_empty()
        || !entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=entries.len() as u32)
    {
        return Err(ActLocalSemanticProvenanceV5Error::Input(
            "prefix-local v5 sequence must be contiguous from Stage 1".to_owned(),
        ));
    }
    let v3_packages = issue_act_local_sequence_v3(entries)
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Input(error.to_string()))?;
    let mut accepted = Vec::<(u32, Telescope)>::new();
    let mut predecessor_members = Vec::<V5UnifiedSurfaceMember>::new();
    let mut cubical_decisions = Vec::<Value>::new();
    let mut proofs = Vec::new();
    for ((stage, candidate), v3) in entries.iter().zip(&v3_packages) {
        let prefix = SealedSignature::from_telescopes(accepted.clone());
        let source_surface = issue_v4_prefix_local_source_surface(&prefix, *stage, candidate, v3)
            .map_err(|error| ActLocalSemanticProvenanceV5Error::Input(error.to_string()))?;
        let source_view = V5SemanticSourceView::prefix_local(
            &source_surface,
            prefix_local_role_declaration_inputs(v3)?.len(),
        );
        cubical_decisions.extend(cubical_decision_values(&source_view));
        let declarations = prefix_local_role_declaration_inputs(v3)?;
        let observed = declarations
            .iter()
            .map(|declaration| declaration.occurrence.kind.clone())
            .collect::<BTreeSet<_>>();
        let base = issue_one_prefix_local(
            &prefix,
            *stage,
            candidate,
            v3,
            &source_surface,
            &observed,
            &predecessor_members,
            &cubical_decisions,
        )?;
        const SENTINEL: &str = "__unused_prefix_local_role_extension_sentinel__";
        if observed.contains(SENTINEL) {
            return Err(ActLocalSemanticProvenanceV5Error::Invariant(
                "observed role grammar collides with extension sentinel".to_owned(),
            ));
        }
        let mut extended_registry = observed.clone();
        extended_registry.insert(SENTINEL.to_owned());
        let extended = issue_one_prefix_local(
            &prefix,
            *stage,
            candidate,
            v3,
            &source_surface,
            &extended_registry,
            &predecessor_members,
            &cubical_decisions,
        )?;
        let observed_role_kinds = observed.into_iter().collect::<Vec<_>>();
        let mut base_proof = project_prefix_local_semantic_package(
            v3,
            &source_view,
            observed_role_kinds.clone(),
            &base,
        );
        let extended_proof = project_prefix_local_semantic_package(
            v3,
            &source_view,
            observed_role_kinds,
            &extended,
        );
        let extension_equal = extension_comparison_projection(&base_proof)
            == extension_comparison_projection(&extended_proof);
        base_proof.unused_registry_extension_projection_equal = extension_equal;
        base_proof.proved &= extension_equal;
        base_proof.derivation_hash = prefix_local_semantic_package_hash(&base_proof);
        if !base_proof.proved {
            return Err(ActLocalSemanticProvenanceV5Error::Invariant(format!(
                "Stage {stage} prefix-local semantic package did not close"
            )));
        }
        predecessor_members.extend(base.unified_quotient.current_members.iter().cloned());
        accepted.push((*stage, candidate.clone()));
        proofs.push(base_proof);
    }
    let semantic_nu_vector = proofs
        .iter()
        .map(|proof| proof.semantic_nu)
        .collect::<Vec<_>>();
    let sorted_unique_observed_role_kinds = proofs
        .iter()
        .flat_map(|proof| proof.observed_role_kinds.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let every_package_registry_extension_invariant = proofs.iter().all(|proof| {
        proof.proved && proof.unused_registry_extension_projection_equal
    });
    let no_historical_registry_or_future_input = proofs.iter().all(|proof| {
        !proof.historical_registry_consulted
            && !proof.archive_structural_bar_verdict_or_future_read
    });
    let t_bi_b1_proved_on_sequence = proofs.iter().all(|proof| proof.t_bi_b1_proved);
    let t_bi_b2_proved_on_sequence = proofs.iter().all(|proof| proof.t_bi_b2_proved);
    let authoritative_sequence_seal = tagged_hash(
        "prefix-local-semantic-package-seal",
        &proofs
            .iter()
            .map(|proof| proof.derivation_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let mut sequence = V5PrefixLocalSemanticSequence {
        schema: V5_PREFIX_LOCAL_SEMANTIC_SEQUENCE_SCHEMA.to_owned(),
        theorem_id: T_BI_B1_B2_PREFIX_LOCAL_THEOREM_ID.to_owned(),
        packages: proofs,
        semantic_nu_vector,
        sorted_unique_observed_role_kinds,
        every_package_registry_extension_invariant,
        no_historical_registry_or_future_input,
        t_bi_b1_proved_on_sequence,
        t_bi_b2_proved_on_sequence,
        authoritative_sequence_seal,
        derivation_hash: String::new(),
    };
    sequence.derivation_hash = prefix_local_semantic_sequence_hash(&sequence);
    Ok(sequence)
}

pub fn replay_prefix_local_semantic_sequence_v5(
    entries: &[(u32, Telescope)],
    claimed: &V5PrefixLocalSemanticSequence,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != prefix_local_semantic_sequence_hash(claimed) {
        errors.push("prefix-local v5 sequence digest mismatch".to_owned());
    }
    match issue_prefix_local_semantic_sequence_v5(entries) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push(
            "prefix-local v5 sequence differs from deterministic reissuance".to_owned(),
        ),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

/// Create the additive B1/B2 theorem sequence over any contiguous prefix.
/// The predecessor semantic input is the fresh, typed unified surface
/// accumulated from earlier packages in this same run.
pub fn issue_act_local_semantic_sequence_v5(
    entries: &[(u32, Telescope)],
) -> Result<ActLocalSemanticSequenceV5, ActLocalSemanticProvenanceV5Error> {
    if entries.is_empty()
        || !entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=entries.len() as u32)
    {
        return Err(ActLocalSemanticProvenanceV5Error::Input(
            "v5 sequence must be contiguous from Stage 1".to_owned(),
        ));
    }
    let v3 = issue_act_local_sequence_v3(entries)
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Input(error.to_string()))?;
    let v4 = issue_act_local_semantic_sequence_v4(entries)
        .map_err(|error| ActLocalSemanticProvenanceV5Error::Input(error.to_string()))?;
    let mut accepted = Vec::new();
    let mut predecessor_members = Vec::<V5UnifiedSurfaceMember>::new();
    let mut historical_cubical_decisions = Vec::<Value>::new();
    let mut packages = Vec::new();
    for (((stage, candidate), v3_package), v4_package) in entries.iter().zip(&v3).zip(&v4) {
        let prefix = SealedSignature::from_telescopes(accepted.clone());
        let source = V5SemanticSourceView::historical(v4_package);
        historical_cubical_decisions.extend(cubical_decision_values(&source));
        let package = issue_one(
            &prefix,
            *stage,
            candidate,
            v3_package,
            v4_package,
            &predecessor_members,
            &historical_cubical_decisions,
        )?;
        predecessor_members.extend(package.unified_quotient.current_members.iter().cloned());
        accepted.push((*stage, candidate.clone()));
        packages.push(package);
    }
    let exact_package_derivation_hashes = packages
        .iter()
        .map(|package| package.derivation_hash.clone())
        .collect::<Vec<_>>();
    let intrinsic_sequence_seal = tagged_hash(
        "intrinsic-v5-package-sequence-seal",
        &exact_package_derivation_hashes,
    );
    let role_declaration_count = packages
        .iter()
        .map(|package| package.role_declaration_count)
        .sum();
    let proved_family_declaration_count = packages
        .iter()
        .map(|package| package.proved_family_declaration_count)
        .sum();
    let theorem_impossibility_declaration_count = packages
        .iter()
        .map(|package| package.theorem_impossibility_declaration_count)
        .sum();
    let named_role_residual_count = packages
        .iter()
        .map(|package| package.named_role_residual_count)
        .sum();
    let named_quotient_residual_count = packages
        .iter()
        .map(|package| package.named_quotient_residual_count)
        .sum();
    let named_a3_residual_count = packages
        .iter()
        .map(|package| package.named_a3_residual_count)
        .sum();
    let total_named_residual_count =
        named_role_residual_count + named_quotient_residual_count + named_a3_residual_count;
    let silent_residue_count = packages
        .iter()
        .map(|package| package.silent_residue_count)
        .sum();
    let t_bi_b1_proved_on_sequence = packages.iter().all(|package| package.t_bi_b1_proved);
    let t_bi_b2_proved_on_sequence =
        packages.iter().all(|package| package.t_bi_b2_proved) && total_named_residual_count == 0;
    let mut sequence = ActLocalSemanticSequenceV5 {
        schema: "act-local-semantic-family-sequence-v5".to_owned(),
        date: ACT_LOCAL_SEMANTIC_PROVENANCE_V5_DATE.to_owned(),
        packages,
        exact_package_derivation_hashes,
        intrinsic_sequence_seal,
        role_declaration_count,
        proved_family_declaration_count,
        theorem_impossibility_declaration_count,
        named_role_residual_count,
        named_quotient_residual_count,
        named_a3_residual_count,
        total_named_residual_count,
        silent_residue_count,
        t_bi_b1_proved_on_sequence,
        t_bi_b2_proved_on_sequence,
        derivation_hash: String::new(),
    };
    sequence.derivation_hash = tagged_hash("semantic-v5-sequence", &sequence);
    Ok(sequence)
}

pub fn issue_reference_act_local_semantic_sequence_v5()
-> Result<ActLocalSemanticSequenceV5, ActLocalSemanticProvenanceV5Error> {
    let entries = (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    issue_act_local_semantic_sequence_v5(&entries)
}

pub fn replay_act_local_semantic_sequence_v5(
    entries: &[(u32, Telescope)],
    claimed: &ActLocalSemanticSequenceV5,
) -> Vec<String> {
    let mut errors = Vec::new();
    let mut projection = claimed.clone();
    projection.derivation_hash.clear();
    if claimed.derivation_hash != tagged_hash("semantic-v5-sequence", &projection) {
        errors.push("v5 sequence digest mismatch".to_owned());
    }
    match issue_act_local_semantic_sequence_v5(entries) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("v5 sequence differs from create-new reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn entries() -> Vec<(u32, Telescope)> {
        (1..=15)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect()
    }

    fn sequence() -> &'static ActLocalSemanticSequenceV5 {
        static SEQUENCE: OnceLock<ActLocalSemanticSequenceV5> = OnceLock::new();
        SEQUENCE
            .get_or_init(|| issue_reference_act_local_semantic_sequence_v5().expect("v5 sequence"))
    }

    fn resign_outer_layers(sequence: &mut ActLocalSemanticSequenceV5, package_index: usize) {
        sequence.packages[package_index].derivation_hash =
            certificate_digest(&sequence.packages[package_index]);
        sequence.exact_package_derivation_hashes[package_index] =
            sequence.packages[package_index].derivation_hash.clone();
        sequence.intrinsic_sequence_seal = tagged_hash(
            "intrinsic-v5-package-sequence-seal",
            &sequence.exact_package_derivation_hashes,
        );
        sequence.derivation_hash.clear();
        sequence.derivation_hash = tagged_hash("semantic-v5-sequence", sequence);
    }

    #[test]
    fn b1_bridges_every_applicable_ordinary_schema_without_flipping_v4_c1() {
        let sequence = sequence();
        assert_eq!(sequence.packages.len(), 15);
        assert!(sequence.t_bi_b1_proved_on_sequence);
        assert_eq!(
            sequence
                .packages
                .iter()
                .map(|package| package.bridges.len())
                .sum::<usize>(),
            24
        );
        assert!(
            sequence
                .packages
                .iter()
                .flat_map(|package| &package.bridges)
                .all(|bridge| bridge.proved
                    && bridge.legacy_v4_c1_flag_remains_false
                    && bridge.zero_charge_support_only)
        );
    }

    #[test]
    fn b2_resolves_all_250_declarations_by_family_or_theorem_impossibility() {
        let sequence = sequence();
        assert_eq!(sequence.role_declaration_count, 250);
        assert_eq!(
            sequence.proved_family_declaration_count
                + sequence.theorem_impossibility_declaration_count,
            250
        );
        assert_eq!(sequence.silent_residue_count, 0);
        assert_eq!(sequence.named_role_residual_count, 0);
        assert_eq!(sequence.named_quotient_residual_count, 0);
        assert_eq!(sequence.named_a3_residual_count, 0);
        assert_eq!(sequence.total_named_residual_count, 0);
        assert!(sequence.t_bi_b2_proved_on_sequence);
        assert!(sequence.packages.iter().all(|package| {
            package.every_role_declaration_resolved
                && package.every_marginal_family_credited_or_theorem_impossible
                && package.local_anchor_nonreuse_holds
                && package.unified_quotient.fresh_core_inventory_exact
                && package.unified_quotient.fresh_ordinary_inventory_exact
                && package.unified_quotient.fresh_cubical_inventory_exact
                && replay_unified_equivalence_closure_v5(
                    &package.unified_quotient.equivalence_closure,
                )
                .is_empty()
                && package.role_resolutions.iter().all(|resolution| {
                    replay_role_constructor_search_v5(&resolution.constructor_search).is_empty()
                })
        }));
    }

    #[test]
    fn closed_equivalence_replay_rejects_a_resigned_cubical_cross_edge() {
        let mut forged = sequence()
            .packages
            .iter()
            .find_map(|package| {
                let mut proof = package.unified_quotient.equivalence_closure.clone();
                let decision = proof.base_decisions.iter_mut().find(|decision| {
                    decision.left_surface != decision.right_surface
                        && (decision.left_surface == V5UnifiedSurface::CubicalPath
                            || decision.right_surface == V5UnifiedSurface::CubicalPath)
                })?;
                decision.relation = V5UnifiedBaseRelation::Equal {
                    rule: V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge,
                };
                decision.evidence = serde_json::json!({
                    "procedure": "forged-cross-surface-bridge",
                    "bridge_membership": true,
                });
                decision.proved = true;
                decision.derivation_hash.clear();
                decision.derivation_hash = tagged_hash("unified-base-equality-decision", decision);
                Some(proof)
            })
            .expect("historical quotient contains a cubical/noncubical pair");
        forged.derivation_hash.clear();
        forged.derivation_hash = tagged_hash("closed-unified-equivalence-closure", &forged);
        assert!(!replay_unified_equivalence_closure_v5(&forged).is_empty());
    }

    #[test]
    fn former_default_map_precomposition_mutation_fails_closed() {
        let mut search = sequence()
            .packages
            .iter()
            .flat_map(|package| &package.role_resolutions)
            .find(|resolution| resolution.constructor_search.role_kind == "map_precomposition")
            .map(|resolution| resolution.constructor_search.clone())
            .expect("historical map_precomposition declaration");
        assert!(replay_role_constructor_search_v5(&search).is_empty());
        search.no_wildcard_or_default_absence_rule = false;
        search.derivation_hash.clear();
        search.derivation_hash = tagged_hash("four-surface-role-constructor-search", &search);
        assert!(!replay_role_constructor_search_v5(&search).is_empty());
    }

    #[test]
    fn create_new_replay_rejects_a_resigned_forged_a3_zero() {
        let source = entries();
        let mut forged = issue_act_local_semantic_sequence_v5(&source).expect("v5 sequence");
        let package_index = forged
            .packages
            .iter()
            .position(|package| package.exact_a3_capability.fresh_orbit_count > 0)
            .expect("historical package with a nonempty exact A3 window");
        let a3 = &mut forged.packages[package_index].exact_a3_capability;
        a3.fresh_scheme_count = 0;
        a3.fresh_instance_count = 0;
        a3.fresh_orbit_count = 0;
        a3.fresh_exported_orbit_count = 0;
        a3.orbit_capabilities.clear();
        a3.cached_v4_zero_used_as_evidence = true;
        a3.derivation_hash.clear();
        a3.derivation_hash = tagged_hash("exact-prefix-A3-capability-proof", a3);
        resign_outer_layers(&mut forged, package_index);
        assert!(!replay_act_local_semantic_sequence_v5(&source, &forged).is_empty());
    }

    #[test]
    fn special_cases_and_r2_remain_count_blind() {
        let sequence = sequence();
        assert!(sequence.packages[0].stage1_r1_preserved);
        assert!(sequence.packages[1].stage2_constitutive_question_not_assumed);
        assert!(sequence.packages[8].stage9_boundary_decided_by_relation_theorem);
        assert!(sequence.packages[7].r2_generated_instance_not_exported);
        assert!(sequence.packages.iter().all(|package| {
            !package.archive_read
                && !package.structural_nu_read
                && !package.bar_read
                && !package.verdict_read
                && !package.enacted_future_read
                && package.phase_receipts.len() == 8
        }));
    }

    #[test]
    fn sequence_replay_rejects_a_resigned_relation_mutation() {
        let source = entries();
        let sequence = issue_act_local_semantic_sequence_v5(&source).expect("v5 sequence");
        assert!(replay_act_local_semantic_sequence_v5(&source, &sequence).is_empty());
        let mut forged = sequence.clone();
        let resolution = forged.packages[14]
            .role_resolutions
            .first_mut()
            .expect("Stage 15 resolution");
        resolution.resolved = false;
        resolution.derivation_hash = tagged_hash("role-declaration-resolution", resolution);
        forged.packages[14].derivation_hash = certificate_digest(&forged.packages[14]);
        forged.exact_package_derivation_hashes[14] = forged.packages[14].derivation_hash.clone();
        forged.intrinsic_sequence_seal = tagged_hash(
            "intrinsic-v5-package-sequence-seal",
            &forged.exact_package_derivation_hashes,
        );
        forged.derivation_hash.clear();
        forged.derivation_hash = tagged_hash("semantic-v5-sequence", &forged);
        assert!(!replay_act_local_semantic_sequence_v5(&source, &forged).is_empty());
    }

    #[test]
    fn prefix_local_registry_erasure_recovers_stage1_through_4_semantics() {
        let source = (1..=4)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let v3 = issue_act_local_sequence_v3(&source).expect("v3 prefix");
        let v4 = issue_act_local_semantic_sequence_v4(&source).expect("v4 prefix");
        let v5 = issue_act_local_semantic_sequence_v5(&source).expect("v5 prefix");
        let mut accepted = Vec::new();
        let mut proofs = Vec::new();
        for index in 0..source.len() {
            let prefix = SealedSignature::from_telescopes(accepted.clone());
            let proof = prove_prefix_local_role_registry_erasure_v5(
                &prefix,
                &source[index].1,
                &v3[index],
                &v4[index],
                &v5.packages[index],
            )
            .expect("prefix-local erasure proof");
            assert!(replay_prefix_local_role_registry_erasure_v5(
                &prefix,
                &source[index].1,
                &v3[index],
                &v4[index],
                &v5.packages[index],
                &proof,
            )
            .is_empty());
            assert!(proof.proved);
            assert!(proof.target_computation_registry_extension_invariant);
            assert!(proof.resolution_registry_extension_invariant);
            assert!(proof.semantic_nu_registry_extension_invariant);
            assert!(proof.every_marginal_family_locally_credited_or_theorem_impossible);
            assert!(!proof.historical_registry_suffix_used_as_semantic_premise);
            assert!(!proof.old_v4_v5_full_hashes_authoritative_for_prefix_theorem);
            proofs.push(proof);
            accepted.push(source[index].clone());
        }
        assert_eq!(
            proofs
                .iter()
                .map(|proof| proof.recomputed_semantic_nu)
                .collect::<Vec<_>>(),
            vec![1, 0, 1, 3]
        );
        assert_eq!(proofs[0].credited_family_ids.len(), 1);
        assert_eq!(proofs[0].rows.len(), 4);
        assert_eq!(
            proofs[0]
                .rows
                .iter()
                .filter(|row| {
                    row.direct_v5_rule == V5RelationRule::GenericR1Completion
                        && row.resolution_class == V5PrefixLocalResolutionClass::ProvedFamily
                })
                .count(),
            1
        );
    }

    #[test]
    fn prefix_local_erasure_replay_rejects_fully_resigned_inner_mutation() {
        let source = (1..=4)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let v3 = issue_act_local_sequence_v3(&source).expect("v3 prefix");
        let v4 = issue_act_local_semantic_sequence_v4(&source).expect("v4 prefix");
        let v5 = issue_act_local_semantic_sequence_v5(&source).expect("v5 prefix");
        let prefix = SealedSignature::from_telescopes(source[..3].to_vec());
        let proof = prove_prefix_local_role_registry_erasure_v5(
            &prefix,
            &source[3].1,
            &v3[3],
            &v4[3],
            &v5.packages[3],
        )
        .expect("Stage-4 erasure proof");
        let mut forged = proof.clone();
        let row = forged.rows.first_mut().expect("Stage-4 erasure row");
        row.unused_registry_extension_projection_equal = false;
        row.derivation_hash = prefix_local_erasure_row_hash(row);
        forged.derivation_hash = prefix_local_erasure_proof_hash(&forged);
        let errors = replay_prefix_local_role_registry_erasure_v5(
            &prefix,
            &source[3].1,
            &v3[3],
            &v4[3],
            &v5.packages[3],
            &forged,
        );
        assert!(errors.iter().any(|error| error.contains("reissuance")));
    }

    #[test]
    fn source_first_prefix_local_sequence_closes_stage1_through_4_without_legacy_authority() {
        let source = (1..=4)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let sequence =
            issue_prefix_local_semantic_sequence_v5(&source).expect("prefix-local v5 sequence");
        assert!(replay_prefix_local_semantic_sequence_v5(&source, &sequence).is_empty());
        assert_eq!(sequence.semantic_nu_vector, vec![1, 0, 1, 3]);
        assert_eq!(sequence.packages.len(), 4);
        assert!(sequence.every_package_registry_extension_invariant);
        assert!(sequence.no_historical_registry_or_future_input);
        assert!(sequence.t_bi_b1_proved_on_sequence);
        assert!(sequence.t_bi_b2_proved_on_sequence);
        assert!(sequence.packages.iter().all(|package| {
            package.proved
                && package.unused_registry_extension_projection_equal
                && !package.historical_registry_consulted
                && !package.archive_structural_bar_verdict_or_future_read
        }));

        let encoded = serde_json::to_string(&sequence).expect("serialize prefix-local sequence");
        for forbidden in [
            "legacy_package",
            "historical_role_registry",
            "historical_registry_suffix",
            "structural_nu",
            "enacted_future",
        ] {
            assert!(
                !encoded.contains(forbidden),
                "authoritative prefix-local sequence contains forbidden field marker {forbidden}"
            );
        }
    }

    #[test]
    fn source_first_prefix_local_replay_rejects_fully_rehashed_semantic_mutation() {
        let source = (1..=4)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let mut forged =
            issue_prefix_local_semantic_sequence_v5(&source).expect("prefix-local v5 sequence");
        forged.packages[3].semantic_nu += 1;
        forged.packages[3].derivation_hash =
            prefix_local_semantic_package_hash(&forged.packages[3]);
        forged.semantic_nu_vector[3] += 1;
        forged.authoritative_sequence_seal = tagged_hash(
            "prefix-local-semantic-package-seal",
            &forged
                .packages
                .iter()
                .map(|package| package.derivation_hash.as_str())
                .collect::<Vec<_>>(),
        );
        forged.derivation_hash = prefix_local_semantic_sequence_hash(&forged);
        let errors = replay_prefix_local_semantic_sequence_v5(&source, &forged);
        assert!(errors.iter().any(|error| error.contains("reissuance")));
    }
}
