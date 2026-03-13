use crate::state::FrontierStateRecV1;
use pen_core::ids::StateId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PriorityInputs {
    pub band_index: u8,
    pub nu_lower_bound: u16,
    pub bit_kappa_used: u16,
    pub clause_kappa_used: u16,
    pub depth: u16,
    pub state_id: StateId,
}

pub fn build_priority_key(inputs: PriorityInputs) -> u32 {
    let band = u32::from(inputs.band_index & 0x0f) << 28;
    let nu = u32::from(inputs.nu_lower_bound.min(0x0fff)) << 16;
    let bit_kappa = u32::from(inputs.bit_kappa_used.min(0x00ff)) << 8;
    let clause_and_depth =
        u32::from(inputs.clause_kappa_used.min(0x0f)) << 4 | u32::from(inputs.depth.min(0x0f));
    band | nu | bit_kappa | clause_and_depth
}

pub fn priority_tuple(record: &FrontierStateRecV1) -> (u8, u16, u16, u16, u16, u64) {
    (
        record.band_index,
        record.nu_lower_bound,
        record.bit_kappa_used,
        record.clause_kappa_used,
        record.depth,
        record.state_id.get(),
    )
}

#[cfg(test)]
mod tests {
    use super::{PriorityInputs, build_priority_key};
    use pen_core::ids::StateId;

    #[test]
    fn priority_key_is_deterministic_and_band_first() {
        let low_band = build_priority_key(PriorityInputs {
            band_index: 1,
            nu_lower_bound: 17,
            bit_kappa_used: 80,
            clause_kappa_used: 4,
            depth: 6,
            state_id: StateId::new(1),
        });
        let high_band = build_priority_key(PriorityInputs {
            band_index: 4,
            nu_lower_bound: 17,
            bit_kappa_used: 80,
            clause_kappa_used: 4,
            depth: 6,
            state_id: StateId::new(1),
        });

        assert!(low_band < high_band);
        assert_eq!(
            low_band,
            build_priority_key(PriorityInputs {
                band_index: 1,
                nu_lower_bound: 17,
                bit_kappa_used: 80,
                clause_kappa_used: 4,
                depth: 6,
                state_id: StateId::new(1),
            })
        );
    }
}
