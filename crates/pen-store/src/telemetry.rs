use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct TelemetryEventV1 {
    pub schema_version: u32,
    pub run_id: String,
    pub event: String,
    pub step_index: Option<u32>,
    pub payload: Value,
}
