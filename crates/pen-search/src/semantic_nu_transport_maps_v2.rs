//! Typed family-class transport maps for the semantic-`nu` invariance theorem.
//!
//! Version 1 checked finite map tables after their family identifiers had
//! already been supplied.  This successor exports the proof-bearing carrier
//! witnesses, the typed map and inverse, and both composition-to-identity
//! computations.  The old finite theorem is then re-run on the maps induced
//! by those typed rows, so equality of cardinalities is never accepted as a
//! substitute for an actual bijection.
//!
//! Scope is intentionally narrow.  The exported equivalence class consists
//! of proof-bearing semantic-family transports carrying a total typed class
//! map, inverse, and coherence.  No Pi/Sigma act equivalence is constructed,
//! no broader library-equivalence functoriality is assumed, and UC-1 is not
//! scored here.

use crate::act_local_semantic_provenance_v5::V5AnchorDisposition;
use crate::naturality_orbit_transport::{
    A3TransportedInstance, NATURALITY_ORBIT_TRANSPORT_SCHEMA, NaturalityOrbitTransportCertificate,
    Stage4SemanticPackage,
};
use crate::semantic_nu_invariance_v1::{
    FamilyMapRow, FiniteFamilyBijectionTheorem, SEMANTIC_NU_INVARIANCE_V1_SCHEMA,
    SemanticNuInvarianceV1Certificate, Stage4Axis, Stage4FamilyPackageSnapshot,
    prove_finite_family_bijection_cardinality, replay_semantic_nu_invariance_v1_json,
};
use crate::t_bi_intrinsic_isolation_v3::issue_replayed_t_bi_intrinsic_isolation_v3_context;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::substitution::{
    ParameterSort, SortedParameterContext, compose_sort_preserving_substitutions,
    identity_sort_preserving_substitution, replay_sort_preserving_substitution,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const SEMANTIC_NU_TRANSPORT_MAPS_V2_SCHEMA: &str = "semantic-nu-transport-maps-v2";
pub const SEMANTIC_NU_TRANSPORT_MAPS_V2_DATE: &str = "2026-07-23";
pub const SEMANTIC_NU_TRANSPORT_MAPS_V2_THEOREM_ID: &str =
    "T-NU-EQ2-typed-family-transport-induces-v1-bijection";
pub const SEMANTIC_NU_TRANSPORT_MAPS_V2_CERTIFICATE_NAME: &str =
    "semantic_nu_transport_maps_v2.json";
pub const SEMANTIC_NU_TRANSPORT_MAPS_V2_REPORT_NAME: &str =
    "SEMANTIC_NU_TRANSPORT_MAPS_V2_RESULT.md";
pub const SEMANTIC_NU_TRANSPORT_SCOPE_BOUNDARY: &str =
    "M1_PROOF_BEARING_TYPED_TRANSPORT_CLASS_ONLY";

const MAINLINE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/mainline_completion_plan.md");
const NU_REGISTER_BYTES: &[u8] = include_bytes!("../../../docs/nu_register_adjudication.md");
const V1_CERTIFICATE_BYTES: &[u8] = include_bytes!("../../../docs/semantic_nu_invariance_v1.json");
const NATURALITY_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/naturality_orbit_transport_v1.json");
const V1_SOURCE_BYTES: &[u8] = include_bytes!("semantic_nu_invariance_v1.rs");
const NATURALITY_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const V5_SOURCE_BYTES: &[u8] = include_bytes!("act_local_semantic_provenance_v5.rs");
const B3_SOURCE_BYTES: &[u8] = include_bytes!("t_bi_intrinsic_isolation_v3.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const SUBSTITUTION_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/substitution.rs");
const TYPED_FAMILIES_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/typed_families.rs");
const NATURALITY_BASIS_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/naturality_basis.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("semantic_nu_transport_maps_v2.rs");

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NumericRegister {
    SemanticFamilyNu,
    ProofInventory,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredNumber {
    pub value: u64,
    pub register: NumericRegister,
    pub meaning: String,
}

fn registered(value: usize, register: NumericRegister, meaning: &str) -> RegisteredNumber {
    RegisteredNumber {
        value: u64::try_from(value).expect("proof inventory fits u64"),
        register,
        meaning: meaning.to_owned(),
    }
}

fn registered_u32(value: u32, register: NumericRegister, meaning: &str) -> RegisteredNumber {
    RegisteredNumber {
        value: u64::from(value),
        register,
        meaning: meaning.to_owned(),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransportV2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: RegisteredNumber,
    pub blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyCarrierSurface {
    Stage4CreditedPackage,
    A3DirectNaturalityOrbit,
    A3PointwiseNaturalityOrbit,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedFamilyCarrierWitness {
    pub object_id: String,
    pub family_id: String,
    pub surface: FamilyCarrierSurface,
    pub member_count: RegisteredNumber,
    pub authoritative_proof_hashes: Vec<String>,
    pub substitution_proof_hashes: Vec<String>,
    pub naturality_proof_hashes: Vec<String>,
    pub equality_proof_hashes: Vec<String>,
    pub typed_normalized_natural: bool,
    pub family_class_replayed: bool,
    pub credited_semantic_family: Option<bool>,
    pub derivation_hash: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TypedTransportRule {
    Stage4ReflexiveTypedIdentity,
    A3DirectPointwiseNaturalityClassIdentity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedFamilyTransportWitness {
    pub relation_id: String,
    pub source_object_id: String,
    pub target_object_id: String,
    pub source_family_id: String,
    pub target_family_id: String,
    pub source_carrier_derivation_hash: String,
    pub target_carrier_derivation_hash: String,
    pub rule: TypedTransportRule,
    pub forward_term_or_class_evidence_hash: String,
    pub inverse_term_or_class_evidence_hash: String,
    pub identity_composition_coherence_hash: String,
    pub map_is_on_family_classes_not_instances: bool,
    pub typed: bool,
    pub inverse_typed: bool,
    pub identity_and_composition_coherent: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedFamilyMapRow {
    pub source_family_id: String,
    pub target_family_id: String,
    pub source_carrier_derivation_hash: String,
    pub target_carrier_derivation_hash: String,
    pub transport_witness_derivation_hash: String,
    pub induced_v1_map_row_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedCompositionRow {
    pub start_family_id: String,
    pub middle_family_id: String,
    pub end_family_id: String,
    pub first_typed_row_derivation_hash: String,
    pub second_typed_row_derivation_hash: String,
    pub induced_identity_row_derivation_hash: String,
    pub is_identity: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedFamilyTransportMap {
    pub theorem_id: String,
    pub relation_id: String,
    pub source_object_id: String,
    pub target_object_id: String,
    pub source_carriers: Vec<TypedFamilyCarrierWitness>,
    pub target_carriers: Vec<TypedFamilyCarrierWitness>,
    pub transport_witnesses: Vec<TypedFamilyTransportWitness>,
    pub forward: Vec<TypedFamilyMapRow>,
    pub inverse: Vec<TypedFamilyMapRow>,
    pub forward_then_inverse: Vec<TypedCompositionRow>,
    pub inverse_then_forward: Vec<TypedCompositionRow>,
    pub source_cardinality: RegisteredNumber,
    pub target_cardinality: RegisteredNumber,
    pub source_identity_coherence: bool,
    pub target_identity_coherence: bool,
    pub forward_is_typed_total_function: bool,
    pub inverse_is_typed_total_function: bool,
    pub maps_are_two_sided_inverses: bool,
    pub induced_v1_bijection_derivation_hash: String,
    pub predecessor_v1_bijection_derivation_hash: String,
    pub induced_bijection_exactly_matches_v1: bool,
    pub semantic_nu_invariant: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrderAxisNoTransportObstruction {
    pub fixed_coordinate: String,
    pub left_candidate_hash: String,
    pub right_candidate_hash: String,
    pub left_semantic_nu: RegisteredNumber,
    pub right_semantic_nu: RegisteredNumber,
    pub semantic_nu_distinct: bool,
    pub accepted_equivalence_requires_typed_map_inverse_and_coherence: bool,
    pub no_accepted_typed_family_bijection_exists: bool,
    pub uc1_verdict_issued_here: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticNuTransportMapsV2Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub source_bindings: Vec<TransportV2SourceBinding>,
    pub predecessor_v1_result_digest: String,
    pub predecessor_v1_replay_valid: bool,
    pub predecessor_naturality_result_digest: String,
    pub predecessor_naturality_content_digest_valid: bool,
    pub numeric_register_policy: String,
    pub exported_map_count: RegisteredNumber,
    pub typed_transport_maps: Vec<TypedFamilyTransportMap>,
    pub every_v1_bijection_has_exactly_one_typed_export: bool,
    pub every_induced_bijection_exactly_matches_v1: bool,
    pub every_map_has_typed_inverse_and_identity_composition_coherence: bool,
    pub generic_typed_transport_preserves_semantic_nu: bool,
    pub order_axis_obstruction_count: RegisteredNumber,
    pub order_axis_obstructions: Vec<OrderAxisNoTransportObstruction>,
    pub f_uc4_transport_gate_ready_for_separate_scoring: bool,
    pub distinct_stage4_cross_package_equivalence_constructed_count: RegisteredNumber,
    pub scope_boundary_id: String,
    pub scope_boundary: String,
    pub no_new_equivalence_semantics_assumed: bool,
    pub uc1_scored: bool,
    pub bi2_outcome_read: bool,
    pub desired_verdict_count_bar_hash_or_enumeration_order_used_as_selector: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub forbidden_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticNuTransportMapsV2Replay {
    pub valid: bool,
    pub predecessor_v1_replay_valid: bool,
    pub exported_map_count: usize,
    pub every_induced_bijection_exactly_matches_v1: bool,
    pub every_map_has_typed_inverse_and_coherence: bool,
    pub f_uc4_transport_gate_ready_for_separate_scoring: bool,
    pub uc1_scored: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SemanticNuTransportMapsV2Error {
    #[error("semantic-nu transport v2 prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("semantic-nu transport v2 binding failed: {0}")]
    Binding(String),
    #[error("semantic-nu transport v2 theorem failed: {0}")]
    Theorem(String),
    #[error("semantic-nu transport v2 JSON failed: {0}")]
    Json(String),
    #[error("semantic-nu transport v2 I/O failed: {0}")]
    Io(String),
    #[error("emitted semantic-nu transport v2 replay failed: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(SEMANTIC_NU_TRANSPORT_MAPS_V2_SCHEMA, domain, value))
        .expect("semantic-nu transport v2 evidence serializes");
    bytes_hash(&bytes)
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("external evidence serializes");
    bytes_hash(&bytes)
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> TransportV2SourceBinding {
    TransportV2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: registered(
            bytes.len(),
            NumericRegister::ArtifactMetadata,
            "source artifact byte length; metadata only",
        ),
        blake3: bytes_hash(bytes),
    }
}

fn source_bindings() -> Vec<TransportV2SourceBinding> {
    vec![
        source_binding(
            "docs/mainline_completion_plan.md",
            "frozen_M1_construction_brief",
            MAINLINE_PLAN_BYTES,
        ),
        source_binding(
            "docs/nu_register_adjudication.md",
            "adopted_semantic_family_register_and_F_NR6",
            NU_REGISTER_BYTES,
        ),
        source_binding(
            "docs/semantic_nu_invariance_v1.json",
            "exact_five_predecessor_bijections",
            V1_CERTIFICATE_BYTES,
        ),
        source_binding(
            "docs/naturality_orbit_transport_v1.json",
            "typed_A3_and_stage4_package_evidence",
            NATURALITY_CERTIFICATE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/semantic_nu_invariance_v1.rs",
            "finite_bijection_theorem_and_replay",
            V1_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "typed_A3_transport_and_stage4_presentations",
            NATURALITY_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/act_local_semantic_provenance_v5.rs",
            "proof_bearing_credited_family_rows",
            V5_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/t_bi_intrinsic_isolation_v3.rs",
            "prefix_local_family_package_reissuance",
            B3_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-type/src/equality.rs",
            "frozen_equality_witness_surface",
            EQUALITY_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-type/src/substitution.rs",
            "typed_identity_inverse_and_composition",
            SUBSTITUTION_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-eval/src/typed_families.rs",
            "canonical_family_and_naturality_surface",
            TYPED_FAMILIES_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-eval/src/naturality_basis.rs",
            "adopted_sort_preserving_identity_and_composition_basis",
            NATURALITY_BASIS_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/semantic_nu_transport_maps_v2.rs",
            "typed_transport_map_issuer_and_replayer",
            THIS_SOURCE_BYTES,
        ),
    ]
}

fn v1_map_row(source: &str, target: &str) -> FamilyMapRow {
    let derivation_hash = external_tagged_hash(
        SEMANTIC_NU_INVARIANCE_V1_SCHEMA,
        "family-map-row",
        &(source, target),
    );
    FamilyMapRow {
        source_family_id: source.to_owned(),
        target_family_id: target.to_owned(),
        derivation_hash,
    }
}

fn carrier_hash(witness: &TypedFamilyCarrierWitness) -> String {
    let mut projection = witness.clone();
    projection.derivation_hash.clear();
    tagged_hash("typed-family-carrier-witness", &projection)
}

fn transport_witness_hash(witness: &TypedFamilyTransportWitness) -> String {
    let mut projection = witness.clone();
    projection.derivation_hash.clear();
    tagged_hash("typed-family-transport-witness", &projection)
}

fn typed_row_hash(row: &TypedFamilyMapRow) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("typed-family-map-row", &projection)
}

fn composition_row_hash(row: &TypedCompositionRow) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("typed-composition-row", &projection)
}

fn transport_map_hash(map: &TypedFamilyTransportMap) -> String {
    let mut projection = map.clone();
    projection.derivation_hash.clear();
    tagged_hash("typed-family-transport-map", &projection)
}

fn obstruction_hash(obstruction: &OrderAxisNoTransportObstruction) -> String {
    let mut projection = obstruction.clone();
    projection.derivation_hash.clear();
    tagged_hash("order-axis-no-typed-transport-obstruction", &projection)
}

fn certificate_hash(certificate: &SemanticNuTransportMapsV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn naturality_certificate_digest_valid(certificate: &NaturalityOrbitTransportCertificate) -> bool {
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

fn parse_sort(sort: &str) -> Result<ParameterSort, SemanticNuTransportMapsV2Error> {
    match sort {
        "Type" => Ok(ParameterSort::Type),
        "Opaque" => Ok(ParameterSort::Opaque),
        other => Err(SemanticNuTransportMapsV2Error::Binding(format!(
            "unknown frozen parameter sort {other:?}"
        ))),
    }
}

fn stage4_package<'a>(
    certificate: &'a NaturalityOrbitTransportCertificate,
    candidate_hash: &str,
) -> Result<&'a Stage4SemanticPackage, SemanticNuTransportMapsV2Error> {
    let rows = certificate
        .stage4
        .packages
        .iter()
        .filter(|package| package.candidate_hash == candidate_hash)
        .collect::<Vec<_>>();
    if rows.len() != 1 {
        return Err(SemanticNuTransportMapsV2Error::Binding(format!(
            "naturality certificate has {} Stage-4 packages for {candidate_hash}",
            rows.len()
        )));
    }
    Ok(rows[0])
}

fn anchor_clause(anchor: &V5AnchorDisposition) -> Result<u16, SemanticNuTransportMapsV2Error> {
    match anchor {
        V5AnchorDisposition::CreditedLocalRole { clause, .. } => Ok(*clause),
        other => Err(SemanticNuTransportMapsV2Error::Binding(format!(
            "a credited Stage-4 family lacks a credited-local-role anchor: {other:?}"
        ))),
    }
}

fn stage4_carriers(
    v1: &SemanticNuInvarianceV1Certificate,
    naturality: &NaturalityOrbitTransportCertificate,
) -> Result<BTreeMap<(String, String), TypedFamilyCarrierWitness>, SemanticNuTransportMapsV2Error> {
    let stem = (1..=3_u32)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    let mut carriers = BTreeMap::new();
    for snapshot in &v1.stage4_packages {
        let mut entries = stem.clone();
        entries.push((4, snapshot.telescope.clone()));
        let context = issue_replayed_t_bi_intrinsic_isolation_v3_context(&entries)
            .map_err(|error| SemanticNuTransportMapsV2Error::Binding(error.to_string()))?;
        if !context.proved || !context.replay_errors.is_empty() {
            return Err(SemanticNuTransportMapsV2Error::Binding(format!(
                "Stage-4 package {} failed independent B3 replay: {:?}",
                snapshot.candidate_hash, context.replay_errors
            )));
        }
        let package = context.sequence.packages.last().ok_or_else(|| {
            SemanticNuTransportMapsV2Error::Binding(
                "Stage-4 B3 replay returned no final package".to_owned(),
            )
        })?;
        if package.candidate_hash != snapshot.candidate_hash
            || package.derivation_hash != snapshot.current_package_derivation_hash
        {
            return Err(SemanticNuTransportMapsV2Error::Binding(format!(
                "Stage-4 live package {} differs from v1",
                snapshot.candidate_hash
            )));
        }
        let presentation_package = stage4_package(naturality, &snapshot.candidate_hash)?;
        for family_id in &snapshot.credited_family_ids {
            let rows = package
                .family_rows
                .iter()
                .filter(|row| &row.family_id == family_id)
                .collect::<Vec<_>>();
            if rows.len() != 1 {
                return Err(SemanticNuTransportMapsV2Error::Binding(format!(
                    "Stage-4 package {} has {} authoritative rows for {family_id}",
                    snapshot.candidate_hash,
                    rows.len()
                )));
            }
            let row = rows[0];
            if !row.credited || !row.typed_normalized_natural {
                return Err(SemanticNuTransportMapsV2Error::Theorem(format!(
                    "Stage-4 family {family_id} is not credited typed-normalized-natural"
                )));
            }
            let clause_index = anchor_clause(&row.anchor)?;
            let clause = presentation_package
                .clauses
                .iter()
                .find(|clause| clause.clause_index == clause_index)
                .ok_or_else(|| {
                    SemanticNuTransportMapsV2Error::Binding(format!(
                        "Stage-4 family {family_id} anchor clause is absent"
                    ))
                })?;
            let sorts = clause
                .parameter_sorts
                .iter()
                .map(|sort| parse_sort(sort))
                .collect::<Result<Vec<_>, _>>()?;
            let parameter_context = SortedParameterContext::new(sorts);
            let identity = identity_sort_preserving_substitution(
                parameter_context,
                clause.canonical_normal_form.clone(),
            )
            .map_err(|error| SemanticNuTransportMapsV2Error::Theorem(error.to_string()))?;
            replay_sort_preserving_substitution(&identity)
                .map_err(|error| SemanticNuTransportMapsV2Error::Theorem(error.to_string()))?;
            let composed = compose_sort_preserving_substitutions(
                &identity,
                &identity,
                clause.canonical_normal_form.clone(),
            )
            .map_err(|error| SemanticNuTransportMapsV2Error::Theorem(error.to_string()))?;
            replay_sort_preserving_substitution(&composed)
                .map_err(|error| SemanticNuTransportMapsV2Error::Theorem(error.to_string()))?;
            if identity.result() != &clause.canonical_normal_form
                || composed.result() != &clause.canonical_normal_form
            {
                return Err(SemanticNuTransportMapsV2Error::Theorem(format!(
                    "Stage-4 family {family_id} identity/composition changed its canonical body"
                )));
            }
            let mut authoritative_proof_hashes = vec![
                row.derivation_hash.clone(),
                package.derivation_hash.clone(),
                clause.presentation_derivation_hash.clone(),
                tagged_hash("stage4-family-anchor", &row.anchor),
            ];
            authoritative_proof_hashes.sort();
            let mut substitution_proof_hashes = vec![
                identity.derivation_hash().to_owned(),
                composed.derivation_hash().to_owned(),
            ];
            substitution_proof_hashes.sort();
            substitution_proof_hashes.dedup();
            let mut witness = TypedFamilyCarrierWitness {
                object_id: snapshot.candidate_hash.clone(),
                family_id: family_id.clone(),
                surface: FamilyCarrierSurface::Stage4CreditedPackage,
                member_count: registered(
                    1,
                    NumericRegister::ProofInventory,
                    "authoritative credited-family carrier member",
                ),
                authoritative_proof_hashes,
                substitution_proof_hashes,
                naturality_proof_hashes: vec![row.derivation_hash.clone()],
                equality_proof_hashes: Vec::new(),
                typed_normalized_natural: row.typed_normalized_natural,
                family_class_replayed: true,
                credited_semantic_family: Some(row.credited),
                derivation_hash: String::new(),
            };
            witness.derivation_hash = carrier_hash(&witness);
            if carriers
                .insert(
                    (snapshot.candidate_hash.clone(), family_id.clone()),
                    witness,
                )
                .is_some()
            {
                return Err(SemanticNuTransportMapsV2Error::Binding(format!(
                    "duplicate Stage-4 carrier witness for {family_id}"
                )));
            }
        }
    }
    Ok(carriers)
}

fn equality_evidence_hash(instance: &A3TransportedInstance) -> String {
    tagged_hash(
        "a3-normalization-equality-evidence",
        &instance.normalization_equality,
    )
}

fn a3_carrier(
    object_id: &str,
    family_id: &str,
    surface: FamilyCarrierSurface,
    instances: &[&A3TransportedInstance],
    whole_a3_derivation_hash: &str,
) -> Result<TypedFamilyCarrierWitness, SemanticNuTransportMapsV2Error> {
    if instances.is_empty()
        || instances
            .iter()
            .any(|instance| instance.family_id != family_id)
    {
        return Err(SemanticNuTransportMapsV2Error::Binding(format!(
            "A3 carrier {family_id} is empty or contains a foreign family"
        )));
    }
    let typed_normalized_natural = instances.iter().all(|instance| {
        instance.normalization_equality.equal
            && instance.uniform_specialization_not_new_family
            && !instance.required_output_kernel_type_json.is_empty()
            && !instance.structural_substitution_hash.is_empty()
            && !instance.typed_image_naturality_hash.is_empty()
            && !instance.normalization_naturality_hash.is_empty()
            && !instance.d_membership_derivation_hash.is_empty()
    });
    if !typed_normalized_natural {
        return Err(SemanticNuTransportMapsV2Error::Theorem(format!(
            "A3 carrier {family_id} lacks typed normalization/naturality evidence"
        )));
    }
    let mut authoritative_proof_hashes = instances
        .iter()
        .map(|instance| instance.derivation_hash.clone())
        .collect::<Vec<_>>();
    authoritative_proof_hashes.push(whole_a3_derivation_hash.to_owned());
    authoritative_proof_hashes.sort();
    let mut substitution_proof_hashes = instances
        .iter()
        .map(|instance| instance.structural_substitution_hash.clone())
        .collect::<Vec<_>>();
    substitution_proof_hashes.sort();
    let mut naturality_proof_hashes = instances
        .iter()
        .flat_map(|instance| {
            [
                instance.typed_image_naturality_hash.clone(),
                instance.normalization_naturality_hash.clone(),
                instance.d_membership_derivation_hash.clone(),
            ]
        })
        .collect::<Vec<_>>();
    naturality_proof_hashes.sort();
    let mut equality_proof_hashes = instances
        .iter()
        .map(|instance| equality_evidence_hash(instance))
        .collect::<Vec<_>>();
    equality_proof_hashes.sort();
    let mut witness = TypedFamilyCarrierWitness {
        object_id: object_id.to_owned(),
        family_id: family_id.to_owned(),
        surface,
        member_count: registered(
            instances.len(),
            NumericRegister::ProofInventory,
            "typed A3 instances carried by this natural-family class",
        ),
        authoritative_proof_hashes,
        substitution_proof_hashes,
        naturality_proof_hashes,
        equality_proof_hashes,
        typed_normalized_natural,
        family_class_replayed: true,
        credited_semantic_family: None,
        derivation_hash: String::new(),
    };
    witness.derivation_hash = carrier_hash(&witness);
    Ok(witness)
}

fn a3_carriers(
    theorem: &FiniteFamilyBijectionTheorem,
    naturality: &NaturalityOrbitTransportCertificate,
) -> Result<BTreeMap<(String, String), TypedFamilyCarrierWitness>, SemanticNuTransportMapsV2Error> {
    if !naturality.a3.j3_regression_64_to_8
        || !naturality.a3.pointwise_instances_join_existing_j3_families
        || !naturality.a3.chronological_instance_transport_bijection
    {
        return Err(SemanticNuTransportMapsV2Error::Prerequisite(
            "the archived A3 typed family join is not certified".to_owned(),
        ));
    }
    let mut carriers = BTreeMap::new();
    for family_id in &theorem.source_family_ids {
        let direct = naturality
            .a3
            .direct_instances
            .iter()
            .filter(|instance| &instance.family_id == family_id)
            .collect::<Vec<_>>();
        let witness = a3_carrier(
            &theorem.source_object_id,
            family_id,
            FamilyCarrierSurface::A3DirectNaturalityOrbit,
            &direct,
            &naturality.a3.derivation_hash,
        )?;
        carriers.insert(
            (theorem.source_object_id.clone(), family_id.clone()),
            witness,
        );
    }
    for family_id in &theorem.target_family_ids {
        let pointwise = naturality
            .a3
            .pointwise_instances
            .iter()
            .filter(|instance| &instance.family_id == family_id)
            .collect::<Vec<_>>();
        let witness = a3_carrier(
            &theorem.target_object_id,
            family_id,
            FamilyCarrierSurface::A3PointwiseNaturalityOrbit,
            &pointwise,
            &naturality.a3.derivation_hash,
        )?;
        carriers.insert(
            (theorem.target_object_id.clone(), family_id.clone()),
            witness,
        );
    }
    Ok(carriers)
}

fn make_transport_witness(
    relation_id: &str,
    source: &TypedFamilyCarrierWitness,
    target: &TypedFamilyCarrierWitness,
    rule: TypedTransportRule,
) -> Result<TypedFamilyTransportWitness, SemanticNuTransportMapsV2Error> {
    if source.family_id != target.family_id {
        return Err(SemanticNuTransportMapsV2Error::Theorem(format!(
            "the adopted typed transport surfaces do not identify {} with {}",
            source.family_id, target.family_id
        )));
    }
    let typed = source.typed_normalized_natural && target.typed_normalized_natural;
    let inverse_typed = typed;
    let forward_term_or_class_evidence_hash = tagged_hash(
        "typed-forward-transport",
        &(
            relation_id,
            &source.object_id,
            &target.object_id,
            &source.derivation_hash,
            &target.derivation_hash,
            rule,
        ),
    );
    let inverse_term_or_class_evidence_hash = tagged_hash(
        "typed-inverse-transport",
        &(
            relation_id,
            &target.object_id,
            &source.object_id,
            &target.derivation_hash,
            &source.derivation_hash,
            rule,
        ),
    );
    let identity_composition_coherence_hash = tagged_hash(
        "typed-transport-identity-composition",
        &(
            &forward_term_or_class_evidence_hash,
            &inverse_term_or_class_evidence_hash,
            &source.family_id,
            &target.family_id,
        ),
    );
    let mut witness = TypedFamilyTransportWitness {
        relation_id: relation_id.to_owned(),
        source_object_id: source.object_id.clone(),
        target_object_id: target.object_id.clone(),
        source_family_id: source.family_id.clone(),
        target_family_id: target.family_id.clone(),
        source_carrier_derivation_hash: source.derivation_hash.clone(),
        target_carrier_derivation_hash: target.derivation_hash.clone(),
        rule,
        forward_term_or_class_evidence_hash,
        inverse_term_or_class_evidence_hash,
        identity_composition_coherence_hash,
        map_is_on_family_classes_not_instances: true,
        typed,
        inverse_typed,
        identity_and_composition_coherent: source.family_id == target.family_id,
        derivation_hash: String::new(),
    };
    witness.derivation_hash = transport_witness_hash(&witness);
    Ok(witness)
}

fn make_typed_row(
    source: &TypedFamilyCarrierWitness,
    target: &TypedFamilyCarrierWitness,
    transport: &TypedFamilyTransportWitness,
) -> TypedFamilyMapRow {
    let mut row = TypedFamilyMapRow {
        source_family_id: source.family_id.clone(),
        target_family_id: target.family_id.clone(),
        source_carrier_derivation_hash: source.derivation_hash.clone(),
        target_carrier_derivation_hash: target.derivation_hash.clone(),
        transport_witness_derivation_hash: transport.derivation_hash.clone(),
        induced_v1_map_row_derivation_hash: v1_map_row(&source.family_id, &target.family_id)
            .derivation_hash,
        derivation_hash: String::new(),
    };
    row.derivation_hash = typed_row_hash(&row);
    row
}

fn composition_rows(
    first: &[TypedFamilyMapRow],
    second: &[TypedFamilyMapRow],
) -> Result<Vec<TypedCompositionRow>, SemanticNuTransportMapsV2Error> {
    let second_by_source = second
        .iter()
        .map(|row| (row.source_family_id.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    if second_by_source.len() != second.len() {
        return Err(SemanticNuTransportMapsV2Error::Theorem(
            "typed inverse is not functional".to_owned(),
        ));
    }
    let mut rows = Vec::new();
    for row in first {
        let inverse = second_by_source
            .get(row.target_family_id.as_str())
            .ok_or_else(|| {
                SemanticNuTransportMapsV2Error::Theorem(format!(
                    "typed composition has no second leg from {}",
                    row.target_family_id
                ))
            })?;
        let identity = v1_map_row(&row.source_family_id, &inverse.target_family_id);
        let mut composition = TypedCompositionRow {
            start_family_id: row.source_family_id.clone(),
            middle_family_id: row.target_family_id.clone(),
            end_family_id: inverse.target_family_id.clone(),
            first_typed_row_derivation_hash: row.derivation_hash.clone(),
            second_typed_row_derivation_hash: inverse.derivation_hash.clone(),
            induced_identity_row_derivation_hash: identity.derivation_hash,
            is_identity: row.source_family_id == inverse.target_family_id,
            derivation_hash: String::new(),
        };
        composition.derivation_hash = composition_row_hash(&composition);
        rows.push(composition);
    }
    rows.sort_by(|left, right| left.start_family_id.cmp(&right.start_family_id));
    Ok(rows)
}

fn build_typed_map(
    predecessor: &FiniteFamilyBijectionTheorem,
    carriers: &BTreeMap<(String, String), TypedFamilyCarrierWitness>,
    rule: TypedTransportRule,
) -> Result<TypedFamilyTransportMap, SemanticNuTransportMapsV2Error> {
    let source_carriers = predecessor
        .source_family_ids
        .iter()
        .map(|family_id| {
            carriers
                .get(&(predecessor.source_object_id.clone(), family_id.clone()))
                .cloned()
                .ok_or_else(|| {
                    SemanticNuTransportMapsV2Error::Binding(format!(
                        "missing source carrier {} / {family_id}",
                        predecessor.source_object_id
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let target_carriers = predecessor
        .target_family_ids
        .iter()
        .map(|family_id| {
            carriers
                .get(&(predecessor.target_object_id.clone(), family_id.clone()))
                .cloned()
                .ok_or_else(|| {
                    SemanticNuTransportMapsV2Error::Binding(format!(
                        "missing target carrier {} / {family_id}",
                        predecessor.target_object_id
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let source_by_id = source_carriers
        .iter()
        .map(|carrier| (carrier.family_id.as_str(), carrier))
        .collect::<BTreeMap<_, _>>();
    let target_by_id = target_carriers
        .iter()
        .map(|carrier| (carrier.family_id.as_str(), carrier))
        .collect::<BTreeMap<_, _>>();
    let mut transport_witnesses = Vec::new();
    let mut forward = Vec::new();
    for row in &predecessor.forward {
        let source = source_by_id
            .get(row.source_family_id.as_str())
            .ok_or_else(|| {
                SemanticNuTransportMapsV2Error::Binding(
                    "predecessor forward row has no source carrier".to_owned(),
                )
            })?;
        let target = target_by_id
            .get(row.target_family_id.as_str())
            .ok_or_else(|| {
                SemanticNuTransportMapsV2Error::Binding(
                    "predecessor forward row has no target carrier".to_owned(),
                )
            })?;
        let witness = make_transport_witness(&predecessor.relation_id, source, target, rule)?;
        let typed_row = make_typed_row(source, target, &witness);
        if typed_row.induced_v1_map_row_derivation_hash != row.derivation_hash {
            return Err(SemanticNuTransportMapsV2Error::Theorem(
                "typed forward row does not induce the predecessor v1 row".to_owned(),
            ));
        }
        transport_witnesses.push(witness);
        forward.push(typed_row);
    }
    let mut inverse = Vec::new();
    for row in &predecessor.backward {
        let source = target_by_id
            .get(row.source_family_id.as_str())
            .ok_or_else(|| {
                SemanticNuTransportMapsV2Error::Binding(
                    "predecessor inverse row has no source carrier".to_owned(),
                )
            })?;
        let target = source_by_id
            .get(row.target_family_id.as_str())
            .ok_or_else(|| {
                SemanticNuTransportMapsV2Error::Binding(
                    "predecessor inverse row has no target carrier".to_owned(),
                )
            })?;
        let witness = make_transport_witness(&predecessor.relation_id, source, target, rule)?;
        let typed_row = make_typed_row(source, target, &witness);
        if typed_row.induced_v1_map_row_derivation_hash != row.derivation_hash {
            return Err(SemanticNuTransportMapsV2Error::Theorem(
                "typed inverse row does not induce the predecessor v1 row".to_owned(),
            ));
        }
        transport_witnesses.push(witness);
        inverse.push(typed_row);
    }
    forward.sort_by(|left, right| {
        (&left.source_family_id, &left.target_family_id)
            .cmp(&(&right.source_family_id, &right.target_family_id))
    });
    inverse.sort_by(|left, right| {
        (&left.source_family_id, &left.target_family_id)
            .cmp(&(&right.source_family_id, &right.target_family_id))
    });
    transport_witnesses.sort_by(|left, right| left.derivation_hash.cmp(&right.derivation_hash));
    transport_witnesses.dedup_by(|left, right| left.derivation_hash == right.derivation_hash);

    let forward_then_inverse = composition_rows(&forward, &inverse)?;
    let inverse_then_forward = composition_rows(&inverse, &forward)?;
    let source_identity_coherence = forward_then_inverse.iter().all(|row| row.is_identity)
        && forward_then_inverse.len() == source_carriers.len();
    let target_identity_coherence = inverse_then_forward.iter().all(|row| row.is_identity)
        && inverse_then_forward.len() == target_carriers.len();
    let forward_is_typed_total_function = forward.len() == source_carriers.len()
        && forward
            .iter()
            .all(|row| !row.transport_witness_derivation_hash.is_empty());
    let inverse_is_typed_total_function = inverse.len() == target_carriers.len()
        && inverse
            .iter()
            .all(|row| !row.transport_witness_derivation_hash.is_empty());
    let maps_are_two_sided_inverses = source_identity_coherence && target_identity_coherence;
    let induced_forward = forward
        .iter()
        .map(|row| v1_map_row(&row.source_family_id, &row.target_family_id))
        .collect::<Vec<_>>();
    let induced_inverse = inverse
        .iter()
        .map(|row| v1_map_row(&row.source_family_id, &row.target_family_id))
        .collect::<Vec<_>>();
    let induced = prove_finite_family_bijection_cardinality(
        &predecessor.relation_id,
        &predecessor.source_object_id,
        &predecessor.target_object_id,
        source_carriers
            .iter()
            .map(|carrier| carrier.family_id.clone())
            .collect(),
        target_carriers
            .iter()
            .map(|carrier| carrier.family_id.clone())
            .collect(),
        induced_forward,
        induced_inverse,
    )
    .map_err(|error| SemanticNuTransportMapsV2Error::Theorem(error.to_string()))?;
    let induced_bijection_exactly_matches_v1 = induced == *predecessor;
    if !induced_bijection_exactly_matches_v1 {
        return Err(SemanticNuTransportMapsV2Error::Theorem(format!(
            "typed transport {} induces a different v1 theorem",
            predecessor.relation_id
        )));
    }
    let mut map = TypedFamilyTransportMap {
        theorem_id: SEMANTIC_NU_TRANSPORT_MAPS_V2_THEOREM_ID.to_owned(),
        relation_id: predecessor.relation_id.clone(),
        source_object_id: predecessor.source_object_id.clone(),
        target_object_id: predecessor.target_object_id.clone(),
        source_carriers,
        target_carriers,
        transport_witnesses,
        forward,
        inverse,
        forward_then_inverse,
        inverse_then_forward,
        source_cardinality: registered(
            predecessor.source_cardinality,
            NumericRegister::SemanticFamilyNu,
            "source credited natural-family cardinality",
        ),
        target_cardinality: registered(
            predecessor.target_cardinality,
            NumericRegister::SemanticFamilyNu,
            "target credited natural-family cardinality",
        ),
        source_identity_coherence,
        target_identity_coherence,
        forward_is_typed_total_function,
        inverse_is_typed_total_function,
        maps_are_two_sided_inverses,
        induced_v1_bijection_derivation_hash: induced.derivation_hash,
        predecessor_v1_bijection_derivation_hash: predecessor.derivation_hash.clone(),
        induced_bijection_exactly_matches_v1,
        semantic_nu_invariant: predecessor.semantic_nu_invariant,
        derivation_hash: String::new(),
    };
    map.derivation_hash = transport_map_hash(&map);
    Ok(map)
}

fn v1_bijections(v1: &SemanticNuInvarianceV1Certificate) -> Vec<FiniteFamilyBijectionTheorem> {
    let mut theorems = vec![v1.a3_naturality_family_bijection.clone()];
    theorems.extend(
        v1.stage4_packages
            .iter()
            .map(|package| package.identity_bijection.clone()),
    );
    theorems
}

fn package_snapshot<'a>(
    packages: &'a [Stage4FamilyPackageSnapshot],
    candidate_hash: &str,
) -> Result<&'a Stage4FamilyPackageSnapshot, SemanticNuTransportMapsV2Error> {
    let rows = packages
        .iter()
        .filter(|package| package.candidate_hash == candidate_hash)
        .collect::<Vec<_>>();
    if rows.len() != 1 {
        return Err(SemanticNuTransportMapsV2Error::Binding(format!(
            "v1 has {} family package snapshots for {candidate_hash}",
            rows.len()
        )));
    }
    Ok(rows[0])
}

fn order_obstructions(
    v1: &SemanticNuInvarianceV1Certificate,
) -> Result<Vec<OrderAxisNoTransportObstruction>, SemanticNuTransportMapsV2Error> {
    let mut rows = Vec::new();
    for audit in v1
        .stage4_axis_audits
        .iter()
        .filter(|audit| audit.axis == Stage4Axis::Order)
    {
        let left = package_snapshot(&v1.stage4_packages, &audit.left_candidate_hash)?;
        let right = package_snapshot(&v1.stage4_packages, &audit.right_candidate_hash)?;
        let semantic_nu_distinct = left.semantic_nu != right.semantic_nu;
        let no_accepted_typed_family_bijection_exists = semantic_nu_distinct
            && left.credited_family_ids.len() != right.credited_family_ids.len();
        if !no_accepted_typed_family_bijection_exists {
            return Err(SemanticNuTransportMapsV2Error::Theorem(format!(
                "order-axis pair {} / {} did not reproduce its finite obstruction",
                left.candidate_hash, right.candidate_hash
            )));
        }
        let mut obstruction = OrderAxisNoTransportObstruction {
            fixed_coordinate: audit.fixed_coordinate.clone(),
            left_candidate_hash: left.candidate_hash.clone(),
            right_candidate_hash: right.candidate_hash.clone(),
            left_semantic_nu: registered_u32(
                left.semantic_nu,
                NumericRegister::SemanticFamilyNu,
                "left Stage-4 credited semantic-family cardinality",
            ),
            right_semantic_nu: registered_u32(
                right.semantic_nu,
                NumericRegister::SemanticFamilyNu,
                "right Stage-4 credited semantic-family cardinality",
            ),
            semantic_nu_distinct,
            accepted_equivalence_requires_typed_map_inverse_and_coherence: true,
            no_accepted_typed_family_bijection_exists,
            uc1_verdict_issued_here: false,
            derivation_hash: String::new(),
        };
        obstruction.derivation_hash = obstruction_hash(&obstruction);
        rows.push(obstruction);
    }
    rows.sort_by(|left, right| left.fixed_coordinate.cmp(&right.fixed_coordinate));
    Ok(rows)
}

pub fn issue_semantic_nu_transport_maps_v2()
-> Result<SemanticNuTransportMapsV2Certificate, SemanticNuTransportMapsV2Error> {
    let mainline = std::str::from_utf8(MAINLINE_PLAN_BYTES)
        .map_err(|error| SemanticNuTransportMapsV2Error::Prerequisite(error.to_string()))?;
    let register = std::str::from_utf8(NU_REGISTER_BYTES)
        .map_err(|error| SemanticNuTransportMapsV2Error::Prerequisite(error.to_string()))?;
    if !mainline.contains("M-1 — Transport-map export")
        || !mainline.contains("identity/composition coherence")
        || !mainline.contains("No new semantics")
        || !register.contains("nu-register-split-v1")
        || !register.contains("F-NR6")
    {
        return Err(SemanticNuTransportMapsV2Error::Prerequisite(
            "M1 or numeric-register markers failed replay".to_owned(),
        ));
    }

    let predecessor_v1_replay = replay_semantic_nu_invariance_v1_json(
        std::str::from_utf8(V1_CERTIFICATE_BYTES)
            .map_err(|error| SemanticNuTransportMapsV2Error::Json(error.to_string()))?,
    );
    if !predecessor_v1_replay.valid {
        return Err(SemanticNuTransportMapsV2Error::Prerequisite(format!(
            "v1 invariance artifact failed replay: {:?}",
            predecessor_v1_replay.errors
        )));
    }
    let v1: SemanticNuInvarianceV1Certificate = serde_json::from_slice(V1_CERTIFICATE_BYTES)
        .map_err(|error| SemanticNuTransportMapsV2Error::Json(error.to_string()))?;
    let naturality: NaturalityOrbitTransportCertificate =
        serde_json::from_slice(NATURALITY_CERTIFICATE_BYTES)
            .map_err(|error| SemanticNuTransportMapsV2Error::Json(error.to_string()))?;
    let predecessor_naturality_content_digest_valid =
        naturality_certificate_digest_valid(&naturality);
    if !predecessor_naturality_content_digest_valid {
        return Err(SemanticNuTransportMapsV2Error::Prerequisite(
            "naturality transport certificate content digest failed".to_owned(),
        ));
    }

    let mut carriers = stage4_carriers(&v1, &naturality)?;
    let a3 = a3_carriers(&v1.a3_naturality_family_bijection, &naturality)?;
    for (key, witness) in a3 {
        if carriers.insert(key, witness).is_some() {
            return Err(SemanticNuTransportMapsV2Error::Binding(
                "A3 and Stage-4 carrier registries overlap".to_owned(),
            ));
        }
    }
    let predecessor_bijections = v1_bijections(&v1);
    let mut typed_transport_maps = predecessor_bijections
        .iter()
        .map(|theorem| {
            let rule = if theorem.relation_id
                == "adopted-A3-naturality-pointwise-to-direct-family-identity"
            {
                TypedTransportRule::A3DirectPointwiseNaturalityClassIdentity
            } else {
                TypedTransportRule::Stage4ReflexiveTypedIdentity
            };
            build_typed_map(theorem, &carriers, rule)
        })
        .collect::<Result<Vec<_>, _>>()?;
    typed_transport_maps.sort_by(|left, right| {
        (&left.relation_id, &left.source_object_id)
            .cmp(&(&right.relation_id, &right.source_object_id))
    });
    let predecessor_keys = predecessor_bijections
        .iter()
        .map(|theorem| {
            (
                theorem.relation_id.clone(),
                theorem.source_object_id.clone(),
                theorem.target_object_id.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    let exported_keys = typed_transport_maps
        .iter()
        .map(|map| {
            (
                map.relation_id.clone(),
                map.source_object_id.clone(),
                map.target_object_id.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    let every_v1_bijection_has_exactly_one_typed_export = predecessor_keys == exported_keys
        && predecessor_keys.len() == predecessor_bijections.len()
        && exported_keys.len() == typed_transport_maps.len();
    let every_induced_bijection_exactly_matches_v1 = typed_transport_maps
        .iter()
        .all(|map| map.induced_bijection_exactly_matches_v1);
    let every_map_has_typed_inverse_and_identity_composition_coherence =
        typed_transport_maps.iter().all(|map| {
            map.forward_is_typed_total_function
                && map.inverse_is_typed_total_function
                && map.maps_are_two_sided_inverses
                && map.source_identity_coherence
                && map.target_identity_coherence
        });
    let generic_typed_transport_preserves_semantic_nu = !typed_transport_maps.is_empty()
        && typed_transport_maps
            .iter()
            .all(|map| map.semantic_nu_invariant);
    if !every_v1_bijection_has_exactly_one_typed_export
        || !every_induced_bijection_exactly_matches_v1
        || !every_map_has_typed_inverse_and_identity_composition_coherence
        || !generic_typed_transport_preserves_semantic_nu
    {
        return Err(SemanticNuTransportMapsV2Error::Theorem(
            "typed transport export failed exact v1 regression or coherence".to_owned(),
        ));
    }
    let order_axis_obstructions = order_obstructions(&v1)?;
    let f_uc4_transport_gate_ready_for_separate_scoring = order_axis_obstructions.len() == 2
        && order_axis_obstructions
            .iter()
            .all(|row| row.no_accepted_typed_family_bijection_exists)
        && generic_typed_transport_preserves_semantic_nu;
    let mut certificate = SemanticNuTransportMapsV2Certificate {
        schema: SEMANTIC_NU_TRANSPORT_MAPS_V2_SCHEMA.to_owned(),
        date: SEMANTIC_NU_TRANSPORT_MAPS_V2_DATE.to_owned(),
        theorem_id: SEMANTIC_NU_TRANSPORT_MAPS_V2_THEOREM_ID.to_owned(),
        source_bindings: source_bindings(),
        predecessor_v1_result_digest: v1.result_digest.clone(),
        predecessor_v1_replay_valid: predecessor_v1_replay.valid,
        predecessor_naturality_result_digest: naturality.result_digest.clone(),
        predecessor_naturality_content_digest_valid,
        numeric_register_policy: "Every quantitative field introduced by v2 is wrapped in RegisteredNumber. Semantic-family cardinalities use semantic_family_nu; theorem/member/map inventories use proof_inventory; byte lengths use artifact_metadata; clause coordinates use syntax_identifier. Numerals embedded in hashes, dates, syntax, or frozen coordinate names are identifiers, not value claims.".to_owned(),
        exported_map_count: registered(
            typed_transport_maps.len(),
            NumericRegister::ProofInventory,
            "typed family-class transport maps exported",
        ),
        typed_transport_maps,
        every_v1_bijection_has_exactly_one_typed_export,
        every_induced_bijection_exactly_matches_v1,
        every_map_has_typed_inverse_and_identity_composition_coherence,
        generic_typed_transport_preserves_semantic_nu,
        order_axis_obstruction_count: registered(
            order_axis_obstructions.len(),
            NumericRegister::ProofInventory,
            "Stage-4 order-axis finite no-transport obstruction rows",
        ),
        order_axis_obstructions,
        f_uc4_transport_gate_ready_for_separate_scoring,
        distinct_stage4_cross_package_equivalence_constructed_count: registered(
            0,
            NumericRegister::ProofInventory,
            "distinct Stage-4 cross-package act equivalences constructed by M1",
        ),
        scope_boundary_id: SEMANTIC_NU_TRANSPORT_SCOPE_BOUNDARY.to_owned(),
        scope_boundary: "The theorem covers exactly proof-bearing semantic-family equivalences that export a typed total family-class map, a typed inverse, and both identity/composition laws. It does not construct a Stage-4 parameter-swap act equivalence, does not construct a Pi/Sigma library-act equivalence, and does not assert functoriality for an unformalized broader equivalence class.".to_owned(),
        no_new_equivalence_semantics_assumed: true,
        uc1_scored: false,
        bi2_outcome_read: false,
        desired_verdict_count_bar_hash_or_enumeration_order_used_as_selector: false,
        outcome:
            "five_v1_bijections_exported_as_exact_typed_maps_f_uc4_transport_gate_ready"
                .to_owned(),
        permitted_conclusion: "Each of the five v1 bijections is now induced by an exported proof-bearing family-class map with a typed inverse and replayable identity/composition coherence. The v1 cardinality theorem replays from those exact induced maps. Within this adopted proof-bearing transport class semantic-family nu is invariant; the two Stage-4 order pairs have certified 3-versus-2 finite no-bijection obstructions ready for the separate UC-1 scoring artifact.".to_owned(),
        forbidden_conclusion: "M1 does not score or burn any UC-1 clause, does not construct a distinct Stage-4 cross-package equivalence, does not prove Pi/Sigma act equivalence, and does not extend the adopted equivalence class beyond transports carrying the exported map/inverse/coherence data.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_hash(&certificate);
    Ok(certificate)
}

fn validate_transport_map(map: &TypedFamilyTransportMap) -> Vec<String> {
    let mut errors = Vec::new();
    if map.derivation_hash != transport_map_hash(map) {
        errors.push(format!(
            "typed transport map {} digest mismatch",
            map.relation_id
        ));
    }
    if map
        .source_carriers
        .iter()
        .chain(&map.target_carriers)
        .any(|carrier| carrier.derivation_hash != carrier_hash(carrier))
    {
        errors.push(format!(
            "typed transport map {} has a carrier digest mismatch",
            map.relation_id
        ));
    }
    if map
        .transport_witnesses
        .iter()
        .any(|witness| witness.derivation_hash != transport_witness_hash(witness))
    {
        errors.push(format!(
            "typed transport map {} has a transport witness digest mismatch",
            map.relation_id
        ));
    }
    if map.forward.iter().chain(&map.inverse).any(|row| {
        row.derivation_hash != typed_row_hash(row)
            || row.induced_v1_map_row_derivation_hash
                != v1_map_row(&row.source_family_id, &row.target_family_id).derivation_hash
    }) {
        errors.push(format!(
            "typed transport map {} has a typed row mismatch",
            map.relation_id
        ));
    }
    if map
        .forward_then_inverse
        .iter()
        .chain(&map.inverse_then_forward)
        .any(|row| row.derivation_hash != composition_row_hash(row) || !row.is_identity)
    {
        errors.push(format!(
            "typed transport map {} has a composition mismatch",
            map.relation_id
        ));
    }
    if !map.forward_is_typed_total_function
        || !map.inverse_is_typed_total_function
        || !map.maps_are_two_sided_inverses
        || !map.source_identity_coherence
        || !map.target_identity_coherence
        || !map.induced_bijection_exactly_matches_v1
        || !map.semantic_nu_invariant
    {
        errors.push(format!(
            "typed transport map {} cached theorem flags are incomplete",
            map.relation_id
        ));
    }
    errors
}

pub fn replay_semantic_nu_transport_maps_v2(
    claimed: &SemanticNuTransportMapsV2Certificate,
) -> SemanticNuTransportMapsV2Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_hash(claimed) {
        errors.push("semantic-nu transport v2 certificate digest mismatch".to_owned());
    }
    for map in &claimed.typed_transport_maps {
        errors.extend(validate_transport_map(map));
    }
    if claimed.order_axis_obstructions.iter().any(|row| {
        row.derivation_hash != obstruction_hash(row)
            || !row.semantic_nu_distinct
            || !row.no_accepted_typed_family_bijection_exists
            || row.uc1_verdict_issued_here
    }) {
        errors.push("an order-axis no-transport obstruction failed replay".to_owned());
    }
    match issue_semantic_nu_transport_maps_v2() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors
            .push("certificate differs from deterministic current-source reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    SemanticNuTransportMapsV2Replay {
        valid: errors.is_empty(),
        predecessor_v1_replay_valid: claimed.predecessor_v1_replay_valid,
        exported_map_count: claimed.typed_transport_maps.len(),
        every_induced_bijection_exactly_matches_v1: claimed
            .every_induced_bijection_exactly_matches_v1,
        every_map_has_typed_inverse_and_coherence: claimed
            .every_map_has_typed_inverse_and_identity_composition_coherence,
        f_uc4_transport_gate_ready_for_separate_scoring: claimed
            .f_uc4_transport_gate_ready_for_separate_scoring,
        uc1_scored: claimed.uc1_scored,
        errors,
    }
}

pub fn replay_semantic_nu_transport_maps_v2_json(json: &str) -> SemanticNuTransportMapsV2Replay {
    match serde_json::from_str::<SemanticNuTransportMapsV2Certificate>(json) {
        Ok(certificate) => replay_semantic_nu_transport_maps_v2(&certificate),
        Err(error) => SemanticNuTransportMapsV2Replay {
            valid: false,
            predecessor_v1_replay_valid: false,
            exported_map_count: 0,
            every_induced_bijection_exactly_matches_v1: false,
            every_map_has_typed_inverse_and_coherence: false,
            f_uc4_transport_gate_ready_for_separate_scoring: false,
            uc1_scored: false,
            errors: vec![format!(
                "semantic-nu transport v2 certificate failed to deserialize: {error}"
            )],
        },
    }
}

fn report(certificate: &SemanticNuTransportMapsV2Certificate) -> String {
    let mut map_rows = String::new();
    for map in &certificate.typed_transport_maps {
        map_rows.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} |\n",
            map.relation_id,
            map.source_object_id,
            map.target_object_id,
            map.source_cardinality.value,
            map.target_cardinality.value,
            map.induced_bijection_exactly_matches_v1 && map.maps_are_two_sided_inverses,
        ));
    }
    let mut obstruction_rows = String::new();
    for row in &certificate.order_axis_obstructions {
        obstruction_rows.push_str(&format!(
            "| `{}` | `{}` / `{}` | {} / {} | {} |\n",
            row.fixed_coordinate,
            &row.left_candidate_hash[7..15],
            &row.right_candidate_hash[7..15],
            row.left_semantic_nu.value,
            row.right_semantic_nu.value,
            row.no_accepted_typed_family_bijection_exists,
        ));
    }
    format!(
        "# Semantic-nu typed transport maps v2\n\n**Date:** {}. **Outcome:** `{}`. **Certificate:** `{}`.\n\nM1 exports {} `[proof_inventory]` typed family-class maps, one for each v1 bijection. Every map carries its source and target proof witnesses, typed forward map, typed inverse, and both composition-to-identity computations. Reconstructing the v1 map rows from these typed rows reproduces every predecessor theorem exactly: **{}**.\n\n## Exported maps\n\n| relation | source object | target object | source `nu` `[semantic_family_nu]` | target `nu` `[semantic_family_nu]` | exact typed inverse/coherence |\n|---|---|---|---:|---:|---|\n{}\n## Order-axis finite obstructions (not UC-1 scoring)\n\n| fixed coordinate | candidates | semantic-family `nu` | no accepted typed bijection |\n|---|---|---:|---|\n{}\nThe F-UC4 transport gate is ready for the separate M2 scoring artifact: **{}**. M1 itself issues no UC-1 verdict (`uc1_scored = {}`). Distinct Stage-4 cross-package equivalences constructed: {} `[proof_inventory]`.\n\nScope boundary `{}`: {}\n\nPermitted conclusion: {}\n\nForbidden conclusion: {}\n",
        certificate.date,
        certificate.outcome,
        certificate.result_digest,
        certificate.exported_map_count.value,
        certificate.every_induced_bijection_exactly_matches_v1,
        map_rows,
        obstruction_rows,
        certificate.f_uc4_transport_gate_ready_for_separate_scoring,
        certificate.uc1_scored,
        certificate
            .distinct_stage4_cross_package_equivalence_constructed_count
            .value,
        certificate.scope_boundary_id,
        certificate.scope_boundary,
        certificate.permitted_conclusion,
        certificate.forbidden_conclusion,
    )
}

pub fn emit_semantic_nu_transport_maps_v2_create_new(
    directory: &Path,
) -> Result<SemanticNuTransportMapsV2Replay, SemanticNuTransportMapsV2Error> {
    let certificate = issue_semantic_nu_transport_maps_v2()?;
    let certificate_path = directory.join(SEMANTIC_NU_TRANSPORT_MAPS_V2_CERTIFICATE_NAME);
    let report_path = directory.join(SEMANTIC_NU_TRANSPORT_MAPS_V2_REPORT_NAME);
    let certificate_bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| SemanticNuTransportMapsV2Error::Json(error.to_string()))?;
    let report_bytes = report(&certificate).into_bytes();
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| SemanticNuTransportMapsV2Error::Io(error.to_string()))?;
    certificate_file
        .write_all(&certificate_bytes)
        .and_then(|_| certificate_file.write_all(b"\n"))
        .map_err(|error| SemanticNuTransportMapsV2Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| SemanticNuTransportMapsV2Error::Io(error.to_string()))?;
    report_file
        .write_all(&report_bytes)
        .map_err(|error| SemanticNuTransportMapsV2Error::Io(error.to_string()))?;
    let replay = replay_semantic_nu_transport_maps_v2(&certificate);
    if !replay.valid {
        return Err(SemanticNuTransportMapsV2Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_five_typed_exports_replay_without_scoring_uc1() {
        let certificate = issue_semantic_nu_transport_maps_v2().expect("M1 issues");
        let replay = replay_semantic_nu_transport_maps_v2(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(replay.exported_map_count, 5);
        assert!(replay.every_induced_bijection_exactly_matches_v1);
        assert!(replay.every_map_has_typed_inverse_and_coherence);
        assert!(replay.f_uc4_transport_gate_ready_for_separate_scoring);
        assert!(!replay.uc1_scored);
    }

    #[test]
    fn typed_evidence_and_inverse_mutations_fail_replay() {
        let mut carrier_mutation = issue_semantic_nu_transport_maps_v2().expect("M1 issues");
        carrier_mutation.typed_transport_maps[0].source_carriers[0].typed_normalized_natural =
            false;
        assert!(!replay_semantic_nu_transport_maps_v2(&carrier_mutation).valid);

        let mut inverse_mutation = issue_semantic_nu_transport_maps_v2().expect("M1 issues");
        inverse_mutation.typed_transport_maps[0].inverse[0]
            .target_family_id
            .push_str("-mutated");
        assert!(!replay_semantic_nu_transport_maps_v2(&inverse_mutation).valid);
    }

    #[test]
    fn register_or_obstruction_mutation_fails_replay() {
        let mut register_mutation = issue_semantic_nu_transport_maps_v2().expect("M1 issues");
        register_mutation.typed_transport_maps[0]
            .source_cardinality
            .register = NumericRegister::ProofInventory;
        assert!(!replay_semantic_nu_transport_maps_v2(&register_mutation).valid);

        let mut obstruction_mutation = issue_semantic_nu_transport_maps_v2().expect("M1 issues");
        obstruction_mutation.order_axis_obstructions[0].no_accepted_typed_family_bijection_exists =
            false;
        assert!(!replay_semantic_nu_transport_maps_v2(&obstruction_mutation).valid);
    }
}
