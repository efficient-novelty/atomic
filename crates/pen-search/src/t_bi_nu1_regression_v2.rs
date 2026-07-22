//! Sound T-BI-NU1 v2 coverage and post-issuance F-AL1 regression.
//!
//! The intrinsic packages are issued and hashed before this module opens the
//! enacted-history archive.  F-AL1 is therefore a falsifying comparator: it
//! can reject T-BI-NU1, but it cannot repair an unanchored family or supply a
//! missing intrinsic count.

use crate::act_local_provenance_v2::{
    ActLocalProvenanceV2Certificate, ActLocalV2TheoremGap, T_BI_NU1_V2_THEOREM_ID,
    issue_act_local_sequence_v2,
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
use pen_eval::semantic_provenance::ProvenanceAnchor;
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const T_BI_NU1_REGRESSION_V2_SCHEMA: &str = "t-bi-nu1-act-local-regression-v2";
pub const T_BI_NU1_REGRESSION_V2_DATE: &str = "2026-07-22";
pub const T_BI_NU1_V2_CERTIFICATE_NAME: &str = "t_bi_nu1_act_local_provenance_v2.json";
pub const T_BI_NU1_V2_REPORT_NAME: &str = "T_BI_NU1_ACT_LOCAL_PROVENANCE_V2_RESULT.md";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/bi0_prerequisites_adjudication.md");
const V1_AUDIT_BURN_BYTES: &[u8] =
    include_bytes!("../../../docs/bi0_prerequisites_v1_audit_burn.md");
const ARCHIVED_HISTORY_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const ACT_LOCAL_V2_SOURCE_BYTES: &[u8] = include_bytes!("act_local_provenance_v2.rs");
const TYPED_FAMILY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/typed_families.rs");
const SEMANTIC_PROVENANCE_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/semantic_provenance.rs");
const DEMAND_ORBIT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/demand_orbits.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_bi_nu1_regression_v2.rs");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_NU1_REGRESSION_V2_SCHEMA, domain, value))
        .expect("T-BI-NU1 v2 evidence serializes");
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
pub struct TBiNu1V2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicIssuerV2Audit {
    pub structural_nu_call_absent_from_intrinsic_source: bool,
    pub archived_history_literal_absent_from_intrinsic_source: bool,
    pub archive_include_absent_from_intrinsic_source: bool,
    pub global_or_enacted_timeline_call_absent_from_intrinsic_source: bool,
    pub all_packages_report_scalar_not_used: bool,
    pub all_packages_report_archive_absent_during_issuance: bool,
    pub all_packages_report_global_timeline_not_read: bool,
    pub conditional_at_most_checker_invoked_for_every_stage: bool,
    pub candidate_minted_required_output_position_count: usize,
    pub used_actual_pre_existing_output_positions_unique: bool,
    pub used_actual_pre_existing_output_positions_globally_unique: bool,
    pub actual_local_role_anchors_unique_per_candidate: bool,
    pub uniform_instances_not_multiplied: bool,
    pub source_and_runtime_audit_passed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV2ClosureRow {
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub kappa: u16,
    pub extracted_natural_family_count: usize,
    pub marginal_natural_family_count: usize,
    pub credited_natural_family_count: usize,
    pub collapsed_non_generator_instance_count: usize,
    pub independently_exported_instance_credit_count: usize,
    pub certified_marginal_nu: Option<u32>,
    pub conditional_local_capacity: Option<u32>,
    pub used_pre_existing_required_output_position_count: usize,
    pub candidate_minted_required_output_position_count: usize,
    pub theorem_gap_ids: Vec<String>,
    pub package_authoritative: bool,
    pub package_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralFillerClassCoverageV2 {
    pub registration_stage: u32,
    pub jurisdiction_filler_stage: u32,
    pub a3_scheme_id: String,
    pub a3_instance_id: String,
    pub constructor: String,
    pub registration_formation_hash: String,
    pub filler_package_hash: String,
    pub filler_certified_marginal_nu: Option<u32>,
    pub filler_theorem_gap_ids: Vec<String>,
    pub filler_package_authoritative: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FAl1V2Divergence {
    pub stage: u32,
    pub field: String,
    pub intrinsic_value: String,
    pub archived_value: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FAl1V2RegressionAudit {
    pub falsifier: String,
    pub intrinsic_issuance_digest_sealed_before_archive_read: bool,
    pub archive_schema: String,
    pub archive_result_digest: String,
    pub archive_self_digest_valid: bool,
    pub archive_used_only_as_post_issuance_comparator: bool,
    pub intrinsic_nu_vector: Vec<Option<u32>>,
    pub archived_nu_vector: Vec<u32>,
    pub candidate_hash_vector_exact: bool,
    pub predecessor_signature_vector_exact: bool,
    pub nu_vector_exact: bool,
    pub divergences: Vec<FAl1V2Divergence>,
    pub first_exact_divergence: Option<FAl1V2Divergence>,
    pub f_al1_triggered: bool,
    pub f_al1_passed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionV2Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub source_bindings: Vec<TBiNu1V2SourceBinding>,
    pub adoption_and_v1_audit_burn_replayed: bool,
    pub intrinsic_packages: Vec<ActLocalProvenanceV2Certificate>,
    pub intrinsic_issuance_digest_before_archive_comparator: String,
    pub intrinsic_issuer_audit: IntrinsicIssuerV2Audit,
    pub closure_rows: Vec<ActLocalV2ClosureRow>,
    pub all_fifteen_acts_issued: bool,
    pub authoritative_package_count: usize,
    pub all_fifteen_packages_authoritative: bool,
    pub theorem_gaps: Vec<ActLocalV2TheoremGap>,
    pub theorem_gap_count: usize,
    pub structural_filler_classes: Vec<StructuralFillerClassCoverageV2>,
    pub structural_filler_class_count: usize,
    pub all_thirteen_structural_filler_classes_covered: bool,
    pub intrinsic_theorem_total_before_f_al1: bool,
    pub f_al1: FAl1V2RegressionAudit,
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
pub struct TBiNu1RegressionV2Replay {
    pub valid: bool,
    pub theorem_proved: bool,
    pub bi0_rerun_authorized: bool,
    pub all_fifteen_acts_issued: bool,
    pub authoritative_package_count: usize,
    pub structural_filler_class_count: usize,
    pub f_al1_passed: bool,
    pub intrinsic_nu_vector: Vec<Option<u32>>,
    pub first_exact_divergence: Option<FAl1V2Divergence>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiNu1RegressionV2Error {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("T-BI-NU1 v2 invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted replay failed: {0}")]
    EmittedReplay(String),
}

fn source_bindings() -> Vec<TBiNu1V2SourceBinding> {
    [
        (
            "docs/bi0_prerequisites_adjudication.md",
            "adopted_act_local_rule_and_F_AL1_falsifier",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/bi0_prerequisites_v1_audit_burn.md",
            "audit_occasion_only_never_semantic_input",
            V1_AUDIT_BURN_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_provenance_v2.rs",
            "archive_free_extraction_first_intrinsic_issuer",
            ACT_LOCAL_V2_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/typed_families.rs",
            "typed_natural_family_extraction_and_marginality",
            TYPED_FAMILY_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/semantic_provenance.rs",
            "injective_conditional_at_most_checker",
            SEMANTIC_PROVENANCE_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/demand_orbits.rs",
            "pre_existing_finite_demand_output_inventory",
            DEMAND_ORBIT_SOURCE_BYTES,
        ),
        (
            "docs/phase5b_full_history_v1.json",
            "post_issuance_F_AL1_comparator_only",
            ARCHIVED_HISTORY_BYTES,
        ),
        (
            "crates/pen-search/src/t_bi_nu1_regression_v2.rs",
            "separate_comparator_replay_and_create_new_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| TBiNu1V2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn replay_adoption_and_burn() -> Result<(), TBiNu1RegressionV2Error> {
    let adoption = std::str::from_utf8(ADJUDICATION_BYTES)
        .map_err(|error| TBiNu1RegressionV2Error::Prerequisite(error.to_string()))?;
    for clause in [
        "act-local-provenance-v1",
        "candidate and its sealed prefix alone",
        "T-BI-NU1",
        "F-AL1",
        "Archived full-history joins are",
    ] {
        if !adoption.contains(clause) {
            return Err(TBiNu1RegressionV2Error::Prerequisite(format!(
                "adjudication omits {clause:?}"
            )));
        }
    }
    let burn = std::str::from_utf8(V1_AUDIT_BURN_BYTES)
        .map_err(|error| TBiNu1RegressionV2Error::Prerequisite(error.to_string()))?;
    for clause in [
        "BURNED AS BI-0 AUTHORITY",
        "candidate-minted demand outputs",
        "v1 prerequisite artifacts are forbidden BI-0 inputs",
        "publish F-AL1 and F-SM1 outcomes",
    ] {
        if !burn.contains(clause) {
            return Err(TBiNu1RegressionV2Error::Prerequisite(format!(
                "v1 audit burn omits {clause:?}"
            )));
        }
    }
    Ok(())
}

fn certificate_digest(certificate: &TBiNu1RegressionV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("t-bi-nu1-v2-regression-certificate", &projection)
}

fn closure_row(package: &ActLocalProvenanceV2Certificate) -> ActLocalV2ClosureRow {
    let mut row = ActLocalV2ClosureRow {
        stage: package.stage,
        candidate_hash: package.candidate_hash.clone(),
        predecessor_signature_digest: package.predecessor_signature_digest.clone(),
        kappa: package.kappa,
        extracted_natural_family_count: package.extracted_natural_family_count,
        marginal_natural_family_count: package.marginal_natural_family_count,
        credited_natural_family_count: package.credited_natural_family_count,
        collapsed_non_generator_instance_count: package.collapsed_non_generator_instance_count,
        independently_exported_instance_credit_count: package
            .independently_exported_instance_credit_count,
        certified_marginal_nu: package.certified_marginal_nu,
        conditional_local_capacity: package
            .conditional_at_most
            .as_ref()
            .map(|bound| bound.local_capacity),
        used_pre_existing_required_output_position_count: package
            .used_pre_existing_required_output_position_count,
        candidate_minted_required_output_position_count: package
            .candidate_minted_required_output_position_count,
        theorem_gap_ids: package
            .theorem_gaps
            .iter()
            .map(|gap| gap.id.clone())
            .collect(),
        package_authoritative: package.authoritative,
        package_derivation_hash: package.derivation_hash.clone(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("act-local-v2-closure-row", &row);
    row
}

fn intrinsic_issuer_audit(packages: &[ActLocalProvenanceV2Certificate]) -> IntrinsicIssuerV2Audit {
    let source = std::str::from_utf8(ACT_LOCAL_V2_SOURCE_BYTES).unwrap_or("");
    let structural_nu_call_absent_from_intrinsic_source = !source.contains("structural_nu(")
        && !source.contains("pen_eval::nu::")
        && !source.contains("crate::nu::");
    let archived_history_literal_absent_from_intrinsic_source =
        !source.contains("phase5b_full_history") && !source.contains("certified_semantic_total");
    let archive_include_absent_from_intrinsic_source =
        !source.contains("include_bytes!") && !source.contains("include_str!");
    let global_or_enacted_timeline_call_absent_from_intrinsic_source =
        !source.contains("directive_debt_timeline(");
    let all_packages_report_scalar_not_used = packages.iter().all(|package| {
        !package.structural_nu_called_by_intrinsic_issuer
            && !package.historical_score_or_total_read_by_intrinsic_issuer
            && !package.bar_or_verdict_read_by_intrinsic_issuer
    });
    let all_packages_report_archive_absent_during_issuance = packages
        .iter()
        .all(|package| !package.archive_available_during_intrinsic_issuance);
    let all_packages_report_global_timeline_not_read = packages
        .iter()
        .all(|package| !package.enacted_or_global_demand_timeline_read_by_intrinsic_issuer);
    let conditional_at_most_checker_invoked_for_every_stage = packages
        .iter()
        .all(|package| package.conditional_at_most.is_some());
    let candidate_minted_required_output_position_count = packages
        .iter()
        .map(|package| package.candidate_minted_required_output_position_count)
        .sum();
    let used_actual_pre_existing_output_positions_unique = packages.iter().all(|package| {
        package.pre_existing_required_output_positions_unique
            && package.used_pre_existing_required_output_positions_unique
    });
    let all_used_outputs = packages
        .iter()
        .flat_map(|package| &package.checker_submission.credited_families)
        .filter_map(|credit| match &credit.anchor {
            ProvenanceAnchor::PreExistingDemandOutput { demand } => Some(demand),
            ProvenanceAnchor::ChargedKernel { .. } => None,
        })
        .collect::<Vec<_>>();
    let used_actual_pre_existing_output_positions_globally_unique = all_used_outputs
        .iter()
        .map(|demand| (*demand).clone())
        .collect::<BTreeSet<_>>()
        .len()
        == all_used_outputs.len();
    let actual_local_role_anchors_unique_per_candidate = packages
        .iter()
        .all(|package| package.actual_local_role_anchors_unique);
    let uniform_instances_not_multiplied = packages
        .iter()
        .all(|package| package.uniform_or_repeated_instances_not_multiplied);
    let source_and_runtime_audit_passed = structural_nu_call_absent_from_intrinsic_source
        && archived_history_literal_absent_from_intrinsic_source
        && archive_include_absent_from_intrinsic_source
        && global_or_enacted_timeline_call_absent_from_intrinsic_source
        && all_packages_report_scalar_not_used
        && all_packages_report_archive_absent_during_issuance
        && all_packages_report_global_timeline_not_read
        && conditional_at_most_checker_invoked_for_every_stage
        && candidate_minted_required_output_position_count == 0
        && used_actual_pre_existing_output_positions_unique
        && used_actual_pre_existing_output_positions_globally_unique
        && actual_local_role_anchors_unique_per_candidate
        && uniform_instances_not_multiplied;
    let mut audit = IntrinsicIssuerV2Audit {
        structural_nu_call_absent_from_intrinsic_source,
        archived_history_literal_absent_from_intrinsic_source,
        archive_include_absent_from_intrinsic_source,
        global_or_enacted_timeline_call_absent_from_intrinsic_source,
        all_packages_report_scalar_not_used,
        all_packages_report_archive_absent_during_issuance,
        all_packages_report_global_timeline_not_read,
        conditional_at_most_checker_invoked_for_every_stage,
        candidate_minted_required_output_position_count,
        used_actual_pre_existing_output_positions_unique,
        used_actual_pre_existing_output_positions_globally_unique,
        actual_local_role_anchors_unique_per_candidate,
        uniform_instances_not_multiplied,
        source_and_runtime_audit_passed,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("intrinsic-issuer-v2-audit", &audit);
    audit
}

fn structural_filler_coverage(
    entries: &[(u32, Telescope)],
    packages: &[ActLocalProvenanceV2Certificate],
) -> Result<Vec<StructuralFillerClassCoverageV2>, TBiNu1RegressionV2Error> {
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
                .map_err(|error| TBiNu1RegressionV2Error::Invariant(error.to_string()))?;
        let proof = prove_a3_window_inventory_for_exact_prefix_unbounded(
            &demand_prefix,
            registration_stage,
        )
        .map_err(|error| TBiNu1RegressionV2Error::Invariant(error.to_string()))?;
        if !proof.relative_rule_inventory_exhaustive_for_window {
            return Err(TBiNu1RegressionV2Error::Invariant(format!(
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
                TBiNu1RegressionV2Error::Invariant(format!(
                    "A3 instance {} has no scheme",
                    instance.instance_id
                ))
            })?;
            if scheme.rule_constructor != A3RuleConstructor::StructuralCompletionHole {
                continue;
            }
            let A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } = scheme.origin
            else {
                return Err(TBiNu1RegressionV2Error::Invariant(format!(
                    "structural scheme {} has a base origin",
                    scheme.scheme_id
                )));
            };
            let registration_prefix = if registration_stage == 3 {
                SealedSignature::from_telescopes(entries[..3].to_vec())
            } else {
                demand_prefix.clone()
            };
            let registration =
                register_structural_future_hole_v2(&registration_prefix, &window, scheme, instance)
                    .map_err(|error| TBiNu1RegressionV2Error::Invariant(error.to_string()))?;
            let FutureHoleRegistrationDispositionV2::Registered(registration) = registration else {
                let FutureHoleRegistrationDispositionV2::Gap(gap) = registration else {
                    unreachable!()
                };
                return Err(TBiNu1RegressionV2Error::Invariant(format!(
                    "structural filler registration {} remained gap {}: {}",
                    instance.instance_id, gap.id, gap.exact_error
                )));
            };
            let FutureHoleOutputContractV2::StructuralProvides(contract) =
                &registration.output_contract
            else {
                return Err(TBiNu1RegressionV2Error::Invariant(format!(
                    "structural registration {} has a unary contract",
                    instance.instance_id
                )));
            };
            let filler_stage = contract.jurisdiction.jurisdiction_stage();
            let filler = package_by_stage.get(&filler_stage).ok_or_else(|| {
                TBiNu1RegressionV2Error::Invariant(format!(
                    "structural class at Stage {registration_stage} requires absent filler Stage {filler_stage}"
                ))
            })?;
            let mut row = StructuralFillerClassCoverageV2 {
                registration_stage,
                jurisdiction_filler_stage: filler_stage,
                a3_scheme_id: scheme.scheme_id.clone(),
                a3_instance_id: instance.instance_id.clone(),
                constructor: constructor.slug().to_owned(),
                registration_formation_hash: registration.formation_hash,
                filler_package_hash: filler.derivation_hash.clone(),
                filler_certified_marginal_nu: filler.certified_marginal_nu,
                filler_theorem_gap_ids: filler
                    .theorem_gaps
                    .iter()
                    .map(|gap| gap.id.clone())
                    .collect(),
                filler_package_authoritative: filler.authoritative,
                derivation_hash: String::new(),
            };
            row.derivation_hash = tagged_hash("structural-filler-class-coverage-v2", &row);
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
) -> FAl1V2Divergence {
    let mut row = FAl1V2Divergence {
        stage,
        field: field.to_owned(),
        intrinsic_value: intrinsic_value.to_string(),
        archived_value: archived_value.to_string(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("f-al1-v2-divergence", &row);
    row
}

fn option_nu(value: Option<u32>) -> String {
    value.map_or_else(|| "THEOREM_GAP".to_owned(), |value| value.to_string())
}

fn run_f_al1(
    intrinsic_issuance_digest: &str,
    packages: &[ActLocalProvenanceV2Certificate],
) -> Result<FAl1V2RegressionAudit, TBiNu1RegressionV2Error> {
    // The archive is first deserialized here, after intrinsic issuance and
    // sealing in the caller.  No result flows back into the intrinsic issuer.
    let archive: Phase5bHistoryCertificate = serde_json::from_slice(ARCHIVED_HISTORY_BYTES)
        .map_err(|error| TBiNu1RegressionV2Error::Json(error.to_string()))?;
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
        if package.certified_marginal_nu != Some(archived.certified_semantic_total) {
            divergences.push(divergence(
                package.stage,
                "certified_semantic_total",
                option_nu(package.certified_marginal_nu),
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
    divergences.sort_by(|left, right| {
        (left.stage, left.field.as_str()).cmp(&(right.stage, right.field.as_str()))
    });
    let first_exact_divergence = divergences.first().cloned();
    let intrinsic_nu_vector = packages
        .iter()
        .map(|package| package.certified_marginal_nu)
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
    let nu_vector_exact = packages.len() == archive.steps.len()
        && packages.iter().zip(&archive.steps).all(|(left, right)| {
            left.stage == right.step
                && left.certified_marginal_nu == Some(right.certified_semantic_total)
        });
    let f_al1_triggered = !self_digest_valid || !divergences.is_empty();
    let mut audit = FAl1V2RegressionAudit {
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
        first_exact_divergence,
        f_al1_triggered,
        f_al1_passed: !f_al1_triggered,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("f-al1-v2-regression-audit", &audit);
    Ok(audit)
}

/// Issue the archive-free theorem packages, seal their digest, and only then
/// execute the enacted F-AL1 comparator.  A failed theorem is a valid result
/// artifact and never authorizes BI-0.
pub fn issue_t_bi_nu1_regression_v2_certificate()
-> Result<TBiNu1RegressionV2Certificate, TBiNu1RegressionV2Error> {
    replay_adoption_and_burn()?;
    let entries = (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();

    let intrinsic_packages = issue_act_local_sequence_v2(&entries)
        .map_err(|error| TBiNu1RegressionV2Error::Invariant(error.to_string()))?;
    let intrinsic_issuance_digest_before_archive_comparator = tagged_hash(
        "sealed-intrinsic-v2-issuance-before-f-al1",
        &intrinsic_packages
            .iter()
            .map(|package| package.derivation_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let intrinsic_issuer_audit = intrinsic_issuer_audit(&intrinsic_packages);
    let closure_rows = intrinsic_packages
        .iter()
        .map(closure_row)
        .collect::<Vec<_>>();
    let all_fifteen_acts_issued = intrinsic_packages.len() == 15
        && intrinsic_packages
            .iter()
            .map(|package| package.stage)
            .eq(1..=15);
    let authoritative_package_count = intrinsic_packages
        .iter()
        .filter(|package| package.authoritative)
        .count();
    let all_fifteen_packages_authoritative = authoritative_package_count == 15;
    let theorem_gaps = intrinsic_packages
        .iter()
        .flat_map(|package| package.theorem_gaps.clone())
        .collect::<Vec<_>>();
    let theorem_gap_count = theorem_gaps.len();
    let structural_filler_classes = structural_filler_coverage(&entries, &intrinsic_packages)?;
    let structural_filler_class_count = structural_filler_classes.len();
    let all_thirteen_structural_filler_classes_covered = structural_filler_class_count == 13
        && structural_filler_classes
            .iter()
            .map(|row| row.registration_stage)
            .eq(3..=15)
        && structural_filler_classes
            .iter()
            .all(|row| row.filler_package_authoritative);
    let intrinsic_theorem_total_before_f_al1 = all_fifteen_acts_issued
        && all_fifteen_packages_authoritative
        && theorem_gaps.is_empty()
        && all_thirteen_structural_filler_classes_covered
        && intrinsic_issuer_audit.source_and_runtime_audit_passed;

    let f_al1 = run_f_al1(
        &intrinsic_issuance_digest_before_archive_comparator,
        &intrinsic_packages,
    )?;
    let t_bi_nu1_proved_on_enacted_branch =
        intrinsic_theorem_total_before_f_al1 && f_al1.f_al1_passed;
    let outcome = if t_bi_nu1_proved_on_enacted_branch {
        "T_BI_NU1_v2_total_and_F_AL1_passed_BI0_rerun_authorized_on_this_side"
    } else if !intrinsic_theorem_total_before_f_al1 {
        "T_BI_NU1_v2_intrinsic_theorem_gap_BI0_stays_closed"
    } else {
        "F_AL1_v2_exact_regression_failed_BI0_stays_closed"
    };
    let mut certificate = TBiNu1RegressionV2Certificate {
        schema: T_BI_NU1_REGRESSION_V2_SCHEMA.to_owned(),
        date: T_BI_NU1_REGRESSION_V2_DATE.to_owned(),
        theorem_id: T_BI_NU1_V2_THEOREM_ID.to_owned(),
        source_bindings: source_bindings(),
        adoption_and_v1_audit_burn_replayed: true,
        intrinsic_packages,
        intrinsic_issuance_digest_before_archive_comparator,
        intrinsic_issuer_audit,
        closure_rows,
        all_fifteen_acts_issued,
        authoritative_package_count,
        all_fifteen_packages_authoritative,
        theorem_gaps,
        theorem_gap_count,
        structural_filler_classes,
        structural_filler_class_count,
        all_thirteen_structural_filler_classes_covered,
        intrinsic_theorem_total_before_f_al1,
        f_al1,
        t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized_by_t_bi_nu1_side: t_bi_nu1_proved_on_enacted_branch,
        non_enacted_branch_work_executed: false,
        outcome: outcome.to_owned(),
        permitted_conclusion: if t_bi_nu1_proved_on_enacted_branch {
            "The extraction-first candidate/prefix-local decomposition is total on the enacted acts and structural fillers and independently passes F-AL1. This artifact authorizes only the T-BI-NU1 side of a BI-0 rerun."
        } else {
            "The v2 computation is a replayable negative prerequisite result. Any named intrinsic gap or F-AL1 divergence keeps T-BI-NU1 unproved and BI-0 closed; the archive was not used to repair issuance."
        }
        .to_owned(),
        required_successor_action: if t_bi_nu1_proved_on_enacted_branch {
            "Join this passing artifact with a passing chronological slot-map v2 artifact and rerun BI-0 create-new; execute no BI-1 work before that pass."
        } else {
            "Resolve the first named intrinsic theorem gap or exact F-AL1 divergence in a versioned successor; do not force the sealed vector and do not run BI."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> TBiNu1RegressionV2Replay {
    TBiNu1RegressionV2Replay {
        valid: false,
        theorem_proved: false,
        bi0_rerun_authorized: false,
        all_fifteen_acts_issued: false,
        authoritative_package_count: 0,
        structural_filler_class_count: 0,
        f_al1_passed: false,
        intrinsic_nu_vector: Vec::new(),
        first_exact_divergence: None,
        errors: vec![error.into()],
    }
}

pub fn replay_t_bi_nu1_regression_v2_certificate(
    claimed: &TBiNu1RegressionV2Certificate,
) -> TBiNu1RegressionV2Replay {
    let expected = match issue_t_bi_nu1_regression_v2_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-BI-NU1 v2 regression certificate digest mismatch".to_owned());
    }
    if claimed != &expected {
        errors.push("T-BI-NU1 v2 certificate differs from create-new reissuance".to_owned());
    }
    let logically_proved =
        claimed.intrinsic_theorem_total_before_f_al1 && claimed.f_al1.f_al1_passed;
    if claimed.t_bi_nu1_proved_on_enacted_branch != logically_proved
        || claimed.bi0_rerun_authorized_by_t_bi_nu1_side != logically_proved
        || claimed.non_enacted_branch_work_executed
    {
        errors.push("T-BI-NU1 v2 conclusion or F-BI sequencing surface is invalid".to_owned());
    }
    TBiNu1RegressionV2Replay {
        valid: errors.is_empty(),
        theorem_proved: claimed.t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized: claimed.bi0_rerun_authorized_by_t_bi_nu1_side,
        all_fifteen_acts_issued: claimed.all_fifteen_acts_issued,
        authoritative_package_count: claimed.authoritative_package_count,
        structural_filler_class_count: claimed.structural_filler_class_count,
        f_al1_passed: claimed.f_al1.f_al1_passed,
        intrinsic_nu_vector: claimed.f_al1.intrinsic_nu_vector.clone(),
        first_exact_divergence: claimed.f_al1.first_exact_divergence.clone(),
        errors,
    }
}

pub fn replay_t_bi_nu1_regression_v2_json(json: &str) -> TBiNu1RegressionV2Replay {
    match serde_json::from_str::<TBiNu1RegressionV2Certificate>(json) {
        Ok(certificate) => replay_t_bi_nu1_regression_v2_certificate(&certificate),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn render_t_bi_nu1_v2_report(certificate: &TBiNu1RegressionV2Certificate) -> String {
    let mut table = String::new();
    for row in &certificate.closure_rows {
        table.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row.stage,
            row.kappa,
            row.extracted_natural_family_count,
            row.marginal_natural_family_count,
            row.credited_natural_family_count,
            option_nu(row.certified_marginal_nu),
            row.theorem_gap_ids.len(),
        ));
    }
    let first_divergence = certificate
        .f_al1
        .first_exact_divergence
        .as_ref()
        .map_or_else(
            || "none".to_owned(),
            |row| {
                format!(
                    "Stage {}, `{}`: intrinsic `{}`, archive `{}`",
                    row.stage, row.field, row.intrinsic_value, row.archived_value
                )
            },
        );
    format!(
        "# T-BI-NU1 act-local provenance v2 result\n\n**Date:** {}. **Outcome:** `{}`.\n\nThe intrinsic issuer ran before the archive comparator. It issued {}/15 authoritative packages, minted {} candidate demand-output positions, and covered {}/13 structural filler classes authoritatively. The conditional-at-most checker ran at every stage: **{}**. T-BI-NU1 is proved: **{}**. BI-0 authorization from this side: **{}**.\n\n| Stage | kappa | Extracted families | Marginal families | Credited families | Intrinsic nu | Gaps |\n|---:|---:|---:|---:|---:|---:|---:|\n{}\nIntrinsic vector: `{:?}`.\n\nArchived vector (post-issuance comparator only): `{:?}`.\n\nF-AL1 passed: **{}**. First exact divergence: {}.\n\nPermitted conclusion: {}\n\nNext: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.authoritative_package_count,
        certificate
            .intrinsic_issuer_audit
            .candidate_minted_required_output_position_count,
        certificate
            .structural_filler_classes
            .iter()
            .filter(|row| row.filler_package_authoritative)
            .count(),
        certificate
            .intrinsic_issuer_audit
            .conditional_at_most_checker_invoked_for_every_stage,
        certificate.t_bi_nu1_proved_on_enacted_branch,
        certificate.bi0_rerun_authorized_by_t_bi_nu1_side,
        table,
        certificate.f_al1.intrinsic_nu_vector,
        certificate.f_al1.archived_nu_vector,
        certificate.f_al1.f_al1_passed,
        first_divergence,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.result_digest,
    )
}

pub fn emit_t_bi_nu1_regression_v2_create_new(
    directory: &Path,
) -> Result<TBiNu1RegressionV2Certificate, TBiNu1RegressionV2Error> {
    let certificate = issue_t_bi_nu1_regression_v2_certificate()?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| TBiNu1RegressionV2Error::Json(error.to_string()))?;
    let report = render_t_bi_nu1_v2_report(&certificate);
    let replay = replay_t_bi_nu1_regression_v2_json(&json);
    if !replay.valid {
        return Err(TBiNu1RegressionV2Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let certificate_path = directory.join(T_BI_NU1_V2_CERTIFICATE_NAME);
    let report_path = directory.join(T_BI_NU1_V2_REPORT_NAME);
    if certificate_path.exists() || report_path.exists() {
        return Err(TBiNu1RegressionV2Error::Io(format!(
            "create-new target already exists: {} or {}",
            certificate_path.display(),
            report_path.display()
        )));
    }
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| TBiNu1RegressionV2Error::Io(error.to_string()))?;
    certificate_file
        .write_all(json.as_bytes())
        .map_err(|error| TBiNu1RegressionV2Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| TBiNu1RegressionV2Error::Io(error.to_string()))?;
    report_file
        .write_all(report.as_bytes())
        .map_err(|error| TBiNu1RegressionV2Error::Io(error.to_string()))?;
    Ok(certificate)
}

pub fn replay_t_bi_nu1_regression_v2_directory(
    directory: &Path,
) -> Result<TBiNu1RegressionV2Replay, TBiNu1RegressionV2Error> {
    let json = std::fs::read_to_string(directory.join(T_BI_NU1_V2_CERTIFICATE_NAME))
        .map_err(|error| TBiNu1RegressionV2Error::Io(error.to_string()))?;
    Ok(replay_t_bi_nu1_regression_v2_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v2_result_replays_without_forcing_the_sealed_vector() {
        let certificate =
            issue_t_bi_nu1_regression_v2_certificate().expect("T-BI-NU1 v2 regression");
        assert!(certificate.all_fifteen_acts_issued);
        assert_eq!(certificate.structural_filler_class_count, 13);
        assert_eq!(
            certificate
                .intrinsic_issuer_audit
                .candidate_minted_required_output_position_count,
            0
        );
        assert!(
            certificate
                .intrinsic_issuer_audit
                .conditional_at_most_checker_invoked_for_every_stage
        );
        assert!(
            certificate
                .intrinsic_issuer_audit
                .used_actual_pre_existing_output_positions_globally_unique
        );
        assert_eq!(
            certificate.bi0_rerun_authorized_by_t_bi_nu1_side,
            certificate.t_bi_nu1_proved_on_enacted_branch
        );
        assert!(replay_t_bi_nu1_regression_v2_certificate(&certificate).valid);
    }

    #[test]
    fn re_signed_intrinsic_and_comparator_mutations_are_rejected() {
        let certificate =
            issue_t_bi_nu1_regression_v2_certificate().expect("T-BI-NU1 v2 regression");
        let mut intrinsic_mutation = certificate.clone();
        intrinsic_mutation.intrinsic_packages[0].candidate_minted_required_output_position_count =
            1;
        intrinsic_mutation.result_digest = certificate_digest(&intrinsic_mutation);
        assert!(!replay_t_bi_nu1_regression_v2_certificate(&intrinsic_mutation).valid);

        let mut comparator_mutation = certificate.clone();
        comparator_mutation.f_al1.first_exact_divergence = None;
        comparator_mutation.f_al1.derivation_hash =
            tagged_hash("f-al1-v2-regression-audit", &comparator_mutation.f_al1);
        comparator_mutation.result_digest = certificate_digest(&comparator_mutation);
        assert!(!replay_t_bi_nu1_regression_v2_certificate(&comparator_mutation).valid);
    }
}
