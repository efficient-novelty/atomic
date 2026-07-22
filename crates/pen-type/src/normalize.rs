//! Weak-head and full normalization for the bounded kernel fragment.
//!
//! This module is the reduction half of the Phase 1 kernel of
//! `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`. It is deliberately NOT a general
//! rewriting engine: the frozen expression alphabet exports exactly one
//! oriented computation clause, generic beta
//! (`App(Lam b, a) -> b[binder := a]`), because no sealed library entry
//! carries a `computation`-role clause. Everything else is a value or a
//! neutral/stuck head, and stuckness is reported as a kernel verdict, never
//! inferred from an AST pattern by callers.
//!
//! Binding convention (kernel v1, frozen):
//! - `Var` carries a 1-based de Bruijn LEVEL into the unified scope
//!   `[ambient parameters, prior telescope fields, enclosing binders]`.
//! - Binders are `Lam` bodies and `Pi`/`Sigma` codomains only. Unary
//!   operators (modal, temporal, `Refl`, `Susp`, `Trunc`) bind nothing.
//! - Because levels are absolute, substitution never shifts references below
//!   the eliminated binder; levels strictly above it shift down by one.
//!
//! Fuel discipline: every entry point takes an explicit fuel budget and
//! returns the number of beta steps consumed. Exhaustion is a verdict
//! (`NormalizeError::FuelExhausted`), not a panic, so downstream phases can
//! fail closed on pathological candidates.

use pen_core::expr::Expr;
use serde::Serialize;
use thiserror::Error;

/// Frozen tag for the kernel binding convention; participates in the
/// elaborator hash discipline (program ground rule 3).
pub const KERNEL_BINDING_CONVENTION: &str = "kernel-v1-debruijn-levels";

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum NormalizeError {
    #[error("fuel exhausted after {steps} beta steps (static bound {budget})")]
    FuelExhausted { steps: u32, budget: u32 },
}

/// Substitute `value` for the binder at absolute `level` in `body`, closing
/// that binder: levels above `level` shift down by one, levels at or below
/// `level - 1` stay fixed. `value`'s free references (all strictly below
/// `level`) never move, but its OWN binder levels (at or above `level`)
/// must be lifted by the number of binders the insertion site sits under,
/// or nested binders would collide (capture). The lift threshold is exactly
/// `level` because `value` was formed at scope `level - 1`.
pub fn substitute_level(body: &Expr, level: u32, value: &Expr) -> Expr {
    substitute_at_depth(body, level, value, 0)
}

fn substitute_at_depth(body: &Expr, level: u32, value: &Expr, depth: u32) -> Expr {
    match body {
        Expr::Var(index) => {
            if *index == level {
                lift_levels(value, level, depth)
            } else if *index > level {
                Expr::Var(index - 1)
            } else {
                Expr::Var(*index)
            }
        }
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(function, argument) => Expr::App(
            Box::new(substitute_at_depth(function, level, value, depth)),
            Box::new(substitute_at_depth(argument, level, value, depth)),
        ),
        Expr::Pi(domain, codomain) => Expr::Pi(
            Box::new(substitute_at_depth(domain, level, value, depth)),
            Box::new(substitute_at_depth(codomain, level, value, depth + 1)),
        ),
        Expr::Sigma(domain, codomain) => Expr::Sigma(
            Box::new(substitute_at_depth(domain, level, value, depth)),
            Box::new(substitute_at_depth(codomain, level, value, depth + 1)),
        ),
        Expr::Lam(inner) => Expr::Lam(Box::new(substitute_at_depth(
            inner,
            level,
            value,
            depth + 1,
        ))),
        Expr::Id(ty, left, right) => Expr::Id(
            Box::new(substitute_at_depth(ty, level, value, depth)),
            Box::new(substitute_at_depth(left, level, value, depth)),
            Box::new(substitute_at_depth(right, level, value, depth)),
        ),
        Expr::Refl(inner) => Expr::Refl(Box::new(substitute_at_depth(inner, level, value, depth))),
        Expr::Susp(inner) => Expr::Susp(Box::new(substitute_at_depth(inner, level, value, depth))),
        Expr::Trunc(inner) => {
            Expr::Trunc(Box::new(substitute_at_depth(inner, level, value, depth)))
        }
        Expr::Flat(inner) => Expr::Flat(Box::new(substitute_at_depth(inner, level, value, depth))),
        Expr::Sharp(inner) => {
            Expr::Sharp(Box::new(substitute_at_depth(inner, level, value, depth)))
        }
        Expr::Disc(inner) => Expr::Disc(Box::new(substitute_at_depth(inner, level, value, depth))),
        Expr::Shape(inner) => {
            Expr::Shape(Box::new(substitute_at_depth(inner, level, value, depth)))
        }
        Expr::Next(inner) => Expr::Next(Box::new(substitute_at_depth(inner, level, value, depth))),
        Expr::Eventually(inner) => {
            Expr::Eventually(Box::new(substitute_at_depth(inner, level, value, depth)))
        }
        Expr::Bang(inner) => Expr::Bang(Box::new(substitute_at_depth(inner, level, value, depth))),
        Expr::WhyNot(inner) => {
            Expr::WhyNot(Box::new(substitute_at_depth(inner, level, value, depth)))
        }
    }
}

