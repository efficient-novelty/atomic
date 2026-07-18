//! Phase 5b of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`: the full
//! sequential semantic reselection (the preregistered BURNED RUN — the
//! first full run's numbers count, whatever they are).
//!
//! Starting from the initial state, for every stage 1..=15:
//! 1. enumerate the COMPLETE admissible candidate cone under the shipped
//!    structural admissibility derived from the REVISED prefix, and
//!    commit its digest;
//! 2. for every candidate, run the kernel pipeline against the revised
//!    prefix — typed family extraction (Phase 2), the stage's demand
//!    orbits over the revised timeline (Phase 3), and the EGP classifier
//!    with dependent anchors — and take the candidate's EXACT revised
//!    score to be its certified marginal family count
//!    (`ConditionalAtMostBound::marginal_nu`): P5/P6/d²/r²/synthesis
//!    contributions survive exactly to the extent their families occupy
//!    independent anchors. A kernel-invalid candidate or a classifier
//!    rejection scores zero, fail closed — no unguarded credit;
//! 3. recompute the exact order with the ENGINE'S OWN acceptance rank
//!    (minimal positive overshoot against the revised bar, then the
//!    frozen structural tie-breakers) applied to the revised scores;
//!    candidates canonically identified with revised accepted entries
//!    score zero by identification;
//! 4. accept the winner, update the revised history and demand ledger,
//!    and derive the next bar from the revised prefix
//!    (`compute_bar`, the frozen bar law, over revised records).
//!
//! The legacy vector 1,1,2,5,7,8,10,18,17,19,26,34,46,62,103 is a
//! regression INPUT: every stage records whether the winner, score, and
//! bar agree with the sealed structural trace, and the first divergence
//! is a data field. Both branches of the decision gate are lawful
//! outcomes; neither is repaired.

use crate::accept::acceptance_rank_for_telescope;
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use pen_core::canonical::canonical_key_telescope;
use pen_core::encode::telescope_bit_cost;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_eval::debt_guard::required_packages_for;
use pen_eval::demand_orbits::{KernelOrbitExtraction, stage_inventories_for_timeline};
use pen_eval::egp::classify_candidate;
use pen_eval::typed_families::{
    CandidateExtractionOutcome, extract_candidate_families, predecessor_closure,
};
use pen_type::admissibility::{AdmissibilityMode, passes_strict_admissibility,
    strict_admissibility_for_mode};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use pen_type::obligations::summarize_structural_debt;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

const WINDOW_DEPTH: u16 = 2;
const RESELECTION_DATE: &str = "2026-07-18";

fn rational_string(value: Rational) -> String {
    format!("{value}")
}

