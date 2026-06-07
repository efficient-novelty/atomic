use pen_core::atom::Atom;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::encode::expr_bit_length;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::Telescope;
use pen_eval::nu::TerminalClauseNuFacts;
use pen_type::admissibility::{AdmissibilityMode, StrictAdmissibility, StructuralFamily};
use pen_type::check::{CheckResult, check_telescope};
use pen_type::connectivity::{TerminalClauseConnectivityFacts, analyze_connectivity};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LateFamilySurface {
    None,
    RealisticShadow,
    ClaimGeneric,
    DemoBreadthShadow,
}

impl LateFamilySurface {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RealisticShadow => "realistic_shadow",
            Self::ClaimGeneric => "claim_generic",
            Self::DemoBreadthShadow => "demo_breadth_shadow",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnumerationSurfaceDiagnostics {
    pub library_size: u32,
    pub late_family_surface: LateFamilySurface,
    pub claim_widening_band7_active: bool,
    pub claim_widening_band8_active: bool,
    pub claim_widening_band9_active: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnumerationContext {
    pub library_size: u32,
    pub scope_size: u32,
    pub max_path_dimension: u32,
    pub include_trunc: bool,
    pub include_modal: bool,
    pub include_temporal: bool,
    pub include_linear_exponential: bool,
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
    terminal_connectivity_facts_by_position: Vec<TerminalConnectivityFactsByPosition>,
    terminal_nu_facts_by_position: Vec<Vec<TerminalClauseNuFacts>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TerminalConnectivityFactsByPosition {
    Eager(Vec<TerminalClauseConnectivityFacts>),
    Deferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RawClauseCatalogWidthProgress {
    PositionStarted {
        position: usize,
    },
    PositionExprNodesReady {
        position: usize,
        expr_nodes: u8,
        max_expr_nodes: u8,
        width_so_far: usize,
    },
    PositionReady {
        position: usize,
        width: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClauseCatalogBuildProgress {
    PositionStarted {
        position: usize,
    },
    PositionExprNodesGenerated {
        position: usize,
        expr_nodes: u8,
        max_expr_nodes: u8,
        expr_count: usize,
    },
    PositionExprNodesAccumulationProgress {
        position: usize,
        expr_nodes: u8,
        max_expr_nodes: u8,
        scanned_expr_count: usize,
        clause_count_so_far: usize,
    },
    PositionExprNodesReady {
        position: usize,
        expr_nodes: u8,
        max_expr_nodes: u8,
        clause_count_so_far: usize,
    },
    PositionSortStarted {
        position: usize,
        clause_count: usize,
    },
    PositionSorted {
        position: usize,
        clause_count: usize,
    },
    PositionConnectivityFactsReady {
        position: usize,
        clause_count: usize,
    },
    PositionNuFactsReady {
        position: usize,
        clause_count: usize,
    },
    PositionReady {
        position: usize,
        clause_count: usize,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TelescopeEnumerationProgress {
    pub prefix_attempts: usize,
    pub prefix_states_explored: usize,
    pub terminal_prefixes: usize,
    pub dfs_prefix_rejections: usize,
    pub dfs_leaf_rejections: usize,
    pub dfs_leaf_check_rejections: usize,
    pub dfs_leaf_connectivity_rejections: usize,
    pub dfs_leaf_disconnected_rejections: usize,
    pub dfs_leaf_connected_unqualified_rejections: usize,
    pub completed_telescopes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TelescopeEnumerationProgressEvent {
    ClauseCatalog(ClauseCatalogBuildProgress),
    EnumerationHandoff(TelescopeEnumerationProgress),
    Enumeration(TelescopeEnumerationProgress),
}

const CLAUSE_MATERIALIZATION_PROGRESS_CHUNK: usize = 1_000_000;
const MAX_EAGER_TERMINAL_CONNECTIVITY_FACTS_PER_POSITION: usize = 1_000_000;
const MIN_STREAMED_EXACT_EXPR_BUCKET_SIZE: usize = 5_000_000;

impl TerminalConnectivityFactsByPosition {
    fn for_clauses(clauses: &[ClauseRec]) -> Self {
        if clauses.len() > MAX_EAGER_TERMINAL_CONNECTIVITY_FACTS_PER_POSITION {
            return Self::Deferred;
        }
        Self::Eager(
            clauses
                .iter()
                .map(TerminalClauseConnectivityFacts::from_clause)
                .collect(),
        )
    }

    fn as_slice(&self) -> Option<&[TerminalClauseConnectivityFacts]> {
        match self {
            Self::Eager(facts) => Some(facts),
            Self::Deferred => None,
        }
    }
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

    pub fn terminal_connectivity_facts_at(
        &self,
        position: usize,
    ) -> &[TerminalClauseConnectivityFacts] {
        self.precomputed_terminal_connectivity_facts_at(position)
            .unwrap_or(&[])
    }

    pub(crate) fn precomputed_terminal_connectivity_facts_at(
        &self,
        position: usize,
    ) -> Option<&[TerminalClauseConnectivityFacts]> {
        self.terminal_connectivity_facts_by_position
            .get(position)
            .and_then(TerminalConnectivityFactsByPosition::as_slice)
    }

    pub fn terminal_nu_facts_at(&self, position: usize) -> &[TerminalClauseNuFacts] {
        self.terminal_nu_facts_by_position
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
            include_linear_exponential: admissibility.include_linear_exponential,
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

    pub fn surface_diagnostics(self) -> EnumerationSurfaceDiagnostics {
        EnumerationSurfaceDiagnostics {
            library_size: self.library_size,
            late_family_surface: self.late_family_surface,
            claim_widening_band7_active: self.library_size >= 11,
            claim_widening_band8_active: self.library_size >= 11,
            claim_widening_band9_active: self.library_size >= 13,
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
                && (!context.require_temporal_shell_clauses
                    || supports_temporal_shell_clause(
                        expr,
                        context.include_linear_exponential,
                    ))
        })
        .map(|expr| ClauseRec::new(primary_role(&expr), expr))
        .collect()
}

fn enumerate_raw_next_clauses(context: EnumerationContext) -> Vec<ClauseRec> {
    enumerate_exprs_raw(context)
        .into_iter()
        .filter(|expr| raw_clause_matches_context(expr, context))
        .map(|expr| ClauseRec::new(primary_role(&expr), expr))
        .collect()
}

fn dedupe_sorted_clauses(mut clauses: Vec<ClauseRec>) -> Vec<ClauseRec> {
    sort_clauses_in_place(&mut clauses);
    clauses.dedup();
    clauses
}

fn sort_clauses_in_place(clauses: &mut [ClauseRec]) {
    clauses.sort_by_cached_key(clause_sort_key);
}

#[cfg_attr(not(test), allow(dead_code))]
fn compare_clause_sort_order(left: &ClauseRec, right: &ClauseRec) -> Ordering {
    (left.role as u8)
        .cmp(&(right.role as u8))
        .then_with(|| compare_expr_sort_order(&left.expr, &right.expr))
}

#[cfg_attr(not(test), allow(dead_code))]
fn clause_sort_key(clause: &ClauseRec) -> (u8, (u8, u32, String)) {
    (clause.role as u8, expr_sort_key(&clause.expr))
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
    if context.library_size < 2 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest - 1;
    let widen = context.library_size >= 12;
    match position {
        0 => {
            if widen {
                demo_operator_bundle_clauses(position, context)
            } else {
                vec![
                    Expr::Sigma(
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    ),
                    Expr::Sigma(
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                        Box::new(Expr::Pi(
                            Box::new(Expr::Var(1)),
                            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                        )),
                    ),
                    Expr::Sigma(
                        Box::new(Expr::Pi(
                            Box::new(Expr::Var(1)),
                            Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                        )),
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    ),
                ]
            }
        }
        1 => {
            vec![Expr::Pi(
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                Box::new(Expr::Lib(previous)),
            )]
        }
        2 => {
            let mut clauses = vec![Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            )];
            if widen {
                clauses.extend([
                    Expr::Pi(
                        Box::new(Expr::Var(1)),
                        Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    ),
                    Expr::Pi(
                        Box::new(Expr::Var(2)),
                        Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    ),
                ]);
            }
            clauses
        }
        3 => {
            let mut clauses = vec![Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            )))];
            if widen {
                clauses.extend([
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Var(1)),
                        Box::new(Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                    ))),
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Var(2)),
                        Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                    ))),
                ]);
            }
            clauses
        }
        4 => {
            if widen {
                demo_operator_bundle_clauses(position, context)
            } else {
                vec![Expr::Pi(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Lib(latest)),
                )]
            }
        }
        5 => {
            if widen {
                demo_operator_bundle_clauses(position, context)[..3].to_vec()
            } else {
                vec![Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(1)),
                )))]
            }
        }
        6 => {
            if widen {
                vec![
                    Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
                    Expr::Pi(
                        Box::new(Expr::Lib(latest)),
                        Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                    ),
                ]
            } else {
                vec![Expr::Pi(
                    Box::new(Expr::Lib(latest)),
                    Box::new(Expr::Var(1)),
                )]
            }
        }
        _ => Vec::new(),
    }
}

fn temporal_shell_left_expr(body: Expr, include_linear_exponential: bool) -> Expr {
    if include_linear_exponential {
        Expr::Bang(Box::new(body))
    } else {
        Expr::Next(Box::new(body))
    }
}

