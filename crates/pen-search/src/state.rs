use pen_core::ids::{ClauseId, ObligationSetId, StateId};

pub const FRONTIER_STATE_REC_V1_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrefixState {
    pub state_id: StateId,
    pub parent_state_id: StateId,
    pub last_clause_id: ClauseId,
    pub obligation_set_id: ObligationSetId,
    pub shape_hash64: u64,
    pub support_hash64: u64,
    pub nu_lower_bound: u16,
    pub nu_upper_bound: u16,
    pub bit_kappa_used: u16,
    pub clause_kappa_used: u16,
    pub depth: u16,
    pub step_index: u8,
    pub band_index: u8,
    pub flags: u16,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FrontierStateRecV1 {
    pub state_id: StateId,
    pub parent_state_id: StateId,
    pub last_clause_id: ClauseId,
    pub obligation_set_id: ObligationSetId,
    pub shape_hash64: u64,
    pub support_hash64: u64,
    pub nu_lower_bound: u16,
    pub nu_upper_bound: u16,
    pub bit_kappa_used: u16,
    pub clause_kappa_used: u16,
    pub depth: u16,
    pub step_index: u8,
    pub band_index: u8,
    pub flags: u16,
    pub priority_key: u32,
    pub worker_hint: u16,
    pub reserved: u32,
}

impl FrontierStateRecV1 {
    pub const BYTE_LEN: usize = FRONTIER_STATE_REC_V1_BYTES;

    pub fn from_prefix(prefix: PrefixState, priority_key: u32, worker_hint: u16) -> Self {
        Self {
            state_id: prefix.state_id,
            parent_state_id: prefix.parent_state_id,
            last_clause_id: prefix.last_clause_id,
            obligation_set_id: prefix.obligation_set_id,
            shape_hash64: prefix.shape_hash64,
            support_hash64: prefix.support_hash64,
            nu_lower_bound: prefix.nu_lower_bound,
            nu_upper_bound: prefix.nu_upper_bound,
            bit_kappa_used: prefix.bit_kappa_used,
            clause_kappa_used: prefix.clause_kappa_used,
            depth: prefix.depth,
            step_index: prefix.step_index,
            band_index: prefix.band_index,
            flags: prefix.flags,
            priority_key,
            worker_hint,
            reserved: 0,
        }
    }

    pub fn to_le_bytes(self) -> [u8; Self::BYTE_LEN] {
        let mut bytes = [0_u8; Self::BYTE_LEN];
        bytes[0..8].copy_from_slice(&self.state_id.get().to_le_bytes());
        bytes[8..16].copy_from_slice(&self.parent_state_id.get().to_le_bytes());
        bytes[16..20].copy_from_slice(&self.last_clause_id.get().to_le_bytes());
        bytes[20..24].copy_from_slice(&self.obligation_set_id.get().to_le_bytes());
        bytes[24..32].copy_from_slice(&self.shape_hash64.to_le_bytes());
        bytes[32..40].copy_from_slice(&self.support_hash64.to_le_bytes());
        bytes[40..42].copy_from_slice(&self.nu_lower_bound.to_le_bytes());
        bytes[42..44].copy_from_slice(&self.nu_upper_bound.to_le_bytes());
        bytes[44..46].copy_from_slice(&self.bit_kappa_used.to_le_bytes());
        bytes[46..48].copy_from_slice(&self.clause_kappa_used.to_le_bytes());
        bytes[48..50].copy_from_slice(&self.depth.to_le_bytes());
        bytes[50] = self.step_index;
        bytes[51] = self.band_index;
        bytes[52..54].copy_from_slice(&self.flags.to_le_bytes());
        bytes[54..58].copy_from_slice(&self.priority_key.to_le_bytes());
        bytes[58..60].copy_from_slice(&self.worker_hint.to_le_bytes());
        bytes[60..64].copy_from_slice(&self.reserved.to_le_bytes());
        bytes
    }

    pub fn from_le_bytes(bytes: [u8; Self::BYTE_LEN]) -> Self {
        Self {
            state_id: StateId::new(u64::from_le_bytes(
                bytes[0..8].try_into().expect("state_id"),
            )),
            parent_state_id: StateId::new(u64::from_le_bytes(
                bytes[8..16].try_into().expect("parent_state_id"),
            )),
            last_clause_id: ClauseId::new(u32::from_le_bytes(
                bytes[16..20].try_into().expect("last_clause_id"),
            )),
            obligation_set_id: ObligationSetId::new(u32::from_le_bytes(
                bytes[20..24].try_into().expect("obligation_set_id"),
            )),
            shape_hash64: u64::from_le_bytes(bytes[24..32].try_into().expect("shape_hash64")),
            support_hash64: u64::from_le_bytes(bytes[32..40].try_into().expect("support_hash64")),
            nu_lower_bound: u16::from_le_bytes(bytes[40..42].try_into().expect("nu_lower_bound")),
            nu_upper_bound: u16::from_le_bytes(bytes[42..44].try_into().expect("nu_upper_bound")),
            bit_kappa_used: u16::from_le_bytes(bytes[44..46].try_into().expect("bit_kappa_used")),
            clause_kappa_used: u16::from_le_bytes(
                bytes[46..48].try_into().expect("clause_kappa_used"),
            ),
            depth: u16::from_le_bytes(bytes[48..50].try_into().expect("depth")),
            step_index: bytes[50],
            band_index: bytes[51],
            flags: u16::from_le_bytes(bytes[52..54].try_into().expect("flags")),
            priority_key: u32::from_le_bytes(bytes[54..58].try_into().expect("priority_key")),
            worker_hint: u16::from_le_bytes(bytes[58..60].try_into().expect("worker_hint")),
            reserved: u32::from_le_bytes(bytes[60..64].try_into().expect("reserved")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FRONTIER_STATE_REC_V1_BYTES, FrontierStateRecV1, PrefixState};
    use pen_core::ids::{ClauseId, ObligationSetId, StateId};

    #[test]
    fn frontier_record_round_trips_exact_64_byte_layout() {
        let prefix = PrefixState {
            state_id: StateId::new(11),
            parent_state_id: StateId::new(7),
            last_clause_id: ClauseId::new(3),
            obligation_set_id: ObligationSetId::new(5),
            shape_hash64: 101,
            support_hash64: 117,
            nu_lower_bound: 13,
            nu_upper_bound: 19,
            bit_kappa_used: 76,
            clause_kappa_used: 4,
            depth: 9,
            step_index: 10,
            band_index: 4,
            flags: 2,
        };

        let record = FrontierStateRecV1::from_prefix(prefix, 144, 12);
        let bytes = record.to_le_bytes();

        assert_eq!(bytes.len(), FRONTIER_STATE_REC_V1_BYTES);
        assert_eq!(FrontierStateRecV1::from_le_bytes(bytes), record);
    }
}
