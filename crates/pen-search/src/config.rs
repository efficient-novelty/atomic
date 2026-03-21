use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RuntimeConfig {
    pub mode: ModeConfig,
    pub search: SearchConfig,
    pub objective: ObjectiveConfig,
    pub memory: MemoryConfig,
    pub checkpoint: CheckpointConfig,
    pub accel: AccelConfig,
    #[serde(default)]
    pub demo: DemoConfig,
}

impl RuntimeConfig {
    pub fn desktop_16gb() -> Self {
        Self {
            mode: ModeConfig {
                strict: true,
                debug: false,
                search_profile: SearchProfile::StrictCanonGuarded,
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
            demo: DemoConfig::default(),
        }
    }

    pub fn from_toml_str(source: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(source)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchProfile {
    #[default]
    Unknown,
    StrictCanonGuarded,
    RelaxedShadow,
    RealisticFrontierShadow,
    DemoBreadthShadow,
    DesktopClaimShadow,
}

impl SearchProfile {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::StrictCanonGuarded => "strict_canon_guarded",
            Self::RelaxedShadow => "relaxed_shadow",
            Self::RealisticFrontierShadow => "realistic_frontier_shadow",
            Self::DemoBreadthShadow => "demo_breadth_shadow",
            Self::DesktopClaimShadow => "desktop_claim_shadow",
        }
    }

    pub const fn supports_narrative_artifacts(self) -> bool {
        matches!(self, Self::DemoBreadthShadow | Self::DesktopClaimShadow)
    }

    pub const fn narrative_label(self) -> &'static str {
        match self {
            Self::DemoBreadthShadow => "demo",
            Self::DesktopClaimShadow => "claim",
            _ => "lane",
        }
    }
}

