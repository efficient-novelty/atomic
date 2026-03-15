use crate::prefix_cache::PrefixSignature;
use pen_core::clause::ClauseRec;
use pen_core::library::Library;
use pen_core::telescope::Telescope;
use pen_type::check::CheckSummary;
use pen_type::connectivity::ConnectivitySummary;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrefixLegalityCacheStats {
    pub legality_hits: usize,
    pub connectivity_shortcuts: usize,
    pub connectivity_fallbacks: usize,
    pub connectivity_prunes: usize,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalConnectivityDecision {
    PruneDisconnected,
    KeepWithoutFallback,
    NeedsFallback,
}

#[derive(Clone, Debug, Default)]
pub struct PrefixLegalityCache {
    summaries: BTreeMap<PrefixSignature, PrefixLegalitySummary>,
    stats: PrefixLegalityCacheStats,
}

impl PrefixLegalityCache {
    pub fn stats(&self) -> PrefixLegalityCacheStats {
        self.stats
    }

    pub fn insert_root(
        &mut self,
        signature: PrefixSignature,
        library: &Library,
        prefix_telescope: &Telescope,
    ) -> bool {
        let Some(summary) = PrefixLegalitySummary::from_telescope(library, prefix_telescope) else {
            return false;
        };
        self.summaries.insert(signature, summary);
        true
    }

    pub fn insert_child(
        &mut self,
        parent_signature: &PrefixSignature,
        child_signature: PrefixSignature,
        library: &Library,
        clause: &ClauseRec,
    ) -> bool {
        let Some(parent_summary) = self.summaries.get(parent_signature) else {
            return false;
        };
        self.stats.legality_hits += 1;
        let Some(child_summary) = parent_summary.extend(library, clause) else {
            return false;
        };
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

        assert!(cache.insert_root(root_signature.clone(), &library, &root));
        assert!(cache.insert_child(
            &root_signature,
            child_signature,
            &library,
            &child_clause
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

        assert!(cache.insert_root(signature.clone(), &library, &prefix));
        assert_eq!(
            cache.terminal_connectivity(&signature, &library, &last_clause),
            Some(TerminalConnectivityDecision::NeedsFallback)
        );
        assert_eq!(cache.stats().legality_hits, 1);
        assert_eq!(cache.stats().connectivity_fallbacks, 1);
    }
}
