use crate::enumerate::{
    EnumerationContext, clause_kappa_can_match_structural_family,
    clause_supports_structural_family_at_position,
};
use crate::prefix_cache::PrefixSignature;
use pen_core::clause::ClauseRec;
use pen_core::library::Library;
use pen_core::telescope::Telescope;
use pen_type::admissibility::{PackagePolicy, StrictAdmissibility, StructuralFamily};
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrefixLegalitySummary {
    check: CheckSummary,
    connectivity: ConnectivitySummary,
}

impl PrefixLegalitySummary {
    fn from_telescope(library: &Library, prefix_telescope: &Telescope) -> Option<Self> {
        Some(Self {
            check: CheckSummary::from_telescope(library, prefix_telescope).ok()?,
            connectivity: ConnectivitySummary::from_telescope(library, prefix_telescope),
        })
    }

    fn extend(&self, library: &Library, clause: &ClauseRec) -> Option<Self> {
        Some(Self {
            check: self.check.extend_clause(library.len(), clause).ok()?,
            connectivity: self.connectivity.clone().extend(library, clause),
        })
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
}

#[cfg(test)]
mod tests {
    use super::{PrefixLegalityCache, TerminalConnectivityDecision};
    use crate::prefix_cache::PrefixSignature;
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;
    use pen_type::admissibility::{
        AdmissibilityMode, strict_admissibility, strict_admissibility_for_mode,
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
}
