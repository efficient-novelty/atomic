//! Post-seal F-AL1-prime regression for the additive B1/B2/B3 theorem chain.
//!
//! The complete v5 intrinsic sequence and its transitive capability token are
//! issued before the structural register or operational comparator is opened.
//! Structural and semantic values are then printed as a bare, uninterpreted
//! table; equality between the two registers is neither tested nor required.

use crate::act_local_semantic_provenance_v5::{
    ActLocalSemanticSequenceV5, V5AnchorDisposition, issue_act_local_semantic_sequence_v5,
};
use crate::t_bi_intrinsic_isolation_v2::{
    TBiIntrinsicIsolationV2Token, issue_t_bi_intrinsic_isolation_v2,
    replay_t_bi_intrinsic_isolation_v2,
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

pub const T_BI_NU1_REGRESSION_V5_SCHEMA: &str = "t-bi-nu1-semantic-family-regression-v5-b1-b2-b3";
pub const T_BI_NU1_REGRESSION_V5_DATE: &str = "2026-07-22";
pub const T_BI_NU1_REGRESSION_V5_THEOREM_ID: &str =
    "T-BI-NU1-v5-F-AL1-prime-complete-semantic-family-regression";
pub const T_BI_NU1_V5_CERTIFICATE_NAME: &str = "t_bi_nu1_semantic_provenance_v5.json";
pub const T_BI_NU1_V5_REPORT_NAME: &str = "T_BI_NU1_SEMANTIC_PROVENANCE_V5_RESULT.md";

const NU_REGISTER_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/nu_register_adjudication.md");
const SUPPORT_COMPREHENSION_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/support_comprehension_adjudication.md");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_NU1_REGRESSION_V5_SCHEMA, domain, value))
        .expect("T-BI v5 regression evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BareRegisterRowV5 {
    pub stage: u32,
    pub structural: u32,
    pub semantic: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionV5Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub nu_register_adjudication_hash: String,
    pub support_comprehension_adjudication_hash: String,
    pub intrinsic_sequence: ActLocalSemanticSequenceV5,
    pub intrinsic_sequence_seal: String,
    pub intrinsic_isolation: TBiIntrinsicIsolationV2Token,
    pub intrinsic_isolation_replayed: bool,
    pub preseal_completed_before_postseal_inputs: bool,
    pub postseal_v4_result_digest: String,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub named_registry_residual_count: usize,
    pub named_quotient_residual_count: usize,
    pub named_a3_residual_count: usize,
    pub total_named_residual_count: usize,
    pub silent_residue_count: usize,
    pub credited_semantic_family_count: usize,
    pub theorem_anchor_impossibility_count: usize,
    pub t_bi_b1_proved: bool,
    pub t_bi_b2_proved: bool,
    pub t_bi_b3_proved: bool,
    pub stage1_exception_proved: bool,
    pub stage2_constitutive_question_settled: bool,
    pub stage9_boundary_case_settled: bool,
    pub extraction_complete: bool,
    pub operational: OperationalRegressionV4,
    pub structural_register: Vec<u32>,
    pub authoritative_semantic_register: Option<Vec<u32>>,
    pub non_authoritative_extraction_floor: Option<Vec<u32>>,
    pub register_table: Option<Vec<BareRegisterRowV5>>,
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
pub struct TBiNu1RegressionV5Replay {
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
pub enum TBiNu1RegressionV5Error {
    #[error("T-BI v5 intrinsic issuance failed: {0}")]
    Intrinsic(String),
    #[error("T-BI-B3 isolation failed: {0}")]
    Isolation(String),
    #[error("T-BI v5 post-seal regression failed: {0}")]
    PostSeal(String),
    #[error("T-BI v5 invariant failed: {0}")]
    Invariant(String),
    #[error("T-BI v5 JSON failed: {0}")]
    Json(String),
    #[error("T-BI v5 I/O failed: {0}")]
    Io(String),
    #[error("emitted T-BI v5 artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn replay_adoptions() -> Result<(), TBiNu1RegressionV5Error> {
    let nu = std::str::from_utf8(NU_REGISTER_ADJUDICATION_BYTES)
        .map_err(|error| TBiNu1RegressionV5Error::Invariant(error.to_string()))?;
    let support = std::str::from_utf8(SUPPORT_COMPREHENSION_ADJUDICATION_BYTES)
        .map_err(|error| TBiNu1RegressionV5Error::Invariant(error.to_string()))?;
    if !nu.contains("nu-register-split-v1")
        || !nu.contains("F-AL1")
        || !nu.contains("Scalar equality with the archive is")
        || !support.contains("T-BI-B1")
        || !support.contains("T-BI-B2")
        || !support.contains("T-BI-B3")
        || !support.contains("## ADOPTION")
    {
        return Err(TBiNu1RegressionV5Error::Invariant(
            "the ν-register and support-comprehension adoptions did not replay".to_owned(),
        ));
    }
    Ok(())
}

fn certificate_digest(certificate: &TBiNu1RegressionV5Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("T-BI-v5-regression-certificate", &projection)
}

pub fn issue_t_bi_nu1_regression_v5_certificate()
-> Result<TBiNu1RegressionV5Certificate, TBiNu1RegressionV5Error> {
    replay_adoptions()?;

    // Everything above this line in the resulting certificate is intrinsic.
    // The capability theorem is completed before the v4 post-seal comparator
    // is opened below.
    let entries = (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    let intrinsic_sequence = issue_act_local_semantic_sequence_v5(&entries)
        .map_err(|error| TBiNu1RegressionV5Error::Intrinsic(error.to_string()))?;
    let intrinsic_isolation = issue_t_bi_intrinsic_isolation_v2(&entries, &intrinsic_sequence)
        .map_err(|error| TBiNu1RegressionV5Error::Isolation(error.to_string()))?;
    let intrinsic_isolation_errors =
        replay_t_bi_intrinsic_isolation_v2(&entries, &intrinsic_sequence, &intrinsic_isolation);
    let intrinsic_isolation_replayed = intrinsic_isolation_errors.is_empty();
    if !intrinsic_isolation_replayed {
        return Err(TBiNu1RegressionV5Error::Isolation(
            intrinsic_isolation_errors.join("; "),
        ));
    }
    let intrinsic_sequence_seal = intrinsic_sequence.intrinsic_sequence_seal.clone();
    let preseal_completed_before_postseal_inputs = intrinsic_isolation.exact_fifteen_stage_surface
        && intrinsic_isolation.exact_typed_phase_surface
        && intrinsic_isolation.exact_package_commitments_recomputed
        && intrinsic_isolation.exact_sequence_commitments_recomputed
        && intrinsic_isolation.every_predecessor_hash_bound
        && intrinsic_isolation.every_capability_derived_from_operation
        && intrinsic_isolation.every_node_reaches_sequence_seal
        && intrinsic_isolation.no_forbidden_capability_in_transitive_closure
        && !intrinsic_isolation.source_scan_used_as_proof
        && !intrinsic_isolation.runtime_self_report_used_as_proof
        && !intrinsic_isolation.synthetic_receipt_input_accepted
        && intrinsic_isolation.transitive_call_graph_isolation_proved;

    // Post-seal only: this replays the operational history and opens the
    // structural register as comparator testimony.
    let postseal = issue_t_bi_nu1_regression_v4_certificate()
        .map_err(|error| TBiNu1RegressionV5Error::PostSeal(error.to_string()))?;
    let postseal_v4_result_digest = postseal.result_digest.clone();
    let operational = postseal.operational.clone();
    let structural_register = postseal.registers.structural_vector.clone();

    let role_declaration_count = intrinsic_sequence.role_declaration_count;
    let proved_family_declaration_count = intrinsic_sequence.proved_family_declaration_count;
    let theorem_impossibility_declaration_count =
        intrinsic_sequence.theorem_impossibility_declaration_count;
    let named_registry_residual_count = intrinsic_sequence.named_role_residual_count;
    let named_quotient_residual_count = intrinsic_sequence.named_quotient_residual_count;
    let named_a3_residual_count = intrinsic_sequence.named_a3_residual_count;
    let total_named_residual_count = intrinsic_sequence.total_named_residual_count;
    let silent_residue_count = intrinsic_sequence.silent_residue_count;
    let credited_semantic_family_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.credited_semantic_family_count)
        .sum();
    let theorem_anchor_impossibility_count = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.theorem_anchor_impossibility_count)
        .sum();
    let t_bi_b1_proved = intrinsic_sequence.t_bi_b1_proved_on_sequence;
    let t_bi_b2_proved = intrinsic_sequence.t_bi_b2_proved_on_sequence;
    let t_bi_b3_proved = intrinsic_isolation_replayed && preseal_completed_before_postseal_inputs;
    let stage1_exception_proved = intrinsic_sequence
        .packages
        .first()
        .is_some_and(|package| package.stage == 1 && package.stage1_r1_preserved);
    let stage2_constitutive_question_settled =
        intrinsic_sequence.packages.get(1).is_some_and(|package| {
            package.stage == 2 && package.stage2_constitutive_question_not_assumed
        });
    let stage9_boundary_case_settled = intrinsic_sequence.packages.get(8).is_some_and(|package| {
        package.stage == 9 && package.stage9_boundary_decided_by_relation_theorem
    });
    let every_anchor_closed = intrinsic_sequence.packages.iter().all(|package| {
        package.every_marginal_family_credited_or_theorem_impossible
            && package.local_anchor_nonreuse_holds
            && package.semantic_families.iter().all(|family| {
                !family.marginal
                    || match &family.anchor {
                        V5AnchorDisposition::CreditedLocalRole { .. } => true,
                        V5AnchorDisposition::TheoremImpossibleNoRelation {
                            no_constructed_exported_a3_fallback,
                            ..
                        }
                        | V5AnchorDisposition::TheoremImpossibleNonFunctionalRelation {
                            no_constructed_exported_a3_fallback,
                            ..
                        } => *no_constructed_exported_a3_fallback,
                        V5AnchorDisposition::TheoremImpossibleRelationCollision {
                            no_constructed_exported_a3_fallback,
                            ..
                        } => *no_constructed_exported_a3_fallback,
                        V5AnchorDisposition::NamedResidual { .. } => false,
                        V5AnchorDisposition::Internal => false,
                    }
            })
    });
    let extraction_complete = intrinsic_sequence.packages.len() == 15
        && role_declaration_count == 250
        && proved_family_declaration_count + theorem_impossibility_declaration_count == 250
        && named_registry_residual_count
            == role_declaration_count.saturating_sub(
                proved_family_declaration_count + theorem_impossibility_declaration_count,
            )
        && named_registry_residual_count == 0
        && named_quotient_residual_count == 0
        && named_a3_residual_count == 0
        && total_named_residual_count
            == named_registry_residual_count
                + named_quotient_residual_count
                + named_a3_residual_count
        && total_named_residual_count == 0
        && silent_residue_count == 0
        && t_bi_b1_proved
        && t_bi_b2_proved
        && t_bi_b3_proved
        && stage1_exception_proved
        && stage2_constitutive_question_settled
        && stage9_boundary_case_settled
        && every_anchor_closed
        && preseal_completed_before_postseal_inputs;
    let semantic_floor = intrinsic_sequence
        .packages
        .iter()
        .map(|package| package.semantic_family_nu)
        .collect::<Vec<_>>();
    let authoritative_semantic_register = extraction_complete.then(|| semantic_floor.clone());
    let non_authoritative_extraction_floor = (!extraction_complete).then_some(semantic_floor);
    let register_table = authoritative_semantic_register.as_ref().map(|semantic| {
        (1..=15)
            .zip(structural_register.iter().copied())
            .zip(semantic.iter().copied())
            .map(|((stage, structural), semantic)| BareRegisterRowV5 {
                stage,
                structural,
                semantic,
            })
            .collect::<Vec<_>>()
    });
    let register_table_is_bare_and_exact = register_table.as_ref().is_some_and(|table| {
        table.len() == 15
            && table.iter().map(|row| row.stage).eq(1..=15)
            && table.iter().all(|row| {
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
    });
    let structural_semantic_scalar_equality_required = false;
    let f_al1_prime_passed = extraction_complete
        && operational.exact_operational_regression
        && register_table_is_bare_and_exact
        && !structural_semantic_scalar_equality_required;
    let t_bi_nu1_proved_on_enacted_branch = f_al1_prime_passed;
    let bi0_rerun_authorized_by_t_bi_nu1_side = f_al1_prime_passed;
    let non_enacted_branch_work_executed = false;

    let mut certificate = TBiNu1RegressionV5Certificate {
        schema: T_BI_NU1_REGRESSION_V5_SCHEMA.to_owned(),
        date: T_BI_NU1_REGRESSION_V5_DATE.to_owned(),
        theorem_id: T_BI_NU1_REGRESSION_V5_THEOREM_ID.to_owned(),
        nu_register_adjudication_hash: bytes_hash(NU_REGISTER_ADJUDICATION_BYTES),
        support_comprehension_adjudication_hash: bytes_hash(
            SUPPORT_COMPREHENSION_ADJUDICATION_BYTES,
        ),
        intrinsic_sequence,
        intrinsic_sequence_seal,
        intrinsic_isolation,
        intrinsic_isolation_replayed,
        preseal_completed_before_postseal_inputs,
        postseal_v4_result_digest,
        role_declaration_count,
        proved_family_declaration_count,
        theorem_impossibility_declaration_count,
        named_registry_residual_count,
        named_quotient_residual_count,
        named_a3_residual_count,
        total_named_residual_count,
        silent_residue_count,
        credited_semantic_family_count,
        theorem_anchor_impossibility_count,
        t_bi_b1_proved,
        t_bi_b2_proved,
        t_bi_b3_proved,
        stage1_exception_proved,
        stage2_constitutive_question_settled,
        stage9_boundary_case_settled,
        extraction_complete,
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
            "T_BI_NU1_V5_F_AL1_PRIME_PASSED"
        } else {
            "T_BI_NU1_V5_HONEST_NEGATIVE"
        }
        .to_owned(),
        permitted_conclusion: if f_al1_prime_passed {
            "T-BI-B1, T-BI-B2, and T-BI-B3 replay over the enacted fifteen-act surface; all 250 declarations and every marginal anchor are decided without silent residue. F-AL1-prime passes and the T-BI side of BI-0 is reopened."
        } else {
            "At least one proof-bearing extraction or isolation gate remains false. No semantic register or divergence table is issued and the T-BI side of BI-0 remains closed."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> TBiNu1RegressionV5Replay {
    TBiNu1RegressionV5Replay {
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

pub fn replay_t_bi_nu1_regression_v5_certificate(
    claimed: &TBiNu1RegressionV5Certificate,
) -> TBiNu1RegressionV5Replay {
    let expected = match issue_t_bi_nu1_regression_v5_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-BI v5 certificate digest mismatch".to_owned());
    }
    if claimed != &expected {
        errors.push("T-BI v5 certificate differs from create-new reissuance".to_owned());
    }
    TBiNu1RegressionV5Replay {
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

pub fn replay_t_bi_nu1_regression_v5_json(json: &str) -> TBiNu1RegressionV5Replay {
    match serde_json::from_str::<TBiNu1RegressionV5Certificate>(json) {
        Ok(certificate) => replay_t_bi_nu1_regression_v5_certificate(&certificate),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn render_t_bi_nu1_regression_v5_report(certificate: &TBiNu1RegressionV5Certificate) -> String {
    let table = certificate.register_table.as_ref().map_or_else(
        || {
            "F-NR3 suppresses the semantic register and F-NR4 suppresses the register table because extraction is incomplete.".to_owned()
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
        "# T-BI-NU1 semantic-family provenance v5 result\n\n**Date:** {}. **Outcome:** `{}`.\n\nT-BI-B1/B2/B3: **{}/{}/{}**. Role declarations: **{}**; proved-family relations: **{}**; theorem-backed impossibilities: **{}**; named role/quotient/A3/total residuals: **{}/{}/{}/{}**; silent residue: **{}**. Extraction complete: **{}**. Exact operational regression: **{}**. F-AL1-prime: **{}**.\n\nB3 authority: `{}` / `{}`. Exact typed phase surface: **{}**; caller-supplied synthetic receipts accepted: **{}**.\n\n{}\n\nPermitted conclusion: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
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
        certificate.extraction_complete,
        certificate.operational.exact_operational_regression,
        certificate.f_al1_prime_passed,
        certificate.intrinsic_isolation.schema,
        certificate.intrinsic_isolation.theorem_id,
        certificate.intrinsic_isolation.exact_typed_phase_surface,
        certificate
            .intrinsic_isolation
            .synthetic_receipt_input_accepted,
        table,
        certificate.permitted_conclusion,
        certificate.result_digest,
    )
}

pub fn emit_t_bi_nu1_regression_v5_create_new(
    directory: &Path,
) -> Result<TBiNu1RegressionV5Certificate, TBiNu1RegressionV5Error> {
    let certificate = issue_t_bi_nu1_regression_v5_certificate()?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| TBiNu1RegressionV5Error::Json(error.to_string()))?;
    let replay = replay_t_bi_nu1_regression_v5_json(&json);
    if !replay.valid {
        return Err(TBiNu1RegressionV5Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let certificate_path = directory.join(T_BI_NU1_V5_CERTIFICATE_NAME);
    let report_path = directory.join(T_BI_NU1_V5_REPORT_NAME);
    if certificate_path.exists() || report_path.exists() {
        return Err(TBiNu1RegressionV5Error::Io(format!(
            "create-new target already exists: {} or {}",
            certificate_path.display(),
            report_path.display()
        )));
    }
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| TBiNu1RegressionV5Error::Io(error.to_string()))?;
    certificate_file
        .write_all(json.as_bytes())
        .map_err(|error| TBiNu1RegressionV5Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| TBiNu1RegressionV5Error::Io(error.to_string()))?;
    report_file
        .write_all(render_t_bi_nu1_regression_v5_report(&certificate).as_bytes())
        .map_err(|error| TBiNu1RegressionV5Error::Io(error.to_string()))?;
    Ok(certificate)
}

pub fn replay_t_bi_nu1_regression_v5_directory(
    directory: &Path,
) -> Result<TBiNu1RegressionV5Replay, TBiNu1RegressionV5Error> {
    let json = std::fs::read_to_string(directory.join(T_BI_NU1_V5_CERTIFICATE_NAME))
        .map_err(|error| TBiNu1RegressionV5Error::Io(error.to_string()))?;
    Ok(replay_t_bi_nu1_regression_v5_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::t_bi_intrinsic_isolation_v2::{
        T_BI_B3_V2_THEOREM_ID, T_BI_INTRINSIC_ISOLATION_V2_SCHEMA,
    };

    #[test]
    fn f_al1_prime_uses_the_complete_semantic_register_without_scalar_equality() {
        let certificate = issue_t_bi_nu1_regression_v5_certificate().expect("T-BI v5");
        assert_eq!(certificate.role_declaration_count, 250);
        assert_eq!(
            certificate.proved_family_declaration_count
                + certificate.theorem_impossibility_declaration_count,
            250
        );
        assert_eq!(certificate.named_registry_residual_count, 0);
        assert_eq!(certificate.named_quotient_residual_count, 0);
        assert_eq!(certificate.named_a3_residual_count, 0);
        assert_eq!(certificate.total_named_residual_count, 0);
        assert_eq!(certificate.silent_residue_count, 0);
        assert_eq!(
            certificate.intrinsic_isolation.schema,
            T_BI_INTRINSIC_ISOLATION_V2_SCHEMA
        );
        assert_eq!(
            certificate.intrinsic_isolation.theorem_id,
            T_BI_B3_V2_THEOREM_ID
        );
        assert_eq!(certificate.intrinsic_isolation.phase_receipts.len(), 256);
        assert!(certificate.intrinsic_isolation.exact_typed_phase_surface);
        assert!(
            certificate
                .intrinsic_isolation
                .no_forbidden_capability_in_transitive_closure
        );
        assert!(
            !certificate
                .intrinsic_isolation
                .synthetic_receipt_input_accepted
        );
        assert!(certificate.extraction_complete);
        assert!(certificate.register_table_is_bare_and_exact);
        assert!(!certificate.structural_semantic_scalar_equality_required);
        assert!(certificate.f_al1_prime_passed);
        assert!(
            replay_t_bi_nu1_regression_v5_certificate(&certificate)
                .errors
                .is_empty()
        );
    }

    #[test]
    fn register_rows_have_exactly_the_three_authorized_fields() {
        let certificate = issue_t_bi_nu1_regression_v5_certificate().expect("T-BI v5");
        let rows = certificate.register_table.expect("authoritative table");
        assert_eq!(rows.len(), 15);
        assert!(rows.iter().all(|row| {
            serde_json::to_value(row)
                .expect("row JSON")
                .as_object()
                .is_some_and(|object| object.len() == 3)
        }));
    }

    #[test]
    fn historical_input_surface_is_reference_fifteen_only() {
        let entries = (1..=15)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let sequence =
            crate::act_local_semantic_provenance_v5::issue_act_local_semantic_sequence_v5(&entries)
                .expect("reference sequence");
        assert_eq!(sequence.packages.len(), 15);
        assert_eq!(sequence.exact_package_derivation_hashes.len(), 15);
    }
}
