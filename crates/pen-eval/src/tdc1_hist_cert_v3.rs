//! HIST-CERT v3: create-new F-B2 regression rerun over the V3 historical
//! typed-bundle handoff.
//!
//! This successor deliberately does not rewrite or reinterpret HIST-CERT v1.
//! It binds the exact v1 bytes, replays its rule-driven operational inventory,
//! and joins each registered beta/Kan family to the exact B4--B7 typed bundle.
//! The join certifies the registered path subtotals as typed and marginal.
//! Full historical totals remain partial because opaque ordinary
//! natural-family tokens have not been issued.

use crate::tdc1_hist_cert::{
    EvidenceLevel, FamilyChannel, FamilyVerdict, HIST_CERT_SCHEMA, HistCertResult,
    HistoricalFamilyKey, build_hist_cert, replay_hist_cert,
};
use crate::trunc_endpoint_realizer::{
    TRUNC_ENDPOINT_ARTIFACT_SCHEMA, replay_trunc_endpoint_realizer_json,
};
use pen_core::hash::blake3_hex;
use pen_core::{expr::Expr, telescope::Telescope};
use pen_type::cubical::boundary_variants::BoundaryBasisKey;
use pen_type::cubical::typed_boundary::{
    ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION, HISTORICAL_PREFIX_V3_C6_TYPED_HANDOFF_VERSION,
    HistoricalPrefixV3C6TypedBundleToken, HistoricalTypedBundleEvidenceKind,
    RegisteredBoundaryKind, issue_historical_v3_c6_typed_handoff_token,
    replay_historical_v3_c6_typed_handoff_token,
};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use pen_type::tdc1::PathSchemaKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use thiserror::Error;

pub const HIST_CERT_V3_SCHEMA: &str = "hist-cert-historical-hit-v3";
pub const HIST_CERT_V3_DATE: &str = "2026-07-19";
pub const HIST_CERT_V3_JOIN_VERSION: &str = "hist-cert-v3-f-b2-typed-bundle-support-local-join-v1";

const EXPECTED_RAW_TOTALS: [u32; 4] = [7, 8, 10, 18];
const EXPECTED_PATH_TOTALS: [u64; 4] = [2, 2, 5, 10];
const EXPECTED_ORDINARY_GAPS: [u32; 4] = [5, 6, 5, 8];
const EXPECTED_PREDECESSOR_PATH_INVENTORIES: [u32; 4] = [0, 2, 4, 9];
const EXPECTED_SUPPORT_COMPARISON_COUNTS: [u32; 4] = [0, 4, 20, 90];

