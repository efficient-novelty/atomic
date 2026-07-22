//! Global E-4 assembly after the adopted G-2--G-8 completion sequence.
//!
//! The assembly is fail-fast under F-G4.  A single exact raw-surface member
//! that elaborates but remains in the classifier's typed-Unknown branch is a
//! counterexample to global class exhaustion.  The certificate records that
//! witness and withholds every downstream membership/count output.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_schema::e4_generator_basis::{
    issue_e4_generator_basis_audit, replay_e4_generator_basis_audit,
};
use pen_schema::grammar_completion::{
    GrammarCompletionCertificate, replay_grammar_completion_json,
};
use pen_schema::total_classifier::{
    F_G4_TYPED_UNKNOWN, RawCandidateDecision, TotalClassifierCertificate, classify_raw_candidate,
    replay_total_classifier_json,
};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_ASSEMBLY_SCHEMA: &str = "schema2-global-e4-assembly-v1";
pub const GLOBAL_E4_ASSEMBLY_DATE: &str = "2026-07-20";

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_grammar_completion_plan.md");
const SPEC_BYTES: &[u8] = include_bytes!("../../../docs/step_15_completion_open_problem.md");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const CLASSIFIER_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/total_classifier.rs");
const BASIS_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/e4_generator_basis.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssemblySourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenRawContextRecord {
    pub library_size: u32,
    pub base_scope_size: u32,
    pub max_path_dimension: u32,
    pub include_trunc: bool,
    pub include_modal: bool,
    pub include_temporal: bool,
    pub include_linear_exponential: bool,
    pub max_expr_nodes: u8,
    pub late_family_surface: String,
    pub package_focus_requirements_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fg4WitnessRecord {
    pub candidate: Telescope,
    pub candidate_kappa: usize,
    pub candidate_bit_cost: u32,
    pub raw_surface_membership: RawSurfaceMembership,
    pub classifier_decision: RawCandidateDecision,
    pub exact_raw_catalog_member: bool,
    pub typed_elaboration_succeeded: bool,
    pub classifier_returned_unknown: bool,
    pub named_obstruction: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssemblyForbiddenOutputs {
    pub five_pending_membership_verdicts_issued: bool,
    pub independent_family_tokens_issued: bool,
    pub e2b_executed: bool,
    pub historical_scores_recomputed: bool,
    pub fq2_evaluated: bool,
    pub agent_a_handoff_issued: bool,
    pub halt_or_continuation_claimed: bool,
}

impl AssemblyForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.five_pending_membership_verdicts_issued
            && !self.independent_family_tokens_issued
            && !self.e2b_executed
            && !self.historical_scores_recomputed
            && !self.fq2_evaluated
            && !self.agent_a_handoff_issued
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4AssemblyCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<AssemblySourceBinding>,
    pub grammar_completion_digest: String,
    pub g8_classifier_digest: String,
    pub grammar_completion_replayed: bool,
    pub g8_classifier_replayed: bool,
    pub g8_authorized_global_assembly: bool,
    pub generator_basis_derivation_hash: String,
    pub generator_basis_replayed: bool,
    pub legal_e1_morphism_decomposition_available: bool,
    pub predecessor_basis_global_completeness_false: bool,
    pub assembly_inputs_joined: bool,
    pub frozen_raw_context: FrozenRawContextRecord,
    pub f_g4_witness: Fg4WitnessRecord,
    pub f_g4_triggered: bool,
    pub unknown_class_eliminated: bool,
    pub class_exhaustion_claimed: bool,
    pub global_e4_assembly_executed: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub forbidden_outputs: AssemblyForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4AssemblyReplay {
    pub valid: bool,
    pub assembly_executed: bool,
    pub exact_raw_unknown_witness: bool,
    pub f_g4_triggered: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4AssemblyError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("assembly invariant failed: {0}")]
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_ASSEMBLY_SCHEMA, domain, value))
        .expect("global E-4 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(grammar_bytes: &[u8], classifier_bytes: &[u8]) -> Vec<AssemblySourceBinding> {
    [
        (
            "docs/agent_e_grammar_completion_plan.md",
            "phase_order_and_f_g4",
            PLAN_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "frozen_global_e4_spec",
            SPEC_BYTES,
        ),
        (
            "docs/schema2_grammar_completion_v1.json",
            "g2_g7_predecessor",
            grammar_bytes,
        ),
        (
            "docs/schema2_g8_total_classifier_v1.json",
            "g8_predecessor",
            classifier_bytes,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "exact_raw_catalog_membership",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-schema/src/total_classifier.rs",
            "typed_class_or_f_g4_decider",
            CLASSIFIER_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/e4_generator_basis.rs",
            "legal_morphism_generator_basis",
            BASIS_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly.rs",
            "fail_fast_global_assembly",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| AssemblySourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn frozen_raw_context() -> EnumerationContext {
    EnumerationContext {
        library_size: 15,
        scope_size: 2,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes: 6,
        require_former_eliminator_clauses: false,
        require_initial_hit_clauses: false,
        require_truncation_hit_clauses: false,
        require_higher_hit_clauses: false,
        require_sphere_lift_clauses: false,
        require_axiomatic_bundle_clauses: false,
        require_modal_shell_clauses: false,
        require_connection_shell_clauses: false,
        require_curvature_shell_clauses: false,
        require_operator_bundle_clauses: false,
        require_hilbert_functional_clauses: false,
        require_temporal_shell_clauses: false,
        historical_anchor_ref: None,
        late_family_surface: LateFamilySurface::None,
    }
}

fn frozen_raw_context_record() -> FrozenRawContextRecord {
    FrozenRawContextRecord {
        library_size: 15,
        base_scope_size: 2,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes: 6,
        late_family_surface: "none".to_owned(),
        package_focus_requirements_enabled: false,
    }
}

fn f_g4_witness_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(Expr::Var(1)))),
    ])
}

