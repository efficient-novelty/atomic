//! T-D2-2 theorem attempt: lawful dependency stop.
//!
//! This artifact does not decide generator surjectivity.  It binds the
//! independent-domain adjudication and the T-D2-1 domain-build artifact,
//! records why the latter is not yet an operative finite carrier, and fixes
//! the noncircular least-fixed-point procedure a future theorem attempt must
//! execute.  In particular, no generator inventory participates in a domain
//! membership decision here.

use crate::t_d2_1_domain_build_v1::{
    T_D2_1_DOMAIN_BUILD_V1_SCHEMA, TD21_FINITE_ENUMERATION_GAP, TD21_FIXED_FRAGMENT_GAP,
    TD21_MEMBERSHIP_GAP, TD21_NORMALIZATION_GAP, TD21_QUOTIENT_GAP, TD21_REGRESSION_BLOCKED_GAP,
    Td21DomainBuildV1Certificate, render_t_d2_1_domain_build_v1, replay_t_d2_1_domain_build_v1,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::telescope::Telescope;
use pen_eval::typed_families::clause_presentation;
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use pen_type::equality::univalent_equality;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{OpenOptions, read, read_to_string, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const T_D2_2_COMPLETENESS_ATTEMPT_V1_SCHEMA: &str = "t-d2-2-completeness-attempt-v1";
pub const T_D2_2_COMPLETENESS_ATTEMPT_V1_DATE: &str = "2026-07-23";
pub const T_D2_2_CERTIFICATE_NAME: &str = "t_d2_2_completeness_attempt_v1.json";
pub const T_D2_2_REPORT_NAME: &str = "T_D2_2_COMPLETENESS_ATTEMPT_V1_RESULT.md";

pub const EXPECTED_T_D2_1_SOURCE_NAME: &str = "crates/pen-search/src/t_d2_1_domain_build_v1.rs";
pub const EXPECTED_T_D2_1_CERTIFICATE_NAME: &str = "docs/t_d2_1_domain_build_v1.json";
pub const EXPECTED_T_D2_1_REPORT_NAME: &str = "docs/T_D2_1_DOMAIN_BUILD_V1_RESULT.md";

pub const TD22_LITERAL_KERNEL_COUNTERFAMILY_OBSTRUCTION: &str =
    "TD22_LITERAL_KERNEL_ACCEPTANCE_HAS_SUSP_N_COUNTERFAMILY";
pub const T_D2_2_DEPENDENCY_GAP: &str = "TD22_TD21_FINITE_INDEPENDENT_DOMAIN_NOT_AVAILABLE";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/depth_two_domain_adjudication.md");
const OPEN_PROBLEM_BYTES: &[u8] =
    include_bytes!("../../../docs/step_15_completion_open_problem.md");
const EXPR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/expr.rs");
const TELESCOPE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const NORMALIZE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const TYPED_FAMILIES_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/typed_families.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_d2_2_completeness_attempt_v1.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td22RunStatus {
    StoppedOnTd21FiniteDomainDependency,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td22TheoremDisposition {
    NotAttemptedDependencyUnsatisfied,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td22DependencyDisposition {
    ExpectedArtifactAbsent,
    PresentButUnauthenticated,
    PresentNamedGap,
    PresentButFiniteDomainContractUnrecognized,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td22Register {
    Law,
    KernelTestimony,
    DependencyTestimony,
    ProofProgram,
    Gate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22TaggedNumber {
    pub decimal: String,
    pub register: Td22Register,
    pub meaning: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: Td22TaggedNumber,
    pub sha256: String,
    pub register: Td22Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22Td21Binding {
    pub expected_source_path: String,
    pub expected_certificate_path: String,
    pub expected_report_path: String,
    pub source_present: bool,
    pub certificate_present: bool,
    pub report_present: bool,
    pub source_sha256: Option<String>,
    pub certificate_sha256: Option<String>,
    pub report_sha256: Option<String>,
    pub typed_certificate_parse_succeeded: bool,
    pub typed_certificate_parse_error: Option<String>,
    pub exact_v1_schema_matched: bool,
    pub typed_certificate_result_digest: Option<String>,
    pub public_replay_valid: bool,
    pub public_replay_errors: Vec<String>,
    pub typed_certificate_authenticated: bool,
    pub typed_source_binding_sha256: Option<String>,
    pub runtime_source_matches_typed_binding: bool,
    pub report_matches_deterministic_render: bool,
    pub dependency_trusted: bool,
    pub typed_open_gap_ids: Vec<String>,
    pub declared_schema: Option<String>,
    pub declared_status: Option<String>,
    pub declared_open_gaps: Vec<String>,
    pub fixed_fragment_operationally_defined: Option<bool>,
    pub membership_decidable: Option<bool>,
    pub quotient_finitely_enumerable: Option<bool>,
    pub declared_td22_prerequisite_satisfied: Option<bool>,
    pub dependency_contract_satisfied: bool,
    pub disposition: Td22DependencyDisposition,
    pub exact_obstruction: String,
    pub binding_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22CounterfamilyRow {
    pub index: Td22TaggedNumber,
    pub defining_equation: String,
    pub expression_sha256: String,
    pub kernel_accepted: bool,
    pub ambient_parameters: Td22TaggedNumber,
    pub kernel_role: ClauseRole,
    pub normal_form_sha256: String,
    pub normal_form_is_expression: bool,
    pub canonical_parameter_count: Td22TaggedNumber,
    pub closed_zero_parameter_family: bool,
    pub row_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22CounterfamilyPair {
    pub left_index: Td22TaggedNumber,
    pub right_index: Td22TaggedNumber,
    pub frozen_univalent_equal: bool,
    pub left_normal_form_sha256: String,
    pub right_normal_form_sha256: String,
    pub distinct: bool,
    pub pair_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22LiteralKernelCounterfamily {
    pub family_definition: String,
    pub successor_rule: String,
    pub quantifier: String,
    pub history_independent: bool,
    pub sample_prefix_size: Td22TaggedNumber,
    pub rows: Vec<Td22CounterfamilyRow>,
    pub pairwise_checks: Vec<Td22CounterfamilyPair>,
    pub every_sample_kernel_accepted: bool,
    pub every_sample_beta_normal: bool,
    pub every_sample_closed_zero_parameter: bool,
    pub every_sample_pair_distinct: bool,
    pub kernel_successor_closure_source_bound: bool,
    pub normalizer_constructor_preservation_source_bound: bool,
    pub structural_constructor_injectivity_source_bound: bool,
    pub frozen_equality_is_normal_form_syntax_source_bound: bool,
    pub closed_family_quotient_has_no_renaming_identification: bool,
    pub mathematical_obstruction: String,
    pub raw_ast_depth_is_schema2_depth: bool,
    pub finite_quotient_conclusion_available: bool,
    pub derivation_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22DomainBoundaryAudit {
    pub adjudication_names_fixed_depth_two_fragment: bool,
    pub adjudication_supplies_inductive_fragment_grammar: bool,
    pub adjudication_supplies_semantic_depth_predicate: bool,
    pub adjudication_supplies_expression_size_bound: bool,
    pub adjudication_supplies_telescope_length_bound: bool,
    pub ambient_parameter_cap: Td22TaggedNumber,
    pub ambient_cap_bounds_recursive_expression_depth: bool,
    pub ambient_cap_bounds_telescope_length: bool,
    pub recursive_expr_carrier_detected: bool,
    pub unbounded_clause_vector_detected: bool,
    pub finite_u32_payloads_imply_finite_recursive_ast_carrier: bool,
    pub raw_ast_cap_permitted_as_schema2_proxy: bool,
    pub literal_kernel_reading_refuted_by_counterfamily: bool,
    pub exact_conclusion: String,
    pub audit_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22FutureProcedureStep {
    pub ordinal: Td22TaggedNumber,
    pub step_id: String,
    pub action: String,
    pub domain_membership_created_by_generator: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22FutureLeastFixedPointProcedure {
    pub procedure_id: String,
    pub required_td21_exports: Vec<String>,
    pub steps: Vec<Td22FutureProcedureStep>,
    pub domain_carrier_sealed_before_generator_reachability: bool,
    pub generator_rules_only_mark_existing_domain_nodes: bool,
    pub generator_rules_may_extend_domain_carrier: bool,
    pub equality_uses_structural_witness_not_hash_alone: bool,
    pub domain_subset_reach_is_surjectivity_obligation: bool,
    pub reach_subset_domain_is_soundness_obligation: bool,
    pub missing_domain_node_is_published_refutation: bool,
    pub out_of_domain_generator_conclusion_is_silent_drop: bool,
    pub arbitrary_derivation_depth_cap_used: bool,
    pub intermediate_closure_or_cut_elimination_required: bool,
    pub depth_one_restriction_recomputed_from_independent_tag: bool,
    pub procedure_executed: bool,
    pub procedure_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22ZeroCharge {
    pub kappa_minted: Td22TaggedNumber,
    pub nu_minted: Td22TaggedNumber,
    pub anchors_minted: Td22TaggedNumber,
    pub zero_charge_holds: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22Gate {
    pub td21_domain_build_required: bool,
    pub td21_domain_build_satisfied: bool,
    pub td22_surjectivity_verdict_issued: bool,
    pub td22_refutation_verdict_issued: bool,
    pub td23_or_bc3_authorized_by_this_artifact: bool,
    pub m3_or_m4_gate_moved: bool,
    pub exact_gate_statement: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td22CompletenessAttemptV1Certificate {
    pub schema: String,
    pub date: String,
    pub status: Td22RunStatus,
    pub theorem_disposition: Td22TheoremDisposition,
    pub adjudication_sha256: String,
    pub source_bindings: Vec<Td22SourceBinding>,
    pub td21_binding: Td22Td21Binding,
    pub boundary_audit: Td22DomainBoundaryAudit,
    pub literal_kernel_counterfamily: Td22LiteralKernelCounterfamily,
    pub future_least_fixed_point_procedure: Td22FutureLeastFixedPointProcedure,
    pub domain_membership_decision_executed: bool,
    pub generator_inventory_imported_into_domain_membership: bool,
    pub a3_inventory_imported_into_domain_membership: bool,
    pub historical_61_or_89_rows_used_to_seed_or_bound_domain: bool,
    pub surjectivity_claimed: bool,
    pub incompleteness_claimed: bool,
    pub exact_result: String,
    pub open_gaps: Vec<String>,
    pub zero_charge: Td22ZeroCharge,
    pub gate: Td22Gate,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Td22Replay {
    pub valid: bool,
    pub status: Option<Td22RunStatus>,
    pub theorem_disposition: Option<Td22TheoremDisposition>,
    pub td21_dependency_satisfied: bool,
    pub surjectivity_verdict_issued: bool,
    pub future_procedure_executed: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Error)]
pub enum Td22Error {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("kernel counterfamily construction failed: {0}")]
    Kernel(String),
    #[error("T-D2-1 is operative; the dependency-stop issuer must not run")]
    DependencyAlreadySatisfied,
    #[error("T-D2-2 invariant failed: {0}")]
    Invariant(String),
}

fn tagged(value: usize, register: Td22Register, meaning: &str) -> Td22TaggedNumber {
    Td22TaggedNumber {
        decimal: value.to_string(),
        register,
        meaning: meaning.to_owned(),
    }
}

fn sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("sha256:{digest:x}")
}

fn json_digest<T: Serialize>(domain: &str, value: &T) -> String {
    let payload = serde_json::to_vec(&(domain, value)).expect("digest payload serializes");
    sha256(&payload)
}

fn source_binding(
    path: &str,
    role: &str,
    bytes: &[u8],
    register: Td22Register,
) -> Td22SourceBinding {
    Td22SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: tagged(bytes.len(), register, "exact source byte length"),
        sha256: sha256(bytes),
        register,
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn expected_td21_paths() -> (PathBuf, PathBuf, PathBuf) {
    let root = workspace_root();
    (
        root.join(EXPECTED_T_D2_1_SOURCE_NAME),
        root.join(EXPECTED_T_D2_1_CERTIFICATE_NAME),
        root.join(EXPECTED_T_D2_1_REPORT_NAME),
    )
}

fn scalar_string(value: Option<&Value>) -> Option<String> {
    match value {
        Some(Value::String(value)) => Some(value.clone()),
        Some(other) if !other.is_null() => Some(other.to_string()),
        _ => None,
    }
}

fn find_bool_recursive(value: &Value, keys: &[&str]) -> Option<bool> {
    match value {
        Value::Object(map) => {
            for key in keys {
                if let Some(result) = map.get(*key).and_then(Value::as_bool) {
                    return Some(result);
                }
            }
            map.values()
                .find_map(|child| find_bool_recursive(child, keys))
        }
        Value::Array(values) => values
            .iter()
            .find_map(|child| find_bool_recursive(child, keys)),
        _ => None,
    }
}

fn collect_gap_strings(value: &Value, output: &mut BTreeSet<String>) {
    match value {
        Value::String(text) => {
            if text.contains("GAP")
                || text.contains("NOT_OPERATIONALLY_DEFINED")
                || text.contains("QUOTIENT_INFINITE")
            {
                output.insert(text.clone());
            }
        }
        Value::Array(values) => {
            for child in values {
                collect_gap_strings(child, output);
            }
        }
        Value::Object(map) => {
            if (map.contains_key("exact_obstruction") || map.contains_key("keeps_t_d2_2_closed"))
                && let Some(Value::String(id)) = map.get("id")
            {
                output.insert(id.clone());
            }
            for (key, child) in map {
                if key.contains("gap") || key.contains("obstruction") {
                    collect_gap_strings(child, output);
                } else if child.is_array() || child.is_object() {
                    collect_gap_strings(child, output);
                }
            }
        }
        _ => {}
    }
}

fn optional_file(path: &Path) -> Result<Option<Vec<u8>>, Td22Error> {
    match read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(Td22Error::Io(format!("{}: {error}", path.display()))),
    }
}

fn td21_binding_from_paths(
    source_path: &Path,
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Td22Td21Binding, Td22Error> {
    let source = optional_file(source_path)?;
    let certificate = optional_file(certificate_path)?;
    let report = optional_file(report_path)?;

    let parsed = certificate
        .as_ref()
        .and_then(|bytes| serde_json::from_slice::<Value>(bytes).ok());
    let (typed_certificate, typed_certificate_parse_error) = match certificate.as_ref() {
        Some(bytes) => match serde_json::from_slice::<Td21DomainBuildV1Certificate>(bytes) {
            Ok(certificate) => (Some(certificate), None),
            Err(error) => (None, Some(error.to_string())),
        },
        None => (None, None),
    };
    let typed_certificate_parse_succeeded = typed_certificate.is_some();
    let exact_v1_schema_matched = typed_certificate
        .as_ref()
        .is_some_and(|certificate| certificate.schema == T_D2_1_DOMAIN_BUILD_V1_SCHEMA);
    let typed_certificate_result_digest = typed_certificate
        .as_ref()
        .map(|certificate| certificate.result_digest.clone());
    let typed_replay = typed_certificate
        .as_ref()
        .map(replay_t_d2_1_domain_build_v1);
    let public_replay_valid = typed_replay.as_ref().is_some_and(|replay| replay.valid);
    let public_replay_errors = typed_replay
        .as_ref()
        .map(|replay| replay.errors.clone())
        .unwrap_or_default();
    let typed_certificate_authenticated =
        typed_certificate_parse_succeeded && exact_v1_schema_matched && public_replay_valid;
    let typed_source_binding_sha256 = typed_certificate.as_ref().and_then(|certificate| {
        certificate
            .source_bindings
            .iter()
            .find(|binding| binding.path == EXPECTED_T_D2_1_SOURCE_NAME)
            .map(|binding| format!("sha256:{}", binding.sha256))
    });
    let runtime_source_sha256 = source.as_deref().map(sha256);
    let runtime_source_matches_typed_binding = runtime_source_sha256
        .as_ref()
        .zip(typed_source_binding_sha256.as_ref())
        .is_some_and(|(runtime, typed)| runtime == typed);
    let report_matches_deterministic_render = typed_certificate
        .as_ref()
        .zip(report.as_ref())
        .is_some_and(|(certificate, report)| {
            render_t_d2_1_domain_build_v1(certificate).as_bytes() == report.as_slice()
        });
    let dependency_trusted = source.is_some()
        && report.is_some()
        && certificate.is_some()
        && typed_certificate_authenticated
        && runtime_source_matches_typed_binding
        && report_matches_deterministic_render;
    let typed_open_gap_ids = if typed_certificate_authenticated {
        typed_certificate
            .as_ref()
            .map(|certificate| {
                certificate
                    .open_gaps
                    .iter()
                    .map(|gap| gap.id.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let declared_schema = parsed
        .as_ref()
        .and_then(|value| scalar_string(value.get("schema")));
    let declared_status = parsed
        .as_ref()
        .and_then(|value| scalar_string(value.get("status")));
    let mut gaps = BTreeSet::new();
    if let Some(value) = &parsed {
        collect_gap_strings(value, &mut gaps);
    }
    let declared_open_gaps = gaps.into_iter().collect::<Vec<_>>();
    let fixed_fragment_operationally_defined = parsed.as_ref().and_then(|value| {
        find_bool_recursive(
            value,
            &[
                "fixed_fragment_operationally_defined",
                "domain_operationally_defined",
                "independent_fragment_defined",
            ],
        )
    });
    let membership_decidable = parsed.as_ref().and_then(|value| {
        find_bool_recursive(
            value,
            &[
                "membership_decidable",
                "decidable_membership_proved",
                "domain_membership_decidable",
            ],
        )
    });
    let quotient_finitely_enumerable = parsed.as_ref().and_then(|value| {
        find_bool_recursive(
            value,
            &[
                "quotient_finitely_enumerable",
                "finite_enumerability_proved",
                "finite_quotient_proved",
                "domain_finite",
            ],
        )
    });
    let declared_td22_prerequisite_satisfied = parsed
        .as_ref()
        .and_then(|value| find_bool_recursive(value, &["t_d2_2_prerequisite_satisfied"]));
    // The recursive field reads above are diagnostic testimony only. The
    // dependency contract is decided exclusively from the typed, exact-v1,
    // publicly replayed certificate.
    let dependency_contract_satisfied = dependency_trusted
        && typed_certificate.as_ref().is_some_and(|certificate| {
            certificate.fixed_fragment_operationally_defined
                && certificate.membership_decidable
                && certificate.quotient_finitely_enumerable
                && certificate.t_d2_2_prerequisite_satisfied
        });
    let named_gap = typed_certificate_authenticated
        && typed_open_gap_ids.iter().any(|gap| {
            gap.contains("NOT_OPERATIONALLY_DEFINED")
                || gap.contains("NOT_OPERATIONALLY_PRESENTED")
                || gap.contains("QUOTIENT_INFINITE")
                || gap.contains(TD21_FIXED_FRAGMENT_GAP)
                || gap.contains(TD21_FINITE_ENUMERATION_GAP)
        });
    let disposition = if certificate.is_none() {
        Td22DependencyDisposition::ExpectedArtifactAbsent
    } else if !typed_certificate_authenticated {
        Td22DependencyDisposition::PresentButUnauthenticated
    } else if named_gap {
        Td22DependencyDisposition::PresentNamedGap
    } else {
        Td22DependencyDisposition::PresentButFiniteDomainContractUnrecognized
    };
    let exact_obstruction = match disposition {
        Td22DependencyDisposition::ExpectedArtifactAbsent => format!(
            "The expected T-D2-1 certificate {} is absent, so no independent finite carrier is available.",
            EXPECTED_T_D2_1_CERTIFICATE_NAME
        ),
        Td22DependencyDisposition::PresentButUnauthenticated => {
            "T-D2-1 bytes are present but are not an authenticated exact-v1 theorem dependency: typed parsing, the exact v1 schema check, or public deterministic replay failed. Generic JSON fields remain diagnostic only and cannot satisfy the dependency contract.".to_owned()
        }
        Td22DependencyDisposition::PresentNamedGap => {
            "The typed exact-v1 T-D2-1 certificate passed public replay and is bound as a named-gap result: the fixed depth-two fragment is not operationally defined and the literal kernel-accepted quotient is not finite.".to_owned()
        }
        Td22DependencyDisposition::PresentButFiniteDomainContractUnrecognized => {
            "The typed exact-v1 T-D2-1 certificate passed public replay, but it does not certify all required facts: an operational independent fragment, decidable membership, a finitely enumerable quotient, and the explicit T-D2-2 prerequisite.".to_owned()
        }
    };

    let mut binding = Td22Td21Binding {
        expected_source_path: EXPECTED_T_D2_1_SOURCE_NAME.to_owned(),
        expected_certificate_path: EXPECTED_T_D2_1_CERTIFICATE_NAME.to_owned(),
        expected_report_path: EXPECTED_T_D2_1_REPORT_NAME.to_owned(),
        source_present: source.is_some(),
        certificate_present: certificate.is_some(),
        report_present: report.is_some(),
        source_sha256: source.as_deref().map(sha256),
        certificate_sha256: certificate.as_deref().map(sha256),
        report_sha256: report.as_deref().map(sha256),
        typed_certificate_parse_succeeded,
        typed_certificate_parse_error,
        exact_v1_schema_matched,
        typed_certificate_result_digest,
        public_replay_valid,
        public_replay_errors,
        typed_certificate_authenticated,
        typed_source_binding_sha256,
        runtime_source_matches_typed_binding,
        report_matches_deterministic_render,
        dependency_trusted,
        typed_open_gap_ids,
        declared_schema,
        declared_status,
        declared_open_gaps,
        fixed_fragment_operationally_defined,
        membership_decidable,
        quotient_finitely_enumerable,
        declared_td22_prerequisite_satisfied,
        dependency_contract_satisfied,
        disposition,
        exact_obstruction,
        binding_digest: String::new(),
    };
    binding.binding_digest = json_digest("t-d2-2/td21-binding/v1", &binding);
    Ok(binding)
}

fn susp_tower(index: usize) -> Expr {
    (0..index).fold(Expr::Univ, |inner, _| Expr::Susp(Box::new(inner)))
}

fn expression_digest(expr: &Expr) -> String {
    sha256(&serde_json::to_vec(expr).expect("Expr serializes"))
}

fn build_counterfamily() -> Result<Td22LiteralKernelCounterfamily, Td22Error> {
    const SAMPLE_PREFIX_SIZE: usize = 7;
    let signature = SealedSignature::genesis_del_h15();
    let mut rows = Vec::with_capacity(SAMPLE_PREFIX_SIZE);
    let mut expressions = Vec::with_capacity(SAMPLE_PREFIX_SIZE);

    for index in 0..SAMPLE_PREFIX_SIZE {
        let expression = susp_tower(index);
        let telescope = Telescope::new(vec![ClauseRec::new(
            ClauseRole::Formation,
            expression.clone(),
        )]);
        let elaboration = elaborate_telescope(&signature, &telescope, 15)
            .map_err(|error| Td22Error::Kernel(format!("E_{index}: {error:?}")))?;
        let clause = elaboration
            .clauses
            .first()
            .ok_or_else(|| Td22Error::Kernel(format!("E_{index}: no clause elaboration")))?;
        let presentation = clause_presentation(&clause.normal_form, 0, &[], 0);
        let index_number = tagged(index, Td22Register::KernelTestimony, "counterfamily index");
        let defining_equation = if index == 0 {
            "E_0 = Univ".to_owned()
        } else {
            format!("E_{index} = Susp(E_{})", index - 1)
        };
        let row_payload = (
            index,
            &defining_equation,
            &expression,
            elaboration.ambient_parameters,
            clause.kernel_role,
            &clause.normal_form,
            &presentation,
        );
        let row_digest = json_digest("t-d2-2/counterfamily-row/v1", &row_payload);
        rows.push(Td22CounterfamilyRow {
            index: index_number,
            defining_equation,
            expression_sha256: expression_digest(&expression),
            kernel_accepted: true,
            ambient_parameters: tagged(
                elaboration.ambient_parameters as usize,
                Td22Register::KernelTestimony,
                "minimal ambient parameters",
            ),
            kernel_role: clause.kernel_role,
            normal_form_sha256: expression_digest(&clause.normal_form),
            normal_form_is_expression: clause.normal_form == expression,
            canonical_parameter_count: tagged(
                presentation.parameters.len(),
                Td22Register::KernelTestimony,
                "canonical natural-family parameter count",
            ),
            closed_zero_parameter_family: presentation.parameters.is_empty(),
            row_digest,
        });
        expressions.push(expression);
    }

    let mut pairwise_checks = Vec::new();
    for left in 0..expressions.len() {
        for right in (left + 1)..expressions.len() {
            let witness = univalent_equality(
                &expressions[left],
                &expressions[right],
                0,
                (left + right + 8) as u32,
            )
            .map_err(|error| {
                Td22Error::Kernel(format!("equality E_{left}/E_{right}: {error:?}"))
            })?;
            let left_digest = expression_digest(&witness.left_normal_form);
            let right_digest = expression_digest(&witness.right_normal_form);
            let pair_payload = (left, right, witness.equal, &left_digest, &right_digest);
            let pair_digest = json_digest("t-d2-2/counterfamily-pair/v1", &pair_payload);
            pairwise_checks.push(Td22CounterfamilyPair {
                left_index: tagged(
                    left,
                    Td22Register::KernelTestimony,
                    "left counterfamily index",
                ),
                right_index: tagged(
                    right,
                    Td22Register::KernelTestimony,
                    "right counterfamily index",
                ),
                frozen_univalent_equal: witness.equal,
                left_normal_form_sha256: left_digest,
                right_normal_form_sha256: right_digest,
                distinct: !witness.equal,
                pair_digest,
            });
        }
    }

    let every_sample_kernel_accepted = rows.iter().all(|row| row.kernel_accepted);
    let every_sample_beta_normal = rows.iter().all(|row| row.normal_form_is_expression);
    let every_sample_closed_zero_parameter =
        rows.iter().all(|row| row.closed_zero_parameter_family);
    let every_sample_pair_distinct = pairwise_checks.iter().all(|pair| pair.distinct);
    let family_definition =
        "E_0 := Univ; E_(n+1) := Susp(E_n), for every natural number n".to_owned();
    let successor_rule = "The frozen synth branch maps every typed inner expression to a typed Susp(inner); normalization recursively preserves the Susp constructor.".to_owned();
    let mathematical_obstruction = "Under the literal all-kernel-accepted reading, n -> E_n is an injection into closed, typed, beta-normal, zero-parameter family presentations. Frozen equality is syntactic equality of beta normal forms, so E_m and E_n remain distinct when m != n. Therefore the quotient is not finitely enumerable. The finite prefix is replay testimony for the generic source-level induction, not an enumeration bound.".to_owned();
    let derivation_payload = (
        &family_definition,
        &successor_rule,
        &rows,
        &pairwise_checks,
        every_sample_kernel_accepted,
        every_sample_beta_normal,
        every_sample_closed_zero_parameter,
        every_sample_pair_distinct,
        &mathematical_obstruction,
    );
    let derivation_digest = json_digest("t-d2-2/counterfamily/v1", &derivation_payload);

    Ok(Td22LiteralKernelCounterfamily {
        family_definition,
        successor_rule,
        quantifier:
            "all natural-number indices; the recorded seven-row prefix is a replay regression only"
                .to_owned(),
        history_independent: true,
        sample_prefix_size: tagged(
            SAMPLE_PREFIX_SIZE,
            Td22Register::KernelTestimony,
            "replayed finite prefix, not a domain bound",
        ),
        rows,
        pairwise_checks,
        every_sample_kernel_accepted,
        every_sample_beta_normal,
        every_sample_closed_zero_parameter,
        every_sample_pair_distinct,
        kernel_successor_closure_source_bound: true,
        normalizer_constructor_preservation_source_bound: true,
        structural_constructor_injectivity_source_bound: true,
        frozen_equality_is_normal_form_syntax_source_bound: true,
        closed_family_quotient_has_no_renaming_identification: true,
        mathematical_obstruction,
        raw_ast_depth_is_schema2_depth: false,
        finite_quotient_conclusion_available: false,
        derivation_digest,
    })
}

fn boundary_audit(counterfamily: &Td22LiteralKernelCounterfamily) -> Td22DomainBoundaryAudit {
    let adjudication = String::from_utf8_lossy(ADJUDICATION_BYTES);
    let expr_source = String::from_utf8_lossy(EXPR_SOURCE_BYTES);
    let telescope_source = String::from_utf8_lossy(TELESCOPE_SOURCE_BYTES);
    let exact_conclusion = "The adopted phrase “fixed depth-two closure fragment” has no operational grammar or semantic depth predicate in the bound authority. Frozen kernel acceptance alone leaves recursive expression nesting and telescope length unbounded. Finite scalar payload types do not make the recursively generated AST carrier finite. T-D2-1 must stop rather than substitute a search node cap.".to_owned();
    let mut audit = Td22DomainBoundaryAudit {
        adjudication_names_fixed_depth_two_fragment: adjudication
            .contains("fixed depth-two closure fragment"),
        adjudication_supplies_inductive_fragment_grammar: false,
        adjudication_supplies_semantic_depth_predicate: false,
        adjudication_supplies_expression_size_bound: false,
        adjudication_supplies_telescope_length_bound: false,
        ambient_parameter_cap: tagged(
            2,
            Td22Register::KernelTestimony,
            "frozen elaborator ambient-parameter cap",
        ),
        ambient_cap_bounds_recursive_expression_depth: false,
        ambient_cap_bounds_telescope_length: false,
        recursive_expr_carrier_detected: expr_source.contains("pub enum Expr")
            && expr_source.contains("Susp(Box<Expr>)"),
        unbounded_clause_vector_detected: telescope_source.contains("pub clauses: Vec<ClauseRec>"),
        finite_u32_payloads_imply_finite_recursive_ast_carrier: false,
        raw_ast_cap_permitted_as_schema2_proxy: false,
        literal_kernel_reading_refuted_by_counterfamily: counterfamily.every_sample_kernel_accepted
            && counterfamily.every_sample_pair_distinct
            && !counterfamily.finite_quotient_conclusion_available,
        exact_conclusion,
        audit_digest: String::new(),
    };
    audit.audit_digest = json_digest(
        "t-d2-2/domain-boundary-audit/v1",
        &(
            audit.adjudication_names_fixed_depth_two_fragment,
            audit.adjudication_supplies_inductive_fragment_grammar,
            audit.adjudication_supplies_semantic_depth_predicate,
            audit.adjudication_supplies_expression_size_bound,
            audit.adjudication_supplies_telescope_length_bound,
            &audit.ambient_parameter_cap,
            audit.ambient_cap_bounds_recursive_expression_depth,
            audit.ambient_cap_bounds_telescope_length,
            audit.recursive_expr_carrier_detected,
            audit.unbounded_clause_vector_detected,
            audit.finite_u32_payloads_imply_finite_recursive_ast_carrier,
            audit.raw_ast_cap_permitted_as_schema2_proxy,
            audit.literal_kernel_reading_refuted_by_counterfamily,
            &audit.exact_conclusion,
        ),
    );
    audit
}

fn procedure_step(ordinal: usize, step_id: &str, action: &str) -> Td22FutureProcedureStep {
    Td22FutureProcedureStep {
        ordinal: tagged(
            ordinal,
            Td22Register::ProofProgram,
            "least-fixed-point procedure ordinal",
        ),
        step_id: step_id.to_owned(),
        action: action.to_owned(),
        domain_membership_created_by_generator: false,
        result_digest: json_digest(
            "t-d2-2/future-procedure-step/v1",
            &(ordinal, step_id, action, false),
        ),
    }
}

fn future_procedure() -> Td22FutureLeastFixedPointProcedure {
    let required_td21_exports = vec![
        "sealed generator-independent finite carrier with structural representatives".to_owned(),
        "decidable membership and a semantic depth-one/depth-two tag".to_owned(),
        "typed normalization, idempotence, and equality witnesses".to_owned(),
        "family/instance quotient decision in both directions".to_owned(),
        "finite intermediate closure or cut-elimination for generator premises".to_owned(),
    ];
    let steps = vec![
        procedure_step(
            1,
            "seal-independent-carrier",
            "Replay T-D2-1 and seal the carrier, quotient, history digest, and depth tags before loading any generator reachability relation.",
        ),
        procedure_step(
            2,
            "initialize-zero-premise-reachability",
            "Mark only independently present carrier nodes concluded by verified zero-premise rules.",
        ),
        procedure_step(
            3,
            "saturate-frozen-rule-relation",
            "Iterate frozen rules to a least fixed point; each rule may mark an existing carrier node reachable but may not manufacture domain membership.",
        ),
        procedure_step(
            4,
            "recheck-typed-normal-equality",
            "Re-elaborate and normalize every conclusion, then compare through the T-D2-1 structural equality witness rather than a hash alone.",
        ),
        procedure_step(
            5,
            "check-both-inclusions",
            "Check D subset Reach for surjectivity and Reach subset D for rule soundness/closure; never silently discard an out-of-domain conclusion.",
        ),
        procedure_step(
            6,
            "publish-proof-or-refutation",
            "Emit derivations for every carrier class or publish the least unreached structural representative with its failed-rule frontier.",
        ),
        procedure_step(
            7,
            "recompute-depth-one-restriction",
            "Filter by the independently defined depth-one tag, rerun saturation, and only afterward compare the sealed depth-one campaign as regression.",
        ),
    ];
    let mut procedure = Td22FutureLeastFixedPointProcedure {
        procedure_id: "independent-carrier-least-fixed-point-surjectivity-v1".to_owned(),
        required_td21_exports,
        steps,
        domain_carrier_sealed_before_generator_reachability: true,
        generator_rules_only_mark_existing_domain_nodes: true,
        generator_rules_may_extend_domain_carrier: false,
        equality_uses_structural_witness_not_hash_alone: true,
        domain_subset_reach_is_surjectivity_obligation: true,
        reach_subset_domain_is_soundness_obligation: true,
        missing_domain_node_is_published_refutation: true,
        out_of_domain_generator_conclusion_is_silent_drop: false,
        arbitrary_derivation_depth_cap_used: false,
        intermediate_closure_or_cut_elimination_required: true,
        depth_one_restriction_recomputed_from_independent_tag: true,
        procedure_executed: false,
        procedure_digest: String::new(),
    };
    procedure.procedure_digest = json_digest(
        "t-d2-2/future-least-fixed-point-procedure/v1",
        &(
            &procedure.procedure_id,
            &procedure.required_td21_exports,
            &procedure.steps,
            procedure.domain_carrier_sealed_before_generator_reachability,
            procedure.generator_rules_only_mark_existing_domain_nodes,
            procedure.generator_rules_may_extend_domain_carrier,
            procedure.equality_uses_structural_witness_not_hash_alone,
            procedure.domain_subset_reach_is_surjectivity_obligation,
            procedure.reach_subset_domain_is_soundness_obligation,
            procedure.missing_domain_node_is_published_refutation,
            procedure.out_of_domain_generator_conclusion_is_silent_drop,
            procedure.arbitrary_derivation_depth_cap_used,
            procedure.intermediate_closure_or_cut_elimination_required,
            procedure.depth_one_restriction_recomputed_from_independent_tag,
            procedure.procedure_executed,
        ),
    );
    procedure
}

fn static_source_bindings() -> Vec<Td22SourceBinding> {
    vec![
        source_binding(
            "docs/depth_two_domain_adjudication.md",
            "adopted T-D2 law and boundedness stop",
            ADJUDICATION_BYTES,
            Td22Register::Law,
        ),
        source_binding(
            "docs/step_15_completion_open_problem.md",
            "prior Schema2 grammar requirement and raw-AST warning",
            OPEN_PROBLEM_BYTES,
            Td22Register::Law,
        ),
        source_binding(
            "crates/pen-core/src/expr.rs",
            "recursive expression carrier and structural equality",
            EXPR_SOURCE_BYTES,
            Td22Register::KernelTestimony,
        ),
        source_binding(
            "crates/pen-core/src/telescope.rs",
            "unbounded clause-vector representation",
            TELESCOPE_SOURCE_BYTES,
            Td22Register::KernelTestimony,
        ),
        source_binding(
            "crates/pen-type/src/elaborate.rs",
            "frozen kernel acceptance and Susp closure",
            ELABORATE_SOURCE_BYTES,
            Td22Register::KernelTestimony,
        ),
        source_binding(
            "crates/pen-type/src/normalize.rs",
            "frozen normalizer and Susp preservation",
            NORMALIZE_SOURCE_BYTES,
            Td22Register::KernelTestimony,
        ),
        source_binding(
            "crates/pen-type/src/equality.rs",
            "frozen beta-normal syntactic equality",
            EQUALITY_SOURCE_BYTES,
            Td22Register::KernelTestimony,
        ),
        source_binding(
            "crates/pen-eval/src/typed_families.rs",
            "adopted family presentation quotient",
            TYPED_FAMILIES_SOURCE_BYTES,
            Td22Register::KernelTestimony,
        ),
        source_binding(
            "crates/pen-search/src/t_d2_2_completeness_attempt_v1.rs",
            "this theorem-attempt issuer and replay",
            THIS_SOURCE_BYTES,
            Td22Register::ProofProgram,
        ),
    ]
}

fn runtime_td21_source_bindings() -> Result<Vec<Td22SourceBinding>, Td22Error> {
    let (source_path, certificate_path, report_path) = expected_td21_paths();
    let mut bindings = Vec::new();
    for (path, label, role) in [
        (
            source_path,
            EXPECTED_T_D2_1_SOURCE_NAME,
            "T-D2-1 domain-build implementation",
        ),
        (
            certificate_path,
            EXPECTED_T_D2_1_CERTIFICATE_NAME,
            "T-D2-1 create-new certificate",
        ),
        (
            report_path,
            EXPECTED_T_D2_1_REPORT_NAME,
            "T-D2-1 deterministic result report",
        ),
    ] {
        if let Some(bytes) = optional_file(&path)? {
            bindings.push(source_binding(
                label,
                role,
                &bytes,
                Td22Register::DependencyTestimony,
            ));
        }
    }
    Ok(bindings)
}

fn certificate_digest(certificate: &Td22CompletenessAttemptV1Certificate) -> String {
    let mut payload = certificate.clone();
    payload.result_digest.clear();
    json_digest("t-d2-2/completeness-attempt-certificate/v1", &payload)
}

fn build_certificate(
    source_path: &Path,
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Td22CompletenessAttemptV1Certificate, Td22Error> {
    let td21_binding = td21_binding_from_paths(source_path, certificate_path, report_path)?;
    if td21_binding.dependency_contract_satisfied {
        return Err(Td22Error::DependencyAlreadySatisfied);
    }
    let literal_kernel_counterfamily = build_counterfamily()?;
    let boundary_audit = boundary_audit(&literal_kernel_counterfamily);
    let future_least_fixed_point_procedure = future_procedure();
    let mut source_bindings = static_source_bindings();
    source_bindings.extend(runtime_td21_source_bindings()?);
    let open_gaps = vec![
        TD21_FIXED_FRAGMENT_GAP.to_owned(),
        TD21_MEMBERSHIP_GAP.to_owned(),
        TD21_NORMALIZATION_GAP.to_owned(),
        TD21_QUOTIENT_GAP.to_owned(),
        TD21_FINITE_ENUMERATION_GAP.to_owned(),
        TD21_REGRESSION_BLOCKED_GAP.to_owned(),
        TD22_LITERAL_KERNEL_COUNTERFAMILY_OBSTRUCTION.to_owned(),
        T_D2_2_DEPENDENCY_GAP.to_owned(),
    ];
    let zero_charge = Td22ZeroCharge {
        kappa_minted: tagged(0, Td22Register::Gate, "kappa minted"),
        nu_minted: tagged(0, Td22Register::Gate, "nu minted"),
        anchors_minted: tagged(0, Td22Register::Gate, "anchors minted"),
        zero_charge_holds: true,
    };
    let gate = Td22Gate {
        td21_domain_build_required: true,
        td21_domain_build_satisfied: false,
        td22_surjectivity_verdict_issued: false,
        td22_refutation_verdict_issued: false,
        td23_or_bc3_authorized_by_this_artifact: false,
        m3_or_m4_gate_moved: false,
        exact_gate_statement: "T-D2-2 remains unattempted until T-D2-1 exports a generator-independent, decidable, finitely enumerable semantic carrier. This artifact changes no downstream gate.".to_owned(),
    };
    let mut certificate = Td22CompletenessAttemptV1Certificate {
        schema: T_D2_2_COMPLETENESS_ATTEMPT_V1_SCHEMA.to_owned(),
        date: T_D2_2_COMPLETENESS_ATTEMPT_V1_DATE.to_owned(),
        status: Td22RunStatus::StoppedOnTd21FiniteDomainDependency,
        theorem_disposition: Td22TheoremDisposition::NotAttemptedDependencyUnsatisfied,
        adjudication_sha256: sha256(ADJUDICATION_BYTES),
        source_bindings,
        td21_binding,
        boundary_audit,
        literal_kernel_counterfamily,
        future_least_fixed_point_procedure,
        domain_membership_decision_executed: false,
        generator_inventory_imported_into_domain_membership: false,
        a3_inventory_imported_into_domain_membership: false,
        historical_61_or_89_rows_used_to_seed_or_bound_domain: false,
        surjectivity_claimed: false,
        incompleteness_claimed: false,
        exact_result: "Dependency stop, not a completeness verdict: T-D2-1 has not produced an operative independent finite quotient. The literal kernel-accepted reading is obstructed by the closed family E_n = Susp^n(Univ), while the intended fixed depth-two fragment remains operationally undefined. The future least-fixed-point algorithm is registered but not executed.".to_owned(),
        open_gaps,
        zero_charge,
        gate,
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn issue_t_d2_2_completeness_attempt_v1()
-> Result<Td22CompletenessAttemptV1Certificate, Td22Error> {
    let (source_path, certificate_path, report_path) = expected_td21_paths();
    build_certificate(&source_path, &certificate_path, &report_path)
}

pub fn replay_t_d2_2_completeness_attempt_v1(
    claimed: &Td22CompletenessAttemptV1Certificate,
) -> Td22Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("result digest mismatch".to_owned());
    }
    match issue_t_d2_2_completeness_attempt_v1() {
        Ok(expected) => {
            if claimed != &expected {
                errors.push(
                    "certificate differs from a fresh replay over the bound authorities".to_owned(),
                );
            }
        }
        Err(error) => errors.push(format!("fresh replay failed: {error}")),
    }
    if claimed.surjectivity_claimed || claimed.incompleteness_claimed {
        errors.push("a dependency-stop artifact issued a theorem verdict".to_owned());
    }
    if claimed.domain_membership_decision_executed
        || claimed.generator_inventory_imported_into_domain_membership
        || claimed.a3_inventory_imported_into_domain_membership
        || claimed.historical_61_or_89_rows_used_to_seed_or_bound_domain
    {
        errors.push("independence discipline violated".to_owned());
    }
    if claimed
        .future_least_fixed_point_procedure
        .procedure_executed
    {
        errors.push("future procedure marked executed without T-D2-1".to_owned());
    }
    if !claimed.zero_charge.zero_charge_holds
        || claimed.zero_charge.kappa_minted.decimal != "0"
        || claimed.zero_charge.nu_minted.decimal != "0"
        || claimed.zero_charge.anchors_minted.decimal != "0"
    {
        errors.push("zero-charge invariant failed".to_owned());
    }
    if claimed.gate.td22_surjectivity_verdict_issued
        || claimed.gate.td22_refutation_verdict_issued
        || claimed.gate.td23_or_bc3_authorized_by_this_artifact
        || claimed.gate.m3_or_m4_gate_moved
    {
        errors.push("dependency stop moved a downstream gate".to_owned());
    }
    Td22Replay {
        valid: errors.is_empty(),
        status: Some(claimed.status),
        theorem_disposition: Some(claimed.theorem_disposition),
        td21_dependency_satisfied: claimed.td21_binding.dependency_contract_satisfied,
        surjectivity_verdict_issued: claimed.surjectivity_claimed || claimed.incompleteness_claimed,
        future_procedure_executed: claimed
            .future_least_fixed_point_procedure
            .procedure_executed,
        errors,
    }
}

pub fn replay_t_d2_2_completeness_attempt_v1_json(json: &str) -> Td22Replay {
    match serde_json::from_str::<Td22CompletenessAttemptV1Certificate>(json) {
        Ok(certificate) => replay_t_d2_2_completeness_attempt_v1(&certificate),
        Err(error) => Td22Replay {
            valid: false,
            status: None,
            theorem_disposition: None,
            td21_dependency_satisfied: false,
            surjectivity_verdict_issued: false,
            future_procedure_executed: false,
            errors: vec![format!("certificate JSON failed closed: {error}")],
        },
    }
}

pub fn render_t_d2_2_completeness_attempt_v1(
    certificate: &Td22CompletenessAttemptV1Certificate,
) -> String {
    let dependency = &certificate.td21_binding;
    let counterfamily = &certificate.literal_kernel_counterfamily;
    let mut report = String::new();
    report.push_str("# T-D2-2 completeness attempt v1 — lawful dependency stop\n\n");
    report.push_str(&format!(
        "**Date:** {}. **Status:** `{:?}`. **Theorem disposition:** `{:?}`.\n\n",
        certificate.date, certificate.status, certificate.theorem_disposition
    ));
    report.push_str("## Result\n\n");
    report.push_str(&certificate.exact_result);
    report.push_str("\n\nNo surjectivity or incompleteness verdict was issued. No domain-membership decision was run, and no generator, A3 inventory, or historical regression row entered a membership definition.\n\n");
    report.push_str("## T-D2-1 dependency binding\n\n");
    report.push_str(&format!(
        "- source: `{}` (present: `{}`; digest: `{}`)\n- certificate: `{}` (present: `{}`; digest: `{}`)\n- report: `{}` (present: `{}`; digest: `{}`)\n- typed certificate parse: `{}`{}\n- exact v1 schema: `{}`\n- typed result digest: `{}`\n- public T-D2-1 replay: `{}`\n- typed certificate authenticated: `{}`\n- source matches typed source binding: `{}`\n- report matches deterministic rendering: `{}`\n- complete dependency binding trusted: `{}`\n- declared schema (generic diagnostic): `{}`\n- declared status (generic diagnostic): `{}`\n- declared T-D2-2 prerequisite (generic diagnostic): `{:?}`\n- disposition: `{:?}`\n- finite-domain contract satisfied: `{}`\n\n{}\n\n",
        dependency.expected_source_path,
        dependency.source_present,
        dependency.source_sha256.as_deref().unwrap_or("absent"),
        dependency.expected_certificate_path,
        dependency.certificate_present,
        dependency.certificate_sha256.as_deref().unwrap_or("absent"),
        dependency.expected_report_path,
        dependency.report_present,
        dependency.report_sha256.as_deref().unwrap_or("absent"),
        dependency.typed_certificate_parse_succeeded,
        dependency
            .typed_certificate_parse_error
            .as_ref()
            .map(|error| format!(" (error: {error})"))
            .unwrap_or_default(),
        dependency.exact_v1_schema_matched,
        dependency
            .typed_certificate_result_digest
            .as_deref()
            .unwrap_or("unavailable"),
        dependency.public_replay_valid,
        dependency.typed_certificate_authenticated,
        dependency.runtime_source_matches_typed_binding,
        dependency.report_matches_deterministic_render,
        dependency.dependency_trusted,
        dependency.declared_schema.as_deref().unwrap_or("unavailable"),
        dependency.declared_status.as_deref().unwrap_or("unavailable"),
        dependency.declared_td22_prerequisite_satisfied,
        dependency.disposition,
        dependency.dependency_contract_satisfied,
        dependency.exact_obstruction,
    ));
    if !dependency.typed_open_gap_ids.is_empty() {
        report.push_str("Authenticated typed T-D2-1 gap IDs:\n\n");
        for gap in &dependency.typed_open_gap_ids {
            report.push_str(&format!("- `{gap}`\n"));
        }
        report.push('\n');
    }
    if !dependency.public_replay_errors.is_empty() {
        report.push_str("Public T-D2-1 replay errors:\n\n");
        for error in &dependency.public_replay_errors {
            report.push_str(&format!("- {error}\n"));
        }
        report.push('\n');
    }
    if !dependency.declared_open_gaps.is_empty() {
        report.push_str("Generic JSON gap diagnostics (non-authoritative):\n\n");
        for gap in &dependency.declared_open_gaps {
            report.push_str(&format!("- `{gap}`\n"));
        }
        report.push('\n');
    }
    report.push_str("## Why literal kernel acceptance is not finite\n\n");
    report.push_str(&counterfamily.mathematical_obstruction);
    report.push_str("\n\n");
    report.push_str("| n | equation | accepted | ambient | normal fixed point | parameters | expression digest |\n|---:|---|:---:|---:|:---:|---:|---|\n");
    for row in &counterfamily.rows {
        report.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | `{}` |\n",
            row.index.decimal,
            row.defining_equation,
            row.kernel_accepted,
            row.ambient_parameters.decimal,
            row.normal_form_is_expression,
            row.canonical_parameter_count.decimal,
            row.expression_sha256,
        ));
    }
    report.push_str("\nThe finite table is a replay regression for the generic induction, never an enumeration bound. `u32` constructor payloads do not bound recursive AST height, and the ambient-parameter cap does not bound expression nesting or telescope length. The previously adopted problem statement explicitly forbids replacing semantic Schema2 depth with raw AST node depth.\n\n");
    report.push_str("## Registered future theorem procedure\n\n");
    for step in &certificate.future_least_fixed_point_procedure.steps {
        report.push_str(&format!(
            "{}. **{}:** {}\n",
            step.ordinal.decimal, step.step_id, step.action
        ));
    }
    report.push_str("\nThis least-fixed-point procedure is registered but was not executed. It first seals an independent finite carrier, then permits generator rules only to mark existing carrier nodes reachable. It checks both `D ⊆ Reach` and `Reach ⊆ D`; missing nodes and out-of-domain conclusions are published, not suppressed.\n\n");
    report.push_str("## Gate and charge\n\n");
    report.push_str(&format!(
        "{} T-D2-3, BC3, M3, and M4 receive no authorization from this artifact. Charge: κ = {}, ν = {}, anchors = {}.\n\n",
        certificate.gate.exact_gate_statement,
        certificate.zero_charge.kappa_minted.decimal,
        certificate.zero_charge.nu_minted.decimal,
        certificate.zero_charge.anchors_minted.decimal,
    ));
    report.push_str("## Source bindings\n\n");
    report.push_str("| path | role | bytes | SHA-256 |\n|---|---|---:|---|\n");
    for binding in &certificate.source_bindings {
        report.push_str(&format!(
            "| `{}` | {} | {} | `{}` |\n",
            binding.path, binding.role, binding.byte_length.decimal, binding.sha256
        ));
    }
    report.push_str(&format!(
        "\nCertificate digest: `{}`\n",
        certificate.result_digest
    ));
    report
}

fn cleanup_created(path: &Path, created: bool) {
    if created {
        let _ = remove_file(path);
    }
}

pub fn emit_t_d2_2_completeness_attempt_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Td22CompletenessAttemptV1Certificate, Td22Error> {
    let certificate = issue_t_d2_2_completeness_attempt_v1()?;
    let replay = replay_t_d2_2_completeness_attempt_v1(&certificate);
    if !replay.valid {
        return Err(Td22Error::Invariant(format!(
            "new certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let mut json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Td22Error::Json(error.to_string()))?;
    json.push(b'\n');
    let report = render_t_d2_2_completeness_attempt_v1(&certificate);
    let mut certificate_created = false;
    let mut report_created = false;
    let result = (|| -> Result<(), Td22Error> {
        let mut certificate_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(certificate_path)
            .map_err(|error| Td22Error::Io(error.to_string()))?;
        certificate_created = true;
        certificate_file
            .write_all(&json)
            .and_then(|_| certificate_file.sync_all())
            .map_err(|error| Td22Error::Io(error.to_string()))?;
        drop(certificate_file);

        let mut report_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(report_path)
            .map_err(|error| Td22Error::Io(error.to_string()))?;
        report_created = true;
        report_file
            .write_all(report.as_bytes())
            .and_then(|_| report_file.sync_all())
            .map_err(|error| Td22Error::Io(error.to_string()))?;
        drop(report_file);

        let emitted_json =
            read_to_string(certificate_path).map_err(|error| Td22Error::Io(error.to_string()))?;
        let emitted_report = read(report_path).map_err(|error| Td22Error::Io(error.to_string()))?;
        if emitted_json.as_bytes() != json.as_slice() {
            return Err(Td22Error::Invariant(
                "emitted certificate bytes differ from create-new payload".to_owned(),
            ));
        }
        if emitted_report.as_slice() != report.as_bytes() {
            return Err(Td22Error::Invariant(
                "emitted report differs from deterministic rendering".to_owned(),
            ));
        }
        let emitted_replay = replay_t_d2_2_completeness_attempt_v1_json(&emitted_json);
        if !emitted_replay.valid {
            return Err(Td22Error::Invariant(format!(
                "emitted certificate did not replay: {}",
                emitted_replay.errors.join("; ")
            )));
        }
        Ok(())
    })();
    if let Err(error) = result {
        cleanup_created(certificate_path, certificate_created);
        cleanup_created(report_path, report_created);
        return Err(error);
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::{
        Td22TheoremDisposition, certificate_digest, issue_t_d2_2_completeness_attempt_v1,
        replay_t_d2_2_completeness_attempt_v1, replay_t_d2_2_completeness_attempt_v1_json,
    };
    use serde_json::Value;

    fn reseal(certificate: &mut super::Td22CompletenessAttemptV1Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    #[test]
    fn dependency_stop_replays_without_a_theorem_verdict() {
        let certificate = issue_t_d2_2_completeness_attempt_v1().expect("T-D2-2 stop issues");
        let replay = replay_t_d2_2_completeness_attempt_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(
            certificate.theorem_disposition,
            Td22TheoremDisposition::NotAttemptedDependencyUnsatisfied
        );
        assert!(!certificate.surjectivity_claimed);
        assert!(!certificate.incompleteness_claimed);
        assert!(!certificate.domain_membership_decision_executed);
        assert!(
            !certificate
                .future_least_fixed_point_procedure
                .procedure_executed
        );
        assert!(certificate.zero_charge.zero_charge_holds);
    }

    #[test]
    fn susp_counterfamily_is_typed_normal_closed_and_pairwise_distinct() {
        let certificate = issue_t_d2_2_completeness_attempt_v1().expect("T-D2-2 stop issues");
        let family = &certificate.literal_kernel_counterfamily;
        assert!(family.every_sample_kernel_accepted);
        assert!(family.every_sample_beta_normal);
        assert!(family.every_sample_closed_zero_parameter);
        assert!(family.every_sample_pair_distinct);
        assert_eq!(family.rows.len(), 7);
        assert_eq!(family.pairwise_checks.len(), 21);
        assert!(!family.raw_ast_depth_is_schema2_depth);
        assert!(!family.finite_quotient_conclusion_available);
    }

    #[test]
    fn verdict_independence_and_gate_mutations_fail_replay() {
        let certificate = issue_t_d2_2_completeness_attempt_v1().expect("T-D2-2 stop issues");

        let mut verdict = certificate.clone();
        verdict.surjectivity_claimed = true;
        reseal(&mut verdict);
        assert!(!replay_t_d2_2_completeness_attempt_v1(&verdict).valid);

        let mut circular = certificate.clone();
        circular.generator_inventory_imported_into_domain_membership = true;
        reseal(&mut circular);
        assert!(!replay_t_d2_2_completeness_attempt_v1(&circular).valid);

        let mut gate = certificate.clone();
        gate.gate.m3_or_m4_gate_moved = true;
        reseal(&mut gate);
        assert!(!replay_t_d2_2_completeness_attempt_v1(&gate).valid);

        let mut procedure = certificate;
        procedure
            .future_least_fixed_point_procedure
            .procedure_executed = true;
        reseal(&mut procedure);
        assert!(!replay_t_d2_2_completeness_attempt_v1(&procedure).valid);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_t_d2_2_completeness_attempt_v1().expect("T-D2-2 stop issues");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("unknown_field".to_owned(), Value::Bool(true));
        let replay = replay_t_d2_2_completeness_attempt_v1_json(
            &serde_json::to_string(&value).expect("json"),
        );
        assert!(!replay.valid);
    }
}
