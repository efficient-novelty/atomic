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
    records.sort_by_key(schedule_order_key);

    for (ordinal, record) in records.into_iter().enumerate() {
        let worker_index = if record.retention_class().is_rare_head() {
            least_loaded_worker(&assignments)
        } else {
            preferred_worker(record, worker_count).unwrap_or((ordinal as u16) % worker_count)
        };
        assignments[usize::from(worker_index)].records.push(record);
    }

    SchedulerPlan { assignments }
}

fn schedule_order_key(record: &FrontierStateRecV1) -> (u8, u8, u16, u16, u16, u16, u64) {
    let priority = priority_tuple(record);
    (
        record.retention_class().priority_rank(),
        priority.0,
        priority.1,
        priority.2,
        priority.3,
        priority.4,
        priority.5,
    )
}

fn preferred_worker(record: FrontierStateRecV1, worker_count: u16) -> Option<u16> {
    (record.worker_hint < worker_count).then_some(record.worker_hint)
}

fn least_loaded_worker(assignments: &[WorkerAssignment]) -> u16 {
    assignments
        .iter()
        .min_by_key(|assignment| (assignment.records.len(), assignment.worker_id))
        .map(|assignment| assignment.worker_id)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::build_schedule;
    use crate::frontier::FrontierWindow;
    use crate::priority::{PriorityInputs, build_priority_key};
    use crate::state::{FrontierStateRecV1, PrefixState};
    use pen_core::ids::{ClauseId, ObligationSetId, StateId};
    use pen_type::obligations::RetentionClass;

    fn record(
        state_id: u64,
        worker_hint: u16,
        band_index: u8,
        lower_bound: u16,
        retention_class: RetentionClass,
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
            flags: match retention_class {
                RetentionClass::GenericMacro => 0,
                RetentionClass::StructuralSupport => 0b01 << 10,
                RetentionClass::RareBridgeHead => 0b10 << 10,
                RetentionClass::RareFocusHead => 0b11 << 10,
            },
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
        frontier.push_hot(record(1, 1, 1, 10, RetentionClass::StructuralSupport));
        frontier.push_hot(record(2, 0, 2, 20, RetentionClass::GenericMacro));
        frontier.push_cold(record(3, 1, 3, 30, RetentionClass::StructuralSupport));

        let first = build_schedule(&frontier, 2);
        let second = build_schedule(&frontier, 2);

        assert_eq!(first, second);
        assert_eq!(first.assignments[1].records[0].state_id.get(), 1);
        assert_eq!(first.assignments[1].records[1].state_id.get(), 3);
    }

    #[test]
    fn scheduler_spreads_rare_heads_before_following_worker_hints() {
        let mut frontier = FrontierWindow::default();
        frontier.push_hot(record(1, 1, 1, 10, RetentionClass::RareFocusHead));
        frontier.push_hot(record(2, 1, 1, 11, RetentionClass::RareBridgeHead));
        frontier.push_hot(record(3, 1, 1, 12, RetentionClass::GenericMacro));

        let schedule = build_schedule(&frontier, 2);

        assert_eq!(schedule.assignments[0].records[0].state_id.get(), 1);
        assert_eq!(schedule.assignments[1].records[0].state_id.get(), 2);
        assert_eq!(schedule.assignments[1].records[1].state_id.get(), 3);
    }
}
