use crate::expr::Expr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClauseRole {
    Formation,
    Introduction,
    Elimination,
    PathAttach,
    Computation,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
pub struct ClauseRec {
    pub role: ClauseRole,
    pub expr: Expr,
}

impl ClauseRec {
    pub fn new(role: ClauseRole, expr: Expr) -> Self {
        Self { role, expr }
    }
}