fn temporal_shell_right_expr(body: Expr, include_linear_exponential: bool) -> Expr {
    if include_linear_exponential {
        Expr::WhyNot(Box::new(body))
    } else {
        Expr::Eventually(Box::new(body))
    }
}

fn temporal_shell_left_body(expr: &Expr, include_linear_exponential: bool) -> Option<&Expr> {
    match (include_linear_exponential, expr) {
        (false, Expr::Next(body)) | (true, Expr::Bang(body)) => Some(body.as_ref()),
        _ => None,
    }
}

fn temporal_shell_right_body(expr: &Expr, include_linear_exponential: bool) -> Option<&Expr> {
    match (include_linear_exponential, expr) {
        (false, Expr::Eventually(body)) | (true, Expr::WhyNot(body)) => Some(body.as_ref()),
        _ => None,
    }
}

fn matches_temporal_shell_left_var(
    expr: &Expr,
    include_linear_exponential: bool,
    index: u32,
) -> bool {
    temporal_shell_left_body(expr, include_linear_exponential)
        .is_some_and(|body| matches!(body, Expr::Var(found) if *found == index))
}

fn matches_temporal_shell_right_var(
    expr: &Expr,
    include_linear_exponential: bool,
    index: u32,
) -> bool {
    temporal_shell_right_body(expr, include_linear_exponential)
        .is_some_and(|body| matches!(body, Expr::Var(found) if *found == index))
}

fn temporal_shell_realistic_position_four_extension(context: EnumerationContext) -> Expr {
    let include_linear_exponential = context.include_linear_exponential;
    Expr::Pi(
        Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
            Expr::Var(1),
            include_linear_exponential,
        )))),
        Box::new(temporal_shell_left_expr(
            Expr::Flat(Box::new(temporal_shell_left_expr(
                Expr::Var(1),
                include_linear_exponential,
            ))),
            include_linear_exponential,
        )),
    )
}

fn claim_generic_band8_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    let Some(anchor) = context.historical_anchor_ref else {
        return Vec::new();
    };
    let include_linear_exponential = context.include_linear_exponential;

    match position {
        0 => vec![
            temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
            temporal_shell_left_expr(
                Expr::Flat(Box::new(Expr::Var(1))),
                include_linear_exponential,
            ),
            temporal_shell_left_expr(
                temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                include_linear_exponential,
            ),
        ],
        1 => vec![
            temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
            temporal_shell_right_expr(
                Expr::Sharp(Box::new(Expr::Var(1))),
                include_linear_exponential,
            ),
            temporal_shell_right_expr(
                temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                include_linear_exponential,
            ),
        ],
        2 => vec![
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
        ],
        3 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(temporal_shell_left_expr(
                    temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
            ))),
        ],
        4 => vec![
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                    temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(temporal_shell_left_expr(
                        temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                        include_linear_exponential,
                    ))),
                    include_linear_exponential,
                )),
            ),
        ],
        5 => vec![
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(temporal_shell_left_expr(
                        Expr::Var(1),
                        include_linear_exponential,
                    ))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
        ],
        6 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(temporal_shell_right_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
                Box::new(Expr::Var(2)),
            ))),
        ],
        7 => vec![
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(
                        temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                        include_linear_exponential,
                    ),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(
                        temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                        include_linear_exponential,
                    ),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_left_expr(
                    temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
            ),
        ],
        _ => Vec::new(),
    }
}

fn claim_generic_band9_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    if context.library_size < 3 {
        return Vec::new();
    }

    let latest = context.library_size;
    let previous = latest - 1;
    let older = latest - 2;
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
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                )),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
            Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                )),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        ],
        1 => vec![
            Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            Expr::Pi(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(1)),
            ),
            Expr::Pi(
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(1)),
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
        5 => vec![
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Var(1))),
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(latest)), Box::new(Expr::Lib(previous))),
        ],
        6 => vec![
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Var(1))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(previous)), Box::new(Expr::Lib(older))),
        ],
        7 => vec![
            Expr::Pi(Box::new(Expr::Lib(older)), Box::new(Expr::Var(1))),
            Expr::Pi(Box::new(Expr::Lib(older)), Box::new(Expr::Lib(latest))),
            Expr::Pi(Box::new(Expr::Lib(older)), Box::new(Expr::Lib(previous))),
        ],
        8 => vec![
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Univ),
            ))),
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Lib(latest)),
            ))),
            Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Lib(previous)),
            ))),
        ],
        _ => Vec::new(),
    }
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
    let include_linear_exponential = context.include_linear_exponential;

    Some(match position {
        0 => temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
        1 => temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
        2 => Expr::Pi(
            Box::new(temporal_shell_left_expr(
                Expr::Var(1),
                include_linear_exponential,
            )),
            Box::new(temporal_shell_right_expr(
                Expr::Var(1),
                include_linear_exponential,
            )),
        ),
        3 => Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Lib(anchor)),
            Box::new(temporal_shell_left_expr(
                Expr::Var(1),
                include_linear_exponential,
            )),
        ))),
        4 => Expr::Pi(
            Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                Expr::Var(1),
                include_linear_exponential,
            )))),
            Box::new(temporal_shell_left_expr(
                Expr::Flat(Box::new(Expr::Var(1))),
                include_linear_exponential,
            )),
        ),
        5 => Expr::Pi(
            Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                Expr::Var(1),
                include_linear_exponential,
            )))),
            Box::new(temporal_shell_right_expr(
                Expr::Sharp(Box::new(Expr::Var(1))),
                include_linear_exponential,
            )),
        ),
        6 => Expr::Lam(Box::new(Expr::App(
            Box::new(temporal_shell_right_expr(
                Expr::Var(1),
                include_linear_exponential,
            )),
            Box::new(Expr::Var(2)),
        ))),
        7 => Expr::Pi(
            Box::new(temporal_shell_left_expr(
                temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                include_linear_exponential,
            )),
            Box::new(temporal_shell_left_expr(
                Expr::Var(1),
                include_linear_exponential,
            )),
        ),
        _ => return None,
    })
}

