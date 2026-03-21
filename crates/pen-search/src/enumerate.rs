use pen_core::atom::Atom;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::encode::expr_bit_length;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::Telescope;
use pen_type::admissibility::{AdmissibilityMode, StrictAdmissibility, StructuralFamily};
use pen_type::check::{CheckResult, check_telescope};
use pen_type::connectivity::passes_connectivity;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LateFamilySurface {
    None,
    RealisticShadow,
    ClaimGeneric,
    DemoBreadthShadow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnumerationContext {
    pub library_size: u32,
    pub scope_size: u32,
    pub max_path_dimension: u32,
    pub include_trunc: bool,
    pub include_modal: bool,
    pub include_temporal: bool,
    pub max_expr_nodes: u8,
    pub require_former_eliminator_clauses: bool,
    pub require_initial_hit_clauses: bool,
    pub require_truncation_hit_clauses: bool,
    pub require_higher_hit_clauses: bool,
    pub require_sphere_lift_clauses: bool,
    pub require_axiomatic_bundle_clauses: bool,
    pub require_modal_shell_clauses: bool,
    pub require_connection_shell_clauses: bool,
    pub require_curvature_shell_clauses: bool,
    pub require_operator_bundle_clauses: bool,
    pub require_hilbert_functional_clauses: bool,
    pub require_temporal_shell_clauses: bool,
    pub historical_anchor_ref: Option<u32>,
    pub late_family_surface: LateFamilySurface,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TelescopeEnumeration {
    pub telescopes: Vec<Telescope>,
    pub terminal_prefixes: Vec<Telescope>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClauseCatalog {
    clause_kappa: u16,
    options_by_position: Vec<Vec<ClauseRec>>,
}

impl ClauseCatalog {
    pub fn clause_kappa(&self) -> u16 {
        self.clause_kappa
    }

    pub fn clauses_at(&self, position: usize) -> &[ClauseRec] {
        self.options_by_position
            .get(position)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn is_empty(&self) -> bool {
        self.options_by_position.is_empty()
    }
}

impl EnumerationContext {
    pub fn from_admissibility(library: &Library, admissibility: StrictAdmissibility) -> Self {
        Self {
            library_size: library.len() as u32,
            scope_size: admissibility.ambient_depth,
            max_path_dimension: admissibility.max_path_dimension,
            include_trunc: admissibility.include_trunc,
            include_modal: admissibility.include_modal,
            include_temporal: admissibility.include_temporal,
            max_expr_nodes: admissibility.max_expr_nodes,
            require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
            require_initial_hit_clauses: admissibility.require_initial_hit_package,
            require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
            require_higher_hit_clauses: admissibility.require_higher_hit_package,
            require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
            require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
            require_modal_shell_clauses: admissibility.require_modal_shell_package,
            require_connection_shell_clauses: admissibility.require_connection_shell_package,
            require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
            require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
            require_hilbert_functional_clauses: admissibility.require_hilbert_functional_package,
            require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
            historical_anchor_ref: admissibility.historical_anchor_ref,
            late_family_surface: match admissibility.mode {
                AdmissibilityMode::RealisticShadow => LateFamilySurface::RealisticShadow,
                AdmissibilityMode::DemoBreadthShadow => LateFamilySurface::DemoBreadthShadow,
                AdmissibilityMode::DesktopClaimShadow => LateFamilySurface::ClaimGeneric,
                AdmissibilityMode::Guarded | AdmissibilityMode::RelaxedShadow => {
                    LateFamilySurface::None
                }
            },
        }
    }
}

pub fn enumerate_next_clauses(context: EnumerationContext) -> Vec<ClauseRec> {
    enumerate_exprs(context)
        .into_iter()
        .filter(|expr| {
            (!context.require_former_eliminator_clauses || supports_former_eliminator_clause(expr))
                && (!context.require_initial_hit_clauses
                    || supports_initial_hit_clause(expr, context.late_family_surface))
                && (!context.require_truncation_hit_clauses
                    || supports_truncation_hit_clause(expr, context.late_family_surface))
                && (!context.require_higher_hit_clauses
                    || supports_higher_hit_clause(expr, context.late_family_surface))
                && (!context.require_sphere_lift_clauses
                    || supports_sphere_lift_clause(expr, context.late_family_surface))
                && (!context.require_axiomatic_bundle_clauses
                    || supports_axiomatic_bundle_clause(expr))
                && (!context.require_modal_shell_clauses || supports_modal_shell_clause(expr))
                && (!context.require_connection_shell_clauses
                    || supports_connection_shell_clause(expr))
                && (!context.require_curvature_shell_clauses
                    || supports_curvature_shell_clause(expr))
                && (!context.require_operator_bundle_clauses
                    || supports_operator_bundle_clause(expr))
                && (!context.require_hilbert_functional_clauses
                    || supports_hilbert_functional_clause(expr))
                && (!context.require_temporal_shell_clauses || supports_temporal_shell_clause(expr))
        })
        .map(|expr| ClauseRec::new(primary_role(&expr), expr))
        .collect()
}

fn enumerate_raw_next_clauses(context: EnumerationContext) -> Vec<ClauseRec> {
    enumerate_exprs_raw(context)
        .into_iter()
        .filter(|expr| {
            (!context.require_former_eliminator_clauses || supports_former_eliminator_clause(expr))
                && (!context.require_initial_hit_clauses
                    || supports_initial_hit_clause(expr, context.late_family_surface))
                && (!context.require_truncation_hit_clauses
                    || supports_truncation_hit_clause(expr, context.late_family_surface))
                && (!context.require_higher_hit_clauses
                    || supports_higher_hit_clause(expr, context.late_family_surface))
                && (!context.require_sphere_lift_clauses
                    || supports_sphere_lift_clause(expr, context.late_family_surface))
                && (!context.require_axiomatic_bundle_clauses
                    || supports_axiomatic_bundle_clause(expr))
                && (!context.require_modal_shell_clauses || supports_modal_shell_clause(expr))
                && (!context.require_connection_shell_clauses
                    || supports_connection_shell_clause(expr))
                && (!context.require_curvature_shell_clauses
                    || supports_curvature_shell_clause(expr))
                && (!context.require_operator_bundle_clauses
                    || supports_operator_bundle_clause(expr))
                && (!context.require_hilbert_functional_clauses
                    || supports_hilbert_functional_clause(expr))
                && (!context.require_temporal_shell_clauses || supports_temporal_shell_clause(expr))
        })
        .map(|expr| ClauseRec::new(primary_role(&expr), expr))
        .collect()
}

fn dedupe_sorted_clauses(clauses: Vec<ClauseRec>) -> Vec<ClauseRec> {
    let mut keyed = BTreeMap::new();
    for clause in clauses {
        keyed
            .entry((
                clause.role as u8,
                expr_sort_key(&clause.expr),
                serde_json::to_string(&clause.expr).expect("expr should serialize"),
            ))
            .or_insert(clause);
    }
    keyed.into_values().collect()
}

fn relaxed_modal_shell_clause(position: usize) -> Option<Expr> {
    Some(match position {
        0 => Expr::Flat(Box::new(Expr::Var(1))),
        1 => Expr::Sharp(Box::new(Expr::Var(1))),
        2 => Expr::Disc(Box::new(Expr::Var(1))),
        3 => Expr::Shape(Box::new(Expr::Var(1))),
        _ => return None,
    })
}

fn relaxed_axiomatic_bridge_clause(position: usize, context: EnumerationContext) -> Option<Expr> {
    let Some(anchor) = context.historical_anchor_ref else {
        return None;
    };
    if context.library_size < 2 {
        return None;
    }

    let latest = context.library_size;
    let previous = latest - 1;
    Some(match position {
        0 => Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
        1 => Expr::App(Box::new(Expr::Lib(anchor)), Box::new(Expr::Var(1))),
        2 => Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Lib(latest)),
            Box::new(Expr::Lib(previous)),
        ))),
        3 => Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
        _ => return None,
    })
}

fn relaxed_connection_shell_clause(position: usize, context: EnumerationContext) -> Option<Expr> {
    if context.library_size == 0 {
        return None;
    }

    let latest = context.library_size;
    Some(match position {
        0 => Expr::Pi(
            Box::new(Expr::Lib(latest)),
            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
        ),
        1 => Expr::Lam(Box::new(Expr::Pi(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Var(2)),
        ))),
        2 => Expr::Pi(
            Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
            Box::new(Expr::Var(1)),
        ),
        3 => Expr::App(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
        4 => Expr::Lam(Box::new(Expr::Var(1))),
        _ => return None,
    })
}

fn relaxed_curvature_shell_clause(position: usize, context: EnumerationContext) -> Option<Expr> {
    if context.library_size == 0 {
        return None;
    }

    let latest = context.library_size;
    Some(match position {
        0 => Expr::Pi(
            Box::new(Expr::Lib(latest)),
            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
        ),
        1 => Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Lib(latest)),
            Box::new(Expr::Var(1)),
        ))),
        2 => Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Lib(latest))),
        3 => Expr::App(
            Box::new(Expr::Lib(latest)),
            Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
        ),
        4 => Expr::Lam(Box::new(Expr::Pi(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Var(2)),
        ))),
        5 => Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(latest))),
        _ => return None,
    })
}

fn claim_bridge_reanchor_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    let Some(anchor) = context.historical_anchor_ref else {
        return Vec::new();
    };
    if context.library_size < 2 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest - 1;
    match position {
        0 => vec![
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(latest))),
        ],
        1 => vec![
            Expr::App(Box::new(Expr::Lib(anchor)), Box::new(Expr::Var(1))),
            Expr::App(Box::new(Expr::Lib(anchor)), Box::new(Expr::Var(2))),
            Expr::App(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
        ],
        2 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::Lib(previous)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(previous)),
                Box::new(Expr::Lib(latest)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(Expr::Var(1)),
            ))),
        ],
        3 => vec![
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(previous))),
        ],
        _ => Vec::new(),
    }
}

fn claim_modal_lift_clauses(position: usize) -> Vec<Expr> {
    match position {
        0 => vec![
            Expr::Flat(Box::new(Expr::Var(1))),
            Expr::Flat(Box::new(Expr::Var(2))),
            Expr::Flat(Box::new(Expr::Shape(Box::new(Expr::Var(1))))),
        ],
        1 => vec![
            Expr::Sharp(Box::new(Expr::Var(1))),
            Expr::Sharp(Box::new(Expr::Var(2))),
            Expr::Sharp(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ],
        2 => vec![
            Expr::Disc(Box::new(Expr::Var(1))),
            Expr::Disc(Box::new(Expr::Var(2))),
            Expr::Disc(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ],
        3 => vec![
            Expr::Shape(Box::new(Expr::Var(1))),
            Expr::Shape(Box::new(Expr::Var(2))),
            Expr::Shape(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
        ],
        _ => Vec::new(),
    }
}

fn claim_structural_shell_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    if context.library_size == 0 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest
        .checked_sub(1)
        .filter(|index| *index > 0)
        .unwrap_or(latest);
    match position {
        0 => vec![
            Expr::Pi(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Lib(previous)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ),
        ],
        1 => vec![
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::Var(1)),
            ))),
            Expr::Lam(Box::new(Expr::Sigma(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            ))),
        ],
        2 => vec![
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(1)),
            ),
            Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Lib(latest))),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Var(2)))),
                Box::new(Expr::Var(1)),
            ),
        ],
        3 => vec![
            Expr::App(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
            Expr::App(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
            Expr::App(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ),
        ],
        4 => vec![
            Expr::Lam(Box::new(Expr::Var(1))),
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ],
        5 => vec![
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
        ],
        _ => Vec::new(),
    }
}

fn claim_generic_band7_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    operator_bundle_reference_clause(position, context)
        .into_iter()
        .collect()
}

fn claim_generic_band8_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    temporal_shell_reference_clause(position, context)
        .into_iter()
        .collect()
}

fn claim_generic_band9_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    hilbert_functional_reference_clause(position, context)
        .into_iter()
        .collect()
}

fn demo_initial_hit_clauses(position: usize) -> Vec<Expr> {
    match position {
        0 => vec![
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::PathCon(1))),
        ],
        1 => vec![
            Expr::Var(1),
            Expr::Var(2),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
        ],
        2 => vec![
            Expr::PathCon(1),
            Expr::Var(1),
            Expr::Lam(Box::new(Expr::PathCon(1))),
        ],
        _ => Vec::new(),
    }
}

fn demo_truncation_hit_clauses(position: usize) -> Vec<Expr> {
    match position {
        0 => vec![
            Expr::Trunc(Box::new(Expr::Var(1))),
            Expr::Trunc(Box::new(Expr::Univ)),
            Expr::Trunc(Box::new(Expr::PathCon(1))),
        ],
        1 => vec![
            Expr::App(
                Box::new(Expr::Trunc(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(2)),
            ),
            Expr::App(
                Box::new(Expr::Trunc(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(1)),
            ),
            Expr::App(
                Box::new(Expr::Trunc(Box::new(Expr::Univ))),
                Box::new(Expr::Var(1)),
            ),
        ],
        2 => vec![
            Expr::PathCon(1),
            Expr::Var(1),
            Expr::Lam(Box::new(Expr::PathCon(1))),
        ],
        _ => Vec::new(),
    }
}

fn demo_higher_hit_clauses(position: usize) -> Vec<Expr> {
    match position {
        0 => vec![
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::PathCon(2))),
        ],
        1 => vec![
            Expr::Var(1),
            Expr::Var(2),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
        ],
        2 => vec![
            Expr::PathCon(2),
            Expr::PathCon(1),
            Expr::Lam(Box::new(Expr::PathCon(2))),
        ],
        _ => Vec::new(),
    }
}

