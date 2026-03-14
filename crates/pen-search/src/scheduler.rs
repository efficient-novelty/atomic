use crate::frontier::FrontierWindow;
use crate::priority::priority_tuple;
use crate::state::FrontierStateRecV1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerAssignment {
    pub worker_id: u16,
    pub records: Vec<FrontierStateRecV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchedulerPlan {
    pub assignments: Vec<WorkerAssignment>,
}

pub fn build_schedule(frontier: &FrontierWindow, worker_count: u16) -> SchedulerPlan {
    let worker_count = worker_count.max(1);
    let mut assignments: Vec<_> = (0..worker_count)
        .map(|worker_id| WorkerAssignment {
            worker_id,
            records: Vec::new(),
        })
        .collect();

    let mut records = frontier.hot.clone();
    records.extend(frontier.cold.iter().copied());
    records.sort_by_key(priority_tuple);

    for (ordinal, record) in records.into_iter().enumerate() {
        let worker_index =
            preferred_worker(record, worker_count).unwrap_or((ordinal as u16) % worker_count);
        assignments[usize::from(worker_index)].records.push(record);
    }

    SchedulerPlan { assignments }
}

fn preferred_worker(record: FrontierStateRecV1, worker_count: u16) -> Option<u16> {
    (record.worker_hint < worker_count).then_some(record.worker_hint)
}

#[cfg(test)]
mod tests {
    use super::build_schedule;
    use crate::frontier::FrontierWindow;
    use crate::priority::{PriorityInputs, build_priority_key};
    use crate::state::{FrontierStateRecV1, PrefixState};
    use pen_core::ids::{ClauseId, ObligationSetId, StateId};

    fn record(
        state_id: u64,
        worker_hint: u16,
        band_index: u8,
        lower_bound: u16,
    ) -> FrontierStateRecV1 {
        let prefix = PrefixState {
            state_id: StateId::new(state_id),
            parent_state_id: StateId::new(0),
            last_clause_id: ClauseId::new(0),
            obligation_set_id: ObligationSetId::new(0),
            shape_hash64: 0,
            support_hash64: 0,
            nu_lower_bound: lower_bound,
            nu_upper_bound: lower_bound + 2,
            bit_kappa_used: 80,
            clause_kappa_used: 4,
            depth: 3,
            step_index: 10,
            band_index,
            flags: 0,
        };
        let priority = build_priority_key(PriorityInputs {
            band_index,
            nu_lower_bound: lower_bound,
            bit_kappa_used: 80,
            clause_kappa_used: 4,
            depth: 3,
            state_id: StateId::new(state_id),
        });
        FrontierStateRecV1::from_prefix(prefix, priority, worker_hint)
    }

    #[test]
    fn scheduler_is_deterministic_and_respects_worker_hints() {
        let mut frontier = FrontierWindow::default();
        frontier.push_hot(record(1, 1, 1, 10));
        frontier.push_hot(record(2, 0, 2, 20));
        frontier.push_cold(record(3, 1, 3, 30));

        let first = build_schedule(&frontier, 2);
        let second = build_schedule(&frontier, 2);

        assert_eq!(first, second);
        assert_eq!(first.assignments[1].records[0].state_id.get(), 1);
        assert_eq!(first.assignments[1].records[1].state_id.get(), 3);
    }
}