fn demo_temporal_shell_clauses(position: usize, context: EnumerationContext) -> Vec<Expr> {
    let Some(anchor) = context.historical_anchor_ref else {
        return Vec::new();
    };
    let include_linear_exponential = context.include_linear_exponential;

    match position {
        0 => vec![
            temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
            temporal_shell_left_expr(
                Expr::Flat(Box::new(Expr::Var(1))),
                include_linear_exponential,
            ),
            temporal_shell_left_expr(
                Expr::Sharp(Box::new(Expr::Var(1))),
                include_linear_exponential,
            ),
            temporal_shell_left_expr(
                temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                include_linear_exponential,
            ),
            temporal_shell_left_expr(
                temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                include_linear_exponential,
            ),
        ],
        1 => vec![
            temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
            temporal_shell_right_expr(
                Expr::Sharp(Box::new(Expr::Var(1))),
                include_linear_exponential,
            ),
            temporal_shell_right_expr(
                Expr::Flat(Box::new(Expr::Var(1))),
                include_linear_exponential,
            ),
            temporal_shell_right_expr(
                temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                include_linear_exponential,
            ),
            temporal_shell_right_expr(
                temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                include_linear_exponential,
            ),
        ],
        2 => vec![
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_right_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
        ],
        3 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(temporal_shell_left_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(anchor)),
                Box::new(temporal_shell_left_expr(
                    temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
            ))),
        ],
        4 => vec![
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(temporal_shell_left_expr(
                        Expr::Var(1),
                        include_linear_exponential,
                    ))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(temporal_shell_left_expr(
                        Expr::Var(1),
                        include_linear_exponential,
                    ))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Flat(Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_left_expr(
                    Expr::Sharp(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    include_linear_exponential,
                )),
            ),
        ],
        5 => vec![
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(temporal_shell_right_expr(
                        Expr::Var(1),
                        include_linear_exponential,
                    ))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(temporal_shell_left_expr(
                        Expr::Var(1),
                        include_linear_exponential,
                    ))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(Expr::Sharp(Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )))),
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                    include_linear_exponential,
                )),
            ),
        ],
        6 => vec![
            Expr::Lam(Box::new(Expr::App(
                Box::new(temporal_shell_right_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(temporal_shell_right_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(temporal_shell_right_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
                Box::new(Expr::Var(2)),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(temporal_shell_right_expr(
                    Expr::Sharp(Box::new(temporal_shell_left_expr(
                        Expr::Var(1),
                        include_linear_exponential,
                    ))),
                    include_linear_exponential,
                )),
                Box::new(Expr::Var(2)),
            ))),
        ],
        7 => vec![
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_left_expr(
                    Expr::Var(1),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(
                        Expr::Flat(Box::new(Expr::Var(1))),
                        include_linear_exponential,
                    ),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_left_expr(
                    Expr::Flat(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                        include_linear_exponential,
                    ),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_left_expr(
                    Expr::Sharp(Box::new(Expr::Var(1))),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(
                        temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                        include_linear_exponential,
                    ),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
            ),
            Expr::Pi(
                Box::new(temporal_shell_left_expr(
                    temporal_shell_left_expr(
                        temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                        include_linear_exponential,
                    ),
                    include_linear_exponential,
                )),
                Box::new(temporal_shell_left_expr(
                    temporal_shell_right_expr(Expr::Var(1), include_linear_exponential),
                    include_linear_exponential,
                )),
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
            temporal_shell_realistic_position_four_extension(context),
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
        && (context.include_temporal || context.include_linear_exponential)
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
    enumerate_telescopes_with_progress(library, base_context, clause_kappa, |_| {}).0
}

pub fn enumerate_telescopes_with_progress<F>(
    library: &Library,
    base_context: EnumerationContext,
    clause_kappa: u16,
    on_progress: F,
) -> (Vec<Telescope>, TelescopeEnumerationProgress)
where
    F: FnMut(TelescopeEnumerationProgressEvent),
{
    let (enumeration, progress) =
        enumerate_telescopes_with_terminal_prefixes_and_progress_with_raw_catalog_widths(
            library,
            base_context,
            clause_kappa,
            None,
            on_progress,
        );
    (enumeration.telescopes, progress)
}

pub(crate) fn enumerate_telescopes_with_progress_with_raw_catalog_widths<F>(
    library: &Library,
    base_context: EnumerationContext,
    clause_kappa: u16,
    raw_catalog_clause_widths: &[usize],
    on_progress: F,
) -> (Vec<Telescope>, TelescopeEnumerationProgress)
where
    F: FnMut(TelescopeEnumerationProgressEvent),
{
    let (enumeration, progress) =
        enumerate_telescopes_with_terminal_prefixes_and_progress_with_raw_catalog_widths(
            library,
            base_context,
            clause_kappa,
            Some(raw_catalog_clause_widths),
            on_progress,
        );
    (enumeration.telescopes, progress)
}

fn enumerate_telescopes_with_terminal_prefixes_and_progress_with_raw_catalog_widths<F>(
    library: &Library,
    base_context: EnumerationContext,
    clause_kappa: u16,
    raw_catalog_clause_widths: Option<&[usize]>,
    mut on_progress: F,
) -> (TelescopeEnumeration, TelescopeEnumerationProgress)
where
    F: FnMut(TelescopeEnumerationProgressEvent),
{
    let clause_catalog = build_clause_catalog_with_progress_and_raw_catalog_widths(
        base_context,
        clause_kappa,
        raw_catalog_clause_widths,
        |progress| on_progress(TelescopeEnumerationProgressEvent::ClauseCatalog(progress)),
    );
    if clause_catalog.is_empty() {
        return (
            TelescopeEnumeration::default(),
            TelescopeEnumerationProgress::default(),
        );
    }

    let mut telescopes = Vec::new();
    let mut terminal_prefixes = Vec::new();
    let mut prefix = Vec::new();
    let mut progress = TelescopeEnumerationProgress::default();
    on_progress(TelescopeEnumerationProgressEvent::EnumerationHandoff(
        progress,
    ));
    enumerate_telescopes_dfs(
        library,
        clause_catalog.clause_kappa(),
        &clause_catalog.options_by_position,
        &mut prefix,
        &mut telescopes,
        &mut terminal_prefixes,
        &mut progress,
        &mut on_progress,
    );
    telescopes.sort_by_key(|telescope| serde_json::to_string(telescope).expect("serialize"));
    terminal_prefixes.sort_by_key(|telescope| serde_json::to_string(telescope).expect("serialize"));
    terminal_prefixes.dedup();
    (
        TelescopeEnumeration {
            telescopes,
            terminal_prefixes,
        },
        progress,
    )
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
    raw_clause_catalog_widths_with_progress(base_context, clause_kappa, |_, _| {})
}

pub(crate) fn raw_clause_catalog_widths_with_progress<F>(
    base_context: EnumerationContext,
    clause_kappa: u16,
    mut on_progress: F,
) -> Vec<usize>
where
    F: FnMut(RawClauseCatalogWidthProgress, &[usize]),
{
    let mut widths = Vec::with_capacity(usize::from(clause_kappa));
    for position in 0..usize::from(clause_kappa) {
        on_progress(
            RawClauseCatalogWidthProgress::PositionStarted { position },
            widths.as_slice(),
        );
        let width = raw_clause_width_for_position_with_progress(
            base_context,
            clause_kappa,
            position,
            |progress| on_progress(progress, widths.as_slice()),
        );
        widths.push(width);
        on_progress(
            RawClauseCatalogWidthProgress::PositionReady { position, width },
            widths.as_slice(),
        );
    }
    widths
}

type ExactExprCache = BTreeMap<u8, Rc<[Expr]>>;
type RawExprCache = BTreeMap<u8, Rc<[Expr]>>;

fn leaf_expr_count(context: EnumerationContext) -> usize {
    let mut count = 1usize + context.scope_size as usize;
    if context.library_size > 0 {
        let start = context.library_size.saturating_sub(1).max(1);
        count += (context.library_size - start + 1) as usize;
        if let Some(anchor) = context.historical_anchor_ref {
            if (1..=context.library_size).contains(&anchor) && anchor < start {
                count += 1;
            }
        }
    }
    count + context.max_path_dimension as usize
}

fn unary_variant_count(context: EnumerationContext) -> usize {
    1usize
        + usize::from(context.include_trunc)
        + (4 * usize::from(context.include_modal))
        + (2 * usize::from(context.include_temporal))
        + (2 * usize::from(context.include_linear_exponential))
}

fn exact_expr_count(context: EnumerationContext, nodes: u8) -> usize {
    let mut count = 0usize;

    if nodes == 1 {
        count = leaf_expr_count(context);
    }

    if nodes >= 2 {
        count = count.saturating_add(
            exact_expr_count(context, nodes - 1).saturating_mul(unary_variant_count(context)),
        );
    }

    if nodes >= 3 {
        for left_nodes in 1..=nodes - 2 {
            let right_nodes = nodes - 1 - left_nodes;
            count = count.saturating_add(
                exact_expr_count(context, left_nodes)
                    .saturating_mul(exact_expr_count(context, right_nodes))
                    .saturating_mul(3),
            );
        }
    }

    count
}

pub fn enumerate_telescopes_with_terminal_prefixes(
    library: &Library,
    base_context: EnumerationContext,
    clause_kappa: u16,
) -> TelescopeEnumeration {
    enumerate_telescopes_with_terminal_prefixes_and_progress(
        library,
        base_context,
        clause_kappa,
        |_| {},
    )
    .0
}

pub fn enumerate_telescopes_with_terminal_prefixes_and_progress<F>(
    library: &Library,
    base_context: EnumerationContext,
    clause_kappa: u16,
    on_progress: F,
) -> (TelescopeEnumeration, TelescopeEnumerationProgress)
where
    F: FnMut(TelescopeEnumerationProgressEvent),
{
    enumerate_telescopes_with_terminal_prefixes_and_progress_with_raw_catalog_widths(
        library,
        base_context,
        clause_kappa,
        None,
        on_progress,
    )
}

pub fn build_clause_catalog(base_context: EnumerationContext, clause_kappa: u16) -> ClauseCatalog {
    build_clause_catalog_with_progress(base_context, clause_kappa, |_| {})
}

fn build_clause_catalog_with_progress<F>(
    base_context: EnumerationContext,
    clause_kappa: u16,
    mut on_progress: F,
) -> ClauseCatalog
where
    F: FnMut(ClauseCatalogBuildProgress),
{
    build_clause_catalog_with_progress_and_raw_catalog_widths(
        base_context,
        clause_kappa,
        None,
        |progress| on_progress(progress),
    )
}

fn build_clause_catalog_with_progress_and_raw_catalog_widths<F>(
    base_context: EnumerationContext,
    clause_kappa: u16,
    raw_catalog_clause_widths: Option<&[usize]>,
    mut on_progress: F,
) -> ClauseCatalog
where
    F: FnMut(ClauseCatalogBuildProgress),
{
    if base_context.require_curvature_shell_clauses && clause_kappa < 6 {
        return ClauseCatalog::default();
    }
    if let Some(widths) = raw_catalog_clause_widths {
        debug_assert_eq!(widths.len(), usize::from(clause_kappa));
    }

    let mut options_by_position = Vec::with_capacity(usize::from(clause_kappa));
    let mut terminal_connectivity_facts_by_position = Vec::with_capacity(usize::from(clause_kappa));
    let mut terminal_nu_facts_by_position = Vec::with_capacity(usize::from(clause_kappa));
    for position in 0..usize::from(clause_kappa) {
        on_progress(ClauseCatalogBuildProgress::PositionStarted { position });
        // The raw-width probe uses the widest scope at every position, so its
        // reported width is an exact clause count only at the terminal slot.
        let raw_clause_width_hint = if position + 1 == usize::from(clause_kappa) {
            raw_catalog_clause_widths
                .and_then(|widths| widths.get(position))
                .copied()
        } else {
            None
        };
        let clauses = clauses_for_position_with_progress(
            base_context,
            clause_kappa,
            position,
            raw_clause_width_hint,
            |progress| {
                on_progress(progress);
            },
        );
        let clause_count = clauses.len();
        // Large positions keep the catalog-level connectivity side table
        // deferred so clause materialization does not retain a second
        // 1:1 allocation before any remaining-one path asks for it.
        let connectivity_facts = TerminalConnectivityFactsByPosition::for_clauses(&clauses);
        on_progress(ClauseCatalogBuildProgress::PositionConnectivityFactsReady {
            position,
            clause_count,
        });
        let nu_facts = clauses
            .iter()
            .map(TerminalClauseNuFacts::from_clause)
            .collect::<Vec<_>>();
        on_progress(ClauseCatalogBuildProgress::PositionNuFactsReady {
            position,
            clause_count,
        });
        options_by_position.push(clauses);
        terminal_connectivity_facts_by_position.push(connectivity_facts);
        terminal_nu_facts_by_position.push(nu_facts);
        on_progress(ClauseCatalogBuildProgress::PositionReady {
            position,
            clause_count,
        });
    }

    ClauseCatalog {
        clause_kappa,
        options_by_position,
        terminal_connectivity_facts_by_position,
        terminal_nu_facts_by_position,
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn build_clause_catalog_from_options(
    clause_kappa: u16,
    options_by_position: Vec<Vec<ClauseRec>>,
) -> ClauseCatalog {
    assert_eq!(options_by_position.len(), usize::from(clause_kappa));

    let mut terminal_connectivity_facts_by_position = Vec::with_capacity(usize::from(clause_kappa));
    let mut terminal_nu_facts_by_position = Vec::with_capacity(usize::from(clause_kappa));
    for clauses in &options_by_position {
        let connectivity_facts = TerminalConnectivityFactsByPosition::for_clauses(clauses);
        let nu_facts = clauses
            .iter()
            .map(TerminalClauseNuFacts::from_clause)
            .collect::<Vec<_>>();
        terminal_connectivity_facts_by_position.push(connectivity_facts);
        terminal_nu_facts_by_position.push(nu_facts);
    }

    ClauseCatalog {
        clause_kappa,
        options_by_position,
        terminal_connectivity_facts_by_position,
        terminal_nu_facts_by_position,
    }
}

#[cfg(test)]
pub(crate) fn build_clause_catalog_from_options_with_deferred_connectivity_facts(
    clause_kappa: u16,
    options_by_position: Vec<Vec<ClauseRec>>,
    deferred_positions: &[usize],
) -> ClauseCatalog {
    assert_eq!(options_by_position.len(), usize::from(clause_kappa));

    let mut terminal_connectivity_facts_by_position = Vec::with_capacity(usize::from(clause_kappa));
    let mut terminal_nu_facts_by_position = Vec::with_capacity(usize::from(clause_kappa));
    for (position, clauses) in options_by_position.iter().enumerate() {
        let connectivity_facts = if deferred_positions.contains(&position) {
            TerminalConnectivityFactsByPosition::Deferred
        } else {
            TerminalConnectivityFactsByPosition::for_clauses(clauses)
        };
        let nu_facts = clauses
            .iter()
            .map(TerminalClauseNuFacts::from_clause)
            .collect::<Vec<_>>();
        terminal_connectivity_facts_by_position.push(connectivity_facts);
        terminal_nu_facts_by_position.push(nu_facts);
    }

    ClauseCatalog {
        clause_kappa,
        options_by_position,
        terminal_connectivity_facts_by_position,
        terminal_nu_facts_by_position,
    }
}

fn clauses_for_position_with_progress<F>(
    base_context: EnumerationContext,
    clause_kappa: u16,
    position: usize,
    raw_clause_width_hint: Option<usize>,
    mut on_progress: F,
) -> Vec<ClauseRec>
where
    F: FnMut(ClauseCatalogBuildProgress),
{
    let clause_context = EnumerationContext {
        scope_size: base_context.scope_size + position as u32,
        ..base_context
    };
    if let Some(clauses) = late_clause_options(position, clause_context, clause_kappa) {
        return filter_clauses_for_position(
            clauses,
            base_context,
            clause_context,
            clause_kappa,
            position,
        );
    }

    let mut cache = ExactExprCache::new();
    let mut clauses = raw_clause_width_hint.map_or_else(Vec::new, Vec::with_capacity);
    for expr_nodes in 1..=clause_context.max_expr_nodes {
        let stream_exact_bucket = expr_nodes == clause_context.max_expr_nodes;
        if stream_exact_bucket {
            let expr_count = exact_expr_count(clause_context, expr_nodes);
            on_progress(ClauseCatalogBuildProgress::PositionExprNodesGenerated {
                position,
                expr_nodes,
                max_expr_nodes: clause_context.max_expr_nodes,
                expr_count,
            });
            if raw_clause_width_hint.is_none() {
                clauses.reserve_exact(expr_count);
            }
            let mut scanned_expr_count = 0usize;
            if expr_count >= MIN_STREAMED_EXACT_EXPR_BUCKET_SIZE {
                stream_exact_exprs(clause_context, expr_nodes, &mut cache, |expr| {
                    scanned_expr_count += 1;
                    if raw_clause_matches_context(&expr, clause_context)
                        && raw_clause_matches_position(base_context, clause_kappa, position, &expr)
                    {
                        let role = primary_role(&expr);
                        clauses.push(ClauseRec::new(role, expr));
                    }
                    if scanned_expr_count % CLAUSE_MATERIALIZATION_PROGRESS_CHUNK == 0
                        && scanned_expr_count < expr_count
                    {
                        on_progress(
                            ClauseCatalogBuildProgress::PositionExprNodesAccumulationProgress {
                                position,
                                expr_nodes,
                                max_expr_nodes: clause_context.max_expr_nodes,
                                scanned_expr_count,
                                clause_count_so_far: clauses.len(),
                            },
                        );
                    }
                });
            } else {
                let exprs = enumerate_exprs_exact(clause_context, expr_nodes, &mut cache);
                debug_assert_eq!(exprs.len(), expr_count);
                for expr in exprs.iter() {
                    scanned_expr_count += 1;
                    if raw_clause_matches_context(expr, clause_context)
                        && raw_clause_matches_position(base_context, clause_kappa, position, expr)
                    {
                        clauses.push(ClauseRec::new(primary_role(expr), expr.clone()));
                    }
                    if scanned_expr_count % CLAUSE_MATERIALIZATION_PROGRESS_CHUNK == 0
                        && scanned_expr_count < expr_count
                    {
                        on_progress(
                            ClauseCatalogBuildProgress::PositionExprNodesAccumulationProgress {
                                position,
                                expr_nodes,
                                max_expr_nodes: clause_context.max_expr_nodes,
                                scanned_expr_count,
                                clause_count_so_far: clauses.len(),
                            },
                        );
                    }
                }
            }
            debug_assert_eq!(scanned_expr_count, expr_count);
        } else {
            let exprs = enumerate_exprs_exact(clause_context, expr_nodes, &mut cache);
            let expr_count = exprs.len();
            on_progress(ClauseCatalogBuildProgress::PositionExprNodesGenerated {
                position,
                expr_nodes,
                max_expr_nodes: clause_context.max_expr_nodes,
                expr_count,
            });
            if raw_clause_width_hint.is_none() {
                clauses.reserve_exact(expr_count);
            }
            let mut scanned_expr_count = 0usize;
            for expr in exprs.iter() {
                scanned_expr_count += 1;
                if raw_clause_matches_context(expr, clause_context)
                    && raw_clause_matches_position(base_context, clause_kappa, position, expr)
                {
                    clauses.push(ClauseRec::new(primary_role(expr), expr.clone()));
                }
                if scanned_expr_count % CLAUSE_MATERIALIZATION_PROGRESS_CHUNK == 0
                    && scanned_expr_count < expr_count
                {
                    on_progress(
                        ClauseCatalogBuildProgress::PositionExprNodesAccumulationProgress {
                            position,
                            expr_nodes,
                            max_expr_nodes: clause_context.max_expr_nodes,
                            scanned_expr_count,
                            clause_count_so_far: clauses.len(),
                        },
                    );
                }
            }
        };
        if expr_nodes == clause_context.max_expr_nodes {
            cache.clear();
        }
        on_progress(ClauseCatalogBuildProgress::PositionExprNodesReady {
            position,
            expr_nodes,
            max_expr_nodes: clause_context.max_expr_nodes,
            clause_count_so_far: clauses.len(),
        });
    }
    // Exact-node buckets are already unique and disjoint, so the fallback path
    // only needs one final deterministic ordering pass after filtering.
    on_progress(ClauseCatalogBuildProgress::PositionSortStarted {
        position,
        clause_count: clauses.len(),
    });
    sort_clauses_in_place(&mut clauses);
    on_progress(ClauseCatalogBuildProgress::PositionSorted {
        position,
        clause_count: clauses.len(),
    });
    clauses
}

fn filter_clauses_for_position(
    clauses: Vec<ClauseRec>,
    base_context: EnumerationContext,
    clause_context: EnumerationContext,
    clause_kappa: u16,
    position: usize,
) -> Vec<ClauseRec> {
    dedupe_sorted_clauses(
        clauses
            .into_iter()
            .filter(|clause| {
                raw_clause_matches_context(&clause.expr, clause_context)
                    && raw_clause_matches_position(
                        base_context,
                        clause_kappa,
                        position,
                        &clause.expr,
                    )
            })
            .collect(),
    )
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
    clauses.retain(|clause| {
        raw_clause_matches_position(base_context, clause_kappa, position, &clause.expr)
    });
    clauses
}

fn raw_clause_width_for_position_with_progress<F>(
    base_context: EnumerationContext,
    clause_kappa: u16,
    position: usize,
    mut on_progress: F,
) -> usize
where
    F: FnMut(RawClauseCatalogWidthProgress),
{
    let clause_context = EnumerationContext {
        scope_size: base_context
            .scope_size
            .saturating_add(u32::from(clause_kappa).saturating_sub(1)),
        ..base_context
    };
    if let Some(clauses) = late_clause_options(position, clause_context, clause_kappa) {
        return clauses
            .into_iter()
            .filter(|clause| {
                raw_clause_matches_position(base_context, clause_kappa, position, &clause.expr)
            })
            .count();
    }

    let mut cache = RawExprCache::new();
    let mut width = 0usize;
    for expr_nodes in 1..=clause_context.max_expr_nodes {
        let exprs = enumerate_exprs_exact_raw(clause_context, expr_nodes, &mut cache);
        width += exprs
            .iter()
            .filter(|expr| raw_clause_matches_context(expr, clause_context))
            .filter(|expr| raw_clause_matches_position(base_context, clause_kappa, position, expr))
            .count();
        on_progress(RawClauseCatalogWidthProgress::PositionExprNodesReady {
            position,
            expr_nodes,
            max_expr_nodes: clause_context.max_expr_nodes,
            width_so_far: width,
        });
    }
    width
}

fn raw_clause_matches_context(expr: &Expr, context: EnumerationContext) -> bool {
    (!context.require_former_eliminator_clauses || supports_former_eliminator_clause(expr))
        && (!context.require_initial_hit_clauses
            || supports_initial_hit_clause(expr, context.late_family_surface))
        && (!context.require_truncation_hit_clauses
            || supports_truncation_hit_clause(expr, context.late_family_surface))
        && (!context.require_higher_hit_clauses
            || supports_higher_hit_clause(expr, context.late_family_surface))
        && (!context.require_sphere_lift_clauses
            || supports_sphere_lift_clause(expr, context.late_family_surface))
        && (!context.require_axiomatic_bundle_clauses || supports_axiomatic_bundle_clause(expr))
        && (!context.require_modal_shell_clauses || supports_modal_shell_clause(expr))
        && (!context.require_connection_shell_clauses || supports_connection_shell_clause(expr))
        && (!context.require_curvature_shell_clauses || supports_curvature_shell_clause(expr))
        && (!context.require_operator_bundle_clauses || supports_operator_bundle_clause(expr))
        && (!context.require_hilbert_functional_clauses || supports_hilbert_functional_clause(expr))
        && (!context.require_temporal_shell_clauses
            || supports_temporal_shell_clause(
                expr,
                context.include_linear_exponential,
            ))
}

fn raw_clause_matches_position(
    base_context: EnumerationContext,
    clause_kappa: u16,
    position: usize,
    expr: &Expr,
) -> bool {
    (!base_context.require_former_eliminator_clauses
        || supports_former_package_clause_at_position(position, expr))
        && (!base_context.require_initial_hit_clauses
            || supports_initial_hit_clause_at_position(
                position,
                expr,
                base_context.late_family_surface,
            ))
        && (!base_context.require_truncation_hit_clauses
            || supports_truncation_hit_clause_at_position(
                position,
                expr,
                base_context.late_family_surface,
            ))
        && (!base_context.require_higher_hit_clauses
            || supports_higher_hit_clause_at_position(
                position,
                expr,
                base_context.late_family_surface,
            ))
        && (!base_context.require_sphere_lift_clauses
            || supports_sphere_lift_clause_at_position(
                position,
                expr,
                base_context.late_family_surface,
            ))
        && (!base_context.require_axiomatic_bundle_clauses
            || supports_axiomatic_bundle_clause_at_position(
                position,
                expr,
                base_context.library_size,
                base_context.historical_anchor_ref,
                base_context.late_family_surface,
            ))
        && (!base_context.require_modal_shell_clauses
            || supports_modal_shell_clause_at_position(position, expr))
        && (!base_context.require_connection_shell_clauses
            || supports_connection_shell_clause_at_position(
                position,
                expr,
                base_context.library_size,
            ))
        && (!base_context.require_curvature_shell_clauses
            || supports_curvature_shell_clause_at_position(
                position,
                expr,
                base_context.library_size,
            ))
        && (!(base_context.require_operator_bundle_clauses
            || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 7))
            || supports_operator_bundle_clause_at_position(
                position,
                expr,
                base_context.library_size,
                base_context.late_family_surface,
            ))
        && (!(base_context.require_hilbert_functional_clauses
            || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 9))
            || supports_hilbert_functional_clause_at_position(
                position,
                expr,
                base_context.library_size,
                base_context.late_family_surface,
            ))
        && (!(base_context.require_temporal_shell_clauses
            || (base_context.late_family_surface != LateFamilySurface::None && clause_kappa == 8))
            || supports_temporal_shell_clause_at_position(
                position,
                expr,
                base_context.historical_anchor_ref,
                base_context.include_linear_exponential,
                base_context.late_family_surface,
            ))
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
            context.include_linear_exponential,
            context.late_family_surface,
        ),
    }
}

pub fn enumerate_exprs(context: EnumerationContext) -> Vec<Expr> {
    let mut cache = ExactExprCache::new();
    let mut all = Vec::new();
    for nodes in 1..=context.max_expr_nodes {
        let exprs = enumerate_exprs_exact(context, nodes, &mut cache);
        all.extend(exprs.iter().cloned());
    }
    unique_sorted_exprs(all)
}

fn enumerate_exprs_raw(context: EnumerationContext) -> Vec<Expr> {
    let mut cache = RawExprCache::new();
    let mut all = Vec::new();
    for nodes in 1..=context.max_expr_nodes {
        let exprs = enumerate_exprs_exact_raw(context, nodes, &mut cache);
        all.extend(exprs.iter().cloned());
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
    progress: &mut TelescopeEnumerationProgress,
    on_progress: &mut impl FnMut(TelescopeEnumerationProgressEvent),
) {
    if remaining == 0 {
        let telescope = Telescope::new(prefix.clone());
        if check_telescope(library, &telescope) == CheckResult::Ok {
            let witness = analyze_connectivity(library, &telescope);
            if !witness.connected {
                progress.dfs_leaf_rejections += 1;
                progress.dfs_leaf_connectivity_rejections += 1;
                progress.dfs_leaf_disconnected_rejections += 1;
            } else if witness.references_active_window
                || witness.self_contained
                || witness.historical_reanchor
            {
                out.push(telescope);
                progress.completed_telescopes += 1;
            } else {
                progress.dfs_leaf_rejections += 1;
                progress.dfs_leaf_connectivity_rejections += 1;
                progress.dfs_leaf_connected_unqualified_rejections += 1;
            }
        } else {
            progress.dfs_leaf_rejections += 1;
            progress.dfs_leaf_check_rejections += 1;
        }
        maybe_report_telescope_enumeration_progress(*progress, on_progress);
        return;
    }

    let position = prefix.len();
    for clause in &clause_options_by_position[position] {
        prefix.push(clause.clone());
        progress.prefix_attempts += 1;
        let partial = Telescope::new(prefix.clone());
        maybe_report_telescope_enumeration_progress(*progress, on_progress);
        if check_telescope(library, &partial) == CheckResult::Ok {
            progress.prefix_states_explored += 1;
            if remaining == 2 {
                terminal_prefixes.push(partial.clone());
                progress.terminal_prefixes += 1;
            }
            maybe_report_telescope_enumeration_progress(*progress, on_progress);
            enumerate_telescopes_dfs(
                library,
                remaining - 1,
                clause_options_by_position,
                prefix,
                out,
                terminal_prefixes,
                progress,
                on_progress,
            );
        } else {
            progress.dfs_prefix_rejections += 1;
            maybe_report_telescope_enumeration_progress(*progress, on_progress);
        }
        prefix.pop();
    }
}

fn maybe_report_telescope_enumeration_progress(
    progress: TelescopeEnumerationProgress,
    on_progress: &mut impl FnMut(TelescopeEnumerationProgressEvent),
) {
    if progress.prefix_states_explored.is_power_of_two()
        || progress.prefix_attempts.is_power_of_two()
        || progress.terminal_prefixes.is_power_of_two()
        || progress.dfs_prefix_rejections.is_power_of_two()
        || progress.dfs_leaf_rejections.is_power_of_two()
        || progress.dfs_leaf_check_rejections.is_power_of_two()
        || progress.dfs_leaf_connectivity_rejections.is_power_of_two()
        || progress.dfs_leaf_disconnected_rejections.is_power_of_two()
        || progress
            .dfs_leaf_connected_unqualified_rejections
            .is_power_of_two()
        || progress.completed_telescopes.is_power_of_two()
    {
        on_progress(TelescopeEnumerationProgressEvent::Enumeration(progress));
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
    cache: &mut ExactExprCache,
) -> Rc<[Expr]> {
    if let Some(cached) = cache.get(&nodes) {
        return Rc::clone(cached);
    }

    // Exact-node generation is structurally unique, so sizing once up front
    // avoids the allocator doubling into a much wider final slab.
    let expected_len = exact_expr_count(context, nodes);
    let mut exprs = Vec::with_capacity(expected_len);

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
        for body in subexprs.iter() {
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
                exprs.push(Expr::Eventually(Box::new(body.clone())));
            }
            if context.include_linear_exponential {
                exprs.push(Expr::Bang(Box::new(body.clone())));
                exprs.push(Expr::WhyNot(Box::new(body.clone())));
            }
        }
    }

    if nodes >= 3 {
        for left_nodes in 1..=nodes - 2 {
            let right_nodes = nodes - 1 - left_nodes;
            let left_exprs = enumerate_exprs_exact(context, left_nodes, cache);
            let right_exprs = enumerate_exprs_exact(context, right_nodes, cache);
            for left in left_exprs.iter() {
                for right in right_exprs.iter() {
                    exprs.push(Expr::App(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Pi(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Sigma(Box::new(left.clone()), Box::new(right.clone())));
                }
            }
        }
    }

    let unique: Rc<[Expr]> = unique_sorted_exprs(exprs).into();
    debug_assert_eq!(unique.len(), expected_len);
    cache.insert(nodes, Rc::clone(&unique));
    unique
}

fn stream_exact_exprs<F>(
    context: EnumerationContext,
    nodes: u8,
    cache: &mut ExactExprCache,
    mut on_expr: F,
) where
    F: FnMut(Expr),
{
    if let Some(cached) = cache.get(&nodes) {
        for expr in cached.iter() {
            on_expr(expr.clone());
        }
        return;
    }

    if nodes == 1 {
        on_expr(Expr::Univ);
        for index in 1..=context.scope_size {
            on_expr(Expr::Var(index));
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
                on_expr(Expr::Lib(index));
            }
        }
        for dimension in 1..=context.max_path_dimension {
            on_expr(Expr::PathCon(dimension));
        }
    }

    if nodes >= 2 {
        let subexprs = enumerate_exprs_exact(context, nodes - 1, cache);
        for body in subexprs.iter() {
            on_expr(Expr::Lam(Box::new(body.clone())));
            if context.include_trunc {
                on_expr(Expr::Trunc(Box::new(body.clone())));
            }
            if context.include_modal {
                on_expr(Expr::Flat(Box::new(body.clone())));
                on_expr(Expr::Sharp(Box::new(body.clone())));
                on_expr(Expr::Disc(Box::new(body.clone())));
                on_expr(Expr::Shape(Box::new(body.clone())));
            }
            if context.include_temporal {
                on_expr(Expr::Next(Box::new(body.clone())));
                on_expr(Expr::Eventually(Box::new(body.clone())));
            }
            if context.include_linear_exponential {
                on_expr(Expr::Bang(Box::new(body.clone())));
                on_expr(Expr::WhyNot(Box::new(body.clone())));
            }
        }
    }

    if nodes >= 3 {
        for left_nodes in 1..=nodes - 2 {
            let right_nodes = nodes - 1 - left_nodes;
            let left_exprs = enumerate_exprs_exact(context, left_nodes, cache);
            let right_exprs = enumerate_exprs_exact(context, right_nodes, cache);
            for left in left_exprs.iter() {
                for right in right_exprs.iter() {
                    on_expr(Expr::App(Box::new(left.clone()), Box::new(right.clone())));
                    on_expr(Expr::Pi(Box::new(left.clone()), Box::new(right.clone())));
                    on_expr(Expr::Sigma(Box::new(left.clone()), Box::new(right.clone())));
                }
            }
        }
    }
}

fn enumerate_exprs_exact_raw(
    context: EnumerationContext,
    nodes: u8,
    cache: &mut RawExprCache,
) -> Rc<[Expr]> {
    if let Some(cached) = cache.get(&nodes) {
        return Rc::clone(cached);
    }

    let expected_len = exact_expr_count(context, nodes);
    let mut exprs = Vec::with_capacity(expected_len);

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
        for body in subexprs.iter() {
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
                exprs.push(Expr::Eventually(Box::new(body.clone())));
            }
            if context.include_linear_exponential {
                exprs.push(Expr::Bang(Box::new(body.clone())));
                exprs.push(Expr::WhyNot(Box::new(body.clone())));
            }
        }
    }

    if nodes >= 3 {
        for left_nodes in 1..=nodes - 2 {
            let right_nodes = nodes - 1 - left_nodes;
            let left_exprs = enumerate_exprs_exact_raw(context, left_nodes, cache);
            let right_exprs = enumerate_exprs_exact_raw(context, right_nodes, cache);
            for left in left_exprs.iter() {
                for right in right_exprs.iter() {
                    exprs.push(Expr::App(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Pi(Box::new(left.clone()), Box::new(right.clone())));
                    exprs.push(Expr::Sigma(Box::new(left.clone()), Box::new(right.clone())));
                }
            }
        }
    }

    debug_assert_eq!(exprs.len(), expected_len);
    let exprs: Rc<[Expr]> = exprs.into();
    cache.insert(nodes, Rc::clone(&exprs));
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
        | Expr::Bang(_)
        | Expr::WhyNot(_)
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

#[cfg_attr(not(test), allow(dead_code))]
fn compare_expr_sort_order(left: &Expr, right: &Expr) -> Ordering {
    atom_rank(left.atom())
        .cmp(&atom_rank(right.atom()))
        .then_with(|| expr_bit_length(left).cmp(&expr_bit_length(right)))
        .then_with(|| {
            serde_json::to_string(left)
                .expect("expr should serialize")
                .cmp(&serde_json::to_string(right).expect("expr should serialize"))
        })
}

fn atom_rank(atom: Atom) -> u8 {
    atom as u8
}

fn supports_former_eliminator_clause(expr: &Expr) -> bool {
    !expr.has_lib_pointer()
        && !expr.is_modal()
        && !expr.is_temporal_like()
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

fn supports_temporal_shell_clause(expr: &Expr, include_linear_exponential: bool) -> bool {
    matches_temporal_shell_left_var(expr, include_linear_exponential, 1)
        || matches_temporal_shell_right_var(expr, include_linear_exponential, 1)
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches_temporal_shell_left_var(
                    domain.as_ref(),
                    include_linear_exponential,
                    1,
                ) && matches_temporal_shell_right_var(
                    codomain.as_ref(),
                    include_linear_exponential,
                    1,
                )
        )
        || matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches!(function.as_ref(), Expr::Lib(_))
                            && matches_temporal_shell_left_var(
                                argument.as_ref(),
                                include_linear_exponential,
                                1,
                            )
                ) || matches!(
                    body.as_ref(),
                    Expr::App(function, argument)
                        if matches_temporal_shell_right_var(
                            function.as_ref(),
                            include_linear_exponential,
                            1,
                        ) && matches!(argument.as_ref(), Expr::Var(_))
                )
        )
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Flat(body)
                        if matches_temporal_shell_left_var(
                            body.as_ref(),
                            include_linear_exponential,
                            1,
                        )
                ) && temporal_shell_left_body(codomain.as_ref(), include_linear_exponential)
                    .is_some_and(|body| {
                        matches!(body, Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                    })
        )
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(
                    domain.as_ref(),
                    Expr::Sharp(body)
                        if matches_temporal_shell_right_var(
                            body.as_ref(),
                            include_linear_exponential,
                            1,
                        )
                ) && temporal_shell_right_body(codomain.as_ref(), include_linear_exponential)
                    .is_some_and(|body| {
                        matches!(body, Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                    })
        )
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if temporal_shell_left_body(domain.as_ref(), include_linear_exponential)
                    .is_some_and(|body| {
                        matches_temporal_shell_left_var(body, include_linear_exponential, 1)
                    }) && matches_temporal_shell_left_var(
                        codomain.as_ref(),
                        include_linear_exponential,
                        1,
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
            include_linear_exponential: false,
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
            include_linear_exponential: false,
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

    if late_family_surface == LateFamilySurface::ClaimGeneric {
        let context = EnumerationContext {
            library_size,
            scope_size: 0,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            include_linear_exponential: false,
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
        return claim_generic_band7_clauses(position, context)
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
            include_linear_exponential: false,
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

    if late_family_surface == LateFamilySurface::ClaimGeneric {
        let context = EnumerationContext {
            library_size,
            scope_size: 0,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: false,
            include_temporal: false,
            include_linear_exponential: false,
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
        return claim_generic_band9_clauses(position, context)
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
    include_linear_exponential: bool,
    late_family_surface: LateFamilySurface,
) -> bool {
    let Some(anchor) = historical_anchor_ref else {
        return false;
    };
    let context = EnumerationContext {
        library_size: anchor.max(1),
        scope_size: 0,
        max_path_dimension: 0,
        include_trunc: false,
        include_modal: true,
        include_temporal: !include_linear_exponential,
        include_linear_exponential,
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
    temporal_shell_family_clauses(position, context)
        .into_iter()
        .any(|candidate| candidate == *expr)
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
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => contains_former_expr(body),
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
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => contains_lambda_expr(body),
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
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => contains_eliminator_expr(body),
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
        EnumerationContext, LateFamilySurface, build_clause_catalog, clause_sort_key,
        compare_clause_sort_order, enumerate_exprs, enumerate_next_clauses,
        enumerate_raw_telescopes, enumerate_telescopes, raw_clause_catalog_widths,
        raw_clause_catalog_widths_with_progress, supports_axiomatic_bundle_clause_at_position,
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
        AdmissibilityMode, PackagePolicy, StrictAdmissibility, StructuralFamily,
        strict_admissibility, strict_admissibility_for_mode,
    };
    use std::collections::BTreeMap;

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
    fn clause_sort_comparator_matches_cached_key_order() {
        let clauses = vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Var(2)),
                ))),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(2))),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2))),
            ),
            ClauseRec::new(ClauseRole::Formation, Expr::Var(10)),
            ClauseRec::new(ClauseRole::Formation, Expr::Var(2)),
        ];
        let mut expected = clauses.clone();
        expected.sort_by_cached_key(clause_sort_key);
        let mut actual = clauses;
        actual.sort_by(compare_clause_sort_order);
        assert_eq!(actual, expected);
    }

    #[test]
    fn exact_expr_count_matches_materialized_len_for_small_context() {
        let context = EnumerationContext {
            library_size: 4,
            scope_size: 3,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: true,
            include_temporal: false,
            include_linear_exponential: false,
            max_expr_nodes: 5,
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
            historical_anchor_ref: Some(2),
            late_family_surface: LateFamilySurface::None,
        };
        let mut cache = super::ExactExprCache::new();

        for nodes in 1..=context.max_expr_nodes {
            let exprs = super::enumerate_exprs_exact(context, nodes, &mut cache);
            assert_eq!(exprs.len(), super::exact_expr_count(context, nodes));
        }
    }

    #[test]
    fn exact_expr_count_matches_live_no_temporal_position_seven_counts() {
        let library = library_until(14);
        let admissibility = strict_admissibility(15, 2, &library);
        let mut context = context_from_admissibility(&library, admissibility);
        context.scope_size += 7;
        context.include_temporal = false;

        let counts = (1..=7)
            .map(|nodes| super::exact_expr_count(context, nodes))
            .collect::<Vec<_>>();

        assert_eq!(counts, vec![13, 65, 832, 9230, 123721, 1663025, 23641735]);
    }

    #[test]
    fn streamed_exact_expr_bucket_matches_materialized_small_context() {
        let context = EnumerationContext {
            library_size: 4,
            scope_size: 3,
            max_path_dimension: 0,
            include_trunc: false,
            include_modal: true,
            include_temporal: false,
            include_linear_exponential: false,
            max_expr_nodes: 5,
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
            historical_anchor_ref: Some(2),
            late_family_surface: LateFamilySurface::None,
        };
        let expected = super::enumerate_exprs_exact(
            context,
            context.max_expr_nodes,
            &mut super::ExactExprCache::new(),
        );
        let mut actual = Vec::new();

        super::stream_exact_exprs(
            context,
            context.max_expr_nodes,
            &mut super::ExactExprCache::new(),
            |expr| actual.push(expr),
        );
        actual.sort_by(super::compare_expr_sort_order);

        assert_eq!(actual, expected.iter().cloned().collect::<Vec<_>>());
    }

    #[test]
    fn raw_width_hinted_clause_materialization_matches_unhinted_small_context() {
        let library = library_until(0);
        let admissibility = strict_admissibility(1, 2, &library);
        let context = context_from_admissibility(&library, admissibility);
        let position = 1usize;
        let raw_width = super::raw_clause_width_for_position_with_progress(
            context,
            admissibility.min_clause_kappa,
            position,
            |_| {},
        );
        let baseline = super::clauses_for_position_with_progress(
            context,
            admissibility.min_clause_kappa,
            position,
            None,
            |_| {},
        );
        let hinted = super::clauses_for_position_with_progress(
            context,
            admissibility.min_clause_kappa,
            position,
            Some(raw_width),
            |_| {},
        );

        assert_eq!(hinted.len(), raw_width);
        assert_eq!(hinted, baseline);
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
    fn desktop_claim_shadow_step_fifteen_enumeration_context_derivation_stays_claim_generic_until_the_mode_changes()
     {
        let library = library_until(14);
        let live_admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let mut bypassed_temporal_focus = live_admissibility;
        bypassed_temporal_focus.focus_family = Some(StructuralFamily::TemporalShell);
        bypassed_temporal_focus.package_policies.temporal_shell = PackagePolicy::Prefer;
        let mut demo_surface_override = live_admissibility;
        demo_surface_override.mode = AdmissibilityMode::DemoBreadthShadow;

        let live_context = context_from_admissibility(&library, live_admissibility);
        let bypassed_temporal_focus_context =
            context_from_admissibility(&library, bypassed_temporal_focus);
        let demo_surface_override_context =
            context_from_admissibility(&library, demo_surface_override);
        let live_catalog = build_clause_catalog(live_context, 8);
        let bypassed_temporal_focus_catalog =
            build_clause_catalog(bypassed_temporal_focus_context, 8);

        assert_eq!(
            live_context.late_family_surface,
            LateFamilySurface::ClaimGeneric,
            "the live step-15 constructor should still derive ClaimGeneric directly from DesktopClaimShadow mode"
        );
        assert_eq!(
            bypassed_temporal_focus_context, live_context,
            "reintroducing a temporal-shell focus family plus Prefer policy on the same DesktopClaimShadow admissibility should not change the enumeration context until a later explicit surface override fires"
        );
        assert_eq!(
            live_catalog.clauses_at(7),
            bypassed_temporal_focus_catalog.clauses_at(7),
            "the mode-derived claim-generic constructor should keep the same live reference/eventual-lift/next-lift terminal trio even if a bypassed temporal focus is reintroduced upstream"
        );
        assert_eq!(
            live_catalog.clauses_at(7).len(),
            3,
            "the live step-15 claim-generic constructor should keep the already-pinned three-clause remaining-one terminal catalog"
        );
        assert_eq!(
            demo_surface_override_context.late_family_surface,
            LateFamilySurface::DemoBreadthShadow,
            "changing the admissibility mode is the constructor-level mechanism that actually leaves the default claim-generic surface"
        );
    }

    #[test]
    fn desktop_claim_shadow_step_fifteen_claim_generic_band8_raw_catalog_stays_three_wide_across_all_positions()
     {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut realistic_context = claim_context;
        realistic_context.late_family_surface = LateFamilySurface::RealisticShadow;
        let claim_widths = raw_clause_catalog_widths(claim_context, 8);
        let realistic_widths = raw_clause_catalog_widths(realistic_context, 8);
        let claim_raw = enumerate_raw_telescopes(claim_context, 8);
        let realistic_raw = enumerate_raw_telescopes(realistic_context, 8);
        let claim_raw_options = super::raw_clause_options_by_position(claim_context, 8);
        let expected_terminal_family = super::dedupe_sorted_clauses(
            super::claim_generic_band8_clauses(7, claim_context)
                .into_iter()
                .map(|expr| ClauseRec::new(super::primary_role(&expr), expr))
                .collect::<Vec<_>>(),
        );

        assert_eq!(
            claim_widths,
            vec![3, 3, 3, 3, 3, 3, 3, 3],
            "the raw claim-generic band-8 emitter should stay three-wide at every live step-15 position"
        );
        assert_eq!(
            realistic_widths,
            vec![1, 1, 1, 1, 2, 1, 1, 1],
            "the realistic temporal-shell control should stay narrow everywhere except for its already-pinned extra clause at position 4"
        );
        assert_eq!(
            claim_raw.len(),
            6561,
            "three claim-generic choices at all eight positions should keep the live raw band-8 catalog at 3^8 combinations"
        );
        assert_eq!(
            realistic_raw.len(),
            2,
            "the realistic temporal-shell control should still enumerate only the reference path plus its single extra position-4 variant"
        );
        assert_eq!(
            claim_raw_options[7], expected_terminal_family,
            "the raw claim-generic band-8 emitter should materialize the live three-clause terminal family directly at position 7"
        );
    }

    #[test]
    fn desktop_claim_shadow_step_fifteen_late_clause_selector_uses_the_claim_generic_band8_family_until_the_surface_changes()
     {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut realistic_context = claim_context;
        realistic_context.late_family_surface = LateFamilySurface::RealisticShadow;
        let mut demo_context = claim_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;

        for position in 0..8 {
            let selected = super::late_clause_options(position, claim_context, 8)
                .expect("the live step-15 claim context should keep a band-8 late-clause family");
            let expected = super::dedupe_sorted_clauses(
                super::claim_generic_band8_clauses(position, claim_context)
                    .into_iter()
                    .map(|expr| ClauseRec::new(super::primary_role(&expr), expr))
                    .collect::<Vec<_>>(),
            );
            assert_eq!(
                selected, expected,
                "the live step-15 late-clause selector should delegate directly to the raw claim-generic band-8 family at position {position}"
            );
        }

        let claim_terminal_trio = super::late_clause_options(7, claim_context, 8)
            .expect("the live claim surface should expose a terminal trio");
        let realistic_terminal_only = super::late_clause_options(7, realistic_context, 8)
            .expect("the realistic control should expose its temporal-shell terminal clause");
        let demo_terminal_family = super::late_clause_options(7, demo_context, 8)
            .expect("the demo control should expose its wider temporal-shell family");
        let realistic_reference = super::temporal_shell_reference_clause(7, realistic_context)
            .expect("the realistic control should keep its reference terminal clause");

        assert_eq!(claim_terminal_trio.len(), 3);
        assert_eq!(
            realistic_terminal_only,
            vec![ClauseRec::new(
                super::primary_role(&realistic_reference),
                realistic_reference,
            )],
            "changing only the late family surface to RealisticShadow should collapse the live claim-generic terminal trio back to the lone temporal-shell reference clause"
        );
        assert_eq!(
            demo_terminal_family.len(),
            5,
            "changing only the late family surface to DemoBreadthShadow should widen the same terminal position to the demo temporal-shell family"
        );
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
    fn claim_generic_step_eleven_full_enumeration_reports_connected_surface_counts() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut counts = Vec::new();
        let mut total = 0usize;
        let mut final_clause_counts = BTreeMap::new();

        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let telescopes = enumerate_telescopes(&library, claim_context, clause_kappa);
            total += telescopes.len();
            if clause_kappa == 6 {
                for telescope in &telescopes {
                    let last = telescope
                        .clauses
                        .last()
                        .expect("kappa-6 telescope should have a final clause");
                    *final_clause_counts
                        .entry(
                            serde_json::to_string(&last.expr)
                                .expect("final clause expr should serialize"),
                        )
                        .or_insert(0usize) += 1;
                }
            }
            counts.push((clause_kappa, telescopes.len()));
        }

        assert_eq!(counts, vec![(5, 243), (6, 729)]);
        assert_eq!(
            final_clause_counts,
            BTreeMap::from([
                ("{\"Pi\":[{\"Lib\":10},{\"Lib\":10}]}".to_owned(), 243usize,),
                ("{\"Pi\":[{\"Lib\":10},{\"Lib\":9}]}".to_owned(), 243usize,),
                ("{\"Pi\":[{\"Lib\":9},{\"Lib\":10}]}".to_owned(), 243usize,),
            ])
        );
        assert_eq!(total, 972);
    }

    #[test]
    fn claim_generic_kappa_seven_catalog_adds_operator_wrapper_variants() {
        let library = library_until(12);
        let admissibility =
            strict_admissibility_for_mode(13, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut realistic_context = claim_context;
        realistic_context.late_family_surface = LateFamilySurface::RealisticShadow;

        let claim_catalog = build_clause_catalog(claim_context, 7);
        let realistic_catalog = build_clause_catalog(realistic_context, 7);
        let claim_only = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                )),
            ),
        );

        assert!(claim_catalog.clauses_at(0).contains(&claim_only));
        assert!(!realistic_catalog.clauses_at(0).contains(&claim_only));
    }

    #[test]
    fn claim_generic_step_thirteen_scoped_widening_preserves_reference_shell() {
        let library = library_until(12);
        let admissibility =
            strict_admissibility_for_mode(13, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);

        let raw_telescopes = enumerate_raw_telescopes(claim_context, 7);
        assert_eq!(
            raw_clause_catalog_widths(claim_context, 7),
            vec![5, 1, 3, 3, 5, 3, 2]
        );
        assert_eq!(raw_telescopes.len(), 1350);
        assert!(
            raw_telescopes.contains(&Telescope::reference(13)),
            "the scoped step-13 widening should still keep the reference metric shell in the raw claim catalog"
        );
    }

    #[test]
    fn claim_generic_kappa_eight_catalog_adds_modal_temporal_exchange_variants() {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut realistic_context = claim_context;
        realistic_context.late_family_surface = LateFamilySurface::RealisticShadow;

        let claim_catalog = build_clause_catalog(claim_context, 8);
        let realistic_catalog = build_clause_catalog(realistic_context, 8);
        let claim_only = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
        );

        assert!(claim_catalog.clauses_at(0).contains(&claim_only));
        assert!(!realistic_catalog.clauses_at(0).contains(&claim_only));
    }

    #[test]
    fn claim_generic_kappa_nine_catalog_adds_higher_order_binder_variants() {
        let library = library_until(13);
        let admissibility =
            strict_admissibility_for_mode(14, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut realistic_context = claim_context;
        realistic_context.late_family_surface = LateFamilySurface::RealisticShadow;
        let mut demo_context = claim_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;

        let claim_catalog = build_clause_catalog(claim_context, 9);
        let realistic_catalog = build_clause_catalog(realistic_context, 9);
        let demo_catalog = build_clause_catalog(demo_context, 9);
        let claim_only = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(Box::new(Expr::Lib(13)), Box::new(Expr::Lib(12))),
        );

        assert!(claim_catalog.clauses_at(5).contains(&claim_only));
        assert!(!realistic_catalog.clauses_at(5).contains(&claim_only));
        assert!(!demo_catalog.clauses_at(5).contains(&claim_only));
    }

    #[test]
    fn claim_generic_step_fourteen_raw_catalog_is_wider_than_realistic_shadow() {
        let library = library_until(13);
        let admissibility =
            strict_admissibility_for_mode(14, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = context_from_admissibility(&library, admissibility);
        let mut realistic_context = claim_context;
        realistic_context.late_family_surface = LateFamilySurface::RealisticShadow;

        let claim_widths = raw_clause_catalog_widths(claim_context, 9);
        let realistic_widths = raw_clause_catalog_widths(realistic_context, 9);
        let claim_raw = enumerate_raw_telescopes(claim_context, 9);
        let realistic_raw = enumerate_raw_telescopes(realistic_context, 9);

        assert_eq!(claim_widths, vec![3, 3, 3, 3, 3, 3, 3, 3, 3]);
        assert!(
            claim_widths
                .iter()
                .zip(realistic_widths.iter())
                .all(|(claim, realistic)| claim >= realistic)
        );
        assert!(
            claim_raw.len() > realistic_raw.len(),
            "expected claim kappa-9 raw surface to widen beyond realistic shadow, got claim={} vs realistic={}",
            claim_raw.len(),
            realistic_raw.len()
        );
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
    fn claim_generic_late_bands_drop_demo_only_variants() {
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
        let mut demo_thirteen = context_from_admissibility(&step_thirteen_library, step_thirteen);
        demo_thirteen.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let demo_thirteen = build_clause_catalog(demo_thirteen, 7);
        let demo_only_operator = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            ),
        );
        assert!(!claim_thirteen.clauses_at(0).contains(&demo_only_operator));
        assert!(demo_thirteen.clauses_at(0).contains(&demo_only_operator));

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
        let mut demo_fifteen = context_from_admissibility(&step_fifteen_library, step_fifteen);
        demo_fifteen.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let demo_fifteen = build_clause_catalog(demo_fifteen, 8);
        let demo_only_temporal = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        );
        assert!(!claim_fifteen.clauses_at(1).contains(&demo_only_temporal));
        assert!(demo_fifteen.clauses_at(1).contains(&demo_only_temporal));

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
        let mut demo_fourteen = context_from_admissibility(&step_fourteen_library, step_fourteen);
        demo_fourteen.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let demo_fourteen = build_clause_catalog(demo_fourteen, 9);
        let demo_only_hilbert = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(2)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                )),
                Box::new(Expr::Var(1)),
            ),
        );
        assert!(!claim_fourteen.clauses_at(0).contains(&demo_only_hilbert));
        assert!(demo_fourteen.clauses_at(0).contains(&demo_only_hilbert));
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
            include_linear_exponential: false,
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
            include_linear_exponential: false,
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
                include_linear_exponential: false,
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
            include_linear_exponential: false,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
    fn raw_clause_catalog_width_progress_reports_expr_node_checkpoints_on_fallback_surfaces() {
        let library = library_until(0);
        let admissibility = strict_admissibility(1, 2, &library);
        let context = context_from_admissibility(&library, admissibility);
        let mut progress = Vec::new();

        let widths = raw_clause_catalog_widths_with_progress(
            context,
            admissibility.min_clause_kappa,
            |event, widths_so_far| progress.push((event, widths_so_far.to_vec())),
        );

        assert_eq!(widths, vec![36, 36]);
        assert!(
            progress.iter().any(|(event, widths_so_far)| matches!(
                event,
                super::RawClauseCatalogWidthProgress::PositionExprNodesReady {
                    position: 0,
                    ..
                } if widths_so_far.is_empty()
            )),
            "raw width reporting should expose in-position fallback progress before position 0 is complete"
        );
        assert!(
            progress.iter().any(|(event, widths_so_far)| matches!(
                event,
                super::RawClauseCatalogWidthProgress::PositionExprNodesReady {
                    position: 1,
                    ..
                } if widths_so_far == &vec![36]
            )),
            "raw width reporting should keep completed widths separate from the next in-flight position"
        );
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
            false,
            LateFamilySurface::None,
        ));
        assert!(supports_temporal_shell_clause_at_position(
            1,
            &pen_core::expr::Expr::Eventually(Box::new(pen_core::expr::Expr::Var(1))),
            Some(10),
            false,
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
            false,
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
            false,
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
            false,
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
            false,
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
            false,
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
            false,
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
                include_linear_exponential: admissibility.include_linear_exponential,
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
