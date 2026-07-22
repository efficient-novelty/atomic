//! G-8 total raw-candidate classifier over the frozen Step-16 surface.
//!
//! Totality is a disjoint-sum statement: a candidate is rejected by a named
//! surface/typing exclusion, classified into an adopted Schema2 class, or
//! retained as the explicit F-G4 typed obstruction.  The last branch is not
//! silently dropped and does not license a class-exhaustion claim.

use crate::grammar_completion::{GrammarCompletionCertificate, replay_grammar_completion_json};
use pen_core::expr::Expr;
use pen_core::stats::StructuralStats;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const TOTAL_CLASSIFIER_SCHEMA: &str = "schema2-g8-total-raw-candidate-classifier-v1";
pub const TOTAL_CLASSIFIER_DATE: &str = "2026-07-20";
pub const F_G4_TYPED_UNKNOWN: &str = "F_G4_TYPED_CANDIDATE_OUTSIDE_EVERY_ADOPTED_SCHEMA2_CLASS";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassifiedSchema2Class {
    Foundation,
    Former,
    Map,
    Axiomatic,
    Modal,
    HitV2,
    Synthesis,
}

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_grammar_completion_plan.md");
const SPEC_BYTES: &[u8] = include_bytes!("../../../docs/step_15_completion_open_problem.md");
const CAPS_BYTES: &[u8] = include_bytes!("../../../docs/SEMANTIC_NORMALIZATION_PROGRAM.md");
const TRACE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const EXPR_BYTES: &[u8] = include_bytes!("../../pen-core/src/expr.rs");
const ELABORATOR_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("total_classifier.rs");
const AGDA_BYTES: &[u8] = include_bytes!("../../../agda/Schema2GrammarCompletion.agda");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenClassifierCaps {
    pub min_kappa: u16,
    pub max_kappa: u16,
    pub max_direct_support: u16,
    pub max_path_dimension: u32,
    pub max_expr_nodes: u16,
    pub allowed_imports: BTreeSet<u32>,
    pub allow_truncation: bool,
    pub allow_modal: bool,
    pub allow_temporal: bool,
    pub allow_linear_exponential: bool,
}

