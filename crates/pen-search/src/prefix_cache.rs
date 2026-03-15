use crate::bounds::PrefixBound;
use crate::expand::evaluate_checked_candidate;
use anyhow::Result;
use pen_core::canonical::canonical_key_telescope;
use pen_core::encode::telescope_bit_cost;
use pen_core::hash::blake3_hex;
use pen_core::ids::ObligationSetId;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_eval::bar::DiscoveryRecord;
use pen_eval::nu::structural_nu;
use pen_type::obligations::{RetentionClass, RetentionPolicy};
use std::collections::BTreeMap;

const ACTIVE_WINDOW_DEPTH: usize = 2;
const FAMILY_FLAG_LIBRARY_REFS: u16 = 1 << 0;
const FAMILY_FLAG_PATH_SPACE: u16 = 1 << 1;
const FAMILY_FLAG_MODAL: u16 = 1 << 2;
const FAMILY_FLAG_TEMPORAL: u16 = 1 << 3;
const FAMILY_FLAG_DIFFERENTIAL: u16 = 1 << 4;
const FAMILY_FLAG_CURVATURE: u16 = 1 << 5;
const FAMILY_FLAG_METRIC: u16 = 1 << 6;
const FAMILY_FLAG_HILBERT: u16 = 1 << 7;
const FAMILY_FLAG_TEMPORAL_SHELL: u16 = 1 << 8;
const FAMILY_FLAG_DEPENDENT: u16 = 1 << 9;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PrefixSignature {
    pub clause_position: u16,
    pub obligation_set_id: ObligationSetId,
    pub active_window_hash64: u64,
    pub shape_hash64: u64,
    pub support_hash64: u64,
    pub family_flags: u16,
    pub prefix_hash64: u64,
    exact_key: String,
}

impl PrefixSignature {
    pub fn new(step_index: u32, library: &Library, prefix_telescope: &Telescope) -> Self {
        let exact_key =
            serde_json::to_string(prefix_telescope).expect("prefix telescope should serialize");
        let canonical_key = canonical_key_telescope(prefix_telescope);
        let candidate_hash = format!("blake3:{}", blake3_hex(exact_key.as_bytes()));
        let canonical_hash = format!("blake3:{}", blake3_hex(canonical_key.0.as_bytes()));
        Self {
            clause_position: u16::try_from(prefix_telescope.clauses.len())
                .expect("depth exceeded u16"),
            obligation_set_id: ObligationSetId::new(step_index),
            active_window_hash64: active_window_hash64(library),
            shape_hash64: truncated_hash64(candidate_hash.as_bytes()),
            support_hash64: truncated_hash64(canonical_hash.as_bytes()),
            family_flags: structural_family_flags(prefix_telescope, library),
            prefix_hash64: truncated_hash64(exact_key.as_bytes()),
            exact_key,
        }
    }

    pub fn dedupe_key(&self) -> String {
        format!("blake3:{}", blake3_hex(self.exact_key.as_bytes()))
    }

    pub fn has_library_refs(&self) -> bool {
        self.family_flags & FAMILY_FLAG_LIBRARY_REFS != 0
    }

    pub fn has_modal_family(&self) -> bool {
        self.family_flags & FAMILY_FLAG_MODAL != 0
    }

