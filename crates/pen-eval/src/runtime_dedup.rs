//! Runtime calculus, Stage 1 (`docs/RUNTIME_CALCULUS.md`, frozen with the
//! Stage-0 reviewer resolutions of its §8).
//!
//! Implements, as an *extension* of the evaluator surface (nothing in
//! `nu.rs` / `bar.rs` / `runtime_bar.rs` is modified):
//!
//! - **RC-1 (R1, transport-image discount).** Repeated application of a
//!   schema (a *stratum*): the level-k instantiation is produced by the
//!   R-F2 pin (stratum-internal Var references rebind to the level-(k−1)
//!   sealed entry; σ_k is the induced level shift), and a clause whose
//!   expression is the σ_k-image of a clause accepted at a previous level
//!   contributes 0 to all of ν (R-F1: P-internal / P-ν are component-blind).
//!   The genuinely-new residual is scored by the *unchanged* six-principle
//!   evaluator (`crate::nu::structural_nu`) against H_{k−1}. κ is untouched
//!   by R1 (specification κ of the full instantiation).
//!
//! - **RC-2 (throughput/engagement).** Definitional machinery only: the
//!   gauge-invariant engagement fraction e_k = D_serv(k)/D_tot(k) and the
//!   ledger demand Δ_k = F_k. The application schedule that generates
//!   D_serv is the *open* R-F3 pin; no schedule is implemented here.
//!
//! - **RC-3 (runtime obligation-band export).** Definitional machinery
//!   only: obligation typing by referent layer (depth-1 attachment at the
//!   live layer k−1, depth-2 echo at the sealed layer k−2 — P-window), and
//!   the band-export rule (attachments are non-deferrable — P-admissibility;
//!   the unserviced remainder is exported as a typed band).
//!
//! **Stage-1 exclusion (enforced by absence):** no function in this module
//! computes, searches for, or logs a crossing index (first k with e_k < 1),
//! and none of the types here derive `Serialize` — the Stage-2 reporting
//! path deliberately does not exist yet (§5 of the design).

use crate::nu::{structural_nu, StructuralNuResult};
use crate::runtime_bar::fib;
use pen_core::canonical::{canonical_key_expr, CanonKey};
use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// RC-1: stratum instantiation (R-F2 pin) and the R1 transport-image discount
// ---------------------------------------------------------------------------

/// The level-k instantiation of stratum schema `S` (R-F2 pin).
///
/// - `level == 1`: the schema exactly as frozen.
/// - `level >= 2`: every stratum-internal Var reference — `Var(d)` in clause
///   `j` with `d <= j`, the engine's existing clause-connectivity convention —
///   rebinds to `Lib` of the level-(k−1) sealed stratum entry
///   (`base_library_len + level − 1`, 1-based Lib indexing). Ambient
///   references are unchanged.
///
/// By construction `instantiate_level(S, base, k)` equals
/// `σ_k(instantiate_level(S, base, k−1))` for `k >= 3`.
pub fn instantiate_level(schema: &Telescope, base_library_len: u32, level: u32) -> Telescope {
    assert!(level >= 1, "stratum levels are 1-based");
    if level == 1 {
        return schema.clone();
    }

    let previous_entry = base_library_len + level - 1;
    Telescope::new(
        schema
            .clauses
            .iter()
            .enumerate()
            .map(|(clause_index, clause)| {
                ClauseRec::new(
                    clause.role,
                    rebind_internal_vars(&clause.expr, clause_index as u32, previous_entry),
                )
            })
            .collect(),
    )
}

/// The level-shift substitution σ (design §1): stratum-entry Lib references
/// (indices strictly above the ambient library) shift up one level; ambient
/// references and everything else are identity.
pub fn sigma_shift_expr(expr: &Expr, base_library_len: u32) -> Expr {
    map_expr(expr, &|leaf| match leaf {
        Expr::Lib(index) if *index > base_library_len => Some(Expr::Lib(index + 1)),
        _ => None,
    })
}

/// One evaluated level of a stratum under R1.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StratumLevelReport {
    /// 1-based level index k.
    pub level: u32,
    /// The full level-k instantiation (κ is read from this — R1 is ν-only).
    pub instantiation: Telescope,
    /// Indices (into `instantiation.clauses`) discounted as σ-images.
    pub sigma_image_clauses: Vec<usize>,
    /// The genuinely-new residual actually scored by the evaluator.
    pub residual: Telescope,
    /// ν of the residual against H_{k−1}, by the unchanged class formulas.
    pub nu: StructuralNuResult,
}

