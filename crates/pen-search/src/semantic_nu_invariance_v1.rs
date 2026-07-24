//! Finite semantic-family cardinality invariance and its Stage-4 scope.
//!
//! This module proves the theorem that the semantic-family count is
//! invariant under an *exported finite bijection of credited family
//! classes*.  It then audits the exact equivalence surfaces currently
//! available to the Stage-4 cone.  The distinction is intentional:
//! cardinality invariance is a theorem, while UC-1a/UC-1b do not yet export
//! the typed cross-package family transports needed to instantiate it.

use crate::act_local_semantic_provenance_v5::V5PrefixLocalSemanticPackageProof;
use crate::naturality_orbit_transport::{
    NATURALITY_ORBIT_TRANSPORT_SCHEMA, NaturalityOrbitTransportCertificate, Stage4PackageComparison,
};
use crate::stage4_semantic_parsimony_v1::{
    STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA, Stage4SemanticRootAuditV1,
};
use crate::stage4_semantic_parsimony_v3::{
    STAGE4_SEMANTIC_PARSIMONY_V3_SCHEMA, Stage4SemanticParsimonyV3Certificate,
};
use crate::t_bi_intrinsic_isolation_v3::issue_replayed_t_bi_intrinsic_isolation_v3_context;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::candidate_hash;
use pen_type::equality::KERNEL_EQUALITY_PROCEDURE;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const SEMANTIC_NU_INVARIANCE_V1_SCHEMA: &str = "semantic-family-cardinality-invariance-v1";
pub const SEMANTIC_NU_INVARIANCE_V1_DATE: &str = "2026-07-23";
pub const SEMANTIC_NU_INVARIANCE_V1_THEOREM_ID: &str =
    "T-NU-EQ1-finite-family-bijection-preserves-semantic-nu";
pub const SEMANTIC_NU_INVARIANCE_V1_CERTIFICATE_NAME: &str = "semantic_nu_invariance_v1.json";
pub const SEMANTIC_NU_INVARIANCE_V1_REPORT_NAME: &str = "SEMANTIC_NU_INVARIANCE_V1_RESULT.md";

pub const UC1_CROSS_PACKAGE_FAMILY_BIJECTION_API_GAP: &str =
    "UC1_CROSS_PACKAGE_SEMANTIC_FAMILY_BIJECTION_API_ABSENT";
pub const UC1A_ORDER_TRANSPORT_GAP: &str =
    "UC1A_TYPED_PARAMETER_SWAP_AND_FAMILY_TRANSPORT_UNEXPORTED";
pub const UC1B_FORMER_TRANSPORT_GAP: &str =
    "UC1B_LIBRARY_ACT_EQUIVALENCE_AND_FAMILY_TRANSPORT_UNEXPORTED";

const STAGE4_V3_BYTES: &[u8] = include_bytes!("../../../docs/stage4_semantic_parsimony_v3.json");
const NATURALITY_TRANSPORT_BYTES: &[u8] =
    include_bytes!("../../../docs/naturality_orbit_transport_v1.json");
const STAGE4_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/stage4_semantic_divergence_adjudication.md");
const BI1B_PLAN_BYTES: &[u8] =
    include_bytes!("../../../docs/bi1b_prefix_general_provenance_plan.md");
const UC1_HYPOTHESIS_BYTES: &[u8] =
    include_bytes!("../../../docs/univalent_collapse_hypothesis.md");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const V5_SOURCE_BYTES: &[u8] = include_bytes!("act_local_semantic_provenance_v5.rs");
