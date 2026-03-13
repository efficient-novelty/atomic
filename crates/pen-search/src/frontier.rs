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
        self.hot.sort_by_key(priority_tuple);
        self.cold.sort_by_key(priority_tuple);
    }

    pub fn priority_heads(&self, count: usize) -> Vec<u32> {
        let mut heads: Vec<_> = self.hot.iter().map(|record| record.priority_key).collect();
        heads.sort_unstable();
        heads.truncate(count);
        heads
    }
}

#[cfg(test)]
mod tests {
    use super::FrontierWindow;
    use crate::priority::{PriorityInputs, build_priority_key};
    use crate::state::{FrontierStateRecV1, PrefixState};
    use pen_core::ids::{ClauseId, ObligationSetId, StateId};

    fn record(state_id: u64, band_index: u8, lower_bound: u16) -> FrontierStateRecV1 {
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
        FrontierStateRecV1::from_prefix(prefix, priority, 0)
    }

    #[test]
    fn frontier_window_compacts_and_exposes_priority_heads() {
        let mut frontier = FrontierWindow::default();
        frontier.push_hot(record(3, 4, 30));
        frontier.push_hot(record(1, 1, 10));
        frontier.push_cold(record(2, 2, 20));

        frontier.compact_sorted();

        assert_eq!(frontier.total_len(), 3);
        assert_eq!(frontier.hot[0].state_id.get(), 1);
        assert_eq!(frontier.priority_heads(2).len(), 2);
    }
}
