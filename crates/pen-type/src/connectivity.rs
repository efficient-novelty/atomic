use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::{Telescope, TelescopeClass};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConnectivityWitness {
    pub connected: bool,
    pub references_active_window: bool,
    pub self_contained: bool,
    pub max_lib_ref: u32,
    pub historical_reanchor: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectivityTerminalDecision {
    PruneDisconnected,
    KeepWithoutFallback,
    NeedsFallback,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminalClauseConnectivityFacts {
    lib_refs: Box<[u32]>,
    max_lib_ref: u32,
    raw_var_ref_mask: u64,
    dependency_distance_mask: u64,
    is_path_con: bool,
    is_point_constructor: bool,
    is_higher_path_witness: bool,
    is_operator_bundle_closure: bool,
}

impl TerminalClauseConnectivityFacts {
    pub fn from_clause(clause: &ClauseRec) -> Self {
        let mut lib_refs = Vec::new();
        let mut max_lib_ref = 0u32;
        let mut raw_var_ref_mask = 0u64;
        let mut dependency_distance_mask = 0u64;
        collect_terminal_clause_connectivity_facts(
            &clause.expr,
            0,
            &mut lib_refs,
            &mut max_lib_ref,
            &mut raw_var_ref_mask,
            &mut dependency_distance_mask,
        );
        lib_refs.sort_unstable();
        lib_refs.dedup();
        Self {
            lib_refs: lib_refs.into_boxed_slice(),
            max_lib_ref,
            raw_var_ref_mask,
            dependency_distance_mask,
            is_path_con: matches!(clause.expr, Expr::PathCon(_)),
            is_point_constructor: is_point_constructor_expr(&clause.expr),
            is_higher_path_witness: is_higher_path_witness_clause(&clause.expr),
            is_operator_bundle_closure: is_operator_bundle_closure_clause(&clause.expr),
        }
    }

    fn has_lib_pointer(&self) -> bool {
        !self.lib_refs.is_empty()
    }

    fn references_active_window(&self, library_size: u32) -> bool {
        library_size <= 2
            || self
                .lib_refs
                .iter()
                .copied()
                .any(|index| index == library_size || index == library_size.saturating_sub(1))
    }

    fn supports_dependency_distance(&self, distance: u32) -> bool {
        if distance >= 64 {
            return false;
        }
        let bit = 1u64 << distance;
        self.raw_var_ref_mask & bit != 0 || self.dependency_distance_mask & bit != 0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct ConnectivityClauseState {
    has_lib_pointer: bool,
    is_path_con: bool,
    is_trunc_like_anchor: bool,
    is_higher_path_con: bool,
    is_higher_order_bridge: bool,
    is_operator_action: bool,
    is_structural_shell_seal: bool,
    has_prior_operator_bundle_seed: bool,
    later_pathcon_seen: bool,
}

impl ConnectivityClauseState {
    fn new(expr: &Expr, has_prior_operator_bundle_seed: bool) -> Self {
        Self {
            has_lib_pointer: summarize_lib_refs(expr, None).has_any,
            is_path_con: matches!(expr, Expr::PathCon(_)),
            is_trunc_like_anchor: matches!(expr, Expr::Trunc(_))
                || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Trunc(_))),
            is_higher_path_con: matches!(expr, Expr::PathCon(dimension) if *dimension > 1),
            is_higher_order_bridge: is_higher_order_bridge_clause(expr),
            is_operator_action: is_operator_action_clause(expr),
            is_structural_shell_seal: is_structural_shell_seal_clause(expr),
            has_prior_operator_bundle_seed,
            later_pathcon_seen: false,
        }
    }

    fn local_path_anchor(self, has_point_constructor: bool) -> bool {
        (has_point_constructor && !self.is_path_con) || self.is_trunc_like_anchor
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct LibRefSummary {
    has_any: bool,
    references_active_window: bool,
    max_ref: u32,
}

impl LibRefSummary {
    fn merge(mut self, other: Self) -> Self {
        self.has_any |= other.has_any;
        self.references_active_window |= other.references_active_window;
        self.max_ref = self.max_ref.max(other.max_ref);
        self
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HistoricalReanchorSummary {
    temporal_shell_anchor_ref: Option<u32>,
    temporal_shell_prefix_matches: bool,
    anchor_eleven_claim_pair_matches: bool,
    anchor_eleven_clause_zero_side_pocket_matches: bool,
    anchor_eleven_clause_zero_next_side_pocket_matches: bool,
    anchor_eleven_clause_one_side_pocket_matches: bool,
    anchor_eleven_clause_four_side_pocket_matches: bool,
    anchor_eleven_clause_five_side_pocket_matches: bool,
    clause_count: usize,
    matched_clause_count: usize,
    first_mismatch_position: Option<usize>,
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
            anchor_eleven_claim_pair_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_zero_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_zero_next_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_one_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_four_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            anchor_eleven_clause_five_side_pocket_matches: temporal_shell_anchor_ref.is_some(),
            clause_count: 0,
            matched_clause_count: 0,
            first_mismatch_position: None,
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
            self.anchor_eleven_claim_pair_matches = self.anchor_eleven_claim_pair_matches
                && matches_anchor_eleven_claim_pair_clause(position, &clause.expr, anchor);
            self.anchor_eleven_clause_zero_side_pocket_matches = self
                .anchor_eleven_clause_zero_side_pocket_matches
                && matches_anchor_eleven_clause_zero_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_zero_next_side_pocket_matches = self
                .anchor_eleven_clause_zero_next_side_pocket_matches
                && matches_anchor_eleven_clause_zero_next_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_one_side_pocket_matches = self
                .anchor_eleven_clause_one_side_pocket_matches
                && matches_anchor_eleven_clause_one_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_four_side_pocket_matches = self
                .anchor_eleven_clause_four_side_pocket_matches
                && matches_anchor_eleven_clause_four_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            self.anchor_eleven_clause_five_side_pocket_matches = self
                .anchor_eleven_clause_five_side_pocket_matches
                && matches_anchor_eleven_clause_five_side_pocket_clause(
                    position,
                    &clause.expr,
                    anchor,
                );
            return self;
        }

        self.temporal_shell_prefix_matches =
            matches_historical_reanchor_clause(position, &clause.expr, anchor);
        self.anchor_eleven_claim_pair_matches = self.anchor_eleven_claim_pair_matches
            && matches_anchor_eleven_claim_pair_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_zero_side_pocket_matches = self
            .anchor_eleven_clause_zero_side_pocket_matches
            && matches_anchor_eleven_clause_zero_side_pocket_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_zero_next_side_pocket_matches = self
            .anchor_eleven_clause_zero_next_side_pocket_matches
            && matches_anchor_eleven_clause_zero_next_side_pocket_clause(
                position,
                &clause.expr,
                anchor,
            );
        self.anchor_eleven_clause_one_side_pocket_matches = self
            .anchor_eleven_clause_one_side_pocket_matches
            && matches_anchor_eleven_clause_one_side_pocket_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_four_side_pocket_matches = self
            .anchor_eleven_clause_four_side_pocket_matches
            && matches_anchor_eleven_clause_four_side_pocket_clause(position, &clause.expr, anchor);
        self.anchor_eleven_clause_five_side_pocket_matches = self
            .anchor_eleven_clause_five_side_pocket_matches
            && matches_anchor_eleven_clause_five_side_pocket_clause(position, &clause.expr, anchor);
        if self.temporal_shell_prefix_matches {
            self.matched_clause_count += 1;
        } else {
            self.first_mismatch_position = Some(position);
        }
        self
    }

    pub fn allows_historical_reanchor(self) -> bool {
        self.temporal_shell_anchor_ref.is_some()
            && (self.temporal_shell_prefix_matches
                || self.anchor_eleven_claim_pair_matches
                || self.anchor_eleven_clause_zero_side_pocket_matches
                || self.anchor_eleven_clause_zero_next_side_pocket_matches
                || self.anchor_eleven_clause_one_side_pocket_matches
                || self.anchor_eleven_clause_four_side_pocket_matches
                || self.anchor_eleven_clause_five_side_pocket_matches)
            && self.clause_count == 8
    }

    pub fn matched_clause_count(self) -> usize {
        self.matched_clause_count
    }

    pub fn first_mismatch_position(self) -> Option<usize> {
        self.first_mismatch_position
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
        let new_clause_is_path_con = matches!(new_expr, Expr::PathCon(_));
        let new_clause_is_point_constructor = is_point_constructor_expr(new_expr);
        let new_clause_is_higher_path_witness = is_higher_path_witness_clause(new_expr);
        let new_clause_is_operator_bundle_closure = is_operator_bundle_closure_clause(new_expr);
        let lib_size = library.len() as u32;
        let lib_ref_summary = summarize_lib_refs(
            new_expr,
            (lib_size > 2).then_some((lib_size, lib_size.saturating_sub(1))),
        );
        let new_clause_has_lib_pointer = lib_ref_summary.has_any;

        if lib_size <= 2 {
            self.references_active_window = true;
        } else if lib_ref_summary.references_active_window {
            self.references_active_window = true;
        }

        if new_clause_has_lib_pointer {
            self.self_contained = false;
            self.max_lib_ref = self.max_lib_ref.max(lib_ref_summary.max_ref);
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

    pub fn terminal_decision(
        &self,
        library: &Library,
        clause: &ClauseRec,
        historical_reanchor: bool,
    ) -> ConnectivityTerminalDecision {
        let facts = TerminalClauseConnectivityFacts::from_clause(clause);
        self.terminal_decision_with_facts(library, &facts, historical_reanchor)
    }

    pub fn terminal_decision_with_facts(
        &self,
        library: &Library,
        facts: &TerminalClauseConnectivityFacts,
        historical_reanchor: bool,
    ) -> ConnectivityTerminalDecision {
        let new_index = self.clauses.len();
        let previous_latest = new_index.checked_sub(1);
        let has_point_constructor_before = self.has_point_constructor;
        let lib_size = library.len() as u32;
        let new_clause_has_lib_pointer = facts.has_lib_pointer();
        let references_active_window = self.references_active_window
            || lib_size <= 2
            || facts.references_active_window(lib_size);
        let self_contained = self.self_contained && !new_clause_has_lib_pointer;

        let unresolved_previous_latest =
            previous_latest.filter(|index| !self.unresolved_indices.contains(index));
        for index in self
            .unresolved_indices
            .iter()
            .copied()
            .chain(unresolved_previous_latest)
        {
            if !clause_state_satisfied_by_terminal_clause_facts(
                &self.clauses[index],
                index,
                new_index,
                has_point_constructor_before,
                facts,
            ) {
                return ConnectivityTerminalDecision::PruneDisconnected;
            }
        }

        if references_active_window || self_contained || historical_reanchor {
            ConnectivityTerminalDecision::KeepWithoutFallback
        } else {
            ConnectivityTerminalDecision::NeedsFallback
        }
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
    if new_clause_is_path_con && !clause_state.local_path_anchor(has_point_constructor_before) {
        clause_state.later_pathcon_seen = true;
    }
    clause_state_satisfied_by_terminal_clause(
        clause_state,
        earlier_index,
        later_index,
        expr,
        has_point_constructor_before,
        new_clause_has_lib_pointer,
        new_clause_is_path_con,
        new_clause_is_point_constructor,
        new_clause_is_higher_path_witness,
        new_clause_is_operator_bundle_closure,
    )
}

fn clause_state_satisfied_by_terminal_clause(
    clause_state: &ConnectivityClauseState,
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
    let has_var_edge = expr_contains_raw_var_ref(expr, de_bruijn)
        || later_clause_depends_on(earlier_index, later_index, expr, 0);
    let has_path_attachment = if new_clause_is_path_con {
        clause_state.local_path_anchor(has_point_constructor_before)
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
        || (clause_state.is_structural_shell_seal
            && later_index == earlier_index + 1
            && new_clause_is_operator_bundle_closure)
        || (clause_state.is_operator_action
            && clause_state.has_prior_operator_bundle_seed
            && new_clause_is_operator_bundle_closure)
}

fn clause_state_satisfied_by_terminal_clause_facts(
    clause_state: &ConnectivityClauseState,
    earlier_index: usize,
    later_index: usize,
    has_point_constructor_before: bool,
    facts: &TerminalClauseConnectivityFacts,
) -> bool {
    let de_bruijn = (later_index - earlier_index) as u32;
    let has_var_edge = facts.supports_dependency_distance(de_bruijn);
    let has_path_attachment = if facts.is_path_con {
        clause_state.local_path_anchor(has_point_constructor_before)
    } else if facts.is_point_constructor {
        clause_state.later_pathcon_seen && clause_state.local_path_anchor(true)
    } else {
        false
    };

    has_var_edge
        || clause_state.has_lib_pointer
        || has_path_attachment
        || (clause_state.is_higher_path_con && facts.is_higher_path_witness)
        || (clause_state.is_higher_order_bridge && facts.has_lib_pointer())
        || (clause_state.is_structural_shell_seal
            && later_index == earlier_index + 1
            && facts.is_operator_bundle_closure)
        || (clause_state.is_operator_action
            && clause_state.has_prior_operator_bundle_seed
            && facts.is_operator_bundle_closure)
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

fn expr_contains_raw_var_ref(expr: &Expr, target: u32) -> bool {
    match expr {
        Expr::Var(index) => *index == target,
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            expr_contains_raw_var_ref(function, target)
                || expr_contains_raw_var_ref(argument, target)
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
        | Expr::Eventually(body) => expr_contains_raw_var_ref(body, target),
        Expr::Id(ty, left, right) => {
            expr_contains_raw_var_ref(ty, target)
                || expr_contains_raw_var_ref(left, target)
                || expr_contains_raw_var_ref(right, target)
        }
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn collect_terminal_clause_connectivity_facts(
    expr: &Expr,
    binder_depth: u32,
    lib_refs: &mut Vec<u32>,
    max_lib_ref: &mut u32,
    raw_var_ref_mask: &mut u64,
    dependency_distance_mask: &mut u64,
) {
    match expr {
        Expr::Lib(index) => {
            lib_refs.push(*index);
            *max_lib_ref = (*max_lib_ref).max(*index);
        }
        Expr::Var(index) => {
            if *index < 64 {
                *raw_var_ref_mask |= 1u64 << *index;
            }
            if *index > binder_depth && *index < 64 + binder_depth {
                let distance = index - binder_depth;
                *dependency_distance_mask |= 1u64 << distance;
            }
        }
        Expr::Univ | Expr::PathCon(_) => {}
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            collect_terminal_clause_connectivity_facts(
                function,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
            collect_terminal_clause_connectivity_facts(
                argument,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
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
        | Expr::Eventually(body) => collect_terminal_clause_connectivity_facts(
            body,
            binder_depth + 1,
            lib_refs,
            max_lib_ref,
            raw_var_ref_mask,
            dependency_distance_mask,
        ),
        Expr::Id(ty, left, right) => {
            collect_terminal_clause_connectivity_facts(
                ty,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
            collect_terminal_clause_connectivity_facts(
                left,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
            collect_terminal_clause_connectivity_facts(
                right,
                binder_depth,
                lib_refs,
                max_lib_ref,
                raw_var_ref_mask,
                dependency_distance_mask,
            );
        }
    }
}

fn summarize_lib_refs(expr: &Expr, active_window_refs: Option<(u32, u32)>) -> LibRefSummary {
    match expr {
        Expr::Lib(index) => LibRefSummary {
            has_any: true,
            references_active_window: active_window_refs
                .is_some_and(|(latest, previous)| *index == latest || *index == previous),
            max_ref: *index,
        },
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => summarize_lib_refs(function, active_window_refs)
            .merge(summarize_lib_refs(argument, active_window_refs)),
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => summarize_lib_refs(body, active_window_refs),
        Expr::Id(ty, left, right) => summarize_lib_refs(ty, active_window_refs)
            .merge(summarize_lib_refs(left, active_window_refs))
            .merge(summarize_lib_refs(right, active_window_refs)),
        Expr::Univ | Expr::Var(_) | Expr::PathCon(_) => LibRefSummary::default(),
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
    matches!(expr, Expr::Lam(body) if is_operator_action_application(body))
}

fn is_structural_shell_seal_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::Var(1)))
        || matches!(
            expr,
            Expr::Lam(body)
                if matches!(
                    body.as_ref(),
                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        )
}

fn is_operator_bundle_seed_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Sigma(left, right)
            if is_operator_bundle_seed_arm(left) && is_operator_bundle_seed_arm(right)
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

fn is_operator_action_application(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::App(function, argument)
            if matches!(function.as_ref(), Expr::Var(1))
                && matches!(argument.as_ref(), Expr::Var(2))
    ) || matches!(
        expr,
        Expr::App(function, argument)
            if matches!(function.as_ref(), Expr::Var(1) | Expr::Var(2))
                && matches!(
                    argument.as_ref(),
                    Expr::App(inner_function, inner_argument)
                        if matches!(
                            (function.as_ref(), inner_function.as_ref(), inner_argument.as_ref()),
                            (Expr::Var(1), Expr::Var(2), Expr::Var(1))
                                | (Expr::Var(2), Expr::Var(1), Expr::Var(2))
                        )
                )
    )
}

fn is_operator_bundle_seed_arm(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Var(1))
                && (
                    matches!(codomain.as_ref(), Expr::Var(1))
                        || matches!(
                            codomain.as_ref(),
                            Expr::Pi(inner_domain, inner_codomain)
                                if matches!(inner_domain.as_ref(), Expr::Var(1))
                                    && matches!(inner_codomain.as_ref(), Expr::Var(1))
                        )
                )
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

fn matches_historical_reanchor_clause(position: usize, expr: &Expr, anchor: u32) -> bool {
    match position {
        0 => matches_temporal_next_reanchor_clause(expr),
        1 => matches_temporal_eventually_reanchor_clause(expr),
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
        5 => matches_temporal_sharp_eventually_bridge(expr),
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
        // Keep the terminal slot exact so claim-only early-prefix recovery
        // does not reland the broader noncanonical step-15 terminal lifts.
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_claim_pair_clause(position: usize, expr: &Expr, anchor: u32) -> bool {
    match position {
        0 => matches_temporal_next_reanchor_clause(expr),
        1 => matches_temporal_eventually_reanchor_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_temporal_flat_next_bridge(expr),
        5 => matches_temporal_sharp_eventually_bridge(expr),
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
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_zero_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => matches!(
            expr,
            Expr::Next(body)
                if matches!(
                    body.as_ref(),
                    Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        1 => matches_reference_temporal_eventually_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_reference_temporal_flat_next_bridge(expr),
        5 => matches_reference_temporal_sharp_eventually_bridge(expr),
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
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_zero_next_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => matches!(
            expr,
            Expr::Next(body)
                if matches!(
                    body.as_ref(),
                    Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
        ),
        1 => matches_reference_temporal_eventually_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_reference_temporal_flat_next_bridge(expr),
        5 => matches_reference_temporal_sharp_eventually_bridge(expr),
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
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_one_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => matches!(expr, Expr::Eventually(body)
        if matches!(
            body.as_ref(),
            Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
        )),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => matches_reference_temporal_flat_next_bridge(expr),
        5 => matches_reference_temporal_sharp_eventually_bridge(expr),
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
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_four_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => matches_reference_temporal_eventually_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => {
            matches_anchor_eleven_demo_sharp_codomain_clause(expr)
                || matches_anchor_eleven_demo_sharp_bridge_clause(expr)
        }
        5 => matches_reference_temporal_sharp_eventually_bridge(expr),
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
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_anchor_eleven_clause_five_side_pocket_clause(
    position: usize,
    expr: &Expr,
    anchor: u32,
) -> bool {
    match position {
        0 => matches_reference_temporal_next_clause(expr),
        1 => matches_reference_temporal_eventually_clause(expr),
        2 => matches_claim_temporal_pair_clause_two_variant(expr),
        3 => matches_anchor_eleven_exact_argument_clause(expr, anchor + 1),
        4 => {
            matches_reference_temporal_flat_next_bridge(expr)
                || matches_anchor_eleven_demo_sharp_codomain_clause(expr)
                || matches_anchor_eleven_demo_sharp_bridge_clause(expr)
        }
        5 => {
            matches_anchor_eleven_demo_sharp_domain_clause(expr)
                || matches_anchor_eleven_demo_flat_codomain_clause(expr)
        }
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
        7 => matches_reference_temporal_terminal_clause(expr),
        _ => false,
    }
}

fn matches_claim_temporal_pair_clause_two_variant(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1))
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(domain.as_ref(), Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
                && matches!(
                codomain.as_ref(),
                Expr::Eventually(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
    )
}

fn matches_anchor_eleven_exact_argument_clause(expr: &Expr, anchor: u32) -> bool {
    matches!(
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
    )
}

fn matches_reference_temporal_next_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Next(body) if matches!(body.as_ref(), Expr::Var(1)))
}

fn matches_reference_temporal_eventually_clause(expr: &Expr) -> bool {
    matches!(expr, Expr::Eventually(body) if matches!(body.as_ref(), Expr::Var(1)))
}

fn matches_temporal_next_reanchor_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Next(body)
            if matches!(body.as_ref(), Expr::Var(1))
                || matches!(
                    body.as_ref(),
                    Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
                || matches!(
                    body.as_ref(),
                    Expr::Eventually(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
    )
}

fn matches_temporal_eventually_reanchor_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Eventually(body)
            if matches!(body.as_ref(), Expr::Var(1))
                || matches!(
                    body.as_ref(),
                    Expr::Sharp(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
                || matches!(
                    body.as_ref(),
                    Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                )
    )
}

fn matches_temporal_flat_next_bridge(expr: &Expr) -> bool {
    matches_reference_temporal_flat_next_bridge(expr)
        || matches_claim_temporal_flat_next_bridge(expr)
}

fn matches_anchor_eleven_demo_sharp_codomain_clause(expr: &Expr) -> bool {
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
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Sharp(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            )
    )
}

fn matches_anchor_eleven_demo_sharp_bridge_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Flat(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Sharp(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Sharp(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            )
    )
}

fn matches_anchor_eleven_demo_sharp_domain_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Sharp(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Eventually(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Sharp(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
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
}

fn matches_anchor_eleven_demo_flat_codomain_clause(expr: &Expr) -> bool {
    matches!(
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
                        Expr::Sharp(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            )
    )
}

fn matches_reference_temporal_flat_next_bridge(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if (
                matches!(
                    domain.as_ref(),
                    Expr::Flat(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner) if matches!(inner.as_ref(), Expr::Var(1))
                        )
                )
                || matches!(
                    domain.as_ref(),
                    Expr::Flat(body)
                        if matches!(
                            body.as_ref(),
                            Expr::Next(inner)
                                if matches!(
                                    inner.as_ref(),
                                    Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                        )
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

fn matches_claim_temporal_flat_next_bridge(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Flat(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Eventually(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner) if matches!(inner.as_ref(), Expr::Var(1))
                    )
            )
    ) || matches!(
        expr,
        Expr::Pi(domain, codomain)
            if matches!(
                domain.as_ref(),
                Expr::Flat(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Next(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
                    )
            ) && matches!(
                codomain.as_ref(),
                Expr::Next(body)
                    if matches!(
                        body.as_ref(),
                        Expr::Flat(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Next(deeper)
                                    if matches!(
                                        deeper.as_ref(),
                                        Expr::Eventually(deeper_inner)
                                            if matches!(deeper_inner.as_ref(), Expr::Var(1))
                                    )
                            )
                    )
            )
    )
}

fn matches_temporal_sharp_eventually_bridge(expr: &Expr) -> bool {
    matches_reference_temporal_sharp_eventually_bridge(expr)
        || matches_claim_temporal_sharp_eventually_bridge(expr)
}

fn matches_reference_temporal_sharp_eventually_bridge(expr: &Expr) -> bool {
    matches!(
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
                        Expr::Sharp(inner)
                            if matches!(inner.as_ref(), Expr::Var(1))
                                || matches!(
                                    inner.as_ref(),
                                    Expr::Eventually(deeper)
                                        if matches!(deeper.as_ref(), Expr::Var(1))
                                )
                    )
            )
    )
}

fn matches_claim_temporal_sharp_eventually_bridge(expr: &Expr) -> bool {
    matches!(
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
                        Expr::Sharp(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Next(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
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
                        Expr::Eventually(inner)
                            if matches!(
                                inner.as_ref(),
                                Expr::Flat(deeper) if matches!(deeper.as_ref(), Expr::Var(1))
                            )
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
}

fn matches_reference_temporal_terminal_clause(expr: &Expr) -> bool {
    matches!(
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
    )
}

#[cfg(test)]
mod tests {
    use super::{
        ConnectivitySummary, ConnectivityTerminalDecision, ConnectivityWitness,
        HistoricalReanchorSummary, TerminalClauseConnectivityFacts, analyze_connectivity,
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
    fn terminal_decision_matches_incremental_extension_for_keep_prune_and_reanchor_cases() {
        let step_four_library = library_until(3);
        let step_four_prefix = Telescope::new(Telescope::reference(4).clauses[..2].to_vec());
        let step_four_summary =
            ConnectivitySummary::from_telescope(&step_four_library, &step_four_prefix);
        let step_four_last_clause = Telescope::reference(4)
            .clauses
            .last()
            .cloned()
            .expect("reference step four should have a last clause");

        let step_four_extended = step_four_summary
            .clone()
            .extend(&step_four_library, &step_four_last_clause);
        assert_eq!(
            step_four_summary.terminal_decision(&step_four_library, &step_four_last_clause, false),
            ConnectivityTerminalDecision::KeepWithoutFallback
        );
        assert!(step_four_extended.structurally_connected());
        assert!(step_four_extended.passes_without_reanchor());

        let disconnected_library: Library = Vec::new();
        let disconnected_prefix = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::PathCon(1)),
        ]);
        let disconnected_summary =
            ConnectivitySummary::from_telescope(&disconnected_library, &disconnected_prefix);
        let disconnected_clause = ClauseRec::new(ClauseRole::Formation, Expr::Univ);
        let disconnected_extended = disconnected_summary
            .clone()
            .extend(&disconnected_library, &disconnected_clause);
        assert_eq!(
            disconnected_summary.terminal_decision(
                &disconnected_library,
                &disconnected_clause,
                false
            ),
            ConnectivityTerminalDecision::PruneDisconnected
        );
        assert!(!disconnected_extended.structurally_connected());

        let step_fifteen_library = library_until(14);
        let step_fifteen_prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let step_fifteen_summary =
            ConnectivitySummary::from_telescope(&step_fifteen_library, &step_fifteen_prefix);
        let step_fifteen_reanchor =
            HistoricalReanchorSummary::from_telescope(&step_fifteen_library, &step_fifteen_prefix);
        let step_fifteen_last_clause = Telescope::reference(15)
            .clauses
            .last()
            .cloned()
            .expect("reference step fifteen should have a last clause");

        let step_fifteen_extended = step_fifteen_summary
            .clone()
            .extend(&step_fifteen_library, &step_fifteen_last_clause);
        let step_fifteen_reanchor_allowed = step_fifteen_reanchor
            .extend(&step_fifteen_last_clause)
            .allows_historical_reanchor();
        assert_eq!(step_fifteen_reanchor.matched_clause_count(), 7);
        assert_eq!(step_fifteen_reanchor.first_mismatch_position(), None);
        assert!(step_fifteen_extended.structurally_connected());
        assert!(!step_fifteen_extended.passes_without_reanchor());
        assert_eq!(
            step_fifteen_summary.terminal_decision(
                &step_fifteen_library,
                &step_fifteen_last_clause,
                false
            ),
            ConnectivityTerminalDecision::NeedsFallback
        );
        assert_eq!(
            step_fifteen_summary.terminal_decision(
                &step_fifteen_library,
                &step_fifteen_last_clause,
                step_fifteen_reanchor_allowed
            ),
            ConnectivityTerminalDecision::KeepWithoutFallback
        );
    }

    #[test]
    fn terminal_clause_connectivity_facts_match_direct_terminal_decision() {
        let step_four_library = library_until(3);
        let step_four_prefix = Telescope::new(Telescope::reference(4).clauses[..2].to_vec());
        let step_four_summary =
            ConnectivitySummary::from_telescope(&step_four_library, &step_four_prefix);
        let step_four_last_clause = Telescope::reference(4)
            .clauses
            .last()
            .cloned()
            .expect("reference step four should have a last clause");
        let step_four_facts = TerminalClauseConnectivityFacts::from_clause(&step_four_last_clause);
        assert_eq!(
            step_four_summary.terminal_decision(&step_four_library, &step_four_last_clause, false),
            step_four_summary.terminal_decision_with_facts(
                &step_four_library,
                &step_four_facts,
                false,
            )
        );

        let disconnected_library: Library = Vec::new();
        let disconnected_prefix = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::PathCon(1)),
        ]);
        let disconnected_summary =
            ConnectivitySummary::from_telescope(&disconnected_library, &disconnected_prefix);
        let disconnected_clause = ClauseRec::new(ClauseRole::Formation, Expr::Univ);
        let disconnected_facts = TerminalClauseConnectivityFacts::from_clause(&disconnected_clause);
        assert_eq!(
            disconnected_summary.terminal_decision(
                &disconnected_library,
                &disconnected_clause,
                false,
            ),
            disconnected_summary.terminal_decision_with_facts(
                &disconnected_library,
                &disconnected_facts,
                false,
            )
        );

        let step_fifteen_library = library_until(14);
        let step_fifteen_prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let step_fifteen_summary =
            ConnectivitySummary::from_telescope(&step_fifteen_library, &step_fifteen_prefix);
        let step_fifteen_reanchor =
            HistoricalReanchorSummary::from_telescope(&step_fifteen_library, &step_fifteen_prefix);
        let step_fifteen_last_clause = Telescope::reference(15)
            .clauses
            .last()
            .cloned()
            .expect("reference step fifteen should have a last clause");
        let step_fifteen_facts =
            TerminalClauseConnectivityFacts::from_clause(&step_fifteen_last_clause);
        let step_fifteen_reanchor_allowed = step_fifteen_reanchor
            .extend(&step_fifteen_last_clause)
            .allows_historical_reanchor();
        assert_eq!(step_fifteen_reanchor.matched_clause_count(), 7);
        assert_eq!(step_fifteen_reanchor.first_mismatch_position(), None);
        assert_eq!(
            step_fifteen_summary.terminal_decision(
                &step_fifteen_library,
                &step_fifteen_last_clause,
                step_fifteen_reanchor_allowed,
            ),
            step_fifteen_summary.terminal_decision_with_facts(
                &step_fifteen_library,
                &step_fifteen_facts,
                step_fifteen_reanchor_allowed,
            )
        );
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
    fn connectivity_accepts_claim_structural_shell_seal_variants() {
        let library = library_until(10);

        for seal_expr in [
            Expr::Lam(Box::new(Expr::Var(1))),
            Expr::Lam(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
        ] {
            for closure_expr in [
                Expr::Pi(Box::new(Expr::Lib(10)), Box::new(Expr::Lib(10))),
                Expr::Pi(Box::new(Expr::Lib(9)), Box::new(Expr::Lib(10))),
                Expr::Pi(Box::new(Expr::Lib(10)), Box::new(Expr::Lib(9))),
            ] {
                let mut telescope = Telescope::reference(11);
                telescope.clauses[4].expr = seal_expr.clone();
                telescope
                    .clauses
                    .push(ClauseRec::new(ClauseRole::Formation, closure_expr));
                let witness = analyze_connectivity(&library, &telescope);
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: true,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
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
    fn connectivity_accepts_claim_operator_bundle_seed_variants() {
        let library = library_until(12);

        for seed_expr in [
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
        ] {
            let mut telescope = Telescope::reference(13);
            telescope.clauses[0].expr = seed_expr;
            let witness = analyze_connectivity(&library, &telescope);
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
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_claim_operator_bundle_action_variants() {
        let library = library_until(12);

        for action_expr in [
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(1)))),
            ))),
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(2)),
                Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            ))),
        ] {
            let mut telescope = Telescope::reference(13);
            telescope.clauses[3].expr = action_expr;
            let witness = analyze_connectivity(&library, &telescope);
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
            assert!(passes_connectivity(&library, &telescope));
        }
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

    #[test]
    fn connectivity_accepts_demo_temporal_exchange_variants() {
        let library = library_until(14);
        let mut telescope = Telescope::reference(15);
        telescope.clauses[4].expr = Expr::Pi(
            Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Next(
                Box::new(Expr::Var(1)),
            )))))),
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

    fn claim_temporal_variant_exprs(position: usize, anchor: u32) -> Vec<Expr> {
        match position {
            0 => vec![
                Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
                Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(1))))),
            ],
            1 => vec![
                Expr::Eventually(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
                Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            ],
            2 => vec![
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
            ],
            3 => vec![
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor)),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
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
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(
                        Expr::Eventually(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
                ),
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Next(
                        Box::new(Expr::Var(1)),
                    )))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Next(
                        Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                    )))))),
                ),
            ],
            5 => vec![
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
            ],
            6 => vec![
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Var(2)),
                ))),
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Eventually(Box::new(Expr::Next(Box::new(Expr::Var(
                        1,
                    )))))),
                    Box::new(Expr::Var(2)),
                ))),
            ],
            _ => Vec::new(),
        }
    }

    fn reference_temporal_terminal_clause() -> ClauseRec {
        Telescope::reference(15)
            .clauses
            .last()
            .cloned()
            .expect("reference step fifteen should have a terminal clause")
    }

    fn next_lift_temporal_terminal_clause() -> ClauseRec {
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Next(
                    Box::new(Expr::Var(1)),
                )))))),
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
            ),
        )
    }

    #[test]
    fn connectivity_accepts_claim_clause_zero_one_variants_when_the_terminal_stays_reference() {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 0..=1 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should stay historically reanchorable when the terminal clause remains exact"
                );
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
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_claim_clause_zero_one_variants_outside_historical_reanchor_when_the_terminal_lifts()
     {
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 0..=1 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should not reanchor through the lifted terminal clause"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_claim_clause_two_three_variants_fenced_even_with_reference_terminal() {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 2..=3 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should stay fenced even with the exact reference terminal"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_claim_clause_four_five_variants_when_the_terminal_stays_reference() {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 4..=5 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should now stay historically reanchorable when the terminal clause remains exact"
                );
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
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_keeps_claim_clause_four_five_variants_outside_historical_reanchor_when_the_terminal_lifts()
     {
        let library = library_until(14);
        let lifted_terminal = next_lift_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for position in 4..=5 {
            for variant in claim_temporal_variant_exprs(position, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[position].expr = variant;
                telescope.clauses[7] = lifted_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "claim temporal clause-{position} variant should not reanchor through a lifted terminal clause"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 10,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_claim_clause_two_anchor_eleven_exact_argument_pocket_across_repaired_side_variants()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");
        let side_positions = [(0usize, 0usize), (1, 1), (4, 4), (5, 5)];

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for mask in 0u8..(1 << side_positions.len()) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                for (bit_index, position) in side_positions {
                    if mask & (1 << bit_index) != 0 {
                        telescope.clauses[position].expr =
                            claim_temporal_variant_exprs(position, anchor)
                                .into_iter()
                                .next()
                                .expect("repaired side position should expose a claim variant");
                    }
                }
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    reanchor.allows_historical_reanchor(),
                    "claim clause-2 plus anchor-11 exact-argument pocket should stay historically reanchorable across every repaired-side subset while clause 6 and the terminal stay exact"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: true,
                    }
                );
                assert!(passes_connectivity(&library, &telescope));
            }
        }
    }

    #[test]
    fn connectivity_accepts_clause_one_demo_flat_codomain_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-1 demo-flat-codomain opening should now count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_one_demo_flat_codomain_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[1] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Eventually(Box::new(Expr::Flat(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                !reanchor.allows_historical_reanchor(),
                "the clause-1 demo-flat-codomain opening should stay fenced until the exact anchor-11 side pocket is also present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 10,
                    historical_reanchor: false,
                }
            );
            assert!(!passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_zero_demo_sharp_domain_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-0 demo-sharp-domain opening should now count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_zero_demo_next_domain_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[0] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
            );
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-0 demo-next-domain opening should now count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_clause_zero_demo_sharp_domain_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let mut telescope = Telescope::reference(15);
        telescope.clauses[0] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Var(1))))),
        );
        telescope.clauses[7] = reference_terminal;

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert!(
            !reanchor.allows_historical_reanchor(),
            "the clause-0 demo-sharp-domain opening should stay fenced until the exact anchor-11 side pocket is also present"
        );
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: false,
            }
        );
        assert!(!passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_keeps_clause_zero_demo_next_domain_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let mut telescope = Telescope::reference(15);
        telescope.clauses[0] = ClauseRec::new(
            ClauseRole::Formation,
            Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
        );
        telescope.clauses[7] = reference_terminal;

        let witness = analyze_connectivity(&library, &telescope);
        let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
        assert!(
            !reanchor.allows_historical_reanchor(),
            "the clause-0 demo-next-domain opening should stay fenced until the exact anchor-11 side pocket is also present"
        );
        assert_eq!(
            witness,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: false,
            }
        );
        assert!(!passes_connectivity(&library, &telescope));
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_codomain_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the landed clause-4 demo-sharp-codomain opening should now count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_four_demo_sharp_bridge_only_on_the_exact_anchor_eleven_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the new clause-4 demo-sharp-bridge opening should count as historical reanchor only on the exact anchor-11 side pocket"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_only_on_the_exact_anchor_eleven_clause_four_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the next clause-5 demo-sharp-domain opening should count as historical reanchor only once the exact anchor-11 clause-4 side pocket is already present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_only_on_the_exact_anchor_eleven_clause_four_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                    Box::new(Expr::Next(Box::new(Expr::Sharp(Box::new(Expr::Flat(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-flat-codomain opening should count as historical reanchor only once the exact anchor-11 clause-4 side pocket is already present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_on_the_anchor_eleven_clause_four_bridge_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-sharp-domain opening should now also count as historical reanchor once the exact anchor-11 clause-4 bridge pocket is present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_on_the_anchor_eleven_clause_four_bridge_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[4] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                    Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Sharp(
                        Box::new(Expr::Var(1)),
                    )))))),
                ),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-flat-codomain opening should now also count as historical reanchor once the exact anchor-11 clause-4 bridge pocket is present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_sharp_domain_on_the_exact_anchor_eleven_pocket_without_the_clause_four_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Sharp(Box::new(Expr::Var(1))),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Var(1),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-sharp-domain opening should now count as historical reanchor directly on the exact anchor-11 pocket even before the clause-4 side pocket is present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_accepts_clause_five_demo_flat_codomain_on_the_exact_anchor_eleven_pocket_without_the_clause_four_side_pocket()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            let mut telescope = Telescope::reference(15);
            telescope.clauses[2].expr = clause_two_variant;
            telescope.clauses[3] = ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(anchor + 1)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            );
            telescope.clauses[5] = ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                        Expr::Var(1),
                    ))))),
                    Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                        Expr::Flat(Box::new(Expr::Var(1))),
                    ))))),
                ),
            );
            telescope.clauses[7] = reference_terminal.clone();

            let witness = analyze_connectivity(&library, &telescope);
            let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
            assert!(
                reanchor.allows_historical_reanchor(),
                "the clause-5 demo-flat-codomain opening should now count as historical reanchor directly on the exact anchor-11 pocket even before the clause-4 side pocket is present"
            );
            assert_eq!(
                witness,
                ConnectivityWitness {
                    connected: true,
                    references_active_window: false,
                    self_contained: false,
                    max_lib_ref: 11,
                    historical_reanchor: true,
                }
            );
            assert!(passes_connectivity(&library, &telescope));
        }
    }

    #[test]
    fn connectivity_keeps_claim_clause_two_anchor_eleven_exact_argument_pocket_outside_historical_reanchor_when_clause_six_moves()
     {
        let library = library_until(14);
        let reference_terminal = reference_temporal_terminal_clause();
        let anchor = super::latest_modal_shell_anchor_ref(&library)
            .expect("step fifteen history should still expose a modal shell anchor");

        for clause_two_variant in claim_temporal_variant_exprs(2, anchor) {
            for clause_six_variant in claim_temporal_variant_exprs(6, anchor) {
                let mut telescope = Telescope::reference(15);
                telescope.clauses[2].expr = clause_two_variant.clone();
                telescope.clauses[3] = ClauseRec::new(
                    ClauseRole::Introduction,
                    Expr::Lam(Box::new(Expr::App(
                        Box::new(Expr::Lib(anchor + 1)),
                        Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    ))),
                );
                telescope.clauses[6].expr = clause_six_variant;
                telescope.clauses[7] = reference_terminal.clone();

                let witness = analyze_connectivity(&library, &telescope);
                let reanchor = HistoricalReanchorSummary::from_telescope(&library, &telescope);
                assert!(
                    !reanchor.allows_historical_reanchor(),
                    "the isolated anchor-11 exact-argument pocket should stay fenced once clause 6 also deviates"
                );
                assert_eq!(
                    witness,
                    ConnectivityWitness {
                        connected: true,
                        references_active_window: false,
                        self_contained: false,
                        max_lib_ref: 11,
                        historical_reanchor: false,
                    }
                );
                assert!(!passes_connectivity(&library, &telescope));
            }
        }
    }
}