fn demo_sphere_lift_clauses(position: usize) -> Vec<Expr> {
    match position {
        0 => vec![
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::PathCon(3))),
        ],
        1 => vec![
            Expr::Var(1),
            Expr::Var(2),
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
        ],
        2 => vec![Expr::PathCon(3), Expr::PathCon(2), Expr::PathCon(1)],
        3 => vec![
            Expr::Lam(Box::new(Expr::Var(1))),
            Expr::Lam(Box::new(Expr::Var(2))),
            Expr::Lam(Box::new(Expr::PathCon(1))),
        ],
        4 => vec![
            Expr::Lam(Box::new(Expr::Var(2))),
            Expr::Lam(Box::new(Expr::Var(1))),
            Expr::Lam(Box::new(Expr::PathCon(2))),
        ],
        _ => Vec::new(),
    }
}

fn demo_modal_shell_clauses(position: usize) -> Vec<Expr> {
    match position {
        0 => vec![
            Expr::Flat(Box::new(Expr::Var(1))),
            Expr::Flat(Box::new(Expr::Var(2))),
            Expr::Flat(Box::new(Expr::Shape(Box::new(Expr::Var(1))))),
        ],
        1 => vec![
            Expr::Sharp(Box::new(Expr::Var(1))),
            Expr::Sharp(Box::new(Expr::Var(2))),
            Expr::Sharp(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ],
        2 => vec![
            Expr::Disc(Box::new(Expr::Var(1))),
            Expr::Disc(Box::new(Expr::Var(2))),
            Expr::Disc(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ],
        3 => vec![
            Expr::Shape(Box::new(Expr::Var(1))),
            Expr::Shape(Box::new(Expr::Var(2))),
            Expr::Shape(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
        ],
        _ => Vec::new(),
    }
}

fn demo_axiomatic_bridge_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    let Some(anchor) = context.historical_anchor_ref else {
        return Vec::new();
    };
    if context.library_size < 2 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest - 1;
    match position {
        0 => vec![
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(latest))),
        ],
        1 => vec![
            Expr::App(Box::new(Expr::Lib(anchor)), Box::new(Expr::Var(1))),
            Expr::App(Box::new(Expr::Lib(anchor)), Box::new(Expr::Var(2))),
            Expr::App(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
        ],
        2 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::Lib(previous)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(previous)),
                Box::new(Expr::Lib(latest)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(Expr::Var(1)),
            ))),
        ],
        3 => vec![
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(previous))),
        ],
        _ => Vec::new(),
    }
}

fn demo_connection_shell_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    if context.library_size == 0 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest
        .checked_sub(1)
        .filter(|index| *index > 0)
        .unwrap_or(latest);
    let widen = context.library_size >= 11;
    match position {
        0 => {
            let mut clauses = vec![
                Expr::Pi(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ),
                Expr::Pi(
                    Box::new(Expr::Lib(previous)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ),
                Expr::Pi(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                ),
            ];
            if widen {
                clauses.extend([
                    Expr::Pi(
                        Box::new(Expr::Lib(latest)),
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                    ),
                    Expr::Pi(
                        Box::new(Expr::Lib(previous)),
                        Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                    ),
                ]);
            }
            clauses
        }
        1 => {
            let mut clauses = vec![
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                ))),
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(2)),
                    Box::new(Expr::Var(1)),
                ))),
                Expr::Lam(Box::new(Expr::Sigma(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                ))),
            ];
            if widen {
                clauses.extend([
                    Expr::Lam(Box::new(Expr::Pi(
                        Box::new(Expr::Var(1)),
                        Box::new(Expr::Var(1)),
                    ))),
                    Expr::Lam(Box::new(Expr::Sigma(
                        Box::new(Expr::Var(2)),
                        Box::new(Expr::Var(1)),
                    ))),
                ]);
            }
            clauses
        }
        2 => {
            let mut clauses = vec![
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Var(1)),
                ),
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Var(2)))),
                    Box::new(Expr::Var(1)),
                ),
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                ),
            ];
            if widen {
                clauses.extend([
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                        Box::new(Expr::Var(2)),
                    ),
                    Expr::Pi(
                        Box::new(Expr::Flat(Box::new(Expr::Var(2)))),
                        Box::new(Expr::Var(2)),
                    ),
                ]);
            }
            clauses
        }
        3 => {
            let mut clauses = vec![
                Expr::App(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
                Expr::App(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
                Expr::App(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                ),
            ];
            if widen {
                clauses.extend([
                    Expr::App(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(2))),
                    Expr::App(
                        Box::new(Expr::Lib(previous)),
                        Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                    ),
                ]);
            }
            clauses
        }
        4 => {
            let mut clauses = vec![
                Expr::Lam(Box::new(Expr::Var(1))),
                Expr::Lam(Box::new(Expr::Var(2))),
                Expr::Lam(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            ];
            if widen {
                clauses.extend([
                    Expr::Lam(Box::new(Expr::Flat(Box::new(Expr::Var(2))))),
                    Expr::Lam(Box::new(Expr::Pi(
                        Box::new(Expr::Var(1)),
                        Box::new(Expr::Var(1)),
                    ))),
                ]);
            }
            clauses
        }
        _ => Vec::new(),
    }
}

fn demo_curvature_shell_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    if context.library_size == 0 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest
        .checked_sub(1)
        .filter(|index| *index > 0)
        .unwrap_or(latest);
    let widen = context.library_size >= 11;
    match position {
        0 => {
            let mut clauses = vec![
                Expr::Pi(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ),
                Expr::Pi(
                    Box::new(Expr::Lib(previous)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ),
                Expr::Pi(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                ),
            ];
            if widen {
                clauses.extend([
                    Expr::Pi(
                        Box::new(Expr::Lib(latest)),
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                    ),
                    Expr::Pi(
                        Box::new(Expr::Lib(previous)),
                        Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                    ),
                ]);
            }
            clauses
        }
        1 => {
            let mut clauses = vec![
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Var(1)),
                ))),
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(previous)),
                    Box::new(Expr::Var(1)),
                ))),
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                ))),
            ];
            if widen {
                clauses.extend([
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(previous)),
                        Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                    ))),
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(latest)),
                        Box::new(Expr::Var(2)),
                    ))),
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(previous)),
                        Box::new(Expr::Var(2)),
                    ))),
                ]);
            }
            clauses
        }
        2 => {
            let mut clauses = vec![
                Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Lib(latest))),
                Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Lib(latest))),
                Expr::Pi(
                    Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                    Box::new(Expr::Lib(latest)),
                ),
            ];
            if widen {
                clauses.extend([
                    Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Lib(previous))),
                    Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Lib(previous))),
                    Expr::Pi(
                        Box::new(Expr::Sigma(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                        Box::new(Expr::Lib(previous)),
                    ),
                ]);
            }
            clauses
        }
        3 => {
            let mut clauses = vec![
                Expr::App(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                ),
                Expr::App(
                    Box::new(Expr::Lib(previous)),
                    Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                ),
                Expr::App(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                ),
            ];
            if widen {
                clauses.extend([
                    Expr::App(
                        Box::new(Expr::Lib(previous)),
                        Box::new(Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                    ),
                    Expr::App(
                        Box::new(Expr::Lib(latest)),
                        Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    ),
                ]);
            }
            clauses
        }
        4 => {
            let mut clauses = vec![
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                ))),
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(2)),
                    Box::new(Expr::Var(1)),
                ))),
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Lib(latest)),
                ))),
            ];
            if widen {
                clauses.extend([
                    Expr::Lam(Box::new(Expr::Pi(
                        Box::new(Expr::Var(1)),
                        Box::new(Expr::Var(1)),
                    ))),
                    Expr::Lam(Box::new(Expr::Pi(
                        Box::new(Expr::Var(2)),
                        Box::new(Expr::Var(2)),
                    ))),
                ]);
            }
            clauses
        }
        5 => {
            let mut clauses = vec![
                Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(latest))),
                Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
                Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
            ];
            if widen {
                clauses.extend([
                    Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(previous))),
                    Expr::Pi(
                        Box::new(Expr::Lib(previous)),
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    ),
                ]);
            }
            clauses
        }
        _ => Vec::new(),
    }
}

fn operator_bundle_reference_clause(position: usize, context: EnumerationContext) -> Option<Expr> {
    if context.library_size < 2 {
        return None;
    }

    let latest = context.library_size;
    let previous = latest - 1;
    Some(match position {
        0 => Expr::Sigma(
            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
        ),
        1 => Expr::Pi(
            Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            Box::new(Expr::Lib(previous)),
        ),
        2 => Expr::Pi(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
        ),
        3 => Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Var(2)),
        ))),
        4 => Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(latest))),
        5 => Expr::Lam(Box::new(Expr::Pi(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Var(1)),
        ))),
        6 => Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
        _ => return None,
    })
}

fn demo_operator_bundle_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    if context.library_size < 2 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest - 1;
    match position {
        0 => vec![
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
            ),
        ],
        1 => vec![
            Expr::Pi(
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                Box::new(Expr::Lib(previous)),
            ),
            Expr::Pi(
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Lib(previous)),
            ),
            Expr::Pi(
                Box::new(Expr::Sigma(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Lib(latest)),
            ),
            Expr::Pi(
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                Box::new(Expr::Lib(latest)),
            ),
            Expr::Pi(
                Box::new(Expr::Sigma(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Lib(previous)),
            ),
        ],
        2 => vec![
            Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        ],
        3 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Var(1)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(2)),
                Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ))),
        ],
        4 => vec![
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(previous))),
            Expr::Pi(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::Pi(
                    Box::new(Expr::Lib(previous)),
                    Box::new(Expr::Lib(latest)),
                )),
            ),
        ],
        5 => vec![
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(1)),
            ))),
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Var(1)),
            ))),
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ))),
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ))),
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ))),
        ],
        6 => vec![
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
            Expr::Pi(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Lib(previous)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Lib(latest)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        ],
        _ => Vec::new(),
    }
}

fn hilbert_functional_reference_clause(
    position: usize,
    context: EnumerationContext,
) -> Option<Expr> {
    if context.library_size < 3 {
        return None;
    }

    let latest = context.library_size;
    let previous = latest - 1;
    let older = latest - 2;
    Some(match position {
        0 => Expr::Sigma(
            Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
            )),
            Box::new(Expr::Var(1)),
        ),
        1 => Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
        2 => Expr::Pi(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
        ),
        3 => Expr::Pi(
            Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
            Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
        ),
        4 => Expr::Sigma(
            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
        ),
        5 => Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
        6 => Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
        7 => Expr::Pi(Box::new(Expr::Lib(older)), Box::new(Expr::Var(1))),
        8 => Expr::Lam(Box::new(Expr::Pi(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Univ),
        ))),
        _ => return None,
    })
}

fn demo_hilbert_functional_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    if context.library_size < 3 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest - 1;
    let older = latest - 2;
    let widen = context.library_size >= 13;
    match position {
        0 => vec![
            Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                )),
                Box::new(Expr::Var(1)),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(2)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                )),
                Box::new(Expr::Var(1)),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Univ))),
                )),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        ],
        1 => vec![
            Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1))),
            Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        ],
        2 => vec![
            Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ),
        ],
        3 => vec![
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ),
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(1)),
                )))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ),
            Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ),
        ],
        4 => vec![
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        ],
        5 => {
            let mut clauses = vec![
                Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
                Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
                Expr::Pi(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ),
            ];
            if widen {
                clauses.push(Expr::Pi(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ));
            }
            clauses
        }
        6 => {
            let mut clauses = vec![
                Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
                Expr::Pi(Box::new(Expr::Lib(older)), Box::new(Expr::Var(1))),
                Expr::Pi(
                    Box::new(Expr::Lib(previous)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ),
            ];
            if widen {
                clauses.push(Expr::Pi(
                    Box::new(Expr::Lib(previous)),
                    Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ));
            }
            clauses
        }
        7 => {
            let mut clauses = vec![
                Expr::Pi(Box::new(Expr::Lib(older)), Box::new(Expr::Var(1))),
                Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
                Expr::Pi(
                    Box::new(Expr::Lib(older)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ),
            ];
            if widen {
                clauses.push(Expr::Pi(
                    Box::new(Expr::Lib(older)),
                    Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                ));
            }
            clauses
        }
        8 => {
            let mut clauses = vec![
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Univ),
                ))),
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(2)),
                    Box::new(Expr::Univ),
                ))),
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                ))),
            ];
            if widen {
                clauses.push(Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(2)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                ))));
            }
            clauses
        }
        _ => Vec::new(),
    }
}

fn temporal_shell_reference_clause(position: usize, context: EnumerationContext) -> Option<Expr> {
    let anchor = context.historical_anchor_ref?;

    Some(match position {
        0 => Expr::Next(Box::new(Expr::Var(1))),
        1 => Expr::Eventually(Box::new(Expr::Var(1))),
        2 => Expr::Pi(
            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
        ),
        3 => Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Lib(anchor)),
            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
        ))),
        4 => Expr::Pi(
            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
        ),
        5 => Expr::Pi(
            Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                Expr::Var(1),
            ))))),
            Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                Expr::Var(1),
            ))))),
        ),
        6 => Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
            Box::new(Expr::Var(2)),
        ))),
        7 => Expr::Pi(
            Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
        ),
        _ => return None,
    })
}

