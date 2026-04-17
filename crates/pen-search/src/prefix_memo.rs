use crate::bounds::PrefixBound;
use crate::branch_bound::AcceptRank;
use crate::enumerate::{
    EnumerationContext, LateFamilySurface, clause_kappa_can_match_structural_family,
    clause_supports_structural_family_at_position,
};
use crate::prefix_cache::PrefixSignature;
use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_type::admissibility::{
    AdmissibilityDecision, AdmissibilityDiagnostics, AdmissibilityMode, PackagePolicies,
    PackagePolicy, StrictAdmissibility, StructuralFamily, StructuralFamilyMatchMask,
    assess_strict_admissibility_from_terminal_summary,
};
use pen_type::check::CheckSummary;
use pen_type::connectivity::{
    ConnectivitySummary, ConnectivityTerminalDecision, HistoricalReanchorSummary,
    TerminalClauseConnectivityFacts,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrefixLegalityCacheStats {
    pub legality_hits: usize,
    pub connectivity_shortcuts: usize,
    pub connectivity_fallbacks: usize,
    pub connectivity_prunes: usize,
    pub clause_family_filter_hits: usize,
    pub clause_family_prunes: usize,
    pub active_window_clause_filter_hits: usize,
    pub active_window_clause_filter_prunes: usize,
    pub terminal_clause_filter_hits: usize,
    pub terminal_clause_filter_prunes: usize,
    pub trivial_derivability_hits: usize,
    pub trivial_derivability_prunes: usize,
    pub terminal_admissibility_hits: usize,
    pub terminal_admissibility_rejections: usize,
    pub terminal_prefix_completion_hits: usize,
    pub terminal_prefix_bound_hits: usize,
    pub terminal_prefix_rank_hits: usize,
    pub partial_prefix_bound_hits: usize,
}

#[cfg(test)]
thread_local! {
    static CLAIM_STEP_FIFTEEN_OPEN_BAND_TERMINAL_FILTER_OVERRIDE:
        std::cell::RefCell<Option<BTreeSet<&'static str>>> =
            const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub struct ClaimStepFifteenOpenBandTerminalFilterOverrideGuard;

#[cfg(test)]
impl Drop for ClaimStepFifteenOpenBandTerminalFilterOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_OPEN_BAND_TERMINAL_FILTER_OVERRIDE.with(|override_labels| {
            *override_labels.borrow_mut() = None;
        });
    }
}

