use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AgdaExportManifest {
    pub schema_version: u32,
    pub run_id: String,
    pub generated_utc: String,
    pub output_dir: String,
    pub verify_requested: bool,
    pub steps: Vec<AgdaExportedStep>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AgdaExportedStep {
    pub step_index: u32,
    pub label: String,
    pub module_name: String,
    pub source_file: String,
    pub verify_log_file: String,
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub verification: VerificationStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    Skipped,
    Passed,
    Failed,
}
