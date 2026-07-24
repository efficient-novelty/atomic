//! BI-1b prefix-general provenance sweep.
//!
//! The orchestration boundary is intentionally strict:
//!
//! 1. Parse the four sealed BI-1 v3 branch certificates and their manifest.
//! 2. Attempt the public v3 replay once and record its exact result.  BI-1b
//!    consumes the frozen v3 testimony by its preregistered manifest digest,
//!    certificate self-digests, and exact cone/root cross-bindings; unrelated
//!    post-seal BI-0 dependency drift is never mislabeled as a successful v3
//!    reissuance.
//! 3. Reproduce the enacted Stage-8 v6 classification with the one
//!    prefix-general v4 procedure and seal a row-free authorization.
//! 4. Give each alternate runner only its own sealed certificate and its own
//!    narrow window authorization.
//! 5. Seal all four branch digests before constructing the correspondence
//!    diagnostic.
//!
//! This module issues BI-1b only.  It cannot issue BI-2, BI-4, G2/G3, UC-1,
//! or an unindexed halt/debt claim.

use crate::branch_invariance::{BranchContinuationLimits, BranchContinuationOutcome};
use crate::branch_invariance_resume_v1::{
    Bi1bCandidateAssessmentV1, Bi1bRegressionGate, Bi1bResumeOutcomeV1,
    Bi1bSealedWindowAuthorizationV1, Bi1bStageRecordV1, BranchInvarianceResumeV1,
    issue_bi1b_sealed_window_authorization_v1, issue_branch_invariance_resume_v1,
    replay_bi1b_sealed_window_authorization_v1, replay_branch_invariance_resume_v1,
};
use crate::branch_invariance_sweep_v3::{
    BI1_BRANCH_V3_SCHEMA, BI1_ENACTED_REGRESSION_V3_SCHEMA, BI1_OPTION_A_SWEEP_V3_SCHEMA,
    Bi1BranchCertificateV3, Bi1BranchManifestRowV3, Bi1OptionASweepBundleV3,
    Bi1OptionASweepManifestV3, replay_bi1_option_a_sweep_v3,
};
use crate::branch_prefix_general_provenance_v4::{
    BranchPrefixGeneralProvenanceV4Authority, BranchPrefixGeneralProvenanceV4Token,
    BranchPrefixGeneralRoleClassificationV4, issue_branch_prefix_general_provenance_v4,
    issue_branch_prefix_general_provenance_v4_authority,
    project_branch_prefix_general_role_classifications_v4,
    replay_branch_prefix_general_provenance_v4,
    replay_branch_prefix_general_provenance_v4_authority,
    validate_branch_prefix_general_provenance_v4_token_integrity,
};
use crate::t_bi_nu1_regression_v6::{
    T_BI_NU1_REGRESSION_V6_SCHEMA, TBiNu1RegressionV6Certificate,
    issue_t_bi_nu1_regression_v6_certificate, replay_t_bi_nu1_regression_v6_certificate,
};
use pen_core::canonical::canonicalize_telescope;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const BI1B_PREFIX_GENERAL_SWEEP_V1_SCHEMA: &str = "bi1b-prefix-general-provenance-sweep-v1";
pub const BI1B_PREFIX_GENERAL_SWEEP_V1_DATE: &str = "2026-07-23";
pub const BI1B_REGRESSION_V1_SCHEMA: &str = "bi1b-enacted-stage8-prefix-general-v6-regression-v1";
pub const BI1B_REGRESSION_AUTHORIZATION_V1_SCHEMA: &str =
    "bi1b-row-free-enacted-regression-authorization-v1";
pub const BI1B_BRANCH_V1_SCHEMA: &str = "bi1b-prefix-general-branch-result-v1";
pub const BI1B_CORRESPONDENCE_V1_SCHEMA: &str = "bi1b-write-only-byte-correspondence-diagnostic-v1";

pub const BI1B_REGRESSION_CERTIFICATE_NAME: &str = "BI1B_REGRESSION_CERTIFICATE.json";
pub const BI1B_REGRESSION_REPORT_NAME: &str = "BI1B_REGRESSION_RESULT.md";
pub const BI1B_CORRESPONDENCE_NAME: &str = "BI1B_CORRESPONDENCE_DIAGNOSTIC.json";
pub const BI1B_SWEEP_CERTIFICATE_NAME: &str = "BI1B_PREFIX_GENERAL_SWEEP_V1_CERTIFICATE.json";
pub const BI1B_SWEEP_REPORT_NAME: &str = "BI1B_PREFIX_GENERAL_SWEEP_V1_RESULT.md";

const FROZEN_BI1_V3_MANIFEST_DIGEST: &str =
    "blake3:7a03ff1dab14cff0b385b9d2b482071671296a8c6132dc63f37cb9eeb2506fdb";
const FROZEN_BI1_V3_STAGE4_CONE_DIGEST: &str =
    "blake3:417719ae12e4bf3f72196a35064c8767cd41ddca765c28736bfe760c999b3a93";
const FROZEN_T_BI_V6_CERTIFICATE_DIGEST: &str =
    "blake3:d51d0543c32542200fff6e5cfd234da1bb2ffc12c058c805ac7974385727c8b4";
const T_BI_V6_CURRENT_REISSUANCE_DRIFT: &str =
    "T-BI v6 certificate differs from deterministic create-new reissuance";
const FROZEN_BI1_V3_BRANCH_BINDINGS: [(&str, &str); 4] = [
    (
        "blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407",
        "blake3:5ac75d60b84ce1bc6871c9c027426c28f4d80cbfd44bf5cedaeb99fc8e216a70",
    ),
    (
        "blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308",
        "blake3:473d05bc808fc034d20541c49484374af2992c45a1d3658901ecba7c01e6b984",
    ),
    (
        "blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b",
        "blake3:07d452d788afa084128350ba8fec4cad01f56b1afacf57abe6b6f3d21083e048",
    ),
    (
        "blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4",
        "blake3:0056f219ad010c22ea2c73c893d346dde9335c7fbe6b51340e13141115cbc54c",
    ),
];

const BI1B_MAX_INSPECTED_STAGE: u32 = 64;
const BI1B_MAX_ENUMERATED_CANDIDATES_PER_STAGE: usize = 10_000_000;

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/bi1b_prefix_general_provenance_plan.md");
const SEALED_SWEEP_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1_OPTION_A_SWEEP_V3_CERTIFICATE.json");
const SEALED_ENACTED_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json");
const SEALED_ALT_A_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json");
const SEALED_ALT_B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json");
const SEALED_ALT_C_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json");
const SEALED_T_BI_V6_BYTES: &[u8] =
    include_bytes!("../../../docs/t_bi_nu1_semantic_provenance_v6.json");
const TYPED_BOUNDARY_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/cubical/typed_boundary.rs");
const STEP8_R2_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/step8_r2.rs");
const E4_GENERATOR_BASIS_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/e4_generator_basis.rs");
const ACT_LOCAL_V4_SOURCE_BYTES: &[u8] = include_bytes!("act_local_semantic_provenance_v4.rs");
const ACT_LOCAL_V5_SOURCE_BYTES: &[u8] = include_bytes!("act_local_semantic_provenance_v5.rs");
const PREFIX_GENERAL_V4_SOURCE_BYTES: &[u8] =
    include_bytes!("branch_prefix_general_provenance_v4.rs");
