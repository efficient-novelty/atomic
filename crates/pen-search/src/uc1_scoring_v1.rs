//! Mechanical UC-1 scoring over the sealed BI cone.
//!
//! This module observes no new candidate and constructs no new equivalence.
//! It authenticates the already-sealed BI-1/BI-1b/BI-2/BI-4 and R-T2
//! lineages, performs the one registered fork-shadow relabeling comparison,
//! and scores P1--P3 plus F-UC4.  Every published quantity is explicitly
//! registered under `nu-register-split-v1`.

use crate::bi1b_prefix_general_sweep_v1::{
    Bi1bBranchCertificateV1, Bi1bCorrespondenceDiagnosticV1,
};
use crate::bi2_branch_finales_v1::{
    Bi2FourBranchFinalesBundleV1, replay_bi2_four_branch_finales_v1,
};
use crate::bi4_cone_assembly_v2::{
    Bi4ConeAssemblyV2Certificate, Bi4VerdictV2, replay_bi4_cone_assembly_v2,
};
use crate::branch_invariance_sweep_v3::Bi1BranchCertificateV3;
use crate::r_t2_future_hole_confluence_v2::{
    R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA, Rt2FutureHoleConfluenceV2Certificate,
    Rt2V2PairwiseSchemeSetComparison, replay_archived_stage4_fork_projection,
    replay_r_t2_future_hole_confluence_v2_certificate,
};
use crate::semantic_nu_transport_maps_v2::{
    NumericRegister as TransportNumericRegister, SemanticNuTransportMapsV2Certificate,
    replay_semantic_nu_transport_maps_v2,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const UC1_SCORING_V1_SCHEMA: &str = "uc1-scoring-v1";
pub const UC1_SCORING_V1_DATE: &str = "2026-07-23";
pub const UC1_SCORING_V1_CERTIFICATE_NAME: &str = "UC1_SCORING_V1_CERTIFICATE.json";
pub const UC1_SCORING_V1_REPORT_NAME: &str = "UC1_SCORING_V1_RESULT.md";
pub const UC1_SCORING_V1_ZONE: &str = "Z-LEDGER";
pub const UC1_SCORING_V1_RESIDUAL: &str = "U_T2_UNATTEMPTED";

const MAINLINE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/mainline_completion_plan.md");
const UC1_HYPOTHESIS_BYTES: &[u8] =
    include_bytes!("../../../docs/univalent_collapse_hypothesis.md");
const NU_REGISTER_BYTES: &[u8] = include_bytes!("../../../docs/nu_register_adjudication.md");
const M1_TRANSPORT_BYTES: &[u8] =
    include_bytes!("../../../docs/semantic_nu_transport_maps_v2.json");
const BI4_BYTES: &[u8] = include_bytes!("../../../docs/BI4_CONE_V2_CERTIFICATE.json");
const R_T2_BYTES: &[u8] =
    include_bytes!("../../../docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json");
const BI1_201_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json");
const BI1_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json");
const BI1_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json");
const BI1_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json");
const BI1B_201_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json");
const BI1B_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json");
const BI1B_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json");
const BI1B_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json");
const CORRESPONDENCE_BYTES: &[u8] =
    include_bytes!("../../../docs/BI1B_CORRESPONDENCE_DIAGNOSTIC.json");
const BI2_INDEX_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json");
const BI2_201_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_2016726758f3_V2_CERTIFICATE.json");
const BI2_43_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_43a0ed707770_V2_CERTIFICATE.json");
const BI2_4B_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_4b2211ecae25_V2_CERTIFICATE.json");
const BI2_B4_BYTES: &[u8] =
    include_bytes!("../../../docs/BI2_BRANCH_b4f821d9bb28_V2_CERTIFICATE.json");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("uc1_scoring_v1.rs");
const EXAMPLE_SOURCE_BYTES: &[u8] = include_bytes!("../examples/uc1_scoring_v1.rs");

const BI1_INPUTS: [(&str, &[u8]); 4] = [
    (
        "docs/BI_BRANCH_V3_2016726758f3_CERTIFICATE.json",
        BI1_201_BYTES,
    ),
    (
        "docs/BI_BRANCH_V3_43a0ed707770_CERTIFICATE.json",
        BI1_43_BYTES,
    ),
    (
        "docs/BI_BRANCH_V3_4b2211ecae25_CERTIFICATE.json",
        BI1_4B_BYTES,
    ),
    (
        "docs/BI_BRANCH_V3_b4f821d9bb28_CERTIFICATE.json",
        BI1_B4_BYTES,
    ),
];

const BI1B_INPUTS: [(&str, &[u8]); 4] = [
    (
        "docs/BI1B_BRANCH_2016726758f3_CERTIFICATE.json",
        BI1B_201_BYTES,
    ),
    (
        "docs/BI1B_BRANCH_43a0ed707770_CERTIFICATE.json",
        BI1B_43_BYTES,
    ),
    (
        "docs/BI1B_BRANCH_4b2211ecae25_CERTIFICATE.json",
        BI1B_4B_BYTES,
    ),
    (
        "docs/BI1B_BRANCH_b4f821d9bb28_CERTIFICATE.json",
        BI1B_B4_BYTES,
    ),
];

const BI2_BRANCH_INPUTS: [(&str, &[u8]); 4] = [
    (
        "docs/BI2_BRANCH_2016726758f3_V2_CERTIFICATE.json",
        BI2_201_BYTES,
    ),
    (
        "docs/BI2_BRANCH_43a0ed707770_V2_CERTIFICATE.json",
        BI2_43_BYTES,
    ),
    (
        "docs/BI2_BRANCH_4b2211ecae25_V2_CERTIFICATE.json",
        BI2_4B_BYTES,
    ),
    (
        "docs/BI2_BRANCH_b4f821d9bb28_V2_CERTIFICATE.json",
        BI2_B4_BYTES,
    ),
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Uc1NumericRegister {
    SemanticFamilyNu,
    ProofInventory,
    StructuralTestimony,
    SyntaxIdentifier,
    ArtifactMetadata,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredNumber {
    pub value: u64,
    pub register: Uc1NumericRegister,
    pub meaning: String,
}

fn registered_usize(value: usize, register: Uc1NumericRegister, meaning: &str) -> RegisteredNumber {
    RegisteredNumber {
        value: u64::try_from(value).expect("UC-1 proof inventory fits u64"),
        register,
        meaning: meaning.to_owned(),
    }
}

fn registered_u32(value: u32, register: Uc1NumericRegister, meaning: &str) -> RegisteredNumber {
    RegisteredNumber {
        value: u64::from(value),
        register,
        meaning: meaning.to_owned(),
    }
}

fn proof_inventory(value: usize, meaning: &str) -> RegisteredNumber {
    registered_usize(value, Uc1NumericRegister::ProofInventory, meaning)
}

fn syntax_identifier(value: u32, meaning: &str) -> RegisteredNumber {
    registered_u32(value, Uc1NumericRegister::SyntaxIdentifier, meaning)
}

fn semantic_family_nu(value: u64, meaning: &str) -> RegisteredNumber {
    RegisteredNumber {
        value,
        register: Uc1NumericRegister::SemanticFamilyNu,
        meaning: meaning.to_owned(),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1SourceBindingV1 {
    pub path: String,
    pub role: String,
    pub byte_length: RegisteredNumber,
    pub blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PredictionVerdict {
    Passed,
    Refuted,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimQuantifier {
    ConeAllBranchesStageFiveThroughFifteen,
    EveryUnorderedBranchPairStageFiveThroughFifteen,
    EveryRt2BranchPairStageFiveLiveDemand,
    AcceptedProofBearingTransportClassOrderAxis,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Uc1ClauseStatus {
    Burned,
    Untested,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1LineageAuthenticationV1 {
    pub m1_transport_replay_valid: bool,
    pub bi2_bundle_replay_valid: bool,
    pub bi4_replay_valid: bool,
    pub r_t2_full_current_source_reissue_attempted: bool,
    pub r_t2_live_reissue_valid: bool,
    pub r_t2_live_reissue_errors: Vec<String>,
    pub r_t2_live_reissue_serialized_blockers: Vec<String>,
    pub r_t2_sealed_fallback_used: bool,
    pub r_t2_certificate_self_digest_valid: bool,
    pub r_t2_archived_stage4_projection_replay_valid: bool,
    pub r_t2_p3_pair_surface_valid: bool,
    pub r_t2_sealed_surface_authentication_valid: bool,
    pub bi4_source_bindings_match_all_consumed_bi_inputs: bool,
    pub every_bi1_bi1b_bi2_branch_digest_joined: bool,
    pub correspondence_joined_to_all_bi1b_branches: bool,
    pub exact_joined_branch_count: RegisteredNumber,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PredictionOneScoreV1 {
    pub prediction_id: String,
    pub quantifier: ClaimQuantifier,
    pub verdict: PredictionVerdict,
    pub source_level: String,
    pub source_verdict: String,
    pub branch_count: RegisteredNumber,
    pub unordered_pair_count: RegisteredNumber,
    pub stage_band_first: RegisteredNumber,
    pub stage_band_last: RegisteredNumber,
    pub stage_count: RegisteredNumber,
    pub every_g3a_pair_equal: bool,
    pub g3a_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ForkShadowComparisonV1 {
    pub stage: RegisteredNumber,
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub left_winner_hash: String,
    pub right_winner_hash: String,
    pub left_canonical_telescope_hash: String,
    pub right_canonical_telescope_hash: String,
    pub left_stage4_reference_relabel_count: RegisteredNumber,
    pub right_stage4_reference_relabel_count: RegisteredNumber,
    pub left_relabel_applied: bool,
    pub right_relabel_applied: bool,
    pub canonical_telescopes_identical: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnactedRootTransitivityComparisonV1 {
    pub stage: RegisteredNumber,
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub enacted_branch_root: String,
    pub left_winner_hash: String,
    pub right_winner_hash: String,
    pub enacted_winner_hash: String,
    pub left_to_enacted_evidence_hash: String,
    pub right_to_enacted_evidence_hash: String,
    pub left_edge_is_reflexive: bool,
    pub right_edge_is_reflexive: bool,
    pub both_edges_replayed: bool,
    pub identity_proved_by_enacted_root_transitivity: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PredictionTwoScoreV1 {
    pub prediction_id: String,
    pub quantifier: ClaimQuantifier,
    pub verdict: PredictionVerdict,
    pub branch_count: RegisteredNumber,
    pub unordered_pair_count: RegisteredNumber,
    pub fork_shadow_stage_count: RegisteredNumber,
    pub fork_shadow_pair_stage_comparison_count: RegisteredNumber,
    pub correspondence_stage_count: RegisteredNumber,
    pub sealed_correspondence_input_row_count: RegisteredNumber,
    pub transitive_pair_stage_proof_count: RegisteredNumber,
    pub fork_shadow_comparisons: Vec<ForkShadowComparisonV1>,
    pub enacted_root_transitivity_comparisons: Vec<EnactedRootTransitivityComparisonV1>,
    pub every_stage4_reference_recursively_relabelled: bool,
    pub all_fork_shadow_telescopes_identical: bool,
    pub all_later_winners_identical_by_enacted_root_transitivity: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MarginConfinementCounterexampleV1 {
    pub stage: RegisteredNumber,
    pub left_branch_root: String,
    pub right_branch_root: String,
    pub left_live_structural_scheme_id: String,
    pub right_live_structural_scheme_id: String,
    pub left_live_scheme_count: RegisteredNumber,
    pub right_live_scheme_count: RegisteredNumber,
    pub perfect_matching_edge_count: RegisteredNumber,
    pub live_pair_is_literal_perfect_matching_edge: bool,
    pub f_uc2_counterexample: bool,
    pub r_t2_pair_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PredictionThreeScoreV1 {
    pub prediction_id: String,
    pub quantifier: ClaimQuantifier,
    pub verdict: PredictionVerdict,
    pub unordered_pair_count: RegisteredNumber,
    pub counterexample_count: RegisteredNumber,
    pub counterexamples: Vec<MarginConfinementCounterexampleV1>,
    pub every_live_pair_in_pairwise_matching: bool,
    pub f_uc2_triggered: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fuc4OrderObstructionV1 {
    pub fixed_coordinate: String,
    pub left_candidate_hash: String,
    pub right_candidate_hash: String,
    pub left_semantic_nu: RegisteredNumber,
    pub right_semantic_nu: RegisteredNumber,
    pub semantic_nu_distinct: bool,
    pub no_accepted_typed_family_bijection_exists: bool,
    pub m1_obstruction_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fuc4ScoreV1 {
    pub falsifier_id: String,
    pub quantifier: ClaimQuantifier,
    pub transport_scope_boundary_id: String,
    pub exported_typed_transport_map_count: RegisteredNumber,
    pub order_axis_obstruction_count: RegisteredNumber,
    pub order_axis_obstructions: Vec<Fuc4OrderObstructionV1>,
    pub generic_transport_invariance_replayed: bool,
    pub both_order_axis_three_versus_two_obstructions_replayed: bool,
    pub order_axis_equivalence_refuted_in_adopted_transport_class: bool,
    pub burns_uc1a: bool,
    pub burns_uc1d: bool,
    pub does_not_assess_uc1b_former_semantics: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1ClauseDispositionV1 {
    pub clause_id: String,
    pub status: Uc1ClauseStatus,
    pub grounds: String,
    pub evidence_derivation_hashes: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1ScoringV1Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Uc1SourceBindingV1>,
    pub input_artifact_count: RegisteredNumber,
    pub numeric_register_policy: String,
    pub lineage: Uc1LineageAuthenticationV1,
    pub p1: PredictionOneScoreV1,
    pub p2: PredictionTwoScoreV1,
    pub p3: PredictionThreeScoreV1,
    pub f_uc4: Fuc4ScoreV1,
    pub clause_dispositions: Vec<Uc1ClauseDispositionV1>,
    pub outcome_zone: String,
    pub sole_former_axis_residual: String,
    pub residual_nonblocking_because_p3_refuted: bool,
    pub z_collapse_or_z_heal_guessed: bool,
    pub no_new_equivalence_semantics_assumed: bool,
    pub no_candidate_or_value_recomputed_or_newly_observed_for_scoring: bool,
    pub semantic_nu_bar_hash_or_enumeration_order_used_as_selector: bool,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub forbidden_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Uc1ScoringV1Replay {
    pub valid: bool,
    pub p1_verdict: Option<PredictionVerdict>,
    pub p2_verdict: Option<PredictionVerdict>,
    pub p3_verdict: Option<PredictionVerdict>,
    pub outcome_zone: Option<String>,
    pub sole_residual: Option<String>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Uc1ScoringV1Error {
    #[error("UC-1 prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("UC-1 input failed: {0}")]
    Input(String),
    #[error("UC-1 invariant failed: {0}")]
    Invariant(String),
    #[error("UC-1 JSON failed: {0}")]
    Json(String),
    #[error("UC-1 I/O failed: {0}")]
    Io(String),
    #[error("emitted UC-1 artifact did not replay: {0}")]
    EmittedReplay(String),
}

#[derive(Clone, Debug)]
struct SealedInputs {
    m1: SemanticNuTransportMapsV2Certificate,
    bi4: Bi4ConeAssemblyV2Certificate,
    r_t2: Rt2FutureHoleConfluenceV2Certificate,
    bi1: Vec<Bi1BranchCertificateV3>,
    bi1b: Vec<Bi1bBranchCertificateV1>,
    correspondence: Bi1bCorrespondenceDiagnosticV1,
    bi2: Bi2FourBranchFinalesBundleV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "payload")]
enum ForkShadowExpr {
    App(Box<ForkShadowExpr>, Box<ForkShadowExpr>),
    Lam(Box<ForkShadowExpr>),
    Pi(Box<ForkShadowExpr>, Box<ForkShadowExpr>),
    Sigma(Box<ForkShadowExpr>, Box<ForkShadowExpr>),
    Univ,
    Var(u32),
    Lib(u32),
    Stage4BranchReference,
    Id(
        Box<ForkShadowExpr>,
        Box<ForkShadowExpr>,
        Box<ForkShadowExpr>,
    ),
    Refl(Box<ForkShadowExpr>),
    Susp(Box<ForkShadowExpr>),
    Trunc(Box<ForkShadowExpr>),
    PathCon(u32),
    Flat(Box<ForkShadowExpr>),
    Sharp(Box<ForkShadowExpr>),
    Disc(Box<ForkShadowExpr>),
    Shape(Box<ForkShadowExpr>),
    Next(Box<ForkShadowExpr>),
    Eventually(Box<ForkShadowExpr>),
    Bang(Box<ForkShadowExpr>),
    WhyNot(Box<ForkShadowExpr>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ForkShadowTelescope {
    clauses: Vec<(ClauseRole, ForkShadowExpr)>,
}

#[derive(Clone, Debug)]
struct EnactedEdge {
    own_winner_hash: String,
    enacted_winner_hash: String,
    evidence_hash: String,
    reflexive: bool,
    replayed: bool,
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(UC1_SCORING_V1_SCHEMA, domain, value))
        .expect("UC-1 scoring evidence serializes");
    bytes_hash(&bytes)
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes =
        serde_json::to_vec(&(schema, domain, value)).expect("external UC-1 evidence serializes");
    bytes_hash(&bytes)
}

fn r_t2_certificate_self_digest_valid(certificate: &Rt2FutureHoleConfluenceV2Certificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA,
            "certificate",
            &projection,
        )
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> Uc1SourceBindingV1 {
    Uc1SourceBindingV1 {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: registered_usize(
            bytes.len(),
            Uc1NumericRegister::ArtifactMetadata,
            "bound source byte length; artifact metadata only",
        ),
        blake3: bytes_hash(bytes),
    }
}

fn source_bindings() -> Vec<Uc1SourceBindingV1> {
    let mut bindings = vec![
        source_binding(
            "docs/mainline_completion_plan.md",
            "frozen M-2 construction brief and quantifier policy",
            MAINLINE_PLAN_BYTES,
        ),
        source_binding(
            "docs/univalent_collapse_hypothesis.md",
            "pre-registered UC-1 predictions, zones, and falsifiers",
            UC1_HYPOTHESIS_BYTES,
        ),
        source_binding(
            "docs/nu_register_adjudication.md",
            "adopted semantic-family register and F-NR6",
            NU_REGISTER_BYTES,
        ),
        source_binding(
            "docs/semantic_nu_transport_maps_v2.json",
            "sealed M-1 typed transport maps and order-axis obstructions",
            M1_TRANSPORT_BYTES,
        ),
        source_binding(
            "docs/BI4_CONE_V2_CERTIFICATE.json",
            "sealed cone-level G3a verdict and authenticated BI lineage",
            BI4_BYTES,
        ),
        source_binding(
            "docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json",
            "sealed pairwise Stage-5 scheme-set matchings",
            R_T2_BYTES,
        ),
    ];
    bindings.extend(BI1_INPUTS.iter().map(|(path, bytes)| {
        source_binding(
            path,
            "retired sealed BI-1 v3 winner telescopes and live demand records",
            bytes,
        )
    }));
    bindings.extend(BI1B_INPUTS.iter().map(|(path, bytes)| {
        source_binding(
            path,
            "sealed BI-1b branch certificate joined through BI-2",
            bytes,
        )
    }));
    bindings.push(source_binding(
        "docs/BI1B_CORRESPONDENCE_DIAGNOSTIC.json",
        "sealed write-only winner byte-identity correspondence",
        CORRESPONDENCE_BYTES,
    ));
    bindings.push(source_binding(
        "docs/BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json",
        "sealed BI-2 v2 aggregate and lineage index",
        BI2_INDEX_BYTES,
    ));
    bindings.extend(
        BI2_BRANCH_INPUTS
            .iter()
            .map(|(path, bytes)| source_binding(path, "sealed BI-2 v2 branch finale", bytes)),
    );
    bindings.push(source_binding(
        "crates/pen-search/src/uc1_scoring_v1.rs",
        "M-2 issue, replay, scoring, and create-new implementation",
        THIS_SOURCE_BYTES,
    ));
    bindings.push(source_binding(
        "crates/pen-search/examples/uc1_scoring_v1.rs",
        "M-2 create-new executable boundary",
        EXAMPLE_SOURCE_BYTES,
    ));
    bindings
}

fn parse_json<T: DeserializeOwned>(name: &str, bytes: &[u8]) -> Result<T, Uc1ScoringV1Error> {
    serde_json::from_slice(bytes)
        .map_err(|error| Uc1ScoringV1Error::Json(format!("{name}: {error}")))
}

fn parse_inputs() -> Result<SealedInputs, Uc1ScoringV1Error> {
    let bi1 = BI1_INPUTS
        .iter()
        .map(|(path, bytes)| parse_json(path, bytes))
        .collect::<Result<Vec<_>, _>>()?;
    let bi1b = BI1B_INPUTS
        .iter()
        .map(|(path, bytes)| parse_json(path, bytes))
        .collect::<Result<Vec<_>, _>>()?;
    let bi2_branches = BI2_BRANCH_INPUTS
        .iter()
        .map(|(path, bytes)| parse_json(path, bytes))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(SealedInputs {
        m1: parse_json("semantic_nu_transport_maps_v2.json", M1_TRANSPORT_BYTES)?,
        bi4: parse_json("BI4_CONE_V2_CERTIFICATE.json", BI4_BYTES)?,
        r_t2: parse_json(
            "r_t2_future_hole_confluence_v2_dependent_context_v2.json",
            R_T2_BYTES,
        )?,
        bi1,
        bi1b,
        correspondence: parse_json("BI1B_CORRESPONDENCE_DIAGNOSTIC.json", CORRESPONDENCE_BYTES)?,
        bi2: Bi2FourBranchFinalesBundleV1 {
            branches: bi2_branches,
            index: parse_json(
                "BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json",
                BI2_INDEX_BYTES,
            )?,
        },
    })
}

fn lineage_hash(lineage: &Uc1LineageAuthenticationV1) -> String {
    let mut projection = lineage.clone();
    projection.derivation_hash.clear();
    tagged_hash("lineage-authentication", &projection)
}

fn p1_hash(score: &PredictionOneScoreV1) -> String {
    let mut projection = score.clone();
    projection.derivation_hash.clear();
    tagged_hash("P1-score", &projection)
}

fn fork_shadow_hash(row: &ForkShadowComparisonV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("P2-fork-shadow-comparison", &projection)
}

fn transitivity_hash(row: &EnactedRootTransitivityComparisonV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("P2-enacted-root-transitivity-comparison", &projection)
}

fn p2_hash(score: &PredictionTwoScoreV1) -> String {
    let mut projection = score.clone();
    projection.derivation_hash.clear();
    tagged_hash("P2-score", &projection)
}

fn p3_counterexample_hash(row: &MarginConfinementCounterexampleV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("P3-margin-confinement-counterexample", &projection)
}

fn p3_hash(score: &PredictionThreeScoreV1) -> String {
    let mut projection = score.clone();
    projection.derivation_hash.clear();
    tagged_hash("P3-score", &projection)
}

fn fuc4_obstruction_hash(row: &Fuc4OrderObstructionV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("F-UC4-order-obstruction", &projection)
}

fn fuc4_hash(score: &Fuc4ScoreV1) -> String {
    let mut projection = score.clone();
    projection.derivation_hash.clear();
    tagged_hash("F-UC4-score", &projection)
}

fn clause_hash(row: &Uc1ClauseDispositionV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("UC-1-clause-disposition", &projection)
}

fn certificate_hash(certificate: &Uc1ScoringV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn canonicalize_expr(expr: &Expr, relabel_count: &mut usize) -> ForkShadowExpr {
    match expr {
        Expr::App(left, right) => ForkShadowExpr::App(
            Box::new(canonicalize_expr(left, relabel_count)),
            Box::new(canonicalize_expr(right, relabel_count)),
        ),
        Expr::Lam(body) => ForkShadowExpr::Lam(Box::new(canonicalize_expr(body, relabel_count))),
        Expr::Pi(domain, codomain) => ForkShadowExpr::Pi(
            Box::new(canonicalize_expr(domain, relabel_count)),
            Box::new(canonicalize_expr(codomain, relabel_count)),
        ),
        Expr::Sigma(domain, codomain) => ForkShadowExpr::Sigma(
            Box::new(canonicalize_expr(domain, relabel_count)),
            Box::new(canonicalize_expr(codomain, relabel_count)),
        ),
        Expr::Univ => ForkShadowExpr::Univ,
        Expr::Var(index) => ForkShadowExpr::Var(*index),
        Expr::Lib(4) => {
            *relabel_count += 1;
            ForkShadowExpr::Stage4BranchReference
        }
        Expr::Lib(index) => ForkShadowExpr::Lib(*index),
        Expr::Id(ty, left, right) => ForkShadowExpr::Id(
            Box::new(canonicalize_expr(ty, relabel_count)),
            Box::new(canonicalize_expr(left, relabel_count)),
            Box::new(canonicalize_expr(right, relabel_count)),
        ),
        Expr::Refl(body) => ForkShadowExpr::Refl(Box::new(canonicalize_expr(body, relabel_count))),
        Expr::Susp(body) => ForkShadowExpr::Susp(Box::new(canonicalize_expr(body, relabel_count))),
        Expr::Trunc(body) => {
            ForkShadowExpr::Trunc(Box::new(canonicalize_expr(body, relabel_count)))
        }
        Expr::PathCon(dimension) => ForkShadowExpr::PathCon(*dimension),
        Expr::Flat(body) => ForkShadowExpr::Flat(Box::new(canonicalize_expr(body, relabel_count))),
        Expr::Sharp(body) => {
            ForkShadowExpr::Sharp(Box::new(canonicalize_expr(body, relabel_count)))
        }
        Expr::Disc(body) => ForkShadowExpr::Disc(Box::new(canonicalize_expr(body, relabel_count))),
        Expr::Shape(body) => {
            ForkShadowExpr::Shape(Box::new(canonicalize_expr(body, relabel_count)))
        }
        Expr::Next(body) => ForkShadowExpr::Next(Box::new(canonicalize_expr(body, relabel_count))),
        Expr::Eventually(body) => {
            ForkShadowExpr::Eventually(Box::new(canonicalize_expr(body, relabel_count)))
        }
        Expr::Bang(body) => ForkShadowExpr::Bang(Box::new(canonicalize_expr(body, relabel_count))),
        Expr::WhyNot(body) => {
            ForkShadowExpr::WhyNot(Box::new(canonicalize_expr(body, relabel_count)))
        }
    }
}

fn canonicalize_telescope(telescope: &Telescope) -> (ForkShadowTelescope, usize) {
    let mut relabel_count = 0;
    let clauses = telescope
        .clauses
        .iter()
        .map(|clause| {
            (
                clause.role,
                canonicalize_expr(&clause.expr, &mut relabel_count),
            )
        })
        .collect();
    (ForkShadowTelescope { clauses }, relabel_count)
}

fn canonical_telescope_hash(telescope: &ForkShadowTelescope) -> String {
    tagged_hash("fork-shadow-normalized-telescope", telescope)
}

fn branch_pairs<T>(values: &[T]) -> Vec<(&T, &T)> {
    let mut pairs = Vec::new();
    for left in 0..values.len() {
        for right in (left + 1)..values.len() {
            pairs.push((&values[left], &values[right]));
        }
    }
    pairs
}

fn sorted_bi1(
    branches: &[Bi1BranchCertificateV3],
) -> Result<Vec<&Bi1BranchCertificateV3>, Uc1ScoringV1Error> {
    let mut sorted = branches.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|branch| branch.issuance_ordinal);
    let ordinals = sorted
        .iter()
        .map(|branch| branch.issuance_ordinal)
        .collect::<BTreeSet<_>>();
    let roots = sorted
        .iter()
        .map(|branch| branch.branch_root_hash.as_str())
        .collect::<BTreeSet<_>>();
    if sorted.len() != 4 || ordinals != BTreeSet::from([0, 1, 2, 3]) || roots.len() != 4 {
        return Err(Uc1ScoringV1Error::Input(
            "retired BI-1 v3 surface is not exactly four distinct ordered branches".to_owned(),
        ));
    }
    Ok(sorted)
}

fn stage_winner<'a>(
    branch: &'a Bi1BranchCertificateV3,
    stage: u32,
) -> Result<&'a crate::branch_invariance::BranchWinner, Uc1ScoringV1Error> {
    let records = branch
        .continuation
        .stages
        .iter()
        .filter(|record| record.stage == stage)
        .collect::<Vec<_>>();
    if records.len() != 1 {
        return Err(Uc1ScoringV1Error::Input(format!(
            "branch {} has {} retired BI-1 records for stage {stage}",
            branch.branch_root_hash,
            records.len()
        )));
    }
    records[0].winner.as_ref().ok_or_else(|| {
        Uc1ScoringV1Error::Input(format!(
            "branch {} has no winner at stage {stage}",
            branch.branch_root_hash
        ))
    })
}

fn stage_live_scheme_ids(
    branch: &Bi1BranchCertificateV3,
    stage: u32,
) -> Result<Vec<String>, Uc1ScoringV1Error> {
    let records = branch
        .continuation
        .stages
        .iter()
        .filter(|record| record.stage == stage)
        .collect::<Vec<_>>();
    if records.len() != 1 {
        return Err(Uc1ScoringV1Error::Input(format!(
            "branch {} has {} demand records for stage {stage}",
            branch.branch_root_hash,
            records.len()
        )));
    }
    Ok(records[0].demand.structural_scheme_ids.clone())
}

fn bi4_consumed_source_bindings_match(inputs: &SealedInputs) -> bool {
    let expected = BI1_INPUTS
        .iter()
        .chain(BI1B_INPUTS.iter())
        .chain(BI2_BRANCH_INPUTS.iter())
        .chain(
            [
                (
                    "docs/BI2_FOUR_BRANCH_FINALES_V2_CERTIFICATE.json",
                    BI2_INDEX_BYTES,
                ),
                (
                    "docs/BI1B_CORRESPONDENCE_DIAGNOSTIC.json",
                    CORRESPONDENCE_BYTES,
                ),
                (
                    "docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json",
                    R_T2_BYTES,
                ),
            ]
            .iter(),
        )
        .all(|(path, bytes)| {
            inputs
                .bi4
                .source_bindings
                .iter()
                .any(|binding| binding.path == *path && binding.blake3 == bytes_hash(bytes))
        });
    expected
        && inputs.bi4.correspondence_corroboration.artifact_blake3
            == bytes_hash(CORRESPONDENCE_BYTES)
        && inputs.bi4.inherited_g1_testimony.artifact_blake3 == bytes_hash(R_T2_BYTES)
}

fn r_t2_p3_pair_surface_valid(certificate: &Rt2FutureHoleConfluenceV2Certificate) -> bool {
    let branch_roots = certificate
        .branches
        .iter()
        .map(|branch| branch.stage4_candidate_hash.as_str())
        .collect::<BTreeSet<_>>();
    let expected_pairs = branch_roots
        .iter()
        .enumerate()
        .flat_map(|(left_index, left)| {
            branch_roots
                .iter()
                .skip(left_index + 1)
                .map(move |right| ((*left).to_owned(), (*right).to_owned()))
        })
        .collect::<BTreeSet<_>>();
    let observed_pairs = certificate
        .pairwise_scheme_set_comparisons
        .iter()
        .map(|row| {
            if row.left_candidate_hash < row.right_candidate_hash {
                (
                    row.left_candidate_hash.clone(),
                    row.right_candidate_hash.clone(),
                )
            } else {
                (
                    row.right_candidate_hash.clone(),
                    row.left_candidate_hash.clone(),
                )
            }
        })
        .collect::<BTreeSet<_>>();
    let branch_scheme_ids = certificate
        .branches
        .iter()
        .map(|branch| {
            (
                branch.stage4_candidate_hash.as_str(),
                branch
                    .schemes
                    .iter()
                    .map(|scheme| scheme.local_scheme_id.as_str())
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let branch_gates_valid = certificate.branches.iter().all(|branch| {
        branch.complete_certified_scheme_count == branch.schemes.len()
            && branch.complete_certified_scheme_count == 5
            && branch.every_instance_typed
            && branch.every_scheme_has_formation_evidence
            && branch.coverage_gate.theorem_replay_valid
            && branch.coverage_gate.full_relative_seed_coverage
            && branch.coverage_gate.full_relative_constructor_coverage
            && branch.coverage_gate.all_generated_schemes_accounted_for
            && branch
                .coverage_gate
                .exact_compared_scheme_set_equals_inventory_promotions
            && branch.coverage_gate.zero_named_gaps
            && branch
                .coverage_gate
                .full_relative_a3_seed_constructor_coverage
            && branch
                .coverage_gate
                .full_adopted_frozen_semantic_scope_exhaustive
            && branch
                .coverage_gate
                .unresolved_in_domain_semantic_scope_premises
                .is_empty()
            && branch.coverage_gate.law_level_promotion_authorized
    });
    let pair_gates_valid = certificate
        .pairwise_scheme_set_comparisons
        .iter()
        .all(|row| {
            let Some(left_ids) = branch_scheme_ids.get(row.left_candidate_hash.as_str()) else {
                return false;
            };
            let Some(right_ids) = branch_scheme_ids.get(row.right_candidate_hash.as_str()) else {
                return false;
            };
            let matching_edges_unique = row.perfect_matching.iter().collect::<BTreeSet<_>>().len()
                == row.perfect_matching.len();
            let matching_edges_well_formed = row.perfect_matching.iter().all(|(left, right)| {
                left_ids.contains(left.as_str())
                    && right_ids.contains(right.as_str())
                    && row.comparisons.iter().any(|comparison| {
                        comparison.left_scheme_id == *left
                            && comparison.right_scheme_id == *right
                            && comparison.equivalent_under_certified_family_quotient
                    })
            });
            row.left_complete_scheme_count == left_ids.len()
                && row.right_complete_scheme_count == right_ids.len()
                && row.left_complete_scheme_count == 5
                && row.right_complete_scheme_count == 5
                && row.comparison_well_formed
                && row.relative_coverage_and_zero_gap_gate_passed
                && !row.full_scheme_sets_equivalent
                && !row.hash_or_enumeration_order_used_as_selector
                && !row.perfect_matching.is_empty()
                && matching_edges_unique
                && matching_edges_well_formed
                && !row.derivation_hash.is_empty()
        });
    certificate.branch_count == 4
        && certificate.branches.len() == 4
        && branch_roots.len() == 4
        && certificate.exact_four_way_r_t1_class_join
        && certificate.four_exact_stage5_prefixes_independently_generated
        && certificate.full_relative_a3_seed_constructor_coverage_every_branch
        && certificate.zero_named_gaps_every_branch
        && certificate.full_adopted_frozen_semantic_scope_exhaustive_every_branch
        && certificate
            .unresolved_in_domain_semantic_scope_premises
            .is_empty()
        && certificate.every_pairwise_comparison_well_formed
        && !certificate.all_stage5_scheme_sets_equivalent
        && certificate.order_reversal_invariant
        && certificate.immediate_inequivalent_successor_on_certified_relative_surface
        && certificate.r_t2_confluence_refuted
        && !certificate.r_t2_confluence_proved
        && !certificate.hash_or_enumeration_order_used_as_selector
        && !certificate.desired_history_count_score_or_bar_used_as_premise
        && certificate.pairwise_scheme_set_comparisons.len() == 6
        && expected_pairs == observed_pairs
        && branch_gates_valid
        && pair_gates_valid
}

fn authenticate_lineage(
    inputs: &SealedInputs,
) -> Result<Uc1LineageAuthenticationV1, Uc1ScoringV1Error> {
    let m1_replay = replay_semantic_nu_transport_maps_v2(&inputs.m1);
    if !m1_replay.valid {
        return Err(Uc1ScoringV1Error::Input(format!(
            "M-1 replay failed: {}",
            m1_replay.errors.join("; ")
        )));
    }
    let bi2_replay = replay_bi2_four_branch_finales_v1(&inputs.bi2);
    if !bi2_replay.valid {
        return Err(Uc1ScoringV1Error::Input(format!(
            "BI-2 replay failed: {}",
            bi2_replay.errors.join("; ")
        )));
    }
    let bi4_replay = replay_bi4_cone_assembly_v2(&inputs.bi4);
    if !bi4_replay.valid {
        return Err(Uc1ScoringV1Error::Input(format!(
            "BI-4 replay failed: {}",
            bi4_replay.errors.join("; ")
        )));
    }
    let r_t2_replay = replay_r_t2_future_hole_confluence_v2_certificate(&inputs.r_t2);
    let r_t2_self_digest_valid = r_t2_certificate_self_digest_valid(&inputs.r_t2);
    let r_t2_archived_projection_valid =
        replay_archived_stage4_fork_projection(&inputs.r_t2).is_empty();
    let r_t2_pair_surface_valid = r_t2_p3_pair_surface_valid(&inputs.r_t2);
    let r_t2_sealed_surface_authentication_valid =
        r_t2_self_digest_valid && r_t2_archived_projection_valid && r_t2_pair_surface_valid;
    let r_t2_sealed_fallback_used = !r_t2_replay.valid && r_t2_sealed_surface_authentication_valid;
    if !r_t2_sealed_surface_authentication_valid {
        return Err(Uc1ScoringV1Error::Input(format!(
            "the sealed R-T2 gates failed: self_digest={r_t2_self_digest_valid}; archived_projection={r_t2_archived_projection_valid}; pair_surface={r_t2_pair_surface_valid}"
        )));
    }
    if !r_t2_replay.valid && !r_t2_sealed_fallback_used {
        return Err(Uc1ScoringV1Error::Input(format!(
            "R-T2 live replay failed and the sealed fallback did not authenticate: live errors={}; self_digest={r_t2_self_digest_valid}; archived_projection={r_t2_archived_projection_valid}; pair_surface={r_t2_pair_surface_valid}",
            r_t2_replay.errors.join("; ")
        )));
    }

    let source_bindings_match = bi4_consumed_source_bindings_match(inputs);
    let bi1_by_root = inputs
        .bi1
        .iter()
        .map(|branch| (branch.branch_root_hash.as_str(), branch))
        .collect::<BTreeMap<_, _>>();
    let bi1b_by_root = inputs
        .bi1b
        .iter()
        .map(|branch| (branch.branch_root_hash.as_str(), branch))
        .collect::<BTreeMap<_, _>>();
    let bi2_by_root = inputs
        .bi2
        .branches
        .iter()
        .map(|branch| (branch.branch_root_hash.as_str(), branch))
        .collect::<BTreeMap<_, _>>();
    let every_branch_digest_joined = inputs.bi2.index.rows.iter().all(|row| {
        let Some(bi1) = bi1_by_root.get(row.branch_root_hash.as_str()) else {
            return false;
        };
        let Some(bi1b) = bi1b_by_root.get(row.branch_root_hash.as_str()) else {
            return false;
        };
        let Some(bi2) = bi2_by_root.get(row.branch_root_hash.as_str()) else {
            return false;
        };
        row.branch_certificate_digest == bi2.result_digest
            && bi2.sealed_bi1_branch_certificate_digest == bi1.result_digest
            && bi2.bi1b_branch_certificate_digest == bi1b.result_digest
    }) && inputs.bi2.index.rows.len() == 4
        && bi1_by_root.len() == 4
        && bi1b_by_root.len() == 4
        && bi2_by_root.len() == 4
        && inputs.bi4.bi2_index_digest == inputs.bi2.index.result_digest
        && inputs.bi4.upstream_lineage_authenticated_by_bi2_replay;
    let correspondence_digests = inputs
        .correspondence
        .sealed_branch_certificate_digests
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let bi1b_digests = inputs
        .bi1b
        .iter()
        .map(|branch| branch.result_digest.as_str())
        .collect::<BTreeSet<_>>();
    let correspondence_joined = correspondence_digests == bi1b_digests
        && inputs
            .bi4
            .correspondence_corroboration
            .artifact_self_digest_valid
        && inputs
            .bi4
            .correspondence_corroboration
            .every_winner_joins_sealed_bi2_ledger;
    if !source_bindings_match || !every_branch_digest_joined || !correspondence_joined {
        return Err(Uc1ScoringV1Error::Input(
            "the sealed BI-1/BI-1b/BI-2/BI-4 lineage did not join exactly".to_owned(),
        ));
    }
    let mut lineage = Uc1LineageAuthenticationV1 {
        m1_transport_replay_valid: m1_replay.valid,
        bi2_bundle_replay_valid: bi2_replay.valid,
        bi4_replay_valid: bi4_replay.valid,
        r_t2_full_current_source_reissue_attempted: true,
        r_t2_live_reissue_valid: r_t2_replay.valid,
        r_t2_live_reissue_errors: r_t2_replay.errors.clone(),
        r_t2_live_reissue_serialized_blockers: r_t2_replay.serialized_blockers.clone(),
        r_t2_sealed_fallback_used,
        r_t2_certificate_self_digest_valid: r_t2_self_digest_valid,
        r_t2_archived_stage4_projection_replay_valid: r_t2_archived_projection_valid,
        r_t2_p3_pair_surface_valid: r_t2_pair_surface_valid,
        r_t2_sealed_surface_authentication_valid,
        bi4_source_bindings_match_all_consumed_bi_inputs: source_bindings_match,
        every_bi1_bi1b_bi2_branch_digest_joined: every_branch_digest_joined,
        correspondence_joined_to_all_bi1b_branches: correspondence_joined,
        exact_joined_branch_count: proof_inventory(
            inputs.bi2.index.rows.len(),
            "BI branch lineages joined exactly",
        ),
        derivation_hash: String::new(),
    };
    lineage.derivation_hash = lineage_hash(&lineage);
    Ok(lineage)
}

fn score_p1(inputs: &SealedInputs) -> Result<PredictionOneScoreV1, Uc1ScoringV1Error> {
    let all_pairs_equal = inputs.bi4.g3a_pair_comparisons.len() == 6
        && inputs
            .bi4
            .g3a_pair_comparisons
            .iter()
            .all(|row| row.equal && row.first_divergence_stage.is_none());
    let passed =
        inputs.bi4.g3a.verdict == Bi4VerdictV2::Passed && inputs.bi4.g3a.passed && all_pairs_equal;
    if !passed {
        return Err(Uc1ScoringV1Error::Invariant(
            "BI-4 G3a no longer scores P1 Passed".to_owned(),
        ));
    }
    let mut score = PredictionOneScoreV1 {
        prediction_id: "P1_LEDGER_IDENTITY".to_owned(),
        quantifier: ClaimQuantifier::ConeAllBranchesStageFiveThroughFifteen,
        verdict: PredictionVerdict::Passed,
        source_level: inputs.bi4.g3a.level.clone(),
        source_verdict: "passed".to_owned(),
        branch_count: proof_inventory(inputs.bi4.branches.len(), "cone branches compared by G3a"),
        unordered_pair_count: proof_inventory(
            inputs.bi4.g3a_pair_comparisons.len(),
            "unordered branch pairs compared by G3a",
        ),
        stage_band_first: syntax_identifier(5, "first successor stage in P1"),
        stage_band_last: syntax_identifier(15, "last successor stage in P1"),
        stage_count: proof_inventory(11, "successor stages covered by P1"),
        every_g3a_pair_equal: all_pairs_equal,
        g3a_derivation_hash: inputs.bi4.g3a.derivation_hash.clone(),
        derivation_hash: String::new(),
    };
    score.derivation_hash = p1_hash(&score);
    Ok(score)
}

fn score_p2(inputs: &SealedInputs) -> Result<PredictionTwoScoreV1, Uc1ScoringV1Error> {
    let branches = sorted_bi1(&inputs.bi1)?;
    let pairs = branch_pairs(&branches);
    if pairs.len() != 6 {
        return Err(Uc1ScoringV1Error::Invariant(
            "P2 did not obtain the exact six branch pairs".to_owned(),
        ));
    }
    let mut fork_shadow_comparisons = Vec::new();
    for (left, right) in &pairs {
        for stage in 5..=7 {
            let left_winner = stage_winner(left, stage)?;
            let right_winner = stage_winner(right, stage)?;
            let (left_canonical, left_count) = canonicalize_telescope(&left_winner.telescope);
            let (right_canonical, right_count) = canonicalize_telescope(&right_winner.telescope);
            let identical = left_canonical == right_canonical;
            let mut row = ForkShadowComparisonV1 {
                stage: syntax_identifier(stage, "P2 fork-shadow comparison stage"),
                left_branch_root: left.branch_root_hash.clone(),
                right_branch_root: right.branch_root_hash.clone(),
                left_winner_hash: left_winner.candidate_hash.clone(),
                right_winner_hash: right_winner.candidate_hash.clone(),
                left_canonical_telescope_hash: canonical_telescope_hash(&left_canonical),
                right_canonical_telescope_hash: canonical_telescope_hash(&right_canonical),
                left_stage4_reference_relabel_count: proof_inventory(
                    left_count,
                    "recursive Expr::Lib(4) occurrences relabelled on the left",
                ),
                right_stage4_reference_relabel_count: proof_inventory(
                    right_count,
                    "recursive Expr::Lib(4) occurrences relabelled on the right",
                ),
                left_relabel_applied: left_count > 0,
                right_relabel_applied: right_count > 0,
                canonical_telescopes_identical: identical,
                derivation_hash: String::new(),
            };
            row.derivation_hash = fork_shadow_hash(&row);
            fork_shadow_comparisons.push(row);
        }
    }
    let all_fork_shadow_identical = fork_shadow_comparisons
        .iter()
        .all(|row| row.canonical_telescopes_identical);
    if fork_shadow_comparisons.len() != 18 || !all_fork_shadow_identical {
        return Err(Uc1ScoringV1Error::Invariant(
            "P2 fork-shadow comparison did not prove all early successor telescopes identical"
                .to_owned(),
        ));
    }

    let enacted = branches
        .iter()
        .copied()
        .find(|branch| branch.enacted_root)
        .ok_or_else(|| Uc1ScoringV1Error::Input("no enacted BI-1 branch".to_owned()))?;
    if branches.iter().filter(|branch| branch.enacted_root).count() != 1 {
        return Err(Uc1ScoringV1Error::Input(
            "the BI-1 surface does not have exactly one enacted root".to_owned(),
        ));
    }
    let correspondence_rows = inputs
        .correspondence
        .rows
        .iter()
        .map(|row| ((row.alternate_branch_root_hash.as_str(), row.stage), row))
        .collect::<BTreeMap<_, _>>();
    let alternate_roots = branches
        .iter()
        .filter(|branch| !branch.enacted_root)
        .map(|branch| branch.branch_root_hash.as_str())
        .collect::<BTreeSet<_>>();
    let expected_correspondence_keys = alternate_roots
        .iter()
        .flat_map(|root| (8..=15).map(move |stage| ((*root, stage), ())))
        .collect::<BTreeMap<_, _>>();
    if inputs.correspondence.rows.len() != 24
        || correspondence_rows.len() != 24
        || correspondence_rows.keys().copied().collect::<BTreeSet<_>>()
            != expected_correspondence_keys
                .keys()
                .copied()
                .collect::<BTreeSet<_>>()
    {
        return Err(Uc1ScoringV1Error::Input(
            "BI-1b correspondence is not the exact alternate-by-later-stage surface".to_owned(),
        ));
    }

    // The retired BI-1 v3 records are the proof surface for stages 5--7 only.
    // Some alternate records deliberately stop before Stage 8, so the later
    // winner edges must be read from the sealed BI-1b correspondence itself.
    // For each later stage the three alternate rows agree on one non-null
    // enacted winner.  Their three row derivations jointly witness the
    // enacted branch's reflexive edge; no absent retired winner is invented.
    let mut enacted_stage_edges = BTreeMap::<u32, (String, Vec<String>)>::new();
    for stage in 8..=15 {
        let stage_rows = inputs
            .correspondence
            .rows
            .iter()
            .filter(|row| row.stage == stage)
            .collect::<Vec<_>>();
        if stage_rows.len() != 3 {
            return Err(Uc1ScoringV1Error::Input(format!(
                "BI-1b correspondence has {} rows rather than three at stage {stage}",
                stage_rows.len()
            )));
        }
        let enacted_hashes = stage_rows
            .iter()
            .filter_map(|row| row.enacted_winner_hash.as_deref())
            .collect::<BTreeSet<_>>();
        let derivation_hashes = stage_rows
            .iter()
            .map(|row| row.derivation_hash.clone())
            .collect::<BTreeSet<_>>();
        let every_row_is_a_lawful_edge = stage_rows.iter().all(|row| {
            row.enacted_winner_hash.is_some()
                && row.alternate_winner_hash.is_some()
                && row.enacted_winner_hash == row.alternate_winner_hash
                && row.winners_byte_identical
                && row.stage4_only_prefix_shadow_for_byte_identical_candidates
                && !row.correspondence_used_as_provenance
                && !row.correspondence_used_as_selector
        });
        if enacted_hashes.len() != 1 || derivation_hashes.len() != 3 || !every_row_is_a_lawful_edge
        {
            return Err(Uc1ScoringV1Error::Input(format!(
                "BI-1b correspondence does not provide three lawful rows with one enacted winner at stage {stage}"
            )));
        }
        enacted_stage_edges.insert(
            stage,
            (
                (*enacted_hashes
                    .iter()
                    .next()
                    .expect("one enacted winner after cardinality check"))
                .to_owned(),
                derivation_hashes.into_iter().collect(),
            ),
        );
    }

    let edge_to_enacted =
        |branch: &Bi1BranchCertificateV3, stage: u32| -> Result<EnactedEdge, Uc1ScoringV1Error> {
            let (enacted_winner, stage_row_derivations) =
                enacted_stage_edges.get(&stage).ok_or_else(|| {
                    Uc1ScoringV1Error::Input(format!(
                        "missing enacted correspondence witness at stage {stage}"
                    ))
                })?;
            if branch.enacted_root {
                return Ok(EnactedEdge {
                    own_winner_hash: enacted_winner.clone(),
                    enacted_winner_hash: enacted_winner.clone(),
                    evidence_hash: tagged_hash(
                        "P2-enacted-root-reflexive-edge",
                        &(
                            branch.branch_root_hash.as_str(),
                            stage,
                            enacted_winner.as_str(),
                            stage_row_derivations,
                        ),
                    ),
                    reflexive: true,
                    replayed: stage_row_derivations.len() == 3,
                });
            }
            let row = correspondence_rows
                .get(&(branch.branch_root_hash.as_str(), stage))
                .ok_or_else(|| {
                    Uc1ScoringV1Error::Input(format!(
                        "missing correspondence row for {} at stage {stage}",
                        branch.branch_root_hash
                    ))
                })?;
            let row_own = row.alternate_winner_hash.clone().ok_or_else(|| {
                Uc1ScoringV1Error::Input("correspondence alternate winner is absent".to_owned())
            })?;
            let row_enacted = row.enacted_winner_hash.clone().ok_or_else(|| {
                Uc1ScoringV1Error::Input("correspondence enacted winner is absent".to_owned())
            })?;
            let replayed = row.winners_byte_identical
                && row.stage4_only_prefix_shadow_for_byte_identical_candidates
                && !row.correspondence_used_as_provenance
                && !row.correspondence_used_as_selector
                && row_own == row_enacted
                && &row_enacted == enacted_winner
                && stage_row_derivations.contains(&row.derivation_hash);
            Ok(EnactedEdge {
                own_winner_hash: row_own,
                enacted_winner_hash: row_enacted,
                evidence_hash: row.derivation_hash.clone(),
                reflexive: false,
                replayed,
            })
        };

    let mut transitivity_comparisons = Vec::new();
    for (left, right) in &pairs {
        for stage in 8..=15 {
            let left_edge = edge_to_enacted(left, stage)?;
            let right_edge = edge_to_enacted(right, stage)?;
            let transitive = left_edge.replayed
                && right_edge.replayed
                && left_edge.enacted_winner_hash == right_edge.enacted_winner_hash
                && left_edge.own_winner_hash == right_edge.own_winner_hash;
            let mut row = EnactedRootTransitivityComparisonV1 {
                stage: syntax_identifier(stage, "P2 enacted-root transitivity stage"),
                left_branch_root: left.branch_root_hash.clone(),
                right_branch_root: right.branch_root_hash.clone(),
                enacted_branch_root: enacted.branch_root_hash.clone(),
                left_winner_hash: left_edge.own_winner_hash,
                right_winner_hash: right_edge.own_winner_hash,
                enacted_winner_hash: left_edge.enacted_winner_hash,
                left_to_enacted_evidence_hash: left_edge.evidence_hash,
                right_to_enacted_evidence_hash: right_edge.evidence_hash,
                left_edge_is_reflexive: left_edge.reflexive,
                right_edge_is_reflexive: right_edge.reflexive,
                both_edges_replayed: left_edge.replayed && right_edge.replayed,
                identity_proved_by_enacted_root_transitivity: transitive,
                derivation_hash: String::new(),
            };
            row.derivation_hash = transitivity_hash(&row);
            transitivity_comparisons.push(row);
        }
    }
    let all_later_identical = transitivity_comparisons
        .iter()
        .all(|row| row.identity_proved_by_enacted_root_transitivity);
    if transitivity_comparisons.len() != 48 || !all_later_identical {
        return Err(Uc1ScoringV1Error::Invariant(
            "P2 did not prove every later pair identity through enacted-root transitivity"
                .to_owned(),
        ));
    }

    let every_reference_relabelled = fork_shadow_comparisons.iter().all(|row| {
        row.left_relabel_applied == (row.left_stage4_reference_relabel_count.value > 0)
            && row.right_relabel_applied == (row.right_stage4_reference_relabel_count.value > 0)
    });
    let mut score = PredictionTwoScoreV1 {
        prediction_id: "P2_WINNER_IDENTITY_MODULO_FORK_SHADOW".to_owned(),
        quantifier: ClaimQuantifier::EveryUnorderedBranchPairStageFiveThroughFifteen,
        verdict: PredictionVerdict::Passed,
        branch_count: proof_inventory(branches.len(), "sealed branches compared by P2"),
        unordered_pair_count: proof_inventory(pairs.len(), "unordered branch pairs in P2"),
        fork_shadow_stage_count: proof_inventory(3, "stages normalized by fork-shadow relabeling"),
        fork_shadow_pair_stage_comparison_count: proof_inventory(
            fork_shadow_comparisons.len(),
            "fork-shadow pair-stage comparisons",
        ),
        correspondence_stage_count: proof_inventory(
            8,
            "later stages proved through correspondence transitivity",
        ),
        sealed_correspondence_input_row_count: proof_inventory(
            inputs.correspondence.rows.len(),
            "sealed alternate-to-enacted correspondence rows",
        ),
        transitive_pair_stage_proof_count: proof_inventory(
            transitivity_comparisons.len(),
            "pair-stage identities proved by enacted-root transitivity",
        ),
        fork_shadow_comparisons,
        enacted_root_transitivity_comparisons: transitivity_comparisons,
        every_stage4_reference_recursively_relabelled: every_reference_relabelled,
        all_fork_shadow_telescopes_identical: all_fork_shadow_identical,
        all_later_winners_identical_by_enacted_root_transitivity: all_later_identical,
        derivation_hash: String::new(),
    };
    score.derivation_hash = p2_hash(&score);
    Ok(score)
}

fn find_rt2_pair<'a>(
    comparisons: &'a [Rt2V2PairwiseSchemeSetComparison],
    left_root: &str,
    right_root: &str,
) -> Result<&'a Rt2V2PairwiseSchemeSetComparison, Uc1ScoringV1Error> {
    let rows = comparisons
        .iter()
        .filter(|row| {
            (row.left_candidate_hash == left_root && row.right_candidate_hash == right_root)
                || (row.left_candidate_hash == right_root && row.right_candidate_hash == left_root)
        })
        .collect::<Vec<_>>();
    if rows.len() != 1 {
        return Err(Uc1ScoringV1Error::Input(format!(
            "R-T2 has {} rows for branch pair {left_root} / {right_root}",
            rows.len()
        )));
    }
    Ok(rows[0])
}

fn score_p3(inputs: &SealedInputs) -> Result<PredictionThreeScoreV1, Uc1ScoringV1Error> {
    let branches = sorted_bi1(&inputs.bi1)?;
    let pairs = branch_pairs(&branches);
    if pairs.len() != 6 || inputs.r_t2.pairwise_scheme_set_comparisons.len() != 6 {
        return Err(Uc1ScoringV1Error::Input(
            "P3 requires the exact six R-T2 branch pairs".to_owned(),
        ));
    }
    let mut rows = Vec::new();
    for (left, right) in pairs {
        let comparison = find_rt2_pair(
            &inputs.r_t2.pairwise_scheme_set_comparisons,
            &left.branch_root_hash,
            &right.branch_root_hash,
        )?;
        let left_live_ids = stage_live_scheme_ids(left, 5)?;
        let right_live_ids = stage_live_scheme_ids(right, 5)?;
        if left_live_ids.len() != 1 || right_live_ids.len() != 1 {
            return Err(Uc1ScoringV1Error::Input(format!(
                "P3 branch pair {} / {} does not have singleton Stage-5 live demands",
                left.branch_root_hash, right.branch_root_hash
            )));
        }
        let (left_live, right_live, live_pair_in_matching) =
            if comparison.left_candidate_hash == left.branch_root_hash {
                (
                    left_live_ids[0].clone(),
                    right_live_ids[0].clone(),
                    comparison
                        .perfect_matching
                        .contains(&(left_live_ids[0].clone(), right_live_ids[0].clone())),
                )
            } else {
                (
                    left_live_ids[0].clone(),
                    right_live_ids[0].clone(),
                    comparison
                        .perfect_matching
                        .contains(&(right_live_ids[0].clone(), left_live_ids[0].clone())),
                )
            };
        let mut row = MarginConfinementCounterexampleV1 {
            stage: syntax_identifier(5, "Stage-5 live demand compared with R-T2 matching"),
            left_branch_root: left.branch_root_hash.clone(),
            right_branch_root: right.branch_root_hash.clone(),
            left_live_structural_scheme_id: left_live,
            right_live_structural_scheme_id: right_live,
            left_live_scheme_count: proof_inventory(
                left_live_ids.len(),
                "left branch live structural demand schemes",
            ),
            right_live_scheme_count: proof_inventory(
                right_live_ids.len(),
                "right branch live structural demand schemes",
            ),
            perfect_matching_edge_count: proof_inventory(
                comparison.perfect_matching.len(),
                "literal edges in this sealed R-T2 perfect matching",
            ),
            live_pair_is_literal_perfect_matching_edge: live_pair_in_matching,
            f_uc2_counterexample: !live_pair_in_matching,
            r_t2_pair_derivation_hash: comparison.derivation_hash.clone(),
            derivation_hash: String::new(),
        };
        row.derivation_hash = p3_counterexample_hash(&row);
        rows.push(row);
    }
    let every_live_pair_in_matching = rows
        .iter()
        .all(|row| row.live_pair_is_literal_perfect_matching_edge);
    let counterexample_count = rows.iter().filter(|row| row.f_uc2_counterexample).count();
    if rows.len() != 6 || counterexample_count != 6 || every_live_pair_in_matching {
        return Err(Uc1ScoringV1Error::Invariant(
            "P3 did not reproduce all six registered live-demand counterexamples".to_owned(),
        ));
    }
    let mut score = PredictionThreeScoreV1 {
        prediction_id: "P3_MARGIN_CONFINEMENT".to_owned(),
        quantifier: ClaimQuantifier::EveryRt2BranchPairStageFiveLiveDemand,
        verdict: PredictionVerdict::Refuted,
        unordered_pair_count: proof_inventory(rows.len(), "R-T2 branch pairs scored by P3"),
        counterexample_count: proof_inventory(
            counterexample_count,
            "literal live-demand pairs absent from their perfect matching",
        ),
        counterexamples: rows,
        every_live_pair_in_pairwise_matching: every_live_pair_in_matching,
        f_uc2_triggered: true,
        derivation_hash: String::new(),
    };
    score.derivation_hash = p3_hash(&score);
    Ok(score)
}

fn score_f_uc4(inputs: &SealedInputs) -> Result<Fuc4ScoreV1, Uc1ScoringV1Error> {
    if !inputs.m1.f_uc4_transport_gate_ready_for_separate_scoring
        || inputs.m1.uc1_scored
        || !inputs.m1.generic_typed_transport_preserves_semantic_nu
        || !inputs
            .m1
            .every_map_has_typed_inverse_and_identity_composition_coherence
        || !inputs.m1.every_induced_bijection_exactly_matches_v1
    {
        return Err(Uc1ScoringV1Error::Input(
            "M-1 did not present its exact unscored F-UC4 transport gate".to_owned(),
        ));
    }
    let mut rows = Vec::new();
    for obstruction in &inputs.m1.order_axis_obstructions {
        if obstruction.left_semantic_nu.register != TransportNumericRegister::SemanticFamilyNu
            || obstruction.right_semantic_nu.register != TransportNumericRegister::SemanticFamilyNu
        {
            return Err(Uc1ScoringV1Error::Input(
                "M-1 order obstruction carries the wrong numeric register".to_owned(),
            ));
        }
        let pair = BTreeSet::from([
            obstruction.left_semantic_nu.value,
            obstruction.right_semantic_nu.value,
        ]);
        if pair != BTreeSet::from([2_u64, 3_u64])
            || !obstruction.semantic_nu_distinct
            || !obstruction.no_accepted_typed_family_bijection_exists
            || !obstruction.accepted_equivalence_requires_typed_map_inverse_and_coherence
        {
            return Err(Uc1ScoringV1Error::Invariant(
                "an M-1 order obstruction is not the required semantic-family 3-versus-2 row"
                    .to_owned(),
            ));
        }
        let mut row = Fuc4OrderObstructionV1 {
            fixed_coordinate: obstruction.fixed_coordinate.clone(),
            left_candidate_hash: obstruction.left_candidate_hash.clone(),
            right_candidate_hash: obstruction.right_candidate_hash.clone(),
            left_semantic_nu: semantic_family_nu(
                obstruction.left_semantic_nu.value,
                "left act semantic-family cardinality",
            ),
            right_semantic_nu: semantic_family_nu(
                obstruction.right_semantic_nu.value,
                "right act semantic-family cardinality",
            ),
            semantic_nu_distinct: obstruction.semantic_nu_distinct,
            no_accepted_typed_family_bijection_exists: obstruction
                .no_accepted_typed_family_bijection_exists,
            m1_obstruction_derivation_hash: obstruction.derivation_hash.clone(),
            derivation_hash: String::new(),
        };
        row.derivation_hash = fuc4_obstruction_hash(&row);
        rows.push(row);
    }
    let both_obstructions = rows.len() == 2
        && rows
            .iter()
            .map(|row| row.fixed_coordinate.as_str())
            .collect::<BTreeSet<_>>()
            == BTreeSet::from(["former=pi", "former=sigma"])
        && rows
            .iter()
            .all(|row| row.semantic_nu_distinct && row.no_accepted_typed_family_bijection_exists);
    if !both_obstructions {
        return Err(Uc1ScoringV1Error::Invariant(
            "F-UC4 did not receive both order-axis obstructions".to_owned(),
        ));
    }
    let mut score = Fuc4ScoreV1 {
        falsifier_id: "F-UC4".to_owned(),
        quantifier: ClaimQuantifier::AcceptedProofBearingTransportClassOrderAxis,
        transport_scope_boundary_id: inputs.m1.scope_boundary_id.clone(),
        exported_typed_transport_map_count: proof_inventory(
            inputs.m1.typed_transport_maps.len(),
            "M-1 proof-bearing typed transport maps replayed",
        ),
        order_axis_obstruction_count: proof_inventory(
            rows.len(),
            "order-axis no-bijection obstructions applied",
        ),
        order_axis_obstructions: rows,
        generic_transport_invariance_replayed: true,
        both_order_axis_three_versus_two_obstructions_replayed: both_obstructions,
        order_axis_equivalence_refuted_in_adopted_transport_class: true,
        burns_uc1a: true,
        burns_uc1d: true,
        does_not_assess_uc1b_former_semantics: true,
        derivation_hash: String::new(),
    };
    score.derivation_hash = fuc4_hash(&score);
    Ok(score)
}

fn clause_disposition(
    clause_id: &str,
    status: Uc1ClauseStatus,
    grounds: &str,
    evidence_derivation_hashes: Vec<String>,
) -> Uc1ClauseDispositionV1 {
    let mut row = Uc1ClauseDispositionV1 {
        clause_id: clause_id.to_owned(),
        status,
        grounds: grounds.to_owned(),
        evidence_derivation_hashes,
        derivation_hash: String::new(),
    };
    row.derivation_hash = clause_hash(&row);
    row
}

fn build_certificate() -> Result<Uc1ScoringV1Certificate, Uc1ScoringV1Error> {
    let mainline = std::str::from_utf8(MAINLINE_PLAN_BYTES)
        .map_err(|error| Uc1ScoringV1Error::Prerequisite(error.to_string()))?;
    let hypothesis = std::str::from_utf8(UC1_HYPOTHESIS_BYTES)
        .map_err(|error| Uc1ScoringV1Error::Prerequisite(error.to_string()))?;
    let register = std::str::from_utf8(NU_REGISTER_BYTES)
        .map_err(|error| Uc1ScoringV1Error::Prerequisite(error.to_string()))?;
    if !mainline.contains("M-2")
        || !mainline.contains("P1")
        || !mainline.contains("F-UC4")
        || !mainline.contains("U_T2_UNATTEMPTED")
        || !hypothesis.contains("UC-1a (order gauge)")
        || !hypothesis.contains("P3 (margin confinement)")
        || !hypothesis.contains("Z-LEDGER")
        || !register.contains("nu-register-split-v1")
        || !register.contains("F-NR6")
    {
        return Err(Uc1ScoringV1Error::Prerequisite(
            "the frozen M-2, UC-1, or numeric-register brief drifted".to_owned(),
        ));
    }

    let inputs = parse_inputs()?;
    let lineage = authenticate_lineage(&inputs)?;
    let p1 = score_p1(&inputs)?;
    let p2 = score_p2(&inputs)?;
    let p3 = score_p3(&inputs)?;
    let f_uc4 = score_f_uc4(&inputs)?;
    let clause_dispositions = vec![
        clause_disposition(
            "UC-1a",
            Uc1ClauseStatus::Burned,
            "F-UC4: semantic-family nu is invariant in the adopted proof-bearing transport class, while both within-former order pairs carry certified 3-versus-2 [semantic_family_nu] obstructions",
            vec![f_uc4.derivation_hash.clone()],
        ),
        clause_disposition(
            "UC-1b",
            Uc1ClauseStatus::Untested,
            "U_T2_UNATTEMPTED: M-1's narrow transport class does not construct or obstruct Pi/Sigma act-level former equivalence",
            vec![inputs.m1.result_digest.clone()],
        ),
        clause_disposition(
            "UC-1c",
            Uc1ClauseStatus::Burned,
            "F-UC2: every sealed Stage-5 live-demand pair is absent from its R-T2 perfect matching, so margin confinement is refuted",
            vec![p3.derivation_hash.clone()],
        ),
        clause_disposition(
            "UC-1d",
            Uc1ClauseStatus::Burned,
            "full collapse fails independently through the burned order-gauge and healing clauses",
            vec![f_uc4.derivation_hash.clone(), p3.derivation_hash.clone()],
        ),
    ];
    let bindings = source_bindings();
    let mut certificate = Uc1ScoringV1Certificate {
        schema: UC1_SCORING_V1_SCHEMA.to_owned(),
        date: UC1_SCORING_V1_DATE.to_owned(),
        input_artifact_count: registered_usize(
            bindings.len(),
            Uc1NumericRegister::ArtifactMetadata,
            "byte-bound plan, certificate, and issuer inputs",
        ),
        source_bindings: bindings,
        numeric_register_policy:
            "nu-register-split-v1: semantic_family_nu is law-level authority; proof_inventory counts evidence; structural_testimony is diagnostic only; syntax_identifier labels stages; artifact_metadata describes files"
                .to_owned(),
        lineage,
        p1,
        p2,
        p3,
        f_uc4,
        clause_dispositions,
        outcome_zone: UC1_SCORING_V1_ZONE.to_owned(),
        sole_former_axis_residual: UC1_SCORING_V1_RESIDUAL.to_owned(),
        residual_nonblocking_because_p3_refuted: true,
        z_collapse_or_z_heal_guessed: false,
        no_new_equivalence_semantics_assumed: true,
        no_candidate_or_value_recomputed_or_newly_observed_for_scoring: true,
        semantic_nu_bar_hash_or_enumeration_order_used_as_selector: false,
        mutation_falsifiers: vec![
            "flip_any_prediction_or_clause_verdict_then_replay_must_fail".to_owned(),
            "change_any_bound_digest_then_replay_must_fail".to_owned(),
            "change_any_claim_quantifier_then_replay_must_fail".to_owned(),
            "change_any_numeric_register_tag_then_replay_must_fail".to_owned(),
            "promote_any_P3_live_pair_to_a_matching_edge_then_replay_must_fail".to_owned(),
            "drop_or_weaken_either_F_UC4_order_obstruction_then_replay_must_fail".to_owned(),
            "toggle_R_T2_live_or_sealed_fallback_status_or_remove_its_exact_error_then_replay_must_fail"
                .to_owned(),
        ],
        permitted_conclusion: "P1 Passed and P2 Passed, but P3 Refuted under F-UC2; F-UC4 independently burns UC-1a in the adopted narrow transport class. Therefore UC-1c and UC-1d are burned, the scored zone is Z-LEDGER, and UC-1b remains only as U_T2_UNATTEMPTED.".to_owned(),
        forbidden_conclusion: "This artifact does not infer Pi/Sigma former equivalence or inequivalence, does not guess Z-COLLAPSE or Z-HEAL, does not broaden M-1's transport class, and does not use semantic nu, structural testimony, a bar, a hash, or enumeration order as a selector.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_hash(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<Result<Uc1ScoringV1Certificate, String>> = OnceLock::new();

fn expected_certificate() -> Result<&'static Uc1ScoringV1Certificate, Uc1ScoringV1Error> {
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(Uc1ScoringV1Error::Input(error.clone())),
    }
}

pub fn issue_uc1_scoring_v1() -> Result<Uc1ScoringV1Certificate, Uc1ScoringV1Error> {
    expected_certificate().cloned()
}

fn registered_number_has(number: &RegisteredNumber, register: Uc1NumericRegister) -> bool {
    number.register == register && !number.meaning.is_empty()
}

fn registers_valid(certificate: &Uc1ScoringV1Certificate) -> bool {
    certificate.source_bindings.iter().all(|binding| {
        registered_number_has(&binding.byte_length, Uc1NumericRegister::ArtifactMetadata)
    }) && registered_number_has(
        &certificate.input_artifact_count,
        Uc1NumericRegister::ArtifactMetadata,
    ) && registered_number_has(
        &certificate.lineage.exact_joined_branch_count,
        Uc1NumericRegister::ProofInventory,
    ) && [
        &certificate.p1.branch_count,
        &certificate.p1.unordered_pair_count,
        &certificate.p1.stage_count,
        &certificate.p2.branch_count,
        &certificate.p2.unordered_pair_count,
        &certificate.p2.fork_shadow_stage_count,
        &certificate.p2.fork_shadow_pair_stage_comparison_count,
        &certificate.p2.correspondence_stage_count,
        &certificate.p2.sealed_correspondence_input_row_count,
        &certificate.p2.transitive_pair_stage_proof_count,
        &certificate.p3.unordered_pair_count,
        &certificate.p3.counterexample_count,
        &certificate.f_uc4.exported_typed_transport_map_count,
        &certificate.f_uc4.order_axis_obstruction_count,
    ]
    .iter()
    .all(|number| registered_number_has(number, Uc1NumericRegister::ProofInventory))
        && [
            &certificate.p1.stage_band_first,
            &certificate.p1.stage_band_last,
        ]
        .iter()
        .all(|number| registered_number_has(number, Uc1NumericRegister::SyntaxIdentifier))
        && certificate.p2.fork_shadow_comparisons.iter().all(|row| {
            registered_number_has(&row.stage, Uc1NumericRegister::SyntaxIdentifier)
                && registered_number_has(
                    &row.left_stage4_reference_relabel_count,
                    Uc1NumericRegister::ProofInventory,
                )
                && registered_number_has(
                    &row.right_stage4_reference_relabel_count,
                    Uc1NumericRegister::ProofInventory,
                )
        })
        && certificate
            .p2
            .enacted_root_transitivity_comparisons
            .iter()
            .all(|row| registered_number_has(&row.stage, Uc1NumericRegister::SyntaxIdentifier))
        && certificate.p3.counterexamples.iter().all(|row| {
            registered_number_has(&row.stage, Uc1NumericRegister::SyntaxIdentifier)
                && [
                    &row.left_live_scheme_count,
                    &row.right_live_scheme_count,
                    &row.perfect_matching_edge_count,
                ]
                .iter()
                .all(|number| registered_number_has(number, Uc1NumericRegister::ProofInventory))
        })
        && certificate.f_uc4.order_axis_obstructions.iter().all(|row| {
            registered_number_has(&row.left_semantic_nu, Uc1NumericRegister::SemanticFamilyNu)
                && registered_number_has(
                    &row.right_semantic_nu,
                    Uc1NumericRegister::SemanticFamilyNu,
                )
        })
}

fn row_hashes_valid(certificate: &Uc1ScoringV1Certificate) -> bool {
    certificate.lineage.derivation_hash == lineage_hash(&certificate.lineage)
        && certificate.p1.derivation_hash == p1_hash(&certificate.p1)
        && certificate
            .p2
            .fork_shadow_comparisons
            .iter()
            .all(|row| row.derivation_hash == fork_shadow_hash(row))
        && certificate
            .p2
            .enacted_root_transitivity_comparisons
            .iter()
            .all(|row| row.derivation_hash == transitivity_hash(row))
        && certificate.p2.derivation_hash == p2_hash(&certificate.p2)
        && certificate
            .p3
            .counterexamples
            .iter()
            .all(|row| row.derivation_hash == p3_counterexample_hash(row))
        && certificate.p3.derivation_hash == p3_hash(&certificate.p3)
        && certificate
            .f_uc4
            .order_axis_obstructions
            .iter()
            .all(|row| row.derivation_hash == fuc4_obstruction_hash(row))
        && certificate.f_uc4.derivation_hash == fuc4_hash(&certificate.f_uc4)
        && certificate
            .clause_dispositions
            .iter()
            .all(|row| row.derivation_hash == clause_hash(row))
}

fn replay_against_expected(
    claimed: &Uc1ScoringV1Certificate,
    expected: &Uc1ScoringV1Certificate,
) -> Uc1ScoringV1Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_hash(claimed) {
        errors.push("UC-1 certificate digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("UC-1 source bindings drifted".to_owned());
    }
    if !registers_valid(claimed) {
        errors.push("UC-1 numeric register policy failed".to_owned());
    }
    if !row_hashes_valid(claimed) {
        errors.push("UC-1 row derivation hash failed".to_owned());
    }
    let r_t2_lineage_valid = claimed.lineage.r_t2_full_current_source_reissue_attempted
        && !claimed.lineage.r_t2_live_reissue_valid
        && !claimed.lineage.r_t2_live_reissue_errors.is_empty()
        && !claimed
            .lineage
            .r_t2_live_reissue_serialized_blockers
            .is_empty()
        && claimed.lineage.r_t2_sealed_fallback_used
        && claimed.lineage.r_t2_certificate_self_digest_valid
        && claimed.lineage.r_t2_archived_stage4_projection_replay_valid
        && claimed.lineage.r_t2_p3_pair_surface_valid
        && claimed.lineage.r_t2_sealed_surface_authentication_valid;
    if !r_t2_lineage_valid {
        errors.push("UC-1 R-T2 live/fallback lineage failed".to_owned());
    }
    if claimed != expected {
        errors.push("UC-1 certificate differs from deterministic reissuance".to_owned());
    }
    Uc1ScoringV1Replay {
        valid: errors.is_empty(),
        p1_verdict: Some(claimed.p1.verdict),
        p2_verdict: Some(claimed.p2.verdict),
        p3_verdict: Some(claimed.p3.verdict),
        outcome_zone: Some(claimed.outcome_zone.clone()),
        sole_residual: Some(claimed.sole_former_axis_residual.clone()),
        errors,
    }
}

pub fn replay_uc1_scoring_v1(claimed: &Uc1ScoringV1Certificate) -> Uc1ScoringV1Replay {
    match expected_certificate() {
        Ok(expected) => replay_against_expected(claimed, expected),
        Err(error) => Uc1ScoringV1Replay {
            valid: false,
            p1_verdict: None,
            p2_verdict: None,
            p3_verdict: None,
            outcome_zone: None,
            sole_residual: None,
            errors: vec![error.to_string()],
        },
    }
}

pub fn replay_uc1_scoring_v1_json(json: &str) -> Uc1ScoringV1Replay {
    match serde_json::from_str::<Uc1ScoringV1Certificate>(json) {
        Ok(certificate) => replay_uc1_scoring_v1(&certificate),
        Err(error) => Uc1ScoringV1Replay {
            valid: false,
            p1_verdict: None,
            p2_verdict: None,
            p3_verdict: None,
            outcome_zone: None,
            sole_residual: None,
            errors: vec![format!("UC-1 certificate failed to deserialize: {error}")],
        },
    }
}

fn register_label(number: &RegisteredNumber) -> &'static str {
    match number.register {
        Uc1NumericRegister::SemanticFamilyNu => "semantic_family_nu",
        Uc1NumericRegister::ProofInventory => "proof_inventory",
        Uc1NumericRegister::StructuralTestimony => "structural_testimony",
        Uc1NumericRegister::SyntaxIdentifier => "syntax_identifier",
        Uc1NumericRegister::ArtifactMetadata => "artifact_metadata",
    }
}

fn rendered_number(number: &RegisteredNumber) -> String {
    format!("{} [{}]", number.value, register_label(number))
}

pub fn render_uc1_scoring_v1(certificate: &Uc1ScoringV1Certificate) -> String {
    let mut early_rows = String::new();
    for row in &certificate.p2.fork_shadow_comparisons {
        early_rows.push_str(&format!(
            "| {} | `{}` / `{}` | {} / {} | {} |\n",
            rendered_number(&row.stage),
            &row.left_branch_root[7..19],
            &row.right_branch_root[7..19],
            rendered_number(&row.left_stage4_reference_relabel_count),
            rendered_number(&row.right_stage4_reference_relabel_count),
            row.canonical_telescopes_identical,
        ));
    }
    let mut p3_rows = String::new();
    for row in &certificate.p3.counterexamples {
        p3_rows.push_str(&format!(
            "| `{}` / `{}` | `{}` / `{}` | {} | {} |\n",
            &row.left_branch_root[7..19],
            &row.right_branch_root[7..19],
            &row.left_live_structural_scheme_id[7..19],
            &row.right_live_structural_scheme_id[7..19],
            rendered_number(&row.perfect_matching_edge_count),
            row.live_pair_is_literal_perfect_matching_edge,
        ));
    }
    let mut obstruction_rows = String::new();
    for row in &certificate.f_uc4.order_axis_obstructions {
        obstruction_rows.push_str(&format!(
            "| `{}` | `{}` / `{}` | {} / {} | {} |\n",
            row.fixed_coordinate,
            &row.left_candidate_hash[7..19],
            &row.right_candidate_hash[7..19],
            rendered_number(&row.left_semantic_nu),
            rendered_number(&row.right_semantic_nu),
            row.no_accepted_typed_family_bijection_exists,
        ));
    }
    format!(
        "# UC-1 scoring v1\n\n**Date:** {}. **Zone:** `{}`. **Certificate:** `{}`.\n\nThe sealed inputs replay and join: M-1 transport `{}`, BI-2 `{}`, and BI-4 `{}`. Input surface: {}. Prerequisite reissuance is authentication only; it is never a scoring observation.\n\n**R-T2 drift disclosure:** live current-source reissue valid = `{}`; sealed fallback used = `{}`; sealed fallback authenticated = `{}`. Exact live errors: `{}`. Exact serialized blockers: `{}`. The fallback requires, and this certificate separately records, the certificate self-digest, archived Stage-4 fork projection replay, and full P3 pair-surface gates.\n\nRegister notation: dates are `[artifact_metadata]`; P/M/F/U labels, Stage labels, hash suffixes, and the numeral in `Expr::Lib(4)` are `[syntax_identifier]`. Every quantitative payload below is rendered as `value [register]`.\n\n## Mechanical score\n\n| prediction | verdict | quantified evidence |\n|---|---|---|\n| P1 ledger identity | `{:?}` | {} branches, {} unordered pairs, {} stages |\n| P2 winner identity modulo fork shadow | `{:?}` | {} normalized pair-stage comparisons plus {} enacted-root-transitive pair-stage proofs |\n| P3 margin confinement | `{:?}` | {} counterexamples over {} unordered pairs |\n\nEvery displayed quantity is followed by its numeric register. Structural testimony is not consumed as semantic authority.\n\n## P2 fork-shadow normalization\n\nThe relabeler recursively replaces every `Expr::Lib(4)` with a branch-neutral Stage-4-reference marker before comparing telescopes. A zero `[proof_inventory]` count means the registered relabeling was run but no such reference occurred.\n\n| stage | branch pair | relabelled references (left / right) | normalized telescope equal |\n|---|---|---:|---|\n{}\nThe later-band proof uses {} sealed alternate-to-enacted rows to construct {} pair-stage identities through enacted-root transitivity; no pair identity is assumed.\n\n## P3 / F-UC2 counterexamples\n\nThe live demand is read from each retired BI-1 Stage-5 demand record and tested as the literal ordered pair required by its sealed R-T2 perfect matching.\n\n| branch pair | live structural scheme IDs | matching edges | live pair matched |\n|---|---|---:|---|\n{}\nP3 is `Refuted`; F-UC2 burns UC-1c and therefore UC-1d.\n\n## F-UC4\n\nM-1 contributes {} typed maps and {} order-axis obstructions in its exact narrow proof-bearing transport class.\n\n| fixed former | order candidates | semantic-family nu | accepted typed bijection absent |\n|---|---|---:|---|\n{}\nThe two within-former semantic-family 3-versus-2 `[semantic_family_nu]` obstructions burn UC-1a and UC-1d in the adopted class. They do not assess UC-1b's Pi/Sigma act-level former semantics.\n\n## Disposition\n\n`UC-1a = Burned`; `UC-1b = Untested`; `UC-1c = Burned`; `UC-1d = Burned`. The scored zone is `{}`. The sole former-axis residual is `{}`; it is nonblocking here because P3 is already refuted. This artifact does not guess Z-COLLAPSE or Z-HEAL.\n\nPermitted conclusion: {}\n\nForbidden conclusion: {}\n",
        certificate.date,
        certificate.outcome_zone,
        certificate.result_digest,
        certificate.lineage.m1_transport_replay_valid,
        certificate.lineage.bi2_bundle_replay_valid,
        certificate.lineage.bi4_replay_valid,
        rendered_number(&certificate.input_artifact_count),
        certificate.lineage.r_t2_live_reissue_valid,
        certificate.lineage.r_t2_sealed_fallback_used,
        certificate.lineage.r_t2_sealed_surface_authentication_valid,
        certificate.lineage.r_t2_live_reissue_errors.join("; "),
        certificate
            .lineage
            .r_t2_live_reissue_serialized_blockers
            .join("; "),
        certificate.p1.verdict,
        rendered_number(&certificate.p1.branch_count),
        rendered_number(&certificate.p1.unordered_pair_count),
        rendered_number(&certificate.p1.stage_count),
        certificate.p2.verdict,
        rendered_number(&certificate.p2.fork_shadow_pair_stage_comparison_count),
        rendered_number(&certificate.p2.transitive_pair_stage_proof_count),
        certificate.p3.verdict,
        rendered_number(&certificate.p3.counterexample_count),
        rendered_number(&certificate.p3.unordered_pair_count),
        early_rows,
        rendered_number(&certificate.p2.sealed_correspondence_input_row_count),
        rendered_number(&certificate.p2.transitive_pair_stage_proof_count),
        p3_rows,
        rendered_number(&certificate.f_uc4.exported_typed_transport_map_count),
        rendered_number(&certificate.f_uc4.order_axis_obstruction_count),
        obstruction_rows,
        certificate.outcome_zone,
        certificate.sole_former_axis_residual,
        certificate.permitted_conclusion,
        certificate.forbidden_conclusion,
    )
}

pub fn emit_uc1_scoring_v1_create_new(
    directory: &Path,
) -> Result<Uc1ScoringV1Replay, Uc1ScoringV1Error> {
    let certificate = issue_uc1_scoring_v1()?;
    let certificate_path = directory.join(UC1_SCORING_V1_CERTIFICATE_NAME);
    let report_path = directory.join(UC1_SCORING_V1_REPORT_NAME);
    if certificate_path.exists() || report_path.exists() {
        return Err(Uc1ScoringV1Error::Io(format!(
            "create-new target already exists: certificate={} report={}",
            certificate_path.display(),
            report_path.display()
        )));
    }
    let certificate_bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Uc1ScoringV1Error::Json(error.to_string()))?;
    let report_bytes = render_uc1_scoring_v1(&certificate).into_bytes();
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| Uc1ScoringV1Error::Io(error.to_string()))?;
    let mut report_file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
    {
        Ok(file) => file,
        Err(error) => {
            drop(certificate_file);
            let _ = remove_file(&certificate_path);
            return Err(Uc1ScoringV1Error::Io(error.to_string()));
        }
    };
    if let Err(error) = certificate_file
        .write_all(&certificate_bytes)
        .and_then(|_| certificate_file.write_all(b"\n"))
        .and_then(|_| report_file.write_all(&report_bytes))
    {
        drop(certificate_file);
        drop(report_file);
        let _ = remove_file(&certificate_path);
        let _ = remove_file(&report_path);
        return Err(Uc1ScoringV1Error::Io(error.to_string()));
    }
    drop(certificate_file);
    drop(report_file);
    let emitted_json = match std::fs::read_to_string(&certificate_path) {
        Ok(json) => json,
        Err(error) => {
            let _ = remove_file(&certificate_path);
            let _ = remove_file(&report_path);
            return Err(Uc1ScoringV1Error::Io(format!(
                "could not reread emitted UC-1 certificate: {error}"
            )));
        }
    };
    let replay = replay_uc1_scoring_v1_json(&emitted_json);
    if !replay.valid {
        let _ = remove_file(&certificate_path);
        let _ = remove_file(&report_path);
        return Err(Uc1ScoringV1Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uc1_scoring_replays_and_all_registered_mutations_fail() {
        let certificate = issue_uc1_scoring_v1().expect("M-2 issues");
        let replay = replay_uc1_scoring_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(certificate.p1.verdict, PredictionVerdict::Passed);
        assert_eq!(certificate.p2.verdict, PredictionVerdict::Passed);
        assert_eq!(certificate.p3.verdict, PredictionVerdict::Refuted);
        assert_eq!(certificate.outcome_zone, UC1_SCORING_V1_ZONE);
        assert_eq!(
            certificate.sole_former_axis_residual,
            UC1_SCORING_V1_RESIDUAL
        );

        let expected = certificate;

        let mut verdict = expected.clone();
        verdict.p3.verdict = PredictionVerdict::Passed;
        assert!(!replay_against_expected(&verdict, &expected).valid);

        let mut digest = expected.clone();
        digest.result_digest.push_str("-mutated");
        assert!(!replay_against_expected(&digest, &expected).valid);

        let mut quantifier = expected.clone();
        quantifier.p2.quantifier = ClaimQuantifier::ConeAllBranchesStageFiveThroughFifteen;
        assert!(!replay_against_expected(&quantifier, &expected).valid);

        let mut register = expected.clone();
        register.p1.branch_count.register = Uc1NumericRegister::StructuralTestimony;
        assert!(!replay_against_expected(&register, &expected).valid);

        let mut p3_edge = expected.clone();
        p3_edge.counterexamples_mut()[0].live_pair_is_literal_perfect_matching_edge = true;
        assert!(!replay_against_expected(&p3_edge, &expected).valid);

        let mut f_uc4 = expected.clone();
        f_uc4.f_uc4.order_axis_obstructions[0].no_accepted_typed_family_bijection_exists = false;
        assert!(!replay_against_expected(&f_uc4, &expected).valid);

        let mut fallback_status = expected.clone();
        fallback_status.lineage.r_t2_sealed_fallback_used = false;
        assert!(!replay_against_expected(&fallback_status, &expected).valid);

        let mut fallback_error = expected.clone();
        fallback_error.lineage.r_t2_live_reissue_errors.clear();
        assert!(!replay_against_expected(&fallback_error, &expected).valid);

        let temporary_directory = std::env::temp_dir().join(format!(
            "atomic-uc1-scoring-v1-{}-{}",
            std::process::id(),
            &expected.result_digest[7..19]
        ));
        std::fs::create_dir(&temporary_directory).expect("unique UC-1 test directory creates");
        let emitted =
            emit_uc1_scoring_v1_create_new(&temporary_directory).expect("create-new emits");
        assert!(emitted.valid, "{:?}", emitted.errors);
        let certificate_path = temporary_directory.join(UC1_SCORING_V1_CERTIFICATE_NAME);
        let report_path = temporary_directory.join(UC1_SCORING_V1_REPORT_NAME);
        assert!(certificate_path.is_file());
        assert!(report_path.is_file());
        let emitted_json =
            std::fs::read_to_string(&certificate_path).expect("emitted JSON rereads");
        let emitted_replay = replay_uc1_scoring_v1_json(&emitted_json);
        assert!(emitted_replay.valid, "{:?}", emitted_replay.errors);
        std::fs::remove_file(certificate_path).expect("exact temporary certificate removes");
        std::fs::remove_file(report_path).expect("exact temporary report removes");
        std::fs::remove_dir(temporary_directory).expect("empty exact temporary directory removes");
    }
}

impl Uc1ScoringV1Certificate {
    #[cfg(test)]
    fn counterexamples_mut(&mut self) -> &mut Vec<MarginConfinementCounterexampleV1> {
        &mut self.p3.counterexamples
    }
}
