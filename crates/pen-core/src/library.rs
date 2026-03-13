use crate::capability::CapabilityFlags;
use crate::expr::Expr;
use crate::telescope::{Telescope, TelescopeClass};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct LibraryEntry {
    pub constructors: u16,
    pub path_dims: Vec<u32>,
    pub has_loop: bool,
    pub is_truncated: Option<u8>,
    pub axiomatic_exports: u16,
    pub capabilities: CapabilityFlags,
    pub class: TelescopeClass,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct LibrarySnapshot {
    pub window_depth: u16,
    pub entries: Vec<LibrarySnapshotEntry>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct LibrarySnapshotEntry {
    pub step: u32,
    pub candidate_hash: String,
    pub telescope: Telescope,
}

pub type Library = Vec<LibraryEntry>;

impl LibraryEntry {
    pub fn from_telescope(telescope: &Telescope, library: &Library) -> Self {
        let exprs: Vec<&Expr> = telescope
            .clauses
            .iter()
            .map(|clause| &clause.expr)
            .collect();
        let path_dims = telescope.path_dimensions();
        let has_loop = !path_dims.is_empty();
        let is_truncated = find_truncation(&exprs);

        let has_type_formation = exprs
            .iter()
            .any(|expr| matches!(expr, Expr::Univ) || is_univ_application(expr));
        let has_suspension = exprs.iter().any(|expr| matches!(expr, Expr::Susp(_)));
        let has_lib_operation = exprs.iter().any(is_lib_operation);
        let has_operator_top = exprs.iter().any(is_operator_top);

        let point_count = if has_lib_operation {
            exprs
                .iter()
                .filter(|expr| matches!(expr, Expr::Var(_)))
                .count()
        } else {
            exprs.iter().filter(|expr| is_point_term(expr)).count()
        };

        let constructors = if has_type_formation || has_suspension {
            point_count.max(1)
        } else if has_lib_operation || has_operator_top {
            0
        } else {
            point_count
        };

        let class = telescope.classify(library);
        let axiomatic_exports = export_weight(constructors, &path_dims, has_loop, is_truncated);
        let capabilities = CapabilityFlags {
            constructors: constructors > 0,
            path_dimensions: path_dims
                .iter()
                .copied()
                .map(|dimension| dimension as u16)
                .collect(),
            has_loop,
            is_truncated: is_truncated.is_some(),
            axiomatic_exports: usize::from(axiomatic_exports),
            has_dependent_functions: matches!(class, TelescopeClass::Former),
            has_modal_ops: matches!(class, TelescopeClass::Modal),
            has_differential_ops: matches!(class, TelescopeClass::Axiomatic),
            has_curvature: exprs.iter().any(is_curvature_expr),
            has_metric: exprs.iter().any(is_metric_expr),
            has_hilbert: exprs.iter().any(is_hilbert_expr),
            has_temporal_ops: exprs.iter().any(|expr| expr.is_temporal()),
        };

        Self {
            constructors: constructors as u16,
            path_dims,
            has_loop,
            is_truncated,
            axiomatic_exports,
            capabilities,
            class,
        }
    }
}

fn find_truncation(exprs: &[&Expr]) -> Option<u8> {
    if exprs.iter().any(|expr| matches!(expr, Expr::Trunc(_))) {
        Some(0)
    } else {
        None
    }
}

fn export_weight(
    constructors: usize,
    path_dims: &[u32],
    has_loop: bool,
    is_truncated: Option<u8>,
) -> u16 {
    let mut total = constructors + path_dims.len();
    if has_loop {
        total += 1;
    }
    if is_truncated.is_some() {
        total += 1;
    }
    total.max(1) as u16
}

fn is_univ_application(expr: &&Expr) -> bool {
    match expr {
        Expr::App(left, _) => matches!(left.as_ref(), Expr::Univ),
        _ => false,
    }
}

fn is_lib_operation(expr: &&Expr) -> bool {
    match expr {
        Expr::Pi(left, right) => {
            matches!(left.as_ref(), Expr::Lib(_)) || matches!(right.as_ref(), Expr::Lib(_))
        }
        _ => false,
    }
}

fn is_operator_top(expr: &&Expr) -> bool {
    matches!(
        expr,
        Expr::Flat(_)
            | Expr::Sharp(_)
            | Expr::Disc(_)
            | Expr::Shape(_)
            | Expr::Next(_)
            | Expr::Eventually(_)
            | Expr::Trunc(_)
    )
}

fn is_point_term(expr: &&Expr) -> bool {
    match expr {
        Expr::Var(_) => true,
        Expr::App(left, right) => {
            matches!(left.as_ref(), Expr::Lib(_)) && matches!(right.as_ref(), Expr::Var(_))
        }
        _ => false,
    }
}

fn is_curvature_expr(expr: &&Expr) -> bool {
    match expr {
        Expr::Pi(left, _) | Expr::App(left, _) => matches!(left.as_ref(), Expr::Lib(11)),
        _ => false,
    }
}

fn is_metric_expr(expr: &&Expr) -> bool {
    match expr {
        Expr::Sigma(_, _) => true,
        Expr::Pi(left, _) => matches!(left.as_ref(), Expr::Lib(12)),
        _ => false,
    }
}

fn is_hilbert_expr(expr: &&Expr) -> bool {
    match expr {
        Expr::Sigma(_, _) => true,
        Expr::Pi(left, _) => matches!(left.as_ref(), Expr::Lib(13) | Expr::Lib(12)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::LibraryEntry;
    use crate::telescope::{Telescope, TelescopeClass};

    #[test]
    fn modal_and_temporal_capabilities_are_detected_structurally() {
        let empty = Vec::new();
        let cohesion = LibraryEntry::from_telescope(&Telescope::reference(10), &empty);
        assert_eq!(cohesion.class, TelescopeClass::Modal);
        assert!(cohesion.capabilities.has_modal_ops);

        let dct = LibraryEntry::from_telescope(&Telescope::reference(15), &empty);
        assert_eq!(dct.class, TelescopeClass::Synthesis);
        assert!(dct.capabilities.has_temporal_ops);
    }
}
