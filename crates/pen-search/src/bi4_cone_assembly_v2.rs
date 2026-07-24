//! BI-4 v2: cone assembly over the sealed BI-2 v2 branch finales.
//!
//! This module changes only the input lineage of the retired BI-4 issuer.
//! G2, G3, G4, and the zone taxonomy retain the frozen BI-1 meanings.
//! The write-only BI-1b correspondence ledger is opened only after all
//! comparison verdicts and the F-R3-B1 disposition have been computed.

use crate::bi1b_prefix_general_sweep_v1::{
    BI1B_BRANCH_V1_SCHEMA, BI1B_CORRESPONDENCE_V1_SCHEMA, BI1B_PREFIX_GENERAL_SWEEP_V1_SCHEMA,
    Bi1bBranchCertificateV1, Bi1bCorrespondenceDiagnosticV1,
};
use crate::bi2_branch_finales_v1::{
    BI2_FOUR_BRANCH_FINALES_V2_SCHEMA, Bi2BranchFinaleCertificateV1, Bi2FourBranchFinalesBundleV1,
    Bi2FourBranchFinalesIndexV1, replay_bi2_four_branch_finales_v1,
};
use crate::branch_invariance::BranchContinuationOutcome;
use crate::branch_invariance_resume_v1::Bi1bResumeOutcomeV1;
use crate::branch_invariance_sweep_v3::{
    BI1_BRANCH_V3_SCHEMA, BI1_OPTION_A_SWEEP_V3_SCHEMA, Bi1BranchCertificateV3,
};
use crate::r_t2_future_hole_confluence_v2::{
    R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA, Rt2FutureHoleConfluenceV2Certificate,
    Rt2FutureHoleConfluenceV2Outcome,
};
use pen_core::hash::blake3_hex;
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const BI4_CONE_ASSEMBLY_V2_SCHEMA: &str = "bi4-cone-assembly-v2";
pub const BI4_CONE_ASSEMBLY_V2_DATE: &str = "2026-07-23";
pub const BI4_CONE_V2_CERTIFICATE_NAME: &str = "BI4_CONE_V2_CERTIFICATE.json";
pub const BI4_CONE_V2_REPORT_NAME: &str = "BI4_CONE_REPORT_V2.md";
pub const FROZEN_BI2_V2_INDEX_DIGEST: &str =
    "blake3:cfa5bbd27c6f2c9c788ae6eac68e2128f87d18ce54362e72850b8d3aa1fc03ff";

const BI4_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/bi4_assembly_v2_plan.md");
const BI1_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/branch_invariance_program_plan.md");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("bi4_cone_assembly_v2.rs");
const BI2_SOURCE_BYTES: &[u8] = include_bytes!("bi2_branch_finales_v1.rs");
const FINALE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_finale.rs");
const OBSOLETE_BI4_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_program.rs");
const BAR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/bar.rs");
const COHERENCE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/coherence.rs");