fn demo_temporal_shell_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    let Some(anchor) = context.historical_anchor_ref else {
        return Vec::new();
    };

    match position {
        0 => vec![
            Expr::Next(Box::new(Expr::Var(1))),
            Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
            Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
        ],
        1 => vec![
            Expr::Eventually(Box::new(Expr::Var(1))),
            Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            Expr::Eventually(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        ],
        2 => vec![
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Var(1),
                ))))),
            ),
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                Box::new(Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(
                    1,
                )))))),
            ),
        ],
        3 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Var(1)))))),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(
                    1,
                )))))),
            ))),
        ],
        4 => vec![
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                    Box::new(Expr::Var(1)),
                )))))),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Next(
                    Box::new(Expr::Var(1)),
                )))))),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                    Box::new(Expr::Var(1)),
                )))))),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                    Box::new(Expr::Var(1)),
                )))))),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                    Box::new(Expr::Var(1)),
                )))))),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                    Box::new(Expr::Var(1)),
                )))))),
            ),
        ],
        5 => vec![
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Var(1),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Var(1),
                ))))),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Var(1),
                ))))),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Var(1),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Eventually(Box::new(Expr::Var(1))),
                ))))),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Var(1),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Next(Box::new(Expr::Var(1))),
                ))))),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Flat(Box::new(Expr::Var(1))),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Var(1),
                ))))),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Var(1),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Flat(Box::new(Expr::Var(1))),
                ))))),
            ),
        ],
        6 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Var(1),
                ))))),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(
                    1,
                )))))),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(
                    1,
                )))))),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Next(Box::new(Expr::Var(1))),
                ))))),
                Box::new(Expr::Var(2)),
            ))),
        ],
        7 => vec![
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            ),
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Flat(
                    Box::new(Expr::Var(1)),
                )))))),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
            ),
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Sharp(
                    Box::new(Expr::Var(1)),
                )))))),
                Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Var(1)))))),
            ),
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Next(
                    Box::new(Expr::Var(1)),
                )))))),
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
            ),
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(
                    Expr::Eventually(Box::new(Expr::Var(1))),
                ))))),
                Box::new(Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(
                    1,
                )))))),
            ),
        ],
        _ => Vec::new(),
    }
}

fn operator_bundle_family_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    let Some(reference) = operator_bundle_reference_clause(position, context) else {
        return Vec::new();
    };
    match context.late_family_surface {
        LateFamilySurface::None => vec![reference],
        LateFamilySurface::RealisticShadow if position == 5 => vec![
            reference,
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Var(1)),
            ))),
        ],
        LateFamilySurface::RealisticShadow => vec![reference],
        LateFamilySurface::ClaimGeneric => claim_generic_band7_clauses(position, context),
        LateFamilySurface::DemoBreadthShadow => demo_operator_bundle_clauses(position, context),
    }
}

fn hilbert_functional_family_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    let Some(reference) = hilbert_functional_reference_clause(position, context) else {
        return Vec::new();
    };
    match context.late_family_surface {
        LateFamilySurface::None => vec![reference],
        LateFamilySurface::RealisticShadow if context.library_size >= 3 && position == 8 => vec![
            reference,
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Univ),
            ))),
        ],
        LateFamilySurface::RealisticShadow => vec![reference],
        LateFamilySurface::ClaimGeneric => claim_generic_band9_clauses(position, context),
        LateFamilySurface::DemoBreadthShadow => demo_hilbert_functional_clauses(position, context),
    }
}

fn temporal_shell_family_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    let Some(reference) = temporal_shell_reference_clause(position, context) else {
        return Vec::new();
    };
    match context.late_family_surface {
        LateFamilySurface::None => vec![reference],
        LateFamilySurface::RealisticShadow if position == 4 => vec![
            reference,
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                    Box::new(Expr::Var(1)),
                )))))),
            ),
        ],
        LateFamilySurface::RealisticShadow => vec![reference],
        LateFamilySurface::ClaimGeneric => claim_generic_band8_clauses(position, context),
        LateFamilySurface::DemoBreadthShadow => demo_temporal_shell_clauses(position, context),
    }
}

