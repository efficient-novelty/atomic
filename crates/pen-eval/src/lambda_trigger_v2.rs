//! L-Lambda1 clause computation, phase 2: diagnosis of the v1 run and
//! the two no-go results that reclassify its verdict.
//!
//! The v1 report (`docs/lambda_trigger_computation.json`) recorded "miss".
//! This module establishes that the run was *invalid* rather than an
//! informative miss:
//!
//! 1. The v1 frozen schemas used bare `Var` expressions with no library
//!    references, no path clauses, and no recognized formation structure,
//!    so the evaluator scored nu = 0 for all variants. A rho = 0 candidate
//!    is inadmissible from the start and cannot be the runtime producer
//!    whose plateau defines the trigger. (The v2 variants below are
//!    library-connected and score nu > 0, demonstrating the pathology was
//!    the construction, not the concept.)
//!
//! 2. No-go (C1): for ANY constant-(nu, kappa) stratum with rho > 0, the
//!    fresh-stratum bar crosses at n = 3, because Phi_3 = F_3/F_2 = 2 is
//!    the first inflation factor exceeding 1 and Bar_n = Phi_n * rho-bar.
//!    Hence no frozen constant schema can probe H16 under the current C1
//!    semantics; a declining per-step rho profile (novelty deduplication
//!    across repeated application) is a *missing evaluator capability*,
//!    not a schema-construction choice.
//!
//! 3. No-go (C2): the v1 capacity was the schema's specification kappa.
//!    Any admissible schema specification has kappa of order 1..=10
//!    (Genesis range), giving a debt crossing at n <= 7; crossing at 16
//!    requires capacity in [F_15, F_16) = [610, 987) clause units of
//!    *throughput per cadence step* -- a quantity the present calculus
//!    does not define. Specification cost and discharge throughput are
//!    different quantities.
//!
//! Consequence: the correct v1 verdict is "invalid-run", and the phase-2
//! status is "blocked-on-runtime-calculus": the internal derivation of
//! n* = 16, the coefficient note's Premises A/B, and the w_eff(z) profile
//! are three faces of one missing layer (runtime multiplicity, repeated-
//! application deduplication, and runtime obligation-band export).

use crate::bar::compute_rho;
use crate::nu::{compute_native_nu, structural_nu};
use crate::runtime_bar::{fib, fresh_stratum_bars_through};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use serde::Serialize;

pub const LAMBDA_TRIGGER_V2_DATE: &str = "2026-07-04";

/// Library-connected v2 schema variants, frozen a priori.
///
/// Purpose: demonstrate that a bound-structure schema built the way the
/// reference telescopes are built (library pointers to the metric shell
/// (13), connection (11), Hilbert/measure shell (14), temporal operator
/// (15); a formation clause; a binding path; post entries) is a live
/// candidate with nu > 0. These variants are NOT read as probes of H16:
/// per the no-go results, the current C1/C2 semantics cannot probe H16
/// for any frozen schema.
pub fn frozen_structure_schema_variants_v2() -> Vec<(&'static str, &'static str, Telescope)> {
    vec![
        (
            "bound_interface_minimal",
            "Formation of the bound-structure type over the Step-13 metric shell, transport of the overdensity along the Step-11 connection, and the binding (virial-closure) path.",
            Telescope::new(vec![
                formation_over_metric_shell(),
                transport_along_connection(),
                binding_path(),
            ]),
        ),
        (
            "bound_interface_weighted",
            "Adds hierarchical pairing (merger) and Step-14 measure compatibility as post-path entries.",
            Telescope::new(vec![
                formation_over_metric_shell(),
                transport_along_connection(),
                binding_path(),
                merger_pairing(),
                measure_compatibility(),
            ]),
        ),
        (
            "bound_interface_persistent",
            "Adds Step-15 temporal persistence of the bound interface as a further post-path entry.",
            Telescope::new(vec![
                formation_over_metric_shell(),
                transport_along_connection(),
                binding_path(),
                merger_pairing(),
                measure_compatibility(),
                temporal_persistence(),
            ]),
        ),
    ]
}

fn formation_over_metric_shell() -> ClauseRec {
    // App(Univ, Lib(13)): a type formation (recognized head Univ) over the
    // Step-13 metric shell.
    ClauseRec::new(
        ClauseRole::Formation,
        Expr::App(Box::new(Expr::Univ), Box::new(Expr::Lib(13))),
    )
}

