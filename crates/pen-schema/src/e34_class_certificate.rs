//! Create-new certificate for the full operational class-indexed E-3
//! normalization/naturality pass and dependent public cubical-action induction.
//!
//! The certificate is deliberately narrower than global intended-Schema2
//! completeness.  It proves all nine constructors currently registered by the
//! operational grammar and all eight cubical constructors accepted by the
//! public typing context.  It fails closed at the four private endpoint-premise
//! forms and at the missing modal, synthesis, and raw-candidate grammar.

use crate::e4_generator_basis::Schema2NaturalityClass;
use crate::e34_certificate::{
    SCHEMA2_E34_CERTIFICATE_SCHEMA, Schema2E34Certificate, replay_schema2_e34_json,
};
use crate::e34_class_induction::{
    CLASS_INDUCTION_ATTEMPT_VERSION, CubicalConstructorKind, INTENDED_SCHEMA2_GRAMMAR_GAP,
    issue_class_indexed_e3_e4_attempt, replay_class_indexed_e3_e4_attempt,
};
use crate::grammar::OrdinarySchemaKind;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const SCHEMA2_E34_CLASS_CERTIFICATE_SCHEMA: &str = "schema2-e3-e4-class-induction-successor-v4";
pub const SCHEMA2_E34_CLASS_CERTIFICATE_DATE: &str = "2026-07-20";

