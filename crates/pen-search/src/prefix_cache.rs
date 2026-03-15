use crate::bounds::PrefixBound;
use crate::expand::evaluate_checked_candidate;
use anyhow::Result;
use pen_core::encode::telescope_bit_cost;
use pen_core::hash::blake3_hex;
use pen_core::ids::ObligationSetId;
use pen_core::library::Library;
use pen_core::telescope::Telescope;
use pen_eval::bar::DiscoveryRecord;
use pen_eval::nu::structural_nu;
use pen_type::obligations::{RetentionClass, RetentionPolicy};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PrefixSignature {
    pub clause_position: u16,
    pub obligation_set_id: ObligationSetId,
    pub prefix_hash64: u64,
    exact_key: String,
}

impl PrefixSignature {
    pub fn new(step_index: u32, prefix_telescope: &Telescope) -> Self {
        let exact_key =
            serde_json::to_string(prefix_telescope).expect("prefix telescope should serialize");
        Self {
            clause_position: u16::try_from(prefix_telescope.clauses.len())
                .expect("depth exceeded u16"),
            obligation_set_id: ObligationSetId::new(step_index),
            prefix_hash64: truncated_hash64(exact_key.as_bytes()),
            exact_key,
        }
    }

    pub fn dedupe_key(&self) -> String {
        format!("blake3:{}", blake3_hex(self.exact_key.as_bytes()))
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
        let signature = PrefixSignature::new(step_index, &prefix_telescope);
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
            shape_hash64: truncated_hash64(prefix_candidate.candidate_hash.as_bytes()),
            support_hash64: truncated_hash64(prefix_candidate.canonical_hash.as_bytes()),
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

#[cfg(test)]
mod tests {
    use super::{PrefixCache, PrefixSignature};
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::library::Library;
    use pen_core::telescope::Telescope;
    use pen_eval::bar::DiscoveryRecord;
    use pen_type::obligations::{RetentionFocus, RetentionPolicy};

    fn clause(expr: Expr) -> ClauseRec {
        ClauseRec::new(ClauseRole::Formation, expr)
    }

    #[test]
    fn prefix_signature_keeps_exactness_while_exposing_compact_fields() {
        let telescope = Telescope::reference(10);
        let signature = PrefixSignature::new(10, &telescope);

        assert_eq!(signature.clause_position, 4);
        assert_eq!(signature.obligation_set_id.get(), 10);
        assert_ne!(signature.prefix_hash64, 0);
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
