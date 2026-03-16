use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::{Telescope, TelescopeClass};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConnectivityWitness {
    pub connected: bool,
    pub references_active_window: bool,
    pub self_contained: bool,
    pub max_lib_ref: u32,
    pub historical_reanchor: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ConnectivityClauseState {
    has_lib_pointer: bool,
    is_path_con: bool,
    is_trunc_like_anchor: bool,
    is_higher_path_con: bool,
    is_higher_order_bridge: bool,
    is_operator_action: bool,
    has_prior_operator_bundle_seed: bool,
    later_pathcon_seen: bool,
}

impl ConnectivityClauseState {
    fn new(expr: &Expr, has_prior_operator_bundle_seed: bool) -> Self {
        Self {
            has_lib_pointer: expr.has_lib_pointer(),
            is_path_con: matches!(expr, Expr::PathCon(_)),
            is_trunc_like_anchor: matches!(expr, Expr::Trunc(_))
                || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Trunc(_))),
            is_higher_path_con: matches!(expr, Expr::PathCon(dimension) if *dimension > 1),
            is_higher_order_bridge: is_higher_order_bridge_clause(expr),
            is_operator_action: is_operator_action_clause(expr),
            has_prior_operator_bundle_seed,
            later_pathcon_seen: false,
        }
    }

    fn local_path_anchor(self, has_point_constructor: bool) -> bool {
        (has_point_constructor && !self.is_path_con) || self.is_trunc_like_anchor
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HistoricalReanchorSummary {
    temporal_shell_anchor_ref: Option<u32>,
    temporal_shell_prefix_matches: bool,
    clause_count: usize,
}

impl HistoricalReanchorSummary {
    pub fn from_telescope(library: &Library, telescope: &Telescope) -> Self {
        let mut summary = Self::for_library(library);
        for clause in &telescope.clauses {
            summary = summary.extend(clause);
        }
        summary
    }

    fn for_library(library: &Library) -> Self {
        let temporal_shell_anchor_ref = historical_reanchor_anchor_ref(library);
        Self {
            temporal_shell_anchor_ref,
            temporal_shell_prefix_matches: temporal_shell_anchor_ref.is_some(),
            clause_count: 0,
        }
    }

    pub fn extend(mut self, clause: &ClauseRec) -> Self {
        let position = self.clause_count;
        self.clause_count += 1;

        let Some(anchor) = self.temporal_shell_anchor_ref else {
            self.temporal_shell_prefix_matches = false;
            return self;
        };
        if !self.temporal_shell_prefix_matches {
            return self;
        }

        self.temporal_shell_prefix_matches =
            matches_temporal_shell_clause(position, &clause.expr, anchor);
        self
    }

    pub fn allows_historical_reanchor(self) -> bool {
        self.temporal_shell_anchor_ref.is_some()
            && self.temporal_shell_prefix_matches
            && self.clause_count == 8
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConnectivitySummary {
    clauses: Vec<ConnectivityClauseState>,
    unresolved_indices: BTreeSet<usize>,
    has_point_constructor: bool,
    has_operator_bundle_seed: bool,
    references_active_window: bool,
    self_contained: bool,
    max_lib_ref: u32,
}

impl ConnectivitySummary {
    pub fn from_telescope(library: &Library, telescope: &Telescope) -> Self {
        let mut summary = Self {
            references_active_window: library.len() <= 2,
            self_contained: true,
            ..Self::default()
        };
        for clause in &telescope.clauses {
            summary = summary.extend(library, clause);
        }
        summary
    }

    pub fn extend(mut self, library: &Library, clause: &ClauseRec) -> Self {
        let new_index = self.clauses.len();
        let previous_latest = new_index.checked_sub(1);
        let has_point_constructor_before = self.has_point_constructor;
        let new_expr = &clause.expr;
        let new_clause_has_lib_pointer = new_expr.has_lib_pointer();
        let new_clause_is_path_con = matches!(new_expr, Expr::PathCon(_));
        let new_clause_is_point_constructor = is_point_constructor_expr(new_expr);
        let new_clause_is_higher_path_witness = is_higher_path_witness_clause(new_expr);
        let new_clause_is_operator_bundle_closure = is_operator_bundle_closure_clause(new_expr);
        let new_clause_refs = new_expr.lib_refs();
        let lib_size = library.len() as u32;

        if lib_size <= 2 {
            self.references_active_window = true;
        } else if new_clause_refs.contains(&lib_size)
            || new_clause_refs.contains(&lib_size.saturating_sub(1))
        {
            self.references_active_window = true;
        }

        if !new_clause_refs.is_empty() {
            self.self_contained = false;
            self.max_lib_ref = self
                .max_lib_ref
                .max(new_clause_refs.iter().next_back().copied().unwrap_or(0));
        }

        let mut newly_resolved = Vec::new();
        let candidate_indices = self
            .unresolved_indices
            .iter()
            .copied()
            .chain(previous_latest)
            .collect::<Vec<_>>();
        for index in candidate_indices {
            let clause_state = &mut self.clauses[index];
            if clause_state_satisfied_by_new_clause(
                clause_state,
                index,
                new_index,
                new_expr,
                has_point_constructor_before,
                new_clause_has_lib_pointer,
                new_clause_is_path_con,
                new_clause_is_point_constructor,
                new_clause_is_higher_path_witness,
                new_clause_is_operator_bundle_closure,
            ) {
                newly_resolved.push(index);
            }
        }

        if let Some(previous_latest) = previous_latest {
            if !newly_resolved.contains(&previous_latest)
                && !self.unresolved_indices.contains(&previous_latest)
            {
                self.unresolved_indices.insert(previous_latest);
            }
        }
        for index in newly_resolved {
            self.unresolved_indices.remove(&index);
        }

        self.clauses.push(ConnectivityClauseState::new(
            new_expr,
            self.has_operator_bundle_seed,
        ));
        self.has_point_constructor |= new_clause_is_point_constructor;
        self.has_operator_bundle_seed |= is_operator_bundle_seed_clause(new_expr);

        if new_clause_is_point_constructor && !has_point_constructor_before {
            let mut path_resolved = Vec::new();
            for index in self.unresolved_indices.iter().copied().collect::<Vec<_>>() {
                let clause_state = &self.clauses[index];
                if clause_state.later_pathcon_seen && clause_state.local_path_anchor(true) {
                    path_resolved.push(index);
                }
            }
            for index in path_resolved {
                self.unresolved_indices.remove(&index);
            }
        }

        self
    }

    pub fn structurally_connected(&self) -> bool {
        self.unresolved_indices.is_empty()
    }

    pub fn references_active_window(&self) -> bool {
        self.references_active_window
    }

    pub fn self_contained(&self) -> bool {
        self.self_contained
    }

    pub fn max_lib_ref(&self) -> u32 {
        self.max_lib_ref
    }

    pub fn passes_without_reanchor(&self) -> bool {
        self.structurally_connected() && (self.references_active_window || self.self_contained)
    }

    pub fn needs_reanchor_fallback(&self) -> bool {
        self.structurally_connected() && !self.references_active_window && !self.self_contained
    }
}

pub fn analyze_connectivity(library: &Library, telescope: &Telescope) -> ConnectivityWitness {
    let summary = ConnectivitySummary::from_telescope(library, telescope);
    ConnectivityWitness {
        connected: summary.structurally_connected(),
        references_active_window: summary.references_active_window(),
        self_contained: summary.self_contained(),
        max_lib_ref: summary.max_lib_ref(),
        historical_reanchor: allows_temporal_modal_reanchor(library, telescope),
    }
}

pub fn passes_connectivity(library: &Library, telescope: &Telescope) -> bool {
    let witness = analyze_connectivity(library, telescope);
    witness.connected
        && (witness.references_active_window
            || witness.self_contained
            || witness.historical_reanchor)
}

fn clause_state_satisfied_by_new_clause(
    clause_state: &mut ConnectivityClauseState,
    earlier_index: usize,
    later_index: usize,
    expr: &Expr,
    has_point_constructor_before: bool,
    new_clause_has_lib_pointer: bool,
    new_clause_is_path_con: bool,
    new_clause_is_point_constructor: bool,
    new_clause_is_higher_path_witness: bool,
    new_clause_is_operator_bundle_closure: bool,
) -> bool {
    let de_bruijn = (later_index - earlier_index) as u32;
    let has_var_edge = expr.var_refs().contains(&de_bruijn)
        || later_clause_depends_on(earlier_index, later_index, expr, 0);
    let has_path_attachment = if new_clause_is_path_con {
        if clause_state.local_path_anchor(has_point_constructor_before) {
            true
        } else {
            clause_state.later_pathcon_seen = true;
            false
        }
    } else if new_clause_is_point_constructor {
        clause_state.later_pathcon_seen && clause_state.local_path_anchor(true)
    } else {
        false
    };

    has_var_edge
        || clause_state.has_lib_pointer
        || has_path_attachment
        || (clause_state.is_higher_path_con && new_clause_is_higher_path_witness)
        || (clause_state.is_higher_order_bridge && new_clause_has_lib_pointer)
        || (clause_state.is_operator_action
            && clause_state.has_prior_operator_bundle_seed
            && new_clause_is_operator_bundle_closure)
}

fn later_clause_depends_on(
    earlier_index: usize,
    later_index: usize,
    expr: &Expr,
    binder_depth: usize,
) -> bool {
    match expr {
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            later_clause_depends_on(earlier_index, later_index, function, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, argument, binder_depth)
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
            later_clause_depends_on(earlier_index, later_index, body, binder_depth + 1)
        }
        Expr::Var(index) => {
            let index = *index as usize;
            index > binder_depth
                && later_index
                    .checked_sub(index - binder_depth)
                    .is_some_and(|dep| dep == earlier_index)
        }
        Expr::Id(ty, left, right) => {
            later_clause_depends_on(earlier_index, later_index, ty, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, left, binder_depth)
                || later_clause_depends_on(earlier_index, later_index, right, binder_depth)
        }
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn is_point_constructor_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
        || matches!(expr, Expr::Var(_))
        || matches!(
            expr,
            Expr::App(left, right)
                if matches!(left.as_ref(), Expr::Lib(_))
                    && matches!(right.as_ref(), Expr::Var(_))
        )
}

fn is_higher_path_witness_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1) | Expr::Var(2)))
}

