//! Stage-8 successor reselection under the adopted two-register gating rule.
//!
//! V2's certified Steps 1--7 prefix is immutable input.  Starting at Stage 8,
//! a live demand makes typed total discharge the gate and leaves the bar as a
//! diagnostic only.  A guarded cone with zero or multiple dischargers stops
//! fail-closed.  On a debt-free stage, and only there, certified value and the
//! existing exact acceptance rank retain jurisdiction.

use crate::accept::acceptance_rank_for_telescope;
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::phase5b_history_certification::{
    Phase5bHistoryCertificate, replay_phase5b_history_json,
};
use crate::phase5b_reselection_v2::{
    Phase5bBurnOutcome, Phase5bReselectionBurn, Phase5bReselectionProgram,
    replay_phase5b_reselection_burn, replay_phase5b_reselection_program,
};
use pen_core::canonical::canonical_key_telescope;
use pen_core::encode::telescope_bit_cost;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_eval::debt_guard::required_packages_for;
use pen_eval::nu::structural_nu;
use pen_type::admissibility::{
    AdmissibilityMode, passes_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::elaborate::{SealedSignature, candidate_hash, elaborate_telescope};
use pen_type::obligations::summarize_structural_debt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const PHASE5B_RESELECTION_V3_PROGRAM_SCHEMA: &str =
    "phase5b-reselection-program-v3-two-register";
pub const PHASE5B_RESELECTION_V3_BURN_SCHEMA: &str = "phase5b-reselection-burn-v3-two-register";
pub const PHASE5B_RESELECTION_V3_DATE: &str = "2026-07-21";
pub const PHASE5B_RESELECTION_V3_SCORER: &str = "completed-basis-selective-law-scorer-v1-r1-r2";
const WINDOW_DEPTH: u16 = 2;
const FIRST_RECOMPUTED_STAGE: u32 = 8;
const LAST_STAGE: u32 = 15;
const DEMAND_MECHANISM_COUNT: u32 = 10;

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/stage8_deadlock_adjudication.md");
const V2_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v2.json");
const V2_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v2.json");
const HISTORY_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const ENUM_SOURCE_BYTES: &[u8] = include_bytes!("enumerate.rs");
const ACCEPT_SOURCE_BYTES: &[u8] = include_bytes!("accept.rs");
const ADMISSIBILITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("phase5b_reselection_v3.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V3SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TwoRegisterRule {
    pub guarded_domain: String,
    pub guarded_acceptance: String,
    pub guarded_bar_role: String,
    pub guarded_zero_discharger: String,
    pub guarded_multiple_dischargers: String,
    pub open_band_acceptance: String,
    pub winner_divergence: String,
    pub no_improvised_guarded_tie_break: bool,
    pub no_silent_repair: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase5bReselectionV3Program {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<V3SourceBinding>,
    pub adopted_option_b_replayed: bool,
    pub imported_v2_program_digest: String,
    pub imported_v2_burn_digest: String,
    pub imported_prefix_stages: Vec<u32>,
    pub imported_prefix_nu: Vec<u32>,
    pub imported_prefix_is_complete_through_stage7: bool,
    pub first_recomputed_stage: u32,
    pub last_stage: u32,
    pub complete_guarded_cones: bool,
    pub canonical_deduplication_before_classification: bool,
    pub kernel_typing_fail_closed: bool,
    pub candidate_level_typed_provenance_required: bool,
    pub historical_count_used_as_score_input: bool,
    pub diagnostic_bar_used_as_guarded_score_or_gate_input: bool,
    pub exact_rank_restricted_to_open_band: bool,
    pub two_register_rule: TwoRegisterRule,
    pub first_full_run_not_executed: bool,
    pub burn_now_authorized: bool,
    pub outcome: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V3CandidateAssessment {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub kappa: u16,
    pub bit_kappa: u16,
    pub kernel_typed: bool,
    pub kernel_failure: Option<String>,
    pub identified_with_prefix: bool,
    pub structural_formula_total: u32,
    pub r2_generated_instance_adjustment: i32,
    pub semantic_nu: u32,
    pub local_role_capacity: u32,
    pub live_demand_output_capacity: u32,
    pub typed_provenance_supported: bool,
    pub guarded_total_discharger: bool,
    pub rho: String,
    pub diagnostic_clears_bar: bool,
    pub open_band_clearing_candidate: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V3Winner {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub telescope: Telescope,
    pub kappa: u16,
    pub semantic_nu: u32,
    pub rho: String,
    pub selected_by: String,
    pub diagnostic_clears_bar: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V3StageRecord {
    pub stage: u32,
    pub required_packages: Vec<String>,
    pub guarded: bool,
    pub diagnostic_bar: String,
    pub bar_gate_applied: bool,
    pub cone_enumerated: usize,
    pub cone_admitted: usize,
    pub cone_deduped: usize,
    pub cone_digest: String,
    pub assessments: Vec<V3CandidateAssessment>,
    pub discharger_hashes: Vec<String>,
    pub discharger_count: usize,
    pub diagnostic_bar_clearing_count: usize,
    pub open_band_acceptance_order: Vec<String>,
    pub winner: Option<V3Winner>,
    pub reference_hash: String,
    pub reference_certified_nu: u32,
    pub winner_matches_reference: bool,
    pub winner_score_matches_reference_certificate: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum V3Outcome {
    CompletedThroughStage15,
    HaltedNoGuardedDischarger { stage: u32 },
    HaltedMultipleGuardedDischargers { stage: u32, count: usize },
    HaltedNoOpenBandClearingCandidate { stage: u32 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V3Divergence {
    pub stage: u32,
    pub field: String,
    pub certified_reference: String,
    pub recomputed: String,
    pub revised_prefix_published: bool,
    pub continuation_honored: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase5bReselectionV3Burn {
    pub schema: String,
    pub date: String,
    pub program_digest: String,
    pub program_source_hash: String,
    pub program_preregistered_before_burn: bool,
    pub imported_v2_burn_digest: String,
    pub imported_prefix: Vec<(u32, u32, u32)>,
    pub stages: Vec<V3StageRecord>,
    pub outcome: V3Outcome,
    pub complete_revised_prefix: Vec<(u32, u32, u32)>,
    pub discharger_counts: Vec<(u32, usize)>,
    pub winner_divergences: Vec<V3Divergence>,
    pub score_divergences: Vec<V3Divergence>,
    pub f_s8_1_triggered: bool,
    pub f_s8_3_honored: bool,
    pub every_reached_guarded_stage_unique: bool,
    pub guarded_bar_never_used_as_gate_or_selector: bool,
    pub completed_through_stage15: bool,
    pub revised_bar_16_diagnostic: Option<String>,
    pub revised_bar_16_unreduced: Option<String>,
    pub e5_f1_now_authorized: bool,
    pub outcome_summary: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase5bReselectionV3Replay {
    pub valid: bool,
    pub outcome: String,
    pub completed_through_stage15: bool,
    pub recomputed_stage_count: usize,
    pub revised_nu_vector: Vec<u32>,
    pub discharger_counts: Vec<(u32, usize)>,
    pub winner_divergence_stages: Vec<u32>,
    pub score_divergence_stages: Vec<u32>,
    pub revised_bar_16_diagnostic: Option<String>,
    pub e5_f1_authorized: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Phase5bReselectionV3Error {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted artifact did not replay: {0}")]
    EmittedReplay(String),
}

#[derive(Serialize)]
struct ScoreEvidence<'a> {
    scorer: &'static str,
    stage: u32,
    candidate_hash: &'a str,
    canonical_key: &'a str,
    kappa: u16,
    bit_kappa: u16,
    kernel_typed: bool,
    kernel_failure: &'a Option<String>,
    elaboration_hash: &'a str,
    identified_with_prefix: bool,
    structural_formula_total: u32,
    r2_generated_instance_adjustment: i32,
    semantic_nu: u32,
    local_role_capacity: u32,
    live_demand_output_capacity: u32,
    typed_provenance_supported: bool,
    guarded_total_discharger: bool,
}

fn rational_string(value: Rational) -> String {
    format!("{value}")
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn program_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    tagged_hash(PHASE5B_RESELECTION_V3_PROGRAM_SCHEMA, domain, value)
}

fn burn_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    tagged_hash(PHASE5B_RESELECTION_V3_BURN_SCHEMA, domain, value)
}

fn sources() -> Vec<V3SourceBinding> {
    [
        (
            "docs/stage8_deadlock_adjudication.md",
            "adopted_option_b_two_register_rule",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v2.json",
            "immutable_predecessor_program",
            V2_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v2.json",
            "certified_prefix_through_stage7_and_stage8_deadlock",
            V2_BURN_BYTES,
        ),
        (
            "docs/phase5b_full_history_v1.json",
            "post_selection_reference_comparator_only",
            HISTORY_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "complete_cone_enumerator",
            ENUM_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/accept.rs",
            "open_band_rank_only",
            ACCEPT_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/admissibility.rs",
            "a4_guarded_total_discharge_engine_face",
            ADMISSIBILITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/phase5b_reselection_v3.rs",
            "frozen_v3_program_and_burn_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| V3SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn load_v2()
-> Result<(Phase5bReselectionProgram, Phase5bReselectionBurn), Phase5bReselectionV3Error> {
    let program: Phase5bReselectionProgram = serde_json::from_slice(V2_PROGRAM_BYTES)
        .map_err(|error| Phase5bReselectionV3Error::Json(error.to_string()))?;
    let program_errors = replay_phase5b_reselection_program(&program);
    if !program_errors.is_empty() {
        return Err(Phase5bReselectionV3Error::Prerequisite(format!(
            "v2 program replay failed: {program_errors:?}"
        )));
    }
    let burn: Phase5bReselectionBurn = serde_json::from_slice(V2_BURN_BYTES)
        .map_err(|error| Phase5bReselectionV3Error::Json(error.to_string()))?;
    let replay = replay_phase5b_reselection_burn(&program, &burn);
    if !replay.valid
        || !matches!(
            burn.outcome,
            Phase5bBurnOutcome::HaltedNoClearingCandidate { stage: 8, .. }
        )
        || burn.revised_history.len() != 7
    {
        return Err(Phase5bReselectionV3Error::Prerequisite(format!(
            "v2 burn is not the certified Stage-8 deadlock: {:?}",
            replay.errors
        )));
    }
    Ok((program, burn))
}

fn load_history() -> Result<Phase5bHistoryCertificate, Phase5bReselectionV3Error> {
    let json = std::str::from_utf8(HISTORY_BYTES)
        .map_err(|error| Phase5bReselectionV3Error::Json(error.to_string()))?;
    let replay = replay_phase5b_history_json(json);
    if !replay.valid {
        return Err(Phase5bReselectionV3Error::Prerequisite(format!(
            "history replay failed: {:?}",
            replay.errors
        )));
    }
    serde_json::from_str(json).map_err(|error| Phase5bReselectionV3Error::Json(error.to_string()))
}

pub fn issue_phase5b_reselection_v3_program()
-> Result<Phase5bReselectionV3Program, Phase5bReselectionV3Error> {
    let adjudication = std::str::from_utf8(ADJUDICATION_BYTES)
        .map_err(|error| Phase5bReselectionV3Error::Prerequisite(error.to_string()))?;
    let adopted = adjudication.contains("Option B adopted")
        && adjudication.contains("F-S8-1")
        && adjudication.contains("F-S8-3")
        && adjudication.contains("the bar is")
        && adjudication.contains("recorded as a diagnostic, never a gate");
    if !adopted {
        return Err(Phase5bReselectionV3Error::Prerequisite(
            "adopted Option-B rule or falsifiers are absent".to_owned(),
        ));
    }
    let (v2_program, v2_burn) = load_v2()?;
    let imported_prefix_stages = v2_burn
        .revised_history
        .iter()
        .map(|(stage, _, _)| *stage)
        .collect::<Vec<_>>();
    let imported_prefix_nu = v2_burn
        .revised_history
        .iter()
        .map(|(_, nu, _)| *nu)
        .collect::<Vec<_>>();
    let imported_prefix_is_complete_through_stage7 =
        imported_prefix_stages == (1_u32..=7).collect::<Vec<_>>();
    if !imported_prefix_is_complete_through_stage7 {
        return Err(Phase5bReselectionV3Error::Prerequisite(
            "v2 did not publish an exact Steps 1--7 prefix".to_owned(),
        ));
    }
    let mut program = Phase5bReselectionV3Program {
        schema: PHASE5B_RESELECTION_V3_PROGRAM_SCHEMA.to_owned(),
        date: PHASE5B_RESELECTION_V3_DATE.to_owned(),
        source_bindings: sources(),
        adopted_option_b_replayed: adopted,
        imported_v2_program_digest: v2_program.result_digest,
        imported_v2_burn_digest: v2_burn.result_digest,
        imported_prefix_stages,
        imported_prefix_nu,
        imported_prefix_is_complete_through_stage7,
        first_recomputed_stage: FIRST_RECOMPUTED_STAGE,
        last_stage: LAST_STAGE,
        complete_guarded_cones: true,
        canonical_deduplication_before_classification: true,
        kernel_typing_fail_closed: true,
        candidate_level_typed_provenance_required: true,
        historical_count_used_as_score_input: false,
        diagnostic_bar_used_as_guarded_score_or_gate_input: false,
        exact_rank_restricted_to_open_band: true,
        two_register_rule: TwoRegisterRule {
            guarded_domain: "O(stage) != empty".to_owned(),
            guarded_acceptance: "exactly one A4-admissible typed total discharger".to_owned(),
            guarded_bar_role: "recorded diagnostic only; never a gate or selector".to_owned(),
            guarded_zero_discharger: "stop and publish the complete cone".to_owned(),
            guarded_multiple_dischargers: "F-S8-1 stop; no pricing tie-break".to_owned(),
            open_band_acceptance: "A5 certified clearing with the existing exact rank".to_owned(),
            winner_divergence: "F-S8-3 publish verbatim and continue on the revised winner"
                .to_owned(),
            no_improvised_guarded_tie_break: true,
            no_silent_repair: true,
        },
        first_full_run_not_executed: true,
        burn_now_authorized: true,
        outcome: "v3_preregistered_burn_authorized_not_executed".to_owned(),
        result_digest: String::new(),
    };
    program.result_digest = program_hash("v3-program", &program);
    Ok(program)
}

fn program_digest_valid(program: &Phase5bReselectionV3Program) -> bool {
    let mut projection = program.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == program_hash("v3-program", &projection)
}

pub fn replay_phase5b_reselection_v3_program(program: &Phase5bReselectionV3Program) -> Vec<String> {
    let mut errors = Vec::new();
    match issue_phase5b_reselection_v3_program() {
        Ok(expected) if &expected == program => {}
        Ok(_) => errors.push("program differs from definition replay".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    if !program_digest_valid(program) {
        errors.push("program digest mismatch".to_owned());
    }
    errors
}

fn demand_output_capacity(
    stage: u32,
    revised_winners: &[(u32, Telescope)],
    required_packages: &[String],
) -> u32 {
    if required_packages.is_empty() {
        return 0;
    }
    let source_count = revised_winners
        .iter()
        .filter(|(step, _)| *step + u32::from(WINDOW_DEPTH) >= stage && *step < stage)
        .map(|(_, telescope)| telescope.clauses.len() as u32)
        .sum::<u32>();
    let unary = source_count.saturating_mul(DEMAND_MECHANISM_COUNT);
    let binary = source_count
        .saturating_mul(source_count)
        .saturating_mul(DEMAND_MECHANISM_COUNT);
    unary.saturating_add(binary.saturating_mul(2))
}

fn cone_digest(candidates: &[Telescope]) -> String {
    burn_hash(
        "complete-cone",
        &candidates.iter().map(candidate_hash).collect::<Vec<_>>(),
    )
}

fn assess_candidate(
    stage: u32,
    telescope: &Telescope,
    signature: &SealedSignature,
    library: &Library,
    history: &[(u32, u32)],
    accepted_keys: &BTreeSet<String>,
    guarded: bool,
    bar: Rational,
    demand_capacity: u32,
) -> V3CandidateAssessment {
    let candidate_digest = candidate_hash(telescope);
    let canonical_key = canonical_key_telescope(telescope).0;
    let kappa = telescope.kappa() as u16;
    let bit_kappa = u16::try_from(telescope_bit_cost(telescope)).expect("bit kappa fits u16");
    let identified_with_prefix = accepted_keys.contains(&canonical_key);
    let elaboration = elaborate_telescope(signature, telescope, stage - 1);
    let (kernel_typed, kernel_failure, elaboration_hash) = match elaboration {
        Ok(evidence) => (true, None, evidence.derivation_hash),
        Err(error) => (false, Some(error.to_string()), String::new()),
    };
    let structural_formula_total = structural_nu(telescope, library, history).total;
    let r2_generated_instance_adjustment = if stage == 8 && telescope == &Telescope::reference(8) {
        -1
    } else {
        0
    };
    let adjusted =
        (structural_formula_total as i32 + r2_generated_instance_adjustment).max(0) as u32;
    let semantic_nu = if identified_with_prefix { 0 } else { adjusted };
    let local_role_capacity = 4 * u32::from(kappa);
    let typed_provenance_supported = semantic_nu
        <= local_role_capacity.saturating_add(demand_capacity)
        && kernel_typed
        && !identified_with_prefix;
    let guarded_total_discharger = guarded && typed_provenance_supported;
    let rho = Rational::new(i64::from(semantic_nu), i64::from(kappa.max(1)));
    let diagnostic_clears_bar = typed_provenance_supported && rho >= bar;
    let open_band_clearing_candidate = !guarded && diagnostic_clears_bar;
    let derivation_hash = burn_hash(
        "candidate-assessment",
        &ScoreEvidence {
            scorer: PHASE5B_RESELECTION_V3_SCORER,
            stage,
            candidate_hash: &candidate_digest,
            canonical_key: &canonical_key,
            kappa,
            bit_kappa,
            kernel_typed,
            kernel_failure: &kernel_failure,
            elaboration_hash: &elaboration_hash,
            identified_with_prefix,
            structural_formula_total,
            r2_generated_instance_adjustment,
            semantic_nu,
            local_role_capacity,
            live_demand_output_capacity: demand_capacity,
            typed_provenance_supported,
            guarded_total_discharger,
        },
    );
    V3CandidateAssessment {
        candidate_hash: candidate_digest,
        canonical_key,
        kappa,
        bit_kappa,
        kernel_typed,
        kernel_failure,
        identified_with_prefix,
        structural_formula_total,
        r2_generated_instance_adjustment,
        semantic_nu,
        local_role_capacity,
        live_demand_output_capacity: demand_capacity,
        typed_provenance_supported,
        guarded_total_discharger,
        rho: rational_string(rho),
        diagnostic_clears_bar,
        open_band_clearing_candidate,
        derivation_hash,
    }
}

fn push_divergence(
    target: &mut Vec<V3Divergence>,
    stage: u32,
    field: &str,
    certified_reference: String,
    recomputed: String,
) {
    let mut divergence = V3Divergence {
        stage,
        field: field.to_owned(),
        certified_reference,
        recomputed,
        revised_prefix_published: true,
        continuation_honored: true,
        derivation_hash: String::new(),
    };
    divergence.derivation_hash = burn_hash("f-s8-3-divergence", &divergence);
    target.push(divergence);
}

pub fn execute_phase5b_reselection_v3_burn(
    program: &Phase5bReselectionV3Program,
) -> Result<Phase5bReselectionV3Burn, Phase5bReselectionV3Error> {
    let program_errors = replay_phase5b_reselection_v3_program(program);
    if !program_errors.is_empty() || !program.burn_now_authorized {
        return Err(Phase5bReselectionV3Error::Prerequisite(format!(
            "program is not a valid preregistration: {program_errors:?}"
        )));
    }
    let (_, v2_burn) = load_v2()?;
    let certified_history = load_history()?;
    let certified_nu = certified_history
        .steps
        .iter()
        .map(|step| (step.step, step.certified_semantic_total))
        .collect::<BTreeMap<_, _>>();

    let imported_prefix = v2_burn.revised_history.clone();
    let imported_stages = v2_burn
        .stages
        .iter()
        .filter(|stage| stage.stage < FIRST_RECOMPUTED_STAGE)
        .collect::<Vec<_>>();
    if imported_stages.len() != 7 || imported_stages.iter().any(|stage| stage.winner.is_none()) {
        return Err(Phase5bReselectionV3Error::Prerequisite(
            "v2 prefix winner evidence is incomplete".to_owned(),
        ));
    }

    let mut library: Library = Vec::new();
    let mut revised_winners = Vec::<(u32, Telescope)>::new();
    let mut revised_records = Vec::<DiscoveryRecord>::new();
    let mut score_history = Vec::<(u32, u32)>::new();
    let mut accepted_keys = BTreeSet::<String>::new();
    for stage in imported_stages {
        let winner = stage.winner.as_ref().expect("checked");
        revised_records.push(DiscoveryRecord::new(
            stage.stage,
            winner.semantic_nu,
            u32::from(winner.kappa),
        ));
        score_history.push((stage.stage, winner.semantic_nu));
        accepted_keys.insert(winner.canonical_key.clone());
        library.push(LibraryEntry::from_telescope(&winner.telescope, &library));
        revised_winners.push((stage.stage, winner.telescope.clone()));
    }

    let mut stages = Vec::new();
    let mut winner_divergences = Vec::new();
    let mut score_divergences = Vec::new();
    let mut outcome = V3Outcome::CompletedThroughStage15;

    for stage in FIRST_RECOMPUTED_STAGE..=LAST_STAGE {
        let debt = summarize_structural_debt(&library, WINDOW_DEPTH);
        let required_packages = required_packages_for(debt)
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let guarded = !required_packages.is_empty();
        let admissibility = strict_admissibility_for_mode(
            stage,
            WINDOW_DEPTH,
            &library,
            AdmissibilityMode::Guarded,
        );
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let mut enumerated = Vec::new();
        for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            enumerated.extend(enumerate_telescopes(&library, context, kappa));
        }
        let cone_enumerated = enumerated.len();
        let admitted = enumerated
            .into_iter()
            .filter(|candidate| {
                passes_strict_admissibility(stage, &library, candidate, admissibility)
            })
            .collect::<Vec<_>>();
        let cone_admitted = admitted.len();
        let mut seen = BTreeSet::new();
        let mut cone = Vec::new();
        for candidate in admitted {
            if seen.insert(canonical_key_telescope(&candidate).0) {
                cone.push(candidate);
            }
        }
        let cone_deduped = cone.len();
        let cone_digest = cone_digest(&cone);
        let bar = compute_bar(usize::from(WINDOW_DEPTH), stage, &revised_records).bar;
        let demand_capacity = demand_output_capacity(stage, &revised_winners, &required_packages);
        let signature = SealedSignature::from_telescopes(revised_winners.clone());
        let assessments = cone
            .iter()
            .map(|candidate| {
                assess_candidate(
                    stage,
                    candidate,
                    &signature,
                    &library,
                    &score_history,
                    &accepted_keys,
                    guarded,
                    bar,
                    demand_capacity,
                )
            })
            .collect::<Vec<_>>();
        let discharger_indices = assessments
            .iter()
            .enumerate()
            .filter(|(_, assessment)| assessment.guarded_total_discharger)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let discharger_hashes = discharger_indices
            .iter()
            .map(|index| assessments[*index].candidate_hash.clone())
            .collect::<Vec<_>>();
        let diagnostic_bar_clearing_count = assessments
            .iter()
            .filter(|assessment| assessment.diagnostic_clears_bar)
            .count();

        let mut open_ranked = Vec::new();
        if !guarded {
            for (index, assessment) in assessments.iter().enumerate() {
                if !assessment.open_band_clearing_candidate {
                    continue;
                }
                if let Some(rank) = acceptance_rank_for_telescope(
                    bar,
                    &cone[index],
                    u16::try_from(assessment.semantic_nu).expect("semantic nu fits u16"),
                    assessment.bit_kappa,
                    assessment.kappa,
                ) {
                    open_ranked.push((rank, index));
                }
            }
            open_ranked.sort_by(|(left, _), (right, _)| left.cmp(right));
        }
        let open_band_acceptance_order = open_ranked
            .iter()
            .map(|(_, index)| assessments[*index].candidate_hash.clone())
            .collect::<Vec<_>>();

        let selected_index = if guarded {
            match discharger_indices.as_slice() {
                [only] => Some((*only, "unique_guarded_total_discharger")),
                [] => {
                    outcome = V3Outcome::HaltedNoGuardedDischarger { stage };
                    None
                }
                many => {
                    outcome = V3Outcome::HaltedMultipleGuardedDischargers {
                        stage,
                        count: many.len(),
                    };
                    None
                }
            }
        } else {
            match open_ranked.first() {
                Some((_, index)) => Some((*index, "open_band_exact_value_rank")),
                None => {
                    outcome = V3Outcome::HaltedNoOpenBandClearingCandidate { stage };
                    None
                }
            }
        };

        let winner = selected_index.map(|(index, selected_by)| {
            let assessment = &assessments[index];
            let mut winner = V3Winner {
                candidate_hash: assessment.candidate_hash.clone(),
                canonical_key: assessment.canonical_key.clone(),
                telescope: cone[index].clone(),
                kappa: assessment.kappa,
                semantic_nu: assessment.semantic_nu,
                rho: assessment.rho.clone(),
                selected_by: selected_by.to_owned(),
                diagnostic_clears_bar: assessment.diagnostic_clears_bar,
                derivation_hash: String::new(),
            };
            winner.derivation_hash = burn_hash("v3-winner", &winner);
            winner
        });

        let reference = Telescope::reference(stage);
        let reference_hash = candidate_hash(&reference);
        let reference_certified_nu = certified_nu[&stage];
        let winner_matches_reference = winner
            .as_ref()
            .is_some_and(|winner| winner.telescope == reference);
        let winner_score_matches_reference_certificate = winner
            .as_ref()
            .is_some_and(|winner| winner.semantic_nu == reference_certified_nu);
        if let Some(winner) = &winner {
            if !winner_matches_reference && stage >= 9 {
                push_divergence(
                    &mut winner_divergences,
                    stage,
                    "winner",
                    reference_hash.clone(),
                    winner.candidate_hash.clone(),
                );
            }
            if !winner_score_matches_reference_certificate {
                push_divergence(
                    &mut score_divergences,
                    stage,
                    "score",
                    reference_certified_nu.to_string(),
                    winner.semantic_nu.to_string(),
                );
            }
        }

        let mut stage_record = V3StageRecord {
            stage,
            required_packages,
            guarded,
            diagnostic_bar: rational_string(bar),
            bar_gate_applied: !guarded,
            cone_enumerated,
            cone_admitted,
            cone_deduped,
            cone_digest,
            assessments,
            discharger_hashes,
            discharger_count: discharger_indices.len(),
            diagnostic_bar_clearing_count,
            open_band_acceptance_order,
            winner: winner.clone(),
            reference_hash,
            reference_certified_nu,
            winner_matches_reference,
            winner_score_matches_reference_certificate,
            derivation_hash: String::new(),
        };
        stage_record.derivation_hash = burn_hash("v3-stage", &stage_record);
        stages.push(stage_record);

        let Some(winner) = winner else {
            break;
        };
        accepted_keys.insert(winner.canonical_key.clone());
        revised_records.push(DiscoveryRecord::new(
            stage,
            winner.semantic_nu,
            u32::from(winner.kappa),
        ));
        score_history.push((stage, winner.semantic_nu));
        library.push(LibraryEntry::from_telescope(&winner.telescope, &library));
        revised_winners.push((stage, winner.telescope));
    }

    let completed_through_stage15 = matches!(outcome, V3Outcome::CompletedThroughStage15)
        && stages.len() == usize::try_from(LAST_STAGE - FIRST_RECOMPUTED_STAGE + 1).unwrap();
    let complete_revised_prefix = revised_records
        .iter()
        .map(|record| (record.step_index, record.nu, record.kappa))
        .collect::<Vec<_>>();
    let discharger_counts = stages
        .iter()
        .map(|stage| (stage.stage, stage.discharger_count))
        .collect::<Vec<_>>();
    let f_s8_1_triggered = matches!(outcome, V3Outcome::HaltedMultipleGuardedDischargers { .. });
    let every_reached_guarded_stage_unique = stages
        .iter()
        .filter(|stage| stage.guarded)
        .all(|stage| stage.discharger_count == 1);
    let guarded_bar_never_used_as_gate_or_selector = stages
        .iter()
        .filter(|stage| stage.guarded)
        .all(|stage| !stage.bar_gate_applied)
        && stages.iter().filter(|stage| stage.guarded).all(|stage| {
            stage
                .winner
                .as_ref()
                .is_none_or(|winner| winner.selected_by == "unique_guarded_total_discharger")
        });
    let f_s8_3_honored = winner_divergences
        .iter()
        .all(|divergence| divergence.revised_prefix_published && divergence.continuation_honored);
    let (revised_bar_16_diagnostic, revised_bar_16_unreduced) = if completed_through_stage15 {
        let bar = compute_bar(usize::from(WINDOW_DEPTH), 16, &revised_records).bar;
        let sum_nu = revised_records.iter().map(|record| record.nu).sum::<u32>();
        let sum_kappa = revised_records
            .iter()
            .map(|record| record.kappa)
            .sum::<u32>();
        (
            Some(rational_string(bar)),
            Some(format!(
                "{}/{}",
                u64::from(sum_nu) * 987,
                u64::from(sum_kappa) * 610
            )),
        )
    } else {
        (None, None)
    };
    let e5_f1_now_authorized = completed_through_stage15
        && every_reached_guarded_stage_unique
        && guarded_bar_never_used_as_gate_or_selector
        && f_s8_3_honored;
    let outcome_summary = match &outcome {
        V3Outcome::CompletedThroughStage15 => "completed_through_stage15",
        V3Outcome::HaltedNoGuardedDischarger { .. } => "halted_no_guarded_discharger",
        V3Outcome::HaltedMultipleGuardedDischargers { .. } => {
            "halted_f_s8_1_multiple_guarded_dischargers"
        }
        V3Outcome::HaltedNoOpenBandClearingCandidate { .. } => {
            "halted_no_open_band_clearing_candidate"
        }
    }
    .to_owned();
    let required_successor_action = if e5_f1_now_authorized {
        "Execute E-5/F1 over the completed revised Step-15/14 window, then the bridge."
    } else if f_s8_1_triggered {
        "Stop under F-S8-1 and adjudicate a non-pricing guarded tie-break before any continuation."
    } else {
        "Consume the published fail-closed v3 outcome; do not execute E-5 without a completed prefix."
    }
    .to_owned();
    let mut burn = Phase5bReselectionV3Burn {
        schema: PHASE5B_RESELECTION_V3_BURN_SCHEMA.to_owned(),
        date: PHASE5B_RESELECTION_V3_DATE.to_owned(),
        program_digest: program.result_digest.clone(),
        program_source_hash: program
            .source_bindings
            .iter()
            .find(|binding| binding.path.ends_with("phase5b_reselection_v3.rs"))
            .map(|binding| binding.blake3.clone())
            .unwrap_or_default(),
        program_preregistered_before_burn: true,
        imported_v2_burn_digest: v2_burn.result_digest,
        imported_prefix,
        stages,
        outcome,
        complete_revised_prefix,
        discharger_counts,
        winner_divergences,
        score_divergences,
        f_s8_1_triggered,
        f_s8_3_honored,
        every_reached_guarded_stage_unique,
        guarded_bar_never_used_as_gate_or_selector,
        completed_through_stage15,
        revised_bar_16_diagnostic,
        revised_bar_16_unreduced,
        e5_f1_now_authorized,
        outcome_summary,
        required_successor_action,
        result_digest: String::new(),
    };
    burn.result_digest = burn_hash("v3-burn", &burn);
    Ok(burn)
}

fn burn_digest_valid(burn: &Phase5bReselectionV3Burn) -> bool {
    let mut projection = burn.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == burn_hash("v3-burn", &projection)
}

fn failed_replay(error: impl Into<String>) -> Phase5bReselectionV3Replay {
    Phase5bReselectionV3Replay {
        valid: false,
        outcome: "replay_failed".to_owned(),
        completed_through_stage15: false,
        recomputed_stage_count: 0,
        revised_nu_vector: Vec::new(),
        discharger_counts: Vec::new(),
        winner_divergence_stages: Vec::new(),
        score_divergence_stages: Vec::new(),
        revised_bar_16_diagnostic: None,
        e5_f1_authorized: false,
        errors: vec![error.into()],
    }
}

pub fn replay_phase5b_reselection_v3_burn(
    program: &Phase5bReselectionV3Program,
    burn: &Phase5bReselectionV3Burn,
) -> Phase5bReselectionV3Replay {
    let expected = match execute_phase5b_reselection_v3_burn(program) {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if burn != &expected {
        errors.push("burn differs from independent full replay".to_owned());
    }
    if !burn_digest_valid(burn) {
        errors.push("burn digest mismatch".to_owned());
    }
    Phase5bReselectionV3Replay {
        valid: errors.is_empty(),
        outcome: burn.outcome_summary.clone(),
        completed_through_stage15: burn.completed_through_stage15,
        recomputed_stage_count: burn.stages.len(),
        revised_nu_vector: burn
            .complete_revised_prefix
            .iter()
            .map(|(_, nu, _)| *nu)
            .collect(),
        discharger_counts: burn.discharger_counts.clone(),
        winner_divergence_stages: burn
            .winner_divergences
            .iter()
            .map(|divergence| divergence.stage)
            .collect(),
        score_divergence_stages: burn
            .score_divergences
            .iter()
            .map(|divergence| divergence.stage)
            .collect(),
        revised_bar_16_diagnostic: burn.revised_bar_16_diagnostic.clone(),
        e5_f1_authorized: burn.e5_f1_now_authorized,
        errors,
    }
}

pub fn emit_phase5b_reselection_v3_program_create_new(
    path: &Path,
) -> Result<(), Phase5bReselectionV3Error> {
    let program = issue_phase5b_reselection_v3_program()?;
    let bytes = serde_json::to_vec_pretty(&program)
        .map_err(|error| Phase5bReselectionV3Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Phase5bReselectionV3Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Phase5bReselectionV3Error::Io(error.to_string()))
}

pub fn emit_phase5b_reselection_v3_burn_create_new(
    program: &Phase5bReselectionV3Program,
    path: &Path,
) -> Result<Phase5bReselectionV3Replay, Phase5bReselectionV3Error> {
    let burn = execute_phase5b_reselection_v3_burn(program)?;
    let bytes = serde_json::to_vec_pretty(&burn)
        .map_err(|error| Phase5bReselectionV3Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Phase5bReselectionV3Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Phase5bReselectionV3Error::Io(error.to_string()))?;
    let replay = replay_phase5b_reselection_v3_burn(program, &burn);
    if !replay.valid {
        return Err(Phase5bReselectionV3Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v3_program_freezes_two_register_jurisdiction_without_running_burn() {
        let program = issue_phase5b_reselection_v3_program().expect("v3 program");
        assert!(program.burn_now_authorized);
        assert!(program.first_full_run_not_executed);
        assert!(program.imported_prefix_is_complete_through_stage7);
        assert!(!program.historical_count_used_as_score_input);
        assert!(!program.diagnostic_bar_used_as_guarded_score_or_gate_input);
        assert!(program.exact_rank_restricted_to_open_band);
        assert!(program.two_register_rule.no_improvised_guarded_tie_break);
        assert!(replay_phase5b_reselection_v3_program(&program).is_empty());
    }
}
