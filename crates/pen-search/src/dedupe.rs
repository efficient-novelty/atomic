use crate::branch_bound::{BranchDecision, PruneClass};
use std::collections::BTreeSet;

pub fn dedupe_stable_by<T, K, F>(
    candidates: Vec<T>,
    key_fn: F,
) -> (Vec<T>, usize, Vec<BranchDecision>)
where
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut seen = BTreeSet::new();
    let mut kept = Vec::new();
    let mut prunes = Vec::new();

    for candidate in candidates {
        let key = key_fn(&candidate);
        if seen.insert(key) {
            kept.push(candidate);
        } else {
            prunes.push(BranchDecision::Prune(PruneClass::QuotientDedupe));
        }
    }

    let pruned = prunes.len();
    (kept, pruned, prunes)
}

#[cfg(test)]
mod tests {
    use super::dedupe_stable_by;
    use crate::branch_bound::{BranchDecision, PruneClass};

    #[test]
    fn dedupe_keeps_first_seen_representative() {
        let candidates = vec!["a1", "b", "a2"];
        let (kept, pruned, decisions) = dedupe_stable_by(candidates, |value| {
            value.chars().next().expect("value should not be empty")
        });

        assert_eq!(kept, vec!["a1", "b"]);
        assert_eq!(pruned, 1);
        assert_eq!(
            decisions,
            vec![BranchDecision::Prune(PruneClass::QuotientDedupe)]
        );
    }
}
