use crate::enumerate::{
    EnumerationContext, clause_kappa_can_match_structural_family,
    clause_supports_structural_family_at_position,
};
use crate::prefix_cache::PrefixSignature;
use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::library::Library;
use pen_core::telescope::Telescope;
use pen_type::admissibility::{
    AdmissibilityDecision, PackagePolicy, StrictAdmissibility, StructuralFamily,
    StructuralFamilyMatchMask, assess_strict_admissibility_from_terminal_summary,
};
use pen_type::check::CheckSummary;
use pen_type::connectivity::ConnectivitySummary;
use std::collections::BTreeMap;

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
    pub trivial_derivability_hits: usize,
    pub trivial_derivability_prunes: usize,
    pub terminal_admissibility_hits: usize,
    pub terminal_admissibility_rejections: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrefixLegalitySummary {
    check: CheckSummary,
    connectivity: ConnectivitySummary,
    admissibility: PrefixAdmissibilitySummary,
}

impl PrefixLegalitySummary {
    fn from_telescope(library: &Library, prefix_telescope: &Telescope) -> Option<Self> {
        Some(Self {
            check: CheckSummary::from_telescope(library, prefix_telescope).ok()?,
            connectivity: ConnectivitySummary::from_telescope(library, prefix_telescope),
            admissibility: PrefixAdmissibilitySummary::from_telescope(prefix_telescope),
        })
    }

    fn extend(&self, library: &Library, clause: &ClauseRec) -> Option<Self> {
        Some(Self {
            check: self.check.extend_clause(library.len(), clause).ok()?,
            connectivity: self.connectivity.clone().extend(library, clause),
            admissibility: self.admissibility.extend(clause),
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
    ) -> Option<Self> {
        let mut summary = Self::for_admissibility(clause_kappa, admissibility)?;
        let context = EnumerationContext::from_admissibility(library, admissibility);
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalConnectivityDecision {
    PruneDisconnected,
    KeepWithoutFallback,
    NeedsFallback,
}

#[derive(Clone, Debug, Default)]
pub struct PrefixLegalityCache {
    summaries: BTreeMap<PrefixSignature, PrefixLegalitySummary>,
    family_filters: BTreeMap<PrefixSignature, PrefixFamilyFilterSummary>,
    stats: PrefixLegalityCacheStats,
}

impl PrefixLegalityCache {
    pub fn stats(&self) -> PrefixLegalityCacheStats {
        self.stats
    }

    pub fn insert_root(
        &mut self,
        signature: PrefixSignature,
        clause_kappa: u16,
        library: &Library,
        prefix_telescope: &Telescope,
        admissibility: StrictAdmissibility,
    ) -> bool {
        let Some(summary) = PrefixLegalitySummary::from_telescope(library, prefix_telescope) else {
            return false;
        };
        if let Some(filter_summary) = PrefixFamilyFilterSummary::from_prefix(
            clause_kappa,
            prefix_telescope,
            library,
            admissibility,
        ) {
            if filter_summary.is_empty() {
                self.stats.clause_family_prunes += 1;
                return false;
            }
            self.family_filters
                .insert(signature.clone(), filter_summary);
        }
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
        let Some(parent_summary) = self.summaries.get(parent_signature) else {
            return false;
        };
        self.stats.legality_hits += 1;
        let Some(child_summary) = parent_summary.extend(library, clause) else {
            return false;
        };
        if let Some(parent_filter) = self.family_filters.get(parent_signature).copied() {
            self.stats.clause_family_filter_hits += 1;
            let child_filter = parent_filter.retain_matching(
                usize::from(child_signature.clause_position.saturating_sub(1)),
                clause,
                EnumerationContext::from_admissibility(library, admissibility),
            );
            if child_filter.is_empty() {
                self.stats.clause_family_prunes += 1;
                return false;
            }
            self.family_filters
                .insert(child_signature.clone(), child_filter);
        }
        self.summaries.insert(child_signature, child_summary);
        true
    }

    pub fn terminal_connectivity(
        &mut self,
        parent_signature: &PrefixSignature,
        library: &Library,
        clause: &ClauseRec,
    ) -> Option<TerminalConnectivityDecision> {
        let Some(parent_summary) = self.summaries.get(parent_signature) else {
            return None;
        };
        self.stats.legality_hits += 1;
        let Some(terminal_summary) = parent_summary.extend(library, clause) else {
            return None;
        };
        if !terminal_summary.connectivity.structurally_connected() {
            self.stats.connectivity_prunes += 1;
            return Some(TerminalConnectivityDecision::PruneDisconnected);
        }
        if terminal_summary.connectivity.passes_without_reanchor() {
            self.stats.connectivity_shortcuts += 1;
            return Some(TerminalConnectivityDecision::KeepWithoutFallback);
        }

        self.stats.connectivity_fallbacks += 1;
        Some(TerminalConnectivityDecision::NeedsFallback)
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
        let context = EnumerationContext::from_admissibility(library, admissibility);
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
        let clause_kappa = parent_signature.clause_position.saturating_add(1);
        self.stats.trivial_derivability_hits += 1;
        if parent_summary.admissibility.extend(clause).is_trivially_derivable() {
            self.stats.trivial_derivability_prunes += 1;
            return Some(assess_strict_admissibility_from_terminal_summary(
                step_index,
                clause_kappa,
                admissibility,
                StructuralFamilyMatchMask::empty(),
                true,
            ));
        }

        let parent_filter = self.family_filters.get(parent_signature).copied()?;
        self.stats.terminal_admissibility_hits += 1;
        let terminal_filter = parent_filter.retain_matching(
            usize::from(parent_signature.clause_position),
            clause,
            EnumerationContext::from_admissibility(library, admissibility),
        );
        let decision = assess_strict_admissibility_from_terminal_summary(
            step_index,
            clause_kappa,
            admissibility,
            terminal_filter.match_mask(),
            false,
        );
        if !decision.is_admitted() {
            self.stats.terminal_admissibility_rejections += 1;
        }
        Some(decision)
    }
}

#[cfg(test)]
mod tests {
    use super::{PrefixAdmissibilitySummary, PrefixLegalityCache, TerminalConnectivityDecision};
    use crate::enumerate::{EnumerationContext, build_clause_catalog};
    use crate::prefix_cache::PrefixSignature;
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;
    use pen_type::admissibility::{
        AdmissibilityMode, assess_strict_admissibility, strict_admissibility,
        strict_admissibility_for_mode,
    };

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

        assert!(cache.insert_root(root_signature.clone(), 2, &library, &root, admissibility));
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
    fn temporal_shell_terminal_summary_flags_reanchor_fallback() {
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

        assert!(cache.insert_root(signature.clone(), 8, &library, &prefix, admissibility));
        assert_eq!(
            cache.terminal_connectivity(&signature, &library, &last_clause),
            Some(TerminalConnectivityDecision::NeedsFallback)
        );
        assert_eq!(cache.stats().legality_hits, 1);
        assert_eq!(cache.stats().connectivity_fallbacks, 1);
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

        assert!(cache.insert_root(signature.clone(), 8, &library, &prefix, admissibility));

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
    fn active_window_clause_filter_skips_impossible_terminal_continuations() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RealisticShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 5);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(signature.clone(), 5, &library, &prefix, admissibility));

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
}
