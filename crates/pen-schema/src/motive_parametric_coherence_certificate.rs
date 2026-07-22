//! Source-bound certificate for motive-parametric instantiation coherence.

use crate::internal_classifier_branch_v7::declared_context_control_candidate;
use crate::internal_classifier_branch_v8::{
    WrappedCandidateDecisionV9, classify_wrapped_candidate_v9, wrap_candidate,
};
use crate::internal_classifier_branch_v9::{
    WrappedCandidateDecisionV10, classify_wrapped_candidate_v10,
};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_type::contextual_internality::ContextualMotive;
use pen_type::motive_parametric_coherence::{
    CLOSURE_RULE_INVENTORY, MOTIVE_PARAMETRIC_COHERENCE_VERSION, MotiveParametricCoherenceError,
    MotiveParametricCoherenceTheoremProjection, issue_motive_parametric_coherence_theorem,
    replay_motive_parametric_coherence_theorem,
};
use pen_type::substitution::EXPR_CONSTRUCTOR_COVERAGE;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const MOTIVE_PARAMETRIC_COHERENCE_SCHEMA: &str = "schema2-motive-parametric-coherence-v1";
pub const MOTIVE_PARAMETRIC_COHERENCE_DATE: &str = "2026-07-21";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/motive_parametric_coherence_adjudication.md");
const V9_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v9.json");
const SUBSTITUTION_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/substitution.rs");
const CONTEXTUAL_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/contextual_internality.rs");
const THEOREM_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence.rs");
const PREDECESSOR_CLASSIFIER_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v8.rs");
const SUCCESSOR_CLASSIFIER_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v9.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("motive_parametric_coherence_certificate.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveParametricSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V9DomainBinding {
    pub result_digest: String,
    pub wrapper_version: String,
    pub motive_node_cap: u32,
    pub syntactic_motive_count_upper_bound: u128,
    pub finite: bool,
    pub silent_truncation: bool,
    pub predecessor_unknown_named_gap: String,
    pub predecessor_f_a5_retained: bool,
    pub byte_binding_matches: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveParametricFalsifierRecord {
    pub f_m1_all_six_rule_kinds_covered: bool,
    pub f_m1_all_expression_constructors_covered: bool,
    pub f_m1_every_rule_specialization_replayed: bool,
    pub f_m1_mutated_rule_proof_rejected: bool,
    pub f_m1_no_soundness_event_found: bool,
    pub f_m2_full_v9_syntactic_upper_bound_preserved: bool,
    pub f_m2_generic_over_all_motives: bool,
    pub f_m2_no_motive_grammar_restriction: bool,
    pub probes_demoted_to_regression: bool,
    pub vacuous_uninhabited_case_preserved: bool,
    pub marginal_nu_zero: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveParametricCoherenceCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<MotiveParametricSourceBinding>,
    pub adopted_rule: String,
    pub v9_domain: V9DomainBinding,
    pub generic_theorem: MotiveParametricCoherenceTheoremProjection,
    pub generic_theorem_replayed: bool,
    pub v9_survivor_predecessor_decision: WrappedCandidateDecisionV9,
    pub v9_survivor_successor_decision: WrappedCandidateDecisionV10,
    pub v9_survivor_now_internal: bool,
    pub marginal_nu: u32,
    pub falsifiers: MotiveParametricFalsifierRecord,
    pub global_e4_v10_authorized: bool,
    pub downstream_outputs_withheld: bool,
    pub outcome: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveParametricCoherenceReplay {
    pub valid: bool,
    pub theorem_replayed: bool,
    pub all_six_rule_kinds_covered: bool,
    pub no_soundness_event_found: bool,
    pub no_motive_filter: bool,
    pub v9_survivor_resolved: bool,
    pub marginal_nu: u32,
    pub global_e4_v10_authorized: bool,
    pub downstream_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum MotiveParametricCertificateError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn workspace_doc_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs")
        .join(name)
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(MOTIVE_PARAMETRIC_COHERENCE_SCHEMA, domain, value))
        .expect("motive-parametric certificate serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(v9_bytes: &[u8]) -> Vec<MotiveParametricSourceBinding> {
    [
        (
            "docs/motive_parametric_coherence_adjudication.md",
            "adopted_theorem_and_falsifiers",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v9.json",
            "sealed_wrapped_domain",
            v9_bytes,
        ),
        (
            "crates/pen-type/src/substitution.rs",
            "full_expression_substitution",
            SUBSTITUTION_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/contextual_internality.rs",
            "typed_precheck_and_probe_predecessor",
            CONTEXTUAL_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence.rs",
            "six_rule_generic_induction",
            THEOREM_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v8.rs",
            "wrapped_predecessor_classifier",
            PREDECESSOR_CLASSIFIER_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v9.rs",
            "parametric_successor_classifier",
            SUCCESSOR_CLASSIFIER_BYTES,
        ),
        (
            "crates/pen-schema/src/motive_parametric_coherence_certificate.rs",
            "source_bound_certificate",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| MotiveParametricSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn parse_v9_domain(bytes: Vec<u8>) -> Result<V9DomainBinding, MotiveParametricCertificateError> {
    let worker = std::thread::Builder::new()
        .name("motive-parametric-v9-summary".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(move || -> Result<V9DomainBinding, String> {
            let value: serde_json::Value =
                serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            let string = |p: &str| {
                value
                    .pointer(p)
                    .and_then(|v| v.as_str())
                    .map(str::to_owned)
                    .ok_or_else(|| format!("missing {p}"))
            };
            let number = |p: &str| {
                value
                    .pointer(p)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| format!("missing {p}"))
            };
            let boolean = |p: &str| {
                value
                    .pointer(p)
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| format!("missing {p}"))
            };
            Ok(V9DomainBinding {
                result_digest: string("/result_digest")?,
                wrapper_version: string("/wrapped_domain/wrapper_version")?,
                motive_node_cap: number("/wrapped_domain/finiteness/motive_node_cap")? as u32,
                syntactic_motive_count_upper_bound: number(
                    "/wrapped_domain/finiteness/syntactic_motive_count_upper_bound",
                )? as u128,
                finite: boolean("/wrapped_domain/finiteness/finite")?,
                silent_truncation: boolean("/wrapped_domain/finiteness/silent_truncation")?,
                predecessor_unknown_named_gap: string(
                    "/wrapped_fiber_scan/next_unknown/named_gap",
                )?,
                predecessor_f_a5_retained: boolean(
                    "/wrapped_fiber_scan/next_unknown/f_a5_retained",
                )?,
                byte_binding_matches: true,
            })
        })
        .map_err(|e| MotiveParametricCertificateError::Json(e.to_string()))?;
    worker
        .join()
        .map_err(|_| MotiveParametricCertificateError::Json("v9 parser panicked".to_owned()))?
        .map_err(MotiveParametricCertificateError::Json)
}

fn certificate_digest(certificate: &MotiveParametricCoherenceCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("motive-parametric-coherence-certificate", &projection)
}

pub fn issue_motive_parametric_coherence_certificate()
-> Result<MotiveParametricCoherenceCertificate, MotiveParametricCertificateError> {
    let v9_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v9.json"))
        .map_err(|e| MotiveParametricCertificateError::Io(e.to_string()))?;
    let mut v9_domain = parse_v9_domain(v9_bytes.clone())?;
    v9_domain.byte_binding_matches = v9_bytes == V9_ARTIFACT_BYTES;
    if !v9_domain.byte_binding_matches
        || !v9_domain.finite
        || v9_domain.silent_truncation
        || v9_domain.motive_node_cap != 6
        || v9_domain.syntactic_motive_count_upper_bound != 914_612
        || !v9_domain.predecessor_f_a5_retained
    {
        return Err(MotiveParametricCertificateError::Prerequisite(format!(
            "v9 domain drifted: {v9_domain:?}"
        )));
    }

    let theorem = issue_motive_parametric_coherence_theorem()
        .map_err(|e| MotiveParametricCertificateError::Invariant(e.to_string()))?;
    replay_motive_parametric_coherence_theorem(theorem.projection())
        .map_err(|e| MotiveParametricCertificateError::Invariant(e.to_string()))?;
    let generic_theorem = theorem.projection().clone();
    let generic_theorem_replayed = true;
    let survivor = wrap_candidate(
        declared_context_control_candidate(),
        vec![ContextualMotive::Function {
            domain: Box::new(ContextualMotive::Type),
            codomain: Box::new(ContextualMotive::Element(Expr::Univ)),
        }],
    )
    .map_err(|e| MotiveParametricCertificateError::Invariant(e.to_string()))?;
    let predecessor = classify_wrapped_candidate_v9(&survivor)
        .map_err(|e| MotiveParametricCertificateError::Invariant(e.to_string()))?;
    let successor = classify_wrapped_candidate_v10(&survivor)
        .map_err(|e| MotiveParametricCertificateError::Invariant(e.to_string()))?;
    let (v9_survivor_now_internal, marginal_nu) = match &successor {
        WrappedCandidateDecisionV10::InternalParametricContextual { marginal_nu, .. } => {
            (true, *marginal_nu)
        }
        _ => (false, u32::MAX),
    };
    if !matches!(
        predecessor,
        WrappedCandidateDecisionV9::NamedTypedObstruction { .. }
    ) || !v9_survivor_now_internal
        || marginal_nu != 0
    {
        return Err(MotiveParametricCertificateError::Invariant(
            "v9 survivor did not resolve at zero credit".to_owned(),
        ));
    }

    let mut mutation = generic_theorem.clone();
    mutation.rule_cases[0].rule_preserved = false;
    let mutation_rejected = matches!(
        replay_motive_parametric_coherence_theorem(&mutation),
        Err(MotiveParametricCoherenceError::TheoremReplayMismatch)
    );
    let six = generic_theorem.closure_rule_inventory.len() == CLOSURE_RULE_INVENTORY.len()
        && generic_theorem.every_rule_covered_exactly_once;
    let constructors =
        generic_theorem.expression_constructor_inventory.len() == EXPR_CONSTRUCTOR_COVERAGE.len();
    let rules_replay = generic_theorem
        .rule_cases
        .iter()
        .all(|case| case.rule_preserved && case.structural_recursion_used);
    let no_soundness_event = six && constructors && rules_replay && mutation_rejected;
    let full_bound = v9_domain.syntactic_motive_count_upper_bound == 914_612;
    let no_filter =
        generic_theorem.generic_over_motives && !generic_theorem.motive_grammar_restriction_applied;
    let probes_demoted = !generic_theorem.registered_probes_used_as_evidence;
    let vacuous = generic_theorem.uninhabited_motive_case_vacuous;
    let zero = marginal_nu == 0 && generic_theorem.marginal_nu == 0;
    let derivation_hash = tagged_hash(
        "motive-parametric-falsifiers",
        &(
            six,
            constructors,
            rules_replay,
            mutation_rejected,
            no_soundness_event,
            full_bound,
            no_filter,
            probes_demoted,
            vacuous,
            zero,
        ),
    );
    let falsifiers = MotiveParametricFalsifierRecord {
        f_m1_all_six_rule_kinds_covered: six,
        f_m1_all_expression_constructors_covered: constructors,
        f_m1_every_rule_specialization_replayed: rules_replay,
        f_m1_mutated_rule_proof_rejected: mutation_rejected,
        f_m1_no_soundness_event_found: no_soundness_event,
        f_m2_full_v9_syntactic_upper_bound_preserved: full_bound,
        f_m2_generic_over_all_motives: generic_theorem.generic_over_motives,
        f_m2_no_motive_grammar_restriction: !generic_theorem.motive_grammar_restriction_applied,
        probes_demoted_to_regression: probes_demoted,
        vacuous_uninhabited_case_preserved: vacuous,
        marginal_nu_zero: zero,
        derivation_hash,
    };
    if !no_soundness_event || !full_bound || !no_filter || !probes_demoted || !vacuous || !zero {
        return Err(MotiveParametricCertificateError::Invariant(
            "F-M1 or F-M2 failed".to_owned(),
        ));
    }

    let mut certificate = MotiveParametricCoherenceCertificate {
        schema: MOTIVE_PARAMETRIC_COHERENCE_SCHEMA.to_owned(),
        date: MOTIVE_PARAMETRIC_COHERENCE_DATE.to_owned(),
        source_bindings: source_bindings(&v9_bytes),
        adopted_rule: MOTIVE_PARAMETRIC_COHERENCE_VERSION.to_owned(),
        v9_domain,
        generic_theorem,
        generic_theorem_replayed,
        v9_survivor_predecessor_decision: predecessor,
        v9_survivor_successor_decision: successor,
        v9_survivor_now_internal,
        marginal_nu,
        falsifiers,
        global_e4_v10_authorized: true,
        downstream_outputs_withheld: true,
        outcome: "motive_parametric_coherence_proved_all_six_closure_rules_v9_survivor_internal".to_owned(),
        required_successor_action: "Run global E-4 v10 create-new over the full wrapped domain; F-A5 remains authoritative.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> MotiveParametricCoherenceReplay {
    MotiveParametricCoherenceReplay {
        valid: false,
        theorem_replayed: false,
        all_six_rule_kinds_covered: false,
        no_soundness_event_found: false,
        no_motive_filter: false,
        v9_survivor_resolved: false,
        marginal_nu: u32::MAX,
        global_e4_v10_authorized: false,
        downstream_outputs_withheld: false,
        outcome: "motive_parametric_coherence_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &MotiveParametricCoherenceCertificate,
    expected: &MotiveParametricCoherenceCertificate,
) -> MotiveParametricCoherenceReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    MotiveParametricCoherenceReplay {
        valid: errors.is_empty(),
        theorem_replayed: certificate.generic_theorem_replayed,
        all_six_rule_kinds_covered: certificate.falsifiers.f_m1_all_six_rule_kinds_covered,
        no_soundness_event_found: certificate.falsifiers.f_m1_no_soundness_event_found,
        no_motive_filter: certificate.falsifiers.f_m2_no_motive_grammar_restriction,
        v9_survivor_resolved: certificate.v9_survivor_now_internal,
        marginal_nu: certificate.marginal_nu,
        global_e4_v10_authorized: certificate.global_e4_v10_authorized,
        downstream_outputs_withheld: certificate.downstream_outputs_withheld,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_motive_parametric_coherence_certificate(
    certificate: &MotiveParametricCoherenceCertificate,
) -> MotiveParametricCoherenceReplay {
    match issue_motive_parametric_coherence_certificate() {
        Ok(expected) => replay_against_expected(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn replay_motive_parametric_coherence_json(json: &str) -> MotiveParametricCoherenceReplay {
    match serde_json::from_str::<MotiveParametricCoherenceCertificate>(json) {
        Ok(certificate) => replay_motive_parametric_coherence_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_motive_parametric_coherence_create_new(
    path: &Path,
) -> Result<MotiveParametricCoherenceReplay, MotiveParametricCertificateError> {
    let certificate = issue_motive_parametric_coherence_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|e| MotiveParametricCertificateError::Json(e.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| MotiveParametricCertificateError::Io(e.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|e| MotiveParametricCertificateError::Io(e.to_string()))?;
    let replay = replay_motive_parametric_coherence_certificate(&certificate);
    if !replay.valid {
        return Err(MotiveParametricCertificateError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn theorem_certificate_resolves_v9_without_filtering() {
        let certificate = issue_motive_parametric_coherence_certificate().expect("certificate");
        assert!(certificate.falsifiers.f_m1_no_soundness_event_found);
        assert!(certificate.falsifiers.f_m2_no_motive_grammar_restriction);
        assert_eq!(
            certificate.v9_domain.syntactic_motive_count_upper_bound,
            914_612
        );
        assert!(certificate.v9_survivor_now_internal);
        assert_eq!(certificate.marginal_nu, 0);
    }
    #[test]
    fn certificate_mutations_fail_closed() {
        let certificate = issue_motive_parametric_coherence_certificate().expect("certificate");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutation = certificate.clone();
        mutation.falsifiers.f_m2_no_motive_grammar_restriction = false;
        assert!(!replay_against_expected(&mutation, &certificate).valid);
    }
}