fn late_clause_options(
    position: usize,
    context: EnumerationContext,
    clause_kappa: u16,
) -> Option<Vec<ClauseRec>> {
    if clause_kappa == 4
        && !context.include_modal
        && !context.require_axiomatic_bundle_clauses
        && context.historical_anchor_ref.is_some()
    {
        let mut clauses = Vec::new();
        match context.late_family_surface {
            LateFamilySurface::DemoBreadthShadow => {
                clauses.extend(
                    demo_axiomatic_bridge_clauses(position, context)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
            }
            LateFamilySurface::ClaimGeneric => {
                clauses.extend(
                    claim_bridge_reanchor_clauses(position, context)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
            }
            LateFamilySurface::None | LateFamilySurface::RealisticShadow => {
                if let Some(expr) = relaxed_axiomatic_bridge_clause(position, context) {
                    clauses.push(ClauseRec::new(primary_role(&expr), expr));
                }
            }
        }
        return Some(dedupe_sorted_clauses(clauses));
    }

    if clause_kappa == 4
        && context.include_modal
        && !context.require_modal_shell_clauses
        && context.historical_anchor_ref.is_some()
    {
        let mut clauses = Vec::new();
        match context.late_family_surface {
            LateFamilySurface::DemoBreadthShadow => {
                clauses.extend(
                    demo_modal_shell_clauses(position)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
                clauses.extend(
                    demo_axiomatic_bridge_clauses(position, context)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
            }
            LateFamilySurface::ClaimGeneric => {
                clauses.extend(
                    claim_modal_lift_clauses(position)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
                clauses.extend(
                    claim_bridge_reanchor_clauses(position, context)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
            }
            LateFamilySurface::None | LateFamilySurface::RealisticShadow => {
                if let Some(expr) = relaxed_modal_shell_clause(position) {
                    clauses.push(ClauseRec::new(primary_role(&expr), expr));
                }
                if let Some(expr) = relaxed_axiomatic_bridge_clause(position, context) {
                    clauses.push(ClauseRec::new(primary_role(&expr), expr));
                }
            }
        }
        return Some(dedupe_sorted_clauses(clauses));
    }

    if matches!(clause_kappa, 5 | 6)
        && context.include_modal
        && !context.require_connection_shell_clauses
        && !context.require_curvature_shell_clauses
        && context.max_expr_nodes == 5
    {
        let mut clauses = Vec::new();
        match context.late_family_surface {
            LateFamilySurface::DemoBreadthShadow => {
                clauses.extend(
                    demo_connection_shell_clauses(position, context)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
                clauses.extend(
                    demo_curvature_shell_clauses(position, context)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
            }
            LateFamilySurface::ClaimGeneric => {
                clauses.extend(
                    claim_structural_shell_clauses(position, context)
                        .into_iter()
                        .map(|expr| ClauseRec::new(primary_role(&expr), expr)),
                );
            }
            LateFamilySurface::None | LateFamilySurface::RealisticShadow => {
                if let Some(expr) = relaxed_connection_shell_clause(position, context) {
                    clauses.push(ClauseRec::new(primary_role(&expr), expr));
                }
                if let Some(expr) = relaxed_curvature_shell_clause(position, context) {
                    clauses.push(ClauseRec::new(primary_role(&expr), expr));
                }
            }
        }
        return Some(dedupe_sorted_clauses(clauses));
    }

    if clause_kappa == 7 && context.max_expr_nodes >= 7 && context.max_path_dimension == 0 {
        let clauses = match context.late_family_surface {
            LateFamilySurface::ClaimGeneric => claim_generic_band7_clauses(position, context),
            _ => operator_bundle_family_clauses(position, context),
        }
        .into_iter()
        .map(|expr| ClauseRec::new(primary_role(&expr), expr))
        .collect();
        return Some(dedupe_sorted_clauses(clauses));
    }

    if clause_kappa == 9 && context.max_expr_nodes >= 7 && context.max_path_dimension == 0 {
        let clauses = match context.late_family_surface {
            LateFamilySurface::ClaimGeneric => claim_generic_band9_clauses(position, context),
            _ => hilbert_functional_family_clauses(position, context),
        }
        .into_iter()
        .map(|expr| ClauseRec::new(primary_role(&expr), expr))
        .collect();
        return Some(dedupe_sorted_clauses(clauses));
    }

    if clause_kappa == 8
        && context.max_expr_nodes >= 7
        && context.max_path_dimension == 0
        && context.include_modal
        && context.include_temporal
        && context.historical_anchor_ref.is_some()
    {
        let clauses = match context.late_family_surface {
            LateFamilySurface::ClaimGeneric => claim_generic_band8_clauses(position, context),
            _ => temporal_shell_family_clauses(position, context),
        }
        .into_iter()
        .map(|expr| ClauseRec::new(primary_role(&expr), expr))
        .collect();
        return Some(dedupe_sorted_clauses(clauses));
    }

    None
}

pub fn enumerate_telescopes(
    library: &Library,
    base_context: EnumerationContext,
    clause_kappa: u16,
) -> Vec<Telescope> {
    enumerate_telescopes_with_terminal_prefixes(library, base_context, clause_kappa).telescopes
}

pub fn enumerate_raw_telescopes(
    base_context: EnumerationContext,
    clause_kappa: u16,
) -> Vec<Telescope> {
    let options_by_position = raw_clause_options_by_position(base_context, clause_kappa);
    if options_by_position.iter().any(Vec::is_empty) {
        return Vec::new();
    }

    let mut telescopes = Vec::new();
    let mut prefix = Vec::new();
    enumerate_raw_telescopes_dfs(
        clause_kappa,
        &options_by_position,
        &mut prefix,
        &mut telescopes,
    );
    telescopes.sort_by_key(|telescope| serde_json::to_string(telescope).expect("serialize"));
    telescopes
}

pub(crate) fn raw_clause_catalog_widths(
    base_context: EnumerationContext,
    clause_kappa: u16,
) -> Vec<usize> {
    raw_clause_options_by_position(base_context, clause_kappa)
        .into_iter()
        .map(|clauses| clauses.len())
        .collect()
}

pub fn enumerate_telescopes_with_terminal_prefixes(
    library: &Library,
    base_context: EnumerationContext,
    clause_kappa: u16,
) -> TelescopeEnumeration {
    let clause_catalog = build_clause_catalog(base_context, clause_kappa);
    if clause_catalog.is_empty() {
        return TelescopeEnumeration::default();
    }

    let mut telescopes = Vec::new();
    let mut terminal_prefixes = Vec::new();
    let mut prefix = Vec::new();
    enumerate_telescopes_dfs(
        library,
        clause_catalog.clause_kappa(),
        &clause_catalog.options_by_position,
        &mut prefix,
        &mut telescopes,
        &mut terminal_prefixes,
    );
    telescopes.sort_by_key(|telescope| serde_json::to_string(telescope).expect("serialize"));
    terminal_prefixes.sort_by_key(|telescope| serde_json::to_string(telescope).expect("serialize"));
    terminal_prefixes.dedup();
    TelescopeEnumeration {
        telescopes,
        terminal_prefixes,
    }
}

pub fn build_clause_catalog(base_context: EnumerationContext, clause_kappa: u16) -> ClauseCatalog {
    if base_context.require_curvature_shell_clauses && clause_kappa < 6 {
        return ClauseCatalog::default();
    }

    let options_by_position = (0..usize::from(clause_kappa))
        .map(|position| clauses_for_position(base_context, clause_kappa, position))
        .collect::<Vec<_>>();

    ClauseCatalog {
        clause_kappa,
        options_by_position,
    }
}

fn clauses_for_position(
    base_context: EnumerationContext,
    clause_kappa: u16,
    position: usize,
) -> Vec<ClauseRec> {
    let clause_context = EnumerationContext {
        scope_size: base_context.scope_size + position as u32,
        ..base_context
    };
    let mut clauses = late_clause_options(position, clause_context, clause_kappa)
        .unwrap_or_else(|| enumerate_next_clauses(clause_context));
    if base_context.require_former_eliminator_clauses {
        clauses.retain(|clause| supports_former_package_clause_at_position(position, &clause.expr));
    }
    if base_context.require_initial_hit_clauses {
        clauses.retain(|clause| {
            supports_initial_hit_clause_at_position(
                position,
                &clause.expr,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_truncation_hit_clauses {
        clauses.retain(|clause| {
            supports_truncation_hit_clause_at_position(
                position,
                &clause.expr,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_higher_hit_clauses {
        clauses.retain(|clause| {
            supports_higher_hit_clause_at_position(
                position,
                &clause.expr,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_sphere_lift_clauses {
        clauses.retain(|clause| {
            supports_sphere_lift_clause_at_position(
                position,
                &clause.expr,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_axiomatic_bundle_clauses {
        clauses.retain(|clause| {
            supports_axiomatic_bundle_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
                base_context.historical_anchor_ref,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_modal_shell_clauses {
        clauses.retain(|clause| supports_modal_shell_clause_at_position(position, &clause.expr));
    }
    if base_context.require_connection_shell_clauses {
        clauses.retain(|clause| {
            supports_connection_shell_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
            )
        });
    }
    if base_context.require_curvature_shell_clauses {
        clauses.retain(|clause| {
            supports_curvature_shell_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
            )
        });
    }
    if base_context.require_operator_bundle_clauses
        || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 7)
    {
        clauses.retain(|clause| {
            supports_operator_bundle_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_hilbert_functional_clauses
        || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 9)
    {
        clauses.retain(|clause| {
            supports_hilbert_functional_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_temporal_shell_clauses
        || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 8)
    {
        clauses.retain(|clause| {
            supports_temporal_shell_clause_at_position(
                position,
                &clause.expr,
                base_context.historical_anchor_ref,
                base_context.late_family_surface,
            )
        });
    }
    dedupe_sorted_clauses(clauses)
}

fn raw_clauses_for_position(
    base_context: EnumerationContext,
    clause_kappa: u16,
    position: usize,
) -> Vec<ClauseRec> {
    let clause_context = EnumerationContext {
        scope_size: base_context
            .scope_size
            .saturating_add(u32::from(clause_kappa).saturating_sub(1)),
        ..base_context
    };
    let mut clauses = late_clause_options(position, clause_context, clause_kappa)
        .unwrap_or_else(|| enumerate_raw_next_clauses(clause_context));
    if base_context.require_former_eliminator_clauses {
        clauses.retain(|clause| supports_former_package_clause_at_position(position, &clause.expr));
    }
    if base_context.require_initial_hit_clauses {
        clauses.retain(|clause| {
            supports_initial_hit_clause_at_position(
                position,
                &clause.expr,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_truncation_hit_clauses {
        clauses.retain(|clause| {
            supports_truncation_hit_clause_at_position(
                position,
                &clause.expr,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_higher_hit_clauses {
        clauses.retain(|clause| {
            supports_higher_hit_clause_at_position(
                position,
                &clause.expr,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_sphere_lift_clauses {
        clauses.retain(|clause| {
            supports_sphere_lift_clause_at_position(
                position,
                &clause.expr,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_axiomatic_bundle_clauses {
        clauses.retain(|clause| {
            supports_axiomatic_bundle_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
                base_context.historical_anchor_ref,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_modal_shell_clauses {
        clauses.retain(|clause| supports_modal_shell_clause_at_position(position, &clause.expr));
    }
    if base_context.require_connection_shell_clauses {
        clauses.retain(|clause| {
            supports_connection_shell_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
            )
        });
    }
    if base_context.require_curvature_shell_clauses {
        clauses.retain(|clause| {
            supports_curvature_shell_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
            )
        });
    }
    if base_context.require_operator_bundle_clauses
        || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 7)
    {
        clauses.retain(|clause| {
            supports_operator_bundle_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_hilbert_functional_clauses
        || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 9)
    {
        clauses.retain(|clause| {
            supports_hilbert_functional_clause_at_position(
                position,
                &clause.expr,
                base_context.library_size,
                base_context.late_family_surface,
            )
        });
    }
    if base_context.require_temporal_shell_clauses
        || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 8)
    {
        clauses.retain(|clause| {
            supports_temporal_shell_clause_at_position(
                position,
                &clause.expr,
                base_context.historical_anchor_ref,
                base_context.late_family_surface,
            )
        });
    }
    clauses
}

fn raw_clause_options_by_position(
    base_context: EnumerationContext,
    clause_kappa: u16,
) -> Vec<Vec<ClauseRec>> {
    (0..usize::from(clause_kappa))
        .map(|position| raw_clauses_for_position(base_context, clause_kappa, position))
        .collect::<Vec<_>>()
}

pub(crate) fn clause_kappa_can_match_structural_family(
    family: StructuralFamily,
    clause_kappa: u16,
) -> bool {
    match family {
        StructuralFamily::FormerEliminator
        | StructuralFamily::InitialHit
        | StructuralFamily::TruncationHit
        | StructuralFamily::HigherHit => clause_kappa == 3,
        StructuralFamily::SphereLift | StructuralFamily::ConnectionShell => clause_kappa == 5,
        StructuralFamily::AxiomaticBundle | StructuralFamily::ModalShell => clause_kappa == 4,
        StructuralFamily::CurvatureShell => clause_kappa == 6,
        StructuralFamily::OperatorBundle => clause_kappa == 7,
        StructuralFamily::TemporalShell => clause_kappa == 8,
        StructuralFamily::HilbertFunctional => clause_kappa == 9,
    }
}

pub(crate) fn clause_supports_structural_family_at_position(
    family: StructuralFamily,
    position: usize,
    clause: &ClauseRec,
    context: EnumerationContext,
) -> bool {
    match family {
        StructuralFamily::FormerEliminator => {
            supports_former_package_clause_at_position(position, &clause.expr)
        }
        StructuralFamily::InitialHit => supports_initial_hit_clause_at_position(
            position,
            &clause.expr,
            context.late_family_surface,
        ),
        StructuralFamily::TruncationHit => supports_truncation_hit_clause_at_position(
            position,
            &clause.expr,
            context.late_family_surface,
        ),
        StructuralFamily::HigherHit => supports_higher_hit_clause_at_position(
            position,
            &clause.expr,
            context.late_family_surface,
        ),
        StructuralFamily::SphereLift => supports_sphere_lift_clause_at_position(
            position,
            &clause.expr,
            context.late_family_surface,
        ),
        StructuralFamily::AxiomaticBundle => supports_axiomatic_bundle_clause_at_position(
            position,
            &clause.expr,
            context.library_size,
            context.historical_anchor_ref,
            context.late_family_surface,
        ),
        StructuralFamily::ModalShell => {
            if context.late_family_surface == LateFamilySurface::DemoBreadthShadow {
                demo_modal_shell_clauses(position)
                    .into_iter()
                    .any(|candidate| candidate == clause.expr)
            } else {
                supports_modal_shell_clause_at_position(position, &clause.expr)
            }
        }
        StructuralFamily::ConnectionShell => {
            if context.late_family_surface == LateFamilySurface::DemoBreadthShadow {
                demo_connection_shell_clauses(position, context)
                    .into_iter()
                    .any(|candidate| candidate == clause.expr)
            } else {
                supports_connection_shell_clause_at_position(
                    position,
                    &clause.expr,
                    context.library_size,
                )
            }
        }
        StructuralFamily::CurvatureShell => {
            if context.late_family_surface == LateFamilySurface::DemoBreadthShadow {
                demo_curvature_shell_clauses(position, context)
                    .into_iter()
                    .any(|candidate| candidate == clause.expr)
            } else {
                supports_curvature_shell_clause_at_position(
                    position,
                    &clause.expr,
                    context.library_size,
                )
            }
        }
        StructuralFamily::OperatorBundle => supports_operator_bundle_clause_at_position(
            position,
            &clause.expr,
            context.library_size,
            context.late_family_surface,
        ),
        StructuralFamily::HilbertFunctional => supports_hilbert_functional_clause_at_position(
            position,
            &clause.expr,
            context.library_size,
            context.late_family_surface,
        ),
        StructuralFamily::TemporalShell => supports_temporal_shell_clause_at_position(
            position,
            &clause.expr,
            context.historical_anchor_ref,
            context.late_family_surface,
        ),
    }
}

pub fn enumerate_exprs(context: EnumerationContext) -> Vec<Expr> {
    let mut cache = BTreeMap::new();
    let mut all = Vec::new();
    for nodes in 1..=context.max_expr_nodes {
        all.extend(enumerate_exprs_exact(context, nodes, &mut cache));
    }
    unique_sorted_exprs(all)
}

fn enumerate_exprs_raw(context: EnumerationContext) -> Vec<Expr> {
    let mut cache = BTreeMap::new();
    let mut all = Vec::new();
    for nodes in 1..=context.max_expr_nodes {
        all.extend(enumerate_exprs_exact_raw(context, nodes, &mut cache));
    }
    all
}

fn enumerate_telescopes_dfs(
    library: &Library,
    remaining: u16,
    clause_options_by_position: &[Vec<ClauseRec>],
    prefix: &mut Vec<ClauseRec>,
    out: &mut Vec<Telescope>,
    terminal_prefixes: &mut Vec<Telescope>,
) {
    if remaining == 0 {
        let telescope = Telescope::new(prefix.clone());
        if check_telescope(library, &telescope) == CheckResult::Ok
            && passes_connectivity(library, &telescope)
        {
            out.push(telescope);
        }
        return;
    }

    let position = prefix.len();
    for clause in &clause_options_by_position[position] {
        prefix.push(clause.clone());
        let partial = Telescope::new(prefix.clone());
        if check_telescope(library, &partial) == CheckResult::Ok {
            if remaining == 2 {
                terminal_prefixes.push(partial.clone());
            }
            enumerate_telescopes_dfs(
                library,
                remaining - 1,
                clause_options_by_position,
                prefix,
                out,
                terminal_prefixes,
            );
        }
        prefix.pop();
    }
}

fn enumerate_raw_telescopes_dfs(
    remaining: u16,
    clause_options_by_position: &[Vec<ClauseRec>],
    prefix: &mut Vec<ClauseRec>,
    out: &mut Vec<Telescope>,
) {
    if remaining == 0 {
        out.push(Telescope::new(prefix.clone()));
        return;
    }

    let position = prefix.len();
    for clause in &clause_options_by_position[position] {
        prefix.push(clause.clone());
        enumerate_raw_telescopes_dfs(remaining - 1, clause_options_by_position, prefix, out);
        prefix.pop();
    }
}

fn enumerate_exprs_exact(
    context: EnumerationContext,
    nodes: u8,
    cache: &mut BTreeMap<u8, Vec<Expr>>,
) -> Vec<Expr> {
    if let Some(cached) = cache.get(&nodes) {
        return cached.clone();
    }

    let mut exprs = Vec::new();

    if nodes == 1 {
        exprs.push(Expr::Univ);
        for index in 1..=context.scope_size {
            exprs.push(Expr::Var(index));
        }
        if context.library_size > 0 {
            let start = context.library_size.saturating_sub(1).max(1);
            let mut refs = BTreeSet::new();
            for index in start..=context.library_size {
                refs.insert(index);
            }
            if let Some(anchor) = context.historical_anchor_ref {
                if (1..=context.library_size).contains(&anchor) {
                    refs.insert(anchor);
                }
            }
            for index in refs {
                exprs.push(Expr::Lib(index));
            }
        }
        for dimension in 1..=context.max_path_dimension {
            exprs.push(Expr::PathCon(dimension));
        }
    }

    if nodes >= 2 {
        let subexprs = enumerate_exprs_exact(context, nodes - 1, cache);
        for body in subexprs {
            exprs.push(Expr::Lam(Box::new(body.clone())));
            if context.include_trunc {
                exprs.push(Expr::Trunc(Box::new(body.clone())));
            }
            if context.include_modal {
                exprs.push(Expr::Flat(Box::new(body.clone())));
                exprs.push(Expr::Sharp(Box::new(body.clone())));
                exprs.push(Expr::Disc(Box::new(body.clone())));
                exprs.push(Expr::Shape(Box::new(body.clone())));
            }
            if context.include_temporal {
                exprs.push(Expr::Next(Box::new(body.clone())));
                exprs.push(Expr::Eventually(Box::new(body)));
            }
        }
    }

    if nodes >= 3 {
        for left_nodes in 1..=nodes - 2 {
            let right_nodes = nodes - 1 - left_nodes;
            let left_exprs = enumerate_exprs_exact(context, left_nodes, cache);
            let right_exprs = enumerate_exprs_exact(context, right_nodes, cache);
            for left in &left_exprs {
                for right in &right_exprs {
                    exprs.push(Expr::App(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Pi(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Sigma(Box::new(left.clone()), Box::new(right.clone())));
                }
            }
        }
    }

    let unique = unique_sorted_exprs(exprs);
    cache.insert(nodes, unique.clone());
    unique
}

fn enumerate_exprs_exact_raw(
    context: EnumerationContext,
    nodes: u8,
    cache: &mut BTreeMap<u8, Vec<Expr>>,
) -> Vec<Expr> {
    if let Some(cached) = cache.get(&nodes) {
        return cached.clone();
    }

    let mut exprs = Vec::new();

    if nodes == 1 {
        exprs.push(Expr::Univ);
        for index in 1..=context.scope_size {
            exprs.push(Expr::Var(index));
        }
        if context.library_size > 0 {
            let start = context.library_size.saturating_sub(1).max(1);
            let mut refs = BTreeSet::new();
            for index in start..=context.library_size {
                refs.insert(index);
            }
            if let Some(anchor) = context.historical_anchor_ref {
                if (1..=context.library_size).contains(&anchor) {
                    refs.insert(anchor);
                }
            }
            for index in refs {
                exprs.push(Expr::Lib(index));
            }
        }
        for dimension in 1..=context.max_path_dimension {
            exprs.push(Expr::PathCon(dimension));
        }
    }

    if nodes >= 2 {
        let subexprs = enumerate_exprs_exact_raw(context, nodes - 1, cache);
        for body in subexprs {
            exprs.push(Expr::Lam(Box::new(body.clone())));
            if context.include_trunc {
                exprs.push(Expr::Trunc(Box::new(body.clone())));
            }
            if context.include_modal {
                exprs.push(Expr::Flat(Box::new(body.clone())));
                exprs.push(Expr::Sharp(Box::new(body.clone())));
                exprs.push(Expr::Disc(Box::new(body.clone())));
                exprs.push(Expr::Shape(Box::new(body.clone())));
            }
            if context.include_temporal {
                exprs.push(Expr::Next(Box::new(body.clone())));
                exprs.push(Expr::Eventually(Box::new(body)));
            }
        }
    }

    if nodes >= 3 {
        for left_nodes in 1..=nodes - 2 {
            let right_nodes = nodes - 1 - left_nodes;
            let left_exprs = enumerate_exprs_exact_raw(context, left_nodes, cache);
            let right_exprs = enumerate_exprs_exact_raw(context, right_nodes, cache);
            for left in &left_exprs {
                for right in &right_exprs {
                    exprs.push(Expr::App(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Pi(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Sigma(Box::new(left.clone()), Box::new(right.clone())));
                }
            }
        }
    }

    cache.insert(nodes, exprs.clone());
    exprs
}

fn unique_sorted_exprs(exprs: Vec<Expr>) -> Vec<Expr> {
    let mut keyed = BTreeMap::new();
    for expr in exprs {
        keyed.entry(expr_sort_key(&expr)).or_insert(expr);
    }
    keyed.into_values().collect()
}

fn primary_role(expr: &Expr) -> ClauseRole {
    match expr {
        Expr::Univ | Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Id(_, _, _) => {
            ClauseRole::Formation
        }
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ) => ClauseRole::Formation,
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)) => {
            ClauseRole::Elimination
        }
        Expr::PathCon(_) => ClauseRole::PathAttach,
        Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => ClauseRole::Introduction,
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Lib(_)) => {
            ClauseRole::Introduction
        }
        Expr::App(_, _) => ClauseRole::Introduction,
        Expr::Susp(_)
        | Expr::Trunc(_)
        | Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_)
        | Expr::Lib(_) => ClauseRole::Formation,
    }
}

fn expr_sort_key(expr: &Expr) -> (u8, u32, String) {
    (
        atom_rank(expr.atom()),
        expr_bit_length(expr),
        serde_json::to_string(expr).expect("expr should serialize"),
    )
}

fn atom_rank(atom: Atom) -> u8 {
    atom as u8
}

fn supports_former_eliminator_clause(expr: &Expr) -> bool {
    !expr.has_lib_pointer()
        && !expr.is_modal()
        && !expr.is_temporal()
        && (contains_former_expr(expr)
            || contains_lambda_expr(expr)
            || contains_eliminator_expr(expr))
}

fn supports_initial_hit_clause(expr: &Expr, late_family_surface: LateFamilySurface) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        return (0..=2).any(|position| {
            demo_initial_hit_clauses(position)
                .into_iter()
                .any(|candidate| candidate == *expr)
        });
    }

    matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Var(_))
        || matches!(expr, Expr::PathCon(1))
}

fn supports_truncation_hit_clause(expr: &Expr, late_family_surface: LateFamilySurface) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        return (0..=2).any(|position| {
            demo_truncation_hit_clauses(position)
                .into_iter()
                .any(|candidate| candidate == *expr)
        });
    }

    matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        || matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(1)))
                    && matches!(argument.as_ref(), Expr::Var(2))
        )
        || matches!(expr, Expr::PathCon(1))
}

fn supports_higher_hit_clause(expr: &Expr, late_family_surface: LateFamilySurface) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        return (0..=2).any(|position| {
            demo_higher_hit_clauses(position)
                .into_iter()
                .any(|candidate| candidate == *expr)
        });
    }

    matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Var(_))
        || matches!(expr, Expr::PathCon(2))
}

fn supports_sphere_lift_clause(expr: &Expr, late_family_surface: LateFamilySurface) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        return (0..=4).any(|position| {
            demo_sphere_lift_clauses(position)
                .into_iter()
                .any(|candidate| candidate == *expr)
        });
    }

    matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Var(_))
        || matches!(expr, Expr::PathCon(3))
        || matches!(
            expr,
            Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1) | Expr::Var(2))
        )
}

fn supports_axiomatic_bundle_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(domain, codomain)
        if matches!(domain.as_ref(), Expr::Lib(_)) && matches!(codomain.as_ref(), Expr::Lib(_)))
        || matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(_))
                    && matches!(argument.as_ref(), Expr::Var(_))
        )
        || matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(_))
                            && matches!(argument.as_ref(), Expr::Lib(_))
                )
        )
}

fn supports_modal_shell_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Flat(body) if matches!(body.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Sharp(body) if matches!(body.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Disc(body) if matches!(body.as_ref(), Expr::Var(_)))
        || matches!(expr, Expr::Shape(body) if matches!(body.as_ref(), Expr::Var(_)))
}

fn supports_connection_shell_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && matches!(
                    codomain.as_ref(),
                    Expr::Pi(inner_domain, inner_codomain)
                        if matches!(inner_domain.as_ref(), Expr::Var(_))
                            && matches!(inner_codomain.as_ref(), Expr::Var(_))
                )
    ) || matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Var(_))
            ) || matches!(body.as_ref(), Expr::Var(_))
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                && matches!(codomain.as_ref(), Expr::Var(_))
    ) || matches!(
        expr,
        Expr::App(function, argument)
            if matches!(function.as_ref(), Expr::Lib(_))
                && matches!(argument.as_ref(), Expr::Var(_))
    )
}

fn supports_curvature_shell_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && matches!(
                    codomain.as_ref(),
                    Expr::Pi(inner_domain, inner_codomain)
                        if matches!(inner_domain.as_ref(), Expr::Var(_))
                            && matches!(inner_codomain.as_ref(), Expr::Var(_))
                )
    ) || matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(_))
                        && matches!(argument.as_ref(), Expr::Var(_))
            ) || matches!(
                body.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Var(_))
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Var(_))
                && matches!(codomain.as_ref(), Expr::Lib(_))
    ) || matches!(
        expr,
        Expr::App(function, argument)
            if matches!(function.as_ref(), Expr::Lib(_))
                && matches!(
                    argument.as_ref(),
                    Expr::App(inner_function, inner_argument)
                        if matches!(inner_function.as_ref(), Expr::Var(_))
                            && matches!(inner_argument.as_ref(), Expr::Var(_))
                )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && matches!(codomain.as_ref(), Expr::Lib(_))
    )
}

fn supports_operator_bundle_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Sigma(left, right)
            if matches!(
                left.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Var(_))
            ) && matches!(
                right.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Var(_))
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sigma(left, right)
                    if matches!(left.as_ref(), Expr::Var(_))
                        && matches!(right.as_ref(), Expr::Var(_))
            ) && matches!(codomain.as_ref(), Expr::Lib(_))
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Var(_))
                && matches!(
                    codomain.as_ref(),
                    Expr::Pi(inner_domain, inner_codomain)
                        if matches!(inner_domain.as_ref(), Expr::Var(_))
                            && matches!(inner_codomain.as_ref(), Expr::Var(_))
                )
    ) || matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Var(_))
                        && matches!(argument.as_ref(), Expr::Var(_))
            ) || matches!(
                body.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Var(_))
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && (matches!(codomain.as_ref(), Expr::Lib(_))
                    || matches!(codomain.as_ref(), Expr::Var(_)))
    )
}

fn supports_hilbert_functional_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Sigma(left, right)
            if matches!(
                left.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(
                            codomain.as_ref(),
                            Expr::Pi(inner_domain, inner_codomain)
                                if matches!(inner_domain.as_ref(), Expr::Var(_))
                                    && matches!(inner_codomain.as_ref(), Expr::Univ)
                        )
            ) && matches!(right.as_ref(), Expr::Var(_))
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Var(_))
                && matches!(codomain.as_ref(), Expr::Var(_) | Expr::Univ)
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Var(_))
                && matches!(
                    codomain.as_ref(),
                    Expr::Sigma(left, right)
                        if matches!(left.as_ref(), Expr::Var(_))
                            && matches!(right.as_ref(), Expr::Var(_))
                )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(_)))
                && matches!(
                    codomain.as_ref(),
                    Expr::Sigma(left, right)
                        if matches!(left.as_ref(), Expr::Var(_))
                            && matches!(right.as_ref(), Expr::Var(_))
                )
    ) || matches!(
        expr,
        Expr::Sigma(left, right)
            if matches!(
                left.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Var(_))
            ) && matches!(
                right.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Var(_))
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && matches!(codomain.as_ref(), Expr::Var(_))
    ) || matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::Pi(domain, codomain)
                    if matches!(domain.as_ref(), Expr::Var(_))
                        && matches!(codomain.as_ref(), Expr::Univ)
            )
    )
}