const NATURALITY_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("semantic_nu_invariance_v1.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NuInvarianceSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyMapRow {
    pub source_family_id: String,
    pub target_family_id: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FiniteFamilyBijectionTheorem {
    pub theorem_id: String,
    pub relation_id: String,
    pub source_object_id: String,
    pub target_object_id: String,
    pub source_family_ids: Vec<String>,
    pub target_family_ids: Vec<String>,
    pub forward: Vec<FamilyMapRow>,
    pub backward: Vec<FamilyMapRow>,
    pub source_is_finite_set: bool,
    pub target_is_finite_set: bool,
    pub forward_is_total_function: bool,
    pub backward_is_total_function: bool,
    pub forward_is_injective: bool,
    pub backward_is_injective: bool,
    pub forward_then_backward_is_identity: bool,
    pub backward_then_forward_is_identity: bool,
    pub certified_bijection: bool,
    pub source_cardinality: usize,
    pub target_cardinality: usize,
    pub cardinalities_equal: bool,
    pub proof_by_two_inverse_injections: bool,
    pub semantic_nu_invariant: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4FamilyPackageSnapshot {
    pub candidate_hash: String,
    pub former_axis: String,
    pub application_order_axis: String,
    pub telescope: Telescope,
    pub prefix_signature_digest: String,
    pub archived_root_derivation_hash: String,
    pub archived_stage4_package_derivation_hash: String,
    pub current_b3_token_derivation_hash: String,
    pub current_package_derivation_hash: String,
    pub current_package_proved: bool,
    pub current_package_t_bi_b1_proved: bool,
    pub current_package_t_bi_b2_proved: bool,
    pub current_package_zero_named_or_silent_residue: bool,
    pub credited_family_ids: Vec<String>,
    pub credited_family_row_derivation_hashes: Vec<String>,
    pub semantic_nu: u32,
    pub cardinality_matches_semantic_nu: bool,
    pub archived_and_current_semantic_nu_match: bool,
    pub archived_and_current_package_hash_match: bool,
    pub identity_bijection: FiniteFamilyBijectionTheorem,
    pub derivation_hash: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdoptedEquivalenceSurfaceKind {
    FrozenKernelUnivalentEquality,
    V5UnifiedWithinPackageEquivalenceClosure,
    NaturalityOrbitFamilyTransport,
    Stage4Rt1PointwisePackageEquality,
    Rt2SuccessorSchemeRenaming,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptedEquivalenceSurfaceAudit {
    pub surface: AdoptedEquivalenceSurfaceKind,
    pub evidence_hash: String,
    pub exact_carrier: String,
    pub exports_finite_semantic_family_bijection: bool,
    pub exported_bijection_instances: usize,
    pub covered_by_cardinality_theorem: bool,
    pub cannot_be_used_as_cross_package_family_transport: bool,
    pub exact_reason: String,
    pub derivation_hash: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage4Axis {
    Order,
    Former,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4AxisInstantiationAudit {
    pub axis: Stage4Axis,
    pub fixed_coordinate: String,
    pub left_candidate_hash: String,
    pub right_candidate_hash: String,
    pub left_semantic_nu: u32,
    pub right_semantic_nu: u32,
    pub semantic_nu_equal: bool,
    pub r_t1_comparison_derivation_hash: String,
    pub r_t1_packages_equal: bool,
    pub r_t1_exports_cross_package_family_map: bool,
    pub typed_act_equivalence_witness_present: bool,
    pub extractor_functoriality_witness_present: bool,
    pub finite_family_bijection_witness_present: bool,
    pub cardinality_theorem_instantiated: bool,
    pub conditional_obstruction_if_equivalence_requires_family_bijection: bool,
    pub gap_id: String,
    pub exact_missing_construction: String,
    pub f_uc4_assessment_authorized: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticNuInvarianceV1Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub source_bindings: Vec<NuInvarianceSourceBinding>,
    pub stage4_option_a_and_f_uc4_markers_replayed: bool,
    pub bi1b_invariance_gate_marker_replayed: bool,
    pub uc1_hypothesis_markers_replayed: bool,
    pub archived_stage4_v3_digest_valid: bool,
    pub archived_naturality_transport_digest_valid: bool,
    pub frozen_equality_procedure: String,
    pub stage4_packages: Vec<Stage4FamilyPackageSnapshot>,
    pub exact_four_current_b1_b2_b3_packages: bool,
    pub exact_stage4_semantic_vector: Vec<u32>,
    pub a3_naturality_family_bijection: FiniteFamilyBijectionTheorem,
    pub adopted_surface_audits: Vec<AdoptedEquivalenceSurfaceAudit>,
    pub every_exported_semantic_family_bijection_covered: bool,
    pub stage4_axis_audits: Vec<Stage4AxisInstantiationAudit>,
    pub uc1a_order_axis_instantiates_invariance: bool,
    pub uc1b_former_axis_instantiates_invariance: bool,
    pub order_axis_has_conditional_cardinality_obstruction: bool,
    pub former_axis_semantic_nu_invisible: bool,
    pub f_uc4_assessment_authorized: bool,
    pub named_construction_gaps: Vec<String>,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub forbidden_conclusion: String,
    pub uc1_scored: bool,
    pub bi2_outcome_read: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticNuInvarianceV1Replay {
    pub valid: bool,
    pub generic_bijection_theorem_valid: bool,
    pub every_exported_semantic_family_bijection_covered: bool,
    pub exact_four_current_packages: bool,
    pub f_uc4_assessment_authorized: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SemanticNuInvarianceV1Error {
    #[error("semantic-nu invariance prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("semantic-nu invariance theorem failed: {0}")]
    Theorem(String),
    #[error("semantic-nu invariance binding failed: {0}")]
    Binding(String),
    #[error("semantic-nu invariance JSON failed: {0}")]
    Json(String),
    #[error("semantic-nu invariance I/O failed: {0}")]
    Io(String),
    #[error("emitted semantic-nu invariance replay failed: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(SEMANTIC_NU_INVARIANCE_V1_SCHEMA, domain, value))
        .expect("semantic-nu invariance evidence serializes");
    bytes_hash(&bytes)
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("external evidence serializes");
    bytes_hash(&bytes)
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> NuInvarianceSourceBinding {
    NuInvarianceSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    }
}

fn source_bindings() -> Vec<NuInvarianceSourceBinding> {
    vec![
        source_binding(
            "docs/stage4_semantic_divergence_adjudication.md",
            "adopted_Option_A_and_F_UC4_scope",
            STAGE4_ADJUDICATION_BYTES,
        ),
        source_binding(
            "docs/bi1b_prefix_general_provenance_plan.md",
            "BI1b_invariance_gate_without_UC1_scoring",
            BI1B_PLAN_BYTES,
        ),
        source_binding(
            "docs/univalent_collapse_hypothesis.md",
            "frozen_UC1a_UC1b_and_F_UC4_statement",
            UC1_HYPOTHESIS_BYTES,
        ),
        source_binding(
            "docs/stage4_semantic_parsimony_v3.json",
            "four_cross_bound_source_first_semantic_package_proofs",
            STAGE4_V3_BYTES,
        ),
        source_binding(
            "docs/naturality_orbit_transport_v1.json",
            "adopted_R_T1_package_equality_and_A3_family_transport_surface",
            NATURALITY_TRANSPORT_BYTES,
        ),
        source_binding(
            "crates/pen-type/src/equality.rs",
            "frozen_expression_equality_surface",
            EQUALITY_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/act_local_semantic_provenance_v5.rs",
            "credited_family_sets_and_within_package_equivalence_closure",
            V5_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "A3_and_R_T1_transport_issuer",
            NATURALITY_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/semantic_nu_invariance_v1.rs",
            "finite_bijection_cardinality_theorem_and_scope_audit",
            THIS_SOURCE_BYTES,
        ),
    ]
}

fn map_row(source: &str, target: &str) -> FamilyMapRow {
    let mut row = FamilyMapRow {
        source_family_id: source.to_owned(),
        target_family_id: target.to_owned(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash(
        "family-map-row",
        &(&row.source_family_id, &row.target_family_id),
    );
    row
}

fn map_row_valid(row: &FamilyMapRow) -> bool {
    row.derivation_hash
        == tagged_hash(
            "family-map-row",
            &(&row.source_family_id, &row.target_family_id),
        )
}

fn theorem_hash(theorem: &FiniteFamilyBijectionTheorem) -> String {
    let mut projection = theorem.clone();
    projection.derivation_hash.clear();
    tagged_hash("finite-family-bijection-cardinality-theorem", &projection)
}

/// Verify a concrete finite bijection and derive equality of family counts.
///
/// The inputs are family-class identifiers, not family instances.  Duplicate
/// identifiers are rejected, and both inverse laws are checked extensionally.
/// Consequently any relation admitted by this API preserves semantic `nu`.
pub fn prove_finite_family_bijection_cardinality(
    relation_id: &str,
    source_object_id: &str,
    target_object_id: &str,
    mut source_family_ids: Vec<String>,
    mut target_family_ids: Vec<String>,
    mut forward: Vec<FamilyMapRow>,
    mut backward: Vec<FamilyMapRow>,
) -> Result<FiniteFamilyBijectionTheorem, SemanticNuInvarianceV1Error> {
    let source_input_len = source_family_ids.len();
    let target_input_len = target_family_ids.len();
    source_family_ids.sort();
    target_family_ids.sort();
    let source_is_finite_set = source_family_ids.windows(2).all(|row| row[0] != row[1])
        && source_family_ids.len() == source_input_len;
    let target_is_finite_set = target_family_ids.windows(2).all(|row| row[0] != row[1])
        && target_family_ids.len() == target_input_len;
    if !source_is_finite_set || !target_is_finite_set {
        return Err(SemanticNuInvarianceV1Error::Theorem(
            "family carriers must be duplicate-free finite sets".to_owned(),
        ));
    }
    forward.sort_by(|left, right| {
        (&left.source_family_id, &left.target_family_id)
            .cmp(&(&right.source_family_id, &right.target_family_id))
    });
    backward.sort_by(|left, right| {
        (&left.source_family_id, &left.target_family_id)
            .cmp(&(&right.source_family_id, &right.target_family_id))
    });
    if forward.iter().any(|row| !map_row_valid(row))
        || backward.iter().any(|row| !map_row_valid(row))
    {
        return Err(SemanticNuInvarianceV1Error::Theorem(
            "a family map row failed its content-addressed replay".to_owned(),
        ));
    }
    let source_set = source_family_ids.iter().cloned().collect::<BTreeSet<_>>();
    let target_set = target_family_ids.iter().cloned().collect::<BTreeSet<_>>();
    let forward_by_source = forward
        .iter()
        .map(|row| (row.source_family_id.clone(), row.target_family_id.clone()))
        .collect::<BTreeMap<_, _>>();
    let backward_by_source = backward
        .iter()
        .map(|row| (row.source_family_id.clone(), row.target_family_id.clone()))
        .collect::<BTreeMap<_, _>>();
    let forward_is_total_function = forward.len() == source_set.len()
        && forward_by_source.len() == source_set.len()
        && forward_by_source.keys().cloned().collect::<BTreeSet<_>>() == source_set
        && forward_by_source
            .values()
            .all(|target| target_set.contains(target));
    let backward_is_total_function = backward.len() == target_set.len()
        && backward_by_source.len() == target_set.len()
        && backward_by_source.keys().cloned().collect::<BTreeSet<_>>() == target_set
        && backward_by_source
            .values()
            .all(|source| source_set.contains(source));
    let forward_is_injective = forward_by_source
        .values()
        .cloned()
        .collect::<BTreeSet<_>>()
        .len()
        == forward_by_source.len();
    let backward_is_injective = backward_by_source
        .values()
        .cloned()
        .collect::<BTreeSet<_>>()
        .len()
        == backward_by_source.len();
    let forward_then_backward_is_identity = forward_is_total_function
        && backward_is_total_function
        && source_set.iter().all(|source| {
            forward_by_source
                .get(source)
                .and_then(|target| backward_by_source.get(target))
                == Some(source)
        });
    let backward_then_forward_is_identity = forward_is_total_function
        && backward_is_total_function
        && target_set.iter().all(|target| {
            backward_by_source
                .get(target)
                .and_then(|source| forward_by_source.get(source))
                == Some(target)
        });
    let certified_bijection = forward_is_total_function
        && backward_is_total_function
        && forward_is_injective
        && backward_is_injective
        && forward_then_backward_is_identity
        && backward_then_forward_is_identity;
    if !certified_bijection {
        return Err(SemanticNuInvarianceV1Error::Theorem(
            "supplied maps do not satisfy the two finite inverse laws".to_owned(),
        ));
    }
    let source_cardinality = source_set.len();
    let target_cardinality = target_set.len();
    let cardinalities_equal = source_cardinality == target_cardinality;
    let proof_by_two_inverse_injections =
        forward_is_injective && backward_is_injective && cardinalities_equal;
    let semantic_nu_invariant =
        certified_bijection && cardinalities_equal && proof_by_two_inverse_injections;
    if !semantic_nu_invariant {
        return Err(SemanticNuInvarianceV1Error::Theorem(
            "a certified finite bijection failed cardinality invariance".to_owned(),
        ));
    }
    let mut theorem = FiniteFamilyBijectionTheorem {
        theorem_id: SEMANTIC_NU_INVARIANCE_V1_THEOREM_ID.to_owned(),
        relation_id: relation_id.to_owned(),
        source_object_id: source_object_id.to_owned(),
        target_object_id: target_object_id.to_owned(),
        source_family_ids,
        target_family_ids,
        forward,
        backward,
        source_is_finite_set,
        target_is_finite_set,
        forward_is_total_function,
        backward_is_total_function,
        forward_is_injective,
        backward_is_injective,
        forward_then_backward_is_identity,
        backward_then_forward_is_identity,
        certified_bijection,
        source_cardinality,
        target_cardinality,
        cardinalities_equal,
        proof_by_two_inverse_injections,
        semantic_nu_invariant,
        derivation_hash: String::new(),
    };
    theorem.derivation_hash = theorem_hash(&theorem);
    Ok(theorem)
}

pub fn replay_finite_family_bijection_cardinality(
    theorem: &FiniteFamilyBijectionTheorem,
) -> Vec<String> {
    let mut errors = Vec::new();
    if theorem.derivation_hash != theorem_hash(theorem) {
        errors.push("finite-family theorem digest mismatch".to_owned());
    }
    match prove_finite_family_bijection_cardinality(
        &theorem.relation_id,
        &theorem.source_object_id,
        &theorem.target_object_id,
        theorem.source_family_ids.clone(),
        theorem.target_family_ids.clone(),
        theorem.forward.clone(),
        theorem.backward.clone(),
    ) {
        Ok(expected) if expected == *theorem => {}
        Ok(_) => errors.push("finite-family theorem differs from deterministic replay".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn identity_bijection(
    relation_id: &str,
    object_id: &str,
    family_ids: &[String],
) -> Result<FiniteFamilyBijectionTheorem, SemanticNuInvarianceV1Error> {
    let rows = family_ids
        .iter()
        .map(|family_id| map_row(family_id, family_id))
        .collect::<Vec<_>>();
    prove_finite_family_bijection_cardinality(
        relation_id,
        object_id,
        object_id,
        family_ids.to_vec(),
        family_ids.to_vec(),
        rows.clone(),
        rows,
    )
}

fn archived_v3_digest_valid(certificate: &Stage4SemanticParsimonyV3Certificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            STAGE4_SEMANTIC_PARSIMONY_V3_SCHEMA,
            "authorized-cross-bound-stage4-audit",
            &projection,
        )
}

fn archived_root_digest_valid(root: &Stage4SemanticRootAuditV1) -> bool {
    let mut projection = root.clone();
    let observed = projection.derivation_hash.clone();
    projection.derivation_hash.clear();
    observed
        == external_tagged_hash(
            STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA,
            "semantic-root-audit",
            &projection,
        )
}

fn archived_naturality_digest_valid(certificate: &NaturalityOrbitTransportCertificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            NATURALITY_ORBIT_TRANSPORT_SCHEMA,
            "certificate",
            &projection,
        )
}

fn parse_stage4_axes(
    telescope: &Telescope,
) -> Result<(String, String), SemanticNuInvarianceV1Error> {
    if telescope.clauses.len() != 3 {
        return Err(SemanticNuInvarianceV1Error::Binding(
            "Stage-4 candidate is not a three-clause package".to_owned(),
        ));
    }
    let former_axis = match &telescope.clauses[0].expr {
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::Pi(left, right)
                    if left.as_ref() == &Expr::Var(1) && right.as_ref() == &Expr::Var(2)
            ) =>
        {
            "pi"
        }
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::Sigma(left, right)
                    if left.as_ref() == &Expr::Var(1) && right.as_ref() == &Expr::Var(2)
            ) =>
        {
            "sigma"
        }
        _ => {
            return Err(SemanticNuInvarianceV1Error::Binding(
                "Stage-4 former coordinate is outside the frozen Pi/Sigma surface".to_owned(),
            ));
        }
    };
    let application_order_axis = match &telescope.clauses[1].expr {
        Expr::App(left, right)
            if right.as_ref() == &Expr::Var(3)
                && matches!(
                    left.as_ref(),
                    Expr::App(head, first)
                        if head.as_ref() == &Expr::Var(1)
                            && first.as_ref() == &Expr::Var(2)
                ) =>
        {
            "2_then_3"
        }
        Expr::App(left, right)
            if right.as_ref() == &Expr::Var(2)
                && matches!(
                    left.as_ref(),
                    Expr::App(head, first)
                        if head.as_ref() == &Expr::Var(1)
                            && first.as_ref() == &Expr::Var(3)
                ) =>
        {
            "3_then_2"
        }
        _ => {
            return Err(SemanticNuInvarianceV1Error::Binding(
                "Stage-4 order coordinate is outside the frozen two-order surface".to_owned(),
            ));
        }
    };
    Ok((former_axis.to_owned(), application_order_axis.to_owned()))
}

fn package_snapshot_hash(snapshot: &Stage4FamilyPackageSnapshot) -> String {
    let mut projection = snapshot.clone();
    projection.derivation_hash.clear();
    tagged_hash("stage4-current-family-package-snapshot", &projection)
}

fn credited_row_hashes(package: &V5PrefixLocalSemanticPackageProof) -> Vec<String> {
    let credited = package
        .credited_family_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut hashes = package
        .family_rows
        .iter()
        .filter(|row| row.credited && credited.contains(&row.family_id))
        .map(|row| row.derivation_hash.clone())
        .collect::<Vec<_>>();
    hashes.sort();
    hashes
}

fn reissue_stage4_packages(
    archived: &Stage4SemanticParsimonyV3Certificate,
) -> Result<Vec<Stage4FamilyPackageSnapshot>, SemanticNuInvarianceV1Error> {
    let v1 = &archived.replayed_v2_claim.blind_audit;
    let preseal = &v1.preseal;
    let stem = preseal
        .opening_token
        .common_prefix_entries
        .iter()
        .map(|entry| (entry.stage, entry.telescope.clone()))
        .collect::<Vec<_>>();
    if stem.len() != 3 || !stem.iter().map(|(stage, _)| *stage).eq(1..=3) {
        return Err(SemanticNuInvarianceV1Error::Binding(
            "archived Stage-4 proof does not expose the exact contiguous Stage-1-through-3 stem"
                .to_owned(),
        ));
    }
    let geometry_by_hash = preseal
        .live_strict_cone_geometry
        .roots
        .iter()
        .map(|root| (root.candidate_hash.clone(), root.telescope.clone()))
        .collect::<BTreeMap<_, _>>();
    if geometry_by_hash.len() != 4 || preseal.roots.len() != 4 {
        return Err(SemanticNuInvarianceV1Error::Binding(format!(
            "Stage-4 proof has {} geometry roots and {} semantic roots, expected four/four",
            geometry_by_hash.len(),
            preseal.roots.len()
        )));
    }
    let mut snapshots = Vec::new();
    for archived_root in &preseal.roots {
        if !archived_root_digest_valid(archived_root) || !archived_root.root_semantic_audit_proved {
            return Err(SemanticNuInvarianceV1Error::Binding(format!(
                "archived semantic root {} failed its content proof",
                archived_root.candidate_hash
            )));
        }
        let telescope = geometry_by_hash
            .get(&archived_root.candidate_hash)
            .cloned()
            .ok_or_else(|| {
                SemanticNuInvarianceV1Error::Binding(format!(
                    "semantic root {} has no exact geometry telescope",
                    archived_root.candidate_hash
                ))
            })?;
        if candidate_hash(&telescope) != archived_root.candidate_hash {
            return Err(SemanticNuInvarianceV1Error::Binding(
                "Stage-4 geometry candidate hash failed recomputation".to_owned(),
            ));
        }
        let mut entries = stem.clone();
        entries.push((4, telescope.clone()));
        let context = issue_replayed_t_bi_intrinsic_isolation_v3_context(&entries)
            .map_err(|error| SemanticNuInvarianceV1Error::Binding(error.to_string()))?;
        let package = context.sequence.packages.last().ok_or_else(|| {
            SemanticNuInvarianceV1Error::Binding(
                "current B3 replay omitted its Stage-4 package".to_owned(),
            )
        })?;
        let mut credited_family_ids = package.credited_family_ids.clone();
        credited_family_ids.sort();
        let family_set_is_unique = credited_family_ids.windows(2).all(|row| row[0] != row[1]);
        let cardinality_matches_semantic_nu = family_set_is_unique
            && u32::try_from(credited_family_ids.len()).ok() == Some(package.semantic_nu);
        let current_package_zero_named_or_silent_residue = package.named_role_residual_count == 0
            && package.named_quotient_residual_count == 0
            && package.named_a3_residual_count == 0
            && package.silent_residue_count == 0;
        let archived_and_current_semantic_nu_match =
            archived_root.stage4_semantic_nu == package.semantic_nu;
        let archived_and_current_package_hash_match = archived_root
            .authoritative_prefix_local_v5_stage4_package_hash
            == package.derivation_hash;
        if !context.proved
            || !context.replay_errors.is_empty()
            || !package.proved
            || !package.t_bi_b1_proved
            || !package.t_bi_b2_proved
            || !current_package_zero_named_or_silent_residue
            || !cardinality_matches_semantic_nu
            || !archived_and_current_semantic_nu_match
            || !archived_and_current_package_hash_match
        {
            return Err(SemanticNuInvarianceV1Error::Binding(format!(
                "current Stage-4 B1/B2/B3 package {} failed exact archived replay",
                archived_root.candidate_hash
            )));
        }
        let (former_axis, application_order_axis) = parse_stage4_axes(&telescope)?;
        let identity_bijection = identity_bijection(
            "stage4-r-t1-reflexive-family-identity",
            &archived_root.candidate_hash,
            &credited_family_ids,
        )?;
        let mut snapshot = Stage4FamilyPackageSnapshot {
            candidate_hash: archived_root.candidate_hash.clone(),
            former_axis,
            application_order_axis,
            telescope,
            prefix_signature_digest: archived_root.prefix_signature_digest.clone(),
            archived_root_derivation_hash: archived_root.derivation_hash.clone(),
            archived_stage4_package_derivation_hash: archived_root
                .authoritative_prefix_local_v5_stage4_package_hash
                .clone(),
            current_b3_token_derivation_hash: context.token.derivation_hash.clone(),
            current_package_derivation_hash: package.derivation_hash.clone(),
            current_package_proved: package.proved,
            current_package_t_bi_b1_proved: package.t_bi_b1_proved,
            current_package_t_bi_b2_proved: package.t_bi_b2_proved,
            current_package_zero_named_or_silent_residue,
            credited_family_ids,
            credited_family_row_derivation_hashes: credited_row_hashes(package),
            semantic_nu: package.semantic_nu,
            cardinality_matches_semantic_nu,
            archived_and_current_semantic_nu_match,
            archived_and_current_package_hash_match,
            identity_bijection,
            derivation_hash: String::new(),
        };
        snapshot.derivation_hash = package_snapshot_hash(&snapshot);
        snapshots.push(snapshot);
    }
    snapshots.sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));
    Ok(snapshots)
}

fn surface_hash(audit: &AdoptedEquivalenceSurfaceAudit) -> String {
    let mut projection = audit.clone();
    projection.derivation_hash.clear();
    tagged_hash("adopted-equivalence-surface-audit", &projection)
}

fn surface_audit(
    surface: AdoptedEquivalenceSurfaceKind,
    evidence_hash: String,
    exact_carrier: &str,
    exports_finite_semantic_family_bijection: bool,
    exported_bijection_instances: usize,
    covered_by_cardinality_theorem: bool,
    cannot_be_used_as_cross_package_family_transport: bool,
    exact_reason: &str,
) -> AdoptedEquivalenceSurfaceAudit {
    let mut audit = AdoptedEquivalenceSurfaceAudit {
        surface,
        evidence_hash,
        exact_carrier: exact_carrier.to_owned(),
        exports_finite_semantic_family_bijection,
        exported_bijection_instances,
        covered_by_cardinality_theorem,
        cannot_be_used_as_cross_package_family_transport,
        exact_reason: exact_reason.to_owned(),
        derivation_hash: String::new(),
    };
    audit.derivation_hash = surface_hash(&audit);
    audit
}

fn comparison_for<'a>(
    comparisons: &'a [Stage4PackageComparison],
    left: &str,
    right: &str,
) -> Result<&'a Stage4PackageComparison, SemanticNuInvarianceV1Error> {
    let rows = comparisons
        .iter()
        .filter(|row| {
            (row.left_candidate_hash == left && row.right_candidate_hash == right)
                || (row.left_candidate_hash == right && row.right_candidate_hash == left)
        })
        .collect::<Vec<_>>();
    if rows.len() != 1 {
        return Err(SemanticNuInvarianceV1Error::Binding(format!(
            "R-T1 has {} comparisons for Stage-4 pair {left}/{right}",
            rows.len()
        )));
    }
    Ok(rows[0])
}

fn axis_hash(audit: &Stage4AxisInstantiationAudit) -> String {
    let mut projection = audit.clone();
    projection.derivation_hash.clear();
    tagged_hash("stage4-axis-invariance-instantiation-audit", &projection)
}

fn axis_audit(
    axis: Stage4Axis,
    fixed_coordinate: &str,
    left: &Stage4FamilyPackageSnapshot,
    right: &Stage4FamilyPackageSnapshot,
    comparison: &Stage4PackageComparison,
) -> Stage4AxisInstantiationAudit {
    let semantic_nu_equal = left.semantic_nu == right.semantic_nu;
    let (gap_id, exact_missing_construction) = match axis {
        Stage4Axis::Order => (
            UC1A_ORDER_TRANSPORT_GAP,
            "Construct a typed proof that the two parameters are mutually non-dependent; construct the context-swap equivalence and its inverse laws; prove that source-first family extraction is functorial under that swap; and export the resulting forward/backward maps on credited semantic-family classes.",
        ),
        Stage4Axis::Former => (
            UC1B_FORMER_TRANSPORT_GAP,
            "Construct the claimed library equivalence B4^Pi ~= B4^Sigma with former-eliminator witness correspondence; prove source-first extractor functoriality across that library equivalence; and export inverse forward/backward maps on credited semantic-family classes.",
        ),
    };
    let mut audit = Stage4AxisInstantiationAudit {
        axis,
        fixed_coordinate: fixed_coordinate.to_owned(),
        left_candidate_hash: left.candidate_hash.clone(),
        right_candidate_hash: right.candidate_hash.clone(),
        left_semantic_nu: left.semantic_nu,
        right_semantic_nu: right.semantic_nu,
        semantic_nu_equal,
        r_t1_comparison_derivation_hash: comparison.derivation_hash.clone(),
        r_t1_packages_equal: comparison.packages_equal,
        r_t1_exports_cross_package_family_map: false,
        typed_act_equivalence_witness_present: false,
        extractor_functoriality_witness_present: false,
        finite_family_bijection_witness_present: false,
        cardinality_theorem_instantiated: false,
        conditional_obstruction_if_equivalence_requires_family_bijection: !semantic_nu_equal,
        gap_id: gap_id.to_owned(),
        exact_missing_construction: exact_missing_construction.to_owned(),
        f_uc4_assessment_authorized: false,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = axis_hash(&audit);
    audit
}

fn package_by_axes<'a>(
    packages: &'a [Stage4FamilyPackageSnapshot],
    former: &str,
    order: &str,
) -> Result<&'a Stage4FamilyPackageSnapshot, SemanticNuInvarianceV1Error> {
    let rows = packages
        .iter()
        .filter(|package| package.former_axis == former && package.application_order_axis == order)
        .collect::<Vec<_>>();
    if rows.len() != 1 {
        return Err(SemanticNuInvarianceV1Error::Binding(format!(
            "Stage-4 package grid has {} rows at ({former},{order})",
            rows.len()
        )));
    }
    Ok(rows[0])
}

fn certificate_hash(certificate: &SemanticNuInvarianceV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

pub fn issue_semantic_nu_invariance_v1()
-> Result<SemanticNuInvarianceV1Certificate, SemanticNuInvarianceV1Error> {
    let stage4_adjudication = std::str::from_utf8(STAGE4_ADJUDICATION_BYTES)
        .map_err(|error| SemanticNuInvarianceV1Error::Prerequisite(error.to_string()))?;
    let bi1b_plan = std::str::from_utf8(BI1B_PLAN_BYTES)
        .map_err(|error| SemanticNuInvarianceV1Error::Prerequisite(error.to_string()))?;
    let uc1 = std::str::from_utf8(UC1_HYPOTHESIS_BYTES)
        .map_err(|error| SemanticNuInvarianceV1Error::Prerequisite(error.to_string()))?;
    let stage4_option_a_and_f_uc4_markers_replayed = stage4_adjudication
        .contains("Option A adopted")
        && stage4_adjudication.contains("F-UC4")
        && stage4_adjudication.contains("UC-1 is not scored here");
    let bi1b_invariance_gate_marker_replayed = bi1b_plan.contains("F-UC4")
        && bi1b_plan.contains("semantic-family invariance lemma")
        && bi1b_plan.contains("UC-1 scoring");
    let uc1_hypothesis_markers_replayed = uc1.contains("UC-1a (order gauge)")
        && uc1.contains("UC-1b (act-level former collapse)")
        && uc1.contains("F-UC4")
        && uc1.contains("F-UC6");
    if !stage4_option_a_and_f_uc4_markers_replayed
        || !bi1b_invariance_gate_marker_replayed
        || !uc1_hypothesis_markers_replayed
    {
        return Err(SemanticNuInvarianceV1Error::Prerequisite(
            "adjudication, BI1b, or frozen UC-1 markers did not replay".to_owned(),
        ));
    }

    let archived_stage4: Stage4SemanticParsimonyV3Certificate =
        serde_json::from_slice(STAGE4_V3_BYTES)
            .map_err(|error| SemanticNuInvarianceV1Error::Json(error.to_string()))?;
    let archived_transport: NaturalityOrbitTransportCertificate =
        serde_json::from_slice(NATURALITY_TRANSPORT_BYTES)
            .map_err(|error| SemanticNuInvarianceV1Error::Json(error.to_string()))?;
    let archived_stage4_v3_digest_valid = archived_v3_digest_valid(&archived_stage4);
    let archived_naturality_transport_digest_valid =
        archived_naturality_digest_valid(&archived_transport);
    if !archived_stage4_v3_digest_valid || !archived_naturality_transport_digest_valid {
        return Err(SemanticNuInvarianceV1Error::Prerequisite(
            "a frozen Stage-4 or naturality certificate failed its own content digest".to_owned(),
        ));
    }

    let stage4_packages = reissue_stage4_packages(&archived_stage4)?;
    let exact_four_current_b1_b2_b3_packages = stage4_packages.len() == 4
        && stage4_packages.iter().all(|package| {
            package.current_package_proved
                && package.current_package_t_bi_b1_proved
                && package.current_package_t_bi_b2_proved
                && package.current_package_zero_named_or_silent_residue
                && package.cardinality_matches_semantic_nu
                && package.archived_and_current_semantic_nu_match
                && package.archived_and_current_package_hash_match
                && replay_finite_family_bijection_cardinality(&package.identity_bijection)
                    .is_empty()
        });
    if !exact_four_current_b1_b2_b3_packages {
        return Err(SemanticNuInvarianceV1Error::Binding(
            "the exact four current Stage-4 package proofs did not close".to_owned(),
        ));
    }
    let exact_stage4_semantic_vector = stage4_packages
        .iter()
        .map(|package| package.semantic_nu)
        .collect::<Vec<_>>();

    if !archived_transport.a3.j3_regression_64_to_8
        || !archived_transport
            .a3
            .pointwise_instances_join_existing_j3_families
        || !archived_transport
            .a3
            .chronological_instance_transport_bijection
    {
        return Err(SemanticNuInvarianceV1Error::Prerequisite(
            "archived naturality theorem does not export its certified family join".to_owned(),
        ));
    }
    let direct_family_ids = archived_transport
        .a3
        .direct_instances
        .iter()
        .map(|instance| instance.family_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let pointwise_family_ids = archived_transport
        .a3
        .pointwise_instances
        .iter()
        .map(|instance| instance.family_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if direct_family_ids != pointwise_family_ids
        || direct_family_ids.len() != archived_transport.a3.direct_natural_family_count
        || pointwise_family_ids.len() != archived_transport.a3.pointwise_natural_family_count
    {
        return Err(SemanticNuInvarianceV1Error::Binding(
            "A3 naturality family carriers do not reproduce the certified common eight".to_owned(),
        ));
    }
    let a3_rows = direct_family_ids
        .iter()
        .map(|family_id| map_row(family_id, family_id))
        .collect::<Vec<_>>();
    let a3_naturality_family_bijection = prove_finite_family_bijection_cardinality(
        "adopted-A3-naturality-pointwise-to-direct-family-identity",
        "A3-direct-J3-family-carrier",
        "A3-pointwise-specialization-family-carrier",
        direct_family_ids,
        pointwise_family_ids,
        a3_rows.clone(),
        a3_rows,
    )?;

    let adopted_surface_audits = vec![
        surface_audit(
            AdoptedEquivalenceSurfaceKind::FrozenKernelUnivalentEquality,
            bytes_hash(EQUALITY_SOURCE_BYTES),
            "two expressions at one scope, equal exactly when their fuel-bounded beta normal forms are syntactically equal",
            false,
            0,
            false,
            true,
            "EqualityWitness has no library-equivalence, family-extractor transport, or forward/backward semantic-family map field.",
        ),
        surface_audit(
            AdoptedEquivalenceSurfaceKind::V5UnifiedWithinPackageEquivalenceClosure,
            bytes_hash(V5_SOURCE_BYTES),
            "the finite CoreExpr/OrdinarySchema2/CubicalPath member universe of one source-first package",
            false,
            0,
            false,
            true,
            "The closed four-rule quotient computes components within one package; it exports no map between credited-family carriers of two candidate packages.",
        ),
        surface_audit(
            AdoptedEquivalenceSurfaceKind::NaturalityOrbitFamilyTransport,
            archived_transport.a3.derivation_hash.clone(),
            "the eight direct J3 natural-family IDs and the eight pointwise specializations joined to those same family IDs",
            true,
            1,
            true,
            true,
            "The exported family identity is covered by T-NU-EQ1. Its carrier is the historical A3/J3 naturality orbit, not either UC-1 Stage-4 act axis.",
        ),
        surface_audit(
            AdoptedEquivalenceSurfaceKind::Stage4Rt1PointwisePackageEquality,
            archived_transport.stage4.derivation_hash.clone(),
            "three clause presentations compared pointwise by role, kernel type, parameter sorts, and frozen expression equality",
            false,
            0,
            false,
            true,
            "All six distinct Stage-4 comparisons have packages_equal=false, and the comparison record exports no semantic-family map even in the equality case.",
        ),
        surface_audit(
            AdoptedEquivalenceSurfaceKind::Rt2SuccessorSchemeRenaming,
            archived_stage4
                .replayed_v2_claim
                .blind_audit
                .frozen_r_t2_certificate_digest
                .clone(),
            "canonical renaming of parameters inside a five-scheme successor comparison",
            false,
            0,
            false,
            true,
            "R-T2's renaming bijection is scheme-local and does not transport source-first credited semantic-family classes between Stage-4 acts.",
        ),
    ];
    let every_exported_semantic_family_bijection_covered = adopted_surface_audits
        .iter()
        .filter(|surface| surface.exports_finite_semantic_family_bijection)
        .all(|surface| surface.covered_by_cardinality_theorem)
        && replay_finite_family_bijection_cardinality(&a3_naturality_family_bijection).is_empty()
        && stage4_packages.iter().all(|package| {
            replay_finite_family_bijection_cardinality(&package.identity_bijection).is_empty()
        });

    let pi_23 = package_by_axes(&stage4_packages, "pi", "2_then_3")?;
    let pi_32 = package_by_axes(&stage4_packages, "pi", "3_then_2")?;
    let sigma_23 = package_by_axes(&stage4_packages, "sigma", "2_then_3")?;
    let sigma_32 = package_by_axes(&stage4_packages, "sigma", "3_then_2")?;
    let comparisons = &archived_transport.stage4.pairwise_comparisons;
    if comparisons.len() != 6 || comparisons.iter().any(|row| row.packages_equal) {
        return Err(SemanticNuInvarianceV1Error::Binding(
            "R-T1 is not the frozen six-comparison/four-singleton-class surface".to_owned(),
        ));
    }
    let stage4_axis_audits = vec![
        axis_audit(
            Stage4Axis::Order,
            "former=pi",
            pi_23,
            pi_32,
            comparison_for(comparisons, &pi_23.candidate_hash, &pi_32.candidate_hash)?,
        ),
        axis_audit(
            Stage4Axis::Order,
            "former=sigma",
            sigma_23,
            sigma_32,
            comparison_for(
                comparisons,
                &sigma_23.candidate_hash,
                &sigma_32.candidate_hash,
            )?,
        ),
        axis_audit(
            Stage4Axis::Former,
            "application_order=2_then_3",
            pi_23,
            sigma_23,
            comparison_for(comparisons, &pi_23.candidate_hash, &sigma_23.candidate_hash)?,
        ),
        axis_audit(
            Stage4Axis::Former,
            "application_order=3_then_2",
            pi_32,
            sigma_32,
            comparison_for(comparisons, &pi_32.candidate_hash, &sigma_32.candidate_hash)?,
        ),
    ];
    let uc1a_order_axis_instantiates_invariance = stage4_axis_audits
        .iter()
        .filter(|audit| audit.axis == Stage4Axis::Order)
        .all(|audit| audit.cardinality_theorem_instantiated);
    let uc1b_former_axis_instantiates_invariance = stage4_axis_audits
        .iter()
        .filter(|audit| audit.axis == Stage4Axis::Former)
        .all(|audit| audit.cardinality_theorem_instantiated);
    let order_axis_has_conditional_cardinality_obstruction = stage4_axis_audits
        .iter()
        .filter(|audit| audit.axis == Stage4Axis::Order)
        .all(|audit| audit.conditional_obstruction_if_equivalence_requires_family_bijection);
    let former_axis_semantic_nu_invisible = stage4_axis_audits
        .iter()
        .filter(|audit| audit.axis == Stage4Axis::Former)
        .all(|audit| audit.semantic_nu_equal);
    let f_uc4_assessment_authorized =
        uc1a_order_axis_instantiates_invariance || uc1b_former_axis_instantiates_invariance;
    let named_construction_gaps = vec![
        UC1_CROSS_PACKAGE_FAMILY_BIJECTION_API_GAP.to_owned(),
        UC1A_ORDER_TRANSPORT_GAP.to_owned(),
        UC1B_FORMER_TRANSPORT_GAP.to_owned(),
    ];
    let outcome =
        "generic_invariance_proved_exported_surfaces_covered_stage4_axis_instantiation_blocked"
            .to_owned();
    let mut certificate = SemanticNuInvarianceV1Certificate {
        schema: SEMANTIC_NU_INVARIANCE_V1_SCHEMA.to_owned(),
        date: SEMANTIC_NU_INVARIANCE_V1_DATE.to_owned(),
        theorem_id: SEMANTIC_NU_INVARIANCE_V1_THEOREM_ID.to_owned(),
        source_bindings: source_bindings(),
        stage4_option_a_and_f_uc4_markers_replayed,
        bi1b_invariance_gate_marker_replayed,
        uc1_hypothesis_markers_replayed,
        archived_stage4_v3_digest_valid,
        archived_naturality_transport_digest_valid,
        frozen_equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
        stage4_packages,
        exact_four_current_b1_b2_b3_packages,
        exact_stage4_semantic_vector,
        a3_naturality_family_bijection,
        adopted_surface_audits,
        every_exported_semantic_family_bijection_covered,
        stage4_axis_audits,
        uc1a_order_axis_instantiates_invariance,
        uc1b_former_axis_instantiates_invariance,
        order_axis_has_conditional_cardinality_obstruction,
        former_axis_semantic_nu_invisible,
        f_uc4_assessment_authorized,
        named_construction_gaps,
        outcome,
        permitted_conclusion: "T-NU-EQ1 proves that semantic-family nu is invariant under every concrete finite family-class bijection accepted by its two-inverse-law verifier. Every semantic-family bijection actually exported on the audited adopted surfaces is covered: the A3 direct/pointwise eight-family identity and each Stage-4 package's reflexive identity. The two order pairs have a conditional 3-versus-2 cardinality obstruction if a future UC-1a act equivalence is required to transport credited families bijectively; the former pairs are nu-invisible at 3=3 and 2=2.".to_owned(),
        forbidden_conclusion: "This certificate does not prove or refute UC-1a or UC-1b, does not fire F-UC4, and does not score UC-1. No adopted API currently constructs a typed Stage-4 parameter swap or Pi/Sigma library-act equivalence together with functorial source-first extraction and inverse maps on credited family classes.".to_owned(),
        uc1_scored: false,
        bi2_outcome_read: false,
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_hash(&certificate);
    Ok(certificate)
}

pub fn replay_semantic_nu_invariance_v1(
    claimed: &SemanticNuInvarianceV1Certificate,
) -> SemanticNuInvarianceV1Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_hash(claimed) {
        errors.push("semantic-nu invariance certificate digest mismatch".to_owned());
    }
    for theorem in std::iter::once(&claimed.a3_naturality_family_bijection).chain(
        claimed
            .stage4_packages
            .iter()
            .map(|package| &package.identity_bijection),
    ) {
        errors.extend(replay_finite_family_bijection_cardinality(theorem));
    }
    if claimed
        .stage4_packages
        .iter()
        .any(|package| package.derivation_hash != package_snapshot_hash(package))
    {
        errors.push("a Stage-4 family package snapshot digest mismatched".to_owned());
    }
    if claimed
        .adopted_surface_audits
        .iter()
        .any(|audit| audit.derivation_hash != surface_hash(audit))
    {
        errors.push("an adopted equivalence surface audit digest mismatched".to_owned());
    }
    if claimed
        .stage4_axis_audits
        .iter()
        .any(|audit| audit.derivation_hash != axis_hash(audit))
    {
        errors.push("a Stage-4 axis audit digest mismatched".to_owned());
    }
    match issue_semantic_nu_invariance_v1() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors
            .push("certificate differs from deterministic current-source reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    SemanticNuInvarianceV1Replay {
        valid: errors.is_empty(),
        generic_bijection_theorem_valid: replay_finite_family_bijection_cardinality(
            &claimed.a3_naturality_family_bijection,
        )
        .is_empty(),
        every_exported_semantic_family_bijection_covered: claimed
            .every_exported_semantic_family_bijection_covered,
        exact_four_current_packages: claimed.exact_four_current_b1_b2_b3_packages,
        f_uc4_assessment_authorized: claimed.f_uc4_assessment_authorized,
        errors,
    }
}

pub fn replay_semantic_nu_invariance_v1_json(json: &str) -> SemanticNuInvarianceV1Replay {
    match serde_json::from_str::<SemanticNuInvarianceV1Certificate>(json) {
        Ok(certificate) => replay_semantic_nu_invariance_v1(&certificate),
        Err(error) => SemanticNuInvarianceV1Replay {
            valid: false,
            generic_bijection_theorem_valid: false,
            every_exported_semantic_family_bijection_covered: false,
            exact_four_current_packages: false,
            f_uc4_assessment_authorized: false,
            errors: vec![format!(
                "semantic-nu invariance certificate failed to deserialize: {error}"
            )],
        },
    }
}

fn report(certificate: &SemanticNuInvarianceV1Certificate) -> String {
    let mut package_rows = String::new();
    for package in &certificate.stage4_packages {
        package_rows.push_str(&format!(
            "| `{}` | {} | {} | {} | `{}` |\n",
            &package.candidate_hash[7..15],
            package.former_axis,
            package.application_order_axis,
            package.semantic_nu,
            &package.current_package_derivation_hash[7..19],
        ));
    }
    let mut axis_rows = String::new();
    for audit in &certificate.stage4_axis_audits {
        axis_rows.push_str(&format!(
            "| `{:?}` | `{}` | `{}` / `{}` | {} / {} | {} | `{}` |\n",
            audit.axis,
            audit.fixed_coordinate,
            &audit.left_candidate_hash[7..15],
            &audit.right_candidate_hash[7..15],
            audit.left_semantic_nu,
            audit.right_semantic_nu,
            audit.cardinality_theorem_instantiated,
            audit.gap_id,
        ));
    }
    format!(
        "# Semantic-family count invariance v1\n\n**Date:** {}. **Outcome:** `{}`. **Certificate:** `{}`.\n\nThe finite theorem passed: every duplicate-free finite credited-family carrier equipped with total forward/backward maps satisfying both inverse laws has equal cardinality, hence equal semantic `nu`. The audited A3 direct/pointwise eight-family transport instantiates it, and all four Stage-4 packages instantiate reflexivity. Every semantic-family bijection actually exported by the audited adopted surfaces is covered: **{}**.\n\n## Exact current Stage-4 package proofs\n\n| candidate | former | order | semantic `nu` | package proof |\n|---|---|---|---:|---|\n{}\nAll four source-first B1/B2/B3 packages reissued with zero named or silent residue and reproduced their archived proof hashes/counts exactly: **{}**. The candidate-sorted vector is `{:?}`.\n\n## UC-1 axis instantiation audit\n\n| axis | fixed coordinate | candidates | `nu` | theorem instantiated | disposition |\n|---|---|---|---|---|---|\n{}\nThe order pairs expose a conditional cardinality obstruction (3 versus 2) if the intended exchange equivalence is required to carry credited semantic families bijectively: **{}**. The former pairs are `nu`-invisible (3=3 and 2=2): **{}**. Neither axis currently exports the typed act equivalence plus extractor-functoriality and inverse family maps needed to instantiate the theorem, so F-UC4 assessment is authorized: **{}**.\n\nNamed gaps: `{}`; `{}`; `{}`.\n\nPermitted conclusion: {}\n\nForbidden conclusion: {}\n",
        certificate.date,
        certificate.outcome,
        certificate.result_digest,
        certificate.every_exported_semantic_family_bijection_covered,
        package_rows,
        certificate.exact_four_current_b1_b2_b3_packages,
        certificate.exact_stage4_semantic_vector,
        axis_rows,
        certificate.order_axis_has_conditional_cardinality_obstruction,
        certificate.former_axis_semantic_nu_invisible,
        certificate.f_uc4_assessment_authorized,
        UC1_CROSS_PACKAGE_FAMILY_BIJECTION_API_GAP,
        UC1A_ORDER_TRANSPORT_GAP,
        UC1B_FORMER_TRANSPORT_GAP,
        certificate.permitted_conclusion,
        certificate.forbidden_conclusion,
    )
}

pub fn emit_semantic_nu_invariance_v1_create_new(
    directory: &Path,
) -> Result<SemanticNuInvarianceV1Replay, SemanticNuInvarianceV1Error> {
    let certificate = issue_semantic_nu_invariance_v1()?;
    let certificate_path = directory.join(SEMANTIC_NU_INVARIANCE_V1_CERTIFICATE_NAME);
    let report_path = directory.join(SEMANTIC_NU_INVARIANCE_V1_REPORT_NAME);
    let certificate_bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| SemanticNuInvarianceV1Error::Json(error.to_string()))?;
    let report_bytes = report(&certificate).into_bytes();
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| SemanticNuInvarianceV1Error::Io(error.to_string()))?;
    certificate_file
        .write_all(&certificate_bytes)
        .and_then(|_| certificate_file.write_all(b"\n"))
        .map_err(|error| SemanticNuInvarianceV1Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| SemanticNuInvarianceV1Error::Io(error.to_string()))?;
    report_file
        .write_all(&report_bytes)
        .map_err(|error| SemanticNuInvarianceV1Error::Io(error.to_string()))?;
    let replay = replay_semantic_nu_invariance_v1(&certificate);
    if !replay.valid {
        return Err(SemanticNuInvarianceV1Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_permutation_preserves_cardinality() {
        let left = vec!["a".to_owned(), "b".to_owned()];
        let right = vec!["x".to_owned(), "y".to_owned()];
        let theorem = prove_finite_family_bijection_cardinality(
            "swap",
            "left",
            "right",
            left,
            right,
            vec![map_row("a", "y"), map_row("b", "x")],
            vec![map_row("x", "b"), map_row("y", "a")],
        )
        .expect("finite permutation");
        assert!(theorem.semantic_nu_invariant);
        assert!(replay_finite_family_bijection_cardinality(&theorem).is_empty());
    }

    #[test]
    fn unequal_cardinality_cannot_fake_inverse_laws() {
        let result = prove_finite_family_bijection_cardinality(
            "not-a-bijection",
            "left",
            "right",
            vec!["a".to_owned(), "b".to_owned()],
            vec!["x".to_owned()],
            vec![map_row("a", "x"), map_row("b", "x")],
            vec![map_row("x", "a")],
        );
        assert!(result.is_err());
    }

    #[test]
    fn map_row_mutation_invalidates_replay() {
        let mut theorem =
            identity_bijection("identity", "object", &["a".to_owned(), "b".to_owned()])
                .expect("identity");
        theorem.forward[0].target_family_id = "mutated".to_owned();
        assert!(!replay_finite_family_bijection_cardinality(&theorem).is_empty());
    }
}