fn is_higher_order_bridge_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain)
                    if domain.var_refs().contains(&1) && codomain.var_refs().contains(&2)
            )
    )
}

fn is_operator_action_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Var(1))
                        && matches!(argument.as_ref(), Expr::Var(2))
            )
    )
}

fn is_operator_bundle_seed_clause(expr: &Expr) -> bool {
    matches!(
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
    )
}

fn is_operator_bundle_closure_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Lib(_))
                && (matches!(codomain.as_ref(), Expr::Lib(_))
                    || matches!(codomain.as_ref(), Expr::Var(1)))
    )
}

fn allows_temporal_modal_reanchor(library: &Library, telescope: &Telescope) -> bool {
    HistoricalReanchorSummary::from_telescope(library, telescope).allows_historical_reanchor()
}

fn historical_reanchor_anchor_ref(library: &Library) -> Option<u32> {
    library
        .last()
        .and_then(|entry| entry.capabilities.has_hilbert.then_some(()))
        .and_then(|_| latest_modal_shell_anchor_ref(library))
}

fn latest_modal_shell_anchor_ref(library: &Library) -> Option<u32> {
    library.iter().enumerate().rev().find_map(|(index, entry)| {
        (entry.class == TelescopeClass::Modal && entry.capabilities.has_modal_ops)
            .then_some(index as u32 + 1)
    })
}