/// Evaluates `levels` repeated applications of `schema` on top of the ambient
/// library/history, with the R1 transport-image discount enabled or disabled.
///
/// With `r1_enabled == false` the residual is the full instantiation at every
/// level: RC-1 is inert and the standard evaluator semantics apply unchanged
/// (Gate 2 of design §4). With no repetition (`levels == 1`) the two modes
/// coincide exactly (Gate 1, conservativity).
///
/// Each level seals as a whole: the full instantiation becomes a library
/// entry (Rule 1, cumulative growth) and the level's discounted ν enters the
/// history consumed by later levels' class formulas.
pub fn evaluate_stratum_levels(
    schema: &Telescope,
    ambient_library: &Library,
    ambient_history: &[(u32, u32)],
    levels: u32,
    r1_enabled: bool,
) -> Vec<StratumLevelReport> {
    let base = u32::try_from(ambient_library.len()).expect("library size should fit u32");
    let mut library = ambient_library.clone();
    let mut history = ambient_history.to_vec();

    // Accepted stratum clauses, presented at the most recently sealed level.
    let mut accepted: Vec<Expr> = Vec::new();
    let mut reports = Vec::with_capacity(levels as usize);

    for level in 1..=levels {
        let instantiation = instantiate_level(schema, base, level);

        // Transport the accepted set one level forward: at level k the
        // discountable images are σ_k(accepted at levels < k).
        let transported: Vec<Expr> = accepted
            .iter()
            .map(|expr| sigma_shift_expr(expr, base))
            .collect();
        let transported_keys: BTreeSet<CanonKey> =
            transported.iter().map(canonical_key_expr).collect();

        let mut sigma_image_clauses = Vec::new();
        let mut residual_clauses = Vec::new();
        for (index, clause) in instantiation.clauses.iter().enumerate() {
            let is_image = r1_enabled
                && transported_keys.contains(&canonical_key_expr(&clause.expr));
            if is_image {
                sigma_image_clauses.push(index);
            } else {
                residual_clauses.push(clause.clone());
            }
        }

        let residual = Telescope::new(residual_clauses);
        let nu = structural_nu(&residual, &library, &history);

        // Seal the level: full instantiation enters the library; the
        // discounted ν enters the history (P-ν is the definition; R1 is its
        // faithful evaluation under repetition).
        library.push(LibraryEntry::from_telescope(&instantiation, &library));
        history.push((base + level, nu.total));

        // Fold the level's clauses into the accepted set (deduplicated up to
        // the engine's canonical presentation).
        let mut accepted_keys: BTreeSet<CanonKey> =
            transported.iter().map(canonical_key_expr).collect();
        accepted = transported;
        for clause in &instantiation.clauses {
            if accepted_keys.insert(canonical_key_expr(&clause.expr)) {
                accepted.push(clause.expr.clone());
            }
        }

        reports.push(StratumLevelReport {
            level,
            instantiation,
            sigma_image_clauses,
            residual,
            nu,
        });
    }

    reports
}

/// The ν_k profile (totals only) of a stratum — the object Gate 3 of design
/// §4 inspects. Nothing here relates the profile to any bar or crossing.
pub fn nu_profile(
    schema: &Telescope,
    ambient_library: &Library,
    ambient_history: &[(u32, u32)],
    levels: u32,
    r1_enabled: bool,
) -> Vec<u32> {
    evaluate_stratum_levels(schema, ambient_library, ambient_history, levels, r1_enabled)
        .into_iter()
        .map(|report| report.nu.total)
        .collect()
}

// ---------------------------------------------------------------------------
// RC-2: throughput and engagement (definitional machinery; R-F3 pin is OPEN)
// ---------------------------------------------------------------------------

/// The ledger's total demand at cadence step k, in ledger units: Δ_k = F_k
/// (the register-blind Fibonacci debt; design §2).
pub fn ledger_demand(step: usize) -> u64 {
    fib(step)
}

