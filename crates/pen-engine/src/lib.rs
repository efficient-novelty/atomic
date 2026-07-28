//! Oracle-free orchestration boundary.
//!
//! The incomplete Law V2 lane intentionally has no `Ready`, `Advance`,
//! `Blocked`, or `Halt` constructor. It can report only `Unknown` until every
//! proof layer needed by the two laws is installed.

#![forbid(unsafe_code)]

use pen_law::{DEMAND_WINDOW_WIDTH, Digest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceLimits {
    pub max_resident_bytes: u64,
    pub max_frontier_items: u64,
    pub worker_count: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LawfulRuntimeConfig {
    pub window_width: u8,
    pub kernel_digest: Digest,
    pub grammar_digest: Digest,
    pub scheme_calculus_digest: Digest,
    pub law_digest: Digest,
    pub resources: ResourceLimits,
}

impl LawfulRuntimeConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.window_width != DEMAND_WINDOW_WIDTH {
            return Err(ConfigError::WrongWindowWidth {
                expected: DEMAND_WINDOW_WIDTH,
                actual: self.window_width,
            });
        }
        if self.resources.max_resident_bytes == 0
            || self.resources.max_frontier_items == 0
            || self.resources.worker_count == 0
        {
            return Err(ConfigError::ZeroResourceLimit);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ConfigError {
    #[error("law-v2 requires window width {expected}, received {actual}")]
    WrongWindowWidth { expected: u8, actual: u8 },
    #[error("resource limits must be positive")]
    ZeroResourceLimit,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingComponent {
    AmbientCubicalKernel,
    AuthoritativeDemandCensus,
    ResponseCone,
    SemanticAudit,
    FreeSealingUniversalProperty,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReadinessManifest {
    missing: Vec<MissingComponent>,
}

impl ReadinessManifest {
    pub fn certificate_types_milestone() -> Self {
        Self {
            missing: vec![
                MissingComponent::AmbientCubicalKernel,
                MissingComponent::AuthoritativeDemandCensus,
                MissingComponent::ResponseCone,
                MissingComponent::SemanticAudit,
                MissingComponent::FreeSealingUniversalProperty,
            ],
        }
    }

    pub fn missing(&self) -> &[MissingComponent] {
        &self.missing
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum StartupOutcome {
    Unknown {
        reason: String,
        manifest: ReadinessManifest,
    },
}

pub fn certificate_types_milestone_outcome() -> StartupOutcome {
    StartupOutcome::Unknown {
        reason: "the full Law V2 proof stack is incomplete".to_owned(),
        manifest: ReadinessManifest::certificate_types_milestone(),
    }
}

/// Compatibility name retained for callers introduced with the first
/// oracle-firewall slice.
pub fn firewall_milestone_outcome() -> StartupOutcome {
    certificate_types_milestone_outcome()
}

#[cfg(test)]
mod tests {
    use super::{
        ConfigError, LawfulRuntimeConfig, ResourceLimits, StartupOutcome,
        certificate_types_milestone_outcome,
    };
    use pen_law::{DEMAND_WINDOW_WIDTH, Digest};

    #[test]
    fn firewall_milestone_has_no_success_path() {
        assert!(matches!(
            certificate_types_milestone_outcome(),
            StartupOutcome::Unknown { .. }
        ));
    }

    fn config() -> LawfulRuntimeConfig {
        LawfulRuntimeConfig {
            window_width: DEMAND_WINDOW_WIDTH,
            kernel_digest: Digest::of_bytes(b"kernel"),
            grammar_digest: Digest::of_bytes(b"grammar"),
            scheme_calculus_digest: Digest::of_bytes(b"schemes"),
            law_digest: Digest::of_bytes(b"law"),
            resources: ResourceLimits {
                max_resident_bytes: 1,
                max_frontier_items: 1,
                worker_count: 1,
            },
        }
    }

    #[test]
    fn lawful_config_contains_only_fixed_memory_and_resource_inputs() {
        config().validate().expect("valid contract");
        let mut wrong_width = config();
        wrong_width.window_width = DEMAND_WINDOW_WIDTH + 1;
        assert_eq!(
            wrong_width.validate(),
            Err(ConfigError::WrongWindowWidth {
                expected: DEMAND_WINDOW_WIDTH,
                actual: DEMAND_WINDOW_WIDTH + 1,
            })
        );
    }

    #[test]
    fn unknown_configuration_fields_fail_closed() {
        let mut value = serde_json::to_value(config()).expect("serialize config");
        value
            .as_object_mut()
            .expect("config is an object")
            .insert("forbidden_endpoint".to_owned(), serde_json::json!(99));
        assert!(serde_json::from_value::<LawfulRuntimeConfig>(value).is_err());
    }
}