fn supports_temporal_shell_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Next(body) | Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(_))
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Next(body) if matches!(body.as_ref(), Expr::Var(_))
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(_))
            )
    ) || matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(_))
                        && matches!(
                            argument.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(_))
                        )
            ) || matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(
                        function.as_ref(),
                        Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(_))
                    ) && matches!(argument.as_ref(), Expr::Var(_))
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Flat(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(_))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(_))
                    )
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sharp(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(_))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(_))
                    )
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(_))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body) if matches!(body.as_ref(), Expr::Var(_))
            )
    )
}

fn supports_initial_hit_clause_at_position(
    position: usize,
    expr: &Expr,
    late_family_surface: LateFamilySurface,
) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        return demo_initial_hit_clauses(position)
            .into_iter()
            .any(|candidate| candidate == *expr);
    }

    match position {
        0 => {
            matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(1)))
        }
        1 => matches!(expr, Expr::Var(1)),
        _ => matches!(expr, Expr::PathCon(1)),
    }
}

fn supports_former_package_clause_at_position(position: usize, expr: &Expr) -> bool {
    match position {
        0 => is_minimal_former_intro_clause(expr),
        1 => is_multi_argument_eliminator_clause(expr),
        _ => is_beta_like_computation_clause(expr),
    }
}

fn supports_truncation_hit_clause_at_position(
    position: usize,
    expr: &Expr,
    late_family_surface: LateFamilySurface,
) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        return demo_truncation_hit_clauses(position)
            .into_iter()
            .any(|candidate| candidate == *expr);
    }

    match position {
        0 => matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(1))),
        1 => matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(1)))
                    && matches!(argument.as_ref(), Expr::Var(2))
        ),
        _ => matches!(expr, Expr::PathCon(1)),
    }
}

fn supports_higher_hit_clause_at_position(
    position: usize,
    expr: &Expr,
    late_family_surface: LateFamilySurface,
) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        return demo_higher_hit_clauses(position)
            .into_iter()
            .any(|candidate| candidate == *expr);
    }

    match position {
        0 => {
            matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(1)))
        }
        1 => matches!(expr, Expr::Var(1)),
        _ => matches!(expr, Expr::PathCon(2)),
    }
}

fn supports_sphere_lift_clause_at_position(
    position: usize,
    expr: &Expr,
    late_family_surface: LateFamilySurface,
) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        return demo_sphere_lift_clauses(position)
            .into_iter()
            .any(|candidate| candidate == *expr);
    }

    match position {
        0 => {
            matches!(expr, Expr::App(left, right) if matches!(left.as_ref(), Expr::Univ) && matches!(right.as_ref(), Expr::Var(1)))
        }
        1 => matches!(expr, Expr::Var(1)),
        2 => matches!(expr, Expr::PathCon(3)),
        3 => matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1))),
        _ => matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(2))),
    }
}

fn supports_axiomatic_bundle_clause_at_position(
    position: usize,
    expr: &Expr,
    library_size: u32,
    historical_anchor_ref: Option<u32>,
    late_family_surface: LateFamilySurface,
) -> bool {
    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        let context = EnumerationContext {
            library_size,
            scope_size: 0,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            max_expr_nodes: 0,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            require_curvature_shell_clauses: false,
            require_operator_bundle_clauses: false,
            require_hilbert_functional_clauses: false,
            require_temporal_shell_clauses: false,
            historical_anchor_ref,
            late_family_surface,
        };
        return demo_axiomatic_bridge_clauses(position, context)
            .into_iter()
            .any(|candidate| candidate == *expr);
    }

    let Some(anchor) = historical_anchor_ref else {
        return false;
    };
    if library_size < 2 {
        return false;
    }

    let latest = library_size;
    let previous = latest - 1;

    match position {
        0 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == previous)
        ),
        1 => matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(index) if *index == anchor)
                    && matches!(argument.as_ref(), Expr::Var(1))
        ),
        2 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                            && matches!(argument.as_ref(), Expr::Lib(index) if *index == previous)
                )
        ),
        _ => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == previous)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        ),
    }
}

fn supports_modal_shell_clause_at_position(position: usize, expr: &Expr) -> bool {
    match position {
        0 => matches!(expr, Expr::Flat(body) if matches!(body.as_ref(), Expr::Var(1))),
        1 => matches!(expr, Expr::Sharp(body) if matches!(body.as_ref(), Expr::Var(1))),
        2 => matches!(expr, Expr::Disc(body) if matches!(body.as_ref(), Expr::Var(1))),
        _ => matches!(expr, Expr::Shape(body) if matches!(body.as_ref(), Expr::Var(1))),
    }
}

fn supports_connection_shell_clause_at_position(
    position: usize,
    expr: &Expr,
    library_size: u32,
) -> bool {
    if library_size == 0 {
        return false;
    }

    let latest = library_size;
    match position {
        0 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(
                        codomain.as_ref(),
                        Expr::Pi(inner_domain, inner_codomain)
                            if matches!(inner_domain.as_ref(), Expr::Var(1))
                                && matches!(inner_codomain.as_ref(), Expr::Var(1))
                    )
        ),
        1 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(2))
                )
        ),
        2 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1)))
                    && matches!(codomain.as_ref(), Expr::Var(1))
        ),
        3 => matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(argument.as_ref(), Expr::Var(1))
        ),
        _ => matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1))),
    }
}

fn supports_curvature_shell_clause_at_position(
    position: usize,
    expr: &Expr,
    library_size: u32,
) -> bool {
    if library_size == 0 {
        return false;
    }

    let latest = library_size;
    match position {
        0 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(
                        codomain.as_ref(),
                        Expr::Pi(inner_domain, inner_codomain)
                            if matches!(inner_domain.as_ref(), Expr::Var(1))
                                && matches!(inner_codomain.as_ref(), Expr::Var(1))
                    )
        ),
        1 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                            && matches!(argument.as_ref(), Expr::Var(1))
                )
        ),
        2 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        ),
        3 => matches!(
            expr,
            Expr::App(function, argument)
                if matches!(function.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(
                        argument.as_ref(),
                        Expr::App(inner_function, inner_argument)
                            if matches!(inner_function.as_ref(), Expr::Var(1))
                                && matches!(inner_argument.as_ref(), Expr::Var(2))
                    )
        ),
        4 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(2))
                )
        ),
        _ => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        ),
    }
}

