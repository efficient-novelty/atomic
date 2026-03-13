use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct CapabilityFlags {
    pub constructors: bool,
    pub path_dimensions: Vec<u16>,
    pub has_loop: bool,
    pub is_truncated: bool,
    pub axiomatic_exports: usize,
    pub has_dependent_functions: bool,
    pub has_modal_ops: bool,
    pub has_differential_ops: bool,
    pub has_curvature: bool,
    pub has_metric: bool,
    pub has_hilbert: bool,
    pub has_temporal_ops: bool,
}