fn transport_along_connection() -> ClauseRec {
    // Pi over the Step-11 connection interface: the overdensity is carried
    // by the single transport stratum.
    ClauseRec::new(
        ClauseRole::Formation,
        Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Var(1))),
    )
}

fn binding_path() -> ClauseRec {
    // The binding identity: virialized self-consistency as a dimension-1
    // path constructor.
    ClauseRec::new(ClauseRole::Computation, Expr::PathCon(1))
}

fn merger_pairing() -> ClauseRec {
    // Hierarchical composition: a pair of bound interfaces composes back
    // into the bound-interface former.
    ClauseRec::new(
        ClauseRole::Formation,
        Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
    )
}

fn measure_compatibility() -> ClauseRec {
    // Weighting against the Step-14 Hilbert/measure shell.
    ClauseRec::new(
        ClauseRole::Computation,
        Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Var(1))),
    )
}

fn temporal_persistence() -> ClauseRec {
    // Persistence of the bound interface under the Step-15 temporal shell.
    ClauseRec::new(
        ClauseRole::Computation,
        Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
    )
}

/// No-go (C1): first fresh-bar crossing for a constant-(nu, kappa) stratum.
///
/// Returns the first n with Bar_n^(s) > rho-bar. For every nu > 0 this is 3.
pub fn constant_stratum_crossing(nu: u64, kappa: u64, max_n: usize) -> usize {
    let steps: Vec<(u64, u64)> = std::iter::repeat((nu, kappa)).take(max_n).collect();
    let bars = fresh_stratum_bars_through(&steps, max_n);
    let rho = Rational::new(nu as i64, kappa as i64);
    bars.iter()
        .position(|bar| *bar > rho)
        .map(|index| index + 1)
        .unwrap_or(0)
}

