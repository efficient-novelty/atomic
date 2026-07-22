//! Create-new replay certificate for the registered d=1 Trunc endpoint
//! realizer.  Opaque kernel tokens are never deserialized from this JSON:
//! replay reconstructs them from the sealed H15/B5 sources and compares the
//! exact audit projection.

use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::cubical::TRUNC_ENDPOINT_REALIZER_FRAGMENT_VERSION;
use pen_type::cubical::boundary_variants::BoundaryBasisKey;
use pen_type::cubical::typed_boundary::{
    ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION, BOUNDARY_CHARGE_POLICY_VERSION,
    C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION_GAP,
    C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP,
    HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION, HISTORICAL_PREFIX_V3_C6_TYPED_HANDOFF_VERSION,
    HistoricalTypedBundleEvidenceKind, RegisteredBoundaryKind, TruncEndpointSourceScope,
    TruncEndpointV3C6BundleToken, TruncRestrictedInstanceKind,
    issue_historical_v3_c6_typed_handoff_token, issue_trunc_endpoint_v3_c6_bundle_token,
    issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix,
    replay_historical_v3_c6_typed_handoff_token, replay_trunc_endpoint_v3_c6_bundle_token,
    replay_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix,
};
use pen_type::elaborate::SealedSignature;
use pen_type::tdc1::PathSchemaKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub const TRUNC_ENDPOINT_ARTIFACT_SCHEMA: &str = "trunc-endpoint-realizer-v1";
pub const TRUNC_ENDPOINT_ARTIFACT_DATE: &str = "2026-07-19";
pub const GENERAL_C6_GAP: &str = "C6_V3_GENERAL_HISTORICAL_TERM_LEVEL_COMPLETION";
pub const C8_GAP: &str = "C8_CANDIDATE_BOUNDARY_PROVENANCE_JOIN";
pub const INTENDED_SCHEMA_GAP: &str = "INTENDED_DEPTH_TWO_SCHEMA_CLASSIFICATION";
pub const F_B2_GAP: &str = "F-B2-HIST-CERT-V3-CREATE-NEW-RERUN-PENDING";