#[cfg_attr(not(test), allow(dead_code))]
fn supports_operator_bundle_clause_at_position(
    position: usize,
    expr: &Expr,
    library_size: u32,
    late_family_surface: LateFamilySurface,
) -> bool {
    if library_size < 2 {
        return false;
    }

    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        let context = EnumerationContext {
            library_size,
            scope_size: 0,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            max_expr_nodes: 0,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            require_curvature_shell_clauses: false,
            require_operator_bundle_clauses: false,
            require_hilbert_functional_clauses: false,
            require_temporal_shell_clauses: false,
            historical_anchor_ref: None,
            late_family_surface,
        };
        return demo_operator_bundle_clauses(position, context)
            .into_iter()
            .any(|candidate| candidate == *expr);
    }

    let latest = library_size;
    let previous = latest - 1;
    if late_family_surface == LateFamilySurface::RealisticShadow && position == 5 {
        return matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1) | Expr::Var(2))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                )
        );
    }

    match position {
        0 => matches!(
            expr,
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
        ),
        1 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Sigma(left, right)
                        if matches!(left.as_ref(), Expr::Var(1))
                            && matches!(right.as_ref(), Expr::Var(2))
                ) && matches!(codomain.as_ref(), Expr::Lib(index) if *index == previous)
        ),
        2 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Pi(inner_domain, inner_codomain)
                            if matches!(inner_domain.as_ref(), Expr::Var(1))
                                && matches!(inner_codomain.as_ref(), Expr::Var(1))
                    )
        ),
        3 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Var(1))
                            && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        4 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Lib(index) if *index == latest)
        ),
        5 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Var(1))
                )
        ),
        _ => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        ),
    }
}

#[cfg_attr(not(test), allow(dead_code))]
fn supports_hilbert_functional_clause_at_position(
    position: usize,
    expr: &Expr,
    library_size: u32,
    late_family_surface: LateFamilySurface,
) -> bool {
    if library_size < 3 {
        return false;
    }

    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        let context = EnumerationContext {
            library_size,
            scope_size: 0,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            max_expr_nodes: 0,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            require_curvature_shell_clauses: false,
            require_operator_bundle_clauses: false,
            require_hilbert_functional_clauses: false,
            require_temporal_shell_clauses: false,
            historical_anchor_ref: None,
            late_family_surface,
        };
        return demo_hilbert_functional_clauses(position, context)
            .into_iter()
            .any(|candidate| candidate == *expr);
    }

    let latest = library_size;
    let previous = latest - 1;
    let older = latest - 2;
    if late_family_surface == LateFamilySurface::RealisticShadow && position == 8 {
        return matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1) | Expr::Var(2))
                            && matches!(codomain.as_ref(), Expr::Univ)
                )
        );
    }

    match position {
        0 => matches!(
            expr,
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
        ),
        1 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(codomain.as_ref(), Expr::Var(1))
        ),
        2 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Var(1))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Sigma(left, right)
                            if matches!(left.as_ref(), Expr::Var(1))
                                && matches!(right.as_ref(), Expr::Var(1))
                    )
        ),
        3 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1)))
                    && matches!(
                        codomain.as_ref(),
                        Expr::Sigma(left, right)
                            if matches!(left.as_ref(), Expr::Var(1))
                                && matches!(right.as_ref(), Expr::Var(2))
                    )
        ),
        4 => matches!(
            expr,
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
        ),
        5 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == latest)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        ),
        6 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == previous)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        ),
        7 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Lib(index) if *index == older)
                    && matches!(codomain.as_ref(), Expr::Var(1))
        ),
        _ => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Pi(domain, codomain)
                        if matches!(domain.as_ref(), Expr::Var(1))
                            && matches!(codomain.as_ref(), Expr::Univ)
                )
        ),
    }
}

#[cfg_attr(not(test), allow(dead_code))]
fn supports_temporal_shell_clause_at_position(
    position: usize,
    expr: &Expr,
    historical_anchor_ref: Option<u32>,
    late_family_surface: LateFamilySurface,
) -> bool {
    let Some(anchor) = historical_anchor_ref else {
        return false;
    };

    if late_family_surface == LateFamilySurface::DemoBreadthShadow {
        let context = EnumerationContext {
            library_size: anchor.max(1),
            scope_size: 0,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: true,
            include_temporal: true,
            max_expr_nodes: 0,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            require_curvature_shell_clauses: false,
            require_operator_bundle_clauses: false,
            require_hilbert_functional_clauses: false,
            require_temporal_shell_clauses: false,
            historical_anchor_ref: Some(anchor),
            late_family_surface,
        };
        return demo_temporal_shell_clauses(position, context)
            .into_iter()
            .any(|candidate| candidate == *expr);
    }

    if late_family_surface == LateFamilySurface::RealisticShadow && position == 4 {
        return matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Flat(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                ) && (
                    matches!(
                        codomain.as_ref(),
                        Expr::Next(body)
                            if matches!(
                                body.as_ref(),
                                Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                            )
                    )
                    || matches!(
                        codomain.as_ref(),
                        Expr::Next(body)
                            if matches!(
                                body.as_ref(),
                                Expr::Flat(inner)
                                    if matches!(
                                        inner.as_ref(),
                                        Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                    )
                            )
                    )
                )
        );
    }

    match position {
        0 => matches!(expr, Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))),
        1 => matches!(expr, Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))),
        2 => matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1))
                ) && matches!(
                    codomain.as_ref(),
                    Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
                )
        ),
        3 => matches!(
            expr,
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
        ),
        4 => matches!(
            expr,
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
        ),
        5 => matches!(
            expr,
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
        ),
        6 => matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(
                            function.as_ref(),
                            Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        ) && matches!(argument.as_ref(), Expr::Var(2))
                )
        ),
        _ => matches!(
            expr,
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
        ),
    }
}

fn is_minimal_former_intro_clause(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(body) => {
            let has_forward_dependency = match body.as_ref() {
                Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
                    max_var_ref(domain) < max_var_ref(codomain)
                }
                _ => false,
            };
            contains_former_expr(body)
                && !contains_lambda_expr(body)
                && distinct_var_ref_count(expr) >= 2
                && has_forward_dependency
        }
        _ => false,
    }
}

fn is_multi_argument_eliminator_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::App(_, _))
        && !contains_former_expr(expr)
        && application_spine_len(expr) >= 3
        && distinct_var_ref_count(expr) >= 3
        && application_head_var(expr) == Some(1)
}

fn is_beta_like_computation_clause(expr: &Expr) -> bool {
    match expr {
        Expr::App(function, argument) => {
            matches!(argument.as_ref(), Expr::Var(index) if *index <= 2)
                && matches!(
                    function.as_ref(),
                    Expr::Lam(body)
                        if !contains_lambda_expr(body)
                            && !contains_former_expr(body)
                            && body.var_refs().len() == 1
                            && body.var_refs().contains(&1)
                )
                && distinct_var_ref_count(expr) >= 2
        }
        _ => false,
    }
}

fn application_spine_len(expr: &Expr) -> usize {
    match expr {
        Expr::App(function, _) => 1 + application_spine_len(function),
        _ => 1,
    }
}

fn distinct_var_ref_count(expr: &Expr) -> usize {
    expr.var_refs().len()
}

fn application_head_var(expr: &Expr) -> Option<u32> {
    match expr {
        Expr::App(function, _) => application_head_var(function),
        Expr::Var(index) => Some(*index),
        _ => None,
    }
}

fn max_var_ref(expr: &Expr) -> u32 {
    expr.var_refs().iter().next_back().copied().unwrap_or(0)
}