const fn default_search_profile() -> SearchProfile {
    SearchProfile::StrictCanonGuarded
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModeConfig {
    pub strict: bool,
    pub debug: bool,
    #[serde(default = "default_search_profile")]
    pub search_profile: SearchProfile,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_demo_profile")]
    pub profile: String,
    #[serde(default = "default_demo_total_budget_sec")]
    pub total_budget_sec: u32,
    #[serde(default = "default_demo_early_exhaustive_budget_sec")]
    pub early_exhaustive_budget_sec: u32,
    #[serde(default = "default_demo_adaptive_reserve_sec")]
    pub adaptive_reserve_sec: u32,
    #[serde(default = "default_demo_proof_close_reserve_fraction")]
    pub proof_close_reserve_fraction: String,
    #[serde(default = "default_demo_scout_fraction")]
    pub scout_fraction: String,
    #[serde(default)]
    pub narrative: NarrativeConfig,
    #[serde(default)]
    pub floors: DemoFloors,
    #[serde(default)]
    pub caps: DemoCaps,
}

impl Default for DemoConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            profile: default_demo_profile(),
            total_budget_sec: default_demo_total_budget_sec(),
            early_exhaustive_budget_sec: default_demo_early_exhaustive_budget_sec(),
            adaptive_reserve_sec: default_demo_adaptive_reserve_sec(),
            proof_close_reserve_fraction: default_demo_proof_close_reserve_fraction(),
            scout_fraction: default_demo_scout_fraction(),
            narrative: NarrativeConfig::default(),
            floors: DemoFloors::default(),
            caps: DemoCaps::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NarrativeConfig {
    #[serde(default = "default_demo_live_console")]
    pub live_console: bool,
    #[serde(default = "default_demo_min_lines_per_step")]
    pub min_lines_per_step: u16,
    #[serde(default = "default_demo_max_lines_per_step")]
    pub max_lines_per_step: u16,
    #[serde(default = "default_demo_pulse_interval_millis")]
    pub pulse_interval_millis: u32,
    #[serde(default = "default_demo_persist_step_events")]
    pub persist_step_events: bool,
    #[serde(default = "default_demo_show_progress_bar")]
    pub show_progress_bar: bool,
}

impl Default for NarrativeConfig {
    fn default() -> Self {
        Self {
            live_console: default_demo_live_console(),
            min_lines_per_step: default_demo_min_lines_per_step(),
            max_lines_per_step: default_demo_max_lines_per_step(),
            pulse_interval_millis: default_demo_pulse_interval_millis(),
            persist_step_events: default_demo_persist_step_events(),
            show_progress_bar: default_demo_show_progress_bar(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoFloors {
    #[serde(default = "default_demo_step_floor_sec")]
    pub step_floor_sec: BTreeMap<String, u32>,
    #[serde(default = "default_demo_generated_floor")]
    pub generated_floor: BTreeMap<String, u64>,
    #[serde(default = "default_demo_exact_screened_floor")]
    pub exact_screened_floor: BTreeMap<String, u64>,
}

impl Default for DemoFloors {
    fn default() -> Self {
        Self {
            step_floor_sec: default_demo_step_floor_sec(),
            generated_floor: default_demo_generated_floor(),
            exact_screened_floor: default_demo_exact_screened_floor(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoCaps {
    #[serde(default = "default_demo_full_eval_soft_cap")]
    pub full_eval_soft_cap: BTreeMap<String, u64>,
}

impl Default for DemoCaps {
    fn default() -> Self {
        Self {
            full_eval_soft_cap: default_demo_full_eval_soft_cap(),
        }
    }
}

fn default_demo_profile() -> String {
    "10m".to_owned()
}

const fn default_demo_total_budget_sec() -> u32 {
    600
}

const fn default_demo_early_exhaustive_budget_sec() -> u32 {
    90
}

const fn default_demo_adaptive_reserve_sec() -> u32 {
    120
}

fn default_demo_proof_close_reserve_fraction() -> String {
    "0.30".to_owned()
}

fn default_demo_scout_fraction() -> String {
    "0.10".to_owned()
}

const fn default_demo_live_console() -> bool {
    true
}

const fn default_demo_min_lines_per_step() -> u16 {
    30
}

const fn default_demo_max_lines_per_step() -> u16 {
    200
}

const fn default_demo_pulse_interval_millis() -> u32 {
    500
}

const fn default_demo_persist_step_events() -> bool {
    true
}

const fn default_demo_show_progress_bar() -> bool {
    true
}

fn default_demo_step_floor_sec() -> BTreeMap<String, u32> {
    [
        ("5".to_owned(), 12),
        ("6".to_owned(), 14),
        ("7".to_owned(), 18),
        ("8".to_owned(), 24),
        ("9".to_owned(), 32),
        ("10".to_owned(), 15),
        ("11".to_owned(), 25),
        ("12".to_owned(), 35),
        ("13".to_owned(), 55),
        ("14".to_owned(), 70),
        ("15".to_owned(), 90),
    ]
    .into_iter()
    .collect()
}

fn default_demo_generated_floor() -> BTreeMap<String, u64> {
    [
        ("10".to_owned(), 500),
        ("11".to_owned(), 800),
        ("12".to_owned(), 1200),
        ("13".to_owned(), 2200),
        ("14".to_owned(), 3500),
        ("15".to_owned(), 5000),
    ]
    .into_iter()
    .collect()
}

fn default_demo_exact_screened_floor() -> BTreeMap<String, u64> {
    [
        ("10".to_owned(), 120),
        ("11".to_owned(), 220),
        ("12".to_owned(), 400),
        ("13".to_owned(), 700),
        ("14".to_owned(), 1100),
        ("15".to_owned(), 1800),
    ]
    .into_iter()
    .collect()
}

fn default_demo_full_eval_soft_cap() -> BTreeMap<String, u64> {
    [
        ("10".to_owned(), 25),
        ("11".to_owned(), 35),
        ("12".to_owned(), 50),
        ("13".to_owned(), 70),
        ("14".to_owned(), 90),
        ("15".to_owned(), 120),
    ]
    .into_iter()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::{DemoConfig, RuntimeConfig, SearchProfile, WorkerSetting};
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
        assert_eq!(
            config.mode.search_profile,
            SearchProfile::StrictCanonGuarded
        );
    }

    #[test]
    fn shadow_profiles_parse_with_expected_labels() {
        assert_eq!(
            load_config("strict_canon_guarded.toml").mode.search_profile,
            SearchProfile::StrictCanonGuarded
        );
        assert_eq!(
            load_config("relaxed_shadow.toml").mode.search_profile,
            SearchProfile::RelaxedShadow
        );
        assert_eq!(
            load_config("realistic_frontier_shadow.toml")
                .mode
                .search_profile,
            SearchProfile::RealisticFrontierShadow
        );
        assert_eq!(
            load_config("demo_breadth_shadow_5m.toml")
                .mode
                .search_profile,
            SearchProfile::DemoBreadthShadow
        );
        assert_eq!(
            load_config("demo_breadth_shadow_10m.toml")
                .mode
                .search_profile,
            SearchProfile::DemoBreadthShadow
        );
        assert_eq!(
            load_config("demo_breadth_shadow_15m.toml")
                .mode
                .search_profile,
            SearchProfile::DemoBreadthShadow
        );
        assert_eq!(
            load_config("desktop_claim_shadow_smoke.toml")
                .mode
                .search_profile,
            SearchProfile::DesktopClaimShadow
        );
        assert_eq!(
            load_config("desktop_claim_shadow_1h.toml")
                .mode
                .search_profile,
            SearchProfile::DesktopClaimShadow
        );
        assert_eq!(
            load_config("desktop_claim_shadow_10h.toml")
                .mode
                .search_profile,
            SearchProfile::DesktopClaimShadow
        );
    }

    #[test]
    fn missing_search_profile_defaults_to_strict_canon_guarded() {
        let source = fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .ancestors()
                .nth(2)
                .expect("workspace root")
                .join("configs")
                .join("debug.toml"),
        )
        .expect("config file should exist")
        .replace("search_profile = \"strict_canon_guarded\"\n", "");
        let config = RuntimeConfig::from_toml_str(&source).expect("config should parse");
        assert_eq!(
            config.mode.search_profile,
            SearchProfile::StrictCanonGuarded
        );
    }

    #[test]
    fn missing_demo_section_defaults_cleanly() {
        let config = load_config("strict_canon_guarded.toml");
        assert_eq!(config.demo, DemoConfig::default());
    }

    #[test]
    fn demo_profiles_parse_with_expected_budget_metadata() {
        let five = load_config("demo_breadth_shadow_5m.toml");
        assert_eq!(five.demo.profile, "5m");
        assert_eq!(five.demo.total_budget_sec, 300);
        assert_eq!(five.demo.early_exhaustive_budget_sec, 90);
        assert_eq!(five.demo.adaptive_reserve_sec, 30);
        assert_eq!(five.demo.floors.generated_floor.get("15"), Some(&3500));

        let ten = load_config("demo_breadth_shadow_10m.toml");
        assert_eq!(ten.demo.profile, "10m");
        assert_eq!(ten.demo.total_budget_sec, 600);
        assert_eq!(ten.demo.early_exhaustive_budget_sec, 90);
        assert_eq!(ten.demo.adaptive_reserve_sec, 120);
        assert_eq!(ten.demo.floors.generated_floor.get("15"), Some(&5000));

        let fifteen = load_config("demo_breadth_shadow_15m.toml");
        assert_eq!(fifteen.demo.profile, "15m");
        assert_eq!(fifteen.demo.total_budget_sec, 900);
        assert_eq!(fifteen.demo.early_exhaustive_budget_sec, 90);
        assert_eq!(fifteen.demo.adaptive_reserve_sec, 340);
        assert_eq!(fifteen.demo.floors.generated_floor.get("15"), Some(&6500));
    }

    #[test]
    fn claim_profiles_parse_with_expected_budget_metadata() {
        let smoke = load_config("desktop_claim_shadow_smoke.toml");
        assert_eq!(smoke.search.until_step, 6);
        assert_eq!(smoke.demo.profile, "claim-smoke");
        assert_eq!(smoke.demo.total_budget_sec, 120);
        assert_eq!(smoke.demo.early_exhaustive_budget_sec, 30);
        assert_eq!(smoke.demo.adaptive_reserve_sec, 30);
        assert!(smoke.mode.search_profile.supports_narrative_artifacts());

        let one_hour = load_config("desktop_claim_shadow_1h.toml");
        assert_eq!(one_hour.search.until_step, 15);
        assert_eq!(one_hour.demo.profile, "claim-1h");
        assert_eq!(one_hour.demo.total_budget_sec, 3600);
        assert_eq!(one_hour.demo.early_exhaustive_budget_sec, 90);
        assert_eq!(one_hour.demo.adaptive_reserve_sec, 900);
        assert_eq!(one_hour.demo.floors.generated_floor.get("15"), Some(&5000));

        let ten_hour = load_config("desktop_claim_shadow_10h.toml");
        assert_eq!(ten_hour.search.until_step, 15);
        assert_eq!(ten_hour.demo.profile, "claim-10h");
        assert_eq!(ten_hour.demo.total_budget_sec, 36_000);
        assert_eq!(ten_hour.demo.early_exhaustive_budget_sec, 90);
        assert_eq!(ten_hour.demo.adaptive_reserve_sec, 5_400);
        assert_eq!(ten_hour.demo.floors.generated_floor.get("15"), Some(&5000));
    }
}
