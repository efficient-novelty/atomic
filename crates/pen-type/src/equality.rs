//! The frozen judgmental / univalent equality decision procedure.
//!
//! Phase 1 of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md` needs a judgmental
//! equality for naturality squares; Phase 2 identifies typed normal forms
//! "only through the frozen univalent-equality decision procedure" and must
//! retain its equality witness. In the bounded kernel fragment both notions
//! coincide with equality of fuel-bounded beta normal forms under the
//! de Bruijn level convention:
//!
//! - levels make alpha-equivalence syntactic (no renaming quotient is
//!   needed beyond the level discipline itself);
//! - beta is the only oriented computation clause exported by the sealed
//!   signature, so definitional equality is beta-convertibility;
//! - the fragment has no eta, no path computation, and no library
//!   delta-unfolding (entries are opaque constants), so nothing else may be
//!   identified without leaving the frozen surface.
//!
//! Every decision returns a serializable witness carrying both normal forms
//! and the fuel spent, so replay can re-derive the verdict instead of
//! trusting it.

use crate::normalize::{NormalizeError, normalize};
use pen_core::expr::Expr;
use serde::Serialize;

/// Frozen tag for the equality procedure; participates in the elaborator
/// hash discipline (program ground rule 3).
pub const KERNEL_EQUALITY_PROCEDURE: &str = "kernel-v1-beta-normal-equality";

/// A replayable equality decision: both sides normalized at the same scope,
/// compared syntactically (levels make alpha structural).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EqualityWitness {
    pub scope_len: u32,
    pub left_normal_form: Expr,
    pub right_normal_form: Expr,
    pub left_steps: u32,
    pub right_steps: u32,
    pub equal: bool,
}

/// Decide judgmental equality of `left` and `right` at scope size
/// `scope_len` with the given fuel budget, producing a witness.
pub fn judgmental_equality(
    left: &Expr,
    right: &Expr,
    scope_len: u32,
    fuel: u32,
) -> Result<EqualityWitness, NormalizeError> {
    let left_nf = normalize(left, scope_len, fuel)?;
    let right_nf = normalize(right, scope_len, fuel)?;
    let equal = left_nf.expr == right_nf.expr;
    Ok(EqualityWitness {
        scope_len,
        left_normal_form: left_nf.expr,
        right_normal_form: right_nf.expr,
        left_steps: left_nf.steps,
        right_steps: right_nf.steps,
        equal,
    })
}

/// The univalent equality of the frozen fragment. Identifications beyond
/// judgmental equality would require transport structure the frozen
/// alphabet does not export, so univalent equality coincides with
/// judgmental equality here; the distinct entry point exists so Phase 2
/// callers name the procedure they rely on and downstream certificates can
/// record which procedure produced a witness.
pub fn univalent_equality(
    left: &Expr,
    right: &Expr,
    scope_len: u32,
    fuel: u32,
) -> Result<EqualityWitness, NormalizeError> {
    judgmental_equality(left, right, scope_len, fuel)
}

#[cfg(test)]
mod tests {
    use super::{KERNEL_EQUALITY_PROCEDURE, judgmental_equality, univalent_equality};
    use pen_core::expr::Expr;

    fn app(function: Expr, argument: Expr) -> Expr {
        Expr::App(Box::new(function), Box::new(argument))
    }

    fn lam(body: Expr) -> Expr {
        Expr::Lam(Box::new(body))
    }

    #[test]
    fn equality_procedure_tag_is_frozen() {
        assert_eq!(KERNEL_EQUALITY_PROCEDURE, "kernel-v1-beta-normal-equality");
    }

    #[test]
    fn beta_convertible_terms_are_judgmentally_equal_with_witness() {
        // At scope 1: App(Lam(Var 2), Var 1) reduces to Var 1.
        let redex = app(lam(Expr::Var(2)), Expr::Var(1));
        let witness = judgmental_equality(&redex, &Expr::Var(1), 1, 16).expect("fuel");
        assert!(witness.equal);
        assert_eq!(witness.left_normal_form, Expr::Var(1));
        assert_eq!(witness.right_normal_form, Expr::Var(1));
        assert_eq!(witness.left_steps, 1);
        assert_eq!(witness.right_steps, 0);
    }

    #[test]
    fn distinct_stuck_heads_are_not_identified() {
        let next = Expr::Next(Box::new(Expr::Var(1)));
        let eventually = Expr::Eventually(Box::new(Expr::Var(1)));
        let witness = judgmental_equality(&next, &eventually, 1, 16).expect("fuel");
        assert!(!witness.equal);
    }

    #[test]
    fn univalent_equality_coincides_with_judgmental_equality_on_the_fragment() {
        let redex = app(lam(Expr::Var(2)), Expr::Var(1));
        let judgmental = judgmental_equality(&redex, &Expr::Var(1), 1, 16).expect("fuel");
        let univalent = univalent_equality(&redex, &Expr::Var(1), 1, 16).expect("fuel");
        assert_eq!(judgmental, univalent);
    }
}
