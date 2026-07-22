//! T1 — Genesis squeeze verification: the verdict layer.
//!
//! The book's halt claim (ch_genesis_mathematics, "Why the Sequence Halts:
//! The Univalent Horizon") and its engine target (`note_canonical_course.md`
//! §7, T1): no Step-16 candidate clears the bar in the current lane.
//!
//! Two cases, verified separately:
//!
//! - **Internal case (univalence's engine face).** Candidates identified
//!   with accepted structure — canonical-presentation duplicates of sealed
//!   entries — contribute ν = 0 *by identification*, not by formula: the
//!   class formulas must never be consulted for them (re-scoring an
//!   accepted entry would produce its original positive ν, which is exactly
//!   the mistake univalence forbids). Trivially derivable presentations
//!   (library/variable re-references, trunc-hybrid re-expressions) score
//!   ν = 0 through the evaluator directly.
//! - **External case.** Every remaining admissible sixteenth extension is
//!   scored by the unchanged evaluator against the 15-step history and must
//!   have ρ < Bar₁₆ = Φ₁₆·Ω₁₅ = (987/610)·(359/64) = 354333/39040 ≈ 9.077.
//!
//! This crate cannot enumerate (pen-search depends on pen-eval). The lane
//! driver lives in `pen-search::halting_probe`: it obtains the external
//! verdict from the engine's own step-16 acceptance stage and uses this
//! module for the bar constants and the internal-class exemplars. The
//! stream-assessment functions below (`assess_step16_stream`,
//! `build_genesis_squeeze_report`) remain available for bounded candidate
//! streams fed explicitly. Honest limitation, disclosed: the engine's
//! identification is canonical presentation (Var-renaming, canonical
//! ordering) — equivalences deeper than presentation are beyond its reach,
//! so the internal case is verified for the identification the lane
//! actually uses.

use crate::bar::{DiscoveryRecord, clears_bar, compute_bar, compute_rho};
use crate::lambda_trigger::rational_string;
use crate::nu::structural_nu;
use pen_core::canonical::{CanonKey, canonical_key_telescope};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::BTreeSet;

pub const GENESIS_HALTING_DATE: &str = "2026-07-05";

/// The frozen Step-16 bar: Φ₁₆·Ω₁₅ = (987/610)·(359/64).
pub fn genesis_bar_16() -> Rational {
    Rational::new(354333, 39040)
}

/// Reference replay through Step 15: the sealed library, the ν history
/// pairs consumed by the evaluator, and the (ν, κ) discovery records
/// consumed by the bar.
pub fn genesis_history() -> (Library, Vec<(u32, u32)>, Vec<DiscoveryRecord>) {
    let mut library = Vec::new();
    let mut pairs = Vec::new();
    let mut records = Vec::new();
    for step in 1..=15 {
        let telescope = Telescope::reference(step);
        let result = structural_nu(&telescope, &library, &pairs);
        let kappa = u32::try_from(telescope.kappa()).expect("reference kappa fits u32");
        library.push(LibraryEntry::from_telescope(&telescope, &library));
        pairs.push((step, result.total));
        records.push(DiscoveryRecord::new(step, result.total, kappa));
    }
    (library, pairs, records)
}