/// The engagement fraction e_k = D_serv(k) / D_tot(k) ∈ [0, 1].
///
/// Capacity enters ONLY as this ratio (P-gauge): both arguments are κ-weights
/// in the same ledger units, so e_k is invariant under global rescaling of
/// the schema-counting unit. Serviced demand cannot exceed total demand, so
/// the ratio is capped at 1 (a producer cannot discharge obligations that do
/// not exist).
///
/// How D_serv is *generated* per cadence step is the open R-F3 pin; this
/// function is the frozen ratio, not a schedule.
pub fn engagement_fraction(serviced_kappa_weight: u64, total_demand: u64) -> Rational {
    assert!(total_demand > 0, "ledger demand is strictly positive");
    if serviced_kappa_weight >= total_demand {
        Rational::one()
    } else {
        Rational::new(serviced_kappa_weight as i64, total_demand as i64)
    }
}

// ---------------------------------------------------------------------------
// RC-3: runtime obligation-band typing and export (definitional machinery)
// ---------------------------------------------------------------------------

/// The two-layer chronological window (P-window, Appendix D): an obligation's
/// referent lives in the live layer (k−1, *attachment*) or the sealed layer
/// (k−2, *echo*). No other typing input is admissible (R3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObligationDepth {
    /// Depth-1: referent at the live layer k−1. Non-deferrable
    /// (P-admissibility, the bare-S³ precedent).
    Attachment,
    /// Depth-2: referent at the sealed layer k−2 — the chronological echo.
    Echo,
}

/// κ-weighted obligations of one application, typed by referent layer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TypedObligations {
    /// Total κ-weight of depth-1 (attachment) obligations.
    pub attachment: u64,
    /// Total κ-weight of depth-2 (echo) obligations.
    pub echo: u64,
}

/// Types the obligations of a level-k application by the chronological
/// factorization: each clause referencing the live layer (library index
/// `base + level − 1`) carries one κ-unit attachment obligation, and each
/// clause referencing the sealed layer (`base + level − 2`) carries one
/// κ-unit echo obligation. Deeper references are derived fillers, not
/// primitive obligations (Appendix D), and are not typed into the band.
pub fn type_level_obligations(
    instantiation: &Telescope,
    base_library_len: u32,
    level: u32,
) -> TypedObligations {
    assert!(level >= 1, "stratum levels are 1-based");
    let live_layer = base_library_len + level - 1;
    let sealed_layer = live_layer.checked_sub(1);

    let mut typed = TypedObligations::default();
    for clause in &instantiation.clauses {
        let refs = clause.expr.lib_refs();
        if refs.contains(&live_layer) {
            typed.attachment += 1;
        }
        if let Some(sealed) = sealed_layer {
            if sealed >= 1 && refs.contains(&sealed) {
                typed.echo += 1;
            }
        }
    }
    typed
}

/// The result of servicing typed obligations against a serviced κ-weight
/// (R3). Depth composition of the export is the report; nothing here decides
/// *when* this situation occurs (that is the excluded crossing).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BandExport {
    /// Whether applications seal at this step: depth-1 obligations must be
    /// fully serviced for any application to seal (P-admissibility).
    pub sealed: bool,
    /// Unserviced depth-1 κ-weight exported on the interface.
    pub attachment_exported: u64,
    /// Unserviced depth-2 κ-weight exported on the interface.
    pub echo_exported: u64,
}

/// Applies the frozen R3 export rule: attachments are serviced first and are
/// non-deferrable — if the serviced κ-weight cannot cover them, nothing
/// seals and the full typed obligation load stands exported. Otherwise the
/// remainder services echoes and the unserviced echo κ-weight is the typed
/// band on the interface.
pub fn band_export(obligations: TypedObligations, serviced_kappa_weight: u64) -> BandExport {
    if serviced_kappa_weight < obligations.attachment {
        return BandExport {
            sealed: false,
            attachment_exported: obligations.attachment,
            echo_exported: obligations.echo,
        };
    }

    let remainder = serviced_kappa_weight - obligations.attachment;
    BandExport {
        sealed: true,
        attachment_exported: 0,
        echo_exported: obligations.echo.saturating_sub(remainder),
    }
}

// ---------------------------------------------------------------------------
// Expression rewriting helpers (private)
// ---------------------------------------------------------------------------

