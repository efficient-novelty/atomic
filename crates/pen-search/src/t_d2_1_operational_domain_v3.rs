//! T-D2-1 v3: unchanged operational-domain obligations rerun against DNF-Q.
//!
//! This layer does not weaken the v2 bounds or turn partial DNF reconstruction
//! into membership.  It consumes only `Projected` DNF-4 dispositions.

use crate::dnfq_theorem_layer_v1::{
    issue_dnf1_canonical_contexts_v1, issue_dnf2_unified_judgments_v1,
    issue_dnf3_naturality_closure_v1, issue_dnf4_corpus_projection_v1,
    replay_dnf1_canonical_contexts_v1, replay_dnf2_unified_judgments_v1,
    replay_dnf3_naturality_closure_v1, replay_dnf4_corpus_projection_v1, Dnf4ProjectionDisposition,
    DnfRunStatus,
};
use crate::t_d2_1_operational_domain_v2::{
    issue_t_d2_1_operational_domain_v2, replay_t_d2_1_operational_domain_v2, Td21V2BoundCitation,
    Td21V2BoundSnapshot, Td21V2RunStatus, Td21V2SourceBinding,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, remove_file, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const T_D2_1_OPERATIONAL_DOMAIN_V3_SCHEMA: &str = "t-d2-1-operational-domain-build-v3-dnfq";
pub const T_D2_1_OPERATIONAL_DOMAIN_V3_DATE: &str = "2026-07-24";

const DNF_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/dnfq_theorem_layer_plan.md");
const OPERATIONAL_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_operational_domain_adjudication.md");
const V2_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/t_d2_1_operational_domain_v2.json");

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21V3RunStatus {
    Passed,
    StoppedDnfNamedGaps,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V3SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: usize,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V3TheoremObligation {
    pub theorem_id: String,
    pub statement: String,
    pub satisfied: bool,
    pub exact_obstruction: Option<String>,
    pub evidence_digests: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V3Regression {
    pub expected_family_count: usize,
    pub observed_family_count: usize,
    pub recognized_family_count: usize,
    pub every_family_recognized: bool,
    pub expected_membership_row_count: usize,
    pub observed_membership_row_count: usize,
    pub recognized_membership_row_count: usize,
    pub every_membership_row_recognized: bool,
    pub recognition_uses_only_dnf4_projected_dispositions: bool,
    pub named_gap_rows_receive_no_partial_credit: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21OperationalDomainV3Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Td21V3SourceBinding>,
    pub predecessor_v2_result_digest: String,
    pub predecessor_v2_replay_valid: bool,
    pub predecessor_v2_status: Td21V2RunStatus,
    pub frozen_bound_snapshot_digest: String,
    pub frozen_bound_snapshot: Td21V2BoundSnapshot,
    pub frozen_bound_citations: Vec<Td21V2BoundCitation>,
    pub predecessor_v2_source_bindings: Vec<Td21V2SourceBinding>,
    pub frozen_bounds_unchanged: bool,
    pub frozen_citations_unchanged: bool,
    pub dnf1_result_digest: String,
    pub dnf2_result_digest: String,
    pub dnf3_result_digest: String,
    pub dnf4_result_digest: String,
    pub all_dnf_certificates_replay: bool,
    pub dnf1_status: DnfRunStatus,
    pub dnf2_status: DnfRunStatus,
    pub dnf3_status: DnfRunStatus,
    pub dnf4_status: DnfRunStatus,
    pub context_finiteness: Td21V3TheoremObligation,
    pub typed_normalization: Td21V3TheoremObligation,
    pub natural_family_quotient: Td21V3TheoremObligation,
    pub regression: Td21V3Regression,
    pub fixed_fragment_operationally_defined: bool,
    pub membership_decidable: bool,
    pub typed_normalization_total: bool,
    pub quotient_decidable: bool,
    pub quotient_finitely_enumerable: bool,
    pub family_regression_passed: bool,
    pub membership_row_regression_passed: bool,
    pub t_d2_2_prerequisite_satisfied: bool,
    pub bc1_opened: bool,
    pub bridge_431_gap_rerun_executed: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m4_authorized: bool,
    pub status: Td21V3RunStatus,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Td21V3Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub status: Option<Td21V3RunStatus>,
    pub frozen_bounds_unchanged: bool,
    pub family_regression_passed: bool,
    pub membership_row_regression_passed: bool,
    pub t_d2_2_prerequisite_satisfied: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Td21V3Error {
    #[error("T-D2-1 v3 input failure: {0}")]
    Input(String),
    #[error("T-D2-1 v3 invariant failure: {0}")]
    Invariant(String),
    #[error("T-D2-1 v3 JSON failure: {0}")]
    Json(String),
    #[error("T-D2-1 v3 I/O failure: {0}")]
    Io(String),
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_D2_1_OPERATIONAL_DOMAIN_V3_SCHEMA, domain, value))
        .expect("T-D2-1 v3 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> Td21V3SourceBinding {
    Td21V3SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len(),
        blake3: bytes_hash(bytes),
    }
}

fn source_bindings() -> Vec<Td21V3SourceBinding> {
    vec![
        source_binding(
            "docs/dnfq_theorem_layer_plan.md",
            "frozen DNF-Q theorem program and DNF-5 gate",
            DNF_PLAN_BYTES,
        ),
        source_binding(
            "docs/schema2_operational_domain_adjudication.md",
            "frozen operational-domain bounds",
            OPERATIONAL_ADJUDICATION_BYTES,
        ),
        source_binding(
            "docs/t_d2_1_operational_domain_v2.json",
            "predecessor bound snapshot, citations, and unchanged theorem obligations",
            V2_CERTIFICATE_BYTES,
        ),
    ]
}

fn obligation(
    theorem_id: &str,
    statement: &str,
    satisfied: bool,
    exact_obstruction: Option<String>,
    evidence_digests: Vec<String>,
) -> Td21V3TheoremObligation {
    let mut result = Td21V3TheoremObligation {
        theorem_id: theorem_id.to_owned(),
        statement: statement.to_owned(),
        satisfied,
        exact_obstruction,
        evidence_digests,
        derivation_hash: String::new(),
    };
    result.derivation_hash = tagged_hash("theorem-obligation", &result);
    result
}

fn regression_hash(regression: &mut Td21V3Regression) {
    regression.derivation_hash.clear();
    regression.derivation_hash = tagged_hash("regression", regression);
}

fn certificate_digest(certificate: &Td21OperationalDomainV3Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn build_certificate() -> Result<Td21OperationalDomainV3Certificate, Td21V3Error> {
    let v2 = issue_t_d2_1_operational_domain_v2()
        .map_err(|error| Td21V3Error::Input(format!("predecessor v2 issuance failed: {error}")))?;
    let v2_replay = replay_t_d2_1_operational_domain_v2(&v2);
    if !v2_replay.valid {
        return Err(Td21V3Error::Input(format!(
            "predecessor v2 replay failed: {}",
            v2_replay.errors.join("; ")
        )));
    }

    let dnf1 = issue_dnf1_canonical_contexts_v1()
        .map_err(|error| Td21V3Error::Input(error.to_string()))?;
    let dnf2 =
        issue_dnf2_unified_judgments_v1().map_err(|error| Td21V3Error::Input(error.to_string()))?;
    let dnf3 = issue_dnf3_naturality_closure_v1()
        .map_err(|error| Td21V3Error::Input(error.to_string()))?;
    let dnf4 =
        issue_dnf4_corpus_projection_v1().map_err(|error| Td21V3Error::Input(error.to_string()))?;
    let dnf_replays = [
        replay_dnf1_canonical_contexts_v1(&dnf1),
        replay_dnf2_unified_judgments_v1(&dnf2),
        replay_dnf3_naturality_closure_v1(&dnf3),
        replay_dnf4_corpus_projection_v1(&dnf4),
    ];
    let all_dnf_certificates_replay = dnf_replays.iter().all(|replay| replay.valid);
    if !all_dnf_certificates_replay {
        return Err(Td21V3Error::Input(
            "at least one DNF certificate failed deterministic replay".to_owned(),
        ));
    }

    let archived_v2: crate::t_d2_1_operational_domain_v2::Td21OperationalDomainV2Certificate =
        serde_json::from_slice(V2_CERTIFICATE_BYTES)
            .map_err(|error| Td21V3Error::Json(error.to_string()))?;
    let frozen_bounds_unchanged = v2.bound_snapshot == archived_v2.bound_snapshot
        && v2.bound_snapshot_digest == archived_v2.bound_snapshot_digest
        && v2.bound_snapshot.public_entry_count == 15
        && v2.bound_snapshot.public_clause_count == 64
        && v2.bound_snapshot.maximum_ambient_parameter_count == 2
        && v2.bound_snapshot.maximum_free_scope_length == 9
        && v2.bound_snapshot.maximum_binder_nesting == 2
        && v2.bound_snapshot.maximum_path_dimension == 3
        && v2.bound_snapshot.semantic_former_depth == 2;
    let frozen_citations_unchanged =
        v2.bound_snapshot.citations == archived_v2.bound_snapshot.citations;
    if !frozen_bounds_unchanged || !frozen_citations_unchanged {
        return Err(Td21V3Error::Invariant(
            "DNF-5 changed a frozen bound or bound citation".to_owned(),
        ));
    }

    let context_finiteness_satisfied = dnf1.full_context_grammar_finitely_enumerated
        && dnf1.universal_canonicity_proved
        && dnf1.schema_and_kernel_context_calculi_identified;
    let typed_normalization_satisfied = dnf2.every_surface_has_full_typed_judgment
        && dnf2.translations_compose_with_frozen_equality
        && dnf3.normalization_commutes_by_constructor_induction;
    let quotient_satisfied = dnf3.universal_naturality_closure_proved
        && dnf3.quotient_congruence_under_every_legal_substitution
        && dnf4.full_content_projection_complete;

    let context_finiteness = obligation(
        "T-D2-1-CONTEXT-FINITENESS-v3",
        "The frozen bounded dependent-context grammar has a finite canonical enumeration.",
        context_finiteness_satisfied,
        (!context_finiteness_satisfied).then(|| {
            "DNF-1 canonicalizes the local Type/Element fragment but publishes gaps for the full constructor grammar, the kernel/Schema2 context bridge, and universal canonicity.".to_owned()
        }),
        vec![dnf1.result_digest.clone()],
    );
    let typed_normalization = obligation(
        "T-D2-1-TYPED-NORMALIZATION-v3",
        "Typed normalization is total on every judgment in the frozen bounded carrier.",
        typed_normalization_satisfied,
        (!typed_normalization_satisfied).then(|| {
            "DNF-2 retains every source payload but does not identify all four surfaces as full dependent judgments; DNF-3 lacks the all-substitution constructor induction.".to_owned()
        }),
        vec![dnf2.result_digest.clone(), dnf3.result_digest.clone()],
    );
    let natural_family_quotient = obligation(
        "T-D2-1-NATURAL-FAMILY-QUOTIENT-v3",
        "The adopted natural-family quotient is decidable and finitely enumerable.",
        quotient_satisfied,
        (!quotient_satisfied).then(|| {
            "DNF-3 reconstructs 72 concrete commuting squares but proves no universal quotient congruence; DNF-4 consequently classifies all 150 rows as named gaps.".to_owned()
        }),
        vec![dnf3.result_digest.clone(), dnf4.result_digest.clone()],
    );

    let recognized_family_count = dnf4
        .rows
        .iter()
        .filter(|row| {
            row.row_kind == crate::dnfq_theorem_layer_v1::Dnf4RowKind::ProvedFamily
                && matches!(row.disposition, Dnf4ProjectionDisposition::Projected { .. })
        })
        .count();
    let recognized_membership_row_count = dnf4
        .rows
        .iter()
        .filter(|row| {
            row.row_kind == crate::dnfq_theorem_layer_v1::Dnf4RowKind::A3Membership
                && matches!(row.disposition, Dnf4ProjectionDisposition::Projected { .. })
        })
        .count();
    let mut regression = Td21V3Regression {
        expected_family_count: 61,
        observed_family_count: dnf4.observed_family_row_count,
        recognized_family_count,
        every_family_recognized: recognized_family_count == 61,
        expected_membership_row_count: 89,
        observed_membership_row_count: dnf4.observed_a3_row_count,
        recognized_membership_row_count,
        every_membership_row_recognized: recognized_membership_row_count == 89,
        recognition_uses_only_dnf4_projected_dispositions: true,
        named_gap_rows_receive_no_partial_credit: true,
        derivation_hash: String::new(),
    };
    regression_hash(&mut regression);

    let membership_decidable = context_finiteness.satisfied
        && typed_normalization.satisfied
        && natural_family_quotient.satisfied;
    let family_regression_passed = regression.every_family_recognized;
    let membership_row_regression_passed = regression.every_membership_row_recognized;
    let t_d2_2_prerequisite_satisfied =
        membership_decidable && family_regression_passed && membership_row_regression_passed;
    let status = if t_d2_2_prerequisite_satisfied {
        Td21V3RunStatus::Passed
    } else {
        Td21V3RunStatus::StoppedDnfNamedGaps
    };
    let permitted_conclusion = if t_d2_2_prerequisite_satisfied {
        "T-D2-1 v3 passed unchanged against exact DNF content; T-D2-2 may rerun."
    } else {
        "T-D2-1 v3 replayed its original bounds and obligations unchanged, but DNF named gaps leave exact recognition incomplete. T-D2-2, BC1, the 431-gap bridge rerun, and M-4 remain closed."
    };
    let mut certificate = Td21OperationalDomainV3Certificate {
        schema: T_D2_1_OPERATIONAL_DOMAIN_V3_SCHEMA.to_owned(),
        date: T_D2_1_OPERATIONAL_DOMAIN_V3_DATE.to_owned(),
        source_bindings: source_bindings(),
        predecessor_v2_result_digest: v2.result_digest.clone(),
        predecessor_v2_replay_valid: v2_replay.valid,
        predecessor_v2_status: v2.status,
        frozen_bound_snapshot_digest: v2.bound_snapshot_digest.clone(),
        frozen_bound_snapshot: v2.bound_snapshot.clone(),
        frozen_bound_citations: v2.bound_snapshot.citations.clone(),
        predecessor_v2_source_bindings: v2.source_bindings.clone(),
        frozen_bounds_unchanged,
        frozen_citations_unchanged,
        dnf1_result_digest: dnf1.result_digest.clone(),
        dnf2_result_digest: dnf2.result_digest.clone(),
        dnf3_result_digest: dnf3.result_digest.clone(),
        dnf4_result_digest: dnf4.result_digest.clone(),
        all_dnf_certificates_replay,
        dnf1_status: dnf1.status,
        dnf2_status: dnf2.status,
        dnf3_status: dnf3.status,
        dnf4_status: dnf4.status,
        context_finiteness,
        typed_normalization,
        natural_family_quotient,
        regression,
        fixed_fragment_operationally_defined: v2.fixed_fragment_operationally_defined,
        membership_decidable,
        typed_normalization_total: typed_normalization_satisfied,
        quotient_decidable: quotient_satisfied,
        quotient_finitely_enumerable: quotient_satisfied && context_finiteness_satisfied,
        family_regression_passed,
        membership_row_regression_passed,
        t_d2_2_prerequisite_satisfied,
        bc1_opened: false,
        bridge_431_gap_rerun_executed: false,
        m3_v1_remains_authoritative: true,
        m4_authorized: false,
        status,
        permitted_conclusion: permitted_conclusion.to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED: OnceLock<Result<Td21OperationalDomainV3Certificate, String>> = OnceLock::new();

fn expected() -> Result<&'static Td21OperationalDomainV3Certificate, Td21V3Error> {
    match EXPECTED.get_or_init(|| build_certificate().map_err(|error| error.to_string())) {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(Td21V3Error::Input(error.clone())),
    }
}

pub fn issue_t_d2_1_operational_domain_v3(
) -> Result<Td21OperationalDomainV3Certificate, Td21V3Error> {
    expected().cloned()
}

fn failed_replay(error: impl Into<String>) -> Td21V3Replay {
    Td21V3Replay {
        valid: false,
        errors: vec![error.into()],
        status: None,
        frozen_bounds_unchanged: false,
        family_regression_passed: false,
        membership_row_regression_passed: false,
        t_d2_2_prerequisite_satisfied: false,
        m4_authorized: false,
    }
}

pub fn replay_t_d2_1_operational_domain_v3(
    claimed: &Td21OperationalDomainV3Certificate,
) -> Td21V3Replay {
    let expected = match expected() {
        Ok(certificate) => certificate,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-D2-1 v3 result digest mismatch".to_owned());
    }
    if claimed != expected {
        errors.push("T-D2-1 v3 certificate differs from deterministic reissuance".to_owned());
    }
    Td21V3Replay {
        valid: errors.is_empty(),
        errors,
        status: Some(claimed.status),
        frozen_bounds_unchanged: claimed.frozen_bounds_unchanged
            && claimed.frozen_citations_unchanged,
        family_regression_passed: claimed.family_regression_passed,
        membership_row_regression_passed: claimed.membership_row_regression_passed,
        t_d2_2_prerequisite_satisfied: claimed.t_d2_2_prerequisite_satisfied,
        m4_authorized: claimed.m4_authorized,
    }
}

pub fn replay_t_d2_1_operational_domain_v3_json(json: &str) -> Td21V3Replay {
    match serde_json::from_str::<Td21OperationalDomainV3Certificate>(json) {
        Ok(certificate) => replay_t_d2_1_operational_domain_v3(&certificate),
        Err(error) => failed_replay(format!("invalid T-D2-1 v3 JSON: {error}")),
    }
}

pub fn render_t_d2_1_operational_domain_v3(
    certificate: &Td21OperationalDomainV3Certificate,
) -> String {
    format!(
        "# T-D2-1 operational depth-two domain v3 (DNF-Q rerun)\n\n\
**Date:** {}. **Status:** `{:?}`. **Certificate:** `{}`.\n\n\
The v2 bounds and citations replay unchanged: entries/clauses {}/{}, ambient/free scope {}/{}, binder {}, dimension {}, semantic depth {}. No DNF theorem changed or parameterized a bound.\n\n\
The three unchanged obligations report context finiteness `{}`, typed-normalization totality `{}`, and natural-family quotient decidability/finiteness `{}`. All four DNF certificates replay: `{}`.\n\n\
Exact DNF-4 regression recognizes **{}/{}** proved families and **{}/{}** A3 membership rows. A named-gap disposition receives no partial credit.\n\n\
## Exact stop\n\n\
{}\n\n\
T-D2-2 prerequisite: `{}`. BC1 opened: `{}`. 431-gap bridge rerun executed: `{}`. M-3 v1 remains authoritative: `{}`. M-4 authorized: `{}`.\n",
        certificate.date,
        certificate.status,
        certificate.result_digest,
        certificate.frozen_bound_snapshot.public_entry_count,
        certificate.frozen_bound_snapshot.public_clause_count,
        certificate.frozen_bound_snapshot.maximum_ambient_parameter_count,
        certificate.frozen_bound_snapshot.maximum_free_scope_length,
        certificate.frozen_bound_snapshot.maximum_binder_nesting,
        certificate.frozen_bound_snapshot.maximum_path_dimension,
        certificate.frozen_bound_snapshot.semantic_former_depth,
        certificate.context_finiteness.satisfied,
        certificate.typed_normalization.satisfied,
        certificate.natural_family_quotient.satisfied,
        certificate.all_dnf_certificates_replay,
        certificate.regression.recognized_family_count,
        certificate.regression.expected_family_count,
        certificate.regression.recognized_membership_row_count,
        certificate.regression.expected_membership_row_count,
        certificate.permitted_conclusion,
        certificate.t_d2_2_prerequisite_satisfied,
        certificate.bc1_opened,
        certificate.bridge_431_gap_rerun_executed,
        certificate.m3_v1_remains_authoritative,
        certificate.m4_authorized,
    )
}

pub fn emit_t_d2_1_operational_domain_v3_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Td21OperationalDomainV3Certificate, Td21V3Error> {
    if certificate_path.exists() || report_path.exists() {
        return Err(Td21V3Error::Io(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_t_d2_1_operational_domain_v3()?;
    let replay = replay_t_d2_1_operational_domain_v3(&certificate);
    if !replay.valid {
        return Err(Td21V3Error::Invariant(replay.errors.join("; ")));
    }
    let mut json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Td21V3Error::Json(error.to_string()))?;
    json.push(b'\n');
    let report = render_t_d2_1_operational_domain_v3(&certificate);
    let result = (|| -> Result<(), Td21V3Error> {
        let mut certificate_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(certificate_path)
            .map_err(|error| Td21V3Error::Io(error.to_string()))?;
        certificate_file
            .write_all(&json)
            .and_then(|_| certificate_file.sync_all())
            .map_err(|error| Td21V3Error::Io(error.to_string()))?;
        let mut report_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(report_path)
            .map_err(|error| Td21V3Error::Io(error.to_string()))?;
        report_file
            .write_all(report.as_bytes())
            .and_then(|_| report_file.sync_all())
            .map_err(|error| Td21V3Error::Io(error.to_string()))?;
        let emitted =
            read_to_string(certificate_path).map_err(|error| Td21V3Error::Io(error.to_string()))?;
        let emitted_replay = replay_t_d2_1_operational_domain_v3_json(&emitted);
        if !emitted_replay.valid {
            return Err(Td21V3Error::Invariant(
                "emitted T-D2-1 v3 JSON failed replay".to_owned(),
            ));
        }
        Ok(())
    })();
    if let Err(error) = result {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(error);
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn reseal(certificate: &mut Td21OperationalDomainV3Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    #[test]
    fn v3_preserves_every_frozen_bound_and_stops_on_dnf_gaps() {
        let certificate = issue_t_d2_1_operational_domain_v3().expect("v3 issuer");
        let replay = replay_t_d2_1_operational_domain_v3(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(certificate.frozen_bounds_unchanged);
        assert!(certificate.frozen_citations_unchanged);
        assert_eq!(certificate.frozen_bound_snapshot.public_entry_count, 15);
        assert_eq!(certificate.frozen_bound_snapshot.public_clause_count, 64);
        assert_eq!(
            certificate.frozen_bound_snapshot.maximum_free_scope_length,
            9
        );
        assert_eq!(certificate.frozen_bound_snapshot.maximum_binder_nesting, 2);
        assert_eq!(certificate.frozen_bound_snapshot.maximum_path_dimension, 3);
        assert_eq!(certificate.frozen_bound_snapshot.semantic_former_depth, 2);
        assert_eq!(certificate.regression.observed_family_count, 61);
        assert_eq!(certificate.regression.observed_membership_row_count, 89);
        assert_eq!(certificate.regression.recognized_family_count, 0);
        assert_eq!(certificate.regression.recognized_membership_row_count, 0);
        assert_eq!(certificate.status, Td21V3RunStatus::StoppedDnfNamedGaps);
        assert!(!certificate.t_d2_2_prerequisite_satisfied);
        assert!(!certificate.bc1_opened);
        assert!(!certificate.bridge_431_gap_rerun_executed);
        assert!(!certificate.m4_authorized);
    }

    #[test]
    fn resealed_bound_or_dnf_mutations_fail_definition_replay() {
        let mut bound_mutation = issue_t_d2_1_operational_domain_v3().expect("v3 issuer");
        bound_mutation
            .frozen_bound_snapshot
            .maximum_free_scope_length = 10;
        reseal(&mut bound_mutation);
        assert!(!replay_t_d2_1_operational_domain_v3(&bound_mutation).valid);

        let mut dnf_mutation = issue_t_d2_1_operational_domain_v3().expect("v3 issuer");
        dnf_mutation.dnf4_result_digest = "forged".to_owned();
        reseal(&mut dnf_mutation);
        assert!(!replay_t_d2_1_operational_domain_v3(&dnf_mutation).valid);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_t_d2_1_operational_domain_v3().expect("v3 issuer");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("unknown_field".to_owned(), Value::Bool(true));
        let replay =
            replay_t_d2_1_operational_domain_v3_json(&serde_json::to_string(&value).unwrap());
        assert!(!replay.valid);
        assert!(!replay.t_d2_2_prerequisite_satisfied);
        assert!(!replay.m4_authorized);
    }
}
