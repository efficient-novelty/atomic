use pen_core::rational::Rational;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AgdaExportManifest {
    pub schema_version: u32,
    pub run_id: String,
    pub generated_utc: String,
    pub output_dir: String,
    pub verify_requested: bool,
    pub source: ExportSource,
    pub steps: Vec<AgdaExportedStep>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportSource {
    #[default]
    AcceptedRunArtifacts,
    ReferenceReplayFallback,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AgdaExportedStep {
    pub step_index: u32,
    pub label: String,
    pub module_name: String,
    pub source_file: String,
    pub payload_module_name: String,
    pub payload_file: String,
    pub verify_log_file: String,
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub claims: ProofClaims,
    pub verification: VerificationStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProofClaims {
    pub canonical_key: String,
    pub bit_kappa: u16,
    pub clause_kappa: u16,
    pub desugared_kappa: u16,
    pub rho: Rational,
    pub import_steps: Vec<u32>,
    pub nu_claim: NuClaim,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NuClaim {
    pub nu_g: u32,
    pub nu_h: u32,
    pub nu_c: u32,
    pub nu_total: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    Skipped,
    Passed,
    Failed,
}
