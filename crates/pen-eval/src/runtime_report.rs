//! Runtime calculus, Stage 2 (`docs/RUNTIME_CALCULUS.md` §5): the reporting
//! path, implemented AFTER the freeze commit adopting §§1–3, 8, 9, 9.1.
//!
//! This module is the first and only place where the engagement fraction is
//! evaluated over a stratum run and where a crossing index (first k with
//! e_k < 1) can be computed. It exists only post-freeze; Stage 1's modules
//! (`runtime_dedup`, `runtime_field`) remain untouched and still contain no
//! crossing path.
//!
//! Frozen inputs (nothing here re-decides semantics):
//! - The schedule is §9's step loop, run in exhaustive mode — the §9.1
//!   "dissolution path": for the frozen v2 variants the per-round field is
//!   finite, the budget dial is retired, and the enumeration order is a
//!   tie-break (gauge).
//! - D_serv(k) = Σ specification κ of the applications accepted during
//!   cadence step k (RC-2; R-F1 fixes κ as the full instantiation's clause
//!   count). D_tot(k) = Δ_k = F_k in ledger units.
//! - e_k = D_serv(k)/D_tot(k), capped at 1 (P-gauge; `engagement_fraction`).
//!   The crossing index is the first k with e_k < 1 (§2: the widening move
//!   is active at every step close, since every step ends at local
//!   exhaustion under §9).
//! - Band typing at the crossing (R3): the ledger demand factors through
//!   the two-layer chronological window by its own recurrence
//!   Δ_k = Δ_{k−1} + Δ_{k−2} (Appendix D; P-window): the live-layer share
//!   Δ_{k−1} is the depth-1 attachment demand, the sealed-layer share
//!   Δ_{k−2} is the depth-2 echo demand. Edge convention (k ≤ 2, recorded
//!   here because the recurrence has no two predecessors yet): the echo
//!   share is 0 and the whole demand is attachment. The export rule is the
//!   frozen `band_export` (attachments non-deferrable, P-admissibility).
//!
//! Per §5 the run reports whatever comes out; nothing in this module
//! branches on the §6 registered outcomes.

use crate::lambda_trigger::rational_string;
use crate::runtime_bar::fib;
use crate::runtime_dedup::{band_export, engagement_fraction, nu_profile, TypedObligations};
use crate::runtime_field::{
    genesis_reference_context, run_step_loop, CadenceStepReport, FieldPolicy,
};
use pen_core::rational::Rational;
use serde::Serialize;

pub const RUNTIME_CALCULUS_RUN_DATE: &str = "2026-07-05";

/// D_serv(k): the κ-weight discharged by the applications accepted during
/// one cadence step (RC-2 with R-F1's specification κ).
pub fn serviced_kappa_weight(report: &CadenceStepReport) -> u64 {
    report
        .accepted
        .iter()
        .map(|application| u64::from(application.kappa))
        .sum()
}

/// The ledger demand of step k, typed by the two-layer chronological window
/// via the ledger's own factorization Δ_k = Δ_{k−1} + Δ_{k−2}: echo (depth-2)
/// is Δ_{k−2} for k ≥ 3, else 0; attachment (depth-1) is the remainder.
pub fn demand_window_split(step: usize) -> TypedObligations {
    let echo = if step >= 3 { fib(step - 2) } else { 0 };
    TypedObligations {
        attachment: fib(step) - echo,
        echo,
    }
}

/// The crossing index: the first cadence step k (1-based) whose serviced
/// κ-weight falls short of the ledger demand Δ_k = F_k, i.e. the first k
/// with e_k < 1. Returns `None` if no computed step crosses (the horizon
/// must then be extended).
pub fn first_engagement_crossing(serviced_per_step: &[u64]) -> Option<usize> {
    serviced_per_step
        .iter()
        .enumerate()
        .find(|(index, serviced)| **serviced < fib(index + 1))
        .map(|(index, _)| index + 1)
}