#[cfg(test)]
pub fn override_claim_step_fifteen_open_band_terminal_filter(
    labels: &[&'static str],
) -> ClaimStepFifteenOpenBandTerminalFilterOverrideGuard {
    CLAIM_STEP_FIFTEEN_OPEN_BAND_TERMINAL_FILTER_OVERRIDE.with(|override_labels| {
        *override_labels.borrow_mut() = Some(labels.iter().copied().collect());
    });
    ClaimStepFifteenOpenBandTerminalFilterOverrideGuard
}

#[cfg(test)]
thread_local! {
    static CLAIM_STEP_FIFTEEN_PREFIX_LOCAL_OPEN_BAND_TERMINAL_FILTER_OVERRIDE:
        std::cell::RefCell<Option<BTreeMap<PrefixSignature, BTreeSet<&'static str>>>> =
            const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub struct ClaimStepFifteenPrefixLocalOpenBandTerminalFilterOverrideGuard;

#[cfg(test)]
impl Drop for ClaimStepFifteenPrefixLocalOpenBandTerminalFilterOverrideGuard {
    fn drop(&mut self) {
        CLAIM_STEP_FIFTEEN_PREFIX_LOCAL_OPEN_BAND_TERMINAL_FILTER_OVERRIDE.with(
            |override_labels| {
                *override_labels.borrow_mut() = None;
            },
        );
    }
}

#[cfg(test)]
pub fn override_claim_step_fifteen_prefix_local_open_band_terminal_filter(
    prefixes: &[PrefixSignature],
    labels: &[&'static str],
) -> ClaimStepFifteenPrefixLocalOpenBandTerminalFilterOverrideGuard {
    let allowed_labels = labels.iter().copied().collect::<BTreeSet<_>>();
    CLAIM_STEP_FIFTEEN_PREFIX_LOCAL_OPEN_BAND_TERMINAL_FILTER_OVERRIDE.with(|override_labels| {
        *override_labels.borrow_mut() = Some(
            prefixes
                .iter()
                .cloned()
                .map(|signature| (signature, allowed_labels.clone()))
                .collect(),
        );
    });
    ClaimStepFifteenPrefixLocalOpenBandTerminalFilterOverrideGuard
}

#[cfg(test)]
fn claim_step_fifteen_open_band_terminal_label(clause: &ClauseRec) -> Option<&'static str> {
    let reference_terminal = Telescope::reference(15)
        .clauses
        .last()
        .cloned()
        .expect("reference step 15 should have a terminal clause");
    let next_lift_terminal = ClauseRec::new(
        pen_core::clause::ClauseRole::Formation,
        Expr::Pi(
            Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Next(
                Box::new(Expr::Var(1)),
            )))))),
            Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
        ),
    );
    let eventual_lift_terminal = ClauseRec::new(
        pen_core::clause::ClauseRole::Formation,
        Expr::Pi(
            Box::new(Expr::Next(Box::new(Expr::Next(Box::new(
                Expr::Eventually(Box::new(Expr::Var(1))),
            ))))),
            Box::new(Expr::Next(Box::new(Expr::Eventually(Box::new(Expr::Var(
                1,
            )))))),
        ),
    );

    if *clause == reference_terminal {
        Some("reference")
    } else if *clause == next_lift_terminal {
        Some("next_lift")
    } else if *clause == eventual_lift_terminal {
        Some("eventual_lift")
    } else {
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrefixLegalitySummary {
    check: CheckSummary,
    connectivity: ConnectivitySummary,
    admissibility: PrefixAdmissibilitySummary,
    reanchor: HistoricalReanchorSummary,
}

impl PrefixLegalitySummary {
    fn from_telescope(library: &Library, prefix_telescope: &Telescope) -> Option<Self> {
        Some(Self {
            check: CheckSummary::from_telescope(library, prefix_telescope).ok()?,
            connectivity: ConnectivitySummary::from_telescope(library, prefix_telescope),
            admissibility: PrefixAdmissibilitySummary::from_telescope(prefix_telescope),
            reanchor: HistoricalReanchorSummary::from_telescope(library, prefix_telescope),
        })
    }

    fn extend(&self, library: &Library, clause: &ClauseRec) -> Option<Self> {
        Some(Self {
            check: self.check.extend_clause(library.len(), clause).ok()?,
            connectivity: self.connectivity.clone().extend(library, clause),
            admissibility: self.admissibility.extend(clause),
            reanchor: self.reanchor.extend(clause),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PrefixAdmissibilitySummary {
    all_clauses_lib_or_var: bool,
    has_higher_path: bool,
    non_path_expr_count: usize,
    non_path_exprs_all_trunc_context: bool,
}

impl PrefixAdmissibilitySummary {
    fn from_telescope(prefix_telescope: &Telescope) -> Self {
        prefix_telescope
            .clauses
            .iter()
            .fold(Self::default(), |summary, clause| summary.extend(clause))
    }

    fn extend(self, clause: &ClauseRec) -> Self {
        let is_path = matches!(clause.expr, Expr::PathCon(_));
        let is_higher_path = matches!(clause.expr, Expr::PathCon(dimension) if dimension > 1);
        Self {
            all_clauses_lib_or_var: self.all_clauses_lib_or_var
                && matches!(clause.expr, Expr::Lib(_) | Expr::Var(_)),
            has_higher_path: self.has_higher_path || is_higher_path,
            non_path_expr_count: self.non_path_expr_count + usize::from(!is_path),
            non_path_exprs_all_trunc_context: self.non_path_exprs_all_trunc_context
                && (is_path || clause.expr.is_trunc_context()),
        }
    }

    fn is_trivially_derivable(self) -> bool {
        self.all_clauses_lib_or_var
            || (self.has_higher_path
                && self.non_path_expr_count > 0
                && self.non_path_exprs_all_trunc_context)
    }
}

impl Default for PrefixAdmissibilitySummary {
    fn default() -> Self {
        Self {
            all_clauses_lib_or_var: true,
            has_higher_path: false,
            non_path_expr_count: 0,
            non_path_exprs_all_trunc_context: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct PrefixFamilyFilterSummary {
    possible_families_mask: u16,
}

impl PrefixFamilyFilterSummary {
    fn from_prefix(
        clause_kappa: u16,
        prefix_telescope: &Telescope,
        library: &Library,
        admissibility: StrictAdmissibility,
        late_family_surface: LateFamilySurface,
    ) -> Option<Self> {
        let mut summary = Self::for_admissibility(clause_kappa, admissibility)?;
        let context = family_filter_context(library, admissibility, late_family_surface);
        for (position, clause) in prefix_telescope.clauses.iter().enumerate() {
            summary = summary.retain_matching(position, clause, context);
            if summary.is_empty() {
                break;
            }
        }
        Some(summary)
    }

    fn for_admissibility(clause_kappa: u16, admissibility: StrictAdmissibility) -> Option<Self> {
        admissibility.focus_family?;

        let mut possible_families_mask = 0_u16;
        if let Some(required_family) = admissibility.required_focus_family() {
            if clause_kappa_can_match_structural_family(required_family, clause_kappa) {
                possible_families_mask |= structural_family_bit(required_family);
            }
        } else {
            for family in StructuralFamily::ALL {
                if admissibility.policy_for(family) == PackagePolicy::Forbid {
                    continue;
                }
                if clause_kappa_can_match_structural_family(family, clause_kappa) {
                    possible_families_mask |= structural_family_bit(family);
                }
            }
        }

        Some(Self {
            possible_families_mask,
        })
    }

    fn retain_matching(
        self,
        position: usize,
        clause: &ClauseRec,
        context: EnumerationContext,
    ) -> Self {
        let mut possible_families_mask = 0_u16;
        for family in StructuralFamily::ALL {
            let family_bit = structural_family_bit(family);
            if self.possible_families_mask & family_bit == 0 {
                continue;
            }
            if clause_supports_structural_family_at_position(family, position, clause, context) {
                possible_families_mask |= family_bit;
            }
        }

        Self {
            possible_families_mask,
        }
    }

    fn is_empty(self) -> bool {
        self.possible_families_mask == 0
    }

    fn family_count(self) -> u8 {
        u8::try_from(self.possible_families_mask.count_ones())
            .expect("structural family count exceeded u8")
    }

    fn match_mask(self) -> StructuralFamilyMatchMask {
        let mut mask = StructuralFamilyMatchMask::empty();
        for family in StructuralFamily::ALL {
            if self.possible_families_mask & structural_family_bit(family) != 0 {
                mask.insert(family);
            }
        }
        mask
    }
}

fn structural_family_bit(family: StructuralFamily) -> u16 {
    match family {
        StructuralFamily::FormerEliminator => 1 << 0,
        StructuralFamily::InitialHit => 1 << 1,
        StructuralFamily::TruncationHit => 1 << 2,
        StructuralFamily::HigherHit => 1 << 3,
        StructuralFamily::SphereLift => 1 << 4,
        StructuralFamily::AxiomaticBundle => 1 << 5,
        StructuralFamily::ModalShell => 1 << 6,
        StructuralFamily::ConnectionShell => 1 << 7,
        StructuralFamily::CurvatureShell => 1 << 8,
        StructuralFamily::OperatorBundle => 1 << 9,
        StructuralFamily::HilbertFunctional => 1 << 10,
        StructuralFamily::TemporalShell => 1 << 11,
    }
}

fn late_family_surface_for_admissibility(admissibility: StrictAdmissibility) -> LateFamilySurface {
    match admissibility.mode {
        pen_type::admissibility::AdmissibilityMode::Guarded
        | pen_type::admissibility::AdmissibilityMode::RelaxedShadow => LateFamilySurface::None,
        pen_type::admissibility::AdmissibilityMode::RealisticShadow => {
            LateFamilySurface::RealisticShadow
        }
        pen_type::admissibility::AdmissibilityMode::DemoBreadthShadow => {
            LateFamilySurface::DemoBreadthShadow
        }
        pen_type::admissibility::AdmissibilityMode::DesktopClaimShadow => {
            LateFamilySurface::ClaimGeneric
        }
    }
}

fn family_filter_context(
    library: &Library,
    admissibility: StrictAdmissibility,
    late_family_surface: LateFamilySurface,
) -> EnumerationContext {
    let mut context = EnumerationContext::from_admissibility(library, admissibility);
    context.late_family_surface = late_family_surface;
    context
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalConnectivityDecision {
    PruneDisconnected,
    KeepWithoutFallback,
    NeedsFallback,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilteredTerminalClause<'a> {
    pub clause: &'a ClauseRec,
    pub admissibility_decision: AdmissibilityDecision,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalPrefixCompletion {
    pub telescope: Telescope,
    pub exact_nu: u16,
    pub bit_kappa_used: u16,
    pub clause_kappa_used: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalPrefixSurvivorSketchEntry {
    pub clause_index: usize,
    pub exact_nu: u16,
    pub bit_kappa_used: u16,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TerminalPrefixSurvivorSketch {
    pub survivors: Vec<TerminalPrefixSurvivorSketchEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TerminalPrefixPrimaryRank {
    pub overshoot: Rational,
    pub clause_kappa: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalPrefixClauseEvaluation {
    Disconnected,
    AdmissibilityRejected {
        decision: AdmissibilityDecision,
    },
    Admitted {
        decision: AdmissibilityDecision,
        completion: TerminalPrefixCompletion,
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TerminalPrefixCompletionSummary {
    pub evaluations: Option<Vec<TerminalPrefixClauseEvaluation>>,
    pub generated_candidate_count: usize,
    pub admissibility_diagnostics: AdmissibilityDiagnostics,
    pub bound: Option<PrefixBound>,
    pub compact_survivor_sketch: Option<TerminalPrefixSurvivorSketch>,
    pub best_accept_primary_rank: Option<TerminalPrefixPrimaryRank>,
    pub best_accept_rank: Option<AcceptRank>,
    pub admitted_candidate_count: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TerminalPrefixRankSummary {
    pub best_accept_primary_rank: Option<TerminalPrefixPrimaryRank>,
    pub best_accept_rank: Option<AcceptRank>,
    pub admitted_candidate_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartialPrefixBoundDecision {
    CanClearBar,
    CannotClearBar,
}

#[derive(Clone, Debug, Default)]
pub struct PrefixLegalityCache {
    summaries: BTreeMap<PrefixSignature, PrefixLegalitySummary>,
    family_filters: BTreeMap<PrefixSignature, PrefixFamilyFilterSummary>,
    family_surfaces: BTreeMap<PrefixSignature, LateFamilySurface>,
    terminal_prefix_completions: BTreeMap<PrefixSignature, TerminalPrefixCompletionSummary>,
    partial_prefix_bounds: BTreeMap<(PrefixSignature, u16), PartialPrefixBoundDecision>,
    stats: PrefixLegalityCacheStats,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrefixLegalityCacheEntryCounts {
    pub summaries: usize,
    pub family_filters: usize,
    pub family_surfaces: usize,
    pub terminal_prefix_completions: usize,
    pub partial_prefix_bounds: usize,
}

impl PrefixLegalityCache {
    pub fn stats(&self) -> PrefixLegalityCacheStats {
        self.stats
    }

    pub fn entry_counts(&self) -> PrefixLegalityCacheEntryCounts {
        PrefixLegalityCacheEntryCounts {
            summaries: self.summaries.len(),
            family_filters: self.family_filters.len(),
            family_surfaces: self.family_surfaces.len(),
            terminal_prefix_completions: self.terminal_prefix_completions.len(),
            partial_prefix_bounds: self.partial_prefix_bounds.len(),
        }
    }

    pub fn terminal_prefix_completion_summary(
        &mut self,
        signature: &PrefixSignature,
    ) -> Option<TerminalPrefixCompletionSummary> {
        let summary = self.terminal_prefix_completions.get(signature)?.clone();
        self.stats.terminal_prefix_completion_hits += 1;
        Some(summary)
    }

    pub fn terminal_prefix_bound_summary(
        &mut self,
        signature: &PrefixSignature,
    ) -> Option<Option<PrefixBound>> {
        let summary = self.terminal_prefix_completions.get(signature)?;
        self.stats.terminal_prefix_bound_hits += 1;
        Some(summary.bound)
    }

    pub fn take_terminal_prefix_completion_summary(
        &mut self,
        signature: &PrefixSignature,
    ) -> Option<TerminalPrefixCompletionSummary> {
        let summary = self.terminal_prefix_completions.remove(signature)?;
        self.stats.terminal_prefix_completion_hits += 1;
        Some(summary)
    }

    pub fn terminal_prefix_rank_summary(
        &mut self,
        signature: &PrefixSignature,
    ) -> Option<TerminalPrefixRankSummary> {
        let summary = self.terminal_prefix_completions.get(signature)?;
        self.stats.terminal_prefix_rank_hits += 1;
        Some(TerminalPrefixRankSummary {
            best_accept_primary_rank: summary.best_accept_primary_rank.clone(),
            best_accept_rank: summary.best_accept_rank.clone(),
            admitted_candidate_count: summary.admitted_candidate_count,
        })
    }

    pub fn store_terminal_prefix_completion_summary(
        &mut self,
        signature: PrefixSignature,
        summary: TerminalPrefixCompletionSummary,
    ) {
        self.terminal_prefix_completions.insert(signature, summary);
    }

    pub fn partial_prefix_bound_decision(
        &mut self,
        signature: &PrefixSignature,
        clause_kappa: u16,
    ) -> Option<PartialPrefixBoundDecision> {
        let decision = *self
            .partial_prefix_bounds
            .get(&(signature.clone(), clause_kappa))?;
        self.stats.partial_prefix_bound_hits += 1;
        Some(decision)
    }

    pub fn store_partial_prefix_bound_decision(
        &mut self,
        signature: PrefixSignature,
        clause_kappa: u16,
        decision: PartialPrefixBoundDecision,
    ) {
        self.partial_prefix_bounds
            .insert((signature, clause_kappa), decision);
    }

    pub fn family_option_count(&self, signature: &PrefixSignature) -> Option<u8> {
        self.family_filters
            .get(signature)
            .copied()
            .map(|summary| summary.family_count())
    }

    pub fn insert_root(
        &mut self,
        signature: PrefixSignature,
        clause_kappa: u16,
        library: &Library,
        prefix_telescope: &Telescope,
        admissibility: StrictAdmissibility,
        late_family_surface: LateFamilySurface,
    ) -> bool {
        let Some(summary) = PrefixLegalitySummary::from_telescope(library, prefix_telescope) else {
            return false;
        };
        if !should_skip_demo_family_filter(
            signature.obligation_set_id.get(),
            clause_kappa,
            late_family_surface,
        ) {
            if let Some(filter_summary) = PrefixFamilyFilterSummary::from_prefix(
                clause_kappa,
                prefix_telescope,
                library,
                admissibility,
                late_family_surface,
            ) {
                if filter_summary.is_empty() {
                    self.stats.clause_family_prunes += 1;
                    return false;
                }
                self.family_filters
                    .insert(signature.clone(), filter_summary);
            }
        }
        self.family_surfaces
            .insert(signature.clone(), late_family_surface);
        self.summaries.insert(signature, summary);
        true
    }

    pub fn insert_child(
        &mut self,
        parent_signature: &PrefixSignature,
        child_signature: PrefixSignature,
        library: &Library,
        clause: &ClauseRec,
        admissibility: StrictAdmissibility,
    ) -> bool {
        if self.summaries.contains_key(&child_signature) {
            self.stats.legality_hits += 1;
            return true;
        }

        let Some(parent_summary) = self.summaries.get(parent_signature) else {
            return false;
        };
        self.stats.legality_hits += 1;
        let Some(child_summary) = parent_summary.extend(library, clause) else {
            return false;
        };
        let late_family_surface = self
            .family_surfaces
            .get(parent_signature)
            .copied()
            .unwrap_or_else(|| late_family_surface_for_admissibility(admissibility));
        if let Some(parent_filter) = self.family_filters.get(parent_signature).copied() {
            self.stats.clause_family_filter_hits += 1;
            let child_filter = parent_filter.retain_matching(
                usize::from(child_signature.clause_position.saturating_sub(1)),
                clause,
                family_filter_context(library, admissibility, late_family_surface),
            );
            if child_filter.is_empty() {
                self.stats.clause_family_prunes += 1;
                return false;
            }
            self.family_filters
                .insert(child_signature.clone(), child_filter);
        }
        self.family_surfaces
            .insert(child_signature.clone(), late_family_surface);
        self.summaries.insert(child_signature, child_summary);
        true
    }

    pub fn terminal_connectivity(
        &mut self,
        parent_signature: &PrefixSignature,
        library: &Library,
        clause: &ClauseRec,
    ) -> Option<TerminalConnectivityDecision> {
        self.terminal_connectivity_with_facts(parent_signature, library, clause, None)
    }

    pub fn terminal_connectivity_with_facts(
        &mut self,
        parent_signature: &PrefixSignature,
        library: &Library,
        clause: &ClauseRec,
        facts: Option<&TerminalClauseConnectivityFacts>,
    ) -> Option<TerminalConnectivityDecision> {
        let Some(parent_summary) = self.summaries.get(parent_signature) else {
            return None;
        };
        self.stats.legality_hits += 1;
        parent_summary
            .check
            .extend_clause(library.len(), clause)
            .ok()?;
        let historical_reanchor = parent_summary
            .reanchor
            .extend(clause)
            .allows_historical_reanchor();
        let connectivity_decision = if let Some(facts) = facts {
            parent_summary.connectivity.terminal_decision_with_facts(
                library,
                facts,
                historical_reanchor,
            )
        } else {
            parent_summary
                .connectivity
                .terminal_decision(library, clause, historical_reanchor)
        };
        match connectivity_decision {
            ConnectivityTerminalDecision::PruneDisconnected => {
                self.stats.connectivity_prunes += 1;
                Some(TerminalConnectivityDecision::PruneDisconnected)
            }
            ConnectivityTerminalDecision::KeepWithoutFallback => {
                self.stats.connectivity_shortcuts += 1;
                Some(TerminalConnectivityDecision::KeepWithoutFallback)
            }
            ConnectivityTerminalDecision::NeedsFallback => {
                self.stats.connectivity_fallbacks += 1;
                Some(TerminalConnectivityDecision::NeedsFallback)
            }
        }
    }

    pub fn filter_active_window_clauses<'a>(
        &mut self,
        parent_signature: &PrefixSignature,
        position: usize,
        library: &Library,
        admissibility: StrictAdmissibility,
        clauses: &'a [ClauseRec],
    ) -> Option<Vec<&'a ClauseRec>> {
        let parent_filter = self.family_filters.get(parent_signature).copied()?;
        self.stats.active_window_clause_filter_hits += 1;
        let context = family_filter_context(
            library,
            admissibility,
            self.family_surface(parent_signature, admissibility),
        );
        let mut filtered = Vec::with_capacity(clauses.len());
        for clause in clauses {
            if parent_filter
                .retain_matching(position, clause, context)
                .is_empty()
            {
                self.stats.active_window_clause_filter_prunes += 1;
                continue;
            }
            filtered.push(clause);
        }
        Some(filtered)
    }

    pub fn filter_terminal_clauses<'a>(
        &mut self,
        step_index: u32,
        parent_signature: &PrefixSignature,
        library: &Library,
        admissibility: StrictAdmissibility,
        clauses: &'a [ClauseRec],
    ) -> Option<Vec<FilteredTerminalClause<'a>>> {
        if step_index <= 3
            || !self.summaries.contains_key(parent_signature)
            || !self.supports_terminal_summary_admissibility(parent_signature, admissibility)
            || self.uses_family_surface_override(parent_signature, admissibility)
        {
            return None;
        }

        self.stats.terminal_clause_filter_hits += 1;
        let mut filtered = Vec::with_capacity(clauses.len());
        for clause in clauses {
            let decision = self.terminal_admissibility(
                step_index,
                parent_signature,
                library,
                clause,
                admissibility,
            )?;
            if !decision.is_admitted() {
                self.stats.terminal_clause_filter_prunes += 1;
                continue;
            }
            filtered.push(FilteredTerminalClause {
                clause,
                admissibility_decision: decision,
            });
        }

        Some(filtered)
    }

    pub fn filter_claim_open_band_terminal_clauses<'a>(
        &mut self,
        step_index: u32,
        parent_signature: &PrefixSignature,
        admissibility: StrictAdmissibility,
        clauses: &'a [ClauseRec],
    ) -> Option<Vec<&'a ClauseRec>> {
        if step_index <= 3
            || !self.summaries.contains_key(parent_signature)
            || !self.supports_terminal_summary_admissibility(parent_signature, admissibility)
            || self.uses_family_surface_override(parent_signature, admissibility)
            || !matches!(admissibility.mode, AdmissibilityMode::DesktopClaimShadow)
            || admissibility.focus_family.is_some()
            || admissibility.package_policies != PackagePolicies::default()
        {
            return None;
        }

        self.stats.terminal_clause_filter_hits += 1;
        let clause_kappa = parent_signature.clause_position.saturating_add(1);
        let parent_admissibility = self.summaries.get(parent_signature)?.admissibility;
        let mut filtered = Vec::with_capacity(clauses.len());
        for clause in clauses {
            self.stats.trivial_derivability_hits += 1;
            if parent_admissibility.extend(clause).is_trivially_derivable() {
                self.stats.trivial_derivability_prunes += 1;
                self.stats.terminal_clause_filter_prunes += 1;
                continue;
            }

            self.stats.terminal_admissibility_hits += 1;
            if !admissibility.supports_exact_clause_kappa(clause_kappa) {
                self.stats.terminal_admissibility_rejections += 1;
                self.stats.terminal_clause_filter_prunes += 1;
                continue;
            }

            #[cfg(test)]
            {
                let allowed = claim_step_fifteen_open_band_terminal_label(clause)
                    .map(|label| {
                        let globally_allowed =
                            CLAIM_STEP_FIFTEEN_OPEN_BAND_TERMINAL_FILTER_OVERRIDE.with(
                                |override_labels| {
                                    override_labels
                                        .borrow()
                                        .as_ref()
                                        .is_none_or(|labels| labels.contains(label))
                                },
                            );
                        let locally_allowed =
                            CLAIM_STEP_FIFTEEN_PREFIX_LOCAL_OPEN_BAND_TERMINAL_FILTER_OVERRIDE
                                .with(|override_labels| {
                                    override_labels
                                        .borrow()
                                        .as_ref()
                                        .and_then(|labels| labels.get(parent_signature))
                                        .is_none_or(|labels| labels.contains(label))
                                });
                        globally_allowed && locally_allowed
                    })
                    .unwrap_or(true);
                if !allowed {
                    self.stats.terminal_clause_filter_prunes += 1;
                    continue;
                }
            }

            filtered.push(clause);
        }

        Some(filtered)
    }

    pub fn terminal_admissibility(
        &mut self,
        step_index: u32,
        parent_signature: &PrefixSignature,
        library: &Library,
        clause: &ClauseRec,
        admissibility: StrictAdmissibility,
    ) -> Option<AdmissibilityDecision> {
        let parent_summary = self.summaries.get(parent_signature)?;
        if step_index <= 3 {
            return None;
        }
        if self.uses_family_surface_override(parent_signature, admissibility) {
            return None;
        }
        let clause_kappa = parent_signature.clause_position.saturating_add(1);
        self.stats.trivial_derivability_hits += 1;
        if parent_summary
            .admissibility
            .extend(clause)
            .is_trivially_derivable()
        {
            self.stats.trivial_derivability_prunes += 1;
            return Some(assess_strict_admissibility_from_terminal_summary(
                step_index,
                clause_kappa,
                admissibility,
                StructuralFamilyMatchMask::empty(),
                true,
            ));
        }

        self.stats.terminal_admissibility_hits += 1;
        let terminal_filter =
            if let Some(parent_filter) = self.family_filters.get(parent_signature).copied() {
                parent_filter
                    .retain_matching(
                        usize::from(parent_signature.clause_position),
                        clause,
                        family_filter_context(
                            library,
                            admissibility,
                            self.family_surface(parent_signature, admissibility),
                        ),
                    )
                    .match_mask()
            } else if supports_family_agnostic_terminal_summary_admissibility(admissibility) {
                StructuralFamilyMatchMask::empty()
            } else {
                return None;
            };
        let decision = assess_strict_admissibility_from_terminal_summary(
            step_index,
            clause_kappa,
            admissibility,
            terminal_filter,
            false,
        );
        if !decision.is_admitted() {
            self.stats.terminal_admissibility_rejections += 1;
        }
        Some(decision)
    }

    pub fn uses_family_surface_override_for_signature(
        &self,
        signature: &PrefixSignature,
        admissibility: StrictAdmissibility,
    ) -> bool {
        self.uses_family_surface_override(signature, admissibility)
    }

    pub fn override_family_surface_for_signature(
        &mut self,
        signature: &PrefixSignature,
        clause_kappa: u16,
        library: &Library,
        prefix_telescope: &Telescope,
        admissibility: StrictAdmissibility,
        late_family_surface: LateFamilySurface,
    ) -> bool {
        if !self.summaries.contains_key(signature) {
            return false;
        }

        if let Some(filter_summary) = PrefixFamilyFilterSummary::from_prefix(
            clause_kappa,
            prefix_telescope,
            library,
            admissibility,
            late_family_surface,
        ) {
            if filter_summary.is_empty() {
                return false;
            }
            self.family_filters
                .insert(signature.clone(), filter_summary);
        } else {
            self.family_filters.remove(signature);
        }
        self.family_surfaces
            .insert(signature.clone(), late_family_surface);
        true
    }

    fn family_surface(
        &self,
        signature: &PrefixSignature,
        admissibility: StrictAdmissibility,
    ) -> LateFamilySurface {
        self.family_surfaces
            .get(signature)
            .copied()
            .unwrap_or_else(|| late_family_surface_for_admissibility(admissibility))
    }

    fn uses_family_surface_override(
        &self,
        signature: &PrefixSignature,
        admissibility: StrictAdmissibility,
    ) -> bool {
        self.family_surface(signature, admissibility)
            != late_family_surface_for_admissibility(admissibility)
    }

    fn supports_terminal_summary_admissibility(
        &self,
        signature: &PrefixSignature,
        admissibility: StrictAdmissibility,
    ) -> bool {
        self.family_filters.contains_key(signature)
            || supports_family_agnostic_terminal_summary_admissibility(admissibility)
    }
}

fn should_skip_demo_family_filter(
    step_index: u32,
    clause_kappa: u16,
    late_family_surface: LateFamilySurface,
) -> bool {
    late_family_surface == LateFamilySurface::DemoBreadthShadow
        && matches!((step_index, clause_kappa), (10, 4) | (11, 5))
}

fn supports_family_agnostic_terminal_summary_admissibility(
    admissibility: StrictAdmissibility,
) -> bool {
    admissibility.focus_family.is_none()
        && StructuralFamily::ALL
            .into_iter()
            .all(|family| !matches!(admissibility.policy_for(family), PackagePolicy::Forbid))
}

#[cfg(test)]
mod tests {
    use super::{
        PartialPrefixBoundDecision, PrefixAdmissibilitySummary, PrefixLegalityCache,
        TerminalConnectivityDecision, TerminalPrefixClauseEvaluation, TerminalPrefixCompletion,
        TerminalPrefixCompletionSummary, TerminalPrefixPrimaryRank,
        late_family_surface_for_admissibility,
    };
    use crate::accept::acceptance_rank_for_telescope;
    use crate::bounds::PrefixBound;
    use crate::enumerate::{EnumerationContext, LateFamilySurface, build_clause_catalog};
    use crate::prefix_cache::PrefixSignature;
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::rational::Rational;
    use pen_core::telescope::Telescope;
    use pen_type::admissibility::{
        AdmissibilityDecisionClass, AdmissibilityDiagnostics, AdmissibilityMode, PackagePolicies,
        PackagePolicy, StrictAdmissibility, StructuralFamily, assess_strict_admissibility,
        strict_admissibility, strict_admissibility_for_mode,
    };
    use pen_type::connectivity::analyze_connectivity;

    fn library_until(step: u32) -> Library {
        let mut library = Vec::new();
        for current in 1..=step {
            let telescope = Telescope::reference(current);
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
        library
    }

    #[test]
    fn cache_reuses_parent_summary_for_child_prefixes() {
        let library: Library = Vec::new();
        let root = Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Univ)]);
        let child_clause = ClauseRec::new(
            ClauseRole::Introduction,
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
        );
        let root_signature = PrefixSignature::new(1, &library, &root);
        let mut child = root.clone();
        child.clauses.push(child_clause.clone());
        let child_signature = PrefixSignature::new(1, &library, &child);
        let mut cache = PrefixLegalityCache::default();
        let admissibility = strict_admissibility(1, 2, &library);

        assert!(cache.insert_root(
            root_signature.clone(),
            2,
            &library,
            &root,
            admissibility,
            LateFamilySurface::None
        ));
        assert!(cache.insert_child(
            &root_signature,
            child_signature,
            &library,
            &child_clause,
            admissibility,
        ));
        assert_eq!(cache.stats().legality_hits, 1);
    }

    #[test]
    fn claim_shadow_defaults_to_claim_generic_family_surface() {
        let library = library_until(9);
        let admissibility =
            strict_admissibility_for_mode(10, 2, &library, AdmissibilityMode::DesktopClaimShadow);

        assert_eq!(
            late_family_surface_for_admissibility(admissibility),
            LateFamilySurface::ClaimGeneric
        );
    }

    #[test]
    fn admissibility_summary_tracks_trivial_derivability_incrementally() {
        let lib_only = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Lib(2)),
        ]);
        assert!(PrefixAdmissibilitySummary::from_telescope(&lib_only).is_trivially_derivable());

        let trunc_hybrid = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Trunc(Box::new(Expr::Var(1)))),
            ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(2)),
        ]);
        assert!(PrefixAdmissibilitySummary::from_telescope(&trunc_hybrid).is_trivially_derivable());
    }

    #[test]
    fn temporal_shell_terminal_summary_shortcuts_historical_reanchor() {
        let library = library_until(14);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let last_clause = Telescope::reference(15)
            .clauses
            .last()
            .cloned()
            .expect("reference temporal shell should have a last clause");
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::RealisticShadow);

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));
        assert_eq!(
            cache.terminal_connectivity(&signature, &library, &last_clause),
            Some(TerminalConnectivityDecision::KeepWithoutFallback)
        );
        assert_eq!(cache.stats().legality_hits, 1);
        assert_eq!(cache.stats().connectivity_shortcuts, 1);
        assert_eq!(cache.stats().connectivity_fallbacks, 0);
    }

    #[test]
    fn claim_terminal_connectivity_keeps_reference_step_four_winner_clause() {
        let library = library_until(3);
        let admissibility =
            strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let prefix = Telescope::new(Telescope::reference(4).clauses[..2].to_vec());
        let last_clause = Telescope::reference(4)
            .clauses
            .last()
            .cloned()
            .expect("reference step four should have a last clause");
        let signature = PrefixSignature::new(4, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            3,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));
        assert_eq!(
            cache.terminal_connectivity(&signature, &library, &last_clause),
            Some(TerminalConnectivityDecision::KeepWithoutFallback)
        );
        assert_eq!(cache.stats().connectivity_shortcuts, 1);
        assert_eq!(cache.stats().connectivity_prunes, 0);
        assert_eq!(cache.stats().connectivity_fallbacks, 0);
    }

    #[test]
    fn claim_terminal_connectivity_matches_direct_step_four_assessment() {
        let library = library_until(3);
        let admissibility =
            strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 3);
        let prefix = Telescope::new(Telescope::reference(4).clauses[..2].to_vec());
        let prefix_check = pen_type::check::CheckSummary::from_telescope(&library, &prefix)
            .expect("reference step four prefix should pass conservative checks");
        let signature = PrefixSignature::new(4, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            3,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        for clause in clause_catalog.clauses_at(2) {
            let mut telescope = prefix.clone();
            telescope.clauses.push(clause.clone());
            let expected = if prefix_check.extend_clause(library.len(), clause).is_err() {
                None
            } else {
                let witness = analyze_connectivity(&library, &telescope);
                Some(if !witness.connected {
                    TerminalConnectivityDecision::PruneDisconnected
                } else if witness.references_active_window
                    || witness.self_contained
                    || witness.historical_reanchor
                {
                    TerminalConnectivityDecision::KeepWithoutFallback
                } else {
                    TerminalConnectivityDecision::NeedsFallback
                })
            };

            assert_eq!(
                cache.terminal_connectivity(&signature, &library, clause),
                expected
            );
        }

        assert!(cache.stats().connectivity_prunes > 0);
        assert_eq!(cache.stats().connectivity_fallbacks, 0);
    }

    #[test]
    fn terminal_admissibility_matches_direct_assessment_when_family_summary_is_available() {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::RealisticShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));

        for clause in clause_catalog.clauses_at(7) {
            let mut telescope = prefix.clone();
            telescope.clauses.push(clause.clone());

            let cached = cache
                .terminal_admissibility(15, &signature, &library, clause, admissibility)
                .expect("terminal admissibility should reuse family summary");
            let direct = assess_strict_admissibility(15, &library, &telescope, admissibility);

            assert_eq!(cached, direct);
        }

        assert!(cache.stats().terminal_admissibility_hits > 0);
    }

    #[test]
    fn claim_terminal_admissibility_matches_direct_assessment_without_family_summary() {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));
        assert_eq!(cache.entry_counts().family_filters, 0);

        for clause in clause_catalog.clauses_at(7) {
            let mut telescope = prefix.clone();
            telescope.clauses.push(clause.clone());

            let cached = cache
                .terminal_admissibility(15, &signature, &library, clause, admissibility)
                .expect("claim terminal admissibility should reuse legality summary");
            let direct = assess_strict_admissibility(15, &library, &telescope, admissibility);

            assert_eq!(cached, direct);
        }

        assert!(cache.stats().terminal_admissibility_hits > 0);
    }

    #[test]
    fn claim_step_fifteen_family_summary_stays_disabled_until_a_focus_family_is_reintroduced() {
        let library = library_until(14);
        let default_admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let context = EnumerationContext::from_admissibility(&library, default_admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let focused_admissibility = StrictAdmissibility {
            focus_family: Some(StructuralFamily::TemporalShell),
            package_policies: PackagePolicies {
                temporal_shell: PackagePolicy::Prefer,
                ..PackagePolicies::default()
            },
            ..default_admissibility
        };
        let mut default_cache = PrefixLegalityCache::default();
        let mut focused_cache = PrefixLegalityCache::default();

        assert!(default_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            default_admissibility,
            LateFamilySurface::ClaimGeneric
        ));
        assert_eq!(
            default_cache.entry_counts().family_filters,
            0,
            "the live step-15 claim profile should not materialize a family summary while it stays no-focus"
        );
        assert_eq!(
            default_cache.family_option_count(&signature),
            None,
            "search-side family-option counts should stay absent on the default claim-generic route"
        );
        assert_eq!(
            default_cache.filter_active_window_clauses(
                &signature,
                7,
                &library,
                default_admissibility,
                clause_catalog.clauses_at(7),
            ),
            None,
            "without a focus family, the first search-side consumer should fall through rather than activate family filtering"
        );

        assert!(focused_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            focused_admissibility,
            LateFamilySurface::ClaimGeneric
        ));
        assert_eq!(
            focused_cache.entry_counts().family_filters,
            1,
            "reintroducing a focused structural route should re-enable the cached family summary on the same prefix"
        );
        let focused_filtered = focused_cache
            .filter_active_window_clauses(
                &signature,
                7,
                &library,
                focused_admissibility,
                clause_catalog.clauses_at(7),
            )
            .expect("a focused structural route should expose an active-window family filter");
        assert_eq!(
            focused_cache.family_option_count(&signature),
            Some(1),
            "the same step-15 prefix should collapse to one remaining family option once temporal focus is reinstated"
        );
        assert_eq!(
            focused_cache.stats().active_window_clause_filter_hits,
            1,
            "reintroducing focus should make the first search-side active-window consumer observable again on the same prefix"
        );
        assert_eq!(
            focused_filtered,
            clause_catalog.clauses_at(7).iter().collect::<Vec<_>>(),
            "the focused replay should still run through the family-summary path even when this exact step-15 terminal catalog remains fully compatible with the temporal route"
        );
    }

    #[test]
    fn active_window_clause_filter_skips_impossible_terminal_continuations() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RealisticShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 5);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            5,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));

        let filtered = cache
            .filter_active_window_clauses(
                &signature,
                4,
                &library,
                admissibility,
                clause_catalog.clauses_at(4),
            )
            .expect("family summary should enable active-window filtering");

        assert!(!filtered.is_empty());
        assert!(filtered.len() < clause_catalog.clauses_at(4).len());
        assert_eq!(cache.stats().active_window_clause_filter_hits, 1);
        assert!(cache.stats().active_window_clause_filter_prunes > 0);
    }

    #[test]
    fn terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RealisticShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 5);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            5,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));

        let terminal_filtered = cache
            .filter_terminal_clauses(
                11,
                &signature,
                &library,
                admissibility,
                clause_catalog.clauses_at(4),
            )
            .expect("terminal summary should enable terminal filtering");

        assert!(!terminal_filtered.is_empty());
        assert!(terminal_filtered.len() < clause_catalog.clauses_at(4).len());
        assert_eq!(cache.stats().terminal_clause_filter_hits, 1);
        assert!(cache.stats().terminal_clause_filter_prunes > 0);
        assert!(cache.stats().terminal_admissibility_hits > 0);
    }

    #[test]
    fn claim_open_band_terminal_clause_filter_matches_direct_admissibility_without_family_summary()
    {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));
        assert_eq!(cache.entry_counts().family_filters, 0);

        let terminal_filtered = cache
            .filter_claim_open_band_terminal_clauses(
                15,
                &signature,
                admissibility,
                clause_catalog.clauses_at(7),
            )
            .expect("claim legality summary should enable open-band terminal filtering");

        let mut direct_admitted = Vec::new();
        for clause in clause_catalog.clauses_at(7) {
            let mut telescope = prefix.clone();
            telescope.clauses.push(clause.clone());
            let decision = assess_strict_admissibility(15, &library, &telescope, admissibility);
            if decision.is_admitted() {
                direct_admitted.push(clause);
            }
        }

        assert!(!terminal_filtered.is_empty());
        assert_eq!(terminal_filtered.len(), direct_admitted.len());
        assert_eq!(terminal_filtered, direct_admitted);
        assert_eq!(cache.stats().terminal_clause_filter_hits, 1);
        assert_eq!(
            cache.stats().terminal_clause_filter_prunes,
            clause_catalog.clauses_at(7).len() - direct_admitted.len()
        );
        assert!(cache.stats().terminal_admissibility_hits > 0);
    }

    #[test]
    fn claim_open_band_terminal_clause_filter_requires_default_claim_generic_route() {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let focused_admissibility = StrictAdmissibility {
            focus_family: Some(StructuralFamily::TemporalShell),
            ..admissibility
        };
        let package_constrained_admissibility = StrictAdmissibility {
            require_temporal_shell_package: true,
            package_policies: PackagePolicies {
                temporal_shell: PackagePolicy::Require,
                ..PackagePolicies::default()
            },
            ..admissibility
        };
        let mut default_cache = PrefixLegalityCache::default();
        let mut overridden_surface_cache = PrefixLegalityCache::default();

        assert!(default_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));
        assert!(overridden_surface_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::DemoBreadthShadow
        ));

        let default_terminal_filtered = default_cache
            .filter_claim_open_band_terminal_clauses(
                15,
                &signature,
                admissibility,
                clause_catalog.clauses_at(7),
            )
            .expect(
                "the claim open-band route should stay available on the live default claim-generic admissibility profile",
            );
        assert_eq!(
            default_terminal_filtered,
            clause_catalog.clauses_at(7).iter().collect::<Vec<_>>(),
            "the live default claim-generic route should keep the full three-clause step-15 claim-generic terminal trio available before any alternate caller profile closes the selector"
        );
        assert_eq!(
            default_cache.filter_claim_open_band_terminal_clauses(
                15,
                &signature,
                focused_admissibility,
                clause_catalog.clauses_at(7),
            ),
            None,
            "adding a family focus should exit the open-band route so the caller must fall back to the general terminal path"
        );
        assert_eq!(
            default_cache.filter_claim_open_band_terminal_clauses(
                15,
                &signature,
                package_constrained_admissibility,
                clause_catalog.clauses_at(7),
            ),
            None,
            "adding any nondefault package policy should also exit the open-band route before terminal filtering continues"
        );
        assert_eq!(
            overridden_surface_cache.filter_claim_open_band_terminal_clauses(
                15,
                &signature,
                admissibility,
                clause_catalog.clauses_at(7),
            ),
            None,
            "changing the late family surface away from ClaimGeneric should keep the route selector closed even on the same step-15 prefix"
        );
    }

    #[test]
    fn family_option_count_exposes_remaining_exact_family_surface() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RealisticShadow);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            5,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));
        assert_eq!(cache.family_option_count(&signature), Some(1));
    }

    #[test]
    fn demo_surface_override_keeps_demo_only_terminal_clause_in_active_window_filter() {
        let library = library_until(14);
        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::RealisticShadow);
        let realistic_context = EnumerationContext::from_admissibility(&library, admissibility);
        let realistic_catalog = build_clause_catalog(realistic_context, 8);
        let mut demo_context = realistic_context;
        demo_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
        let demo_catalog = build_clause_catalog(demo_context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let demo_only_clause = demo_catalog
            .clauses_at(7)
            .iter()
            .find(|clause| !realistic_catalog.clauses_at(7).contains(*clause))
            .expect("demo surface should expose an extra terminal clause")
            .clone();
        let demo_only_slice = [demo_only_clause.clone()];

        let mut realistic_cache = PrefixLegalityCache::default();
        assert!(realistic_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));
        let realistic_filtered = realistic_cache
            .filter_active_window_clauses(&signature, 7, &library, admissibility, &demo_only_slice)
            .expect("realistic family summary should enable filtering");
        assert!(realistic_filtered.is_empty());

        let mut demo_cache = PrefixLegalityCache::default();
        assert!(demo_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::DemoBreadthShadow
        ));
        let demo_filtered = demo_cache
            .filter_active_window_clauses(&signature, 7, &library, admissibility, &demo_only_slice)
            .expect("demo family summary should enable filtering");
        assert_eq!(demo_filtered, vec![&demo_only_clause]);
        assert_eq!(
            demo_cache.filter_terminal_clauses(
                15,
                &signature,
                &library,
                admissibility,
                &demo_only_slice,
            ),
            None
        );
    }

    #[test]
    fn terminal_prefix_completion_summary_reuses_cached_exact_results() {
        let library = library_until(10);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        let decision = assess_strict_admissibility(
            11,
            &library,
            &Telescope::reference(11),
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RealisticShadow),
        );
        let mut diagnostics = AdmissibilityDiagnostics::default();
        diagnostics.record(&decision);
        cache.store_terminal_prefix_completion_summary(
            signature.clone(),
            TerminalPrefixCompletionSummary {
                evaluations: Some(vec![TerminalPrefixClauseEvaluation::Admitted {
                    decision: decision.clone(),
                    completion: TerminalPrefixCompletion {
                        telescope: Telescope::reference(11),
                        exact_nu: 26,
                        bit_kappa_used: 79,
                        clause_kappa_used: 5,
                    },
                }]),
                generated_candidate_count: 1,
                admissibility_diagnostics: diagnostics,
                bound: Some(PrefixBound::singleton(26, 5, 79)),
                compact_survivor_sketch: None,
                best_accept_primary_rank: Some(TerminalPrefixPrimaryRank {
                    overshoot: Rational::new(42, 145),
                    clause_kappa: 5,
                }),
                best_accept_rank: acceptance_rank_for_telescope(
                    Rational::new(712, 145),
                    &Telescope::reference(11),
                    26,
                    79,
                    5,
                ),
                admitted_candidate_count: 1,
            },
        );

        let summary = cache
            .terminal_prefix_completion_summary(&signature)
            .expect("summary should be cached");
        let rank_summary = cache
            .terminal_prefix_rank_summary(&signature)
            .expect("rank summary should be cached");

        assert_eq!(summary.bound, Some(PrefixBound::singleton(26, 5, 79)));
        assert_eq!(cache.stats().terminal_prefix_completion_hits, 1);
        assert_eq!(cache.stats().terminal_prefix_rank_hits, 1);
        assert_eq!(rank_summary.admitted_candidate_count, 1);
        assert!(rank_summary.best_accept_rank.is_some());
        assert_eq!(summary.generated_candidate_count, 1);
        match summary.evaluations.as_deref() {
            Some(
                [
                    TerminalPrefixClauseEvaluation::Admitted {
                        decision,
                        completion,
                    },
                ],
            ) => {
                assert_eq!(
                    decision.class,
                    AdmissibilityDecisionClass::AdmittedFocusAligned
                );
                assert_eq!(decision.reason, "focus_connection_shell");
                assert_eq!(completion.exact_nu, 26);
                assert_eq!(completion.clause_kappa_used, 5);
                assert_eq!(completion.bit_kappa_used, 79);
                assert_eq!(completion.telescope, Telescope::reference(11));
            }
            other => panic!("unexpected cached summary shape: {other:?}"),
        }
        assert!(
            summary
                .bound
                .expect("bound should exist")
                .can_clear_bar(Rational::new(5, 1))
        );
    }

    #[test]
    fn terminal_prefix_bound_summary_reads_cached_bound_without_cloning_payload() {
        let library = library_until(10);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        cache.store_terminal_prefix_completion_summary(
            signature.clone(),
            TerminalPrefixCompletionSummary {
                evaluations: None,
                generated_candidate_count: 1,
                admissibility_diagnostics: AdmissibilityDiagnostics::default(),
                bound: Some(PrefixBound::singleton(26, 5, 79)),
                compact_survivor_sketch: None,
                best_accept_primary_rank: None,
                best_accept_rank: None,
                admitted_candidate_count: 0,
            },
        );

        let bound = cache
            .terminal_prefix_bound_summary(&signature)
            .expect("bound summary should be cached");

        assert_eq!(bound, Some(PrefixBound::singleton(26, 5, 79)));
        assert_eq!(cache.stats().terminal_prefix_bound_hits, 1);
        assert_eq!(cache.stats().terminal_prefix_completion_hits, 0);
    }

    #[test]
    fn take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse() {
        let library = library_until(10);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        cache.store_terminal_prefix_completion_summary(
            signature.clone(),
            TerminalPrefixCompletionSummary {
                evaluations: None,
                generated_candidate_count: 1,
                admissibility_diagnostics: AdmissibilityDiagnostics::default(),
                bound: Some(PrefixBound::singleton(26, 5, 79)),
                compact_survivor_sketch: None,
                best_accept_primary_rank: None,
                best_accept_rank: None,
                admitted_candidate_count: 0,
            },
        );

        let summary = cache
            .take_terminal_prefix_completion_summary(&signature)
            .expect("summary should be removable");

        assert_eq!(summary.bound, Some(PrefixBound::singleton(26, 5, 79)));
        assert_eq!(cache.stats().terminal_prefix_completion_hits, 1);
        assert!(
            cache
                .terminal_prefix_completion_summary(&signature)
                .is_none(),
            "consumed summaries should not stay resident"
        );
        assert!(
            cache.terminal_prefix_rank_summary(&signature).is_none(),
            "rank summaries should disappear with the consumed payload"
        );
    }

    #[test]
    fn partial_prefix_bound_decision_reuses_cached_exact_result() {
        let library = library_until(10);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..3].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        cache.store_partial_prefix_bound_decision(
            signature.clone(),
            5,
            PartialPrefixBoundDecision::CanClearBar,
        );

        assert_eq!(
            cache.partial_prefix_bound_decision(&signature, 5),
            Some(PartialPrefixBoundDecision::CanClearBar)
        );
        assert_eq!(cache.partial_prefix_bound_decision(&signature, 6), None);
        assert_eq!(cache.stats().partial_prefix_bound_hits, 1);
    }
}
