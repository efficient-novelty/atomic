use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernorState {
    Green,
    Yellow,
    Orange,
    Red,
    Black,
}

impl GovernorState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Orange => "orange",
            Self::Red => "red",
            Self::Black => "black",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PressureAction {
    None,
    CompactQueues,
    SpillCold,
    SpillAndCheckpoint,
    EmergencyCheckpoint,
}

impl PressureAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::CompactQueues => "compact_queues",
            Self::SpillCold => "spill_cold",
            Self::SpillAndCheckpoint => "spill_and_checkpoint",
            Self::EmergencyCheckpoint => "emergency_checkpoint",
        }
    }

    pub const fn spills(self) -> bool {
        matches!(
            self,
            Self::SpillCold | Self::SpillAndCheckpoint | Self::EmergencyCheckpoint
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GovernorConfig {
    pub green_limit_bytes: u64,
    pub yellow_limit_bytes: u64,
    pub orange_limit_bytes: u64,
    pub red_limit_bytes: u64,
    pub hard_limit_bytes: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MemoryUsage {
    pub hot_frontier_bytes: u64,
    pub cold_frontier_bytes: u64,
    pub interner_bytes: u64,
    pub dedupe_bytes: u64,
    pub cache_bytes: u64,
    pub worker_scratch_bytes: u64,
    pub checkpoint_bytes: u64,
    pub spill_buffer_bytes: u64,
}

impl MemoryUsage {
    pub fn rss_bytes(self) -> u64 {
        self.hot_frontier_bytes
            + self.cold_frontier_bytes
            + self.interner_bytes
            + self.dedupe_bytes
            + self.cache_bytes
            + self.worker_scratch_bytes
            + self.checkpoint_bytes
            + self.spill_buffer_bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GovernorDecision {
    pub state: GovernorState,
    pub action: PressureAction,
    pub rss_bytes: u64,
    pub exceeded_hard_limit: bool,
}

pub fn evaluate_governor(config: GovernorConfig, usage: MemoryUsage) -> GovernorDecision {
    let rss_bytes = usage.rss_bytes();
    let (state, action) = if rss_bytes > config.red_limit_bytes {
        (GovernorState::Black, PressureAction::EmergencyCheckpoint)
    } else if rss_bytes >= config.orange_limit_bytes {
        (GovernorState::Red, PressureAction::SpillAndCheckpoint)
    } else if rss_bytes >= config.yellow_limit_bytes {
        (GovernorState::Orange, PressureAction::SpillCold)
    } else if rss_bytes >= config.green_limit_bytes {
        (GovernorState::Yellow, PressureAction::CompactQueues)
    } else {
        (GovernorState::Green, PressureAction::None)
    };

    GovernorDecision {
        state,
        action,
        rss_bytes,
        exceeded_hard_limit: rss_bytes > config.hard_limit_bytes,
    }
}

#[cfg(test)]
mod tests {
    use super::{GovernorConfig, GovernorState, MemoryUsage, PressureAction, evaluate_governor};

    #[test]
    fn governor_transitions_follow_frozen_threshold_order() {
        let config = GovernorConfig {
            green_limit_bytes: 75,
            yellow_limit_bytes: 90,
            orange_limit_bytes: 105,
            red_limit_bytes: 115,
            hard_limit_bytes: 120,
        };

        assert_eq!(
            evaluate_governor(
                config,
                MemoryUsage {
                    hot_frontier_bytes: 50,
                    ..MemoryUsage::default()
                }
            ),
            super::GovernorDecision {
                state: GovernorState::Green,
                action: PressureAction::None,
                rss_bytes: 50,
                exceeded_hard_limit: false,
            }
        );
        assert_eq!(
            evaluate_governor(
                config,
                MemoryUsage {
                    hot_frontier_bytes: 80,
                    ..MemoryUsage::default()
                }
            )
            .state,
            GovernorState::Yellow
        );
        assert_eq!(
            evaluate_governor(
                config,
                MemoryUsage {
                    hot_frontier_bytes: 95,
                    ..MemoryUsage::default()
                }
            )
            .action,
            PressureAction::SpillCold
        );
        assert_eq!(
            evaluate_governor(
                config,
                MemoryUsage {
                    hot_frontier_bytes: 110,
                    ..MemoryUsage::default()
                }
            )
            .state,
            GovernorState::Red
        );
        let black = evaluate_governor(
            config,
            MemoryUsage {
                hot_frontier_bytes: 121,
                ..MemoryUsage::default()
            },
        );
        assert_eq!(black.state, GovernorState::Black);
        assert!(black.exceeded_hard_limit);
    }
}
