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
    pub library_refs: u16,
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
        let library_refs =
            u16::try_from(telescope.lib_refs().len()).expect("library ref count exceeded u16");
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
            has_modal_ops: exprs.iter().any(|expr| contains_modal_expr(expr)),
            has_differential_ops: matches!(class, TelescopeClass::Axiomatic),
            has_curvature: exprs.iter().any(is_curvature_expr),
            has_metric: exprs.iter().any(is_metric_expr),
            has_hilbert: matches_hilbert_functional_shell(telescope, library),
            has_temporal_ops: exprs.iter().any(|expr| expr.is_temporal()),
            has_temporal_shell: matches_temporal_cohesive_shell(telescope, library),
        };

        Self {
            constructors: constructors as u16,
            path_dims,
            has_loop,
            is_truncated,
            axiomatic_exports,
            library_refs,
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

fn contains_modal_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Flat(_) | Expr::Sharp(_) | Expr::Disc(_) | Expr::Shape(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            contains_modal_expr(left) || contains_modal_expr(right)
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => contains_modal_expr(body),
        Expr::Id(ty, left, right) => {
            contains_modal_expr(ty) || contains_modal_expr(left) || contains_modal_expr(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
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

fn matches_hilbert_functional_shell(telescope: &Telescope, library: &Library) -> bool {
    let latest = library.len() as u32;
    if latest < 3 {
        return false;
    }
    let previous = latest - 1;
    let older = latest - 2;

    telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 9
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Sigma(left, right)
                if matches!(
                    left.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(
                                codomain.as_ref(),
                                Expr::Pi(inner_domain, inner_codomain)
                                    if matches!(inner_domain.as_ref(), Expr::Var(1))
                                        && matches!(inner_codomain.as_ref(), Expr::Univ)
                            )
                ) && matches!(right.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Sigma(left, right)
                            if matches!(left.as_ref(), Expr::Var(1))
                                && matches!(right.as_ref(), Expr::Var(1))
                    )
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1)))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Sigma(left, right)
                            if matches!(left.as_ref(), Expr::Var(1))
                                && matches!(right.as_ref(), Expr::Var(2))
                    )
        )
        && matches!(
            &telescope.clauses[4].expr,
            Expr::Sigma(left, right)
                if matches!(
                    left.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                ) && matches!(
                    right.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                )
        )
        && matches!(
            &telescope.clauses[5].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[6].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == previous)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[7].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == older)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[8].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Univ)
                )
        )
}

fn matches_temporal_cohesive_shell(telescope: &Telescope, library: &Library) -> bool {
    let Some(anchor) = latest_modal_shell_anchor_ref(library) else {
        return false;
    };
    if !library.iter().any(|entry| entry.capabilities.has_hilbert) {
        return false;
    }

    telescope.path_dimensions().is_empty()
        && telescope.clauses.len() == 8
        && matches!(
            &telescope.clauses[0].expr,
            Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[1].expr,
            Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
        )
        && matches!(
            &telescope.clauses[2].expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
                    )
        )
        && matches!(
            &telescope.clauses[3].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                            && matches!(
                                argument.as_ref(),
                                Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                            )
                )
        )
        && matches!(
            &telescope.clauses[4].expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Flat(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Next(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                )
        )
        && matches!(
            &telescope.clauses[5].expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Sharp(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Eventually(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                )
        )
        && matches!(
            &telescope.clauses[6].expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        )
        && matches!(
            &telescope.clauses[7].expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Next(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))
                )
        )
}

fn latest_modal_shell_anchor_ref(library: &Library) -> Option<u32> {
    library.iter().enumerate().rev().find_map(|(index, entry)| {
        (entry.class == TelescopeClass::Modal && entry.capabilities.has_modal_ops)
            .then_some(index as u32 + 1)
    })
}

#[cfg(test)]
mod tests {
    use super::LibraryEntry;
    use crate::library::Library;
    use crate::telescope::{Telescope, TelescopeClass};

    #[test]
    fn modal_and_temporal_capabilities_are_detected_structurally() {
        let empty = Vec::new();
        let cohesion = LibraryEntry::from_telescope(&Telescope::reference(10), &empty);
        assert_eq!(cohesion.class, TelescopeClass::Modal);
        assert!(cohesion.capabilities.has_modal_ops);
        assert_eq!(cohesion.library_refs, 0);

        let connections = LibraryEntry::from_telescope(&Telescope::reference(11), &empty);
        assert_eq!(connections.class, TelescopeClass::Axiomatic);
        assert!(connections.capabilities.has_modal_ops);
        assert_eq!(connections.library_refs, 1);

        let curvature = LibraryEntry::from_telescope(&Telescope::reference(12), &empty);
        assert_eq!(curvature.class, TelescopeClass::Axiomatic);
        assert!(curvature.capabilities.has_curvature);

        let dct = LibraryEntry::from_telescope(&Telescope::reference(15), &empty);
        assert_eq!(dct.class, TelescopeClass::Synthesis);
        assert!(dct.capabilities.has_temporal_ops);
        assert!(!dct.capabilities.has_temporal_shell);
    }

    #[test]
    fn genuine_hilbert_marker_stays_closed_until_the_post_metric_shell() {
        let mut library: Library = Vec::new();
        for step in 1..=12 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let metric = LibraryEntry::from_telescope(&Telescope::reference(13), &library);
        assert!(metric.capabilities.has_metric);
        assert!(!metric.capabilities.has_hilbert);
        library.push(metric);

        let hilbert = LibraryEntry::from_telescope(&Telescope::reference(14), &library);
        assert!(hilbert.capabilities.has_metric);
        assert!(hilbert.capabilities.has_hilbert);
    }

    #[test]
    fn genuine_temporal_marker_stays_closed_until_the_post_hilbert_shell() {
        let mut library: Library = Vec::new();
        for step in 1..=14 {
            let telescope = Telescope::reference(step);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }

        let dct = LibraryEntry::from_telescope(&Telescope::reference(15), &library);
        assert!(dct.capabilities.has_temporal_ops);
        assert!(dct.capabilities.has_temporal_shell);
    }
}
