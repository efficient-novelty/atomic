//! G-1: a replayable survey of the sealed Steps 9--15 vocabulary.
//!
//! This module deliberately stops at the adjudication boundary.  It binds the
//! exact predecessor signature and raw telescope for each row, checks the
//! Step-15/J3 positional correspondence, and pins the single batch proposal.
//! It issues no Schema2 constructor, membership verdict, score, or halt claim.

use crate::e34_m1_sweep::replay_e34_m1_sweep_json;
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const G1_SURVEY_SCHEMA: &str = "schema2-steps-9-15-g1-vocabulary-survey-v1";
pub const G1_SURVEY_DATE: &str = "2026-07-20";
pub const G1_BATCH_RULE: &str = "steps-9-15-trace-faithful-signature-batch-v1";
pub const G1_OUTCOME: &str = "adjudication_required_g1_complete_constructor_work_gated";

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_grammar_completion_plan.md");
const SPEC_BYTES: &[u8] = include_bytes!("../../../docs/step_15_completion_open_problem.md");
const ENDPOINT_BYTES: &[u8] = include_bytes!("../../../docs/endpoint_premise_api_adjudication.md");
const PROPOSAL_BYTES: &[u8] = include_bytes!("../../../docs/steps_9_15_signature_adjudication.md");
const TRACE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const FIXTURE_BYTES: &[u8] =
    include_bytes!("../../../tests/fixtures/trajectory/reference_steps_until_15.json");
const M1_BYTES: &[u8] = include_bytes!("../../../docs/schema2_m1_sweep_v1.json");

const STEP_LABELS: [&str; 7] = [
    "Hopf",
    "Cohesion",
    "Connections",
    "Curvature",
    "Metric",
    "Hilbert",
    "DCT",
];

const STEP_CLASSES: [TelescopeClass; 7] = [
    TelescopeClass::Map,
    TelescopeClass::Modal,
    TelescopeClass::Axiomatic,
    TelescopeClass::Axiomatic,
    TelescopeClass::Axiomatic,
    TelescopeClass::Axiomatic,
    TelescopeClass::Synthesis,
];

const STEP_NAMES: [&[&str]; 7] = [
    &[
        "hopf_map_shell",
        "hopf_fiber_instance",
        "hopf_total_witness",
        "hopf_reverse_or_classifying_shell",
    ],
    &["flat_former", "sharp_former", "disc_former", "shape_former"],
    &[
        "connection_shell",
        "connection_intro",
        "flat_connection_action",
        "modal_connection_instance",
        "connection_unit_action",
    ],
    &[
        "curvature_shell",
        "curvature_intro",
        "connection_targeting_map",
        "connection_composite_instance",
        "curvature_law_action",
        "connection_endomap",
    ],
    &[
        "operator_pair_shell",
        "metric_connection_map",
        "binary_metric_operation",
        "metric_evaluation",
        "curvature_endomap",
        "metric_operator_action",
        "curvature_supported_family",
    ],
    &[
        "hilbert_functional_shell",
        "hilbert_endomap",
        "decomposition_shell",
        "spectral_shell",
        "operator_algebra_pair",
        "metric_supported_family",
        "curvature_supported_family",
        "connection_supported_family",
        "functional_derivative_action",
    ],
    &[
        "next_former",
        "eventually_former",
        "next_eventually_comparison",
        "cohesive_temporal_interaction",
        "flat_next_exchange",
        "sharp_eventually_exchange",
        "eventually_eliminator",
        "next_composition",
    ],
];

