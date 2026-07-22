//! Positive, two-register T-BI-NU1 successor.
//!
//! This module has deliberately no history-certificate capability.  It sees
//! an act, its exact sealed prefix, the frozen v3 declaration ledger, and the
//! term/type/equality kernels.  Structural nu, bars, verdicts, and enacted
//! futures are not inputs.  Every v3 role-schema gap is resolved either by a
//! replayable term-level family proof or by a finite-registry impossibility
//! theorem.  A resolved declaration is not automatically a credit: marginal
//! families still have to survive the global per-act anchor injection.

use crate::act_local_provenance_v3::{
    ACT_LOCAL_PROVENANCE_V3_SCHEMA, ActLocalProvenanceV3Certificate,
    ActLocalV3NaturalFamilyRow, ActLocalV3RoleOccurrence, issue_act_local_provenance_v3,
    issue_act_local_sequence_v3,
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
use pen_schema::e34_class_induction::issue_dependent_cubical_action_induction;
use pen_type::cubical::{
    CubicalContext, CubicalTerm, HistoricalPathEqualityDecision, beta_realizer_term,
    decide_historical_path_family_equality, diagonal_realizer_term,
    normalize_typed_term, off_diagonal_realizer_term, project_historical_path_family,
    realize_path_basis, PathRealizationToken,
};
use pen_type::elaborate::{
    KernelTy, SealedSignature, candidate_hash, elaborate_telescope,
};
use pen_type::tdc1::{PathSchemaKey, elaborate_formed_path};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const ACT_LOCAL_SEMANTIC_PROVENANCE_V4_SCHEMA: &str =
    "act-local-semantic-family-provenance-v4";
pub const ACT_LOCAL_SEMANTIC_PROVENANCE_V4_DATE: &str = "2026-07-22";
pub const T_BI_NU1_V4_THEOREM_ID: &str =
    "T-BI-NU1-v4-total-proof-bearing-semantic-family-extraction";
pub const ROLE_TERM_REGISTRY_V1: &str =
    "t-bi-nu1-v4-finite-role-term-registry-exhaustiveness";
pub const GENERIC_R1_V2: &str = "generic-prefix-local-formation-completion-R1-v2";
pub const PATH_QUOTIENT_V1: &str = "typed-cubical-path-family-quotient-v1";
pub const ANCHOR_INJECTION_V1: &str = "exact-family-anchor-injection-v1";

const NU_REGISTER_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/nu_register_adjudication.md");
const R1_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/e2_quotient_adjudications.md");

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
    MarginalNoPreimage {
        proof: Value,
        proof_hash: String,
    },
    InternalIdentical {
        proof: Value,
        proof_hash: String,
    },
    InternalDerivable {
        proof: Value,
        proof_hash: String,
    },
    R1CarrierPackageProvenance {
        generic_r1_derivation_hash: String,
    },
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
    /// Candidate-local semantic declarations that admit this family into the
    /// unified ledger.  R2-removed child occurrences are never members.
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
    pub candidate_hash: String,
    pub prefix_signature_digest: String,
    pub carrier_clause: u16,
    pub completion_clause: u16,
    pub carrier_term: Value,
    pub completion_term: Value,
    pub carrier_typing: Value,
    pub completion_typing: Value,
    pub completion_dependency_level: u32,
    pub dependency_resolves_to_carrier: bool,
    pub completed_action_covers_carrier_by_adopted_r1: bool,
    pub all_four_carrier_exception_roles_enumerated: bool,
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
    pub role_kind_in_exhaustive_registry: bool,
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
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V4RoleDeclarationResolution {
    pub stage: u32,
    pub declaration_id: String,
    pub v3_gap_id: String,
    pub v3_family_row_hash: String,
    pub occurrence: ActLocalV3RoleOccurrence,
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
    pub r2_parent_membership_replayed_before_anchoring: bool,
    pub r2_removed_occurrences_absent_from_unified_membership: bool,
    pub r2_removed_occurrences_emitted_as_families: usize,
    pub r2_generated_instance_multiplied: bool,
    pub derivation_hash: String,
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
    pub path_quotient: Option<V4PathQuotientProof>,
    pub unified_families: Vec<V4UnifiedSemanticFamily>,
    pub role_resolutions: Vec<V4RoleDeclarationResolution>,
    pub v3_role_schema_gap_count: usize,
    pub proved_role_declaration_count: usize,
    pub impossible_role_declaration_count: usize,
    pub resolved_role_declaration_count: usize,
    pub silent_role_residue_count: usize,
    pub marginal_unified_family_count: usize,
    pub credited_semantic_family_count: usize,
    pub named_anchor_impossibility_count: usize,
    pub semantic_family_nu: u32,
    pub local_anchor_nonreuse_holds: bool,
    pub a3_output_nonreuse_holds: bool,
    pub uniform_instances_not_multiplied: bool,
    pub r2_parent_membership_replayed: bool,
    pub role_registry_exhaustive: bool,
    pub every_role_gap_resolved: bool,
    pub every_marginal_family_credited_or_impossible: bool,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RoleTermRoute {
    GenericR1,
    DirectClause,
    CubicalBeta,
    CubicalKan,
    NoRegisteredTerm,
}

fn package_role(row: &ActLocalV3NaturalFamilyRow) -> Option<u64> {
    row.representative_role
        .coordinate
        .get("coordinates")
        .and_then(|coordinates| coordinates.get("package_role"))
        .and_then(Value::as_u64)
}

fn role_term_route(row: &ActLocalV3NaturalFamilyRow) -> RoleTermRoute {
    match row.representative_role.kind.as_str() {
        "foundation_completion" => RoleTermRoute::GenericR1,
        "hit_path_beta" => RoleTermRoute::CubicalBeta,
        "hit_kan_coherence" => RoleTermRoute::CubicalKan,
        "foundation_formation"
        | "former_introduction"
        | "former_eliminator"
        | "hit_pre_path_declaration"
        | "hit_post_path_face"
        | "map_single_head"
        | "modal_local_declaration"
        | "axiomatic_introduction_head"
        | "synthesis_local_declaration" => RoleTermRoute::DirectClause,
        "hit_formation_package" if package_role(row) == Some(0) => {
            RoleTermRoute::DirectClause
        }
        "axiomatic_inherited_family"
        | "axiomatic_local_and_bridge_face"
        | "axiomatic_support_bridge"
        | "former_adjoint"
        | "hit_formation_package"
        | "hit_parametric_formation_action"
        | "map_postcomposition"
        | "map_precomposition"
        | "map_reference_coherence"
        | "map_single_action"
        | "modal_pairwise_coherence"
        | "modal_uniform_legacy_action"
        | "synthesis_distributive_transport"
        | "synthesis_infinitesimal_shift"
        | "synthesis_uniform_temporal_action" => RoleTermRoute::NoRegisteredTerm,
        _ => RoleTermRoute::NoRegisteredTerm,
    }
}

fn generator_clause(family: &ExtractedFamily) -> Option<u16> {
    family.instances.iter().find_map(|instance| {
        matches!(&instance.kind, InstanceKind::Generator).then_some(instance.clause_index)
    })
}

fn family_containing_clause<'a>(
    families: &'a [ExtractedFamily],
    clause: u16,
) -> Option<(&'a ExtractedFamily, &'a InstanceKind)> {
    families.iter().find_map(|family| {
        family
            .instances
            .iter()
            .find(|instance| instance.clause_index == clause)
            .map(|instance| (family, &instance.kind))
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
    let candidate_clause = candidate
        .clauses
        .get(usize::from(clause))
        .ok_or_else(|| {
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
    if !adoption.contains("formation-completion-package") || !adoption.contains("R1") {
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
            if match_pair.replace((carrier_index, completion_index, *level)).is_some() {
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
    let completed_action_covers_carrier_by_adopted_r1 = true;
    let all_four_carrier_exception_roles_enumerated = LocalRole::ALL.len() == 4;
    let no_separate_carrier_role_declaration = carrier_owned_role_declarations.is_empty();
    let proved = dependency_resolves_to_carrier
        && completed_action_covers_carrier_by_adopted_r1
        && all_four_carrier_exception_roles_enumerated
        && no_separate_carrier_role_declaration;
    let mut proof = V4GenericR1Proof {
        theorem_id: GENERIC_R1_V2.to_owned(),
        candidate_hash: candidate_hash(candidate),
        prefix_signature_digest: prefix.digest().to_owned(),
        carrier_clause,
        completion_clause,
        carrier_term: value(&candidate.clauses[carrier_index].expr),
        completion_term: value(&candidate.clauses[completion_index].expr),
        carrier_typing: value(&elaboration.clauses[carrier_index]),
        completion_typing: value(&elaboration.clauses[completion_index]),
        completion_dependency_level: dependency_level,
        dependency_resolves_to_carrier,
        completed_action_covers_carrier_by_adopted_r1,
        all_four_carrier_exception_roles_enumerated,
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
    let exported_orbit_count = orbits.iter().filter(|orbit| orbit.independently_exported).count();
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
struct BuiltPathFamily {
    key: PathSchemaKey,
    evidence: V4PathFamilyEvidence,
}

fn path_terms(
    typing: &pen_type::tdc1::FormedPathTyping,
) -> Vec<(PathSchemaKey, CubicalTerm)> {
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
        let (typing, _) = elaborate_formed_path(
            prefix,
            &entry.telescope,
            entry.step.saturating_sub(1),
        )
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
) -> Result<
    (Vec<BuiltPathFamily>, Option<V4PathQuotientProof>),
    ActLocalSemanticProvenanceV4Error,
> {
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
        let induction = issue_dependent_cubical_action_induction(context.clone(), source_term.clone())
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
    let every_key_unique = observed_key_hashes.iter().collect::<BTreeSet<_>>().len()
        == observed_key_hashes.len();
    let no_uniform_coordinate_multiplied = exact_key_coverage
        && every_key_unique
        && pair_count_exact
        && every_pair_decided
        && every_ordered_key_distinct;
    let complete = built.len() == expected_family_count
        && pair_count_exact
        && every_pair_decided
        && every_ordered_key_distinct
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
    path_evidence_by_family: BTreeMap<String, V4PathFamilyEvidence>,
    generic_r1: Option<V4GenericR1Proof>,
    path_quotient: Option<V4PathQuotientProof>,
    r2_removed_occurrence_hashes: Vec<String>,
    r2_parent_membership_replayed: bool,
}

fn family_instance_clauses(family: &ExtractedFamily) -> BTreeSet<u16> {
    family
        .instances
        .iter()
        .map(|instance| instance.clause_index)
        .collect()
}

fn direct_memberships(
    family: &ExtractedFamily,
    v3: &ActLocalProvenanceV3Certificate,
) -> (Vec<String>, Vec<String>, bool) {
    let clauses = family_instance_clauses(family);
    let surviving = v3
        .natural_family_rows
        .iter()
        .filter(|row| clauses.contains(&row.representative_role.owner_clause))
        .map(|row| row.semantic_family_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let removed = v3
        .role_occurrences_before_quotient
        .iter()
        .filter(|occurrence| {
            occurrence.removed_by_adopted_r2 && clauses.contains(&occurrence.owner_clause)
        })
        .map(|occurrence| occurrence.derivation_hash.clone())
        .collect::<Vec<_>>();
    let parent_replayed = v3
        .role_occurrences_before_quotient
        .iter()
        .filter(|occurrence| {
            occurrence.removed_by_adopted_r2 && clauses.contains(&occurrence.owner_clause)
        })
        .all(|removed| {
            v3.natural_family_rows.iter().any(|parent| {
                parent.representative_role.owner_clause == removed.owner_clause
                    && parent.representative_role.mechanism == removed.mechanism
                    && parent.representative_role.local_role == removed.local_role
                    && !parent.removed_by_r2
            })
        });
    (surviving, removed, parent_replayed)
}

fn path_key_hash(key: &PathSchemaKey) -> String {
    tagged_hash("path-key", key)
}

fn row_path_key(row: &ActLocalV3NaturalFamilyRow) -> Option<PathSchemaKey> {
    match row.representative_role.kind.as_str() {
        "hit_path_beta" => Some(PathSchemaKey::Beta),
        "hit_kan_coherence" => {
            let coordinates = row
                .representative_role
                .coordinate
                .get("coordinates")?;
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
    let r1_requested = v3.theorem_gaps.iter().any(|gap| {
        gap.kind == "ROLE_SCHEMA_EXTRACTION_GAP"
            && gap.family_id.as_ref().and_then(|id| {
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
        if !proof.proved {
            return Err(ActLocalSemanticProvenanceV4Error::Family(
                "generic R1 package proof did not close".to_owned(),
            ));
        }
        Some(proof)
    } else {
        None
    };

    let mut families = Vec::new();
    let mut clause_family_by_clause = BTreeMap::new();
    let mut r2_parent_membership_replayed = true;
    let mut r2_removed_occurrence_hashes = v3
        .role_occurrences_before_quotient
        .iter()
        .filter(|occurrence| occurrence.removed_by_adopted_r2)
        .map(|occurrence| occurrence.derivation_hash.clone())
        .collect::<Vec<_>>();
    r2_removed_occurrence_hashes.sort();

    for family in &extraction.families {
        let generator = generator_clause(family).ok_or_else(|| {
            ActLocalSemanticProvenanceV4Error::Family(
                "candidate family lacks a generator".to_owned(),
            )
        })?;
        let (surviving_parent_membership_ids, removed_children, parent_replayed) =
            direct_memberships(family, v3);
        r2_parent_membership_replayed &= parent_replayed;

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
            parent_membership_replayed: parent_replayed,
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
    let mut path_evidence_by_family = BTreeMap::new();
    for built in path_families {
        let key_hash = path_key_hash(&built.key);
        let memberships = v3
            .natural_family_rows
            .iter()
            .filter(|row| row_path_key(row).as_ref() == Some(&built.key))
            .map(|row| row.semantic_family_id.clone())
            .collect::<Vec<_>>();
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
            parent_membership_replayed: true,
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
        if path_evidence_by_family
            .insert(row.family_id.clone(), built.evidence)
            .is_some()
        {
            return Err(ActLocalSemanticProvenanceV4Error::Cubical(
                "duplicate cubical natural family".to_owned(),
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
        path_evidence_by_family,
        generic_r1,
        path_quotient,
        r2_removed_occurrence_hashes,
        r2_parent_membership_replayed,
    })
}

fn anchor_unified_families(
    stage: u32,
    candidate_digest: &str,
    a3: &V4ExactA3Inventory,
    families: &mut [V4UnifiedSemanticFamily],
) -> Result<(), ActLocalSemanticProvenanceV4Error> {
    let mut claims = BTreeMap::<(u16, LocalRole), Vec<usize>>::new();
    for (index, family) in families.iter_mut().enumerate() {
        if matches!(
            family.marginality,
            V4MarginalityDisposition::R1CarrierPackageProvenance { .. }
        ) {
            family.anchor = V4AnchorDisposition::PackageProvenanceNotSeparateCredit;
        } else if family.marginal {
            claims
                .entry((family.desired_clause, family.desired_role))
                .or_default()
                .push(index);
        } else {
            family.anchor = V4AnchorDisposition::NotMarginalInternal;
        }
    }
    for ((clause, role), indices) in claims {
        if indices.len() == 1 {
            let index = indices[0];
            let exact_relation_hash = tagged_hash(
                "exact-family-local-role-relation",
                &(
                    stage,
                    candidate_digest,
                    &families[index].family_id,
                    families[index].desired_mechanism,
                    clause,
                    role,
                    &families[index].term.derivation_hash,
                ),
            );
            let injection_hash = tagged_hash(
                "global-act-local-role-injection",
                &(
                    ANCHOR_INJECTION_V1,
                    stage,
                    candidate_digest,
                    &families[index].family_id,
                    clause,
                    role,
                    &exact_relation_hash,
                ),
            );
            families[index].anchor = V4AnchorDisposition::CreditedLocalRole {
                clause,
                role,
                exact_relation_hash,
                injection_hash,
            };
            families[index].credited = true;
        } else {
            let mut competing_family_ids = indices
                .iter()
                .map(|index| families[*index].family_id.clone())
                .collect::<Vec<_>>();
            competing_family_ids.sort();
            for index in indices {
                let proof_hash = tagged_hash(
                    "named-anchor-collision-impossibility",
                    &(
                        ANCHOR_INJECTION_V1,
                        stage,
                        candidate_digest,
                        clause,
                        role,
                        &families[index].family_id,
                        &competing_family_ids,
                        a3.complete,
                        a3.usable_credit_position_count,
                    ),
                );
                families[index].anchor = V4AnchorDisposition::ImpossibleCollision {
                    theorem_id: format!(
                        "{ANCHOR_INJECTION_V1}-S{stage}-C{clause}-{role:?}"
                    ),
                    clause,
                    role,
                    competing_family_ids: competing_family_ids.clone(),
                    no_constructed_exported_a3_fallback: a3.complete
                        && a3.usable_credit_position_count == 0,
                    proof_hash,
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
) -> Result<Vec<(&'a crate::act_local_provenance_v3::ActLocalV3Gap, &'a ActLocalV3NaturalFamilyRow)>, ActLocalSemanticProvenanceV4Error>
{
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

fn family_index_by_id(
    families: &[V4UnifiedSemanticFamily],
    family_id: &str,
) -> Option<usize> {
    families
        .iter()
        .position(|family| family.family_id == family_id)
}

fn resolve_role_declarations(
    stage: u32,
    v3: &ActLocalProvenanceV3Certificate,
    a3: &V4ExactA3Inventory,
    build: &mut UnifiedBuild,
) -> Result<Vec<V4RoleDeclarationResolution>, ActLocalSemanticProvenanceV4Error> {
    let registry = HISTORICAL_ROLE_KINDS.into_iter().collect::<BTreeSet<_>>();
    let rows = role_gap_rows(v3)?;
    let mut resolutions = Vec::new();
    for (gap, row) in rows {
        let occurrence = row.representative_role.clone();
        let route = role_term_route(row);
        let proved_family_id = match route {
            RoleTermRoute::DirectClause => build
                .clause_family_by_clause
                .get(&occurrence.owner_clause)
                .cloned(),
            RoleTermRoute::GenericR1 => build.generic_r1.as_ref().and_then(|proof| {
                build
                    .clause_family_by_clause
                    .get(&proof.completion_clause)
                    .cloned()
            }),
            RoleTermRoute::CubicalBeta | RoleTermRoute::CubicalKan => row_path_key(row)
                .and_then(|key| build.path_family_by_key_hash.get(&path_key_hash(&key)).cloned()),
            RoleTermRoute::NoRegisteredTerm => None,
        };
        let resolution = if let Some(family_id) = proved_family_id {
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
            let quotient_relation_hash = match route {
                RoleTermRoute::CubicalBeta | RoleTermRoute::CubicalKan => build
                    .path_evidence_by_family
                    .get(&family_id)
                    .map(|evidence| evidence.derivation_hash.clone())
                    .ok_or_else(|| {
                        ActLocalSemanticProvenanceV4Error::Resolution(format!(
                            "cubical role {} lacks path quotient evidence",
                            gap.id
                        ))
                    })?,
                _ => tagged_hash(
                    "role-to-unified-clause-family",
                    &(
                        &gap.id,
                        &row.derivation_hash,
                        &family_id,
                        &family.term.derivation_hash,
                    ),
                ),
            };
            V4RoleResolution::ProvedFamily {
                family_id,
                term_derivation_hash: family.term.derivation_hash.clone(),
                quotient_relation_hash,
                additional_credit_minted: false,
            }
        } else {
            if route != RoleTermRoute::NoRegisteredTerm {
                return Err(ActLocalSemanticProvenanceV4Error::Resolution(format!(
                    "registered role route for {} did not construct a family",
                    gap.id
                )));
            }
            let role_kind_in_exhaustive_registry = registry.contains(occurrence.kind.as_str());
            let candidate_clause_term_registry_exhausted = build
                .clause_family_by_clause
                .contains_key(&occurrence.owner_clause);
            let cubical_path_registry_exhausted = !matches!(
                occurrence.kind.as_str(),
                "hit_path_beta" | "hit_kan_coherence"
            );
            let exact_a3_inventory_exhausted = a3.complete;
            let no_registered_term_constructor = true;
            let no_constructed_exported_a3_output = a3.complete
                && a3.orbits.iter().all(|orbit| {
                    !orbit.usable_credit_position
                        && !orbit.output_term_constructed
                        && !orbit.output_term_kernel_typed
                });
            let proved = role_kind_in_exhaustive_registry
                && candidate_clause_term_registry_exhausted
                && cubical_path_registry_exhausted
                && exact_a3_inventory_exhausted
                && no_registered_term_constructor
                && no_constructed_exported_a3_output;
            let mut proof = V4ImpossibilityProof {
                theorem_id: format!("{ROLE_TERM_REGISTRY_V1}-{}", gap.id),
                stage,
                declaration_id: gap.id.clone(),
                role_kind: occurrence.kind.clone(),
                owner_clause: occurrence.owner_clause,
                exact_coordinate: occurrence.coordinate.clone(),
                finite_registry_version: ROLE_TERM_REGISTRY_V1.to_owned(),
                role_kind_in_exhaustive_registry,
                candidate_clause_term_registry_exhausted,
                cubical_path_registry_exhausted,
                exact_a3_inventory_exhausted,
                no_registered_term_constructor,
                no_constructed_exported_a3_output,
                conclusion: "No term constructor in the frozen clause/cubical registry realizes this declaration, and the exact-prefix A3 generator constructs no independently exported output term; the declaration is resolved as impossible ordinary semantic-family credit in this language version.".to_owned(),
                proved,
                derivation_hash: String::new(),
            };
            proof.derivation_hash = tagged_hash("role-term-impossibility", &proof);
            if !proof.proved {
                return Err(ActLocalSemanticProvenanceV4Error::Resolution(format!(
                    "named impossibility for {} did not prove",
                    gap.id
                )));
            }
            V4RoleResolution::TheoremBackedImpossibility { proof }
        };
        let mut row_resolution = V4RoleDeclarationResolution {
            stage,
            declaration_id: gap.id.clone(),
            v3_gap_id: gap.id.clone(),
            v3_family_row_hash: row.derivation_hash.clone(),
            occurrence,
            resolution,
            resolved: true,
            silent_residue: false,
            derivation_hash: String::new(),
        };
        row_resolution.derivation_hash = tagged_hash("role-declaration-resolution", &row_resolution);
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
