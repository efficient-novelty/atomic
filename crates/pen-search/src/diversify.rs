use crate::branch_bound::{BranchDecision, PruneClass};
use crate::state::FrontierStateRecV1;
use pen_store::memory::{
    GovernorConfig, GovernorState, MemoryUsage, PressureAction, evaluate_governor,
};
use pen_store::spill::SpillConfig;
use pen_type::obligations::{RetentionClass, RetentionPolicy};
use serde::{Deserialize, Serialize};
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

pub fn retain_with_obligation_policy<T, F>(
    candidates: Vec<T>,
    policy: RetentionPolicy,
    class_fn: F,
) -> (Vec<T>, usize, Vec<BranchDecision>)
where
    F: Fn(&T) -> RetentionClass,
{
    let mut counts = BTreeMap::new();
    let mut kept = Vec::new();
    let mut prunes = Vec::new();

    for candidate in candidates {
        let retention_class = class_fn(&candidate);
        let count = counts.entry(retention_class).or_insert(0usize);
        if *count < policy.quota_for(retention_class) {
            *count += 1;
            kept.push(candidate);
        } else {
            prunes.push(BranchDecision::Prune(PruneClass::HeuristicShaping));
        }
    }

    let pruned = prunes.len();
    (kept, pruned, prunes)
}

const DEDUPE_BYTES_PER_RECORD: u64 = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrontierRuntimeLimits {
    pub worker_count: u16,
    pub governor: GovernorConfig,
    pub spill: SpillConfig,
    pub record_bytes: u64,
    pub dedupe_bytes_per_record: u64,
    pub worker_scratch_bytes: u64,
    pub checkpoint_bytes: u64,
    pub spill_buffer_bytes: u64,
}

impl FrontierRuntimeLimits {
    pub fn unlimited() -> Self {
        let max = u64::MAX / 4;
        Self {
            worker_count: 1,
            governor: GovernorConfig {
                green_limit_bytes: max,
                yellow_limit_bytes: max,
                orange_limit_bytes: max,
                red_limit_bytes: max,
                hard_limit_bytes: max,
            },
            spill: SpillConfig {
                max_records_per_shard: usize::MAX,
                max_dedupe_keys_per_segment: usize::MAX,
                resident_cold_records: usize::MAX,
            },
            record_bytes: FrontierStateRecV1::BYTE_LEN as u64,
            dedupe_bytes_per_record: DEDUPE_BYTES_PER_RECORD,
            worker_scratch_bytes: 0,
            checkpoint_bytes: 0,
            spill_buffer_bytes: 0,
        }
    }

