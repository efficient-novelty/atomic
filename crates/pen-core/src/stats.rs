use crate::expr::Expr;
use crate::telescope::Telescope;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StructuralStats {
    pub node_count: u32,
    pub max_depth: u16,
    pub lib_refs: u32,
    pub var_refs: u32,
}

impl StructuralStats {
    pub fn from_expr(expr: &Expr) -> Self {
        let (node_count, max_depth) = walk(expr, 1);
        Self {
            node_count,
            max_depth,
            lib_refs: expr.lib_refs().len() as u32,
            var_refs: expr.var_refs().len() as u32,
        }
    }

    pub fn from_telescope(telescope: &Telescope) -> Self {
        telescope
            .clauses
            .iter()
            .fold(Self::default(), |mut stats, clause| {
                let next = Self::from_expr(&clause.expr);
                stats.node_count += next.node_count;
                stats.max_depth = stats.max_depth.max(next.max_depth);
                stats.lib_refs += next.lib_refs;
                stats.var_refs += next.var_refs;
                stats
            })
    }
}

fn walk(expr: &Expr, depth: u16) -> (u32, u16) {
    match expr {
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            let (left_count, left_depth) = walk(left, depth + 1);
            let (right_count, right_depth) = walk(right, depth + 1);
            (1 + left_count + right_count, left_depth.max(right_depth))
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => {
            let (count, max_depth) = walk(body, depth + 1);
            (1 + count, max_depth)
        }
        Expr::Id(a, x, y) => {
            let (a_count, a_depth) = walk(a, depth + 1);
            let (x_count, x_depth) = walk(x, depth + 1);
            let (y_count, y_depth) = walk(y, depth + 1);
            (
                1 + a_count + x_count + y_count,
                a_depth.max(x_depth).max(y_depth),
            )
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => (1, depth),
    }
}