const HIST_CERT_BYTES: &[u8] = include_bytes!("../../../docs/hist_cert_v1.json");
const BOUNDARY_AUDIT_BYTES: &[u8] = include_bytes!("../../../docs/boundary_audit_v1.json");
const TDC_CUBICAL_BYTES: &[u8] = include_bytes!("../../../docs/tdc1_cubical_regression_v3.json");
const SCHEMA3_BYTES: &[u8] = include_bytes!("../../../docs/ip1_candidate_verdict_join_v3.json");
const SCHEMA4_BYTES: &[u8] = include_bytes!("../../../docs/ip1_candidate_verdict_join_v4.json");
const SCHEMA5_BYTES: &[u8] =
    include_bytes!("../../../docs/ip1_candidate_verdict_join_v5_element_overlay.json");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TruncEndpointArtifactError {
    #[error("kernel endpoint realization failed: {0}")]
    Kernel(String),
    #[error("archival byte binding failed for {name}: {reason}")]
    Archive { name: String, reason: String },
    #[error("certificate JSON failed: {0}")]
    Json(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionPins {
    pub endpoint_fragment: String,
    pub boundary_axiom_v3: String,
    pub element_overlay: String,
    pub boundary_charge_policy: String,
    pub handoff_api: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceAudit {
    pub step: u32,
    pub kind: String,
    pub dimension: u32,
    pub full_h15_signature_digest: String,
    pub exact_b5_signature_digest: String,
    pub candidate_hash: String,
    pub full_telescope_digest: String,
    pub prefix_telescope_digest: String,
    pub full_typing_derivation_hash: String,
    pub prefix_typing_derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredEndpointAudit {
    pub axis: u32,
    pub endpoint: bool,
    pub parameter_index: u16,
    pub parameter_name: String,
    pub parameter_type: String,
    pub parameter_context_digest: String,
    pub boundary_term_normal_form_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredBoundaryAudit {
    pub context_label: String,
    pub full_typed_boundary_derivation_hash: String,
    pub prefix_typed_boundary_derivation_hash: String,
    pub full_boundary_binding_digest: String,
    pub prefix_boundary_binding_digest: String,
    pub endpoints: Vec<DeclaredEndpointAudit>,
    pub exact_x_y_distinction: bool,
    pub overlay_entry_used: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MethodAudit {
    pub shape: String,
    pub formal_schema_premises_explicit: bool,
    pub context_free_endpoint_hypotheses_rejected: bool,
    pub premise_source_digest_full: String,
    pub premise_source_digest_prefix: String,
    pub premise_context_derivation_hash_full: String,
    pub premise_context_derivation_hash_prefix: String,
    pub method_shape_digest_full: String,
    pub method_shape_digest_prefix: String,
    pub computation_audit_derivation_hash_full: String,
    pub computation_audit_derivation_hash_prefix: String,
    pub zero_restriction_computes_to_method_image: bool,
    pub one_restriction_computes_to_method_image: bool,
    pub zero_scrutinee_computes_by_endpoint_instantiation: bool,
    pub one_scrutinee_computes_by_endpoint_instantiation: bool,
    pub constructor_scrutinee_computes_to_dependent_method: bool,
    pub typed_neutral_stays_stuck_at_motive_instance: bool,
    pub boundary_aware_coe_targets_right_endpoint: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyCorrespondenceAudit {
    pub boundary_key: String,
    pub path_key: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RealizationAudit {
    pub key: String,
    pub term_hash: String,
    pub normal_form_hash: String,
    pub type_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeAudit {
    pub scope: String,
    pub signature_digest: String,
    pub typed_boundary_derivation_hash: String,
    pub boundary_attachment_derivation_hash: String,
    pub boundary_basis_presentation_derivation_hash: String,
    pub endpoint_premise_context_derivation_hash: String,
    pub method_shape_digest: String,
    pub key_correspondence: Vec<KeyCorrespondenceAudit>,
    pub key_bijection_digest: String,
    pub path_basis_derivation_hash: String,
    pub path_realizations: Vec<RealizationAudit>,
    pub expected_basis_count: u64,
    pub realized_basis_count: u64,
    pub exact_key_set: bool,
    pub replayed: bool,
    pub bundle_derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestrictedInstantiationAudit {
    pub theorem_scope: String,
    pub owner_identity_derivation_hash: String,
    pub zero_endpoint_identity_derivation_hash: String,
    pub one_endpoint_identity_derivation_hash: String,
    pub endpoint_swap_derivation_hash: String,
    pub all_images_are_sort_identical_variables: bool,
    pub all_four_fixed_context_maps_replayed: bool,
    pub variable_instances: Vec<RestrictedVariableInstanceAudit>,
    pub arbitrary_typed_images_used: bool,
    pub inherited_arbitrary_typed_image_gap: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RestrictedVariableInstanceAudit {
    pub kind: String,
    pub source_to_target_variables: Vec<(u32, u32)>,
    pub zero_target_variable: u32,
    pub one_target_variable: u32,
    pub zero_boundary_parameter: u16,
    pub one_boundary_parameter: u16,
    pub has_degenerate_endpoints: bool,
    pub owner_substitution_derivation_hash: String,
    pub zero_endpoint_substitution_derivation_hash: String,
    pub one_endpoint_substitution_derivation_hash: String,
    pub substitution_derivation_hash: String,
    pub premise_source_digest: String,
    pub premise_context_derivation_hash: String,
    pub computation_audit_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffRow {
    pub kind: String,
    pub step: u32,
    pub dimension: u32,
    pub prefix_signature_digest: String,
    pub evidence_kind: String,
    pub expected_basis_count: u64,
    pub realized_basis_count: u64,
    pub child_derivation_hash: String,
    pub wrapper_derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentAHandoffAudit {
    pub api_version: String,
    pub registered_counts: Vec<u64>,
    pub rows: Vec<HandoffRow>,
    pub all_replayed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchiveBinding {
    pub name: String,
    pub path: String,
    pub byte_length: u64,
    pub sha256: String,
    pub internal_digest: String,
    pub exact_bytes_pinned: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FalsifierAudit {
    pub f_tr1_registered_computation_realizable_under_formal_method_premises: bool,
    pub f_tr2_exact_archival_bytes_unchanged: bool,
    pub f_tr2_definition_replay_external_gate_required: bool,
    pub definition_replay_proved_by_this_certificate: bool,
    pub f_tr3_exactly_two_exported_keys: bool,
    pub f_tr4_arbitrary_typed_images_unused_and_gap_named: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionalityAudit {
    pub conditional_on_boundary_axiom_v3: bool,
    pub judgement_is_under_formal_eliminator_method_premises: bool,
    pub closed_term_without_method_premises_proved: bool,
    pub trunc_element_overlay_entry_required: bool,
    pub registered_trunc_c6_endpoint_realizer_proved: bool,
    pub general_c6_proved: bool,
    pub basis_independence_proved: bool,
    pub intended_basis_exhaustiveness_proved: bool,
    pub candidate_level_c8_proved: bool,
    pub intended_schema_classification_proved: bool,
    pub step16_or_global_halt_proved: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruncEndpointRealizerCertificate {
    pub schema: String,
    pub date: String,
    pub versions: VersionPins,
    pub source: SourceAudit,
    pub declared_boundary: DeclaredBoundaryAudit,
    pub method: MethodAudit,
    pub scopes: Vec<ScopeAudit>,
    pub restricted_instantiation: RestrictedInstantiationAudit,
    pub agent_a_handoff: AgentAHandoffAudit,
    pub archival_bindings: Vec<ArchiveBinding>,
    pub falsifiers: FalsifierAudit,
    pub conditionality: ConditionalityAudit,
    pub obstruction_retired: String,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TruncEndpointReplay {
    pub valid: bool,
    pub full_h15_replayed: bool,
    pub exact_b5_replayed: bool,
    pub exact_two_key_basis: bool,
    pub all_four_handoff_counts_2_2_5_10: bool,
    pub archival_bytes_pinned: bool,
    pub general_c6_proved: bool,
    pub errors: Vec<String>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:X}", Sha256::digest(bytes))
}

fn archive_binding(
    name: &str,
    path: &str,
    bytes: &[u8],
    expected_length: u64,
    expected_sha256: &str,
    expected_internal_digest: &str,
) -> Result<ArchiveBinding, TruncEndpointArtifactError> {
    let byte_length = bytes.len() as u64;
    let sha256 = sha256_hex(bytes);
    if byte_length != expected_length || sha256 != expected_sha256 {
        return Err(TruncEndpointArtifactError::Archive {
            name: name.to_owned(),
            reason: format!(
                "expected {expected_length}/{expected_sha256}, found {byte_length}/{sha256}"
            ),
        });
    }
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|error| TruncEndpointArtifactError::Archive {
            name: name.to_owned(),
            reason: error.to_string(),
        })?;
    let internal_digest = value
        .get("result_digest")
        .or_else(|| value.get("digest"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| TruncEndpointArtifactError::Archive {
            name: name.to_owned(),
            reason: "missing result_digest/digest".to_owned(),
        })?
        .to_owned();
    if internal_digest != expected_internal_digest {
        return Err(TruncEndpointArtifactError::Archive {
            name: name.to_owned(),
            reason: format!(
                "expected internal digest {expected_internal_digest}, found {internal_digest}"
            ),
        });
    }
    Ok(ArchiveBinding {
        name: name.to_owned(),
        path: path.to_owned(),
        byte_length,
        sha256,
        internal_digest,
        exact_bytes_pinned: true,
    })
}

fn archival_bindings() -> Result<Vec<ArchiveBinding>, TruncEndpointArtifactError> {
    Ok(vec![
        archive_binding(
            "HIST-CERT v1",
            "docs/hist_cert_v1.json",
            HIST_CERT_BYTES,
            207_055,
            "6D9FE83446ACEFB66AEB86168B5E96C677DAD355F3E2CF7D44F1EA5298AE6726",
            "blake3:e840f99c519756bc6215fbf8db03a738dae7c9127043599c06eaafed25d77c27",
        )?,
        archive_binding(
            "BOUNDARY-AUDIT v1",
            "docs/boundary_audit_v1.json",
            BOUNDARY_AUDIT_BYTES,
            38_629,
            "1A3D8575784A1987C7A56EE13D265F56CC7F47F808EE3E4D28B248A85A7E7CB4",
            "blake3:76c389766a5c54582d1e2ffe9942db5c53cd3764137cfc62254b9857e1f1c435",
        )?,
        archive_binding(
            "TDC cubical regression v3",
            "docs/tdc1_cubical_regression_v3.json",
            TDC_CUBICAL_BYTES,
            36_033,
            "65CF20883420C5777BF487FFFCE046A447E097DDEFD0772E4B58BF3808F12753",
            "blake3:075791e9024a8c36a3f0dab593fa1460fb8288d3e9fc3345aa76c48cff978e56",
        )?,
        archive_binding(
            "IP-1 schema 3",
            "docs/ip1_candidate_verdict_join_v3.json",
            SCHEMA3_BYTES,
            168_692,
            "4913E818C080D67C81CE1770F047E22DB9EA6975BFE704FB01352F268FA2B729",
            "blake3:e4cb89fe83476fb8a0eee43bcc976053bcbbff0bd6db35607d7e8154a3ef858b",
        )?,
        archive_binding(
            "IP-1 schema 4",
            "docs/ip1_candidate_verdict_join_v4.json",
            SCHEMA4_BYTES,
            535_972,
            "4094B2E65E36507157029760C71FF62BA241E82D066CF590F9184DE66BEDB5F4",
            "blake3:6a6c3ccfe4731080bfaa944b98f50afe2bde1eb29fdd679b8f62e3a42f72d1e2",
        )?,
        archive_binding(
            "IP-1 schema 5 element overlay",
            "docs/ip1_candidate_verdict_join_v5_element_overlay.json",
            SCHEMA5_BYTES,
            484_717,
            "C4CFBB693BB3EB6F2CBBDD14495475EE856BE64EC42890A4FC5B2EA612EB1AA2",
            "blake3:ee5a0883ef0a81450035d472c855a81d259d6058498983eddc05707b8c3b637b",
        )?,
    ])
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

fn path_key_name(key: &PathSchemaKey) -> String {
    match key {
        PathSchemaKey::Beta => "beta".to_owned(),
        PathSchemaKey::Kan { principal, probe } => format!("kan({principal},{probe})"),
    }
}

fn kind_name(kind: RegisteredBoundaryKind) -> String {
    match kind {
        RegisteredBoundaryKind::S1 => "s1",
        RegisteredBoundaryKind::Trunc => "trunc",
        RegisteredBoundaryKind::S2 => "s2",
        RegisteredBoundaryKind::S3 => "s3",
    }
    .to_owned()
}

fn source_scope_name(scope: TruncEndpointSourceScope) -> String {
    match scope {
        TruncEndpointSourceScope::FullH15 => "full_h15",
        TruncEndpointSourceScope::HistoricalPrefixB5 => "historical_prefix_b5",
    }
    .to_owned()
}

fn evidence_kind_name(kind: HistoricalTypedBundleEvidenceKind) -> String {
    match kind {
        HistoricalTypedBundleEvidenceKind::ArchivalConstantBridge => "archival_constant_bridge",
        HistoricalTypedBundleEvidenceKind::TruncEndpointDependentV1 => {
            "trunc_endpoint_dependent_v1"
        }
    }
    .to_owned()
}

fn restricted_instance_kind_name(kind: TruncRestrictedInstanceKind) -> String {
    match kind {
        TruncRestrictedInstanceKind::Identity => "identity",
        TruncRestrictedInstanceKind::Swap => "swap",
        TruncRestrictedInstanceKind::CollapseToX => "collapse_to_x",
        TruncRestrictedInstanceKind::CollapseToY => "collapse_to_y",
    }
    .to_owned()
}

fn scope_audit(token: &TruncEndpointV3C6BundleToken) -> ScopeAudit {
    let key_correspondence = token
        .key_correspondence()
        .iter()
        .map(|entry| KeyCorrespondenceAudit {
            boundary_key: boundary_key_name(entry.boundary_key()),
            path_key: path_key_name(entry.path_key()),
        })
        .collect::<Vec<_>>();
    let path_realizations = token
        .path_realizations()
        .iter()
        .map(|audit| RealizationAudit {
            key: path_key_name(audit.key()),
            term_hash: audit.term_hash().to_owned(),
            normal_form_hash: audit.normal_form_hash().to_owned(),
            type_hash: audit.type_hash().to_owned(),
            derivation_hash: audit.derivation_hash().to_owned(),
        })
        .collect::<Vec<_>>();
    let exact_key_set = key_correspondence
        == [
            KeyCorrespondenceAudit {
                boundary_key: "beta".to_owned(),
                path_key: "beta".to_owned(),
            },
            KeyCorrespondenceAudit {
                boundary_key: "principal_transport(0)".to_owned(),
                path_key: "kan(0,0)".to_owned(),
            },
        ]
        && path_realizations
            .iter()
            .map(|audit| audit.key.as_str())
            .collect::<Vec<_>>()
            == ["beta", "kan(0,0)"];
    ScopeAudit {
        scope: source_scope_name(token.source_scope()),
        signature_digest: token.signature_digest().to_owned(),
        typed_boundary_derivation_hash: token.typed_boundary_derivation_hash().to_owned(),
        boundary_attachment_derivation_hash: token.boundary_attachment_derivation_hash().to_owned(),
        boundary_basis_presentation_derivation_hash: token
            .boundary_basis_presentation_derivation_hash()
            .to_owned(),
        endpoint_premise_context_derivation_hash: token
            .endpoint_premise_context_derivation_hash()
            .to_owned(),
        method_shape_digest: token.method_shape_digest().to_owned(),
        key_correspondence,
        key_bijection_digest: token.key_bijection_digest().to_owned(),
        path_basis_derivation_hash: token.path_basis_derivation_hash().to_owned(),
        path_realizations,
        expected_basis_count: token.expected_basis_count(),
        realized_basis_count: token.realized_basis_count(),
        exact_key_set,
        replayed: true,
        bundle_derivation_hash: token.derivation_hash().to_owned(),
    }
}

fn certificate_digest(certificate: &TruncEndpointRealizerCertificate) -> String {
    let mut payload = certificate.clone();
    payload.result_digest.clear();
    let bytes = serde_json::to_vec(&payload).expect("certificate serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

pub fn build_trunc_endpoint_realizer_certificate()
-> Result<TruncEndpointRealizerCertificate, TruncEndpointArtifactError> {
    let full_signature = SealedSignature::genesis_del_h15();
    let full = issue_trunc_endpoint_v3_c6_bundle_token(&full_signature)
        .map_err(|error| TruncEndpointArtifactError::Kernel(error.to_string()))?;
    replay_trunc_endpoint_v3_c6_bundle_token(&full_signature, &full)
        .map_err(|error| TruncEndpointArtifactError::Kernel(error.to_string()))?;

    let prefix = SealedSignature::from_telescopes(
        (1..6)
            .map(|step| (step, Telescope::reference(step)))
            .collect(),
    );
    let current = Telescope::reference(6);
    let historical =
        issue_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(&prefix, &current)
            .map_err(|error| TruncEndpointArtifactError::Kernel(error.to_string()))?;
    replay_trunc_endpoint_v3_c6_bundle_token_for_historical_prefix(&prefix, &current, &historical)
        .map_err(|error| TruncEndpointArtifactError::Kernel(error.to_string()))?;

    let handoff = issue_historical_v3_c6_typed_handoff_token()
        .map_err(|error| TruncEndpointArtifactError::Kernel(error.to_string()))?;
    replay_historical_v3_c6_typed_handoff_token(&handoff)
        .map_err(|error| TruncEndpointArtifactError::Kernel(error.to_string()))?;

    let endpoints = full
        .endpoint_faces()
        .iter()
        .map(|binding| DeclaredEndpointAudit {
            axis: binding.face().axis,
            endpoint: binding.face().endpoint,
            parameter_index: binding.parameter_index(),
            parameter_name: binding.parameter_name().to_owned(),
            parameter_type: format!("{:?}", binding.ty()),
            parameter_context_digest: binding.parameter_context_digest().to_owned(),
            boundary_term_normal_form_digest: binding.boundary_term_normal_form_digest().to_owned(),
        })
        .collect::<Vec<_>>();
    let exact_x_y_distinction = endpoints.len() == 2
        && endpoints[0].axis == 0
        && !endpoints[0].endpoint
        && endpoints[0].parameter_index == 1
        && endpoints[0].parameter_name == "x"
        && endpoints[1].axis == 0
        && endpoints[1].endpoint
        && endpoints[1].parameter_index == 2
        && endpoints[1].parameter_name == "y";
    if !exact_x_y_distinction {
        return Err(TruncEndpointArtifactError::Kernel(
            "registered x/y endpoint distinction failed".to_owned(),
        ));
    }

    let restricted = full.restricted_instantiation();
    let restricted_variable_instances = restricted
        .variable_instances()
        .iter()
        .map(|instance| {
            let source_to_target_variables = instance
                .images()
                .iter()
                .map(|image| {
                    let Expr::Var(target_parameter) = &image.term else {
                        unreachable!("restricted instance token contains only variable images")
                    };
                    (image.source_parameter, *target_parameter)
                })
                .collect::<Vec<_>>();
            RestrictedVariableInstanceAudit {
                kind: restricted_instance_kind_name(instance.kind()),
                source_to_target_variables,
                zero_target_variable: instance.zero_target_variable(),
                one_target_variable: instance.one_target_variable(),
                zero_boundary_parameter: instance.zero_boundary_parameter(),
                one_boundary_parameter: instance.one_boundary_parameter(),
                has_degenerate_endpoints: instance.has_degenerate_endpoints(),
                owner_substitution_derivation_hash: instance
                    .owner_substitution_derivation_hash()
                    .to_owned(),
                zero_endpoint_substitution_derivation_hash: instance
                    .zero_endpoint_substitution_derivation_hash()
                    .to_owned(),
                one_endpoint_substitution_derivation_hash: instance
                    .one_endpoint_substitution_derivation_hash()
                    .to_owned(),
                substitution_derivation_hash: instance.substitution_derivation_hash().to_owned(),
                premise_source_digest: instance.premise_source_digest().to_owned(),
                premise_context_derivation_hash: instance
                    .premise_context_derivation_hash()
                    .to_owned(),
                computation_audit_derivation_hash: instance
                    .computation_audit_derivation_hash()
                    .to_owned(),
                derivation_hash: instance.derivation_hash().to_owned(),
            }
        })
        .collect::<Vec<_>>();
    let rows = handoff
        .bundles()
        .iter()
        .map(|bundle| HandoffRow {
            kind: kind_name(bundle.kind()),
            step: bundle.step(),
            dimension: bundle.dimension(),
            prefix_signature_digest: bundle.signature_digest().to_owned(),
            evidence_kind: evidence_kind_name(bundle.evidence_kind()),
            expected_basis_count: bundle.expected_basis_count(),
            realized_basis_count: bundle.realized_basis_count(),
            child_derivation_hash: bundle.child_derivation_hash().to_owned(),
            wrapper_derivation_hash: bundle.derivation_hash().to_owned(),
        })
        .collect::<Vec<_>>();

    let mut certificate = TruncEndpointRealizerCertificate {
        schema: TRUNC_ENDPOINT_ARTIFACT_SCHEMA.to_owned(),
        date: TRUNC_ENDPOINT_ARTIFACT_DATE.to_owned(),
        versions: VersionPins {
            endpoint_fragment: TRUNC_ENDPOINT_REALIZER_FRAGMENT_VERSION.to_owned(),
            boundary_axiom_v3: ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION.to_owned(),
            element_overlay: HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION.to_owned(),
            boundary_charge_policy: BOUNDARY_CHARGE_POLICY_VERSION.to_owned(),
            handoff_api: HISTORICAL_PREFIX_V3_C6_TYPED_HANDOFF_VERSION.to_owned(),
        },
        source: SourceAudit {
            step: full.step(),
            kind: "trunc".to_owned(),
            dimension: full.dimension(),
            full_h15_signature_digest: full.signature_digest().to_owned(),
            exact_b5_signature_digest: historical.signature_digest().to_owned(),
            candidate_hash: full.candidate_hash().to_owned(),
            full_telescope_digest: full.telescope_digest().to_owned(),
            prefix_telescope_digest: historical.telescope_digest().to_owned(),
            full_typing_derivation_hash: full.typing_derivation_hash().to_owned(),
            prefix_typing_derivation_hash: historical.typing_derivation_hash().to_owned(),
        },
        declared_boundary: DeclaredBoundaryAudit {
            context_label: "A:Type; x,y:Trunc(A)".to_owned(),
            full_typed_boundary_derivation_hash: full.typed_boundary_derivation_hash().to_owned(),
            prefix_typed_boundary_derivation_hash: historical
                .typed_boundary_derivation_hash()
                .to_owned(),
            full_boundary_binding_digest: full.boundary_binding_digest().to_owned(),
            prefix_boundary_binding_digest: historical.boundary_binding_digest().to_owned(),
            endpoints,
            exact_x_y_distinction,
            overlay_entry_used: false,
        },
        method: MethodAudit {
            shape: "PathP(i -> P(squash(x,y,i)), method(x), method(y))".to_owned(),
            formal_schema_premises_explicit: true,
            context_free_endpoint_hypotheses_rejected: true,
            premise_source_digest_full: full.endpoint_premise_source_digest().to_owned(),
            premise_source_digest_prefix: historical.endpoint_premise_source_digest().to_owned(),
            premise_context_derivation_hash_full: full
                .endpoint_premise_context_derivation_hash()
                .to_owned(),
            premise_context_derivation_hash_prefix: historical
                .endpoint_premise_context_derivation_hash()
                .to_owned(),
            method_shape_digest_full: full.method_shape_digest().to_owned(),
            method_shape_digest_prefix: historical.method_shape_digest().to_owned(),
            computation_audit_derivation_hash_full: full
                .computation_audit_derivation_hash()
                .to_owned(),
            computation_audit_derivation_hash_prefix: historical
                .computation_audit_derivation_hash()
                .to_owned(),
            zero_restriction_computes_to_method_image: true,
            one_restriction_computes_to_method_image: true,
            zero_scrutinee_computes_by_endpoint_instantiation: true,
            one_scrutinee_computes_by_endpoint_instantiation: true,
            constructor_scrutinee_computes_to_dependent_method: true,
            typed_neutral_stays_stuck_at_motive_instance: true,
            boundary_aware_coe_targets_right_endpoint: true,
        },
        scopes: vec![scope_audit(&full), scope_audit(&historical)],
        restricted_instantiation: RestrictedInstantiationAudit {
            theorem_scope: restricted.theorem_scope().to_owned(),
            owner_identity_derivation_hash: restricted
                .owner_substitution_derivation_hash()
                .to_owned(),
            zero_endpoint_identity_derivation_hash: restricted
                .zero_endpoint_substitution_derivation_hash()
                .to_owned(),
            one_endpoint_identity_derivation_hash: restricted
                .one_endpoint_substitution_derivation_hash()
                .to_owned(),
            endpoint_swap_derivation_hash: restricted
                .endpoint_swap_substitution_derivation_hash()
                .to_owned(),
            all_images_are_sort_identical_variables: restricted
                .all_sort_identical_variable_images_covered(),
            all_four_fixed_context_maps_replayed: restricted_variable_instances.len() == 4
                && restricted_variable_instances
                    .iter()
                    .map(|instance| instance.kind.as_str())
                    .eq(["identity", "swap", "collapse_to_x", "collapse_to_y"]),
            variable_instances: restricted_variable_instances,
            arbitrary_typed_images_used: restricted.arbitrary_typed_images_used(),
            inherited_arbitrary_typed_image_gap: restricted
                .arbitrary_typed_instance_gap()
                .to_owned(),
        },
        agent_a_handoff: AgentAHandoffAudit {
            api_version: HISTORICAL_PREFIX_V3_C6_TYPED_HANDOFF_VERSION.to_owned(),
            registered_counts: handoff.registered_counts().to_vec(),
            rows,
            all_replayed: handoff.all_replayed(),
            derivation_hash: handoff.derivation_hash().to_owned(),
        },
        archival_bindings: archival_bindings()?,
        falsifiers: FalsifierAudit {
            f_tr1_registered_computation_realizable_under_formal_method_premises: true,
            f_tr2_exact_archival_bytes_unchanged: true,
            f_tr2_definition_replay_external_gate_required: true,
            definition_replay_proved_by_this_certificate: false,
            f_tr3_exactly_two_exported_keys: true,
            f_tr4_arbitrary_typed_images_unused_and_gap_named: true,
        },
        conditionality: ConditionalityAudit {
            conditional_on_boundary_axiom_v3: true,
            judgement_is_under_formal_eliminator_method_premises: true,
            closed_term_without_method_premises_proved: false,
            trunc_element_overlay_entry_required: false,
            registered_trunc_c6_endpoint_realizer_proved: true,
            general_c6_proved: false,
            basis_independence_proved: false,
            intended_basis_exhaustiveness_proved: false,
            candidate_level_c8_proved: false,
            intended_schema_classification_proved: false,
            step16_or_global_halt_proved: false,
        },
        obstruction_retired: C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP.to_owned(),
        remaining_obligations: vec![
            GENERAL_C6_GAP.to_owned(),
            C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION_GAP.to_owned(),
            C8_GAP.to_owned(),
            INTENDED_SCHEMA_GAP.to_owned(),
            F_B2_GAP.to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn replay_trunc_endpoint_realizer_certificate(
    certificate: &TruncEndpointRealizerCertificate,
) -> TruncEndpointReplay {
    let expected = match build_trunc_endpoint_realizer_certificate() {
        Ok(expected) => expected,
        Err(error) => {
            return TruncEndpointReplay {
                valid: false,
                full_h15_replayed: false,
                exact_b5_replayed: false,
                exact_two_key_basis: false,
                all_four_handoff_counts_2_2_5_10: false,
                archival_bytes_pinned: false,
                general_c6_proved: false,
                errors: vec![error.to_string()],
            };
        }
    };
    replay_trunc_endpoint_realizer_certificate_against(certificate, &expected)
}

fn replay_trunc_endpoint_realizer_certificate_against(
    certificate: &TruncEndpointRealizerCertificate,
    expected: &TruncEndpointRealizerCertificate,
) -> TruncEndpointReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    let full_h15_replayed = certificate
        .scopes
        .iter()
        .any(|scope| scope.scope == "full_h15" && scope.replayed);
    let exact_b5_replayed = certificate
        .scopes
        .iter()
        .any(|scope| scope.scope == "historical_prefix_b5" && scope.replayed);
    let exact_two_key_basis = certificate.scopes.len() == 2
        && certificate
            .scopes
            .iter()
            .all(|scope| scope.exact_key_set && scope.realized_basis_count == 2);
    let all_four_handoff_counts_2_2_5_10 = certificate.agent_a_handoff.registered_counts
        == [2, 2, 5, 10]
        && certificate.agent_a_handoff.all_replayed;
    let archival_bytes_pinned = certificate
        .archival_bindings
        .iter()
        .all(|binding| binding.exact_bytes_pinned);
    TruncEndpointReplay {
        valid: errors.is_empty(),
        full_h15_replayed,
        exact_b5_replayed,
        exact_two_key_basis,
        all_four_handoff_counts_2_2_5_10,
        archival_bytes_pinned,
        general_c6_proved: certificate.conditionality.general_c6_proved,
        errors,
    }
}

pub fn trunc_endpoint_realizer_json_pretty() -> Result<String, TruncEndpointArtifactError> {
    serde_json::to_string_pretty(&build_trunc_endpoint_realizer_certificate()?)
        .map(|json| format!("{json}\n"))
        .map_err(|error| TruncEndpointArtifactError::Json(error.to_string()))
}

pub fn replay_trunc_endpoint_realizer_json(json: &str) -> TruncEndpointReplay {
    let raw_value: serde_json::Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(error) => {
            return TruncEndpointReplay {
                valid: false,
                full_h15_replayed: false,
                exact_b5_replayed: false,
                exact_two_key_basis: false,
                all_four_handoff_counts_2_2_5_10: false,
                archival_bytes_pinned: false,
                general_c6_proved: false,
                errors: vec![format!("invalid JSON: {error}")],
            };
        }
    };
    let certificate: TruncEndpointRealizerCertificate = match serde_json::from_str(json) {
        Ok(certificate) => certificate,
        Err(error) => {
            return TruncEndpointReplay {
                valid: false,
                full_h15_replayed: false,
                exact_b5_replayed: false,
                exact_two_key_basis: false,
                all_four_handoff_counts_2_2_5_10: false,
                archival_bytes_pinned: false,
                general_c6_proved: false,
                errors: vec![format!("certificate shape error: {error}")],
            };
        }
    };
    let typed_value = serde_json::to_value(&certificate).expect("certificate serializes");
    if raw_value != typed_value {
        return TruncEndpointReplay {
            valid: false,
            full_h15_replayed: false,
            exact_b5_replayed: false,
            exact_two_key_basis: false,
            all_four_handoff_counts_2_2_5_10: false,
            archival_bytes_pinned: false,
            general_c6_proved: false,
            errors: vec!["JSON contains unknown or ignored structure".to_owned()],
        };
    }
    replay_trunc_endpoint_realizer_certificate(&certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ValuePathItem {
        Field(String),
        Index(usize),
    }

    fn redigest(certificate: &mut TruncEndpointRealizerCertificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    fn collect_value_paths(
        value: &serde_json::Value,
        path: &mut Vec<ValuePathItem>,
        leaf_paths: &mut Vec<Vec<ValuePathItem>>,
        array_paths: &mut Vec<Vec<ValuePathItem>>,
    ) {
        match value {
            serde_json::Value::Object(object) => {
                for (field, child) in object {
                    path.push(ValuePathItem::Field(field.clone()));
                    collect_value_paths(child, path, leaf_paths, array_paths);
                    path.pop();
                }
            }
            serde_json::Value::Array(array) => {
                array_paths.push(path.clone());
                for (index, child) in array.iter().enumerate() {
                    path.push(ValuePathItem::Index(index));
                    collect_value_paths(child, path, leaf_paths, array_paths);
                    path.pop();
                }
            }
            _ => leaf_paths.push(path.clone()),
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

    fn assert_redigested_value_rejected(
        value: serde_json::Value,
        expected: &TruncEndpointRealizerCertificate,
        label: &str,
    ) {
        let Ok(mut mutated): Result<TruncEndpointRealizerCertificate, _> =
            serde_json::from_value(value)
        else {
            // Shape rejection is stronger than a definition-replay failure
            // for structural mutations such as changing a fixed-size tuple.
            return;
        };
        redigest(&mut mutated);
        assert!(
            !replay_trunc_endpoint_realizer_certificate_against(&mutated, expected).valid,
            "redigested mutation at {label} must fail definition replay"
        );
    }

    #[test]
    fn certificate_replays_with_exact_scoped_result() {
        let certificate = build_trunc_endpoint_realizer_certificate().expect("certificate");
        let replay = replay_trunc_endpoint_realizer_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(replay.full_h15_replayed);
        assert!(replay.exact_b5_replayed);
        assert!(replay.exact_two_key_basis);
        assert!(replay.all_four_handoff_counts_2_2_5_10);
        assert!(replay.archival_bytes_pinned);
        assert!(!replay.general_c6_proved);
        assert_eq!(certificate.agent_a_handoff.registered_counts, [2, 2, 5, 10]);
        assert_eq!(certificate.scopes.len(), 2);
        assert_eq!(certificate.scopes[0].path_realizations.len(), 2);
        assert_eq!(certificate.scopes[1].path_realizations.len(), 2);
    }

    #[test]
    fn every_scalar_projection_mutation_fails_even_after_redigest() {
        let original = build_trunc_endpoint_realizer_certificate().expect("certificate");
        let original_value = serde_json::to_value(&original).expect("certificate projects");
        let mut leaf_paths = Vec::new();
        let mut array_paths = Vec::new();
        collect_value_paths(
            &original_value,
            &mut Vec::new(),
            &mut leaf_paths,
            &mut array_paths,
        );
        let result_digest_path = [ValuePathItem::Field("result_digest".to_owned())];
        for path in leaf_paths {
            if path == result_digest_path {
                continue;
            }
            let mut mutated_value = original_value.clone();
            let leaf = value_at_path_mut(&mut mutated_value, &path);
            match leaf {
                serde_json::Value::Bool(value) => *value = !*value,
                serde_json::Value::Number(value) => {
                    let value = value.as_u64().expect("certificate numbers are unsigned");
                    *leaf = serde_json::Value::from(value + 1);
                }
                serde_json::Value::String(value) => value.push_str(":mutated"),
                serde_json::Value::Null
                | serde_json::Value::Array(_)
                | serde_json::Value::Object(_) => unreachable!("recorded path is a scalar leaf"),
            }
            assert_redigested_value_rejected(mutated_value, &original, &path_label(&path));
        }

        let mut bad_outer_digest = original;
        bad_outer_digest.result_digest.push_str(":mutated");
        assert!(!replay_trunc_endpoint_realizer_certificate(&bad_outer_digest).valid);
    }

    #[test]
    fn every_vector_structure_mutation_fails_even_after_redigest() {
        let original = build_trunc_endpoint_realizer_certificate().expect("certificate");
        let original_value = serde_json::to_value(&original).expect("certificate projects");
        let mut leaf_paths = Vec::new();
        let mut array_paths = Vec::new();
        collect_value_paths(
            &original_value,
            &mut Vec::new(),
            &mut leaf_paths,
            &mut array_paths,
        );
        for path in array_paths {
            let original_array = value_at_path_mut(&mut original_value.clone(), &path)
                .as_array()
                .expect("recorded path is an array")
                .clone();
            if original_array.is_empty() {
                continue;
            }
            let label = path_label(&path);

            let mut deleted = original_value.clone();
            value_at_path_mut(&mut deleted, &path)
                .as_array_mut()
                .expect("array")
                .remove(0);
            assert_redigested_value_rejected(deleted, &original, &format!("{label}:delete"));

            let mut duplicated = original_value.clone();
            value_at_path_mut(&mut duplicated, &path)
                .as_array_mut()
                .expect("array")
                .push(original_array[0].clone());
            assert_redigested_value_rejected(duplicated, &original, &format!("{label}:duplicate"));

            if original_array.len() > 1 {
                let mut reordered = original_value.clone();
                let array = value_at_path_mut(&mut reordered, &path)
                    .as_array_mut()
                    .expect("array");
                array.reverse();
                if array != &original_array {
                    assert_redigested_value_rejected(
                        reordered,
                        &original,
                        &format!("{label}:reorder"),
                    );
                }
            }
        }
    }

    #[test]
    fn strict_json_rejects_unknown_and_duplicate_fields_at_all_depths() {
        let json = trunc_endpoint_realizer_json_pretty().expect("json");
        assert!(replay_trunc_endpoint_realizer_json(&json).valid);

        let unknown_top = json.replacen("{\n", "{\n  \"unknown_top\": true,\n", 1);
        assert!(!replay_trunc_endpoint_realizer_json(&unknown_top).valid);

        let unknown_deep = json.replacen(
            "\"versions\": {",
            "\"versions\": {\n    \"unknown_deep\": true,",
            1,
        );
        assert!(!replay_trunc_endpoint_realizer_json(&unknown_deep).valid);

        let unknown_vector_row = json.replacen(
            "\"path_realizations\": [\n        {",
            "\"path_realizations\": [\n        {\n          \"unknown_row_field\": true,",
            1,
        );
        assert!(!replay_trunc_endpoint_realizer_json(&unknown_vector_row).valid);

        let duplicate_top = json.replacen(
            "\"schema\": \"trunc-endpoint-realizer-v1\",",
            "\"schema\": \"trunc-endpoint-realizer-v1\",\n  \"schema\": \"trunc-endpoint-realizer-v1\",",
            1,
        );
        assert!(!replay_trunc_endpoint_realizer_json(&duplicate_top).valid);

        let duplicate_deep = json.replacen(
            "\"endpoint_fragment\": \"tdc1-trunc-endpoint-dependent-pathcon-realizer-v1\",",
            "\"endpoint_fragment\": \"tdc1-trunc-endpoint-dependent-pathcon-realizer-v1\",\n    \"endpoint_fragment\": \"tdc1-trunc-endpoint-dependent-pathcon-realizer-v1\",",
            1,
        );
        assert!(!replay_trunc_endpoint_realizer_json(&duplicate_deep).valid);

        let duplicate_vector_row = json.replacen(
            "\"term_hash\": ",
            "\"term_hash\": \"duplicate\",\n          \"term_hash\": ",
            1,
        );
        assert!(!replay_trunc_endpoint_realizer_json(&duplicate_vector_row).valid);
    }
}
