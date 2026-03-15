use crate::clause::{ClauseRec, ClauseRole};
use crate::encode::expr_bit_length;
use crate::expr::Expr;
use crate::library::Library;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct Telescope {
    pub clauses: Vec<ClauseRec>,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum TelescopeClass {
    Foundation,
    Former,
    Hit,
    Suspension,
    Map,
    Modal,
    Axiomatic,
    Synthesis,
    Unknown,
}

impl Default for TelescopeClass {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Telescope {
    pub fn new(clauses: Vec<ClauseRec>) -> Self {
        Self { clauses }
    }

    pub fn kappa(&self) -> usize {
        self.clauses.len()
    }

    pub fn bit_cost(&self) -> u32 {
        self.clauses
            .iter()
            .map(|clause| expr_bit_length(&clause.expr))
            .sum()
    }

    pub fn lib_refs(&self) -> BTreeSet<u32> {
        self.clauses
            .iter()
            .flat_map(|clause| clause.expr.lib_refs())
            .collect()
    }

    pub fn var_refs(&self) -> BTreeSet<u32> {
        self.clauses
            .iter()
            .flat_map(|clause| clause.expr.var_refs())
            .collect()
    }

    pub fn max_lib_ref(&self) -> u32 {
        self.lib_refs().iter().next_back().copied().unwrap_or(0)
    }

    pub fn desugared_kappa(&self) -> usize {
        self.clauses
            .iter()
            .map(|clause| desugared_roles(&clause.expr).len())
            .sum::<usize>()
            + self.missing_formation_clause()
    }

    pub fn is_connected(&self) -> bool {
        if self.clauses.len() <= 1 {
            return true;
        }

        (0..self.clauses.len() - 1).all(|i| {
            ((i + 1)..self.clauses.len()).any(|j| {
                let de_bruijn = (j - i) as u32;
                self.clauses[j].expr.var_refs().contains(&de_bruijn)
            }) || self.clauses[i].expr.has_lib_pointer()
        })
    }

    pub fn references_window(&self, lib_size: u32) -> bool {
        if lib_size == 0 || lib_size <= 2 {
            return true;
        }

        let refs = self.lib_refs();
        refs.contains(&lib_size) || refs.contains(&(lib_size - 1))
    }

    pub fn has_point_constructor(&self) -> bool {
        self.clauses.iter().any(|clause| match &clause.expr {
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => true,
            Expr::Var(_) => true,
            Expr::App(left, right)
                if matches!(left.as_ref(), Expr::Lib(_))
                    && matches!(right.as_ref(), Expr::Var(_)) =>
            {
                true
            }
            _ => false,
        })
    }

    pub fn path_dimensions(&self) -> Vec<u32> {
        self.clauses
            .iter()
            .filter_map(|clause| match clause.expr {
                Expr::PathCon(dimension) => Some(dimension),
                _ => None,
            })
            .collect()
    }

    pub fn has_loop(&self) -> bool {
        !self.path_dimensions().is_empty()
    }

    pub fn is_trivially_derivable(&self, _library: &Library) -> bool {
        if self.clauses.is_empty() {
            return true;
        }

        if self
            .clauses
            .iter()
            .all(|clause| matches!(clause.expr, Expr::Lib(_) | Expr::Var(_)))
        {
            return true;
        }

        self.is_trunc_only_higher_path_hybrid()
    }

    pub fn classify(&self, _library: &Library) -> TelescopeClass {
        if self.clauses.is_empty() {
            return TelescopeClass::Unknown;
        }

        if self.kappa() == 1 && is_formation_only(&self.clauses[0].expr) {
            return classify_single_entry(&self.clauses[0].expr);
        }

        if self.kappa() == 1 {
            if let Expr::App(left, _) = &self.clauses[0].expr {
                if matches!(left.as_ref(), Expr::Lib(_)) {
                    return TelescopeClass::Map;
                }
            }
        }

        let exprs: Vec<&Expr> = self.clauses.iter().map(|clause| &clause.expr).collect();
        let is_foundation_like = self.kappa() >= 2
            && exprs.iter().all(|expr| is_basic_formation_entry(expr))
            && !exprs.iter().any(|expr| expr.has_lib_pointer());

        if is_foundation_like {
            return TelescopeClass::Foundation;
        }
        if exprs.iter().any(|expr| matches!(expr, Expr::PathCon(_))) {
            return TelescopeClass::Hit;
        }
        if exprs.iter().any(|expr| expr.is_modal()) && !exprs.iter().any(|expr| expr.is_temporal())
        {
            return TelescopeClass::Modal;
        }
        if exprs.iter().any(|expr| expr.is_temporal()) {
            return TelescopeClass::Synthesis;
        }
        if exprs.iter().any(|expr| matches!(expr, Expr::Susp(_))) {
            return TelescopeClass::Suspension;
        }
        if (2..=4).contains(&self.kappa())
            && exprs.iter().take(2).all(|expr| expr.has_lib_pointer())
        {
            return TelescopeClass::Map;
        }
        if self.kappa() >= 3
            && exprs.iter().any(|expr| expr.has_lib_pointer())
            && !exprs.iter().any(|expr| expr.is_modal())
        {
            return TelescopeClass::Axiomatic;
        }
        if exprs.iter().all(|expr| is_pi_sigma_expr(expr))
            && !exprs.iter().any(|expr| expr.has_lib_pointer())
        {
            return TelescopeClass::Former;
        }

        TelescopeClass::Unknown
    }

    pub fn reference(step: u32) -> Self {
        match step {
            1 => Self::new(vec![
                clause(Expr::Univ),
                clause(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
            ]),
            2 => Self::new(vec![clause(Expr::App(
                Box::new(Expr::Univ),
                Box::new(Expr::Var(1)),
            ))]),
            3 => Self::new(vec![clause(Expr::App(
                Box::new(Expr::Lib(2)),
                Box::new(Expr::Var(1)),
            ))]),
            4 => Self::new(vec![
                clause(Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                )))),
                clause(Expr::App(
                    Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                    Box::new(Expr::Var(3)),
                )),
                clause(Expr::App(
                    Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Var(2)),
                )),
            ]),
            5 => Self::new(vec![
                clause(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
                clause(Expr::Var(1)),
                clause(Expr::PathCon(1)),
            ]),
            6 => Self::new(vec![
                clause(Expr::Trunc(Box::new(Expr::Var(1)))),
                clause(Expr::App(
                    Box::new(Expr::Trunc(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Var(2)),
                )),
                clause(Expr::PathCon(1)),
            ]),
            7 => Self::new(vec![
                clause(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
                clause(Expr::Var(1)),
                clause(Expr::PathCon(2)),
            ]),
            8 => Self::new(vec![
                clause(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
                clause(Expr::Var(1)),
                clause(Expr::PathCon(3)),
                clause(Expr::Lam(Box::new(Expr::Var(1)))),
                clause(Expr::Lam(Box::new(Expr::Var(2)))),
            ]),
            9 => Self::new(vec![
                clause(Expr::Pi(Box::new(Expr::Lib(8)), Box::new(Expr::Lib(7)))),
                clause(Expr::App(Box::new(Expr::Lib(5)), Box::new(Expr::Var(1)))),
                clause(Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(8)),
                    Box::new(Expr::Lib(7)),
                )))),
                clause(Expr::Pi(Box::new(Expr::Lib(7)), Box::new(Expr::Lib(8)))),
            ]),
            10 => Self::new(vec![
                clause(Expr::Flat(Box::new(Expr::Var(1)))),
                clause(Expr::Sharp(Box::new(Expr::Var(1)))),
                clause(Expr::Disc(Box::new(Expr::Var(1)))),
                clause(Expr::Shape(Box::new(Expr::Var(1)))),
            ]),
            11 => Self::new(vec![
                clause(Expr::Pi(
                    Box::new(Expr::Lib(10)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                clause(Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                )))),
                clause(Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Var(1)),
                )),
                clause(Expr::App(Box::new(Expr::Lib(10)), Box::new(Expr::Var(1)))),
                clause(Expr::Lam(Box::new(Expr::Var(1)))),
            ]),
            12 => Self::new(vec![
                clause(Expr::Pi(
                    Box::new(Expr::Lib(11)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                clause(Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(11)),
                    Box::new(Expr::Var(1)),
                )))),
                clause(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Lib(11)))),
                clause(Expr::App(
                    Box::new(Expr::Lib(11)),
                    Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                )),
                clause(Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                )))),
                clause(Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Lib(11)))),
            ]),
            13 => Self::new(vec![
                clause(Expr::Sigma(
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                clause(Expr::Pi(
                    Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                    Box::new(Expr::Lib(11)),
                )),
                clause(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                clause(Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                )))),
                clause(Expr::Pi(Box::new(Expr::Lib(12)), Box::new(Expr::Lib(12)))),
                clause(Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(1)),
                )))),
                clause(Expr::Pi(Box::new(Expr::Lib(12)), Box::new(Expr::Var(1)))),
            ]),
            14 => Self::new(vec![
                clause(Expr::Sigma(
                    Box::new(Expr::Pi(
                        Box::new(Expr::Var(1)),
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                    )),
                    Box::new(Expr::Var(1)),
                )),
                clause(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                clause(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                clause(Expr::Pi(
                    Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                )),
                clause(Expr::Sigma(
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                clause(Expr::Pi(Box::new(Expr::Lib(13)), Box::new(Expr::Var(1)))),
                clause(Expr::Pi(Box::new(Expr::Lib(12)), Box::new(Expr::Var(1)))),
                clause(Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Var(1)))),
                clause(Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Univ),
                )))),
            ]),
            15 => Self::new(vec![
                clause(Expr::Next(Box::new(Expr::Var(1)))),
                clause(Expr::Eventually(Box::new(Expr::Var(1)))),
                clause(Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                )),
                clause(Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(10)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                )))),
                clause(Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                )),
                clause(Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                )),
                clause(Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Var(2)),
                )))),
                clause(Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                )),
            ]),
            _ => Self::default(),
        }
    }

    pub fn all_reference_telescopes() -> Vec<(u32, Telescope)> {
        (1..=15).map(|step| (step, Self::reference(step))).collect()
    }

    fn missing_formation_clause(&self) -> usize {
        let exprs: Vec<&Expr> = self.clauses.iter().map(|clause| &clause.expr).collect();
        let path_dims = self.path_dimensions();
        let has_path = !path_dims.is_empty();
        let has_susp = exprs.iter().any(|expr| matches!(expr, Expr::Susp(_)));
        let has_low_dim_paths = has_path && path_dims.iter().all(|dimension| *dimension <= 1);
        let requires_formation = has_path || has_susp;
        let has_explicit_formation = exprs.iter().any(|expr| is_concrete_type_formation(expr))
            || has_susp
            || (has_low_dim_paths && exprs.iter().any(|expr| matches!(expr, Expr::Trunc(_))));

        usize::from(requires_formation && !has_explicit_formation)
    }

    fn is_trunc_only_higher_path_hybrid(&self) -> bool {
        let has_higher_path = self
            .clauses
            .iter()
            .any(|clause| matches!(clause.expr, Expr::PathCon(dimension) if dimension > 1));
        let non_path_exprs: Vec<&Expr> = self
            .clauses
            .iter()
            .filter_map(|clause| match clause.expr {
                Expr::PathCon(_) => None,
                ref expr => Some(expr),
            })
            .collect();

        has_higher_path
            && !non_path_exprs.is_empty()
            && non_path_exprs.into_iter().all(|expr| expr.is_trunc_context())
    }
}

