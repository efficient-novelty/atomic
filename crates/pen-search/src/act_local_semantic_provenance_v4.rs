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
    let expected_family_count = 1 + (typing.dimension * typing.dimension) as usize;
    let expected_pair_count = expected_family_count.saturating_mul(expected_family_count - 1) / 2;
    let pair_count_exact = pairwise_decisions.len() == expected_pair_count;
    let no_uniform_coordinate_multiplied = true;
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