const PREDECESSOR_BYTES: &[u8] = include_bytes!("../../../docs/schema2_v3.json");
const P1_BYTES: &[u8] = include_bytes!("../../../docs/e2_phase_order_adjudication.md");
const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_schema2_plan.md");
const SPEC_BYTES: &[u8] = include_bytes!("../../../docs/step_15_completion_open_problem.md");
const CLASS_INDUCTION_BYTES: &[u8] = include_bytes!("e34_class_induction.rs");
const CLASS_INDUCTION_AGDA_BYTES: &[u8] =
    include_bytes!("../../../agda/Schema2E3E4ClassInduction.agda");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ClassCertificateError {
    #[error("predecessor replay failed: {0}")]
    Predecessor(String),
    #[error("class induction failed: {0}")]
    Induction(String),
    #[error("Agda source boundary failed: {0}")]
    Agda(String),
    #[error("certificate invariant failed: {0}")]
    Invariant(String),
    #[error("invalid UTF-8 in {0}")]
    Utf8(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClassSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClassPredecessorEvidence {
    pub path: String,
    pub schema: String,
    pub result_digest: String,
    pub strict_replay_valid: bool,
    pub e3_global_complete: bool,
    pub e4_global_complete: bool,
    pub e2b_executed: bool,
    pub global_halt_proved: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrdinaryClassInductionEvidence {
    pub version: String,
    pub aggregate_derivation_hash: String,
    pub registered_constructor_count: usize,
    pub registered_constructors: Vec<String>,
    pub inventory_exhaustive_for_operational_registry: bool,
    pub every_constructor_normalized_and_natural: bool,
    pub genuine_typed_substitution_used: bool,
    pub source_and_target_normal_forms_rechecked: bool,
    pub presentation_disposition_and_support_preserved: bool,
    pub intended_schema2_inventory_exhaustive: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CubicalClassInductionEvidence {
    pub exemplar_count: usize,
    pub enum_constructor_count: usize,
    pub public_constructor_count: usize,
    pub private_endpoint_premise_constructor_count: usize,
    pub observed_public_constructors: Vec<String>,
    pub recursive_match_exhaustive_over_kernel_enum: bool,
    pub public_constructor_inventory_exhaustive: bool,
    pub typing_preserved: bool,
    pub normalization_replay_stable: bool,
    pub dimension_substitution_witness_count: usize,
    pub every_checked_dimension_action_natural: bool,
    pub coe_node_count: usize,
    pub hcom_node_count: usize,
    pub typed_tube_face_count: usize,
    pub checked_overlap_count: usize,
    pub registered_trunc_endpoint_replayed: bool,
    pub registered_trunc_basis_count: u64,
    pub generic_endpoint_premise_induction_complete: bool,
    pub registered_bundle_e1_naturality_complete: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PerClassInductionEvidence {
    pub class: String,
    pub registered_ordinary_constructors: Vec<String>,
    pub every_registered_constructor_normalized_and_natural: bool,
    pub public_dependent_cubical_induction_available: bool,
    pub intended_constructor_inventory_exhaustive: bool,
    pub complete: bool,
    pub named_gaps: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClassAgdaEvidence {
    pub source_bound: bool,
    pub safe_without_k_declared: bool,
    pub postulates_absent: bool,
    pub nine_constructor_naturality_induction_declared: bool,
    pub class_indexed_theorem_declared: bool,
    pub dependent_cubical_action_induction_declared: bool,
    pub coe_and_hcom_present: bool,
    pub absent_modal_and_synthesis_constructors_proved: bool,
    pub endpoint_premise_api_left_empty: bool,
    pub rust_agda_operational_correspondence_proved: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClassForbiddenOutputs {
    pub independent_verdict_issued: bool,
    pub ordinary_family_token_issued: bool,
    pub stage_count_issued: bool,
    pub e2b_executed: bool,
    pub global_e3_claimed: bool,
    pub global_e4_claimed: bool,
    pub global_halt_proved: bool,
}

impl ClassForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.independent_verdict_issued
            && !self.ordinary_family_token_issued
            && !self.stage_count_issued
            && !self.e2b_executed
            && !self.global_e3_claimed
            && !self.global_e4_claimed
            && !self.global_halt_proved
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schema2E34ClassCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<ClassSourceBinding>,
    pub predecessor: ClassPredecessorEvidence,
    pub ordinary: OrdinaryClassInductionEvidence,
    pub cubical: CubicalClassInductionEvidence,
    pub per_class: Vec<PerClassInductionEvidence>,
    pub agda: ClassAgdaEvidence,
    pub every_class_complete: bool,
    pub full_intended_schema2_e3_complete: bool,
    pub full_e4_complete: bool,
    pub forbidden_outputs: ClassForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schema2E34ClassReplay {
    pub valid: bool,
    pub predecessor_replayed: bool,
    pub ordinary_constructor_count: usize,
    pub ordinary_registry_complete: bool,
    pub public_cubical_constructor_count: usize,
    pub public_cubical_induction_complete: bool,
    pub private_endpoint_premise_constructor_count: usize,
    pub every_class_complete: bool,
    pub full_e4_complete: bool,
    pub e2b_executed: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

fn utf8<'a>(path: &str, bytes: &'a [u8]) -> Result<&'a str, ClassCertificateError> {
    std::str::from_utf8(bytes).map_err(|_| ClassCertificateError::Utf8(path.to_owned()))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(SCHEMA2_E34_CLASS_CERTIFICATE_SCHEMA, domain, value))
        .expect("class certificate data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn source_bindings() -> Vec<ClassSourceBinding> {
    [
        (
            "docs/schema2_v3.json",
            "strict_byte_pinned_e3_e4_predecessor",
            PREDECESSOR_BYTES,
        ),
        (
            "docs/e2_phase_order_adjudication.md",
            "adopted_p1_phase_and_output_boundary",
            P1_BYTES,
        ),
        (
            "docs/agent_e_schema2_plan.md",
            "governing_e3_e4_execution_plan",
            PLAN_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "frozen_intended_schema2_specification",
            SPEC_BYTES,
        ),
        (
            "crates/pen-schema/src/e34_class_induction.rs",
            "operational_class_induction_issuer",
            CLASS_INDUCTION_BYTES,
        ),
        (
            "agda/Schema2E3E4ClassInduction.agda",
            "safe_constructive_proof_mirror",
            CLASS_INDUCTION_AGDA_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| ClassSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn ordinary_name(kind: OrdinarySchemaKind) -> &'static str {
    match kind {
        OrdinarySchemaKind::FreshFormation => "fresh_formation",
        OrdinarySchemaKind::PointOrUnitIntro => "point_or_unit_intro",
        OrdinarySchemaKind::PathConstructorIntro => "path_constructor_intro",
        OrdinarySchemaKind::Recursor => "recursor",
        OrdinarySchemaKind::Inductor => "inductor",
        OrdinarySchemaKind::TruncParametricAction => "trunc_parametric_action",
        OrdinarySchemaKind::PostPathOperation => "post_path_operation",
        OrdinarySchemaKind::PostPathCoherence => "post_path_coherence",
        OrdinarySchemaKind::CellAction => "cell_action",
    }
}

fn cubical_name(kind: CubicalConstructorKind) -> &'static str {
    match kind {
        CubicalConstructorKind::Point => "point",
        CubicalConstructorKind::MotiveBase => "motive_base",
        CubicalConstructorKind::EndpointEvaluationHypothesis => "endpoint_evaluation_hypothesis",
        CubicalConstructorKind::PathMethod => "path_method",
        CubicalConstructorKind::EndpointMethodHypothesis => "endpoint_method_hypothesis",
        CubicalConstructorKind::PathElim => "path_elim",
        CubicalConstructorKind::EndpointPathElim => "endpoint_path_elim",
        CubicalConstructorKind::EndpointElimNeutral => "endpoint_elim_neutral",
        CubicalConstructorKind::DimLambda => "dim_lambda",
        CubicalConstructorKind::DimApp => "dim_app",
        CubicalConstructorKind::Coe => "coe",
        CubicalConstructorKind::Hcom => "hcom",
    }
}

fn class_name(class: Schema2NaturalityClass) -> &'static str {
    match class {
        Schema2NaturalityClass::Foundation => "foundation",
        Schema2NaturalityClass::Former => "former",
        Schema2NaturalityClass::Map => "map",
        Schema2NaturalityClass::Axiomatic => "axiomatic",
        Schema2NaturalityClass::Modal => "modal",
        Schema2NaturalityClass::HitV2 => "hit_v2",
        Schema2NaturalityClass::Synthesis => "synthesis",
        Schema2NaturalityClass::Unknown => "unknown",
    }
}

fn validate_predecessor() -> Result<ClassPredecessorEvidence, ClassCertificateError> {
    let json = utf8("docs/schema2_v3.json", PREDECESSOR_BYTES)?;
    let replay = replay_schema2_e34_json(json);
    if !replay.valid {
        return Err(ClassCertificateError::Predecessor(replay.errors.join("; ")));
    }
    let predecessor: Schema2E34Certificate = serde_json::from_str(json)
        .map_err(|error| ClassCertificateError::Predecessor(error.to_string()))?;
    if predecessor.schema != SCHEMA2_E34_CERTIFICATE_SCHEMA
        || predecessor.e3_global_complete
        || predecessor.e4_global_complete
        || predecessor.e2b_executed
        || predecessor.forbidden_outputs.global_halt_proved
    {
        return Err(ClassCertificateError::Predecessor(
            "v3 predecessor crossed its sealed partial boundary".to_owned(),
        ));
    }
    Ok(ClassPredecessorEvidence {
        path: "docs/schema2_v3.json".to_owned(),
        schema: predecessor.schema,
        result_digest: predecessor.result_digest,
        strict_replay_valid: true,
        e3_global_complete: false,
        e4_global_complete: false,
        e2b_executed: false,
        global_halt_proved: false,
    })
}

fn validate_agda() -> Result<ClassAgdaEvidence, ClassCertificateError> {
    let source = utf8(
        "agda/Schema2E3E4ClassInduction.agda",
        CLASS_INDUCTION_AGDA_BYTES,
    )?;
    let markers = [
        "ordinary-normalization-naturality",
        "full-registered-class-indexed-naturality",
        "normalize-cubical-action",
        "dependent-public-cubical-constructor-induction",
        "c-coe",
        "c-hcom",
        "no-modal-registered-normal",
        "no-synthesis-registered-normal",
        "data PublicEndpointPremiseContext : Set where",
    ];
    if !source.starts_with("{-# OPTIONS --safe --without-K #-}")
        || source
            .lines()
            .any(|line| line.trim_start().starts_with("postulate"))
        || markers.iter().any(|marker| !source.contains(marker))
    {
        return Err(ClassCertificateError::Agda(
            "safe/no-postulate declaration or required induction marker missing".to_owned(),
        ));
    }
    Ok(ClassAgdaEvidence {
        source_bound: true,
        safe_without_k_declared: true,
        postulates_absent: true,
        nine_constructor_naturality_induction_declared: true,
        class_indexed_theorem_declared: true,
        dependent_cubical_action_induction_declared: true,
        coe_and_hcom_present: true,
        absent_modal_and_synthesis_constructors_proved: true,
        endpoint_premise_api_left_empty: true,
        rust_agda_operational_correspondence_proved: false,
    })
}

fn certificate_digest(certificate: &Schema2E34ClassCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("class-induction-successor-certificate", &projection)
}

pub fn build_schema2_e34_class_certificate()
-> Result<Schema2E34ClassCertificate, ClassCertificateError> {
    let predecessor = validate_predecessor()?;
    let attempt = issue_class_indexed_e3_e4_attempt()
        .map_err(|error| ClassCertificateError::Induction(error.to_string()))?;
    replay_class_indexed_e3_e4_attempt(&attempt)
        .map_err(|error| ClassCertificateError::Induction(error.to_string()))?;

    let registered_constructors = attempt
        .ordinary_naturality()
        .iter()
        .map(|token| ordinary_name(token.constructor()).to_owned())
        .collect::<Vec<_>>();
    let ordinary = OrdinaryClassInductionEvidence {
        version: attempt.version().to_owned(),
        aggregate_derivation_hash: attempt.derivation_hash().to_owned(),
        registered_constructor_count: registered_constructors.len(),
        registered_constructors,
        inventory_exhaustive_for_operational_registry: attempt
            .registered_ordinary_inventory_exhaustive(),
        every_constructor_normalized_and_natural: attempt
            .every_registered_ordinary_constructor_natural(),
        genuine_typed_substitution_used: true,
        source_and_target_normal_forms_rechecked: true,
        presentation_disposition_and_support_preserved: true,
        intended_schema2_inventory_exhaustive: false,
    };

    let observed = attempt
        .cubical_inductions()
        .iter()
        .flat_map(|token| token.observed_constructors().iter().copied())
        .collect::<BTreeSet<_>>();
    let cubical = CubicalClassInductionEvidence {
        exemplar_count: attempt.cubical_inductions().len(),
        enum_constructor_count: CubicalConstructorKind::ALL.len(),
        public_constructor_count: observed.len(),
        private_endpoint_premise_constructor_count: CubicalConstructorKind::ALL.len()
            - observed.len(),
        observed_public_constructors: observed
            .iter()
            .copied()
            .map(|kind| cubical_name(kind).to_owned())
            .collect(),
        recursive_match_exhaustive_over_kernel_enum: attempt
            .cubical_inductions()
            .iter()
            .all(|token| token.structural_match_exhaustive_over_enum()),
        public_constructor_inventory_exhaustive: attempt
            .public_cubical_constructor_match_exhaustive(),
        typing_preserved: attempt
            .cubical_inductions()
            .iter()
            .all(|token| token.typing_preserved()),
        normalization_replay_stable: attempt
            .cubical_inductions()
            .iter()
            .all(|token| token.normalization_stable()),
        dimension_substitution_witness_count: attempt
            .cubical_inductions()
            .iter()
            .map(|token| token.substitution_witness_count())
            .sum(),
        every_checked_dimension_action_natural: attempt.public_cubical_normalization_natural(),
        coe_node_count: attempt
            .cubical_inductions()
            .iter()
            .map(|token| token.coe_node_count())
            .sum(),
        hcom_node_count: attempt
            .cubical_inductions()
            .iter()
            .map(|token| token.hcom_node_count())
            .sum(),
        typed_tube_face_count: attempt
            .cubical_inductions()
            .iter()
            .map(|token| token.typed_tube_face_count())
            .sum(),
        checked_overlap_count: attempt
            .cubical_inductions()
            .iter()
            .map(|token| token.checked_overlap_count())
            .sum(),
        registered_trunc_endpoint_replayed: attempt.registered_trunc_endpoint_replayed(),
        registered_trunc_basis_count: attempt.registered_trunc_basis_count(),
        generic_endpoint_premise_induction_complete: attempt
            .generic_endpoint_premise_induction_complete(),
        registered_bundle_e1_naturality_complete: attempt
            .registered_bundle_e1_naturality_complete(),
    };

    let per_class = attempt
        .per_class()
        .iter()
        .map(|record| PerClassInductionEvidence {
            class: class_name(record.class()).to_owned(),
            registered_ordinary_constructors: record
                .registered_ordinary_constructors()
                .iter()
                .copied()
                .map(|kind| ordinary_name(kind).to_owned())
                .collect(),
            every_registered_constructor_normalized_and_natural: record
                .every_registered_constructor_normalized_and_natural(),
            public_dependent_cubical_induction_available: record
                .public_dependent_cubical_induction_available(),
            intended_constructor_inventory_exhaustive: record
                .intended_constructor_inventory_exhaustive(),
            complete: record.complete(),
            named_gaps: record.named_gaps().to_vec(),
        })
        .collect::<Vec<_>>();
    let agda = validate_agda()?;
    let forbidden_outputs = ClassForbiddenOutputs {
        independent_verdict_issued: attempt.independent_verdict_issued(),
        ordinary_family_token_issued: attempt.ordinary_family_token_issued(),
        stage_count_issued: attempt.stage_count_issued(),
        e2b_executed: attempt.e2b_executed(),
        global_e3_claimed: attempt.full_intended_schema2_e3_complete(),
        global_e4_claimed: attempt.full_e4_complete(),
        global_halt_proved: attempt.global_halt_proved(),
    };

    if attempt.version() != CLASS_INDUCTION_ATTEMPT_VERSION
        || ordinary.registered_constructor_count != OrdinarySchemaKind::ALL.len()
        || ordinary.registered_constructor_count != 9
        || !ordinary.inventory_exhaustive_for_operational_registry
        || !ordinary.every_constructor_normalized_and_natural
        || cubical.enum_constructor_count != 12
        || cubical.public_constructor_count != 8
        || cubical.private_endpoint_premise_constructor_count != 4
        || !cubical.recursive_match_exhaustive_over_kernel_enum
        || !cubical.public_constructor_inventory_exhaustive
        || !cubical.typing_preserved
        || !cubical.normalization_replay_stable
        || !cubical.every_checked_dimension_action_natural
        || cubical.coe_node_count == 0
        || cubical.hcom_node_count == 0
        || cubical.typed_tube_face_count == 0
        || cubical.checked_overlap_count == 0
        || !cubical.registered_trunc_endpoint_replayed
        || cubical.registered_trunc_basis_count != 2
        || cubical.generic_endpoint_premise_induction_complete
        || cubical.registered_bundle_e1_naturality_complete
        || per_class.len() != Schema2NaturalityClass::ALL.len()
        || per_class.iter().any(|record| record.complete)
        || attempt.every_class_complete()
        || attempt.full_intended_schema2_e3_complete()
        || attempt.full_e4_complete()
        || attempt.global_gap() != INTENDED_SCHEMA2_GRAMMAR_GAP
        || !forbidden_outputs.all_withheld()
    {
        return Err(ClassCertificateError::Invariant(
            "class-induction result crossed or failed its operational boundary".to_owned(),
        ));
    }

    let mut certificate = Schema2E34ClassCertificate {
        schema: SCHEMA2_E34_CLASS_CERTIFICATE_SCHEMA.to_owned(),
        date: SCHEMA2_E34_CLASS_CERTIFICATE_DATE.to_owned(),
        source_bindings: source_bindings(),
        predecessor,
        ordinary,
        cubical,
        per_class,
        agda,
        every_class_complete: false,
        full_intended_schema2_e3_complete: false,
        full_e4_complete: false,
        forbidden_outputs,
        outcome: "full_operational_nine_constructor_e3_naturality_and_public_dependent_cubical_action_induction_complete_global_intended_schema2_still_partial".to_owned(),
        permitted_conclusion: "Every constructor in the nine-kind operational ordinary registry is normalization-natural under a genuine typed ambient substitution, and all eight constructors admitted by the public cubical context are covered by typed dimension-action induction, including dependent coe and hcom. This does not cover the four private endpoint-premise forms or supply missing modal, synthesis, map, axiomatic, unknown-class, and raw-candidate grammar; therefore global E-3/E-4, Independent verdicts, E-2b, counts, and halt remain unavailable.".to_owned(),
        remaining_obligations: vec![
            "publish a generic motive-typed endpoint-premise context and prove normalization/action induction for endpoint evaluation hypothesis, endpoint method hypothesis, endpoint PathCon elimination, and endpoint neutral elimination".to_owned(),
            "prove transport/naturality of registered boundary bundles under arbitrary legal E-1 substitutions rather than replaying only the sealed Trunc endpoint bundle".to_owned(),
            "adjudicate and implement typed modal and synthesis schema constructors plus map natural transformations, exported support action, axiomatic adjoint/mate/interchange, and unknown-row classification".to_owned(),
            "prove that the intended raw candidate grammar is exhausted by the class-indexed typed constructors; only then may E-4 completeness and Independent verdict issuance be enabled".to_owned(),
            "after complete E-4, execute E-2b and its count regression in the P1 order".to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> Schema2E34ClassReplay {
    Schema2E34ClassReplay {
        valid: false,
        predecessor_replayed: false,
        ordinary_constructor_count: 0,
        ordinary_registry_complete: false,
        public_cubical_constructor_count: 0,
        public_cubical_induction_complete: false,
        private_endpoint_premise_constructor_count: 0,
        every_class_complete: false,
        full_e4_complete: false,
        e2b_executed: false,
        forbidden_outputs_withheld: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against(
    certificate: &Schema2E34ClassCertificate,
    expected: &Schema2E34ClassCertificate,
) -> Schema2E34ClassReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    Schema2E34ClassReplay {
        valid: errors.is_empty(),
        predecessor_replayed: certificate.predecessor.strict_replay_valid,
        ordinary_constructor_count: certificate.ordinary.registered_constructor_count,
        ordinary_registry_complete: certificate
            .ordinary
            .inventory_exhaustive_for_operational_registry
            && certificate
                .ordinary
                .every_constructor_normalized_and_natural,
        public_cubical_constructor_count: certificate.cubical.public_constructor_count,
        public_cubical_induction_complete: certificate
            .cubical
            .public_constructor_inventory_exhaustive
            && certificate.cubical.typing_preserved
            && certificate.cubical.every_checked_dimension_action_natural,
        private_endpoint_premise_constructor_count: certificate
            .cubical
            .private_endpoint_premise_constructor_count,
        every_class_complete: certificate.every_class_complete,
        full_e4_complete: certificate.full_e4_complete,
        e2b_executed: certificate.forbidden_outputs.e2b_executed,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_schema2_e34_class_certificate(
    certificate: &Schema2E34ClassCertificate,
) -> Schema2E34ClassReplay {
    match build_schema2_e34_class_certificate() {
        Ok(expected) => replay_against(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn schema2_e34_class_json_pretty() -> Result<String, ClassCertificateError> {
    serde_json::to_string_pretty(&build_schema2_e34_class_certificate()?)
        .map(|json| format!("{json}\n"))
        .map_err(|error| ClassCertificateError::Json(error.to_string()))
}

pub fn replay_schema2_e34_class_json(json: &str) -> Schema2E34ClassReplay {
    let raw: Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(error) => return failed_replay(format!("invalid JSON: {error}")),
    };
    let certificate: Schema2E34ClassCertificate = match serde_json::from_str(json) {
        Ok(certificate) => certificate,
        Err(error) => return failed_replay(format!("certificate shape error: {error}")),
    };
    let typed = serde_json::to_value(&certificate).expect("class certificate serializes");
    if raw != typed {
        return failed_replay("JSON contains unknown, duplicate, or ignored structure");
    }
    replay_schema2_e34_class_certificate(&certificate)
}

pub fn emit_schema2_e34_class_create_new(
    path: &Path,
) -> Result<Schema2E34ClassReplay, ClassCertificateError> {
    let json = schema2_e34_class_json_pretty()?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| ClassCertificateError::Io(error.to_string()))?;
    output
        .write_all(json.as_bytes())
        .and_then(|()| output.flush())
        .map_err(|error| ClassCertificateError::Io(error.to_string()))?;
    let replay = replay_schema2_e34_class_json(&json);
    if !replay.valid {
        return Err(ClassCertificateError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn redigest(certificate: &mut Schema2E34ClassCertificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    #[test]
    fn class_certificate_replays_exact_operational_result() {
        let certificate = build_schema2_e34_class_certificate().unwrap();
        let replay = replay_schema2_e34_class_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(replay.ordinary_constructor_count, 9);
        assert!(replay.ordinary_registry_complete);
        assert_eq!(replay.public_cubical_constructor_count, 8);
        assert_eq!(replay.private_endpoint_premise_constructor_count, 4);
        assert!(replay.public_cubical_induction_complete);
        assert!(!replay.every_class_complete);
        assert!(!replay.full_e4_complete);
        assert!(!replay.e2b_executed);
    }

    #[test]
    fn completeness_and_count_mutations_fail_definition_replay() {
        let certificate = build_schema2_e34_class_certificate().unwrap();

        let mut completion = certificate.clone();
        completion.full_e4_complete = true;
        redigest(&mut completion);
        assert!(!replay_schema2_e34_class_certificate(&completion).valid);

        let mut constructor_count = certificate;
        constructor_count.ordinary.registered_constructor_count = 10;
        redigest(&mut constructor_count);
        assert!(!replay_schema2_e34_class_certificate(&constructor_count).valid);
    }
}
