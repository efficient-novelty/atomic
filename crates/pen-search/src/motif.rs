use pen_core::clause::ClauseRec;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MotifPolicy {
    pub enabled: bool,
}

impl MotifPolicy {
    pub const fn disabled() -> Self {
        Self { enabled: false }
    }
}

pub fn motif_suggestions(policy: MotifPolicy) -> Vec<ClauseRec> {
    if policy.enabled {
        Vec::new()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{MotifPolicy, motif_suggestions};

    #[test]
    fn motifs_are_disabled_in_early_phases() {
        assert!(motif_suggestions(MotifPolicy::disabled()).is_empty());
    }
}