/// Rebinds stratum-internal Var references (`Var(d)` with `d <= clause_index`,
/// the engine's clause-connectivity convention) to `Lib(target)`.
fn rebind_internal_vars(expr: &Expr, clause_index: u32, target: u32) -> Expr {
    map_expr(expr, &|leaf| match leaf {
        Expr::Var(depth) if *depth <= clause_index => Some(Expr::Lib(target)),
        _ => None,
    })
}

/// Structure-preserving map over an expression: `leaf` may replace any node;
/// unreplaced interior nodes recurse. Traversal order is deterministic
/// (pre-order, left to right) — the field generator (`runtime_field`) relies
/// on this for its slot numbering.
pub(crate) fn map_expr(expr: &Expr, leaf: &dyn Fn(&Expr) -> Option<Expr>) -> Expr {
    if let Some(replacement) = leaf(expr) {
        return replacement;
    }

    match expr {
        Expr::App(function, argument) => Expr::App(
            Box::new(map_expr(function, leaf)),
            Box::new(map_expr(argument, leaf)),
        ),
        Expr::Lam(body) => Expr::Lam(Box::new(map_expr(body, leaf))),
        Expr::Pi(domain, codomain) => Expr::Pi(
            Box::new(map_expr(domain, leaf)),
            Box::new(map_expr(codomain, leaf)),
        ),
        Expr::Sigma(domain, codomain) => Expr::Sigma(
            Box::new(map_expr(domain, leaf)),
            Box::new(map_expr(codomain, leaf)),
        ),
        Expr::Id(ty, left, right) => Expr::Id(
            Box::new(map_expr(ty, leaf)),
            Box::new(map_expr(left, leaf)),
            Box::new(map_expr(right, leaf)),
        ),
        Expr::Refl(body) => Expr::Refl(Box::new(map_expr(body, leaf))),
        Expr::Susp(body) => Expr::Susp(Box::new(map_expr(body, leaf))),
        Expr::Trunc(body) => Expr::Trunc(Box::new(map_expr(body, leaf))),
        Expr::Flat(body) => Expr::Flat(Box::new(map_expr(body, leaf))),
        Expr::Sharp(body) => Expr::Sharp(Box::new(map_expr(body, leaf))),
        Expr::Disc(body) => Expr::Disc(Box::new(map_expr(body, leaf))),
        Expr::Shape(body) => Expr::Shape(Box::new(map_expr(body, leaf))),
        Expr::Next(body) => Expr::Next(Box::new(map_expr(body, leaf))),
        Expr::Eventually(body) => Expr::Eventually(Box::new(map_expr(body, leaf))),
        Expr::Bang(body) => Expr::Bang(Box::new(map_expr(body, leaf))),
        Expr::WhyNot(body) => Expr::WhyNot(Box::new(map_expr(body, leaf))),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => expr.clone(),
    }
}

