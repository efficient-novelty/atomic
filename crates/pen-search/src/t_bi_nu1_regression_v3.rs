//! T-BI-NU1 v3 create-new/replay surface and post-seal F-AL1 comparator.
//!
//! The intrinsic packages are issued and hashed before this module opens the
//! historical certificate.  The archive can therefore falsify v3 but cannot
//! contribute a family, orbit, output coordinate, or count to issuance.

use crate::act_local_provenance_v3::{
    ACT_LOCAL_PROVENANCE_V3_SCHEMA, ActLocalProvenanceV3Certificate, ActLocalV3Gap,
    T_BI_NU1_V3_THEOREM_ID, issue_act_local_sequence_v3,
};
use crate::phase5b_history_certification::{
    PHASE5B_HISTORY_CERT_SCHEMA, Phase5bHistoryCertificate,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const T_BI_NU1_REGRESSION_V3_SCHEMA: &str = "t-bi-nu1-act-local-regression-v3";
pub const T_BI_NU1_REGRESSION_V3_DATE: &str = "2026-07-22";
pub const T_BI_NU1_V3_CERTIFICATE_NAME: &str = "t_bi_nu1_act_local_provenance_v3.json";
pub const T_BI_NU1_V3_REPORT_NAME: &str = "T_BI_NU1_ACT_LOCAL_PROVENANCE_V3_RESULT.md";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/bi0_prerequisites_adjudication.md");
const V1_BURN_BYTES: &[u8] = include_bytes!("../../../docs/bi0_prerequisites_v1_audit_burn.md");
const ARCHIVED_HISTORY_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const ACT_LOCAL_V3_SOURCE_BYTES: &[u8] = include_bytes!("act_local_provenance_v3.rs");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const A3_EXHAUSTIVENESS_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/a3_rule_inventory_exhaustiveness.rs");
const TYPED_FAMILY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/typed_families.rs");
const SEMANTIC_PROVENANCE_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/semantic_provenance.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_bi_nu1_regression_v3.rs");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_NU1_REGRESSION_V3_SCHEMA, domain, value))
        .expect("T-BI-NU1 v3 evidence serializes");
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
pub struct TBiNu1V3SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

fn source_bindings() -> Vec<TBiNu1V3SourceBinding> {
    [
        (
            "docs/bi0_prerequisites_adjudication.md",
            "adopted_act_local_rule_and_F_AL1_falsifier",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/bi0_prerequisites_v1_audit_burn.md",
            "prior_unsound_issuers_burned_as_authority",
            V1_BURN_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_provenance_v3.rs",
            "candidate_and_prefix_only_full_A3_issuer",
            ACT_LOCAL_V3_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "full_pre_candidate_typed_scheme_instance_orbit_surface",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_rule_inventory_exhaustiveness.rs",
            "exact_prefix_relative_rule_inventory_exhaustiveness_theorem",
            A3_EXHAUSTIVENESS_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/typed_families.rs",
            "normalization_weakening_natural_family_and_marginality_quotient",
            TYPED_FAMILY_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/semantic_provenance.rs",
            "blind_four_local_role_inventory_and_export_law",
            SEMANTIC_PROVENANCE_SOURCE_BYTES,
        ),
        (
            "docs/phase5b_full_history_v1.json",
            "post_issuance_F_AL1_comparator_only",
            ARCHIVED_HISTORY_BYTES,
        ),
        (
            "crates/pen-search/src/t_bi_nu1_regression_v3.rs",
            "this_post_seal_regression_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| TBiNu1V3SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicIssuerV3Audit {
    pub intrinsic_schema: String,
    pub legacy_novelty_module_import_absent: bool,
    pub legacy_novelty_scalar_call_absent: bool,
    pub phase5b_archive_literal_absent: bool,
    pub phase5b_certificate_type_absent: bool,
    pub all_packages_report_forbidden_inputs_absent: bool,
    pub all_packages_mint_zero_output_positions: bool,
    pub all_packages_sweep_full_pre_candidate_a3_surface: bool,
    pub all_packages_apply_family_instance_quotient: bool,
    pub all_packages_preserve_a3_generator_output_boundary: bool,
    pub source_and_runtime_audit_passed: bool,
    pub derivation_hash: String,
}

fn intrinsic_issuer_audit(packages: &[ActLocalProvenanceV3Certificate]) -> IntrinsicIssuerV3Audit {
    let source = std::str::from_utf8(ACT_LOCAL_V3_SOURCE_BYTES).unwrap_or_default();
    let legacy_novelty_module_import_absent = !source.contains("use pen_eval::nu");
    let legacy_novelty_scalar_call_absent = !source.contains("structural_nu(");
    let phase5b_archive_literal_absent = !source.contains("phase5b_full_history");
    let phase5b_certificate_type_absent = !source.contains("Phase5bHistoryCertificate");
    let all_packages_report_forbidden_inputs_absent = packages.iter().all(|package| {
        !package.forbidden_inputs.archive_read
            && !package.forbidden_inputs.historical_total_read
            && !package.forbidden_inputs.legacy_novelty_scalar_called
            && !package.forbidden_inputs.bar_read
            && !package.forbidden_inputs.verdict_read
            && !package.forbidden_inputs.enacted_future_read
            && !package
                .forbidden_inputs
                .caller_supplied_demand_timeline_read
    });
    let all_packages_mint_zero_output_positions = packages
        .iter()
        .all(|package| package.candidate_minted_output_position_count == 0);
    let all_packages_sweep_full_pre_candidate_a3_surface = packages.iter().all(|package| {
        package.full_pre_candidate_a3_surface_replayed
            && package.a3_orbit_count == package.a3_orbits.len()
    });
    let all_packages_apply_family_instance_quotient = packages
        .iter()
        .all(|package| package.uniform_specializations_not_multiplied);
    let all_packages_preserve_a3_generator_output_boundary = packages.iter().all(|package| {
        !package.a3_output_terms_constructed_on_generator_boundary
            && !package.a3_output_terms_kernel_typed_on_generator_boundary
            && package
                .a3_orbits
                .iter()
                .all(|orbit| !orbit.usable_as_ordinary_credit_without_new_theorem)
    });
    let source_and_runtime_audit_passed = legacy_novelty_module_import_absent
        && legacy_novelty_scalar_call_absent
        && phase5b_archive_literal_absent
        && phase5b_certificate_type_absent
        && all_packages_report_forbidden_inputs_absent
        && all_packages_mint_zero_output_positions
        && all_packages_sweep_full_pre_candidate_a3_surface
        && all_packages_apply_family_instance_quotient
        && all_packages_preserve_a3_generator_output_boundary;
    let mut audit = IntrinsicIssuerV3Audit {
        intrinsic_schema: ACT_LOCAL_PROVENANCE_V3_SCHEMA.to_owned(),
        legacy_novelty_module_import_absent,
        legacy_novelty_scalar_call_absent,
        phase5b_archive_literal_absent,
        phase5b_certificate_type_absent,
        all_packages_report_forbidden_inputs_absent,
        all_packages_mint_zero_output_positions,
        all_packages_sweep_full_pre_candidate_a3_surface,
        all_packages_apply_family_instance_quotient,
        all_packages_preserve_a3_generator_output_boundary:
            all_packages_preserve_a3_generator_output_boundary,
        source_and_runtime_audit_passed,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("intrinsic-issuer-audit", &audit);
    audit
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1V3StageRow {
    pub stage: u32,
    pub kappa: u32,
    pub role_occurrence_count_before_quotient: usize,
    pub r2_removed_count: usize,
    pub natural_family_count_after_quotient: usize,
    pub role_schema_extraction_complete: bool,
    pub marginal_natural_family_count: usize,
    pub authoritative_token_count: usize,
    pub exact_certified_nu: Option<u32>,
    pub a3_instance_count: usize,
    pub a3_orbit_count: usize,
    pub independently_exported_a3_orbit_count: usize,
    pub blind_local_role_capacity_4kappa: u32,
    pub adopted_export_upper_bound: u32,
    pub counterfactual_all_quotient_orbits_upper_bound: u32,
    pub invalid_raw_occurrence_upper_bound: u32,
    pub theorem_gap_ids: Vec<String>,
    pub authoritative: bool,
    pub package_derivation_hash: String,
    pub derivation_hash: String,
}

fn stage_row(package: &ActLocalProvenanceV3Certificate) -> TBiNu1V3StageRow {
    let mut row = TBiNu1V3StageRow {
        stage: package.stage,
        kappa: package.kappa,
        role_occurrence_count_before_quotient: package.role_occurrence_count_before_quotient,
        r2_removed_count: package.r2_generated_instance_removal_count,
        natural_family_count_after_quotient: package.natural_family_count_after_quotient,
        role_schema_extraction_complete: package.role_schema_extraction_complete,
        marginal_natural_family_count: package.marginal_natural_family_count,
        authoritative_token_count: package.ordinary_family_tokens.len(),
        exact_certified_nu: package.exact_certified_nu,
        a3_instance_count: package.a3_instance_count,
        a3_orbit_count: package.a3_orbit_count,
        independently_exported_a3_orbit_count: package.independently_exported_a3_orbit_count,
        blind_local_role_capacity_4kappa: package.blind_local_role_capacity_4kappa,
        adopted_export_upper_bound: package.adopted_export_upper_bound_4kappa_plus_exported_orbits,
        counterfactual_all_quotient_orbits_upper_bound: package
            .counterfactual_all_quotient_orbits_upper_bound,
        invalid_raw_occurrence_upper_bound: package.invalid_raw_occurrence_upper_bound,
        theorem_gap_ids: package
            .theorem_gaps
            .iter()
            .map(|gap| gap.id.clone())
            .collect(),
        authoritative: package.authoritative,
        package_derivation_hash: package.derivation_hash.clone(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("stage-row", &row);
    row
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FAl1V3Divergence {
    pub stage: u32,
    pub field: String,
    pub intrinsic_value: String,
    pub archived_value: String,
    pub derivation_hash: String,
}

fn divergence(
    stage: u32,
    field: &str,
    intrinsic_value: impl ToString,
    archived_value: impl ToString,
) -> FAl1V3Divergence {
    let mut row = FAl1V3Divergence {
        stage,
        field: field.to_owned(),
        intrinsic_value: intrinsic_value.to_string(),
        archived_value: archived_value.to_string(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("F-AL1-divergence", &row);
    row
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FAl1V3Audit {
    pub falsifier: String,
    pub intrinsic_issuance_digest_sealed_before_archive_read: bool,
    pub archive_schema: String,
    pub archive_schema_exact: bool,
    pub archive_result_digest: String,
    pub archive_self_digest_valid: bool,
    pub archive_step_count_exact: bool,
    pub archive_used_only_as_post_issuance_comparator: bool,
    pub intrinsic_nu_vector: Vec<Option<u32>>,
    pub archived_nu_vector: Vec<u32>,
    pub candidate_hash_vector_exact: bool,
    pub predecessor_signature_vector_exact: bool,
    pub nu_vector_exact: bool,
    pub divergences: Vec<FAl1V3Divergence>,
    pub first_exact_divergence: Option<FAl1V3Divergence>,
    pub f_al1_triggered: bool,
    pub f_al1_passed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage15CapacityCountermodelV3 {
    pub stage: u32,
    pub kappa: u32,
    pub archived_total_post_seal: u32,
    pub blind_local_role_capacity_4kappa: u32,
    pub independently_exported_a3_orbit_count: usize,
    pub adopted_export_upper_bound: u32,
    pub quotient_a3_orbit_count: usize,
    pub counterfactual_all_quotient_orbits_upper_bound: u32,
    pub raw_a3_instance_count: usize,
    pub invalid_raw_occurrence_upper_bound: u32,
    pub relative_a3_rule_inventory_exhaustive_for_window: bool,
    pub archive_exceeds_adopted_bound: bool,
    pub archive_exceeds_even_counterfactual_all_quotient_orbits_bound: bool,
    pub archive_matches_unquotiented_raw_occurrence_relaxation: bool,
    pub quotient_collapse_count: usize,
    pub impossibility_under_current_rules_proved: bool,
    pub exact_falsifier: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PerStageCapacityComparisonV3 {
    pub stage: u32,
    pub kappa: u32,
    pub archived_total_post_seal: u32,
    pub relative_a3_rule_inventory_exhaustive_for_window: bool,
    pub independently_exported_a3_orbit_count: usize,
    pub adopted_export_upper_bound: u32,
    pub quotient_a3_orbit_count: usize,
    pub counterfactual_all_quotient_orbits_upper_bound: u32,
    pub raw_a3_instance_count: usize,
    pub invalid_raw_occurrence_upper_bound: u32,
    pub archive_exceeds_adopted_bound: bool,
    pub archive_exceeds_all_quotient_orbits_relaxation: bool,
    pub archive_matches_unquotiented_raw_occurrence_relaxation: bool,
    pub current_rule_capacity_impossibility: bool,
    pub derivation_hash: String,
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

fn option_nu(value: Option<u32>) -> String {
    value.map_or_else(|| "THEOREM_GAP".to_owned(), |value| value.to_string())
}

fn run_f_al1(
    intrinsic_issuance_digest: &str,
    packages: &[ActLocalProvenanceV3Certificate],
) -> Result<(FAl1V3Audit, Stage15CapacityCountermodelV3), TBiNu1RegressionV3Error> {
    // This is the first archive read in the execution path.
    let archive: Phase5bHistoryCertificate = serde_json::from_slice(ARCHIVED_HISTORY_BYTES)
        .map_err(|error| TBiNu1RegressionV3Error::Json(error.to_string()))?;
    let archive_valid = archive_self_digest_valid(&archive);
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
                "archive_row",
                "present",
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
        if package.exact_certified_nu != Some(archived.certified_semantic_total) {
            divergences.push(divergence(
                package.stage,
                "certified_semantic_total",
                option_nu(package.exact_certified_nu),
                archived.certified_semantic_total,
            ));
        }
    }
    divergences.sort_by(|left, right| {
        (left.stage, left.field.as_str()).cmp(&(right.stage, right.field.as_str()))
    });
    let intrinsic_nu_vector = packages
        .iter()
        .map(|package| package.exact_certified_nu)
        .collect::<Vec<_>>();
    let archived_nu_vector = archive
        .steps
        .iter()
        .map(|step| step.certified_semantic_total)
        .collect::<Vec<_>>();
    let archive_schema_exact = archive.schema == PHASE5B_HISTORY_CERT_SCHEMA;
    let archive_step_count_exact = packages.len() == 15 && archive.steps.len() == packages.len();
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
                && left.exact_certified_nu == Some(right.certified_semantic_total)
        });
    let f_al1_passed = archive_valid
        && archive_schema_exact
        && archive_step_count_exact
        && candidate_hash_vector_exact
        && predecessor_signature_vector_exact
        && nu_vector_exact
        && divergences.is_empty();
    let f_al1_triggered = !f_al1_passed;
    let mut f_al1 = FAl1V3Audit {
        falsifier:
            "F-AL1-enacted-act-local-decomposition-must-reproduce-archived-certified-totals-exactly"
                .to_owned(),
        intrinsic_issuance_digest_sealed_before_archive_read: !intrinsic_issuance_digest.is_empty(),
        archive_schema: archive.schema.clone(),
        archive_schema_exact,
        archive_result_digest: archive.result_digest.clone(),
        archive_self_digest_valid: archive_valid,
        archive_step_count_exact,
        archive_used_only_as_post_issuance_comparator: true,
        intrinsic_nu_vector,
        archived_nu_vector,
        candidate_hash_vector_exact,
        predecessor_signature_vector_exact,
        nu_vector_exact,
        first_exact_divergence: divergences.first().cloned(),
        divergences,
        f_al1_triggered,
        f_al1_passed,
        derivation_hash: String::new(),
    };
    f_al1.derivation_hash = tagged_hash("F-AL1-audit", &f_al1);

    let package15 = packages
        .iter()
        .find(|package| package.stage == 15)
        .ok_or_else(|| TBiNu1RegressionV3Error::Invariant("Stage 15 package absent".to_owned()))?;
    let archive15 = archived_by_stage.get(&15).ok_or_else(|| {
        TBiNu1RegressionV3Error::Invariant("Stage 15 archive row absent".to_owned())
    })?;
    let quotient_collapse_count = package15
        .a3_instance_count
        .saturating_sub(package15.a3_orbit_count);
    let archive_exceeds_adopted_bound = archive15.certified_semantic_total
        > package15.adopted_export_upper_bound_4kappa_plus_exported_orbits;
    let archive_exceeds_even_counterfactual_all_quotient_orbits_bound = archive15
        .certified_semantic_total
        > package15.counterfactual_all_quotient_orbits_upper_bound;
    let archive_matches_unquotiented_raw_occurrence_relaxation =
        archive15.certified_semantic_total == package15.invalid_raw_occurrence_upper_bound;
    let impossibility_under_current_rules_proved = archive_valid
        && package15.relative_a3_rule_inventory_exhaustive_for_window
        && archive_exceeds_adopted_bound
        && archive_exceeds_even_counterfactual_all_quotient_orbits_bound;
    let mut countermodel = Stage15CapacityCountermodelV3 {
        stage: 15,
        kappa: package15.kappa,
        archived_total_post_seal: archive15.certified_semantic_total,
        blind_local_role_capacity_4kappa: package15.blind_local_role_capacity_4kappa,
        independently_exported_a3_orbit_count: package15
            .independently_exported_a3_orbit_count,
        adopted_export_upper_bound: package15
            .adopted_export_upper_bound_4kappa_plus_exported_orbits,
        quotient_a3_orbit_count: package15.a3_orbit_count,
        counterfactual_all_quotient_orbits_upper_bound: package15
            .counterfactual_all_quotient_orbits_upper_bound,
        raw_a3_instance_count: package15.a3_instance_count,
        invalid_raw_occurrence_upper_bound: package15.invalid_raw_occurrence_upper_bound,
        relative_a3_rule_inventory_exhaustive_for_window: package15
            .relative_a3_rule_inventory_exhaustive_for_window,
        archive_exceeds_adopted_bound,
        archive_exceeds_even_counterfactual_all_quotient_orbits_bound,
        archive_matches_unquotiented_raw_occurrence_relaxation,
        quotient_collapse_count,
        impossibility_under_current_rules_proved,
        exact_falsifier: if impossibility_under_current_rules_proved {
            "F-AL1-STAGE15-CAPACITY-COUNTERMODEL: the sealed total exceeds both the exhaustive adopted codomain and the larger all-quotient-orbits relaxation. It numerically matches the still larger unquotiented raw-occurrence relaxation; that equality is diagnostic, not a causal claim"
        } else {
            "Stage-15 capacity relation did not instantiate the preregistered countermodel shape; inspect the recorded operands"
        }
        .to_owned(),
        derivation_hash: String::new(),
    };
    countermodel.derivation_hash = tagged_hash("Stage-15-capacity-countermodel", &countermodel);
    Ok((f_al1, countermodel))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionV3Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub source_bindings: Vec<TBiNu1V3SourceBinding>,
    pub adoption_and_prior_burn_replayed: bool,
    pub intrinsic_packages: Vec<ActLocalProvenanceV3Certificate>,
    pub intrinsic_issuance_digest_before_archive_comparator: String,
    pub intrinsic_issuer_audit: IntrinsicIssuerV3Audit,
    pub stage_rows: Vec<TBiNu1V3StageRow>,
    pub all_fifteen_acts_issued: bool,
    pub authoritative_package_count: usize,
    pub all_fifteen_packages_authoritative: bool,
    pub theorem_gaps: Vec<ActLocalV3Gap>,
    pub theorem_gap_count: usize,
    pub intrinsic_theorem_total_before_f_al1: bool,
    pub f_al1: FAl1V3Audit,
    pub per_stage_capacity_comparisons: Vec<PerStageCapacityComparisonV3>,
    pub first_capacity_impossibility_stage: Option<u32>,
    pub stage15_capacity_countermodel: Stage15CapacityCountermodelV3,
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
pub struct TBiNu1RegressionV3Replay {
    pub valid: bool,
    pub theorem_proved: bool,
    pub bi0_rerun_authorized: bool,
    pub all_fifteen_acts_issued: bool,
    pub authoritative_package_count: usize,
    pub f_al1_passed: bool,
    pub stage15_impossibility_proved: bool,
    pub intrinsic_nu_vector: Vec<Option<u32>>,
    pub first_exact_divergence: Option<FAl1V3Divergence>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiNu1RegressionV3Error {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("T-BI-NU1 v3 invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted replay failed: {0}")]
    EmittedReplay(String),
}

fn replay_adoption_and_burn() -> Result<(), TBiNu1RegressionV3Error> {
    let adoption = std::str::from_utf8(ADJUDICATION_BYTES)
        .map_err(|error| TBiNu1RegressionV3Error::Prerequisite(error.to_string()))?;
    let burn = std::str::from_utf8(V1_BURN_BYTES)
        .map_err(|error| TBiNu1RegressionV3Error::Prerequisite(error.to_string()))?;
    if !adoption.contains("act-local-provenance-v1")
        || !adoption.contains("F-AL1")
        || !adoption.contains("archive join")
        || !burn.contains("BURNED AS BI-0 AUTHORITY")
        || !burn.contains("candidate-minted")
    {
        return Err(TBiNu1RegressionV3Error::Prerequisite(
            "adoption or prior-issuer burn did not replay".to_owned(),
        ));
    }
    Ok(())
}

fn certificate_digest(certificate: &TBiNu1RegressionV3Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("regression-certificate", &projection)
}

pub fn issue_t_bi_nu1_regression_v3_certificate()
-> Result<TBiNu1RegressionV3Certificate, TBiNu1RegressionV3Error> {
    replay_adoption_and_burn()?;
    let entries = (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    let intrinsic_packages = issue_act_local_sequence_v3(&entries)
        .map_err(|error| TBiNu1RegressionV3Error::Invariant(error.to_string()))?;
    let intrinsic_issuance_digest_before_archive_comparator = tagged_hash(
        "sealed-intrinsic-issuance-before-F-AL1",
        &intrinsic_packages
            .iter()
            .map(|package| package.derivation_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let intrinsic_issuer_audit = intrinsic_issuer_audit(&intrinsic_packages);
    let stage_rows = intrinsic_packages.iter().map(stage_row).collect::<Vec<_>>();
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
    let intrinsic_theorem_total_before_f_al1 = all_fifteen_acts_issued
        && all_fifteen_packages_authoritative
        && theorem_gaps.is_empty()
        && intrinsic_issuer_audit.source_and_runtime_audit_passed;
    let (f_al1, stage15_capacity_countermodel) = run_f_al1(
        &intrinsic_issuance_digest_before_archive_comparator,
        &intrinsic_packages,
    )?;
    let archive: Phase5bHistoryCertificate = serde_json::from_slice(ARCHIVED_HISTORY_BYTES)
        .map_err(|error| TBiNu1RegressionV3Error::Json(error.to_string()))?;
    let archive_valid_for_capacity = archive_self_digest_valid(&archive);
    let archived_by_stage = archive
        .steps
        .iter()
        .map(|step| (step.step, step))
        .collect::<BTreeMap<_, _>>();
    let mut per_stage_capacity_comparisons = intrinsic_packages
        .iter()
        .map(|package| {
            let archived = archived_by_stage.get(&package.stage).ok_or_else(|| {
                TBiNu1RegressionV3Error::Invariant(format!(
                    "Stage {} archive row absent during post-seal capacity audit",
                    package.stage
                ))
            })?;
            let archive_exceeds_adopted_bound = archived.certified_semantic_total
                > package.adopted_export_upper_bound_4kappa_plus_exported_orbits;
            let archive_exceeds_all_quotient_orbits_relaxation = archived.certified_semantic_total
                > package.counterfactual_all_quotient_orbits_upper_bound;
            let archive_matches_unquotiented_raw_occurrence_relaxation =
                archived.certified_semantic_total == package.invalid_raw_occurrence_upper_bound;
            let current_rule_capacity_impossibility = archive_valid_for_capacity
                && package.relative_a3_rule_inventory_exhaustive_for_window
                && archive_exceeds_adopted_bound;
            let mut row = PerStageCapacityComparisonV3 {
                stage: package.stage,
                kappa: package.kappa,
                archived_total_post_seal: archived.certified_semantic_total,
                relative_a3_rule_inventory_exhaustive_for_window: package
                    .relative_a3_rule_inventory_exhaustive_for_window,
                independently_exported_a3_orbit_count: package
                    .independently_exported_a3_orbit_count,
                adopted_export_upper_bound: package
                    .adopted_export_upper_bound_4kappa_plus_exported_orbits,
                quotient_a3_orbit_count: package.a3_orbit_count,
                counterfactual_all_quotient_orbits_upper_bound: package
                    .counterfactual_all_quotient_orbits_upper_bound,
                raw_a3_instance_count: package.a3_instance_count,
                invalid_raw_occurrence_upper_bound: package.invalid_raw_occurrence_upper_bound,
                archive_exceeds_adopted_bound,
                archive_exceeds_all_quotient_orbits_relaxation,
                archive_matches_unquotiented_raw_occurrence_relaxation,
                current_rule_capacity_impossibility,
                derivation_hash: String::new(),
            };
            row.derivation_hash = tagged_hash("per-stage-capacity-comparison", &row);
            Ok(row)
        })
        .collect::<Result<Vec<_>, TBiNu1RegressionV3Error>>()?;
    per_stage_capacity_comparisons.sort_by_key(|row| row.stage);
    let first_capacity_impossibility_stage = per_stage_capacity_comparisons
        .iter()
        .find(|row| row.current_rule_capacity_impossibility)
        .map(|row| row.stage);
    let t_bi_nu1_proved_on_enacted_branch =
        intrinsic_theorem_total_before_f_al1 && f_al1.f_al1_passed;
    let outcome = if t_bi_nu1_proved_on_enacted_branch {
        "T_BI_NU1_V3_PROVED_F_AL1_PASSED"
    } else if stage15_capacity_countermodel.impossibility_under_current_rules_proved {
        "T_BI_NU1_V3_REFUTED_BY_STAGE15_QUOTIENT_CAPACITY_COUNTERMODEL"
    } else if !intrinsic_theorem_total_before_f_al1 {
        "T_BI_NU1_V3_INTRINSIC_THEOREM_GAP"
    } else {
        "T_BI_NU1_V3_F_AL1_DIVERGENCE"
    };
    let mut certificate = TBiNu1RegressionV3Certificate {
        schema: T_BI_NU1_REGRESSION_V3_SCHEMA.to_owned(),
        date: T_BI_NU1_REGRESSION_V3_DATE.to_owned(),
        theorem_id: T_BI_NU1_V3_THEOREM_ID.to_owned(),
        source_bindings: source_bindings(),
        adoption_and_prior_burn_replayed: true,
        intrinsic_packages,
        intrinsic_issuance_digest_before_archive_comparator,
        intrinsic_issuer_audit,
        stage_rows,
        all_fifteen_acts_issued,
        authoritative_package_count,
        all_fifteen_packages_authoritative,
        theorem_gaps,
        theorem_gap_count,
        intrinsic_theorem_total_before_f_al1,
        f_al1,
        per_stage_capacity_comparisons,
        first_capacity_impossibility_stage,
        stage15_capacity_countermodel,
        t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized_by_t_bi_nu1_side: t_bi_nu1_proved_on_enacted_branch,
        non_enacted_branch_work_executed: false,
        outcome: outcome.to_owned(),
        permitted_conclusion: if t_bi_nu1_proved_on_enacted_branch {
            "Only the T-BI-NU1 side of a BI-0 rerun is authorized; the separate slot-map gate must also pass."
        } else {
            "This is a replayable negative prerequisite. Under the current export and quotient rules the archived Stage-15 total is outside the finite provenance codomain; BI-0, BI-1, BI-4, and UC scoring remain closed."
        }
        .to_owned(),
        required_successor_action: if t_bi_nu1_proved_on_enacted_branch {
            "Join a separately passing slot-map artifact and rerun BI-0 create-new."
        } else {
            "Amend the semantics only through a versioned adjudication proving typed pre-candidate A3 output construction/membership or changing the sealed total/quotient law; do not count raw occurrences and do not mint output positions."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> TBiNu1RegressionV3Replay {
    TBiNu1RegressionV3Replay {
        valid: false,
        theorem_proved: false,
        bi0_rerun_authorized: false,
        all_fifteen_acts_issued: false,
        authoritative_package_count: 0,
        f_al1_passed: false,
        stage15_impossibility_proved: false,
        intrinsic_nu_vector: Vec::new(),
        first_exact_divergence: None,
        errors: vec![error.into()],
    }
}

pub fn replay_t_bi_nu1_regression_v3_certificate(
    claimed: &TBiNu1RegressionV3Certificate,
) -> TBiNu1RegressionV3Replay {
    let expected = match issue_t_bi_nu1_regression_v3_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-BI-NU1 v3 regression certificate digest mismatch".to_owned());
    }
    if claimed != &expected {
        errors.push("T-BI-NU1 v3 certificate differs from create-new reissuance".to_owned());
    }
    let logically_proved =
        claimed.intrinsic_theorem_total_before_f_al1 && claimed.f_al1.f_al1_passed;
    if claimed.t_bi_nu1_proved_on_enacted_branch != logically_proved
        || claimed.bi0_rerun_authorized_by_t_bi_nu1_side != logically_proved
        || claimed.non_enacted_branch_work_executed
    {
        errors.push("T-BI-NU1 v3 conclusion or F-BI sequencing surface is invalid".to_owned());
    }
    TBiNu1RegressionV3Replay {
        valid: errors.is_empty(),
        theorem_proved: claimed.t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized: claimed.bi0_rerun_authorized_by_t_bi_nu1_side,
        all_fifteen_acts_issued: claimed.all_fifteen_acts_issued,
        authoritative_package_count: claimed.authoritative_package_count,
        f_al1_passed: claimed.f_al1.f_al1_passed,
        stage15_impossibility_proved: claimed
            .stage15_capacity_countermodel
            .impossibility_under_current_rules_proved,
        intrinsic_nu_vector: claimed.f_al1.intrinsic_nu_vector.clone(),
        first_exact_divergence: claimed.f_al1.first_exact_divergence.clone(),
        errors,
    }
}

pub fn replay_t_bi_nu1_regression_v3_json(json: &str) -> TBiNu1RegressionV3Replay {
    match serde_json::from_str::<TBiNu1RegressionV3Certificate>(json) {
        Ok(certificate) => replay_t_bi_nu1_regression_v3_certificate(&certificate),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn render_t_bi_nu1_v3_report(certificate: &TBiNu1RegressionV3Certificate) -> String {
    let mut table = String::new();
    for row in &certificate.stage_rows {
        table.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.stage,
            row.kappa,
            row.role_occurrence_count_before_quotient,
            row.natural_family_count_after_quotient,
            row.authoritative_token_count,
            option_nu(row.exact_certified_nu),
            row.a3_instance_count,
            row.a3_orbit_count,
            row.independently_exported_a3_orbit_count,
            row.theorem_gap_ids.len(),
        ));
    }
    let first = certificate
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
    let countermodel = &certificate.stage15_capacity_countermodel;
    format!(
        "# T-BI-NU1 act-local provenance v3 result\n\n**Date:** {}. **Outcome:** `{}`.\n\nThe candidate/prefix-only issuer ran before the archive comparator. It swept the full A3 orbit surface at all fifteen stages, minted zero candidate output positions, and issued {}/15 authoritative packages. T-BI-NU1 proved: **{}**. BI-0 authorized by this side: **{}**.\n\n| Stage | kappa | Role occurrences | Quotient families | Tokens | Certified nu | A3 instances | A3 orbits | Exported orbits | Gaps |\n|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n{}\nF-AL1 passed: **{}**. First divergence: {}. First post-seal adopted-capacity impossibility: `{:?}`.\n\nStage 15 countermodel: local capacity `{}`, adopted `4k+exported` bound `{}`, even the inadmissible all-quotient-orbits bound `{}`, sealed total `{}`, and raw-occurrence bound `{}`. The sealed total matches the unquotiented raw-occurrence relaxation: **{}** (diagnostic equality, not a causal claim). Current-rule impossibility proved: **{}**.\n\nPermitted conclusion: {}\n\nNext: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.authoritative_package_count,
        certificate.t_bi_nu1_proved_on_enacted_branch,
        certificate.bi0_rerun_authorized_by_t_bi_nu1_side,
        table,
        certificate.f_al1.f_al1_passed,
        first,
        certificate.first_capacity_impossibility_stage,
        countermodel.blind_local_role_capacity_4kappa,
        countermodel.adopted_export_upper_bound,
        countermodel.counterfactual_all_quotient_orbits_upper_bound,
        countermodel.archived_total_post_seal,
        countermodel.invalid_raw_occurrence_upper_bound,
        countermodel.archive_matches_unquotiented_raw_occurrence_relaxation,
        countermodel.impossibility_under_current_rules_proved,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.result_digest,
    )
}

pub fn emit_t_bi_nu1_regression_v3_create_new(
    directory: &Path,
) -> Result<TBiNu1RegressionV3Certificate, TBiNu1RegressionV3Error> {
    let certificate = issue_t_bi_nu1_regression_v3_certificate()?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| TBiNu1RegressionV3Error::Json(error.to_string()))?;
    let report = render_t_bi_nu1_v3_report(&certificate);
    let replay = replay_t_bi_nu1_regression_v3_json(&json);
    if !replay.valid {
        return Err(TBiNu1RegressionV3Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let certificate_path = directory.join(T_BI_NU1_V3_CERTIFICATE_NAME);
    let report_path = directory.join(T_BI_NU1_V3_REPORT_NAME);
    if certificate_path.exists() || report_path.exists() {
        return Err(TBiNu1RegressionV3Error::Io(format!(
            "create-new target already exists: {} or {}",
            certificate_path.display(),
            report_path.display()
        )));
    }
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| TBiNu1RegressionV3Error::Io(error.to_string()))?;
    certificate_file
        .write_all(json.as_bytes())
        .map_err(|error| TBiNu1RegressionV3Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| TBiNu1RegressionV3Error::Io(error.to_string()))?;
    report_file
        .write_all(report.as_bytes())
        .map_err(|error| TBiNu1RegressionV3Error::Io(error.to_string()))?;
    Ok(certificate)
}

pub fn replay_t_bi_nu1_regression_v3_directory(
    directory: &Path,
) -> Result<TBiNu1RegressionV3Replay, TBiNu1RegressionV3Error> {
    let json = std::fs::read_to_string(directory.join(T_BI_NU1_V3_CERTIFICATE_NAME))
        .map_err(|error| TBiNu1RegressionV3Error::Io(error.to_string()))?;
    Ok(replay_t_bi_nu1_regression_v3_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v3_is_a_replayable_negative_result_not_a_forced_vector() {
        let certificate =
            issue_t_bi_nu1_regression_v3_certificate().expect("T-BI-NU1 v3 regression");
        assert!(certificate.all_fifteen_acts_issued);
        assert!(
            certificate
                .intrinsic_issuer_audit
                .source_and_runtime_audit_passed
        );
        assert!(!certificate.t_bi_nu1_proved_on_enacted_branch);
        assert!(!certificate.bi0_rerun_authorized_by_t_bi_nu1_side);
        assert!(!certificate.f_al1.f_al1_passed);
        assert!(certificate.f_al1.archive_schema_exact);
        assert!(certificate.f_al1.archive_step_count_exact);
        assert!(certificate.f_al1.candidate_hash_vector_exact);
        assert!(certificate.f_al1.predecessor_signature_vector_exact);
        assert!(!certificate.f_al1.nu_vector_exact);
        assert!(certificate.f_al1.f_al1_triggered);
        assert!(
            certificate
                .stage15_capacity_countermodel
                .impossibility_under_current_rules_proved
        );
        assert_eq!(certificate.first_capacity_impossibility_stage, Some(10));
        assert!(replay_t_bi_nu1_regression_v3_certificate(&certificate).valid);
    }

    #[test]
    fn re_signed_countermodel_mutation_fails_reissuance() {
        let certificate =
            issue_t_bi_nu1_regression_v3_certificate().expect("T-BI-NU1 v3 regression");
        let mut forged = certificate.clone();
        forged
            .stage15_capacity_countermodel
            .impossibility_under_current_rules_proved = false;
        forged.stage15_capacity_countermodel.derivation_hash = tagged_hash(
            "Stage-15-capacity-countermodel",
            &forged.stage15_capacity_countermodel,
        );
        forged.result_digest = certificate_digest(&forged);
        assert!(!replay_t_bi_nu1_regression_v3_certificate(&forged).valid);
    }
}