fn clause(expr: Expr) -> ClauseRec {
    ClauseRec::new(primary_role(&expr), expr)
}

fn desugared_roles(expr: &Expr) -> Vec<ClauseRole> {
    match expr {
        Expr::Susp(_) => vec![
            ClauseRole::Formation,
            ClauseRole::Introduction,
            ClauseRole::Introduction,
            ClauseRole::PathAttach,
        ],
        Expr::Univ | Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Id(_, _, _) => {
            vec![ClauseRole::Formation]
        }
        Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => vec![ClauseRole::Formation],
        Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => vec![ClauseRole::Introduction],
        Expr::App(left, _) if matches!(left.as_ref(), Expr::Lib(_)) => {
            vec![ClauseRole::Introduction]
        }
        Expr::App(left, _) if matches!(left.as_ref(), Expr::Lam(_)) => {
            vec![ClauseRole::Elimination]
        }
        Expr::App(_, _) => vec![ClauseRole::Introduction],
        Expr::PathCon(_) => vec![ClauseRole::PathAttach],
        Expr::Trunc(_)
        | Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_)
        | Expr::Lib(_) => vec![ClauseRole::Formation],
    }
}

fn primary_role(expr: &Expr) -> ClauseRole {
    desugared_roles(expr)
        .into_iter()
        .next()
        .unwrap_or(ClauseRole::Formation)
}