    pub fn has_temporal_family(&self) -> bool {
        self.family_flags & FAMILY_FLAG_TEMPORAL != 0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrefixCacheStats {
    pub created: usize,
    pub merged_by_signature: usize,
}

#[derive(Clone, Debug)]
pub struct PrefixCandidateGroup {
    pub prefix_telescope: Telescope,
    pub retention_class: RetentionClass,
    pub shape_hash64: u64,
    pub support_hash64: u64,
    pub depth: u16,
    pub bound: PrefixBound,
    pub telescopes: Vec<Telescope>,
}

#[derive(Clone, Debug, Default)]
pub struct PrefixCache {
    groups: BTreeMap<PrefixSignature, PrefixCandidateGroup>,
    stats: PrefixCacheStats,
}

impl PrefixCache {
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn stats(&self) -> PrefixCacheStats {
        self.stats
    }

    pub fn iter(&self) -> impl Iterator<Item = (&PrefixSignature, &PrefixCandidateGroup)> {
        self.groups.iter()
    }

    pub fn get(&self, signature: &PrefixSignature) -> Option<&PrefixCandidateGroup> {
        self.groups.get(signature)
    }

    pub fn record_group(
        &mut self,
        step_index: u32,
        prefix_telescope: Telescope,
        telescope: Telescope,
        library: &Library,
        history: &[DiscoveryRecord],
        nu_history: &[(u32, u32)],
        retention_policy: RetentionPolicy,
    ) -> Result<()> {
        let signature = PrefixSignature::new(step_index, library, &prefix_telescope);
        let exact_nu = u16::try_from(structural_nu(&telescope, library, nu_history).total)
            .expect("nu exceeded u16");
        let bit_kappa_used =
            u16::try_from(telescope_bit_cost(&telescope)).expect("bit cost exceeded u16");
        let clause_kappa_used = u16::try_from(telescope.kappa()).expect("kappa exceeded u16");

        if let Some(group) = self.groups.get_mut(&signature) {
            group
                .bound
                .absorb_completion(exact_nu, clause_kappa_used, bit_kappa_used);
            group.telescopes.push(telescope);
            self.stats.merged_by_signature += 1;
            return Ok(());
        }

        let prefix_candidate =
            evaluate_checked_candidate(library, history, prefix_telescope.clone())?;
        let group = PrefixCandidateGroup {
            depth: signature.clause_position,
            prefix_telescope,
            retention_class: retention_policy.classify(prefix_candidate.retention_signals()),
            shape_hash64: signature.shape_hash64,
            support_hash64: signature.support_hash64,
            bound: PrefixBound::singleton(exact_nu, clause_kappa_used, bit_kappa_used),
            telescopes: vec![telescope],
        };
        self.groups.insert(signature, group);
        self.stats.created += 1;
        Ok(())
    }
}

fn truncated_hash64(bytes: &[u8]) -> u64 {
    let hash = blake3_hex(bytes);
    u64::from_str_radix(&hash[..16], 16).expect("blake3 prefix should parse")
}

fn active_window_hash64(library: &Library) -> u64 {
    let start = library.len().saturating_sub(ACTIVE_WINDOW_DEPTH);
    let active_window = &library[start..];
    if active_window.is_empty() {
        return 0;
    }

    let bytes = serde_json::to_vec(active_window).expect("active window should serialize");
    truncated_hash64(&bytes)
}

fn structural_family_flags(prefix_telescope: &Telescope, library: &Library) -> u16 {
    let entry = LibraryEntry::from_telescope(prefix_telescope, library);
    let mut flags = 0_u16;

    if entry.library_refs > 0 {
        flags |= FAMILY_FLAG_LIBRARY_REFS;
    }
    if entry.has_loop || !entry.path_dims.is_empty() {
        flags |= FAMILY_FLAG_PATH_SPACE;
    }
    if entry.capabilities.has_modal_ops {
        flags |= FAMILY_FLAG_MODAL;
    }
    if entry.capabilities.has_temporal_ops {
        flags |= FAMILY_FLAG_TEMPORAL;
    }
    if entry.capabilities.has_differential_ops {
        flags |= FAMILY_FLAG_DIFFERENTIAL;
    }
    if entry.capabilities.has_curvature {
        flags |= FAMILY_FLAG_CURVATURE;
    }
    if entry.capabilities.has_metric {
        flags |= FAMILY_FLAG_METRIC;
    }
    if entry.capabilities.has_hilbert {
        flags |= FAMILY_FLAG_HILBERT;
    }
    if entry.capabilities.has_temporal_shell {
        flags |= FAMILY_FLAG_TEMPORAL_SHELL;
    }
    if entry.capabilities.has_dependent_functions {
        flags |= FAMILY_FLAG_DEPENDENT;
    }

    flags
}

#[cfg(test)]
mod tests {
    use super::{PrefixCache, PrefixSignature};
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;
    use pen_eval::bar::DiscoveryRecord;
    use pen_type::obligations::{RetentionFocus, RetentionPolicy};

    fn replay_reference_library(last_step: u32) -> Library {
        let mut library = Vec::new();
        for step in 1..=last_step {
            let telescope = Telescope::reference(step);
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
        library
    }

    fn clause(expr: Expr) -> ClauseRec {
        ClauseRec::new(ClauseRole::Formation, expr)
    }

    #[test]
    fn prefix_signature_keeps_exactness_while_exposing_compact_fields() {
        let library = replay_reference_library(14);
        let telescope = Telescope::reference(15);
        let signature = PrefixSignature::new(15, &library, &telescope);

        assert_eq!(signature.clause_position, 8);
        assert_eq!(signature.obligation_set_id.get(), 15);
        assert_ne!(signature.active_window_hash64, 0);
        assert_ne!(signature.shape_hash64, 0);
        assert_ne!(signature.support_hash64, 0);
        assert_ne!(signature.prefix_hash64, 0);
        assert!(signature.has_library_refs());
        assert!(signature.has_modal_family());
        assert!(signature.has_temporal_family());
        assert!(signature.dedupe_key().starts_with("blake3:"));
    }

    #[test]
    fn cache_merges_telescopes_that_share_an_exact_terminal_prefix() {
        let prefix = Telescope::new(vec![clause(Expr::Univ), clause(Expr::Var(1))]);
        let telescope_a = Telescope::new(vec![
            clause(Expr::Univ),
            clause(Expr::Var(1)),
            clause(Expr::PathCon(1)),
        ]);
        let telescope_b = Telescope::new(vec![
            clause(Expr::Univ),
            clause(Expr::Var(1)),
            clause(Expr::PathCon(2)),
        ]);
        let mut cache = PrefixCache::default();
        let history = vec![DiscoveryRecord::new(1, 2, 2)];
        let nu_history = vec![(1, 2)];
        let policy = RetentionPolicy {
            focus: RetentionFocus::Former,
            focus_quota: 1,
            bridge_quota: 1,
            support_quota: 1,
            macro_quota: 1,
            cold_limit: 4,
        };

        cache
            .record_group(
                2,
                prefix.clone(),
                telescope_a,
                &Library::default(),
                &history,
                &nu_history,
                policy,
            )
            .expect("first prefix should record");
        cache
            .record_group(
                2,
                prefix,
                telescope_b,
                &Library::default(),
                &history,
                &nu_history,
                policy,
            )
            .expect("second prefix should merge");

        assert_eq!(cache.len(), 1);
        assert_eq!(cache.stats().created, 1);
        assert_eq!(cache.stats().merged_by_signature, 1);
        let (_, group) = cache.iter().next().expect("group should exist");
        assert_eq!(group.telescopes.len(), 2);
        assert!(group.bound.nu_upper_bound >= group.bound.nu_lower_bound);
    }
}
