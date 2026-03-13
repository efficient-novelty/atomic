use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RuntimeConfig {
    pub mode: ModeConfig,
    pub search: SearchConfig,
    pub objective: ObjectiveConfig,
    pub memory: MemoryConfig,
    pub checkpoint: CheckpointConfig,
    pub accel: AccelConfig,
}

impl RuntimeConfig {
    pub fn desktop_16gb() -> Self {
        Self {
            mode: ModeConfig {
                strict: true,
                debug: false,
            },
            search: SearchConfig {
                until_step: 15,
                workers: WorkerSetting::Auto,
                max_workers: 12,
                frontier_mode: "obligation_guided".to_owned(),
                learned_motifs: false,
            },
            objective: ObjectiveConfig {
                exact_clause_policy: "strict".to_owned(),
                window_depth: 2,
                selector: "minimal_positive_overshoot".to_owned(),
            },
            memory: MemoryConfig {
                reserve_for_os_gib: 4.0,
                target_rss_gib: 8.5,
                soft_rss_gib: 9.0,
                pressure_rss_gib: 10.5,
                emergency_rss_gib: 11.5,
                hard_rss_gib: 12.0,
                hot_frontier_gib: 2.25,
                intern_gib: 2.0,
                dedupe_gib: 1.25,
                cache_gib: 0.75,
                spill_buffers_gib: 0.50,
                checkpoint_buffers_gib: 0.25,
                worker_arena_mib: 64,
            },
            checkpoint: CheckpointConfig {
                root: "runs".to_owned(),
                step_on_accept: true,
                frontier_interval_sec: 180,
                retain_frontier_generations: 3,
                compression: "zstd".to_owned(),
            },
            accel: AccelConfig {
                backend: "auto".to_owned(),
                verify_on_cpu: true,
            },
        }
    }

    pub fn from_toml_str(source: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(source)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModeConfig {
    pub strict: bool,
    pub debug: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchConfig {
    pub until_step: u32,
    pub workers: WorkerSetting,
    pub max_workers: u16,
    pub frontier_mode: String,
    pub learned_motifs: bool,
}

impl SearchConfig {
    pub fn resolve_workers(&self, logical_cpus: u16) -> u16 {
        match self.workers {
            WorkerSetting::Fixed(workers) => workers.min(self.max_workers.max(1)),
            WorkerSetting::Auto => logical_cpus
                .saturating_sub(2)
                .clamp(1, self.max_workers.max(1)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerSetting {
    Auto,
    Fixed(u16),
}

impl Serialize for WorkerSetting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Fixed(value) => serializer.serialize_u16(*value),
        }
    }
}

impl<'de> Deserialize<'de> for WorkerSetting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WorkerSettingVisitor;

        impl<'de> Visitor<'de> for WorkerSettingVisitor {
            type Value = WorkerSetting;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("the string 'auto' or a positive worker count")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "auto" => Ok(WorkerSetting::Auto),
                    _ => Err(E::custom(format!(
                        "unsupported worker setting '{value}', expected 'auto'"
                    ))),
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = u16::try_from(value)
                    .map_err(|_| E::custom("worker count exceeded u16 range"))?;
                Ok(WorkerSetting::Fixed(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value < 0 {
                    return Err(E::custom("worker count must be non-negative"));
                }
                self.visit_u64(value as u64)
            }
        }

        deserializer.deserialize_any(WorkerSettingVisitor)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ObjectiveConfig {
    pub exact_clause_policy: String,
    pub window_depth: u16,
    pub selector: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MemoryConfig {
    pub reserve_for_os_gib: f64,
    pub target_rss_gib: f64,
    pub soft_rss_gib: f64,
    pub pressure_rss_gib: f64,
    pub emergency_rss_gib: f64,
    pub hard_rss_gib: f64,
    pub hot_frontier_gib: f64,
    pub intern_gib: f64,
    pub dedupe_gib: f64,
    pub cache_gib: f64,
    pub spill_buffers_gib: f64,
    pub checkpoint_buffers_gib: f64,
    pub worker_arena_mib: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckpointConfig {
    pub root: String,
    pub step_on_accept: bool,
    pub frontier_interval_sec: u32,
    pub retain_frontier_generations: u16,
    pub compression: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccelConfig {
    pub backend: String,
    pub verify_on_cpu: bool,
}

#[cfg(test)]
mod tests {
    use super::{RuntimeConfig, WorkerSetting};
    use std::fs;

    fn load_config(name: &str) -> RuntimeConfig {
        let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("workspace root");
        let source = fs::read_to_string(workspace.join("configs").join(name))
            .expect("config file should exist");
        RuntimeConfig::from_toml_str(&source).expect("config should parse")
    }

    #[test]
    fn desktop_profile_matches_frozen_defaults() {
        let config = load_config("desktop_16gb.toml");
        assert_eq!(config, RuntimeConfig::desktop_16gb());
        assert_eq!(config.search.resolve_workers(16), 12);
    }

    #[test]
    fn debug_profile_keeps_search_single_threaded() {
        let config = load_config("debug.toml");
        assert_eq!(config.search.workers, WorkerSetting::Fixed(1));
        assert_eq!(config.search.resolve_workers(16), 1);
        assert!(config.mode.debug);
    }
}