const HIST_CERT_V1_BYTES: &[u8] = include_bytes!("../../../docs/hist_cert_v1.json");
const TRUNC_ENDPOINT_V1_BYTES: &[u8] =
    include_bytes!("../../../docs/trunc_endpoint_realizer_v1.json");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum HistCertV3Error {
    #[error("kernel typed-bundle replay failed: {0}")]
    Kernel(String),
    #[error("HIST-CERT v1 replay failed: {0}")]
    Legacy(String),
    #[error("archive binding failed for {name}: {reason}")]
    Archive { name: String, reason: String },
    #[error("F-B2 join failed at historical step {step}: {reason}")]
    Join { step: u32, reason: String },
    #[error("certificate JSON failed: {0}")]
    Json(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistCertV3Versions {
    pub join: String,
    pub legacy_hist_cert: String,
    pub typed_handoff: String,
    pub boundary_axiom_v3: String,
    pub trunc_endpoint_artifact: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistCertV3ArchiveBinding {
    pub name: String,
    pub path: String,
    pub byte_length: u64,
    pub sha256: String,
    pub internal_digest: String,
    pub exact_bytes_pinned: bool,
    pub definition_replayed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryPathKeyAudit {
    pub boundary_key: String,
    pub path_key: PathSchemaKey,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportComparisonAudit {
    pub predecessor_step: u32,
    pub predecessor_subject_hash: String,
    pub predecessor_owner_support_hash: String,
    pub predecessor_key: PathSchemaKey,
    pub current_subject_hash: String,
    pub current_owner_support_hash: String,
    pub historical_prefix_equality_free: bool,
    pub decision: String,
    pub reason: String,
    pub derivation_hash: String,
    pub distinct_by_fresh_support: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedMarginalPathFamilyAudit {
    pub key: PathSchemaKey,
    pub legacy_operational_inventory_digest: String,
    pub bundle_derivation_hash: String,
    pub current_subject_hash: String,
    pub current_owner_support_hash: String,
    pub predecessor_comparisons: Vec<SupportComparisonAudit>,
    pub predecessor_inventory_empty: bool,
    pub exact_key_correspondence_membership: bool,
    pub source_bound_to_exact_predecessor: bool,
    pub marginality_recomputed_from_fresh_owner_support: bool,
    pub proof_scope: String,
    pub typed: bool,
    pub marginal: bool,
    pub typed_and_marginal: bool,
    pub credit_verdict: FamilyVerdict,
    pub egp_provenance_obstruction: String,
    pub individual_join_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrdinaryNaturalFamilyGap {
    pub key: HistoricalFamilyKey,
    pub judgement: String,
    pub source_clauses: Vec<u16>,
    pub existing_evidence_level: String,
    pub required_opaque_token: String,
    pub natural_family_proved: bool,
    pub full_predecessor_sweep_proved: bool,
    pub status: String,
    pub legacy_family_derivation_hash: String,
    pub gap_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistCertV3PackageAudit {
    pub step: u32,
    pub label: String,
    pub dimension: u32,
    pub predecessor_library: u32,
    pub predecessor_signature_digest: String,
    pub bundle_signature_digest: String,
    pub bundle_kind: String,
    pub bundle_evidence_kind: String,
    pub typed_boundary_derivation_hash: String,
    pub bundle_derivation_hash: String,
    pub child_derivation_hash: String,
    pub bundle_path_realization_derivation_hashes: Vec<String>,
    pub bundle_path_realization_inventory_digest: String,
    pub key_correspondence: Vec<BoundaryPathKeyAudit>,
    pub current_subject_hash: String,
    pub current_owner_support_hash: String,
    pub historical_prefix_equality_free: bool,
    pub predecessor_registered_path_inventory_count: u32,
    pub support_comparison_count: u32,
    pub raw_operational_total: u32,
    pub recorded_total: u32,
    pub expected_registered_path_total: u64,
    pub realized_registered_path_total: u64,
    pub typed_and_marginal_path_subtotal: u32,
    pub exact_operational_inventory_replayed: bool,
    pub exact_registered_path_inventory_replayed: bool,
    pub predecessor_weakening_erasure_inverse_laws_checked: bool,
    pub path_families: Vec<TypedMarginalPathFamilyAudit>,
    pub ordinary_family_gaps: Vec<OrdinaryNaturalFamilyGap>,
    pub ordinary_family_gap_count: u32,
    pub full_certified_total: Option<u32>,
    pub full_total_status: String,
    pub registered_trunc_endpoint_gap_retired: bool,
    pub conditional_on_boundary_axiom_v3: bool,
    pub package_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FB2RegressionAudit {
    pub expected_raw_totals: Vec<u32>,
    pub observed_raw_totals: Vec<u32>,
    pub raw_totals_exact: bool,
    pub expected_typed_key_subtotals: Vec<u64>,
    pub observed_typed_key_subtotals: Vec<u64>,
    pub typed_key_subtotals_exact: bool,
    pub expected_marginal_subtotals: Vec<u64>,
    pub observed_marginal_subtotals: Vec<u64>,
    pub marginal_subtotals_exact: bool,
    pub expected_ordinary_unresolved: Vec<u32>,
    pub observed_ordinary_unresolved: Vec<u32>,
    pub ordinary_unresolved_exact: bool,
    pub expected_predecessor_path_inventories: Vec<u32>,
    pub observed_predecessor_path_inventories: Vec<u32>,
    pub predecessor_path_inventories_exact: bool,
    pub expected_support_comparison_counts: Vec<u32>,
    pub observed_support_comparison_counts: Vec<u32>,
    pub support_comparison_counts_exact: bool,
    pub all_four_exact_predecessors_joined: bool,
    pub all_registered_path_families_typed_and_marginal: bool,
    pub full_certified_totals: Vec<Option<u32>>,
    pub full_totals_partial: bool,
    pub f_b2_regression_signature_replayed: bool,
    pub partial_but_regression_clean: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistCertV3Conditionality {
    pub conditional_on_boundary_axiom_v3: bool,
    pub uses_adopted_boundary_axiom_v3: bool,
    pub registered_historical_packages_only: bool,
    pub trunc_endpoint_judgement_under_formal_method_premises: bool,
    pub trunc_premise_free_closed_term_proved: bool,
    pub registered_path_grammar_exhaustive_for_intended_schemas: bool,
    pub intended_path_basis_independence_proved: bool,
    pub arbitrary_typed_instantiation_proved: bool,
    pub path_family_egp_provenance_complete: bool,
    pub ordinary_natural_family_inventory_complete: bool,
    pub general_c6_proved: bool,
    pub candidate_c8_proved: bool,
    pub intended_schema_classification_proved: bool,
    pub derivable_from_sealed_trace_alone: bool,
    pub full_historical_totals_certified: bool,
    pub f_t1_discharged: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistCertV3Certificate {
    pub schema: String,
    pub date: String,
    pub versions: HistCertV3Versions,
    pub archival_bindings: Vec<HistCertV3ArchiveBinding>,
    pub packages: Vec<HistCertV3PackageAudit>,
    pub f_b2: FB2RegressionAudit,
    pub conditionality: HistCertV3Conditionality,
    pub outcome: String,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistCertV3Replay {
    pub valid: bool,
    pub raw_totals_7_8_10_18: bool,
    pub path_subtotals_2_2_5_10: bool,
    pub all_registered_paths_typed_and_marginal: bool,
    pub full_certified_totals_partial: bool,
    pub f_t1_discharged: bool,
    pub errors: Vec<String>,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(HIST_CERT_V3_SCHEMA, domain, value))
        .expect("HIST-CERT v3 proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:X}", Sha256::digest(bytes))
}

fn artifact_binding(
    name: &str,
    path: &str,
    bytes: &[u8],
    expected_length: u64,
    expected_sha256: &str,
    expected_internal_digest: &str,
    definition_replayed: bool,
) -> Result<HistCertV3ArchiveBinding, HistCertV3Error> {
    let byte_length = bytes.len() as u64;
    let sha256 = sha256_hex(bytes);
    if byte_length != expected_length || sha256 != expected_sha256 {
        return Err(HistCertV3Error::Archive {
            name: name.to_owned(),
            reason: format!(
                "expected {expected_length}/{expected_sha256}, found {byte_length}/{sha256}"
            ),
        });
    }
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|error| HistCertV3Error::Archive {
            name: name.to_owned(),
            reason: error.to_string(),
        })?;
    let internal_digest = value
        .get("result_digest")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| HistCertV3Error::Archive {
            name: name.to_owned(),
            reason: "missing result_digest".to_owned(),
        })?
        .to_owned();
    if internal_digest != expected_internal_digest || !definition_replayed {
        return Err(HistCertV3Error::Archive {
            name: name.to_owned(),
            reason: format!(
                "internal digest/replay mismatch: expected {expected_internal_digest}, found {internal_digest}, replay={definition_replayed}"
            ),
        });
    }
    Ok(HistCertV3ArchiveBinding {
        name: name.to_owned(),
        path: path.to_owned(),
        byte_length,
        sha256,
        internal_digest,
        exact_bytes_pinned: true,
        definition_replayed,
    })
}

fn kind_name(kind: RegisteredBoundaryKind) -> &'static str {
    match kind {
        RegisteredBoundaryKind::S1 => "s1",
        RegisteredBoundaryKind::Trunc => "trunc",
        RegisteredBoundaryKind::S2 => "s2",
        RegisteredBoundaryKind::S3 => "s3",
    }
}

fn evidence_kind_name(kind: HistoricalTypedBundleEvidenceKind) -> &'static str {
    match kind {
        HistoricalTypedBundleEvidenceKind::ArchivalConstantBridge => "archival_constant_bridge",
        HistoricalTypedBundleEvidenceKind::TruncEndpointDependentV1 => {
            "trunc_endpoint_dependent_v1"
        }
    }
}

fn boundary_key_name(key: &BoundaryBasisKey) -> String {
    match key {
        BoundaryBasisKey::Beta => "beta".to_owned(),
        BoundaryBasisKey::PrincipalTransport { principal } => {
            format!("principal_transport({principal})")
        }
        BoundaryBasisKey::TransportNaturality { principal, probe } => {
            format!("transport_naturality({principal},{probe})")
        }
    }
}

fn boundary_key_matches_path(key: &BoundaryBasisKey, path: &PathSchemaKey) -> bool {
    match (key, path) {
        (BoundaryBasisKey::Beta, PathSchemaKey::Beta) => true,
        (
            BoundaryBasisKey::PrincipalTransport { principal },
            PathSchemaKey::Kan {
                principal: path_principal,
                probe,
            },
        ) => principal == path_principal && principal == probe,
        (
            BoundaryBasisKey::TransportNaturality { principal, probe },
            PathSchemaKey::Kan {
                principal: path_principal,
                probe: path_probe,
            },
        ) => principal != probe && principal == path_principal && probe == path_probe,
        _ => false,
    }
}

fn canonical_path_keys(dimension: u32) -> Vec<PathSchemaKey> {
    let mut keys = Vec::with_capacity(1 + (dimension * dimension) as usize);
    keys.push(PathSchemaKey::Beta);
    for principal in 0..dimension {
        for probe in 0..dimension {
            keys.push(PathSchemaKey::Kan { principal, probe });
        }
    }
    keys
}

fn evidence_level_name(level: &EvidenceLevel) -> &'static str {
    match level {
        EvidenceLevel::ShallowClauseTyping => "shallow_clause_typing",
        EvidenceLevel::TypedCubicalRepresentative => "typed_cubical_representative",
        EvidenceLevel::TypedPathToken => "typed_path_token",
        EvidenceLevel::MissingTypedNaturalFamily => "missing_typed_natural_family",
    }
}

fn required_ordinary_token(key: &HistoricalFamilyKey) -> &'static str {
    match key {
        HistoricalFamilyKey::Formation { .. } => "opaque_formation_natural_family_token",
        HistoricalFamilyKey::PointIntroduction { .. } => {
            "opaque_point_or_unit_natural_family_token"
        }
        HistoricalFamilyKey::PathIntroduction { .. } => {
            "opaque_path_constructor_natural_family_token"
        }
        HistoricalFamilyKey::Recursor { .. } => "opaque_recursor_natural_family_token",
        HistoricalFamilyKey::Inductor { .. } => "opaque_inductor_natural_family_token",
        HistoricalFamilyKey::ParametricAction { .. } => "opaque_trunc_action_natural_family_token",
        HistoricalFamilyKey::PostPathForward { .. } | HistoricalFamilyKey::CellAction { .. } => {
            "opaque_post_path_operation_natural_family_token"
        }
        HistoricalFamilyKey::Path { .. } => unreachable!("path keys use the typed-bundle join"),
    }
}

fn join_error(step: u32, reason: impl Into<String>) -> HistCertV3Error {
    HistCertV3Error::Join {
        step,
        reason: reason.into(),
    }
}

fn expression_contains_identity(expression: &Expr) -> bool {
    match expression {
        Expr::Id(_, _, _) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            expression_contains_identity(left) || expression_contains_identity(right)
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => expression_contains_identity(body),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn exact_historical_prefix(step: u32) -> SealedSignature {
    SealedSignature::from_telescopes(
        (1..step)
            .map(|prior| (prior, Telescope::reference(prior)))
            .collect(),
    )
}

fn prefix_is_equality_free(signature: &SealedSignature) -> bool {
    signature.entries().iter().all(|entry| {
        entry
            .telescope
            .clauses
            .iter()
            .all(|clause| !expression_contains_identity(&clause.expr))
    })
}

fn fresh_owner_support_hash(subject_hash: &str) -> String {
    tagged_hash("v3-fresh-owner-support", &subject_hash)
}

fn bundle_correspondence(
    step: u32,
    bundle: &HistoricalPrefixV3C6TypedBundleToken,
) -> Result<(Vec<PathSchemaKey>, Vec<BoundaryPathKeyAudit>), HistCertV3Error> {
    let expected_keys = canonical_path_keys(bundle.dimension());
    let expected_key_set = expected_keys.iter().cloned().collect::<BTreeSet<_>>();
    let correspondence = bundle
        .key_correspondence()
        .iter()
        .map(|entry| {
            if !boundary_key_matches_path(entry.boundary_key(), entry.path_key()) {
                return Err(join_error(
                    step,
                    "boundary/path key correspondence is malformed",
                ));
            }
            Ok(BoundaryPathKeyAudit {
                boundary_key: boundary_key_name(entry.boundary_key()),
                path_key: entry.path_key().clone(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let correspondence_key_set = correspondence
        .iter()
        .map(|entry| entry.path_key.clone())
        .collect::<BTreeSet<_>>();
    let realization_hashes = bundle.path_realization_derivation_hashes();
    if correspondence.len() != expected_keys.len()
        || correspondence_key_set != expected_key_set
        || realization_hashes.len() != expected_keys.len()
        || realization_hashes.iter().collect::<BTreeSet<_>>().len() != expected_keys.len()
        || bundle.expected_basis_count() != expected_keys.len() as u64
        || bundle.realized_basis_count() != expected_keys.len() as u64
    {
        return Err(join_error(
            step,
            "typed bundle does not realize exactly beta plus the ordered d-by-d Kan key set",
        ));
    }
    Ok((expected_keys, correspondence))
}

fn build_package(
    legacy_step: &crate::tdc1_hist_cert::HistoricalStepAudit,
    bundle: &HistoricalPrefixV3C6TypedBundleToken,
    predecessor_bundles: &[HistoricalPrefixV3C6TypedBundleToken],
) -> Result<HistCertV3PackageAudit, HistCertV3Error> {
    let step = legacy_step.step;
    let prefix = exact_historical_prefix(step);
    let historical_prefix_equality_free = prefix_is_equality_free(&prefix);
    if bundle.step() != step
        || bundle.dimension() != legacy_step.dimension
        || prefix.digest() != legacy_step.predecessor.signature_digest
        || bundle.signature_digest() != legacy_step.predecessor.signature_digest
        || !historical_prefix_equality_free
    {
        return Err(join_error(
            step,
            "typed bundle is not source-bound to the exact equality-free historical predecessor/package",
        ));
    }

    let (expected_keys, correspondence) = bundle_correspondence(step, bundle)?;
    let expected_key_set = expected_keys.iter().cloned().collect::<BTreeSet<_>>();
    let correspondence_key_set = correspondence
        .iter()
        .map(|entry| entry.path_key.clone())
        .collect::<BTreeSet<_>>();

    let legacy_paths = legacy_step
        .families
        .iter()
        .filter(|family| family.channel == FamilyChannel::Path)
        .collect::<Vec<_>>();
    let legacy_path_key_set = legacy_paths
        .iter()
        .map(|family| match &family.key {
            HistoricalFamilyKey::Path { key } => Ok(key.clone()),
            _ => Err(join_error(step, "path channel contains a non-path key")),
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if legacy_paths.len() != expected_keys.len() || legacy_path_key_set != expected_key_set {
        return Err(join_error(
            step,
            "legacy path inventory and typed-bundle key inventory differ",
        ));
    }

    let current_subject_hash = candidate_hash(&Telescope::reference(step));
    let current_owner_support_hash = fresh_owner_support_hash(&current_subject_hash);
    let mut predecessor_inventory = Vec::new();
    for predecessor in predecessor_bundles {
        if predecessor.step() >= step {
            return Err(join_error(step, "predecessor bundle ordering is malformed"));
        }
        let predecessor_prefix = exact_historical_prefix(predecessor.step());
        if predecessor.signature_digest() != predecessor_prefix.digest()
            || !prefix_is_equality_free(&predecessor_prefix)
        {
            return Err(join_error(
                step,
                "predecessor path bundle is not bound to its exact equality-free prefix",
            ));
        }
        let (predecessor_keys, _) = bundle_correspondence(step, predecessor)?;
        let predecessor_subject_hash = candidate_hash(&Telescope::reference(predecessor.step()));
        let predecessor_owner_support_hash = fresh_owner_support_hash(&predecessor_subject_hash);
        if predecessor_subject_hash == current_subject_hash
            || predecessor_owner_support_hash == current_owner_support_hash
        {
            return Err(join_error(step, "fresh historical owner supports collide"));
        }
        predecessor_inventory.extend(predecessor_keys.into_iter().map(|key| {
            (
                predecessor.step(),
                predecessor_subject_hash.clone(),
                predecessor_owner_support_hash.clone(),
                key,
            )
        }));
    }

    let mut path_families = Vec::with_capacity(expected_keys.len());
    for key in &expected_keys {
        let _legacy_slot = legacy_paths
            .iter()
            .find(|family| {
                matches!(&family.key, HistoricalFamilyKey::Path { key: found } if found == key)
            })
            .ok_or_else(|| join_error(step, "missing legacy path family"))?;
        let exact_key_correspondence_membership = correspondence_key_set.contains(key);
        let comparisons = predecessor_inventory
            .iter()
            .map(
                |(
                    predecessor_step,
                    predecessor_subject_hash,
                    predecessor_owner_support_hash,
                    predecessor_key,
                )| {
                    let distinct_by_fresh_support = historical_prefix_equality_free
                        && predecessor_subject_hash != &current_subject_hash
                        && predecessor_owner_support_hash != &current_owner_support_hash;
                    let decision = if distinct_by_fresh_support {
                        "distinct_within_fragment"
                    } else {
                        "undefined"
                    }
                    .to_owned();
                    let reason = if distinct_by_fresh_support {
                        "distinct fresh declaration support under the equality-free sealed prefix"
                    } else {
                        "fresh-owner separation failed"
                    }
                    .to_owned();
                    let derivation_hash = tagged_hash(
                        "v3-fresh-owner-support-separation",
                        &(
                            step,
                            key,
                            &current_subject_hash,
                            &current_owner_support_hash,
                            predecessor_step,
                            predecessor_key,
                            predecessor_subject_hash,
                            predecessor_owner_support_hash,
                            historical_prefix_equality_free,
                            &decision,
                            &reason,
                        ),
                    );
                    SupportComparisonAudit {
                        predecessor_step: *predecessor_step,
                        predecessor_subject_hash: predecessor_subject_hash.clone(),
                        predecessor_owner_support_hash: predecessor_owner_support_hash.clone(),
                        predecessor_key: predecessor_key.clone(),
                        current_subject_hash: current_subject_hash.clone(),
                        current_owner_support_hash: current_owner_support_hash.clone(),
                        historical_prefix_equality_free,
                        decision,
                        reason,
                        derivation_hash,
                        distinct_by_fresh_support,
                    }
                },
            )
            .collect::<Vec<_>>();
        let predecessor_inventory_empty = predecessor_inventory.is_empty();
        let marginality_recomputed_from_fresh_owner_support = historical_prefix_equality_free
            && comparisons.len() == predecessor_inventory.len()
            && comparisons
                .iter()
                .all(|comparison| comparison.distinct_by_fresh_support);
        if !exact_key_correspondence_membership || !marginality_recomputed_from_fresh_owner_support
        {
            return Err(join_error(
                step,
                format!("independent v3 typed/marginal join failed for {key:?}"),
            ));
        }
        let proof_scope = "typed_and_marginal_in_registered_v3_fragment".to_owned();
        let egp_provenance_obstruction = "no demand-orbit/EGP token binds this registered path family to an independently chargeable novelty credit".to_owned();
        let individual_join_derivation_hash = tagged_hash(
            "typed-marginal-path-family-join",
            &(
                step,
                key,
                &legacy_step.family_inventory_digest,
                bundle.derivation_hash(),
                &current_subject_hash,
                &current_owner_support_hash,
                &comparisons,
                exact_key_correspondence_membership,
                marginality_recomputed_from_fresh_owner_support,
                &proof_scope,
                &egp_provenance_obstruction,
            ),
        );
        path_families.push(TypedMarginalPathFamilyAudit {
            key: key.clone(),
            legacy_operational_inventory_digest: legacy_step.family_inventory_digest.clone(),
            bundle_derivation_hash: bundle.derivation_hash().to_owned(),
            current_subject_hash: current_subject_hash.clone(),
            current_owner_support_hash: current_owner_support_hash.clone(),
            predecessor_comparisons: comparisons,
            predecessor_inventory_empty,
            exact_key_correspondence_membership,
            source_bound_to_exact_predecessor: true,
            marginality_recomputed_from_fresh_owner_support,
            proof_scope,
            typed: true,
            marginal: true,
            typed_and_marginal: true,
            credit_verdict: FamilyVerdict::Undefined,
            egp_provenance_obstruction,
            individual_join_derivation_hash,
        });
    }

    let ordinary_family_gaps = legacy_step
        .families
        .iter()
        .filter(|family| family.channel != FamilyChannel::Path)
        .map(|family| {
            let required_opaque_token = required_ordinary_token(&family.key).to_owned();
            let status = "undefined_missing_opaque_natural_family_token".to_owned();
            let gap_derivation_hash = tagged_hash(
                "ordinary-natural-family-gap",
                &(
                    step,
                    &family.key,
                    &family.judgement,
                    &family.source_clauses,
                    &required_opaque_token,
                    &status,
                    &family.derivation_hash,
                ),
            );
            OrdinaryNaturalFamilyGap {
                key: family.key.clone(),
                judgement: family.judgement.clone(),
                source_clauses: family.source_clauses.clone(),
                existing_evidence_level: evidence_level_name(&family.evidence_level).to_owned(),
                required_opaque_token,
                natural_family_proved: false,
                full_predecessor_sweep_proved: false,
                status,
                legacy_family_derivation_hash: family.derivation_hash.clone(),
                gap_derivation_hash,
            }
        })
        .collect::<Vec<_>>();
    if ordinary_family_gaps.len() + path_families.len() != legacy_step.families.len()
        || ordinary_family_gaps.is_empty()
    {
        return Err(join_error(
            step,
            "operational family partition is not exact",
        ));
    }

    let exact_operational_inventory_replayed = legacy_step.operational_inventory_complete
        && legacy_step.families.len() as u32 == legacy_step.presented_family_count;
    let exact_registered_path_inventory_replayed = path_families.len() as u64
        == bundle.realized_basis_count()
        && path_families.iter().all(|family| family.typed_and_marginal);
    let full_certified_total = None;
    let full_total_status = "partial_missing_ordinary_natural_family_tokens".to_owned();
    let registered_trunc_endpoint_gap_retired = bundle.kind() == RegisteredBoundaryKind::Trunc
        && bundle.evidence_kind() == HistoricalTypedBundleEvidenceKind::TruncEndpointDependentV1;
    let bundle_path_realization_derivation_hashes =
        bundle.path_realization_derivation_hashes().to_vec();
    let bundle_path_realization_inventory_digest = tagged_hash(
        "unordered-bundle-path-realization-inventory",
        &bundle_path_realization_derivation_hashes,
    );
    let support_comparison_count = path_families
        .iter()
        .map(|family| family.predecessor_comparisons.len() as u32)
        .sum::<u32>();
    let package_derivation_hash = tagged_hash(
        "historical-package-v3",
        &(
            (
                step,
                &legacy_step.predecessor.signature_digest,
                bundle.derivation_hash(),
                &bundle_path_realization_derivation_hashes,
                &bundle_path_realization_inventory_digest,
            ),
            (
                &correspondence,
                &current_subject_hash,
                &current_owner_support_hash,
                historical_prefix_equality_free,
                predecessor_inventory.len(),
                support_comparison_count,
            ),
            (
                legacy_step.presented_family_count,
                bundle.realized_basis_count(),
                &path_families,
                &ordinary_family_gaps,
            ),
            (
                full_certified_total,
                &full_total_status,
                registered_trunc_endpoint_gap_retired,
            ),
        ),
    );
    Ok(HistCertV3PackageAudit {
        step,
        label: legacy_step.label.clone(),
        dimension: legacy_step.dimension,
        predecessor_library: legacy_step.predecessor.visible_library,
        predecessor_signature_digest: legacy_step.predecessor.signature_digest.clone(),
        bundle_signature_digest: bundle.signature_digest().to_owned(),
        bundle_kind: kind_name(bundle.kind()).to_owned(),
        bundle_evidence_kind: evidence_kind_name(bundle.evidence_kind()).to_owned(),
        typed_boundary_derivation_hash: bundle.typed_boundary_derivation_hash().to_owned(),
        bundle_derivation_hash: bundle.derivation_hash().to_owned(),
        child_derivation_hash: bundle.child_derivation_hash().to_owned(),
        bundle_path_realization_derivation_hashes,
        bundle_path_realization_inventory_digest,
        key_correspondence: correspondence,
        current_subject_hash,
        current_owner_support_hash,
        historical_prefix_equality_free,
        predecessor_registered_path_inventory_count: predecessor_inventory.len() as u32,
        support_comparison_count,
        raw_operational_total: legacy_step.presented_family_count,
        recorded_total: legacy_step.recorded_total,
        expected_registered_path_total: bundle.expected_basis_count(),
        realized_registered_path_total: bundle.realized_basis_count(),
        typed_and_marginal_path_subtotal: path_families.len() as u32,
        exact_operational_inventory_replayed,
        exact_registered_path_inventory_replayed,
        predecessor_weakening_erasure_inverse_laws_checked: legacy_step
            .predecessor
            .weakening_erasure_inverse_laws_checked,
        path_families,
        ordinary_family_gap_count: ordinary_family_gaps.len() as u32,
        ordinary_family_gaps,
        full_certified_total,
        full_total_status,
        registered_trunc_endpoint_gap_retired,
        conditional_on_boundary_axiom_v3: true,
        package_derivation_hash,
    })
}

fn certificate_digest(certificate: &HistCertV3Certificate) -> String {
    let mut payload = certificate.clone();
    payload.result_digest.clear();
    tagged_hash("result", &payload)
}

pub fn build_hist_cert_v3() -> Result<HistCertV3Certificate, HistCertV3Error> {
    let archived_legacy: HistCertResult = serde_json::from_slice(HIST_CERT_V1_BYTES)
        .map_err(|error| HistCertV3Error::Legacy(error.to_string()))?;
    replay_hist_cert(&archived_legacy)
        .map_err(|error| HistCertV3Error::Legacy(error.to_string()))?;
    let live_legacy =
        build_hist_cert().map_err(|error| HistCertV3Error::Legacy(error.to_string()))?;
    if archived_legacy != live_legacy {
        return Err(HistCertV3Error::Legacy(
            "exact v1 artifact differs from live definition replay".to_owned(),
        ));
    }

    let trunc_json =
        std::str::from_utf8(TRUNC_ENDPOINT_V1_BYTES).map_err(|error| HistCertV3Error::Archive {
            name: "Trunc endpoint realizer v1".to_owned(),
            reason: error.to_string(),
        })?;
    let trunc_replay = replay_trunc_endpoint_realizer_json(trunc_json);
    if !trunc_replay.valid {
        return Err(HistCertV3Error::Archive {
            name: "Trunc endpoint realizer v1".to_owned(),
            reason: format!("definition replay failed: {:?}", trunc_replay.errors),
        });
    }

    let archival_bindings = vec![
        artifact_binding(
            "HIST-CERT v1",
            "docs/hist_cert_v1.json",
            HIST_CERT_V1_BYTES,
            207_055,
            "6D9FE83446ACEFB66AEB86168B5E96C677DAD355F3E2CF7D44F1EA5298AE6726",
            "blake3:e840f99c519756bc6215fbf8db03a738dae7c9127043599c06eaafed25d77c27",
            true,
        )?,
        artifact_binding(
            "Trunc endpoint realizer v1",
            "docs/trunc_endpoint_realizer_v1.json",
            TRUNC_ENDPOINT_V1_BYTES,
            21_372,
            "22C14233391B251379F3E99D6A3F199ECC3AD5F214291CF6CEA4451164D47967",
            "blake3:62b9e72a0047dd578f6881dfbeff9ff3b87cd19118e3f84ea917fdef604a84a1",
            trunc_replay.valid,
        )?,
    ];

    let handoff = issue_historical_v3_c6_typed_handoff_token()
        .map_err(|error| HistCertV3Error::Kernel(error.to_string()))?;
    replay_historical_v3_c6_typed_handoff_token(&handoff)
        .map_err(|error| HistCertV3Error::Kernel(error.to_string()))?;
    if handoff.bundles().len() != 4 || handoff.registered_counts() != EXPECTED_PATH_TOTALS {
        return Err(HistCertV3Error::Kernel(
            "all-four handoff is not exactly 2/2/5/10".to_owned(),
        ));
    }
    if live_legacy.steps.len() != 4 {
        return Err(HistCertV3Error::Legacy(
            "legacy audit does not contain exactly Steps 5--8".to_owned(),
        ));
    }
    let mut packages = Vec::with_capacity(4);
    for (index, legacy) in live_legacy.steps.iter().enumerate() {
        let bundle = &handoff.bundles()[index];
        packages.push(build_package(legacy, bundle, &handoff.bundles()[..index])?);
    }

    let observed_raw_totals = packages
        .iter()
        .map(|package| package.raw_operational_total)
        .collect::<Vec<_>>();
    let observed_typed_key_subtotals = packages
        .iter()
        .map(|package| {
            package
                .path_families
                .iter()
                .filter(|family| family.typed && family.exact_key_correspondence_membership)
                .count() as u64
        })
        .collect::<Vec<_>>();
    let observed_marginal_subtotals = packages
        .iter()
        .map(|package| {
            package
                .path_families
                .iter()
                .filter(|family| {
                    family.marginal && family.marginality_recomputed_from_fresh_owner_support
                })
                .count() as u64
        })
        .collect::<Vec<_>>();
    let observed_ordinary_unresolved = packages
        .iter()
        .map(|package| package.ordinary_family_gap_count)
        .collect::<Vec<_>>();
    let observed_predecessor_path_inventories = packages
        .iter()
        .map(|package| package.predecessor_registered_path_inventory_count)
        .collect::<Vec<_>>();
    let observed_support_comparison_counts = packages
        .iter()
        .map(|package| package.support_comparison_count)
        .collect::<Vec<_>>();
    let raw_totals_exact = observed_raw_totals == EXPECTED_RAW_TOTALS;
    let typed_key_subtotals_exact = observed_typed_key_subtotals == EXPECTED_PATH_TOTALS;
    let marginal_subtotals_exact = observed_marginal_subtotals == EXPECTED_PATH_TOTALS;
    let ordinary_unresolved_exact = observed_ordinary_unresolved == EXPECTED_ORDINARY_GAPS;
    let predecessor_path_inventories_exact =
        observed_predecessor_path_inventories == EXPECTED_PREDECESSOR_PATH_INVENTORIES;
    let support_comparison_counts_exact =
        observed_support_comparison_counts == EXPECTED_SUPPORT_COMPARISON_COUNTS;
    let all_four_exact_predecessors_joined = packages.iter().all(|package| {
        package.predecessor_library + 1 == package.step
            && package.predecessor_signature_digest == package.bundle_signature_digest
            && package.exact_operational_inventory_replayed
            && package.exact_registered_path_inventory_replayed
    });
    let all_registered_path_families_typed_and_marginal = packages
        .iter()
        .flat_map(|package| &package.path_families)
        .all(|family| family.typed_and_marginal);
    let full_certified_totals = packages
        .iter()
        .map(|package| package.full_certified_total)
        .collect::<Vec<_>>();
    let full_totals_partial = full_certified_totals.iter().all(Option::is_none)
        && packages
            .iter()
            .all(|package| package.ordinary_family_gap_count > 0);
    let f_b2_regression_signature_replayed =
        raw_totals_exact && typed_key_subtotals_exact && marginal_subtotals_exact;
    let partial_but_regression_clean = raw_totals_exact
        && typed_key_subtotals_exact
        && marginal_subtotals_exact
        && ordinary_unresolved_exact
        && predecessor_path_inventories_exact
        && support_comparison_counts_exact
        && all_four_exact_predecessors_joined
        && all_registered_path_families_typed_and_marginal
        && full_totals_partial;
    if !partial_but_regression_clean {
        return Err(HistCertV3Error::Legacy(
            "F-B2 did not reach the prescribed partial-but-regression-clean outcome".to_owned(),
        ));
    }

    let mut certificate = HistCertV3Certificate {
        schema: HIST_CERT_V3_SCHEMA.to_owned(),
        date: HIST_CERT_V3_DATE.to_owned(),
        versions: HistCertV3Versions {
            join: HIST_CERT_V3_JOIN_VERSION.to_owned(),
            legacy_hist_cert: HIST_CERT_SCHEMA.to_owned(),
            typed_handoff: HISTORICAL_PREFIX_V3_C6_TYPED_HANDOFF_VERSION.to_owned(),
            boundary_axiom_v3: ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION.to_owned(),
            trunc_endpoint_artifact: TRUNC_ENDPOINT_ARTIFACT_SCHEMA.to_owned(),
        },
        archival_bindings,
        packages,
        f_b2: FB2RegressionAudit {
            expected_raw_totals: EXPECTED_RAW_TOTALS.to_vec(),
            observed_raw_totals,
            raw_totals_exact,
            expected_typed_key_subtotals: EXPECTED_PATH_TOTALS.to_vec(),
            observed_typed_key_subtotals,
            typed_key_subtotals_exact,
            expected_marginal_subtotals: EXPECTED_PATH_TOTALS.to_vec(),
            observed_marginal_subtotals,
            marginal_subtotals_exact,
            expected_ordinary_unresolved: EXPECTED_ORDINARY_GAPS.to_vec(),
            observed_ordinary_unresolved,
            ordinary_unresolved_exact,
            expected_predecessor_path_inventories: EXPECTED_PREDECESSOR_PATH_INVENTORIES.to_vec(),
            observed_predecessor_path_inventories,
            predecessor_path_inventories_exact,
            expected_support_comparison_counts: EXPECTED_SUPPORT_COMPARISON_COUNTS.to_vec(),
            observed_support_comparison_counts,
            support_comparison_counts_exact,
            all_four_exact_predecessors_joined,
            all_registered_path_families_typed_and_marginal,
            full_certified_totals,
            full_totals_partial,
            f_b2_regression_signature_replayed,
            partial_but_regression_clean,
        },
        conditionality: HistCertV3Conditionality {
            conditional_on_boundary_axiom_v3: true,
            uses_adopted_boundary_axiom_v3: true,
            registered_historical_packages_only: true,
            trunc_endpoint_judgement_under_formal_method_premises: true,
            trunc_premise_free_closed_term_proved: false,
            registered_path_grammar_exhaustive_for_intended_schemas: false,
            intended_path_basis_independence_proved: false,
            arbitrary_typed_instantiation_proved: false,
            path_family_egp_provenance_complete: false,
            ordinary_natural_family_inventory_complete: false,
            general_c6_proved: false,
            candidate_c8_proved: false,
            intended_schema_classification_proved: false,
            derivable_from_sealed_trace_alone: false,
            full_historical_totals_certified: false,
            f_t1_discharged: false,
        },
        outcome: "partial_but_regression_clean".to_owned(),
        remaining_obligations: vec![
            "issue opaque natural-family tokens for formation, point/unit, path constructor, recursor, inductor, Trunc action, and post-path operations, including complete predecessor equality/weakening sweeps"
                .to_owned(),
            "prove the registered beta/Kan grammar exhaustive for intended depth-two semantic schemas"
                .to_owned(),
            "prove registered path-basis independence rather than only exact finite key presentation"
                .to_owned(),
            "bind every typed-and-marginal path family to an injective EGP role or live demand orbit before assigning novelty credit"
                .to_owned(),
            "issue typed natural-family signatures for the S3 post-path operation, coherence, and cell-action rows"
                .to_owned(),
            "extend restricted Trunc variable-image naturality to arbitrary typed images"
                .to_owned(),
            "retain the Trunc endpoint result under its explicit formal eliminator-method premises; no premise-free closed term is proved"
                .to_owned(),
            "prove general C6, the candidate-level C8 join, and intended-schema classification beyond the four registered historical packages"
                .to_owned(),
            "do not promote the partial registered-path subtotals to full certified historical totals until every ordinary-family gap is replayably closed"
                .to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn replay_against(
    certificate: &HistCertV3Certificate,
    expected: &HistCertV3Certificate,
) -> HistCertV3Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    HistCertV3Replay {
        valid: errors.is_empty(),
        raw_totals_7_8_10_18: certificate.f_b2.raw_totals_exact
            && certificate.f_b2.observed_raw_totals == EXPECTED_RAW_TOTALS,
        path_subtotals_2_2_5_10: certificate.f_b2.typed_key_subtotals_exact
            && certificate.f_b2.marginal_subtotals_exact
            && certificate.f_b2.observed_typed_key_subtotals == EXPECTED_PATH_TOTALS
            && certificate.f_b2.observed_marginal_subtotals == EXPECTED_PATH_TOTALS,
        all_registered_paths_typed_and_marginal: certificate
            .f_b2
            .all_registered_path_families_typed_and_marginal,
        full_certified_totals_partial: certificate.f_b2.full_totals_partial
            && certificate
                .f_b2
                .full_certified_totals
                .iter()
                .all(Option::is_none),
        f_t1_discharged: certificate.conditionality.f_t1_discharged,
        errors,
    }
}

fn failed_replay(error: impl Into<String>) -> HistCertV3Replay {
    HistCertV3Replay {
        valid: false,
        raw_totals_7_8_10_18: false,
        path_subtotals_2_2_5_10: false,
        all_registered_paths_typed_and_marginal: false,
        full_certified_totals_partial: false,
        f_t1_discharged: false,
        errors: vec![error.into()],
    }
}

pub fn replay_hist_cert_v3(certificate: &HistCertV3Certificate) -> HistCertV3Replay {
    match build_hist_cert_v3() {
        Ok(expected) => replay_against(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn hist_cert_v3_json_pretty() -> Result<String, HistCertV3Error> {
    serde_json::to_string_pretty(&build_hist_cert_v3()?)
        .map(|json| format!("{json}\n"))
        .map_err(|error| HistCertV3Error::Json(error.to_string()))
}

pub fn replay_hist_cert_v3_json(json: &str) -> HistCertV3Replay {
    let raw: serde_json::Value = match serde_json::from_str(json) {
        Ok(raw) => raw,
        Err(error) => return failed_replay(format!("invalid JSON: {error}")),
    };
    let certificate: HistCertV3Certificate = match serde_json::from_str(json) {
        Ok(certificate) => certificate,
        Err(error) => return failed_replay(format!("certificate shape error: {error}")),
    };
    let typed = serde_json::to_value(&certificate).expect("certificate serializes");
    if raw != typed {
        return failed_replay("JSON contains unknown, duplicate, or ignored structure");
    }
    replay_hist_cert_v3(&certificate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::{ClauseRec, ClauseRole};

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ValuePathItem {
        Field(String),
        Index(usize),
    }

    fn redigest(certificate: &mut HistCertV3Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    fn collect_paths(
        value: &serde_json::Value,
        path: &mut Vec<ValuePathItem>,
        leaves: &mut Vec<Vec<ValuePathItem>>,
        arrays: &mut Vec<Vec<ValuePathItem>>,
    ) {
        match value {
            serde_json::Value::Object(object) => {
                for (field, child) in object {
                    path.push(ValuePathItem::Field(field.clone()));
                    collect_paths(child, path, leaves, arrays);
                    path.pop();
                }
            }
            serde_json::Value::Array(array) => {
                arrays.push(path.clone());
                for (index, child) in array.iter().enumerate() {
                    path.push(ValuePathItem::Index(index));
                    collect_paths(child, path, leaves, arrays);
                    path.pop();
                }
            }
            _ => leaves.push(path.clone()),
        }
    }

    fn value_at_path_mut<'a>(
        mut value: &'a mut serde_json::Value,
        path: &[ValuePathItem],
    ) -> &'a mut serde_json::Value {
        for item in path {
            value = match item {
                ValuePathItem::Field(field) => value
                    .as_object_mut()
                    .and_then(|object| object.get_mut(field))
                    .expect("recorded object path exists"),
                ValuePathItem::Index(index) => value
                    .as_array_mut()
                    .and_then(|array| array.get_mut(*index))
                    .expect("recorded array path exists"),
            };
        }
        value
    }

    fn path_label(path: &[ValuePathItem]) -> String {
        let mut label = String::new();
        for item in path {
            match item {
                ValuePathItem::Field(field) => {
                    if !label.is_empty() {
                        label.push('.');
                    }
                    label.push_str(field);
                }
                ValuePathItem::Index(index) => label.push_str(&format!("[{index}]")),
            }
        }
        label
    }

    fn assert_redigested_rejected(
        value: serde_json::Value,
        expected: &HistCertV3Certificate,
        label: &str,
    ) {
        let Ok(mut mutated): Result<HistCertV3Certificate, _> = serde_json::from_value(value)
        else {
            return;
        };
        redigest(&mut mutated);
        assert!(
            !replay_against(&mutated, expected).valid,
            "redigested mutation at {label} must fail definition replay"
        );
    }

    #[test]
    fn f_b2_is_partial_but_regression_clean() {
        let certificate = build_hist_cert_v3().expect("HIST-CERT v3");
        let replay = replay_against(&certificate, &certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(replay.raw_totals_7_8_10_18);
        assert!(replay.path_subtotals_2_2_5_10);
        assert!(replay.all_registered_paths_typed_and_marginal);
        assert!(replay.full_certified_totals_partial);
        assert!(!replay.f_t1_discharged);
        assert_eq!(
            certificate
                .packages
                .iter()
                .map(|package| package.typed_and_marginal_path_subtotal)
                .collect::<Vec<_>>(),
            [2, 2, 5, 10]
        );
        assert!(
            certificate
                .packages
                .iter()
                .all(|package| package.full_certified_total.is_none())
        );
        assert!(certificate.packages[1].registered_trunc_endpoint_gap_retired);
    }

    #[test]
    fn equality_free_gate_rejects_a_nested_identity_clause() {
        let nested_identity = Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Univ),
            Box::new(Expr::Id(
                Box::new(Expr::Univ),
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(1)),
            )),
        )));
        let signature = SealedSignature::from_telescopes(vec![(
            1,
            Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, nested_identity)]),
        )]);
        assert!(!prefix_is_equality_free(&signature));
        assert!(prefix_is_equality_free(&exact_historical_prefix(8)));
    }

    #[test]
    fn every_scalar_projection_mutation_fails_after_redigest() {
        let expected = build_hist_cert_v3().expect("HIST-CERT v3");
        let original = serde_json::to_value(&expected).expect("certificate projects");
        let mut leaves = Vec::new();
        let mut arrays = Vec::new();
        collect_paths(&original, &mut Vec::new(), &mut leaves, &mut arrays);
        let digest_path = [ValuePathItem::Field("result_digest".to_owned())];
        for path in leaves {
            if path == digest_path {
                continue;
            }
            let mut mutated = original.clone();
            match value_at_path_mut(&mut mutated, &path) {
                serde_json::Value::Bool(value) => *value = !*value,
                serde_json::Value::Number(value) => {
                    let number = value.as_u64().expect("certificate numbers are unsigned");
                    *value = serde_json::Number::from(number + 1);
                }
                serde_json::Value::String(value) => value.push_str(":mutated"),
                value @ serde_json::Value::Null => *value = serde_json::Value::Bool(true),
                serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                    unreachable!("recorded path is scalar")
                }
            }
            assert_redigested_rejected(mutated, &expected, &path_label(&path));
        }

        let mut bad_digest = expected;
        bad_digest.result_digest.push_str(":mutated");
        assert!(!replay_hist_cert_v3(&bad_digest).valid);
    }

    #[test]
    fn every_nonempty_vector_structure_mutation_fails_after_redigest() {
        let expected = build_hist_cert_v3().expect("HIST-CERT v3");
        let original = serde_json::to_value(&expected).expect("certificate projects");
        let mut leaves = Vec::new();
        let mut arrays = Vec::new();
        collect_paths(&original, &mut Vec::new(), &mut leaves, &mut arrays);
        for path in arrays {
            let items = value_at_path_mut(&mut original.clone(), &path)
                .as_array()
                .expect("recorded array path")
                .clone();
            if items.is_empty() {
                continue;
            }
            let label = path_label(&path);
            let mut deleted = original.clone();
            value_at_path_mut(&mut deleted, &path)
                .as_array_mut()
                .expect("array")
                .remove(0);
            assert_redigested_rejected(deleted, &expected, &format!("{label}:delete"));

            let mut duplicated = original.clone();
            value_at_path_mut(&mut duplicated, &path)
                .as_array_mut()
                .expect("array")
                .push(items[0].clone());
            assert_redigested_rejected(duplicated, &expected, &format!("{label}:duplicate"));

            if items.len() > 1 {
                let mut reordered = original.clone();
                let array = value_at_path_mut(&mut reordered, &path)
                    .as_array_mut()
                    .expect("array");
                array.reverse();
                if array != &items {
                    assert_redigested_rejected(reordered, &expected, &format!("{label}:reorder"));
                }
            }
        }
    }

    #[test]
    fn omitted_support_comparison_and_credit_promotion_fail_definition_replay() {
        let expected = build_hist_cert_v3().expect("HIST-CERT v3");

        let mut omitted_comparison = expected.clone();
        omitted_comparison.packages[1].path_families[0]
            .predecessor_comparisons
            .pop()
            .expect("Step 6 has predecessor comparisons");
        redigest(&mut omitted_comparison);
        assert!(!replay_against(&omitted_comparison, &expected).valid);

        let mut promoted_path = expected.clone();
        promoted_path.packages[0].path_families[0].credit_verdict = FamilyVerdict::Marginal;
        redigest(&mut promoted_path);
        assert!(!replay_against(&promoted_path, &expected).valid);

        let mut promoted_total = expected.clone();
        promoted_total.packages[0].full_certified_total = Some(7);
        promoted_total
            .conditionality
            .full_historical_totals_certified = true;
        promoted_total.conditionality.f_t1_discharged = true;
        redigest(&mut promoted_total);
        assert!(!replay_against(&promoted_total, &expected).valid);
    }

    #[test]
    fn strict_json_rejects_unknown_and_duplicate_fields_at_all_depths() {
        let json = hist_cert_v3_json_pretty().expect("HIST-CERT v3 JSON");
        assert!(replay_hist_cert_v3_json(&json).valid);

        let unknown_top = json.replacen("{\n", "{\n  \"unknown_top\": true,\n", 1);
        assert!(!replay_hist_cert_v3_json(&unknown_top).valid);

        let unknown_deep = json.replacen(
            "\"versions\": {",
            "\"versions\": {\n    \"unknown_deep\": true,",
            1,
        );
        assert!(!replay_hist_cert_v3_json(&unknown_deep).valid);

        let unknown_row = json.replacen(
            "\"path_families\": [\n        {",
            "\"path_families\": [\n        {\n          \"unknown_row\": true,",
            1,
        );
        assert_ne!(unknown_row, json);
        assert!(!replay_hist_cert_v3_json(&unknown_row).valid);

        let duplicate_top = json.replacen(
            "\"schema\": \"hist-cert-historical-hit-v3\",",
            "\"schema\": \"hist-cert-historical-hit-v3\",\n  \"schema\": \"hist-cert-historical-hit-v3\",",
            1,
        );
        assert!(!replay_hist_cert_v3_json(&duplicate_top).valid);

        let duplicate_deep = json.replacen(
            "\"join\": \"hist-cert-v3-f-b2-typed-bundle-support-local-join-v1\",",
            "\"join\": \"hist-cert-v3-f-b2-typed-bundle-support-local-join-v1\",\n    \"join\": \"hist-cert-v3-f-b2-typed-bundle-support-local-join-v1\",",
            1,
        );
        assert!(!replay_hist_cert_v3_json(&duplicate_deep).valid);

        let duplicate_row = json.replacen(
            "\"typed\": true,",
            "\"typed\": true,\n        \"typed\": true,",
            1,
        );
        assert!(!replay_hist_cert_v3_json(&duplicate_row).valid);
    }
}
