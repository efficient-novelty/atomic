//! Mainline M-3: cone-aware E-7/E-8 bridge audit.
//!
//! This issuer is intentionally fail closed.  It authenticates the completed
//! BI-4 cone, checks the enacted E-5/BI-2 semantic regression, and then records
//! the exact theorem obligations that are still missing.  It does not turn a
//! structural row join into a semantic classifier or candidate bridge.

use crate::bi2_branch_finales_v1::{BI2_BRANCH_FINALE_V2_SCHEMA, Bi2BranchFinaleCertificateV1};
use crate::bi4_cone_assembly_v2::{
    Bi4ConeAssemblyV2Certificate, Bi4DispositionStatusV2, replay_bi4_cone_assembly_v2_json,
};
use crate::candidate_join::{CANDIDATE_JOIN_V4_SCHEMA_VERSION, CandidateJoinV4Certificate};
use crate::candidate_join_v5::{CandidateJoinV5Certificate, replay_candidate_join_v5_json};
use crate::e5_future_hole_finale_v2::{
    E5_FUTURE_HOLE_FINALE_V2_SCHEMA, E5FutureHoleFinaleV2Certificate,
    replay_e5_future_hole_finale_v2_certificate,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{OpenOptions, read_to_string, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const M3_E7_E8_BRIDGE_V1_SCHEMA: &str = "mainline-m3-e7-e8-bridge-v1";
pub const M3_E7_E8_BRIDGE_V1_DATE: &str = "2026-07-23";
pub const M3_E7_GAP: &str = "C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS";
pub const M3_E8_GAP: &str = "C8_CANDIDATE_BOUNDARY_PROVENANCE_JOIN";
pub const M3_PARENT_ROW_GAP: &str =
    "E8_SCHEMA3_AGGREGATE_PARENT_ROW_REFINEMENT_OR_UNIVERSAL_THEOREM_NOT_PROVED";
pub const M3_REGISTERED_E5_DRIFT: &str = "prerequisite failed: stable A3 exhaustiveness certificate did not replay: certificate differs from definition replay";
pub const M3_CERTIFICATE_NAME: &str = "schema2_e7_e8_bridge_v1.json";
pub const M3_REPORT_NAME: &str = "SCHEMA2_E7_E8_BRIDGE_V1_RESULT.md";

const MAINLINE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/mainline_completion_plan.md");
const STEP15_SPEC_BYTES: &[u8] = include_bytes!("../../../docs/step_15_completion_open_problem.md");
const AGENT_E_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_schema2_plan.md");
const NU_REGISTER_BYTES: &[u8] = include_bytes!("../../../docs/nu_register_adjudication.md");
const SCHEMA2_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_RESULT.md");
const E2_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_quotient_adjudications.md");
const BI4_CERTIFICATE_BYTES: &[u8] = include_bytes!("../../../docs/BI4_CONE_V2_CERTIFICATE.json");
const E5_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_future_hole_finale_v2_dependent_context.json");
const ENACTED_BI2_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_2016726758f3_V2_CERTIFICATE.json");
const BI2_INDEX_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json");
const ALTERNATE_BI2_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_43a0ed707770_V2_CERTIFICATE.json");
const ALTERNATE_BI2_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_4b2211ecae25_V2_CERTIFICATE.json");
const ALTERNATE_BI2_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_b4f821d9bb28_V2_CERTIFICATE.json");
const SCHEMA3_BYTES: &[u8] = include_bytes!("../../../docs/ip1_candidate_verdict_join_v3.json");
const SCHEMA4_BYTES: &[u8] = include_bytes!("../../../docs/ip1_candidate_verdict_join_v4.json");
const SCHEMA5_BYTES: &[u8] =
    include_bytes!("../../../docs/ip1_candidate_verdict_join_v5_element_overlay.json");
const E5_SOURCE_BYTES: &[u8] = include_bytes!("e5_future_hole_finale_v2.rs");
const BI4_SOURCE_BYTES: &[u8] = include_bytes!("bi4_cone_assembly_v2.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("m3_e7_e8_bridge_v1.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum M3Register {
    SemanticRegisterAuthority,
    SemanticFamilyAuthority,
    SemanticDemandInstanceAuthority,
    StructuralTestimony,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum M3Quantifier {
    ConeLevelAllFourBranches,
    BranchIndexedExactRoot,
    FrozenEnactedRegression,
    FrozenWrappedSurface,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum M3ClaimStatus {
    Certified,
    OpenNamedGap,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum M3RunStatus {
    StoppedNamedGaps,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: M3TaggedCount,
    pub blake3: String,
    pub register: M3Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3TaggedCount {
    pub value: usize,
    pub register: M3Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3QuantifiedClaim {
    pub claim: String,
    pub component: String,
    pub quantifier: M3Quantifier,
    pub branch_root: Option<String>,
    pub register: M3Register,
    pub status: M3ClaimStatus,
    pub exact_scope: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3ConeAudit {
    pub bi4_result_digest: String,
    pub public_replay_valid: bool,
    pub public_replay_errors: Vec<String>,
    pub exact_four_distinct_branch_roots: bool,
    pub branch_root_count: M3TaggedCount,
    pub enacted_branch_root: String,
    pub disposition_row_count: M3TaggedCount,
    pub quantified_statement_count: M3TaggedCount,
    pub every_disposition_mirrored_with_exact_quantifier: bool,
    pub structural_testimony_used_for_law_level_content: bool,
    pub claims: Vec<M3QuantifiedClaim>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3EnactedProjection {
    pub exact_a3_inventory: M3TaggedCount,
    pub unary_families: M3TaggedCount,
    pub direct_chronological_instances: M3TaggedCount,
    pub pointwise_chronological_families: M3TaggedCount,
    pub higher_families: M3TaggedCount,
    pub final_window_structural_instances: M3TaggedCount,
    pub historical_structural_registrations: M3TaggedCount,
    pub historical_structural_realizations: M3TaggedCount,
    pub membership_pair_count: M3TaggedCount,
    pub membership_pair_digest: String,
    pub exact_d_partition: bool,
    pub semantic_o16_empty: bool,
    pub f1_executed: bool,
    pub f1_excluded: bool,
    pub theorem12_full_instance_granularity: bool,
    pub focus_projection_reproduces_ladder: bool,
    pub stage_three_wrinkle_reproduced: bool,
    pub e5_complete: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3EnactedRegressionAudit {
    pub quantifier: M3Quantifier,
    pub e5_frozen_result_digest: String,
    pub e5_frozen_self_digest_valid: bool,
    pub e5_native_replay_valid: bool,
    pub e5_native_replay_errors: Vec<String>,
    pub frozen_testimony_fallback_used: bool,
    pub fallback_limited_to_exact_digest_bound_frozen_e5: bool,
    pub drift_repaired_inside_m3: bool,
    pub drift_concealed_inside_m3: bool,
    pub enacted_bi2_result_digest: String,
    pub enacted_bi2_self_digest_valid: bool,
    pub enacted_bi2_root_exactly_joins_bi4: bool,
    pub e5_projection: M3EnactedProjection,
    pub bi2_projection: M3EnactedProjection,
    pub projections_exactly_equal: bool,
    pub cross_branch_statements_issued_only_after_regression: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3E7Audit {
    pub quantifier: M3Quantifier,
    pub register: M3Register,
    pub slnf_defined_independently: bool,
    pub classify_w_total: bool,
    pub faithfulness_proved: bool,
    pub realization_claimed: bool,
    pub inverse_laws_proved: bool,
    pub one_way_injection_declared_and_proved: bool,
    pub class_by_class_generator_completeness_proved: bool,
    pub c2_closed: bool,
    pub named_gap: String,
    pub no_support_local_code_pruned_from_failed_realization: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3BridgeCondition {
    pub ordinal: M3TaggedCount,
    pub condition: String,
    pub proved: bool,
    pub named_gap: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3E8Audit {
    pub quantifier: M3Quantifier,
    pub register: M3Register,
    pub schema3_strata: M3TaggedCount,
    pub schema3_aggregate_rows: M3TaggedCount,
    pub every_schema3_row_is_aggregate_bucket: bool,
    pub minimum_parent_candidate_multiplicity: M3TaggedCount,
    pub schema4_all_rows_structurally_joined: bool,
    pub schema5_public_replay_valid: bool,
    pub schema5_public_replay_errors: Vec<String>,
    pub schema5_no_row_promoted: bool,
    pub bridge_conditions: Vec<M3BridgeCondition>,
    pub bridge_condition_count: M3TaggedCount,
    pub proved_bridge_condition_count: M3TaggedCount,
    pub all_nine_conditions_proved: bool,
    pub candidate_level_join_proved: bool,
    pub c8_closed: bool,
    pub domain_wide_c3_closed: bool,
    pub named_gaps: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3NamedGap {
    pub id: String,
    pub phase: String,
    pub quantifier: M3Quantifier,
    pub register: M3Register,
    pub exact_obstruction: String,
    pub keeps_rows_unpromoted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3E7E8BridgeV1Certificate {
    pub schema: String,
    pub date: String,
    pub syntax_identifier_policy: String,
    pub syntax_identifier_register: M3Register,
    pub source_bindings: Vec<M3SourceBinding>,
    pub cone: M3ConeAudit,
    pub enacted_regression: M3EnactedRegressionAudit,
    pub e7: M3E7Audit,
    pub e8: M3E8Audit,
    pub open_gaps: Vec<M3NamedGap>,
    pub open_gap_count: M3TaggedCount,
    pub zero_promotions_made: bool,
    pub promoted_row_count: M3TaggedCount,
    pub uc1_cited_or_anticipated: bool,
    pub bridge_claim_issued: bool,
    pub m3_status: M3RunStatus,
    pub m4_authorized: bool,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct M3Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub m3_status: M3RunStatus,
    pub bridge_claim_issued: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum M3Error {
    #[error("M-3 input failure: {0}")]
    Input(String),
    #[error("M-3 invariant failure: {0}")]
    Invariant(String),
    #[error("M-3 JSON failure: {0}")]
    Json(String),
    #[error("M-3 create-new I/O failure: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(M3_E7_E8_BRIDGE_V1_SCHEMA, domain, value))
        .expect("M-3 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn semantic_count(value: usize) -> M3TaggedCount {
    M3TaggedCount {
        value,
        register: M3Register::SemanticFamilyAuthority,
    }
}

fn semantic_instance_count(value: usize) -> M3TaggedCount {
    M3TaggedCount {
        value,
        register: M3Register::SemanticDemandInstanceAuthority,
    }
}

fn structural_count(value: usize) -> M3TaggedCount {
    M3TaggedCount {
        value,
        register: M3Register::StructuralTestimony,
    }
}

fn metadata_count(value: usize) -> M3TaggedCount {
    M3TaggedCount {
        value,
        register: M3Register::ArtifactMetadata,
    }
}

fn syntax_identifier(value: usize) -> M3TaggedCount {
    M3TaggedCount {
        value,
        register: M3Register::SyntaxIdentifier,
    }
}

fn source_bindings() -> Vec<M3SourceBinding> {
    [
        (
            "docs/mainline_completion_plan.md",
            "frozen M-3 mission, ordering, and stop semantics",
            MAINLINE_PLAN_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "rewritten E-7/E-8 theorem and complete bridge-condition specification",
            STEP15_SPEC_BYTES,
        ),
        (
            "docs/agent_e_schema2_plan.md",
            "adopted E-7 one-way fallback and E-8 phase contract",
            AGENT_E_PLAN_BYTES,
        ),
        (
            "docs/nu_register_adjudication.md",
            "semantic-authority versus structural-testimony register law",
            NU_REGISTER_BYTES,
        ),
        (
            "docs/SCHEMA2_RESULT.md",
            "archived aggregate-parent-row obstruction testimony",
            SCHEMA2_RESULT_BYTES,
        ),
        (
            "docs/e2_quotient_adjudications.md",
            "adopted separation of aggregate parent-row refinement",
            E2_ADJUDICATION_BYTES,
        ),
        (
            "docs/BI4_CONE_V2_CERTIFICATE.json",
            "sealed cone disposition and four-branch authority",
            BI4_CERTIFICATE_BYTES,
        ),
        (
            "docs/schema2_e5_future_hole_finale_v2_dependent_context.json",
            "frozen enacted E-5 regression testimony",
            E5_CERTIFICATE_BYTES,
        ),
        (
            "docs/BI2_BRANCH_2016726758f3_V2_CERTIFICATE.json",
            "sealed enacted BI-2 semantic projection",
            ENACTED_BI2_CERTIFICATE_BYTES,
        ),
        (
            "docs/BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json",
            "sealed BI-2 cone index authenticated transitively by BI-4 replay",
            BI2_INDEX_BYTES,
        ),
        (
            "docs/BI2_BRANCH_43a0ed707770_V2_CERTIFICATE.json",
            "sealed alternate BI-2 branch input",
            ALTERNATE_BI2_43_BYTES,
        ),
        (
            "docs/BI2_BRANCH_4b2211ecae25_V2_CERTIFICATE.json",
            "sealed alternate BI-2 branch input",
            ALTERNATE_BI2_4B_BYTES,
        ),
        (
            "docs/BI2_BRANCH_b4f821d9bb28_V2_CERTIFICATE.json",
            "sealed alternate BI-2 branch input",
            ALTERNATE_BI2_B4_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v3.json",
            "exact archived schema-3 aggregate candidate surface",
            SCHEMA3_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v4.json",
            "schema-4 structural join and open semantic obligations",
            SCHEMA4_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v5_element_overlay.json",
            "schema-5 conservative element overlay",
            SCHEMA5_BYTES,
        ),
        (
            "crates/pen-search/src/e5_future_hole_finale_v2.rs",
            "native E-5 replay implementation whose drift is disclosed",
            E5_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/bi4_cone_assembly_v2.rs",
            "public BI-4 replay implementation",
            BI4_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/m3_e7_e8_bridge_v1.rs",
            "M-3 fail-closed issuer and replay implementation",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| M3SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: metadata_count(bytes.len()),
        blake3: bytes_hash(bytes),
        register: M3Register::ArtifactMetadata,
    })
    .collect()
}

fn certificate_digest(certificate: &M3E7E8BridgeV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn frozen_e5_digest(certificate: &E5FutureHoleFinaleV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(E5_FUTURE_HOLE_FINALE_V2_SCHEMA, "certificate", &projection))
        .expect("frozen E-5 certificate serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn frozen_bi2_digest(certificate: &Bi2BranchFinaleCertificateV1) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(
        BI2_BRANCH_FINALE_V2_SCHEMA,
        "branch-finale-certificate",
        &projection,
    ))
    .expect("frozen BI-2 certificate serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn claim_register(claim: &str) -> M3Register {
    match claim {
        "diagnostic_bar_trajectory_and_WB1_cumulative_tables"
        | "complete_Stage1_through15_Sigma_kappa"
        | "complete_cumulative_Delta_Phi_Omega_interface_trajectories" => {
            M3Register::StructuralTestimony
        }
        "G2_obligation_profile_and_one-demand-per-stage_O_ladder_from_Stage5_through_halt" => {
            M3Register::SemanticDemandInstanceAuthority
        }
        "complete_Stage1_through15_Sigma_semantic_nu"
        | "G1_Stage5_successor_scheme_sets_and_exact_derivations" => {
            M3Register::SemanticFamilyAuthority
        }
        _ => M3Register::SemanticRegisterAuthority,
    }
}

fn claim_components(claim: &str) -> Vec<(&'static str, M3Register)> {
    if claim == "G3a_per_stage_kappa_and_semantic_nu_vector_Stage5_through15" {
        vec![
            ("semantic_nu_vector", M3Register::SemanticFamilyAuthority),
            ("kappa_vector", M3Register::StructuralTestimony),
        ]
    } else {
        vec![("whole_claim", claim_register(claim))]
    }
}

fn expected_dispositions() -> BTreeMap<&'static str, Bi4DispositionStatusV2> {
    [
        (
            "G2_obligation_profile_and_one-demand-per-stage_O_ladder_from_Stage5_through_halt",
            Bi4DispositionStatusV2::PromotedToConeLevel,
        ),
        (
            "G3a_per_stage_kappa_and_semantic_nu_vector_Stage5_through15",
            Bi4DispositionStatusV2::PromotedToConeLevel,
        ),
        (
            "complete_Stage1_through15_Sigma_semantic_nu",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
        ),
        (
            "diagnostic_bar_trajectory_and_WB1_cumulative_tables",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
        ),
        (
            "G4_debt_free_halt_at_15",
            Bi4DispositionStatusV2::PromotedToConeLevel,
        ),
        (
            "semantic_O16_empty_at_full_instance_granularity",
            Bi4DispositionStatusV2::PromotedToConeLevel,
        ),
        (
            "Guard_Rail_F1_excluded",
            Bi4DispositionStatusV2::PromotedToConeLevel,
        ),
        (
            "Theorem12_full_instance_granularity_relative_to_adopted_A3",
            Bi4DispositionStatusV2::PromotedToConeLevel,
        ),
        (
            "Stage4_root_act_identity_and_semantic_nu",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
        ),
        (
            "G1_Stage5_successor_scheme_sets_and_exact_derivations",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
        ),
        (
            "branch_specific_candidate_winner_and_act_local_provenance_identities",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
        ),
        (
            "complete_Stage1_through15_Sigma_kappa",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
        ),
        (
            "complete_cumulative_Delta_Phi_Omega_interface_trajectories",
            Bi4DispositionStatusV2::RemainsBranchIndexed,
        ),
    ]
    .into_iter()
    .collect()
}

fn quantified_claim(
    claim: &str,
    component: &str,
    register: M3Register,
    quantifier: M3Quantifier,
    branch_root: Option<String>,
) -> M3QuantifiedClaim {
    let exact_scope = match (&quantifier, branch_root.as_deref()) {
        (M3Quantifier::ConeLevelAllFourBranches, None) => format!(
            "BI-4 promoted claim component `{component}`; universally quantified over all exact sealed branch roots"
        ),
        (M3Quantifier::BranchIndexedExactRoot, Some(root)) => {
            format!(
                "BI-4 branch-indexed claim component `{component}`; asserted only for exact branch root {root}"
            )
        }
        _ => "invalid quantifier/branch pairing".to_owned(),
    };
    let derivation_hash = tagged_hash(
        "quantified-claim",
        &(
            claim,
            component,
            &quantifier,
            branch_root.as_deref(),
            &register,
            &exact_scope,
        ),
    );
    M3QuantifiedClaim {
        claim: claim.to_owned(),
        component: component.to_owned(),
        quantifier,
        branch_root,
        register,
        status: M3ClaimStatus::Certified,
        exact_scope,
        derivation_hash,
    }
}

fn build_cone_audit(certificate: &Bi4ConeAssemblyV2Certificate) -> Result<M3ConeAudit, M3Error> {
    let replay = replay_bi4_cone_assembly_v2_json(
        std::str::from_utf8(BI4_CERTIFICATE_BYTES)
            .map_err(|error| M3Error::Input(error.to_string()))?,
    );
    if !replay.valid {
        return Err(M3Error::Input(format!(
            "sealed BI-4 v2 public replay failed: {}",
            replay.errors.join("; ")
        )));
    }

    let expected = expected_dispositions();
    let actual = certificate
        .branch_index_disposition
        .iter()
        .map(|row| (row.claim.as_str(), row.status.clone()))
        .collect::<BTreeMap<_, _>>();
    if actual.len() != 13
        || expected.len() != 13
        || expected
            .iter()
            .any(|(claim, status)| actual.get(claim) != Some(status))
    {
        return Err(M3Error::Invariant(
            "BI-4 disposition table is not the exact frozen 13-row table".to_owned(),
        ));
    }

    let branch_roots = certificate
        .branches
        .iter()
        .map(|branch| branch.branch_root_hash.clone())
        .collect::<Vec<_>>();
    let distinct_roots = branch_roots.iter().cloned().collect::<BTreeSet<_>>();
    let bound_branch_pairs = [
        ENACTED_BI2_CERTIFICATE_BYTES,
        ALTERNATE_BI2_43_BYTES,
        ALTERNATE_BI2_4B_BYTES,
        ALTERNATE_BI2_B4_BYTES,
    ]
    .into_iter()
    .map(|bytes| {
        serde_json::from_slice::<Bi2BranchFinaleCertificateV1>(bytes)
            .map(|branch| (branch.branch_root_hash, branch.result_digest))
            .map_err(|error| M3Error::Json(error.to_string()))
    })
    .collect::<Result<BTreeSet<_>, _>>()?;
    let cone_branch_pairs = certificate
        .branches
        .iter()
        .map(|branch| {
            (
                branch.branch_root_hash.clone(),
                branch.branch_certificate_digest.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    let enacted = certificate
        .branches
        .iter()
        .filter(|branch| branch.enacted_root)
        .collect::<Vec<_>>();
    if branch_roots.len() != 4
        || distinct_roots.len() != 4
        || enacted.len() != 1
        || bound_branch_pairs != cone_branch_pairs
    {
        return Err(M3Error::Invariant(
            "BI-4 did not expose the exact four bound branch files and one enacted index"
                .to_owned(),
        ));
    }

    let mut claims = Vec::new();
    for row in &certificate.branch_index_disposition {
        for (component, register) in claim_components(&row.claim) {
            match row.status {
                Bi4DispositionStatusV2::PromotedToConeLevel => claims.push(quantified_claim(
                    &row.claim,
                    component,
                    register,
                    M3Quantifier::ConeLevelAllFourBranches,
                    None,
                )),
                Bi4DispositionStatusV2::RemainsBranchIndexed => {
                    for root in &branch_roots {
                        claims.push(quantified_claim(
                            &row.claim,
                            component,
                            register,
                            M3Quantifier::BranchIndexedExactRoot,
                            Some(root.clone()),
                        ));
                    }
                }
            }
        }
    }
    if claims.len() != 35 {
        return Err(M3Error::Invariant(format!(
            "expected 35 register-homogeneous quantified statements, found {}",
            claims.len()
        )));
    }
    let branch_claims_exact = claims.iter().all(|claim| match claim.quantifier {
        M3Quantifier::ConeLevelAllFourBranches => claim.branch_root.is_none(),
        M3Quantifier::BranchIndexedExactRoot => claim
            .branch_root
            .as_ref()
            .is_some_and(|root| distinct_roots.contains(root)),
        _ => false,
    });
    if !branch_claims_exact {
        return Err(M3Error::Invariant(
            "a BI-4 statement lacks its exact cone/branch quantifier".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "cone-audit",
        &(
            certificate.result_digest.as_str(),
            &branch_roots,
            &claims,
            &replay.errors,
        ),
    );
    Ok(M3ConeAudit {
        bi4_result_digest: certificate.result_digest.clone(),
        public_replay_valid: true,
        public_replay_errors: replay.errors,
        exact_four_distinct_branch_roots: true,
        branch_root_count: metadata_count(4),
        enacted_branch_root: enacted[0].branch_root_hash.clone(),
        disposition_row_count: metadata_count(13),
        quantified_statement_count: metadata_count(35),
        every_disposition_mirrored_with_exact_quantifier: true,
        structural_testimony_used_for_law_level_content: false,
        claims,
        derivation_hash,
    })
}

fn membership_digest(pairs: &[(String, String)]) -> String {
    tagged_hash("enacted-membership-pairs", pairs)
}

fn e5_projection(
    certificate: &E5FutureHoleFinaleV2Certificate,
) -> Result<(M3EnactedProjection, Vec<(String, String)>), M3Error> {
    let mut pairs = certificate
        .stage16
        .membership_rows
        .iter()
        .map(|row| (row.a3_instance_id.clone(), row.a3_scheme_id.clone()))
        .collect::<Vec<_>>();
    pairs.sort();
    if pairs.windows(2).any(|window| window[0] == window[1]) {
        return Err(M3Error::Invariant(
            "frozen E-5 contains duplicate membership pairs".to_owned(),
        ));
    }
    let fields = (
        certificate.stage16.exact_a3_inventory_count,
        certificate.stage16.unary_registration_count,
        certificate.stage16.direct_chronological_instance_count,
        certificate.stage16.pointwise_chronological_instance_count,
        certificate.stage16.higher_instance_count,
        certificate.stage16.structural_instance_count,
        certificate.historical.structural_registration_count,
        certificate.historical.structural_realization_count,
        pairs.len(),
        membership_digest(&pairs),
        certificate.stage16.d_partition_complete,
        certificate.stage16.semantic_o16_empty == Some(true),
        certificate.f1.f1_executed,
        certificate.f1.f1_excluded == Some(true),
        certificate.f1.theorem12_full_instance_granularity_proved == Some(true),
        certificate
            .historical
            .focus_projection_reproduces_entire_coarse_ladder,
        certificate.historical.stage_three_wrinkle_reproduced,
        certificate.e5_complete,
    );
    let projection = M3EnactedProjection {
        exact_a3_inventory: semantic_instance_count(fields.0),
        unary_families: semantic_count(fields.1),
        direct_chronological_instances: structural_count(fields.2),
        pointwise_chronological_families: semantic_count(fields.3),
        higher_families: semantic_count(fields.4),
        final_window_structural_instances: structural_count(fields.5),
        historical_structural_registrations: structural_count(fields.6),
        historical_structural_realizations: structural_count(fields.7),
        membership_pair_count: semantic_instance_count(fields.8),
        membership_pair_digest: fields.9,
        exact_d_partition: fields.10,
        semantic_o16_empty: fields.11,
        f1_executed: fields.12,
        f1_excluded: fields.13,
        theorem12_full_instance_granularity: fields.14,
        focus_projection_reproduces_ladder: fields.15,
        stage_three_wrinkle_reproduced: fields.16,
        e5_complete: fields.17,
        derivation_hash: String::new(),
    };
    let mut projection = projection;
    projection.derivation_hash = tagged_hash("enacted-projection", &projection);
    Ok((projection, pairs))
}

fn bi2_projection(
    certificate: &Bi2BranchFinaleCertificateV1,
) -> Result<(M3EnactedProjection, Vec<(String, String)>), M3Error> {
    let mut pairs = certificate
        .finale
        .membership
        .iter()
        .map(|row| (row.a3_instance_id.clone(), row.a3_scheme_id.clone()))
        .collect::<Vec<_>>();
    pairs.sort();
    if pairs.windows(2).any(|window| window[0] == window[1]) {
        return Err(M3Error::Invariant(
            "enacted BI-2 contains duplicate membership pairs".to_owned(),
        ));
    }
    let fields = (
        certificate.finale.final_a3_inventory_count,
        certificate.finale.unary_registration_count,
        certificate.finale.final_direct_chronological_count,
        certificate.finale.final_pointwise_chronological_count,
        certificate.finale.final_higher_count,
        certificate.finale.final_structural_count,
        certificate.finale.structural_registration_count,
        certificate.finale.structural_realization_count,
        pairs.len(),
        membership_digest(&pairs),
        certificate.finale.exact_d_partition,
        certificate.finale.semantic_successor_o_empty,
        certificate.finale.f1_executed,
        certificate.finale.f1_excluded,
        certificate
            .finale
            .theorem12_full_instance_granularity_proved,
        certificate.finale.focus_projection_reproduces_branch_ladder,
        certificate.finale.stage3_wrinkle_reproduced,
        certificate.finale.e5_class_complete,
    );
    let projection = M3EnactedProjection {
        exact_a3_inventory: semantic_instance_count(fields.0),
        unary_families: semantic_count(fields.1),
        direct_chronological_instances: structural_count(fields.2),
        pointwise_chronological_families: semantic_count(fields.3),
        higher_families: semantic_count(fields.4),
        final_window_structural_instances: structural_count(fields.5),
        historical_structural_registrations: structural_count(fields.6),
        historical_structural_realizations: structural_count(fields.7),
        membership_pair_count: semantic_instance_count(fields.8),
        membership_pair_digest: fields.9,
        exact_d_partition: fields.10,
        semantic_o16_empty: fields.11,
        f1_executed: fields.12,
        f1_excluded: fields.13,
        theorem12_full_instance_granularity: fields.14,
        focus_projection_reproduces_ladder: fields.15,
        stage_three_wrinkle_reproduced: fields.16,
        e5_complete: fields.17,
        derivation_hash: String::new(),
    };
    let mut projection = projection;
    projection.derivation_hash = tagged_hash("enacted-projection", &projection);
    Ok((projection, pairs))
}

fn build_enacted_regression(
    bi4: &Bi4ConeAssemblyV2Certificate,
    cone: &M3ConeAudit,
    e5: &E5FutureHoleFinaleV2Certificate,
    bi2: &Bi2BranchFinaleCertificateV1,
) -> Result<M3EnactedRegressionAudit, M3Error> {
    let e5_self_digest_valid = e5.result_digest == frozen_e5_digest(e5);
    let bi2_self_digest_valid = bi2.result_digest == frozen_bi2_digest(bi2);
    if !e5_self_digest_valid || !bi2_self_digest_valid {
        return Err(M3Error::Input(
            "frozen enacted regression testimony failed its internal digest".to_owned(),
        ));
    }
    let native_e5 = replay_e5_future_hole_finale_v2_certificate(e5);
    if native_e5.valid {
        return Err(M3Error::Invariant(
            "M-3 was specified against disclosed native E-5 replay drift, but native replay unexpectedly passed; issue a successor rather than silently changing fallback semantics"
                .to_owned(),
        ));
    }
    if native_e5.errors.len() != 1 || native_e5.errors[0] != M3_REGISTERED_E5_DRIFT {
        return Err(M3Error::Input(format!(
            "native E-5 failed for an unregistered reason: {}",
            native_e5.errors.join("; ")
        )));
    }
    let enacted_joins = bi4.branches.iter().any(|branch| {
        branch.enacted_root
            && branch.branch_root_hash == bi2.branch_root_hash
            && branch.branch_certificate_digest == bi2.result_digest
            && branch.branch_root_hash == cone.enacted_branch_root
    });
    if !enacted_joins {
        return Err(M3Error::Invariant(
            "enacted BI-2 does not exactly join the sealed BI-4 enacted branch".to_owned(),
        ));
    }

    let (e5_projection, e5_pairs) = e5_projection(e5)?;
    let (bi2_projection, bi2_pairs) = bi2_projection(bi2)?;
    if e5_pairs != bi2_pairs || e5_projection != bi2_projection {
        return Err(M3Error::Invariant(
            "enacted bridge regression diverges between frozen E-5 and sealed BI-2".to_owned(),
        ));
    }
    if e5_projection.exact_a3_inventory.value != 89
        || e5_projection.unary_families.value != 17
        || e5_projection.direct_chronological_instances.value != 64
        || e5_projection.pointwise_chronological_families.value != 8
        || e5_projection.higher_families.value != 0
        || e5_projection.final_window_structural_instances.value != 0
        || e5_projection.historical_structural_registrations.value != 13
        || e5_projection.historical_structural_realizations.value != 13
        || !e5_projection.exact_d_partition
        || !e5_projection.semantic_o16_empty
        || !e5_projection.f1_executed
        || !e5_projection.f1_excluded
        || !e5_projection.theorem12_full_instance_granularity
        || !e5_projection.focus_projection_reproduces_ladder
        || !e5_projection.stage_three_wrinkle_reproduced
        || !e5_projection.e5_complete
    {
        return Err(M3Error::Invariant(
            "enacted regression does not reproduce the frozen E-5/BI-2 projection".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "enacted-regression",
        &(
            e5.result_digest.as_str(),
            bi2.result_digest.as_str(),
            &native_e5.errors,
            &e5_projection,
        ),
    );
    Ok(M3EnactedRegressionAudit {
        quantifier: M3Quantifier::FrozenEnactedRegression,
        e5_frozen_result_digest: e5.result_digest.clone(),
        e5_frozen_self_digest_valid: true,
        e5_native_replay_valid: false,
        e5_native_replay_errors: native_e5.errors,
        frozen_testimony_fallback_used: true,
        fallback_limited_to_exact_digest_bound_frozen_e5: true,
        drift_repaired_inside_m3: false,
        drift_concealed_inside_m3: false,
        enacted_bi2_result_digest: bi2.result_digest.clone(),
        enacted_bi2_self_digest_valid: true,
        enacted_bi2_root_exactly_joins_bi4: true,
        e5_projection: e5_projection.clone(),
        bi2_projection,
        projections_exactly_equal: true,
        cross_branch_statements_issued_only_after_regression: true,
        derivation_hash,
    })
}

fn schema3_aggregate_audit() -> Result<(usize, usize, u128), M3Error> {
    let archive: Value =
        serde_json::from_slice(SCHEMA3_BYTES).map_err(|error| M3Error::Json(error.to_string()))?;
    let strata = archive
        .get("strata")
        .and_then(Value::as_array)
        .ok_or_else(|| M3Error::Input("schema-3 archive lacks strata".to_owned()))?;
    let mut row_count = 0usize;
    let mut minimum = u128::MAX;
    for stratum in strata {
        let rows = stratum
            .get("rows")
            .and_then(Value::as_array)
            .ok_or_else(|| M3Error::Input("schema-3 stratum lacks rows".to_owned()))?;
        for row in rows {
            if row.get("candidate").is_some() || row.get("telescope").is_some() {
                return Err(M3Error::Invariant(
                    "schema-3 aggregate row unexpectedly embeds a candidate or telescope"
                        .to_owned(),
                ));
            }
            let count = row
                .get("count")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    M3Error::Input("schema-3 row lacks decimal aggregate count".to_owned())
                })?
                .parse::<u128>()
                .map_err(|error| M3Error::Input(error.to_string()))?;
            if count <= 1 {
                return Err(M3Error::Invariant(
                    "schema-3 row is not an aggregate parent bucket".to_owned(),
                ));
            }
            minimum = minimum.min(count);
            row_count += 1;
        }
    }
    if strata.len() != 3 || row_count != 213 || minimum == u128::MAX {
        return Err(M3Error::Invariant(format!(
            "schema-3 aggregate surface expected 3 strata/213 rows, found {}/{}",
            strata.len(),
            row_count
        )));
    }
    Ok((strata.len(), row_count, minimum))
}

fn bridge_condition(
    ordinal: u8,
    condition: &str,
    proved: bool,
    named_gap: Option<&str>,
) -> M3BridgeCondition {
    let derivation_hash = tagged_hash(
        "e8-bridge-condition",
        &(ordinal, condition, proved, named_gap),
    );
    M3BridgeCondition {
        ordinal: syntax_identifier(usize::from(ordinal)),
        condition: condition.to_owned(),
        proved,
        named_gap: named_gap.map(ToOwned::to_owned),
        derivation_hash,
    }
}

fn build_e7_audit(schema4: &CandidateJoinV4Certificate) -> Result<M3E7Audit, M3Error> {
    if schema4.schema_version != CANDIDATE_JOIN_V4_SCHEMA_VERSION
        || !schema4.completeness.exact_schema3_archive_bound
        || !schema4.completeness.allowed_projection_only
        || !schema4.completeness.all_213_rows_joined
        || schema4.completeness.c2_depth_two_schema_completeness_proved
        || schema4.completeness.intended_semantic_candidate_join_proved
    {
        return Err(M3Error::Invariant(
            "schema-4 does not expose the registered fail-closed E-7 boundary".to_owned(),
        ));
    }
    let fields = (
        M3Quantifier::FrozenWrappedSurface,
        M3Register::SemanticRegisterAuthority,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        M3_E7_GAP,
        true,
    );
    Ok(M3E7Audit {
        quantifier: fields.0,
        register: fields.1,
        slnf_defined_independently: fields.2,
        classify_w_total: fields.3,
        faithfulness_proved: fields.4,
        realization_claimed: fields.5,
        inverse_laws_proved: fields.6,
        one_way_injection_declared_and_proved: fields.7,
        class_by_class_generator_completeness_proved: fields.8,
        c2_closed: fields.9,
        named_gap: fields.10.to_owned(),
        no_support_local_code_pruned_from_failed_realization: fields.11,
        derivation_hash: tagged_hash("e7-audit", &fields),
    })
}

fn build_e8_audit(
    schema4: &CandidateJoinV4Certificate,
    schema5: &CandidateJoinV5Certificate,
) -> Result<M3E8Audit, M3Error> {
    let (strata_count, row_count, minimum) = schema3_aggregate_audit()?;
    let replay = replay_candidate_join_v5_json(
        SCHEMA3_BYTES,
        SCHEMA4_BYTES,
        std::str::from_utf8(SCHEMA5_BYTES).map_err(|error| M3Error::Input(error.to_string()))?,
    );
    if !replay.valid {
        return Err(M3Error::Input(format!(
            "schema-5 public replay failed: {}",
            replay.errors.join("; ")
        )));
    }
    let c4 = &schema4.completeness;
    let c5 = &schema5.completeness;
    if !c4.exact_schema3_archive_bound
        || !c4.allowed_projection_only
        || !c4.all_213_rows_joined
        || !c4.no_row_promoted_across_open_gap
        || c4.c8_candidate_boundary_provenance_join_proved
        || c4.full_candidate_extraction_join_proved
        || c4.intended_semantic_candidate_join_proved
        || !c5.exact_schema4_archive_bound_and_replayed
        || !c5.all_213_schema4_rows_joined_by_row_digest
        || !c5.c8_registered_reference_only_charging_proved
        || c5.c8_candidate_boundary_provenance_join_proved
        || !c5.no_row_promoted
        || c5.full_candidate_extraction_join_proved
        || c5.intended_semantic_candidate_join_proved
    {
        return Err(M3Error::Invariant(
            "schema-4/schema-5 do not expose the exact conservative E-8 boundary".to_owned(),
        ));
    }

    let conditions = vec![
        bridge_condition(
            1,
            "bind the exact archived schema-3 bytes and digest",
            true,
            None,
        ),
        bridge_condition(2, "use only the approved schema-3 projection", true, None),
        bridge_condition(
            3,
            "elaborate every complete candidate telescope with composed fuel",
            false,
            Some(M3_PARENT_ROW_GAP),
        ),
        bridge_condition(
            4,
            "extract every natural family and semantic demand orbit",
            false,
            Some(M3_E8_GAP),
        ),
        bridge_condition(
            5,
            "decide marginality and candidate-local provenance for every family",
            false,
            Some(M3_E8_GAP),
        ),
        bridge_condition(
            6,
            "attach the E-7 classifier derivation",
            false,
            Some(M3_E7_GAP),
        ),
        bridge_condition(
            7,
            "attach registered V2 reference-only boundary provenance where relevant",
            true,
            None,
        ),
        bridge_condition(
            8,
            "reject every missing duplicate or unjoined family and orbit",
            false,
            Some(M3_PARENT_ROW_GAP),
        ),
        bridge_condition(
            9,
            "keep every row with a named theorem gap unpromoted",
            true,
            None,
        ),
    ];
    let all_nine = conditions.len() == 9 && conditions.iter().all(|condition| condition.proved);
    let named_gaps = vec![
        M3_E7_GAP.to_owned(),
        M3_E8_GAP.to_owned(),
        M3_PARENT_ROW_GAP.to_owned(),
    ];
    let derivation_hash = tagged_hash(
        "e8-audit",
        &(
            strata_count,
            row_count,
            minimum.to_string(),
            &conditions,
            &named_gaps,
            &replay.errors,
        ),
    );
    Ok(M3E8Audit {
        quantifier: M3Quantifier::FrozenWrappedSurface,
        register: M3Register::SemanticRegisterAuthority,
        schema3_strata: metadata_count(strata_count),
        schema3_aggregate_rows: metadata_count(row_count),
        every_schema3_row_is_aggregate_bucket: true,
        minimum_parent_candidate_multiplicity: structural_count(usize::try_from(minimum).map_err(
            |_| {
                M3Error::Invariant(
                    "minimum schema-3 parent multiplicity does not fit certificate count"
                        .to_owned(),
                )
            },
        )?),
        schema4_all_rows_structurally_joined: true,
        schema5_public_replay_valid: true,
        schema5_public_replay_errors: replay.errors,
        schema5_no_row_promoted: true,
        bridge_condition_count: metadata_count(conditions.len()),
        proved_bridge_condition_count: metadata_count(
            conditions
                .iter()
                .filter(|condition| condition.proved)
                .count(),
        ),
        bridge_conditions: conditions,
        all_nine_conditions_proved: all_nine,
        candidate_level_join_proved: false,
        c8_closed: false,
        domain_wide_c3_closed: false,
        named_gaps,
        derivation_hash,
    })
}

fn named_gap(id: &str, phase: &str, exact_obstruction: &str) -> M3NamedGap {
    let quantifier = M3Quantifier::FrozenWrappedSurface;
    let register = M3Register::SemanticRegisterAuthority;
    let derivation_hash = tagged_hash(
        "named-gap",
        &(id, phase, &quantifier, &register, exact_obstruction),
    );
    M3NamedGap {
        id: id.to_owned(),
        phase: phase.to_owned(),
        quantifier,
        register,
        exact_obstruction: exact_obstruction.to_owned(),
        keeps_rows_unpromoted: true,
        derivation_hash,
    }
}

fn build_certificate() -> Result<M3E7E8BridgeV1Certificate, M3Error> {
    let bi4: Bi4ConeAssemblyV2Certificate = serde_json::from_slice(BI4_CERTIFICATE_BYTES)
        .map_err(|error| M3Error::Json(error.to_string()))?;
    let e5: E5FutureHoleFinaleV2Certificate = serde_json::from_slice(E5_CERTIFICATE_BYTES)
        .map_err(|error| M3Error::Json(error.to_string()))?;
    let enacted_bi2: Bi2BranchFinaleCertificateV1 =
        serde_json::from_slice(ENACTED_BI2_CERTIFICATE_BYTES)
            .map_err(|error| M3Error::Json(error.to_string()))?;
    let schema4: CandidateJoinV4Certificate =
        serde_json::from_slice(SCHEMA4_BYTES).map_err(|error| M3Error::Json(error.to_string()))?;
    let schema5: CandidateJoinV5Certificate =
        serde_json::from_slice(SCHEMA5_BYTES).map_err(|error| M3Error::Json(error.to_string()))?;

    // F-BI5 ordering: the enacted regression completes before any cone-aware
    // bridge statement is retained in the new certificate.
    let provisional_cone = build_cone_audit(&bi4)?;
    let enacted_regression = build_enacted_regression(&bi4, &provisional_cone, &e5, &enacted_bi2)?;
    if !enacted_regression.projections_exactly_equal {
        return Err(M3Error::Invariant(
            "cross-branch statement attempted before enacted regression".to_owned(),
        ));
    }
    let cone = provisional_cone;
    let e7 = build_e7_audit(&schema4)?;
    let e8 = build_e8_audit(&schema4, &schema5)?;
    let open_gaps = vec![
        named_gap(
            M3_E7_GAP,
            "E-7",
            "No independently defined support-local semantic normal-form type, total classifier, faithfulness theorem, exact realization/inverse laws, or explicitly declared-and-proved one-way injection exists on the intended depth-two semantic domain.",
        ),
        named_gap(
            M3_E8_GAP,
            "E-8",
            "The conservative schema-4/schema-5 lineage joins the complete bound aggregate-row surface structurally but exports no total candidate-to-telescope-to-Marg2 provenance join satisfying every bridge condition.",
        ),
        named_gap(
            M3_PARENT_ROW_GAP,
            "E-8 prerequisite",
            "Each schema-3 row is an aggregate bucket with non-singleton multiplicity; no theorem refines every parent candidate or proves a universal candidate-level result for the bucket.",
        ),
    ];
    if e7.c2_closed
        || e8.c8_closed
        || e8.candidate_level_join_proved
        || e8.all_nine_conditions_proved
        || open_gaps.len() != 3
    {
        return Err(M3Error::Invariant(
            "M-3 attempted to cross an open E-7/E-8 theorem boundary".to_owned(),
        ));
    }

    let mut certificate = M3E7E8BridgeV1Certificate {
        schema: M3_E7_E8_BRIDGE_V1_SCHEMA.to_owned(),
        date: M3_E7_E8_BRIDGE_V1_DATE.to_owned(),
        syntax_identifier_policy: "Numeric substrings in dates, artifact versions, theorem IDs, stage labels, and obligation labels are identifiers rather than quantitative evidence.".to_owned(),
        syntax_identifier_register: M3Register::SyntaxIdentifier,
        source_bindings: source_bindings(),
        cone,
        enacted_regression,
        e7,
        e8,
        open_gaps,
        open_gap_count: metadata_count(3),
        zero_promotions_made: true,
        promoted_row_count: metadata_count(0),
        uc1_cited_or_anticipated: false,
        bridge_claim_issued: false,
        m3_status: M3RunStatus::StoppedNamedGaps,
        m4_authorized: false,
        mutation_falsifiers: vec![
            "change_any_bound_input_byte_or_digest_then_replay_must_fail".to_owned(),
            "flip_any_BI4_claim_quantifier_or_branch_root_then_replay_must_fail".to_owned(),
            "retag_any_semantic_or_structural_quantity_then_replay_must_fail".to_owned(),
            "conceal_repair_or_expand_the_native_E5_drift_fallback_then_replay_must_fail"
                .to_owned(),
            "change_any_enacted_projection_value_or_membership_pair_then_replay_must_fail"
                .to_owned(),
            "remove_or_rename_any_E7_E8_or_parent_row_gap_then_replay_must_fail".to_owned(),
            "promote_any_schema_row_or_claim_E7_E8_domain_wide_C3_or_bridge_then_replay_must_fail"
                .to_owned(),
            "cite_UC1_or_authorize_M4_while_M3_is_stopped_then_replay_must_fail".to_owned(),
            "add_any_unknown_JSON_field_then_replay_must_fail".to_owned(),
        ],
        permitted_conclusion: "The exact BI-4 cone and enacted E-5/BI-2 semantic projection are authenticated, including semantic O(16) = empty and F1 excluded at the quantifiers BI-4 permits. M-3 does not prove the intended depth-two semantic classifier or the total candidate-level bridge. It stops with the complete named-gap list and makes no promotions.".to_owned(),
        required_successor_action: "Define and prove E-7 (or explicitly declare and prove its one-way injection fallback), refine each schema-3 aggregate bucket to all parent candidates or prove a universal parent-row theorem, implement every E-8 join condition, and rerun M-3 create-new. M-4 remains unauthorized until that replay passes.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<Result<M3E7E8BridgeV1Certificate, String>> = OnceLock::new();

fn expected_certificate() -> Result<&'static M3E7E8BridgeV1Certificate, M3Error> {
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(M3Error::Input(error.clone())),
    }
}

pub fn issue_m3_e7_e8_bridge_v1() -> Result<M3E7E8BridgeV1Certificate, M3Error> {
    expected_certificate().cloned()
}

fn invalid_replay(error: impl Into<String>) -> M3Replay {
    M3Replay {
        valid: false,
        errors: vec![error.into()],
        m3_status: M3RunStatus::StoppedNamedGaps,
        bridge_claim_issued: false,
        m4_authorized: false,
    }
}

pub fn replay_m3_e7_e8_bridge_v1(claimed: &M3E7E8BridgeV1Certificate) -> M3Replay {
    let expected = match expected_certificate() {
        Ok(value) => value,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("M-3 certificate digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("M-3 source bindings drifted".to_owned());
    }
    if claimed != expected {
        errors.push("M-3 certificate differs from deterministic reissuance".to_owned());
    }
    M3Replay {
        valid: errors.is_empty(),
        errors,
        m3_status: claimed.m3_status.clone(),
        bridge_claim_issued: claimed.bridge_claim_issued,
        m4_authorized: claimed.m4_authorized,
    }
}

pub fn replay_m3_e7_e8_bridge_v1_json(json: &str) -> M3Replay {
    match serde_json::from_str::<M3E7E8BridgeV1Certificate>(json) {
        Ok(certificate) => replay_m3_e7_e8_bridge_v1(&certificate),
        Err(error) => invalid_replay(format!("invalid M-3 JSON: {error}")),
    }
}

fn register_label(register: M3Register) -> &'static str {
    match register {
        M3Register::SemanticRegisterAuthority => "semantic_register_authority",
        M3Register::SemanticFamilyAuthority => "semantic_family_authority",
        M3Register::SemanticDemandInstanceAuthority => "semantic_demand_instance_authority",
        M3Register::StructuralTestimony => "structural_testimony",
        M3Register::ArtifactMetadata => "artifact_metadata",
        M3Register::SyntaxIdentifier => "syntax_identifier",
    }
}

fn render_count(count: &M3TaggedCount) -> String {
    format!(
        "{} [register: `{}`]",
        count.value,
        register_label(count.register)
    )
}

pub fn render_m3_e7_e8_bridge_v1(certificate: &M3E7E8BridgeV1Certificate) -> String {
    let mut out = String::new();
    out.push_str("# M-3 E-7/E-8 bridge result\n\n");
    out.push_str(&format!(
        "**Date (syntax identifier):** {}. **Status:** `stopped_named_gaps`. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("Labels in names (M-3, E-7/E-8, BI-4, E-5/BI-2, Schema-3/4/5, Stage-3, and O(16)) are syntax identifiers, not quantitative evidence. Every quantitative value below carries its register explicitly.\n\n");
    out.push_str(&format!(
        "M-3 stops fail-closed. The BI-4 cone replays, and the enacted branch reproduces the frozen E-5/BI-2 semantic projection exactly before any cone-aware statement is retained. Promoted rows: **{}**. No bridge claim is issued, and M-4 is not authorized.\n\n",
        render_count(&certificate.promoted_row_count),
    ));
    out.push_str("## Enacted regression\n\n");
    out.push_str(&format!(
        "The exact projection is **{}** A3 instances = **{}** unary + **{}** direct chronological instances / **{}** pointwise families + **{}** higher + **{}** final-window structural, with **{}** historical structural registrations and **{}** realizations. Semantic `O(16) = empty`: **{}**; F1 executed/excluded: **{}/{}**; Stage-3 wrinkle reproduced: **{}**.\n\n",
        render_count(&certificate.enacted_regression.e5_projection.exact_a3_inventory),
        render_count(&certificate.enacted_regression.e5_projection.unary_families),
        render_count(&certificate.enacted_regression.e5_projection.direct_chronological_instances),
        render_count(&certificate.enacted_regression.e5_projection.pointwise_chronological_families),
        render_count(&certificate.enacted_regression.e5_projection.higher_families),
        render_count(&certificate.enacted_regression.e5_projection.final_window_structural_instances),
        render_count(&certificate.enacted_regression.e5_projection.historical_structural_registrations),
        render_count(&certificate.enacted_regression.e5_projection.historical_structural_realizations),
        certificate.enacted_regression.e5_projection.semantic_o16_empty,
        certificate.enacted_regression.e5_projection.f1_executed,
        certificate.enacted_regression.e5_projection.f1_excluded,
        certificate.enacted_regression.e5_projection.stage_three_wrinkle_reproduced,
    ));
    out.push_str(&format!(
        "Native E-5 replay passed: **{}**. Exact frozen-testimony fallback used: **{}**. The disclosed replay error is `{}`. M-3 repaired or concealed no drift.\n\n",
        certificate.enacted_regression.e5_native_replay_valid,
        certificate.enacted_regression.frozen_testimony_fallback_used,
        certificate.enacted_regression.e5_native_replay_errors.join("; "),
    ));
    out.push_str("## Cone quantifiers and registers\n\n");
    out.push_str(&format!(
        "The exact **{}** BI-4 disposition rows become **{}** explicit statements: cone-level rows quantify over **{}** exact roots; branch-indexed rows are repeated once per named root. Structural testimony is never consumed as law-level semantic authority.\n\n",
        render_count(&certificate.cone.disposition_row_count),
        render_count(&certificate.cone.quantified_statement_count),
        render_count(&certificate.cone.branch_root_count),
    ));
    out.push_str("## E-7/E-8 stop\n\n");
    out.push_str(&format!(
        "Schema-3 contains **{}** aggregate rows in **{}** strata; the smallest bucket contains **{}** parent candidates. Schema-4/5 replay the structural joins and make no promotions, but only **{}** of **{}** candidate-bridge conditions are proved. Open gaps: **{}**.\n\n",
        render_count(&certificate.e8.schema3_aggregate_rows),
        render_count(&certificate.e8.schema3_strata),
        render_count(&certificate.e8.minimum_parent_candidate_multiplicity),
        render_count(&certificate.e8.proved_bridge_condition_count),
        render_count(&certificate.e8.bridge_condition_count),
        render_count(&certificate.open_gap_count),
    ));
    out.push_str("| Gap | Phase | Exact obstruction |\n|---|---|---|\n");
    for gap in &certificate.open_gaps {
        out.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            gap.id, gap.phase, gap.exact_obstruction
        ));
    }
    out.push_str("\n## Permitted conclusion\n\n");
    out.push_str(&certificate.permitted_conclusion);
    out.push_str("\n\n## Required successor\n\n");
    out.push_str(&certificate.required_successor_action);
    out.push('\n');
    out
}

pub fn emit_m3_e7_e8_bridge_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<M3E7E8BridgeV1Certificate, M3Error> {
    if certificate_path.exists() || report_path.exists() {
        return Err(M3Error::Io(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_m3_e7_e8_bridge_v1()?;
    let json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| M3Error::Json(error.to_string()))?;
    let replay = replay_m3_e7_e8_bridge_v1(&certificate);
    if !replay.valid {
        return Err(M3Error::Invariant(format!(
            "new M-3 certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(certificate_path)
        .map_err(|error| M3Error::Io(error.to_string()))?;
    let certificate_write = certificate_file
        .write_all(&json)
        .and_then(|_| certificate_file.write_all(b"\n"));
    drop(certificate_file);
    if let Err(error) = certificate_write {
        let _ = remove_file(certificate_path);
        return Err(M3Error::Io(error.to_string()));
    }
    let report = render_m3_e7_e8_bridge_v1(&certificate);
    let report_result = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(report_path)
        .and_then(|mut file| file.write_all(report.as_bytes()));
    if let Err(error) = report_result {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(M3Error::Io(error.to_string()));
    }
    let emitted_json = match read_to_string(certificate_path) {
        Ok(json) => json,
        Err(error) => {
            let _ = remove_file(certificate_path);
            let _ = remove_file(report_path);
            return Err(M3Error::Io(error.to_string()));
        }
    };
    let emitted_replay = replay_m3_e7_e8_bridge_v1_json(&emitted_json);
    if !emitted_replay.valid {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(M3Error::Invariant(format!(
            "emitted M-3 JSON did not replay: {}",
            emitted_replay.errors.join("; ")
        )));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::{
        M3_E7_GAP, M3_E8_GAP, M3_PARENT_ROW_GAP, M3Quantifier, M3Register, M3RunStatus,
        issue_m3_e7_e8_bridge_v1, render_m3_e7_e8_bridge_v1, replay_m3_e7_e8_bridge_v1,
        replay_m3_e7_e8_bridge_v1_json,
    };
    use std::collections::BTreeSet;

    #[test]
    fn issues_replayable_fail_closed_certificate() {
        let certificate = issue_m3_e7_e8_bridge_v1().expect("M-3 issues");
        let replay = replay_m3_e7_e8_bridge_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(certificate.m3_status, M3RunStatus::StoppedNamedGaps);
        assert!(!certificate.bridge_claim_issued);
        assert!(!certificate.m4_authorized);
        assert!(certificate.zero_promotions_made);
        assert_eq!(certificate.promoted_row_count.value, 0);
        assert_eq!(certificate.cone.claims.len(), 35);
        assert_eq!(certificate.e8.bridge_conditions.len(), 9);
        assert!(!certificate.e8.all_nine_conditions_proved);
        assert_eq!(
            certificate
                .enacted_regression
                .e5_projection
                .exact_a3_inventory
                .register,
            M3Register::SemanticDemandInstanceAuthority
        );
        assert_eq!(
            certificate
                .enacted_regression
                .e5_projection
                .pointwise_chronological_families
                .register,
            M3Register::SemanticFamilyAuthority
        );
        assert_eq!(
            certificate
                .enacted_regression
                .e5_projection
                .direct_chronological_instances
                .register,
            M3Register::StructuralTestimony
        );
        assert!(
            certificate
                .e8
                .bridge_conditions
                .iter()
                .all(|condition| condition.ordinal.register == M3Register::SyntaxIdentifier)
        );
        let g3a_registers = certificate
            .cone
            .claims
            .iter()
            .filter(|claim| {
                claim.claim == "G3a_per_stage_kappa_and_semantic_nu_vector_Stage5_through15"
            })
            .map(|claim| (claim.component.as_str(), claim.register))
            .collect::<BTreeSet<_>>();
        assert_eq!(
            g3a_registers,
            [
                ("kappa_vector", M3Register::StructuralTestimony),
                ("semantic_nu_vector", M3Register::SemanticFamilyAuthority),
            ]
            .into_iter()
            .collect()
        );
        let report = render_m3_e7_e8_bridge_v1(&certificate);
        assert!(report.contains("89 [register: `semantic_demand_instance_authority`]"));
        assert!(report.contains("64 [register: `structural_testimony`]"));
        assert!(report.contains("8 [register: `semantic_family_authority`]"));
        assert!(report.contains("4 [register: `artifact_metadata`]"));
    }

    #[test]
    fn exact_three_stop_gaps_are_retained() {
        let certificate = issue_m3_e7_e8_bridge_v1().expect("M-3 issues");
        let gaps = certificate
            .open_gaps
            .iter()
            .map(|gap| gap.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(gaps, vec![M3_E7_GAP, M3_E8_GAP, M3_PARENT_ROW_GAP]);
        assert!(
            certificate
                .open_gaps
                .iter()
                .all(|gap| gap.keeps_rows_unpromoted)
        );
    }

    #[test]
    fn verdict_gap_quantifier_register_and_digest_mutations_fail() {
        let certificate = issue_m3_e7_e8_bridge_v1().expect("M-3 issues");

        let mut verdict = certificate.clone();
        verdict.bridge_claim_issued = true;
        assert!(!replay_m3_e7_e8_bridge_v1(&verdict).valid);

        let mut e7_verdict = certificate.clone();
        e7_verdict.e7.c2_closed = true;
        assert!(!replay_m3_e7_e8_bridge_v1(&e7_verdict).valid);

        let mut authorization = certificate.clone();
        authorization.m4_authorized = true;
        assert!(!replay_m3_e7_e8_bridge_v1(&authorization).valid);

        let mut gap = certificate.clone();
        gap.open_gaps[0].id.push_str("_MUTATED");
        assert!(!replay_m3_e7_e8_bridge_v1(&gap).valid);

        let mut quantifier = certificate.clone();
        quantifier.cone.claims[0].quantifier = M3Quantifier::FrozenWrappedSurface;
        assert!(!replay_m3_e7_e8_bridge_v1(&quantifier).valid);

        let mut register = certificate.clone();
        register.e8.schema3_aggregate_rows.register = M3Register::SemanticFamilyAuthority;
        assert!(!replay_m3_e7_e8_bridge_v1(&register).valid);

        let mut source = certificate.clone();
        source.source_bindings[0].blake3.push('0');
        assert!(!replay_m3_e7_e8_bridge_v1(&source).valid);

        let mut fallback = certificate.clone();
        fallback
            .enacted_regression
            .e5_native_replay_errors
            .push("unregistered drift".to_owned());
        assert!(!replay_m3_e7_e8_bridge_v1(&fallback).valid);

        let mut promotion = certificate.clone();
        promotion.promoted_row_count.value = 1;
        assert!(!replay_m3_e7_e8_bridge_v1(&promotion).valid);

        let mut digest = certificate;
        digest.result_digest.push('0');
        assert!(!replay_m3_e7_e8_bridge_v1(&digest).valid);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_m3_e7_e8_bridge_v1().expect("M-3 issues");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("unknown_field".to_owned(), serde_json::Value::Bool(true));
        let replay = replay_m3_e7_e8_bridge_v1_json(&serde_json::to_string(&value).expect("json"));
        assert!(!replay.valid);
    }
}