/// Canonical keys of the fifteen sealed entries — the identification set.
pub fn accepted_canonical_keys() -> BTreeSet<CanonKey> {
    (1..=15)
        .map(|step| canonical_key_telescope(&Telescope::reference(step)))
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CandidateAssessment {
    pub clause_kappa: u32,
    pub nu_g: u32,
    pub nu_c: u32,
    pub nu_h: u32,
    pub nu_total: u32,
    pub rho: String,
    pub clears_bar: bool,
    pub telescope: Telescope,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExternalSqueezeReport {
    pub phi_16: String,
    pub omega_15: String,
    pub bar_from_history: String,
    pub bar_frozen_constant: String,
    pub bar_matches_frozen_constant: bool,
    /// Candidates scored by the evaluator (identified duplicates excluded).
    pub candidates_assessed: usize,
    pub clears_bar_count: usize,
    pub max_rho: Option<String>,
    /// The strongest external candidate — the squeeze's closest approach.
    pub max_rho_assessment: Option<CandidateAssessment>,
    /// Every clearing candidate, verbatim (falsifier evidence; empty iff
    /// the squeeze holds).
    pub clearing_candidates: Vec<CandidateAssessment>,
    /// True iff no assessed candidate clears the bar.
    pub squeeze_holds: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InternalExemplar {
    pub description: String,
    pub mechanism: String,
    pub nu_total: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InternalCaseReport {
    /// Stream candidates identified with sealed entries (ν = 0 by
    /// identification; never scored by formula).
    pub identified_in_stream: usize,
    pub exemplars: Vec<InternalExemplar>,
    /// True iff every exemplar scored or was identified to ν = 0.
    pub all_zero: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StreamDiagnostics {
    pub kappa_band_min: u32,
    pub kappa_band_max: u32,
    pub enumerated: usize,
    pub admissibility_rejections: usize,
    pub admitted: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GenesisSqueezeReport {
    pub date: String,
    pub claim: String,
    pub stream: StreamDiagnostics,
    pub external: ExternalSqueezeReport,
    pub internal: InternalCaseReport,
    /// The T1 verdict: external squeeze holds AND the internal class is
    /// uniformly zero.
    pub squeeze_verified: bool,
}

/// Scores the admitted stream against Bar₁₆, applying the univalent
/// identification first: candidates whose canonical presentation matches a
/// sealed entry are internal (ν = 0 by identification) and are not scored.
pub fn assess_step16_stream(
    candidates: &[Telescope],
    library: &Library,
    history_pairs: &[(u32, u32)],
    records: &[DiscoveryRecord],
) -> (ExternalSqueezeReport, usize) {
    let bar = compute_bar(2, 16, records);
    let frozen = genesis_bar_16();
    let accepted_keys = accepted_canonical_keys();

    let mut assessed = 0;
    let mut identified = 0;
    let mut clearing = Vec::new();
    let mut max_rho: Option<(Rational, CandidateAssessment)> = None;

    for candidate in candidates {
        if accepted_keys.contains(&canonical_key_telescope(candidate)) {
            identified += 1;
            continue;
        }
        let kappa = u32::try_from(candidate.kappa()).expect("candidate kappa fits u32");
        let result = structural_nu(candidate, library, history_pairs);
        let Some(rho) = compute_rho(result.total, kappa) else {
            continue; // κ = 0: not a candidate
        };
        assessed += 1;
        let assessment = CandidateAssessment {
            clause_kappa: kappa,
            nu_g: result.nu_g,
            nu_c: result.nu_c,
            nu_h: result.nu_h,
            nu_total: result.total,
            rho: rational_string(rho),
            clears_bar: clears_bar(rho, bar.bar),
            telescope: candidate.clone(),
        };
        if assessment.clears_bar {
            clearing.push(assessment.clone());
        }
        let is_new_max = match &max_rho {
            None => true,
            Some((current, _)) => rho > *current,
        };
        if is_new_max {
            max_rho = Some((rho, assessment));
        }
    }

    let squeeze_holds = clearing.is_empty();
    (
        ExternalSqueezeReport {
            phi_16: rational_string(bar.phi),
            omega_15: rational_string(bar.omega),
            bar_from_history: rational_string(bar.bar),
            bar_frozen_constant: rational_string(frozen),
            bar_matches_frozen_constant: bar.bar == frozen,
            candidates_assessed: assessed,
            clears_bar_count: clearing.len(),
            max_rho: max_rho.as_ref().map(|(rho, _)| rational_string(*rho)),
            max_rho_assessment: max_rho.map(|(_, assessment)| assessment),
            clearing_candidates: clearing,
            squeeze_holds,
        },
        identified,
    )
}

/// Constructed internal-class exemplars: presentations of already-accepted
/// structure that the engine must not charge.
pub fn internal_case_exemplars(
    library: &Library,
    history_pairs: &[(u32, u32)],
) -> Vec<InternalExemplar> {
    let accepted_keys = accepted_canonical_keys();
    let mut exemplars = Vec::new();

    // (1) Canonical re-proposals of sealed entries: caught by identification.
    for step in [13, 14, 15] {
        let reproposal = Telescope::reference(step);
        let identified = accepted_keys.contains(&canonical_key_telescope(&reproposal));
        exemplars.push(InternalExemplar {
            description: format!("verbatim re-proposal of the Step-{step} entry"),
            mechanism: "canonical identification (univalence's engine face)".to_owned(),
            nu_total: if identified {
                0
            } else {
                // Identification failed: the formula score stands as the
                // exemplar's ν, and all_zero will report the failure.
                structural_nu(&reproposal, library, history_pairs).total
            },
        });
    }

    // (2) Library re-references: reachable by pure pointer, ν = 0 through
    // the evaluator (trivially derivable).
    let re_reference = Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
        ClauseRec::new(ClauseRole::Formation, Expr::Lib(14)),
        ClauseRec::new(ClauseRole::Formation, Expr::Lib(13)),
    ]);
    exemplars.push(InternalExemplar {
        description: "re-reference of the sealed window (Lib 13/14/15)".to_owned(),
        mechanism: "trivially derivable (P-internal: already there)".to_owned(),
        nu_total: structural_nu(&re_reference, library, history_pairs).total,
    });

    // (3) Bare variable presentation: no new structure at all.
    let bare = Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, Expr::Var(1))]);
    exemplars.push(InternalExemplar {
        description: "bare variable presentation".to_owned(),
        mechanism: "trivially derivable".to_owned(),
        nu_total: structural_nu(&bare, library, history_pairs).total,
    });

    // (4) Truncation-context re-expression of a higher path — the
    // trunc-hybrid internal form the evaluator already recognizes.
    let hybrid = Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Trunc(Box::new(Expr::Var(1)))),
        ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(2)),
    ]);
    exemplars.push(InternalExemplar {
        description: "truncation-context re-expression of a higher path".to_owned(),
        mechanism: "trivially derivable (trunc-hybrid internal form)".to_owned(),
        nu_total: structural_nu(&hybrid, library, history_pairs).total,
    });

    exemplars
}

