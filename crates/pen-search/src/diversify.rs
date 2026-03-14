use crate::branch_bound::{BranchDecision, PruneClass};
use std::collections::BTreeMap;

pub fn retain_with_quota_by<T, K, F>(
    candidates: Vec<T>,
    quota_per_bucket: usize,
    bucket_fn: F,
) -> (Vec<T>, usize, Vec<BranchDecision>)
where
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut counts = BTreeMap::new();
    let mut kept = Vec::new();
    let mut prunes = Vec::new();

    for candidate in candidates {
        let bucket = bucket_fn(&candidate);
        let count = counts.entry(bucket).or_insert(0usize);
        if *count < quota_per_bucket {
            *count += 1;
            kept.push(candidate);
        } else {
            prunes.push(BranchDecision::Prune(PruneClass::HeuristicShaping));
        }
    }

    let pruned = prunes.len();
    (kept, pruned, prunes)
}

#[cfg(test)]
mod tests {
    use super::retain_with_quota_by;
    use crate::branch_bound::{BranchDecision, PruneClass};

    #[test]
    fn diversity_quota_keeps_stable_first_entries_per_bucket() {
        let candidates = vec![("a", 1), ("b", 1), ("c", 2), ("d", 1)];
        let (kept, pruned, decisions) =
            retain_with_quota_by(candidates, 2, |candidate| candidate.1);

        assert_eq!(kept, vec![("a", 1), ("b", 1), ("c", 2)]);
        assert_eq!(pruned, 1);
        assert_eq!(
            decisions,
            vec![BranchDecision::Prune(PruneClass::HeuristicShaping)]
        );
    }
}