fn matches_temporal_shell_clause(position: usize, expr: &Expr, anchor: u32) -> bool {
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
        4 => matches_temporal_flat_next_bridge(expr),
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
        7 => matches!(
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
        _ => false,
    }
}

fn matches_temporal_flat_next_bridge(expr: &Expr) -> bool {
    matches!(
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
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ConnectivitySummary, ConnectivityWitness, HistoricalReanchorSummary, analyze_connectivity,
        passes_connectivity,
    };
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;

    fn library_until(step: u32) -> Library {
        let mut library = Vec::new();
        for current in 1..=step {
            let telescope = Telescope::reference(current);
            let entry = LibraryEntry::from_telescope(&telescope, &library);
            library.push(entry);
        }
        library
    }

    #[test]
    fn connectivity_accepts_reference_witness_step() {
        let library = library_until(2);
        let witness = analyze_connectivity(&library, &Telescope::reference(3));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 2,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(3)));
    }

    #[test]
    fn incremental_summary_matches_reference_temporal_shell_connectivity() {
        let library = library_until(14);
        let telescope = Telescope::reference(15);
        let witness = analyze_connectivity(&library, &telescope);
        let summary = ConnectivitySummary::from_telescope(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);

        assert!(summary.structurally_connected());
        assert!(!summary.references_active_window());
        assert!(!summary.self_contained());
        assert!(summary.needs_reanchor_fallback());
        assert!(reanchor.allows_historical_reanchor());
        assert_eq!(summary.max_lib_ref(), witness.max_lib_ref);
        assert!(witness.historical_reanchor);
    }

    #[test]
    fn point_constructor_can_close_an_earlier_path_attachment_gap_incrementally() {
        let telescope = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::PathCon(1)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let library: Library = Vec::new();
        let summary = ConnectivitySummary::from_telescope(&library, &telescope);
        let witness = analyze_connectivity(&library, &telescope);

        assert!(summary.structurally_connected());
        assert_eq!(summary.structurally_connected(), witness.connected);
        assert_eq!(
            summary.references_active_window(),
            witness.references_active_window
        );
        assert_eq!(summary.self_contained(), witness.self_contained);
        assert_eq!(summary.max_lib_ref(), witness.max_lib_ref);
    }

    #[test]
    fn connectivity_accepts_self_contained_former_packages() {
        let library = library_until(3);
        let witness = analyze_connectivity(&library, &Telescope::reference(4));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(4)));
    }

    #[test]
    fn connectivity_accepts_local_path_attachment_packages() {
        let library = library_until(4);
        let witness = analyze_connectivity(&library, &Telescope::reference(5));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(5)));
    }

    #[test]
    fn connectivity_accepts_self_contained_truncation_packages() {
        let library = library_until(5);
        let witness = analyze_connectivity(&library, &Telescope::reference(6));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(6)));
    }

    #[test]
    fn connectivity_accepts_self_contained_higher_hit_packages() {
        let library = library_until(6);
        let witness = analyze_connectivity(&library, &Telescope::reference(7));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(7)));
    }

    #[test]
    fn connectivity_accepts_self_contained_sphere_lift_packages() {
        let library = library_until(7);
        let witness = analyze_connectivity(&library, &Telescope::reference(8));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(8)));
    }

    #[test]
    fn connectivity_accepts_active_window_axiomatic_bundles() {
        let library = library_until(8);
        let witness = analyze_connectivity(&library, &Telescope::reference(9));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 8,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(9)));
    }

    #[test]
    fn connectivity_accepts_self_contained_modal_shells() {
        let library = library_until(9);
        let witness = analyze_connectivity(&library, &Telescope::reference(10));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: true,
                max_lib_ref: 0,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(10)));
    }

    #[test]
    fn connectivity_accepts_higher_order_bridge_shells_over_the_active_window() {
        let library = library_until(11);
        let witness = analyze_connectivity(&library, &Telescope::reference(12));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 11,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(12)));
    }

    #[test]
    fn connectivity_accepts_operator_bundle_bridges_over_curvature() {
        let library = library_until(12);
        let witness = analyze_connectivity(&library, &Telescope::reference(13));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: true,
                self_contained: false,
                max_lib_ref: 12,
                historical_reanchor: false,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(13)));
    }

    #[test]
    fn connectivity_rejects_stale_library_refs() {
        let library = library_until(4);
        assert!(!passes_connectivity(&library, &Telescope::reference(3)));
    }

    #[test]
    fn connectivity_accepts_temporal_shells_reanchored_through_the_modal_history() {
        let library = library_until(14);
        let witness = analyze_connectivity(&library, &Telescope::reference(15));
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: true,
            }
        );
        assert!(passes_connectivity(&library, &Telescope::reference(15)));
    }

    #[test]
    fn connectivity_accepts_realistic_temporal_reanchor_variants() {
        let library = library_until(14);
        let mut telescope = Telescope::reference(15);
        telescope.clauses[4].expr = Expr::Pi(
            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
            Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                Box::new(Expr::Var(1)),
            )))))),
        );

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: true,
            }
        );
        assert!(reanchor.allows_historical_reanchor());
        assert!(passes_connectivity(&library, &telescope));
    }
}