/// Assembles the full T1 report from the admitted stream and its
/// enumeration diagnostics.
pub fn build_genesis_squeeze_report(
    admitted: &[Telescope],
    library: &Library,
    history_pairs: &[(u32, u32)],
    records: &[DiscoveryRecord],
    kappa_band: (u32, u32),
    enumerated: usize,
    admissibility_rejections: usize,
) -> GenesisSqueezeReport {
    let (external, identified_in_stream) =
        assess_step16_stream(admitted, library, history_pairs, records);
    let exemplars = internal_case_exemplars(library, history_pairs);
    let all_zero = exemplars.iter().all(|exemplar| exemplar.nu_total == 0);
    let internal = InternalCaseReport {
        identified_in_stream,
        exemplars,
        all_zero,
    };
    let squeeze_verified =
        external.squeeze_holds && external.bar_matches_frozen_constant && internal.all_zero;

    GenesisSqueezeReport {
        date: GENESIS_HALTING_DATE.to_owned(),
        claim: "no Step-16 candidate clears the bar in the current lane \
                (ch_genesis_mathematics, the Step-16 squeeze; T1 of \
                note_canonical_course.md)"
            .to_owned(),
        stream: StreamDiagnostics {
            kappa_band_min: kappa_band.0,
            kappa_band_max: kappa_band.1,
            enumerated,
            admissibility_rejections,
            admitted: admitted.len(),
        },
        external,
        internal,
        squeeze_verified,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        accepted_canonical_keys, genesis_bar_16, genesis_history, internal_case_exemplars,
    };
    use crate::bar::compute_bar;
    use pen_core::rational::Rational;

    #[test]
    fn replayed_history_reproduces_the_frozen_bar_constant() {
        let (_, _, records) = genesis_history();
        let bar = compute_bar(2, 16, &records);
        assert_eq!(bar.phi, Rational::new(987, 610));
        assert_eq!(bar.omega, Rational::new(359, 64));
        assert_eq!(bar.bar, genesis_bar_16());
    }

    #[test]
    fn internal_class_scores_zero() {
        let (library, pairs, _) = genesis_history();
        for exemplar in internal_case_exemplars(&library, &pairs) {
            assert_eq!(
                exemplar.nu_total, 0,
                "internal exemplar must be free: {} ({})",
                exemplar.description, exemplar.mechanism
            );
        }
    }

    #[test]
    fn identification_set_covers_the_fifteen_sealed_entries() {
        assert_eq!(accepted_canonical_keys().len(), 15);
    }
}
