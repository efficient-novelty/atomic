//! Post-seal F-AL1-prime regression for semantic-family provenance v4.
//!
//! The intrinsic issuer is executed to completion and its fifteen-package
//! digest is sealed before this module opens any structural register or
//! reselection artifact.  The comparison table is intentionally bare: it has
//! only stage, structural, and semantic columns and carries no explanatory
//! or reconciliation field.

use crate::act_local_semantic_provenance_v4::{
    ACT_LOCAL_SEMANTIC_PROVENANCE_V4_SCHEMA, ActLocalSemanticProvenanceV4Certificate,
    issue_reference_act_local_semantic_sequence_v4,
};
use crate::phase5b_history_certification::{
    PHASE5B_HISTORY_CERT_SCHEMA, Phase5bHistoryCertificate,
};
use crate::phase5b_reselection_v2::{
    PHASE5B_RESELECTION_BURN_SCHEMA, PHASE5B_RESELECTION_PROGRAM_SCHEMA, Phase5bReselectionBurn,
    Phase5bReselectionProgram,
};
use crate::phase5b_reselection_v3::{
    PHASE5B_RESELECTION_V3_BURN_SCHEMA, PHASE5B_RESELECTION_V3_PROGRAM_SCHEMA,
    Phase5bReselectionV3Burn, Phase5bReselectionV3Program,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const T_BI_NU1_REGRESSION_V4_SCHEMA: &str = "t-bi-nu1-semantic-family-regression-v4";
pub const T_BI_NU1_REGRESSION_V4_DATE: &str = "2026-07-22";
pub const T_BI_NU1_REGRESSION_V4_THEOREM_ID: &str =
    "T-BI-NU1-v4-F-AL1-prime-complete-semantic-family-regression";
pub const T_BI_NU1_V4_CERTIFICATE_NAME: &str = "t_bi_nu1_semantic_provenance_v4.json";
pub const T_BI_NU1_V4_REPORT_NAME: &str = "T_BI_NU1_SEMANTIC_PROVENANCE_V4_RESULT.md";

const NU_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/nu_register_adjudication.md");
const COMPATIBILITY_AUDIT_BYTES: &[u8] =
    include_bytes!("../../../docs/T_BI_NU1_FULL_A3_COMPATIBILITY_AUDIT.md");
const HISTORY_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const V2_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v2.json");
const V2_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v2.json");
const V3_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v3.json");
const V3_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v3.json");
const INTRINSIC_SOURCE_BYTES: &[u8] = include_bytes!("act_local_semantic_provenance_v4.rs");
const TYPED_FAMILY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/typed_families.rs");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const A3_EXHAUSTIVENESS_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/a3_rule_inventory_exhaustiveness.rs");
const CUBICAL_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/cubical.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_bi_nu1_regression_v4.rs");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_NU1_REGRESSION_V4_SCHEMA, domain, value))
        .expect("T-BI-NU1 v4 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("external evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1V4SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

fn source_bindings() -> Vec<TBiNu1V4SourceBinding> {
    [
        (
            "docs/nu_register_adjudication.md",
            "adopted_two_register_rule_and_F_AL1_prime",
            NU_ADJUDICATION_BYTES,
        ),
        (
            "docs/T_BI_NU1_FULL_A3_COMPATIBILITY_AUDIT.md",
            "six_step_theorem_program",
            COMPATIBILITY_AUDIT_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_semantic_provenance_v4.rs",
            "archive_free_intrinsic_semantic_family_issuer",
            INTRINSIC_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/typed_families.rs",
            "typed_normalized_natural_clause_family_extraction",
            TYPED_FAMILY_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "exact_prefix_A3_scheme_instance_orbit_inventory",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_rule_inventory_exhaustiveness.rs",
            "exact_prefix_A3_exhaustiveness_theorem",
            A3_EXHAUSTIVENESS_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/cubical.rs",
            "typed_path_projection_equality_and_naturality",
            CUBICAL_SOURCE_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v2.json",
            "post_seal_stage_1_7_operational_program",
            V2_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v2.json",
            "post_seal_stage_1_7_operational_trace",
            V2_BURN_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v3.json",
            "post_seal_stage_8_15_operational_program",
            V3_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v3.json",
            "post_seal_stage_8_15_operational_trace",
            V3_BURN_BYTES,
        ),
        (
            "docs/phase5b_full_history_v1.json",
            "post_seal_structural_register_comparator_only",
            HISTORY_BYTES,
        ),
        (
            "crates/pen-search/src/t_bi_nu1_regression_v4.rs",
            "this_post_seal_regression_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| TBiNu1V4SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicV4IsolationAudit {
    pub intrinsic_schema: String,
    pub structural_nu_function_call_absent: bool,
    pub phase5b_history_literal_absent: bool,
    pub history_certificate_type_absent: bool,
    pub reselection_type_absent: bool,
    pub all_packages_report_forbidden_inputs_absent: bool,
    pub all_packages_exact_prefix_a3_complete: bool,
    pub all_path_quotients_complete: bool,
    pub all_role_registries_exhaustive: bool,
    /// A diagnostic scan of this issuer's direct source plus the intrinsic
    /// packages' self-reported forbidden-input flags.  This is useful
    /// evidence, but it is not a transitive call-graph theorem.
    pub direct_source_scan_and_runtime_flags_clear: bool,
    /// A proof that every transitive callee of the intrinsic issuer is free
    /// of archive/register reads.  No such proof object exists yet.
    pub transitive_call_graph_isolation_proved: bool,
    /// The law-level isolation gate.  Direct scans cannot set it by
    /// themselves.
    pub proof_bearing_isolation_passed: bool,
    pub named_residuals: Vec<String>,
    pub evidence_scope: String,
    pub derivation_hash: String,
}

fn intrinsic_isolation_audit(
    packages: &[ActLocalSemanticProvenanceV4Certificate],
) -> IntrinsicV4IsolationAudit {
    let source = std::str::from_utf8(INTRINSIC_SOURCE_BYTES).unwrap_or_default();
    let structural_nu_function_call_absent = !source.contains("structural_nu(");
    let phase5b_history_literal_absent = !source.contains("phase5b_full_history_v1");
    let history_certificate_type_absent = !source.contains("Phase5bHistoryCertificate");
    let reselection_type_absent = !source.contains("Phase5bReselection");
    let all_packages_report_forbidden_inputs_absent = packages.iter().all(|package| {
        !package.archive_read
            && !package.structural_nu_read
            && !package.bar_read
            && !package.verdict_read
            && !package.enacted_future_read
    });
    let all_packages_exact_prefix_a3_complete =
        packages.iter().all(|package| package.exact_a3.complete);
    let all_path_quotients_complete = packages.iter().all(|package| {
        package
            .path_quotient
            .as_ref()
            .map_or(true, |proof| proof.complete)
    });
    let all_role_registries_exhaustive = packages
        .iter()
        .all(|package| package.role_registry_exhaustive);
    let direct_source_scan_and_runtime_flags_clear = structural_nu_function_call_absent
        && phase5b_history_literal_absent
        && history_certificate_type_absent
        && reselection_type_absent
        && all_packages_report_forbidden_inputs_absent
        && all_packages_exact_prefix_a3_complete
        && all_path_quotients_complete
        && all_role_registries_exhaustive;
    // A substring scan of one source file and runtime booleans are not a
    // proof about the complete transitive call graph.  Keep the missing
    // theorem explicit rather than promoting diagnostic evidence.
    let transitive_call_graph_isolation_proved = false;
    let proof_bearing_isolation_passed =
        direct_source_scan_and_runtime_flags_clear && transitive_call_graph_isolation_proved;
    let named_residuals = if transitive_call_graph_isolation_proved {
        Vec::new()
    } else {
        vec!["T_BI_NU1_TRANSITIVE_INTRINSIC_ISOLATION_UNPROVED".to_owned()]
    };
    let mut audit = IntrinsicV4IsolationAudit {
        intrinsic_schema: ACT_LOCAL_SEMANTIC_PROVENANCE_V4_SCHEMA.to_owned(),
        structural_nu_function_call_absent,
        phase5b_history_literal_absent,
        history_certificate_type_absent,
        reselection_type_absent,
        all_packages_report_forbidden_inputs_absent,
        all_packages_exact_prefix_a3_complete,
        all_path_quotients_complete,
        all_role_registries_exhaustive,
        direct_source_scan_and_runtime_flags_clear,
        transitive_call_graph_isolation_proved,
        proof_bearing_isolation_passed,
        named_residuals,
        evidence_scope: "Direct source substring scan plus intrinsic package flags only; no transitive call-graph or capability proof is claimed.".to_owned(),
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("intrinsic-isolation-audit", &audit);
    audit
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationalStageRegressionV4 {
    pub stage: u32,
    pub regime: String,
    pub cone_enumerated: usize,
    pub cone_admitted: usize,
    pub cone_deduped: usize,
    pub open_band_clearing_count: Option<usize>,
    pub guarded_discharger_count: Option<usize>,
    pub guarded_discharger_hashes: Vec<String>,
    pub winner_candidate_hash: String,
    pub semantic_package_candidate_hash: String,
    pub winner_hash_exact: bool,
    pub winner_kernel_typed: bool,
    pub winner_admissible_under_recorded_regime: bool,
    pub winner_is_exact_guarded_discharger: bool,
    pub bar_applied_as_guard_or_selector: bool,
    pub operational_row_exact: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationalRegressionV4 {
    pub v2_program_self_digest_valid: bool,
    pub v2_burn_self_digest_valid: bool,
    pub v2_program_burn_link_exact: bool,
    pub v3_program_self_digest_valid: bool,
    pub v3_burn_self_digest_valid: bool,
    pub v3_program_burn_link_exact: bool,
    pub v3_imported_v2_links_exact: bool,
    pub legacy_live_definition_reexecution_required: bool,
    pub rows: Vec<OperationalStageRegressionV4>,
    pub stage_count_exact: bool,
    pub winner_hash_vector_exact: bool,
    pub admissibility_vector_exact: bool,
    pub guarded_discharger_counts_exact: bool,
    pub guarded_discharger_hashes_exact: bool,
    pub no_winner_divergence: bool,
    pub exact_operational_regression: bool,
    pub derivation_hash: String,
}

fn v2_program_self_digest_valid(program: &Phase5bReselectionProgram) -> bool {
    let mut projection = program.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            PHASE5B_RESELECTION_PROGRAM_SCHEMA,
            "reselection-program",
            &projection,
        )
}

fn v2_burn_self_digest_valid(burn: &Phase5bReselectionBurn) -> bool {
    let mut projection = burn.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            PHASE5B_RESELECTION_BURN_SCHEMA,
            "reselection-burn",
            &projection,
        )
}

fn v3_program_self_digest_valid(program: &Phase5bReselectionV3Program) -> bool {
    let mut projection = program.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            PHASE5B_RESELECTION_V3_PROGRAM_SCHEMA,
            "v3-program",
            &projection,
        )
}

fn v3_burn_self_digest_valid(burn: &Phase5bReselectionV3Burn) -> bool {
    let mut projection = burn.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == external_tagged_hash(PHASE5B_RESELECTION_V3_BURN_SCHEMA, "v3-burn", &projection)
}

fn post_seal_operational_regression(
    packages: &[ActLocalSemanticProvenanceV4Certificate],
) -> Result<OperationalRegressionV4, TBiNu1RegressionV4Error> {
    let sealed_v2_program: Phase5bReselectionProgram = serde_json::from_slice(V2_PROGRAM_BYTES)
        .map_err(|error| TBiNu1RegressionV4Error::Json(error.to_string()))?;
    let sealed_v2_burn: Phase5bReselectionBurn = serde_json::from_slice(V2_BURN_BYTES)
        .map_err(|error| TBiNu1RegressionV4Error::Json(error.to_string()))?;
    let v2_program_digest_valid = v2_program_self_digest_valid(&sealed_v2_program);
    let v2_burn_digest_valid = v2_burn_self_digest_valid(&sealed_v2_burn);
    let v2_program_burn_link_exact = sealed_v2_burn.program_digest
        == sealed_v2_program.result_digest
        && sealed_v2_burn.program_schema == sealed_v2_program.schema;

    let sealed_v3_program: Phase5bReselectionV3Program =
        serde_json::from_slice(V3_PROGRAM_BYTES)
            .map_err(|error| TBiNu1RegressionV4Error::Json(error.to_string()))?;
    let sealed_v3_burn: Phase5bReselectionV3Burn = serde_json::from_slice(V3_BURN_BYTES)
        .map_err(|error| TBiNu1RegressionV4Error::Json(error.to_string()))?;
    let v3_program_digest_valid = v3_program_self_digest_valid(&sealed_v3_program);
    let v3_burn_digest_valid = v3_burn_self_digest_valid(&sealed_v3_burn);
    let v3_program_burn_link_exact =
        sealed_v3_burn.program_digest == sealed_v3_program.result_digest;
    let v3_imported_v2_links_exact = sealed_v3_program.imported_v2_program_digest
        == sealed_v2_program.result_digest
        && sealed_v3_program.imported_v2_burn_digest == sealed_v2_burn.result_digest
        && sealed_v3_burn.imported_v2_burn_digest == sealed_v2_burn.result_digest;

    let package_by_stage = packages
        .iter()
        .map(|package| (package.stage, package))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    for stage in sealed_v2_burn
        .stages
        .iter()
        .filter(|stage| stage.stage <= 7)
    {
        let package = package_by_stage.get(&stage.stage).ok_or_else(|| {
            TBiNu1RegressionV4Error::Operational(format!(
                "semantic package missing Stage {}",
                stage.stage
            ))
        })?;
        let winner = stage.winner.as_ref().ok_or_else(|| {
            TBiNu1RegressionV4Error::Operational(format!(
                "v2 operational prefix lacks Stage {} winner",
                stage.stage
            ))
        })?;
        let score = stage
            .scored
            .iter()
            .find(|score| score.candidate_hash == winner.candidate_hash)
            .ok_or_else(|| {
                TBiNu1RegressionV4Error::Operational(format!(
                    "v2 Stage {} winner lacks candidate assessment",
                    stage.stage
                ))
            })?;
        let winner_hash_exact = winner.candidate_hash == package.candidate_hash
            && stage.certified_reference_hash == package.candidate_hash
            && stage.winner_matches_certified_reference;
        let winner_kernel_typed = score.kernel_typed && score.kernel_failure.is_none();
        let winner_admissible_under_recorded_regime = winner_kernel_typed
            && !score.invalid_or_unclassified
            && score.every_score_unit_anchor_supported
            && stage
                .exact_acceptance_order
                .contains(&winner.candidate_hash);
        let operational_row_exact = winner_hash_exact
            && winner_admissible_under_recorded_regime
            && stage.winner_matches_certified_reference;
        let mut row = OperationalStageRegressionV4 {
            stage: stage.stage,
            regime: "sealed_v2_open_band_prefix".to_owned(),
            cone_enumerated: stage.cone_enumerated,
            cone_admitted: stage.cone_admitted,
            cone_deduped: stage.cone_deduped,
            open_band_clearing_count: Some(stage.clearing_count),
            guarded_discharger_count: None,
            guarded_discharger_hashes: Vec::new(),
            winner_candidate_hash: winner.candidate_hash.clone(),
            semantic_package_candidate_hash: package.candidate_hash.clone(),
            winner_hash_exact,
            winner_kernel_typed,
            winner_admissible_under_recorded_regime,
            winner_is_exact_guarded_discharger: false,
            bar_applied_as_guard_or_selector: true,
            operational_row_exact,
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("operational-stage-regression", &row);
        rows.push(row);
    }
    for stage in &sealed_v3_burn.stages {
        let package = package_by_stage.get(&stage.stage).ok_or_else(|| {
            TBiNu1RegressionV4Error::Operational(format!(
                "semantic package missing Stage {}",
                stage.stage
            ))
        })?;
        let winner = stage.winner.as_ref().ok_or_else(|| {
            TBiNu1RegressionV4Error::Operational(format!(
                "v3 operational suffix lacks Stage {} winner",
                stage.stage
            ))
        })?;
        let assessment = stage
            .assessments
            .iter()
            .find(|assessment| assessment.candidate_hash == winner.candidate_hash)
            .ok_or_else(|| {
                TBiNu1RegressionV4Error::Operational(format!(
                    "v3 Stage {} winner lacks candidate assessment",
                    stage.stage
                ))
            })?;
        let winner_hash_exact = winner.candidate_hash == package.candidate_hash
            && stage.reference_hash == package.candidate_hash
            && stage.winner_matches_reference;
        let winner_kernel_typed = assessment.kernel_typed && assessment.kernel_failure.is_none();
        let winner_is_exact_guarded_discharger = stage.guarded
            && stage.discharger_count == 1
            && stage.discharger_hashes == vec![winner.candidate_hash.clone()]
            && assessment.guarded_total_discharger
            && winner.selected_by == "unique_guarded_total_discharger";
        let winner_admissible_under_recorded_regime = winner_kernel_typed
            && assessment.typed_provenance_supported
            && winner_is_exact_guarded_discharger;
        let bar_applied_as_guard_or_selector = stage.bar_gate_applied;
        let operational_row_exact = winner_hash_exact
            && winner_admissible_under_recorded_regime
            && !bar_applied_as_guard_or_selector;
        let mut row = OperationalStageRegressionV4 {
            stage: stage.stage,
            regime: "sealed_v3_guarded_suffix".to_owned(),
            cone_enumerated: stage.cone_enumerated,
            cone_admitted: stage.cone_admitted,
            cone_deduped: stage.cone_deduped,
            open_band_clearing_count: None,
            guarded_discharger_count: Some(stage.discharger_count),
            guarded_discharger_hashes: stage.discharger_hashes.clone(),
            winner_candidate_hash: winner.candidate_hash.clone(),
            semantic_package_candidate_hash: package.candidate_hash.clone(),
            winner_hash_exact,
            winner_kernel_typed,
            winner_admissible_under_recorded_regime,
            winner_is_exact_guarded_discharger,
            bar_applied_as_guard_or_selector,
            operational_row_exact,
            derivation_hash: String::new(),
        };
        row.derivation_hash = tagged_hash("operational-stage-regression", &row);
        rows.push(row);
    }
    rows.sort_by_key(|row| row.stage);
    let stage_count_exact = rows.len() == 15 && rows.iter().map(|row| row.stage).eq(1..=15);
    let winner_hash_vector_exact = rows.iter().all(|row| row.winner_hash_exact);
    let admissibility_vector_exact = rows
        .iter()
        .all(|row| row.winner_kernel_typed && row.winner_admissible_under_recorded_regime);
    let guarded_discharger_counts_exact = rows
        .iter()
        .filter(|row| row.stage >= 8)
        .all(|row| row.guarded_discharger_count == Some(1));
    let guarded_discharger_hashes_exact = rows
        .iter()
        .filter(|row| row.stage >= 8)
        .all(|row| row.winner_is_exact_guarded_discharger);
    let no_winner_divergence = sealed_v2_burn.winner_divergences.is_empty()
        && sealed_v3_burn.winner_divergences.is_empty()
        && rows.iter().all(|row| row.winner_hash_exact);
    let legacy_live_definition_reexecution_required = false;
    let exact_operational_regression = v2_program_digest_valid
        && v2_burn_digest_valid
        && v2_program_burn_link_exact
        && v3_program_digest_valid
        && v3_burn_digest_valid
        && v3_program_burn_link_exact
        && v3_imported_v2_links_exact
        && stage_count_exact
        && winner_hash_vector_exact
        && admissibility_vector_exact
        && guarded_discharger_counts_exact
        && guarded_discharger_hashes_exact
        && no_winner_divergence
        && rows.iter().all(|row| row.operational_row_exact);
    let mut audit = OperationalRegressionV4 {
        v2_program_self_digest_valid: v2_program_digest_valid,
        v2_burn_self_digest_valid: v2_burn_digest_valid,
        v2_program_burn_link_exact,
        v3_program_self_digest_valid: v3_program_digest_valid,
        v3_burn_self_digest_valid: v3_burn_digest_valid,
        v3_program_burn_link_exact,
        v3_imported_v2_links_exact,
        legacy_live_definition_reexecution_required,
        rows,
        stage_count_exact,
        winner_hash_vector_exact,
        admissibility_vector_exact,
        guarded_discharger_counts_exact,
        guarded_discharger_hashes_exact,
        no_winner_divergence,
        exact_operational_regression,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("operational-regression", &audit);
    Ok(audit)
}

/// F-NR4 surface: deliberately no annotation, cause, verdict, or
/// reconciliation field can be serialized in a row.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BareRegisterRowV4 {
    pub stage: u32,
    pub structural: u32,
    pub semantic: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PostSealRegisterAuditV4 {
    pub archive_schema: String,
    pub archive_schema_exact: bool,
    pub archive_self_digest_valid: bool,
    pub archive_step_count_exact: bool,
    pub candidate_hash_vector_exact: bool,
    pub predecessor_signature_vector_exact: bool,
    pub structural_vector: Vec<u32>,
    pub authoritative_semantic_vector: Option<Vec<u32>>,
    pub non_authoritative_extraction_floor: Option<Vec<u32>>,
    pub table: Vec<BareRegisterRowV4>,
    pub divergence_table_emitted: bool,
    pub table_has_exactly_fifteen_rows: bool,
    pub table_rows_have_exactly_three_bare_fields: bool,
    pub intrinsic_seal_matches_package_vector_before_archive_comparison: bool,
    pub ordering_evidence_scope: String,
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

fn post_seal_register_audit(
    sealed_intrinsic_digest: &str,
    packages: &[ActLocalSemanticProvenanceV4Certificate],
    global_extraction_complete: bool,
) -> Result<PostSealRegisterAuditV4, TBiNu1RegressionV4Error> {
    // This is the structural-register read.  The caller has already sealed
    // the complete intrinsic package vector.
    let archive: Phase5bHistoryCertificate = serde_json::from_slice(HISTORY_BYTES)
        .map_err(|error| TBiNu1RegressionV4Error::Json(error.to_string()))?;
    let archive_schema_exact = archive.schema == PHASE5B_HISTORY_CERT_SCHEMA;
    let archive_digest_valid = archive_self_digest_valid(&archive);
    let archive_step_count_exact = archive.steps.len() == 15 && packages.len() == 15;
    let candidate_hash_vector_exact = packages.len() == archive.steps.len()
        && packages
            .iter()
            .zip(&archive.steps)
            .all(|(package, archived)| {
                package.stage == archived.step && package.candidate_hash == archived.candidate_hash
            });
    let predecessor_signature_vector_exact = packages.len() == archive.steps.len()
        && packages
            .iter()
            .zip(&archive.steps)
            .all(|(package, archived)| {
                package.stage == archived.step
                    && package.predecessor_signature_digest == archived.predecessor_signature_digest
            });
    let structural_vector = archive
        .steps
        .iter()
        .map(|step| step.certified_semantic_total)
        .collect::<Vec<_>>();
    let extraction_floor = packages
        .iter()
        .map(|package| package.semantic_family_nu)
        .collect::<Vec<_>>();
    let intrinsic_extraction_authoritative = global_extraction_complete
        && packages
            .iter()
            .all(|package| package.authoritative_semantic_extraction);
    let authoritative_semantic_vector =
        intrinsic_extraction_authoritative.then(|| extraction_floor.clone());
    let non_authoritative_extraction_floor =
        (!intrinsic_extraction_authoritative).then_some(extraction_floor);
    let table = if intrinsic_extraction_authoritative {
        archive
            .steps
            .iter()
            .zip(packages)
            .map(|(archived, package)| BareRegisterRowV4 {
                stage: archived.step,
                structural: archived.certified_semantic_total,
                semantic: package.semantic_family_nu,
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let divergence_table_emitted = !table.is_empty();
    let table_has_exactly_fifteen_rows =
        table.len() == 15 && table.iter().map(|row| row.stage).eq(1..=15);
    let table_rows_have_exactly_three_bare_fields = divergence_table_emitted
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
        });
    let expected_intrinsic_digest = tagged_hash(
        "sealed-intrinsic-issuance-before-post-seal-inputs",
        &packages
            .iter()
            .map(|package| package.derivation_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let intrinsic_seal_matches_package_vector_before_archive_comparison =
        sealed_intrinsic_digest == expected_intrinsic_digest;
    let mut audit = PostSealRegisterAuditV4 {
        archive_schema: archive.schema,
        archive_schema_exact,
        archive_self_digest_valid: archive_digest_valid,
        archive_step_count_exact,
        candidate_hash_vector_exact,
        predecessor_signature_vector_exact,
        structural_vector,
        authoritative_semantic_vector,
        non_authoritative_extraction_floor,
        table,
        divergence_table_emitted,
        table_has_exactly_fifteen_rows,
        table_rows_have_exactly_three_bare_fields,
        intrinsic_seal_matches_package_vector_before_archive_comparison,
        ordering_evidence_scope: "The regression issuer constructs and hashes the full intrinsic package vector, then passes that matching seal into this archive-comparison function. Transitive archive-freedom of intrinsic callees is tracked separately and remains unproved.".to_owned(),
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("post-seal-register-audit", &audit);
    Ok(audit)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiNu1RegressionV4Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub source_bindings: Vec<TBiNu1V4SourceBinding>,
    pub intrinsic_packages: Vec<ActLocalSemanticProvenanceV4Certificate>,
    pub intrinsic_issuance_digest_sealed_before_post_seal_inputs: String,
    pub intrinsic_isolation: IntrinsicV4IsolationAudit,
    pub role_declaration_count: usize,
    pub proved_family_declaration_count: usize,
    pub theorem_impossibility_declaration_count: usize,
    pub named_registry_residual_count: usize,
    pub resolved_role_declaration_count: usize,
    pub silent_residue_count: usize,
    pub extraction_complete: bool,
    pub operational: OperationalRegressionV4,
    pub registers: PostSealRegisterAuditV4,
    pub f_al1_prime_passed: bool,
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
pub struct TBiNu1RegressionV4Replay {
    pub valid: bool,
    pub theorem_proved: bool,
    pub bi0_rerun_authorized: bool,
    pub extraction_complete: bool,
    pub operational_regression_exact: bool,
    pub f_al1_prime_passed: bool,
    pub authoritative_semantic_vector: Option<Vec<u32>>,
    pub non_authoritative_extraction_floor: Option<Vec<u32>>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiNu1RegressionV4Error {
    #[error("intrinsic semantic extraction failed: {0}")]
    Intrinsic(String),
    #[error("operational regression failed: {0}")]
    Operational(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted replay failed: {0}")]
    EmittedReplay(String),
    #[error("positive gate did not pass: {0}")]
    Gate(String),
}

fn certificate_digest(certificate: &TBiNu1RegressionV4Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("regression-certificate", &projection)
}

pub fn issue_t_bi_nu1_regression_v4_certificate()
-> Result<TBiNu1RegressionV4Certificate, TBiNu1RegressionV4Error> {
    let packages = issue_reference_act_local_semantic_sequence_v4()
        .map_err(|error| TBiNu1RegressionV4Error::Intrinsic(error.to_string()))?;
    let intrinsic_issuance_digest_sealed_before_post_seal_inputs = tagged_hash(
        "sealed-intrinsic-issuance-before-post-seal-inputs",
        &packages
            .iter()
            .map(|package| package.derivation_hash.as_str())
            .collect::<Vec<_>>(),
    );
    let intrinsic_isolation = intrinsic_isolation_audit(&packages);
    let role_declaration_count = packages
        .iter()
        .map(|package| package.v3_role_schema_gap_count)
        .sum::<usize>();
    let proved_family_declaration_count = packages
        .iter()
        .map(|package| package.proved_role_declaration_count)
        .sum::<usize>();
    let theorem_impossibility_declaration_count = packages
        .iter()
        .map(|package| package.impossible_role_declaration_count)
        .sum::<usize>();
    let named_registry_residual_count = packages
        .iter()
        .map(|package| package.named_role_residual_count)
        .sum::<usize>();
    let resolved_role_declaration_count = packages
        .iter()
        .map(|package| package.resolved_role_declaration_count)
        .sum::<usize>();
    let silent_residue_count = packages
        .iter()
        .map(|package| package.silent_role_residue_count)
        .sum::<usize>();
    let extraction_complete = packages.len() == 15
        && role_declaration_count == 250
        && resolved_role_declaration_count == 250
        && proved_family_declaration_count + theorem_impossibility_declaration_count == 250
        && silent_residue_count == 0
        && packages
            .iter()
            .all(|package| package.authoritative_semantic_extraction)
        && intrinsic_isolation.proof_bearing_isolation_passed;

    // No post-seal input is touched before the complete intrinsic digest
    // above exists.
    let operational = post_seal_operational_regression(&packages)?;
    let registers = post_seal_register_audit(
        &intrinsic_issuance_digest_sealed_before_post_seal_inputs,
        &packages,
        extraction_complete,
    )?;
    let f_al1_prime_passed = extraction_complete
        && operational.exact_operational_regression
        && registers.archive_self_digest_valid
        && registers.archive_schema_exact
        && registers.archive_step_count_exact
        && registers.candidate_hash_vector_exact
        && registers.predecessor_signature_vector_exact
        && registers.table_has_exactly_fifteen_rows
        && registers.table_rows_have_exactly_three_bare_fields
        && registers.intrinsic_seal_matches_package_vector_before_archive_comparison;
    let t_bi_nu1_proved_on_enacted_branch = f_al1_prime_passed;
    let mut certificate = TBiNu1RegressionV4Certificate {
        schema: T_BI_NU1_REGRESSION_V4_SCHEMA.to_owned(),
        date: T_BI_NU1_REGRESSION_V4_DATE.to_owned(),
        theorem_id: T_BI_NU1_REGRESSION_V4_THEOREM_ID.to_owned(),
        source_bindings: source_bindings(),
        intrinsic_packages: packages,
        intrinsic_issuance_digest_sealed_before_post_seal_inputs,
        intrinsic_isolation,
        role_declaration_count,
        proved_family_declaration_count,
        theorem_impossibility_declaration_count,
        named_registry_residual_count,
        resolved_role_declaration_count,
        silent_residue_count,
        extraction_complete,
        operational,
        registers,
        f_al1_prime_passed,
        t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized_by_t_bi_nu1_side: t_bi_nu1_proved_on_enacted_branch,
        non_enacted_branch_work_executed: false,
        outcome: if t_bi_nu1_proved_on_enacted_branch {
            "T_BI_NU1_V4_PROVED_F_AL1_PRIME_PASSED"
        } else {
            "T_BI_NU1_V4_GATE_FAILED"
        }
        .to_owned(),
        permitted_conclusion: if t_bi_nu1_proved_on_enacted_branch {
            "The enacted fifteen-act surface has a complete act-local semantic-family ledger. Structural totals remain post-seal testimony. This artifact authorizes only the T-BI-NU1 side of BI-0; the independent slot-map prerequisite must also pass."
        } else {
            "The unified registry audit is a replayable negative result. No semantic vector, authoritative ledger, or divergence table is issued: ordinary schemas remain behind their explicit C1 bridge, exact role/anchor relations remain named residuals, and transitive archive isolation is not yet proved. T-BI-NU1, F-AL1-prime, BI-0, BI-1, and cone scoring remain closed."
        }
        .to_owned(),
        required_successor_action: if t_bi_nu1_proved_on_enacted_branch {
            "Join the separately passing chronological slot-map prerequisite and rerun BI-0 create-new before BI-1, BI-4, or cone scoring."
        } else {
            "Prove a candidate-plus-prefix-local pen_core::Expr-to-schema bridge, the exact family-to-role anchor relations, and a transitive intrinsic call-graph/capability isolation theorem; then rerun this create-new audit without consulting the structural register."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> TBiNu1RegressionV4Replay {
    TBiNu1RegressionV4Replay {
        valid: false,
        theorem_proved: false,
        bi0_rerun_authorized: false,
        extraction_complete: false,
        operational_regression_exact: false,
        f_al1_prime_passed: false,
        authoritative_semantic_vector: None,
        non_authoritative_extraction_floor: None,
        errors: vec![error.into()],
    }
}

pub fn replay_t_bi_nu1_regression_v4_certificate(
    claimed: &TBiNu1RegressionV4Certificate,
) -> TBiNu1RegressionV4Replay {
    let expected = match issue_t_bi_nu1_regression_v4_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-BI-NU1 v4 regression certificate digest mismatch".to_owned());
    }
    if claimed != &expected {
        errors.push("T-BI-NU1 v4 certificate differs from create-new reissuance".to_owned());
    }
    let logical_gate = claimed.extraction_complete
        && claimed.operational.exact_operational_regression
        && claimed.f_al1_prime_passed;
    if claimed.t_bi_nu1_proved_on_enacted_branch != logical_gate
        || claimed.bi0_rerun_authorized_by_t_bi_nu1_side != logical_gate
        || claimed.non_enacted_branch_work_executed
    {
        errors.push("T-BI-NU1 v4 conclusion or BI sequencing surface is invalid".to_owned());
    }
    TBiNu1RegressionV4Replay {
        valid: errors.is_empty(),
        theorem_proved: claimed.t_bi_nu1_proved_on_enacted_branch,
        bi0_rerun_authorized: claimed.bi0_rerun_authorized_by_t_bi_nu1_side,
        extraction_complete: claimed.extraction_complete,
        operational_regression_exact: claimed.operational.exact_operational_regression,
        f_al1_prime_passed: claimed.f_al1_prime_passed,
        authoritative_semantic_vector: claimed.registers.authoritative_semantic_vector.clone(),
        non_authoritative_extraction_floor: claimed
            .registers
            .non_authoritative_extraction_floor
            .clone(),
        errors,
    }
}

pub fn replay_t_bi_nu1_regression_v4_json(json: &str) -> TBiNu1RegressionV4Replay {
    match serde_json::from_str::<TBiNu1RegressionV4Certificate>(json) {
        Ok(certificate) => replay_t_bi_nu1_regression_v4_certificate(&certificate),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn render_t_bi_nu1_v4_report(certificate: &TBiNu1RegressionV4Certificate) -> String {
    let register_section = if certificate
        .registers
        .authoritative_semantic_vector
        .is_some()
    {
        let mut table = String::new();
        for row in &certificate.registers.table {
            table.push_str(&format!(
                "| {} | {} | {} |\n",
                row.stage, row.structural, row.semantic
            ));
        }
        format!(
            "The required register table follows verbatim and uninterpreted.\n\n| Stage | Structural | Semantic |\n|---:|---:|---:|\n{table}"
        )
    } else {
        format!(
            "F-NR3 suppresses the structural/semantic divergence table because extraction is incomplete. The only intrinsic diagnostic is the explicitly non-authoritative extraction floor: `{:?}`. It is not a semantic register or history.",
            certificate
                .registers
                .non_authoritative_extraction_floor
                .as_deref()
                .unwrap_or(&[])
        )
    };
    format!(
        "# T-BI-NU1 semantic-family provenance v4 result\n\n**Date:** {}. **Outcome:** `{}`.\n\nThe intrinsic issuer resolved {}/{} declarations ({} proved families; {} theorem-backed impossibilities; {} named registry residuals) with {} silent residue. Its complete package digest was sealed before operational or structural artifacts were opened. Direct source/runtime isolation diagnostics: **{}**. Transitive proof-bearing isolation: **{}** (`{:?}`). Exact operational regression: **{}**. F-AL1-prime passed: **{}**.\n\n{}\n\nPermitted conclusion: {}\n\nNext: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.resolved_role_declaration_count,
        certificate.role_declaration_count,
        certificate.proved_family_declaration_count,
        certificate.theorem_impossibility_declaration_count,
        certificate.named_registry_residual_count,
        certificate.silent_residue_count,
        certificate
            .intrinsic_isolation
            .direct_source_scan_and_runtime_flags_clear,
        certificate
            .intrinsic_isolation
            .transitive_call_graph_isolation_proved,
        certificate.intrinsic_isolation.named_residuals,
        certificate.operational.exact_operational_regression,
        certificate.f_al1_prime_passed,
        register_section,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.result_digest,
    )
}

pub fn emit_t_bi_nu1_regression_v4_create_new(
    directory: &Path,
) -> Result<TBiNu1RegressionV4Certificate, TBiNu1RegressionV4Error> {
    let certificate = issue_t_bi_nu1_regression_v4_certificate()?;
    // A negative create-new artifact is a theorem result, not a promotion.
    // The certificate's explicit gates remain false and replay checks them.
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| TBiNu1RegressionV4Error::Json(error.to_string()))?;
    let report = render_t_bi_nu1_v4_report(&certificate);
    let replay = replay_t_bi_nu1_regression_v4_json(&json);
    if !replay.valid {
        return Err(TBiNu1RegressionV4Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let certificate_path = directory.join(T_BI_NU1_V4_CERTIFICATE_NAME);
    let report_path = directory.join(T_BI_NU1_V4_REPORT_NAME);
    if certificate_path.exists() || report_path.exists() {
        return Err(TBiNu1RegressionV4Error::Io(format!(
            "create-new target already exists: {} or {}",
            certificate_path.display(),
            report_path.display()
        )));
    }
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| TBiNu1RegressionV4Error::Io(error.to_string()))?;
    certificate_file
        .write_all(json.as_bytes())
        .map_err(|error| TBiNu1RegressionV4Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| TBiNu1RegressionV4Error::Io(error.to_string()))?;
    report_file
        .write_all(report.as_bytes())
        .map_err(|error| TBiNu1RegressionV4Error::Io(error.to_string()))?;
    Ok(certificate)
}

pub fn replay_t_bi_nu1_regression_v4_directory(
    directory: &Path,
) -> Result<TBiNu1RegressionV4Replay, TBiNu1RegressionV4Error> {
    let json = std::fs::read_to_string(directory.join(T_BI_NU1_V4_CERTIFICATE_NAME))
        .map_err(|error| TBiNu1RegressionV4Error::Io(error.to_string()))?;
    Ok(replay_t_bi_nu1_regression_v4_json(&json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_v4_gate_is_replayable_and_table_is_bare() {
        let certificate = issue_t_bi_nu1_regression_v4_certificate().expect("v4 regression");
        assert!(!certificate.extraction_complete);
        assert!(certificate.operational.exact_operational_regression);
        assert!(!certificate.f_al1_prime_passed);
        assert!(!certificate.t_bi_nu1_proved_on_enacted_branch);
        assert!(!certificate.bi0_rerun_authorized_by_t_bi_nu1_side);
        assert!(
            certificate
                .intrinsic_isolation
                .direct_source_scan_and_runtime_flags_clear
        );
        assert!(
            !certificate
                .intrinsic_isolation
                .transitive_call_graph_isolation_proved
        );
        assert!(
            !certificate
                .intrinsic_isolation
                .proof_bearing_isolation_passed
        );
        assert_eq!(
            certificate.intrinsic_isolation.named_residuals,
            vec!["T_BI_NU1_TRANSITIVE_INTRINSIC_ISOLATION_UNPROVED"]
        );
        assert_eq!(certificate.role_declaration_count, 250);
        assert_eq!(certificate.named_registry_residual_count, 250);
        assert_eq!(certificate.proved_family_declaration_count, 0);
        assert_eq!(certificate.silent_residue_count, 0);
        assert!(certificate.registers.table.is_empty());
        assert!(!certificate.registers.divergence_table_emitted);
        assert!(
            certificate
                .registers
                .authoritative_semantic_vector
                .is_none()
        );
        assert_eq!(
            certificate.registers.non_authoritative_extraction_floor,
            Some(vec![0; 15])
        );
        assert!(replay_t_bi_nu1_regression_v4_certificate(&certificate).valid);
    }

    #[test]
    fn f_nr3_uses_the_global_extraction_gate_not_package_flags_alone() {
        let mut packages =
            issue_reference_act_local_semantic_sequence_v4().expect("intrinsic semantic packages");
        for package in &mut packages {
            package.authoritative_semantic_extraction = true;
        }
        let seal = tagged_hash(
            "sealed-intrinsic-issuance-before-post-seal-inputs",
            &packages
                .iter()
                .map(|package| package.derivation_hash.as_str())
                .collect::<Vec<_>>(),
        );
        let audit = post_seal_register_audit(&seal, &packages, false)
            .expect("post-seal negative register audit");
        assert!(audit.authoritative_semantic_vector.is_none());
        assert!(audit.table.is_empty());
        assert!(!audit.divergence_table_emitted);
    }

    #[test]
    fn resigned_register_or_operational_mutations_fail_reissue() {
        let certificate = issue_t_bi_nu1_regression_v4_certificate().expect("v4 regression");
        let mut register_forgery = certificate.clone();
        register_forgery
            .registers
            .non_authoritative_extraction_floor
            .as_mut()
            .expect("negative floor")[0] += 1;
        register_forgery.registers.derivation_hash =
            tagged_hash("post-seal-register-audit", &register_forgery.registers);
        register_forgery.result_digest = certificate_digest(&register_forgery);
        assert!(!replay_t_bi_nu1_regression_v4_certificate(&register_forgery).valid);

        let mut operational_forgery = certificate;
        operational_forgery.operational.rows[7].guarded_discharger_count = Some(2);
        operational_forgery.operational.rows[7].derivation_hash = tagged_hash(
            "operational-stage-regression",
            &operational_forgery.operational.rows[7],
        );
        operational_forgery.operational.derivation_hash =
            tagged_hash("operational-regression", &operational_forgery.operational);
        operational_forgery.result_digest = certificate_digest(&operational_forgery);
        assert!(!replay_t_bi_nu1_regression_v4_certificate(&operational_forgery).valid);
    }
}