impl FrozenClassifierCaps {
    pub fn genesis_step16() -> Self {
        Self {
            min_kappa: 2,
            max_kappa: 4,
            max_direct_support: 2,
            max_path_dimension: 1,
            max_expr_nodes: 6,
            allowed_imports: [14, 15].into_iter().collect(),
            allow_truncation: false,
            allow_modal: true,
            allow_temporal: true,
            allow_linear_exponential: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClassifierSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RawCandidateDecision {
    NamedExclusion {
        code: String,
        detail: String,
        candidate_digest: String,
        derivation_hash: String,
    },
    Classified {
        telescope_class: TelescopeClass,
        schema2_class: ClassifiedSchema2Class,
        elaboration_derivation_hash: String,
        candidate_digest: String,
        derivation_hash: String,
    },
    NamedTypedObstruction {
        code: String,
        telescope_class: TelescopeClass,
        elaboration_derivation_hash: String,
        candidate_digest: String,
        derivation_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KernelClassBranch {
    pub telescope_class: TelescopeClass,
    pub schema2_class: Option<ClassifiedSchema2Class>,
    pub named_obstruction: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TotalClassifierForbiddenOutputs {
    pub global_e4_membership_assembly_executed: bool,
    pub pending_membership_verdicts_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub fq2_score_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl TotalClassifierForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.global_e4_membership_assembly_executed
            && !self.pending_membership_verdicts_issued
            && !self.e2b_executed
            && !self.stage_count_issued
            && !self.fq2_score_evaluated
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TotalClassifierCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<ClassifierSourceBinding>,
    pub prerequisite_completion_digest: String,
    pub prerequisite_g2_g7_replayed: bool,
    pub frozen_caps: FrozenClassifierCaps,
    pub surface_exclusion_codes: Vec<String>,
    pub exhaustive_expr_head_inventory: Vec<String>,
    pub kernel_class_branches: Vec<KernelClassBranch>,
    pub every_expr_constructor_covered: bool,
    pub every_telescope_class_branch_covered: bool,
    pub elaboration_success_and_failure_both_named: bool,
    pub total_disjoint_sum_classifier_proved: bool,
    pub unknown_rows_dropped_or_averaged: bool,
    pub typed_unknown_retained_as_named_f_g4_obstruction: bool,
    pub class_exhaustion_claimed_over_typed_unknown: bool,
    pub global_e4_assembly_now_authorized: bool,
    pub forbidden_outputs: TotalClassifierForbiddenOutputs,
    pub outcome: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TotalClassifierReplay {
    pub valid: bool,
    pub expression_head_count: usize,
    pub kernel_class_branch_count: usize,
    pub total_classifier_proved: bool,
    pub typed_unknown_retained: bool,
    pub global_e4_assembly_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TotalClassifierError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(TOTAL_CLASSIFIER_SCHEMA, domain, value))
        .expect("total classifier evidence serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn candidate_digest(candidate: &Telescope) -> String {
    tagged_hash("raw-candidate", candidate)
}

fn source_bindings(completion_bytes: &[u8]) -> Vec<ClassifierSourceBinding> {
    [
        (
            "docs/agent_e_grammar_completion_plan.md",
            "g8_phase_order",
            PLAN_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "frozen_classifier_spec",
            SPEC_BYTES,
        ),
        (
            "docs/SEMANTIC_NORMALIZATION_PROGRAM.md",
            "frozen_surface_caps",
            CAPS_BYTES,
        ),
        (
            "docs/schema2_grammar_completion_v1.json",
            "g2_g7_predecessor",
            completion_bytes,
        ),
        (
            "crates/pen-core/src/telescope.rs",
            "exhaustive_telescope_class_match",
            TRACE_BYTES,
        ),
        (
            "crates/pen-core/src/expr.rs",
            "exhaustive_expression_head_match",
            EXPR_BYTES,
        ),
        (
            "crates/pen-type/src/elaborate.rs",
            "typed_success_or_named_failure",
            ELABORATOR_BYTES,
        ),
        (
            "crates/pen-schema/src/total_classifier.rs",
            "total_classifier_implementation",
            THIS_SOURCE_BYTES,
        ),
        (
            "agda/Schema2GrammarCompletion.agda",
            "safe_totality_mirror",
            AGDA_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| ClassifierSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn head_name(expr: &Expr) -> &'static str {
    match expr {
        Expr::App(_, _) => "app",
        Expr::Lam(_) => "lam",
        Expr::Pi(_, _) => "pi",
        Expr::Sigma(_, _) => "sigma",
        Expr::Univ => "univ",
        Expr::Var(_) => "var",
        Expr::Lib(_) => "lib",
        Expr::Id(_, _, _) => "id",
        Expr::Refl(_) => "refl",
        Expr::Susp(_) => "susp",
        Expr::Trunc(_) => "trunc",
        Expr::PathCon(_) => "path_con",
        Expr::Flat(_) => "flat",
        Expr::Sharp(_) => "sharp",
        Expr::Disc(_) => "disc",
        Expr::Shape(_) => "shape",
        Expr::Next(_) => "next",
        Expr::Eventually(_) => "eventually",
        Expr::Bang(_) => "bang",
        Expr::WhyNot(_) => "why_not",
    }
}

fn expr_head_inventory() -> Vec<String> {
    let u = || Box::new(Expr::Univ);
    vec![
        Expr::App(u(), u()),
        Expr::Lam(u()),
        Expr::Pi(u(), u()),
        Expr::Sigma(u(), u()),
        Expr::Univ,
        Expr::Var(1),
        Expr::Lib(15),
        Expr::Id(u(), u(), u()),
        Expr::Refl(u()),
        Expr::Susp(u()),
        Expr::Trunc(u()),
        Expr::PathCon(1),
        Expr::Flat(u()),
        Expr::Sharp(u()),
        Expr::Disc(u()),
        Expr::Shape(u()),
        Expr::Next(u()),
        Expr::Eventually(u()),
        Expr::Bang(u()),
        Expr::WhyNot(u()),
    ]
    .iter()
    .map(head_name)
    .map(str::to_owned)
    .collect()
}

fn expr_surface_exclusion(
    expr: &Expr,
    caps: &FrozenClassifierCaps,
) -> Option<(&'static str, String)> {
    let nodes = StructuralStats::from_expr(expr).node_count;
    if nodes > u32::from(caps.max_expr_nodes) {
        return Some((
            "G8_EXCLUDE_EXPR_NODE_CAP",
            format!(
                "expression has {nodes} nodes; cap is {}",
                caps.max_expr_nodes
            ),
        ));
    }
    match expr {
        Expr::PathCon(dimension) if *dimension == 0 || *dimension > caps.max_path_dimension => {
            Some((
                "G8_EXCLUDE_PATH_DIMENSION",
                format!(
                    "PathCon({dimension}) is outside 1..={}",
                    caps.max_path_dimension
                ),
            ))
        }
        Expr::Trunc(_) if !caps.allow_truncation => Some((
            "G8_EXCLUDE_TRUNCATION_DISABLED",
            "truncation is outside the frozen Step-16 surface".to_owned(),
        )),
        Expr::Flat(_) | Expr::Sharp(_) | Expr::Disc(_) | Expr::Shape(_) if !caps.allow_modal => {
            Some((
                "G8_EXCLUDE_MODAL_DISABLED",
                "modal syntax is disabled".to_owned(),
            ))
        }
        Expr::Next(_) | Expr::Eventually(_) if !caps.allow_temporal => Some((
            "G8_EXCLUDE_TEMPORAL_DISABLED",
            "temporal syntax is disabled".to_owned(),
        )),
        Expr::Bang(_) | Expr::WhyNot(_) if !caps.allow_linear_exponential => Some((
            "G8_EXCLUDE_LINEAR_EXPONENTIAL_DISABLED",
            "linear-exponential syntax is outside the frozen surface".to_owned(),
        )),
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            expr_surface_exclusion(left, caps).or_else(|| expr_surface_exclusion(right, caps))
        }
        Expr::Id(carrier, left, right) => expr_surface_exclusion(carrier, caps)
            .or_else(|| expr_surface_exclusion(left, caps))
            .or_else(|| expr_surface_exclusion(right, caps)),
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => expr_surface_exclusion(body, caps),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => None,
    }
}

fn surface_exclusion(
    candidate: &Telescope,
    caps: &FrozenClassifierCaps,
) -> Option<(String, String)> {
    let kappa = candidate.kappa();
    if kappa < usize::from(caps.min_kappa) || kappa > usize::from(caps.max_kappa) {
        return Some((
            "G8_EXCLUDE_KAPPA_CAP".to_owned(),
            format!(
                "candidate kappa {kappa} is outside {}..={}",
                caps.min_kappa, caps.max_kappa
            ),
        ));
    }
    let support = candidate.lib_refs();
    if support.len() > usize::from(caps.max_direct_support) {
        return Some((
            "G8_EXCLUDE_DIRECT_SUPPORT_CAP".to_owned(),
            format!(
                "direct support {} exceeds {}",
                support.len(),
                caps.max_direct_support
            ),
        ));
    }
    if !support.is_subset(&caps.allowed_imports) {
        return Some((
            "G8_EXCLUDE_IMPORT_WINDOW".to_owned(),
            format!(
                "support {support:?} is not a subset of {:?}",
                caps.allowed_imports
            ),
        ));
    }
    for (index, clause) in candidate.clauses.iter().enumerate() {
        if let Some((code, detail)) = expr_surface_exclusion(&clause.expr, caps) {
            return Some((code.to_owned(), format!("clause {index}: {detail}")));
        }
    }
    None
}

fn map_telescope_class(class: TelescopeClass) -> Option<ClassifiedSchema2Class> {
    match class {
        TelescopeClass::Foundation => Some(ClassifiedSchema2Class::Foundation),
        TelescopeClass::Former => Some(ClassifiedSchema2Class::Former),
        TelescopeClass::Hit | TelescopeClass::Suspension => Some(ClassifiedSchema2Class::HitV2),
        TelescopeClass::Map => Some(ClassifiedSchema2Class::Map),
        TelescopeClass::Modal => Some(ClassifiedSchema2Class::Modal),
        TelescopeClass::Axiomatic => Some(ClassifiedSchema2Class::Axiomatic),
        TelescopeClass::Synthesis => Some(ClassifiedSchema2Class::Synthesis),
        TelescopeClass::Unknown => None,
    }
}

fn kernel_class_branches() -> Vec<KernelClassBranch> {
    [
        TelescopeClass::Foundation,
        TelescopeClass::Former,
        TelescopeClass::Hit,
        TelescopeClass::Suspension,
        TelescopeClass::Map,
        TelescopeClass::Modal,
        TelescopeClass::Axiomatic,
        TelescopeClass::Synthesis,
        TelescopeClass::Unknown,
    ]
    .into_iter()
    .map(|telescope_class| KernelClassBranch {
        schema2_class: map_telescope_class(telescope_class),
        named_obstruction: (telescope_class == TelescopeClass::Unknown)
            .then(|| F_G4_TYPED_UNKNOWN.to_owned()),
        telescope_class,
    })
    .collect()
}

/// Total executable classifier over arbitrary input.  Inputs outside the
/// frozen surface are named exclusions rather than preconditions that can be
/// forgotten by a caller.
pub fn classify_raw_candidate(candidate: &Telescope) -> RawCandidateDecision {
    let caps = FrozenClassifierCaps::genesis_step16();
    let candidate_digest = candidate_digest(candidate);
    if let Some((code, detail)) = surface_exclusion(candidate, &caps) {
        let derivation_hash =
            tagged_hash("surface-exclusion", &(&code, &detail, &candidate_digest));
        return RawCandidateDecision::NamedExclusion {
            code,
            detail,
            candidate_digest,
            derivation_hash,
        };
    }

    let signature = SealedSignature::genesis_del_h15();
    let elaboration = match elaborate_telescope(&signature, candidate, 15) {
        Ok(elaboration) => elaboration,
        Err(error) => {
            let code = "G8_EXCLUDE_TYPED_ELABORATION_FAILURE".to_owned();
            let detail = error.to_string();
            let derivation_hash = tagged_hash(
                "typed-elaboration-exclusion",
                &(&code, &detail, &candidate_digest),
            );
            return RawCandidateDecision::NamedExclusion {
                code,
                detail,
                candidate_digest,
                derivation_hash,
            };
        }
    };

    let telescope_class = candidate.classify(&Vec::new());
    if let Some(schema2_class) = map_telescope_class(telescope_class) {
        let derivation_hash = tagged_hash(
            "typed-schema2-classification",
            &(
                telescope_class,
                schema2_class,
                &elaboration.derivation_hash,
                &candidate_digest,
            ),
        );
        RawCandidateDecision::Classified {
            telescope_class,
            schema2_class,
            elaboration_derivation_hash: elaboration.derivation_hash,
            candidate_digest,
            derivation_hash,
        }
    } else {
        let code = F_G4_TYPED_UNKNOWN.to_owned();
        let derivation_hash = tagged_hash(
            "typed-f-g4-obstruction",
            &(
                &code,
                telescope_class,
                &elaboration.derivation_hash,
                &candidate_digest,
            ),
        );
        RawCandidateDecision::NamedTypedObstruction {
            code,
            telescope_class,
            elaboration_derivation_hash: elaboration.derivation_hash,
            candidate_digest,
            derivation_hash,
        }
    }
}

fn certificate_digest(certificate: &TotalClassifierCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("total-classifier-certificate", &projection)
}

pub fn issue_total_classifier_certificate()
-> Result<TotalClassifierCertificate, TotalClassifierError> {
    let completion_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/schema2_grammar_completion_v1.json");
    let completion_bytes = std::fs::read(completion_path)
        .map_err(|error| TotalClassifierError::Io(error.to_string()))?;
    let completion_replay = replay_grammar_completion_json(
        std::str::from_utf8(&completion_bytes)
            .map_err(|error| TotalClassifierError::Prerequisite(error.to_string()))?,
    );
    if !completion_replay.valid || !completion_replay.g8_authorized {
        return Err(TotalClassifierError::Prerequisite(format!(
            "G2-G7 replay failed or did not authorize G8: {}",
            completion_replay.errors.join("; ")
        )));
    }
    let completion: GrammarCompletionCertificate = serde_json::from_slice(&completion_bytes)
        .map_err(|error| TotalClassifierError::Json(error.to_string()))?;
    let exhaustive_expr_head_inventory = expr_head_inventory();
    let kernel_class_branches = kernel_class_branches();
    let every_expr_constructor_covered = exhaustive_expr_head_inventory.len() == 20
        && exhaustive_expr_head_inventory
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            == 20;
    let every_telescope_class_branch_covered = kernel_class_branches.len() == 9
        && kernel_class_branches
            .iter()
            .filter(|branch| branch.schema2_class.is_none())
            .all(|branch| {
                branch.telescope_class == TelescopeClass::Unknown
                    && branch.named_obstruction.as_deref() == Some(F_G4_TYPED_UNKNOWN)
            });
    let surface_exclusion_codes = vec![
        "G8_EXCLUDE_KAPPA_CAP",
        "G8_EXCLUDE_DIRECT_SUPPORT_CAP",
        "G8_EXCLUDE_IMPORT_WINDOW",
        "G8_EXCLUDE_EXPR_NODE_CAP",
        "G8_EXCLUDE_PATH_DIMENSION",
        "G8_EXCLUDE_TRUNCATION_DISABLED",
        "G8_EXCLUDE_MODAL_DISABLED",
        "G8_EXCLUDE_TEMPORAL_DISABLED",
        "G8_EXCLUDE_LINEAR_EXPONENTIAL_DISABLED",
        "G8_EXCLUDE_TYPED_ELABORATION_FAILURE",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    let elaboration_success_and_failure_both_named = true;
    let total_disjoint_sum_classifier_proved = every_expr_constructor_covered
        && every_telescope_class_branch_covered
        && elaboration_success_and_failure_both_named;
    let unknown_rows_dropped_or_averaged = false;
    let typed_unknown_retained_as_named_f_g4_obstruction = true;
    let class_exhaustion_claimed_over_typed_unknown = false;
    let global_e4_assembly_now_authorized = total_disjoint_sum_classifier_proved;
    let forbidden_outputs = TotalClassifierForbiddenOutputs {
        global_e4_membership_assembly_executed: false,
        pending_membership_verdicts_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        fq2_score_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if !total_disjoint_sum_classifier_proved
        || unknown_rows_dropped_or_averaged
        || class_exhaustion_claimed_over_typed_unknown
        || !forbidden_outputs.all_withheld()
    {
        return Err(TotalClassifierError::Prerequisite(
            "G8 totality/firewall invariant failed".to_owned(),
        ));
    }
    let mut certificate = TotalClassifierCertificate {
        schema: TOTAL_CLASSIFIER_SCHEMA.to_owned(),
        date: TOTAL_CLASSIFIER_DATE.to_owned(),
        source_bindings: source_bindings(&completion_bytes),
        prerequisite_completion_digest: completion.result_digest,
        prerequisite_g2_g7_replayed: true,
        frozen_caps: FrozenClassifierCaps::genesis_step16(),
        surface_exclusion_codes,
        exhaustive_expr_head_inventory,
        kernel_class_branches,
        every_expr_constructor_covered,
        every_telescope_class_branch_covered,
        elaboration_success_and_failure_both_named,
        total_disjoint_sum_classifier_proved,
        unknown_rows_dropped_or_averaged,
        typed_unknown_retained_as_named_f_g4_obstruction,
        class_exhaustion_claimed_over_typed_unknown,
        global_e4_assembly_now_authorized,
        forbidden_outputs,
        outcome: "g8_total_disjoint_sum_classifier_complete_global_e4_assembly_not_run".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> TotalClassifierReplay {
    TotalClassifierReplay {
        valid: false,
        expression_head_count: 0,
        kernel_class_branch_count: 0,
        total_classifier_proved: false,
        typed_unknown_retained: false,
        global_e4_assembly_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "g8_total_classifier_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_total_classifier_certificate(
    certificate: &TotalClassifierCertificate,
) -> TotalClassifierReplay {
    let expected = match issue_total_classifier_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != &expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    TotalClassifierReplay {
        valid: errors.is_empty(),
        expression_head_count: certificate.exhaustive_expr_head_inventory.len(),
        kernel_class_branch_count: certificate.kernel_class_branches.len(),
        total_classifier_proved: certificate.total_disjoint_sum_classifier_proved,
        typed_unknown_retained: certificate.typed_unknown_retained_as_named_f_g4_obstruction,
        global_e4_assembly_authorized: certificate.global_e4_assembly_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_total_classifier_json(json: &str) -> TotalClassifierReplay {
    match serde_json::from_str::<TotalClassifierCertificate>(json) {
        Ok(certificate) => replay_total_classifier_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_total_classifier_create_new(
    path: &Path,
) -> Result<TotalClassifierReplay, TotalClassifierError> {
    let certificate = issue_total_classifier_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| TotalClassifierError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| TotalClassifierError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| TotalClassifierError::Io(error.to_string()))?;
    let replay = replay_total_classifier_certificate(&certificate);
    if !replay.valid {
        return Err(TotalClassifierError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::{ClauseRec, ClauseRole};

    fn tel(exprs: Vec<Expr>) -> Telescope {
        Telescope::new(
            exprs
                .into_iter()
                .map(|expr| ClauseRec::new(ClauseRole::Formation, expr))
                .collect(),
        )
    }

    #[test]
    fn every_expression_and_telescope_class_branch_is_explicit() {
        let certificate = issue_total_classifier_certificate().expect("G8 issues");
        assert_eq!(certificate.exhaustive_expr_head_inventory.len(), 20);
        assert_eq!(certificate.kernel_class_branches.len(), 9);
        assert!(certificate.total_disjoint_sum_classifier_proved);
        assert!(certificate.typed_unknown_retained_as_named_f_g4_obstruction);
        assert!(!certificate.class_exhaustion_claimed_over_typed_unknown);
    }

    #[test]
    fn caps_and_typed_failures_are_named_not_panics() {
        let too_short = tel(vec![Expr::Univ]);
        assert!(matches!(
            classify_raw_candidate(&too_short),
            RawCandidateDecision::NamedExclusion { ref code, .. }
                if code == "G8_EXCLUDE_KAPPA_CAP"
        ));
        let wrong_import = tel(vec![Expr::Lib(13), Expr::Lib(14)]);
        assert!(matches!(
            classify_raw_candidate(&wrong_import),
            RawCandidateDecision::NamedExclusion { ref code, .. }
                if code == "G8_EXCLUDE_IMPORT_WINDOW"
        ));
        let bad_var = tel(vec![Expr::Var(99), Expr::Var(99)]);
        assert!(matches!(
            classify_raw_candidate(&bad_var),
            RawCandidateDecision::NamedExclusion { ref code, .. }
                if code == "G8_EXCLUDE_TYPED_ELABORATION_FAILURE"
        ));
    }

    #[test]
    fn a_typed_frozen_candidate_enters_a_named_class() {
        let candidate = Telescope::reference(1);
        assert!(matches!(
            classify_raw_candidate(&candidate),
            RawCandidateDecision::Classified {
                telescope_class: TelescopeClass::Foundation,
                schema2_class: ClassifiedSchema2Class::Foundation,
                ..
            }
        ));
    }

    #[test]
    fn certificate_replay_and_mutation_fail_closed() {
        let certificate = issue_total_classifier_certificate().expect("G8 issues");
        assert!(replay_total_classifier_certificate(&certificate).valid);
        let mut mutated = certificate;
        mutated.every_expr_constructor_covered = false;
        assert!(!replay_total_classifier_certificate(&mutated).valid);
    }
}
