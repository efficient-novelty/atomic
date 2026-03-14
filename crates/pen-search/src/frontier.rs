use crate::priority::priority_tuple;
use crate::state::FrontierStateRecV1;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrontierWindow {
    pub hot: Vec<FrontierStateRecV1>,
    pub cold: Vec<FrontierStateRecV1>,
}

impl FrontierWindow {
    pub fn push_hot(&mut self, record: FrontierStateRecV1) {
        self.hot.push(record);
    }

    pub fn push_cold(&mut self, record: FrontierStateRecV1) {
        self.cold.push(record);
    }

    pub fn total_len(&self) -> usize {
        self.hot.len() + self.cold.len()
    }

    pub fn compact_sorted(&mut self) {
        self.hot.sort_by_key(frontier_order_key);
        self.cold.sort_by_key(frontier_order_key);
    }

    pub fn priority_heads(&self, count: usize) -> Vec<u32> {
        self.hot
            .iter()
            .take(count)
            .map(|record| record.priority_key)
            .collect()
    }
}

fn frontier_order_key(record: &FrontierStateRecV1) -> (u8, u8, u16, u16, u16, u16, u64) {
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

#[cfg(test)]
mod tests {
    use super::FrontierWindow;
    use crate::priority::{PriorityInputs, build_priority_key};
    use crate::state::{FrontierStateRecV1, PrefixState};
    use pen_core::ids::{ClauseId, ObligationSetId, StateId};
    use pen_type::obligations::RetentionClass;

    fn record(
        state_id: u64,
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
        FrontierStateRecV1::from_prefix(prefix, priority, 0)
    }

    #[test]
    fn frontier_window_compacts_and_exposes_priority_heads() {
        let mut frontier = FrontierWindow::default();
        frontier.push_hot(record(3, 4, 30, RetentionClass::GenericMacro));
        frontier.push_hot(record(1, 1, 10, RetentionClass::RareFocusHead));
        frontier.push_cold(record(2, 2, 20, RetentionClass::StructuralSupport));

        frontier.compact_sorted();

        assert_eq!(frontier.total_len(), 3);
        assert_eq!(frontier.hot[0].state_id.get(), 1);
        assert_eq!(frontier.priority_heads(2).len(), 2);
    }

    #[test]
    fn priority_heads_follow_frontier_order_instead_of_raw_priority_sorting() {
        let mut frontier = FrontierWindow::default();
        frontier.push_hot(record(1, 4, 5, RetentionClass::GenericMacro));
        frontier.push_hot(record(2, 4, 7, RetentionClass::RareFocusHead));

        frontier.compact_sorted();

        assert_eq!(frontier.hot[0].state_id.get(), 2);
        assert_eq!(
            frontier.priority_heads(2),
            frontier
                .hot
                .iter()
                .take(2)
                .map(|record| record.priority_key)
                .collect::<Vec<_>>()
        );
    }
}
