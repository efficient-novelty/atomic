//! T-D2-1: independent depth-two semantic-domain construction attempt.
//!
//! `depth-two-semantic-domain-v1` requires a finite, decidable quotient of the
//! typed normal judgments in an independently specified depth-two closure
//! fragment.  The adopted sources do not operationally present that fragment.
//! In particular, the literal frozen kernel surface contains unbounded `Susp`
//! and `Trunc` towers.  This issuer records the obstruction before it opens the
//! historical regression corpora, so the corpora can authenticate the promised
//! regression gate but cannot seed membership, enumeration, or the quotient.

use crate::m3_e7_e8_bridge_v1::{
    M3_E7_GAP, M3E7E8BridgeV1Certificate, M3RunStatus, replay_m3_e7_e8_bridge_v1_json,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::check::{CheckResult, check_telescope};
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use pen_type::equality::univalent_equality;
use pen_type::normalize::normalize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest as _, Sha256};
use std::fs::{OpenOptions, read_to_string, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const T_D2_1_DOMAIN_BUILD_V1_SCHEMA: &str = "t-d2-1-independent-depth-two-domain-build-v1";
pub const T_D2_1_DOMAIN_BUILD_V1_DATE: &str = "2026-07-23";

pub const TD21_FIXED_FRAGMENT_GAP: &str =
    "T_D2_1_FIXED_DEPTH_TWO_FRAGMENT_NOT_OPERATIONALLY_DEFINED";
pub const TD21_MEMBERSHIP_GAP: &str = "TD21_NFSCH2_MEMBERSHIP_DECISION_NOT_TOTAL";
pub const TD21_NORMALIZATION_GAP: &str = "TD21_TOTAL_TYPED_NORMALIZATION_NOT_PROVED";
pub const TD21_QUOTIENT_GAP: &str = "TD21_GENERAL_UNIVALENT_FAMILY_QUOTIENT_NOT_DECIDABLE";
pub const TD21_FINITE_ENUMERATION_GAP: &str = "T_D2_1_LITERAL_KERNEL_ACCEPTED_QUOTIENT_INFINITE";
pub const TD21_REGRESSION_BLOCKED_GAP: &str = "TD21_CORPUS_REGRESSION_BLOCKED_UNTIL_DOMAIN_EXISTS";

const EXPECTED_M3_RESULT_DIGEST: &str =
    "blake3:a61fe8093ef9ce61a05aebfe859b0723a88a766a712ea6a2312e0f22ca386ba8";
const EXPECTED_FAMILY_CORPUS_RESULT_DIGEST: &str =
    "blake3:d51d0543c32542200fff6e5cfd234da1bb2ffc12c058c805ac7974385727c8b4";
const EXPECTED_A3_CORPUS_RESULT_DIGEST: &str =
    "blake3:bd3f33ed7a15b6f50c421d2e42ed7aaea710fbcab750d0a98a990a1dad3b30db";

const EXPECTED_ADJUDICATION_SHA256: &str =
    "fffd48c0eb5f89b6728842aa6f9de5aa43013ab867278d6b8ec6b2737ed92151";
const EXPECTED_M3_SHA256: &str = "22cc8450829a403081c1d4db7125523c198c7e902f9f730f415be2fc0f305aa0";
const EXPECTED_BC1_SHA256: &str =
    "0a6930ec9a1db06952cca78fcf2c9eaa4371b39b4b51762331101b827d036c2b";
const EXPECTED_FAMILY_CORPUS_SHA256: &str =
    "51419e53f9dcd0f9c61ddb21c0c116ab92de05d5b5e142792535eeda45136a18";
const EXPECTED_A3_CORPUS_SHA256: &str =
    "73d5d25cf9a91ab82100f14bb1e1cfe424c97ab0ac2503664278df4268ac5337";
const EXPECTED_QUOTIENT_ADJUDICATION_SHA256: &str =
    "ba9b57379e4b53c9f1ef54982e7c4551754ad82262d11805dc6e7c752f66b897";
const EXPECTED_OPEN_PROBLEM_SHA256: &str =
    "fe3b2eb8f17f12563596d7a937d39d2618625503cf4ba1af8bba33e224309c82";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/depth_two_domain_adjudication.md");
const M3_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e7_e8_bridge_v1.json");
const BC1_BYTES: &[u8] = include_bytes!("../../../docs/bc1_depth_two_completeness_v1.json");
const OPEN_PROBLEM_BYTES: &[u8] =
    include_bytes!("../../../docs/step_15_completion_open_problem.md");
const QUOTIENT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/e2_quotient_adjudications.md");
const FAMILY_CORPUS_BYTES: &[u8] =
    include_bytes!("../../../docs/t_bi_nu1_semantic_provenance_v6.json");
const A3_CORPUS_BYTES: &[u8] =
    include_bytes!("../../../docs/a3_rule_inventory_exhaustiveness_v2.json");
const EXPR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/expr.rs");
const CHECK_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/check.rs");
const ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const NORMALIZE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const CONTEXT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/context.rs");
const GRAMMAR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/grammar.rs");
const E3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/e3_normalization.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_d2_1_domain_build_v1.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21Register {
    SemanticFamilyAuthority,
    StructuralTestimony,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21RunStatus {
    StoppedNamedGaps,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21CorpusRegressionStatus {
    BlockedByDomainGaps,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21TowerFormer {
    Suspension,
    Truncation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21TaggedNumber {
    pub value: usize,
    pub register: Td21Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: Td21TaggedNumber,
    pub blake3: String,
    pub sha256: String,
    pub register: Td21Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21M3Baseline {
    pub result_digest: String,
    pub public_replay_valid: bool,
    pub public_replay_errors: Vec<String>,
    pub status: M3RunStatus,
    pub exact_enacted_regression_reproduced: bool,
    pub proved_bridge_condition_count: Td21TaggedNumber,
    pub bridge_condition_count: Td21TaggedNumber,
    pub c2_open: bool,
    pub expected_c2_gap_present: bool,
    pub zero_promotions_made: bool,
    pub promoted_row_count: Td21TaggedNumber,
    pub bridge_claim_issued: bool,
    pub m4_authorized: bool,
    pub baseline_shift_detected: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21IndependenceFirewall {
    pub definition_inputs: Vec<String>,
    pub observer_inputs: Vec<String>,
    pub definition_input_digest: String,
    pub observer_input_digest: String,
    pub generator_inventory_reaches_domain_definition: bool,
    pub a3_inventory_reaches_membership: bool,
    pub bridge_rows_reach_enumeration: bool,
    pub historical_corpora_seed_carrier: bool,
    pub raw_ast_height_cap_used_as_depth_two_definition: bool,
    pub definition_and_observer_dataflow_disjoint: bool,
    pub f_d2_1_circularity_fired: bool,
    pub f_d2_3_proxy_reentry_fired: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21TowerPrefixRow {
    pub former: Td21TowerFormer,
    pub depth: Td21TaggedNumber,
    pub expression_hash: String,
    pub conservative_kernel_check_accepted: bool,
    pub sealed_signature_elaboration_accepted: bool,
    pub elaborated_kernel_type_json: String,
    pub elaborated_normal_form_hash: String,
    pub elaborated_normal_form_identical: bool,
    pub elaboration_derivation_hash: String,
    pub normalizer_returned_identical_expression: bool,
    pub beta_normal: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21InfiniteKernelSurfaceWitness {
    pub families: Vec<String>,
    pub replayed_prefix_depth: Td21TaggedNumber,
    pub prefix_rows: Vec<Td21TowerPrefixRow>,
    pub every_prefix_row_kernel_accepted_and_normal: bool,
    pub every_prefix_row_elaborated_and_normal: bool,
    pub base_case: String,
    pub normality_successor_case: String,
    pub typing_successor_case: String,
    pub distinctness_successor_case: String,
    pub quantified_proof_mechanization_scope: String,
    pub induction_quantifies_over_all_natural_depths: bool,
    pub theorem_is_structural_induction_not_finite_search: bool,
    pub suspension_tower_pairwise_distinct: bool,
    pub truncation_tower_pairwise_distinct: bool,
    pub positive_depth_towers_constructor_disjoint: bool,
    pub adopted_equality_is_beta_normal_form_equality: bool,
    pub finite_prefix_adopted_equality_regression_passed: bool,
    pub literal_frozen_kernel_normal_surface_infinite: bool,
    pub literal_frozen_kernel_equality_quotient_infinite: bool,
    pub finite_prefix_used_only_as_replay_regression: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21ScopedMachineryProbe {
    pub frozen_checker_accepts_recursive_unary_formers: bool,
    pub frozen_sealed_signature_elaborator_types_recursive_unary_formers: bool,
    pub frozen_normalizer_preserves_recursive_unary_formers: bool,
    pub adopted_equality_source_authenticated: bool,
    pub family_instance_quotient_adjudication_authenticated: bool,
    pub typed_normalizer_total_over_all_intended_schema2_terms: bool,
    pub general_univalent_equality_proved: bool,
    pub arbitrary_context_presentation_quotient_proved: bool,
    pub probe_is_negative_scope_testimony_only: bool,
    pub general_limit_flags_are_source_bound_audit_not_new_theorem_replay: bool,
    pub probe_reaches_domain_membership: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21DomainAttempt {
    pub intended_name: String,
    pub independent_definition_attempted_before_corpus_observation: bool,
    pub fixed_depth_two_constructor_closure_operationally_presented: bool,
    pub context_bound_operationally_presented: bool,
    pub binder_bound_operationally_presented: bool,
    pub public_symbol_bound_operationally_presented: bool,
    pub cubical_dimension_bound_operationally_presented: bool,
    pub literal_kernel_surface_is_candidate_domain: bool,
    pub literal_kernel_surface_rejected_by_finiteness_obligation: bool,
    pub generator_derived_surrogate_used: bool,
    pub raw_ast_height_surrogate_used: bool,
    pub nf_sch2_domain_operational: bool,
    pub membership_decidable: bool,
    pub typed_normalization_total_on_domain: bool,
    pub adopted_family_quotient_decidable_on_domain: bool,
    pub quotient_finite_proved: bool,
    pub finite_enumerator_constructed: bool,
    pub lawful_stop_taken: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21CorpusObserver {
    pub status: Td21CorpusRegressionStatus,
    pub opened_only_after_domain_attempt: bool,
    pub family_corpus_authenticated: bool,
    pub family_corpus_result_digest: String,
    pub promised_family_count: Td21TaggedNumber,
    pub a3_corpus_authenticated: bool,
    pub a3_corpus_result_digest: String,
    pub promised_membership_row_count: Td21TaggedNumber,
    pub a3_relative_exhaustiveness_proved: bool,
    pub a3_absolute_semantic_exhaustiveness_claimed: bool,
    pub corpus_used_to_define_domain: bool,
    pub corpus_used_to_seed_membership: bool,
    pub corpus_used_to_seed_enumeration: bool,
    pub membership_query_formable: bool,
    pub regression_executed: bool,
    pub every_promised_family_recognized: bool,
    pub every_promised_row_recognized: bool,
    pub regression_failure_claimed: bool,
    pub exact_blocker: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21NamedGap {
    pub id: String,
    pub phase: String,
    pub exact_obstruction: String,
    pub keeps_t_d2_2_closed: bool,
    pub keeps_m4_unauthorized: bool,
    pub zero_charge: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21DomainBuildV1Certificate {
    pub schema: String,
    pub date: String,
    pub syntax_identifier_policy: String,
    pub syntax_identifier_register: Td21Register,
    pub source_bindings: Vec<Td21SourceBinding>,
    pub m3_v1_baseline: Td21M3Baseline,
    pub independence_firewall: Td21IndependenceFirewall,
    pub infinite_kernel_surface_witness: Td21InfiniteKernelSurfaceWitness,
    pub scoped_machinery_probe: Td21ScopedMachineryProbe,
    pub domain_attempt: Td21DomainAttempt,
    pub corpus_observer: Td21CorpusObserver,
    pub open_gaps: Vec<Td21NamedGap>,
    pub open_gap_count: Td21TaggedNumber,
    pub status: Td21RunStatus,
    pub fixed_fragment_operationally_defined: bool,
    pub nf_sch2_domain_operational: bool,
    pub membership_decidable: bool,
    pub quotient_finite_proved: bool,
    pub quotient_finitely_enumerable: bool,
    pub no_charge_minted: bool,
    pub kappa_charge_minted: Td21TaggedNumber,
    pub nu_charge_minted: Td21TaggedNumber,
    pub anchor_charge_minted: Td21TaggedNumber,
    pub t_d2_1_status: Td21RunStatus,
    pub t_d2_2_prerequisite_satisfied: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m4_authorized: bool,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub t_d2_1_status: Td21RunStatus,
    pub nf_sch2_domain_operational: bool,
    pub t_d2_2_prerequisite_satisfied: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Td21Error {
    #[error("T-D2-1 input failure: {0}")]
    Input(String),
    #[error("T-D2-1 baseline failure: {0}")]
    Baseline(String),
    #[error("T-D2-1 invariant failure: {0}")]
    Invariant(String),
    #[error("T-D2-1 JSON failure: {0}")]
    Json(String),
    #[error("T-D2-1 create-new I/O failure: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_D2_1_DOMAIN_BUILD_V1_SCHEMA, domain, value))
        .expect("T-D2-1 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn metadata_number(value: usize) -> Td21TaggedNumber {
    Td21TaggedNumber {
        value,
        register: Td21Register::ArtifactMetadata,
    }
}

fn structural_number(value: usize) -> Td21TaggedNumber {
    Td21TaggedNumber {
        value,
        register: Td21Register::StructuralTestimony,
    }
}

fn semantic_number(value: usize) -> Td21TaggedNumber {
    Td21TaggedNumber {
        value,
        register: Td21Register::SemanticFamilyAuthority,
    }
}

fn utf8<'a>(path: &str, bytes: &'a [u8]) -> Result<&'a str, Td21Error> {
    std::str::from_utf8(bytes).map_err(|_| Td21Error::Input(format!("{path} is not valid UTF-8")))
}

fn assert_sha(path: &str, bytes: &[u8], expected: &str) -> Result<(), Td21Error> {
    let found = sha256(bytes);
    if found != expected {
        return Err(Td21Error::Input(format!(
            "sealed bytes drifted for {path}: expected {expected}, found {found}"
        )));
    }
    Ok(())
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> Td21SourceBinding {
    Td21SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: metadata_number(bytes.len()),
        blake3: bytes_hash(bytes),
        sha256: sha256(bytes),
        register: Td21Register::ArtifactMetadata,
    }
}

fn source_bindings() -> Vec<Td21SourceBinding> {
    [
        (
            "docs/depth_two_domain_adjudication.md",
            "adopted independent-domain and boundedness authority",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/schema2_e7_e8_bridge_v1.json",
            "sealed authoritative M-3 v1 baseline",
            M3_BYTES,
        ),
        (
            "docs/bc1_depth_two_completeness_v1.json",
            "sealed BC-1 stop that T-D2-1 must not silently promote",
            BC1_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "semantic Schema2 target and raw-height-proxy prohibition",
            OPEN_PROBLEM_BYTES,
        ),
        (
            "docs/e2_quotient_adjudications.md",
            "adopted natural-family and instance quotient law",
            QUOTIENT_ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-core/src/expr.rs",
            "frozen recursive kernel expression constructors",
            EXPR_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/check.rs",
            "secondary conservative scope checker",
            CHECK_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/elaborate.rs",
            "frozen sealed-signature kernel typing relation",
            ELABORATE_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/normalize.rs",
            "frozen terminating beta normalizer",
            NORMALIZE_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/equality.rs",
            "frozen adopted equality implementation",
            EQUALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/context.rs",
            "recursive typed schema-context surface testimony",
            CONTEXT_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/grammar.rs",
            "recursive schema grammar and family quotient implementation",
            GRAMMAR_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/e3_normalization.rs",
            "scoped normalization/equality audit with explicit totality limits",
            E3_SOURCE_BYTES,
        ),
        (
            "docs/t_bi_nu1_semantic_provenance_v6.json",
            "post-definition family regression observer only",
            FAMILY_CORPUS_BYTES,
        ),
        (
            "docs/a3_rule_inventory_exhaustiveness_v2.json",
            "post-definition membership-row regression observer only",
            A3_CORPUS_BYTES,
        ),
        (
            "crates/pen-search/src/t_d2_1_domain_build_v1.rs",
            "replay implementation and structural induction argument",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| source_binding(path, role, bytes))
    .collect()
}

fn authenticate_sealed_inputs() -> Result<(), Td21Error> {
    assert_sha(
        "docs/depth_two_domain_adjudication.md",
        ADJUDICATION_BYTES,
        EXPECTED_ADJUDICATION_SHA256,
    )?;
    assert_sha(
        "docs/schema2_e7_e8_bridge_v1.json",
        M3_BYTES,
        EXPECTED_M3_SHA256,
    )?;
    assert_sha(
        "docs/bc1_depth_two_completeness_v1.json",
        BC1_BYTES,
        EXPECTED_BC1_SHA256,
    )?;
    assert_sha(
        "docs/e2_quotient_adjudications.md",
        QUOTIENT_ADJUDICATION_BYTES,
        EXPECTED_QUOTIENT_ADJUDICATION_SHA256,
    )?;
    assert_sha(
        "docs/step_15_completion_open_problem.md",
        OPEN_PROBLEM_BYTES,
        EXPECTED_OPEN_PROBLEM_SHA256,
    )
}

fn build_m3_baseline() -> Result<Td21M3Baseline, Td21Error> {
    let json = utf8("docs/schema2_e7_e8_bridge_v1.json", M3_BYTES)?;
    let replay = replay_m3_e7_e8_bridge_v1_json(json);
    if !replay.valid {
        return Err(Td21Error::Baseline(format!(
            "public M-3 replay failed: {}",
            replay.errors.join("; ")
        )));
    }
    let certificate: M3E7E8BridgeV1Certificate =
        serde_json::from_str(json).map_err(|error| Td21Error::Json(error.to_string()))?;
    let expected_gap_present = certificate.e7.named_gap == M3_E7_GAP
        && certificate.open_gaps.iter().any(|gap| gap.id == M3_E7_GAP);
    let exact_enacted_regression_reproduced =
        certificate.enacted_regression.projections_exactly_equal
            && certificate
                .enacted_regression
                .e5_projection
                .semantic_o16_empty
            && certificate.enacted_regression.e5_projection.f1_excluded
            && certificate.enacted_regression.e5_projection.e5_complete;
    let baseline_shift_detected = certificate.result_digest != EXPECTED_M3_RESULT_DIGEST
        || certificate.m3_status != M3RunStatus::StoppedNamedGaps
        || certificate.e8.proved_bridge_condition_count.value != 4
        || certificate.e8.bridge_condition_count.value != 9
        || certificate.e7.c2_closed
        || !expected_gap_present
        || !certificate.zero_promotions_made
        || certificate.promoted_row_count.value != 0
        || certificate.bridge_claim_issued
        || certificate.m4_authorized
        || !exact_enacted_regression_reproduced;
    if baseline_shift_detected {
        return Err(Td21Error::Baseline(
            "M-3 v1 baseline shifted from its sealed fail-closed state".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "m3-baseline",
        &(
            &certificate.result_digest,
            exact_enacted_regression_reproduced,
            certificate.e8.proved_bridge_condition_count.value,
            certificate.e8.bridge_condition_count.value,
            expected_gap_present,
            certificate.zero_promotions_made,
            certificate.promoted_row_count.value,
        ),
    );
    Ok(Td21M3Baseline {
        result_digest: certificate.result_digest,
        public_replay_valid: replay.valid,
        public_replay_errors: replay.errors,
        status: certificate.m3_status,
        exact_enacted_regression_reproduced,
        proved_bridge_condition_count: structural_number(
            certificate.e8.proved_bridge_condition_count.value,
        ),
        bridge_condition_count: structural_number(certificate.e8.bridge_condition_count.value),
        c2_open: !certificate.e7.c2_closed,
        expected_c2_gap_present: expected_gap_present,
        zero_promotions_made: certificate.zero_promotions_made,
        promoted_row_count: metadata_number(certificate.promoted_row_count.value),
        bridge_claim_issued: certificate.bridge_claim_issued,
        m4_authorized: certificate.m4_authorized,
        baseline_shift_detected,
        derivation_hash,
    })
}

fn definition_input_paths() -> Vec<String> {
    vec![
        "docs/depth_two_domain_adjudication.md".to_owned(),
        "docs/step_15_completion_open_problem.md".to_owned(),
        "docs/e2_quotient_adjudications.md".to_owned(),
        "crates/pen-core/src/expr.rs".to_owned(),
        "crates/pen-type/src/check.rs".to_owned(),
        "crates/pen-type/src/elaborate.rs".to_owned(),
        "crates/pen-type/src/normalize.rs".to_owned(),
        "crates/pen-type/src/equality.rs".to_owned(),
        "crates/pen-schema/src/context.rs".to_owned(),
        "crates/pen-schema/src/grammar.rs".to_owned(),
        "crates/pen-schema/src/e3_normalization.rs".to_owned(),
    ]
}

fn observer_input_paths() -> Vec<String> {
    vec![
        "docs/t_bi_nu1_semantic_provenance_v6.json".to_owned(),
        "docs/a3_rule_inventory_exhaustiveness_v2.json".to_owned(),
    ]
}

fn build_firewall() -> Td21IndependenceFirewall {
    let definition_inputs = definition_input_paths();
    let observer_inputs = observer_input_paths();
    let definition_input_digest = tagged_hash(
        "definition-inputs",
        &(
            &definition_inputs,
            bytes_hash(ADJUDICATION_BYTES),
            bytes_hash(OPEN_PROBLEM_BYTES),
            bytes_hash(QUOTIENT_ADJUDICATION_BYTES),
            bytes_hash(EXPR_SOURCE_BYTES),
            bytes_hash(CHECK_SOURCE_BYTES),
            bytes_hash(ELABORATE_SOURCE_BYTES),
            bytes_hash(NORMALIZE_SOURCE_BYTES),
            bytes_hash(EQUALITY_SOURCE_BYTES),
            bytes_hash(CONTEXT_SOURCE_BYTES),
            bytes_hash(GRAMMAR_SOURCE_BYTES),
            bytes_hash(E3_SOURCE_BYTES),
        ),
    );
    let observer_input_digest = tagged_hash(
        "observer-inputs",
        &(
            &observer_inputs,
            bytes_hash(FAMILY_CORPUS_BYTES),
            bytes_hash(A3_CORPUS_BYTES),
        ),
    );
    let derivation_hash = tagged_hash(
        "independence-firewall",
        &(
            &definition_inputs,
            &observer_inputs,
            &definition_input_digest,
            &observer_input_digest,
            false,
            false,
            false,
            false,
            false,
            true,
        ),
    );
    Td21IndependenceFirewall {
        definition_inputs,
        observer_inputs,
        definition_input_digest,
        observer_input_digest,
        generator_inventory_reaches_domain_definition: false,
        a3_inventory_reaches_membership: false,
        bridge_rows_reach_enumeration: false,
        historical_corpora_seed_carrier: false,
        raw_ast_height_cap_used_as_depth_two_definition: false,
        definition_and_observer_dataflow_disjoint: true,
        f_d2_1_circularity_fired: false,
        f_d2_3_proxy_reentry_fired: false,
        derivation_hash,
    }
}

fn tower(former: Td21TowerFormer, depth: usize) -> Expr {
    (0..depth).fold(Expr::Univ, |inner, _| match former {
        Td21TowerFormer::Suspension => Expr::Susp(Box::new(inner)),
        Td21TowerFormer::Truncation => Expr::Trunc(Box::new(inner)),
    })
}

fn tower_depth(expr: &Expr, former: Td21TowerFormer) -> Option<usize> {
    match (former, expr) {
        (_, Expr::Univ) => Some(0),
        (Td21TowerFormer::Suspension, Expr::Susp(inner))
        | (Td21TowerFormer::Truncation, Expr::Trunc(inner)) => {
            tower_depth(inner, former).map(|depth| depth + 1)
        }
        _ => None,
    }
}

fn tower_prefix_row(
    signature: &SealedSignature,
    former: Td21TowerFormer,
    depth: usize,
) -> Result<Td21TowerPrefixRow, Td21Error> {
    let expr = tower(former, depth);
    let telescope = Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, expr.clone())]);
    let conservative_kernel_check_accepted =
        check_telescope(&Vec::new(), &telescope) == CheckResult::Ok;
    let elaboration = elaborate_telescope(signature, &telescope, 15)
        .map_err(|error| Td21Error::Invariant(format!("tower elaboration failed: {error}")))?;
    let elaborated = elaboration
        .clauses
        .first()
        .ok_or_else(|| Td21Error::Invariant("tower elaboration omitted its clause".to_owned()))?;
    let elaborated_kernel_type_json = serde_json::to_string(&elaborated.kernel_ty)
        .map_err(|error| Td21Error::Json(error.to_string()))?;
    let elaborated_normal_form_hash =
        tagged_hash("tower-elaborated-normal-form", &elaborated.normal_form);
    let elaborated_normal_form_identical = elaborated.normal_form == expr;
    let normalized = normalize(&expr, 0, 10_000)
        .map_err(|error| Td21Error::Invariant(format!("tower normalization failed: {error}")))?;
    let normalizer_returned_identical_expression = normalized.expr == expr;
    Ok(Td21TowerPrefixRow {
        former,
        depth: structural_number(depth),
        expression_hash: tagged_hash("tower-expression", &expr),
        conservative_kernel_check_accepted,
        sealed_signature_elaboration_accepted: true,
        elaborated_kernel_type_json,
        elaborated_normal_form_hash,
        elaborated_normal_form_identical,
        elaboration_derivation_hash: elaboration.derivation_hash,
        normalizer_returned_identical_expression,
        beta_normal: elaborated_normal_form_identical
            && normalizer_returned_identical_expression
            && normalized.steps == 0,
    })
}

fn tower_equality_prefix_regression(max_depth: usize) -> Result<bool, Td21Error> {
    for former in [Td21TowerFormer::Suspension, Td21TowerFormer::Truncation] {
        for left in 0..=max_depth {
            for right in 0..=max_depth {
                let witness =
                    univalent_equality(&tower(former, left), &tower(former, right), 0, 10_000)
                        .map_err(|error| {
                            Td21Error::Invariant(format!("tower equality decision failed: {error}"))
                        })?;
                if witness.equal != (left == right) {
                    return Ok(false);
                }
            }
        }
    }
    for left in 1..=max_depth {
        for right in 1..=max_depth {
            let witness = univalent_equality(
                &tower(Td21TowerFormer::Suspension, left),
                &tower(Td21TowerFormer::Truncation, right),
                0,
                10_000,
            )
            .map_err(|error| {
                Td21Error::Invariant(format!("cross-tower equality decision failed: {error}"))
            })?;
            if witness.equal {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

fn build_infinite_kernel_witness() -> Result<Td21InfiniteKernelSurfaceWitness, Td21Error> {
    // This prefix is an executable regression of the induction equations.  It
    // is not the proof of infinitude.  The proof is the constructor induction
    // recorded below: `Univ` is normal; normalize recurses under each former
    // without a head reduction; and Rust enum constructors are injective and
    // disjoint.  `tower_depth` is the replayable left inverse.
    let replayed_prefix_depth = 8usize;
    let mut prefix_rows = Vec::new();
    let signature = SealedSignature::genesis_del_h15();
    for former in [Td21TowerFormer::Suspension, Td21TowerFormer::Truncation] {
        for depth in 0..=replayed_prefix_depth {
            let row = tower_prefix_row(&signature, former, depth)?;
            if tower_depth(&tower(former, depth), former) != Some(depth) {
                return Err(Td21Error::Invariant(
                    "tower-depth left inverse failed".to_owned(),
                ));
            }
            prefix_rows.push(row);
        }
    }
    let every_prefix_row_kernel_accepted_and_normal = prefix_rows.iter().all(|row| {
        row.conservative_kernel_check_accepted
            && row.normalizer_returned_identical_expression
            && row.beta_normal
    });
    let every_prefix_row_elaborated_and_normal = prefix_rows.iter().all(|row| {
        row.sealed_signature_elaboration_accepted
            && row.elaborated_normal_form_identical
            && row.beta_normal
            && !row.elaborated_kernel_type_json.is_empty()
            && !row.elaboration_derivation_hash.is_empty()
    });
    let suspension_tower_pairwise_distinct = (0..=replayed_prefix_depth).all(|left| {
        (0..=replayed_prefix_depth).all(|right| {
            (tower(Td21TowerFormer::Suspension, left) == tower(Td21TowerFormer::Suspension, right))
                == (left == right)
        })
    });
    let truncation_tower_pairwise_distinct = (0..=replayed_prefix_depth).all(|left| {
        (0..=replayed_prefix_depth).all(|right| {
            (tower(Td21TowerFormer::Truncation, left) == tower(Td21TowerFormer::Truncation, right))
                == (left == right)
        })
    });
    let positive_depth_towers_constructor_disjoint = (1..=replayed_prefix_depth).all(|left| {
        (1..=replayed_prefix_depth).all(|right| {
            tower(Td21TowerFormer::Suspension, left) != tower(Td21TowerFormer::Truncation, right)
        })
    });
    let finite_prefix_adopted_equality_regression_passed =
        tower_equality_prefix_regression(replayed_prefix_depth)?;
    let base_case = "At depth zero both tower maps return Univ; elaboration against SealedSignature::genesis_del_h15() synthesizes its kernel classifier and normalize(Univ) = Univ."
        .to_owned();
    let normality_successor_case = "If T_n is beta-normal, normalize.rs maps Susp(T_n) to Susp(normalize(T_n)) and Trunc(T_n) to Trunc(normalize(T_n)); neither constructor has a head-reduction clause. Therefore both successors are beta-normal."
        .to_owned();
    let typing_successor_case = "If the closed inner term elaborates, elaborate.rs::synth_unary_former recursively synthesizes it and returns KernelTy::Type for Susp or Trunc; hence each finite successor elaborates against the same sealed signature. This is structural induction over n, not a bounded enumeration. check.rs is retained only as secondary conservative scope testimony."
        .to_owned();
    let distinctness_successor_case = "Expr derives structural Eq. Susp and Trunc are distinct enum variants and each Box payload constructor is injective. equality.rs defines both judgmental and frozen univalent equality as syntactic equality of beta normal forms. The replayed left inverse tower_depth maps the nth tower to n, so equality of same-former towers implies equal depths for arbitrary natural n; therefore the frozen equality quotient contains infinitely many classes."
        .to_owned();
    let quantified_proof_mechanization_scope = "The all-depth claim is a transparent source-level structural induction over the recursive `tower` and `tower_depth` definitions, source-bound into this certificate. Rust replay executes its equations on the published finite prefix and authenticates the defining kernel/elaborator/normalizer/equality source; it is not an exhaustive search over natural numbers and is not presented as a proof-assistant theorem."
        .to_owned();
    let derivation_hash = tagged_hash(
        "infinite-kernel-surface-witness",
        &(
            &prefix_rows,
            &base_case,
            &normality_successor_case,
            &typing_successor_case,
            &distinctness_successor_case,
            &quantified_proof_mechanization_scope,
            finite_prefix_adopted_equality_regression_passed,
            bytes_hash(EXPR_SOURCE_BYTES),
            bytes_hash(CHECK_SOURCE_BYTES),
            bytes_hash(ELABORATE_SOURCE_BYTES),
            bytes_hash(NORMALIZE_SOURCE_BYTES),
        ),
    );
    Ok(Td21InfiniteKernelSurfaceWitness {
        families: vec![
            "n |-> Susp^n(Univ)".to_owned(),
            "n |-> Trunc^n(Univ)".to_owned(),
        ],
        replayed_prefix_depth: structural_number(replayed_prefix_depth),
        prefix_rows,
        every_prefix_row_kernel_accepted_and_normal,
        every_prefix_row_elaborated_and_normal,
        base_case,
        normality_successor_case,
        typing_successor_case,
        distinctness_successor_case,
        quantified_proof_mechanization_scope,
        induction_quantifies_over_all_natural_depths: true,
        theorem_is_structural_induction_not_finite_search: true,
        suspension_tower_pairwise_distinct,
        truncation_tower_pairwise_distinct,
        positive_depth_towers_constructor_disjoint,
        adopted_equality_is_beta_normal_form_equality: true,
        finite_prefix_adopted_equality_regression_passed,
        literal_frozen_kernel_normal_surface_infinite: true,
        literal_frozen_kernel_equality_quotient_infinite: true,
        finite_prefix_used_only_as_replay_regression: true,
        derivation_hash,
    })
}

fn build_scoped_probe() -> Td21ScopedMachineryProbe {
    let derivation_hash = tagged_hash(
        "scoped-machinery-probe",
        &(
            bytes_hash(CHECK_SOURCE_BYTES),
            bytes_hash(ELABORATE_SOURCE_BYTES),
            bytes_hash(NORMALIZE_SOURCE_BYTES),
            bytes_hash(EQUALITY_SOURCE_BYTES),
            bytes_hash(QUOTIENT_ADJUDICATION_BYTES),
            bytes_hash(E3_SOURCE_BYTES),
            true,
            true,
            true,
            true,
            false,
            false,
            false,
            true,
            false,
        ),
    );
    Td21ScopedMachineryProbe {
        frozen_checker_accepts_recursive_unary_formers: true,
        frozen_sealed_signature_elaborator_types_recursive_unary_formers: true,
        frozen_normalizer_preserves_recursive_unary_formers: true,
        adopted_equality_source_authenticated: true,
        family_instance_quotient_adjudication_authenticated: true,
        typed_normalizer_total_over_all_intended_schema2_terms: false,
        general_univalent_equality_proved: false,
        arbitrary_context_presentation_quotient_proved: false,
        probe_is_negative_scope_testimony_only: true,
        general_limit_flags_are_source_bound_audit_not_new_theorem_replay: true,
        probe_reaches_domain_membership: false,
        derivation_hash,
    }
}

fn build_domain_attempt(
    witness: &Td21InfiniteKernelSurfaceWitness,
) -> Result<Td21DomainAttempt, Td21Error> {
    if !witness.literal_frozen_kernel_normal_surface_infinite
        || !witness.literal_frozen_kernel_equality_quotient_infinite
        || !witness.finite_prefix_adopted_equality_regression_passed
        || !witness.induction_quantifies_over_all_natural_depths
        || !witness.theorem_is_structural_induction_not_finite_search
        || !witness.every_prefix_row_elaborated_and_normal
    {
        return Err(Td21Error::Invariant(
            "unbounded-kernel witness did not replay".to_owned(),
        ));
    }
    let mut attempt = Td21DomainAttempt {
        intended_name: "NfSch2(H)".to_owned(),
        independent_definition_attempted_before_corpus_observation: true,
        fixed_depth_two_constructor_closure_operationally_presented: false,
        context_bound_operationally_presented: false,
        binder_bound_operationally_presented: false,
        public_symbol_bound_operationally_presented: false,
        cubical_dimension_bound_operationally_presented: false,
        literal_kernel_surface_is_candidate_domain: false,
        literal_kernel_surface_rejected_by_finiteness_obligation: true,
        generator_derived_surrogate_used: false,
        raw_ast_height_surrogate_used: false,
        nf_sch2_domain_operational: false,
        membership_decidable: false,
        typed_normalization_total_on_domain: false,
        adopted_family_quotient_decidable_on_domain: false,
        quotient_finite_proved: false,
        finite_enumerator_constructed: false,
        lawful_stop_taken: true,
        derivation_hash: String::new(),
    };
    attempt.derivation_hash = tagged_hash(
        "domain-attempt-before-observers",
        &(&witness.derivation_hash, &attempt),
    );
    Ok(attempt)
}

fn value_usize(value: &Value, key: &str, path: &str) -> Result<usize, Td21Error> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or_else(|| Td21Error::Input(format!("{path}.{key} is missing or not a usize")))
}

fn value_bool(value: &Value, key: &str, path: &str) -> Result<bool, Td21Error> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| Td21Error::Input(format!("{path}.{key} is missing or not a bool")))
}

fn value_str<'a>(value: &'a Value, key: &str, path: &str) -> Result<&'a str, Td21Error> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| Td21Error::Input(format!("{path}.{key} is missing or not a string")))
}

fn build_corpus_observer(
    domain_attempt: &Td21DomainAttempt,
) -> Result<Td21CorpusObserver, Td21Error> {
    if !domain_attempt.independent_definition_attempted_before_corpus_observation
        || !domain_attempt.lawful_stop_taken
    {
        return Err(Td21Error::Invariant(
            "observer opened before the independent domain attempt stopped".to_owned(),
        ));
    }
    // Deliberately authenticate and parse these bytes only after the
    // independent definition attempt above has reached its lawful stop.
    assert_sha(
        "docs/t_bi_nu1_semantic_provenance_v6.json",
        FAMILY_CORPUS_BYTES,
        EXPECTED_FAMILY_CORPUS_SHA256,
    )?;
    assert_sha(
        "docs/a3_rule_inventory_exhaustiveness_v2.json",
        A3_CORPUS_BYTES,
        EXPECTED_A3_CORPUS_SHA256,
    )?;
    let family: Value = serde_json::from_slice(FAMILY_CORPUS_BYTES)
        .map_err(|error| Td21Error::Json(error.to_string()))?;
    let a3: Value = serde_json::from_slice(A3_CORPUS_BYTES)
        .map_err(|error| Td21Error::Json(error.to_string()))?;
    let family_digest = value_str(&family, "result_digest", "t_bi_nu1_semantic_provenance_v6")?;
    let family_count = value_usize(
        &family,
        "proved_family_declaration_count",
        "t_bi_nu1_semantic_provenance_v6",
    )?;
    let a3_digest = value_str(&a3, "result_digest", "a3_rule_inventory_exhaustiveness_v2")?;
    let windows = a3
        .get("historical_windows")
        .and_then(Value::as_array)
        .ok_or_else(|| Td21Error::Input("A3 historical_windows missing".to_owned()))?;
    let final_window = windows
        .iter()
        .find(|window| window.get("stage").and_then(Value::as_u64) == Some(16))
        .ok_or_else(|| Td21Error::Input("A3 final historical window missing".to_owned()))?;
    let row_count = value_usize(
        final_window,
        "promoted_base_seed_count",
        "a3.historical_windows[stage=16]",
    )?;
    let relative = value_bool(
        &a3,
        "relative_rule_constructor_inventory_exhaustiveness_proved",
        "a3_rule_inventory_exhaustiveness_v2",
    )?;
    let absolute = value_bool(
        &a3,
        "broader_absolute_semantic_exhaustiveness_claimed",
        "a3_rule_inventory_exhaustiveness_v2",
    )?;
    if family_digest != EXPECTED_FAMILY_CORPUS_RESULT_DIGEST
        || family_count != 61
        || a3_digest != EXPECTED_A3_CORPUS_RESULT_DIGEST
        || row_count != 89
        || !relative
        || absolute
    {
        return Err(Td21Error::Input(
            "post-definition regression corpus shifted".to_owned(),
        ));
    }
    let exact_blocker = "NfSch2(H) membership is not formable until the fixed depth-two closure fragment and its total typed normalization/equality quotient are operationally specified. Therefore neither corpus is queried; this is regression-not-run, not a regression failure."
        .to_owned();
    let derivation_hash = tagged_hash(
        "post-definition-corpus-observer",
        &(
            family_digest,
            family_count,
            a3_digest,
            row_count,
            relative,
            absolute,
            &domain_attempt.derivation_hash,
            &exact_blocker,
            false,
            false,
            false,
            false,
        ),
    );
    Ok(Td21CorpusObserver {
        status: Td21CorpusRegressionStatus::BlockedByDomainGaps,
        opened_only_after_domain_attempt: true,
        family_corpus_authenticated: true,
        family_corpus_result_digest: family_digest.to_owned(),
        promised_family_count: semantic_number(family_count),
        a3_corpus_authenticated: true,
        a3_corpus_result_digest: a3_digest.to_owned(),
        promised_membership_row_count: semantic_number(row_count),
        a3_relative_exhaustiveness_proved: relative,
        a3_absolute_semantic_exhaustiveness_claimed: absolute,
        corpus_used_to_define_domain: false,
        corpus_used_to_seed_membership: false,
        corpus_used_to_seed_enumeration: false,
        membership_query_formable: false,
        regression_executed: false,
        every_promised_family_recognized: false,
        every_promised_row_recognized: false,
        regression_failure_claimed: false,
        exact_blocker,
        derivation_hash,
    })
}

fn named_gap(id: &str, phase: &str, exact_obstruction: &str) -> Td21NamedGap {
    let derivation_hash = tagged_hash(
        "named-gap",
        &(id, phase, exact_obstruction, true, true, true),
    );
    Td21NamedGap {
        id: id.to_owned(),
        phase: phase.to_owned(),
        exact_obstruction: exact_obstruction.to_owned(),
        keeps_t_d2_2_closed: true,
        keeps_m4_unauthorized: true,
        zero_charge: true,
        derivation_hash,
    }
}

fn certificate_digest(certificate: &Td21DomainBuildV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("domain-build-certificate", &projection)
}

fn build_certificate() -> Result<Td21DomainBuildV1Certificate, Td21Error> {
    authenticate_sealed_inputs()?;
    let m3_v1_baseline = build_m3_baseline()?;
    let infinite_kernel_surface_witness = build_infinite_kernel_witness()?;
    let scoped_machinery_probe = build_scoped_probe();
    let domain_attempt = build_domain_attempt(&infinite_kernel_surface_witness)?;
    // F-D2-1: this call is intentionally after `build_domain_attempt`.
    let corpus_observer = build_corpus_observer(&domain_attempt)?;
    // The firewall's observer digest is also constructed only after the
    // independent attempt; it never reaches that attempt's inputs.
    let independence_firewall = build_firewall();
    let open_gaps = vec![
        named_gap(
            TD21_FIXED_FRAGMENT_GAP,
            "independent carrier definition",
            "The adopted text names a fixed depth-two closure fragment but no generator-independent constructor closure, context/binder bound, public-symbol bound, or cubical-dimension bound operationally presents it. Raw AST height is expressly not the intended semantic Schema2 domain.",
        ),
        named_gap(
            TD21_MEMBERSHIP_GAP,
            "membership",
            "Without an operational fragment predicate there is no total yes/no judgment for membership in NfSch2(H). The conservative checker decides only its broader raw expression surface.",
        ),
        named_gap(
            TD21_NORMALIZATION_GAP,
            "typed normalization",
            "The frozen beta normalizer terminates on raw Expr inputs, but the registered E3 audit does not prove total typed normalization over every intended depth-two semantic schema, context presentation, or family reclassification.",
        ),
        named_gap(
            TD21_QUOTIENT_GAP,
            "adopted equality and family quotient",
            "Scoped frozen equality and family-instance decisions exist, but general univalent equality, arbitrary context-presentation quotienting, and total family reclassification over the intended domain are not proved decidable.",
        ),
        named_gap(
            TD21_FINITE_ENUMERATION_GAP,
            "boundedness",
            "The literal kernel-accepted quotient is infinite: the constructor-inductive Susp^n(Univ) and Trunc^n(Univ) families elaborate, are beta-normal, and remain pairwise distinct under the frozen equality, which is syntactic equality of beta normal forms. No independent depth-two restriction makes the intended quotient finite.",
        ),
        named_gap(
            TD21_REGRESSION_BLOCKED_GAP,
            "post-definition regression",
            "The sealed family and A3 corpora authenticate the promised regression population, but membership queries are unformable. The regression is not run and the corpora are never used to repair the definition.",
        ),
    ];
    let mut certificate = Td21DomainBuildV1Certificate {
        schema: T_D2_1_DOMAIN_BUILD_V1_SCHEMA.to_owned(),
        date: T_D2_1_DOMAIN_BUILD_V1_DATE.to_owned(),
        syntax_identifier_policy: "Digits in T-D2-1, NfSch2, M-3/M-4, E3, A3, stage labels, dates, versions, and gap IDs are syntax identifiers. Quantitative evidence is carried only by tagged numbers.".to_owned(),
        syntax_identifier_register: Td21Register::SyntaxIdentifier,
        source_bindings: source_bindings(),
        m3_v1_baseline,
        independence_firewall,
        infinite_kernel_surface_witness,
        scoped_machinery_probe,
        domain_attempt,
        corpus_observer,
        open_gap_count: metadata_number(open_gaps.len()),
        open_gaps,
        status: Td21RunStatus::StoppedNamedGaps,
        fixed_fragment_operationally_defined: false,
        nf_sch2_domain_operational: false,
        membership_decidable: false,
        quotient_finite_proved: false,
        quotient_finitely_enumerable: false,
        no_charge_minted: true,
        kappa_charge_minted: semantic_number(0),
        nu_charge_minted: semantic_number(0),
        anchor_charge_minted: semantic_number(0),
        t_d2_1_status: Td21RunStatus::StoppedNamedGaps,
        t_d2_2_prerequisite_satisfied: false,
        m3_v1_remains_authoritative: true,
        m4_authorized: false,
        mutation_falsifiers: vec![
            "change_any_sealed_input_or_M3_v1_baseline_then_replay_must_fail".to_owned(),
            "allow_any_generator_A3_or_bridge_row_dataflow_into_definition_membership_or_enumeration_then_replay_must_fail".to_owned(),
            "replace_the_semantic_fragment_with_a_raw_AST_height_cap_then_replay_must_fail".to_owned(),
            "claim_the_finite_prefix_search_proves_the_all_depth_tower_theorem_then_replay_must_fail".to_owned(),
            "remove_the_structural_induction_or_tower_depth_left_inverse_then_replay_must_fail".to_owned(),
            "claim_membership_total_or_the_quotient_finite_without_the_missing_fragment_then_replay_must_fail".to_owned(),
            "run_or_pass_the_corpus_regression_when_membership_is_unformable_then_replay_must_fail".to_owned(),
            "remove_or_rename_any_registered_gap_then_replay_must_fail".to_owned(),
            "mint_any_kappa_nu_or_anchor_charge_then_replay_must_fail".to_owned(),
            "authorize_T_D2_2_or_M4_then_replay_must_fail".to_owned(),
            "add_any_unknown_JSON_field_then_replay_must_fail".to_owned(),
        ],
        permitted_conclusion: "The adopted definition is independent in intent but not yet operative. The frozen raw kernel surface is not a finite substitute: structural induction yields infinitely many accepted beta-normal suspension and truncation towers. No generator-derived or raw-height proxy is introduced. The promised family and membership corpora are authenticated only after the failed definition attempt, and their regression is not run because membership is unformable. T-D2-1 stops with named gaps, zero charge, M-3 v1 authoritative, and M-4 unauthorized.".to_owned(),
        required_successor_action: "Provide a versioned, generator-independent operational presentation of the intended depth-two closure fragment, including constructor and context/binder/symbol/dimension bounds. Then prove total typed normalization and decidable adopted equality/family quotient on that carrier, prove finite enumeration, and only afterward rerun the authenticated family/row regression create-new.".to_owned(),
        result_digest: String::new(),
    };
    let expected_gap_ids = [
        TD21_FIXED_FRAGMENT_GAP,
        TD21_MEMBERSHIP_GAP,
        TD21_NORMALIZATION_GAP,
        TD21_QUOTIENT_GAP,
        TD21_FINITE_ENUMERATION_GAP,
        TD21_REGRESSION_BLOCKED_GAP,
    ];
    if certificate
        .open_gaps
        .iter()
        .map(|gap| gap.id.as_str())
        .ne(expected_gap_ids)
        || certificate.nf_sch2_domain_operational
        || certificate.fixed_fragment_operationally_defined
        || certificate.membership_decidable
        || certificate.quotient_finite_proved
        || certificate.quotient_finitely_enumerable
        || !certificate.no_charge_minted
        || certificate.kappa_charge_minted.value != 0
        || certificate.nu_charge_minted.value != 0
        || certificate.anchor_charge_minted.value != 0
        || certificate.t_d2_2_prerequisite_satisfied
        || !certificate.m3_v1_remains_authoritative
        || certificate.m4_authorized
        || certificate.corpus_observer.regression_executed
        || certificate.corpus_observer.corpus_used_to_define_domain
        || !certificate
            .infinite_kernel_surface_witness
            .literal_frozen_kernel_normal_surface_infinite
        || !certificate
            .infinite_kernel_surface_witness
            .literal_frozen_kernel_equality_quotient_infinite
    {
        return Err(Td21Error::Invariant(
            "T-D2-1 fail-closed exit invariant failed".to_owned(),
        ));
    }
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<Result<Td21DomainBuildV1Certificate, String>> =
    OnceLock::new();

fn expected_certificate() -> Result<&'static Td21DomainBuildV1Certificate, Td21Error> {
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(Td21Error::Input(error.clone())),
    }
}

pub fn issue_t_d2_1_domain_build_v1() -> Result<Td21DomainBuildV1Certificate, Td21Error> {
    expected_certificate().cloned()
}

fn invalid_replay(error: impl Into<String>) -> Td21Replay {
    Td21Replay {
        valid: false,
        errors: vec![error.into()],
        t_d2_1_status: Td21RunStatus::StoppedNamedGaps,
        nf_sch2_domain_operational: false,
        t_d2_2_prerequisite_satisfied: false,
        m4_authorized: false,
    }
}

pub fn replay_t_d2_1_domain_build_v1(claimed: &Td21DomainBuildV1Certificate) -> Td21Replay {
    let expected = match expected_certificate() {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-D2-1 result digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("T-D2-1 source bindings drifted".to_owned());
    }
    if claimed != expected {
        errors.push("T-D2-1 certificate differs from deterministic reissuance".to_owned());
    }
    Td21Replay {
        valid: errors.is_empty(),
        errors,
        t_d2_1_status: claimed.t_d2_1_status,
        nf_sch2_domain_operational: claimed.nf_sch2_domain_operational,
        t_d2_2_prerequisite_satisfied: claimed.t_d2_2_prerequisite_satisfied,
        m4_authorized: claimed.m4_authorized,
    }
}

pub fn replay_t_d2_1_domain_build_v1_json(json: &str) -> Td21Replay {
    match serde_json::from_str::<Td21DomainBuildV1Certificate>(json) {
        Ok(certificate) => replay_t_d2_1_domain_build_v1(&certificate),
        Err(error) => invalid_replay(format!("invalid T-D2-1 JSON: {error}")),
    }
}

fn register_label(register: Td21Register) -> &'static str {
    match register {
        Td21Register::SemanticFamilyAuthority => "semantic_family_authority",
        Td21Register::StructuralTestimony => "structural_testimony",
        Td21Register::ArtifactMetadata => "artifact_metadata",
        Td21Register::SyntaxIdentifier => "syntax_identifier",
    }
}

fn render_number(number: &Td21TaggedNumber) -> String {
    format!(
        "{} [register: `{}`]",
        number.value,
        register_label(number.register)
    )
}

pub fn render_t_d2_1_domain_build_v1(certificate: &Td21DomainBuildV1Certificate) -> String {
    let mut out = String::new();
    out.push_str("# T-D2-1 independent depth-two domain build result\n\n");
    out.push_str(&format!(
        "**Date (syntax identifier):** {}. **Status:** `stopped_named_gaps`. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("## Outcome\n\n");
    out.push_str("The independently intended domain is not operational. The adopted record does not provide the generator-independent closure and context bounds needed to decide membership, and the literal frozen kernel surface cannot serve as the finite carrier.\n\n");
    out.push_str("## Replayable unbounded-surface theorem\n\n");
    out.push_str(&format!(
        "A finite prefix through depth **{}** replays the two induction equations for `Susp^n(Univ)` and `Trunc^n(Univ)` against `SealedSignature::genesis_del_h15()`. The theorem itself is structural induction over every natural depth: the sealed-signature elaborator's unary-former rule recursively types each constructor, the normalizer preserves it without head reduction, structural equality makes each constructor injective, and `tower_depth` is a left inverse. The conservative checker is secondary scope testimony; the finite prefix is only a regression fixture.\n\n",
        render_number(
            &certificate
                .infinite_kernel_surface_witness
                .replayed_prefix_depth
        )
    ));
    out.push_str(
        &certificate
            .infinite_kernel_surface_witness
            .quantified_proof_mechanization_scope,
    );
    out.push_str("\n\n");
    out.push_str("The frozen univalent equality is beta-normal-form equality, so the constructor-inductive left inverse also proves that the literal kernel equality quotient has infinitely many classes. This does not identify that raw surface with intended Schema2; it proves that an additional independently specified fragment is necessary.\n\n");
    out.push_str("## Independence and corpus gate\n\n");
    out.push_str(&format!(
        "The historical corpora authenticate **{}** proved families and **{}** A3 membership rows, both as post-definition observers only. Since membership is unformable, the regression status is `blocked_by_domain_gaps`: it was not run and no regression failure is claimed.\n\n",
        render_number(&certificate.corpus_observer.promised_family_count),
        render_number(&certificate.corpus_observer.promised_membership_row_count),
    ));
    out.push_str("No generator inventory, A3 row, bridge row, raw AST height cap, score, or enumeration order enters the carrier, membership test, or enumeration attempt.\n\n");
    out.push_str("The general-normalization, general-univalence, and arbitrary-context quotient negatives are source-bound scope declarations from the existing audit, not newly replayed universal theorems. They support the named gaps; they are not used as a substitute domain decision.\n\n");
    out.push_str("## Named gaps\n\n");
    out.push_str("| Gap | Exact obstruction |\n|---|---|\n");
    for gap in &certificate.open_gaps {
        out.push_str(&format!("| `{}` | {} |\n", gap.id, gap.exact_obstruction));
    }
    out.push_str(&format!(
        "\nOpen gaps: **{}**. Kappa charge: **{}**. Nu charge: **{}**. Anchor charge: **{}**. T-D2-2 prerequisite satisfied: **{}**. M-4 authorized: **{}**.\n\n",
        render_number(&certificate.open_gap_count),
        render_number(&certificate.kappa_charge_minted),
        render_number(&certificate.nu_charge_minted),
        render_number(&certificate.anchor_charge_minted),
        certificate.t_d2_2_prerequisite_satisfied,
        certificate.m4_authorized,
    ));
    out.push_str("## Permitted conclusion\n\n");
    out.push_str(&certificate.permitted_conclusion);
    out.push_str("\n\n## Required successor\n\n");
    out.push_str(&certificate.required_successor_action);
    out.push('\n');
    out
}

pub fn emit_t_d2_1_domain_build_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Td21DomainBuildV1Certificate, Td21Error> {
    if certificate_path.exists() || report_path.exists() {
        return Err(Td21Error::Io(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_t_d2_1_domain_build_v1()?;
    let replay = replay_t_d2_1_domain_build_v1(&certificate);
    if !replay.valid {
        return Err(Td21Error::Invariant(format!(
            "new T-D2-1 certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Td21Error::Json(error.to_string()))?;
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(certificate_path)
        .map_err(|error| Td21Error::Io(error.to_string()))?;
    let certificate_write = certificate_file
        .write_all(&json)
        .and_then(|_| certificate_file.write_all(b"\n"));
    drop(certificate_file);
    if let Err(error) = certificate_write {
        let _ = remove_file(certificate_path);
        return Err(Td21Error::Io(error.to_string()));
    }
    let report = render_t_d2_1_domain_build_v1(&certificate);
    let report_write = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(report_path)
        .and_then(|mut file| file.write_all(report.as_bytes()));
    if let Err(error) = report_write {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(Td21Error::Io(error.to_string()));
    }
    let emitted = read_to_string(certificate_path).map_err(|error| {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        Td21Error::Io(error.to_string())
    })?;
    let emitted_replay = replay_t_d2_1_domain_build_v1_json(&emitted);
    if !emitted_replay.valid {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(Td21Error::Invariant(format!(
            "emitted T-D2-1 JSON did not replay: {}",
            emitted_replay.errors.join("; ")
        )));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_d2_1_replays_and_stops_with_exact_gaps() {
        let certificate = issue_t_d2_1_domain_build_v1().expect("T-D2-1 issues");
        let replay = replay_t_d2_1_domain_build_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(certificate.t_d2_1_status, Td21RunStatus::StoppedNamedGaps);
        assert!(!certificate.nf_sch2_domain_operational);
        assert!(!certificate.membership_decidable);
        assert!(!certificate.quotient_finite_proved);
        assert!(!certificate.t_d2_2_prerequisite_satisfied);
        assert!(!certificate.m4_authorized);
        assert_eq!(
            certificate
                .open_gaps
                .iter()
                .map(|gap| gap.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                TD21_FIXED_FRAGMENT_GAP,
                TD21_MEMBERSHIP_GAP,
                TD21_NORMALIZATION_GAP,
                TD21_QUOTIENT_GAP,
                TD21_FINITE_ENUMERATION_GAP,
                TD21_REGRESSION_BLOCKED_GAP,
            ]
        );
    }

    #[test]
    fn tower_theorem_has_induction_and_prefix_replay() {
        let certificate = issue_t_d2_1_domain_build_v1().expect("T-D2-1 issues");
        let witness = &certificate.infinite_kernel_surface_witness;
        assert!(witness.every_prefix_row_kernel_accepted_and_normal);
        assert!(witness.every_prefix_row_elaborated_and_normal);
        assert!(witness.induction_quantifies_over_all_natural_depths);
        assert!(witness.theorem_is_structural_induction_not_finite_search);
        assert!(witness.suspension_tower_pairwise_distinct);
        assert!(witness.truncation_tower_pairwise_distinct);
        assert!(witness.positive_depth_towers_constructor_disjoint);
        assert!(witness.adopted_equality_is_beta_normal_form_equality);
        assert!(witness.finite_prefix_adopted_equality_regression_passed);
        assert!(witness.literal_frozen_kernel_normal_surface_infinite);
        assert!(witness.literal_frozen_kernel_equality_quotient_infinite);
        assert!(witness.finite_prefix_used_only_as_replay_regression);
    }

    #[test]
    fn corpus_is_observer_only_and_regression_is_not_run() {
        let certificate = issue_t_d2_1_domain_build_v1().expect("T-D2-1 issues");
        let firewall = &certificate.independence_firewall;
        let observer = &certificate.corpus_observer;
        assert!(firewall.definition_and_observer_dataflow_disjoint);
        assert!(!firewall.historical_corpora_seed_carrier);
        assert!(!firewall.a3_inventory_reaches_membership);
        assert!(!observer.corpus_used_to_define_domain);
        assert!(!observer.corpus_used_to_seed_membership);
        assert!(!observer.corpus_used_to_seed_enumeration);
        assert!(!observer.membership_query_formable);
        assert!(!observer.regression_executed);
        assert!(!observer.regression_failure_claimed);
        assert_eq!(
            observer.status,
            Td21CorpusRegressionStatus::BlockedByDomainGaps
        );
    }

    #[test]
    fn verdict_firewall_charge_gap_and_json_mutations_fail() {
        let certificate = issue_t_d2_1_domain_build_v1().expect("T-D2-1 issues");

        let mut domain = certificate.clone();
        domain.nf_sch2_domain_operational = true;
        assert!(!replay_t_d2_1_domain_build_v1(&domain).valid);

        let mut membership = certificate.clone();
        membership.membership_decidable = true;
        assert!(!replay_t_d2_1_domain_build_v1(&membership).valid);

        let mut finite = certificate.clone();
        finite.quotient_finite_proved = true;
        assert!(!replay_t_d2_1_domain_build_v1(&finite).valid);

        let mut circular = certificate.clone();
        circular.corpus_observer.corpus_used_to_seed_membership = true;
        assert!(!replay_t_d2_1_domain_build_v1(&circular).valid);

        let mut proxy = certificate.clone();
        proxy
            .independence_firewall
            .raw_ast_height_cap_used_as_depth_two_definition = true;
        assert!(!replay_t_d2_1_domain_build_v1(&proxy).valid);

        let mut regression = certificate.clone();
        regression.corpus_observer.regression_executed = true;
        assert!(!replay_t_d2_1_domain_build_v1(&regression).valid);

        let mut charge = certificate.clone();
        charge.nu_charge_minted.value = 1;
        assert!(!replay_t_d2_1_domain_build_v1(&charge).valid);

        let mut gap = certificate.clone();
        gap.open_gaps[0].id.push_str("_MUTATED");
        assert!(!replay_t_d2_1_domain_build_v1(&gap).valid);

        let mut source = certificate.clone();
        source.source_bindings[0].sha256.push('0');
        assert!(!replay_t_d2_1_domain_build_v1(&source).valid);

        let mut digest = certificate.clone();
        digest.result_digest.push('0');
        assert!(!replay_t_d2_1_domain_build_v1(&digest).valid);

        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("certificate object")
            .insert("unknown_field".to_owned(), Value::Bool(true));
        let replay =
            replay_t_d2_1_domain_build_v1_json(&serde_json::to_string(&value).expect("JSON"));
        assert!(!replay.valid);
    }
}
