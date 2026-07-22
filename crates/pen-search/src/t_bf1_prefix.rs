//! T-BF1: bar-free prefix replay for \`demand-parsimony-law-v1\`.
//!
//! The selector sees only the complete strict-discharge cone and the ordered
//! pair \`(kappa, certified_nu)\`. Reference hashes, the diagnostic bar,
//! bit-cost, and canonical ordering are retained only as audit data after the
//! selection attempt. An equal minimum pair is therefore a terminal F-BF1
//! outcome, not an invitation to reuse the legacy rank.

use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::phase5b_reselection_v2::{
    CandidateSemanticScore, Phase5bReselectionBurn, Phase5bReselectionProgram,
    replay_phase5b_reselection_program,
};
use crate::phase5b_reselection_v3::Phase5bReselectionV3Burn;
use pen_core::canonical::canonical_key_telescope;
use pen_core::encode::telescope_bit_cost;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_eval::debt_guard::directive_debt_timeline;
use pen_eval::nu::structural_nu;
use pen_type::admissibility::{
    AdmissibilityMode, passes_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::elaborate::{SealedSignature, candidate_hash, elaborate_telescope};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const T_BF1_SCHEMA: &str = "t-bf1-prefix-v1";
pub const T_BF1_DATE: &str = "2026-07-21";
pub const T_BF1_LAW: &str = "demand-parsimony-law-v1";
pub const T_BF1_V3_BURN_DIGEST: &str =
    "blake3:65ac7f569968666035bf88026e8f9034d51b23eaa15d793b9ca6aaf08e256870";

const PROPOSAL_BYTES: &[u8] = include_bytes!("../../../docs/bar_free_law_proposal.md");
const V2_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v2.json");
const V2_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v2.json");
const V3_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v3.json");
const V2_SOURCE_BYTES: &[u8] = include_bytes!("phase5b_reselection_v2.rs");
const ENUM_SOURCE_BYTES: &[u8] = include_bytes!("enumerate.rs");
const NU_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/nu.rs");
const ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const DEBT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/debt_guard.rs");
const ADMISSIBILITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_bf1_prefix.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf1SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParsimonyPair {
    pub kappa: u16,
    pub certified_nu: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf1Candidate {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub kappa: u16,
    pub certified_nu: u32,
    pub bit_kappa_audit_only: u16,
    pub kernel_typed: bool,
    pub strict_total_discharger: bool,
    pub score_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrefixJurisdiction {
    ConstitutiveDemandless,
    ConstitutivePreStructuralWrinkle,
    DemandParsimony,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrefixDecision {
    ConstitutivelyRegistered,
    SelectedUniqueMinimum,
    StoppedFBf1EqualMinimumPair,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf1Stage {
    pub stage: u32,
    pub jurisdiction: PrefixJurisdiction,
    pub required_packages: Vec<String>,
    pub engine_focus_family: String,
    pub pre_structural_band: bool,
    pub stage3_demand_before_jurisdiction_wrinkle: bool,
    pub cone_enumerated: usize,
    pub strict_cone_admitted: usize,
    pub canonical_cone_deduped: usize,
    pub candidates_seen_by_parsimony: Vec<Tbf1Candidate>,
    pub minimum_pair: Option<ParsimonyPair>,
    pub minimizer_hashes: Vec<String>,
    pub selected_hash: Option<String>,
    pub certified_v3_reference_hash_audit_only: String,
    pub selection_matches_reference: Option<bool>,
    pub decision: PrefixDecision,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tbf1Outcome {
    ProvedThroughStage7,
    StoppedFBf1ParsimonyTie { stage: u32, count: usize },
    StoppedReferenceDivergence { stage: u32 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf1PrefixCertificate {
    pub schema: String,
    pub date: String,
    pub law: String,
    pub source_bindings: Vec<Tbf1SourceBinding>,
    pub proposal_status_remains_proposal: bool,
    pub v2_frozen_program_digest_valid: bool,
    pub v2_frozen_burn_digest_valid: bool,
    pub v2_live_definition_replay_valid: bool,
    pub v2_live_definition_replay_errors: Vec<String>,
    pub stage4_live_cone_recomputed_without_v2_history_chain: bool,
    pub stage4_live_cone_matches_frozen_certificate: bool,
    pub stage4_live_nu_matches_certified_nu: bool,
    pub v3_prefix_binding_digest: String,
    pub v3_imported_prefix_matches_v2: bool,
    pub selector_order: Vec<String>,
    pub selector_can_read_diagnostic_bar: bool,
    pub selector_can_read_bit_kappa: bool,
    pub selector_can_read_candidate_hash_or_canonical_key: bool,
    pub selector_can_read_certified_reference: bool,
    pub desired_outcome_used_as_premise: bool,
    pub stages: Vec<Tbf1Stage>,
    pub unreached_stages: Vec<u32>,
    pub first_equal_pair_tie_stage: Option<u32>,
    pub first_reference_divergence_stage: Option<u32>,
    pub stage3_wrinkle_preserved: bool,
    pub theorem_t_bf1_proved: bool,
    pub bar_free_adoption_unlocked_by_t_bf1: bool,
    pub outcome: Tbf1Outcome,
    pub finding: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tbf1Replay {
    pub valid: bool,
    pub theorem_t_bf1_proved: bool,
    pub stopped_stage: Option<u32>,
    pub minimizer_count_at_stop: Option<usize>,
    pub stage3_wrinkle_preserved: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Tbf1Error {
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

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BF1_SCHEMA, domain, value)).expect("evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn predecessor_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn v2_program_digest_valid(program: &Phase5bReselectionProgram) -> bool {
    let mut projection = program.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == predecessor_hash(
            "phase5b-reselection-program-v2",
            "reselection-program",
            &projection,
        )
}

fn v2_burn_digest_valid(burn: &Phase5bReselectionBurn) -> bool {
    let mut projection = burn.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == predecessor_hash(
            "phase5b-reselection-burn-v2",
            "reselection-burn",
            &projection,
        )
}

fn v3_burn_digest_valid(burn: &Phase5bReselectionV3Burn) -> bool {
    let mut projection = burn.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == predecessor_hash(
            "phase5b-reselection-burn-v3-two-register",
            "v3-burn",
            &projection,
        )
}

fn source_bindings() -> Vec<Tbf1SourceBinding> {
    [
        (
            "docs/bar_free_law_proposal.md",
            "unsigned_law_and_falsifier_specification",
            PROPOSAL_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v2.json",
            "frozen_complete_prefix_cone_program",
            V2_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v2.json",
            "replayable_complete_prefix_cones_and_certified_candidate_nu",
            V2_BURN_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v3.json",
            "post_selection_reference_comparator_only",
            V3_BURN_BYTES,
        ),
        (
            "crates/pen-search/src/phase5b_reselection_v2.rs",
            "complete_cone_and_candidate_score_replay",
            V2_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "current_complete_stage4_cone_enumerator",
            ENUM_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/nu.rs",
            "current_stage4_structural_nu_recomputation",
            NU_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/elaborate.rs",
            "current_stage4_kernel_typing_recomputation",
            ELABORATE_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/debt_guard.rs",
            "coarse_demand_and_stage3_wrinkle_projection",
            DEBT_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/admissibility.rs",
            "strict_a4_discharge_admission",
            ADMISSIBILITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_bf1_prefix.rs",
            "count_blind_t_bf1_issuer_and_replay",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Tbf1SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn load_inputs() -> Result<
    (
        Phase5bReselectionProgram,
        Phase5bReselectionBurn,
        Phase5bReselectionV3Burn,
    ),
    Tbf1Error,
> {
    let proposal = std::str::from_utf8(PROPOSAL_BYTES)
        .map_err(|error| Tbf1Error::Prerequisite(error.to_string()))?;
    for needle in [
        T_BF1_LAW,
        "least κ, then least certified ν",
        "F-BF1",
        "equal κ and equal certified ν",
        "the stage-3 wrinkle stands as recorded",
    ] {
        if !proposal.contains(needle) {
            return Err(Tbf1Error::Prerequisite(format!(
                "proposal no longer contains frozen clause {needle:?}"
            )));
        }
    }

    let program: Phase5bReselectionProgram = serde_json::from_slice(V2_PROGRAM_BYTES)
        .map_err(|error| Tbf1Error::Json(error.to_string()))?;
    let burn: Phase5bReselectionBurn = serde_json::from_slice(V2_BURN_BYTES)
        .map_err(|error| Tbf1Error::Json(error.to_string()))?;
    if !v2_program_digest_valid(&program)
        || !v2_burn_digest_valid(&burn)
        || burn.program_digest != program.result_digest
        || burn.stages.len() < 7
    {
        return Err(Tbf1Error::Prerequisite(
            "frozen v2 program/burn digest, linkage, or prefix cone is invalid".to_owned(),
        ));
    }

    let v3: Phase5bReselectionV3Burn = serde_json::from_slice(V3_BURN_BYTES)
        .map_err(|error| Tbf1Error::Json(error.to_string()))?;
    if v3.result_digest != T_BF1_V3_BURN_DIGEST
        || !v3_burn_digest_valid(&v3)
        || !v3.completed_through_stage15
    {
        return Err(Tbf1Error::Prerequisite(
            "v3 reference artifact identity or completion flag drifted".to_owned(),
        ));
    }
    Ok((program, burn, v3))
}

fn candidate(score: &CandidateSemanticScore) -> Tbf1Candidate {
    Tbf1Candidate {
        candidate_hash: score.candidate_hash.clone(),
        canonical_key: score.canonical_key.clone(),
        kappa: score.kappa,
        certified_nu: score.semantic_nu,
        bit_kappa_audit_only: score.bit_kappa,
        kernel_typed: score.kernel_typed,
        strict_total_discharger: score.kernel_typed
            && !score.invalid_or_unclassified
            && !score.identified_with_revised_history
            && score.every_score_unit_anchor_supported,
        score_derivation_hash: score.score_derivation_hash.clone(),
    }
}

fn recompute_live_stage4_candidates() -> Result<Vec<Tbf1Candidate>, Tbf1Error> {
    let winners = (1..=3_u32)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    let mut library: Library = Vec::new();
    for (_, telescope) in &winners {
        library.push(LibraryEntry::from_telescope(telescope, &library));
    }
    let admissibility = strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::Guarded);
    let context = EnumerationContext::from_admissibility(&library, admissibility);
    let mut enumerated = Vec::new();
    for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        enumerated.extend(enumerate_telescopes(&library, context, kappa));
    }
    let mut seen = BTreeSet::new();
    let cone = enumerated
        .into_iter()
        .filter(|telescope| passes_strict_admissibility(4, &library, telescope, admissibility))
        .filter(|telescope| seen.insert(canonical_key_telescope(telescope).0))
        .collect::<Vec<_>>();
    let signature = SealedSignature::from_telescopes(winners);
    let score_history = [(1, 1), (2, 1), (3, 2)];
    cone.iter()
        .map(|telescope| {
            let elaboration = elaborate_telescope(&signature, telescope, 3)
                .map_err(|error| Tbf1Error::Invariant(error.to_string()))?;
            let kappa = u16::try_from(telescope.kappa())
                .map_err(|error| Tbf1Error::Invariant(error.to_string()))?;
            let certified_nu = structural_nu(telescope, &library, &score_history).total;
            Ok(Tbf1Candidate {
                candidate_hash: candidate_hash(telescope),
                canonical_key: canonical_key_telescope(telescope).0,
                kappa,
                certified_nu,
                bit_kappa_audit_only: u16::try_from(telescope_bit_cost(telescope))
                    .map_err(|error| Tbf1Error::Invariant(error.to_string()))?,
                kernel_typed: true,
                strict_total_discharger: true,
                score_derivation_hash: elaboration.derivation_hash,
            })
        })
        .collect()
}

pub fn issue_t_bf1_prefix_certificate() -> Result<Tbf1PrefixCertificate, Tbf1Error> {
    let (program, burn, v3) = load_inputs()?;
    let v2_live_definition_replay_errors = replay_phase5b_reselection_program(&program);
    let v2_live_definition_replay_valid = v2_live_definition_replay_errors.is_empty();
    let live_stage4 = recompute_live_stage4_candidates()?;
    let v2_prefix = burn
        .revised_history
        .iter()
        .take(7)
        .copied()
        .collect::<Vec<_>>();
    if v2_prefix.len() != 7 || v3.imported_prefix != v2_prefix {
        return Err(Tbf1Error::Prerequisite(
            "v3 did not import the exact certified v2 Steps 1--7 prefix".to_owned(),
        ));
    }

    let debt = directive_debt_timeline();
    let stage3 = debt
        .iter()
        .find(|row| row.stage == 3)
        .ok_or_else(|| Tbf1Error::Invariant("debt timeline omits Stage 3".to_owned()))?;
    let stage3_wrinkle_preserved = stage3.pre_structural_band
        && stage3.focus_family == "None"
        && stage3.required_packages == vec!["former_eliminator"];
    if !stage3_wrinkle_preserved {
        return Err(Tbf1Error::Invariant(
            "Stage-3 demand-before-jurisdiction wrinkle was not preserved".to_owned(),
        ));
    }
    let frozen_stage4 = burn
        .stages
        .iter()
        .find(|stage| stage.stage == 4)
        .ok_or_else(|| Tbf1Error::Invariant("v2 burn omits Stage 4".to_owned()))?;
    let frozen_stage4_candidates = frozen_stage4
        .scored
        .iter()
        .map(candidate)
        .collect::<Vec<_>>();
    let live_keys = live_stage4
        .iter()
        .map(|candidate| {
            (
                candidate.candidate_hash.as_str(),
                candidate.canonical_key.as_str(),
                candidate.kappa,
                candidate.bit_kappa_audit_only,
            )
        })
        .collect::<Vec<_>>();
    let frozen_keys = frozen_stage4_candidates
        .iter()
        .map(|candidate| {
            (
                candidate.candidate_hash.as_str(),
                candidate.canonical_key.as_str(),
                candidate.kappa,
                candidate.bit_kappa_audit_only,
            )
        })
        .collect::<Vec<_>>();
    let stage4_live_cone_matches_frozen_certificate = live_keys == frozen_keys;
    let stage4_live_nu_matches_certified_nu = live_stage4
        .iter()
        .zip(&frozen_stage4_candidates)
        .all(|(live, frozen)| live.certified_nu == frozen.certified_nu);
    if !stage4_live_cone_matches_frozen_certificate || !stage4_live_nu_matches_certified_nu {
        return Err(Tbf1Error::Invariant(
            "current count-blind Stage-4 cone or nu diverges from the frozen certificate"
                .to_owned(),
        ));
    }

    let mut stages = Vec::new();
    let mut first_equal_pair_tie_stage = None;
    let mut first_reference_divergence_stage = None;

    for stage_number in 1..=7_u32 {
        let source = burn
            .stages
            .iter()
            .find(|stage| stage.stage == stage_number)
            .ok_or_else(|| Tbf1Error::Invariant(format!("v2 burn omits Stage {stage_number}")))?;
        let debt_row = debt
            .iter()
            .find(|row| row.stage == stage_number)
            .ok_or_else(|| {
                Tbf1Error::Invariant(format!("debt timeline omits Stage {stage_number}"))
            })?;
        if source.required_packages
            != debt_row
                .required_packages
                .iter()
                .map(|name| (*name).to_owned())
                .collect::<Vec<_>>()
        {
            return Err(Tbf1Error::Invariant(format!(
                "Stage {stage_number} demand projection disagrees with the replayed cone"
            )));
        }

        let (jurisdiction, decision, candidates, minimum_pair, minimizer_hashes, selected_hash) =
            if stage_number <= 3 {
                let winner = source.winner.as_ref().ok_or_else(|| {
                    Tbf1Error::Invariant(format!(
                        "constitutive Stage {stage_number} has no registered act"
                    ))
                })?;
                let jurisdiction = if stage_number == 3 {
                    PrefixJurisdiction::ConstitutivePreStructuralWrinkle
                } else {
                    PrefixJurisdiction::ConstitutiveDemandless
                };
                (
                    jurisdiction,
                    PrefixDecision::ConstitutivelyRegistered,
                    Vec::new(),
                    None,
                    Vec::new(),
                    Some(winner.candidate_hash.clone()),
                )
            } else {
                if source.required_packages.is_empty() {
                    return Err(Tbf1Error::Invariant(format!(
                        "structural Stage {stage_number} has no live demand"
                    )));
                }
                if source.scored.len() != source.cone_deduped {
                    return Err(Tbf1Error::Invariant(format!(
                        "Stage {stage_number} has an unscored strict candidate"
                    )));
                }
                let candidates = source.scored.iter().map(candidate).collect::<Vec<_>>();
                if candidates
                    .iter()
                    .any(|candidate| !candidate.strict_total_discharger)
                {
                    return Err(Tbf1Error::Invariant(format!(
                        "Stage {stage_number} strict cone contains a non-discharger"
                    )));
                }
                let minimum_pair = candidates
                    .iter()
                    .map(|candidate| ParsimonyPair {
                        kappa: candidate.kappa,
                        certified_nu: candidate.certified_nu,
                    })
                    .min()
                    .ok_or_else(|| {
                        Tbf1Error::Invariant(format!(
                            "Stage {stage_number} strict discharge cone is empty"
                        ))
                    })?;
                let minimizer_hashes = candidates
                    .iter()
                    .filter(|candidate| {
                        candidate.kappa == minimum_pair.kappa
                            && candidate.certified_nu == minimum_pair.certified_nu
                    })
                    .map(|candidate| candidate.candidate_hash.clone())
                    .collect::<Vec<_>>();
                if minimizer_hashes.len() > 1 {
                    first_equal_pair_tie_stage = Some(stage_number);
                    (
                        PrefixJurisdiction::DemandParsimony,
                        PrefixDecision::StoppedFBf1EqualMinimumPair,
                        candidates,
                        Some(minimum_pair),
                        minimizer_hashes,
                        None,
                    )
                } else {
                    let selected_hash = minimizer_hashes.first().cloned();
                    if selected_hash.as_deref() != Some(source.certified_reference_hash.as_str()) {
                        first_reference_divergence_stage = Some(stage_number);
                    }
                    (
                        PrefixJurisdiction::DemandParsimony,
                        PrefixDecision::SelectedUniqueMinimum,
                        candidates,
                        Some(minimum_pair),
                        minimizer_hashes,
                        selected_hash,
                    )
                }
            };

        let selection_matches_reference = selected_hash
            .as_ref()
            .map(|selected| selected == &source.certified_reference_hash);
        let mut record = Tbf1Stage {
            stage: stage_number,
            jurisdiction,
            required_packages: source.required_packages.clone(),
            engine_focus_family: debt_row.focus_family.clone(),
            pre_structural_band: debt_row.pre_structural_band,
            stage3_demand_before_jurisdiction_wrinkle: stage_number == 3
                && stage3_wrinkle_preserved,
            cone_enumerated: source.cone_enumerated,
            strict_cone_admitted: source.cone_admitted,
            canonical_cone_deduped: source.cone_deduped,
            candidates_seen_by_parsimony: candidates,
            minimum_pair,
            minimizer_hashes,
            selected_hash,
            certified_v3_reference_hash_audit_only: source.certified_reference_hash.clone(),
            selection_matches_reference,
            decision,
            derivation_hash: String::new(),
        };
        record.derivation_hash = tagged_hash("stage", &record);
        stages.push(record);

        if first_equal_pair_tie_stage.is_some() || first_reference_divergence_stage.is_some() {
            break;
        }
    }

    let reached = stages.last().map(|stage| stage.stage).unwrap_or(0);
    let unreached_stages = ((reached + 1)..=7).collect::<Vec<_>>();
    let outcome = if let Some(stage) = first_equal_pair_tie_stage {
        let count = stages
            .last()
            .map(|stage| stage.minimizer_hashes.len())
            .unwrap_or(0);
        Tbf1Outcome::StoppedFBf1ParsimonyTie { stage, count }
    } else if let Some(stage) = first_reference_divergence_stage {
        Tbf1Outcome::StoppedReferenceDivergence { stage }
    } else {
        Tbf1Outcome::ProvedThroughStage7
    };
    let theorem_t_bf1_proved = matches!(outcome, Tbf1Outcome::ProvedThroughStage7)
        && stages.len() == 7
        && stages.iter().filter(|stage| stage.stage >= 4).all(|stage| {
            stage.minimizer_hashes.len() == 1 && stage.selection_matches_reference == Some(true)
        });
    let (finding, required_successor_action) = match &outcome {
        Tbf1Outcome::StoppedFBf1ParsimonyTie { stage, count } => (
            format!(
                "F-BF1 fires at Stage {stage}: {count} distinct strict total dischargers share the minimum pair (kappa, certified_nu); the proposed law supplies no selector."
            ),
            "Stop the bar-free adoption. A separate verdict-blind tie adjudication or a stronger independently certified equivalence/nu refinement is required before T-BF1 may be rerun create-new; do not reuse bit-kappa, hash order, the bar, or the recorded winner silently.".to_owned(),
        ),
        Tbf1Outcome::StoppedReferenceDivergence { stage } => (
            format!(
                "The unique parsimony selection diverges from the certified v3 prefix at Stage {stage}."
            ),
            "Publish the divergence and reject demand-parsimony-law-v1 as stated.".to_owned(),
        ),
        Tbf1Outcome::ProvedThroughStage7 => (
            "Stages 1--3 registered constitutively and Stages 4--7 selected uniquely by the frozen parsimony pair.".to_owned(),
            "T-BF1 is available as one prerequisite only; await independent T-BF2 and T-BF3 before any adoption.".to_owned(),
        ),
    };

    let mut certificate = Tbf1PrefixCertificate {
        schema: T_BF1_SCHEMA.to_owned(),
        date: T_BF1_DATE.to_owned(),
        law: T_BF1_LAW.to_owned(),
        source_bindings: source_bindings(),
        proposal_status_remains_proposal: true,
        v2_frozen_program_digest_valid: true,
        v2_frozen_burn_digest_valid: true,
        v2_live_definition_replay_valid,
        v2_live_definition_replay_errors,
        stage4_live_cone_recomputed_without_v2_history_chain: true,
        stage4_live_cone_matches_frozen_certificate,
        stage4_live_nu_matches_certified_nu,
        v3_prefix_binding_digest: v3.result_digest,
        v3_imported_prefix_matches_v2: true,
        selector_order: vec!["least_kappa".to_owned(), "least_certified_nu".to_owned()],
        selector_can_read_diagnostic_bar: false,
        selector_can_read_bit_kappa: false,
        selector_can_read_candidate_hash_or_canonical_key: false,
        selector_can_read_certified_reference: false,
        desired_outcome_used_as_premise: false,
        stages,
        unreached_stages,
        first_equal_pair_tie_stage,
        first_reference_divergence_stage,
        stage3_wrinkle_preserved,
        theorem_t_bf1_proved,
        bar_free_adoption_unlocked_by_t_bf1: theorem_t_bf1_proved,
        outcome,
        finding,
        required_successor_action,
        result_digest: String::new(),
    };
    certificate.result_digest = tagged_hash("certificate", &certificate);
    Ok(certificate)
}

fn digest_valid(certificate: &Tbf1PrefixCertificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == tagged_hash("certificate", &projection)
}

pub fn replay_t_bf1_prefix_certificate(certificate: &Tbf1PrefixCertificate) -> Tbf1Replay {
    let expected = match issue_t_bf1_prefix_certificate() {
        Ok(expected) => expected,
        Err(error) => {
            return Tbf1Replay {
                valid: false,
                theorem_t_bf1_proved: false,
                stopped_stage: None,
                minimizer_count_at_stop: None,
                stage3_wrinkle_preserved: false,
                outcome: "replay_failed".to_owned(),
                errors: vec![error.to_string()],
            };
        }
    };
    let mut errors = Vec::new();
    if certificate != &expected {
        errors.push("certificate differs from independent T-BF1 replay".to_owned());
    }
    if !digest_valid(certificate) {
        errors.push("certificate digest mismatch".to_owned());
    }
    let (stopped_stage, minimizer_count_at_stop, outcome) = match certificate.outcome {
        Tbf1Outcome::StoppedFBf1ParsimonyTie { stage, count } => (
            Some(stage),
            Some(count),
            "stopped_f_bf1_parsimony_tie".to_owned(),
        ),
        Tbf1Outcome::StoppedReferenceDivergence { stage } => {
            (Some(stage), None, "stopped_reference_divergence".to_owned())
        }
        Tbf1Outcome::ProvedThroughStage7 => (None, None, "proved_through_stage7".to_owned()),
    };
    Tbf1Replay {
        valid: errors.is_empty(),
        theorem_t_bf1_proved: certificate.theorem_t_bf1_proved,
        stopped_stage,
        minimizer_count_at_stop,
        stage3_wrinkle_preserved: certificate.stage3_wrinkle_preserved,
        outcome,
        errors,
    }
}

pub fn emit_t_bf1_prefix_create_new(path: &Path) -> Result<Tbf1Replay, Tbf1Error> {
    let certificate = issue_t_bf1_prefix_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Tbf1Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Tbf1Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Tbf1Error::Io(error.to_string()))?;
    let replay = replay_t_bf1_prefix_certificate(&certificate);
    if !replay.valid {
        return Err(Tbf1Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_bf1_honors_stage4_f_bf1_instead_of_importing_a_hidden_tie_break() {
        let certificate = issue_t_bf1_prefix_certificate().expect("T-BF1 certificate");
        assert!(certificate.stage3_wrinkle_preserved);
        assert!(!certificate.theorem_t_bf1_proved);
        assert!(!certificate.bar_free_adoption_unlocked_by_t_bf1);
        assert_eq!(certificate.first_equal_pair_tie_stage, Some(4));
        assert_eq!(certificate.unreached_stages, vec![5, 6, 7]);
        let stage4 = certificate.stages.last().expect("Stage 4");
        assert_eq!(stage4.stage, 4);
        assert_eq!(
            stage4.minimum_pair,
            Some(ParsimonyPair {
                kappa: 3,
                certified_nu: 5,
            })
        );
        assert_eq!(stage4.minimizer_hashes.len(), 4);
        assert!(stage4.selected_hash.is_none());
        assert!(matches!(
            certificate.outcome,
            Tbf1Outcome::StoppedFBf1ParsimonyTie { stage: 4, count: 4 }
        ));
        assert!(!certificate.selector_can_read_diagnostic_bar);
        assert!(!certificate.selector_can_read_bit_kappa);
        assert!(!certificate.selector_can_read_candidate_hash_or_canonical_key);
        assert!(!certificate.selector_can_read_certified_reference);
        assert!(!certificate.desired_outcome_used_as_premise);
        let replay = replay_t_bf1_prefix_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(replay.stopped_stage, Some(4));
        assert_eq!(replay.minimizer_count_at_stop, Some(4));
    }
}