const BI2_INDEX_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json");
const BI2_BRANCH_201_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_2016726758f3_V2_CERTIFICATE.json");
const BI2_BRANCH_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_43a0ed707770_V2_CERTIFICATE.json");
const BI2_BRANCH_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_4b2211ecae25_V2_CERTIFICATE.json");
const BI2_BRANCH_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_b4f821d9bb28_V2_CERTIFICATE.json");
const RETIRED_BI1_BRANCH_201_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json");
const RETIRED_BI1_BRANCH_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json");
const RETIRED_BI1_BRANCH_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json");
const RETIRED_BI1_BRANCH_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json");
const BI1B_BRANCH_201_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json");
const BI1B_BRANCH_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json");
const BI1B_BRANCH_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json");
const BI1B_BRANCH_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json");
const CORRESPONDENCE_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_CORRESPONDENCE_DIAGNOSTIC.json");
const R_T2_BYTES: &[u8] =
    include_bytes!("../../../docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4SourceBindingV2 {
    pub path: String,
    pub role: String,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi4VerdictV2 {
    Passed,
    Refuted,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4GranularityAuditV2 {
    pub level: String,
    pub verdict: Bi4VerdictV2,
    pub passed: bool,
    pub exact_basis: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4G2ProfileRowV2 {
    pub stage: u32,
    pub required_packages: Vec<String>,
    pub structural_constructors: Vec<String>,
    pub live_obligation_count: usize,
    pub total_typed_discharge: bool,
    pub debt_free: bool,
    pub exact_bi2_window_step_join: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4LedgerRowV2 {
    pub stage: u32,
    pub candidate_hash: String,
    pub kappa: u32,
    pub semantic_nu: u32,
    pub provenance_replayed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4BarRowV2 {
    pub stage: u32,
    pub phi: String,
    pub omega: String,
    pub bar: String,
    pub complete_prior_history_used: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4BranchMeasurementV2 {
    pub branch_root_hash: String,
    pub branch_certificate_digest: String,
    pub retired_bi1_branch_certificate_digest: String,
    pub bi1b_branch_certificate_digest: String,
    pub enacted_root: bool,
    pub g2_profile_stage5_through16: Vec<Bi4G2ProfileRowV2>,
    pub complete_ledger_stage1_through15: Vec<Bi4LedgerRowV2>,
    pub g3a_ledger_stage5_through15: Vec<Bi4LedgerRowV2>,
    pub complete_sum_kappa: u32,
    pub complete_sum_semantic_nu: u32,
    pub suffix_sum_semantic_nu_stage5_through15: u32,
    pub diagnostic_bar_stage5_through15: Vec<Bi4BarRowV2>,
    pub g2_original_unique_winner_total_discharge_predicate_replayed: bool,
    pub every_ledger_row_provenance_replayed: bool,
    pub local_g4_debt_free_halt_at_15: bool,
    pub halt_step: u32,
    pub successor_stage: u32,
    pub semantic_o16_empty: bool,
    pub f1_executed: bool,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub exact_d_partition: bool,
    pub theorem12_full_instance_granularity: bool,
    pub e5_complete: bool,
    pub expressivity_gap_count: usize,
    pub halt_boundary_premise_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4PairComparisonV2 {
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub equal: bool,
    pub first_divergence_stage: Option<u32>,
    pub exact_context: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bi4DispositionStatusV2 {
    PromotedToConeLevel,
    RemainsBranchIndexed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4DispositionRowV2 {
    pub claim: String,
    pub status: Bi4DispositionStatusV2,
    pub exact_verdict_premise: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4DriftDisclosureV2 {
    pub bi1b_public_deterministic_replay_passed: bool,
    pub frozen_testimony_fallback_used: bool,
    pub public_replay_errors: Vec<String>,
    pub drift_repaired_inside_assembly: bool,
    pub drift_concealed_inside_assembly: bool,
    pub obsolete_bi4_issuer_retired_not_deleted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4CorrespondenceCorroborationV2 {
    pub opened_after_all_comparison_verdicts: bool,
    pub artifact_blake3: String,
    pub artifact_self_digest_valid: bool,
    pub row_count: usize,
    pub exact_three_alternate_by_eight_stage_surface: bool,
    pub every_stage8_through15_winner_byte_identical: bool,
    pub every_winner_joins_sealed_bi2_ledger: bool,
    pub used_by_any_comparison_verdict: bool,
    pub used_by_selector: bool,
    pub used_as_provenance: bool,
    pub corroboration_only: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4InheritedG1TestimonyV2 {
    pub artifact_blake3: String,
    pub certificate_digest: String,
    pub outcome: String,
    pub artifact_self_digest_valid: bool,
    pub r_t2_confluence_refuted: bool,
    pub all_stage5_scheme_sets_inequivalent: bool,
    pub order_reversal_invariant: bool,
    pub selector_absent: bool,
    pub exact_six_inequivalent_five_scheme_pairs: bool,
    pub re_litigated_by_bi4_v2: bool,
    pub used_to_compute_g2_g3_or_g4: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4ConeAssemblyV2Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Bi4SourceBindingV2>,
    pub bi2_index_digest: String,
    pub bi2_index_schema: String,
    pub bi2_index_digest_matches_frozen_plan: bool,
    pub bi1b_sweep_digest: String,
    pub bi1b_regression_certificate_digest: String,
    pub prefix_general_extraction_authority_digest: String,
    pub bi2_bundle_replay_valid_before_comparison: bool,
    pub bi2_bundle_replay_errors: Vec<String>,
    pub upstream_lineage_authenticated_by_bi2_replay: bool,
    pub exact_four_distinct_branch_roots: bool,
    pub every_branch_digest_joins_bi2_index: bool,
    pub every_branch_certificate_reissued_before_comparison: bool,
    pub bi2_ready_for_bi4_assembly: bool,
    pub comparison_semantics_changed_from_bi1: bool,
    pub granularity_definitions_changed_from_bi1: bool,
    pub zone_taxonomy_changed_from_bi1: bool,
    pub all_comparisons_computed_before_any_aggregate_verdict: bool,
    pub all_six_pairs_published_for_each_comparison: bool,
    pub branches: Vec<Bi4BranchMeasurementV2>,
    pub g2_pair_comparisons: Vec<Bi4PairComparisonV2>,
    pub g3a_pair_comparisons: Vec<Bi4PairComparisonV2>,
    pub g3b_pair_comparisons: Vec<Bi4PairComparisonV2>,
    pub g3c_pair_comparisons: Vec<Bi4PairComparisonV2>,
    pub g2: Bi4GranularityAuditV2,
    pub g3a: Bi4GranularityAuditV2,
    pub g3b: Bi4GranularityAuditV2,
    pub g3c: Bi4GranularityAuditV2,
    pub g3_composite: Bi4GranularityAuditV2,
    pub cone_g4: Bi4GranularityAuditV2,
    pub cone_g4_halt_boundary_premise_branch_index_free: bool,
    pub stage4_offset_only_explanation_proved: bool,
    pub z_tree_precondition_present: bool,
    pub z_stop_precondition_present: bool,
    pub outcome_zone: String,
    pub refinement_label: Option<String>,
    pub first_new_divergence_published_verbatim: Option<String>,
    pub inherited_g1_testimony: Bi4InheritedG1TestimonyV2,
    pub branch_index_disposition: Vec<Bi4DispositionRowV2>,
    pub every_claim_disposed_explicitly: bool,
    pub correspondence_corroboration: Bi4CorrespondenceCorroborationV2,
    pub drift_disclosure: Bi4DriftDisclosureV2,
    pub no_divergence_suppressed_averaged_or_repaired: bool,
    pub class_representative_substitution_used: bool,
    pub enacted_branch_used_as_comparison_baseline: bool,
    pub uc1_content_present: bool,
    pub bridge_executed: bool,
    pub final_certificate_issued: bool,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi4ConeAssemblyV2Replay {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bi4ConeAssemblyV2Error {
    #[error("BI-4 v2 input failure: {0}")]
    Input(String),
    #[error("BI-4 v2 invariant failure: {0}")]
    Invariant(String),
    #[error("BI-4 v2 JSON failure: {0}")]
    Json(String),
    #[error("BI-4 v2 create-new I/O failure: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI4_CONE_ASSEMBLY_V2_SCHEMA, domain, value))
        .expect("BI-4 v2 evidence serializes");
    bytes_hash(&bytes)
}

fn foreign_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes =
        serde_json::to_vec(&(schema, domain, value)).expect("foreign BI evidence serializes");
    bytes_hash(&bytes)
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> Bi4SourceBindingV2 {
    Bi4SourceBindingV2 {
        path: path.to_owned(),
        role: role.to_owned(),
        blake3: bytes_hash(bytes),
    }
}

fn source_bindings() -> Vec<Bi4SourceBindingV2> {
    vec![
        source_binding(
            "docs/bi4_assembly_v2_plan.md",
            "frozen BI-4 v2 input-lineage successor and falsifiers",
            BI4_PLAN_BYTES,
        ),
        source_binding(
            "docs/branch_invariance_program_plan.md",
            "frozen G1-G4 meanings and zone taxonomy",
            BI1_PLAN_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/bi4_cone_assembly_v2.rs",
            "BI-4 v2 issue, replay, comparison, and create-new implementation",
            THIS_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/bi2_branch_finales_v1.rs",
            "sealed BI-2 v2 replay boundary",
            BI2_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/branch_invariance_finale.rs",
            "branch-local A3, O16, F1, and Theorem-12 evidence",
            FINALE_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/branch_invariance_program.rs",
            "retired BI-4 issuer retained as frozen testimony",
            OBSOLETE_BI4_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-eval/src/bar.rs",
            "registered descriptive bar recurrence",
            BAR_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-eval/src/coherence.rs",
            "registered d=2 Fibonacci delta sequence",
            COHERENCE_SOURCE_BYTES,
        ),
        source_binding(
            "docs/BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json",
            "sealed BI-2 v2 aggregate",
            BI2_INDEX_BYTES,
        ),
        source_binding(
            "docs/BI2_BRANCH_2016726758f3_V2_CERTIFICATE.json",
            "sealed BI-2 v2 branch",
            BI2_BRANCH_201_BYTES,
        ),
        source_binding(
            "docs/BI2_BRANCH_43a0ed707770_V2_CERTIFICATE.json",
            "sealed BI-2 v2 branch",
            BI2_BRANCH_43_BYTES,
        ),
        source_binding(
            "docs/BI2_BRANCH_4b2211ecae25_V2_CERTIFICATE.json",
            "sealed BI-2 v2 branch",
            BI2_BRANCH_4B_BYTES,
        ),
        source_binding(
            "docs/BI2_BRANCH_b4f821d9bb28_V2_CERTIFICATE.json",
            "sealed BI-2 v2 branch",
            BI2_BRANCH_B4_BYTES,
        ),
        source_binding(
            "docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json",
            "retired BI-1 branch testimony for the original G2 total-discharge predicate",
            RETIRED_BI1_BRANCH_201_BYTES,
        ),
        source_binding(
            "docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json",
            "retired BI-1 branch testimony for the original G2 total-discharge predicate",
            RETIRED_BI1_BRANCH_43_BYTES,
        ),
        source_binding(
            "docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json",
            "retired BI-1 branch testimony for the original G2 total-discharge predicate",
            RETIRED_BI1_BRANCH_4B_BYTES,
        ),
        source_binding(
            "docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json",
            "retired BI-1 branch testimony for the original G2 total-discharge predicate",
            RETIRED_BI1_BRANCH_B4_BYTES,
        ),
        source_binding(
            "docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json",
            "retained BI-1b continuation testimony for the original G2 total-discharge predicate",
            BI1B_BRANCH_201_BYTES,
        ),
        source_binding(
            "docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json",
            "retained BI-1b continuation testimony for the original G2 total-discharge predicate",
            BI1B_BRANCH_43_BYTES,
        ),
        source_binding(
            "docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json",
            "retained BI-1b continuation testimony for the original G2 total-discharge predicate",
            BI1B_BRANCH_4B_BYTES,
        ),
        source_binding(
            "docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json",
            "retained BI-1b continuation testimony for the original G2 total-discharge predicate",
            BI1B_BRANCH_B4_BYTES,
        ),
        source_binding(
            "docs/BI1B_CORRESPONDENCE_DIAGNOSTIC.json",
            "post-verdict read-only corroboration",
            CORRESPONDENCE_BYTES,
        ),
        source_binding(
            "docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json",
            "inherited G1 refutation testimony only",
            R_T2_BYTES,
        ),
    ]
}

fn row_hash<T: Serialize + Clone>(domain: &str, row: &T) -> String {
    tagged_hash(domain, row)
}

fn embedded_bi2_bundle() -> Result<Bi2FourBranchFinalesBundleV1, Bi4ConeAssemblyV2Error> {
    let index = serde_json::from_slice::<Bi2FourBranchFinalesIndexV1>(BI2_INDEX_BYTES)
        .map_err(|error| Bi4ConeAssemblyV2Error::Json(error.to_string()))?;
    let mut branches = [
        BI2_BRANCH_201_BYTES,
        BI2_BRANCH_43_BYTES,
        BI2_BRANCH_4B_BYTES,
        BI2_BRANCH_B4_BYTES,
    ]
    .into_iter()
    .map(|bytes| {
        serde_json::from_slice::<Bi2BranchFinaleCertificateV1>(bytes)
            .map_err(|error| Bi4ConeAssemblyV2Error::Json(error.to_string()))
    })
    .collect::<Result<Vec<_>, _>>()?;
    branches.sort_by(|left, right| left.branch_root_hash.cmp(&right.branch_root_hash));
    Ok(Bi2FourBranchFinalesBundleV1 { branches, index })
}

fn embedded_retired_bi1_branches()
-> Result<BTreeMap<String, Bi1BranchCertificateV3>, Bi4ConeAssemblyV2Error> {
    let mut result = BTreeMap::new();
    for bytes in [
        RETIRED_BI1_BRANCH_201_BYTES,
        RETIRED_BI1_BRANCH_43_BYTES,
        RETIRED_BI1_BRANCH_4B_BYTES,
        RETIRED_BI1_BRANCH_B4_BYTES,
    ] {
        let certificate = serde_json::from_slice::<Bi1BranchCertificateV3>(bytes)
            .map_err(|error| Bi4ConeAssemblyV2Error::Json(error.to_string()))?;
        let mut projection = certificate.clone();
        projection.result_digest.clear();
        let self_digest_valid = certificate.schema == BI1_BRANCH_V3_SCHEMA
            && certificate.result_digest
                == foreign_tagged_hash(
                    BI1_OPTION_A_SWEEP_V3_SCHEMA,
                    BI1_BRANCH_V3_SCHEMA,
                    &projection,
                );
        if !self_digest_valid
            || result
                .insert(certificate.branch_root_hash.clone(), certificate)
                .is_some()
        {
            return Err(Bi4ConeAssemblyV2Error::Input(
                "retired BI-1 branch testimony is not an exact distinct self-digested four-surface"
                    .to_owned(),
            ));
        }
    }
    if result.len() != 4 {
        return Err(Bi4ConeAssemblyV2Error::Input(format!(
            "retired BI-1 branch testimony has {} roots, expected four",
            result.len()
        )));
    }
    Ok(result)
}

fn embedded_bi1b_branches()
-> Result<BTreeMap<String, Bi1bBranchCertificateV1>, Bi4ConeAssemblyV2Error> {
    let mut result = BTreeMap::new();
    for bytes in [
        BI1B_BRANCH_201_BYTES,
        BI1B_BRANCH_43_BYTES,
        BI1B_BRANCH_4B_BYTES,
        BI1B_BRANCH_B4_BYTES,
    ] {
        let certificate = serde_json::from_slice::<Bi1bBranchCertificateV1>(bytes)
            .map_err(|error| Bi4ConeAssemblyV2Error::Json(error.to_string()))?;
        let mut projection = certificate.clone();
        projection.result_digest.clear();
        let self_digest_valid = certificate.result_digest
            == foreign_tagged_hash(
                BI1B_PREFIX_GENERAL_SWEEP_V1_SCHEMA,
                BI1B_BRANCH_V1_SCHEMA,
                &projection,
            );
        if !self_digest_valid
            || result
                .insert(certificate.branch_root_hash.clone(), certificate)
                .is_some()
        {
            return Err(Bi4ConeAssemblyV2Error::Input(
                "BI-1b branch testimony is not an exact distinct self-digested four-surface"
                    .to_owned(),
            ));
        }
    }
    if result.len() != 4 {
        return Err(Bi4ConeAssemblyV2Error::Input(format!(
            "BI-1b branch testimony has {} roots, expected four",
            result.len()
        )));
    }
    Ok(result)
}

fn original_total_discharge_witness(
    retired_bi1: &Bi1BranchCertificateV3,
    bi1b: &Bi1bBranchCertificateV1,
    stage: u32,
    expected_candidate_hash: &str,
) -> bool {
    if retired_bi1
        .continuation
        .stages
        .iter()
        .find(|row| row.stage == stage)
        .is_some_and(|row| {
            row.discharger_count == 1
                && row.discharger_hashes.len() == 1
                && row.discharger_hashes[0] == expected_candidate_hash
                && row
                    .winner
                    .as_ref()
                    .is_some_and(|winner| winner.candidate_hash == expected_candidate_hash)
        })
    {
        return true;
    }
    bi1b.resume
        .as_ref()
        .and_then(|resume| resume.stages.iter().find(|row| row.stage == stage))
        .is_some_and(|row| {
            row.every_candidate_classified_before_census
                && row.unknown_count == 0
                && row.selection_or_stop_is_lawful
                && row.census.as_ref().is_some_and(|census| {
                    census.classification_complete_before_census
                        && census.discharger_count == 1
                        && census.discharger_hashes.len() == 1
                        && census.discharger_hashes[0] == expected_candidate_hash
                        && census.unique_discharger_hash.as_deref() == Some(expected_candidate_hash)
                        && census.no_improvised_selection
                })
                && row
                    .winner
                    .as_ref()
                    .is_some_and(|winner| winner.candidate_hash == expected_candidate_hash)
        })
}

fn final_debt_free_halt_witness(
    retired_bi1: &Bi1BranchCertificateV3,
    bi1b: &Bi1bBranchCertificateV1,
) -> bool {
    if let Some(resume) = &bi1b.resume {
        return resume.branch_lawfully_disposed
            && resume.terminal_demand.as_ref().is_some_and(|demand| {
                demand.stage == 16
                    && demand.debt_free
                    && demand.coarse_required_packages.is_empty()
                    && demand.a3_required_packages.is_empty()
                    && demand.structural_constructors.is_empty()
                    && demand.exact_prefix_inventory_exhaustive
                    && demand.coarse_and_a3_demands_agree
            })
            && matches!(
                &resume.outcome,
                Bi1bResumeOutcomeV1::DebtFreeHalt {
                    halt_stage: 15,
                    next_stage: 16
                }
            );
    }
    retired_bi1.continuation.debt_free_halt_at_stage15
        && retired_bi1.continuation.completed_through_stage15
        && matches!(
            &retired_bi1.continuation.outcome,
            BranchContinuationOutcome::DebtFreeHalt {
                halt_stage: 15,
                next_stage: 16
            }
        )
}

fn final_tree_precondition(
    retired_bi1: &Bi1BranchCertificateV3,
    bi1b: &Bi1bBranchCertificateV1,
) -> bool {
    if let Some(resume) = &bi1b.resume {
        return matches!(
            &resume.outcome,
            Bi1bResumeOutcomeV1::HaltedMultipleGuardedDischargers { .. }
        );
    }
    matches!(
        &retired_bi1.continuation.outcome,
        BranchContinuationOutcome::HaltedMultipleGuardedDischargers { .. }
    )
}

fn final_stop_precondition(
    retired_bi1: &Bi1BranchCertificateV3,
    bi1b: &Bi1bBranchCertificateV1,
) -> bool {
    if let Some(resume) = &bi1b.resume {
        return matches!(
            &resume.outcome,
            Bi1bResumeOutcomeV1::ExpressivityGap { .. }
                | Bi1bResumeOutcomeV1::ResourceLimitReached { .. }
        ) || resume
            .stages
            .iter()
            .any(|stage| !stage.every_candidate_classified_before_census);
    }
    matches!(
        &retired_bi1.continuation.outcome,
        BranchContinuationOutcome::ExpressivityGap { .. }
            | BranchContinuationOutcome::ResourceLimitReached { .. }
    ) || retired_bi1
        .guarded_stages
        .iter()
        .any(|stage| !stage.candidate_semantic_gaps_absent)
}

fn g2_row_digest(row: &Bi4G2ProfileRowV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("g2-profile-row", &projection)
}

fn ledger_row_digest(row: &Bi4LedgerRowV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("ledger-row", &projection)
}

fn bar_row_digest(row: &Bi4BarRowV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("bar-row", &projection)
}

fn measurement_digest(row: &Bi4BranchMeasurementV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("branch-measurement", &projection)
}

fn audit_digest(row: &Bi4GranularityAuditV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("granularity-audit", &projection)
}

fn disposition_digest(row: &Bi4DispositionRowV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("branch-index-disposition", &projection)
}

fn drift_digest(row: &Bi4DriftDisclosureV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("drift-disclosure", &projection)
}

fn correspondence_projection_digest(row: &Bi4CorrespondenceCorroborationV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("correspondence-corroboration", &projection)
}

fn inherited_g1_digest(row: &Bi4InheritedG1TestimonyV2) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    row_hash("inherited-g1-testimony", &projection)
}

fn certificate_digest(certificate: &Bi4ConeAssemblyV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    row_hash("cone-certificate", &projection)
}

fn build_measurement(
    branch: &Bi2BranchFinaleCertificateV1,
    retired_bi1: &Bi1BranchCertificateV3,
    bi1b: &Bi1bBranchCertificateV1,
) -> Result<Bi4BranchMeasurementV2, Bi4ConeAssemblyV2Error> {
    let steps = branch
        .finale_input
        .steps
        .iter()
        .map(|row| (row.stage, row))
        .collect::<BTreeMap<_, _>>();
    let provenance = branch
        .step_provenance
        .iter()
        .map(|row| (row.stage, row))
        .collect::<BTreeMap<_, _>>();
    let windows = branch
        .finale
        .windows
        .iter()
        .map(|row| (row.stage, row))
        .collect::<BTreeMap<_, _>>();
    if steps.len() != 15
        || provenance.len() != 15
        || (1..=15).any(|stage| !steps.contains_key(&stage) || !provenance.contains_key(&stage))
        || (5..=16).any(|stage| !windows.contains_key(&stage))
        || retired_bi1.branch_root_hash != branch.branch_root_hash
        || retired_bi1.result_digest != branch.sealed_bi1_branch_certificate_digest
        || bi1b.branch_root_hash != branch.branch_root_hash
        || bi1b.sealed_bi1_certificate_digest != retired_bi1.result_digest
        || bi1b.result_digest != branch.bi1b_branch_certificate_digest
    {
        return Err(Bi4ConeAssemblyV2Error::Invariant(format!(
            "branch {} lacks a contiguous 1-15 ledger, 5-16 G2 window, or exact retired BI-1 join",
            branch.branch_root_hash
        )));
    }

    let mut complete_ledger = Vec::with_capacity(15);
    for stage in 1..=15 {
        let step = steps[&stage];
        let proof = provenance[&stage];
        let provenance_replayed = proof.stage == stage
            && proof.candidate_hash == step.candidate_hash
            && proof.semantic_nu == step.certified_nu
            && proof.exact_branch_ledger_join
            && proof.exact_prefix_candidate_package_binding
            && proof.all_residual_counts_zero
            && proof.package_proved
            && proof.no_archive_structural_bar_verdict_or_future_input
            && step.ordinary_charge_provenance_authoritative;
        let mut row = Bi4LedgerRowV2 {
            stage,
            candidate_hash: step.candidate_hash.clone(),
            kappa: step.kappa,
            semantic_nu: step.certified_nu,
            provenance_replayed,
            derivation_hash: String::new(),
        };
        row.derivation_hash = ledger_row_digest(&row);
        complete_ledger.push(row);
    }

    let mut g2_profile = Vec::with_capacity(12);
    for stage in 5..=15 {
        let window = windows[&stage];
        let step = steps[&stage];
        let proof = provenance[&stage];
        let original_total_discharge =
            original_total_discharge_witness(retired_bi1, bi1b, stage, &step.candidate_hash);
        let exact_join = window.stage == stage
            && window.relative_inventory_exhaustive
            && window.focus_projection_matches_branch_ledger
            && window.required_packages == step.required_packages_before_selection
            && window.required_packages == window.structural_constructors
            && window.required_packages == window.expected_branch_demand
            && window.required_packages.len() == 1
            && proof.exact_branch_ledger_join
            && proof.package_proved
            && step.ordinary_charge_provenance_authoritative
            && original_total_discharge;
        let mut row = Bi4G2ProfileRowV2 {
            stage,
            required_packages: window.required_packages.clone(),
            structural_constructors: window.structural_constructors.clone(),
            live_obligation_count: window.expected_branch_demand.len(),
            total_typed_discharge: exact_join,
            debt_free: false,
            exact_bi2_window_step_join: exact_join,
            derivation_hash: String::new(),
        };
        row.derivation_hash = g2_row_digest(&row);
        g2_profile.push(row);
    }
    let terminal = windows[&16];
    let terminal_exact = terminal.stage == 16
        && terminal.relative_inventory_exhaustive
        && terminal.focus_projection_matches_branch_ledger
        && terminal.required_packages.is_empty()
        && terminal.structural_constructors.is_empty()
        && terminal.expected_branch_demand.is_empty()
        && branch.finale_input.terminal_required_packages.is_empty()
        && branch.semantic_o16_empty_branch_indexed
        && branch.finale.semantic_successor_o_empty
        && final_debt_free_halt_witness(retired_bi1, bi1b);
    let mut terminal_row = Bi4G2ProfileRowV2 {
        stage: 16,
        required_packages: terminal.required_packages.clone(),
        structural_constructors: terminal.structural_constructors.clone(),
        live_obligation_count: terminal.expected_branch_demand.len(),
        total_typed_discharge: false,
        debt_free: terminal_exact,
        exact_bi2_window_step_join: terminal_exact,
        derivation_hash: String::new(),
    };
    terminal_row.derivation_hash = g2_row_digest(&terminal_row);
    g2_profile.push(terminal_row.clone());

    let history = complete_ledger
        .iter()
        .map(|row| DiscoveryRecord::new(row.stage, row.semantic_nu, row.kappa))
        .collect::<Vec<_>>();
    let mut bars = Vec::with_capacity(11);
    for stage in 5..=15 {
        let computation = compute_bar(2, stage, &history);
        let mut row = Bi4BarRowV2 {
            stage,
            phi: computation.phi.to_string(),
            omega: computation.omega.to_string(),
            bar: computation.bar.to_string(),
            complete_prior_history_used: history
                .iter()
                .take((stage - 1) as usize)
                .map(|record| record.step_index)
                .eq(1..stage),
            derivation_hash: String::new(),
        };
        row.derivation_hash = bar_row_digest(&row);
        bars.push(row);
    }

    let complete_sum_kappa = complete_ledger.iter().map(|row| row.kappa).sum();
    let complete_sum_semantic_nu = complete_ledger.iter().map(|row| row.semantic_nu).sum();
    let suffix_sum_semantic_nu_stage5_through15 = complete_ledger
        .iter()
        .filter(|row| row.stage >= 5)
        .map(|row| row.semantic_nu)
        .sum();
    let g3a_ledger_stage5_through15 = complete_ledger
        .iter()
        .filter(|row| row.stage >= 5)
        .cloned()
        .collect::<Vec<_>>();
    let every_ledger_row_provenance_replayed =
        complete_ledger.iter().all(|row| row.provenance_replayed)
            && branch.prefix_general_sequence.deterministic_replay_valid
            && branch
                .prefix_general_sequence
                .every_package_closed_observed_grammar
            && branch
                .prefix_general_sequence
                .every_package_registry_extension_invariant
            && branch
                .prefix_general_sequence
                .no_historical_registry_or_future_input
            && branch.prefix_general_sequence.t_bi_b1_proved_on_sequence
            && branch.prefix_general_sequence.t_bi_b2_proved_on_sequence
            && branch.prefix_general_sequence.one_prefix_general_procedure;
    let g2_original_unique_winner_total_discharge_predicate_replayed = (5..=15).all(|stage| {
        let step = steps[&stage];
        original_total_discharge_witness(retired_bi1, bi1b, stage, &step.candidate_hash)
    });
    let halt_boundary_premise_hash = tagged_hash(
        "halt-boundary-premise",
        &(
            terminal_row.required_packages.clone(),
            terminal_row.structural_constructors.clone(),
            terminal_row.live_obligation_count,
            terminal_row.debt_free,
            branch.finale.halt_step,
            branch.finale.successor_stage,
            branch.final_a3_inventory_count,
            branch.finale.inventory_scope.clone(),
        ),
    );
    let mut measurement = Bi4BranchMeasurementV2 {
        branch_root_hash: branch.branch_root_hash.clone(),
        branch_certificate_digest: branch.result_digest.clone(),
        retired_bi1_branch_certificate_digest: retired_bi1.result_digest.clone(),
        bi1b_branch_certificate_digest: bi1b.result_digest.clone(),
        enacted_root: branch.enacted_root,
        g2_profile_stage5_through16: g2_profile,
        complete_ledger_stage1_through15: complete_ledger,
        g3a_ledger_stage5_through15,
        complete_sum_kappa,
        complete_sum_semantic_nu,
        suffix_sum_semantic_nu_stage5_through15,
        diagnostic_bar_stage5_through15: bars,
        g2_original_unique_winner_total_discharge_predicate_replayed,
        every_ledger_row_provenance_replayed,
        local_g4_debt_free_halt_at_15: branch.local_g4_debt_free_halt_at_15,
        halt_step: branch.finale.halt_step,
        successor_stage: branch.finale.successor_stage,
        semantic_o16_empty: branch.semantic_o16_empty_branch_indexed,
        f1_executed: branch.f1_executed,
        f1_triggered: branch.f1_triggered,
        f1_excluded: branch.f1_excluded,
        exact_d_partition: branch.finale.exact_d_partition,
        theorem12_full_instance_granularity: branch
            .theorem12_full_instance_granularity_branch_indexed,
        e5_complete: branch.e5_class_complete,
        expressivity_gap_count: branch.expressivity_gaps.len(),
        halt_boundary_premise_hash,
        derivation_hash: String::new(),
    };
    measurement.derivation_hash = measurement_digest(&measurement);
    Ok(measurement)
}

fn pair(
    domain: &str,
    left: &Bi4BranchMeasurementV2,
    right: &Bi4BranchMeasurementV2,
    equal: bool,
    first_divergence_stage: Option<u32>,
    exact_context: String,
) -> Bi4PairComparisonV2 {
    let mut row = Bi4PairComparisonV2 {
        left_branch_root: left.branch_root_hash.clone(),
        right_branch_root: right.branch_root_hash.clone(),
        equal,
        first_divergence_stage,
        exact_context,
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash(domain, &{
        let mut projection = row.clone();
        projection.derivation_hash.clear();
        projection
    });
    row
}

fn compare_g2(
    left: &Bi4BranchMeasurementV2,
    right: &Bi4BranchMeasurementV2,
) -> Bi4PairComparisonV2 {
    let first = left
        .g2_profile_stage5_through16
        .iter()
        .zip(&right.g2_profile_stage5_through16)
        .find(|(a, b)| {
            a.stage != b.stage
                || a.required_packages != b.required_packages
                || a.structural_constructors != b.structural_constructors
                || a.live_obligation_count != b.live_obligation_count
                || a.total_typed_discharge != b.total_typed_discharge
                || a.debt_free != b.debt_free
        });
    let same_len =
        left.g2_profile_stage5_through16.len() == right.g2_profile_stage5_through16.len();
    let equal = same_len && first.is_none();
    let (stage, context) = if let Some((a, b)) = first {
        (
            Some(a.stage.min(b.stage)),
            format!(
                "G2 Stage {}: left packages={:?}, constructors={:?}, live={}, total={}, debt_free={}; right packages={:?}, constructors={:?}, live={}, total={}, debt_free={}",
                a.stage,
                a.required_packages,
                a.structural_constructors,
                a.live_obligation_count,
                a.total_typed_discharge,
                a.debt_free,
                b.required_packages,
                b.structural_constructors,
                b.live_obligation_count,
                b.total_typed_discharge,
                b.debt_free
            ),
        )
    } else if !same_len {
        (
            None,
            format!(
                "G2 profile length differs: left={}, right={}",
                left.g2_profile_stage5_through16.len(),
                right.g2_profile_stage5_through16.len()
            ),
        )
    } else {
        (
            None,
            "G2 exact obligation profiles agree at stages 5-15 and the debt-free successor 16"
                .to_owned(),
        )
    };
    pair("g2-pair", left, right, equal, stage, context)
}

fn compare_g3a(
    left: &Bi4BranchMeasurementV2,
    right: &Bi4BranchMeasurementV2,
) -> Bi4PairComparisonV2 {
    let first = left
        .g3a_ledger_stage5_through15
        .iter()
        .zip(&right.g3a_ledger_stage5_through15)
        .find(|(a, b)| a.stage != b.stage || a.kappa != b.kappa || a.semantic_nu != b.semantic_nu);
    let same_len =
        left.g3a_ledger_stage5_through15.len() == right.g3a_ledger_stage5_through15.len();
    let equal = same_len && first.is_none();
    let (stage, context) = if let Some((a, b)) = first {
        (
            Some(a.stage.min(b.stage)),
            format!(
                "G3a Stage {}: left (kappa,semantic_nu)=({},{}); right=({},{})",
                a.stage, a.kappa, a.semantic_nu, b.kappa, b.semantic_nu
            ),
        )
    } else if !same_len {
        (
            None,
            format!(
                "G3a ledger length differs: left={}, right={}",
                left.g3a_ledger_stage5_through15.len(),
                right.g3a_ledger_stage5_through15.len()
            ),
        )
    } else {
        (
            None,
            "G3a exact-certified (kappa,semantic_nu) vectors agree at every stage 5-15; winner identities are excluded"
                .to_owned(),
        )
    };
    pair("g3a-pair", left, right, equal, stage, context)
}

fn compare_g3b(
    left: &Bi4BranchMeasurementV2,
    right: &Bi4BranchMeasurementV2,
) -> Bi4PairComparisonV2 {
    let equal = left.complete_sum_semantic_nu == right.complete_sum_semantic_nu;
    let first_source_difference = left
        .complete_ledger_stage1_through15
        .iter()
        .zip(&right.complete_ledger_stage1_through15)
        .find(|(a, b)| a.stage != b.stage || a.semantic_nu != b.semantic_nu);
    let stage = (!equal)
        .then(|| first_source_difference.map(|(a, b)| a.stage.min(b.stage)))
        .flatten();
    let context = if equal {
        format!(
            "G3b complete Stage 1-15 semantic Sigma-nu agrees at {}",
            left.complete_sum_semantic_nu
        )
    } else if let Some((a, b)) = first_source_difference {
        format!(
            "G3b complete Stage 1-15 semantic Sigma-nu differs: left={}, right={}; the first source-row difference is Stage {} with left semantic_nu={} and right semantic_nu={}",
            left.complete_sum_semantic_nu,
            right.complete_sum_semantic_nu,
            a.stage.min(b.stage),
            a.semantic_nu,
            b.semantic_nu,
        )
    } else {
        format!(
            "G3b complete Stage 1-15 semantic Sigma-nu differs: left={}, right={}; no differing source row was available, so the aggregate is internally inconsistent",
            left.complete_sum_semantic_nu, right.complete_sum_semantic_nu
        )
    };
    pair("g3b-pair", left, right, equal, stage, context)
}

fn compare_g3c(
    left: &Bi4BranchMeasurementV2,
    right: &Bi4BranchMeasurementV2,
) -> Bi4PairComparisonV2 {
    let first = left
        .diagnostic_bar_stage5_through15
        .iter()
        .zip(&right.diagnostic_bar_stage5_through15)
        .find(|(a, b)| a.stage != b.stage || a.bar != b.bar);
    let same_len =
        left.diagnostic_bar_stage5_through15.len() == right.diagnostic_bar_stage5_through15.len();
    let equal = same_len && first.is_none();
    let (stage, context) = if let Some((a, b)) = first {
        (
            Some(a.stage.min(b.stage)),
            format!(
                "G3c diagnostic bar first differs at Stage {}: left={} (phi={},omega={}); right={} (phi={},omega={})",
                a.stage, a.bar, a.phi, a.omega, b.bar, b.phi, b.omega
            ),
        )
    } else if !same_len {
        (
            None,
            format!(
                "G3c bar trajectory length differs: left={}, right={}",
                left.diagnostic_bar_stage5_through15.len(),
                right.diagnostic_bar_stage5_through15.len()
            ),
        )
    } else {
        (
            None,
            "G3c exact recomputed descriptive bar trajectories agree at stages 5-15".to_owned(),
        )
    };
    pair("g3c-pair", left, right, equal, stage, context)
}

fn all_pairs(
    rows: &[Bi4BranchMeasurementV2],
    compare: fn(&Bi4BranchMeasurementV2, &Bi4BranchMeasurementV2) -> Bi4PairComparisonV2,
) -> Vec<Bi4PairComparisonV2> {
    let mut result = Vec::with_capacity(6);
    for left in 0..rows.len() {
        for right in (left + 1)..rows.len() {
            result.push(compare(&rows[left], &rows[right]));
        }
    }
    result
}

fn granularity(level: &str, passed: bool, basis: impl Into<String>) -> Bi4GranularityAuditV2 {
    let mut row = Bi4GranularityAuditV2 {
        level: level.to_owned(),
        verdict: if passed {
            Bi4VerdictV2::Passed
        } else {
            Bi4VerdictV2::Refuted
        },
        passed,
        exact_basis: basis.into(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = audit_digest(&row);
    row
}

fn stage4_offset_only(
    branches: &[Bi4BranchMeasurementV2],
    g3a_passed: bool,
    g3b_passed: bool,
    g3c_passed: bool,
) -> bool {
    if !g3a_passed || g3b_passed || g3c_passed || branches.len() != 4 {
        return false;
    }
    let every_pair_differs_only_in_stage4_nu = (0..branches.len()).all(|left| {
        ((left + 1)..branches.len()).all(|right| {
            branches[left]
                .complete_ledger_stage1_through15
                .iter()
                .zip(&branches[right].complete_ledger_stage1_through15)
                .all(|(a, b)| {
                    a.stage == b.stage
                        && a.kappa == b.kappa
                        && (a.semantic_nu == b.semantic_nu || a.stage == 4)
                })
        })
    });
    let at_least_one_stage4_nu_difference = branches
        .iter()
        .map(|branch| branch.complete_ledger_stage1_through15[3].semantic_nu)
        .collect::<BTreeSet<_>>()
        .len()
        > 1;
    let sum_offsets_exact = (0..branches.len()).all(|left| {
        ((left + 1)..branches.len()).all(|right| {
            i64::from(branches[left].complete_sum_semantic_nu)
                - i64::from(branches[right].complete_sum_semantic_nu)
                == i64::from(branches[left].complete_ledger_stage1_through15[3].semantic_nu)
                    - i64::from(branches[right].complete_ledger_stage1_through15[3].semantic_nu)
        })
    });
    let normalized_bar_trajectories = branches
        .iter()
        .map(|branch| {
            let history = branch
                .complete_ledger_stage1_through15
                .iter()
                .map(|row| {
                    DiscoveryRecord::new(
                        row.stage,
                        if row.stage == 4 { 0 } else { row.semantic_nu },
                        row.kappa,
                    )
                })
                .collect::<Vec<_>>();
            (5..=15)
                .map(|stage| compute_bar(2, stage, &history).bar.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<BTreeSet<_>>();
    every_pair_differs_only_in_stage4_nu
        && at_least_one_stage4_nu_difference
        && sum_offsets_exact
        && normalized_bar_trajectories.len() == 1
}

fn disposition_row(
    claim: &str,
    status: Bi4DispositionStatusV2,
    premise: &str,
) -> Bi4DispositionRowV2 {
    let mut row = Bi4DispositionRowV2 {
        claim: claim.to_owned(),
        status,
        exact_verdict_premise: premise.to_owned(),
        derivation_hash: String::new(),
    };
    row.derivation_hash = disposition_digest(&row);
    row
}

fn inherited_g1_testimony() -> Result<Bi4InheritedG1TestimonyV2, Bi4ConeAssemblyV2Error> {
    let certificate = serde_json::from_slice::<Rt2FutureHoleConfluenceV2Certificate>(R_T2_BYTES)
        .map_err(|error| Bi4ConeAssemblyV2Error::Json(error.to_string()))?;
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let artifact_self_digest_valid = certificate.schema == R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA
        && certificate.result_digest
            == foreign_tagged_hash(
                R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA,
                "certificate",
                &projection,
            );
    let exact_outcome = certificate.outcome
        == Rt2FutureHoleConfluenceV2Outcome::LawLevelConfluenceRefutedRungRt3Opened;
    let pairs = &certificate
        .order_reversal_audit
        .forward_normalized_pair_verdicts;
    let pair_keys = pairs
        .iter()
        .map(|row| {
            let (left, right) = &row.unordered_candidate_pair;
            if left <= right {
                (left.as_str(), right.as_str())
            } else {
                (right.as_str(), left.as_str())
            }
        })
        .collect::<BTreeSet<_>>();
    let exact_six_inequivalent_five_scheme_pairs = pairs.len() == 6
        && pair_keys.len() == 6
        && pairs.iter().all(|row| {
            row.left_scheme_count == 5 && row.right_scheme_count == 5 && !row.equivalent
        });
    let all_stage5_scheme_sets_inequivalent = !certificate.all_stage5_scheme_sets_equivalent
        && certificate
            .pairwise_scheme_set_comparisons
            .iter()
            .all(|row| !row.full_scheme_sets_equivalent && row.comparison_well_formed);
    let order_reversal_invariant = certificate.order_reversal_invariant
        && certificate
            .order_reversal_audit
            .exact_pair_verdict_invariance
        && certificate
            .order_reversal_audit
            .exact_complete_set_invariance
        && certificate.order_reversal_audit.selection_invariant;
    let selector_absent = certificate.selected_candidate_hash.is_none()
        && !certificate.hash_or_enumeration_order_used_as_selector
        && !certificate.desired_history_count_score_or_bar_used_as_premise;
    if !artifact_self_digest_valid
        || !exact_outcome
        || !certificate.r_t2_confluence_refuted
        || !all_stage5_scheme_sets_inequivalent
        || !order_reversal_invariant
        || !selector_absent
        || !exact_six_inequivalent_five_scheme_pairs
    {
        return Err(Bi4ConeAssemblyV2Error::Input(
            "inherited R-T2 G1 testimony failed its typed self-digest, outcome, order, selector, or exact six-pair audit"
                .to_owned(),
        ));
    }
    let mut row = Bi4InheritedG1TestimonyV2 {
        artifact_blake3: bytes_hash(R_T2_BYTES),
        certificate_digest: certificate.result_digest,
        outcome: "law_level_confluence_refuted_rung_rt3_opened".to_owned(),
        artifact_self_digest_valid,
        r_t2_confluence_refuted: certificate.r_t2_confluence_refuted,
        all_stage5_scheme_sets_inequivalent,
        order_reversal_invariant,
        selector_absent,
        exact_six_inequivalent_five_scheme_pairs,
        re_litigated_by_bi4_v2: false,
        used_to_compute_g2_g3_or_g4: false,
        derivation_hash: String::new(),
    };
    row.derivation_hash = inherited_g1_digest(&row);
    Ok(row)
}

fn bi1b_tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI1B_PREFIX_GENERAL_SWEEP_V1_SCHEMA, domain, value))
        .expect("BI-1b evidence serializes");
    bytes_hash(&bytes)
}

fn correspondence_row_digest(
    row: &crate::bi1b_prefix_general_sweep_v1::Bi1bCorrespondenceRowV1,
) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    bi1b_tagged_hash("correspondence-row", &projection)
}

fn correspondence_digest(row: &Bi1bCorrespondenceDiagnosticV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    bi1b_tagged_hash(BI1B_CORRESPONDENCE_V1_SCHEMA, &projection)
}

fn open_correspondence_after_verdicts(
    branches: &[Bi4BranchMeasurementV2],
) -> Result<Bi4CorrespondenceCorroborationV2, Bi4ConeAssemblyV2Error> {
    let diagnostic = serde_json::from_slice::<Bi1bCorrespondenceDiagnosticV1>(CORRESPONDENCE_BYTES)
        .map_err(|error| Bi4ConeAssemblyV2Error::Json(error.to_string()))?;
    let artifact_self_digest_valid = diagnostic.derivation_hash
        == correspondence_digest(&diagnostic)
        && diagnostic
            .rows
            .iter()
            .all(|row| row.derivation_hash == correspondence_row_digest(row));
    let enacted = branches
        .iter()
        .find(|branch| branch.enacted_root)
        .ok_or_else(|| Bi4ConeAssemblyV2Error::Invariant("no enacted index row".to_owned()))?;
    let alternates = branches
        .iter()
        .filter(|branch| !branch.enacted_root)
        .map(|branch| branch.branch_root_hash.as_str())
        .collect::<BTreeSet<_>>();
    let diagnostic_alternates = diagnostic
        .rows
        .iter()
        .map(|row| row.alternate_branch_root_hash.as_str())
        .collect::<BTreeSet<_>>();
    let exact_three_alternate_by_eight_stage_surface = diagnostic.rows.len() == 24
        && diagnostic_alternates == alternates
        && alternates.iter().all(|root| {
            diagnostic
                .rows
                .iter()
                .filter(|row| row.alternate_branch_root_hash == **root)
                .map(|row| row.stage)
                .eq(8..=15)
        });
    let every_stage8_through15_winner_byte_identical = diagnostic.rows.iter().all(|row| {
        row.winners_byte_identical
            && row.enacted_winner_hash == row.alternate_winner_hash
            && row.stage4_only_prefix_shadow_for_byte_identical_candidates
            && !row.correspondence_used_as_provenance
            && !row.correspondence_used_as_selector
    });
    let every_winner_joins_sealed_bi2_ledger = diagnostic.rows.iter().all(|row| {
        let alternate = branches
            .iter()
            .find(|branch| branch.branch_root_hash == row.alternate_branch_root_hash);
        let enacted_winner = enacted
            .complete_ledger_stage1_through15
            .iter()
            .find(|ledger| ledger.stage == row.stage)
            .map(|ledger| ledger.candidate_hash.as_str());
        let alternate_winner = alternate.and_then(|branch| {
            branch
                .complete_ledger_stage1_through15
                .iter()
                .find(|ledger| ledger.stage == row.stage)
                .map(|ledger| ledger.candidate_hash.as_str())
        });
        row.enacted_winner_hash.as_deref() == enacted_winner
            && row.alternate_winner_hash.as_deref() == alternate_winner
    });
    let mut result = Bi4CorrespondenceCorroborationV2 {
        opened_after_all_comparison_verdicts: true,
        artifact_blake3: bytes_hash(CORRESPONDENCE_BYTES),
        artifact_self_digest_valid,
        row_count: diagnostic.rows.len(),
        exact_three_alternate_by_eight_stage_surface,
        every_stage8_through15_winner_byte_identical,
        every_winner_joins_sealed_bi2_ledger,
        used_by_any_comparison_verdict: false,
        used_by_selector: diagnostic.used_by_branch_selector,
        used_as_provenance: diagnostic.used_by_provenance_issuer,
        corroboration_only: diagnostic.byte_identity_is_zero_charge_diagnostic_only
            && !diagnostic.used_by_branch_selector
            && !diagnostic.used_by_provenance_issuer,
        derivation_hash: String::new(),
    };
    result.derivation_hash = correspondence_projection_digest(&result);
    if !artifact_self_digest_valid
        || !exact_three_alternate_by_eight_stage_surface
        || !every_stage8_through15_winner_byte_identical
        || !every_winner_joins_sealed_bi2_ledger
        || !result.corroboration_only
    {
        return Err(Bi4ConeAssemblyV2Error::Invariant(
            "post-verdict correspondence corroboration failed exact replay".to_owned(),
        ));
    }
    Ok(result)
}

fn assemble_after_bi2_replay(
    bundle: &Bi2FourBranchFinalesBundleV1,
) -> Result<Bi4ConeAssemblyV2Certificate, Bi4ConeAssemblyV2Error> {
    if bundle.index.result_digest != FROZEN_BI2_V2_INDEX_DIGEST {
        return Err(Bi4ConeAssemblyV2Error::Input(format!(
            "BI-2 v2 aggregate digest {} does not match frozen plan digest {}",
            bundle.index.result_digest, FROZEN_BI2_V2_INDEX_DIGEST
        )));
    }
    let retired_bi1_branches = embedded_retired_bi1_branches()?;
    let bi1b_branches = embedded_bi1b_branches()?;
    let mut branches = bundle
        .branches
        .iter()
        .map(|branch| {
            retired_bi1_branches
                .get(&branch.branch_root_hash)
                .ok_or_else(|| {
                    Bi4ConeAssemblyV2Error::Invariant(format!(
                        "branch {} has no retained BI-1 testimony",
                        branch.branch_root_hash
                    ))
                })
                .and_then(|retired| {
                    bi1b_branches
                        .get(&branch.branch_root_hash)
                        .ok_or_else(|| {
                            Bi4ConeAssemblyV2Error::Invariant(format!(
                                "branch {} has no retained BI-1b continuation testimony",
                                branch.branch_root_hash
                            ))
                        })
                        .and_then(|bi1b| build_measurement(branch, retired, bi1b))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    branches.sort_by(|left, right| left.branch_root_hash.cmp(&right.branch_root_hash));
    let roots = branches
        .iter()
        .map(|branch| branch.branch_root_hash.as_str())
        .collect::<BTreeSet<_>>();
    let exact_four_distinct_branch_roots = branches.len() == 4
        && roots.len() == 4
        && branches.iter().filter(|b| b.enacted_root).count() == 1;
    let every_branch_digest_joins_bi2_index = branches.iter().all(|branch| {
        bundle.index.rows.iter().any(|row| {
            row.branch_root_hash == branch.branch_root_hash
                && row.branch_certificate_digest == branch.branch_certificate_digest
        })
    });
    if !exact_four_distinct_branch_roots || !every_branch_digest_joins_bi2_index {
        return Err(Bi4ConeAssemblyV2Error::Invariant(
            "BI-2 branch/index surface does not form one exact four-cone".to_owned(),
        ));
    }

    // F-B4-2: materialize every pair at every comparison before deriving any
    // aggregate verdict. Nothing below may stop after the first divergence.
    let g2_pairs = all_pairs(&branches, compare_g2);
    let g3a_pairs = all_pairs(&branches, compare_g3a);
    let g3b_pairs = all_pairs(&branches, compare_g3b);
    let g3c_pairs = all_pairs(&branches, compare_g3c);

    let g2_passed = g2_pairs.iter().all(|row| row.equal)
        && branches.iter().all(|branch| {
            branch.g2_profile_stage5_through16.len() == 12
                && branch.g2_original_unique_winner_total_discharge_predicate_replayed
                && branch.g2_profile_stage5_through16[..11].iter().all(|row| {
                    row.live_obligation_count == 1
                        && row.total_typed_discharge
                        && !row.debt_free
                        && row.exact_bi2_window_step_join
                })
                && branch.g2_profile_stage5_through16[11].stage == 16
                && branch.g2_profile_stage5_through16[11].live_obligation_count == 0
                && !branch.g2_profile_stage5_through16[11].total_typed_discharge
                && branch.g2_profile_stage5_through16[11].debt_free
                && branch.g2_profile_stage5_through16[11].exact_bi2_window_step_join
        });
    let g3a_passed = g3a_pairs.iter().all(|row| row.equal)
        && branches.iter().all(|branch| {
            branch.every_ledger_row_provenance_replayed
                && branch
                    .g3a_ledger_stage5_through15
                    .iter()
                    .map(|row| row.stage)
                    .eq(5..=15)
        });
    let g3b_passed = g3b_pairs.iter().all(|row| row.equal);
    let g3c_passed = g3c_pairs.iter().all(|row| row.equal)
        && branches.iter().all(|branch| {
            branch
                .diagnostic_bar_stage5_through15
                .iter()
                .all(|row| row.complete_prior_history_used)
        });
    let g3_composite_passed = g3a_passed && g3b_passed && g3c_passed;
    let halt_boundaries = branches
        .iter()
        .map(|branch| branch.halt_boundary_premise_hash.as_str())
        .collect::<BTreeSet<_>>();
    let cone_g4_halt_boundary_premise_branch_index_free = halt_boundaries.len() == 1;
    let cone_g4_passed = cone_g4_halt_boundary_premise_branch_index_free
        && branches.iter().all(|branch| {
            branch.local_g4_debt_free_halt_at_15
                && branch.halt_step == 15
                && branch.successor_stage == 16
                && branch.semantic_o16_empty
                && branch.f1_executed
                && !branch.f1_triggered
                && branch.f1_excluded
                && branch.exact_d_partition
                && branch.theorem12_full_instance_granularity
                && branch.e5_complete
                && branch.expressivity_gap_count == 0
        });

    let g2 = granularity(
        "G2_obligation_profile",
        g2_passed,
        "all six pairwise comparisons of the original Stage 5-15 required-package/constructor/live-count/total-discharge/debt-free rows plus terminal successor 16",
    );
    let g3a = granularity(
        "G3a_stage5_to15_kappa_semantic_nu",
        g3a_passed,
        "all six pairwise comparisons of replayed branch-local (kappa,semantic_nu) rows at stages 5-15; winner identity excluded",
    );
    let g3b = granularity(
        "G3b_complete_sigma_semantic_nu",
        g3b_passed,
        "all six pairwise comparisons of complete Stage 1-15 semantic Sigma-nu; the common Stage 5-15 suffix is not substituted",
    );
    let g3c = granularity(
        "G3c_diagnostic_bar_trajectory",
        g3c_passed,
        "all six pairwise comparisons of exact d=2 bars recomputed from each complete ordered Stage 1-15 semantic ledger",
    );
    let g3_composite = granularity(
        "G3_numeric_ledger_composite",
        g3_composite_passed,
        "the frozen composite passes iff G3a, G3b, and G3c all pass; sub-reporting does not weaken the registered standard",
    );
    let cone_g4 = granularity(
        "cone_G4_debt_free_halt",
        cone_g4_passed,
        "four replayed BI-2 v2 finales with halt=15, successor=16, exact D, semantic O16 empty, F1 executed/excluded, Theorem-12 full-instance proof, E5 complete, zero gaps, and one branch-index-free halt premise",
    );

    let stage4_offset_only_explanation_proved =
        stage4_offset_only(&branches, g3a_passed, g3b_passed, g3c_passed);
    let z_tree_precondition_present = retired_bi1_branches.iter().any(|(root, branch)| {
        bi1b_branches
            .get(root)
            .is_some_and(|bi1b| final_tree_precondition(branch, bi1b))
    });
    let z_stop_precondition_present = retired_bi1_branches.iter().any(|(root, branch)| {
        bi1b_branches
            .get(root)
            .is_none_or(|bi1b| final_stop_precondition(branch, bi1b))
    }) || branches
        .iter()
        .any(|branch| branch.expressivity_gap_count != 0);
    let outcome_zone = if z_tree_precondition_present {
        "Z-TREE"
    } else if z_stop_precondition_present {
        "Z-STOP"
    } else if !cone_g4_passed || !g3_composite_passed {
        "Z-SPLIT"
    } else if !g2_passed {
        "Z-ISO"
    } else {
        "Z-CONE"
    }
    .to_owned();
    let refinement_label = (outcome_zone == "Z-SPLIT" && stage4_offset_only_explanation_proved)
        .then(|| "Z-SPLIT-4".to_owned());

    let comparison_sets = [
        ("G2", &g2_pairs),
        ("G3a", &g3a_pairs),
        ("G3b", &g3b_pairs),
        ("G3c", &g3c_pairs),
    ];
    let first_refuted_pair = comparison_sets
        .iter()
        .flat_map(|(label, rows)| {
            rows.iter()
                .filter(|row| !row.equal)
                .map(move |row| (*label, row))
        })
        .min_by(|(left_label, left), (right_label, right)| {
            (
                left.first_divergence_stage.unwrap_or(u32::MAX),
                *left_label,
                &left.left_branch_root,
                &left.right_branch_root,
            )
                .cmp(&(
                    right.first_divergence_stage.unwrap_or(u32::MAX),
                    *right_label,
                    &right.left_branch_root,
                    &right.right_branch_root,
                ))
        });
    let first_cross = first_refuted_pair
        .map(|(first_label, first)| {
            let matching_context = comparison_sets
                .iter()
                .filter_map(|(label, rows)| {
                    rows.iter()
                        .find(|row| {
                            !row.equal
                                && row.left_branch_root == first.left_branch_root
                                && row.right_branch_root == first.right_branch_root
                        })
                        .map(|row| format!("{label}: {}", row.exact_context))
                })
                .collect::<Vec<_>>()
                .join("; ");
            format!(
                "First new divergence is {first_label} for branch pair {} versus {} at source stage {}: {matching_context}. Composite G3={}; no divergence was averaged, suppressed, or repaired.",
                first.left_branch_root,
                first.right_branch_root,
                first
                    .first_divergence_stage
                    .map(|stage| stage.to_string())
                    .unwrap_or_else(|| "not-applicable".to_owned()),
                if g3_composite_passed {
                    "passed"
                } else {
                    "refuted"
                },
            )
        })
        .or_else(|| {
            (!cone_g4_passed).then(|| {
                let failing_roots = branches
                    .iter()
                    .filter(|branch| {
                        !branch.local_g4_debt_free_halt_at_15
                            || !branch.semantic_o16_empty
                            || !branch.f1_excluded
                            || !branch.e5_complete
                            || branch.expressivity_gap_count != 0
                    })
                    .map(|branch| branch.branch_root_hash.clone())
                    .collect::<Vec<_>>();
                format!(
                    "The first new divergence is cone-G4 failure on branches {failing_roots:?}; no divergence was averaged, suppressed, or repaired."
                )
            })
        });

    let mut disposition = vec![
        disposition_row(
            "G2_obligation_profile_and_one-demand-per-stage_O_ladder_from_Stage5_through_halt",
            if g2_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "G2_pass",
        ),
        disposition_row(
            "G3a_per_stage_kappa_and_semantic_nu_vector_Stage5_through15",
            if g3a_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "G3a_pass",
        ),
        disposition_row(
            "complete_Stage1_through15_Sigma_semantic_nu",
            if g3b_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "G3b_pass",
        ),
        disposition_row(
            "diagnostic_bar_trajectory_and_WB1_cumulative_tables",
            if g3c_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "G3c_pass",
        ),
        disposition_row(
            "G4_debt_free_halt_at_15",
            if cone_g4_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "cone_G4_pass",
        ),
        disposition_row(
            "semantic_O16_empty_at_full_instance_granularity",
            if cone_g4_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "cone_G4_pass",
        ),
        disposition_row(
            "Guard_Rail_F1_excluded",
            if cone_g4_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "cone_G4_pass",
        ),
        disposition_row(
            "Theorem12_full_instance_granularity_relative_to_adopted_A3",
            if cone_g4_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "cone_G4_pass",
        ),
    ];
    disposition.extend([
        disposition_row(
            "Stage4_root_act_identity_and_semantic_nu",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
            "R-T3_free_choice_and_G3b_G3c_refutation",
        ),
        disposition_row(
            "G1_Stage5_successor_scheme_sets_and_exact_derivations",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
            "inherited_R_T2_refutation",
        ),
        disposition_row(
            "branch_specific_candidate_winner_and_act_local_provenance_identities",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
            "G3a_excludes_identity_and_compares_only_kappa_semantic_nu",
        ),
        disposition_row(
            "complete_Stage1_through15_Sigma_kappa",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
            "not_a_registered_G3_sub_verdict",
        ),
        disposition_row(
            "complete_cumulative_Delta_Phi_Omega_interface_trajectories",
            if g3b_passed && g3c_passed {
                Bi4DispositionStatusV2::PromotedToConeLevel
            } else {
                Bi4DispositionStatusV2::RemainsBranchIndexed
            },
            "G3b_and_G3c_pass",
        ),
    ]);

    let drift_authority = &bundle.index.row_free_authorization;
    let mut drift_disclosure = Bi4DriftDisclosureV2 {
        bi1b_public_deterministic_replay_passed: drift_authority.public_deterministic_replay_passed,
        frozen_testimony_fallback_used: drift_authority.frozen_testimony_fallback_used,
        public_replay_errors: drift_authority.public_replay_errors.clone(),
        drift_repaired_inside_assembly: false,
        drift_concealed_inside_assembly: false,
        obsolete_bi4_issuer_retired_not_deleted: true,
        derivation_hash: String::new(),
    };
    drift_disclosure.derivation_hash = drift_digest(&drift_disclosure);
    let inherited_g1_testimony = inherited_g1_testimony()?;

    // The write-only correspondence diagnostic is intentionally parsed only
    // here: every G2/G3/G4 verdict, zone, refinement, divergence, and
    // disposition row already exists and is independent of it.
    let correspondence_corroboration = open_correspondence_after_verdicts(&branches)?;

    let expected_disposition = BTreeMap::from([
        (
            "G2_obligation_profile_and_one-demand-per-stage_O_ladder_from_Stage5_through_halt",
            (
                if g2_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "G2_pass",
            ),
        ),
        (
            "G3a_per_stage_kappa_and_semantic_nu_vector_Stage5_through15",
            (
                if g3a_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "G3a_pass",
            ),
        ),
        (
            "complete_Stage1_through15_Sigma_semantic_nu",
            (
                if g3b_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "G3b_pass",
            ),
        ),
        (
            "diagnostic_bar_trajectory_and_WB1_cumulative_tables",
            (
                if g3c_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "G3c_pass",
            ),
        ),
        (
            "G4_debt_free_halt_at_15",
            (
                if cone_g4_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "cone_G4_pass",
            ),
        ),
        (
            "semantic_O16_empty_at_full_instance_granularity",
            (
                if cone_g4_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "cone_G4_pass",
            ),
        ),
        (
            "Guard_Rail_F1_excluded",
            (
                if cone_g4_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "cone_G4_pass",
            ),
        ),
        (
            "Theorem12_full_instance_granularity_relative_to_adopted_A3",
            (
                if cone_g4_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "cone_G4_pass",
            ),
        ),
        (
            "Stage4_root_act_identity_and_semantic_nu",
            (
                Bi4DispositionStatusV2::RemainsBranchIndexed,
                "R-T3_free_choice_and_G3b_G3c_refutation",
            ),
        ),
        (
            "G1_Stage5_successor_scheme_sets_and_exact_derivations",
            (
                Bi4DispositionStatusV2::RemainsBranchIndexed,
                "inherited_R_T2_refutation",
            ),
        ),
        (
            "branch_specific_candidate_winner_and_act_local_provenance_identities",
            (
                Bi4DispositionStatusV2::RemainsBranchIndexed,
                "G3a_excludes_identity_and_compares_only_kappa_semantic_nu",
            ),
        ),
        (
            "complete_Stage1_through15_Sigma_kappa",
            (
                Bi4DispositionStatusV2::RemainsBranchIndexed,
                "not_a_registered_G3_sub_verdict",
            ),
        ),
        (
            "complete_cumulative_Delta_Phi_Omega_interface_trajectories",
            (
                if g3b_passed && g3c_passed {
                    Bi4DispositionStatusV2::PromotedToConeLevel
                } else {
                    Bi4DispositionStatusV2::RemainsBranchIndexed
                },
                "G3b_and_G3c_pass",
            ),
        ),
    ]);
    let every_claim_disposed_explicitly = disposition.len() == expected_disposition.len()
        && disposition
            .iter()
            .map(|row| row.claim.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            == expected_disposition.len()
        && disposition.iter().all(|row| {
            expected_disposition
                .get(row.claim.as_str())
                .is_some_and(|(status, premise)| {
                    &row.status == status && row.exact_verdict_premise == *premise
                })
        });
    let all_six_pairs = [&g2_pairs, &g3a_pairs, &g3b_pairs, &g3c_pairs]
        .iter()
        .all(|rows| rows.len() == 6);
    if !every_claim_disposed_explicitly || !all_six_pairs {
        return Err(Bi4ConeAssemblyV2Error::Invariant(
            "F-R3-B1 exact disposition or six-pair publication surface is incomplete".to_owned(),
        ));
    }
    let refinement_text = refinement_label
        .as_deref()
        .map(|label| format!(" with refinement {label}"))
        .unwrap_or_default();
    let verdict_text = |passed: bool| if passed { "passed" } else { "refuted" };
    let permitted_conclusion = format!(
        "BI-4 v2 assigns {outcome_zone}{refinement_text} from the computed comparisons: G2={}, G3a={}, G3b={}, G3c={}, composite-G3={}, and cone-G4={}. Cone-level promotion and retained branch indices are exactly those in the disposition table. No UC-1, bridge, or final-certificate conclusion is issued.",
        verdict_text(g2_passed),
        verdict_text(g3a_passed),
        verdict_text(g3b_passed),
        verdict_text(g3c_passed),
        verdict_text(g3_composite_passed),
        verdict_text(cone_g4_passed),
    );

    let mut certificate = Bi4ConeAssemblyV2Certificate {
        schema: BI4_CONE_ASSEMBLY_V2_SCHEMA.to_owned(),
        date: BI4_CONE_ASSEMBLY_V2_DATE.to_owned(),
        source_bindings: source_bindings(),
        bi2_index_digest: bundle.index.result_digest.clone(),
        bi2_index_schema: bundle.index.schema.clone(),
        bi2_index_digest_matches_frozen_plan: true,
        bi1b_sweep_digest: drift_authority.bi1b_sweep_digest.clone(),
        bi1b_regression_certificate_digest: drift_authority
            .bi1b_regression_certificate_digest
            .clone(),
        prefix_general_extraction_authority_digest: drift_authority
            .prefix_general_extraction_authority_digest
            .clone(),
        bi2_bundle_replay_valid_before_comparison: true,
        bi2_bundle_replay_errors: Vec::new(),
        upstream_lineage_authenticated_by_bi2_replay: drift_authority
            .exact_four_branch_surface_authenticated
            && drift_authority.every_bi1b_branch_certificate_replayed
            && drift_authority.frozen_bi1b_sweep_digest_exact
            && !drift_authority.branch_family_rows_exported
            && !drift_authority.branch_semantic_vectors_exported
            && !drift_authority.branch_winners_exported
            && !drift_authority.branch_outcomes_exported
            && !drift_authority.correspondence_rows_exported
            && !drift_authority.selector_capability_exported,
        exact_four_distinct_branch_roots,
        every_branch_digest_joins_bi2_index,
        every_branch_certificate_reissued_before_comparison: bundle
            .index
            .every_branch_certificate_reissued_equal,
        bi2_ready_for_bi4_assembly: bundle.index.ready_for_bi4_assembly,
        comparison_semantics_changed_from_bi1: false,
        granularity_definitions_changed_from_bi1: false,
        zone_taxonomy_changed_from_bi1: false,
        all_comparisons_computed_before_any_aggregate_verdict: true,
        all_six_pairs_published_for_each_comparison: all_six_pairs,
        branches,
        g2_pair_comparisons: g2_pairs,
        g3a_pair_comparisons: g3a_pairs,
        g3b_pair_comparisons: g3b_pairs,
        g3c_pair_comparisons: g3c_pairs,
        g2,
        g3a,
        g3b,
        g3c,
        g3_composite,
        cone_g4,
        cone_g4_halt_boundary_premise_branch_index_free,
        stage4_offset_only_explanation_proved,
        z_tree_precondition_present,
        z_stop_precondition_present,
        outcome_zone,
        refinement_label,
        first_new_divergence_published_verbatim: first_cross,
        inherited_g1_testimony,
        branch_index_disposition: disposition,
        every_claim_disposed_explicitly,
        correspondence_corroboration,
        drift_disclosure,
        no_divergence_suppressed_averaged_or_repaired: true,
        class_representative_substitution_used: false,
        enacted_branch_used_as_comparison_baseline: false,
        uc1_content_present: false,
        bridge_executed: false,
        final_certificate_issued: false,
        mutation_falsifiers: vec![
            "replace_the_frozen_BI2_v2_aggregate_digest_then_issuance_must_fail".to_owned(),
            "flip_any_G2_G3a_G3b_G3c_composite_or_cone_G4_verdict_then_replay_must_fail".to_owned(),
            "change_any_original_BI1_or_BI1b_unique_discharger_or_winner_join_then_G2_must_fail"
                .to_owned(),
            "change_any_stage_kappa_semantic_nu_sum_or_bar_then_replay_must_fail".to_owned(),
            "change_nu_outside_Stage4_then_Z_SPLIT_4_must_disappear".to_owned(),
            "omit_Stage4_from_complete_sum_or_bar_history_then_replay_must_fail".to_owned(),
            "shuffle_duplicate_or_omit_any_history_stage_then_replay_must_fail".to_owned(),
            "drop_or_promote_any_disposition_without_its_exact_verdict_then_replay_must_fail"
                .to_owned(),
            "use_correspondence_as_verdict_provenance_or_selector_then_replay_must_fail".to_owned(),
            "conceal_or_repair_the_frozen_BI1b_drift_then_replay_must_fail".to_owned(),
            "change_zone_or_refinement_label_then_replay_must_fail".to_owned(),
            "add_UC1_bridge_or_final_certificate_content_then_replay_must_fail".to_owned(),
        ],
        permitted_conclusion,
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

/// Issue BI-4 v2 only after the exact emitted BI-2 v2 bundle replays.
pub fn issue_bi4_cone_assembly_v2() -> Result<Bi4ConeAssemblyV2Certificate, Bi4ConeAssemblyV2Error>
{
    let bundle = embedded_bi2_bundle()?;
    let replay = replay_bi2_four_branch_finales_v1(&bundle);
    if !replay.valid
        || bundle.index.schema != BI2_FOUR_BRANCH_FINALES_V2_SCHEMA
        || bundle.index.result_digest != FROZEN_BI2_V2_INDEX_DIGEST
        || !bundle.index.ready_for_bi4_assembly
    {
        return Err(Bi4ConeAssemblyV2Error::Input(format!(
            "sealed BI-2 v2 did not replay as cone-ready: {:?}",
            replay.errors
        )));
    }
    assemble_after_bi2_replay(&bundle)
}

pub fn replay_bi4_cone_assembly_v2(
    claimed: &Bi4ConeAssemblyV2Certificate,
) -> Bi4ConeAssemblyV2Replay {
    match issue_bi4_cone_assembly_v2() {
        Ok(expected) => replay_bi4_against_expected(claimed, &expected),
        Err(error) => Bi4ConeAssemblyV2Replay {
            valid: false,
            errors: vec![error.to_string()],
        },
    }
}

fn replay_bi4_against_expected(
    claimed: &Bi4ConeAssemblyV2Certificate,
    expected: &Bi4ConeAssemblyV2Certificate,
) -> Bi4ConeAssemblyV2Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("BI-4 v2 certificate digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("BI-4 v2 source bindings drifted".to_owned());
    }
    if expected != claimed {
        errors.push("BI-4 v2 certificate differs from deterministic reissuance".to_owned());
    }
    Bi4ConeAssemblyV2Replay {
        valid: errors.is_empty(),
        errors,
    }
}

pub fn replay_bi4_cone_assembly_v2_json(json: &str) -> Bi4ConeAssemblyV2Replay {
    match serde_json::from_str::<Bi4ConeAssemblyV2Certificate>(json) {
        Ok(certificate) => replay_bi4_cone_assembly_v2(&certificate),
        Err(error) => Bi4ConeAssemblyV2Replay {
            valid: false,
            errors: vec![error.to_string()],
        },
    }
}

pub fn render_bi4_cone_assembly_v2(certificate: &Bi4ConeAssemblyV2Certificate) -> String {
    let mut out = String::new();
    out.push_str("# BI-4 v2 cone report\n\n");
    out.push_str(&format!(
        "**Date:** {}. **Zone:** `{}`. **Refinement:** `{}`. **Certificate:** `{}`.\n\n",
        certificate.date,
        certificate.outcome_zone,
        certificate.refinement_label.as_deref().unwrap_or("none"),
        certificate.result_digest
    ));
    out.push_str(&format!(
        "The exact frozen BI-2 v2 aggregate `{}` replayed before comparison: **{}**. All four branch certificates reissued, and the enacted root was retained only as indexical testimony, never as a comparison baseline or selector. The full frozen taxonomy was applied after certifying `Z-TREE`/`Z-STOP` preconditions as **{}/{}**.\n\n",
        certificate.bi2_index_digest,
        certificate.bi2_index_digest_matches_frozen_plan,
        certificate.z_tree_precondition_present,
        certificate.z_stop_precondition_present,
    ));
    out.push_str("| Measurement | Verdict | Exact basis |\n|---|---|---|\n");
    for row in [
        &certificate.g2,
        &certificate.g3a,
        &certificate.g3b,
        &certificate.g3c,
        &certificate.g3_composite,
        &certificate.cone_g4,
    ] {
        out.push_str(&format!(
            "| `{}` | `{:?}` | {} |\n",
            row.level, row.verdict, row.exact_basis
        ));
    }
    out.push_str("\n## All six branch pairs\n\n");
    for (label, rows) in [
        ("G2", &certificate.g2_pair_comparisons),
        ("G3a", &certificate.g3a_pair_comparisons),
        ("G3b", &certificate.g3b_pair_comparisons),
        ("G3c", &certificate.g3c_pair_comparisons),
    ] {
        out.push_str(&format!("### {label}\n\n"));
        out.push_str(
            "| Left | Right | Equal | First stage | Exact context |\n|---|---|---:|---:|---|\n",
        );
        for row in rows {
            let short = |root: &str| {
                root.strip_prefix("blake3:")
                    .unwrap_or(root)
                    .chars()
                    .take(12)
                    .collect::<String>()
            };
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} |\n",
                short(&row.left_branch_root),
                short(&row.right_branch_root),
                row.equal,
                row.first_divergence_stage
                    .map(|stage| stage.to_string())
                    .unwrap_or_else(|| "—".to_owned()),
                row.exact_context,
            ));
        }
        out.push('\n');
    }
    out.push_str("\n## Branch measurements\n\n");
    out.push_str("| Branch | Enacted | Original G2 discharge predicate | sum kappa | complete sum semantic nu | suffix sum nu 5-15 | halt/O16/F1/E5 |\n|---|---:|---:|---:|---:|---:|---|\n");
    for branch in &certificate.branches {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} | `{}/{}/{}/{}` |\n",
            branch
                .branch_root_hash
                .strip_prefix("blake3:")
                .unwrap_or(&branch.branch_root_hash)
                .chars()
                .take(12)
                .collect::<String>(),
            branch.enacted_root,
            branch.g2_original_unique_winner_total_discharge_predicate_replayed,
            branch.complete_sum_kappa,
            branch.complete_sum_semantic_nu,
            branch.suffix_sum_semantic_nu_stage5_through15,
            branch.local_g4_debt_free_halt_at_15,
            branch.semantic_o16_empty,
            branch.f1_excluded,
            branch.e5_complete,
        ));
    }
    out.push_str("\n## Diagnostic bar trajectories\n\n");
    for branch in &certificate.branches {
        out.push_str(&format!(
            "- `{}`: {}\n",
            branch
                .branch_root_hash
                .strip_prefix("blake3:")
                .unwrap_or(&branch.branch_root_hash)
                .chars()
                .take(12)
                .collect::<String>(),
            branch
                .diagnostic_bar_stage5_through15
                .iter()
                .map(|row| format!("{}={}", row.stage, row.bar))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    out.push_str("\n## First new divergence\n\n");
    out.push_str(
        certificate
            .first_new_divergence_published_verbatim
            .as_deref()
            .unwrap_or("No new G2/G3/G4 divergence."),
    );
    out.push_str(&format!(
        "\n\nThe inherited G1 R-T2 refutation remains testimony and was not re-litigated. Its self-digest, exact inequivalent outcome, order-reversal audit, selector absence, and six inequivalent five-scheme pairs replayed: **{}/{}/{}/{}/{}**.\n\n",
        certificate.inherited_g1_testimony.artifact_self_digest_valid,
        certificate
            .inherited_g1_testimony
            .all_stage5_scheme_sets_inequivalent,
        certificate.inherited_g1_testimony.order_reversal_invariant,
        certificate.inherited_g1_testimony.selector_absent,
        certificate
            .inherited_g1_testimony
            .exact_six_inequivalent_five_scheme_pairs,
    ));
    out.push_str("## F-R3-B1 branch-index disposition\n\n");
    out.push_str("| Claim | Disposition | Exact premise |\n|---|---|---|\n");
    for row in &certificate.branch_index_disposition {
        out.push_str(&format!(
            "| `{}` | `{:?}` | `{}` |\n",
            row.claim, row.status, row.exact_verdict_premise
        ));
    }
    out.push_str("\n## Frozen drift and correspondence hygiene\n\n");
    out.push_str(&format!(
        "BI-1b public deterministic replay passed: **{}**; frozen-testimony fallback used: **{}**; exact disclosed errors: `{:?}`. The drift was repaired inside assembly: **{}**; concealed: **{}**.\n\n",
        certificate
            .drift_disclosure
            .bi1b_public_deterministic_replay_passed,
        certificate.drift_disclosure.frozen_testimony_fallback_used,
        certificate.drift_disclosure.public_replay_errors,
        certificate.drift_disclosure.drift_repaired_inside_assembly,
        certificate.drift_disclosure.drift_concealed_inside_assembly,
    ));
    out.push_str(&format!(
        "The correspondence ledger was opened after every comparison verdict: **{}**. It corroborates 24/24 Stage-8–15 rows and was used by a verdict, selector, or provenance issuer: **{}/{}/{}**.\n\n",
        certificate
            .correspondence_corroboration
            .opened_after_all_comparison_verdicts,
        certificate
            .correspondence_corroboration
            .used_by_any_comparison_verdict,
        certificate.correspondence_corroboration.used_by_selector,
        certificate.correspondence_corroboration.used_as_provenance,
    ));
    out.push_str(&format!(
        "No UC-1 content, bridge execution, or final certificate is present: **{}/{}/{}**.\n\n{}",
        !certificate.uc1_content_present,
        !certificate.bridge_executed,
        !certificate.final_certificate_issued,
        certificate.permitted_conclusion,
    ));
    out
}

fn create_new(path: &Path, contents: &[u8]) -> Result<(), Bi4ConeAssemblyV2Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Bi4ConeAssemblyV2Error::Io(format!("{}: {error}", path.display())))?;
    if let Err(error) = file.write_all(contents) {
        drop(file);
        let rollback = remove_file(path);
        return Err(Bi4ConeAssemblyV2Error::Io(format!(
            "{}: {error}; partial-file rollback={rollback:?}",
            path.display()
        )));
    }
    Ok(())
}

pub fn emit_bi4_cone_assembly_v2_create_new(
    directory: &Path,
) -> Result<Bi4ConeAssemblyV2Certificate, Bi4ConeAssemblyV2Error> {
    let certificate = issue_bi4_cone_assembly_v2()?;
    let replay = replay_bi4_cone_assembly_v2(&certificate);
    if !replay.valid {
        return Err(Bi4ConeAssemblyV2Error::Invariant(format!(
            "create-new emission requires exact replay: {}",
            replay.errors.join("; ")
        )));
    }
    let json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Bi4ConeAssemblyV2Error::Json(error.to_string()))?;
    let report = render_bi4_cone_assembly_v2(&certificate);
    let outputs = [
        (directory.join(BI4_CONE_V2_CERTIFICATE_NAME), json),
        (directory.join(BI4_CONE_V2_REPORT_NAME), report.into_bytes()),
    ];
    if let Some((path, _)) = outputs.iter().find(|(path, _)| path.exists()) {
        return Err(Bi4ConeAssemblyV2Error::Io(format!(
            "{} already exists; create-new refuses overwrite",
            path.display()
        )));
    }
    create_new(&outputs[0].0, &outputs[0].1)?;
    if let Err(error) = create_new(&outputs[1].0, &outputs[1].1) {
        let rollback = remove_file(&outputs[0].0);
        return Err(Bi4ConeAssemblyV2Error::Io(format!(
            "paired create-new report failed: {error}; certificate rollback={rollback:?}"
        )));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rehash(certificate: &mut Bi4ConeAssemblyV2Certificate) {
        certificate.result_digest.clear();
        certificate.result_digest = certificate_digest(certificate);
    }

    fn flip_audit(row: &mut Bi4GranularityAuditV2) {
        row.passed = !row.passed;
        row.verdict = if row.passed {
            Bi4VerdictV2::Passed
        } else {
            Bi4VerdictV2::Refuted
        };
        row.derivation_hash = audit_digest(row);
    }

    fn assert_rehashed_mutation_rejected(
        expected: &Bi4ConeAssemblyV2Certificate,
        mut claimed: Bi4ConeAssemblyV2Certificate,
    ) {
        rehash(&mut claimed);
        let replay = replay_bi4_against_expected(&claimed, expected);
        assert!(
            !replay.valid,
            "fully rehashed counterfactual unexpectedly replayed"
        );
    }

    #[test]
    fn frozen_oracle_has_registered_split_without_suppressing_g4() {
        let bundle = embedded_bi2_bundle().expect("embedded BI-2");
        let certificate = assemble_after_bi2_replay(&bundle).expect("BI-4 assembly");
        assert!(certificate.g2.passed);
        assert!(certificate.g3a.passed);
        assert!(!certificate.g3b.passed);
        assert!(!certificate.g3c.passed);
        assert!(!certificate.g3_composite.passed);
        assert!(certificate.cone_g4.passed);
        assert_eq!(certificate.outcome_zone, "Z-SPLIT");
        assert_eq!(certificate.refinement_label.as_deref(), Some("Z-SPLIT-4"));
        assert_eq!(
            certificate
                .branches
                .iter()
                .map(|row| row.complete_sum_semantic_nu)
                .collect::<Vec<_>>(),
            vec![32, 31, 31, 32]
        );
        assert!(certificate.all_six_pairs_published_for_each_comparison);
        assert!(certificate.every_claim_disposed_explicitly);
    }

    #[test]
    fn non_stage4_ledger_change_removes_split4_and_refutes_g3a() {
        let mut bundle = embedded_bi2_bundle().expect("embedded BI-2");
        bundle.branches[0].finale_input.steps[4].certified_nu += 1;
        bundle.branches[0].step_provenance[4].semantic_nu += 1;
        let certificate = assemble_after_bi2_replay(&bundle).expect("counterfactual assembly");
        assert!(!certificate.g3a.passed);
        assert!(!certificate.stage4_offset_only_explanation_proved);
        assert_eq!(certificate.refinement_label, None);
    }

    #[test]
    fn branch_input_order_is_not_a_semantic_input() {
        let bundle = embedded_bi2_bundle().expect("embedded BI-2");
        let expected = assemble_after_bi2_replay(&bundle).expect("assembly");
        let mut reversed = bundle;
        reversed.branches.reverse();
        let observed = assemble_after_bi2_replay(&reversed).expect("reordered assembly");
        assert_eq!(expected, observed);
    }

    #[test]
    fn non_frozen_bi2_index_digest_is_rejected_before_comparison() {
        let mut bundle = embedded_bi2_bundle().expect("embedded BI-2");
        bundle.index.result_digest = "blake3:counterfactual".to_owned();
        assert!(matches!(
            assemble_after_bi2_replay(&bundle),
            Err(Bi4ConeAssemblyV2Error::Input(_))
        ));
    }

    #[test]
    fn fully_rehashed_mutation_matrix_fails_frozen_reissuance() {
        let bundle = embedded_bi2_bundle().expect("embedded BI-2");
        let expected = assemble_after_bi2_replay(&bundle).expect("BI-4 assembly");

        for select in 0..6 {
            let mut claimed = expected.clone();
            let row = match select {
                0 => &mut claimed.g2,
                1 => &mut claimed.g3a,
                2 => &mut claimed.g3b,
                3 => &mut claimed.g3c,
                4 => &mut claimed.g3_composite,
                _ => &mut claimed.cone_g4,
            };
            flip_audit(row);
            assert_rehashed_mutation_rejected(&expected, claimed);
        }

        let mut claimed = expected.clone();
        claimed.branches[0].g2_profile_stage5_through16[0].live_obligation_count += 1;
        claimed.branches[0].g2_profile_stage5_through16[0].derivation_hash =
            g2_row_digest(&claimed.branches[0].g2_profile_stage5_through16[0]);
        claimed.branches[0].derivation_hash = measurement_digest(&claimed.branches[0]);
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed.branches[0].g2_profile_stage5_through16[11].debt_free = false;
        claimed.branches[0].g2_profile_stage5_through16[11].derivation_hash =
            g2_row_digest(&claimed.branches[0].g2_profile_stage5_through16[11]);
        claimed.branches[0].derivation_hash = measurement_digest(&claimed.branches[0]);
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed.branches[0].complete_ledger_stage1_through15[4]
            .candidate_hash
            .push_str("-mutated");
        claimed.branches[0].complete_ledger_stage1_through15[4].derivation_hash =
            ledger_row_digest(&claimed.branches[0].complete_ledger_stage1_through15[4]);
        claimed.branches[0].g3a_ledger_stage5_through15[0] =
            claimed.branches[0].complete_ledger_stage1_through15[4].clone();
        claimed.branches[0].derivation_hash = measurement_digest(&claimed.branches[0]);
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed.branches[0].diagnostic_bar_stage5_through15[0].bar = "counterfactual".to_owned();
        claimed.branches[0].diagnostic_bar_stage5_through15[0].derivation_hash =
            bar_row_digest(&claimed.branches[0].diagnostic_bar_stage5_through15[0]);
        claimed.branches[0].derivation_hash = measurement_digest(&claimed.branches[0]);
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed.branch_index_disposition[0].status = Bi4DispositionStatusV2::RemainsBranchIndexed;
        claimed.branch_index_disposition[0].derivation_hash =
            disposition_digest(&claimed.branch_index_disposition[0]);
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed.outcome_zone = "Z-CONE".to_owned();
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed.refinement_label = None;
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed
            .correspondence_corroboration
            .used_by_any_comparison_verdict = true;
        claimed.correspondence_corroboration.derivation_hash =
            correspondence_projection_digest(&claimed.correspondence_corroboration);
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed.drift_disclosure.drift_repaired_inside_assembly = true;
        claimed.drift_disclosure.derivation_hash = drift_digest(&claimed.drift_disclosure);
        assert_rehashed_mutation_rejected(&expected, claimed);

        for select in 0..3 {
            let mut claimed = expected.clone();
            match select {
                0 => claimed.uc1_content_present = true,
                1 => claimed.bridge_executed = true,
                _ => claimed.final_certificate_issued = true,
            }
            assert_rehashed_mutation_rejected(&expected, claimed);
        }

        let mut claimed = expected.clone();
        claimed.z_tree_precondition_present = true;
        assert_rehashed_mutation_rejected(&expected, claimed);

        let mut claimed = expected.clone();
        claimed.source_bindings[0].blake3.push_str("-mutated");
        assert_rehashed_mutation_rejected(&expected, claimed);
    }

    #[test]
    fn fully_rehashed_verdict_mutation_fails_replay() {
        let mut certificate = issue_bi4_cone_assembly_v2().expect("issued certificate");
        certificate.g3b.passed = true;
        certificate.g3b.verdict = Bi4VerdictV2::Passed;
        certificate.g3b.derivation_hash = audit_digest(&certificate.g3b);
        rehash(&mut certificate);
        assert!(!replay_bi4_cone_assembly_v2(&certificate).valid);
    }
}