/// One scored cone candidate, published in full (the guarded cones are
/// small; the largest sealed-era cone is a few hundred candidates).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ScoredCandidate {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub kappa: u16,
    pub identified_with_accepted: bool,
    pub kernel_invalid: Option<String>,
    pub classifier_rejection: Option<String>,
    pub marginal_families: u32,
    pub revised_nu: u32,
    pub rho: String,
    pub clears_bar: bool,
    pub overshoot: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WinnerRecord {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub kappa: u16,
    pub revised_nu: u32,
    pub rho: String,
    pub overshoot: String,
    pub telescope: Telescope,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StageLedgerSummary {
    pub required_packages: Vec<String>,
    pub live_orbits: usize,
    pub answered_orbits: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StageReselection {
    pub stage: u32,
    pub bar: String,
    pub cone_enumerated: usize,
    pub cone_admitted: usize,
    pub cone_deduped: usize,
    pub cone_digest: String,
    pub identified: usize,
    pub clearing: usize,
    pub scored: Vec<ScoredCandidate>,
    /// `None` exactly when no candidate cleared the revised bar — the
    /// halted stage is published in full, not swallowed as an error.
    pub winner: Option<WinnerRecord>,
    pub ledger: StageLedgerSummary,
    pub winner_matches_legacy: bool,
    pub score_matches_legacy: bool,
    pub bar_matches_legacy: bool,
    pub legacy_nu: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FirstDivergence {
    pub stage: u32,
    pub field: String,
    pub legacy: String,
    pub revised: String,
}

/// How the burned run ended: the full fifteen stages, or an early halt
/// (no candidate cleared the revised bar) — the latter is the certified
/// independence-flavored outcome, published with the complete revised
/// prefix and the halted stage's whole cone.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReselectionOutcome {
    CompletedThroughStage15,
    HaltedNoClearingCandidate { stage: u32, bar: String },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticReselectionCertificate {
    pub schema_version: u32,
    pub date: String,
    pub burned_run: bool,
    pub outcome: ReselectionOutcome,
    pub stages: Vec<StageReselection>,
    pub revised_history: Vec<(u32, u32, u32)>,
    pub legacy_history: Vec<(u32, u32, u32)>,
    pub semantic_reenactment: bool,
    pub winners_reenact: bool,
    pub first_divergence: Option<FirstDivergence>,
    /// `None` when the run halted before stage 15: there is no revised
    /// fifteen-stage history to derive a Bar16 from, and the legacy
    /// Bar16 may NOT be reused (program rule).
    pub revised_bar_16: Option<String>,
    pub legacy_bar_16: String,
    pub bar_16_matches: bool,
    /// The two lawful continuations of the decision gate (program §5b);
    /// the CHOICE between them is Two-Law bridge content for the book
    /// ledger, deliberately NOT made here.
    pub lawful_continuations: [String; 2],
    pub digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ReselectionError {
    #[error("stage {stage}: no candidate cleared the revised bar {bar}")]
    NoClearingCandidate { stage: u32, bar: String },
    #[error("stage {stage}: cone is empty under revised admissibility")]
    EmptyCone { stage: u32 },
    #[error("stage {stage}: orbit extraction failed: {detail}")]
    OrbitExtraction { stage: u32, detail: String },
    #[error("stage {stage}: predecessor closure failed: {detail}")]
    Closure { stage: u32, detail: String },
}

fn cone_digest(candidates: &[Telescope]) -> String {
    let keys: Vec<String> = candidates
        .iter()
        .map(|telescope| candidate_hash(telescope))
        .collect();
    let payload = serde_json::json!({ "count": keys.len(), "candidate_hashes": keys });
    format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&payload).expect("cone digest serialization"))
    )
}

fn revised_orbits_for_prefix(
    stage: u32,
    signature: &SealedSignature,
    timeline: &[(u32, Vec<String>)],
) -> Result<KernelOrbitExtraction, ReselectionError> {
    let closure = predecessor_closure(signature).map_err(|error| ReselectionError::Closure {
        stage,
        detail: format!("{error:?}"),
    })?;
    stage_inventories_for_timeline(signature, &closure, timeline).map_err(|error| {
        ReselectionError::OrbitExtraction {
            stage,
            detail: error.to_string(),
        }
    })
}

/// Run the full burned reselection through stage 15.
pub fn run_semantic_reselection() -> Result<SemanticReselectionCertificate, ReselectionError> {
    let mut structural_library: Library = Vec::new();
    let mut revised_winners: Vec<(u32, Telescope)> = Vec::new();
    let mut revised_records: Vec<DiscoveryRecord> = Vec::new();
    let mut accepted_keys: BTreeSet<String> = BTreeSet::new();
    let mut timeline: Vec<(u32, Vec<String>)> = Vec::new();
    let mut stages = Vec::new();
    let mut first_divergence: Option<FirstDivergence> = None;
    let mut halted_outcome: Option<ReselectionOutcome> = None;

    let legacy_history: Vec<(u32, u32, u32)> = {
        let mut library: Library = Vec::new();
        let mut history: Vec<(u32, u32)> = Vec::new();
        let mut rows = Vec::new();
        for step in 1..=15u32 {
            let telescope = Telescope::reference(step);
            let result = pen_eval::nu::structural_nu(&telescope, &library, &history);
            rows.push((step, result.total, telescope.kappa() as u32));
            history.push((step, result.total));
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
        rows
    };

    for stage in 1..=15u32 {
        // Demand timeline entry for this stage (library sealed through
        // stage - 1, REVISED prefix).
        let debt = summarize_structural_debt(&structural_library, WINDOW_DEPTH);
        let required: Vec<String> = required_packages_for(debt)
            .iter()
            .map(|package| (*package).to_string())
            .collect();
        timeline.push((stage, required.clone()));

        // The revised prefix as a sealed-signature value (empty at
        // stage 1) plus its closure and stage orbits.
        let signature = SealedSignature::from_telescopes(revised_winners.clone());
        let closure =
            predecessor_closure(&signature).map_err(|error| ReselectionError::Closure {
                stage,
                detail: format!("{error:?}"),
            })?;
        let orbits = revised_orbits_for_prefix(stage, &signature, &timeline)?;
        let ledger = {
            let inventory = orbits.stage(stage);
            StageLedgerSummary {
                required_packages: required.clone(),
                live_orbits: orbits.live_orbit_count(stage),
                answered_orbits: inventory
                    .map(|inventory| {
                        inventory
                            .orbits
                            .iter()
                            .filter(|orbit| {
                                matches!(
                                    orbit.resolution,
                                    pen_eval::semantic_provenance::OrbitResolution::Answered { .. }
                                )
                            })
                            .count()
                    })
                    .unwrap_or(0),
            }
        };

        // The complete admissible cone under the revised prefix.
        let admissibility = strict_admissibility_for_mode(
            stage,
            WINDOW_DEPTH,
            &structural_library,
            AdmissibilityMode::Guarded,
        );
        let context = EnumerationContext::from_admissibility(&structural_library, admissibility);
        let mut enumerated: Vec<Telescope> = Vec::new();
        for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            enumerated.extend(enumerate_telescopes(&structural_library, context, kappa));
        }
        let cone_enumerated = enumerated.len();

        let admitted: Vec<Telescope> = enumerated
            .into_iter()
            .filter(|telescope| {
                passes_strict_admissibility(stage, &structural_library, telescope, admissibility)
            })
            .collect();
        let cone_admitted = admitted.len();

        // Deterministic canonical dedupe (keep the first presenter in
        // the enumerator's sorted order).
        let mut seen = BTreeSet::new();
        let mut cone: Vec<Telescope> = Vec::new();
        for telescope in admitted {
            let key = canonical_key_telescope(&telescope).0;
            if seen.insert(key) {
                cone.push(telescope);
            }
        }
        let cone_deduped = cone.len();
        if cone.is_empty() {
            return Err(ReselectionError::EmptyCone { stage });
        }
        let digest = cone_digest(&cone);

        // Revised bar from the revised prefix.
        let bar = compute_bar(usize::from(WINDOW_DEPTH), stage, &revised_records).bar;

        // Score every cone candidate through the kernel pipeline.
        let visible_library = revised_winners.len() as u32;
        let mut scored = Vec::with_capacity(cone.len());
        let mut identified_count = 0usize;
        let mut ranked: Vec<(crate::branch_bound::AcceptRank, usize)> = Vec::new();
        for (index, telescope) in cone.iter().enumerate() {
            let kappa = telescope.kappa() as u16;
            let canonical_key = canonical_key_telescope(telescope).0;
            let identified = accepted_keys.contains(&canonical_key);
            let mut kernel_invalid = None;
            let mut classifier_rejection = None;
            let mut marginal_families = 0u32;
            let revised_nu: u32;
            if identified {
                identified_count += 1;
                revised_nu = 0;
            } else {
                match extract_candidate_families(&signature, &closure, telescope, visible_library)
                {
                    CandidateExtractionOutcome::Extracted(extraction) => {
                        match classify_candidate(&extraction, &orbits, stage, kappa) {
                            Ok(disposition) => {
                                marginal_families = disposition.marginal_family_count as u32;
                                revised_nu = disposition.bound.marginal_nu;
                            }
                            Err(error) => {
                                classifier_rejection = Some(error.to_string());
                                revised_nu = 0;
                            }
                        }
                    }
                    CandidateExtractionOutcome::KernelInvalid { failure } => {
                        kernel_invalid = Some(failure.to_string());
                        revised_nu = 0;
                    }
                }
            }
            let rho = Rational::new(i64::from(revised_nu), i64::from(kappa.max(1)));
            let clears = !identified && rho >= bar;
            if clears {
                let bit_kappa = u16::try_from(telescope_bit_cost(telescope))
                    .expect("bit cost fits u16");
                if let Some(rank) = acceptance_rank_for_telescope(
                    bar,
                    telescope,
                    u16::try_from(revised_nu).expect("revised nu fits u16"),
                    bit_kappa,
                    kappa,
                ) {
                    ranked.push((rank, index));
                }
            }
            scored.push(ScoredCandidate {
                candidate_hash: candidate_hash(telescope),
                canonical_key,
                kappa,
                identified_with_accepted: identified,
                kernel_invalid,
                classifier_rejection,
                marginal_families,
                revised_nu,
                rho: rational_string(rho),
                clears_bar: clears,
                overshoot: clears.then(|| rational_string(rho - bar)),
            });
        }

        let clearing = ranked.len();
        ranked.sort_by(|(left, _), (right, _)| left.cmp(right));
        let halted = ranked.is_empty();
        let winner = ranked.first().cloned().map(|(winner_rank, winner_index)| {
            let winner_score = &scored[winner_index];
            WinnerRecord {
                candidate_hash: winner_score.candidate_hash.clone(),
                canonical_key: winner_score.canonical_key.clone(),
                kappa: winner_score.kappa,
                revised_nu: winner_score.revised_nu,
                rho: winner_score.rho.clone(),
                overshoot: rational_string(winner_rank.overshoot),
                telescope: cone[winner_index].clone(),
            }
        });

        // Legacy comparison for this stage.
        let legacy_telescope = Telescope::reference(stage);
        let legacy_nu = legacy_history[stage as usize - 1].1;
        let winner_matches_legacy = winner
            .as_ref()
            .is_some_and(|winner| winner.telescope == legacy_telescope);
        let score_matches_legacy = winner
            .as_ref()
            .is_some_and(|winner| winner.revised_nu == legacy_nu);
        let legacy_bar = {
            let legacy_records: Vec<DiscoveryRecord> = legacy_history
                .iter()
                .take(stage as usize - 1)
                .map(|(step, nu, kappa)| DiscoveryRecord::new(*step, *nu, *kappa))
                .collect();
            compute_bar(usize::from(WINDOW_DEPTH), stage, &legacy_records).bar
        };
        let bar_matches_legacy = bar == legacy_bar;
        if first_divergence.is_none() {
            if !bar_matches_legacy {
                first_divergence = Some(FirstDivergence {
                    stage,
                    field: "bar".to_string(),
                    legacy: rational_string(legacy_bar),
                    revised: rational_string(bar),
                });
            } else if halted {
                first_divergence = Some(FirstDivergence {
                    stage,
                    field: "no_clearing_candidate".to_string(),
                    legacy: format!(
                        "legacy winner {} accepted with nu {legacy_nu}",
                        candidate_hash(&legacy_telescope)
                    ),
                    revised: format!(
                        "no cone candidate clears the revised bar {}",
                        rational_string(bar)
                    ),
                });
            } else if !winner_matches_legacy {
                first_divergence = Some(FirstDivergence {
                    stage,
                    field: "winner".to_string(),
                    legacy: candidate_hash(&legacy_telescope),
                    revised: winner
                        .as_ref()
                        .map(|winner| winner.candidate_hash.clone())
                        .unwrap_or_default(),
                });
            } else if !score_matches_legacy {
                first_divergence = Some(FirstDivergence {
                    stage,
                    field: "score".to_string(),
                    legacy: legacy_nu.to_string(),
                    revised: winner
                        .as_ref()
                        .map(|winner| winner.revised_nu.to_string())
                        .unwrap_or_default(),
                });
            }
        }

        stages.push(StageReselection {
            stage,
            bar: rational_string(bar),
            cone_enumerated,
            cone_admitted,
            cone_deduped,
            cone_digest: digest,
            identified: identified_count,
            clearing,
            scored,
            winner: winner.clone(),
            ledger,
            winner_matches_legacy,
            score_matches_legacy,
            bar_matches_legacy,
            legacy_nu,
        });

        // Accept the winner, or STOP: a stage with no clearing candidate
        // ends the run, published in full, never repaired.
        let Some(winner) = winner else {
            halted_outcome = Some(ReselectionOutcome::HaltedNoClearingCandidate {
                stage,
                bar: rational_string(bar),
            });
            break;
        };
        accepted_keys.insert(winner.canonical_key.clone());
        revised_records.push(DiscoveryRecord::new(
            stage,
            winner.revised_nu,
            u32::from(winner.kappa),
        ));
        revised_winners.push((stage, winner.telescope.clone()));
        structural_library.push(LibraryEntry::from_telescope(
            &winner.telescope,
            &structural_library,
        ));
    }

    let completed = halted_outcome.is_none();
    let outcome = halted_outcome.unwrap_or(ReselectionOutcome::CompletedThroughStage15);
    // A revised Bar16 exists only for a completed fifteen-stage history;
    // the legacy Bar16 may not be reused in its place (program rule).
    let revised_bar_16 = if completed {
        Some(rational_string(
            compute_bar(usize::from(WINDOW_DEPTH), 16, &revised_records).bar,
        ))
    } else {
        None
    };
    let legacy_bar_16 = Rational::new(354_333, 39_040);
    let revised_history: Vec<(u32, u32, u32)> = revised_records
        .iter()
        .map(|record| (record.step_index, record.nu, record.kappa))
        .collect();
    let winners_reenact =
        completed && stages.iter().all(|stage| stage.winner_matches_legacy);
    let semantic_reenactment = winners_reenact
        && stages
            .iter()
            .all(|stage| stage.score_matches_legacy && stage.bar_matches_legacy);

    let mut certificate = SemanticReselectionCertificate {
        schema_version: 1,
        date: RESELECTION_DATE.to_string(),
        burned_run: true,
        outcome,
        stages,
        revised_history,
        legacy_history,
        semantic_reenactment,
        winners_reenact,
        first_divergence,
        bar_16_matches: revised_bar_16.as_deref()
            == Some(rational_string(legacy_bar_16).as_str()),
        revised_bar_16,
        legacy_bar_16: rational_string(legacy_bar_16),
        lawful_continuations: [
            "declare the shipped structural audit a constitutive scoring bridge with a \
             separately justified semantic halt gate"
                .to_string(),
            "adopt the semantic audit and continue re-deriving its history".to_string(),
        ],
        digest: String::new(),
    };
    let digest_payload =
        serde_json::to_vec(&certificate).expect("certificate serialization for digest");
    certificate.digest = format!("blake3:{}", blake3_hex(&digest_payload));
    Ok(certificate)
}

/// Replay grader: an independent rerun must reconstruct the certificate
/// byte for byte (all orders and ledger transitions included).
pub fn replay_semantic_reselection(
    certificate: &SemanticReselectionCertificate,
) -> Result<(), String> {
    let fresh = run_semantic_reselection().map_err(|error| error.to_string())?;
    if &fresh == certificate {
        Ok(())
    } else {
        Err("reselection replay diverged from the presented certificate".to_string())
    }
}

/// Convenience map for Phase 6: revised nu per stage.
pub fn revised_scores(certificate: &SemanticReselectionCertificate) -> BTreeMap<u32, u32> {
    certificate
        .revised_history
        .iter()
        .map(|(step, nu, _)| (*step, *nu))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The Phase 5b grader: the burned run replays deterministically and
    /// its certified outcome is pinned. The first full run's numbers
    /// count (program §5b); this test freezes them as regression data.
    #[test]
    fn burned_run_replays_and_pins_the_certified_outcome() {
        let certificate = run_semantic_reselection().expect("the burned run publishes");

        // The certified outcome: the semantic reselection HALTS at
        // stage 2 — the guarded bootstrap cone admits exactly one
        // candidate (the Unit universe-application), whose sole typed
        // family is internal-identical to the stage-1 winner's family:
        // zero marginal novelty, below the bar of 1/2.
        assert_eq!(
            certificate.outcome,
            ReselectionOutcome::HaltedNoClearingCandidate {
                stage: 2,
                bar: "1/2".to_string(),
            }
        );
        assert_eq!(certificate.stages.len(), 2);

        // Stage 1: the legacy winner re-enacts, but the strengthened law
        // scores it 2 (both families marginal over the empty closure)
        // where the sealed trace recorded 1 — the first divergence.
        let stage_one = &certificate.stages[0];
        assert!(stage_one.winner_matches_legacy);
        assert!(!stage_one.score_matches_legacy);
        assert_eq!(stage_one.winner.as_ref().expect("winner").revised_nu, 2);
        let divergence = certificate.first_divergence.as_ref().expect("divergence");
        assert_eq!(divergence.stage, 1);
        assert_eq!(divergence.field, "score");
        assert_eq!(divergence.legacy, "1");
        assert_eq!(divergence.revised, "2");

        // Stage 2: full cone published, nothing clears, nothing wins.
        let stage_two = &certificate.stages[1];
        assert_eq!(stage_two.clearing, 0);
        assert!(stage_two.winner.is_none());
        assert_eq!(stage_two.cone_deduped, 1);
        assert!(stage_two.scored.iter().all(|candidate| !candidate.clears_bar));

        // No reenactment, no revised Bar16, and the legacy Bar16 is not
        // reused in its place.
        assert!(!certificate.semantic_reenactment);
        assert!(!certificate.winners_reenact);
        assert!(certificate.revised_bar_16.is_none());
        assert!(!certificate.bar_16_matches);

        // Independent replay reconstructs the run byte for byte.
        replay_semantic_reselection(&certificate).expect("replay");
    }
}