/// No-go (C2): debt crossing when capacity is read as specification kappa.
///
/// Returns the first n with F_n > kappa_spec. For kappa_spec <= 21 this is
/// at most 8; crossing at 16 would require kappa_spec in [610, 987).
pub fn spec_kappa_debt_crossing(kappa_spec: u64) -> usize {
    (1..=64)
        .find(|n| fib(*n) > kappa_spec)
        .unwrap_or(0)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LambdaTriggerV2Report {
    pub date: String,
    pub phase: String,
    pub freeze_commit: String,
    pub eval_commit: String,
    pub v1_assessment: V1Assessment,
    pub no_go: NoGoResults,
    pub v2_variants: Vec<V2VariantOutput>,
    pub blocked_on: Vec<String>,
    pub consolidation: String,
    pub verdict: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct V1Assessment {
    pub recorded_verdict: String,
    pub corrected_verdict: String,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NoGoResults {
    pub c1_constant_stratum_crossing: usize,
    pub c1_statement: String,
    pub c2_max_crossing_for_genesis_range_kappa: usize,
    pub c2_statement: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct V2VariantOutput {
    pub name: String,
    pub rationale: String,
    pub nu_total: u32,
    pub nu_g: u32,
    pub nu_c: u32,
    pub nu_h: u32,
    pub kappa: u32,
    pub rho: String,
    pub reading: String,
}

pub fn build_lambda_trigger_v2_report(
    freeze_commit: impl Into<String>,
    eval_commit: impl Into<String>,
) -> LambdaTriggerV2Report {
    let (library, history) = reference_library(15);

    let v2_variants = frozen_structure_schema_variants_v2()
        .into_iter()
        .map(|(name, rationale, telescope)| {
            let native = compute_native_nu(&telescope, &library, &history);
            let kappa = u32::try_from(telescope.kappa()).expect("kappa should fit u32");
            let rho = compute_rho(native.total, kappa).expect("v2 variants have nonzero kappa");
            V2VariantOutput {
                name: name.to_owned(),
                rationale: rationale.to_owned(),
                nu_total: native.total,
                nu_g: native.nu_g,
                nu_c: native.nu_c,
                nu_h: native.nu_h,
                kappa,
                rho: format!("{}/{}", rho.num(), rho.den()),
                reading: "mechanics demonstration only; NOT an H16 probe (see no_go)".to_owned(),
            }
        })
        .collect();

    let c1_crossing = constant_stratum_crossing(7, 3, 24);
    let c2_max = (1..=10).map(spec_kappa_debt_crossing).max().unwrap_or(0);

    LambdaTriggerV2Report {
        date: LAMBDA_TRIGGER_V2_DATE.to_owned(),
        phase: "v2-diagnosis".to_owned(),
        freeze_commit: freeze_commit.into(),
        eval_commit: eval_commit.into(),
        v1_assessment: V1Assessment {
            recorded_verdict: "miss".to_owned(),
            corrected_verdict: "invalid-run".to_owned(),
            reasons: vec![
                "v1 schemas had no library references, no path clauses, and no recognized formation: evaluator scored nu = 0, rho = 0; a rho = 0 candidate is inadmissible and cannot be the plateaued runtime producer".to_owned(),
                "v1 C2 capacity was specification kappa (1..3), not discharge throughput; the target band [610, 987) was never probeable".to_owned(),
                "v1 C3 band was hardcoded from the C2 crossing step; no obligation-band machinery was consulted".to_owned(),
            ],
        },
        no_go: NoGoResults {
            c1_constant_stratum_crossing: c1_crossing,
            c1_statement: "for any constant-(nu,kappa) stratum with rho > 0, Bar_n = Phi_n * rho-bar and Phi_3 = 2 is the first inflation factor > 1, so the crossing is always n = 3; H16 is unprobeable by any frozen constant schema under current C1 semantics".to_owned(),
            c2_max_crossing_for_genesis_range_kappa: c2_max,
            c2_statement: "for specification kappa in the Genesis range (<= 10), the debt crossing is <= 7; crossing at 16 requires throughput capacity in [610, 987) clause units per cadence step, a quantity the calculus does not yet define".to_owned(),
        },
        v2_variants,
        blocked_on: vec![
            "runtime multiplicity: clause-throughput units per cadence step (needed by C2)".to_owned(),
            "novelty deduplication across repeated schema application: the declining per-step rho profile (needed by C1)".to_owned(),
            "runtime obligation-band export at the crossing step (needed by C3; Premise A check)".to_owned(),
        ],
        consolidation: "the internal derivation of n* = 16 (L-Lambda1), Premises A/B of the coefficient note (L-Lambda2), and the w_eff(z) response profile are three faces of one missing layer: the runtime extension of the clause calculus".to_owned(),
        verdict: "blocked-on-runtime-calculus".to_owned(),
    }
}

fn reference_library(last_step: u32) -> (Library, Vec<(u32, u32)>) {
    let mut library = Vec::new();
    let mut history = Vec::new();

    for step in 1..=last_step {
        let telescope = Telescope::reference(step);
        let result = structural_nu(&telescope, &library, &history);
        library.push(LibraryEntry::from_telescope(&telescope, &library));
        history.push((step, result.total));
    }

    (library, history)
}

#[cfg(test)]
mod tests {
    use super::{
        build_lambda_trigger_v2_report, constant_stratum_crossing,
        frozen_structure_schema_variants_v2, spec_kappa_debt_crossing,
    };

    #[test]
    fn v2_variants_are_frozen_in_a_priori_order_and_library_connected() {
        let variants = frozen_structure_schema_variants_v2();
        let names: Vec<_> = variants.iter().map(|(name, _, _)| *name).collect();
        assert_eq!(
            names,
            vec![
                "bound_interface_minimal",
                "bound_interface_weighted",
                "bound_interface_persistent"
            ]
        );
        assert_eq!(variants[0].2.kappa(), 3);
        assert_eq!(variants[1].2.kappa(), 5);
        assert_eq!(variants[2].2.kappa(), 6);
    }

    #[test]
    fn v2_variants_score_nonzero_nu_unlike_v1() {
        let report = build_lambda_trigger_v2_report("freeze", "eval");
        for variant in &report.v2_variants {
            assert!(
                variant.nu_total > 0,
                "v2 variant {} must be a live candidate (nu > 0); got 0",
                variant.name
            );
        }
    }

    #[test]
    fn no_go_c1_constant_stratum_always_crosses_at_three() {
        for (nu, kappa) in [(1, 1), (7, 3), (12, 6), (103, 8)] {
            assert_eq!(constant_stratum_crossing(nu, kappa, 24), 3);
        }
    }

    #[test]
    fn no_go_c2_spec_kappa_cannot_reach_sixteen() {
        for kappa in 1..=21 {
            assert!(spec_kappa_debt_crossing(kappa) <= 8);
        }
        // Crossing at 16 would require spec kappa in [F_15, F_16) = [610, 987).
        assert_eq!(spec_kappa_debt_crossing(610), 16);
        assert_eq!(spec_kappa_debt_crossing(986), 16);
        assert_eq!(spec_kappa_debt_crossing(987), 17);
    }
}
