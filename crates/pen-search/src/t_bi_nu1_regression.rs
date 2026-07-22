//! Post-issuance T-BI-NU1 coverage and F-AL1 regression.
//!
//! This module is intentionally separate from `act_local_provenance`.  It
//! first issues and seals all candidate/prefix-local packages.  Only after
//! that intrinsic issuance digest exists does it open the frozen enacted
//! history as a read-only F-AL1 comparator.  Archive bytes can therefore
//! corroborate or falsify the theorem output, but cannot enter a token hash.

use crate::act_local_provenance::{
    ActLocalNuProvenanceCertificate, ActLocalProvenanceAnchor, T_BI_NU1_THEOREM_ID,
    issue_act_local_sequence,
};
use crate::phase5b_history_certification::{
    PHASE5B_HISTORY_CERT_SCHEMA, Phase5bHistoryCertificate,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3DemandSchemeOrigin, A3RuleConstructor, generate_a3_window_for_exact_prefix_unbounded,
};
use pen_eval::a3_rule_inventory_exhaustiveness::prove_a3_window_inventory_for_exact_prefix_unbounded;
use pen_eval::future_hole_hypothesis_v2::{
    FutureHoleOutputContractV2, FutureHoleRegistrationDispositionV2,
    register_structural_future_hole_v2,
};
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const T_BI_NU1_REGRESSION_SCHEMA: &str = "t-bi-nu1-act-local-regression-v1";
pub const T_BI_NU1_REGRESSION_DATE: &str = "2026-07-22";
pub const T_BI_NU1_CERTIFICATE_NAME: &str = "t_bi_nu1_act_local_provenance_v1.json";
pub const T_BI_NU1_REPORT_NAME: &str = "T_BI_NU1_ACT_LOCAL_PROVENANCE_RESULT.md";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/bi0_prerequisites_adjudication.md");
const ARCHIVED_HISTORY_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const ACT_LOCAL_SOURCE_BYTES: &[u8] = include_bytes!("act_local_provenance.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_bi_nu1_regression.rs");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_NU1_REGRESSION_SCHEMA, domain, value))
        .expect("T-BI-NU1 regression evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value))
        .expect("external certificate evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalClosureRow {
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub structural_formula_total: u32,
    pub generated_instance_adjustment: i32,
    pub exact_certified_nu: u32,
    pub local_role_token_count: usize,
    pub live_demand_output_token_count: usize,
    pub independent_structural_demand_orbit_count: usize,
    pub ordinary_token_count: usize,
    pub ordinary_token_hashes_unique: bool,
    pub package_authoritative: bool,
    pub package_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralFillerClassCoverage {
    pub registration_stage: u32,
    pub jurisdiction_filler_stage: u32,
    pub a3_scheme_id: String,
    pub a3_instance_id: String,
    pub constructor: String,
    pub registration_formation_hash: String,
    pub filler_package_hash: String,
    pub filler_exact_certified_nu: u32,
    pub filler_ordinary_token_hashes: Vec<String>,
    pub filler_tokens_nonempty_when_positive: bool,
    pub filler_package_authoritative: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FAl1Divergence {
    pub stage: u32,
    pub field: String,
    pub intrinsic_value: String,
    pub archived_value: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FAl1RegressionAudit {
    pub falsifier: String,
    pub intrinsic_issuance_digest_sealed_before_archive_read: bool,
    pub archive_schema: String,
    pub archive_result_digest: String,
    pub archive_self_digest_valid: bool,
    pub archive_used_only_as_post_issuance_comparator: bool,
    pub intrinsic_nu_vector: Vec<u32>,
    pub archived_nu_vector: Vec<u32>,
    pub candidate_hash_vector_exact: bool,
    pub predecessor_signature_vector_exact: bool,
    pub nu_vector_exact: bool,
    pub divergences: Vec<FAl1Divergence>,
    pub f_al1_triggered: bool,
    pub f_al1_passed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionCertificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub source_bindings: Vec<TBiNu1SourceBinding>,
    pub adoption_replayed: bool,
    pub intrinsic_packages: Vec<ActLocalNuProvenanceCertificate>,
    pub intrinsic_issuance_digest_before_archive_comparator: String,
    pub closure_rows: Vec<ActLocalClosureRow>,
    pub all_fifteen_acts_issued: bool,
    pub all_fifteen_packages_authoritative: bool,
    pub authoritative_token_hashes_globally_unique: bool,
    pub structural_filler_classes: Vec<StructuralFillerClassCoverage>,
    pub structural_filler_class_count: usize,
    pub all_thirteen_structural_filler_classes_covered: bool,
    pub f_al1: FAl1RegressionAudit,
    pub t_bi_nu1_proved_on_enacted_branch: bool,
    pub bi0_rerun_authorized_by_t_bi_nu1_side: bool,
    pub non_enacted_branch_work_executed: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionReplay {
    pub valid: bool,
    pub all_fifteen_acts_issued: bool,
    pub structural_filler_class_count: usize,
    pub f_al1_passed: bool,
    pub intrinsic_nu_vector: Vec<u32>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiNu1RegressionError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("T-BI-NU1 invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted replay failed: {0}")]
    EmittedReplay(String),
}

fn source_bindings() -> Vec<TBiNu1SourceBinding> {
    [
        (
            "docs/bi0_prerequisites_adjudication.md",
            "adopted_act_local_provenance_rule_and_falsifiers",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_provenance.rs",
            "archive_free_candidate_prefix_token_issuer",
            ACT_LOCAL_SOURCE_BYTES,
        ),
        (
            "docs/phase5b_full_history_v1.json",
            "post_issuance_F_AL1_comparator_only",
            ARCHIVED_HISTORY_BYTES,
        ),
        (
            "crates/pen-search/src/t_bi_nu1_regression.rs",
            "separate_post_issuance_comparator_and_artifact_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| TBiNu1SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn replay_adoption() -> Result<(), TBiNu1RegressionError> {
    let text = std::str::from_utf8(ADJUDICATION_BYTES)
        .map_err(|error| TBiNu1RegressionError::Prerequisite(error.to_string()))?;
    for clause in [
        "act-local-provenance-v1",
        "T-BI-NU1",
        "F-AL1",
        "candidate and its sealed prefix alone",
        "archived joins as",
        "regression references only",
    ] {
        if !text.contains(clause) {
            return Err(TBiNu1RegressionError::Prerequisite(format!(
                "adjudication omits {clause:?}"
            )));
        }
    }
    Ok(())
}

fn certificate_digest(certificate: &TBiNu1RegressionCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("t-bi-nu1-regression-certificate", &projection)
}

fn closure_row(package: &ActLocalNuProvenanceCertificate) -> ActLocalClosureRow {
    let local_role_token_count = package
        .ordinary_family_tokens
        .iter()
        .filter(|token| {
            matches!(
                token.anchor,
                ActLocalProvenanceAnchor::ChargedLocalRole { .. }
            )
        })
        .count();
    let live_demand_output_token_count =
        package.ordinary_family_tokens.len() - local_role_token_count;
    let hashes = package.authoritative_token_hashes();
    let mut row = ActLocalClosureRow {
        stage: package.stage,
        candidate_hash: package.candidate_hash.clone(),
        predecessor_signature_digest: package.predecessor_signature_digest.clone(),
        structural_formula_total: package.structural_formula_total,
        generated_instance_adjustment: package.generated_instance_adjustment,
        exact_certified_nu: package.exact_certified_nu,
        local_role_token_count,
        live_demand_output_token_count,
        independent_structural_demand_orbit_count: package
            .independent_structural_demand_orbit_ids
            .len(),
        ordinary_token_count: hashes.len(),
        ordinary_token_hashes_unique: hashes.iter().collect::<BTreeSet<_>>().len() == hashes.len(),
        package_authoritative: package.authoritative,
        package_derivation_hash: package.derivation_hash.clone(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("act-local-closure-row", &row);
    row
}

fn structural_filler_coverage(
    entries: &[(u32, Telescope)],
    packages: &[ActLocalNuProvenanceCertificate],
) -> Result<Vec<StructuralFillerClassCoverage>, TBiNu1RegressionError> {
    let package_by_stage = packages
        .iter()
        .map(|package| (package.stage, package))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    for registration_stage in 1..=entries.len() as u32 {
        let demand_prefix = SealedSignature::from_telescopes(
            entries[..registration_stage.saturating_sub(1) as usize].to_vec(),
        );
        let window =
            generate_a3_window_for_exact_prefix_unbounded(&demand_prefix, registration_stage)
                .map_err(|error| TBiNu1RegressionError::Invariant(error.to_string()))?;
        let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(
            &demand_prefix,
            registration_stage,
        )
        .map_err(|error| TBiNu1RegressionError::Invariant(error.to_string()))?;
        if !proof.relative_rule_inventory_exhaustive_for_window {
            return Err(TBiNu1RegressionError::Invariant(format!(
                "Stage {registration_stage} A3 inventory is not exhaustive"
            )));
        }
        let schemes = window
            .schemes
            .iter()
            .map(|scheme| (scheme.scheme_id.as_str(), scheme))
            .collect::<BTreeMap<_, _>>();
        for instance in &window.instances {
            let scheme = schemes.get(instance.scheme_id.as_str()).ok_or_else(|| {
                TBiNu1RegressionError::Invariant(format!(
                    "A3 instance {} has no scheme",
                    instance.instance_id
                ))
            })?;
            if scheme.rule_constructor != A3RuleConstructor::StructuralCompletionHole {
                continue;
            }
            let A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } = scheme.origin
            else {
                return Err(TBiNu1RegressionError::Invariant(format!(
                    "structural scheme {} has a base origin",
                    scheme.scheme_id
                )));
            };
            // The adopted Stage-3 wrinkle separates the demand window from
            // the declaration jurisdiction: its Former demand is generated
            // over [1,2], while the filler is registered only after Stage 3
            // has entered the sealed signature.  Every other stage uses the
            // ordinary pre-stage demand prefix for both operations.
            let registration_prefix = if registration_stage == 3 {
                SealedSignature::from_telescopes(entries[..3].to_vec())
            } else {
                demand_prefix.clone()
            };
            let registration =
                register_structural_future_hole_v2(&registration_prefix, &window, scheme, instance)
                    .map_err(|error| TBiNu1RegressionError::Invariant(error.to_string()))?;
            let FutureHoleRegistrationDispositionV2::Registered(registration) = registration else {
                let FutureHoleRegistrationDispositionV2::Gap(gap) = registration else {
                    unreachable!()
                };
                return Err(TBiNu1RegressionError::Invariant(format!(
                    "structural filler registration {} remained gap {}: {}",
                    instance.instance_id, gap.id, gap.exact_error
                )));
            };
            let FutureHoleOutputContractV2::StructuralProvides(contract) =
                &registration.output_contract
            else {
                return Err(TBiNu1RegressionError::Invariant(format!(
                    "structural registration {} has a unary contract",
                    instance.instance_id
                )));
            };
            let filler_stage = contract.jurisdiction.jurisdiction_stage();
            let filler = package_by_stage.get(&filler_stage).ok_or_else(|| {
                TBiNu1RegressionError::Invariant(format!(
                    "structural class at Stage {registration_stage} requires absent filler Stage {filler_stage}"
                ))
            })?;
            let hashes = filler.authoritative_token_hashes();
            let mut row = StructuralFillerClassCoverage {
                registration_stage,
                jurisdiction_filler_stage: filler_stage,
                a3_scheme_id: scheme.scheme_id.clone(),
                a3_instance_id: instance.instance_id.clone(),
                constructor: constructor.slug().to_owned(),
                registration_formation_hash: registration.formation_hash,
                filler_package_hash: filler.derivation_hash.clone(),
                filler_exact_certified_nu: filler.exact_certified_nu,
                filler_ordinary_token_hashes: hashes.clone(),
                filler_tokens_nonempty_when_positive: filler.exact_certified_nu == 0
                    || !hashes.is_empty(),
                filler_package_authoritative: filler.authoritative,
                derivation_hash: String::new(),
            };
            row.derivation_hash = tagged_hash("structural-filler-class-coverage", &row);
            rows.push(row);
        }
    }
    rows.sort_by(|left, right| {
        (left.registration_stage, left.a3_instance_id.as_str())
            .cmp(&(right.registration_stage, right.a3_instance_id.as_str()))
    });
    Ok(rows)
}

fn archive_self_digest_valid(archive: &Phase5bHistoryCertificate) -> bool {
    let mut projection = archive.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            PHASE5B_HISTORY_CERT_SCHEMA,
            "phase5b-history-certificate",
            &projection,
        )
}

fn divergence(
    stage: u32,
    field: &str,
    intrinsic_value: impl ToString,
    archived_value: impl ToString,
) -> FAl1Divergence {
    let mut row = FAl1Divergence {
        stage,
        field: field.to_owned(),
        intrinsic_value: intrinsic_value.to_string(),
        archived_value: archived_value.to_string(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("f-al1-divergence", &row);
    row
}

fn run_f_al1(
    intrinsic_issuance_digest: &str,
    packages: &[ActLocalNuProvenanceCertificate],
) -> Result<FAl1RegressionAudit, TBiNu1RegressionError> {
    // This parse is intentionally below issuance and sealing in the call
    // graph.  Nothing returned from it is passed back to the token issuer.
    let archive: Phase5bHistoryCertificate = serde_json::from_slice(ARCHIVED_HISTORY_BYTES)
        .map_err(|error| TBiNu1RegressionError::Json(error.to_string()))?;
    let self_digest_valid = archive_self_digest_valid(&archive);
    let archived_by_stage = archive
        .steps
        .iter()
        .map(|step| (step.step, step))
        .collect::<BTreeMap<_, _>>();
    let mut divergences = Vec::new();
    for package in packages {
        let Some(archived) = archived_by_stage.get(&package.stage) else {
            divergences.push(divergence(
                package.stage,
                "missing_archive_row",
                package.stage,
                "absent",
            ));
            continue;
        };
        if package.candidate_hash != archived.candidate_hash {
            divergences.push(divergence(
                package.stage,
                "candidate_hash",
                &package.candidate_hash,
                &archived.candidate_hash,
            ));
        }
        if package.predecessor_signature_digest != archived.predecessor_signature_digest {
            divergences.push(divergence(
                package.stage,
                "predecessor_signature_digest",
                &package.predecessor_signature_digest,
                &archived.predecessor_signature_digest,
            ));
        }
        if package.exact_certified_nu != archived.certified_semantic_total {
            divergences.push(divergence(
                package.stage,
                "certified_semantic_total",
                package.exact_certified_nu,
                archived.certified_semantic_total,
            ));
        }
    }
    if packages.len() != archive.steps.len() {
        divergences.push(divergence(
            0,
            "step_count",
            packages.len(),
            archive.steps.len(),
        ));
    }
    let intrinsic_nu_vector = packages
        .iter()
        .map(|package| package.exact_certified_nu)
        .collect::<Vec<_>>();
    let archived_nu_vector = archive
        .steps
        .iter()
        .map(|step| step.certified_semantic_total)
        .collect::<Vec<_>>();
    let candidate_hash_vector_exact = packages.len() == archive.steps.len()
        && packages.iter().zip(&archive.steps).all(|(left, right)| {
            left.stage == right.step && left.candidate_hash == right.candidate_hash
        });
    let predecessor_signature_vector_exact = packages.len() == archive.steps.len()
        && packages.iter().zip(&archive.steps).all(|(left, right)| {
            left.stage == right.step
                && left.predecessor_signature_digest == right.predecessor_signature_digest
        });
    let nu_vector_exact = intrinsic_nu_vector == archived_nu_vector;
    let f_al1_triggered = !self_digest_valid || !divergences.is_empty();
    let mut audit = FAl1RegressionAudit {
        falsifier:
            "F-AL1-enacted-act-local-decomposition-must-reproduce-archived-certified-totals-exactly"
                .to_owned(),
        intrinsic_issuance_digest_sealed_before_archive_read: !intrinsic_issuance_digest.is_empty(),
        archive_schema: archive.schema,
        archive_result_digest: archive.result_digest,
        archive_self_digest_valid: self_digest_valid,
        archive_used_only_as_post_issuance_comparator: true,
        intrinsic_nu_vector,
        archived_nu_vector,
        candidate_hash_vector_exact,
        predecessor_signature_vector_exact,
        nu_vector_exact,
        divergences,
        f_al1_triggered,
        f_al1_passed: !f_al1_triggered,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("f-al1-regression-audit", &audit);
    Ok(audit)
}

/// Issue the theorem packages first, seal their digest, then run F-AL1.
/// This function touches only the enacted/reference history and performs no
/// non-enacted Stage-4 continuation.
pub fn issue_t_bi_nu1_regression_certificate()
-> Result<TBiNu1RegressionCertificate, TBiNu1RegressionError> {
    replay_adoption()?;
    let entries = (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();

    // Intrinsic theorem issuance is complete before the archive comparator is
    // called.  This sequencing is part of the typed artifact surface.
    let intrinsic_packages = issue_act_local_sequence(&entries)
        .map_err(|error| TBiNu1RegressionError::Invariant(error.to_string()))?;
    let intrinsic_issuance_digest_before_archive_comparator = tagged_hash(
        "sealed-intrinsic-issuance-before-f-al1",
        &intrinsic_packages
            .iter()
            .map(|package| package.derivation_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let closure_rows = intrinsic_packages
        .iter()
        .map(closure_row)
        .collect::<Vec<_>>();
    let all_fifteen_acts_issued = intrinsic_packages.len() == 15
        && intrinsic_packages
            .iter()
            .map(|package| package.stage)
            .eq(1..=15);
    let all_fifteen_packages_authoritative = intrinsic_packages.iter().all(|package| {
        package.authoritative
            && package.exact_certified_nu as usize == package.ordinary_family_tokens.len()
    });
    let all_hashes = intrinsic_packages
        .iter()
        .flat_map(|package| package.authoritative_token_hashes())
        .collect::<Vec<_>>();
    let authoritative_token_hashes_globally_unique =
        all_hashes.iter().collect::<BTreeSet<_>>().len() == all_hashes.len();
    let structural_filler_classes = structural_filler_coverage(&entries, &intrinsic_packages)?;
    let structural_filler_class_count = structural_filler_classes.len();
    let all_thirteen_structural_filler_classes_covered = structural_filler_class_count == 13
        && structural_filler_classes
            .iter()
            .map(|row| row.registration_stage)
            .eq(3..=15)
        && structural_filler_classes.iter().all(|row| {
            row.filler_package_authoritative && row.filler_tokens_nonempty_when_positive
        });
    let f_al1 = run_f_al1(
        &intrinsic_issuance_digest_before_archive_comparator,
        &intrinsic_packages,
    )?;
    let t_bi_nu1_proved_on_enacted_branch = all_fifteen_acts_issued
        && all_fifteen_packages_authoritative
        && authoritative_token_hashes_globally_unique
        && all_thirteen_structural_filler_classes_covered
        && f_al1.f_al1_passed;
    let mut certificate = TBiNu1RegressionCertificate {
        schema: T_BI_NU1_REGRESSION_SCHEMA.to_owned(),
        date: T_BI_NU1_REGRESSION_DATE.to_owned(),
        theorem_id: T_BI_NU1_THEOREM_ID.to_owned(),
        source_bindings: source_bindings(),
        adoption_replayed: true,
        intrinsic_packages,
        intrinsic_issuance_digest_before_archive_comparator,
        closure_rows,
        all_fifteen_acts_issued,
        all_fifteen_packages_authoritative,
        authoritative_token_hashes_globally_unique,
        structural_filler_classes,
        structural_filler_class_count,
        all_thirteen_structural_filler_classes_covered,
        f_al1,
        t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized_by_t_bi_nu1_side: t_bi_nu1_proved_on_enacted_branch,
        non_enacted_branch_work_executed: false,
        outcome: if t_bi_nu1_proved_on_enacted_branch {
            "T_BI_NU1_intrinsic_decomposition_total_F_AL1_passed_BI0_rerun_authorized_on_this_side"
                .to_owned()
        } else {
            "T_BI_NU1_or_F_AL1_incomplete_BI0_stays_closed".to_owned()
        },
        permitted_conclusion: "Candidate/prefix-local ordinary-charge provenance is total on the enacted fifteen acts and all thirteen structural filler classes; F-AL1 is a post-issuance exact regression only. This artifact alone authorizes a BI-0 rerun, not BI-1 or the cone.".to_owned(),
        required_successor_action: "Complete F-SM1, integrate both prerequisite artifacts into BI-0, and rerun BI-0 create-new. F-BI5 remains armed until that rerun passes.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> TBiNu1RegressionReplay {
    TBiNu1RegressionReplay {
        valid: false,
        all_fifteen_acts_issued: false,
        structural_filler_class_count: 0,
        f_al1_passed: false,
        intrinsic_nu_vector: Vec::new(),
        errors: vec![error.into()],
    }
}

pub fn replay_t_bi_nu1_regression_certificate(
    claimed: &TBiNu1RegressionCertificate,
) -> TBiNu1RegressionReplay {
    let expected = match issue_t_bi_nu1_regression_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-BI-NU1 regression certificate digest mismatch".to_owned());
    }
    if claimed != &expected {
        errors
            .push("T-BI-NU1 regression certificate differs from create-new reissuance".to_owned());
    }
    if claimed.non_enacted_branch_work_executed
        || !claimed.t_bi_nu1_proved_on_enacted_branch
        || !claimed.f_al1.f_al1_passed
    {
        errors.push("T-BI-NU1 conclusion or F-BI sequencing surface is invalid".to_owned());
    }
    TBiNu1RegressionReplay {
        valid: errors.is_empty(),
        all_fifteen_acts_issued: claimed.all_fifteen_acts_issued,
        structural_filler_class_count: claimed.structural_filler_class_count,
        f_al1_passed: claimed.f_al1.f_al1_passed,
        intrinsic_nu_vector: claimed.f_al1.intrinsic_nu_vector.clone(),
        errors,
    }
}

pub fn replay_t_bi_nu1_regression_json(json: &str) -> TBiNu1RegressionReplay {
    match serde_json::from_str::<TBiNu1RegressionCertificate>(json) {
        Ok(certificate) => replay_t_bi_nu1_regression_certificate(&certificate),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn render_t_bi_nu1_report(certificate: &TBiNu1RegressionCertificate) -> String {
    let mut table = String::new();
    for row in &certificate.closure_rows {
        table.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row.stage,
            row.structural_formula_total,
            row.exact_certified_nu,
            row.local_role_token_count,
            row.live_demand_output_token_count,
            row.independent_structural_demand_orbit_count,
        ));
    }
    format!(
        "# T-BI-NU1 act-local provenance result\n\n**Date:** {}. **Outcome:** `{}`.\n\nThe archive-free issuer produced authoritative packages for all fifteen enacted acts and all {} operational structural filler classes. The frozen full-history certificate was opened only after intrinsic issuance digest `{}` had been sealed. F-AL1 passed: **{}**. No non-enacted Stage-4 branch was executed.\n\n| Stage | Formula | Post-R2 nu | Local-role tokens | Live-orbit tokens | Independent structural orbits |\n|---:|---:|---:|---:|---:|---:|\n{}\nIntrinsic vector: `{:?}`.\n\nPermitted conclusion: {}\n\nNext: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.structural_filler_class_count,
        certificate.intrinsic_issuance_digest_before_archive_comparator,
        certificate.f_al1.f_al1_passed,
        table,
        certificate.f_al1.intrinsic_nu_vector,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.result_digest,
    )
}

pub fn emit_t_bi_nu1_regression_create_new(
    directory: &Path,
) -> Result<TBiNu1RegressionCertificate, TBiNu1RegressionError> {
    let certificate = issue_t_bi_nu1_regression_certificate()?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| TBiNu1RegressionError::Json(error.to_string()))?;
    let report = render_t_bi_nu1_report(&certificate);
    let certificate_path = directory.join(T_BI_NU1_CERTIFICATE_NAME);
    let report_path = directory.join(T_BI_NU1_REPORT_NAME);
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| TBiNu1RegressionError::Io(error.to_string()))?;
    certificate_file
        .write_all(json.as_bytes())
        .map_err(|error| TBiNu1RegressionError::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| TBiNu1RegressionError::Io(error.to_string()))?;
    report_file
        .write_all(report.as_bytes())
        .map_err(|error| TBiNu1RegressionError::Io(error.to_string()))?;
    let replay = replay_t_bi_nu1_regression_json(&json);
    if !replay.valid {
        return Err(TBiNu1RegressionError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(certificate)
}

pub fn replay_t_bi_nu1_regression_directory(
    directory: &Path,
) -> Result<TBiNu1RegressionReplay, TBiNu1RegressionError> {
    let json = std::fs::read_to_string(directory.join(T_BI_NU1_CERTIFICATE_NAME))
        .map_err(|error| TBiNu1RegressionError::Io(error.to_string()))?;
    Ok(replay_t_bi_nu1_regression_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_intrinsic_issuance_precedes_and_passes_f_al1() {
        let certificate = issue_t_bi_nu1_regression_certificate().expect("T-BI-NU1 regression");
        assert!(certificate.t_bi_nu1_proved_on_enacted_branch);
        assert!(certificate.all_thirteen_structural_filler_classes_covered);
        assert_eq!(certificate.structural_filler_class_count, 13);
        assert!(certificate.f_al1.f_al1_passed);
        assert_eq!(
            certificate.f_al1.intrinsic_nu_vector,
            vec![1, 1, 2, 5, 7, 8, 10, 17, 17, 19, 26, 34, 46, 62, 103]
        );
        assert!(!certificate.non_enacted_branch_work_executed);
        assert!(replay_t_bi_nu1_regression_certificate(&certificate).valid);
    }

    #[test]
    fn re_signed_token_and_f_al1_mutations_are_rejected() {
        let certificate = issue_t_bi_nu1_regression_certificate().expect("T-BI-NU1 regression");
        let mut token_mutation = certificate.clone();
        token_mutation.intrinsic_packages[14].ordinary_family_tokens[0].archive_join_used = true;
        token_mutation.result_digest = certificate_digest(&token_mutation);
        assert!(!replay_t_bi_nu1_regression_certificate(&token_mutation).valid);

        let mut comparator_mutation = certificate.clone();
        comparator_mutation.f_al1.archived_nu_vector[14] += 1;
        comparator_mutation.f_al1.derivation_hash =
            tagged_hash("f-al1-regression-audit", &comparator_mutation.f_al1);
        comparator_mutation.result_digest = certificate_digest(&comparator_mutation);
        assert!(!replay_t_bi_nu1_regression_certificate(&comparator_mutation).valid);
    }
}
