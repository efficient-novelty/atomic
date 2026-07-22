//! T-BF3: fail-closed enactment-equivalence audit for the proposed
//! threshold-free demand-parsimony law.
//!
//! Selection reads only (clause kappa, certified semantic nu) for candidates
//! that already earned typed total-discharge evidence. Certified winners,
//! bars, overshoots, bit costs, exact ranks, and input order are not selector
//! inputs. An equal-pair minimum therefore halts.

use crate::phase5b_reselection_v2::{
    CandidateSemanticScore, PHASE5B_RESELECTION_BURN_SCHEMA, PHASE5B_RESELECTION_PROGRAM_SCHEMA,
    Phase5bBurnOutcome, Phase5bReselectionBurn, Phase5bReselectionProgram, ReselectionStage,
};
use crate::phase5b_reselection_v3::{
    PHASE5B_RESELECTION_V3_BURN_SCHEMA, PHASE5B_RESELECTION_V3_PROGRAM_SCHEMA,
    Phase5bReselectionV3Burn, Phase5bReselectionV3Program, V3CandidateAssessment, V3Outcome,
    V3StageRecord,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const T_BF3_SCHEMA: &str = "t-bf3-enactment-equivalence-v1";
pub const T_BF3_DATE: &str = "2026-07-21";
pub const T_BF3_LAW: &str = "demand-parsimony-law-v1";

const PROPOSAL_BYTES: &[u8] = include_bytes!("../../../docs/bar_free_law_proposal.md");
const V2_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v2.json");
const V2_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v2.json");
const V3_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v3.json");
const V3_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v3.json");
const ADMISSIBILITY_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_bf3_enactment_equivalence.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf3SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParsimonyCandidate {
    pub candidate_hash: String,
    pub kappa: u16,
    pub certified_nu: u32,
    pub kernel_typed: bool,
    pub total_discharger: bool,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tbf3StageDisposition {
    ConstitutiveBootstrap,
    SelectedUniqueParsimonyMinimum,
    HaltedEqualPairParsimonyTie,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf3StageRecord {
    pub stage: u32,
    pub disposition: Tbf3StageDisposition,
    pub required_packages: Vec<String>,
    pub stage3_exported_demand_wrinkle: bool,
    pub candidate_evidence_origin: String,
    pub candidates: Vec<ParsimonyCandidate>,
    pub minimum_kappa: Option<u16>,
    pub minimum_certified_nu_at_kappa: Option<u32>,
    pub minimizer_hashes: Vec<String>,
    pub minimizer_count: usize,
    pub selected_hash: Option<String>,
    pub certified_sequence_hash: String,
    pub selected_matches_certified_sequence: bool,
    pub selector_read_certified_sequence_hash: bool,
    pub selector_read_bar_or_overshoot: bool,
    pub selector_read_bit_kappa_or_exact_rank: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tbf3Outcome {
    TheoremIssued,
    FailedEqualPairParsimonyTie { stage: u32, count: usize },
    FailedWinnerDivergence { stage: u32 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf3Certificate {
    pub schema: String,
    pub date: String,
    pub proposed_law: String,
    pub source_bindings: Vec<Tbf3SourceBinding>,
    pub frozen_v2_program_digest_valid: bool,
    pub frozen_v2_burn_digest_valid: bool,
    pub frozen_v3_program_digest_valid: bool,
    pub frozen_v3_burn_digest_valid: bool,
    pub frozen_cross_artifact_links_valid: bool,
    pub frozen_surface_relative: bool,
    pub live_upstream_source_replay_claimed: bool,
    pub live_replay_hygiene_note: String,
    pub certified_domain: String,
    pub bootstrap_stages: Vec<u32>,
    pub comparative_stages: Vec<u32>,
    pub parsimony_order: String,
    pub certified_nu_used_as_selector_input: bool,
    pub historical_winner_used_as_selector_input: bool,
    pub testimonial_or_desired_result_used_as_selector_input: bool,
    pub bar_used_as_gate_or_selector: bool,
    pub stage_records: Vec<Tbf3StageRecord>,
    pub first_unresolved_stage: Option<u32>,
    pub unreached_stages_after_fail_closed_halt: Vec<u32>,
    pub f_bf1_equal_pair_tie_triggered: bool,
    pub f_bf2_winner_divergence_triggered: bool,
    pub theorem_issued: bool,
    pub enactment_equivalence_proved: bool,
    pub proposal_adoptable_from_t_bf3: bool,
    pub outcome: Tbf3Outcome,
    pub conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf3Replay {
    pub valid: bool,
    pub theorem_issued: bool,
    pub first_unresolved_stage: Option<u32>,
    pub minimizer_count_at_first_unresolved_stage: Option<usize>,
    pub f_bf1_triggered: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Tbf3Error {
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

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParsimonyDecision {
    Unique {
        kappa: u16,
        certified_nu: u32,
        candidate_hash: String,
    },
    Tie {
        kappa: u16,
        certified_nu: u32,
        candidate_hashes: Vec<String>,
    },
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BF3_SCHEMA, domain, value)).expect("evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn frozen_artifact_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("artifact serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<Tbf3SourceBinding> {
    [
        (
            "docs/bar_free_law_proposal.md",
            "threshold_free_law_and_falsifiers",
            PROPOSAL_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v2.json",
            "preregistered_complete_prefix_cone_program",
            V2_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v2.json",
            "complete_prefix_cones_and_certified_scores",
            V2_BURN_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v3.json",
            "preregistered_guarded_suffix_program",
            V3_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v3.json",
            "guarded_suffix_total_discharger_census",
            V3_BURN_BYTES,
        ),
        (
            "crates/pen-type/src/admissibility.rs",
            "strict_a4_cone_admission_definition",
            ADMISSIBILITY_BYTES,
        ),
        (
            "crates/pen-search/src/t_bf3_enactment_equivalence.rs",
            "fail_closed_parsimony_selector_and_replay",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Tbf3SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn parse_prerequisites() -> Result<
    (
        Phase5bReselectionProgram,
        Phase5bReselectionBurn,
        Phase5bReselectionV3Program,
        Phase5bReselectionV3Burn,
    ),
    Tbf3Error,
> {
    Ok((
        serde_json::from_slice(V2_PROGRAM_BYTES)
            .map_err(|error| Tbf3Error::Json(error.to_string()))?,
        serde_json::from_slice(V2_BURN_BYTES)
            .map_err(|error| Tbf3Error::Json(error.to_string()))?,
        serde_json::from_slice(V3_PROGRAM_BYTES)
            .map_err(|error| Tbf3Error::Json(error.to_string()))?,
        serde_json::from_slice(V3_BURN_BYTES)
            .map_err(|error| Tbf3Error::Json(error.to_string()))?,
    ))
}

fn verify_frozen_prerequisites(
    v2_program: &Phase5bReselectionProgram,
    v2_burn: &Phase5bReselectionBurn,
    v3_program: &Phase5bReselectionV3Program,
    v3_burn: &Phase5bReselectionV3Burn,
) -> Result<(), Tbf3Error> {
    let mut v2_program_projection = v2_program.clone();
    let v2_program_digest = v2_program_projection.result_digest.clone();
    v2_program_projection.result_digest.clear();
    let v2_program_digest_valid = v2_program_digest
        == frozen_artifact_hash(
            PHASE5B_RESELECTION_PROGRAM_SCHEMA,
            "reselection-program",
            &v2_program_projection,
        );

    let mut v2_burn_projection = v2_burn.clone();
    let v2_burn_digest = v2_burn_projection.result_digest.clone();
    v2_burn_projection.result_digest.clear();
    let v2_burn_digest_valid = v2_burn_digest
        == frozen_artifact_hash(
            PHASE5B_RESELECTION_BURN_SCHEMA,
            "reselection-burn",
            &v2_burn_projection,
        );

    let mut v3_program_projection = v3_program.clone();
    let v3_program_digest = v3_program_projection.result_digest.clone();
    v3_program_projection.result_digest.clear();
    let v3_program_digest_valid = v3_program_digest
        == frozen_artifact_hash(
            PHASE5B_RESELECTION_V3_PROGRAM_SCHEMA,
            "v3-program",
            &v3_program_projection,
        );

    let mut v3_burn_projection = v3_burn.clone();
    let v3_burn_digest = v3_burn_projection.result_digest.clone();
    v3_burn_projection.result_digest.clear();
    let v3_burn_digest_valid = v3_burn_digest
        == frozen_artifact_hash(
            PHASE5B_RESELECTION_V3_BURN_SCHEMA,
            "v3-burn",
            &v3_burn_projection,
        );

    let v3_source_hash = v3_program
        .source_bindings
        .iter()
        .find(|binding| binding.path.ends_with("phase5b_reselection_v3.rs"))
        .map(|binding| binding.blake3.as_str());
    let cross_links_valid = v2_burn.program_digest == v2_program.result_digest
        && v2_burn.history_digest == v2_program.history_digest
        && v3_program.imported_v2_program_digest == v2_program.result_digest
        && v3_program.imported_v2_burn_digest == v2_burn.result_digest
        && v3_burn.program_digest == v3_program.result_digest
        && v3_burn.imported_v2_burn_digest == v2_burn.result_digest
        && v3_source_hash == Some(v3_burn.program_source_hash.as_str());

    if !v2_program_digest_valid
        || !v2_burn_digest_valid
        || !v3_program_digest_valid
        || !v3_burn_digest_valid
        || !cross_links_valid
        || !matches!(
            &v2_burn.outcome,
            Phase5bBurnOutcome::HaltedNoClearingCandidate { stage: 8, .. }
        )
        || v2_burn.stages.len() != 8
    {
        return Err(Tbf3Error::Prerequisite(
            "frozen v2/v3 program or burn digest/cross-link validation failed".to_owned(),
        ));
    }
    if !matches!(&v3_burn.outcome, V3Outcome::CompletedThroughStage15)
        || !v3_burn.completed_through_stage15
        || !v3_burn.winner_divergences.is_empty()
        || !v3_burn.score_divergences.is_empty()
    {
        return Err(Tbf3Error::Prerequisite(
            "frozen v3 artifact is not the divergence-free completed burn".to_owned(),
        ));
    }
    Ok(())
}

fn choose_parsimony(candidates: &[ParsimonyCandidate]) -> Result<ParsimonyDecision, Tbf3Error> {
    let eligible = candidates
        .iter()
        .filter(|candidate| candidate.kernel_typed && candidate.total_discharger)
        .collect::<Vec<_>>();
    let minimum_kappa = eligible
        .iter()
        .map(|candidate| candidate.kappa)
        .min()
        .ok_or_else(|| Tbf3Error::Invariant("no typed total discharger".to_owned()))?;
    let minimum_certified_nu = eligible
        .iter()
        .filter(|candidate| candidate.kappa == minimum_kappa)
        .map(|candidate| candidate.certified_nu)
        .min()
        .expect("minimum-kappa subset is nonempty");
    let mut minimizers = eligible
        .into_iter()
        .filter(|candidate| {
            candidate.kappa == minimum_kappa && candidate.certified_nu == minimum_certified_nu
        })
        .map(|candidate| candidate.candidate_hash.clone())
        .collect::<Vec<_>>();
    minimizers.sort();
    minimizers.dedup();
    match minimizers.as_slice() {
        [only] => Ok(ParsimonyDecision::Unique {
            kappa: minimum_kappa,
            certified_nu: minimum_certified_nu,
            candidate_hash: only.clone(),
        }),
        _ => Ok(ParsimonyDecision::Tie {
            kappa: minimum_kappa,
            certified_nu: minimum_certified_nu,
            candidate_hashes: minimizers,
        }),
    }
}

fn v2_candidates(stage: &ReselectionStage) -> Result<Vec<ParsimonyCandidate>, Tbf3Error> {
    if stage.required_packages.is_empty()
        || stage.cone_admitted != stage.cone_deduped
        || stage.cone_deduped != stage.scored.len()
        || stage.scored.is_empty()
    {
        return Err(Tbf3Error::Invariant(format!(
            "stage {} lacks a complete guarded admitted cone",
            stage.stage
        )));
    }
    stage
        .scored
        .iter()
        .map(|score| v2_candidate(stage.stage, score))
        .collect()
}

fn v2_candidate(
    stage: u32,
    score: &CandidateSemanticScore,
) -> Result<ParsimonyCandidate, Tbf3Error> {
    if !score.kernel_typed
        || score.kernel_failure.is_some()
        || score.invalid_or_unclassified
        || !score.every_score_unit_anchor_supported
    {
        return Err(Tbf3Error::Invariant(format!(
            "stage {stage} cone member {} lacks typed score evidence",
            score.candidate_hash
        )));
    }
    Ok(ParsimonyCandidate {
        candidate_hash: score.candidate_hash.clone(),
        kappa: score.kappa,
        certified_nu: score.semantic_nu,
        kernel_typed: true,
        // V2 scores only after strict A4 admission and canonical deduplication.
        total_discharger: true,
        evidence_hash: score.score_derivation_hash.clone(),
    })
}

fn v3_candidates(stage: &V3StageRecord) -> Result<Vec<ParsimonyCandidate>, Tbf3Error> {
    if !stage.guarded || stage.required_packages.is_empty() {
        return Err(Tbf3Error::Invariant(format!(
            "stage {} is not guarded",
            stage.stage
        )));
    }
    let candidates = stage
        .assessments
        .iter()
        .filter(|assessment| assessment.guarded_total_discharger)
        .map(|assessment| v3_candidate(stage.stage, assessment))
        .collect::<Result<Vec<_>, _>>()?;
    if candidates.len() != stage.discharger_count
        || candidates
            .iter()
            .map(|candidate| candidate.candidate_hash.as_str())
            .collect::<Vec<_>>()
            != stage
                .discharger_hashes
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
    {
        return Err(Tbf3Error::Invariant(format!(
            "stage {} discharger census is inconsistent",
            stage.stage
        )));
    }
    Ok(candidates)
}

fn v3_candidate(
    stage: u32,
    assessment: &V3CandidateAssessment,
) -> Result<ParsimonyCandidate, Tbf3Error> {
    if !assessment.kernel_typed
        || assessment.kernel_failure.is_some()
        || !assessment.typed_provenance_supported
        || !assessment.guarded_total_discharger
    {
        return Err(Tbf3Error::Invariant(format!(
            "stage {stage} discharger {} lacks typed evidence",
            assessment.candidate_hash
        )));
    }
    Ok(ParsimonyCandidate {
        candidate_hash: assessment.candidate_hash.clone(),
        kappa: assessment.kappa,
        certified_nu: assessment.semantic_nu,
        kernel_typed: true,
        total_discharger: true,
        evidence_hash: assessment.derivation_hash.clone(),
    })
}

fn finish_stage(record: &mut Tbf3StageRecord) {
    record.derivation_hash.clear();
    record.derivation_hash = tagged_hash("stage", record);
}

fn bootstrap_record(stage: &ReselectionStage) -> Result<Tbf3StageRecord, Tbf3Error> {
    let winner = stage.winner.as_ref().ok_or_else(|| {
        Tbf3Error::Invariant(format!(
            "bootstrap stage {} has no registration",
            stage.stage
        ))
    })?;
    if stage.cone_deduped != 1
        || stage.scored.len() != 1
        || stage.scored[0].candidate_hash != winner.candidate_hash
        || !stage.winner_matches_certified_reference
        || !stage.score_matches_certified_governing_ledger
    {
        return Err(Tbf3Error::Invariant(format!(
            "bootstrap stage {} is not its certified unique registration",
            stage.stage
        )));
    }
    let score = &stage.scored[0];
    let mut record = Tbf3StageRecord {
        stage: stage.stage,
        disposition: Tbf3StageDisposition::ConstitutiveBootstrap,
        required_packages: stage.required_packages.clone(),
        stage3_exported_demand_wrinkle: stage.stage == 3
            && stage.required_packages.len() == 1
            && stage.required_packages[0] == "former_eliminator",
        candidate_evidence_origin: "v2 unique constitutive registration; no comparative selection"
            .to_owned(),
        candidates: vec![ParsimonyCandidate {
            candidate_hash: score.candidate_hash.clone(),
            kappa: score.kappa,
            certified_nu: score.semantic_nu,
            kernel_typed: score.kernel_typed,
            total_discharger: stage.required_packages.is_empty(),
            evidence_hash: score.score_derivation_hash.clone(),
        }],
        minimum_kappa: None,
        minimum_certified_nu_at_kappa: None,
        minimizer_hashes: Vec::new(),
        minimizer_count: 0,
        selected_hash: Some(winner.candidate_hash.clone()),
        certified_sequence_hash: stage.certified_reference_hash.clone(),
        selected_matches_certified_sequence: winner.candidate_hash
            == stage.certified_reference_hash,
        selector_read_certified_sequence_hash: false,
        selector_read_bar_or_overshoot: false,
        selector_read_bit_kappa_or_exact_rank: false,
        derivation_hash: String::new(),
    };
    finish_stage(&mut record);
    Ok(record)
}

fn comparative_record(
    stage: u32,
    required_packages: Vec<String>,
    candidates: Vec<ParsimonyCandidate>,
    certified_sequence_hash: String,
) -> Result<Tbf3StageRecord, Tbf3Error> {
    let decision = choose_parsimony(&candidates)?;
    let (disposition, kappa, nu, minimizer_hashes, selected_hash) = match decision {
        ParsimonyDecision::Unique {
            kappa,
            certified_nu,
            candidate_hash,
        } => (
            Tbf3StageDisposition::SelectedUniqueParsimonyMinimum,
            kappa,
            certified_nu,
            vec![candidate_hash.clone()],
            Some(candidate_hash),
        ),
        ParsimonyDecision::Tie {
            kappa,
            certified_nu,
            candidate_hashes,
        } => (
            Tbf3StageDisposition::HaltedEqualPairParsimonyTie,
            kappa,
            certified_nu,
            candidate_hashes,
            None,
        ),
    };
    let selected_matches_certified_sequence = selected_hash
        .as_ref()
        .is_some_and(|selected| selected == &certified_sequence_hash);
    let mut record = Tbf3StageRecord {
        stage,
        disposition,
        required_packages,
        stage3_exported_demand_wrinkle: false,
        candidate_evidence_origin: if stage <= 7 {
            "v2 complete strict-admitted typed cone; bar/rank ignored"
        } else {
            "v3 complete typed total-discharger census; bar ignored"
        }
        .to_owned(),
        candidates,
        minimum_kappa: Some(kappa),
        minimum_certified_nu_at_kappa: Some(nu),
        minimizer_count: minimizer_hashes.len(),
        minimizer_hashes,
        selected_hash,
        certified_sequence_hash,
        selected_matches_certified_sequence,
        selector_read_certified_sequence_hash: false,
        selector_read_bar_or_overshoot: false,
        selector_read_bit_kappa_or_exact_rank: false,
        derivation_hash: String::new(),
    };
    finish_stage(&mut record);
    Ok(record)
}

pub fn issue_t_bf3_certificate() -> Result<Tbf3Certificate, Tbf3Error> {
    let proposal = std::str::from_utf8(PROPOSAL_BYTES)
        .map_err(|error| Tbf3Error::Prerequisite(error.to_string()))?;
    if !proposal.contains("demand-parsimony-law-v1")
        || !proposal.contains("least κ, then least certified ν")
        || !proposal.contains("F-BF1")
        || !proposal.contains("T-BF3 (enactment equivalence)")
        || !proposal.contains("PROPOSAL")
    {
        return Err(Tbf3Error::Prerequisite(
            "proposal or fail-closed tie falsifier is absent".to_owned(),
        ));
    }
    let (v2_program, v2_burn, v3_program, v3_burn) = parse_prerequisites()?;
    verify_frozen_prerequisites(&v2_program, &v2_burn, &v3_program, &v3_burn)?;

    let mut stage_records = Vec::new();
    for stage_number in 1..=3 {
        let stage = v2_burn
            .stages
            .iter()
            .find(|stage| stage.stage == stage_number)
            .ok_or_else(|| Tbf3Error::Invariant(format!("missing stage {stage_number}")))?;
        stage_records.push(bootstrap_record(stage)?);
    }
    if !stage_records[2].stage3_exported_demand_wrinkle {
        return Err(Tbf3Error::Invariant(
            "Stage-3 demand-export wrinkle was not preserved".to_owned(),
        ));
    }

    let mut first_unresolved_stage = None;
    let mut outcome = Tbf3Outcome::TheoremIssued;
    for stage_number in 4..=7 {
        let stage = v2_burn
            .stages
            .iter()
            .find(|stage| stage.stage == stage_number)
            .ok_or_else(|| Tbf3Error::Invariant(format!("missing stage {stage_number}")))?;
        let reference = stage
            .winner
            .as_ref()
            .ok_or_else(|| Tbf3Error::Invariant(format!("stage {stage_number} has no winner")))?
            .candidate_hash
            .clone();
        let record = comparative_record(
            stage_number,
            stage.required_packages.clone(),
            v2_candidates(stage)?,
            reference,
        )?;
        if matches!(
            &record.disposition,
            Tbf3StageDisposition::HaltedEqualPairParsimonyTie
        ) {
            first_unresolved_stage = Some(stage_number);
            outcome = Tbf3Outcome::FailedEqualPairParsimonyTie {
                stage: stage_number,
                count: record.minimizer_count,
            };
            stage_records.push(record);
            break;
        }
        if !record.selected_matches_certified_sequence {
            first_unresolved_stage = Some(stage_number);
            outcome = Tbf3Outcome::FailedWinnerDivergence {
                stage: stage_number,
            };
            stage_records.push(record);
            break;
        }
        stage_records.push(record);
    }

    if first_unresolved_stage.is_none() {
        for stage in &v3_burn.stages {
            let reference = stage
                .winner
                .as_ref()
                .ok_or_else(|| {
                    Tbf3Error::Invariant(format!("stage {} has no v3 winner", stage.stage))
                })?
                .candidate_hash
                .clone();
            let record = comparative_record(
                stage.stage,
                stage.required_packages.clone(),
                v3_candidates(stage)?,
                reference,
            )?;
            if matches!(
                &record.disposition,
                Tbf3StageDisposition::HaltedEqualPairParsimonyTie
            ) {
                first_unresolved_stage = Some(stage.stage);
                outcome = Tbf3Outcome::FailedEqualPairParsimonyTie {
                    stage: stage.stage,
                    count: record.minimizer_count,
                };
                stage_records.push(record);
                break;
            }
            if !record.selected_matches_certified_sequence {
                first_unresolved_stage = Some(stage.stage);
                outcome = Tbf3Outcome::FailedWinnerDivergence { stage: stage.stage };
                stage_records.push(record);
                break;
            }
            stage_records.push(record);
        }
    }

    let theorem_issued = first_unresolved_stage.is_none()
        && stage_records.len() == 15
        && stage_records
            .iter()
            .all(|record| record.selected_matches_certified_sequence);
    let f_bf1 = matches!(&outcome, Tbf3Outcome::FailedEqualPairParsimonyTie { .. });
    let f_bf2 = matches!(&outcome, Tbf3Outcome::FailedWinnerDivergence { .. });
    let unreached = first_unresolved_stage
        .map(|stage| ((stage + 1)..=15).collect())
        .unwrap_or_default();
    let (conclusion, successor) = if theorem_issued {
        (
            "T-BF3 is proved: demand-parsimony-law-v1 reenacts the certified v3 sequence without consulting the bar.".to_owned(),
            "Combine only with independently replayed T-BF1 and T-BF2; this artifact does not adopt the law.".to_owned(),
        )
    } else if f_bf1 {
        (
            "T-BF3 is not proved. F-BF1 fires at Stage 4: four canonically distinct typed total dischargers share (kappa, certified nu) = (3, 5).".to_owned(),
            "Do not adopt demand-parsimony-law-v1. Prove an independent quotient collapsing the four minimizers or adjudicate a versioned additional selector, then rerun T-BF1 and T-BF3.".to_owned(),
        )
    } else {
        (
            "T-BF3 is not proved because threshold-free selection diverges from the certified sequence.".to_owned(),
            "Publish the divergence and reject the proposal as stated; do not inherit the old rank.".to_owned(),
        )
    };

    let mut certificate = Tbf3Certificate {
        schema: T_BF3_SCHEMA.to_owned(),
        date: T_BF3_DATE.to_owned(),
        proposed_law: T_BF3_LAW.to_owned(),
        source_bindings: source_bindings(),
        frozen_v2_program_digest_valid: true,
        frozen_v2_burn_digest_valid: true,
        frozen_v3_program_digest_valid: true,
        frozen_v3_burn_digest_valid: true,
        frozen_cross_artifact_links_valid: true,
        frozen_surface_relative: true,
        live_upstream_source_replay_claimed: false,
        live_replay_hygiene_note: "The archived v2/v3 bytes and their internal digests/cross-links are replayed. No claim is made that later source-bound upstream prerequisites still equal their frozen issue-time sources; current development has intentionally advanced that live surface."
            .to_owned(),
        certified_domain:
            "constitutive bootstrap Stages 1--3 plus sequential comparative Stages 4--15".to_owned(),
        bootstrap_stages: vec![1, 2, 3],
        comparative_stages: (4..=15).collect(),
        parsimony_order: "least clause kappa, then least certified semantic nu; equal pair halts"
            .to_owned(),
        certified_nu_used_as_selector_input: true,
        historical_winner_used_as_selector_input: false,
        testimonial_or_desired_result_used_as_selector_input: false,
        bar_used_as_gate_or_selector: false,
        stage_records,
        first_unresolved_stage,
        unreached_stages_after_fail_closed_halt: unreached,
        f_bf1_equal_pair_tie_triggered: f_bf1,
        f_bf2_winner_divergence_triggered: f_bf2,
        theorem_issued,
        enactment_equivalence_proved: theorem_issued,
        proposal_adoptable_from_t_bf3: theorem_issued,
        outcome,
        conclusion,
        required_successor_action: successor,
        result_digest: String::new(),
    };
    certificate.result_digest = tagged_hash("certificate", &certificate);
    Ok(certificate)
}

fn digest_valid(certificate: &Tbf3Certificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == tagged_hash("certificate", &projection)
}

pub fn replay_t_bf3_certificate(certificate: &Tbf3Certificate) -> Tbf3Replay {
    let expected = match issue_t_bf3_certificate() {
        Ok(expected) => expected,
        Err(error) => {
            return Tbf3Replay {
                valid: false,
                theorem_issued: false,
                first_unresolved_stage: None,
                minimizer_count_at_first_unresolved_stage: None,
                f_bf1_triggered: false,
                errors: vec![error.to_string()],
            };
        }
    };
    let mut errors = Vec::new();
    if certificate != &expected {
        errors.push("certificate differs from independent replay".to_owned());
    }
    if !digest_valid(certificate) {
        errors.push("certificate digest mismatch".to_owned());
    }
    let count = certificate.first_unresolved_stage.and_then(|stage| {
        certificate
            .stage_records
            .iter()
            .find(|record| record.stage == stage)
            .map(|record| record.minimizer_count)
    });
    Tbf3Replay {
        valid: errors.is_empty(),
        theorem_issued: certificate.theorem_issued,
        first_unresolved_stage: certificate.first_unresolved_stage,
        minimizer_count_at_first_unresolved_stage: count,
        f_bf1_triggered: certificate.f_bf1_equal_pair_tie_triggered,
        errors,
    }
}

pub fn emit_t_bf3_certificate_create_new(path: &Path) -> Result<Tbf3Replay, Tbf3Error> {
    let certificate = issue_t_bf3_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Tbf3Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Tbf3Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Tbf3Error::Io(error.to_string()))?;
    let replay = replay_t_bf3_certificate(&certificate);
    if !replay.valid {
        return Err(Tbf3Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(hash: &str, kappa: u16, nu: u32) -> ParsimonyCandidate {
        ParsimonyCandidate {
            candidate_hash: hash.to_owned(),
            kappa,
            certified_nu: nu,
            kernel_typed: true,
            total_discharger: true,
            evidence_hash: format!("evidence:{hash}"),
        }
    }

    #[test]
    fn equal_pair_tie_is_order_invariant() {
        let forward = vec![
            candidate("old-winner", 3, 5),
            candidate("other-a", 3, 5),
            candidate("other-b", 3, 5),
            candidate("other-c", 3, 5),
            candidate("larger-kappa", 4, 1),
        ];
        let mut reverse = forward.clone();
        reverse.reverse();
        let expected = ParsimonyDecision::Tie {
            kappa: 3,
            certified_nu: 5,
            candidate_hashes: vec![
                "old-winner".to_owned(),
                "other-a".to_owned(),
                "other-b".to_owned(),
                "other-c".to_owned(),
            ],
        };
        assert_eq!(choose_parsimony(&forward).unwrap(), expected);
        assert_eq!(choose_parsimony(&reverse).unwrap(), expected);
    }

    #[test]
    fn ineligible_lower_pair_cannot_enter_selector() {
        let candidates = vec![
            ParsimonyCandidate {
                kernel_typed: false,
                ..candidate("untyped", 1, 0)
            },
            ParsimonyCandidate {
                total_discharger: false,
                ..candidate("partial", 2, 0)
            },
            candidate("earned", 3, 5),
        ];
        assert_eq!(
            choose_parsimony(&candidates).unwrap(),
            ParsimonyDecision::Unique {
                kappa: 3,
                certified_nu: 5,
                candidate_hash: "earned".to_owned(),
            }
        );
    }

    #[test]
    #[ignore = "full v2+v3 prerequisite replay"]
    fn full_certificate_honors_f_bf1_at_stage4() {
        let certificate = issue_t_bf3_certificate().expect("T-BF3 audit");
        assert!(!certificate.theorem_issued);
        assert_eq!(certificate.first_unresolved_stage, Some(4));
        let stage4 = certificate.stage_records.last().unwrap();
        assert_eq!(stage4.minimum_kappa, Some(3));
        assert_eq!(stage4.minimum_certified_nu_at_kappa, Some(5));
        assert_eq!(stage4.minimizer_count, 4);
        assert!(stage4.selected_hash.is_none());
    }
}