// ---------------------------------------------------------------------------
// Stage-1 gates (design §4) and machinery tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        band_export, engagement_fraction, evaluate_stratum_levels, instantiate_level,
        ledger_demand, nu_profile, sigma_shift_expr, type_level_obligations, BandExport,
        TypedObligations,
    };
    use crate::lambda_trigger_v2::{
        constant_stratum_crossing, frozen_structure_schema_variants_v2,
    };
    use crate::nu::{compute_native_nu, structural_nu};
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::rational::Rational;
    use pen_core::telescope::Telescope;

    fn reference_context(last_step: u32) -> (Library, Vec<(u32, u32)>) {
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

    // -- Gate 1: Genesis conservativity ------------------------------------

    #[test]
    fn gate1_reference_history_is_untouched_by_the_extension() {
        // The strict-lane ν values recorded in EVALUATOR_DERIVATION.md and
        // the trajectory fixture; the runtime layer must not perturb them
        // (it cannot — it never touches nu.rs — but the gate pins it).
        let (_, history) = reference_context(15);
        let totals: Vec<u32> = history.iter().map(|(_, nu)| *nu).collect();
        assert_eq!(&totals[10..], &[26, 34, 46, 62, 103]);
    }

    #[test]
    fn gate1_single_application_matches_the_standard_evaluator_exactly() {
        // With no repeated applications, RC-1 changes nothing: level 1 is
        // scored identically to compute_native_nu on the frozen telescope,
        // with an empty σ-image set, in both R1 modes.
        let (library, history) = reference_context(15);
        for (name, _, telescope) in frozen_structure_schema_variants_v2() {
            let native = compute_native_nu(&telescope, &library, &history);
            for r1_enabled in [false, true] {
                let reports =
                    evaluate_stratum_levels(&telescope, &library, &history, 1, r1_enabled);
                assert_eq!(reports.len(), 1);
                let level_one = &reports[0];
                assert!(
                    level_one.sigma_image_clauses.is_empty(),
                    "{name}: level 1 must have no σ-images"
                );
                assert_eq!(level_one.residual, telescope, "{name}");
                assert_eq!(level_one.nu.total, native.total, "{name}");
                assert_eq!(level_one.nu.nu_g, native.nu_g, "{name}");
                assert_eq!(level_one.nu.nu_c, native.nu_c, "{name}");
                assert_eq!(level_one.nu.nu_h, native.nu_h, "{name}");
            }
        }
    }

    // -- Gate 2: no-go regression with R1 disabled --------------------------

    #[test]
    fn gate2_r1_disabled_keeps_the_constant_stratum_crossing_at_three() {
        for (nu, kappa) in [(1, 1), (7, 3), (12, 6), (103, 8)] {
            assert_eq!(constant_stratum_crossing(nu, kappa, 24), 3);
        }
    }

    #[test]
    fn gate2_r1_disabled_never_discounts_a_clause() {
        let (library, history) = reference_context(15);
        for (name, _, telescope) in frozen_structure_schema_variants_v2() {
            let reports = evaluate_stratum_levels(&telescope, &library, &history, 4, false);
            for report in &reports {
                assert!(
                    report.sigma_image_clauses.is_empty(),
                    "{name}: R1 disabled must be inert at level {}",
                    report.level
                );
                assert_eq!(report.residual, report.instantiation, "{name}");
            }
        }
    }

    // -- Gate 3: mechanics check (ν profile only; no crossing) --------------

    #[test]
    fn gate3_hierarchical_variants_decline_and_r1_is_not_inert() {
        let (library, history) = reference_context(15);
        let hierarchical = ["bound_interface_weighted", "bound_interface_persistent"];

        for (name, _, telescope) in frozen_structure_schema_variants_v2() {
            if !hierarchical.contains(&name) {
                continue;
            }
            let profile = nu_profile(&telescope, &library, &history, 6, true);

            // Non-increasing beyond the first level.
            for window in profile[1..].windows(2) {
                assert!(
                    window[1] <= window[0],
                    "{name}: ν profile must be non-increasing beyond the first level, got {profile:?}"
                );
            }
            // Strictly decreasing at some level (else R1 is inert and the
            // implementation is wrong).
            assert!(
                profile.windows(2).any(|window| window[1] < window[0]),
                "{name}: ν profile must strictly decrease at some level, got {profile:?}"
            );
        }
    }

    #[test]
    fn gate3_sigma_images_appear_from_level_two() {
        // R-F2 consequence, mechanics only: ambient-only clauses (the
        // formation over the metric shell, the binding path) become σ-images
        // at level 2 for every variant.
        let (library, history) = reference_context(15);
        for (name, _, telescope) in frozen_structure_schema_variants_v2() {
            let reports = evaluate_stratum_levels(&telescope, &library, &history, 2, true);
            let level_two = &reports[1];
            assert!(
                level_two.sigma_image_clauses.contains(&0),
                "{name}: the ambient-only formation clause must be a σ-image at level 2"
            );
            assert!(
                !level_two.sigma_image_clauses.is_empty(),
                "{name}: R1 must discount something at level 2"
            );
        }
    }

    // -- R-F2 instantiation pin ---------------------------------------------

    #[test]
    fn instantiation_is_sigma_recursive_beyond_level_two() {
        // instantiate(S, k) = σ_k(instantiate(S, k−1)) for k >= 3.
        for (_, _, telescope) in frozen_structure_schema_variants_v2() {
            for level in 3..=5 {
                let direct = instantiate_level(&telescope, 15, level);
                let shifted = Telescope::new(
                    instantiate_level(&telescope, 15, level - 1)
                        .clauses
                        .iter()
                        .map(|clause| {
                            pen_core::clause::ClauseRec::new(
                                clause.role,
                                sigma_shift_expr(&clause.expr, 15),
                            )
                        })
                        .collect(),
                );
                assert_eq!(direct, shifted);
            }
        }
    }

    #[test]
    fn instantiation_rebinds_internal_vars_and_preserves_ambient_refs() {
        let variants = frozen_structure_schema_variants_v2();
        let (_, _, persistent) = &variants[2];
        let level_two = instantiate_level(persistent, 15, 2);

        // Formation over the metric shell: unchanged (ambient refs only).
        assert_eq!(level_two.clauses[0].expr, persistent.clauses[0].expr);
        // Transport: Pi(Lib(11), Var(1)) → Pi(Lib(11), Lib(16)).
        assert_eq!(
            level_two.clauses[1].expr,
            Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Lib(16)))
        );
        // Merger: Sigma(Var(1), Var(1)) → Sigma(Lib(16), Lib(16)).
        assert_eq!(
            level_two.clauses[3].expr,
            Expr::Sigma(Box::new(Expr::Lib(16)), Box::new(Expr::Lib(16)))
        );
        // Temporal persistence keeps its ambient anchor and gains the
        // stratum reference: App(Lib(15), Var(1)) → App(Lib(15), Lib(16)).
        assert_eq!(
            level_two.clauses[5].expr,
            Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Lib(16)))
        );
    }

    // -- RC-2 machinery ------------------------------------------------------

    #[test]
    fn engagement_fraction_is_a_capped_gauge_invariant_ratio() {
        assert_eq!(engagement_fraction(3, 8), Rational::new(3, 8));
        assert_eq!(engagement_fraction(8, 8), Rational::one());
        assert_eq!(engagement_fraction(13, 8), Rational::one());
        // P-gauge: invariant under global rescaling of the counting unit.
        for scale in [2, 7, 610] {
            assert_eq!(
                engagement_fraction(3 * scale, 8 * scale),
                engagement_fraction(3, 8)
            );
        }
    }

    #[test]
    fn ledger_demand_is_the_fibonacci_debt() {
        assert_eq!(ledger_demand(1), 1);
        assert_eq!(ledger_demand(2), 1);
        assert_eq!(ledger_demand(15), 610);
        assert_eq!(ledger_demand(16), 987);
    }

    // -- RC-3 machinery ------------------------------------------------------

    #[test]
    fn obligations_are_typed_by_the_two_layer_window_only() {
        let variants = frozen_structure_schema_variants_v2();
        let (_, _, persistent) = &variants[2];

        // Level 1: live layer = Lib(15) (temporal shell), sealed = Lib(14).
        let level_one = instantiate_level(persistent, 15, 1);
        let typed = type_level_obligations(&level_one, 15, 1);
        assert_eq!(
            typed,
            TypedObligations {
                attachment: 1, // temporal persistence references Lib(15)
                echo: 1,       // measure compatibility references Lib(14)
            }
        );

        // Level 2: live layer = Lib(16) (the level-1 seal), sealed = Lib(15).
        // Four rebound clauses attach to the level-1 entry; the temporal
        // anchor Lib(15) is now the echo; Lib(14)/Lib(13)/Lib(11) are deeper
        // than the window — derived fillers, never typed.
        let level_two = instantiate_level(persistent, 15, 2);
        let typed = type_level_obligations(&level_two, 15, 2);
        assert_eq!(
            typed,
            TypedObligations {
                attachment: 4,
                echo: 1,
            }
        );
    }

    #[test]
    fn band_export_enforces_non_deferrable_attachments() {
        let obligations = TypedObligations {
            attachment: 5,
            echo: 3,
        };

        // Capacity below the depth-1 load: nothing seals; the full typed
        // load stands exported (the bare-S³ precedent: admissibility gates
        // beyond ρ).
        assert_eq!(
            band_export(obligations, 4),
            BandExport {
                sealed: false,
                attachment_exported: 5,
                echo_exported: 3,
            }
        );

        // Depth-1 fully serviced; remainder services the echo band.
        assert_eq!(
            band_export(obligations, 6),
            BandExport {
                sealed: true,
                attachment_exported: 0,
                echo_exported: 2,
            }
        );

        // Full service: nothing exported.
        assert_eq!(
            band_export(obligations, 8),
            BandExport {
                sealed: true,
                attachment_exported: 0,
                echo_exported: 0,
            }
        );
    }
}
