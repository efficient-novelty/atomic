//! The O(16)-emptiness check — the engine verification of the Guard-Rail
//! Theorem's central step (`book/debt_guard_theorem.tex`; recorded in
//! `note_canonical_course.md` and the claim ledger's Univalent Horizon
//! entry; named "cheapest pending engine item" by ch_open_problems Problem
//! Two).
//!
//! The framework carries two debts the halt argument conflated:
//!
//! - **Quantitative debt** Δ_n = F_n — prices, never vanishes.
//! - **Directive debt** O(n) — guards: the set of structural-package
//!   demands the sealed window imposes on stage n's candidate field. In
//!   engine terms, O(n) is the set of `requires_*_package()` demands of
//!   `summarize_structural_debt` over the library sealed through step n−1
//!   (window depth 2), the same object from which the lane derives its
//!   focus family.
//!
//! Claims verified here, exactly as registered:
//!
//! 1. **O(16) = ∅** (Guard-Rail Thm 2): after Step 15 — the free, law-like
//!    guarded modal completion — no package demand stands. The open band at
//!    step 16 is forced by the framework, not an engine artifact.
//! 2. **Steps 4–15 are guarded**: every structural-era selection stage had
//!    at least one standing demand (its guard rail). Stages 1–3 are
//!    pre-structural: the lane's admissibility for them is a hardcoded band
//!    that never consults the debt — recorded, not asserted.
//! 3. **Discharge persistence** (Thm 1's engine face): each package's
//!    demand-interval is contiguous — once discharged it never reappears
//!    within the sequence.
//!
//! The check replays the fifteen reference telescopes and inspects the
//! debt; it computes nothing about candidates, ν, or bars, and is
//! independent of the T1 adjudication (its result stands under Reading A
//! and Reading B alike).
//!
//! **Granularity boundary.** This module proves only the engine's coarse
//! package-label claim.  It does not enumerate semantic demand orbits and
//! therefore cannot by itself close J2/J3.  Use
//! [`crate::semantic_provenance::audit_semantic_debt`] for the stricter
//! orbit-level audit; that audit deliberately reports semantic O(16)
//! emptiness as unknown in the built-in replay. User-supplied orbit and
//! derivability assumptions produce only a conditional result until a typed
//! verifier can mint opaque certificates.

use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_type::admissibility::{AdmissibilityMode, strict_admissibility_for_mode};
use pen_type::obligations::{StructuralDebt, summarize_structural_debt};
use serde::Serialize;

pub const O16_CHECK_DATE: &str = "2026-07-06";

/// The lane's window depth (Appendix D: the two-layer chronological window).
pub const WINDOW_DEPTH: u16 = 2;

/// The twelve structural-package demands the debt can impose, in the
/// engine's own priority order.
pub const PACKAGE_NAMES: [&str; 12] = [
    "former_eliminator",
    "initial_hit",
    "truncation_hit",
    "higher_hit",
    "sphere_lift",
    "axiomatic_bundle",
    "modal_shell",
    "connection_shell",
    "curvature_shell",
    "operator_bundle",
    "hilbert_functional",
    "temporal_shell",
];

/// Public face of the package-demand mapping, for the Phase 5b
/// reselection (which must derive the demand timeline of a REVISED
/// library prefix rather than the frozen reference one).
pub fn required_packages_for(debt: StructuralDebt) -> Vec<&'static str> {
    required_packages(debt)
}

fn required_packages(debt: StructuralDebt) -> Vec<&'static str> {
    let flags = [
        debt.requires_former_eliminator_package(),
        debt.requires_initial_hit_package(),
        debt.requires_truncation_hit_package(),
        debt.requires_higher_hit_package(),
        debt.requires_sphere_lift_package(),
        debt.requires_axiomatic_bundle_package(),
        debt.requires_modal_shell_package(),
        debt.requires_connection_shell_package(),
        debt.requires_curvature_shell_package(),
        debt.requires_operator_bundle_package(),
        debt.requires_hilbert_functional_package(),
        debt.requires_temporal_shell_package(),
    ];
    PACKAGE_NAMES
        .iter()
        .zip(flags)
        .filter_map(|(name, required)| required.then_some(*name))
        .collect()
}

/// The directive debt standing at one selection stage.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DirectiveDebtRecord {
    /// The stage k being selected (library sealed through k−1).
    pub stage: u32,
    /// O(k): the standing package demands.
    pub required_packages: Vec<&'static str>,
    /// |O(k)| = 0.
    pub debt_free: bool,
    /// The focus family the lane's admissibility derives at this stage
    /// (the guard rail as the candidate field experiences it).
    pub focus_family: String,
    /// Stages 1–3 use hardcoded pre-structural bands; the debt is computed
    /// and recorded for them but the lane never consults it.
    pub pre_structural_band: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct O16EmptinessReport {
    pub date: String,
    pub claim: String,
    pub window_depth: u16,
    pub timeline: Vec<DirectiveDebtRecord>,
    /// Thm 2's engine face: O(16) = ∅.
    pub o16_empty: bool,
    /// Structural-era stages (4..=15) with a standing demand.
    pub guarded_stages: Vec<u32>,
    /// Structural-era stages (4..=15) with NO standing demand — the
    /// theorem expects none; any entry here is a finding.
    pub unguarded_structural_stages: Vec<u32>,
    /// The first debt-free stage in the structural era (4..=16).
    pub first_debt_free_stage: Option<u32>,
    /// Thm 1's engine face: every package's demand-interval is contiguous
    /// (once discharged, never reappears).
    pub discharge_persistence_holds: bool,
    /// Packages whose demand-interval is NOT contiguous (findings).
    pub persistence_violations: Vec<&'static str>,
    /// The verified package-label conjunction: O(16) empty, 4..=15 all
    /// guarded, 16 the first debt-free structural stage, persistence holds.
    /// This is not an individual-orbit J2/J3 completeness result.
    pub guard_rail_verified: bool,
}

