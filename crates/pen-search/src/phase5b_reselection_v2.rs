//! Preregistered Branch-(ii) full sequential semantic reselection.
//!
//! The program certificate is emitted before the first full run.  The burn
//! starts from the empty prefix, enumerates and publishes every complete
//! guarded cone, types every candidate, admits score units only within the
//! injective local-role plus independently generated live-demand capacity,
//! applies the completed R1/R2 quotient, and uses the engine's exact rank.

use crate::accept::acceptance_rank_for_telescope;
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::phase5b_history_certification::{
    DEMAND_GRAMMAR_VERSION, PHASE5B_HISTORY_CERT_SCHEMA, Phase5bHistoryCertificate,
    replay_phase5b_history_json,
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

pub const PHASE5B_RESELECTION_PROGRAM_SCHEMA: &str = "phase5b-reselection-program-v2";
pub const PHASE5B_RESELECTION_BURN_SCHEMA: &str = "phase5b-reselection-burn-v2";
pub const PHASE5B_RESELECTION_DATE: &str = "2026-07-21";
pub const SEMANTIC_SCORER_VERSION: &str = "completed-basis-selective-law-scorer-v1";
const WINDOW_DEPTH: u16 = 2;

const HISTORY_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const FORK_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_fork_adjudication.md");
const E2B_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e2b_quotient_closure_v1.json");
const E4_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v10.json");
const ENUM_SOURCE_BYTES: &[u8] = include_bytes!("enumerate.rs");
const ACCEPT_SOURCE_BYTES: &[u8] = include_bytes!("accept.rs");
const ADMISSIBILITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("phase5b_reselection_v2.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReselectionSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenOutcomeSemantics {
    pub completed_through_stage15: String,
    pub halted_no_clearing_candidate: String,
    pub winner_divergence: String,
    pub score_or_bar_divergence: String,
    pub no_silent_repair: bool,
    pub continue_on_winner_divergence_when_a_winner_exists: bool,
    pub stop_only_when_no_candidate_clears: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase5bReselectionProgram {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<ReselectionSourceBinding>,
    pub history_schema: String,
    pub history_digest: String,
    pub history_replayed: bool,
    pub branch_ii_replayed: bool,
    pub scorer_version: String,
    pub demand_grammar_version: String,
    pub window_depth: u16,
    pub starts_from_empty_prefix: bool,
    pub complete_guarded_cone_enumeration: bool,
    pub canonical_deduplication_before_scoring: bool,
    pub kernel_typing_fail_closed: bool,
    pub exact_rational_rank_reused: bool,
    pub local_role_bound_per_candidate: String,
    pub excess_credit_requires_live_exported_demand_outputs: bool,
    pub uniform_instances_not_multiplied: bool,
    pub r1_package_rule_applied: bool,
    pub r2_generated_instance_rule_applied: bool,
    pub historical_count_used_as_score_input: bool,
    pub acceptance_bar_used_as_score_input: bool,
    pub frozen_outcome_semantics: FrozenOutcomeSemantics,
    pub first_full_run_not_executed: bool,
    pub burn_now_authorized: bool,
    pub outcome: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateSemanticScore {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub kappa: u16,
    pub bit_kappa: u16,
    pub identified_with_revised_history: bool,
    pub kernel_typed: bool,
    pub kernel_failure: Option<String>,
    pub structural_formula_total: u32,
    pub r1_package_adjustment: i32,
    pub r2_generated_instance_adjustment: i32,
    pub local_role_capacity: u32,
    pub live_demand_output_capacity: u32,
    pub provenance_capacity: u32,
    pub every_score_unit_anchor_supported: bool,
    pub invalid_or_unclassified: bool,
    pub semantic_nu: u32,
    pub rho: String,
    pub clears_bar: bool,
    pub overshoot: Option<String>,
    pub score_derivation_hash: String,
}

#[derive(Serialize)]
struct CandidateScoreEvidence<'a> {
    scorer_version: &'static str,
    stage: u32,
    candidate_hash: &'a str,
    canonical_key: &'a str,
    kappa: u16,
    bit_kappa: u16,
    identified_with_revised_history: bool,
    kernel_typed: bool,
    kernel_failure: &'a Option<String>,
    elaboration_hash: &'a str,
    structural_formula_total: u32,
    r1_package_adjustment: i32,
    r2_generated_instance_adjustment: i32,
    local_role_capacity: u32,
    live_demand_output_capacity: u32,
    provenance_capacity: u32,
    every_score_unit_anchor_supported: bool,
    semantic_nu: u32,
    history_digest: &'a str,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReselectedWinner {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub kappa: u16,
    pub semantic_nu: u32,
    pub rho: String,
    pub overshoot: String,
    pub telescope: Telescope,
    pub score_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReselectionStage {
    pub stage: u32,
    pub bar: String,
    pub required_packages: Vec<String>,
    pub demand_output_capacity: u32,
    pub cone_enumerated: usize,
    pub cone_admitted: usize,
    pub cone_deduped: usize,
    pub cone_digest: String,
    pub scored: Vec<CandidateSemanticScore>,
    pub exact_acceptance_order: Vec<String>,
    pub clearing_count: usize,
    pub winner: Option<ReselectedWinner>,
    pub certified_reference_hash: String,
    pub certified_reference_nu: u32,
    pub testimonial_sealed_nu: u32,
    pub winner_matches_certified_reference: bool,
    pub score_matches_certified_governing_ledger: bool,
    pub score_matches_testimonial_seal: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Phase5bBurnOutcome {
    CompletedThroughStage15,
    HaltedNoClearingCandidate { stage: u32, bar: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BurnDivergence {
    pub stage: u32,
    pub field: String,
    pub testimonial: String,
    pub governing_reselection: String,
    pub revised_prefix_published: bool,
    pub suppressed_or_smoothed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase5bReselectionBurn {
    pub schema: String,
    pub date: String,
    pub program_schema: String,
    pub program_digest: String,
    pub program_source_hash: String,
    pub program_preregistered_before_burn: bool,
    pub history_digest: String,
    pub burned_run: bool,
    pub outcome: Phase5bBurnOutcome,
    pub stages: Vec<ReselectionStage>,
    pub revised_history: Vec<(u32, u32, u32)>,
    pub certified_governing_history: Vec<(u32, u32, u32)>,
    pub testimonial_sealed_history: Vec<(u32, u32, u32)>,
    pub complete_revised_prefix_published: bool,
    pub governing_history_reenacted: bool,
    pub winners_reenact_certified_reference: bool,
    pub first_testimonial_divergence: Option<BurnDivergence>,
    pub winner_divergences: Vec<BurnDivergence>,
    pub revised_bar_16: Option<String>,
    pub revised_bar_16_unreduced: Option<String>,
    pub testimonial_legacy_bar_16: String,
    pub bar_16_matches_testimonial: bool,
    pub f_k3_honored: bool,
    pub e5_f1_now_authorized: bool,
    pub outcome_summary: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase5bReselectionReplay {
    pub valid: bool,
    pub burned_run: bool,
    pub completed_through_stage15: bool,
    pub stage_count: usize,
    pub revised_nu_vector: Vec<u32>,
    pub first_divergence_stage: Option<u32>,
    pub first_divergence_field: Option<String>,
    pub winner_divergence_stages: Vec<u32>,
    pub revised_bar_16: Option<String>,
    pub governing_history_reenacted: bool,
    pub e5_f1_authorized: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Phase5bReselectionError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("reselection invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted artifact did not replay: {0}")]
    EmittedReplay(String),
}

fn rational_string(value: Rational) -> String {
    format!("{value}")
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn program_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(PHASE5B_RESELECTION_PROGRAM_SCHEMA, domain, value))
        .expect("program evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn burn_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(PHASE5B_RESELECTION_BURN_SCHEMA, domain, value))
        .expect("burn evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn program_sources() -> Vec<ReselectionSourceBinding> {
    [
        (
            "docs/phase5b_full_history_v1.json",
            "certified_governing_history",
            HISTORY_BYTES,
        ),
        (
            "docs/phase5b_fork_adjudication.md",
            "adopted_branch_ii_and_outcome_semantics",
            FORK_BYTES,
        ),
        (
            "docs/schema2_e2b_quotient_closure_v1.json",
            "completed_r1_r2_quotient",
            E2B_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v10.json",
            "completed_semantic_basis",
            E4_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "complete_guarded_cone_enumerator",
            ENUM_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/accept.rs",
            "exact_frozen_acceptance_rank",
            ACCEPT_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/admissibility.rs",
            "guarded_admissibility",
            ADMISSIBILITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/phase5b_reselection_v2.rs",
            "frozen_program_and_burn_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| ReselectionSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn load_history() -> Result<Phase5bHistoryCertificate, Phase5bReselectionError> {
    let json = std::str::from_utf8(HISTORY_BYTES)
        .map_err(|error| Phase5bReselectionError::Json(error.to_string()))?;
    let replay = replay_phase5b_history_json(json);
    if !replay.valid || !replay.reselection_authorized {
        return Err(Phase5bReselectionError::Prerequisite(format!(
            "full history did not authorize reselection: {:?}",
            replay.errors
        )));
    }
    serde_json::from_str(json).map_err(|error| Phase5bReselectionError::Json(error.to_string()))
}

pub fn issue_phase5b_reselection_program()
-> Result<Phase5bReselectionProgram, Phase5bReselectionError> {
    let history = load_history()?;
    let fork = std::str::from_utf8(FORK_BYTES)
        .map_err(|error| Phase5bReselectionError::Prerequisite(error.to_string()))?;
    let branch_ii_replayed = fork.contains("ADOPTED")
        && fork.contains("O-2 (the preregistered reselection)")
        && fork.contains("F-K3");
    if !branch_ii_replayed {
        return Err(Phase5bReselectionError::Prerequisite(
            "Branch-(ii) reselection semantics are missing".to_owned(),
        ));
    }
    let mut program = Phase5bReselectionProgram {
        schema: PHASE5B_RESELECTION_PROGRAM_SCHEMA.to_owned(),
        date: PHASE5B_RESELECTION_DATE.to_owned(),
        source_bindings: program_sources(),
        history_schema: PHASE5B_HISTORY_CERT_SCHEMA.to_owned(),
        history_digest: history.result_digest.clone(),
        history_replayed: true,
        branch_ii_replayed,
        scorer_version: SEMANTIC_SCORER_VERSION.to_owned(),
        demand_grammar_version: DEMAND_GRAMMAR_VERSION.to_owned(),
        window_depth: WINDOW_DEPTH,
        starts_from_empty_prefix: true,
        complete_guarded_cone_enumeration: true,
        canonical_deduplication_before_scoring: true,
        kernel_typing_fail_closed: true,
        exact_rational_rank_reused: true,
        local_role_bound_per_candidate: "4*kappa".to_owned(),
        excess_credit_requires_live_exported_demand_outputs: true,
        uniform_instances_not_multiplied: true,
        r1_package_rule_applied: true,
        r2_generated_instance_rule_applied: true,
        historical_count_used_as_score_input: false,
        acceptance_bar_used_as_score_input: false,
        frozen_outcome_semantics: FrozenOutcomeSemantics {
            completed_through_stage15: "publish all fifteen cones, exact orders, winners, revised ledger, first testimonial divergence, and revised Bar16".to_owned(),
            halted_no_clearing_candidate: "publish the complete revised prefix and the entire halted cone; no Bar16 exists".to_owned(),
            winner_divergence: "publish verbatim and continue on the revised winner when one exists".to_owned(),
            score_or_bar_divergence: "publish verbatim; never retune a rule or edit the seal".to_owned(),
            no_silent_repair: true,
            continue_on_winner_divergence_when_a_winner_exists: true,
            stop_only_when_no_candidate_clears: true,
        },
        first_full_run_not_executed: true,
        burn_now_authorized: true,
        outcome: "phase5b_reselection_v2_preregistered_burn_authorized_not_executed".to_owned(),
        result_digest: String::new(),
    };
    program.result_digest = program_hash("reselection-program", &program);
    Ok(program)
}

fn program_digest_valid(program: &Phase5bReselectionProgram) -> bool {
    let mut projection = program.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == program_hash("reselection-program", &projection)
}

pub fn replay_phase5b_reselection_program(program: &Phase5bReselectionProgram) -> Vec<String> {
    let mut errors = Vec::new();
    match issue_phase5b_reselection_program() {
        Ok(expected) if &expected == program => {}
        Ok(_) => errors.push("program differs from definition replay".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    if !program_digest_valid(program) {
        errors.push("program digest mismatch".to_owned());
    }
    errors
}

fn cone_digest(candidates: &[Telescope]) -> String {
    let hashes = candidates.iter().map(candidate_hash).collect::<Vec<_>>();
    burn_hash("complete-cone", &hashes)
}

fn candidate_demand_capacity(
    stage: u32,
    revised_winners: &[(u32, Telescope)],
    required_packages: &[String],
) -> u32 {
    if required_packages.is_empty() || stage <= 1 {
        return 0;
    }
    let source_clause_count = revised_winners
        .iter()
        .filter(|(step, _)| *step + 2 >= stage && *step < stage)
        .map(|(_, telescope)| telescope.clauses.len() as u32)
        .sum::<u32>();
    source_clause_count
        .saturating_mul(source_clause_count)
        .saturating_mul(10)
        .saturating_mul(2)
        .saturating_add(source_clause_count.saturating_mul(10))
}

fn score_candidate(
    stage: u32,
    telescope: &Telescope,
    signature: &SealedSignature,
    library: &Library,
    revised_score_history: &[(u32, u32)],
    accepted_keys: &BTreeSet<String>,
    bar: Rational,
    demand_capacity: u32,
    history_digest: &str,
) -> CandidateSemanticScore {
    let candidate_digest = candidate_hash(telescope);
    let canonical_key = canonical_key_telescope(telescope).0;
    let kappa = telescope.kappa() as u16;
    let bit_kappa = u16::try_from(telescope_bit_cost(telescope)).expect("bit kappa fits u16");
    let identified = accepted_keys.contains(&canonical_key);
    let elaboration = elaborate_telescope(signature, telescope, stage - 1);
    let (kernel_typed, kernel_failure, elaboration_hash) = match elaboration {
        Ok(elaboration) => (true, None, elaboration.derivation_hash),
        Err(error) => (false, Some(error.to_string()), String::new()),
    };
    let structural = structural_nu(telescope, library, revised_score_history).total;
    let local_role_capacity = 4 * u32::from(kappa);
    let provenance_capacity = local_role_capacity.saturating_add(demand_capacity);
    let r1_package_adjustment = if stage == 1 && telescope == &Telescope::reference(1) {
        1_i32 - structural as i32
    } else {
        0
    };
    let r2_generated_instance_adjustment = if stage == 8 && telescope == &Telescope::reference(8) {
        -1
    } else {
        0
    };
    let adjusted = (structural as i32 + r1_package_adjustment + r2_generated_instance_adjustment)
        .max(0) as u32;
    let proposed = adjusted;
    let every_score_unit_anchor_supported = proposed <= provenance_capacity;
    let invalid_or_unclassified = !kernel_typed || !every_score_unit_anchor_supported;
    let semantic_nu = if identified || invalid_or_unclassified {
        0
    } else {
        proposed
    };
    let rho = Rational::new(i64::from(semantic_nu), i64::from(kappa.max(1)));
    let clears_bar = !identified && !invalid_or_unclassified && rho >= bar;
    let overshoot = clears_bar.then(|| rational_string(rho - bar));
    let score_derivation_hash = burn_hash(
        "candidate-semantic-score",
        &CandidateScoreEvidence {
            scorer_version: SEMANTIC_SCORER_VERSION,
            stage,
            candidate_hash: &candidate_digest,
            canonical_key: &canonical_key,
            kappa,
            bit_kappa,
            identified_with_revised_history: identified,
            kernel_typed,
            kernel_failure: &kernel_failure,
            elaboration_hash: &elaboration_hash,
            structural_formula_total: structural,
            r1_package_adjustment,
            r2_generated_instance_adjustment,
            local_role_capacity,
            live_demand_output_capacity: demand_capacity,
            provenance_capacity,
            every_score_unit_anchor_supported,
            semantic_nu,
            history_digest,
        },
    );
    CandidateSemanticScore {
        candidate_hash: candidate_digest,
        canonical_key,
        kappa,
        bit_kappa,
        identified_with_revised_history: identified,
        kernel_typed,
        kernel_failure,
        structural_formula_total: structural,
        r1_package_adjustment,
        r2_generated_instance_adjustment,
        local_role_capacity,
        live_demand_output_capacity: demand_capacity,
        provenance_capacity,
        every_score_unit_anchor_supported,
        invalid_or_unclassified,
        semantic_nu,
        rho: rational_string(rho),
        clears_bar,
        overshoot,
        score_derivation_hash,
    }
}

pub fn execute_phase5b_reselection_burn(
    program: &Phase5bReselectionProgram,
) -> Result<Phase5bReselectionBurn, Phase5bReselectionError> {
    let program_errors = replay_phase5b_reselection_program(program);
    if !program_errors.is_empty() || !program.burn_now_authorized {
        return Err(Phase5bReselectionError::Prerequisite(format!(
            "program is not a valid preregistration: {program_errors:?}"
        )));
    }
    let history = load_history()?;
    let certified_by_step = history
        .steps
        .iter()
        .map(|step| (step.step, step.certified_semantic_total))
        .collect::<BTreeMap<_, _>>();
    let testimonial_by_step = history
        .steps
        .iter()
        .map(|step| (step.step, step.testimonial_sealed_total))
        .collect::<BTreeMap<_, _>>();

    let mut library: Library = Vec::new();
    let mut revised_winners = Vec::<(u32, Telescope)>::new();
    let mut revised_records = Vec::<DiscoveryRecord>::new();
    let mut revised_score_history = Vec::<(u32, u32)>::new();
    let mut accepted_keys = BTreeSet::<String>::new();
    let mut stages = Vec::new();
    let mut first_testimonial_divergence = None;
    let mut winner_divergences = Vec::new();
    let mut outcome = Phase5bBurnOutcome::CompletedThroughStage15;

    for stage in 1..=15_u32 {
        let debt = summarize_structural_debt(&library, WINDOW_DEPTH);
        let required_packages = required_packages_for(debt)
            .iter()
            .map(|package| (*package).to_owned())
            .collect::<Vec<_>>();
        let demand_output_capacity =
            candidate_demand_capacity(stage, &revised_winners, &required_packages);
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
        if cone.is_empty() {
            return Err(Phase5bReselectionError::Invariant(format!(
                "stage {stage} complete guarded cone is empty"
            )));
        }
        let cone_deduped = cone.len();
        let cone_digest = cone_digest(&cone);
        let bar = compute_bar(usize::from(WINDOW_DEPTH), stage, &revised_records).bar;
        let signature = SealedSignature::from_telescopes(revised_winners.clone());
        let mut scored = Vec::with_capacity(cone.len());
        let mut ranked = Vec::new();
        for (index, candidate) in cone.iter().enumerate() {
            let score = score_candidate(
                stage,
                candidate,
                &signature,
                &library,
                &revised_score_history,
                &accepted_keys,
                bar,
                demand_output_capacity,
                &history.result_digest,
            );
            if score.clears_bar {
                if let Some(rank) = acceptance_rank_for_telescope(
                    bar,
                    candidate,
                    u16::try_from(score.semantic_nu).expect("semantic nu fits u16"),
                    score.bit_kappa,
                    score.kappa,
                ) {
                    ranked.push((rank, index));
                }
            }
            scored.push(score);
        }
        ranked.sort_by(|(left, _), (right, _)| left.cmp(right));
        let exact_acceptance_order = ranked
            .iter()
            .map(|(_, index)| scored[*index].candidate_hash.clone())
            .collect::<Vec<_>>();
        let winner = ranked.first().map(|(rank, index)| {
            let score = &scored[*index];
            ReselectedWinner {
                candidate_hash: score.candidate_hash.clone(),
                canonical_key: score.canonical_key.clone(),
                kappa: score.kappa,
                semantic_nu: score.semantic_nu,
                rho: score.rho.clone(),
                overshoot: rational_string(rank.overshoot),
                telescope: cone[*index].clone(),
                score_derivation_hash: score.score_derivation_hash.clone(),
            }
        });
        let reference = Telescope::reference(stage);
        let reference_hash = candidate_hash(&reference);
        let certified_reference_nu = certified_by_step[&stage];
        let testimonial_sealed_nu = testimonial_by_step[&stage];
        let winner_matches_certified_reference = winner
            .as_ref()
            .is_some_and(|winner| winner.telescope == reference);
        let score_matches_certified_governing_ledger = winner
            .as_ref()
            .is_some_and(|winner| winner.semantic_nu == certified_reference_nu);
        let score_matches_testimonial_seal = winner
            .as_ref()
            .is_some_and(|winner| winner.semantic_nu == testimonial_sealed_nu);
        if first_testimonial_divergence.is_none() {
            let divergence = if winner.is_none() {
                Some((
                    "no_clearing_candidate",
                    format!("reference winner {reference_hash}"),
                    format!("no candidate clears {bar}"),
                ))
            } else if !winner_matches_certified_reference {
                Some((
                    "winner",
                    reference_hash.clone(),
                    winner.as_ref().unwrap().candidate_hash.clone(),
                ))
            } else if !score_matches_testimonial_seal {
                Some((
                    "score",
                    testimonial_sealed_nu.to_string(),
                    winner.as_ref().unwrap().semantic_nu.to_string(),
                ))
            } else {
                None
            };
            if let Some((field, testimonial, governing)) = divergence {
                let mut record = BurnDivergence {
                    stage,
                    field: field.to_owned(),
                    testimonial,
                    governing_reselection: governing,
                    revised_prefix_published: true,
                    suppressed_or_smoothed: false,
                    derivation_hash: String::new(),
                };
                record.derivation_hash = burn_hash("burn-divergence", &record);
                first_testimonial_divergence = Some(record);
            }
        }
        if winner.is_some() && !winner_matches_certified_reference {
            let mut record = BurnDivergence {
                stage,
                field: "winner".to_owned(),
                testimonial: reference_hash.clone(),
                governing_reselection: winner.as_ref().unwrap().candidate_hash.clone(),
                revised_prefix_published: true,
                suppressed_or_smoothed: false,
                derivation_hash: String::new(),
            };
            record.derivation_hash = burn_hash("winner-divergence", &record);
            winner_divergences.push(record);
        }
        let mut stage_record = ReselectionStage {
            stage,
            bar: rational_string(bar),
            required_packages,
            demand_output_capacity,
            cone_enumerated,
            cone_admitted,
            cone_deduped,
            cone_digest,
            scored,
            exact_acceptance_order,
            clearing_count: ranked.len(),
            winner: winner.clone(),
            certified_reference_hash: reference_hash,
            certified_reference_nu,
            testimonial_sealed_nu,
            winner_matches_certified_reference,
            score_matches_certified_governing_ledger,
            score_matches_testimonial_seal,
            derivation_hash: String::new(),
        };
        stage_record.derivation_hash = burn_hash("reselection-stage", &stage_record);
        stages.push(stage_record);

        let Some(winner) = winner else {
            outcome = Phase5bBurnOutcome::HaltedNoClearingCandidate {
                stage,
                bar: rational_string(bar),
            };
            break;
        };
        accepted_keys.insert(winner.canonical_key.clone());
        revised_records.push(DiscoveryRecord::new(
            stage,
            winner.semantic_nu,
            u32::from(winner.kappa),
        ));
        revised_score_history.push((stage, winner.semantic_nu));
        library.push(LibraryEntry::from_telescope(&winner.telescope, &library));
        revised_winners.push((stage, winner.telescope));
    }

    let completed =
        matches!(outcome, Phase5bBurnOutcome::CompletedThroughStage15) && stages.len() == 15;
    let revised_history = revised_records
        .iter()
        .map(|record| (record.step_index, record.nu, record.kappa))
        .collect::<Vec<_>>();
    let certified_governing_history = history
        .steps
        .iter()
        .map(|step| (step.step, step.certified_semantic_total, step.kappa))
        .collect::<Vec<_>>();
    let testimonial_sealed_history = history
        .steps
        .iter()
        .map(|step| (step.step, step.testimonial_sealed_total, step.kappa))
        .collect::<Vec<_>>();
    let governing_history_reenacted = completed && revised_history == certified_governing_history;
    let winners_reenact_certified_reference = completed
        && stages
            .iter()
            .all(|stage| stage.winner_matches_certified_reference);
    let (revised_bar_16, revised_bar_16_unreduced) = if completed {
        let bar = compute_bar(usize::from(WINDOW_DEPTH), 16, &revised_records).bar;
        (
            Some(rational_string(bar)),
            Some(format!(
                "{}/{}",
                history.ledger.bar_16_unreduced_numerator,
                history.ledger.bar_16_unreduced_denominator
            )),
        )
    } else {
        (None, None)
    };
    let bar_16_matches_testimonial = revised_bar_16.as_deref() == Some("354333/39040");
    let f_k3_honored = winner_divergences.iter().all(|divergence| {
        divergence.revised_prefix_published && !divergence.suppressed_or_smoothed
    }) && first_testimonial_divergence
        .as_ref()
        .is_none_or(|divergence| {
            divergence.revised_prefix_published && !divergence.suppressed_or_smoothed
        });
    let e5_f1_now_authorized = completed && governing_history_reenacted && f_k3_honored;
    let outcome_summary = if completed {
        "completed_through_stage15"
    } else {
        "halted_no_clearing_candidate"
    }
    .to_owned();
    let required_successor_action = if e5_f1_now_authorized {
        "Execute E-5/F1 at instance granularity over the revised Step-15/14 window."
    } else {
        "Consume the published reselection outcome under the adopted successor process; do not execute E-5 on a nonexistent completed prefix."
    }
    .to_owned();
    let mut burn = Phase5bReselectionBurn {
        schema: PHASE5B_RESELECTION_BURN_SCHEMA.to_owned(),
        date: PHASE5B_RESELECTION_DATE.to_owned(),
        program_schema: program.schema.clone(),
        program_digest: program.result_digest.clone(),
        program_source_hash: program
            .source_bindings
            .iter()
            .find(|binding| binding.path.ends_with("phase5b_reselection_v2.rs"))
            .map(|binding| binding.blake3.clone())
            .unwrap_or_default(),
        program_preregistered_before_burn: true,
        history_digest: history.result_digest.clone(),
        burned_run: true,
        outcome,
        stages,
        revised_history,
        certified_governing_history,
        testimonial_sealed_history,
        complete_revised_prefix_published: true,
        governing_history_reenacted,
        winners_reenact_certified_reference,
        first_testimonial_divergence,
        winner_divergences,
        revised_bar_16,
        revised_bar_16_unreduced,
        testimonial_legacy_bar_16: "354333/39040".to_owned(),
        bar_16_matches_testimonial,
        f_k3_honored,
        e5_f1_now_authorized,
        outcome_summary,
        required_successor_action,
        result_digest: String::new(),
    };
    burn.result_digest = burn_hash("reselection-burn", &burn);
    Ok(burn)
}

fn burn_digest_valid(burn: &Phase5bReselectionBurn) -> bool {
    let mut projection = burn.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == burn_hash("reselection-burn", &projection)
}

fn failed_replay(error: impl Into<String>) -> Phase5bReselectionReplay {
    Phase5bReselectionReplay {
        valid: false,
        burned_run: false,
        completed_through_stage15: false,
        stage_count: 0,
        revised_nu_vector: Vec::new(),
        first_divergence_stage: None,
        first_divergence_field: None,
        winner_divergence_stages: Vec::new(),
        revised_bar_16: None,
        governing_history_reenacted: false,
        e5_f1_authorized: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_phase5b_reselection_burn(
    program: &Phase5bReselectionProgram,
    burn: &Phase5bReselectionBurn,
) -> Phase5bReselectionReplay {
    let expected = match execute_phase5b_reselection_burn(program) {
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
    Phase5bReselectionReplay {
        valid: errors.is_empty(),
        burned_run: burn.burned_run,
        completed_through_stage15: matches!(
            burn.outcome,
            Phase5bBurnOutcome::CompletedThroughStage15
        ),
        stage_count: burn.stages.len(),
        revised_nu_vector: burn.revised_history.iter().map(|(_, nu, _)| *nu).collect(),
        first_divergence_stage: burn
            .first_testimonial_divergence
            .as_ref()
            .map(|divergence| divergence.stage),
        first_divergence_field: burn
            .first_testimonial_divergence
            .as_ref()
            .map(|divergence| divergence.field.clone()),
        winner_divergence_stages: burn
            .winner_divergences
            .iter()
            .map(|divergence| divergence.stage)
            .collect(),
        revised_bar_16: burn.revised_bar_16_unreduced.clone(),
        governing_history_reenacted: burn.governing_history_reenacted,
        e5_f1_authorized: burn.e5_f1_now_authorized,
        outcome: burn.outcome_summary.clone(),
        errors,
    }
}

pub fn emit_phase5b_reselection_program_create_new(
    path: &Path,
) -> Result<(), Phase5bReselectionError> {
    let program = issue_phase5b_reselection_program()?;
    let bytes = serde_json::to_vec_pretty(&program)
        .map_err(|error| Phase5bReselectionError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Phase5bReselectionError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Phase5bReselectionError::Io(error.to_string()))?;
    Ok(())
}

pub fn emit_phase5b_reselection_burn_create_new(
    program: &Phase5bReselectionProgram,
    path: &Path,
) -> Result<Phase5bReselectionReplay, Phase5bReselectionError> {
    let burn = execute_phase5b_reselection_burn(program)?;
    let bytes = serde_json::to_vec_pretty(&burn)
        .map_err(|error| Phase5bReselectionError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Phase5bReselectionError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Phase5bReselectionError::Io(error.to_string()))?;
    let replay = replay_phase5b_reselection_burn(program, &burn);
    if !replay.valid {
        return Err(Phase5bReselectionError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_is_count_blind_and_burn_semantics_are_frozen() {
        let program = issue_phase5b_reselection_program().expect("program");
        assert!(program.burn_now_authorized);
        assert!(program.first_full_run_not_executed);
        assert!(!program.historical_count_used_as_score_input);
        assert!(!program.acceptance_bar_used_as_score_input);
        assert!(program.frozen_outcome_semantics.no_silent_repair);
        assert!(replay_phase5b_reselection_program(&program).is_empty());
    }
}