fn band_label(sealed: bool, attachment_exported: u64, echo_exported: u64) -> &'static str {
    match (sealed, attachment_exported, echo_exported) {
        (false, _, _) => "unsealed-full-export",
        (true, 0, 0) => "none",
        (true, 0, _) => "depth-2",
        // Unreachable by construction (sealing requires full depth-1
        // service), kept for totality.
        (true, _, _) => "mixed",
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimeCalculusRun {
    pub date: String,
    pub phase: String,
    pub freeze_commit: String,
    pub eval_commit: String,
    pub semantics: String,
    pub cadence_steps_computed: u32,
    pub single_chain_levels: u32,
    pub variants: Vec<VariantRun>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VariantRun {
    pub name: String,
    pub kappa_spec: u32,
    /// RC-1's single-chain ν_k profile (R1 enabled), for the record.
    pub single_chain_nu_profile: Vec<u32>,
    /// The §9 field-schedule run, step by step.
    pub steps: Vec<StepRecord>,
    /// The stratum's Ω-course: one entry per accepted application.
    pub omega_course: Vec<AcceptedRecord>,
    /// First k with e_k < 1, if any step in the horizon crossed.
    pub crossing_index: Option<usize>,
    pub crossing: Option<CrossingRecord>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StepRecord {
    pub step: usize,
    pub accepted_nu: Vec<u32>,
    pub accepted_kappa: Vec<u32>,
    pub d_serv: u64,
    pub d_tot: u64,
    /// e_k = D_serv/D_tot, capped at 1 (P-gauge).
    pub engagement: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AcceptedRecord {
    pub cadence_step: u32,
    pub sealing_index: u32,
    pub nu: u32,
    pub kappa: u32,
    pub rho: String,
    pub bar: String,
    /// Cumulative stratum Ω = Σν/Σκ after this acceptance.
    pub omega_after: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CrossingRecord {
    pub step: usize,
    pub d_serv: u64,
    pub d_tot: u64,
    pub engagement: String,
    /// Δ_{k−1}: depth-1 (attachment) share of the demand.
    pub demand_attachment: u64,
    /// Δ_{k−2}: depth-2 (echo) share of the demand.
    pub demand_echo: u64,
    /// Whether applications seal at the crossing step (P-admissibility:
    /// requires the depth-1 share fully serviced).
    pub sealed: bool,
    pub attachment_exported: u64,
    pub echo_exported: u64,
    /// Total exported share of the demand: (Δ_k − min(D_serv, Δ_k))/Δ_k.
    pub exported_share: String,
    /// Depth composition of the exported band.
    pub band: String,
}

/// Builds the Stage-2 run report for the frozen v2 stratum variants.
///
/// This function performs the computation §5 defines for the blind run. It
/// should be executed once, post-freeze, via
/// `examples/runtime_calculus_run.rs`; per the §4/§5 discipline it is not
/// exercised on the frozen variants by any test.
pub fn build_runtime_calculus_run(
    freeze_commit: impl Into<String>,
    eval_commit: impl Into<String>,
    cadence_steps: u32,
    single_chain_levels: u32,
) -> RuntimeCalculusRun {
    let (library, history) = genesis_reference_context();
    // §9.1 dissolution path: exhaustive rounds; the budget is not policy
    // for the frozen variants.
    let policy = FieldPolicy {
        search_budget: u64::MAX,
    };

    let variants = crate::lambda_trigger_v2::frozen_structure_schema_variants_v2()
        .into_iter()
        .map(|(name, _, telescope)| {
            let single_chain =
                nu_profile(&telescope, &library, &history, single_chain_levels, true);
            let reports = run_step_loop(&telescope, &library, &history, cadence_steps, &policy);

            let mut steps = Vec::with_capacity(reports.len());
            let mut omega_course = Vec::new();
            let mut serviced_per_step = Vec::with_capacity(reports.len());
            let mut sum_nu: u64 = 0;
            let mut sum_kappa: u64 = 0;

            for report in &reports {
                let d_serv = serviced_kappa_weight(report);
                let d_tot = fib(report.cadence_step as usize);
                serviced_per_step.push(d_serv);
                steps.push(StepRecord {
                    step: report.cadence_step as usize,
                    accepted_nu: report.accepted.iter().map(|a| a.nu).collect(),
                    accepted_kappa: report.accepted.iter().map(|a| a.kappa).collect(),
                    d_serv,
                    d_tot,
                    engagement: rational_string(engagement_fraction(d_serv, d_tot)),
                });
                for application in &report.accepted {
                    sum_nu += u64::from(application.nu);
                    sum_kappa += u64::from(application.kappa);
                    omega_course.push(AcceptedRecord {
                        cadence_step: application.cadence_step,
                        sealing_index: application.sealing_index,
                        nu: application.nu,
                        kappa: application.kappa,
                        rho: rational_string(application.rho),
                        bar: rational_string(application.bar),
                        omega_after: rational_string(Rational::new(
                            sum_nu as i64,
                            sum_kappa as i64,
                        )),
                    });
                }
            }

            let crossing_index = first_engagement_crossing(&serviced_per_step);
            let crossing = crossing_index.map(|step| {
                let d_serv = serviced_per_step[step - 1];
                let d_tot = fib(step);
                let window = demand_window_split(step);
                let export = band_export(window, d_serv);
                let exported = d_tot - d_serv.min(d_tot);
                CrossingRecord {
                    step,
                    d_serv,
                    d_tot,
                    engagement: rational_string(engagement_fraction(d_serv, d_tot)),
                    demand_attachment: window.attachment,
                    demand_echo: window.echo,
                    sealed: export.sealed,
                    attachment_exported: export.attachment_exported,
                    echo_exported: export.echo_exported,
                    exported_share: rational_string(Rational::new(
                        exported as i64,
                        d_tot as i64,
                    )),
                    band: band_label(
                        export.sealed,
                        export.attachment_exported,
                        export.echo_exported,
                    )
                    .to_owned(),
                }
            });

            VariantRun {
                name: name.to_owned(),
                kappa_spec: u32::try_from(telescope.kappa()).expect("kappa fits u32"),
                single_chain_nu_profile: single_chain,
                steps,
                omega_course,
                crossing_index,
                crossing,
            }
        })
        .collect();

    RuntimeCalculusRun {
        date: RUNTIME_CALCULUS_RUN_DATE.to_owned(),
        phase: "stage-2-blind-run".to_owned(),
        freeze_commit: freeze_commit.into(),
        eval_commit: eval_commit.into(),
        semantics: "RUNTIME_CALCULUS.md §§1-3 with §8 resolutions, §9 schedule, §9.1 pins; \
                    exhaustive field rounds (budget dial retired); demand window split \
                    Δ_k = Δ_{k−1} (attachment) + Δ_{k−2} (echo), echo = 0 for k ≤ 2"
            .to_owned(),
        cadence_steps_computed: cadence_steps,
        single_chain_levels,
        variants,
    }
}

// ---------------------------------------------------------------------------
// Tests: synthetic data only. The frozen variants' crossing is computed for
// the first time by the single post-freeze run (§5), never by a test.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{band_label, demand_window_split, first_engagement_crossing};
    use crate::runtime_dedup::{band_export, TypedObligations};

    #[test]
    fn demand_window_split_is_the_ledger_recurrence() {
        // Truncated window below k = 3.
        assert_eq!(
            demand_window_split(1),
            TypedObligations {
                attachment: 1,
                echo: 0
            }
        );
        assert_eq!(
            demand_window_split(2),
            TypedObligations {
                attachment: 1,
                echo: 0
            }
        );
        // Exact two-layer partition from k = 3 on: Δ_k = Δ_{k−1} + Δ_{k−2}.
        for step in 3..=24 {
            let split = demand_window_split(step);
            assert_eq!(split.attachment + split.echo, super::fib(step));
            assert_eq!(split.echo, super::fib(step - 2));
            assert_eq!(split.attachment, super::fib(step - 1));
        }
    }

    #[test]
    fn crossing_is_the_first_engagement_shortfall() {
        // Synthetic serviced series against Δ_k = 1, 1, 2, 3, 5, 8, ...
        assert_eq!(first_engagement_crossing(&[]), None);
        assert_eq!(first_engagement_crossing(&[5, 5, 5, 5]), None);
        assert_eq!(first_engagement_crossing(&[0]), Some(1));
        assert_eq!(first_engagement_crossing(&[5, 5, 1]), Some(3));
        assert_eq!(first_engagement_crossing(&[1, 1, 2, 3, 4]), Some(5));
        // Exactly meeting demand does not cross.
        assert_eq!(first_engagement_crossing(&[1, 1, 2, 3, 5]), None);
    }

    #[test]
    fn band_labels_follow_the_export_rule() {
        let window = TypedObligations {
            attachment: 8,
            echo: 5,
        };

        // Depth-1 shortfall: nothing seals, full export.
        let export = band_export(window, 7);
        assert!(!export.sealed);
        assert_eq!(
            band_label(export.sealed, export.attachment_exported, export.echo_exported),
            "unsealed-full-export"
        );

        // Depth-1 serviced, echo partially exported: the depth-2 band.
        let export = band_export(window, 10);
        assert!(export.sealed);
        assert_eq!(export.echo_exported, 3);
        assert_eq!(
            band_label(export.sealed, export.attachment_exported, export.echo_exported),
            "depth-2"
        );

        // Fully serviced: no band.
        let export = band_export(window, 13);
        assert_eq!(
            band_label(export.sealed, export.attachment_exported, export.echo_exported),
            "none"
        );
    }
}
