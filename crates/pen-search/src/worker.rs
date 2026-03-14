use crate::branch_bound::{BranchDecision, PruneClass, sound_prune_by_bar};
use crate::scheduler::WorkerAssignment;
use pen_core::ids::StateId;
use pen_core::rational::Rational;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerBatchResult {
    pub worker_id: u16,
    pub processed_state_ids: Vec<StateId>,
    pub sound_prunes: usize,
    pub heuristic_drops: usize,
}

pub fn run_worker_batch(assignment: &WorkerAssignment, bar: Rational) -> WorkerBatchResult {
    let mut processed_state_ids = Vec::new();
    let mut sound_prunes = 0;
    let heuristic_drops = 0;

    for record in &assignment.records {
        match sound_prune_by_bar(record.nu_upper_bound, record.clause_kappa_used, bar) {
            BranchDecision::Keep => processed_state_ids.push(record.state_id),
            BranchDecision::Prune(PruneClass::SoundImpossible) => sound_prunes += 1,
            BranchDecision::Prune(PruneClass::QuotientDedupe)
            | BranchDecision::Prune(PruneClass::HeuristicShaping) => {}
        }
    }

    WorkerBatchResult {
        worker_id: assignment.worker_id,
        processed_state_ids,
        sound_prunes,
        heuristic_drops,
    }
}

#[cfg(test)]
mod tests {
    use super::run_worker_batch;
    use crate::priority::{PriorityInputs, build_priority_key};
    use crate::scheduler::WorkerAssignment;
    use crate::state::{FrontierStateRecV1, PrefixState};
    use pen_core::ids::{ClauseId, ObligationSetId, StateId};
    use pen_core::rational::Rational;

    fn record(state_id: u64, nu_upper_bound: u16, clause_kappa_used: u16) -> FrontierStateRecV1 {
        let prefix = PrefixState {
            state_id: StateId::new(state_id),
            parent_state_id: StateId::new(0),
            last_clause_id: ClauseId::new(0),
            obligation_set_id: ObligationSetId::new(0),
            shape_hash64: 0,
            support_hash64: 0,
            nu_lower_bound: nu_upper_bound.saturating_sub(1),
            nu_upper_bound,
            bit_kappa_used: 80,
            clause_kappa_used,
            depth: 1,
            step_index: 10,
            band_index: 1,
            flags: 0,
        };
        let priority = build_priority_key(PriorityInputs {
            band_index: 1,
            nu_lower_bound: prefix.nu_lower_bound,
            bit_kappa_used: 80,
            clause_kappa_used,
            depth: 1,
            state_id: StateId::new(state_id),
        });
        FrontierStateRecV1::from_prefix(prefix, priority, 0)
    }

    #[test]
    fn worker_batch_prunes_states_that_cannot_clear_the_bar() {
        let assignment = WorkerAssignment {
            worker_id: 0,
            records: vec![record(1, 12, 4), record(2, 17, 4)],
        };

        let result = run_worker_batch(&assignment, Rational::new(4, 1));
        assert_eq!(result.processed_state_ids, vec![StateId::new(2)]);
        assert_eq!(result.sound_prunes, 1);
    }
}