fn contains_former_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Pi(_, _) | Expr::Sigma(_, _) => true,
        Expr::App(left, right) => contains_former_expr(left) || contains_former_expr(right),
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => contains_former_expr(body),
        Expr::Id(ty, left, right) => {
            contains_former_expr(ty) || contains_former_expr(left) || contains_former_expr(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn contains_lambda_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            contains_lambda_expr(left) || contains_lambda_expr(right)
        }
        Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => contains_lambda_expr(body),
        Expr::Id(ty, left, right) => {
            contains_lambda_expr(ty) || contains_lambda_expr(left) || contains_lambda_expr(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn contains_eliminator_expr(expr: &Expr) -> bool {
    match expr {
        Expr::App(function, argument) => {
            matches!(function.as_ref(), Expr::Lam(_) | Expr::App(_, _))
                || contains_eliminator_expr(function)
                || contains_eliminator_expr(argument)
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
        | Expr::Eventually(body) => contains_eliminator_expr(body),
        Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            contains_eliminator_expr(left) || contains_eliminator_expr(right)
        }
        Expr::Id(ty, left, right) => {
            contains_eliminator_expr(ty)
                || contains_eliminator_expr(left)
                || contains_eliminator_expr(right)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EnumerationContext, LateFamilySurface, build_clause_catalog, enumerate_exprs,
        enumerate_next_clauses, enumerate_raw_telescopes, enumerate_telescopes,
        raw_clause_catalog_widths, supports_axiomatic_bundle_clause_at_position,
        supports_connection_shell_clause_at_position, supports_curvature_shell_clause_at_position,
        supports_former_package_clause_at_position, supports_higher_hit_clause_at_position,
        supports_hilbert_functional_clause_at_position, supports_initial_hit_clause_at_position,
        supports_modal_shell_clause_at_position, supports_operator_bundle_clause_at_position,
        supports_sphere_lift_clause_at_position, supports_temporal_shell_clause_at_position,
        supports_truncation_hit_clause_at_position,
    };
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;
    use pen_type::admissibility::{
        AdmissibilityMode, StrictAdmissibility, strict_admissibility, strict_admissibility_for_mode,
    };

    fn library_until(step: u32) -> Library {
        let mut library = Vec::new();
        for current in 1..=step {
            let telescope = Telescope::reference(current);
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
        library
    }

    fn context_from_admissibility(
        library: &Library,
        admissibility: StrictAdmissibility,
    ) -> EnumerationContext {
        EnumerationContext::from_admissibility(library, admissibility)
    }

    #[test]
    fn claim_shadow_uses_claim_generic_late_surface() {
        let library = library_until(9);
        let admissibility =
            strict_admissibility_for_mode(10, 2, &library, AdmissibilityMode::DesktopClaimShadow);

        let context = context_from_admissibility(&library, admissibility);

        assert_eq!(context.late_family_surface, LateFamilySurface::ClaimGeneric);
    }

    #[test]
    fn claim_generic_kappa_four_catalog_adds_claim_only_bridge_variants() {
        let library = library_until(9);
        let admissibility =
            strict_admissibility_for_mode(10, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut realistic_context = claim_context;
        realistic_context.late_family_surface = LateFamilySurface::RealisticShadow;

        let claim_catalog = build_clause_catalog(claim_context, 4);
        let realistic_catalog = build_clause_catalog(realistic_context, 4);
        let claim_only = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::App(Box::new(Expr::Lib(7)), Box::new(Expr::Var(2))),
        );

        assert!(claim_catalog.clauses_at(1).contains(&claim_only));
        assert!(!realistic_catalog.clauses_at(1).contains(&claim_only));
    }

    #[test]
    fn claim_generic_kappa_six_catalog_adds_structural_shell_variants() {
        let library = library_until(11);
        let admissibility =
            strict_admissibility_for_mode(12, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut realistic_context = claim_context;
        realistic_context.late_family_surface = LateFamilySurface::RealisticShadow;

        let claim_catalog = build_clause_catalog(claim_context, 6);
        let realistic_catalog = build_clause_catalog(realistic_context, 6);
        let claim_only = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::Sigma(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            ))),
        );

        assert!(claim_catalog.clauses_at(1).contains(&claim_only));
        assert!(!realistic_catalog.clauses_at(1).contains(&claim_only));
    }

    #[test]
    fn claim_generic_late_bands_drop_realistic_only_variants() {
        let step_thirteen_library = library_until(12);
        let step_thirteen = strict_admissibility_for_mode(
            13,
            2,
            &step_thirteen_library,
            AdmissibilityMode::DesktopClaimShadow,
        );
        let claim_thirteen = build_clause_catalog(
            context_from_admissibility(&step_thirteen_library, step_thirteen),
            7,
        );
        let mut realistic_thirteen =
            context_from_admissibility(&step_thirteen_library, step_thirteen);
        realistic_thirteen.late_family_surface = LateFamilySurface::RealisticShadow;
        let realistic_thirteen = build_clause_catalog(realistic_thirteen, 7);
        let realistic_operator_extra = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Var(1)),
            ))),
        );
        assert!(
            !claim_thirteen
                .clauses_at(5)
                .contains(&realistic_operator_extra)
        );
        assert!(
            realistic_thirteen
                .clauses_at(5)
                .contains(&realistic_operator_extra)
        );

        let step_fourteen_library = library_until(13);
        let step_fourteen = strict_admissibility_for_mode(
            14,
            2,
            &step_fourteen_library,
            AdmissibilityMode::DesktopClaimShadow,
        );
        let claim_fourteen = build_clause_catalog(
            context_from_admissibility(&step_fourteen_library, step_fourteen),
            9,
        );
        let mut realistic_fourteen =
            context_from_admissibility(&step_fourteen_library, step_fourteen);
        realistic_fourteen.late_family_surface = LateFamilySurface::RealisticShadow;
        let realistic_fourteen = build_clause_catalog(realistic_fourteen, 9);
        let realistic_hilbert_extra = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(2)),
                Box::new(Expr::Univ),
            ))),
        );
        assert!(
            !claim_fourteen
                .clauses_at(8)
                .contains(&realistic_hilbert_extra)
        );
        assert!(
            realistic_fourteen
                .clauses_at(8)
                .contains(&realistic_hilbert_extra)
        );

        let step_fifteen_library = library_until(14);
        let step_fifteen = strict_admissibility_for_mode(
            15,
            2,
            &step_fifteen_library,
            AdmissibilityMode::DesktopClaimShadow,
        );
        let claim_fifteen = build_clause_catalog(
            context_from_admissibility(&step_fifteen_library, step_fifteen),
            8,
        );
        let mut realistic_fifteen = context_from_admissibility(&step_fifteen_library, step_fifteen);
        realistic_fifteen.late_family_surface = LateFamilySurface::RealisticShadow;
        let realistic_fifteen = build_clause_catalog(realistic_fifteen, 8);
        let realistic_temporal_extra = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                    Box::new(Expr::Var(1)),
                )))))),
            ),
        );
        assert!(
            !claim_fifteen
                .clauses_at(4)
                .contains(&realistic_temporal_extra)
        );
        assert!(
            realistic_fifteen
                .clauses_at(4)
                .contains(&realistic_temporal_extra)
        );
    }

    #[test]
    fn enumeration_is_deterministic_and_window_limited() {
        let context = EnumerationContext {
            library_size: 3,
            scope_size: 1,
            max_path_dimension: 2,
            include_trunc: true,
            include_modal: true,
            include_temporal: true,
            max_expr_nodes: 3,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            require_curvature_shell_clauses: false,
            require_operator_bundle_clauses: false,
            require_hilbert_functional_clauses: false,
            require_temporal_shell_clauses: false,
            historical_anchor_ref: None,
            late_family_surface: LateFamilySurface::None,
        };

        let first = enumerate_next_clauses(context);
        let second = enumerate_next_clauses(context);

        assert_eq!(first, second);
        assert!(
            first
                .iter()
                .any(|clause| clause.expr == pen_core::expr::Expr::Lib(3))
        );
        assert!(
            first
                .iter()
                .all(|clause| clause.expr != pen_core::expr::Expr::Lib(1))
        );
        assert!(
            first
                .iter()
                .any(|clause| clause.expr == pen_core::expr::Expr::PathCon(2))
        );
    }

    #[test]
    fn recursive_enumeration_reaches_nested_bootstrap_forms() {
        let exprs = enumerate_exprs(EnumerationContext {
            library_size: 0,
            scope_size: 2,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            max_expr_nodes: 4,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            require_curvature_shell_clauses: false,
            require_operator_bundle_clauses: false,
            require_hilbert_functional_clauses: false,
            require_temporal_shell_clauses: false,
            historical_anchor_ref: None,
            late_family_surface: LateFamilySurface::None,
        });
        let rendered = exprs
            .iter()
            .map(|expr| serde_json::to_string(expr).expect("serialize"))
            .collect::<Vec<_>>();

        assert!(
            rendered
                .iter()
                .any(|expr| expr.contains("\"Lam\":{\"Pi\":"))
        );
    }

    #[test]
    fn telescope_enumeration_discovers_bootstrap_candidates() {
        let telescopes = enumerate_telescopes(
            &Vec::new(),
            EnumerationContext {
                library_size: 0,
                scope_size: 1,
                max_path_dimension: 0,
                include_trunc: false,
                include_modal: false,
                include_temporal: false,
                max_expr_nodes: 3,
                require_former_eliminator_clauses: false,
                require_initial_hit_clauses: false,
                require_truncation_hit_clauses: false,
                require_higher_hit_clauses: false,
                require_sphere_lift_clauses: false,
                require_axiomatic_bundle_clauses: false,
                require_modal_shell_clauses: false,
                require_connection_shell_clauses: false,
                require_curvature_shell_clauses: false,
                require_operator_bundle_clauses: false,
                require_hilbert_functional_clauses: false,
                require_temporal_shell_clauses: false,
                historical_anchor_ref: None,
                late_family_surface: LateFamilySurface::None,
            },
            2,
        );

        assert!(
            telescopes
                .iter()
                .any(|telescope| *telescope == pen_core::telescope::Telescope::reference(1))
        );
    }

    #[test]
    fn former_eliminator_filter_preserves_nested_step_four_clauses() {
        let clauses = enumerate_next_clauses(EnumerationContext {
            library_size: 0,
            scope_size: 3,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            max_expr_nodes: 5,
            require_former_eliminator_clauses: true,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            require_curvature_shell_clauses: false,
            require_operator_bundle_clauses: false,
            require_hilbert_functional_clauses: false,
            require_temporal_shell_clauses: false,
            historical_anchor_ref: None,
            late_family_surface: LateFamilySurface::None,
        });
        assert!(clauses.iter().any(|clause| {
            clause.expr
                == pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(2)),
                )))
        }));
    }

    #[test]
    fn step_four_positional_filters_accept_reference_eliminator_and_computation_clauses() {
        assert!(supports_former_package_clause_at_position(
            1,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::App(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(2)),
                )),
                Box::new(pen_core::expr::Expr::Var(3)),
            )
        ));
        assert!(supports_former_package_clause_at_position(
            2,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lam(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
                Box::new(pen_core::expr::Expr::Var(2)),
            )
        ));
    }

    #[test]
    fn initial_hit_filters_accept_the_reference_point_and_path_package() {
        assert!(supports_initial_hit_clause_at_position(
            0,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Univ),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            LateFamilySurface::None,
        ));
        assert!(supports_initial_hit_clause_at_position(
            1,
            &pen_core::expr::Expr::Var(1),
            LateFamilySurface::None,
        ));
        assert!(supports_initial_hit_clause_at_position(
            2,
            &pen_core::expr::Expr::PathCon(1),
            LateFamilySurface::None,
        ));
    }

    #[test]
    fn truncation_hit_filters_accept_the_reference_shell_and_path_package() {
        assert!(supports_truncation_hit_clause_at_position(
            0,
            &pen_core::expr::Expr::Trunc(Box::new(pen_core::expr::Expr::Var(1))),
            LateFamilySurface::None,
        ));
        assert!(supports_truncation_hit_clause_at_position(
            1,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Trunc(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
                Box::new(pen_core::expr::Expr::Var(2)),
            ),
            LateFamilySurface::None,
        ));
        assert!(supports_truncation_hit_clause_at_position(
            2,
            &pen_core::expr::Expr::PathCon(1),
            LateFamilySurface::None,
        ));
    }

    #[test]
    fn higher_hit_filters_accept_the_reference_point_and_higher_path_package() {
        assert!(supports_higher_hit_clause_at_position(
            0,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Univ),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            LateFamilySurface::None,
        ));
        assert!(supports_higher_hit_clause_at_position(
            1,
            &pen_core::expr::Expr::Var(1),
            LateFamilySurface::None,
        ));
        assert!(supports_higher_hit_clause_at_position(
            2,
            &pen_core::expr::Expr::PathCon(2),
            LateFamilySurface::None,
        ));
    }

    #[test]
    fn sphere_lift_filters_accept_the_reference_point_path_and_witness_package() {
        assert!(supports_sphere_lift_clause_at_position(
            0,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Univ),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            LateFamilySurface::None,
        ));
        assert!(supports_sphere_lift_clause_at_position(
            1,
            &pen_core::expr::Expr::Var(1),
            LateFamilySurface::None,
        ));
        assert!(supports_sphere_lift_clause_at_position(
            2,
            &pen_core::expr::Expr::PathCon(3),
            LateFamilySurface::None,
        ));
        assert!(supports_sphere_lift_clause_at_position(
            3,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Var(1))),
            LateFamilySurface::None,
        ));
        assert!(supports_sphere_lift_clause_at_position(
            4,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Var(2))),
            LateFamilySurface::None,
        ));
    }

    #[test]
    fn axiomatic_bundle_filters_accept_the_reference_library_bundle() {
        assert!(supports_axiomatic_bundle_clause_at_position(
            0,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(8)),
                Box::new(pen_core::expr::Expr::Lib(7)),
            ),
            8,
            Some(5),
            LateFamilySurface::None,
        ));
        assert!(supports_axiomatic_bundle_clause_at_position(
            1,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(5)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            8,
            Some(5),
            LateFamilySurface::None,
        ));
        assert!(supports_axiomatic_bundle_clause_at_position(
            2,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(8)),
                Box::new(pen_core::expr::Expr::Lib(7)),
            ))),
            8,
            Some(5),
            LateFamilySurface::None,
        ));
        assert!(supports_axiomatic_bundle_clause_at_position(
            3,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(7)),
                Box::new(pen_core::expr::Expr::Lib(8)),
            ),
            8,
            Some(5),
            LateFamilySurface::None,
        ));
    }

    #[test]
    fn modal_shell_filters_accept_the_reference_modal_bundle() {
        assert!(supports_modal_shell_clause_at_position(
            0,
            &pen_core::expr::Expr::Flat(Box::new(pen_core::expr::Expr::Var(1)))
        ));
        assert!(supports_modal_shell_clause_at_position(
            1,
            &pen_core::expr::Expr::Sharp(Box::new(pen_core::expr::Expr::Var(1)))
        ));
        assert!(supports_modal_shell_clause_at_position(
            2,
            &pen_core::expr::Expr::Disc(Box::new(pen_core::expr::Expr::Var(1)))
        ));
        assert!(supports_modal_shell_clause_at_position(
            3,
            &pen_core::expr::Expr::Shape(Box::new(pen_core::expr::Expr::Var(1)))
        ));
    }

    #[test]
    fn connection_shell_filters_accept_the_reference_connection_bundle() {
        assert!(supports_connection_shell_clause_at_position(
            0,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(10)),
                Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(1)),
                )),
            ),
            10,
        ));
        assert!(supports_connection_shell_clause_at_position(
            1,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::Var(2)),
            ))),
            10,
        ));
        assert!(supports_connection_shell_clause_at_position(
            2,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Flat(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            10,
        ));
        assert!(supports_connection_shell_clause_at_position(
            3,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(10)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            10,
        ));
        assert!(supports_connection_shell_clause_at_position(
            4,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Var(1))),
            10,
        ));
    }

    #[test]
    fn curvature_shell_filters_accept_the_reference_second_order_bundle() {
        assert!(supports_curvature_shell_clause_at_position(
            0,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(11)),
                Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(1)),
                )),
            ),
            11,
        ));
        assert!(supports_curvature_shell_clause_at_position(
            1,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(11)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ))),
            11,
        ));
        assert!(supports_curvature_shell_clause_at_position(
            2,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::Lib(11)),
            ),
            11,
        ));
        assert!(supports_curvature_shell_clause_at_position(
            3,
            &pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(11)),
                Box::new(pen_core::expr::Expr::App(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(2)),
                )),
            ),
            11,
        ));
        assert!(supports_curvature_shell_clause_at_position(
            4,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::Var(2)),
            ))),
            11,
        ));
        assert!(supports_curvature_shell_clause_at_position(
            5,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(11)),
                Box::new(pen_core::expr::Expr::Lib(11)),
            ),
            11,
        ));
    }

    #[test]
    fn operator_bundle_filters_accept_the_reference_metric_reading_shell() {
        assert!(supports_operator_bundle_clause_at_position(
            0,
            &pen_core::expr::Expr::Sigma(
                Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(1)),
                )),
                Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(1)),
                )),
            ),
            12,
            LateFamilySurface::None,
        ));
        assert!(supports_operator_bundle_clause_at_position(
            1,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Sigma(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(2)),
                )),
                Box::new(pen_core::expr::Expr::Lib(11)),
            ),
            12,
            LateFamilySurface::None,
        ));
        assert!(supports_operator_bundle_clause_at_position(
            2,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(1)),
                )),
            ),
            12,
            LateFamilySurface::None,
        ));
        assert!(supports_operator_bundle_clause_at_position(
            3,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::Var(2)),
            ))),
            12,
            LateFamilySurface::None,
        ));
        assert!(supports_operator_bundle_clause_at_position(
            4,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(12)),
                Box::new(pen_core::expr::Expr::Lib(12)),
            ),
            12,
            LateFamilySurface::None,
        ));
        assert!(supports_operator_bundle_clause_at_position(
            5,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ))),
            12,
            LateFamilySurface::None,
        ));
        assert!(supports_operator_bundle_clause_at_position(
            6,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(12)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            12,
            LateFamilySurface::None,
        ));
    }

    #[test]
    fn hilbert_functional_filters_accept_the_reference_step_fourteen_shell() {
        assert!(supports_hilbert_functional_clause_at_position(
            0,
            &pen_core::expr::Expr::Sigma(
                Box::new(pen_core::expr::Expr::Pi(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Pi(
                        Box::new(pen_core::expr::Expr::Var(1)),
                        Box::new(pen_core::expr::Expr::Univ),
                    )),
                )),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            13,
            LateFamilySurface::None,
        ));
        assert!(supports_hilbert_functional_clause_at_position(
            3,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lam(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
                Box::new(pen_core::expr::Expr::Sigma(
                    Box::new(pen_core::expr::Expr::Var(1)),
                    Box::new(pen_core::expr::Expr::Var(2)),
                )),
            ),
            13,
            LateFamilySurface::None,
        ));
        assert!(supports_hilbert_functional_clause_at_position(
            5,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(13)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            13,
            LateFamilySurface::None,
        ));
        assert!(supports_hilbert_functional_clause_at_position(
            6,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(12)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            13,
            LateFamilySurface::None,
        ));
        assert!(supports_hilbert_functional_clause_at_position(
            7,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Lib(11)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ),
            13,
            LateFamilySurface::None,
        ));
        assert!(supports_hilbert_functional_clause_at_position(
            8,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::Univ),
            ))),
            13,
            LateFamilySurface::None,
        ));
    }

    #[test]
    fn step_four_enumeration_contains_the_reference_former_package() {
        let library = library_until(3);
        let admissibility = strict_admissibility(4, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(4)),
            "enumerated {} step-4 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_five_enumeration_contains_the_reference_hit_package() {
        let library = library_until(4);
        let admissibility = strict_admissibility(5, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(5)),
            "enumerated {} step-5 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_six_enumeration_contains_the_reference_truncation_package() {
        let library = library_until(5);
        let admissibility = strict_admissibility(6, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(6)),
            "enumerated {} step-6 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_seven_enumeration_contains_the_reference_higher_hit_package() {
        let library = library_until(6);
        let admissibility = strict_admissibility(7, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(7)),
            "enumerated {} step-7 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_eight_enumeration_contains_the_reference_sphere_package() {
        let library = library_until(7);
        let admissibility = strict_admissibility(8, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(8)),
            "enumerated {} step-8 telescopes, but not the reference package",
            telescopes.len()
        );
    }

    #[test]
    fn step_nine_enumeration_contains_the_reference_axiomatic_bundle() {
        let library = library_until(8);
        let admissibility = strict_admissibility(9, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(9)),
            "enumerated {} step-9 telescopes, but not the reference bundle",
            telescopes.len()
        );
    }

    #[test]
    fn demo_breadth_shadow_step_five_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(4);
        let admissibility =
            strict_admissibility_for_mode(5, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_raw_telescopes(realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_raw_telescopes(demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(5));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 5 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn demo_breadth_shadow_step_six_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(5);
        let admissibility =
            strict_admissibility_for_mode(6, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_raw_telescopes(realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_raw_telescopes(demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(6));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 6 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn demo_breadth_shadow_step_seven_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(6);
        let admissibility =
            strict_admissibility_for_mode(7, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_raw_telescopes(realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_raw_telescopes(demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(7));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 7 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn demo_breadth_shadow_step_eight_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(7);
        let admissibility =
            strict_admissibility_for_mode(8, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_raw_telescopes(realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_raw_telescopes(demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(8));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 8 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn demo_breadth_shadow_step_nine_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(8);
        let admissibility =
            strict_admissibility_for_mode(9, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_raw_telescopes(realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_raw_telescopes(demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(9));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 9 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn step_ten_enumeration_contains_the_reference_modal_shell() {
        let library = library_until(9);
        let admissibility = strict_admissibility(10, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(10)),
            "enumerated {} step-10 telescopes, but not the reference modal shell",
            telescopes.len()
        );
    }

    #[test]
    fn step_one_raw_enumeration_counts_the_full_clause_catalog_surface() {
        let library = library_until(0);
        let admissibility = strict_admissibility(1, 2, &library);
        let context = context_from_admissibility(&library, admissibility);
        let widths = raw_clause_catalog_widths(context, admissibility.min_clause_kappa);
        let raw_telescopes = enumerate_raw_telescopes(context, admissibility.min_clause_kappa);
        let telescopes = enumerate_telescopes(&library, context, admissibility.min_clause_kappa);

        assert_eq!(widths, vec![36, 36]);
        assert_eq!(raw_telescopes.len(), 1296);
        assert_eq!(telescopes.len(), 288);
    }

    #[test]
    fn relaxed_shadow_step_ten_enumeration_exposes_more_than_one_telescope() {
        let library = library_until(9);
        let admissibility =
            strict_admissibility_for_mode(10, 2, &library, AdmissibilityMode::RelaxedShadow);
        let telescopes = enumerate_telescopes(
            &library,
            context_from_admissibility(&library, admissibility),
            admissibility.min_clause_kappa,
        );

        assert!(telescopes.contains(&Telescope::reference(10)));
        assert!(
            telescopes.len() > 1,
            "expected relaxed shadow step 10 enumeration to expose competition, got {} telescope(s)",
            telescopes.len()
        );
    }

    #[test]
    fn step_eleven_enumeration_contains_the_reference_connection_shell() {
        let library = library_until(10);
        let admissibility = strict_admissibility(11, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.min_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(11)),
            "enumerated {} step-11 telescopes, but not the reference connection shell",
            telescopes.len()
        );
    }

    #[test]
    fn relaxed_shadow_step_eleven_enumeration_exposes_more_than_one_telescope() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RelaxedShadow);
        let mut telescope_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let telescopes = enumerate_telescopes(
                &library,
                context_from_admissibility(&library, admissibility),
                clause_kappa,
            );
            telescope_count += telescopes.len();
            contains_reference |= telescopes.contains(&Telescope::reference(11));
        }

        assert!(contains_reference);
        assert!(
            telescope_count > 1,
            "expected relaxed shadow step 11 enumeration to expose competition, got {} telescope(s)",
            telescope_count
        );
    }

    #[test]
    fn step_twelve_enumeration_contains_the_reference_curvature_shell() {
        let library = library_until(11);
        let admissibility = strict_admissibility(12, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.max_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(12)),
            "enumerated {} step-12 telescopes, but not the reference curvature shell",
            telescopes.len()
        );
    }

    #[test]
    fn relaxed_shadow_step_twelve_enumeration_exposes_more_than_one_telescope() {
        let library = library_until(11);
        let admissibility =
            strict_admissibility_for_mode(12, 2, &library, AdmissibilityMode::RelaxedShadow);
        let mut telescope_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let telescopes = enumerate_telescopes(
                &library,
                context_from_admissibility(&library, admissibility),
                clause_kappa,
            );
            telescope_count += telescopes.len();
            contains_reference |= telescopes.contains(&Telescope::reference(12));
        }

        assert!(contains_reference);
        assert!(
            telescope_count > 1,
            "expected relaxed shadow step 12 enumeration to expose competition, got {} telescope(s)",
            telescope_count
        );
    }

    #[test]
    fn demo_breadth_shadow_step_ten_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(9);
        let admissibility =
            strict_admissibility_for_mode(10, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_telescopes(&library, realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_telescopes(&library, demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(10));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 10 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn demo_breadth_shadow_step_eleven_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_telescopes(&library, realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_telescopes(&library, demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(11));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 11 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn demo_breadth_shadow_step_twelve_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(11);
        let admissibility =
            strict_admissibility_for_mode(12, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_telescopes(&library, realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_telescopes(&library, demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(12));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 12 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn demo_breadth_shadow_step_thirteen_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(12);
        let admissibility =
            strict_admissibility_for_mode(13, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_telescopes(&library, realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_telescopes(&library, demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(13));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 13 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn step_thirteen_enumeration_contains_the_reference_operator_bundle() {
        let library = library_until(12);
        let admissibility = strict_admissibility(13, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.max_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(13)),
            "enumerated {} step-13 telescopes, but not the reference operator bundle",
            telescopes.len()
        );
    }

    #[test]
    fn realistic_shadow_step_thirteen_enumeration_exposes_more_than_one_telescope() {
        let library = library_until(12);
        let admissibility =
            strict_admissibility_for_mode(13, 2, &library, AdmissibilityMode::RealisticShadow);
        let telescopes = enumerate_telescopes(
            &library,
            context_from_admissibility(&library, admissibility),
            admissibility.max_clause_kappa,
        );

        assert!(telescopes.contains(&Telescope::reference(13)));
        assert!(
            telescopes.len() > 1,
            "expected realistic shadow step 13 enumeration to expose competition, got {} telescope(s)",
            telescopes.len()
        );
    }

    #[test]
    fn demo_breadth_shadow_step_fourteen_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(13);
        let admissibility =
            strict_admissibility_for_mode(14, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_telescopes(&library, realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_telescopes(&library, demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(14));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 14 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn step_fourteen_enumeration_contains_the_reference_hilbert_functional_shell() {
        let library = library_until(13);
        let admissibility = strict_admissibility(14, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.max_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(14)),
            "enumerated {} step-14 telescopes, but not the reference Hilbert shell",
            telescopes.len()
        );
    }

    #[test]
    fn realistic_shadow_step_fourteen_enumeration_exposes_more_than_one_telescope() {
        let library = library_until(13);
        let admissibility =
            strict_admissibility_for_mode(14, 2, &library, AdmissibilityMode::RealisticShadow);
        let telescopes = enumerate_telescopes(
            &library,
            context_from_admissibility(&library, admissibility),
            admissibility.max_clause_kappa,
        );

        assert!(telescopes.contains(&Telescope::reference(14)));
        assert!(
            telescopes.len() > 1,
            "expected realistic shadow step 14 enumeration to expose competition, got {} telescope(s)",
            telescopes.len()
        );
    }

    #[test]
    fn demo_breadth_shadow_step_fifteen_enumeration_exposes_more_surface_than_realistic_shadow() {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = context_from_admissibility(&library, admissibility);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let mut realistic_count = 0usize;
        let mut demo_count = 0usize;
        let mut contains_reference = false;

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let realistic = enumerate_telescopes(&library, realistic_context, clause_kappa);
            realistic_count += realistic.len();

            let demo = enumerate_telescopes(&library, demo_context, clause_kappa);
            demo_count += demo.len();
            contains_reference |= demo.contains(&Telescope::reference(15));
        }

        assert!(contains_reference);
        assert!(
            demo_count > realistic_count,
            "expected demo breadth shadow step 15 enumeration to widen the realistic surface, got demo={} vs realistic={}",
            demo_count,
            realistic_count
        );
    }

    #[test]
    fn temporal_shell_filters_accept_the_reference_dct_bundle() {
        assert!(supports_temporal_shell_clause_at_position(
            0,
            &pen_core::expr::Expr::Next(Box::new(pen_core::expr::Expr::Var(1))),
            Some(10),
            LateFamilySurface::None,
        ));
        assert!(supports_temporal_shell_clause_at_position(
            1,
            &pen_core::expr::Expr::Eventually(Box::new(pen_core::expr::Expr::Var(1))),
            Some(10),
            LateFamilySurface::None,
        ));
        assert!(supports_temporal_shell_clause_at_position(
            2,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Next(Box::new(
                    pen_core::expr::Expr::Var(1)
                ))),
                Box::new(pen_core::expr::Expr::Eventually(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
            ),
            Some(10),
            LateFamilySurface::None,
        ));
        assert!(supports_temporal_shell_clause_at_position(
            3,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Lib(10)),
                Box::new(pen_core::expr::Expr::Next(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
            ))),
            Some(10),
            LateFamilySurface::None,
        ));
        assert!(supports_temporal_shell_clause_at_position(
            4,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Flat(Box::new(
                    pen_core::expr::Expr::Next(Box::new(pen_core::expr::Expr::Var(1))),
                ))),
                Box::new(pen_core::expr::Expr::Next(Box::new(
                    pen_core::expr::Expr::Flat(Box::new(pen_core::expr::Expr::Var(1))),
                ))),
            ),
            Some(10),
            LateFamilySurface::None,
        ));
        assert!(supports_temporal_shell_clause_at_position(
            5,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Sharp(Box::new(
                    pen_core::expr::Expr::Eventually(Box::new(pen_core::expr::Expr::Var(1))),
                ))),
                Box::new(pen_core::expr::Expr::Eventually(Box::new(
                    pen_core::expr::Expr::Sharp(Box::new(pen_core::expr::Expr::Var(1))),
                ))),
            ),
            Some(10),
            LateFamilySurface::None,
        ));
        assert!(supports_temporal_shell_clause_at_position(
            6,
            &pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Eventually(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
                Box::new(pen_core::expr::Expr::Var(2)),
            ))),
            Some(10),
            LateFamilySurface::None,
        ));
        assert!(supports_temporal_shell_clause_at_position(
            7,
            &pen_core::expr::Expr::Pi(
                Box::new(pen_core::expr::Expr::Next(Box::new(
                    pen_core::expr::Expr::Next(Box::new(pen_core::expr::Expr::Var(1))),
                ))),
                Box::new(pen_core::expr::Expr::Next(Box::new(
                    pen_core::expr::Expr::Var(1),
                ))),
            ),
            Some(10),
            LateFamilySurface::None,
        ));
    }

    #[test]
    fn step_fifteen_enumeration_contains_the_reference_temporal_shell() {
        let library = library_until(14);
        let admissibility = strict_admissibility(15, 2, &library);
        let telescopes = enumerate_telescopes(
            &library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
                late_family_surface: LateFamilySurface::None,
            },
            admissibility.max_clause_kappa,
        );

        assert!(
            telescopes.contains(&Telescope::reference(15)),
            "enumerated {} step-15 telescopes, but not the reference temporal shell",
            telescopes.len()
        );
    }

    #[test]
    fn realistic_shadow_step_fifteen_enumeration_exposes_more_than_one_telescope() {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::RealisticShadow);
        let telescopes = enumerate_telescopes(
            &library,
            context_from_admissibility(&library, admissibility),
            admissibility.max_clause_kappa,
        );

        assert!(telescopes.contains(&Telescope::reference(15)));
        assert!(
            telescopes.len() > 1,
            "expected realistic shadow step 15 enumeration to expose competition, got {} telescope(s)",
            telescopes.len()
        );
    }
}
