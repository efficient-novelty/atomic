//! Source-first T-BI-NU1 and F-AL1-prime successor.
//!
//! The exact Stage-1-through-15 candidate prefix is the only pre-seal input.
//! The prefix-local v5 issuer constructs B1/B2 packages and emits its real
//! issuance trace.  T-BI-B3 v3 then validates that trace by deterministic
//! reissuance.  Only after that intrinsic surface is sealed does this module
//! open the existing operational/structural comparator.  No desired semantic
//! vector, historical v5 package, or B3 v2 token is accepted as authority.

use crate::act_local_semantic_provenance_v5::V5PrefixLocalSemanticSequence;
use crate::t_bi_intrinsic_isolation_v3::{
    TBiIntrinsicIsolationV3Token, issue_replayed_t_bi_intrinsic_isolation_v3_context,
};
use crate::t_bi_nu1_regression_v4::{
    OperationalRegressionV4, issue_t_bi_nu1_regression_v4_certificate,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const T_BI_NU1_REGRESSION_V6_SCHEMA: &str = "t-bi-nu1-source-first-real-trace-regression-v6";
pub const T_BI_NU1_REGRESSION_V6_DATE: &str = "2026-07-22";
pub const T_BI_NU1_REGRESSION_V6_THEOREM_ID: &str =
    "T-BI-NU1-v6-source-first-B1-B2-real-trace-B3-F-AL1-prime";
pub const T_BI_NU1_V6_CERTIFICATE_NAME: &str = "t_bi_nu1_semantic_provenance_v6.json";
pub const T_BI_NU1_V6_REPORT_NAME: &str = "T_BI_NU1_SEMANTIC_PROVENANCE_V6_RESULT.md";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_NU1_REGRESSION_V6_SCHEMA, domain, value))
        .expect("T-BI v6 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn reference_entries() -> Vec<(u32, Telescope)> {
    (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect()
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BareRegisterRowV6 {
    pub stage: u32,
    pub structural: u32,
    pub semantic: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiPassedOperationalRowV6 {
    pub stage: u32,
    pub winner_candidate_hash: String,
    pub winner_admissible_under_recorded_regime: bool,
    pub operational_row_exact: bool,
}

/// Minimal positive projection intended for a later BI-0 v6 join.
///
/// It contains no full semantic packages, receipt graph, historical package,
/// score, bar, verdict, or desired-vector field.  Consumers can bind it back
/// to the deterministic certificate by `source_certificate_result_digest`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionV6PassedProjection {
    pub schema: String,
    pub theorem_id: String,
    pub source_certificate_result_digest: String,
    pub exact_prefix_candidate_hashes: Vec<String>,
    pub exact_prefix_candidate_hash_digest: String,
    pub stage1_through3_typed_prefix_entries: Vec<(u32, Telescope)>,
    pub stage1_through3_prefix_digest: String,
    pub authoritative_prefix_semantic_seal: String,
    pub intrinsic_sequence_derivation_hash: String,
    pub intrinsic_issuance_trace_root: String,
    pub intrinsic_isolation_theorem_id: String,
    pub intrinsic_isolation_derivation_hash: String,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub named_registry_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub total_named_residual_count: usize,
    pub silent_residue_count: usize,
    pub semantic_register: Vec<u32>,
    pub structural_register: Vec<u32>,
    pub bare_register_table: Vec<BareRegisterRowV6>,
    pub operational_rows: Vec<TBiPassedOperationalRowV6>,
    pub extraction_complete: bool,
    pub exact_operational_regression: bool,
    pub f_al1_prime_passed: bool,
    pub non_enacted_branch_work_executed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionV6Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub intrinsic_sequence: V5PrefixLocalSemanticSequence,
    pub intrinsic_sequence_derivation_hash: String,
    pub intrinsic_sequence_seal: String,
    pub intrinsic_isolation: TBiIntrinsicIsolationV3Token,
    pub intrinsic_isolation_replayed: bool,
    pub isolated_sequence_reissued_exactly: bool,
    pub preseal_completed_before_postseal_inputs: bool,
    pub package_count: usize,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub decided_role_declaration_count: usize,
    pub named_registry_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub total_named_residual_count: usize,
    pub silent_residue_count: usize,
    pub credited_semantic_family_count: usize,
    pub package_role_accounting_exact: bool,
    pub package_semantic_accounting_exact: bool,
    pub semantic_vector_derived_from_packages: Vec<u32>,
    pub semantic_vector_matches_sequence_projection: bool,
    pub t_bi_b1_proved: bool,
    pub t_bi_b2_proved: bool,
    pub t_bi_b3_proved: bool,
    pub special_cases_proved: bool,
    pub extraction_complete: bool,
    pub postseal_v4_result_digest: String,
    pub operational: OperationalRegressionV4,
    pub structural_register: Vec<u32>,
    pub authoritative_semantic_register: Option<Vec<u32>>,
    pub non_authoritative_extraction_floor: Option<Vec<u32>>,
    pub register_table: Option<Vec<BareRegisterRowV6>>,
    pub register_table_is_bare_and_exact: bool,
    pub structural_semantic_scalar_equality_required: bool,
    pub f_al1_prime_passed: bool,
    pub t_bi_nu1_proved_on_enacted_branch: bool,
    pub bi0_rerun_authorized_by_t_bi_nu1_side: bool,
    pub non_enacted_branch_work_executed: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionV6Replay {
    pub valid: bool,
    pub theorem_proved: bool,
    pub bi0_rerun_authorized: bool,
    pub extraction_complete: bool,
    pub operational_regression_exact: bool,
    pub f_al1_prime_passed: bool,
    pub semantic_register: Option<Vec<u32>>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiNu1RegressionV6Error {
    #[error("T-BI v6 source-first issuance failed: {0}")]
    Intrinsic(String),
    #[error("T-BI-B3 v3 isolation failed: {0}")]
    Isolation(String),
    #[error("T-BI v6 post-seal regression failed: {0}")]
    PostSeal(String),
    #[error("T-BI v6 positive projection unavailable: {0}")]
    Projection(String),
    #[error("T-BI v6 JSON failed: {0}")]
    Json(String),
    #[error("T-BI v6 I/O failed: {0}")]
    Io(String),
    #[error("emitted T-BI v6 artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn certificate_digest(certificate: &TBiNu1RegressionV6Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("source-first-regression-certificate", &projection)
}

fn passed_projection_digest(projection: &TBiNu1RegressionV6PassedProjection) -> String {
    let mut hash_projection = projection.clone();
    hash_projection.derivation_hash.clear();
    tagged_hash("BI0-passed-projection", &hash_projection)
}

fn register_table_is_exact(rows: &[BareRegisterRowV6]) -> bool {
    rows.len() == 15
        && rows.iter().map(|row| row.stage).eq(1..=15)
        && rows.iter().all(|row| {
            serde_json::to_value(row)
                .ok()
                .and_then(|value| value.as_object().cloned())
                .is_some_and(|object| {
                    object.len() == 3
                        && object.contains_key("stage")
                        && object.contains_key("structural")
                        && object.contains_key("semantic")
                })
        })
}

pub fn issue_t_bi_nu1_regression_v6_certificate()
-> Result<TBiNu1RegressionV6Certificate, TBiNu1RegressionV6Error> {
    let entries = reference_entries();

    // Intrinsic phase.  The combined B3 context issues the exact candidate
    // prefix once as authority and once as an independent replay, then
    // compares the complete sequence and token evidence before returning the
    // first issuance.  No caller-supplied trace or package vector is accepted.
    let intrinsic_context = issue_replayed_t_bi_intrinsic_isolation_v3_context(&entries)
        .map_err(|error| TBiNu1RegressionV6Error::Isolation(error.to_string()))?;
    let intrinsic_isolation_replayed = intrinsic_context.proved
        && intrinsic_context.replay_errors.is_empty()
        && intrinsic_context.sequence_evidence_equal
        && intrinsic_context.token_evidence_equal;
    let isolated_sequence_reissued_exactly = intrinsic_context.sequence_evidence_equal;
    let intrinsic_sequence = intrinsic_context.sequence;
    let intrinsic_isolation = intrinsic_context.token;
    let intrinsic_sequence_derivation_hash = intrinsic_sequence.derivation_hash.clone();
    let intrinsic_sequence_seal = intrinsic_sequence.authoritative_sequence_seal.clone();

    // Derive every semantic scalar and declaration total from the source-first
    // package vector.  There is deliberately no expected total or vector.
    let package_count = intrinsic_sequence.packages.len();
    let role_declaration_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.role_declaration_count)
        .sum::<usize>();
    let proved_family_declaration_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.proved_family_declaration_count)
        .sum::<usize>();
    let theorem_impossibility_declaration_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.theorem_impossibility_declaration_count)
        .sum::<usize>();
    let decided_role_declaration_count =
        proved_family_declaration_count + theorem_impossibility_declaration_count;
    let named_registry_residual_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.named_role_residual_count)
        .sum::<usize>();
    let named_quotient_residual_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.named_quotient_residual_count)
        .sum::<usize>();
    let named_a3_residual_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.named_a3_residual_count)
        .sum::<usize>();
    let total_named_residual_count =
        named_registry_residual_count + named_quotient_residual_count + named_a3_residual_count;
    let silent_residue_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.silent_residue_count)
        .sum::<usize>();
    let credited_semantic_family_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.credited_family_ids.len())
        .sum::<usize>();
    let package_role_accounting_exact = intrinsic_sequence.packages.iter().all(|package| {
        package.role_declaration_count
            == package.proved_family_declaration_count
                + package.theorem_impossibility_declaration_count
                + package.named_role_residual_count
            && package.role_rows.len() == package.role_declaration_count
    });
    let package_semantic_accounting_exact = intrinsic_sequence.packages.iter().all(|package| {
        package.semantic_nu as usize == package.credited_family_ids.len()
            && package.semantic_nu as usize
                == package
                    .family_rows
                    .iter()
                    .filter(|family| family.credited)
                    .count()
    });
    let semantic_vector_derived_from_packages = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.semantic_nu)
        .collect::<Vec<_>>();
    let semantic_vector_matches_sequence_projection =
        semantic_vector_derived_from_packages == intrinsic_sequence.semantic_nu_vector;
    let t_bi_b1_proved = intrinsic_sequence.t_bi_b1_proved_on_sequence
        && intrinsic_sequence
            .packages
            .iter()
            .all(|package| package.t_bi_b1_proved);
    let t_bi_b2_proved = intrinsic_sequence.t_bi_b2_proved_on_sequence
        && intrinsic_sequence
            .packages
            .iter()
            .all(|package| package.t_bi_b2_proved);
    let t_bi_b3_proved = intrinsic_isolation_replayed
        && isolated_sequence_reissued_exactly
        && intrinsic_isolation.prefix_len == 15
        && intrinsic_isolation.last_stage == 15
        && intrinsic_isolation.issuance_receipt_count == 107
        && intrinsic_isolation.prefix_local_sequence_replayed
        && intrinsic_isolation.every_local_package_proved_b1_b2
        && intrinsic_isolation.every_registry_extension_projection_equal
        && intrinsic_isolation.exact_one_rule_seven_operations_per_stage_and_sequence_seal
        && intrinsic_isolation.every_receipt_replayed_at_its_topological_position
        && intrinsic_isolation.every_predecessor_hash_bound
        && intrinsic_isolation.every_capability_derived_from_operation
        && intrinsic_isolation.exact_package_source_v3_grammar_predecessor_commitments
        && intrinsic_isolation.cross_stage_prefix_accumulator_dependencies_exact
        && intrinsic_isolation.issuance_trace_root_exact
        && intrinsic_isolation.every_node_reaches_sequence_seal
        && intrinsic_isolation.no_caller_receipt_parameter
        && intrinsic_isolation.no_historical_registry_or_legacy_v5_authority
        && intrinsic_isolation.no_archive_structural_bar_verdict_or_future_input
        && intrinsic_isolation.prefix_generic_transitive_isolation_proved;
    let special_cases_proved = intrinsic_sequence
        .packages
        .iter()
        .all(|package| package.special_cases_proved);
    let exact_stage_surface = package_count == 15
        && intrinsic_sequence
            .packages
            .iter()
            .map(|package| package.stage)
            .eq(1..=15);
    let extraction_complete = exact_stage_surface
        && role_declaration_count == decided_role_declaration_count
        && named_registry_residual_count == 0
        && named_quotient_residual_count == 0
        && named_a3_residual_count == 0
        && total_named_residual_count == 0
        && silent_residue_count == 0
        && package_role_accounting_exact
        && package_semantic_accounting_exact
        && semantic_vector_matches_sequence_projection
        && t_bi_b1_proved
        && t_bi_b2_proved
        && t_bi_b3_proved
        && special_cases_proved
        && intrinsic_sequence.every_package_closed_observed_grammar
        && intrinsic_sequence.every_package_registry_extension_invariant
        && intrinsic_sequence.no_historical_registry_or_future_input;
    let preseal_completed_before_postseal_inputs = extraction_complete;

    // Post-seal phase.  The existing operational replay and its archived
    // structural register are inaccessible until every intrinsic value above
    // has been derived and sealed.
    let postseal = issue_t_bi_nu1_regression_v4_certificate()
        .map_err(|error| TBiNu1RegressionV6Error::PostSeal(error.to_string()))?;
    let postseal_v4_result_digest = postseal.result_digest.clone();
    let operational = postseal.operational.clone();
    let structural_register = postseal.registers.structural_vector.clone();
    let authoritative_semantic_register =
        extraction_complete.then(|| semantic_vector_derived_from_packages.clone());
    let non_authoritative_extraction_floor =
        (!extraction_complete).then(|| semantic_vector_derived_from_packages.clone());
    let register_table = authoritative_semantic_register
        .as_ref()
        .and_then(|semantic| {
            (structural_register.len() == 15 && semantic.len() == 15).then(|| {
                (1..=15)
                    .zip(structural_register.iter().copied())
                    .zip(semantic.iter().copied())
                    .map(|((stage, structural), semantic)| BareRegisterRowV6 {
                        stage,
                        structural,
                        semantic,
                    })
                    .collect::<Vec<_>>()
            })
        });
    let register_table_is_bare_and_exact = register_table
        .as_deref()
        .is_some_and(register_table_is_exact);
    let structural_semantic_scalar_equality_required = false;
    let f_al1_prime_passed = extraction_complete
        && operational.exact_operational_regression
        && register_table_is_bare_and_exact
        && !structural_semantic_scalar_equality_required;
    let t_bi_nu1_proved_on_enacted_branch = f_al1_prime_passed;
    let bi0_rerun_authorized_by_t_bi_nu1_side = f_al1_prime_passed;
    let non_enacted_branch_work_executed = false;

    let mut certificate = TBiNu1RegressionV6Certificate {
        schema: T_BI_NU1_REGRESSION_V6_SCHEMA.to_owned(),
        date: T_BI_NU1_REGRESSION_V6_DATE.to_owned(),
        theorem_id: T_BI_NU1_REGRESSION_V6_THEOREM_ID.to_owned(),
        intrinsic_sequence,
        intrinsic_sequence_derivation_hash,
        intrinsic_sequence_seal,
        intrinsic_isolation,
        intrinsic_isolation_replayed,
        isolated_sequence_reissued_exactly,
        preseal_completed_before_postseal_inputs,
        package_count,
        role_declaration_count,
        proved_family_declaration_count,
        theorem_impossibility_declaration_count,
        decided_role_declaration_count,
        named_registry_residual_count,
        named_quotient_residual_count,
        named_a3_residual_count,
        total_named_residual_count,
        silent_residue_count,
        credited_semantic_family_count,
        package_role_accounting_exact,
        package_semantic_accounting_exact,
        semantic_vector_derived_from_packages,
        semantic_vector_matches_sequence_projection,
        t_bi_b1_proved,
        t_bi_b2_proved,
        t_bi_b3_proved,
        special_cases_proved,
        extraction_complete,
        postseal_v4_result_digest,
        operational,
        structural_register,
        authoritative_semantic_register,
        non_authoritative_extraction_floor,
        register_table,
        register_table_is_bare_and_exact,
        structural_semantic_scalar_equality_required,
        f_al1_prime_passed,
        t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized_by_t_bi_nu1_side,
        non_enacted_branch_work_executed,
        outcome: if f_al1_prime_passed {
            "T_BI_NU1_V6_SOURCE_FIRST_F_AL1_PRIME_PASSED"
        } else {
            "T_BI_NU1_V6_SOURCE_FIRST_HONEST_NEGATIVE"
        }
        .to_owned(),
        permitted_conclusion: if f_al1_prime_passed {
            "The exact fifteen-stage source-first v5 construction closes B1/B2, its real 107-receipt trace closes B3 v3, all declaration and semantic totals are derived from those packages without an expected vector, and F-AL1-prime opens the T-BI side of BI-0 v6."
        } else {
            "At least one source-first extraction, real-trace isolation, or post-seal operational gate is false. The semantic register remains non-authoritative and BI-0 v6 remains closed."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> TBiNu1RegressionV6Replay {
    TBiNu1RegressionV6Replay {
        valid: false,
        theorem_proved: false,
        bi0_rerun_authorized: false,
        extraction_complete: false,
        operational_regression_exact: false,
        f_al1_prime_passed: false,
        semantic_register: None,
        errors: vec![error.into()],
    }
}

pub fn replay_t_bi_nu1_regression_v6_certificate(
    claimed: &TBiNu1RegressionV6Certificate,
) -> TBiNu1RegressionV6Replay {
    if claimed.result_digest != certificate_digest(claimed) {
        return TBiNu1RegressionV6Replay {
            valid: false,
            theorem_proved: false,
            bi0_rerun_authorized: false,
            extraction_complete: false,
            operational_regression_exact: false,
            f_al1_prime_passed: false,
            semantic_register: None,
            errors: vec!["T-BI v6 certificate digest mismatch".to_owned()],
        };
    }
    let expected = match issue_t_bi_nu1_regression_v6_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed != &expected {
        errors.push(
            "T-BI v6 certificate differs from deterministic create-new reissuance".to_owned(),
        );
    }
    TBiNu1RegressionV6Replay {
        valid: errors.is_empty(),
        theorem_proved: claimed.t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized: claimed.bi0_rerun_authorized_by_t_bi_nu1_side,
        extraction_complete: claimed.extraction_complete,
        operational_regression_exact: claimed.operational.exact_operational_regression,
        f_al1_prime_passed: claimed.f_al1_prime_passed,
        semantic_register: claimed.authoritative_semantic_register.clone(),
        errors,
    }
}

pub fn replay_t_bi_nu1_regression_v6_json(json: &str) -> TBiNu1RegressionV6Replay {
    match serde_json::from_str::<TBiNu1RegressionV6Certificate>(json) {
        Ok(certificate) => replay_t_bi_nu1_regression_v6_certificate(&certificate),
        Err(error) => failed_replay(error.to_string()),
    }
}

impl TBiNu1RegressionV6Certificate {
    /// Return the narrow BI-0 prerequisite only after deterministic replay of
    /// this certificate.  Internally issued certificates use the cheaper
    /// dedicated projection issuer below.
    pub fn passed_projection(
        &self,
    ) -> Result<TBiNu1RegressionV6PassedProjection, TBiNu1RegressionV6Error> {
        let replay = replay_t_bi_nu1_regression_v6_certificate(self);
        if !replay.valid {
            return Err(TBiNu1RegressionV6Error::Projection(
                replay.errors.join("; "),
            ));
        }
        project_passed_unchecked(self)
    }
}

fn project_passed_unchecked(
    certificate: &TBiNu1RegressionV6Certificate,
) -> Result<TBiNu1RegressionV6PassedProjection, TBiNu1RegressionV6Error> {
    if !certificate.f_al1_prime_passed
        || !certificate.bi0_rerun_authorized_by_t_bi_nu1_side
        || !certificate.extraction_complete
    {
        return Err(TBiNu1RegressionV6Error::Projection(
            "F-AL1-prime did not produce a positive BI-0 prerequisite".to_owned(),
        ));
    }
    let semantic_register = certificate
        .authoritative_semantic_register
        .clone()
        .ok_or_else(|| {
            TBiNu1RegressionV6Error::Projection(
                "authoritative semantic register is absent".to_owned(),
            )
        })?;
    let bare_register_table = certificate.register_table.clone().ok_or_else(|| {
        TBiNu1RegressionV6Error::Projection("bare register table is absent".to_owned())
    })?;
    let operational_rows = certificate
        .operational
        .rows
        .iter()
        .map(|row| TBiPassedOperationalRowV6 {
            stage: row.stage,
            winner_candidate_hash: row.winner_candidate_hash.clone(),
            winner_admissible_under_recorded_regime: row.winner_admissible_under_recorded_regime,
            operational_row_exact: row.operational_row_exact,
        })
        .collect::<Vec<_>>();
    let exact_prefix_candidate_hashes = certificate
        .intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.candidate_hash.clone())
        .collect::<Vec<_>>();
    let exact_prefix_candidate_hash_digest = tagged_hash(
        "exact-stage1-through15-candidate-hash-vector",
        &exact_prefix_candidate_hashes,
    );
    let stage1_through3_typed_prefix_entries =
        reference_entries().into_iter().take(3).collect::<Vec<_>>();
    let stage1_through3_prefix_digest = tagged_hash(
        "exact-stage1-through3-typed-prefix",
        &stage1_through3_typed_prefix_entries,
    );
    let mut projection = TBiNu1RegressionV6PassedProjection {
        schema: T_BI_NU1_REGRESSION_V6_SCHEMA.to_owned(),
        theorem_id: T_BI_NU1_REGRESSION_V6_THEOREM_ID.to_owned(),
        source_certificate_result_digest: certificate.result_digest.clone(),
        exact_prefix_candidate_hashes,
        exact_prefix_candidate_hash_digest,
        stage1_through3_typed_prefix_entries,
        stage1_through3_prefix_digest,
        authoritative_prefix_semantic_seal: certificate.intrinsic_sequence_seal.clone(),
        intrinsic_sequence_derivation_hash: certificate.intrinsic_sequence_derivation_hash.clone(),
        intrinsic_issuance_trace_root: certificate.intrinsic_sequence.issuance_trace_root.clone(),
        intrinsic_isolation_theorem_id: certificate.intrinsic_isolation.theorem_id.clone(),
        intrinsic_isolation_derivation_hash: certificate
            .intrinsic_isolation
            .derivation_hash
            .clone(),
        role_declaration_count: certificate.role_declaration_count,
        proved_family_declaration_count: certificate.proved_family_declaration_count,
        theorem_impossibility_declaration_count: certificate
            .theorem_impossibility_declaration_count,
        named_registry_residual_count: certificate.named_registry_residual_count,
        named_quotient_residual_count: certificate.named_quotient_residual_count,
        named_a3_residual_count: certificate.named_a3_residual_count,
        total_named_residual_count: certificate.total_named_residual_count,
        silent_residue_count: certificate.silent_residue_count,
        semantic_register,
        structural_register: certificate.structural_register.clone(),
        bare_register_table,
        operational_rows,
        extraction_complete: certificate.extraction_complete,
        exact_operational_regression: certificate.operational.exact_operational_regression,
        f_al1_prime_passed: true,
        non_enacted_branch_work_executed: certificate.non_enacted_branch_work_executed,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = passed_projection_digest(&projection);
    Ok(projection)
}

pub fn issue_t_bi_nu1_regression_v6_passed_projection()
-> Result<TBiNu1RegressionV6PassedProjection, TBiNu1RegressionV6Error> {
    let certificate = issue_t_bi_nu1_regression_v6_certificate()?;
    project_passed_unchecked(&certificate)
}

pub fn replay_t_bi_nu1_regression_v6_passed_projection(
    claimed: &TBiNu1RegressionV6PassedProjection,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != passed_projection_digest(claimed) {
        errors.push("T-BI v6 passed projection digest mismatch".to_owned());
    }
    match issue_t_bi_nu1_regression_v6_passed_projection() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push(
            "T-BI v6 passed projection differs from deterministic certificate projection"
                .to_owned(),
        ),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

pub fn render_t_bi_nu1_regression_v6_report(certificate: &TBiNu1RegressionV6Certificate) -> String {
    let table = certificate.register_table.as_ref().map_or_else(
        || {
            "The semantic register and divergence table are suppressed because source-first extraction is incomplete."
                .to_owned()
        },
        |rows| {
            let body = rows
                .iter()
                .map(|row| format!("| {} | {} | {} |", row.stage, row.structural, row.semantic))
                .collect::<Vec<_>>()
                .join("\n");
            format!("| Stage | Structural | Semantic |\n|---:|---:|---:|\n{body}")
        },
    );
    format!(
        "# T-BI-NU1 source-first semantic-family provenance v6 result\n\n**Date:** {}. **Outcome:** `{}`.\n\nThe exact fifteen-stage prefix issued **{}** source-first semantic packages and **{}** real issuance receipts. T-BI-B1/B2/B3: **{}/{}/{}**. Role declarations: **{}**; proved-family relations: **{}**; theorem-backed impossibilities: **{}**; named role/quotient/A3/total residuals: **{}/{}/{}/{}**; silent residue: **{}**. Package role accounting: **{}**; semantic accounting: **{}**; extraction complete: **{}**.\n\nNo desired semantic vector or expected role total is an input. The semantic register shown below is the package-derived vector, compared only after the intrinsic sequence and B3 v3 receipt closure were sealed. Scalar equality with the structural register is not required.\n\n{}\n\nExact operational regression: **{}**. F-AL1-prime: **{}**.\n\nPermitted conclusion: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.package_count,
        certificate.intrinsic_isolation.issuance_receipt_count,
        certificate.t_bi_b1_proved,
        certificate.t_bi_b2_proved,
        certificate.t_bi_b3_proved,
        certificate.role_declaration_count,
        certificate.proved_family_declaration_count,
        certificate.theorem_impossibility_declaration_count,
        certificate.named_registry_residual_count,
        certificate.named_quotient_residual_count,
        certificate.named_a3_residual_count,
        certificate.total_named_residual_count,
        certificate.silent_residue_count,
        certificate.package_role_accounting_exact,
        certificate.package_semantic_accounting_exact,
        certificate.extraction_complete,
        table,
        certificate.operational.exact_operational_regression,
        certificate.f_al1_prime_passed,
        certificate.permitted_conclusion,
        certificate.result_digest,
    )
}

pub fn emit_t_bi_nu1_regression_v6_create_new(
    directory: &Path,
) -> Result<TBiNu1RegressionV6Certificate, TBiNu1RegressionV6Error> {
    let certificate = issue_t_bi_nu1_regression_v6_certificate()?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| TBiNu1RegressionV6Error::Json(error.to_string()))?;
    let replay = replay_t_bi_nu1_regression_v6_json(&json);
    if !replay.valid {
        return Err(TBiNu1RegressionV6Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let certificate_path = directory.join(T_BI_NU1_V6_CERTIFICATE_NAME);
    let report_path = directory.join(T_BI_NU1_V6_REPORT_NAME);
    if certificate_path.exists() || report_path.exists() {
        return Err(TBiNu1RegressionV6Error::Io(format!(
            "create-new target already exists: {} or {}",
            certificate_path.display(),
            report_path.display()
        )));
    }
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| TBiNu1RegressionV6Error::Io(error.to_string()))?;
    certificate_file
        .write_all(json.as_bytes())
        .map_err(|error| TBiNu1RegressionV6Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| TBiNu1RegressionV6Error::Io(error.to_string()))?;
    report_file
        .write_all(render_t_bi_nu1_regression_v6_report(&certificate).as_bytes())
        .map_err(|error| TBiNu1RegressionV6Error::Io(error.to_string()))?;
    Ok(certificate)
}

pub fn replay_t_bi_nu1_regression_v6_directory(
    directory: &Path,
) -> Result<TBiNu1RegressionV6Replay, TBiNu1RegressionV6Error> {
    let json = std::fs::read_to_string(directory.join(T_BI_NU1_V6_CERTIFICATE_NAME))
        .map_err(|error| TBiNu1RegressionV6Error::Io(error.to_string()))?;
    Ok(replay_t_bi_nu1_regression_v6_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::t_bi_intrinsic_isolation_v3::{
        T_BI_B3_V3_THEOREM_ID, T_BI_INTRINSIC_ISOLATION_V3_SCHEMA,
    };
    use std::sync::OnceLock;

    fn cached_certificate() -> &'static TBiNu1RegressionV6Certificate {
        static CERTIFICATE: OnceLock<TBiNu1RegressionV6Certificate> = OnceLock::new();
        CERTIFICATE.get_or_init(|| {
            issue_t_bi_nu1_regression_v6_certificate().expect("cached T-BI v6 certificate")
        })
    }

    #[test]
    fn source_first_real_trace_closes_f_al1_prime_without_an_expected_vector() {
        let certificate = cached_certificate();
        assert_eq!(certificate.package_count, 15);
        assert_eq!(certificate.intrinsic_isolation.issuance_receipt_count, 107);
        assert_eq!(
            certificate.intrinsic_isolation.schema,
            T_BI_INTRINSIC_ISOLATION_V3_SCHEMA
        );
        assert_eq!(
            certificate.intrinsic_isolation.theorem_id,
            T_BI_B3_V3_THEOREM_ID
        );
        assert_eq!(
            certificate.role_declaration_count,
            certificate.proved_family_declaration_count
                + certificate.theorem_impossibility_declaration_count
        );
        assert_eq!(certificate.named_registry_residual_count, 0);
        assert_eq!(certificate.named_quotient_residual_count, 0);
        assert_eq!(certificate.named_a3_residual_count, 0);
        assert_eq!(certificate.total_named_residual_count, 0);
        assert_eq!(certificate.silent_residue_count, 0);
        assert!(certificate.package_role_accounting_exact);
        assert!(certificate.package_semantic_accounting_exact);
        assert!(certificate.semantic_vector_matches_sequence_projection);
        assert!(certificate.t_bi_b1_proved);
        assert!(certificate.t_bi_b2_proved);
        assert!(certificate.t_bi_b3_proved);
        assert!(certificate.preseal_completed_before_postseal_inputs);
        assert!(certificate.extraction_complete);
        assert!(certificate.register_table_is_bare_and_exact);
        assert!(!certificate.structural_semantic_scalar_equality_required);
        assert!(certificate.f_al1_prime_passed);
        assert!(
            replay_t_bi_nu1_regression_v6_certificate(&certificate)
                .errors
                .is_empty()
        );
    }

    #[test]
    fn bare_rows_have_only_stage_structural_and_semantic_fields() {
        let certificate = cached_certificate();
        let rows = certificate
            .register_table
            .as_ref()
            .expect("authoritative table");
        assert_eq!(rows.len(), 15);
        assert!(rows.iter().all(|row| {
            serde_json::to_value(row)
                .expect("row JSON")
                .as_object()
                .is_some_and(|object| object.len() == 3)
        }));
    }

    #[test]
    fn fully_rehashed_semantic_vector_forgery_fails_deterministic_replay() {
        let mut certificate = cached_certificate().clone();
        certificate.semantic_vector_derived_from_packages[0] ^= 1;
        certificate.result_digest = certificate_digest(&certificate);
        let replay = replay_t_bi_nu1_regression_v6_certificate(&certificate);
        assert!(!replay.valid);
        assert!(
            replay
                .errors
                .iter()
                .any(|error| error.contains("reissuance"))
        );
    }

    #[test]
    fn passed_projection_is_minimal_content_addressed_and_replayable() {
        let projection = project_passed_unchecked(cached_certificate()).expect("passed projection");
        assert_eq!(projection.exact_prefix_candidate_hashes.len(), 15);
        assert_eq!(projection.stage1_through3_typed_prefix_entries.len(), 3);
        assert_eq!(projection.semantic_register.len(), 15);
        assert_eq!(projection.structural_register.len(), 15);
        assert_eq!(projection.bare_register_table.len(), 15);
        assert_eq!(projection.operational_rows.len(), 15);
        assert!(projection.exact_operational_regression);
        assert!(projection.f_al1_prime_passed);
        assert!(!projection.non_enacted_branch_work_executed);
        assert_eq!(
            projection.derivation_hash,
            passed_projection_digest(&projection)
        );
        let object = serde_json::to_value(&projection)
            .expect("projection JSON")
            .as_object()
            .cloned()
            .expect("projection object");
        assert!(!object.contains_key("intrinsic_sequence"));
        assert!(!object.contains_key("issuance_receipts"));
        assert!(!object.contains_key("desired_semantic_vector"));
        assert!(!object.contains_key("score"));
        assert!(!object.contains_key("bar"));
        assert!(!object.contains_key("verdict"));
    }

    #[test]
    fn unknown_certificate_fields_are_rejected() {
        let mut value = serde_json::to_value(cached_certificate()).expect("certificate JSON");
        value.as_object_mut().expect("certificate object").insert(
            "desired_semantic_vector".to_owned(),
            serde_json::json!([1, 2, 3]),
        );
        let replay = replay_t_bi_nu1_regression_v6_json(
            &serde_json::to_string(&value).expect("mutated JSON"),
        );
        assert!(!replay.valid);
        assert!(
            replay
                .errors
                .iter()
                .any(|error| error.contains("unknown field"))
        );
    }
}