fn issue_f_g4_witness() -> Result<Fg4WitnessRecord, GlobalE4AssemblyError> {
    let candidate = f_g4_witness_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let classifier_decision = classify_raw_candidate(&candidate);
    let (typed_elaboration_succeeded, classifier_returned_unknown, named_obstruction) =
        match &classifier_decision {
            RawCandidateDecision::NamedTypedObstruction {
                code,
                telescope_class,
                elaboration_derivation_hash,
                ..
            } => (
                !elaboration_derivation_hash.is_empty(),
                *telescope_class == TelescopeClass::Unknown,
                code.clone(),
            ),
            _ => (false, false, String::new()),
        };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || !typed_elaboration_succeeded
        || !classifier_returned_unknown
        || named_obstruction != F_G4_TYPED_UNKNOWN
    {
        return Err(GlobalE4AssemblyError::Invariant(
            "registered F-G4 witness no longer reaches exact raw -> typed Unknown".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "f-g4-live-raw-witness",
        &(
            &candidate,
            &raw_surface_membership,
            &classifier_decision,
            exact_raw_catalog_member,
            typed_elaboration_succeeded,
            classifier_returned_unknown,
            &named_obstruction,
        ),
    );
    Ok(Fg4WitnessRecord {
        candidate_kappa: candidate.kappa(),
        candidate_bit_cost: candidate.bit_cost(),
        candidate,
        raw_surface_membership,
        classifier_decision,
        exact_raw_catalog_member,
        typed_elaboration_succeeded,
        classifier_returned_unknown,
        named_obstruction,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4AssemblyCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-assembly-certificate", &projection)
}

pub fn issue_global_e4_assembly_certificate()
-> Result<GlobalE4AssemblyCertificate, GlobalE4AssemblyError> {
    let grammar_bytes = std::fs::read(workspace_doc_path("schema2_grammar_completion_v1.json"))
        .map_err(|error| GlobalE4AssemblyError::Io(error.to_string()))?;
    let classifier_bytes = std::fs::read(workspace_doc_path("schema2_g8_total_classifier_v1.json"))
        .map_err(|error| GlobalE4AssemblyError::Io(error.to_string()))?;
    let grammar_json = std::str::from_utf8(&grammar_bytes)
        .map_err(|error| GlobalE4AssemblyError::Json(error.to_string()))?;
    let classifier_json = std::str::from_utf8(&classifier_bytes)
        .map_err(|error| GlobalE4AssemblyError::Json(error.to_string()))?;
    let grammar_replay = replay_grammar_completion_json(grammar_json);
    if !grammar_replay.valid || !grammar_replay.g8_authorized {
        return Err(GlobalE4AssemblyError::Prerequisite(format!(
            "G2-G7 replay failed: {}",
            grammar_replay.errors.join("; ")
        )));
    }
    let classifier_replay = replay_total_classifier_json(classifier_json);
    if !classifier_replay.valid || !classifier_replay.global_e4_assembly_authorized {
        return Err(GlobalE4AssemblyError::Prerequisite(format!(
            "G8 replay failed: {}",
            classifier_replay.errors.join("; ")
        )));
    }
    let grammar: GrammarCompletionCertificate = serde_json::from_slice(&grammar_bytes)
        .map_err(|error| GlobalE4AssemblyError::Json(error.to_string()))?;
    let classifier: TotalClassifierCertificate = serde_json::from_slice(&classifier_bytes)
        .map_err(|error| GlobalE4AssemblyError::Json(error.to_string()))?;

    let generator_basis = issue_e4_generator_basis_audit()
        .map_err(|error| GlobalE4AssemblyError::Prerequisite(error.to_string()))?;
    replay_e4_generator_basis_audit(&generator_basis)
        .map_err(|error| GlobalE4AssemblyError::Prerequisite(error.to_string()))?;
    let witness = issue_f_g4_witness()?;

    let grammar_completion_replayed = true;
    let g8_classifier_replayed = true;
    let g8_authorized_global_assembly = true;
    let generator_basis_replayed = true;
    let legal_e1_morphism_decomposition_available =
        generator_basis.current_e1_context_morphism_decomposition_complete();
    let predecessor_basis_global_completeness_false =
        !generator_basis.full_schema2_generator_completeness();
    let assembly_inputs_joined = grammar.full_adopted_grammar_normalization_naturality_complete
        && classifier.total_disjoint_sum_classifier_proved
        && legal_e1_morphism_decomposition_available;
    let f_g4_triggered = witness.exact_raw_catalog_member
        && witness.typed_elaboration_succeeded
        && witness.classifier_returned_unknown;
    let unknown_class_eliminated = false;
    let class_exhaustion_claimed = false;
    let global_e4_assembly_executed = true;
    let global_e4_complete = false;
    let five_pending_memberships_now_authorized = false;
    let forbidden_outputs = AssemblyForbiddenOutputs {
        five_pending_membership_verdicts_issued: false,
        independent_family_tokens_issued: false,
        e2b_executed: false,
        historical_scores_recomputed: false,
        fq2_evaluated: false,
        agent_a_handoff_issued: false,
        halt_or_continuation_claimed: false,
    };
    if !assembly_inputs_joined
        || !f_g4_triggered
        || unknown_class_eliminated
        || class_exhaustion_claimed
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(GlobalE4AssemblyError::Invariant(
            "F-G4 fail-fast disposition drifted".to_owned(),
        ));
    }

    let mut certificate = GlobalE4AssemblyCertificate {
        schema: GLOBAL_E4_ASSEMBLY_SCHEMA.to_owned(),
        date: GLOBAL_E4_ASSEMBLY_DATE.to_owned(),
        source_bindings: source_bindings(&grammar_bytes, &classifier_bytes),
        grammar_completion_digest: grammar.result_digest,
        g8_classifier_digest: classifier.result_digest,
        grammar_completion_replayed,
        g8_classifier_replayed,
        g8_authorized_global_assembly,
        generator_basis_derivation_hash: generator_basis.derivation_hash().to_owned(),
        generator_basis_replayed,
        legal_e1_morphism_decomposition_available,
        predecessor_basis_global_completeness_false,
        assembly_inputs_joined,
        frozen_raw_context: frozen_raw_context_record(),
        f_g4_witness: witness,
        f_g4_triggered,
        unknown_class_eliminated,
        class_exhaustion_claimed,
        global_e4_assembly_executed,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_assembly_executed_f_g4_typed_unknown_witness_global_e4_incomplete"
            .to_owned(),
        permitted_conclusion: "The adopted grammar, total classifier, and legal-morphism basis assemble far enough to expose a live exact raw candidate in the typed-Unknown branch. F-G4 fires, so global E-4 completeness and the five pending membership verdicts remain unavailable."
            .to_owned(),
        required_successor_action: "Adjudicate a versioned grammar/classifier successor for the mixed formation/introduction shape beginning with [Univ, Lam(Var(1))], then rerun global E-4 assembly from a create-new artifact."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4AssemblyReplay {
    GlobalE4AssemblyReplay {
        valid: false,
        assembly_executed: false,
        exact_raw_unknown_witness: false,
        f_g4_triggered: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_assembly_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_global_e4_assembly_certificate(
    certificate: &GlobalE4AssemblyCertificate,
) -> GlobalE4AssemblyReplay {
    let expected = match issue_global_e4_assembly_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

fn replay_against_expected(
    certificate: &GlobalE4AssemblyCertificate,
    expected: &GlobalE4AssemblyCertificate,
) -> GlobalE4AssemblyReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4AssemblyReplay {
        valid: errors.is_empty(),
        assembly_executed: certificate.global_e4_assembly_executed,
        exact_raw_unknown_witness: certificate.f_g4_witness.exact_raw_catalog_member
            && certificate.f_g4_witness.typed_elaboration_succeeded
            && certificate.f_g4_witness.classifier_returned_unknown,
        f_g4_triggered: certificate.f_g4_triggered,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_assembly_json(json: &str) -> GlobalE4AssemblyReplay {
    match serde_json::from_str::<GlobalE4AssemblyCertificate>(json) {
        Ok(certificate) => replay_global_e4_assembly_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_global_e4_assembly_create_new(
    path: &Path,
) -> Result<GlobalE4AssemblyReplay, GlobalE4AssemblyError> {
    let certificate = issue_global_e4_assembly_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4AssemblyError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4AssemblyError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4AssemblyError::Io(error.to_string()))?;
    let replay = replay_global_e4_assembly_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4AssemblyError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_raw_member_is_a_typed_unknown_f_g4_witness() {
        let witness = issue_f_g4_witness().expect("witness issues");
        assert!(witness.raw_surface_membership.is_member);
        assert!(witness.typed_elaboration_succeeded);
        assert!(witness.classifier_returned_unknown);
        assert_eq!(witness.named_obstruction, F_G4_TYPED_UNKNOWN);
    }

    #[test]
    fn assembly_executes_but_cannot_claim_global_e4() {
        let certificate = issue_global_e4_assembly_certificate().expect("assembly issues");
        assert!(certificate.global_e4_assembly_executed);
        assert!(certificate.f_g4_triggered);
        assert!(!certificate.global_e4_complete);
        assert!(!certificate.five_pending_memberships_now_authorized);
        assert!(certificate.forbidden_outputs.all_withheld());
    }

    #[test]
    fn replay_and_mutation_fail_closed() {
        let certificate = issue_global_e4_assembly_certificate().expect("assembly issues");
        assert!(replay_against_expected(&certificate, &certificate).valid);

        let mut mutations = Vec::new();

        let mut source = certificate.clone();
        source.source_bindings[0].blake3.push('0');
        mutations.push(("source binding", source));

        let mut prerequisite = certificate.clone();
        prerequisite.g8_classifier_digest.push('0');
        mutations.push(("prerequisite digest", prerequisite));

        let mut raw_membership = certificate.clone();
        raw_membership.f_g4_witness.raw_surface_membership.is_member = false;
        mutations.push(("raw membership", raw_membership));

        let mut witness_role = certificate.clone();
        witness_role.f_g4_witness.candidate.clauses[1].role = ClauseRole::Formation;
        mutations.push(("witness role", witness_role));

        let mut obstruction = certificate.clone();
        obstruction.f_g4_witness.named_obstruction.push('0');
        mutations.push(("named obstruction", obstruction));

        let mut f_g4 = certificate.clone();
        f_g4.f_g4_triggered = false;
        mutations.push(("F-G4 disposition", f_g4));

        let mut completeness = certificate.clone();
        completeness.global_e4_complete = true;
        mutations.push(("global completeness", completeness));

        let mut authorization = certificate.clone();
        authorization.five_pending_memberships_now_authorized = true;
        mutations.push(("membership authorization", authorization));

        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(("forbidden E-2b output", forbidden));

        let mut digest = certificate.clone();
        digest.result_digest.push('0');
        mutations.push(("result digest", digest));

        for (label, mutated) in mutations {
            assert!(
                !replay_against_expected(&mutated, &certificate).valid,
                "mutation replayed: {label}"
            );
        }
    }
}
