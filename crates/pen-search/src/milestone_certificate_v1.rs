//! MS-1: a digest-bound milestone certificate.
//!
//! This issuer deliberately proves no mathematical content.  It binds the
//! existing legislative record, certified chain, exact BI-4 quantifiers, and
//! named frontier.  Replay is definition-based, so resealing a mutation does
//! not make it valid.

use crate::dnfq_theorem_layer_v1::{
    DnfRunStatus, issue_dnf4_corpus_projection_v1, replay_dnf4_corpus_projection_v1,
};
use crate::t_d2_1_operational_domain_v3::{
    Td21V3RunStatus, issue_t_d2_1_operational_domain_v3, replay_t_d2_1_operational_domain_v3,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const MS1_SCHEMA: &str = "ms1-milestone-certificate-v1";
pub const MS1_DATE: &str = "2026-07-24";

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/milestone_certificate_plan.md");

macro_rules! doc {
    ($name:literal) => {
        include_bytes!(concat!("../../../docs/", $name)) as &[u8]
    };
}

const REGULAR_LAW_DOCUMENTS: &[(&str, &[u8])] = &[
    (
        "ambient_former_closure_adjudication.md",
        doc!("ambient_former_closure_adjudication.md"),
    ),
    (
        "ambient_wrapper_adjudication.md",
        doc!("ambient_wrapper_adjudication.md"),
    ),
    (
        "bi0_prerequisites_adjudication.md",
        doc!("bi0_prerequisites_adjudication.md"),
    ),
    (
        "boundary_adjudication_proposal.md",
        doc!("boundary_adjudication_proposal.md"),
    ),
    (
        "certified_field_dereference_adjudication.md",
        doc!("certified_field_dereference_adjudication.md"),
    ),
    (
        "contextual_internality_adjudication.md",
        doc!("contextual_internality_adjudication.md"),
    ),
    (
        "dependent_context_adjudication.md",
        doc!("dependent_context_adjudication.md"),
    ),
    (
        "depth_two_domain_adjudication.md",
        doc!("depth_two_domain_adjudication.md"),
    ),
    (
        "e2_phase_order_adjudication.md",
        doc!("e2_phase_order_adjudication.md"),
    ),
    (
        "element_overlay_adjudication.md",
        doc!("element_overlay_adjudication.md"),
    ),
    (
        "endpoint_premise_api_adjudication.md",
        doc!("endpoint_premise_api_adjudication.md"),
    ),
    (
        "future_hole_definition_adjudication.md",
        doc!("future_hole_definition_adjudication.md"),
    ),
    (
        "inductive_telescope_internality_adjudication.md",
        doc!("inductive_telescope_internality_adjudication.md"),
    ),
    (
        "internal_classifier_branch_adjudication.md",
        doc!("internal_classifier_branch_adjudication.md"),
    ),
    (
        "motive_parametric_coherence_adjudication.md",
        doc!("motive_parametric_coherence_adjudication.md"),
    ),
    (
        "nu_register_adjudication.md",
        doc!("nu_register_adjudication.md"),
    ),
    (
        "phase5b_fork_adjudication.md",
        doc!("phase5b_fork_adjudication.md"),
    ),
    (
        "r_t3_stage4_adjudication.md",
        doc!("r_t3_stage4_adjudication.md"),
    ),
    (
        "schema2_operational_domain_adjudication.md",
        doc!("schema2_operational_domain_adjudication.md"),
    ),
    (
        "stage4_semantic_divergence_adjudication.md",
        doc!("stage4_semantic_divergence_adjudication.md"),
    ),
    (
        "stage8_deadlock_adjudication.md",
        doc!("stage8_deadlock_adjudication.md"),
    ),
    (
        "support_comprehension_adjudication.md",
        doc!("support_comprehension_adjudication.md"),
    ),
];

const E2_LAW_BYTES: &[u8] = doc!("e2_quotient_adjudications.md");
const STEPS_LAW_BYTES: &[u8] = doc!("steps_9_15_signature_adjudication.md");
const STEPS_ADOPTION_BYTES: &[u8] = doc!("steps_9_15_signature_adoption.md");
const TIE_LAW_BYTES: &[u8] = doc!("tie_resolution_protocol.md");
const A5_PLAN_BYTES: &[u8] = doc!("ip1_tdc1_plan.md");
const A5_EVIDENCE_BYTES: &[u8] = doc!("ip1_candidate_verdict_join_v3.json");

const AUXILIARY_SOURCES: &[(&str, &str, &[u8])] = &[
    (
        "milestone_brief",
        "docs/milestone_certificate_plan.md",
        PLAN_BYTES,
    ),
    (
        "mainline_brief",
        "docs/mainline_completion_plan.md",
        doc!("mainline_completion_plan.md"),
    ),
    (
        "wb1_program",
        "docs/windowed_bar_hypothesis.md",
        doc!("windowed_bar_hypothesis.md"),
    ),
    (
        "bridge_brief",
        "docs/bridge_completion_plan.md",
        doc!("bridge_completion_plan.md"),
    ),
];

const CHAIN_ARTIFACTS: &[(&str, &str, &str, &[u8])] = &[
    (
        "e5_dependent_context_finale",
        "docs/schema2_e5_future_hole_finale_v2_dependent_context.json",
        "E-5 dependent-context finale",
        doc!("schema2_e5_future_hole_finale_v2_dependent_context.json"),
    ),
    (
        "bi0_v6",
        "docs/bi0_semantic_register_v6.json",
        "BI-0 semantic register v6",
        doc!("bi0_semantic_register_v6.json"),
    ),
    (
        "bi1_v3",
        "docs/BI1_OPTION_A_SWEEP_V3_CERTIFICATE.json",
        "BI-1 four-branch sweep",
        doc!("BI1_OPTION_A_SWEEP_V3_CERTIFICATE.json"),
    ),
    (
        "bi1b_sweep",
        "docs/BI1B_PREFIX_GENERAL_SWEEP_V1_CERTIFICATE.json",
        "BI-1b prefix-general sweep",
        doc!("BI1B_PREFIX_GENERAL_SWEEP_V1_CERTIFICATE.json"),
    ),
    (
        "bi1b_regression",
        "docs/BI1B_REGRESSION_CERTIFICATE.json",
        "BI-1b enacted regression",
        doc!("BI1B_REGRESSION_CERTIFICATE.json"),
    ),
    (
        "bi1b_branch_2016",
        "docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json",
        "BI-1b branch certificate",
        doc!("BI1B_BRANCH_2016726758f3_CERTIFICATE.json"),
    ),
    (
        "bi1b_branch_43a0",
        "docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json",
        "BI-1b branch certificate",
        doc!("BI1B_BRANCH_43a0ed707770_CERTIFICATE.json"),
    ),
    (
        "bi1b_branch_4b22",
        "docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json",
        "BI-1b branch certificate",
        doc!("BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json"),
    ),
    (
        "bi1b_branch_b4f8",
        "docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json",
        "BI-1b branch certificate",
        doc!("BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json"),
    ),
    (
        "bi2_index_v2",
        "docs/BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json",
        "BI-2 four-branch index",
        doc!("BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json"),
    ),
    (
        "bi2_branch_2016",
        "docs/BI2_BRANCH_2016726758f3_V2_CERTIFICATE.json",
        "BI-2 branch finale",
        doc!("BI2_BRANCH_2016726758f3_V2_CERTIFICATE.json"),
    ),
    (
        "bi2_branch_43a0",
        "docs/BI2_BRANCH_43a0ed707770_V2_CERTIFICATE.json",
        "BI-2 branch finale",
        doc!("BI2_BRANCH_43a0ed707770_V2_CERTIFICATE.json"),
    ),
    (
        "bi2_branch_4b22",
        "docs/BI2_BRANCH_4b2211ecae25_V2_CERTIFICATE.json",
        "BI-2 branch finale",
        doc!("BI2_BRANCH_4b2211ecae25_V2_CERTIFICATE.json"),
    ),
    (
        "bi2_branch_b4f8",
        "docs/BI2_BRANCH_b4f821d9bb28_V2_CERTIFICATE.json",
        "BI-2 branch finale",
        doc!("BI2_BRANCH_b4f821d9bb28_V2_CERTIFICATE.json"),
    ),
    (
        "bi4_cone_v2",
        "docs/BI4_CONE_V2_CERTIFICATE.json",
        "BI-4 cone report and disposition table",
        doc!("BI4_CONE_V2_CERTIFICATE.json"),
    ),
    (
        "uc1",
        "docs/UC1_SCORING_V1_CERTIFICATE.json",
        "UC-1 prediction score",
        doc!("UC1_SCORING_V1_CERTIFICATE.json"),
    ),
    (
        "m1_transport",
        "docs/semantic_nu_transport_maps_v2.json",
        "M-1 maps and order-axis obstructions",
        doc!("semantic_nu_transport_maps_v2.json"),
    ),
    (
        "m3_v1",
        "docs/schema2_e7_e8_bridge_v1.json",
        "authoritative M-3 v1 bridge",
        doc!("schema2_e7_e8_bridge_v1.json"),
    ),
    (
        "bridge_execution",
        "docs/bridge_completion_execution_v1.json",
        "fail-closed bridge-chain execution",
        doc!("bridge_completion_execution_v1.json"),
    ),
    (
        "stage4_parsimony_v3",
        "docs/stage4_semantic_parsimony_v3.json",
        "Stage-4 semantic parsimony cross-binding",
        doc!("stage4_semantic_parsimony_v3.json"),
    ),
    (
        "semantic_ledger_v6",
        "docs/t_bi_nu1_semantic_provenance_v6.json",
        "semantic ledger and structural divergence",
        doc!("t_bi_nu1_semantic_provenance_v6.json"),
    ),
    (
        "dnf1",
        "docs/dnf1_canonical_contexts_v1.json",
        "DNF-1 theorem layer",
        doc!("dnf1_canonical_contexts_v1.json"),
    ),
    (
        "dnf2",
        "docs/dnf2_unified_judgments_v1.json",
        "DNF-2 theorem layer",
        doc!("dnf2_unified_judgments_v1.json"),
    ),
    (
        "dnf3",
        "docs/dnf3_naturality_closure_v1.json",
        "DNF-3 theorem layer",
        doc!("dnf3_naturality_closure_v1.json"),
    ),
    (
        "dnf4",
        "docs/dnf4_corpus_projection_v1.json",
        "DNF-4 exact frontier",
        doc!("dnf4_corpus_projection_v1.json"),
    ),
    (
        "td21_v3",
        "docs/t_d2_1_operational_domain_v3.json",
        "T-D2-1 v3 frozen-domain attempt",
        doc!("t_d2_1_operational_domain_v3.json"),
    ),
];

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Ms1Register {
    LegislativeRecord,
    SemanticRegisterAuthority,
    StructuralRegisterTestimony,
    ProofInventory,
    ArtifactMetadata,
    SyntaxIdentifier,
    DiagnosticRegister,
    ProgramState,
    MixedExplicitSource,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Ms1Quantifier {
    ConeLevel,
    BranchIndexed,
    AdoptedDoctrine,
    FrozenWrappedSurface,
    ProgramState,
    LegislativeRecord,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ms1TaggedNumber {
    pub decimal: String,
    pub register: Ms1Register,
    pub meaning: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ms1LawBinding {
    pub ordinal: Ms1TaggedNumber,
    pub law_id: String,
    pub normative_path: String,
    pub normative_file_blake3: String,
    pub adoption_evidence_path: String,
    pub adoption_evidence_file_blake3: String,
    pub adoption_block_verbatim: String,
    pub adoption_block_blake3: String,
    pub register: Ms1Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ms1ArtifactBinding {
    pub artifact_id: String,
    pub path: String,
    pub role: String,
    pub byte_length: Ms1TaggedNumber,
    pub file_blake3: String,
    pub schema: Option<String>,
    pub embedded_result_digest: String,
    pub register: Ms1Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ms1SourceBinding {
    pub source_id: String,
    pub path: String,
    pub role: String,
    pub byte_length: Ms1TaggedNumber,
    pub file_blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ms1ClaimBinding {
    pub claim_id: String,
    pub statement: String,
    pub quantifier: Ms1Quantifier,
    pub register: Ms1Register,
    pub source_artifact_id: String,
    pub source_artifact_result_digest: String,
    pub exact_source_snapshot: Value,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ms1FrontierRow {
    pub row_ordinal: Ms1TaggedNumber,
    pub register: Ms1Register,
    pub dnf4_row_verbatim: Value,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ms1DriftOccurrence {
    pub source_artifact_id: String,
    pub archived_artifact_result_digest: String,
    pub archived_artifact_file_blake3: String,
    pub archived_self_digest_presence_and_file_binding_validated: bool,
    pub json_path: String,
    pub live_replay_field: String,
    pub live_replay_value: bool,
    pub exact_live_error_fields: BTreeMap<String, Value>,
    pub archive_binding_context: BTreeMap<String, Value>,
    pub register: Ms1Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Ms1OpenItem {
    pub item_id: String,
    pub exact_statement: String,
    pub status: String,
    pub quantifier: Ms1Quantifier,
    pub register: Ms1Register,
    pub source_id: String,
    pub source_file_blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MilestoneCertificateV1 {
    pub schema: String,
    pub date: String,
    pub issuer_kind: String,
    pub proves_new_content: bool,
    pub milestone_is_m4: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m4_authorized: bool,
    pub milestone_plan_file_blake3: String,
    pub law_count: Ms1TaggedNumber,
    pub legislative_record: Vec<Ms1LawBinding>,
    pub source_bindings: Vec<Ms1SourceBinding>,
    pub certified_chain: Vec<Ms1ArtifactBinding>,
    pub claim_bindings: Vec<Ms1ClaimBinding>,
    pub bi4_disposition_table_verbatim: Value,
    pub semantic_vector: Vec<Ms1TaggedNumber>,
    pub semantic_sum: Ms1TaggedNumber,
    pub structural_divergence_table_verbatim: Value,
    pub dnf4_gap_row_count: Ms1TaggedNumber,
    pub dnf4_projected_row_count: Ms1TaggedNumber,
    pub dnf4_family_gap_count: Ms1TaggedNumber,
    pub dnf4_a3_gap_count: Ms1TaggedNumber,
    pub dnf4_unary_gap_count: Ms1TaggedNumber,
    pub dnf4_frontier_verbatim: Vec<Ms1FrontierRow>,
    pub td21_v3_frozen_bound_snapshot_verbatim: Value,
    pub td21_v3_frozen_bound_citations_verbatim: Value,
    pub km_program: Ms1OpenItem,
    pub open_items: Vec<Ms1OpenItem>,
    pub drift_manifest: Vec<Ms1DriftOccurrence>,
    pub uc1_summary_verbatim: Value,
    pub m1_order_axis_obstructions_verbatim: Value,
    pub m3_baseline_verbatim: Value,
    pub bridge_exit_gate_verbatim: Value,
    pub every_claim_digest_bound: bool,
    pub every_number_register_tagged: bool,
    pub every_headline_quantifier_explicit: bool,
    pub all_150_dnf4_rows_included_without_summary_substitution: bool,
    pub no_gap_discharged: bool,
    pub bridge_claim_issued: bool,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub forbidden_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ms1Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub law_count: usize,
    pub frontier_row_count: usize,
    pub drift_occurrence_count: usize,
    pub m3_v1_remains_authoritative: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Ms1Error {
    #[error("MS-1 input failure: {0}")]
    Input(String),
    #[error("MS-1 invariant failure: {0}")]
    Invariant(String),
    #[error("MS-1 JSON failure: {0}")]
    Json(String),
    #[error("MS-1 I/O failure: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(MS1_SCHEMA, domain, value)).expect("MS-1 value serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn number(value: usize, register: Ms1Register, meaning: &str) -> Ms1TaggedNumber {
    Ms1TaggedNumber {
        decimal: value.to_string(),
        register,
        meaning: meaning.to_owned(),
    }
}

fn parse(bytes: &[u8], label: &str) -> Result<Value, Ms1Error> {
    serde_json::from_slice(bytes)
        .map_err(|error| Ms1Error::Input(format!("{label} is not valid JSON: {error}")))
}

fn string<'a>(bytes: &'a [u8], label: &str) -> Result<&'a str, Ms1Error> {
    std::str::from_utf8(bytes)
        .map_err(|error| Ms1Error::Input(format!("{label} is not UTF-8: {error}")))
}

fn slice_from_marker(bytes: &[u8], label: &str, marker: &str) -> Result<String, Ms1Error> {
    let text = string(bytes, label)?;
    let start = text
        .find(marker)
        .ok_or_else(|| Ms1Error::Input(format!("{label} has no {marker:?} marker")))?;
    Ok(text[start..].trim_end().to_owned())
}

fn law_binding(
    ordinal: usize,
    law_id: &str,
    normative_path: &str,
    normative_bytes: &[u8],
    evidence_path: &str,
    evidence_bytes: &[u8],
    adoption: String,
) -> Ms1LawBinding {
    Ms1LawBinding {
        ordinal: number(
            ordinal,
            Ms1Register::SyntaxIdentifier,
            "legislative act ordinal",
        ),
        law_id: law_id.to_owned(),
        normative_path: normative_path.to_owned(),
        normative_file_blake3: bytes_hash(normative_bytes),
        adoption_evidence_path: evidence_path.to_owned(),
        adoption_evidence_file_blake3: bytes_hash(evidence_bytes),
        adoption_block_blake3: bytes_hash(adoption.as_bytes()),
        adoption_block_verbatim: adoption,
        register: Ms1Register::LegislativeRecord,
    }
}

fn legislative_record() -> Result<Vec<Ms1LawBinding>, Ms1Error> {
    let mut laws = Vec::new();
    for (name, bytes) in REGULAR_LAW_DOCUMENTS {
        let path = format!("docs/{name}");
        let id = name.trim_end_matches(".md");
        laws.push(law_binding(
            laws.len() + 1,
            id,
            &path,
            bytes,
            &path,
            bytes,
            slice_from_marker(bytes, name, "## ADOPTION")?,
        ));
    }

    let e2 = string(E2_LAW_BYTES, "e2_quotient_adjudications.md")?;
    let r1_start = e2
        .find("**R1 adopted**")
        .ok_or_else(|| Ms1Error::Input("E2 R1 adoption marker missing".to_owned()))?;
    let r2_start = e2
        .find("**R2 adopted**")
        .ok_or_else(|| Ms1Error::Input("E2 R2 adoption marker missing".to_owned()))?;
    for (id, block) in [
        ("e2_quotient_r1", e2[r1_start..r2_start].trim_end()),
        ("e2_quotient_r2", e2[r2_start..].trim_end()),
    ] {
        laws.push(law_binding(
            laws.len() + 1,
            id,
            "docs/e2_quotient_adjudications.md",
            E2_LAW_BYTES,
            "docs/e2_quotient_adjudications.md",
            E2_LAW_BYTES,
            String::from(block),
        ));
    }

    laws.push(law_binding(
        laws.len() + 1,
        "steps_9_15_trace_faithful_signature_batch_v1",
        "docs/steps_9_15_signature_adjudication.md",
        STEPS_LAW_BYTES,
        "docs/steps_9_15_signature_adoption.md",
        STEPS_ADOPTION_BYTES,
        string(STEPS_ADOPTION_BYTES, "steps_9_15_signature_adoption.md")?
            .trim_end()
            .to_owned(),
    ));
    laws.push(law_binding(
        laws.len() + 1,
        "tie_resolution_protocol",
        "docs/tie_resolution_protocol.md",
        TIE_LAW_BYTES,
        "docs/tie_resolution_protocol.md",
        TIE_LAW_BYTES,
        slice_from_marker(TIE_LAW_BYTES, "tie_resolution_protocol.md", "## ADOPTION")?,
    ));

    let a5 = parse(A5_EVIDENCE_BYTES, "ip1_candidate_verdict_join_v3.json")?;
    let adoption = a5
        .pointer("/a5/adjudication_source")
        .and_then(Value::as_str)
        .ok_or_else(|| Ms1Error::Input("A5 adoption source missing".to_owned()))?;
    laws.push(law_binding(
        laws.len() + 1,
        "a5_open_band_burden",
        "docs/ip1_tdc1_plan.md",
        A5_PLAN_BYTES,
        "docs/ip1_candidate_verdict_join_v3.json",
        A5_EVIDENCE_BYTES,
        adoption.to_owned(),
    ));
    laws.sort_by(|left, right| left.law_id.cmp(&right.law_id));
    for (index, law) in laws.iter_mut().enumerate() {
        law.ordinal = number(
            index + 1,
            Ms1Register::SyntaxIdentifier,
            "legislative act ordinal",
        );
    }
    if laws.len() != 27 {
        return Err(Ms1Error::Invariant(format!(
            "the legislative record contains {} acts, expected 27",
            laws.len()
        )));
    }
    Ok(laws)
}

fn embedded_digest(value: &Value, label: &str) -> Result<String, Ms1Error> {
    value
        .get("result_digest")
        .or_else(|| value.get("certificate_digest"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| Ms1Error::Input(format!("{label} has no embedded result digest")))
}

fn artifact_bindings() -> Result<(Vec<Ms1ArtifactBinding>, BTreeMap<String, Value>), Ms1Error> {
    let mut bindings = Vec::new();
    let mut values = BTreeMap::new();
    for (id, path, role, bytes) in CHAIN_ARTIFACTS {
        let value = parse(bytes, path)?;
        let digest = embedded_digest(&value, path)?;
        bindings.push(Ms1ArtifactBinding {
            artifact_id: (*id).to_owned(),
            path: (*path).to_owned(),
            role: (*role).to_owned(),
            byte_length: number(
                bytes.len(),
                Ms1Register::ArtifactMetadata,
                "bound artifact byte length",
            ),
            file_blake3: bytes_hash(bytes),
            schema: value
                .get("schema")
                .and_then(Value::as_str)
                .map(str::to_owned),
            embedded_result_digest: digest,
            register: Ms1Register::ArtifactMetadata,
        });
        values.insert((*id).to_owned(), value);
    }
    Ok((bindings, values))
}

fn source_bindings() -> Vec<Ms1SourceBinding> {
    AUXILIARY_SOURCES
        .iter()
        .map(|(id, path, bytes)| Ms1SourceBinding {
            source_id: (*id).to_owned(),
            path: (*path).to_owned(),
            role: match *id {
                "milestone_brief" => "frozen MS-1 scope and no-promotion gate",
                "mainline_brief" => "frozen mainline gate and external-falsifier frontier",
                "wb1_program" => "WB-1 W-T1/W-T2 open program",
                "bridge_brief" => "closed bridge-chain specification",
                _ => unreachable!(),
            }
            .to_owned(),
            byte_length: number(
                bytes.len(),
                Ms1Register::ArtifactMetadata,
                "bound source byte length",
            ),
            file_blake3: bytes_hash(bytes),
        })
        .collect()
}

fn artifact_digest(bindings: &[Ms1ArtifactBinding], id: &str) -> Result<String, Ms1Error> {
    bindings
        .iter()
        .find(|binding| binding.artifact_id == id)
        .map(|binding| binding.embedded_result_digest.clone())
        .ok_or_else(|| Ms1Error::Invariant(format!("missing artifact binding {id}")))
}

fn claim(
    id: &str,
    statement: &str,
    quantifier: Ms1Quantifier,
    register: Ms1Register,
    artifact_id: &str,
    snapshot: Value,
    bindings: &[Ms1ArtifactBinding],
) -> Result<Ms1ClaimBinding, Ms1Error> {
    Ok(Ms1ClaimBinding {
        claim_id: id.to_owned(),
        statement: statement.to_owned(),
        quantifier,
        register,
        source_artifact_id: artifact_id.to_owned(),
        source_artifact_result_digest: artifact_digest(bindings, artifact_id)?,
        exact_source_snapshot: snapshot,
    })
}

fn disposition_register(claim_name: &str) -> Ms1Register {
    if claim_name.contains("diagnostic_bar") || claim_name.contains("Delta_Phi_Omega") {
        Ms1Register::DiagnosticRegister
    } else if claim_name.contains("kappa_and_semantic_nu")
        || claim_name.contains("root_act_identity")
    {
        Ms1Register::MixedExplicitSource
    } else if claim_name.contains("Sigma_kappa") || claim_name.contains("candidate_winner") {
        Ms1Register::StructuralRegisterTestimony
    } else {
        Ms1Register::SemanticRegisterAuthority
    }
}

fn build_claims(
    values: &BTreeMap<String, Value>,
    bindings: &[Ms1ArtifactBinding],
) -> Result<Vec<Ms1ClaimBinding>, Ms1Error> {
    let bi4 = &values["bi4_cone_v2"];
    let dispositions = bi4
        .get("branch_index_disposition")
        .and_then(Value::as_array)
        .ok_or_else(|| Ms1Error::Input("BI-4 disposition table missing".to_owned()))?;
    let mut claims = Vec::new();
    for row in dispositions {
        let name = row
            .get("claim")
            .and_then(Value::as_str)
            .ok_or_else(|| Ms1Error::Input("BI-4 disposition claim missing".to_owned()))?;
        let status = row
            .get("status")
            .and_then(Value::as_str)
            .ok_or_else(|| Ms1Error::Input("BI-4 disposition status missing".to_owned()))?;
        let quantifier = match status {
            "promoted_to_cone_level" => Ms1Quantifier::ConeLevel,
            "remains_branch_indexed" => Ms1Quantifier::BranchIndexed,
            _ => {
                return Err(Ms1Error::Input(format!(
                    "unknown BI-4 disposition status {status}"
                )));
            }
        };
        claims.push(claim(
            &format!("bi4::{name}"),
            name,
            quantifier,
            disposition_register(name),
            "bi4_cone_v2",
            row.clone(),
            bindings,
        )?);
    }

    let uc1 = &values["uc1"];
    for key in ["p1", "p2", "p3"] {
        let value = uc1
            .get(key)
            .ok_or_else(|| Ms1Error::Input(format!("UC-1 {key} missing")))?;
        claims.push(claim(
            &format!("uc1::{key}"),
            &format!(
                "UC-1 {} verdict with its sealed quantifier",
                key.to_uppercase()
            ),
            Ms1Quantifier::FrozenWrappedSurface,
            Ms1Register::ProofInventory,
            "uc1",
            json!({
                "prediction_id": value.get("prediction_id"),
                "quantifier": value.get("quantifier"),
                "verdict": value.get("verdict"),
                "derivation_hash": value.get("derivation_hash")
            }),
            bindings,
        )?);
    }

    for (id, statement) in [
        (
            "doctrine::two_register_acceptance",
            "Two-register acceptance: semantic authority and structural testimony remain distinct.",
        ),
        (
            "doctrine::guarded_value_nonlegislation",
            "Value never legislates at guarded stages.",
        ),
        (
            "doctrine::choice_secured_against_value",
            "Choice is secured against value, not converted into proof.",
        ),
    ] {
        claims.push(claim(
            id,
            statement,
            Ms1Quantifier::AdoptedDoctrine,
            Ms1Register::LegislativeRecord,
            "stage4_parsimony_v3",
            json!({
                "stage4_outcome": values["stage4_parsimony_v3"].get("outcome"),
                "permitted_conclusion": values["stage4_parsimony_v3"].get("permitted_conclusion"),
                "result_digest": values["stage4_parsimony_v3"].get("result_digest")
            }),
            bindings,
        )?);
    }
    Ok(claims)
}

fn scan_drift(
    artifact_id: &str,
    artifact_result_digest: &str,
    artifact_file_blake3: &str,
    value: &Value,
    path: &str,
    output: &mut Vec<Ms1DriftOccurrence>,
) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let replay_flag = (key.contains("replay_valid") || key.contains("replay_passed"))
                    && child == &Value::Bool(false);
                if replay_flag {
                    let exact_live_error_fields = object
                        .iter()
                        .filter(|(name, _)| name.contains("replay_errors"))
                        .map(|(name, value)| (name.clone(), value.clone()))
                        .collect();
                    let archive_binding_context = object
                        .iter()
                        .filter(|(name, value)| {
                            (name.contains("digest")
                                && (name.contains("valid")
                                    || name.contains("exact")
                                    || name.contains("authenticated")))
                                || name.contains("fallback")
                                || name.contains("drift_bound")
                                || name.contains("logical_projection")
                                || (name.contains("repaired") && value.is_boolean())
                                || (name.contains("concealed") && value.is_boolean())
                        })
                        .map(|(name, value)| (name.clone(), value.clone()))
                        .collect();
                    output.push(Ms1DriftOccurrence {
                        source_artifact_id: artifact_id.to_owned(),
                        archived_artifact_result_digest: artifact_result_digest.to_owned(),
                        archived_artifact_file_blake3: artifact_file_blake3.to_owned(),
                        archived_self_digest_presence_and_file_binding_validated:
                            artifact_result_digest.starts_with("blake3:")
                                && artifact_result_digest.len() == 71
                                && artifact_file_blake3.starts_with("blake3:")
                                && artifact_file_blake3.len() == 71,
                        json_path: path.to_owned(),
                        live_replay_field: key.clone(),
                        live_replay_value: false,
                        exact_live_error_fields,
                        archive_binding_context,
                        register: Ms1Register::StructuralRegisterTestimony,
                    });
                }
                scan_drift(
                    artifact_id,
                    artifact_result_digest,
                    artifact_file_blake3,
                    child,
                    &format!("{path}/{}", key.replace('~', "~0").replace('/', "~1")),
                    output,
                );
            }
        }
        Value::Array(array) => {
            for (index, child) in array.iter().enumerate() {
                scan_drift(
                    artifact_id,
                    artifact_result_digest,
                    artifact_file_blake3,
                    child,
                    &format!("{path}/{index}"),
                    output,
                );
            }
        }
        _ => {}
    }
}

fn source_hash(sources: &[Ms1SourceBinding], id: &str) -> Result<String, Ms1Error> {
    sources
        .iter()
        .find(|source| source.source_id == id)
        .map(|source| source.file_blake3.clone())
        .ok_or_else(|| Ms1Error::Invariant(format!("missing source binding {id}")))
}

fn open_item(
    id: &str,
    statement: &str,
    source_id: &str,
    sources: &[Ms1SourceBinding],
) -> Result<Ms1OpenItem, Ms1Error> {
    Ok(Ms1OpenItem {
        item_id: id.to_owned(),
        exact_statement: statement.to_owned(),
        status: "open_named_not_discharged".to_owned(),
        quantifier: Ms1Quantifier::ProgramState,
        register: Ms1Register::ProgramState,
        source_id: source_id.to_owned(),
        source_file_blake3: source_hash(sources, source_id)?,
    })
}

fn result_digest(certificate: &MilestoneCertificateV1) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn build_certificate() -> Result<MilestoneCertificateV1, Ms1Error> {
    let dnf4 = issue_dnf4_corpus_projection_v1()
        .map_err(|error| Ms1Error::Input(format!("DNF-4 issuance failed: {error}")))?;
    let dnf4_replay = replay_dnf4_corpus_projection_v1(&dnf4);
    if !dnf4_replay.valid || dnf4.status != DnfRunStatus::StoppedNamedGaps {
        return Err(Ms1Error::Input(format!(
            "DNF-4 does not replay in its named-gap state: {}",
            dnf4_replay.errors.join("; ")
        )));
    }
    let td21 = issue_t_d2_1_operational_domain_v3()
        .map_err(|error| Ms1Error::Input(format!("T-D2-1 v3 issuance failed: {error}")))?;
    let td21_replay = replay_t_d2_1_operational_domain_v3(&td21);
    if !td21_replay.valid || td21.status != Td21V3RunStatus::StoppedDnfNamedGaps {
        return Err(Ms1Error::Input(format!(
            "T-D2-1 v3 does not replay in its named-gap state: {}",
            td21_replay.errors.join("; ")
        )));
    }

    let legislative_record = legislative_record()?;
    let sources = source_bindings();
    let (certified_chain, values) = artifact_bindings()?;
    if serde_json::to_value(&dnf4).map_err(|error| Ms1Error::Json(error.to_string()))?
        != values["dnf4"]
    {
        return Err(Ms1Error::Invariant(
            "DNF-4 create-new issuance differs from the bound archive".to_owned(),
        ));
    }
    if serde_json::to_value(&td21).map_err(|error| Ms1Error::Json(error.to_string()))?
        != values["td21_v3"]
    {
        return Err(Ms1Error::Invariant(
            "T-D2-1 v3 create-new issuance differs from the bound archive".to_owned(),
        ));
    }
    let claim_bindings = build_claims(&values, &certified_chain)?;
    let ledger = &values["semantic_ledger_v6"];
    let raw_vector = ledger
        .get("authoritative_semantic_register")
        .and_then(Value::as_array)
        .ok_or_else(|| Ms1Error::Input("semantic vector missing".to_owned()))?;
    let semantic_vector = raw_vector
        .iter()
        .enumerate()
        .map(|(index, value)| {
            value
                .as_u64()
                .map(|value| {
                    number(
                        value as usize,
                        Ms1Register::SemanticRegisterAuthority,
                        &format!("authoritative semantic nu at Stage {}", index + 1),
                    )
                })
                .ok_or_else(|| Ms1Error::Input("semantic vector entry is not unsigned".to_owned()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let semantic_sum_value = semantic_vector
        .iter()
        .map(|entry| {
            entry
                .decimal
                .parse::<usize>()
                .expect("issuer-owned decimal")
        })
        .sum::<usize>();
    if semantic_vector.len() != 15 || semantic_sum_value != 32 {
        return Err(Ms1Error::Invariant(format!(
            "semantic ledger is not the frozen 15-entry sum-32 vector: len={}, sum={semantic_sum_value}",
            semantic_vector.len()
        )));
    }

    let mut frontier = Vec::new();
    let mut family = 0usize;
    let mut a3 = 0usize;
    let mut unary = 0usize;
    for (index, row) in dnf4.rows.iter().enumerate() {
        let value = serde_json::to_value(row).map_err(|error| Ms1Error::Json(error.to_string()))?;
        let gap = value
            .pointer("/disposition/gap_id")
            .and_then(Value::as_str)
            .ok_or_else(|| Ms1Error::Invariant(format!("DNF-4 row {index} is not a named gap")))?;
        match gap {
            "DNF4_FAMILY_CONTEXT_OR_UNIVERSAL_NATURALITY_UNEXPORTED" => family += 1,
            "DNF4_A3_INSTANCE_HAS_LOCAL_SQUARE_BUT_NO_FAMILY_LEVEL_CLOSURE" => a3 += 1,
            "DNF4_UNARY_DEPENDENT_DECLARATION_BODY_UNEXPORTED" => unary += 1,
            other => return Err(Ms1Error::Invariant(format!("unknown DNF-4 gap {other}"))),
        }
        frontier.push(Ms1FrontierRow {
            row_ordinal: number(
                index + 1,
                Ms1Register::SyntaxIdentifier,
                "DNF-4 row ordinal",
            ),
            register: Ms1Register::ProgramState,
            dnf4_row_verbatim: value,
        });
    }
    if (frontier.len(), family, a3, unary) != (150, 61, 72, 17) {
        return Err(Ms1Error::Invariant(format!(
            "DNF-4 frontier mismatch: total={}, family={family}, A3={a3}, unary={unary}",
            frontier.len()
        )));
    }

    let mut drift_manifest = Vec::new();
    for binding in &certified_chain {
        scan_drift(
            &binding.artifact_id,
            &binding.embedded_result_digest,
            &binding.file_blake3,
            &values[&binding.artifact_id],
            "",
            &mut drift_manifest,
        );
    }
    drift_manifest.sort_by(|left, right| {
        (
            &left.source_artifact_id,
            &left.json_path,
            &left.live_replay_field,
        )
            .cmp(&(
                &right.source_artifact_id,
                &right.json_path,
                &right.live_replay_field,
            ))
    });

    let km = open_item(
        "KM",
        "Full context-grammar canonicalization; universal naturality over every legal substitution; decidability and finiteness for the bounded fragment re-derived against the full grammar.",
        "milestone_brief",
        &sources,
    )?;
    let open_items = vec![
        open_item(
            "U_T2",
            "The Pi/Sigma act equivalence; the cone's one open identity.",
            "milestone_brief",
            &sources,
        )?,
        open_item(
            "WB1_WT1_WT2",
            "WB-1's W-T1 and W-T2 remain open.",
            "wb1_program",
            &sources,
        )?,
        open_item(
            "OBSERVATIONAL_UNIVALENCE",
            "The observational-univalence conjecture remains parked.",
            "milestone_brief",
            &sources,
        )?,
        open_item(
            "R2_INJECTIVE_JOIN",
            "The R2 occurrence-to-action injective join remains open.",
            "milestone_brief",
            &sources,
        )?,
        open_item(
            "BRIDGE_CHAIN",
            "T-D2-2 -> BC1 -> 431-gap rerun -> M-3 v2 -> M-4 remains specified and closed behind KM.",
            "bridge_brief",
            &sources,
        )?,
        open_item(
            "EXTERNAL_FALSIFIER_FRONTIER",
            "A derived parameter-free quantity exposed to measurement is the declared next front.",
            "mainline_brief",
            &sources,
        )?,
    ];

    let uc1 = &values["uc1"];
    let m1 = &values["m1_transport"];
    let bridge = &values["bridge_execution"];
    let bi4 = &values["bi4_cone_v2"];
    let mut certificate = MilestoneCertificateV1 {
        schema: MS1_SCHEMA.to_owned(),
        date: MS1_DATE.to_owned(),
        issuer_kind: "digest_bound_state_certificate_not_a_theorem".to_owned(),
        proves_new_content: false,
        milestone_is_m4: false,
        m3_v1_remains_authoritative: true,
        m4_authorized: false,
        milestone_plan_file_blake3: bytes_hash(PLAN_BYTES),
        law_count: number(legislative_record.len(), Ms1Register::ProofInventory, "signed legislative acts bound verbatim"),
        legislative_record,
        source_bindings: sources,
        certified_chain,
        claim_bindings,
        bi4_disposition_table_verbatim: bi4["branch_index_disposition"].clone(),
        semantic_vector,
        semantic_sum: number(semantic_sum_value, Ms1Register::SemanticRegisterAuthority, "sum of the authoritative enacted-branch semantic register"),
        structural_divergence_table_verbatim: ledger["register_table"].clone(),
        dnf4_gap_row_count: number(frontier.len(), Ms1Register::ProofInventory, "exact named DNF-4 gap rows"),
        dnf4_projected_row_count: number(0, Ms1Register::ProofInventory, "exact projected DNF-4 rows"),
        dnf4_family_gap_count: number(family, Ms1Register::ProofInventory, "family rows lacking complete-context or universal-naturality evidence"),
        dnf4_a3_gap_count: number(a3, Ms1Register::ProofInventory, "A3 rows with local squares but no family-level closure"),
        dnf4_unary_gap_count: number(unary, Ms1Register::ProofInventory, "unary rows lacking dependent declaration bodies"),
        dnf4_frontier_verbatim: frontier,
        td21_v3_frozen_bound_snapshot_verbatim: serde_json::to_value(&td21.frozen_bound_snapshot)
            .map_err(|error| Ms1Error::Json(error.to_string()))?,
        td21_v3_frozen_bound_citations_verbatim: serde_json::to_value(&td21.frozen_bound_citations)
            .map_err(|error| Ms1Error::Json(error.to_string()))?,
        km_program: km,
        open_items,
        drift_manifest,
        uc1_summary_verbatim: json!({
            "p1": {"prediction_id": uc1["p1"]["prediction_id"], "quantifier": uc1["p1"]["quantifier"], "verdict": uc1["p1"]["verdict"], "derivation_hash": uc1["p1"]["derivation_hash"]},
            "p2": {"prediction_id": uc1["p2"]["prediction_id"], "quantifier": uc1["p2"]["quantifier"], "verdict": uc1["p2"]["verdict"], "derivation_hash": uc1["p2"]["derivation_hash"]},
            "p3": {"prediction_id": uc1["p3"]["prediction_id"], "quantifier": uc1["p3"]["quantifier"], "verdict": uc1["p3"]["verdict"], "derivation_hash": uc1["p3"]["derivation_hash"]},
            "outcome_zone": uc1["outcome_zone"],
            "sole_former_axis_residual": uc1["sole_former_axis_residual"]
        }),
        m1_order_axis_obstructions_verbatim: json!({
            "exported_map_count": m1["exported_map_count"],
            "order_axis_obstruction_count": m1["order_axis_obstruction_count"],
            "order_axis_obstructions": m1["order_axis_obstructions"],
            "result_digest": m1["result_digest"]
        }),
        m3_baseline_verbatim: bridge["baseline"].clone(),
        bridge_exit_gate_verbatim: bridge["gate"].clone(),
        every_claim_digest_bound: true,
        every_number_register_tagged: true,
        every_headline_quantifier_explicit: true,
        all_150_dnf4_rows_included_without_summary_substitution: true,
        no_gap_discharged: true,
        bridge_claim_issued: false,
        mutation_falsifiers: vec![
            "flip_any_bound_file_or_embedded_artifact_digest_then_replay_must_fail".to_owned(),
            "flip_any_claim_quantifier_or_register_then_replay_must_fail".to_owned(),
            "change_omit_reorder_or_summarize_any_DNF4_gap_row_then_replay_must_fail".to_owned(),
            "change_any_legislative_adoption_byte_or_digest_then_replay_must_fail".to_owned(),
            "omit_or_sanitize_any_live_replay_drift_occurrence_then_replay_must_fail".to_owned(),
            "imply_bridge_completion_M4_or_gap_discharge_then_replay_must_fail".to_owned(),
        ],
        permitted_conclusion: "MS-1 binds the frozen legislative record, certified chain, exact BI-4 quantifiers, semantic and structural registers, complete named frontier, and live drift manifest. It proves no new content. Cone-level demand and halt claims have only the scope assigned by BI-4; Stage-4 and cumulative quantities remain branch-indexed.".to_owned(),
        forbidden_conclusion: "MS-1 does not complete the bridge, issue M-3 v2 or M-4, discharge KM or any DNF-4 row, settle U-T2/WB-1/univalence/R2, or amend any gate.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = result_digest(&certificate);
    Ok(certificate)
}

static EXPECTED: OnceLock<Result<MilestoneCertificateV1, Ms1Error>> = OnceLock::new();

pub fn issue_milestone_certificate_v1() -> Result<MilestoneCertificateV1, Ms1Error> {
    EXPECTED.get_or_init(build_certificate).clone()
}

pub fn replay_milestone_certificate_v1(claimed: &MilestoneCertificateV1) -> Ms1Replay {
    let mut errors = Vec::new();
    match issue_milestone_certificate_v1() {
        Ok(expected) => {
            if claimed.result_digest != result_digest(claimed) {
                errors.push("MS-1 result digest mismatch".to_owned());
            }
            if claimed != &expected {
                errors.push(
                    "MS-1 certificate differs from deterministic create-new issuance".to_owned(),
                );
            }
        }
        Err(error) => errors.push(format!("MS-1 deterministic issuance failed: {error}")),
    }
    Ms1Replay {
        valid: errors.is_empty(),
        errors,
        law_count: claimed.legislative_record.len(),
        frontier_row_count: claimed.dnf4_frontier_verbatim.len(),
        drift_occurrence_count: claimed.drift_manifest.len(),
        m3_v1_remains_authoritative: claimed.m3_v1_remains_authoritative,
        m4_authorized: claimed.m4_authorized,
    }
}

pub fn replay_milestone_certificate_v1_json(input: &str) -> Ms1Replay {
    match serde_json::from_str::<MilestoneCertificateV1>(input) {
        Ok(certificate) => replay_milestone_certificate_v1(&certificate),
        Err(error) => Ms1Replay {
            valid: false,
            errors: vec![format!("MS-1 JSON parse failed: {error}")],
            law_count: 0,
            frontier_row_count: 0,
            drift_occurrence_count: 0,
            m3_v1_remains_authoritative: false,
            m4_authorized: false,
        },
    }
}

pub fn render_milestone_certificate_v1(certificate: &MilestoneCertificateV1) -> String {
    let vector = certificate
        .semantic_vector
        .iter()
        .map(|entry| entry.decimal.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let mut report = format!(
        "# MS-1 Milestone Certificate v1\n\n\
**Date:** {}. **Status:** SEALED MILESTONE; NOT M-4.\n\n\
MS-1 binds existing evidence and proves no new mathematical content. M-3 v1 remains authoritative; `m4_authorized=false`; every named gap remains open.\n\n\
## Bound state\n\n\
- Legislative acts: {} (verbatim adoption blocks below).\n\
- Certified-chain artifacts: {}.\n\
- BI-4 claim dispositions: {} exact rows.\n\
- Semantic register: `[{}]`, sum {}. The complete vector and sum are enacted-branch quantities; BI-4 promotes only the Stage-5-through-15 per-stage ledger to cone level.\n\
- DNF-4 frontier: {} named gaps, 0 projected — {} family, {} A3, {} unary.\n\
- Live-replay drift occurrences retained as testimony: {}.\n\
- Bridge: M-3 v1 is 4/9 and fail-closed; no M-3 v2 or M-4 was issued.\n\n\
## Exact claim scopes\n\n",
        certificate.date,
        certificate.law_count.decimal,
        certificate.certified_chain.len(),
        certificate
            .claim_bindings
            .iter()
            .filter(|claim| claim.claim_id.starts_with("bi4::"))
            .count(),
        vector,
        certificate.semantic_sum.decimal,
        certificate.dnf4_gap_row_count.decimal,
        certificate.dnf4_family_gap_count.decimal,
        certificate.dnf4_a3_gap_count.decimal,
        certificate.dnf4_unary_gap_count.decimal,
        certificate.drift_manifest.len(),
    );
    for claim in &certificate.claim_bindings {
        report.push_str(&format!(
            "- `{}` — {:?}; register `{:?}`; source `{}` / `{}`.\n",
            claim.claim_id,
            claim.quantifier,
            claim.register,
            claim.source_artifact_id,
            claim.source_artifact_result_digest
        ));
    }
    report.push_str("\n## Open frontier\n\n");
    report.push_str(&format!(
        "- **KM:** {}\n",
        certificate.km_program.exact_statement
    ));
    for item in &certificate.open_items {
        report.push_str(&format!(
            "- **{}:** {}\n",
            item.item_id, item.exact_statement
        ));
    }
    report.push_str("\n## Drift manifest\n\n");
    for drift in &certificate.drift_manifest {
        report.push_str(&format!(
            "- `{}` `{}` / `{}` — errors `{}`; archive context `{}`.\n",
            drift.source_artifact_id,
            drift.json_path,
            drift.live_replay_field,
            serde_json::to_string(&drift.exact_live_error_fields).expect("drift serializes"),
            serde_json::to_string(&drift.archive_binding_context).expect("drift serializes")
        ));
    }
    report.push_str("\n## Legislative record — verbatim adoption blocks\n\n");
    for law in &certificate.legislative_record {
        report.push_str(&format!(
            "### {}. `{}`\n\nSource: `{}` (`{}`); adoption evidence: `{}` (`{}`). Adoption-block digest: `{}`.\n\n```text\n{}\n```\n\n",
            law.ordinal.decimal,
            law.law_id,
            law.normative_path,
            law.normative_file_blake3,
            law.adoption_evidence_path,
            law.adoption_evidence_file_blake3,
            law.adoption_block_blake3,
            law.adoption_block_verbatim
        ));
    }
    report.push_str("## DNF-4 disposition table — verbatim rows\n\n");
    report.push_str(
        "| # | row_id | kind | stage | disposition | gap_id |\n|---:|---|---|---:|---|---|\n",
    );
    for row in &certificate.dnf4_frontier_verbatim {
        let v = &row.dnf4_row_verbatim;
        report.push_str(&format!(
            "| {} | `{}` | `{}` | {} | `{}` | `{}` |\n",
            row.row_ordinal.decimal,
            v["row_id"].as_str().unwrap_or(""),
            v["row_kind"].as_str().unwrap_or(""),
            v["stage"],
            v["disposition"]["disposition"].as_str().unwrap_or(""),
            v["disposition"]["gap_id"].as_str().unwrap_or("")
        ));
    }
    report.push_str(
        "\n### Full DNF-4 row records (byte-content preserved as structured JSON)\n\n```json\n",
    );
    report.push_str(
        &serde_json::to_string_pretty(&certificate.dnf4_frontier_verbatim)
            .expect("DNF-4 frontier serializes"),
    );
    report.push_str("\n```\n");
    report.push_str("\n## Gate\n\n");
    report.push_str(&format!(
        "{}\n\n{}\n\n",
        certificate.permitted_conclusion, certificate.forbidden_conclusion
    ));
    report.push_str(&format!(
        "Certificate digest: `{}`.\n",
        certificate.result_digest
    ));
    report
}

pub fn render_ms1_index(certificate: &MilestoneCertificateV1) -> String {
    let mut index = "# MS-1 Binding Index\n\n".to_owned();
    index.push_str(&format!(
        "Certificate: `docs/milestone_certificate_v1.json` / `{}`. This index is navigation only; the JSON certificate is authoritative.\n\n## Claims\n\n",
        certificate.result_digest
    ));
    for claim in &certificate.claim_bindings {
        index.push_str(&format!(
            "- `{}` -> `{}` at `{}`; quantifier `{:?}`; register `{:?}`.\n",
            claim.claim_id,
            claim.source_artifact_id,
            claim.source_artifact_result_digest,
            claim.quantifier,
            claim.register
        ));
    }
    index.push_str("\n## Certified chain\n\n");
    for artifact in &certificate.certified_chain {
        index.push_str(&format!(
            "- `{}` -> `{}`; embedded `{}`; file `{}`.\n",
            artifact.artifact_id,
            artifact.path,
            artifact.embedded_result_digest,
            artifact.file_blake3
        ));
    }
    index.push_str("\n## Legislative acts\n\n");
    for law in &certificate.legislative_record {
        index.push_str(&format!(
            "- {}. `{}` -> `{}` / adoption `{}`; block `{}`.\n",
            law.ordinal.decimal,
            law.law_id,
            law.normative_file_blake3,
            law.adoption_evidence_file_blake3,
            law.adoption_block_blake3
        ));
    }
    index.push_str("\n## Frontier\n\n");
    index.push_str(&format!(
        "- DNF-4 rows: {}; projected: {}; family/A3/unary gaps: {}/{}/{}.\n- KM and all other open items remain open. M-3 v1 remains authoritative; M-4 is unauthorized.\n",
        certificate.dnf4_gap_row_count.decimal,
        certificate.dnf4_projected_row_count.decimal,
        certificate.dnf4_family_gap_count.decimal,
        certificate.dnf4_a3_gap_count.decimal,
        certificate.dnf4_unary_gap_count.decimal
    ));
    index
}

fn create_new(path: &Path, bytes: &[u8]) -> Result<(), Ms1Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Ms1Error::Io(format!("could not create {}: {error}", path.display())))?;
    file.write_all(bytes)
        .map_err(|error| Ms1Error::Io(format!("could not write {}: {error}", path.display())))
}

pub fn emit_milestone_certificate_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
    index_path: &Path,
) -> Result<MilestoneCertificateV1, Ms1Error> {
    let certificate = issue_milestone_certificate_v1()?;
    let json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Ms1Error::Json(error.to_string()))?;
    let report = render_milestone_certificate_v1(&certificate);
    let index = render_ms1_index(&certificate);
    create_new(certificate_path, &json)?;
    if let Err(error) = create_new(report_path, report.as_bytes()) {
        let _ = remove_file(certificate_path);
        return Err(error);
    }
    if let Err(error) = create_new(index_path, index.as_bytes()) {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(error);
    }
    let replay = replay_milestone_certificate_v1_json(
        &std::fs::read_to_string(certificate_path).map_err(|error| {
            Ms1Error::Io(format!(
                "could not reread {}: {error}",
                certificate_path.display()
            ))
        })?,
    );
    if !replay.valid {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        let _ = remove_file(index_path);
        return Err(Ms1Error::Invariant(format!(
            "emitted certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reseal(certificate: &mut MilestoneCertificateV1) {
        certificate.result_digest = result_digest(certificate);
    }

    #[test]
    fn issuer_binds_exact_state_and_stays_fail_closed() {
        let certificate = issue_milestone_certificate_v1().unwrap();
        let replay = replay_milestone_certificate_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(certificate.legislative_record.len(), 27);
        assert_eq!(certificate.dnf4_frontier_verbatim.len(), 150);
        assert_eq!(certificate.dnf4_projected_row_count.decimal, "0");
        assert_eq!(certificate.semantic_sum.decimal, "32");
        assert!(certificate.m3_v1_remains_authoritative);
        assert!(!certificate.m4_authorized);
        assert!(!certificate.bridge_claim_issued);
        assert!(certificate.no_gap_discharged);
        assert!(!certificate.drift_manifest.is_empty());
    }

    #[test]
    fn every_bound_artifact_digest_is_mutation_guarded() {
        let certificate = issue_milestone_certificate_v1().unwrap();
        for index in 0..certificate.certified_chain.len() {
            let mut mutated = certificate.clone();
            mutated.certified_chain[index].file_blake3 = "blake3:forged".to_owned();
            reseal(&mut mutated);
            assert!(
                !replay_milestone_certificate_v1(&mutated).valid,
                "artifact {index}"
            );
        }
    }

    #[test]
    fn every_claim_quantifier_and_register_is_mutation_guarded() {
        let certificate = issue_milestone_certificate_v1().unwrap();
        for index in 0..certificate.claim_bindings.len() {
            let mut quantifier = certificate.clone();
            quantifier.claim_bindings[index].quantifier = Ms1Quantifier::ProgramState;
            if quantifier.claim_bindings[index].quantifier
                == certificate.claim_bindings[index].quantifier
            {
                quantifier.claim_bindings[index].quantifier = Ms1Quantifier::ConeLevel;
            }
            reseal(&mut quantifier);
            assert!(
                !replay_milestone_certificate_v1(&quantifier).valid,
                "claim quantifier {index}"
            );

            let mut register = certificate.clone();
            register.claim_bindings[index].register = Ms1Register::ProgramState;
            if register.claim_bindings[index].register == certificate.claim_bindings[index].register
            {
                register.claim_bindings[index].register = Ms1Register::DiagnosticRegister;
            }
            reseal(&mut register);
            assert!(
                !replay_milestone_certificate_v1(&register).valid,
                "claim register {index}"
            );
        }
    }

    #[test]
    fn every_frontier_row_is_mutation_guarded() {
        let certificate = issue_milestone_certificate_v1().unwrap();
        for index in 0..certificate.dnf4_frontier_verbatim.len() {
            let mut mutated = certificate.clone();
            mutated.dnf4_frontier_verbatim[index]
                .dnf4_row_verbatim
                .as_object_mut()
                .unwrap()
                .insert(
                    "derivation_hash".to_owned(),
                    Value::String("blake3:forged".to_owned()),
                );
            reseal(&mut mutated);
            assert!(
                !replay_milestone_certificate_v1(&mutated).valid,
                "DNF-4 row {index}"
            );
        }
    }

    #[test]
    fn every_adoption_block_is_mutation_guarded() {
        let certificate = issue_milestone_certificate_v1().unwrap();
        for index in 0..certificate.legislative_record.len() {
            let mut mutated = certificate.clone();
            mutated.legislative_record[index]
                .adoption_block_verbatim
                .push_str("\nforged");
            reseal(&mut mutated);
            assert!(
                !replay_milestone_certificate_v1(&mutated).valid,
                "law {index}"
            );
        }
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_milestone_certificate_v1().unwrap();
        let mut value = serde_json::to_value(certificate).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("m4_by_implication".to_owned(), Value::Bool(true));
        let replay = replay_milestone_certificate_v1_json(&serde_json::to_string(&value).unwrap());
        assert!(!replay.valid);
    }
}