/// Lift every level at or above `threshold` by `offset`: repositions a
/// term formed at scope `threshold - 1` so it can sit under `offset`
/// additional binders without its own binders colliding.
fn lift_levels(expr: &Expr, threshold: u32, offset: u32) -> Expr {
    if offset == 0 {
        return expr.clone();
    }
    match expr {
        Expr::Var(index) => {
            if *index >= threshold {
                Expr::Var(index + offset)
            } else {
                Expr::Var(*index)
            }
        }
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(function, argument) => Expr::App(
            Box::new(lift_levels(function, threshold, offset)),
            Box::new(lift_levels(argument, threshold, offset)),
        ),
        Expr::Pi(domain, codomain) => Expr::Pi(
            Box::new(lift_levels(domain, threshold, offset)),
            Box::new(lift_levels(codomain, threshold, offset)),
        ),
        Expr::Sigma(domain, codomain) => Expr::Sigma(
            Box::new(lift_levels(domain, threshold, offset)),
            Box::new(lift_levels(codomain, threshold, offset)),
        ),
        Expr::Lam(inner) => Expr::Lam(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Id(ty, left, right) => Expr::Id(
            Box::new(lift_levels(ty, threshold, offset)),
            Box::new(lift_levels(left, threshold, offset)),
            Box::new(lift_levels(right, threshold, offset)),
        ),
        Expr::Refl(inner) => Expr::Refl(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Susp(inner) => Expr::Susp(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Flat(inner) => Expr::Flat(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Disc(inner) => Expr::Disc(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Shape(inner) => Expr::Shape(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Next(inner) => Expr::Next(Box::new(lift_levels(inner, threshold, offset))),
        Expr::Eventually(inner) => {
            Expr::Eventually(Box::new(lift_levels(inner, threshold, offset)))
        }
        Expr::Bang(inner) => Expr::Bang(Box::new(lift_levels(inner, threshold, offset))),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(lift_levels(inner, threshold, offset))),
    }
}

/// Weak-head normalization outcome. `steps` counts beta reductions consumed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Whnf {
    pub expr: Expr,
    pub steps: u32,
}

/// Reduce the head of `expr` at scope size `scope_len` (the number of scope
/// entries — ambient parameters plus prior fields plus enclosing binders —
/// in effect at this position). Only beta fires; all other heads are
/// returned unchanged.
pub fn whnf(expr: &Expr, scope_len: u32, fuel: u32) -> Result<Whnf, NormalizeError> {
    let mut current = expr.clone();
    let mut steps = 0u32;
    loop {
        match current {
            Expr::App(function, argument) => {
                let function_whnf = whnf(&function, scope_len, fuel.saturating_sub(steps))?;
                steps = steps.saturating_add(function_whnf.steps);
                match function_whnf.expr {
                    Expr::Lam(body) => {
                        if steps >= fuel {
                            return Err(NormalizeError::FuelExhausted {
                                steps,
                                budget: fuel,
                            });
                        }
                        steps = steps.saturating_add(1);
                        // The Lam sits at scope `scope_len`, so its binder
                        // occupies absolute level `scope_len + 1`.
                        current = substitute_level(&body, scope_len + 1, &argument);
                    }
                    other => {
                        return Ok(Whnf {
                            expr: Expr::App(Box::new(other), argument),
                            steps,
                        });
                    }
                }
            }
            other => {
                return Ok(Whnf { expr: other, steps });
            }
        }
    }
}

/// Full normalization: weak-head reduce, then recurse into all subterms,
/// growing `scope_len` under binders. Deterministic and fuel-bounded.
pub fn normalize(expr: &Expr, scope_len: u32, fuel: u32) -> Result<Whnf, NormalizeError> {
    let head = whnf(expr, scope_len, fuel)?;
    let mut steps = head.steps;
    let remaining = fuel.saturating_sub(steps);
    let expr = match head.expr {
        Expr::App(function, argument) => {
            let function = normalize(&function, scope_len, remaining)?;
            steps = steps.saturating_add(function.steps);
            let argument = normalize(&argument, scope_len, fuel.saturating_sub(steps))?;
            steps = steps.saturating_add(argument.steps);
            Expr::App(Box::new(function.expr), Box::new(argument.expr))
        }
        Expr::Pi(domain, codomain) => {
            let domain = normalize(&domain, scope_len, remaining)?;
            steps = steps.saturating_add(domain.steps);
            let codomain = normalize(&codomain, scope_len + 1, fuel.saturating_sub(steps))?;
            steps = steps.saturating_add(codomain.steps);
            Expr::Pi(Box::new(domain.expr), Box::new(codomain.expr))
        }
        Expr::Sigma(domain, codomain) => {
            let domain = normalize(&domain, scope_len, remaining)?;
            steps = steps.saturating_add(domain.steps);
            let codomain = normalize(&codomain, scope_len + 1, fuel.saturating_sub(steps))?;
            steps = steps.saturating_add(codomain.steps);
            Expr::Sigma(Box::new(domain.expr), Box::new(codomain.expr))
        }
        Expr::Lam(body) => {
            let body = normalize(&body, scope_len + 1, remaining)?;
            steps = steps.saturating_add(body.steps);
            Expr::Lam(Box::new(body.expr))
        }
        Expr::Id(ty, left, right) => {
            let ty = normalize(&ty, scope_len, remaining)?;
            steps = steps.saturating_add(ty.steps);
            let left = normalize(&left, scope_len, fuel.saturating_sub(steps))?;
            steps = steps.saturating_add(left.steps);
            let right = normalize(&right, scope_len, fuel.saturating_sub(steps))?;
            steps = steps.saturating_add(right.steps);
            Expr::Id(Box::new(ty.expr), Box::new(left.expr), Box::new(right.expr))
        }
        Expr::Refl(inner) => Expr::Refl(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Susp(inner) => Expr::Susp(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Flat(inner) => Expr::Flat(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Disc(inner) => Expr::Disc(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Shape(inner) => Expr::Shape(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Next(inner) => Expr::Next(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::Bang(inner) => Expr::Bang(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(normalize_unary(
            &inner, scope_len, remaining, &mut steps,
        )?)),
        leaf @ (Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_)) => leaf,
    };
    Ok(Whnf { expr, steps })
}

fn normalize_unary(
    inner: &Expr,
    scope_len: u32,
    fuel: u32,
    steps: &mut u32,
) -> Result<Expr, NormalizeError> {
    let normalized = normalize(inner, scope_len, fuel)?;
    *steps = steps.saturating_add(normalized.steps);
    Ok(normalized.expr)
}

#[cfg(test)]
mod tests {
    use super::{KERNEL_BINDING_CONVENTION, NormalizeError, normalize, substitute_level, whnf};
    use pen_core::expr::Expr;

    fn app(function: Expr, argument: Expr) -> Expr {
        Expr::App(Box::new(function), Box::new(argument))
    }

    fn lam(body: Expr) -> Expr {
        Expr::Lam(Box::new(body))
    }

    #[test]
    fn binding_convention_tag_is_frozen() {
        assert_eq!(KERNEL_BINDING_CONVENTION, "kernel-v1-debruijn-levels");
    }

    #[test]
    fn beta_replaces_the_binder_level_and_keeps_lower_levels_fixed() {
        // At scope 2 ([A1, A2]), Lam binds level 3. `Lam(Var 3)` is the
        // identity; `Lam(Var 1)` is the constant-A1 function.
        let identity = lam(Expr::Var(3));
        let constant = lam(Expr::Var(1));
        let reduced = whnf(&app(identity, Expr::Var(2)), 2, 16).expect("fuel");
        assert_eq!(reduced.expr, Expr::Var(2));
        assert_eq!(reduced.steps, 1);
        let reduced = whnf(&app(constant, Expr::Var(2)), 2, 16).expect("fuel");
        assert_eq!(reduced.expr, Expr::Var(1));
    }

    #[test]
    fn substitution_shifts_only_levels_above_the_closed_binder() {
        // body = Lam(Var 4) under an outer binder at level 3: after closing
        // level 3, the inner binder drops to level 3, so Var 4 -> Var 3.
        let body = lam(Expr::Var(4));
        let substituted = substitute_level(&body, 3, &Expr::Univ);
        assert_eq!(substituted, lam(Expr::Var(3)));
        // References below the binder never move.
        let substituted = substitute_level(&Expr::Var(2), 3, &Expr::Univ);
        assert_eq!(substituted, Expr::Var(2));
    }

    #[test]
    fn substitution_lifts_the_inserted_value_under_binders() {
        // Capture case within the 6-node cap: at scope 1,
        // App(Lam(Lam(Var 2)), Lam(Var 2)). The outer binder is level 2;
        // its body Lam(Var 2) uses it under one more binder. The argument
        // Lam(Var 2) is the identity (binder level 2 at scope 1). After
        // beta the identity sits under one binder, so its own binder must
        // lift to level 3: Lam(Lam(Var 3)), the constant-identity — NOT
        // Lam(Lam(Var 2)), which would capture the outer binder.
        let body = lam(Expr::Var(2));
        let identity_at_scope_1 = lam(Expr::Var(2));
        let substituted = substitute_level(&body, 2, &identity_at_scope_1);
        assert_eq!(substituted, lam(lam(Expr::Var(3))));

        let redex = app(lam(lam(Expr::Var(2))), lam(Expr::Var(2)));
        let reduced = whnf(&redex, 1, 16).expect("fuel");
        assert_eq!(reduced.expr, lam(lam(Expr::Var(3))));
    }

    #[test]
    fn nested_beta_normalizes_under_binders() {
        // At scope 1: Lam(App(Lam(Var 3), Var 1)) — the inner redex sits
        // under one binder (scope 2 inside), and reduces to Lam(Var 1).
        let inner = app(lam(Expr::Var(3)), Expr::Var(1));
        let outer = lam(inner);
        let normalized = normalize(&outer, 1, 16).expect("fuel");
        assert_eq!(normalized.expr, lam(Expr::Var(1)));
        assert_eq!(normalized.steps, 1);
    }

    #[test]
    fn non_beta_heads_are_returned_unchanged() {
        // Temporal head applications never reduce: stuck by construction.
        let stuck = app(Expr::Eventually(Box::new(Expr::Var(1))), Expr::Var(2));
        let reduced = whnf(&stuck, 7, 16).expect("fuel");
        assert_eq!(reduced.expr, stuck);
        assert_eq!(reduced.steps, 0);
        let lib_headed = app(Expr::Lib(15), Expr::Var(1));
        assert_eq!(whnf(&lib_headed, 3, 16).expect("fuel").expr, lib_headed);
    }

    #[test]
    fn fuel_exhaustion_is_a_verdict_not_a_panic() {
        // Two chained redexes with fuel for only one.
        let expr = app(lam(app(lam(Expr::Var(2)), Expr::Var(1))), Expr::Univ);
        let result = whnf(&expr, 0, 1);
        assert_eq!(
            result,
            Err(NormalizeError::FuelExhausted {
                steps: 1,
                budget: 1
            })
        );
    }
}