    fn usage_for(self, hot_count: usize, cold_count: usize) -> MemoryUsage {
        let total_records = hot_count.saturating_add(cold_count) as u64;
        MemoryUsage {
            hot_frontier_bytes: hot_count as u64 * self.record_bytes,
            cold_frontier_bytes: cold_count as u64 * self.record_bytes,
            dedupe_bytes: total_records * self.dedupe_bytes_per_record,
            worker_scratch_bytes: self.worker_scratch_bytes,
            checkpoint_bytes: self.checkpoint_bytes,
            spill_buffer_bytes: self.spill_buffer_bytes,
            ..MemoryUsage::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FrontierPressure {
    pub governor_state: GovernorState,
    pub pressure_action: PressureAction,
    pub rss_bytes: u64,
    pub hot_count: usize,
    pub cold_candidate_count: usize,
    pub requested_cold_limit: usize,
    pub retained_cold_limit: usize,
    pub resident_cold_limit: usize,
    pub spill_backed_cold_records: usize,
    pub dropped_cold_records: usize,
}

impl Default for FrontierPressure {
    fn default() -> Self {
        Self {
            governor_state: GovernorState::Green,
            pressure_action: PressureAction::None,
            rss_bytes: 0,
            hot_count: 0,
            cold_candidate_count: 0,
            requested_cold_limit: 0,
            retained_cold_limit: 0,
            resident_cold_limit: 0,
            spill_backed_cold_records: 0,
            dropped_cold_records: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColdRetentionPlan<T> {
    pub resident: Vec<T>,
    pub spill: Vec<T>,
    pub dropped: Vec<T>,
    pub pruned: usize,
    pub decisions: Vec<BranchDecision>,
    pub pressure: FrontierPressure,
}

pub fn plan_pressure_cold_retention<T, F>(
    candidates: Vec<T>,
    hot_count: usize,
    policy: RetentionPolicy,
    runtime: FrontierRuntimeLimits,
    class_fn: F,
) -> ColdRetentionPlan<T>
where
    F: Fn(&T) -> RetentionClass,
{
    let cold_candidate_count = candidates.len();
    let mut counts = BTreeMap::new();
    let mut quota_retained = Vec::new();
    let mut dropped = Vec::new();
    let mut prunes = Vec::new();

    for candidate in candidates {
        let retention_class = class_fn(&candidate);
        let count = counts.entry(retention_class).or_insert(0usize);
        if *count < policy.quota_for(retention_class) {
            *count += 1;
            quota_retained.push(candidate);
        } else {
            dropped.push(candidate);
            prunes.push(BranchDecision::Prune(PruneClass::HeuristicShaping));
        }
    }

    let requested_cold_limit = policy.cold_limit.min(quota_retained.len());
    if quota_retained.len() > requested_cold_limit {
        for candidate in quota_retained.drain(requested_cold_limit..) {
            dropped.push(candidate);
            prunes.push(BranchDecision::Prune(PruneClass::HeuristicShaping));
        }
    }

    let governor_decision = evaluate_governor(
        runtime.governor,
        runtime.usage_for(hot_count, quota_retained.len()),
    );
    let resident_cold_limit = if governor_decision.action.spills() {
        quota_retained
            .len()
            .min(runtime.spill.resident_cold_records.max(1))
    } else {
        quota_retained.len()
    };
    let spill = quota_retained.split_off(resident_cold_limit);
    let resident = quota_retained;
    let pruned = dropped.len();

    ColdRetentionPlan {
        resident,
        spill,
        dropped,
        pruned,
        decisions: prunes,
        pressure: FrontierPressure {
            governor_state: governor_decision.state,
            pressure_action: governor_decision.action,
            rss_bytes: governor_decision.rss_bytes,
            hot_count,
            cold_candidate_count,
            requested_cold_limit,
            retained_cold_limit: requested_cold_limit,
            resident_cold_limit,
            spill_backed_cold_records: requested_cold_limit.saturating_sub(resident_cold_limit),
            dropped_cold_records: pruned,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FrontierRuntimeLimits, plan_pressure_cold_retention, retain_with_obligation_policy,
        retain_with_quota_by,
    };
    use crate::branch_bound::{BranchDecision, PruneClass};
    use pen_store::{memory::GovernorConfig, spill::SpillConfig};
    use pen_type::obligations::{RetentionClass, RetentionFocus, RetentionPolicy};

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

    #[test]
    fn obligation_policy_reserves_rare_heads_ahead_of_generic_macro_pressure() {
        let candidates = vec![
            ("macro-a", RetentionClass::GenericMacro),
            ("focus-a", RetentionClass::RareFocusHead),
            ("focus-b", RetentionClass::RareFocusHead),
            ("bridge-a", RetentionClass::RareBridgeHead),
            ("support-a", RetentionClass::StructuralSupport),
            ("macro-b", RetentionClass::GenericMacro),
        ];
        let policy = RetentionPolicy {
            focus: RetentionFocus::Axiomatic,
            focus_quota: 2,
            bridge_quota: 1,
            support_quota: 1,
            macro_quota: 0,
            cold_limit: 6,
        };
        let (kept, pruned, decisions) =
            retain_with_obligation_policy(candidates, policy, |candidate| candidate.1);

        assert_eq!(
            kept,
            vec![
                ("focus-a", RetentionClass::RareFocusHead),
                ("focus-b", RetentionClass::RareFocusHead),
                ("bridge-a", RetentionClass::RareBridgeHead),
                ("support-a", RetentionClass::StructuralSupport),
            ]
        );
        assert_eq!(pruned, 2);
        assert_eq!(
            decisions,
            vec![
                BranchDecision::Prune(PruneClass::HeuristicShaping),
                BranchDecision::Prune(PruneClass::HeuristicShaping),
            ]
        );
    }

    #[test]
    fn pressure_retention_keeps_rare_heads_resident_and_spills_later_cold_states() {
        let candidates = vec![
            ("focus-a", RetentionClass::RareFocusHead),
            ("bridge-a", RetentionClass::RareBridgeHead),
            ("support-a", RetentionClass::StructuralSupport),
        ];
        let policy = RetentionPolicy {
            focus: RetentionFocus::Temporal,
            focus_quota: 1,
            bridge_quota: 1,
            support_quota: 1,
            macro_quota: 0,
            cold_limit: 3,
        };
        let runtime = FrontierRuntimeLimits {
            worker_count: 1,
            governor: GovernorConfig {
                green_limit_bytes: 1,
                yellow_limit_bytes: 2,
                orange_limit_bytes: 1_000,
                red_limit_bytes: 2_000,
                hard_limit_bytes: 3_000,
            },
            spill: SpillConfig {
                max_records_per_shard: 32,
                max_dedupe_keys_per_segment: 32,
                resident_cold_records: 1,
            },
            record_bytes: 32,
            dedupe_bytes_per_record: 0,
            worker_scratch_bytes: 0,
            checkpoint_bytes: 0,
            spill_buffer_bytes: 0,
        };

        let plan =
            plan_pressure_cold_retention(candidates, 1, policy, runtime, |candidate| candidate.1);

        assert_eq!(
            plan.resident,
            vec![("focus-a", RetentionClass::RareFocusHead)]
        );
        assert_eq!(
            plan.spill,
            vec![
                ("bridge-a", RetentionClass::RareBridgeHead),
                ("support-a", RetentionClass::StructuralSupport),
            ]
        );
        assert!(plan.dropped.is_empty());
        assert_eq!(
            plan.pressure.pressure_action,
            pen_store::memory::PressureAction::SpillCold
        );
        assert_eq!(plan.pressure.resident_cold_limit, 1);
        assert_eq!(plan.pressure.spill_backed_cold_records, 2);
    }
}