const SEALED_BI1_V3_REPLAY_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_sweep_v3.rs");
const T_BI_V6_REPLAY_SOURCE_BYTES: &[u8] = include_bytes!("t_bi_nu1_regression_v6.rs");
const RESUME_V1_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_resume_v1.rs");
const SWEEP_V1_SOURCE_BYTES: &[u8] = include_bytes!("bi1b_prefix_general_sweep_v1.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bSourceBindingV1 {
    pub path: String,
    pub role: String,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bSealedSweepPreflightV1 {
    pub sealed_sweep_digest: String,
    pub sealed_stage4_cone_digest: String,
    pub sealed_branch_certificate_digests: Vec<String>,
    pub public_v3_replay_passed: bool,
    pub public_v3_replay_errors: Vec<String>,
    pub sealed_testimony_fallback_used: bool,
    pub frozen_manifest_digest_exact: bool,
    pub frozen_branch_root_and_certificate_bindings_exact: bool,
    pub common_stage4_cone_binding_exact: bool,
    pub every_branch_root_matches_its_sealed_seed: bool,
    pub exactly_one_enacted_root_cross_bound: bool,
    pub sealed_manifest_self_digest_valid: bool,
    pub every_manifest_row_self_digest_valid: bool,
    pub every_branch_certificate_self_digest_valid: bool,
    pub every_manifest_branch_cross_binding_exact: bool,
    pub sealed_content_preflight_passed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bRegressionAuthorizationV1 {
    pub schema: String,
    pub sealed_global_bi1_sweep_digest: String,
    pub sealed_global_stage4_cone_digest: String,
    pub sealed_global_bi1_sweep_content_authenticated: bool,
    pub extraction_procedure_digest: String,
    pub authorized_sealed_branch_certificate_digests: Vec<String>,
    pub hard_regression_passed: bool,
    pub exact_proved_rows_reproduced: bool,
    pub exact_theorem_impossibility_rows_reproduced: bool,
    pub zero_residuals_reproduced: bool,
    pub one_prefix_parametric_procedure: bool,
    pub family_ids_exported: bool,
    pub semantic_vectors_exported: bool,
    pub enacted_winners_exported: bool,
    pub enacted_outcomes_exported: bool,
    pub selector_capability_exported: bool,
    pub derivation_hash: String,
}

impl Bi1bRegressionGate for Bi1bRegressionAuthorizationV1 {
    fn replay_valid(&self) -> bool {
        self.derivation_hash == regression_authorization_digest(self)
    }

    fn sealed_global_sweep_content_authenticated(&self) -> bool {
        self.sealed_global_bi1_sweep_content_authenticated
    }

    fn sealed_sweep_digest(&self) -> &str {
        &self.sealed_global_bi1_sweep_digest
    }

    fn sealed_stage4_cone_digest(&self) -> &str {
        &self.sealed_global_stage4_cone_digest
    }

    fn authorization_digest(&self) -> &str {
        &self.derivation_hash
    }

    fn extraction_procedure_digest(&self) -> &str {
        &self.extraction_procedure_digest
    }

    fn authorized_branch_certificate_count(&self) -> usize {
        self.authorized_sealed_branch_certificate_digests.len()
    }

    fn authorizes_branch_certificate_digest(&self, digest: &str) -> bool {
        self.authorized_sealed_branch_certificate_digests
            .binary_search_by(|candidate| candidate.as_str().cmp(digest))
            .is_ok()
    }

    fn hard_regression_passed(&self) -> bool {
        self.hard_regression_passed
    }

    fn exact_proved_rows_reproduced(&self) -> bool {
        self.exact_proved_rows_reproduced
    }

    fn exact_theorem_impossibility_rows_reproduced(&self) -> bool {
        self.exact_theorem_impossibility_rows_reproduced
    }

    fn zero_residuals_reproduced(&self) -> bool {
        self.zero_residuals_reproduced
    }

    fn one_prefix_parametric_procedure(&self) -> bool {
        self.one_prefix_parametric_procedure
    }

    fn family_ids_exported(&self) -> bool {
        self.family_ids_exported
    }

    fn semantic_vectors_exported(&self) -> bool {
        self.semantic_vectors_exported
    }

    fn enacted_winners_exported(&self) -> bool {
        self.enacted_winners_exported
    }

    fn enacted_outcomes_exported(&self) -> bool {
        self.enacted_outcomes_exported
    }

    fn selector_capability_exported(&self) -> bool {
        self.selector_capability_exported
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bRegressionCertificateV1 {
    pub schema: String,
    pub date: String,
    pub preflight: Bi1bSealedSweepPreflightV1,
    pub extraction_authority: BranchPrefixGeneralProvenanceV4Authority,
    pub extraction_authority_replayed: bool,
    pub enacted_bi1_branch_certificate_digest: String,
    pub enacted_stage8_candidate_hash: String,
    pub t_bi_v6_certificate_digest: String,
    pub t_bi_v6_frozen_digest_exact: bool,
    pub t_bi_v6_self_digest_valid: bool,
    pub t_bi_v6_public_deterministic_replay_passed: bool,
    pub t_bi_v6_public_replay_errors: Vec<String>,
    pub t_bi_v6_postseal_reissuance_drift_only: bool,
    pub t_bi_v6_frozen_testimony_authenticated: bool,
    pub current_t_bi_v6_reissuance_digest: String,
    pub current_t_bi_v6_global_semantic_surface_matches_frozen: bool,
    pub current_t_bi_v6_stage8_semantic_projection_matches_frozen: bool,
    pub t_bi_v6_stage8_package_derivation_hash: String,
    pub v6_credited_family_ids: Vec<String>,
    pub prefix_general_credited_family_ids: Vec<String>,
    pub v6_proved_rows: Vec<BranchPrefixGeneralRoleClassificationV4>,
    pub prefix_general_proved_rows: Vec<BranchPrefixGeneralRoleClassificationV4>,
    pub v6_theorem_impossibility_rows: Vec<BranchPrefixGeneralRoleClassificationV4>,
    pub prefix_general_theorem_impossibility_rows: Vec<BranchPrefixGeneralRoleClassificationV4>,
    pub prefix_general_stage8_token: BranchPrefixGeneralProvenanceV4Token,
    pub prefix_general_stage8_token_replayed: bool,
    pub exact_family_ids_reproduced: bool,
    pub exact_proved_rows_reproduced: bool,
    pub exact_theorem_impossibility_rows_reproduced: bool,
    pub exact_role_and_semantic_totals_reproduced: bool,
    pub v6_r2_typed_signature_derivation_hash: Option<String>,
    pub prefix_general_r2_typed_signature_derivation_hash: Option<String>,
    pub v6_r2_m1_membership_derivation_hash: Option<String>,
    pub prefix_general_r2_m1_membership_derivation_hash: Option<String>,
    pub r2_proof_hashes_recorded_as_separate_evidence: bool,
    pub exact_r2_regression_reproduced: bool,
    pub zero_residuals_reproduced: bool,
    pub one_procedure_digest: bool,
    pub f_b1b_1_passed: bool,
    pub authorization: Bi1bRegressionAuthorizationV1,
    pub authorization_replayed: bool,
    pub alternate_branch_work_executed_before_regression_sealed: bool,
    pub bi2_finale_issued: bool,
    pub bi4_cone_verdict_issued: bool,
    pub g2_verdict_issued: bool,
    pub g3_verdict_issued: bool,
    pub uc1_scored: bool,
    pub unindexed_claim_issued: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bBranchCertificateV1 {
    pub schema: String,
    pub date: String,
    pub branch_root_hash: String,
    pub branch_digest_prefix: String,
    pub enacted_root: bool,
    pub issuance_ordinal: usize,
    pub sealed_bi1_certificate_digest: String,
    pub regression_authorization_digest: String,
    pub sealed_window_authorization: Bi1bSealedWindowAuthorizationV1,
    pub sealed_window_authorization_replayed: bool,
    pub resume: Option<BranchInvarianceResumeV1>,
    pub resume_replayed: bool,
    pub prior_enacted_halt_revalidated: bool,
    pub terminal_disposition: String,
    pub resumed_stage_count: usize,
    pub every_completed_census_followed_total_classification: bool,
    pub unknown_never_counted_as_discharger_or_exclusion: bool,
    pub one_extraction_procedure_digest: bool,
    pub lawful_branch_disposition: bool,
    pub enacted_classifications_or_outcomes_consumed: bool,
    pub byte_identity_consumed_as_provenance_or_selector: bool,
    pub semantic_value_used_as_selector: bool,
    pub bar_used_as_selector: bool,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub branch_indexed_only: bool,
    pub bi2_finale_issued: bool,
    pub bi4_cone_verdict_issued: bool,
    pub g2_verdict_issued: bool,
    pub g3_verdict_issued: bool,
    pub uc1_scored: bool,
    pub unindexed_claim_issued: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bCanonicalEncodingNecessaryMatchV1 {
    pub enacted_candidate_hash: String,
    pub alternate_candidate_hash: String,
    pub canonical_encoding: String,
    pub enacted_fnv_key_diagnostic: String,
    pub alternate_fnv_key_diagnostic: String,
    pub typed_fork_shadow_proved: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bCorrespondenceRowV1 {
    pub alternate_branch_root_hash: String,
    pub stage: u32,
    pub enacted_candidate_hashes: Vec<String>,
    pub alternate_candidate_hashes: Vec<String>,
    pub byte_identical_candidate_hashes: Vec<String>,
    pub enacted_only_candidate_hashes: Vec<String>,
    pub alternate_only_candidate_hashes: Vec<String>,
    pub predecessor_difference_stages: Vec<u32>,
    pub stage4_only_prefix_shadow_for_byte_identical_candidates: bool,
    pub canonical_encoding_necessary_matches: Vec<Bi1bCanonicalEncodingNecessaryMatchV1>,
    pub enacted_winner_hash: Option<String>,
    pub alternate_winner_hash: Option<String>,
    pub winners_byte_identical: bool,
    pub correspondence_used_as_provenance: bool,
    pub correspondence_used_as_selector: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bCorrespondenceDiagnosticV1 {
    pub schema: String,
    pub date: String,
    pub sealed_branch_certificate_digests: Vec<String>,
    pub issued_after_all_branch_certificate_digests_sealed: bool,
    pub rows: Vec<Bi1bCorrespondenceRowV1>,
    pub byte_identity_is_zero_charge_diagnostic_only: bool,
    pub canonical_encoding_equality_not_treated_as_typed_fork_shadow_proof: bool,
    pub fnv_key_used_only_as_collision_prone_diagnostic: bool,
    pub used_by_provenance_issuer: bool,
    pub used_by_branch_selector: bool,
    pub bi2_finale_issued: bool,
    pub bi4_cone_verdict_issued: bool,
    pub g2_verdict_issued: bool,
    pub g3_verdict_issued: bool,
    pub uc1_scored: bool,
    pub unindexed_claim_issued: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bSweepBranchRowV1 {
    pub issuance_ordinal: usize,
    pub branch_root_hash: String,
    pub enacted_root: bool,
    pub branch_certificate_digest: String,
    pub terminal_disposition: String,
    pub resumed_stage_count: usize,
    pub lawful_branch_disposition: bool,
    pub row_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bPrefixGeneralSweepV1Certificate {
    pub schema: String,
    pub date: String,
    pub plan_blake3: String,
    pub source_bindings: Vec<Bi1bSourceBindingV1>,
    pub sealed_bi1_sweep_digest: String,
    pub regression_certificate_digest: String,
    pub regression_authorization_digest: String,
    pub extraction_procedure_digest: String,
    pub branch_rows: Vec<Bi1bSweepBranchRowV1>,
    pub branch_count: usize,
    pub exact_four_branch_surface: bool,
    pub enacted_branch_revalidated_first: bool,
    pub alternate_branch_count: usize,
    pub every_branch_certificate_replayed: bool,
    pub correspondence_diagnostic_digest: String,
    pub correspondence_issued_after_branch_seals: bool,
    pub supersedes_nothing: bool,
    pub one_extraction_procedure_digest: bool,
    pub f_b1b_1_passed: bool,
    pub f_b1b_2_passed: bool,
    pub f_b1b_3_passed: bool,
    pub f_b1b_4_passed: bool,
    pub f_b1b_5_passed: bool,
    pub f_b1b_6_passed: bool,
    pub bi2_finale_issued: bool,
    pub bi4_cone_report_issued: bool,
    pub cross_branch_g2_verdict_issued: bool,
    pub cross_branch_g3_verdict_issued: bool,
    pub uc1_scored: bool,
    pub unindexed_halt_ledger_or_o_claim_issued: bool,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bi1bPrefixGeneralSweepV1Bundle {
    pub regression: Bi1bRegressionCertificateV1,
    pub branches: Vec<Bi1bBranchCertificateV1>,
    pub correspondence: Bi1bCorrespondenceDiagnosticV1,
    pub sweep: Bi1bPrefixGeneralSweepV1Certificate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi1bPrefixGeneralSweepV1Replay {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bi1bPrefixGeneralSweepV1Error {
    #[error("BI-1b sealed-input failure: {0}")]
    SealedInput(String),
    #[error("BI-1b enacted regression failure: {0}")]
    Regression(String),
    #[error("BI-1b branch resume failure: {0}")]
    Resume(String),
    #[error("BI-1b invariant failure: {0}")]
    Invariant(String),
    #[error("BI-1b JSON failure: {0}")]
    Json(String),
    #[error("BI-1b create-new I/O failure: {0}")]
    Io(String),
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI1B_PREFIX_GENERAL_SWEEP_V1_SCHEMA, domain, value))
        .expect("BI-1b sweep evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn old_bi1_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI1_OPTION_A_SWEEP_V3_SCHEMA, domain, value))
        .expect("sealed BI-1 v3 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn frozen_t_bi_v6_certificate_digest(certificate: &TBiNu1RegressionV6Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(
        T_BI_NU1_REGRESSION_V6_SCHEMA,
        "source-first-regression-certificate",
        projection,
    ))
    .expect("frozen T-BI v6 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn is_blake3_digest(value: &str) -> bool {
    value.starts_with("blake3:")
        && value.len() == "blake3:".len() + 64
        && value["blake3:".len()..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
}

fn source_bindings() -> Vec<Bi1bSourceBindingV1> {
    [
        (
            "docs/bi1b_prefix_general_provenance_plan.md",
            "frozen BI-1b mission, order, stops, and falsifiers",
            PLAN_BYTES,
        ),
        (
            "crates/pen-type/src/cubical/typed_boundary.rs",
            "typed Stage-8 boundary evidence",
            TYPED_BOUNDARY_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/step8_r2.rs",
            "prefix-general R2 occurrence quotient",
            STEP8_R2_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/e4_generator_basis.rs",
            "prefix-general M1 membership evidence",
            E4_GENERATOR_BASIS_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_semantic_provenance_v4.rs",
            "prefix-general semantic source construction",
            ACT_LOCAL_V4_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_semantic_provenance_v5.rs",
            "prefix-general B1/B2 semantic classification",
            ACT_LOCAL_V5_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_prefix_general_provenance_v4.rs",
            "one prefix-parametric candidate-provenance issuer",
            PREFIX_GENERAL_V4_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_invariance_sweep_v3.rs",
            "frozen BI-1 v3 testimony types and public replay",
            SEALED_BI1_V3_REPLAY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_bi_nu1_regression_v6.rs",
            "sealed enacted v6 regression replay",
            T_BI_V6_REPLAY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_invariance_resume_v1.rs",
            "sealed-window branch-local resume kernel",
            RESUME_V1_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/bi1b_prefix_general_sweep_v1.rs",
            "global enacted-first BI-1b orchestration and artifact replay",
            SWEEP_V1_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Bi1bSourceBindingV1 {
        path: path.to_owned(),
        role: role.to_owned(),
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn digest_prefix(digest: &str) -> String {
    digest
        .strip_prefix("blake3:")
        .unwrap_or(digest)
        .chars()
        .take(12)
        .collect()
}

fn old_branch_digest(branch: &Bi1BranchCertificateV3) -> String {
    let mut projection = branch.clone();
    projection.result_digest.clear();
    old_bi1_hash(BI1_BRANCH_V3_SCHEMA, &projection)
}

fn old_manifest_digest(manifest: &Bi1OptionASweepManifestV3) -> String {
    let mut projection = manifest.clone();
    projection.result_digest.clear();
    old_bi1_hash("sweep-manifest", &projection)
}

fn old_manifest_row_digest(row: &Bi1BranchManifestRowV3) -> String {
    let mut projection = row.clone();
    projection.row_hash.clear();
    old_bi1_hash("branch-manifest-row", &projection)
}

fn old_regression_digest(
    regression: &crate::branch_invariance_sweep_v3::Bi1EnactedRegressionAuthorizationV3,
) -> String {
    let mut projection = regression.clone();
    projection.derivation_hash.clear();
    old_bi1_hash(BI1_ENACTED_REGRESSION_V3_SCHEMA, &projection)
}

fn old_guarded_stage_digest(
    row: &crate::branch_invariance_sweep_v3::Bi1GuardedStageSealV3,
) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    old_bi1_hash("guarded-stage-seal", &projection)
}

fn expected_old_manifest_row(branch: &Bi1BranchCertificateV3) -> Bi1BranchManifestRowV3 {
    let mut row = Bi1BranchManifestRowV3 {
        issuance_ordinal: branch.issuance_ordinal,
        branch_root_hash: branch.branch_root_hash.clone(),
        branch_digest_prefix: branch.branch_digest_prefix.clone(),
        enacted_root: branch.enacted_root,
        certificate_digest: branch.result_digest.clone(),
        continuation_outcome: format!("{:?}", branch.continuation.outcome),
        reached_guarded_stage_count: branch.guarded_stages.len(),
        every_reached_guarded_stage_has_unique_typed_discharger: branch
            .every_reached_guarded_stage_has_unique_typed_discharger,
        branch_indexed_only: branch.branch_indexed_only,
        json_name: format!(
            "BI_BRANCH_V3_{}_CERTIFICATE.json",
            branch.branch_digest_prefix
        ),
        report_name: format!("BI_BRANCH_V3_{}_RESULT.md", branch.branch_digest_prefix),
        row_hash: String::new(),
    };
    row.row_hash = old_manifest_row_digest(&row);
    row
}

fn regression_authorization_digest(value: &Bi1bRegressionAuthorizationV1) -> String {
    let mut projection = value.clone();
    projection.derivation_hash.clear();
    tagged_hash(BI1B_REGRESSION_AUTHORIZATION_V1_SCHEMA, &projection)
}

fn regression_authorization_errors(value: &Bi1bRegressionAuthorizationV1) -> Vec<String> {
    let mut errors = Vec::new();
    if value.derivation_hash != regression_authorization_digest(value) {
        errors.push("BI-1b regression authorization digest mismatch".to_owned());
    }
    if value.schema != BI1B_REGRESSION_AUTHORIZATION_V1_SCHEMA
        || !value.sealed_global_bi1_sweep_content_authenticated
        || value.sealed_global_bi1_sweep_digest != FROZEN_BI1_V3_MANIFEST_DIGEST
        || value.sealed_global_stage4_cone_digest != FROZEN_BI1_V3_STAGE4_CONE_DIGEST
        || !value.hard_regression_passed
        || !value.exact_proved_rows_reproduced
        || !value.exact_theorem_impossibility_rows_reproduced
        || !value.zero_residuals_reproduced
        || !value.one_prefix_parametric_procedure
        || value.family_ids_exported
        || value.semantic_vectors_exported
        || value.enacted_winners_exported
        || value.enacted_outcomes_exported
        || value.selector_capability_exported
    {
        errors.push("BI-1b regression authorization is not closed and row-free".to_owned());
    }
    let mut branch_digests = value.authorized_sealed_branch_certificate_digests.clone();
    let original_len = branch_digests.len();
    branch_digests.sort();
    branch_digests.dedup();
    if original_len != 4
        || branch_digests.len() != 4
        || branch_digests != value.authorized_sealed_branch_certificate_digests
        || branch_digests
            .iter()
            .any(|digest| !is_blake3_digest(digest))
    {
        errors.push(
            "BI-1b regression authorization does not bind four sorted unique branch digests"
                .to_owned(),
        );
    }
    errors
}

fn regression_digest(value: &Bi1bRegressionCertificateV1) -> String {
    let mut projection = value.clone();
    projection.result_digest.clear();
    tagged_hash(BI1B_REGRESSION_V1_SCHEMA, &projection)
}

fn branch_digest(value: &Bi1bBranchCertificateV1) -> String {
    let mut projection = value.clone();
    projection.result_digest.clear();
    tagged_hash(BI1B_BRANCH_V1_SCHEMA, &projection)
}

fn correspondence_row_digest(value: &Bi1bCorrespondenceRowV1) -> String {
    let mut projection = value.clone();
    projection.derivation_hash.clear();
    tagged_hash("correspondence-row", &projection)
}

fn correspondence_digest(value: &Bi1bCorrespondenceDiagnosticV1) -> String {
    let mut projection = value.clone();
    projection.derivation_hash.clear();
    tagged_hash(BI1B_CORRESPONDENCE_V1_SCHEMA, &projection)
}

fn sweep_row_digest(value: &Bi1bSweepBranchRowV1) -> String {
    let mut projection = value.clone();
    projection.row_hash.clear();
    tagged_hash("sweep-branch-row", &projection)
}

fn sweep_digest(value: &Bi1bPrefixGeneralSweepV1Certificate) -> String {
    let mut projection = value.clone();
    projection.result_digest.clear();
    tagged_hash("sweep-certificate", &projection)
}

fn parse_json<T: for<'de> Deserialize<'de>>(
    label: &str,
    bytes: &[u8],
) -> Result<T, Bi1bPrefixGeneralSweepV1Error> {
    serde_json::from_slice(bytes)
        .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Json(format!("{label}: {error}")))
}

fn sealed_bi1_bundle() -> Result<Bi1OptionASweepBundleV3, Bi1bPrefixGeneralSweepV1Error> {
    let manifest = parse_json("sealed BI-1 v3 manifest", SEALED_SWEEP_BYTES)?;
    let mut branches = vec![
        parse_json("sealed enacted BI-1 branch", SEALED_ENACTED_BYTES)?,
        parse_json("sealed alternate-A BI-1 branch", SEALED_ALT_A_BYTES)?,
        parse_json("sealed alternate-B BI-1 branch", SEALED_ALT_B_BYTES)?,
        parse_json("sealed alternate-C BI-1 branch", SEALED_ALT_C_BYTES)?,
    ];
    branches.sort_by_key(|branch: &Bi1BranchCertificateV3| branch.issuance_ordinal);
    Ok(Bi1OptionASweepBundleV3 { branches, manifest })
}

fn sealed_content_errors(bundle: &Bi1OptionASweepBundleV3) -> Vec<String> {
    let mut errors = Vec::new();
    if bundle.manifest.schema != BI1_OPTION_A_SWEEP_V3_SCHEMA
        || bundle.manifest.result_digest != FROZEN_BI1_V3_MANIFEST_DIGEST
    {
        errors.push("sealed BI-1 manifest is not the preregistered v3 testimony".to_owned());
    }
    if bundle.manifest.result_digest != old_manifest_digest(&bundle.manifest) {
        errors.push("sealed BI-1 manifest digest mismatch".to_owned());
    }
    if bundle.manifest.enacted_regression.derivation_hash
        != old_regression_digest(&bundle.manifest.enacted_regression)
    {
        errors.push("sealed BI-1 enacted regression digest mismatch".to_owned());
    }
    if bundle
        .manifest
        .branch_rows
        .iter()
        .any(|row| row.row_hash != old_manifest_row_digest(row))
    {
        errors.push("sealed BI-1 manifest row digest mismatch".to_owned());
    }
    if bundle.branches.iter().any(|branch| {
        branch.schema != BI1_BRANCH_V3_SCHEMA
            || branch.result_digest != old_branch_digest(branch)
            || branch
                .guarded_stages
                .iter()
                .any(|row| row.derivation_hash != old_guarded_stage_digest(row))
    }) {
        errors.push("sealed BI-1 branch certificate digest mismatch".to_owned());
    }
    if bundle.branches.len() != 4
        || bundle.manifest.branch_rows.len() != 4
        || bundle.manifest.branch_count != 4
        || !bundle.manifest.exact_four_root_surface
    {
        errors.push("sealed BI-1 four-branch surface is not exact".to_owned());
    }
    let observed_bindings = bundle
        .branches
        .iter()
        .map(|branch| {
            (
                branch.branch_root_hash.as_str(),
                branch.result_digest.as_str(),
            )
        })
        .collect::<Vec<_>>();
    if observed_bindings != FROZEN_BI1_V3_BRANCH_BINDINGS {
        errors.push(
            "sealed BI-1 roots/certificates differ from the preregistered four bindings".to_owned(),
        );
    }
    if bundle.manifest.cone_digest != FROZEN_BI1_V3_STAGE4_CONE_DIGEST
        || bundle
            .branches
            .iter()
            .any(|branch| branch.cone_digest != bundle.manifest.cone_digest)
    {
        errors.push("sealed BI-1 branches are not bound to the one frozen Stage-4 cone".to_owned());
    }
    if bundle.branches.iter().any(|branch| {
        branch.branch_root_hash != branch.continuation.branch.candidate_hash
            || branch.branch_digest_prefix != digest_prefix(&branch.branch_root_hash)
            || branch.option_a_execution_capability_digest
                != bundle.manifest.option_a_execution_capability_digest
    }) {
        errors.push("sealed BI-1 branch root/seed/capability binding mismatch".to_owned());
    }
    if bundle
        .branches
        .iter()
        .map(|branch| branch.issuance_ordinal)
        .collect::<Vec<_>>()
        != vec![0, 1, 2, 3]
        || bundle
            .branches
            .iter()
            .filter(|branch| branch.enacted_root)
            .count()
            != 1
        || bundle
            .branches
            .iter()
            .find(|branch| branch.enacted_root)
            .is_none_or(|branch| branch.branch_root_hash != bundle.manifest.enacted_root_hash)
    {
        errors.push("sealed BI-1 issuance/enacted-root binding mismatch".to_owned());
    }
    for branch in &bundle.branches {
        let expected_row = expected_old_manifest_row(branch);
        if bundle
            .manifest
            .branch_rows
            .iter()
            .filter(|row| **row == expected_row)
            .count()
            != 1
        {
            errors.push(format!(
                "sealed BI-1 branch {} does not have one exact full manifest-row projection",
                branch.branch_root_hash
            ));
        }
    }
    let enacted = bundle.branches.iter().find(|branch| branch.enacted_root);
    if bundle.manifest.enacted_regression.schema != BI1_ENACTED_REGRESSION_V3_SCHEMA
        || bundle
            .manifest
            .enacted_regression
            .option_a_execution_capability_digest
            != bundle.manifest.option_a_execution_capability_digest
        || !bundle.manifest.enacted_regression.enacted_regression_passed
        || bundle
            .manifest
            .enacted_regression
            .enacted_outcome_vectors_exported
        || enacted.is_none_or(|branch| {
            bundle.manifest.enacted_regression.enacted_root_hash != branch.branch_root_hash
                || bundle
                    .manifest
                    .enacted_regression
                    .enacted_branch_certificate_digest
                    != branch.result_digest
                || branch.enacted_regression_authorization_digest.is_some()
        })
        || bundle
            .branches
            .iter()
            .filter(|branch| !branch.enacted_root)
            .any(|branch| {
                branch.enacted_regression_authorization_digest.as_deref()
                    != Some(bundle.manifest.enacted_regression.derivation_hash.as_str())
            })
    {
        errors.push("sealed BI-1 enacted-regression/certificate binding mismatch".to_owned());
    }
    if !bundle
        .manifest
        .option_a_capability_replayed_before_any_branch
        || !bundle.manifest.option_a_real_choice_preserved
        || !bundle.manifest.cone_replayed_before_any_branch
        || !bundle.manifest.enacted_branch_issued_first
        || !bundle
            .manifest
            .enacted_regression_passed_before_non_enacted_issuance
        || !bundle
            .manifest
            .all_branch_certificates_independently_replayed
        || bundle.manifest.non_enacted_branch_count != 3
        || bundle
            .manifest
            .non_enacted_branches_consumed_enacted_outcomes
        || bundle.manifest.value_or_bar_used_as_selector
        || bundle.manifest.divergence_suppression_or_repair_performed
        || bundle.manifest.class_representative_substitution_used
    {
        errors.push("sealed BI-1 issuance-order/capability boundary drifted".to_owned());
    }
    if !bundle.manifest.f_s4_3_preserved
        || !bundle.manifest.f_bi1_preserved
        || !bundle.manifest.f_bi2_preserved
        || !bundle.manifest.f_bi3_preserved
        || !bundle.manifest.f_bi4_preserved
        || !bundle.manifest.f_bi5_preserved
        || !bundle.manifest.f_bi6_preserved
        || bundle.manifest.cross_branch_g2_verdict_issued
        || bundle.manifest.cross_branch_g3_verdict_issued
        || bundle.manifest.bi2_finale_issued
        || bundle.manifest.bi4_cone_report_issued
        || bundle.manifest.unindexed_halt_ledger_or_o_claim_issued
    {
        errors.push("sealed BI-1 scope/falsifier surface drifted".to_owned());
    }
    errors
}

fn issue_sealed_sweep_preflight(
    sealed: &Bi1OptionASweepBundleV3,
) -> Result<Bi1bSealedSweepPreflightV1, Bi1bPrefixGeneralSweepV1Error> {
    let public_replay = replay_bi1_option_a_sweep_v3(sealed);
    let public_v3_replay_passed = public_replay.valid;
    let sealed_errors = sealed_content_errors(sealed);
    let frozen_manifest_digest_exact =
        sealed.manifest.result_digest == FROZEN_BI1_V3_MANIFEST_DIGEST;
    let frozen_branch_root_and_certificate_bindings_exact = sealed
        .branches
        .iter()
        .map(|branch| {
            (
                branch.branch_root_hash.as_str(),
                branch.result_digest.as_str(),
            )
        })
        .collect::<Vec<_>>()
        == FROZEN_BI1_V3_BRANCH_BINDINGS;
    let common_stage4_cone_binding_exact = sealed.manifest.cone_digest
        == FROZEN_BI1_V3_STAGE4_CONE_DIGEST
        && sealed
            .branches
            .iter()
            .all(|branch| branch.cone_digest == sealed.manifest.cone_digest);
    let every_branch_root_matches_its_sealed_seed = sealed
        .branches
        .iter()
        .all(|branch| branch.branch_root_hash == branch.continuation.branch.candidate_hash);
    let exactly_one_enacted_root_cross_bound = sealed
        .branches
        .iter()
        .filter(|branch| branch.enacted_root)
        .count()
        == 1
        && sealed
            .branches
            .iter()
            .find(|branch| branch.enacted_root)
            .is_some_and(|branch| branch.branch_root_hash == sealed.manifest.enacted_root_hash);
    let sealed_manifest_self_digest_valid =
        sealed.manifest.result_digest == old_manifest_digest(&sealed.manifest);
    let every_manifest_row_self_digest_valid = sealed
        .manifest
        .branch_rows
        .iter()
        .all(|row| row.row_hash == old_manifest_row_digest(row));
    let every_branch_certificate_self_digest_valid = sealed
        .branches
        .iter()
        .all(|branch| branch.result_digest == old_branch_digest(branch));
    let every_manifest_branch_cross_binding_exact = sealed.branches.iter().all(|branch| {
        let expected_row = expected_old_manifest_row(branch);
        sealed
            .manifest
            .branch_rows
            .iter()
            .filter(|row| **row == expected_row)
            .count()
            == 1
    });
    let sealed_content_preflight_passed = sealed_errors.is_empty()
        && frozen_manifest_digest_exact
        && frozen_branch_root_and_certificate_bindings_exact
        && common_stage4_cone_binding_exact
        && every_branch_root_matches_its_sealed_seed
        && exactly_one_enacted_root_cross_bound
        && sealed_manifest_self_digest_valid
        && every_manifest_row_self_digest_valid
        && every_branch_certificate_self_digest_valid
        && every_manifest_branch_cross_binding_exact;
    if !sealed_content_preflight_passed {
        let mut messages = sealed_errors;
        if !public_replay.valid {
            messages.push(format!(
                "public v3 replay also failed (recorded, not used as frozen-testimony authority): {}",
                public_replay.errors.join("; ")
            ));
        }
        return Err(Bi1bPrefixGeneralSweepV1Error::SealedInput(format!(
            "sealed BI-1 v3 preflight failed: {}",
            messages.join("; ")
        )));
    }

    Ok(Bi1bSealedSweepPreflightV1 {
        sealed_sweep_digest: sealed.manifest.result_digest.clone(),
        sealed_stage4_cone_digest: sealed.manifest.cone_digest.clone(),
        sealed_branch_certificate_digests: sealed
            .branches
            .iter()
            .map(|branch| branch.result_digest.clone())
            .collect(),
        public_v3_replay_passed,
        public_v3_replay_errors: public_replay.errors,
        sealed_testimony_fallback_used: !public_v3_replay_passed,
        frozen_manifest_digest_exact,
        frozen_branch_root_and_certificate_bindings_exact,
        common_stage4_cone_binding_exact,
        every_branch_root_matches_its_sealed_seed,
        exactly_one_enacted_root_cross_bound,
        sealed_manifest_self_digest_valid,
        every_manifest_row_self_digest_valid,
        every_branch_certificate_self_digest_valid,
        every_manifest_branch_cross_binding_exact,
        sealed_content_preflight_passed,
    })
}

fn predecessor_entries(
    branch: &Bi1BranchCertificateV3,
    stage: u32,
) -> Result<Vec<(u32, Telescope)>, Bi1bPrefixGeneralSweepV1Error> {
    if stage < 5 {
        return Err(Bi1bPrefixGeneralSweepV1Error::Invariant(
            "BI-1b predecessor reconstruction starts after the Stage-4 fork".to_owned(),
        ));
    }
    let mut entries = (1..=3)
        .map(|entry_stage| (entry_stage, Telescope::reference(entry_stage)))
        .collect::<Vec<_>>();
    entries.push((4, branch.continuation.branch.telescope.clone()));
    for entry_stage in 5..stage {
        let winner = branch
            .continuation
            .stages
            .iter()
            .find(|row| row.stage == entry_stage)
            .and_then(|row| row.winner.as_ref())
            .ok_or_else(|| {
                Bi1bPrefixGeneralSweepV1Error::Invariant(format!(
                    "sealed branch {} has no Stage-{entry_stage} winner",
                    branch.branch_root_hash
                ))
            })?;
        entries.push((entry_stage, winner.telescope.clone()));
    }
    Ok(entries)
}

fn issue_regression(
    sealed: &Bi1OptionASweepBundleV3,
    preflight: Bi1bSealedSweepPreflightV1,
) -> Result<Bi1bRegressionCertificateV1, Bi1bPrefixGeneralSweepV1Error> {
    let authority = issue_branch_prefix_general_provenance_v4_authority();
    let authority_errors = replay_branch_prefix_general_provenance_v4_authority(&authority);
    let extraction_authority_replayed = authority_errors.is_empty() && authority.proved;
    if !extraction_authority_replayed {
        return Err(Bi1bPrefixGeneralSweepV1Error::Regression(format!(
            "prefix-general extraction authority failed replay: {}",
            authority_errors.join("; ")
        )));
    }

    let enacted = sealed
        .branches
        .iter()
        .find(|branch| branch.enacted_root)
        .ok_or_else(|| {
            Bi1bPrefixGeneralSweepV1Error::Regression(
                "sealed sweep has no enacted branch".to_owned(),
            )
        })?;
    let stage8 = enacted
        .continuation
        .stages
        .iter()
        .find(|row| row.stage == 8)
        .ok_or_else(|| {
            Bi1bPrefixGeneralSweepV1Error::Regression(
                "sealed enacted branch has no Stage-8 window".to_owned(),
            )
        })?;
    if stage8.assessments.len() != 1 {
        return Err(Bi1bPrefixGeneralSweepV1Error::Regression(
            "enacted Stage-8 hard regression requires its exact singleton cone".to_owned(),
        ));
    }
    let candidate = &stage8.assessments[0];
    let entries = predecessor_entries(enacted, 8)?;
    let prefix_general =
        issue_branch_prefix_general_provenance_v4(&entries, 8, &candidate.telescope)
            .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Regression(error.to_string()))?;
    let prefix_replay_errors =
        replay_branch_prefix_general_provenance_v4(&entries, &candidate.telescope, &prefix_general);
    let prefix_general_stage8_token_replayed = prefix_replay_errors.is_empty()
        && validate_branch_prefix_general_provenance_v4_token_integrity(&prefix_general).is_empty();
    if !prefix_general_stage8_token_replayed {
        return Err(Bi1bPrefixGeneralSweepV1Error::Regression(format!(
            "prefix-general enacted Stage-8 token failed replay: {}",
            prefix_replay_errors.join("; ")
        )));
    }

    let t_bi: TBiNu1RegressionV6Certificate =
        parse_json("sealed T-BI v6 certificate", SEALED_T_BI_V6_BYTES)?;
    let t_bi_replay = replay_t_bi_nu1_regression_v6_certificate(&t_bi);
    let t_bi_v6_frozen_digest_exact = t_bi.result_digest == FROZEN_T_BI_V6_CERTIFICATE_DIGEST;
    let t_bi_v6_self_digest_valid = t_bi.result_digest == frozen_t_bi_v6_certificate_digest(&t_bi);
    let t_bi_v6_public_deterministic_replay_passed = t_bi_replay.valid;
    let t_bi_v6_postseal_reissuance_drift_only = !t_bi_replay.valid
        && t_bi_replay.errors == vec![T_BI_V6_CURRENT_REISSUANCE_DRIFT.to_owned()];
    let t_bi_v6_frozen_testimony_authenticated = t_bi_v6_frozen_digest_exact
        && t_bi_v6_self_digest_valid
        && t_bi.schema == T_BI_NU1_REGRESSION_V6_SCHEMA
        && t_bi.package_count == 15
        && t_bi.intrinsic_sequence.packages.len() == 15
        && t_bi.extraction_complete
        && t_bi.t_bi_b1_proved
        && t_bi.t_bi_b2_proved
        && t_bi.t_bi_b3_proved
        && t_bi.special_cases_proved
        && t_bi.f_al1_prime_passed
        && t_bi.t_bi_nu1_proved_on_enacted_branch
        && !t_bi.non_enacted_branch_work_executed
        && (t_bi_v6_public_deterministic_replay_passed || t_bi_v6_postseal_reissuance_drift_only);
    if !t_bi_v6_frozen_testimony_authenticated {
        return Err(Bi1bPrefixGeneralSweepV1Error::Regression(format!(
            "sealed T-BI v6 testimony failed authentication: public replay: {}",
            t_bi_replay.errors.join("; ")
        )));
    }
    let current_t_bi = issue_t_bi_nu1_regression_v6_certificate()
        .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Regression(error.to_string()))?;
    let current_t_bi_v6_global_semantic_surface_matches_frozen = current_t_bi.package_count
        == t_bi.package_count
        && current_t_bi.role_declaration_count == t_bi.role_declaration_count
        && current_t_bi.proved_family_declaration_count == t_bi.proved_family_declaration_count
        && current_t_bi.theorem_impossibility_declaration_count
            == t_bi.theorem_impossibility_declaration_count
        && current_t_bi.total_named_residual_count == t_bi.total_named_residual_count
        && current_t_bi.silent_residue_count == t_bi.silent_residue_count
        && current_t_bi.authoritative_semantic_register == t_bi.authoritative_semantic_register
        && current_t_bi.extraction_complete == t_bi.extraction_complete
        && current_t_bi.f_al1_prime_passed == t_bi.f_al1_prime_passed
        && current_t_bi.non_enacted_branch_work_executed == t_bi.non_enacted_branch_work_executed;
    let v6_package = t_bi
        .intrinsic_sequence
        .packages
        .iter()
        .find(|package| package.stage == 8)
        .ok_or_else(|| {
            Bi1bPrefixGeneralSweepV1Error::Regression(
                "T-BI v6 contains no Stage-8 semantic package".to_owned(),
            )
        })?;
    let current_v6_package = current_t_bi
        .intrinsic_sequence
        .packages
        .iter()
        .find(|package| package.stage == 8)
        .ok_or_else(|| {
            Bi1bPrefixGeneralSweepV1Error::Regression(
                "current T-BI v6 reissuance contains no Stage-8 semantic package".to_owned(),
            )
        })?;
    if v6_package.candidate_hash != candidate.candidate_hash {
        return Err(Bi1bPrefixGeneralSweepV1Error::Regression(
            "T-BI v6 Stage-8 candidate differs from the sealed enacted cone".to_owned(),
        ));
    }

    let (mut v6_proved_rows, mut v6_theorem_impossibility_rows) =
        project_branch_prefix_general_role_classifications_v4(v6_package)
            .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Regression(error.to_string()))?;
    let (mut current_v6_proved_rows, mut current_v6_theorem_impossibility_rows) =
        project_branch_prefix_general_role_classifications_v4(current_v6_package)
            .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Regression(error.to_string()))?;
    let mut prefix_general_proved_rows = prefix_general.proved_family_rows.clone();
    let mut prefix_general_theorem_impossibility_rows =
        prefix_general.theorem_impossibility_rows.clone();
    for rows in [
        &mut v6_proved_rows,
        &mut v6_theorem_impossibility_rows,
        &mut current_v6_proved_rows,
        &mut current_v6_theorem_impossibility_rows,
        &mut prefix_general_proved_rows,
        &mut prefix_general_theorem_impossibility_rows,
    ] {
        rows.sort_by(|left, right| left.declaration_id.cmp(&right.declaration_id));
    }
    let current_t_bi_v6_stage8_semantic_projection_matches_frozen =
        current_v6_package.candidate_hash == v6_package.candidate_hash
            && current_v6_package.credited_family_ids == v6_package.credited_family_ids
            && current_v6_package.role_declaration_count == v6_package.role_declaration_count
            && current_v6_package.proved_family_declaration_count
                == v6_package.proved_family_declaration_count
            && current_v6_package.theorem_impossibility_declaration_count
                == v6_package.theorem_impossibility_declaration_count
            && current_v6_package.semantic_nu == v6_package.semantic_nu
            && current_v6_package.named_role_residual_count == v6_package.named_role_residual_count
            && current_v6_package.named_quotient_residual_count
                == v6_package.named_quotient_residual_count
            && current_v6_package.named_a3_residual_count == v6_package.named_a3_residual_count
            && current_v6_package.silent_residue_count == v6_package.silent_residue_count
            && current_v6_proved_rows == v6_proved_rows
            && current_v6_theorem_impossibility_rows == v6_theorem_impossibility_rows;

    let exact_family_ids_reproduced =
        v6_package.credited_family_ids == prefix_general.credited_family_ids;
    let exact_proved_rows_reproduced = v6_proved_rows == prefix_general_proved_rows;
    let exact_theorem_impossibility_rows_reproduced =
        v6_theorem_impossibility_rows == prefix_general_theorem_impossibility_rows;
    let exact_role_and_semantic_totals_reproduced = v6_package.role_declaration_count
        == prefix_general.role_declaration_count
        && v6_package.proved_family_declaration_count
            == prefix_general.proved_family_declaration_count
        && v6_package.theorem_impossibility_declaration_count
            == prefix_general.theorem_impossibility_declaration_count
        && v6_package.semantic_nu == prefix_general.semantic_nu;
    let old_evidence = candidate
        .nu_provenance
        .semantic_family_evidence_v3
        .as_ref()
        .ok_or_else(|| {
            Bi1bPrefixGeneralSweepV1Error::Regression(
                "sealed enacted Stage-8 assessment lacks v3 semantic evidence".to_owned(),
            )
        })?;
    let exact_r2_regression_reproduced =
        old_evidence.r2_premise.r2_generated_instance_removed_count
            == prefix_general.r2_generated_instance_removed_count
            && old_evidence.r2_premise.r2_removed_occurrence_hashes
                == prefix_general.r2_removed_occurrence_hashes
            && old_evidence.r2_premise.r2_local_premises_replayed
            && old_evidence
                .r2_premise
                .r2_removed_occurrences_absent_from_unified_membership
            && old_evidence
                .r2_premise
                .r2_removed_occurrences_not_exported_as_families
            && old_evidence.r2_premise.r2_generated_instance_not_multiplied
            && prefix_general.r2_local_premises_replayed;
    let r2_proof_hashes_recorded_as_separate_evidence = old_evidence
        .r2_premise
        .r2_step8_typed_signature_derivation_hash
        .as_ref()
        .is_some_and(|hash| !hash.is_empty())
        && prefix_general
            .r2_step8_typed_signature_derivation_hash
            .as_ref()
            .is_some_and(|hash| !hash.is_empty())
        && old_evidence
            .r2_premise
            .r2_m1_generated_membership_derivation_hash
            .as_ref()
            .is_some_and(|hash| !hash.is_empty())
        && prefix_general
            .r2_m1_generated_membership_derivation_hash
            .as_ref()
            .is_some_and(|hash| !hash.is_empty());
    let zero_residuals_reproduced = v6_package.named_role_residual_count == 0
        && v6_package.named_quotient_residual_count == 0
        && v6_package.named_a3_residual_count == 0
        && v6_package.silent_residue_count == 0
        && prefix_general.all_residual_counts_zero;
    let one_procedure_digest =
        prefix_general.extraction_procedure_digest == authority.derivation_hash;
    let f_b1b_1_passed = preflight.sealed_content_preflight_passed
        && t_bi_v6_frozen_testimony_authenticated
        && current_t_bi_v6_global_semantic_surface_matches_frozen
        && current_t_bi_v6_stage8_semantic_projection_matches_frozen
        && extraction_authority_replayed
        && prefix_general_stage8_token_replayed
        && exact_family_ids_reproduced
        && exact_proved_rows_reproduced
        && exact_theorem_impossibility_rows_reproduced
        && exact_role_and_semantic_totals_reproduced
        && exact_r2_regression_reproduced
        && r2_proof_hashes_recorded_as_separate_evidence
        && zero_residuals_reproduced
        && one_procedure_digest;
    if !f_b1b_1_passed {
        return Err(Bi1bPrefixGeneralSweepV1Error::Regression(format!(
            "F-B1b-1 hard gate failed: current-global={current_t_bi_v6_global_semantic_surface_matches_frozen}, current-Stage8={current_t_bi_v6_stage8_semantic_projection_matches_frozen}, families={exact_family_ids_reproduced}, proved={exact_proved_rows_reproduced}, impossible={exact_theorem_impossibility_rows_reproduced}, totals={exact_role_and_semantic_totals_reproduced}, R2={exact_r2_regression_reproduced}, zero={zero_residuals_reproduced}, one-procedure={one_procedure_digest}"
        )));
    }

    let mut authorized_sealed_branch_certificate_digests =
        preflight.sealed_branch_certificate_digests.clone();
    authorized_sealed_branch_certificate_digests.sort();
    authorized_sealed_branch_certificate_digests.dedup();
    if authorized_sealed_branch_certificate_digests.len() != 4
        || authorized_sealed_branch_certificate_digests
            != sealed
                .branches
                .iter()
                .map(|branch| branch.result_digest.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
    {
        return Err(Bi1bPrefixGeneralSweepV1Error::Regression(
            "preflight did not bind the exact four sealed branch certificate digests".to_owned(),
        ));
    }
    let mut authorization = Bi1bRegressionAuthorizationV1 {
        schema: BI1B_REGRESSION_AUTHORIZATION_V1_SCHEMA.to_owned(),
        sealed_global_bi1_sweep_digest: sealed.manifest.result_digest.clone(),
        sealed_global_stage4_cone_digest: sealed.manifest.cone_digest.clone(),
        sealed_global_bi1_sweep_content_authenticated: preflight.sealed_content_preflight_passed,
        extraction_procedure_digest: authority.derivation_hash.clone(),
        authorized_sealed_branch_certificate_digests,
        hard_regression_passed: true,
        exact_proved_rows_reproduced,
        exact_theorem_impossibility_rows_reproduced,
        zero_residuals_reproduced,
        one_prefix_parametric_procedure: true,
        family_ids_exported: false,
        semantic_vectors_exported: false,
        enacted_winners_exported: false,
        enacted_outcomes_exported: false,
        selector_capability_exported: false,
        derivation_hash: String::new(),
    };
    authorization.derivation_hash = regression_authorization_digest(&authorization);
    let authorization_replayed = regression_authorization_errors(&authorization).is_empty();
    if !authorization_replayed {
        return Err(Bi1bPrefixGeneralSweepV1Error::Regression(
            "row-free regression authorization failed replay before sealing".to_owned(),
        ));
    }

    let mut certificate = Bi1bRegressionCertificateV1 {
        schema: BI1B_REGRESSION_V1_SCHEMA.to_owned(),
        date: BI1B_PREFIX_GENERAL_SWEEP_V1_DATE.to_owned(),
        preflight,
        extraction_authority: authority,
        extraction_authority_replayed,
        enacted_bi1_branch_certificate_digest: enacted.result_digest.clone(),
        enacted_stage8_candidate_hash: candidate.candidate_hash.clone(),
        t_bi_v6_certificate_digest: t_bi.result_digest.clone(),
        t_bi_v6_frozen_digest_exact,
        t_bi_v6_self_digest_valid,
        t_bi_v6_public_deterministic_replay_passed,
        t_bi_v6_public_replay_errors: t_bi_replay.errors,
        t_bi_v6_postseal_reissuance_drift_only,
        t_bi_v6_frozen_testimony_authenticated,
        current_t_bi_v6_reissuance_digest: current_t_bi.result_digest.clone(),
        current_t_bi_v6_global_semantic_surface_matches_frozen,
        current_t_bi_v6_stage8_semantic_projection_matches_frozen,
        t_bi_v6_stage8_package_derivation_hash: v6_package.derivation_hash.clone(),
        v6_credited_family_ids: v6_package.credited_family_ids.clone(),
        prefix_general_credited_family_ids: prefix_general.credited_family_ids.clone(),
        v6_proved_rows,
        prefix_general_proved_rows,
        v6_theorem_impossibility_rows,
        prefix_general_theorem_impossibility_rows,
        prefix_general_stage8_token_replayed,
        exact_family_ids_reproduced,
        exact_proved_rows_reproduced,
        exact_theorem_impossibility_rows_reproduced,
        exact_role_and_semantic_totals_reproduced,
        v6_r2_typed_signature_derivation_hash: old_evidence
            .r2_premise
            .r2_step8_typed_signature_derivation_hash
            .clone(),
        prefix_general_r2_typed_signature_derivation_hash: prefix_general
            .r2_step8_typed_signature_derivation_hash
            .clone(),
        v6_r2_m1_membership_derivation_hash: old_evidence
            .r2_premise
            .r2_m1_generated_membership_derivation_hash
            .clone(),
        prefix_general_r2_m1_membership_derivation_hash: prefix_general
            .r2_m1_generated_membership_derivation_hash
            .clone(),
        prefix_general_stage8_token: prefix_general,
        r2_proof_hashes_recorded_as_separate_evidence,
        exact_r2_regression_reproduced,
        zero_residuals_reproduced,
        one_procedure_digest,
        f_b1b_1_passed,
        authorization,
        authorization_replayed,
        alternate_branch_work_executed_before_regression_sealed: false,
        bi2_finale_issued: false,
        bi4_cone_verdict_issued: false,
        g2_verdict_issued: false,
        g3_verdict_issued: false,
        uc1_scored: false,
        unindexed_claim_issued: false,
        result_digest: String::new(),
    };
    certificate.result_digest = regression_digest(&certificate);
    Ok(certificate)
}

fn outcome_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("BI-1b outcome serializes")
}

fn issue_branch_certificate(
    sealed: &Bi1BranchCertificateV3,
    regression_authorization: &Bi1bRegressionAuthorizationV1,
    issuance_ordinal: usize,
) -> Result<Bi1bBranchCertificateV1, Bi1bPrefixGeneralSweepV1Error> {
    if !regression_authorization_errors(regression_authorization).is_empty() {
        return Err(Bi1bPrefixGeneralSweepV1Error::Resume(
            "narrow enacted-regression authorization failed self-digest replay before mint"
                .to_owned(),
        ));
    }
    let authorization = issue_bi1b_sealed_window_authorization_v1(sealed, regression_authorization)
        .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Resume(error.to_string()))?;
    let authorization_errors = replay_bi1b_sealed_window_authorization_v1(sealed, &authorization);
    let sealed_window_authorization_replayed = authorization_errors.is_empty();
    if !sealed_window_authorization_replayed {
        return Err(Bi1bPrefixGeneralSweepV1Error::Resume(format!(
            "branch authorization failed replay: {}",
            authorization_errors.join("; ")
        )));
    }

    let limits = BranchContinuationLimits {
        max_inspected_stage: BI1B_MAX_INSPECTED_STAGE,
        max_enumerated_candidates_per_stage: BI1B_MAX_ENUMERATED_CANDIDATES_PER_STAGE,
    };
    let (resume, resume_replayed, prior_enacted_halt_revalidated, terminal_disposition) = if sealed
        .enacted_root
    {
        let prior_halt_revalidated = matches!(
            sealed.continuation.outcome,
            BranchContinuationOutcome::DebtFreeHalt {
                halt_stage: 15,
                next_stage: 16
            }
        ) && sealed.continuation.completed_through_stage15
            && sealed.continuation.debt_free_halt_at_stage15
            && sealed.continuation.bar_never_used_as_gate_or_selector
            && sealed.continuation.score_never_used_as_selector
            && !sealed
                .continuation
                .hash_or_enumeration_order_used_as_selector
            && !sealed.continuation.resource_limit_used_as_halt_claim;
        if !prior_halt_revalidated {
            return Err(Bi1bPrefixGeneralSweepV1Error::Resume(
                "sealed enacted branch no longer revalidates its prior Stage-15 halt".to_owned(),
            ));
        }
        (
            None,
            false,
            true,
            "prior_enacted_debt_free_halt_at_stage15_revalidated".to_owned(),
        )
    } else {
        let issued = issue_branch_invariance_resume_v1(sealed, &authorization, &limits)
            .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Resume(error.to_string()))?;
        let replay = replay_branch_invariance_resume_v1(sealed, &authorization, &issued);
        if !replay.valid {
            return Err(Bi1bPrefixGeneralSweepV1Error::Resume(format!(
                "branch {} failed deterministic resume replay: {}",
                sealed.branch_root_hash,
                replay.errors.join("; ")
            )));
        }
        let disposition = outcome_json(&issued.outcome);
        (Some(issued), true, false, disposition)
    };

    let every_completed_census_followed_total_classification = resume.as_ref().map_or_else(
        || {
            sealed
                .guarded_stages
                .iter()
                .all(|stage| stage.candidate_semantic_gaps_absent)
        },
        |issued| {
            issued.every_completed_census_followed_total_classification
                && issued.stages.iter().all(|stage| {
                    stage.census.is_none() || stage.every_candidate_classified_before_census
                })
        },
    );
    let unknown_never_counted_as_discharger_or_exclusion = resume.as_ref().map_or_else(
        || {
            sealed
                .guarded_stages
                .iter()
                .all(|stage| stage.candidate_semantic_gaps_absent)
        },
        |issued| issued.unknown_never_counted_as_discharger_or_exclusion,
    );
    let one_extraction_procedure_digest = resume.as_ref().is_none_or(|issued| {
        issued.exactly_one_extraction_procedure_authorized
            && issued.every_observed_extraction_procedure_matches_authorization
            && issued.authorized_extraction_procedure_digest
                == regression_authorization.extraction_procedure_digest
    });
    let lawful_branch_disposition = resume.as_ref().is_none_or(|issued| {
        issued.branch_lawfully_disposed
            && !matches!(
                issued.outcome,
                Bi1bResumeOutcomeV1::ResourceLimitReached { .. }
            )
            && issued
                .stages
                .iter()
                .all(|stage| stage.selection_or_stop_is_lawful)
            && issued.semantic_value_never_used_as_selector
            && issued.bar_never_used_as_selector
            && issued.hash_or_enumeration_order_never_used_as_selector
            && issued.no_enacted_classification_or_outcome_accepted_as_input
            && issued.byte_identity_never_used_as_provenance_or_selector
            && !issued.resource_limit_used_as_halt_claim
            && issued.branch_indexed_only
            && !issued.bi2_finale_issued
            && !issued.bi4_cone_verdict_issued
            && !issued.uc1_scored
            && !issued.unindexed_claim_issued
    }) && prior_enacted_halt_revalidated == sealed.enacted_root;
    let enacted_classifications_or_outcomes_consumed = resume
        .as_ref()
        .is_some_and(|issued| !issued.no_enacted_classification_or_outcome_accepted_as_input);
    let byte_identity_consumed_as_provenance_or_selector = resume
        .as_ref()
        .is_some_and(|issued| !issued.byte_identity_never_used_as_provenance_or_selector);
    let semantic_value_used_as_selector = resume.as_ref().is_some_and(|issued| {
        !issued.semantic_value_never_used_as_selector
            || issued
                .stages
                .iter()
                .any(|stage| stage.semantic_value_used_as_selector)
    });
    let bar_used_as_selector = resume.as_ref().is_some_and(|issued| {
        !issued.bar_never_used_as_selector
            || issued
                .stages
                .iter()
                .any(|stage| stage.bar_used_as_gate_or_selector)
    });
    let hash_or_enumeration_order_used_as_selector = resume
        .as_ref()
        .is_some_and(|issued| !issued.hash_or_enumeration_order_never_used_as_selector);
    if !lawful_branch_disposition
        || !every_completed_census_followed_total_classification
        || !unknown_never_counted_as_discharger_or_exclusion
        || !one_extraction_procedure_digest
        || enacted_classifications_or_outcomes_consumed
        || byte_identity_consumed_as_provenance_or_selector
        || semantic_value_used_as_selector
        || bar_used_as_selector
        || hash_or_enumeration_order_used_as_selector
    {
        return Err(Bi1bPrefixGeneralSweepV1Error::Resume(format!(
            "branch {} failed the BI-1b lawful-disposition boundary",
            sealed.branch_root_hash
        )));
    }

    let mut certificate = Bi1bBranchCertificateV1 {
        schema: BI1B_BRANCH_V1_SCHEMA.to_owned(),
        date: BI1B_PREFIX_GENERAL_SWEEP_V1_DATE.to_owned(),
        branch_root_hash: sealed.branch_root_hash.clone(),
        branch_digest_prefix: digest_prefix(&sealed.branch_root_hash),
        enacted_root: sealed.enacted_root,
        issuance_ordinal,
        sealed_bi1_certificate_digest: sealed.result_digest.clone(),
        regression_authorization_digest: regression_authorization.derivation_hash.clone(),
        sealed_window_authorization: authorization,
        sealed_window_authorization_replayed,
        resumed_stage_count: resume.as_ref().map_or(0, |issued| issued.stages.len()),
        resume,
        resume_replayed,
        prior_enacted_halt_revalidated,
        terminal_disposition,
        every_completed_census_followed_total_classification,
        unknown_never_counted_as_discharger_or_exclusion,
        one_extraction_procedure_digest,
        lawful_branch_disposition,
        enacted_classifications_or_outcomes_consumed,
        byte_identity_consumed_as_provenance_or_selector,
        semantic_value_used_as_selector,
        bar_used_as_selector,
        hash_or_enumeration_order_used_as_selector,
        branch_indexed_only: true,
        bi2_finale_issued: false,
        bi4_cone_verdict_issued: false,
        g2_verdict_issued: false,
        g3_verdict_issued: false,
        uc1_scored: false,
        unindexed_claim_issued: false,
        result_digest: String::new(),
    };
    certificate.result_digest = branch_digest(&certificate);
    Ok(certificate)
}

#[derive(Clone)]
struct CandidateView {
    hash: String,
    canonical_encoding: String,
    fnv_key_diagnostic: String,
}

#[derive(Clone)]
struct StageView {
    stage: u32,
    candidates: Vec<CandidateView>,
    winner_hash: Option<String>,
}

fn old_stage_views(branch: &Bi1BranchCertificateV3) -> Vec<StageView> {
    branch
        .continuation
        .stages
        .iter()
        .filter(|stage| stage.stage >= 8)
        .map(|stage| StageView {
            stage: stage.stage,
            candidates: stage
                .assessments
                .iter()
                .map(|candidate| CandidateView {
                    hash: candidate.candidate_hash.clone(),
                    canonical_encoding: serde_json::to_string(&canonicalize_telescope(
                        &candidate.telescope,
                    ))
                    .expect("canonical telescope serializes"),
                    fnv_key_diagnostic: candidate.canonical_key.clone(),
                })
                .collect(),
            winner_hash: stage
                .winner
                .as_ref()
                .map(|winner| winner.candidate_hash.clone()),
        })
        .collect()
}

fn resumed_stage_views(stages: &[Bi1bStageRecordV1]) -> Vec<StageView> {
    stages
        .iter()
        .map(|stage| StageView {
            stage: stage.stage,
            candidates: stage
                .assessments
                .iter()
                .map(|candidate: &Bi1bCandidateAssessmentV1| CandidateView {
                    hash: candidate.candidate_hash.clone(),
                    canonical_encoding: candidate.canonical_encoding.clone(),
                    fnv_key_diagnostic: candidate.canonical_key.clone(),
                })
                .collect(),
            winner_hash: stage
                .winner
                .as_ref()
                .map(|winner| winner.candidate_hash.clone()),
        })
        .collect()
}

fn selected_history_old(branch: &Bi1BranchCertificateV3) -> BTreeMap<u32, String> {
    branch
        .continuation
        .complete_ledger
        .iter()
        .map(|row| (row.stage, row.candidate_hash.clone()))
        .collect()
}

fn selected_history_new(resume: &BranchInvarianceResumeV1) -> BTreeMap<u32, String> {
    resume
        .complete_ledger
        .iter()
        .map(|row| (row.stage, row.candidate_hash.clone()))
        .collect()
}

fn issue_correspondence(
    sealed: &Bi1OptionASweepBundleV3,
    branches: &[Bi1bBranchCertificateV1],
) -> Result<Bi1bCorrespondenceDiagnosticV1, Bi1bPrefixGeneralSweepV1Error> {
    if branches.len() != 4
        || branches
            .iter()
            .any(|branch| branch.result_digest != branch_digest(branch))
    {
        return Err(Bi1bPrefixGeneralSweepV1Error::Invariant(
            "correspondence cannot open before all four branch digests seal".to_owned(),
        ));
    }
    let enacted_old = sealed
        .branches
        .iter()
        .find(|branch| branch.enacted_root)
        .ok_or_else(|| {
            Bi1bPrefixGeneralSweepV1Error::Invariant(
                "correspondence has no enacted source branch".to_owned(),
            )
        })?;
    let enacted_views = old_stage_views(enacted_old);
    let enacted_history = selected_history_old(enacted_old);
    let mut rows = Vec::new();
    for branch in branches.iter().filter(|branch| !branch.enacted_root) {
        let resume = branch.resume.as_ref().ok_or_else(|| {
            Bi1bPrefixGeneralSweepV1Error::Invariant(
                "alternate correspondence branch has no resume certificate".to_owned(),
            )
        })?;
        let alternate_views = resumed_stage_views(&resume.stages);
        let alternate_history = selected_history_new(resume);
        for alternate in &alternate_views {
            let Some(enacted) = enacted_views
                .iter()
                .find(|candidate| candidate.stage == alternate.stage)
            else {
                continue;
            };
            let enacted_hashes = enacted
                .candidates
                .iter()
                .map(|candidate| candidate.hash.clone())
                .collect::<BTreeSet<_>>();
            let alternate_hashes = alternate
                .candidates
                .iter()
                .map(|candidate| candidate.hash.clone())
                .collect::<BTreeSet<_>>();
            let byte_identical_candidate_hashes = enacted_hashes
                .intersection(&alternate_hashes)
                .cloned()
                .collect::<Vec<_>>();
            let enacted_only_candidate_hashes = enacted_hashes
                .difference(&alternate_hashes)
                .cloned()
                .collect::<Vec<_>>();
            let alternate_only_candidate_hashes = alternate_hashes
                .difference(&enacted_hashes)
                .cloned()
                .collect::<Vec<_>>();
            let predecessor_difference_stages = (1..alternate.stage)
                .filter(|stage| enacted_history.get(stage) != alternate_history.get(stage))
                .collect::<Vec<_>>();
            let stage4_only_prefix_shadow_for_byte_identical_candidates =
                predecessor_difference_stages == [4] && !byte_identical_candidate_hashes.is_empty();
            let mut canonical_encoding_necessary_matches = Vec::new();
            for left in &enacted.candidates {
                for right in &alternate.candidates {
                    if left.hash != right.hash
                        && left.canonical_encoding == right.canonical_encoding
                    {
                        canonical_encoding_necessary_matches.push(
                            Bi1bCanonicalEncodingNecessaryMatchV1 {
                                enacted_candidate_hash: left.hash.clone(),
                                alternate_candidate_hash: right.hash.clone(),
                                canonical_encoding: left.canonical_encoding.clone(),
                                enacted_fnv_key_diagnostic: left.fnv_key_diagnostic.clone(),
                                alternate_fnv_key_diagnostic: right.fnv_key_diagnostic.clone(),
                                typed_fork_shadow_proved: false,
                            },
                        );
                    }
                }
            }
            canonical_encoding_necessary_matches.sort_by(|left, right| {
                (&left.enacted_candidate_hash, &left.alternate_candidate_hash).cmp(&(
                    &right.enacted_candidate_hash,
                    &right.alternate_candidate_hash,
                ))
            });
            let winners_byte_identical =
                enacted.winner_hash.is_some() && enacted.winner_hash == alternate.winner_hash;
            let mut row = Bi1bCorrespondenceRowV1 {
                alternate_branch_root_hash: branch.branch_root_hash.clone(),
                stage: alternate.stage,
                enacted_candidate_hashes: enacted_hashes.into_iter().collect(),
                alternate_candidate_hashes: alternate_hashes.into_iter().collect(),
                byte_identical_candidate_hashes,
                enacted_only_candidate_hashes,
                alternate_only_candidate_hashes,
                predecessor_difference_stages,
                stage4_only_prefix_shadow_for_byte_identical_candidates,
                canonical_encoding_necessary_matches,
                enacted_winner_hash: enacted.winner_hash.clone(),
                alternate_winner_hash: alternate.winner_hash.clone(),
                winners_byte_identical,
                correspondence_used_as_provenance: false,
                correspondence_used_as_selector: false,
                derivation_hash: String::new(),
            };
            row.derivation_hash = correspondence_row_digest(&row);
            rows.push(row);
        }
    }
    rows.sort_by(|left, right| {
        (&left.alternate_branch_root_hash, left.stage)
            .cmp(&(&right.alternate_branch_root_hash, right.stage))
    });
    let mut diagnostic = Bi1bCorrespondenceDiagnosticV1 {
        schema: BI1B_CORRESPONDENCE_V1_SCHEMA.to_owned(),
        date: BI1B_PREFIX_GENERAL_SWEEP_V1_DATE.to_owned(),
        sealed_branch_certificate_digests: branches
            .iter()
            .map(|branch| branch.result_digest.clone())
            .collect(),
        issued_after_all_branch_certificate_digests_sealed: true,
        rows,
        byte_identity_is_zero_charge_diagnostic_only: true,
        canonical_encoding_equality_not_treated_as_typed_fork_shadow_proof: true,
        fnv_key_used_only_as_collision_prone_diagnostic: true,
        used_by_provenance_issuer: false,
        used_by_branch_selector: false,
        bi2_finale_issued: false,
        bi4_cone_verdict_issued: false,
        g2_verdict_issued: false,
        g3_verdict_issued: false,
        uc1_scored: false,
        unindexed_claim_issued: false,
        derivation_hash: String::new(),
    };
    diagnostic.derivation_hash = correspondence_digest(&diagnostic);
    Ok(diagnostic)
}

fn sweep_row(branch: &Bi1bBranchCertificateV1) -> Bi1bSweepBranchRowV1 {
    let mut row = Bi1bSweepBranchRowV1 {
        issuance_ordinal: branch.issuance_ordinal,
        branch_root_hash: branch.branch_root_hash.clone(),
        enacted_root: branch.enacted_root,
        branch_certificate_digest: branch.result_digest.clone(),
        terminal_disposition: branch.terminal_disposition.clone(),
        resumed_stage_count: branch.resumed_stage_count,
        lawful_branch_disposition: branch.lawful_branch_disposition,
        row_hash: String::new(),
    };
    row.row_hash = sweep_row_digest(&row);
    row
}

pub fn issue_bi1b_prefix_general_sweep_v1()
-> Result<Bi1bPrefixGeneralSweepV1Bundle, Bi1bPrefixGeneralSweepV1Error> {
    let sealed = sealed_bi1_bundle()?;
    let preflight = issue_sealed_sweep_preflight(&sealed)?;
    let regression = issue_regression(&sealed, preflight)?;

    let enacted = sealed
        .branches
        .iter()
        .find(|branch| branch.enacted_root)
        .ok_or_else(|| {
            Bi1bPrefixGeneralSweepV1Error::Invariant("enacted branch is absent".to_owned())
        })?;
    let mut branches = vec![issue_branch_certificate(
        enacted,
        &regression.authorization,
        0,
    )?];
    let mut alternates = sealed
        .branches
        .iter()
        .filter(|branch| !branch.enacted_root)
        .collect::<Vec<_>>();
    alternates.sort_by(|left, right| left.branch_root_hash.cmp(&right.branch_root_hash));
    for (index, alternate) in alternates.into_iter().enumerate() {
        branches.push(issue_branch_certificate(
            alternate,
            &regression.authorization,
            index + 1,
        )?);
    }
    let correspondence = issue_correspondence(&sealed, &branches)?;
    let branch_rows = branches.iter().map(sweep_row).collect::<Vec<_>>();
    let exact_four_branch_surface = branches.len() == 4
        && branches.iter().filter(|branch| branch.enacted_root).count() == 1
        && branches
            .iter()
            .map(|branch| branch.branch_root_hash.as_str())
            .collect::<BTreeSet<_>>()
            == sealed
                .branches
                .iter()
                .map(|branch| branch.branch_root_hash.as_str())
                .collect::<BTreeSet<_>>();
    let every_branch_certificate_replayed = branches.iter().all(|branch| {
        branch.sealed_window_authorization_replayed
            && branch.result_digest == branch_digest(branch)
            && ((branch.enacted_root && branch.prior_enacted_halt_revalidated)
                || (!branch.enacted_root && branch.resume_replayed))
    });
    let one_extraction_procedure_digest = regression.one_procedure_digest
        && branches
            .iter()
            .all(|branch| branch.one_extraction_procedure_digest);
    let f_b1b_2_passed = one_extraction_procedure_digest;
    let f_b1b_3_passed = branches.iter().all(|branch| {
        branch.every_completed_census_followed_total_classification
            && branch.unknown_never_counted_as_discharger_or_exclusion
    });
    let f_b1b_4_passed = branches
        .iter()
        .all(|branch| !branch.enacted_classifications_or_outcomes_consumed);
    let f_b1b_5_passed = correspondence.byte_identity_is_zero_charge_diagnostic_only
        && !correspondence.used_by_provenance_issuer
        && !correspondence.used_by_branch_selector
        && branches
            .iter()
            .all(|branch| !branch.byte_identity_consumed_as_provenance_or_selector);
    let f_b1b_6_passed = branches.iter().all(|branch| {
        !branch.semantic_value_used_as_selector
            && !branch.bar_used_as_selector
            && !branch.hash_or_enumeration_order_used_as_selector
    });
    if !exact_four_branch_surface
        || !every_branch_certificate_replayed
        || !regression.f_b1b_1_passed
        || !f_b1b_2_passed
        || !f_b1b_3_passed
        || !f_b1b_4_passed
        || !f_b1b_5_passed
        || !f_b1b_6_passed
        || regression.bi2_finale_issued
        || regression.bi4_cone_verdict_issued
        || regression.g2_verdict_issued
        || regression.g3_verdict_issued
        || regression.uc1_scored
        || regression.unindexed_claim_issued
        || correspondence.bi2_finale_issued
        || correspondence.bi4_cone_verdict_issued
        || correspondence.g2_verdict_issued
        || correspondence.g3_verdict_issued
        || correspondence.uc1_scored
        || correspondence.unindexed_claim_issued
    {
        return Err(Bi1bPrefixGeneralSweepV1Error::Invariant(
            "BI-1b sweep did not close its exact four-branch falsifier surface".to_owned(),
        ));
    }

    let mut sweep = Bi1bPrefixGeneralSweepV1Certificate {
        schema: BI1B_PREFIX_GENERAL_SWEEP_V1_SCHEMA.to_owned(),
        date: BI1B_PREFIX_GENERAL_SWEEP_V1_DATE.to_owned(),
        plan_blake3: bytes_hash(PLAN_BYTES),
        source_bindings: source_bindings(),
        sealed_bi1_sweep_digest: sealed.manifest.result_digest,
        regression_certificate_digest: regression.result_digest.clone(),
        regression_authorization_digest: regression.authorization.derivation_hash.clone(),
        extraction_procedure_digest: regression.extraction_authority.derivation_hash.clone(),
        branch_count: branch_rows.len(),
        branch_rows,
        exact_four_branch_surface,
        enacted_branch_revalidated_first: branches
            .first()
            .is_some_and(|branch| branch.enacted_root && branch.prior_enacted_halt_revalidated),
        alternate_branch_count: branches.iter().filter(|branch| !branch.enacted_root).count(),
        every_branch_certificate_replayed,
        correspondence_diagnostic_digest: correspondence.derivation_hash.clone(),
        correspondence_issued_after_branch_seals: correspondence
            .issued_after_all_branch_certificate_digests_sealed,
        supersedes_nothing: true,
        one_extraction_procedure_digest,
        f_b1b_1_passed: regression.f_b1b_1_passed,
        f_b1b_2_passed,
        f_b1b_3_passed,
        f_b1b_4_passed,
        f_b1b_5_passed,
        f_b1b_6_passed,
        bi2_finale_issued: false,
        bi4_cone_report_issued: false,
        cross_branch_g2_verdict_issued: false,
        cross_branch_g3_verdict_issued: false,
        uc1_scored: false,
        unindexed_halt_ledger_or_o_claim_issued: false,
        permitted_conclusion: "The enacted Stage-8 prefix-general extraction exactly reproduces the sealed v6 proved/impossible classification, and each alternate branch has been resumed from its own sealed Stage-8 window under the same row-free authorization. Outcomes remain branch-indexed. BI-2, BI-4, G2/G3, UC-1, and every unindexed halt/debt claim remain unissued.".to_owned(),
        result_digest: String::new(),
    };
    sweep.result_digest = sweep_digest(&sweep);
    Ok(Bi1bPrefixGeneralSweepV1Bundle {
        regression,
        branches,
        correspondence,
        sweep,
    })
}

fn bundle_integrity_errors(bundle: &Bi1bPrefixGeneralSweepV1Bundle) -> Vec<String> {
    let mut errors = Vec::new();
    if bundle.regression.result_digest != regression_digest(&bundle.regression) {
        errors.push("BI-1b regression certificate digest mismatch".to_owned());
    }
    if bundle.regression.bi2_finale_issued
        || bundle.regression.bi4_cone_verdict_issued
        || bundle.regression.g2_verdict_issued
        || bundle.regression.g3_verdict_issued
        || bundle.regression.uc1_scored
        || bundle.regression.unindexed_claim_issued
    {
        errors.push("BI-1b regression exceeded its scope".to_owned());
    }
    if bundle.regression.t_bi_v6_certificate_digest != FROZEN_T_BI_V6_CERTIFICATE_DIGEST
        || !bundle.regression.t_bi_v6_frozen_digest_exact
        || !bundle.regression.t_bi_v6_self_digest_valid
        || !bundle.regression.t_bi_v6_frozen_testimony_authenticated
        || !is_blake3_digest(&bundle.regression.current_t_bi_v6_reissuance_digest)
        || !bundle
            .regression
            .current_t_bi_v6_global_semantic_surface_matches_frozen
        || !bundle
            .regression
            .current_t_bi_v6_stage8_semantic_projection_matches_frozen
        || (bundle.regression.t_bi_v6_public_deterministic_replay_passed
            && (bundle.regression.t_bi_v6_postseal_reissuance_drift_only
                || !bundle.regression.t_bi_v6_public_replay_errors.is_empty()))
        || (!bundle.regression.t_bi_v6_public_deterministic_replay_passed
            && (!bundle.regression.t_bi_v6_postseal_reissuance_drift_only
                || bundle.regression.t_bi_v6_public_replay_errors
                    != vec![T_BI_V6_CURRENT_REISSUANCE_DRIFT.to_owned()]))
    {
        errors.push("BI-1b frozen T-BI v6 testimony surface is invalid".to_owned());
    }
    if bundle.regression.authorization.derivation_hash
        != regression_authorization_digest(&bundle.regression.authorization)
    {
        errors.push("BI-1b narrow authorization digest mismatch".to_owned());
    }
    errors.extend(regression_authorization_errors(
        &bundle.regression.authorization,
    ));
    if !bundle.regression.authorization_replayed {
        errors.push("BI-1b regression authorization was not replayed".to_owned());
    }
    let preflight = &bundle.regression.preflight;
    if !preflight.sealed_content_preflight_passed
        || preflight.sealed_sweep_digest != FROZEN_BI1_V3_MANIFEST_DIGEST
        || preflight.sealed_stage4_cone_digest != FROZEN_BI1_V3_STAGE4_CONE_DIGEST
        || !preflight.frozen_manifest_digest_exact
        || !preflight.frozen_branch_root_and_certificate_bindings_exact
        || !preflight.common_stage4_cone_binding_exact
        || !preflight.every_branch_root_matches_its_sealed_seed
        || !preflight.exactly_one_enacted_root_cross_bound
        || !preflight.sealed_manifest_self_digest_valid
        || !preflight.every_manifest_row_self_digest_valid
        || !preflight.every_branch_certificate_self_digest_valid
        || !preflight.every_manifest_branch_cross_binding_exact
        || (preflight.public_v3_replay_passed && preflight.sealed_testimony_fallback_used)
        || (!preflight.public_v3_replay_passed && !preflight.sealed_testimony_fallback_used)
    {
        errors.push("BI-1b frozen-testimony preflight surface is invalid".to_owned());
    }
    let mut preflight_branch_digests = bundle
        .regression
        .preflight
        .sealed_branch_certificate_digests
        .clone();
    let frozen_branch_digests = FROZEN_BI1_V3_BRANCH_BINDINGS
        .iter()
        .map(|(_, certificate)| (*certificate).to_owned())
        .collect::<Vec<_>>();
    if bundle
        .regression
        .preflight
        .sealed_branch_certificate_digests
        != frozen_branch_digests
    {
        errors.push("BI-1b preflight branch digests differ from the frozen four".to_owned());
    }
    preflight_branch_digests.sort();
    preflight_branch_digests.dedup();
    if bundle
        .regression
        .authorization
        .authorized_sealed_branch_certificate_digests
        != preflight_branch_digests
        || bundle
            .regression
            .authorization
            .sealed_global_bi1_sweep_digest
            != bundle.regression.preflight.sealed_sweep_digest
        || bundle
            .regression
            .authorization
            .sealed_global_stage4_cone_digest
            != bundle.regression.preflight.sealed_stage4_cone_digest
        || !bundle
            .regression
            .authorization
            .sealed_global_bi1_sweep_content_authenticated
        || bundle.regression.authorization.extraction_procedure_digest
            != bundle.regression.extraction_authority.derivation_hash
    {
        errors.push("BI-1b regression authorization/preflight cross-binding mismatch".to_owned());
    }
    for branch in &bundle.branches {
        if branch.result_digest != branch_digest(branch) {
            errors.push(format!(
                "BI-1b branch {} certificate digest mismatch",
                branch.branch_root_hash
            ));
        }
    }
    if bundle.correspondence.derivation_hash != correspondence_digest(&bundle.correspondence) {
        errors.push("BI-1b correspondence digest mismatch".to_owned());
    }
    if bundle.correspondence.bi2_finale_issued
        || bundle.correspondence.bi4_cone_verdict_issued
        || bundle.correspondence.g2_verdict_issued
        || bundle.correspondence.g3_verdict_issued
        || bundle.correspondence.uc1_scored
        || bundle.correspondence.unindexed_claim_issued
    {
        errors.push("BI-1b correspondence exceeded its diagnostic-only scope".to_owned());
    }
    for row in &bundle.correspondence.rows {
        if row.derivation_hash != correspondence_row_digest(row) {
            errors.push(format!(
                "BI-1b correspondence row {}/{} digest mismatch",
                row.alternate_branch_root_hash, row.stage
            ));
        }
    }
    if bundle.sweep.result_digest != sweep_digest(&bundle.sweep) {
        errors.push("BI-1b sweep certificate digest mismatch".to_owned());
    }
    for row in &bundle.sweep.branch_rows {
        if row.row_hash != sweep_row_digest(row) {
            errors.push(format!(
                "BI-1b sweep row {} digest mismatch",
                row.branch_root_hash
            ));
        }
    }
    if bundle.sweep.regression_certificate_digest != bundle.regression.result_digest
        || bundle.sweep.regression_authorization_digest
            != bundle.regression.authorization.derivation_hash
        || bundle.sweep.correspondence_diagnostic_digest != bundle.correspondence.derivation_hash
        || bundle.sweep.branch_rows.len() != bundle.branches.len()
    {
        errors.push("BI-1b bundle cross-binding mismatch".to_owned());
    }
    if !bundle.sweep.supersedes_nothing {
        errors.push("BI-1b sweep improperly supersedes prior sealed testimony".to_owned());
    }
    if bundle.sweep.source_bindings != source_bindings()
        || bundle.sweep.plan_blake3 != bytes_hash(PLAN_BYTES)
    {
        errors.push("BI-1b proof-chain source binding drift".to_owned());
    }
    for branch in &bundle.branches {
        if bundle
            .sweep
            .branch_rows
            .iter()
            .filter(|row| {
                row.issuance_ordinal == branch.issuance_ordinal
                    && row.branch_root_hash == branch.branch_root_hash
                    && row.branch_certificate_digest == branch.result_digest
            })
            .count()
            != 1
        {
            errors.push(format!(
                "BI-1b branch {} lacks one exact sweep row",
                branch.branch_root_hash
            ));
        }
    }
    errors
}

pub fn replay_bi1b_prefix_general_sweep_v1(
    claimed: &Bi1bPrefixGeneralSweepV1Bundle,
) -> Bi1bPrefixGeneralSweepV1Replay {
    let mut errors = bundle_integrity_errors(claimed);
    match issue_bi1b_prefix_general_sweep_v1() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("BI-1b sweep differs from deterministic reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    Bi1bPrefixGeneralSweepV1Replay {
        valid: errors.is_empty(),
        errors,
    }
}

fn branch_json_name(prefix: &str) -> String {
    format!("BI1B_BRANCH_{prefix}_CERTIFICATE.json")
}

fn branch_report_name(prefix: &str) -> String {
    format!("BI1B_BRANCH_{prefix}_RESULT.md")
}

pub fn render_bi1b_regression_v1(certificate: &Bi1bRegressionCertificateV1) -> String {
    format!(
        "# BI-1b enacted Stage-8 regression\n\n**Date:** {}. **Certificate:** `{}`.\n\nThe sealed BI-1 v3 public deterministic replay passed: **{}**. Exact replay report: `{}`. Frozen-testimony authentication was used: **{}**; it binds the preregistered manifest, all four certificate/root pairs, their full manifest rows, and the common Stage-4 cone by exact old-domain digests.\n\nThe frozen T-BI v6 testimony authenticated: **{}**. Its current deterministic reissuance passed: **{}**. Exact replay report: `{}`. A current-reissuance inequality is recorded, not represented as a successful replay. The fresh reissuance is `{}`; its global semantic surface and Stage-8 semantic projection match the frozen v6 target: **{}/{}**.\n\nThe prefix-general v4 procedure reproduced the enacted v6 Stage-8 families, proved rows, theorem-impossibility rows, totals, R2 boundary, and zero residuals: **{}/{}/{}/{}/{}/{}**. F-B1b-1: **{}**.\n\nThe exported authorization is `{}`. It contains no family IDs, semantic vector, enacted winner, enacted outcome, or selector capability.\n",
        certificate.date,
        certificate.result_digest,
        certificate.preflight.public_v3_replay_passed,
        if certificate.preflight.public_v3_replay_errors.is_empty() {
            "none".to_owned()
        } else {
            certificate.preflight.public_v3_replay_errors.join("; ")
        },
        certificate.preflight.sealed_testimony_fallback_used,
        certificate.t_bi_v6_frozen_testimony_authenticated,
        certificate.t_bi_v6_public_deterministic_replay_passed,
        if certificate.t_bi_v6_public_replay_errors.is_empty() {
            "none".to_owned()
        } else {
            certificate.t_bi_v6_public_replay_errors.join("; ")
        },
        certificate.current_t_bi_v6_reissuance_digest,
        certificate.current_t_bi_v6_global_semantic_surface_matches_frozen,
        certificate.current_t_bi_v6_stage8_semantic_projection_matches_frozen,
        certificate.exact_family_ids_reproduced,
        certificate.exact_proved_rows_reproduced,
        certificate.exact_theorem_impossibility_rows_reproduced,
        certificate.exact_role_and_semantic_totals_reproduced,
        certificate.exact_r2_regression_reproduced,
        certificate.zero_residuals_reproduced,
        certificate.f_b1b_1_passed,
        certificate.authorization.derivation_hash,
    )
}

pub fn render_bi1b_branch_v1(certificate: &Bi1bBranchCertificateV1) -> String {
    let rows = certificate
        .resume
        .as_ref()
        .map(|resume| {
            resume
                .stages
                .iter()
                .map(|stage| {
                    let census = stage.census.as_ref().map_or_else(
                        || "forbidden (Unknown present)".to_owned(),
                        |census| format!("{}", census.discharger_count),
                    );
                    format!(
                        "| {} | {} | {} | `{}` |",
                        stage.stage,
                        stage.assessments.len(),
                        census,
                        stage
                            .winner
                            .as_ref()
                            .map_or("none", |winner| winner.candidate_hash.as_str())
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| "| 8-15 | sealed enacted replay | n/a | prior halt |".to_owned());
    format!(
        "# BI-1b branch {}\n\n**Date:** {}. **Certificate:** `{}`. **Enacted:** {}.\n\nTerminal disposition: `{}`. Lawful: **{}**. One extraction procedure: **{}**. Unknown was never counted: **{}**.\n\n| Stage | Candidates | Dischargers | Winner |\n|---:|---:|---|---|\n{}\n\nNo BI-2, BI-4, G2/G3, UC-1, or unindexed claim is issued.\n",
        certificate.branch_digest_prefix,
        certificate.date,
        certificate.result_digest,
        certificate.enacted_root,
        certificate.terminal_disposition,
        certificate.lawful_branch_disposition,
        certificate.one_extraction_procedure_digest,
        certificate.unknown_never_counted_as_discharger_or_exclusion,
        rows,
    )
}

pub fn render_bi1b_sweep_v1(certificate: &Bi1bPrefixGeneralSweepV1Certificate) -> String {
    let rows = certificate
        .branch_rows
        .iter()
        .map(|row| {
            format!(
                "| {} | `{}` | {} | `{}` | {} |",
                row.issuance_ordinal,
                row.branch_root_hash,
                row.enacted_root,
                row.terminal_disposition,
                row.lawful_branch_disposition
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# BI-1b prefix-general sweep v1\n\n**Date:** {}. **Certificate:** `{}`.\n\nThe enacted hard regression passed before alternate work: **{}**. Four branch certificates replayed: **{}**. One prefix-parametric procedure served every observed classification: **{}**. Exact proof-chain source bindings: **{}**.\n\n| Ordinal | Root | Enacted | Disposition | Lawful |\n|---:|---|---|---|---|\n{}\n\nF-B1b-1..6: **{}/{}/{}/{}/{}/{}**. Correspondence was built only after the branch seals and is diagnostic-only: **{}**. This artifact supersedes nothing: **{}**; the BI-1 v3 certificates remain sealed testimony.\n\nNo BI-2 finale, BI-4 cone report, G2/G3 verdict, UC-1 score, or unindexed halt/debt/O claim is issued.\n\n{}\n",
        certificate.date,
        certificate.result_digest,
        certificate.f_b1b_1_passed,
        certificate.every_branch_certificate_replayed,
        certificate.one_extraction_procedure_digest,
        certificate.source_bindings.len(),
        rows,
        certificate.f_b1b_1_passed,
        certificate.f_b1b_2_passed,
        certificate.f_b1b_3_passed,
        certificate.f_b1b_4_passed,
        certificate.f_b1b_5_passed,
        certificate.f_b1b_6_passed,
        certificate.correspondence_issued_after_branch_seals,
        certificate.supersedes_nothing,
        certificate.permitted_conclusion,
    )
}

fn create_new(path: &Path, contents: &[u8]) -> Result<(), Bi1bPrefixGeneralSweepV1Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
            Bi1bPrefixGeneralSweepV1Error::Io(format!("{}: {error}", path.display()))
        })?;
    file.write_all(contents)
        .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Io(format!("{}: {error}", path.display())))
}

pub fn emit_bi1b_prefix_general_sweep_v1_create_new(
    directory: &Path,
) -> Result<Bi1bPrefixGeneralSweepV1Bundle, Bi1bPrefixGeneralSweepV1Error> {
    let bundle = issue_bi1b_prefix_general_sweep_v1()?;
    let replay = replay_bi1b_prefix_general_sweep_v1(&bundle);
    if !replay.valid {
        return Err(Bi1bPrefixGeneralSweepV1Error::Invariant(format!(
            "full BI-1b bundle failed replay before any write: {}",
            replay.errors.join("; ")
        )));
    }

    let mut targets = Vec::<(PathBuf, Vec<u8>)>::new();
    targets.push((
        directory.join(BI1B_REGRESSION_CERTIFICATE_NAME),
        serde_json::to_vec_pretty(&bundle.regression)
            .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Json(error.to_string()))?,
    ));
    targets.push((
        directory.join(BI1B_REGRESSION_REPORT_NAME),
        render_bi1b_regression_v1(&bundle.regression).into_bytes(),
    ));
    for branch in &bundle.branches {
        targets.push((
            directory.join(branch_json_name(&branch.branch_digest_prefix)),
            serde_json::to_vec_pretty(branch)
                .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Json(error.to_string()))?,
        ));
        targets.push((
            directory.join(branch_report_name(&branch.branch_digest_prefix)),
            render_bi1b_branch_v1(branch).into_bytes(),
        ));
    }
    targets.push((
        directory.join(BI1B_CORRESPONDENCE_NAME),
        serde_json::to_vec_pretty(&bundle.correspondence)
            .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Json(error.to_string()))?,
    ));
    targets.push((
        directory.join(BI1B_SWEEP_CERTIFICATE_NAME),
        serde_json::to_vec_pretty(&bundle.sweep)
            .map_err(|error| Bi1bPrefixGeneralSweepV1Error::Json(error.to_string()))?,
    ));
    targets.push((
        directory.join(BI1B_SWEEP_REPORT_NAME),
        render_bi1b_sweep_v1(&bundle.sweep).into_bytes(),
    ));
    if let Some(existing) = targets.iter().find(|(path, _)| path.exists()) {
        return Err(Bi1bPrefixGeneralSweepV1Error::Io(format!(
            "{} already exists; create-new emitted nothing",
            existing.0.display()
        )));
    }
    for (path, contents) in targets {
        create_new(&path, &contents)?;
    }
    Ok(bundle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_v3_testimony_requires_literal_manifest_cone_roots_and_certificates() {
        let sealed = sealed_bi1_bundle().expect("frozen BI-1 testimony parses");
        assert!(
            sealed_content_errors(&sealed).is_empty(),
            "{}",
            sealed_content_errors(&sealed).join("; ")
        );

        let mut manifest = sealed.clone();
        manifest.manifest.result_digest.push('0');
        assert!(!sealed_content_errors(&manifest).is_empty());

        let mut cone = sealed.clone();
        cone.branches[1].cone_digest.push('0');
        cone.branches[1].result_digest = old_branch_digest(&cone.branches[1]);
        assert!(!sealed_content_errors(&cone).is_empty());

        let mut root = sealed.clone();
        root.branches[2].branch_root_hash.push('0');
        root.branches[2].result_digest = old_branch_digest(&root.branches[2]);
        assert!(!sealed_content_errors(&root).is_empty());

        let mut certificate = sealed.clone();
        certificate.branches[3].continuation_replayed =
            !certificate.branches[3].continuation_replayed;
        certificate.branches[3].result_digest = old_branch_digest(&certificate.branches[3]);
        certificate.manifest.branch_rows[3] = expected_old_manifest_row(&certificate.branches[3]);
        certificate.manifest.result_digest = old_manifest_digest(&certificate.manifest);
        assert!(!sealed_content_errors(&certificate).is_empty());

        let mut full_row = sealed.clone();
        full_row.manifest.branch_rows[1]
            .continuation_outcome
            .push_str(" forged");
        full_row.manifest.branch_rows[1].row_hash =
            old_manifest_row_digest(&full_row.manifest.branch_rows[1]);
        full_row.manifest.result_digest = old_manifest_digest(&full_row.manifest);
        assert!(!sealed_content_errors(&full_row).is_empty());
    }

    #[test]
    fn frozen_t_bi_v6_target_requires_its_literal_and_self_digest() {
        let frozen: TBiNu1RegressionV6Certificate =
            parse_json("frozen T-BI v6 certificate", SEALED_T_BI_V6_BYTES)
                .expect("frozen T-BI v6 testimony parses");
        assert_eq!(frozen.result_digest, FROZEN_T_BI_V6_CERTIFICATE_DIGEST);
        assert_eq!(
            frozen.result_digest,
            frozen_t_bi_v6_certificate_digest(&frozen)
        );

        let mut fully_rehashed = frozen;
        fully_rehashed.outcome.push_str("-forged");
        fully_rehashed.result_digest = frozen_t_bi_v6_certificate_digest(&fully_rehashed);
        assert_ne!(
            fully_rehashed.result_digest,
            FROZEN_T_BI_V6_CERTIFICATE_DIGEST
        );
    }

    #[test]
    #[ignore = "full BI-1b sweep is intentionally expensive"]
    fn deterministic_replay_closes_the_exact_bi1b_bundle() {
        let bundle = issue_bi1b_prefix_general_sweep_v1().expect("BI-1b sweep issues");
        let replay = replay_bi1b_prefix_general_sweep_v1(&bundle);
        assert!(replay.valid, "{}", replay.errors.join("; "));
        assert_eq!(bundle.branches.len(), 4);
        assert!(bundle.sweep.f_b1b_1_passed);
        assert!(bundle.sweep.f_b1b_2_passed);
        assert!(bundle.sweep.f_b1b_3_passed);
        assert!(bundle.sweep.f_b1b_4_passed);
        assert!(bundle.sweep.f_b1b_5_passed);
        assert!(bundle.sweep.f_b1b_6_passed);
        assert!(!bundle.sweep.bi2_finale_issued);
        assert!(!bundle.sweep.bi4_cone_report_issued);
        assert!(!bundle.sweep.cross_branch_g2_verdict_issued);
        assert!(!bundle.sweep.cross_branch_g3_verdict_issued);
        assert!(!bundle.sweep.uc1_scored);
        assert!(!bundle.sweep.unindexed_halt_ledger_or_o_claim_issued);
    }

    #[test]
    #[ignore = "full BI-1b sweep is intentionally expensive"]
    fn every_emitted_certificate_has_a_mutation_falsifier() {
        let bundle = issue_bi1b_prefix_general_sweep_v1().expect("BI-1b sweep issues");

        let mut regression = bundle.clone();
        regression.regression.result_digest.push('0');
        assert!(!bundle_integrity_errors(&regression).is_empty());

        for index in 0..bundle.branches.len() {
            let mut branch = bundle.clone();
            branch.branches[index].result_digest.push('0');
            assert!(!bundle_integrity_errors(&branch).is_empty());
        }

        let mut correspondence = bundle.clone();
        correspondence.correspondence.derivation_hash.push('0');
        assert!(!bundle_integrity_errors(&correspondence).is_empty());

        let mut sweep = bundle;
        sweep.sweep.result_digest.push('0');
        assert!(!bundle_integrity_errors(&sweep).is_empty());
    }
}