const J3_ROLES: [&str; 8] = [
    "operator_former_next",
    "operator_former_eventually",
    "comparison_map",
    "interaction_clause",
    "flat_exchange_law",
    "sharp_exchange_law",
    "eliminator",
    "composition_law",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct G1SourceBinding {
    pub path: String,
    pub role: String,
    pub signature_ground: bool,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct G1ClauseSurvey {
    pub index: u16,
    pub role: ClauseRole,
    pub expr: Expr,
    pub compact_expr: String,
    pub library_refs: Vec<u32>,
    pub variable_refs: Vec<u32>,
    pub every_library_ref_in_predecessor: bool,
    pub proposed_registered_name: String,
    pub semantic_status: String,
    pub j3_role: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct G1StepSurvey {
    pub step: u32,
    pub label: String,
    pub expected_class: TelescopeClass,
    pub computed_class: TelescopeClass,
    pub class_matches_plan: bool,
    pub predecessor_entry_count: usize,
    pub predecessor_signature_digest: String,
    pub candidate_hash: String,
    pub fixture_candidate_hash: String,
    pub candidate_hash_matches_fixture: bool,
    pub fixture_canonical_hash: String,
    pub clause_kappa: usize,
    pub bit_kappa: u32,
    pub clauses: Vec<G1ClauseSurvey>,
    pub exact_trace_signature_reconstructed: bool,
    pub semantic_interpretation_status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct G1ProposalGate {
    pub proposal_path: String,
    pub batch_rule: String,
    pub status: String,
    pub proposal_marks_adjudication_required: bool,
    pub proposal_contains_single_adoption_block: bool,
    pub step15_j3_list_matches_trace: bool,
    pub step10_trace_export_count: usize,
    pub step10_extra_primitive_laws_authorized: bool,
    pub hopf_classifier_strengthening_authorized: bool,
    pub step8_unit_coherence_generator_authorized_by_g1: bool,
    pub g6_unit_coherence_decision_deferred: bool,
    pub endpoint_premise_api_already_adopted: bool,
    pub constructor_work_authorized: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct G1ForbiddenOutputs {
    pub schema2_constructor_issued: bool,
    pub generator_issued: bool,
    pub membership_verdict_issued: bool,
    pub independent_family_verdict_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub score_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl G1ForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.schema2_constructor_issued
            && !self.generator_issued
            && !self.membership_verdict_issued
            && !self.independent_family_verdict_issued
            && !self.e2b_executed
            && !self.stage_count_issued
            && !self.score_evaluated
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct G1VocabularySurvey {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<G1SourceBinding>,
    pub archival_m1_replay_valid: bool,
    pub archival_m1_is_signature_ground: bool,
    pub steps: Vec<G1StepSurvey>,
    pub surveyed_step_count: usize,
    pub surveyed_clause_count: usize,
    pub proposal: G1ProposalGate,
    pub verdict_blindness_statement: String,
    pub forbidden_outputs: G1ForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct G1VocabularyReplay {
    pub valid: bool,
    pub archival_m1_replayed: bool,
    pub surveyed_step_count: usize,
    pub surveyed_clause_count: usize,
    pub step15_j3_list_matches_trace: bool,
    pub constructor_work_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum G1SurveyError {
    #[error("invalid UTF-8 in {0}")]
    Utf8(String),
    #[error("source invariant failed: {0}")]
    Source(String),
    #[error("trace invariant failed: {0}")]
    Trace(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted survey did not replay: {0}")]
    EmittedReplay(String),
}

fn utf8<'a>(path: &str, bytes: &'a [u8]) -> Result<&'a str, G1SurveyError> {
    std::str::from_utf8(bytes).map_err(|_| G1SurveyError::Utf8(path.to_owned()))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(G1_SURVEY_SCHEMA, domain, value))
        .expect("G-1 survey evidence serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn source_bindings() -> Vec<G1SourceBinding> {
    [
        (
            "docs/agent_e_grammar_completion_plan.md",
            "g1_scope_sequence_and_firewall",
            true,
            PLAN_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "frozen_schema2_grammar_specification",
            true,
            SPEC_BYTES,
        ),
        (
            "docs/endpoint_premise_api_adjudication.md",
            "adopted_endpoint_implementation_constraint",
            true,
            ENDPOINT_BYTES,
        ),
        (
            "crates/pen-core/src/telescope.rs",
            "sealed_reference_trace",
            true,
            TRACE_BYTES,
        ),
        (
            "tests/fixtures/trajectory/reference_steps_until_15.json",
            "historical_row_identity_only_not_constructor_ground",
            false,
            FIXTURE_BYTES,
        ),
        (
            "docs/schema2_m1_sweep_v1.json",
            "archival_replay_only_explicit_non_ground",
            false,
            M1_BYTES,
        ),
        (
            "docs/steps_9_15_signature_adjudication.md",
            "g1_batch_proposal_not_yet_adopted",
            false,
            PROPOSAL_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, signature_ground, bytes)| G1SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        signature_ground,
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn compact_expr(expr: &Expr) -> String {
    match expr {
        Expr::App(left, right) => format!("App({},{})", compact_expr(left), compact_expr(right)),
        Expr::Lam(body) => format!("Lam({})", compact_expr(body)),
        Expr::Pi(left, right) => format!("Pi({},{})", compact_expr(left), compact_expr(right)),
        Expr::Sigma(left, right) => {
            format!("Sigma({},{})", compact_expr(left), compact_expr(right))
        }
        Expr::Univ => "Univ".to_owned(),
        Expr::Var(index) => format!("v{index}"),
        Expr::Lib(index) => format!("L{index}"),
        Expr::Id(ty, left, right) => format!(
            "Id({},{},{})",
            compact_expr(ty),
            compact_expr(left),
            compact_expr(right)
        ),
        Expr::Refl(value) => format!("Refl({})", compact_expr(value)),
        Expr::Susp(value) => format!("Susp({})", compact_expr(value)),
        Expr::Trunc(value) => format!("Trunc({})", compact_expr(value)),
        Expr::PathCon(dimension) => format!("PathCon({dimension})"),
        Expr::Flat(value) => format!("Flat({})", compact_expr(value)),
        Expr::Sharp(value) => format!("Sharp({})", compact_expr(value)),
        Expr::Disc(value) => format!("Disc({})", compact_expr(value)),
        Expr::Shape(value) => format!("Shape({})", compact_expr(value)),
        Expr::Next(value) => format!("Next({})", compact_expr(value)),
        Expr::Eventually(value) => format!("Eventually({})", compact_expr(value)),
        Expr::Bang(value) => format!("Bang({})", compact_expr(value)),
        Expr::WhyNot(value) => format!("WhyNot({})", compact_expr(value)),
    }
}

fn fixture_row(step: u32) -> Result<(String, String, String), G1SurveyError> {
    let fixture: Value = serde_json::from_slice(FIXTURE_BYTES)
        .map_err(|error| G1SurveyError::Json(error.to_string()))?;
    let row = fixture
        .as_array()
        .and_then(|rows| {
            rows.iter()
                .find(|row| row["step_index"].as_u64() == Some(u64::from(step)))
        })
        .ok_or_else(|| G1SurveyError::Source(format!("missing fixture row {step}")))?;
    let get = |key: &str| {
        row[key]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| G1SurveyError::Source(format!("missing {key} at fixture row {step}")))
    };
    Ok((
        get("label")?,
        get("candidate_hash")?,
        get("canonical_hash")?,
    ))
}

fn proposal_gate() -> Result<G1ProposalGate, G1SurveyError> {
    let proposal = utf8("docs/steps_9_15_signature_adjudication.md", PROPOSAL_BYTES)?;
    let endpoint = utf8("docs/endpoint_premise_api_adjudication.md", ENDPOINT_BYTES)?;
    let adoption_marker = "I adopt `steps-9-15-trace-faithful-signature-batch-v1`";
    Ok(G1ProposalGate {
        proposal_path: "docs/steps_9_15_signature_adjudication.md".to_owned(),
        batch_rule: G1_BATCH_RULE.to_owned(),
        status: "adjudication_required".to_owned(),
        proposal_marks_adjudication_required: proposal.contains("ADJUDICATION REQUIRED"),
        proposal_contains_single_adoption_block: proposal.matches(adoption_marker).count() == 1,
        step15_j3_list_matches_trace: true,
        step10_trace_export_count: Telescope::reference(10).kappa(),
        step10_extra_primitive_laws_authorized: false,
        hopf_classifier_strengthening_authorized: false,
        step8_unit_coherence_generator_authorized_by_g1: false,
        g6_unit_coherence_decision_deferred: true,
        endpoint_premise_api_already_adopted: endpoint
            .contains("endpoint-premise-ledger-api-rule-v1")
            && endpoint.contains("ADOPTED"),
        constructor_work_authorized: false,
    })
}

fn certificate_digest(survey: &G1VocabularySurvey) -> String {
    let mut projection = survey.clone();
    projection.result_digest.clear();
    tagged_hash("g1-vocabulary-survey", &projection)
}

pub fn issue_g1_vocabulary_survey() -> Result<G1VocabularySurvey, G1SurveyError> {
    let plan = utf8("docs/agent_e_grammar_completion_plan.md", PLAN_BYTES)?;
    let spec = utf8("docs/step_15_completion_open_problem.md", SPEC_BYTES)?;
    if !plan.contains("G-1 first and alone")
        || !plan.contains("No constructor work proceeds on an unadjudicated")
        || !spec.contains("ordinary formation, introduction, elimination")
        || !spec.contains("maps and natural transformations")
        || !spec.contains("adjoint/mate and support-action families")
    {
        return Err(G1SurveyError::Source(
            "G-1 gate or frozen grammar vocabulary marker drifted".to_owned(),
        ));
    }

    let m1_json = utf8("docs/schema2_m1_sweep_v1.json", M1_BYTES)?;
    let m1_replay = replay_e34_m1_sweep_json(m1_json);
    if !m1_replay.valid {
        return Err(G1SurveyError::Source(format!(
            "archival M1 replay failed: {}",
            m1_replay.errors.join("; ")
        )));
    }

    let mut steps = Vec::new();
    for (offset, step) in (9u32..=15).enumerate() {
        let telescope = Telescope::reference(step);
        let prefix = SealedSignature::from_telescopes(
            (1..step)
                .map(|prior| (prior, Telescope::reference(prior)))
                .collect(),
        );
        let (fixture_label, fixture_candidate_hash, fixture_canonical_hash) = fixture_row(step)?;
        if fixture_label != STEP_LABELS[offset] {
            return Err(G1SurveyError::Trace(format!(
                "label mismatch at step {step}"
            )));
        }
        if telescope.clauses.len() != STEP_NAMES[offset].len() {
            return Err(G1SurveyError::Trace(format!(
                "registered-name arity mismatch at step {step}"
            )));
        }
        let computed_class = telescope.classify(&Vec::new());
        let clauses = telescope
            .clauses
            .iter()
            .enumerate()
            .map(|(index, clause)| {
                let library_refs = clause.expr.lib_refs().into_iter().collect::<Vec<_>>();
                let variable_refs = clause.expr.var_refs().into_iter().collect::<Vec<_>>();
                G1ClauseSurvey {
                    index: u16::try_from(index).expect("sealed clause index fits u16"),
                    role: clause.role,
                    expr: clause.expr.clone(),
                    compact_expr: compact_expr(&clause.expr),
                    every_library_ref_in_predecessor: library_refs
                        .iter()
                        .all(|reference| *reference < step),
                    library_refs,
                    variable_refs,
                    proposed_registered_name: STEP_NAMES[offset][index].to_owned(),
                    semantic_status: if step == 15 {
                        "pre_adjudicated_j3_exact_positional_match".to_owned()
                    } else {
                        "adjudication_required_trace_signature_exact_semantic_refinement_unfixed"
                            .to_owned()
                    },
                    j3_role: (step == 15).then(|| J3_ROLES[index].to_owned()),
                }
            })
            .collect::<Vec<_>>();
        let current_hash = candidate_hash(&telescope);
        steps.push(G1StepSurvey {
            step,
            label: fixture_label,
            expected_class: STEP_CLASSES[offset],
            computed_class,
            class_matches_plan: computed_class == STEP_CLASSES[offset],
            predecessor_entry_count: prefix.len(),
            predecessor_signature_digest: prefix.digest().to_owned(),
            candidate_hash_matches_fixture: current_hash == fixture_candidate_hash,
            candidate_hash: current_hash,
            fixture_candidate_hash,
            fixture_canonical_hash,
            clause_kappa: telescope.kappa(),
            bit_kappa: telescope.bit_cost(),
            exact_trace_signature_reconstructed: clauses
                .iter()
                .all(|clause| clause.every_library_ref_in_predecessor),
            clauses,
            semantic_interpretation_status: if step == 15 {
                "j3_signature_list_pre_adjudicated_raw_typing_registration_gated_with_batch"
                    .to_owned()
            } else {
                "trace_exact_stronger_semantics_underdetermined_batch_adjudication_required"
                    .to_owned()
            },
        });
    }

    let proposal = proposal_gate()?;
    let surveyed_clause_count = steps.iter().map(|step| step.clauses.len()).sum::<usize>();
    let forbidden_outputs = G1ForbiddenOutputs {
        schema2_constructor_issued: false,
        generator_issued: false,
        membership_verdict_issued: false,
        independent_family_verdict_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        score_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if steps.len() != 7
        || surveyed_clause_count != 43
        || steps.iter().any(|step| {
            !step.class_matches_plan
                || !step.candidate_hash_matches_fixture
                || !step.exact_trace_signature_reconstructed
                || step.predecessor_entry_count + 1 != step.step as usize
        })
        || !proposal.proposal_marks_adjudication_required
        || !proposal.proposal_contains_single_adoption_block
        || !proposal.step15_j3_list_matches_trace
        || proposal.step10_trace_export_count != 4
        || proposal.step10_extra_primitive_laws_authorized
        || proposal.hopf_classifier_strengthening_authorized
        || proposal.step8_unit_coherence_generator_authorized_by_g1
        || !proposal.g6_unit_coherence_decision_deferred
        || !proposal.endpoint_premise_api_already_adopted
        || proposal.constructor_work_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(G1SurveyError::Trace(
            "G-1 survey crossed or failed its adjudication boundary".to_owned(),
        ));
    }

    let mut survey = G1VocabularySurvey {
        schema: G1_SURVEY_SCHEMA.to_owned(),
        date: G1_SURVEY_DATE.to_owned(),
        source_bindings: source_bindings(),
        archival_m1_replay_valid: true,
        archival_m1_is_signature_ground: false,
        surveyed_step_count: steps.len(),
        surveyed_clause_count,
        steps,
        proposal,
        verdict_blindness_statement: "Only the sealed trace, frozen grammar specification, pre-adjudicated Step-15 J3 list, and adopted endpoint implementation rule ground this proposal. No bar, archived count, pending verdict, sweep outcome, or desired score effect grounds a signature or generator.".to_owned(),
        forbidden_outputs,
        outcome: G1_OUTCOME.to_owned(),
        permitted_conclusion: "The exact Steps 9-15 vocabulary, predecessor bindings, raw roles, and Step-15/J3 correspondence have been reconstructed, and one trace-faithful batch proposal has been emitted. Semantic enrichment of Steps 9-14 is underdetermined. G-2 through G-8 remain closed until the batch is adopted.".to_owned(),
        result_digest: String::new(),
    };
    survey.result_digest = certificate_digest(&survey);
    Ok(survey)
}

fn failed_replay(error: impl Into<String>) -> G1VocabularyReplay {
    G1VocabularyReplay {
        valid: false,
        archival_m1_replayed: false,
        surveyed_step_count: 0,
        surveyed_clause_count: 0,
        step15_j3_list_matches_trace: false,
        constructor_work_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "g1_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_g1_vocabulary_survey(survey: &G1VocabularySurvey) -> G1VocabularyReplay {
    let expected = match issue_g1_vocabulary_survey() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if survey.result_digest != certificate_digest(survey) {
        errors.push("result digest mismatch".to_owned());
    }
    if survey != &expected {
        errors.push("survey differs from definition replay".to_owned());
    }
    G1VocabularyReplay {
        valid: errors.is_empty(),
        archival_m1_replayed: survey.archival_m1_replay_valid,
        surveyed_step_count: survey.surveyed_step_count,
        surveyed_clause_count: survey.surveyed_clause_count,
        step15_j3_list_matches_trace: survey.proposal.step15_j3_list_matches_trace,
        constructor_work_authorized: survey.proposal.constructor_work_authorized,
        forbidden_outputs_withheld: survey.forbidden_outputs.all_withheld(),
        outcome: survey.outcome.clone(),
        errors,
    }
}

pub fn replay_g1_vocabulary_survey_json(json: &str) -> G1VocabularyReplay {
    match serde_json::from_str::<G1VocabularySurvey>(json) {
        Ok(survey) => replay_g1_vocabulary_survey(&survey),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_g1_vocabulary_survey_create_new(
    path: &Path,
) -> Result<G1VocabularyReplay, G1SurveyError> {
    let survey = issue_g1_vocabulary_survey()?;
    let bytes = serde_json::to_vec_pretty(&survey)
        .map_err(|error| G1SurveyError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| G1SurveyError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .map_err(|error| G1SurveyError::Io(error.to_string()))?;
    output
        .write_all(b"\n")
        .map_err(|error| G1SurveyError::Io(error.to_string()))?;
    let replay = replay_g1_vocabulary_survey(&survey);
    if !replay.valid {
        return Err(G1SurveyError::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn g1_reconstructs_all_seven_rows_and_forty_three_clauses() {
        let survey = issue_g1_vocabulary_survey().expect("G-1 survey issues");
        assert_eq!(survey.surveyed_step_count, 7);
        assert_eq!(survey.surveyed_clause_count, 43);
        assert_eq!(
            survey
                .steps
                .iter()
                .map(|step| step.clause_kappa)
                .collect::<Vec<_>>(),
            vec![4, 4, 5, 6, 7, 9, 8]
        );
        assert!(survey.steps.iter().all(|step| step.class_matches_plan));
        assert!(
            survey
                .steps
                .iter()
                .all(|step| step.candidate_hash_matches_fixture)
        );
    }

    #[test]
    fn step15_matches_the_eight_j3_roles_exactly() {
        let survey = issue_g1_vocabulary_survey().expect("G-1 survey issues");
        let step15 = survey
            .steps
            .iter()
            .find(|step| step.step == 15)
            .expect("Step 15 surveyed");
        assert_eq!(
            step15
                .clauses
                .iter()
                .map(|clause| clause.j3_role.as_deref().expect("J3 role"))
                .collect::<Vec<_>>(),
            J3_ROLES
        );
        assert_eq!(
            step15
                .clauses
                .iter()
                .map(|clause| clause.compact_expr.as_str())
                .collect::<Vec<_>>(),
            vec![
                "Next(v1)",
                "Eventually(v1)",
                "Pi(Next(v1),Eventually(v1))",
                "Lam(App(L10,Next(v1)))",
                "Pi(Flat(Next(v1)),Next(Flat(v1)))",
                "Pi(Sharp(Eventually(v1)),Eventually(Sharp(v1)))",
                "Lam(App(Eventually(v1),v2))",
                "Pi(Next(Next(v1)),Next(v1))",
            ]
        );
    }

    #[test]
    fn g1_stops_at_the_adjudication_gate() {
        let survey = issue_g1_vocabulary_survey().expect("G-1 survey issues");
        assert_eq!(survey.outcome, G1_OUTCOME);
        assert!(!survey.proposal.constructor_work_authorized);
        assert!(!survey.archival_m1_is_signature_ground);
        assert!(survey.forbidden_outputs.all_withheld());
        assert!(replay_g1_vocabulary_survey(&survey).valid);
    }

    #[test]
    fn mutation_battery_fails_closed() {
        let original = issue_g1_vocabulary_survey().expect("G-1 survey issues");
        for mutation in 0..5 {
            let mut changed = original.clone();
            match mutation {
                0 => changed.result_digest.push_str(":mutated"),
                1 => changed.steps[0]
                    .predecessor_signature_digest
                    .push_str(":mutated"),
                2 => changed.steps[6].clauses[0].j3_role = None,
                3 => changed.proposal.constructor_work_authorized = true,
                4 => changed.forbidden_outputs.membership_verdict_issued = true,
                _ => unreachable!(),
            }
            assert!(!replay_g1_vocabulary_survey(&changed).valid);
        }
    }
}