/// Computes the directive-debt timeline for stages 1..=16 by replaying the
/// reference sequence.
pub fn directive_debt_timeline() -> Vec<DirectiveDebtRecord> {
    let mut library: Library = Vec::new();
    let mut timeline = Vec::with_capacity(16);

    for stage in 1..=16_u32 {
        // Library sealed through stage − 1.
        let debt = summarize_structural_debt(&library, WINDOW_DEPTH);
        let admissibility = strict_admissibility_for_mode(
            stage,
            WINDOW_DEPTH,
            &library,
            AdmissibilityMode::Guarded,
        );
        let required = required_packages(debt);
        timeline.push(DirectiveDebtRecord {
            stage,
            debt_free: required.is_empty(),
            required_packages: required,
            focus_family: format!("{:?}", admissibility.focus_family),
            pre_structural_band: stage <= 3,
        });

        if stage <= 15 {
            let telescope = Telescope::reference(stage);
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
    }

    timeline
}

/// Runs the full O(16)-emptiness check.
pub fn o16_emptiness_report() -> O16EmptinessReport {
    let timeline = directive_debt_timeline();

    let o16_empty = timeline
        .iter()
        .find(|record| record.stage == 16)
        .map(|record| record.debt_free)
        .unwrap_or(false);

    let guarded_stages: Vec<u32> = timeline
        .iter()
        .filter(|record| (4..=15).contains(&record.stage) && !record.debt_free)
        .map(|record| record.stage)
        .collect();
    let unguarded_structural_stages: Vec<u32> = timeline
        .iter()
        .filter(|record| (4..=15).contains(&record.stage) && record.debt_free)
        .map(|record| record.stage)
        .collect();
    let first_debt_free_stage = timeline
        .iter()
        .find(|record| record.stage >= 4 && record.debt_free)
        .map(|record| record.stage);

    // Persistence: for each package, the set of stages demanding it must be
    // one contiguous interval.
    let mut persistence_violations = Vec::new();
    for package in PACKAGE_NAMES {
        let demanded: Vec<u32> = timeline
            .iter()
            .filter(|record| record.required_packages.contains(&package))
            .map(|record| record.stage)
            .collect();
        let contiguous = demanded.windows(2).all(|window| window[1] == window[0] + 1);
        if !contiguous {
            persistence_violations.push(package);
        }
    }
    let discharge_persistence_holds = persistence_violations.is_empty();

    let guard_rail_verified = o16_empty
        && unguarded_structural_stages.is_empty()
        && first_debt_free_stage == Some(16)
        && discharge_persistence_holds;

    O16EmptinessReport {
        date: O16_CHECK_DATE.to_owned(),
        claim: "Guard-Rail Thm 2, engine face: O(16) = ∅ — after Step 15's \
                law-like completion no directive demand stands; stages 4–15 \
                were each guarded; 16 is the sequence's first debt-free \
                stage (debt_guard_theorem.tex; ch_open_problems Problem Two)"
            .to_owned(),
        window_depth: WINDOW_DEPTH,
        timeline,
        o16_empty,
        guarded_stages,
        unguarded_structural_stages,
        first_debt_free_stage,
        discharge_persistence_holds,
        persistence_violations,
        guard_rail_verified,
    }
}

#[cfg(test)]
mod tests {
    use super::o16_emptiness_report;

    /// The O(16)-emptiness check (ch_open_problems Problem Two, first
    /// step). Prints the full directive-debt timeline with --nocapture.
    /// A failure is a finding against the Guard-Rail Theorem's engine
    /// face, not a flaky test.
    #[test]
    fn o16_is_empty_and_the_guard_rails_precede_it() {
        let report = o16_emptiness_report();
        for record in &report.timeline {
            println!(
                "stage {:>2}{}: O = {:?}, focus = {}",
                record.stage,
                if record.pre_structural_band {
                    " (pre-structural band)"
                } else {
                    ""
                },
                record.required_packages,
                record.focus_family,
            );
        }

        assert!(
            report.o16_empty,
            "O(16) is NOT empty: {:?} — Guard-Rail Thm 2's engine face fails",
            report
                .timeline
                .iter()
                .find(|record| record.stage == 16)
                .map(|record| &record.required_packages)
        );
        assert!(
            report.unguarded_structural_stages.is_empty(),
            "structural stages without a guard rail: {:?}",
            report.unguarded_structural_stages
        );
        assert_eq!(
            report.first_debt_free_stage,
            Some(16),
            "16 must be the sequence's FIRST debt-free structural stage"
        );
        assert!(
            report.discharge_persistence_holds,
            "non-contiguous demand intervals: {:?}",
            report.persistence_violations
        );
        assert!(report.guard_rail_verified);
    }
}