fn is_concrete_type_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ)
        || matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
}

fn is_basic_formation_entry(expr: &&Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Var(_))
        || matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
}

fn is_formation_only(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Susp(_))
        || matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
}

fn classify_single_entry(expr: &Expr) -> TelescopeClass {
    match expr {
        Expr::Univ => TelescopeClass::Foundation,
        Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => TelescopeClass::Foundation,
        Expr::Susp(_) => TelescopeClass::Suspension,
        _ => TelescopeClass::Unknown,
    }
}

fn is_pi_sigma_expr(expr: &&Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Lam(_) | Expr::App(_, _)
    )
}

#[cfg(test)]
mod tests {
    use super::{Telescope, TelescopeClass};

    #[test]
    fn reference_telescope_metrics_match_expected_shapes() {
        let s3 = Telescope::reference(8);
        assert_eq!(s3.kappa(), 5);
        assert_eq!(s3.path_dimensions(), vec![3]);
        assert!(s3.has_loop());
    }

    #[test]
    fn hopf_references_the_expected_library_entries() {
        let hopf = Telescope::reference(9);
        let refs: Vec<_> = hopf.lib_refs().into_iter().collect();
        assert_eq!(refs, vec![5, 7, 8]);
    }

    #[test]
    fn cohesion_and_dct_classify_structurally() {
        let empty = Vec::new();
        assert_eq!(
            Telescope::reference(10).classify(&empty),
            TelescopeClass::Modal
        );
        assert_eq!(
            Telescope::reference(15).classify(&empty),
            TelescopeClass::Synthesis
        );
    }
}
