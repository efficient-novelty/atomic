//! Verdict-blind Stage-4 semantic parsimony audit.
//!
//! The four-root geometry is enumerated live from the guarded Stage-4 cone;
//! no R-T2 outcome-shaped artifact contributes to that enumeration.  Each
//! root then earns registry-erased B1/B2 and prefix-generic B3 evidence.
//! Structural `(kappa, nu) = (3, 5)` testimony, the frozen R-T2 successor
//! comparison, and the enacted-history digest are read strictly after the
//! semantic parsimony result is sealed.  The theorem issuer has no
//! branch-continuation or artifact-write capability; this module deliberately
//! exposes a separate outer create-new serializer.

use crate::act_local_semantic_provenance_v5::issue_act_local_semantic_sequence_v5;
use crate::bi0_semantic_register_v5::{
    BI0_SEMANTIC_REGISTER_V5_SCHEMA, Bi0SemanticRegisterV5Certificate,
    replay_bi0_semantic_register_v5_certificate,
};
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::naturality_orbit_transport::{
    issue_stage4_r_t1_orbit_audit, replay_stage4_r_t1_orbit_audit,
};
use crate::phase5b_reselection_v2::{PHASE5B_RESELECTION_BURN_SCHEMA, Phase5bReselectionBurn};
use crate::r_t2_future_hole_confluence_v2::{
    R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA, Rt2FutureHoleConfluenceV2Certificate,
    Rt2V2PairwiseSchemeSetComparison, replay_archived_stage4_fork_projection,
};
use crate::t_bi_intrinsic_isolation_v3::{
    issue_t_bi_intrinsic_isolation_v3, replay_t_bi_intrinsic_isolation_v3,
};
use pen_core::canonical::canonical_key_telescope;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_type::admissibility::{
    AdmissibilityMode, passes_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA: &str = "stage4-semantic-parsimony-verdict-blind-v1";
pub const STAGE4_SEMANTIC_PARSIMONY_V1_DATE: &str = "2026-07-22";
pub const STAGE4_SEMANTIC_PARSIMONY_V1_THEOREM_ID: &str =
    "T-SP4-v1-four-root-intrinsic-semantic-parsimony-and-postseal-divergence";

const R_T2_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json");
const PHASE5B_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v2.json");
const BI0_V5_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/bi0_semantic_register_v5.json");
const NU_REGISTER_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/nu_register_adjudication.md");
const TIE_PROTOCOL_BYTES: &[u8] = include_bytes!("../../../docs/tie_resolution_protocol.md");
const R_T3_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/r_t3_stage4_adjudication.md");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA, domain, value))
        .expect("Stage-4 semantic parsimony evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn phase5b_burn_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(PHASE5B_RESELECTION_BURN_SCHEMA, domain, value))
        .expect("Phase-5b burn projection serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn phase5b_burn_digest_valid(burn: &Phase5bReselectionBurn) -> bool {
    let mut projection = burn.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == phase5b_burn_hash("reselection-burn", &projection)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticSourceBindingV1 {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

/// The only BI-0 surface visible before semantic minimization.  Unknown
/// fields in the sealed JSON are deliberately not deserialized.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Bi0OpeningCapabilityProjectionV1 {
    schema: String,
    bi0_attempt_executed: bool,
    bi0_passed: bool,
    bi1_invoked: bool,
    non_enacted_cone_invoked: bool,
    non_enacted_branch_work_executed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4StrictConeRootGeometryV1 {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub telescope: Telescope,
    pub kappa: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4StrictConeGeometryV1 {
    pub stage: u32,
    pub window_depth: u16,
    pub enumerated_kappa_min: u16,
    pub enumerated_kappa_max: u16,
    pub raw_enumerated_count: usize,
    pub strict_admitted_count: usize,
    pub canonical_deduped_count: usize,
    pub least_kappa: u16,
    pub roots: Vec<Stage4StrictConeRootGeometryV1>,
    pub exact_complete_strict_cone: bool,
    pub exact_four_distinct_least_kappa_three_roots: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage4SemanticCapabilityV1 {
    SealedBi0OpeningCapability,
    FullBi0PostsealTestimony,
    AdoptedSemanticRegisterAndTieProtocol,
    LiveStrictConeGeometry,
    CommonStage1Through3Prefix,
    ActLocalV5Reissuance,
    PrefixGenericB3Isolation,
    SemanticParsimonyComparison,
    AdoptedRt3BranchBoundary,
    PostsealStructuralTestimony,
    FrozenRt2PairComparison,
    SealedEnactedRootDigest,
    ContentHashing,
    // Explicitly forbidden in this issuer.
    BranchContinuation,
    Stage5SuccessorEnumeration,
    CanonicalArtifactWrite,
    DesiredVerdict,
}

impl Stage4SemanticCapabilityV1 {
    fn forbidden() -> BTreeSet<Self> {
        [
            Self::BranchContinuation,
            Self::Stage5SuccessorEnumeration,
            Self::CanonicalArtifactWrite,
            Self::DesiredVerdict,
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage4SemanticOperationV1 {
    ReadSealedBi0OpeningProjection,
    BindAdoptedSemanticRegisterAndTieProtocol,
    IssueLiveStrictConeGeometry,
    BindCommonStage1Through3Prefix,
    ReissueActLocalV5,
    ProvePrefixGenericB3,
    SealSemanticParsimony,
    ReplayFullBi0Postseal,
    ReadRt3BranchBoundaryPostseal,
    ReadStructuralTestimonyPostseal,
    JoinFrozenRt2PairPostseal,
    ReadEnactedRootDigestPostseal,
    SealAudit,
}

impl Stage4SemanticOperationV1 {
    fn direct_capabilities(self) -> Vec<Stage4SemanticCapabilityV1> {
        use Stage4SemanticCapabilityV1 as C;
        use Stage4SemanticOperationV1 as O;
        match self {
            O::ReadSealedBi0OpeningProjection => vec![C::SealedBi0OpeningCapability],
            O::BindAdoptedSemanticRegisterAndTieProtocol => {
                vec![C::AdoptedSemanticRegisterAndTieProtocol]
            }
            O::IssueLiveStrictConeGeometry => vec![C::LiveStrictConeGeometry],
            O::BindCommonStage1Through3Prefix => vec![C::CommonStage1Through3Prefix],
            O::ReissueActLocalV5 => vec![C::ActLocalV5Reissuance],
            O::ProvePrefixGenericB3 => vec![C::PrefixGenericB3Isolation],
            O::SealSemanticParsimony => {
                vec![C::SemanticParsimonyComparison, C::ContentHashing]
            }
            O::ReplayFullBi0Postseal => vec![C::FullBi0PostsealTestimony],
            O::ReadRt3BranchBoundaryPostseal => vec![C::AdoptedRt3BranchBoundary],
            O::ReadStructuralTestimonyPostseal => vec![C::PostsealStructuralTestimony],
            O::JoinFrozenRt2PairPostseal => vec![C::FrozenRt2PairComparison],
            O::ReadEnactedRootDigestPostseal => vec![C::SealedEnactedRootDigest],
            O::SealAudit => vec![C::ContentHashing],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticCapabilityRowV1 {
    pub ordinal: u8,
    pub operation: Stage4SemanticOperationV1,
    pub direct_capabilities: Vec<Stage4SemanticCapabilityV1>,
    pub accumulated_capabilities: Vec<Stage4SemanticCapabilityV1>,
    pub forbidden_capabilities: Vec<Stage4SemanticCapabilityV1>,
    pub isolated: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticRootAuditV1 {
    pub candidate_hash: String,
    pub geometry_telescope_hash: String,
    pub kappa: u16,
    pub prefix_signature_digest: String,
    pub non_authoritative_legacy_v5_sequence_derivation_hash: String,
    pub non_authoritative_legacy_v5_sequence_seal: String,
    pub non_authoritative_legacy_v5_package_derivation_hashes: Vec<String>,
    pub authoritative_registry_erased_v3_semantic_seal: String,
    pub prefix_semantic_nu_vector: Vec<u32>,
    pub stage4_semantic_nu: u32,
    pub non_authoritative_legacy_v5_stage4_package_hash: String,
    pub legacy_v5_t_bi_b1_comparison: bool,
    pub legacy_v5_t_bi_b2_comparison: bool,
    pub legacy_v5_named_role_residual_comparison: usize,
    pub legacy_v5_named_quotient_residual_comparison: usize,
    pub legacy_v5_named_a3_residual_comparison: usize,
    pub legacy_v5_silent_residue_comparison: usize,
    pub authoritative_nu_matches_legacy_v5_comparison: bool,
    pub b3_v3_derivation_hash: String,
    pub b3_v3_prefix_generic_isolation_proved: bool,
    pub b3_v3_registry_extension_invariance_proved: bool,
    pub b3_v3_is_sole_prefix_generic_theorem_authority: bool,
    pub b3_v3_legacy_full_hashes_authoritative: bool,
    pub b3_v3_no_forbidden_or_future_semantic_input: bool,
    pub legacy_v5_hashes_used_as_semantic_selector: bool,
    pub legacy_v5_self_reported_forbidden_or_future_input_used: bool,
    pub root_semantic_audit_proved: bool,
    pub derivation_hash: String,
}

/// The exact root projection permitted to influence semantic parsimony.
/// Legacy v5 commitments and the full B3 token digest are intentionally
/// absent: they remain replayable compatibility testimony but cannot enter
/// the pre-testimony selection seal.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticSelectionRootV1 {
    pub candidate_hash: String,
    pub geometry_telescope_hash: String,
    pub kappa: u16,
    pub prefix_signature_digest: String,
    pub authoritative_registry_erased_v3_semantic_seal: String,
    pub prefix_semantic_nu_vector: Vec<u32>,
    pub stage4_semantic_nu: u32,
    pub registry_extension_invariance_proved: bool,
    pub prefix_generic_transitive_isolation_proved: bool,
    pub no_forbidden_or_future_semantic_input: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4Rt2SurvivorJoinV1 {
    pub unordered_candidate_pair: Vec<String>,
    pub left_complete_scheme_count: usize,
    pub right_complete_scheme_count: usize,
    pub matched_scheme_count: usize,
    pub full_scheme_sets_equivalent: bool,
    pub comparison_well_formed: bool,
    pub relative_coverage_and_zero_gap_gate_passed: bool,
    pub hash_or_enumeration_order_used_as_selector: bool,
    pub pairwise_row_derivation_hash_valid: bool,
    pub exactly_one_row_for_surviving_pair: bool,
    pub six_unordered_comparisons_exact: bool,
    pub all_six_pairwise_row_derivation_hashes_valid: bool,
    pub exact_five_five_three_inequivalence_signature: bool,
    pub exact_surviving_pair_join: bool,
    pub frozen_r_t2_comparison_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage4SemanticParsimonyOutcomeV1 {
    TwoSemanticMinimizersRt2InequivalentEnactedRootNonminimalAdjudicationRequired,
    UniqueSemanticMinimumRecordedNoBranchExecuted,
    OtherSemanticConeRecordedAdjudicationRequired,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticParsimonyV1Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub source_bindings: Vec<Stage4SemanticSourceBindingV1>,
    pub preseal_bi0_opening_projection_hash: String,
    pub preseal_bi0_opening_capability_valid: bool,
    pub full_bi0_read_or_replayed_before_semantic_seal: bool,
    pub sealed_bi0_v5_schema: String,
    pub sealed_bi0_v5_digest: String,
    pub sealed_bi0_v5_replay_valid: bool,
    pub sealed_bi0_v5_passed: bool,
    pub sealed_bi0_v5_executed_no_branch_work: bool,
    pub nu_register_adjudication_hash: String,
    pub tie_resolution_protocol_hash: String,
    pub r_t3_stage4_adjudication_hash: String,
    pub preseal_nu_register_and_tie_authority_replayed: bool,
    pub postseal_r_t3_branch_boundary_replayed: bool,
    pub preseal_live_strict_cone_geometry: Stage4StrictConeGeometryV1,
    pub preseal_geometry_projection_hash: String,
    pub preseal_geometry_only_replay_valid: bool,
    pub preseal_exact_exhaustive_four_way_least_kappa_geometry: bool,
    pub frozen_r_t2_certificate_digest: String,
    pub postseal_full_r_t2_projection_replay_valid: bool,
    pub postseal_archived_exact_four_way_r_t1_class_join: bool,
    pub postseal_live_root_hashes_join_archived_r_t2_geometry: bool,
    pub live_strict_cone_only_root_geometry_before_semantic_seal: bool,
    pub common_prefix_steps: Vec<u32>,
    pub common_prefix_signature_digest: String,
    pub roots: Vec<Stage4SemanticRootAuditV1>,
    pub root_count: usize,
    pub every_root_has_authoritative_registry_erased_b1_b2_b3_v3: bool,
    pub pretestimony_semantic_selection_roots: Vec<Stage4SemanticSelectionRootV1>,
    pub semantic_seal_excludes_legacy_v5_commitments: bool,
    pub shared_stage1_through3_semantic_nu: Vec<u32>,
    pub exact_shared_semantic_prefix_across_all_roots: bool,
    pub exact_shared_semantic_prefix_is_1_0_1: bool,
    pub semantic_parsimony_order: String,
    pub minimum_kappa: u16,
    pub minimum_semantic_nu: u32,
    pub semantic_minimizer_hashes: Vec<String>,
    pub semantic_minimizer_count: usize,
    pub semantic_parsimony_seal: String,
    pub structural_testimony_read_after_semantic_seal: bool,
    pub structural_testimony_minimum_kappa: u16,
    pub structural_testimony_minimum_nu: u32,
    pub structural_testimony_minimizer_count: usize,
    pub structural_testimony_used_as_selector: bool,
    pub survivor_r_t2_join: Option<Stage4Rt2SurvivorJoinV1>,
    pub enacted_history_burn_digest: String,
    pub enacted_root_hash: String,
    pub enacted_root_kappa: u16,
    pub enacted_root_semantic_nu: u32,
    pub enacted_root_identified_only_after_semantic_seal: bool,
    pub enacted_root_is_semantic_minimizer: bool,
    pub exact_two_minimizers: bool,
    pub surviving_pair_frozen_r_t2_inequivalent: bool,
    pub enacted_root_nonminimal: bool,
    pub exact_two_minimizer_enacted_nonminimal_divergence: bool,
    pub operation_capability_rows: Vec<Stage4SemanticCapabilityRowV1>,
    pub no_branch_continuation_capability: bool,
    pub no_branch_executed: bool,
    pub theorem_issuer_has_no_artifact_write_capability: bool,
    pub desired_verdict_count_score_or_bar_used_as_semantic_premise: bool,
    pub outcome: Stage4SemanticParsimonyOutcomeV1,
    pub divergence_statement: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Stage4SemanticParsimonyV1Error {
    #[error("Stage-4 semantic parsimony prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("Stage-4 semantic parsimony invariant failed: {0}")]
    Invariant(String),
    #[error("Stage-4 semantic parsimony JSON failed: {0}")]
    Json(String),
    #[error("Stage-4 semantic parsimony I/O failed: {0}")]
    Io(String),
    #[error("Stage-4 semantic parsimony emitted replay failed: {0}")]
    EmittedReplay(String),
}

fn root_hash(root: &Stage4SemanticRootAuditV1) -> String {
    let mut projection = root.clone();
    projection.derivation_hash.clear();
    tagged_hash("semantic-root-audit", &projection)
}

fn semantic_selection_root(root: &Stage4SemanticRootAuditV1) -> Stage4SemanticSelectionRootV1 {
    Stage4SemanticSelectionRootV1 {
        candidate_hash: root.candidate_hash.clone(),
        geometry_telescope_hash: root.geometry_telescope_hash.clone(),
        kappa: root.kappa,
        prefix_signature_digest: root.prefix_signature_digest.clone(),
        authoritative_registry_erased_v3_semantic_seal: root
            .authoritative_registry_erased_v3_semantic_seal
            .clone(),
        prefix_semantic_nu_vector: root.prefix_semantic_nu_vector.clone(),
        stage4_semantic_nu: root.stage4_semantic_nu,
        registry_extension_invariance_proved: root.b3_v3_registry_extension_invariance_proved,
        prefix_generic_transitive_isolation_proved: root.b3_v3_prefix_generic_isolation_proved,
        no_forbidden_or_future_semantic_input: root.b3_v3_no_forbidden_or_future_semantic_input,
    }
}

fn semantic_parsimony_seal(
    roots: &[Stage4SemanticSelectionRootV1],
    minimum_pair: (u16, u32),
    minimizer_hashes: &[String],
) -> String {
    tagged_hash(
        "semantic-parsimony-pretestimony-seal",
        &(
            roots,
            minimum_pair,
            minimizer_hashes,
            "lexicographic least kappa then least authoritative registry-erased semantic-family nu",
        ),
    )
}

fn survivor_join_hash(join: &Stage4Rt2SurvivorJoinV1) -> String {
    let mut projection = join.clone();
    projection.derivation_hash.clear();
    tagged_hash("surviving-pair-r-t2-join", &projection)
}

fn capability_row_hash(row: &Stage4SemanticCapabilityRowV1) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("operation-capability-row", &projection)
}

fn certificate_hash(certificate: &Stage4SemanticParsimonyV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("stage4-semantic-parsimony-certificate", &projection)
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> Stage4SemanticSourceBindingV1 {
    Stage4SemanticSourceBindingV1 {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    }
}

fn strict_cone_geometry_hash(geometry: &Stage4StrictConeGeometryV1) -> String {
    let mut projection = geometry.clone();
    projection.derivation_hash.clear();
    tagged_hash("live-stage4-strict-cone-geometry", &projection)
}

/// Enumerate the complete guarded Stage-4 strict cone without constructing or
/// storing structural nu.  This is the only preseal source of root geometry.
pub fn issue_stage4_strict_cone_geometry_v1()
-> Result<Stage4StrictConeGeometryV1, Stage4SemanticParsimonyV1Error> {
    let stem = common_stem()?;
    let mut library: Library = Vec::new();
    for (_, telescope) in &stem {
        library.push(LibraryEntry::from_telescope(telescope, &library));
    }
    let admissibility = strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::Guarded);
    let context = EnumerationContext::from_admissibility(&library, admissibility);
    let mut raw = Vec::new();
    for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        raw.extend(enumerate_telescopes(&library, context, kappa));
    }
    let raw_enumerated_count = raw.len();
    let strict = raw
        .into_iter()
        .filter(|telescope| passes_strict_admissibility(4, &library, telescope, admissibility))
        .collect::<Vec<_>>();
    let strict_admitted_count = strict.len();
    let mut seen = BTreeSet::new();
    let mut roots = strict
        .into_iter()
        .filter_map(|telescope| {
            let canonical_key = canonical_key_telescope(&telescope).0;
            seen.insert(canonical_key.clone()).then(|| {
                let kappa = u16::try_from(telescope.kappa()).expect("admissible kappa fits u16");
                Stage4StrictConeRootGeometryV1 {
                    candidate_hash: candidate_hash(&telescope),
                    canonical_key,
                    telescope,
                    kappa,
                }
            })
        })
        .collect::<Vec<_>>();
    roots.sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));
    let least_kappa = roots.iter().map(|root| root.kappa).min().ok_or_else(|| {
        Stage4SemanticParsimonyV1Error::Prerequisite("live Stage-4 strict cone is empty".to_owned())
    })?;
    let exact_complete_strict_cone = roots.iter().all(|root| {
        passes_strict_admissibility(4, &library, &root.telescope, admissibility)
            && candidate_hash(&root.telescope) == root.candidate_hash
            && canonical_key_telescope(&root.telescope).0 == root.canonical_key
    });
    let exact_four_distinct_least_kappa_three_roots = roots.len() == 4
        && roots
            .iter()
            .map(|root| root.candidate_hash.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            == 4
        && least_kappa == 3
        && roots.iter().all(|root| root.kappa == least_kappa);
    if !exact_complete_strict_cone || !exact_four_distinct_least_kappa_three_roots {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "live Stage-4 strict cone is not the exact four-root least-kappa-3 surface (raw={raw_enumerated_count}, strict={strict_admitted_count}, deduped={})",
            roots.len()
        )));
    }
    let mut geometry = Stage4StrictConeGeometryV1 {
        stage: 4,
        window_depth: 2,
        enumerated_kappa_min: admissibility.min_clause_kappa,
        enumerated_kappa_max: admissibility.max_clause_kappa,
        raw_enumerated_count,
        strict_admitted_count,
        canonical_deduped_count: roots.len(),
        least_kappa,
        roots,
        exact_complete_strict_cone,
        exact_four_distinct_least_kappa_three_roots,
        derivation_hash: String::new(),
    };
    geometry.derivation_hash = strict_cone_geometry_hash(&geometry);
    Ok(geometry)
}

pub fn replay_stage4_strict_cone_geometry_v1(claimed: &Stage4StrictConeGeometryV1) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != strict_cone_geometry_hash(claimed) {
        errors.push("Stage-4 strict-cone geometry digest mismatch".to_owned());
    }
    match issue_stage4_strict_cone_geometry_v1() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => {
            errors.push("Stage-4 strict-cone geometry differs from live reissuance".to_owned())
        }
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

fn r_t2_pairwise_row_hash(row: &Rt2V2PairwiseSchemeSetComparison) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    let bytes = serde_json::to_vec(&(
        R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA,
        "pairwise-complete-stage5-scheme-sets",
        projection,
    ))
    .expect("R-T2 pairwise projection serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn unordered_pair(left: &str, right: &str) -> Vec<String> {
    let mut pair = vec![left.to_owned(), right.to_owned()];
    pair.sort();
    pair
}

fn common_stem() -> Result<Vec<(u32, Telescope)>, Stage4SemanticParsimonyV1Error> {
    Ok((1..=3)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect())
}

fn root_audit(
    telescope: &Telescope,
    stem: &[(u32, Telescope)],
) -> Result<Stage4SemanticRootAuditV1, Stage4SemanticParsimonyV1Error> {
    let mut entries = stem.to_vec();
    entries.push((4, telescope.clone()));
    let prefix_signature_digest = SealedSignature::from_telescopes(entries.clone())
        .digest()
        .to_owned();
    let sequence = issue_act_local_semantic_sequence_v5(&entries)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Invariant(error.to_string()))?;
    let b3 = issue_t_bi_intrinsic_isolation_v3(&entries, &sequence)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Invariant(error.to_string()))?;
    let b3_replay_errors = replay_t_bi_intrinsic_isolation_v3(&entries, &sequence, &b3);
    if !b3_replay_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(format!(
            "B3 v3 replay failed: {}",
            b3_replay_errors.join("; ")
        )));
    }
    let package = sequence.packages.last().ok_or_else(|| {
        Stage4SemanticParsimonyV1Error::Invariant("Stage-4 package is absent".to_owned())
    })?;
    let legacy_v5_self_reported_forbidden_or_future_input_used =
        sequence.packages.iter().any(|row| {
            row.archive_read
                || row.structural_nu_read
                || row.bar_read
                || row.verdict_read
                || row.enacted_future_read
        });
    let authoritative_semantic_nu_vector = b3.authoritative_semantic_nu_vector.clone();
    let legacy_semantic_nu_vector = sequence
        .packages
        .iter()
        .map(|row| row.semantic_family_nu)
        .collect::<Vec<_>>();
    let authoritative_nu_matches_legacy_v5_comparison =
        authoritative_semantic_nu_vector == legacy_semantic_nu_vector;
    let stage4_semantic_nu = authoritative_semantic_nu_vector
        .last()
        .copied()
        .ok_or_else(|| {
            Stage4SemanticParsimonyV1Error::Invariant(
                "B3 v3 registry-erased semantic vector is empty".to_owned(),
            )
        })?;
    let root_semantic_audit_proved = b3.prefix_generic_transitive_isolation_proved
        && b3.registry_extension_invariance_proved
        && b3.v3_is_sole_prefix_generic_theorem_authority
        && !b3.embedded_v2_exact_fifteen_theorem_authority_claimed
        && !b3.old_v4_v5_full_hashes_authoritative_for_prefix_theorem
        && !b3
            .authoritative_registry_erased_prefix_semantic_seal
            .is_empty()
        && b3.no_archive_structural_bar_verdict_or_future_input;
    if !root_semantic_audit_proved {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(format!(
            "root {} did not close B1/B2/B3 without residuals",
            package.candidate_hash
        )));
    }
    let kappa = u16::try_from(telescope.kappa()).map_err(|_| {
        Stage4SemanticParsimonyV1Error::Invariant("Stage-4 kappa exceeds u16".to_owned())
    })?;
    let mut root = Stage4SemanticRootAuditV1 {
        candidate_hash: candidate_hash(telescope),
        geometry_telescope_hash: tagged_hash("certified-root-geometry", telescope),
        kappa,
        prefix_signature_digest,
        non_authoritative_legacy_v5_sequence_derivation_hash: sequence.derivation_hash.clone(),
        non_authoritative_legacy_v5_sequence_seal: sequence.intrinsic_sequence_seal.clone(),
        non_authoritative_legacy_v5_package_derivation_hashes: sequence
            .exact_package_derivation_hashes
            .clone(),
        authoritative_registry_erased_v3_semantic_seal: b3
            .authoritative_registry_erased_prefix_semantic_seal
            .clone(),
        prefix_semantic_nu_vector: authoritative_semantic_nu_vector,
        stage4_semantic_nu,
        non_authoritative_legacy_v5_stage4_package_hash: package.derivation_hash.clone(),
        legacy_v5_t_bi_b1_comparison: package.t_bi_b1_proved,
        legacy_v5_t_bi_b2_comparison: package.t_bi_b2_proved,
        legacy_v5_named_role_residual_comparison: package.named_role_residual_count,
        legacy_v5_named_quotient_residual_comparison: package.named_quotient_residual_count,
        legacy_v5_named_a3_residual_comparison: package.named_a3_residual_count,
        legacy_v5_silent_residue_comparison: package.silent_residue_count,
        authoritative_nu_matches_legacy_v5_comparison,
        b3_v3_derivation_hash: b3.derivation_hash,
        b3_v3_prefix_generic_isolation_proved: b3.prefix_generic_transitive_isolation_proved,
        b3_v3_registry_extension_invariance_proved: b3.registry_extension_invariance_proved,
        b3_v3_is_sole_prefix_generic_theorem_authority: b3
            .v3_is_sole_prefix_generic_theorem_authority,
        b3_v3_legacy_full_hashes_authoritative: b3
            .old_v4_v5_full_hashes_authoritative_for_prefix_theorem,
        b3_v3_no_forbidden_or_future_semantic_input: b3
            .no_archive_structural_bar_verdict_or_future_input,
        legacy_v5_hashes_used_as_semantic_selector: false,
        legacy_v5_self_reported_forbidden_or_future_input_used,
        root_semantic_audit_proved,
        derivation_hash: String::new(),
    };
    root.derivation_hash = root_hash(&root);
    Ok(root)
}

fn capability_rows() -> Vec<Stage4SemanticCapabilityRowV1> {
    use Stage4SemanticOperationV1 as O;
    let operations = [
        O::ReadSealedBi0OpeningProjection,
        O::BindAdoptedSemanticRegisterAndTieProtocol,
        O::IssueLiveStrictConeGeometry,
        O::BindCommonStage1Through3Prefix,
        O::ReissueActLocalV5,
        O::ProvePrefixGenericB3,
        O::SealSemanticParsimony,
        O::ReplayFullBi0Postseal,
        O::ReadRt3BranchBoundaryPostseal,
        O::ReadStructuralTestimonyPostseal,
        O::JoinFrozenRt2PairPostseal,
        O::ReadEnactedRootDigestPostseal,
        O::SealAudit,
    ];
    let forbidden = Stage4SemanticCapabilityV1::forbidden();
    let mut accumulated = BTreeSet::new();
    operations
        .into_iter()
        .enumerate()
        .map(|(index, operation)| {
            let direct_capabilities = operation.direct_capabilities();
            accumulated.extend(direct_capabilities.iter().copied());
            let forbidden_capabilities = accumulated
                .intersection(&forbidden)
                .copied()
                .collect::<Vec<_>>();
            let mut row = Stage4SemanticCapabilityRowV1 {
                ordinal: u8::try_from(index + 1).expect("thirteen operation rows"),
                operation,
                direct_capabilities,
                accumulated_capabilities: accumulated.iter().copied().collect(),
                isolated: forbidden_capabilities.is_empty(),
                forbidden_capabilities,
                derivation_hash: String::new(),
            };
            row.derivation_hash = capability_row_hash(&row);
            row
        })
        .collect()
}

/// Run the narrow theorem issuer.  It returns data only; no branch is
/// selected or executed and the issuer has no filesystem parameter.
pub fn issue_stage4_semantic_parsimony_v1()
-> Result<Stage4SemanticParsimonyV1Certificate, Stage4SemanticParsimonyV1Error> {
    let bi0_opening: Bi0OpeningCapabilityProjectionV1 =
        serde_json::from_slice(BI0_V5_ARTIFACT_BYTES)
            .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    let preseal_bi0_opening_capability_valid = bi0_opening.schema
        == BI0_SEMANTIC_REGISTER_V5_SCHEMA
        && bi0_opening.bi0_attempt_executed
        && bi0_opening.bi0_passed
        && !bi0_opening.bi1_invoked
        && !bi0_opening.non_enacted_cone_invoked
        && !bi0_opening.non_enacted_branch_work_executed;
    if !preseal_bi0_opening_capability_valid {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "sealed BI-0 opening projection is not passed/no-branch".to_owned(),
        ));
    }
    let preseal_bi0_opening_projection_hash =
        tagged_hash("preseal-bi0-opening-capability", &bi0_opening);
    let nu_adjudication = std::str::from_utf8(NU_REGISTER_ADJUDICATION_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Prerequisite(error.to_string()))?;
    let tie_protocol = std::str::from_utf8(TIE_PROTOCOL_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Prerequisite(error.to_string()))?;
    let preseal_nu_register_and_tie_authority_replayed = nu_adjudication
        .contains("Status:** **ADOPTED")
        && nu_adjudication.contains("F-AL1")
        && tie_protocol.contains("Status:** **ADOPTED")
        && tie_protocol.contains("R-T1")
        && tie_protocol.contains("R-T2");
    if !preseal_nu_register_and_tie_authority_replayed {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "preseal nu-register or R-T1/R-T2 tie-protocol authority is absent".to_owned(),
        ));
    }
    let stem = common_stem()?;
    let common_prefix_signature_digest = SealedSignature::from_telescopes(stem.clone())
        .digest()
        .to_owned();
    let geometry = issue_stage4_strict_cone_geometry_v1()?;
    let geometry_replay_errors = replay_stage4_strict_cone_geometry_v1(&geometry);
    if !geometry_replay_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "live Stage-4 strict-cone geometry replay failed: {}",
            geometry_replay_errors.join("; ")
        )));
    }
    let preseal_geometry_projection_hash = geometry.derivation_hash.clone();

    // The live strict-cone issuer has no structural-nu, R-T2 outcome,
    // Stage-5, pairwise, or selector input.  Only its exact exhaustive
    // four-root geometry reaches the semantic minimizer below.
    let mut roots = geometry
        .roots
        .iter()
        .map(|root| root_audit(&root.telescope, &stem))
        .collect::<Result<Vec<_>, _>>()?;
    roots.sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));
    if roots.len() != 4
        || roots
            .iter()
            .map(|root| root.candidate_hash.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            != 4
    {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(
            "certified geometry did not produce exactly four distinct semantic roots".to_owned(),
        ));
    }
    let shared_stage1_through3_semantic_nu = roots[0].prefix_semantic_nu_vector[..3].to_vec();
    let exact_shared_semantic_prefix_across_all_roots = roots.iter().all(|root| {
        root.prefix_semantic_nu_vector.len() == 4
            && root.prefix_semantic_nu_vector[..3] == shared_stage1_through3_semantic_nu
    });
    let exact_shared_semantic_prefix_is_1_0_1 = shared_stage1_through3_semantic_nu == [1, 0, 1];
    if !exact_shared_semantic_prefix_across_all_roots || !exact_shared_semantic_prefix_is_1_0_1 {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(format!(
            "the four roots do not derive the sealed BI-0 common semantic prefix [1,0,1]: {:?}",
            shared_stage1_through3_semantic_nu
        )));
    }
    let minimum_pair = roots
        .iter()
        .map(|root| (root.kappa, root.stage4_semantic_nu))
        .min()
        .ok_or_else(|| Stage4SemanticParsimonyV1Error::Invariant("empty root cone".to_owned()))?;
    let semantic_minimizer_hashes = roots
        .iter()
        .filter(|root| (root.kappa, root.stage4_semantic_nu) == minimum_pair)
        .map(|root| root.candidate_hash.clone())
        .collect::<Vec<_>>();
    let pretestimony_semantic_selection_roots = roots
        .iter()
        .map(semantic_selection_root)
        .collect::<Vec<_>>();
    let semantic_seal_excludes_legacy_v5_commitments =
        serde_json::to_string(&pretestimony_semantic_selection_roots)
            .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?
            .find("legacy_v5")
            .is_none();
    if !semantic_seal_excludes_legacy_v5_commitments {
        return Err(Stage4SemanticParsimonyV1Error::Invariant(
            "pre-testimony semantic selection projection contains a legacy-v5 commitment"
                .to_owned(),
        ));
    }
    let semantic_parsimony_seal = semantic_parsimony_seal(
        &pretestimony_semantic_selection_roots,
        minimum_pair,
        &semantic_minimizer_hashes,
    );

    // Everything below is post-seal testimony: useful for stating the
    // divergence, incapable of affecting the semantic minimizer above.
    let bi0: Bi0SemanticRegisterV5Certificate = serde_json::from_slice(BI0_V5_ARTIFACT_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    let bi0_replay = replay_bi0_semantic_register_v5_certificate(&bi0);
    if bi0.schema != BI0_SEMANTIC_REGISTER_V5_SCHEMA
        || !bi0_replay.valid
        || !bi0_replay.bi0_passed
        || bi0_replay.bi1_invoked
        || bi0_replay.non_enacted_cone_invoked
        || bi0.non_enacted_branch_work_executed
    {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "postseal full BI-0 v5 replay failed its passed/no-branch gate: {:?}",
            bi0_replay.errors
        )));
    }
    let r_t3_adjudication = std::str::from_utf8(R_T3_ADJUDICATION_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Prerequisite(error.to_string()))?;
    let postseal_r_t3_branch_boundary_replayed = r_t3_adjudication.contains("Option B adopted")
        && r_t3_adjudication.contains("F-R3-B1")
        && r_t3_adjudication.contains("F-R3-B2");
    if !postseal_r_t3_branch_boundary_replayed {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "postseal R-T3 branch-boundary adoption markers are absent".to_owned(),
        ));
    }
    let r_t2: Rt2FutureHoleConfluenceV2Certificate = serde_json::from_slice(R_T2_ARTIFACT_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    let r_t2_projection_errors = replay_archived_stage4_fork_projection(&r_t2);
    if !r_t2_projection_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "postseal frozen R-T2 full projection failed: {}",
            r_t2_projection_errors.join("; ")
        )));
    }
    let archived_candidate_hashes = r_t2
        .branches
        .iter()
        .map(|branch| branch.stage4_candidate_hash.as_str())
        .collect::<BTreeSet<_>>();
    let live_candidate_hashes = roots
        .iter()
        .map(|root| root.candidate_hash.as_str())
        .collect::<BTreeSet<_>>();
    let archived_class_ids = r_t2
        .branches
        .iter()
        .map(|branch| branch.r_t1_class_id.as_str())
        .collect::<BTreeSet<_>>();
    let postseal_archived_exact_four_way_r_t1_class_join =
        r_t2.exact_four_way_r_t1_class_join && archived_class_ids.len() == 4;
    let postseal_live_root_hashes_join_archived_r_t2_geometry = archived_candidate_hashes
        == live_candidate_hashes
        && r_t2.branches.iter().all(|branch| {
            roots.iter().any(|root| {
                root.candidate_hash == branch.stage4_candidate_hash
                    && candidate_hash(&branch.stage4_telescope) == root.candidate_hash
                    && branch.stage4_telescope
                        == geometry
                            .roots
                            .iter()
                            .find(|geometry_root| {
                                geometry_root.candidate_hash == root.candidate_hash
                            })
                            .expect("live root is in live geometry")
                            .telescope
                    && branch.prefix_steps == vec![1, 2, 3, 4]
                    && branch.prefix_steps_exactly_one_through_four
                    && branch.prefix_signature_digest == root.prefix_signature_digest
            })
        });
    if !postseal_archived_exact_four_way_r_t1_class_join
        || !postseal_live_root_hashes_join_archived_r_t2_geometry
    {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "postseal archived R-T2 geometry does not join the live exhaustive strict cone"
                .to_owned(),
        ));
    }
    let structural = issue_stage4_r_t1_orbit_audit()
        .map_err(|error| Stage4SemanticParsimonyV1Error::Prerequisite(error.to_string()))?;
    let structural_errors = replay_stage4_r_t1_orbit_audit(&structural);
    if !structural_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(format!(
            "structural R-T1 replay failed: {}",
            structural_errors.join("; ")
        )));
    }

    let survivor_r_t2_join = if semantic_minimizer_hashes.len() == 2 {
        let live_hashes = roots
            .iter()
            .map(|root| root.candidate_hash.clone())
            .collect::<Vec<_>>();
        let mut expected_unordered_pairs = BTreeSet::new();
        for left in 0..live_hashes.len() {
            for right in (left + 1)..live_hashes.len() {
                expected_unordered_pairs
                    .insert(unordered_pair(&live_hashes[left], &live_hashes[right]));
            }
        }
        let all_unordered_pairs = r_t2
            .pairwise_scheme_set_comparisons
            .iter()
            .map(|comparison| {
                unordered_pair(
                    &comparison.left_candidate_hash,
                    &comparison.right_candidate_hash,
                )
            })
            .collect::<Vec<_>>();
        let observed_unordered_pairs = all_unordered_pairs.iter().cloned().collect::<BTreeSet<_>>();
        let all_six_pairwise_row_derivation_hashes_valid = r_t2
            .pairwise_scheme_set_comparisons
            .iter()
            .all(|comparison| comparison.derivation_hash == r_t2_pairwise_row_hash(comparison));
        let six_unordered_comparisons_exact = expected_unordered_pairs.len() == 6
            && all_unordered_pairs.len() == 6
            && observed_unordered_pairs == expected_unordered_pairs;
        let matching_rows = r_t2
            .pairwise_scheme_set_comparisons
            .iter()
            .filter(|comparison| {
                unordered_pair(
                    &comparison.left_candidate_hash,
                    &comparison.right_candidate_hash,
                ) == semantic_minimizer_hashes
            })
            .collect::<Vec<_>>();
        if matching_rows.len() != 1 {
            return Err(Stage4SemanticParsimonyV1Error::Invariant(format!(
                "frozen R-T2 has {} rows for the exact semantic survivor pair",
                matching_rows.len()
            )));
        }
        let comparison = matching_rows[0];
        let pairwise_row_derivation_hash_valid =
            comparison.derivation_hash == r_t2_pairwise_row_hash(comparison);
        let exact_five_five_three_inequivalence_signature = comparison.left_complete_scheme_count
            == 5
            && comparison.right_complete_scheme_count == 5
            && comparison.perfect_matching.len() == 3
            && !comparison.full_scheme_sets_equivalent
            && comparison.comparison_well_formed
            && comparison.relative_coverage_and_zero_gap_gate_passed
            && !comparison.hash_or_enumeration_order_used_as_selector;
        if !six_unordered_comparisons_exact
            || !all_six_pairwise_row_derivation_hashes_valid
            || !pairwise_row_derivation_hash_valid
            || !exact_five_five_three_inequivalence_signature
        {
            return Err(Stage4SemanticParsimonyV1Error::Invariant(
                "frozen survivor row failed its exact six-row/hash/5-5-3 inequivalence signature"
                    .to_owned(),
            ));
        }
        let mut join = Stage4Rt2SurvivorJoinV1 {
            unordered_candidate_pair: semantic_minimizer_hashes.clone(),
            left_complete_scheme_count: comparison.left_complete_scheme_count,
            right_complete_scheme_count: comparison.right_complete_scheme_count,
            matched_scheme_count: comparison.perfect_matching.len(),
            full_scheme_sets_equivalent: comparison.full_scheme_sets_equivalent,
            comparison_well_formed: comparison.comparison_well_formed,
            relative_coverage_and_zero_gap_gate_passed: comparison
                .relative_coverage_and_zero_gap_gate_passed,
            hash_or_enumeration_order_used_as_selector: comparison
                .hash_or_enumeration_order_used_as_selector,
            pairwise_row_derivation_hash_valid,
            exactly_one_row_for_surviving_pair: true,
            six_unordered_comparisons_exact,
            all_six_pairwise_row_derivation_hashes_valid,
            exact_five_five_three_inequivalence_signature,
            exact_surviving_pair_join: pairwise_row_derivation_hash_valid
                && all_six_pairwise_row_derivation_hashes_valid
                && six_unordered_comparisons_exact
                && exact_five_five_three_inequivalence_signature,
            frozen_r_t2_comparison_derivation_hash: comparison.derivation_hash.clone(),
            derivation_hash: String::new(),
        };
        join.derivation_hash = survivor_join_hash(&join);
        Some(join)
    } else {
        None
    };

    let burn: Phase5bReselectionBurn = serde_json::from_slice(PHASE5B_BURN_BYTES)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    if burn.schema != PHASE5B_RESELECTION_BURN_SCHEMA || !phase5b_burn_digest_valid(&burn) {
        return Err(Stage4SemanticParsimonyV1Error::Prerequisite(
            "sealed Phase-5b burn digest is invalid".to_owned(),
        ));
    }
    let enacted_stage4 = burn
        .stages
        .iter()
        .find(|stage| stage.stage == 4)
        .and_then(|stage| stage.winner.as_ref())
        .ok_or_else(|| {
            Stage4SemanticParsimonyV1Error::Prerequisite(
                "sealed Phase-5b burn has no enacted Stage-4 root".to_owned(),
            )
        })?;
    let enacted_root = roots
        .iter()
        .find(|root| root.candidate_hash == enacted_stage4.candidate_hash)
        .ok_or_else(|| {
            Stage4SemanticParsimonyV1Error::Invariant(
                "sealed enacted Stage-4 digest is outside the certified four-root geometry"
                    .to_owned(),
            )
        })?;

    let exact_two_minimizers = semantic_minimizer_hashes.len() == 2;
    let surviving_pair_frozen_r_t2_inequivalent = survivor_r_t2_join.as_ref().is_some_and(|join| {
        join.exact_surviving_pair_join && join.exact_five_five_three_inequivalence_signature
    });
    let enacted_root_is_semantic_minimizer = semantic_minimizer_hashes
        .iter()
        .any(|hash| hash == &enacted_root.candidate_hash);
    let enacted_root_nonminimal = !enacted_root_is_semantic_minimizer;
    let enacted_root_hash = enacted_root.candidate_hash.clone();
    let enacted_root_kappa = enacted_root.kappa;
    let enacted_root_semantic_nu = enacted_root.stage4_semantic_nu;
    let exact_two_minimizer_enacted_nonminimal_divergence =
        exact_two_minimizers && surviving_pair_frozen_r_t2_inequivalent && enacted_root_nonminimal;
    let outcome = if exact_two_minimizer_enacted_nonminimal_divergence {
        Stage4SemanticParsimonyOutcomeV1::TwoSemanticMinimizersRt2InequivalentEnactedRootNonminimalAdjudicationRequired
    } else if semantic_minimizer_hashes.len() == 1 {
        Stage4SemanticParsimonyOutcomeV1::UniqueSemanticMinimumRecordedNoBranchExecuted
    } else {
        Stage4SemanticParsimonyOutcomeV1::OtherSemanticConeRecordedAdjudicationRequired
    };
    let divergence_statement = format!(
        "Intrinsic semantic parsimony (least kappa, then least semantic nu) yields {} minimizer(s) {:?} at ({},{}). The sealed enacted root {} reissues at ({},{}), so enacted_nonminimal={}. The frozen R-T2 survivor comparison is inequivalent={}, matched_scheme_count={}. No branch was selected or executed.",
        semantic_minimizer_hashes.len(),
        semantic_minimizer_hashes,
        minimum_pair.0,
        minimum_pair.1,
        enacted_root_hash,
        enacted_root_kappa,
        enacted_root_semantic_nu,
        enacted_root_nonminimal,
        survivor_r_t2_join
            .as_ref()
            .is_some_and(|join| !join.full_scheme_sets_equivalent),
        survivor_r_t2_join
            .as_ref()
            .map_or(0, |join| join.matched_scheme_count),
    );
    let operation_capability_rows = capability_rows();
    let no_branch_continuation_capability = operation_capability_rows
        .iter()
        .all(|row| row.isolated && row.forbidden_capabilities.is_empty());
    let theorem_issuer_has_no_artifact_write_capability =
        operation_capability_rows.iter().all(|row| {
            !row.accumulated_capabilities
                .contains(&Stage4SemanticCapabilityV1::CanonicalArtifactWrite)
        });
    let every_root_has_authoritative_registry_erased_b1_b2_b3_v3 =
        roots.iter().all(|root| root.root_semantic_audit_proved);

    let mut certificate = Stage4SemanticParsimonyV1Certificate {
        schema: STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA.to_owned(),
        date: STAGE4_SEMANTIC_PARSIMONY_V1_DATE.to_owned(),
        theorem_id: STAGE4_SEMANTIC_PARSIMONY_V1_THEOREM_ID.to_owned(),
        source_bindings: vec![
            source_binding(
                "docs/bi0_semantic_register_v5.json",
                "preseal_opening_projection_then_postseal_full_replay",
                BI0_V5_ARTIFACT_BYTES,
            ),
            source_binding(
                "docs/nu_register_adjudication.md",
                "adopted_intrinsic_semantic_register_authority",
                NU_REGISTER_ADJUDICATION_BYTES,
            ),
            source_binding(
                "docs/tie_resolution_protocol.md",
                "preseal_adopted_r_t1_r_t2_parsimony_ladder",
                TIE_PROTOCOL_BYTES,
            ),
            source_binding(
                "docs/r_t3_stage4_adjudication.md",
                "adopted_option_b_branch_invariance_boundary",
                R_T3_ADJUDICATION_BYTES,
            ),
            source_binding(
                "docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json",
                "postseal_archived_geometry_and_exact_pair_comparison_only",
                R_T2_ARTIFACT_BYTES,
            ),
            source_binding(
                "docs/phase5b_reselection_burn_v2.json",
                "postseal_enacted_root_digest_only",
                PHASE5B_BURN_BYTES,
            ),
        ],
        preseal_bi0_opening_projection_hash,
        preseal_bi0_opening_capability_valid,
        full_bi0_read_or_replayed_before_semantic_seal: false,
        sealed_bi0_v5_schema: bi0.schema.clone(),
        sealed_bi0_v5_digest: bi0.result_digest.clone(),
        sealed_bi0_v5_replay_valid: bi0_replay.valid,
        sealed_bi0_v5_passed: bi0.bi0_passed,
        sealed_bi0_v5_executed_no_branch_work: bi0.bi0_attempt_executed
            && !bi0.bi1_invoked
            && !bi0.non_enacted_cone_invoked
            && !bi0.non_enacted_branch_work_executed,
        nu_register_adjudication_hash: bytes_hash(NU_REGISTER_ADJUDICATION_BYTES),
        tie_resolution_protocol_hash: bytes_hash(TIE_PROTOCOL_BYTES),
        r_t3_stage4_adjudication_hash: bytes_hash(R_T3_ADJUDICATION_BYTES),
        preseal_nu_register_and_tie_authority_replayed,
        postseal_r_t3_branch_boundary_replayed,
        preseal_live_strict_cone_geometry: geometry.clone(),
        preseal_geometry_projection_hash,
        preseal_geometry_only_replay_valid: true,
        preseal_exact_exhaustive_four_way_least_kappa_geometry: geometry
            .exact_complete_strict_cone
            && geometry.exact_four_distinct_least_kappa_three_roots,
        frozen_r_t2_certificate_digest: r_t2.result_digest,
        postseal_full_r_t2_projection_replay_valid: true,
        postseal_archived_exact_four_way_r_t1_class_join,
        postseal_live_root_hashes_join_archived_r_t2_geometry,
        live_strict_cone_only_root_geometry_before_semantic_seal: true,
        common_prefix_steps: stem.iter().map(|(stage, _)| *stage).collect(),
        common_prefix_signature_digest,
        root_count: roots.len(),
        every_root_has_authoritative_registry_erased_b1_b2_b3_v3,
        pretestimony_semantic_selection_roots,
        semantic_seal_excludes_legacy_v5_commitments,
        shared_stage1_through3_semantic_nu,
        exact_shared_semantic_prefix_across_all_roots,
        exact_shared_semantic_prefix_is_1_0_1,
        roots,
        semantic_parsimony_order: "lexicographic: least kappa, then least authoritative registry-erased semantic-family nu from B3 v3"
            .to_owned(),
        minimum_kappa: minimum_pair.0,
        minimum_semantic_nu: minimum_pair.1,
        semantic_minimizer_count: semantic_minimizer_hashes.len(),
        semantic_minimizer_hashes,
        semantic_parsimony_seal,
        structural_testimony_read_after_semantic_seal: true,
        structural_testimony_minimum_kappa: structural.minimum_kappa,
        structural_testimony_minimum_nu: structural.minimum_certified_nu,
        structural_testimony_minimizer_count: structural.minimizer_count,
        structural_testimony_used_as_selector: false,
        survivor_r_t2_join,
        enacted_history_burn_digest: burn.result_digest,
        enacted_root_hash,
        enacted_root_kappa,
        enacted_root_semantic_nu,
        enacted_root_identified_only_after_semantic_seal: true,
        enacted_root_is_semantic_minimizer,
        exact_two_minimizers,
        surviving_pair_frozen_r_t2_inequivalent,
        enacted_root_nonminimal,
        exact_two_minimizer_enacted_nonminimal_divergence,
        operation_capability_rows,
        no_branch_continuation_capability,
        no_branch_executed: true,
        theorem_issuer_has_no_artifact_write_capability,
        desired_verdict_count_score_or_bar_used_as_semantic_premise: false,
        outcome,
        divergence_statement,
        required_successor_action: "Versioned adjudication is required before any branch continuation; the theorem issuer authorizes no branch and has no artifact-write capability."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_hash(&certificate);
    Ok(certificate)
}

fn validate_against_expected(
    claimed: &Stage4SemanticParsimonyV1Certificate,
    expected: &Stage4SemanticParsimonyV1Certificate,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_hash(claimed) {
        errors.push("Stage-4 semantic parsimony certificate digest mismatch".to_owned());
    }
    if claimed != expected {
        errors.push("certificate differs from deterministic verdict-blind reissuance".to_owned());
    }
    errors
}

pub fn replay_stage4_semantic_parsimony_v1(
    claimed: &Stage4SemanticParsimonyV1Certificate,
) -> Vec<String> {
    match issue_stage4_semantic_parsimony_v1() {
        Ok(expected) => validate_against_expected(claimed, &expected),
        Err(error) => vec![error.to_string()],
    }
}

pub fn replay_stage4_semantic_parsimony_v1_json(json: &str) -> Vec<String> {
    match serde_json::from_str::<Stage4SemanticParsimonyV1Certificate>(json) {
        Ok(certificate) => replay_stage4_semantic_parsimony_v1(&certificate),
        Err(error) => vec![format!(
            "Stage-4 semantic parsimony JSON did not deserialize: {error}"
        )],
    }
}

/// Create a replay-checked artifact.  This outer serialization helper is
/// intentionally separate from the theorem issuer and uses `create_new`.
pub fn emit_stage4_semantic_parsimony_v1_create_new(
    path: &Path,
) -> Result<Stage4SemanticParsimonyV1Certificate, Stage4SemanticParsimonyV1Error> {
    let certificate = issue_stage4_semantic_parsimony_v1()?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Json(error.to_string()))?;
    let replay_errors = replay_stage4_semantic_parsimony_v1_json(&json);
    if !replay_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV1Error::EmittedReplay(
            replay_errors.join("; "),
        ));
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Stage4SemanticParsimonyV1Error::Io(error.to_string()))?;
    output
        .write_all(json.as_bytes())
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Stage4SemanticParsimonyV1Error::Io(error.to_string()))?;
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn cached_certificate() -> Stage4SemanticParsimonyV1Certificate {
        static CERTIFICATE: OnceLock<Stage4SemanticParsimonyV1Certificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_stage4_semantic_parsimony_v1().expect("semantic audit"))
            .clone()
    }

    #[test]
    fn live_strict_cone_geometry_is_exact_and_contains_no_structural_nu() {
        let geometry = issue_stage4_strict_cone_geometry_v1().expect("strict cone");
        assert_eq!(geometry.roots.len(), 4);
        assert_eq!(geometry.least_kappa, 3);
        assert!(geometry.exact_complete_strict_cone);
        assert!(geometry.exact_four_distinct_least_kappa_three_roots);
        assert!(
            !serde_json::to_string(&geometry)
                .expect("geometry serializes")
                .contains("structural_nu")
        );
        assert!(replay_stage4_strict_cone_geometry_v1(&geometry).is_empty());
    }

    #[test]
    fn preseal_bi0_projection_cannot_observe_full_register_fields() {
        let opening: Bi0OpeningCapabilityProjectionV1 =
            serde_json::from_slice(BI0_V5_ARTIFACT_BYTES).expect("opening projection");
        let serialized = serde_json::to_value(&opening).expect("projection serializes");
        assert_eq!(serialized.as_object().expect("projection object").len(), 6);
        let opening_hash = tagged_hash("preseal-bi0-opening-capability", &opening);

        let mut full: serde_json::Value =
            serde_json::from_slice(BI0_V5_ARTIFACT_BYTES).expect("full BI0 JSON");
        full.as_object_mut().expect("full BI0 object").insert(
            "t_bi_result_digest".to_owned(),
            serde_json::Value::String("blake3:postseal-only-mutation".to_owned()),
        );
        let mutated_bytes = serde_json::to_vec(&full).expect("mutated full BI0 serializes");
        let mutated_opening: Bi0OpeningCapabilityProjectionV1 =
            serde_json::from_slice(&mutated_bytes).expect("opening ignores full fields");
        assert_eq!(mutated_opening, opening);
        assert_eq!(
            tagged_hash("preseal-bi0-opening-capability", &mutated_opening),
            opening_hash
        );
        assert_ne!(
            bytes_hash(&mutated_bytes),
            bytes_hash(BI0_V5_ARTIFACT_BYTES)
        );
    }

    #[test]
    fn blind_semantic_audit_publishes_exact_stage4_divergence_without_execution() {
        let certificate = cached_certificate();
        assert_eq!(certificate.root_count, 4);
        assert_eq!(certificate.minimum_kappa, 3);
        assert_eq!(certificate.minimum_semantic_nu, 2);
        assert_eq!(certificate.semantic_minimizer_count, 2);
        assert_eq!(certificate.structural_testimony_minimum_kappa, 3);
        assert_eq!(certificate.structural_testimony_minimum_nu, 5);
        assert_eq!(certificate.structural_testimony_minimizer_count, 4);
        assert!(certificate.exact_two_minimizer_enacted_nonminimal_divergence);
        assert!(certificate.surviving_pair_frozen_r_t2_inequivalent);
        assert!(certificate.enacted_root_nonminimal);
        assert!(certificate.preseal_bi0_opening_capability_valid);
        assert!(!certificate.full_bi0_read_or_replayed_before_semantic_seal);
        assert!(certificate.sealed_bi0_v5_replay_valid);
        assert!(certificate.sealed_bi0_v5_passed);
        assert!(certificate.sealed_bi0_v5_executed_no_branch_work);
        assert!(certificate.preseal_geometry_only_replay_valid);
        assert!(certificate.preseal_exact_exhaustive_four_way_least_kappa_geometry);
        assert!(certificate.postseal_full_r_t2_projection_replay_valid);
        assert!(certificate.postseal_archived_exact_four_way_r_t1_class_join);
        assert!(certificate.postseal_live_root_hashes_join_archived_r_t2_geometry);
        assert!(certificate.every_root_has_authoritative_registry_erased_b1_b2_b3_v3);
        assert!(certificate.semantic_seal_excludes_legacy_v5_commitments);
        assert_eq!(certificate.pretestimony_semantic_selection_roots.len(), 4);
        assert!(certificate.roots.iter().all(|root| {
            root.authoritative_nu_matches_legacy_v5_comparison
                && root.b3_v3_prefix_generic_isolation_proved
                && root.b3_v3_registry_extension_invariance_proved
                && root.b3_v3_is_sole_prefix_generic_theorem_authority
                && !root.b3_v3_legacy_full_hashes_authoritative
                && !root.legacy_v5_hashes_used_as_semantic_selector
                && !root.legacy_v5_self_reported_forbidden_or_future_input_used
        }));
        assert_eq!(
            certificate.shared_stage1_through3_semantic_nu,
            vec![1, 0, 1]
        );
        assert!(certificate.exact_shared_semantic_prefix_across_all_roots);
        assert!(certificate.exact_shared_semantic_prefix_is_1_0_1);
        let join = certificate
            .survivor_r_t2_join
            .as_ref()
            .expect("exact survivor join");
        assert_eq!(join.left_complete_scheme_count, 5);
        assert_eq!(join.right_complete_scheme_count, 5);
        assert_eq!(join.matched_scheme_count, 3);
        assert!(join.pairwise_row_derivation_hash_valid);
        assert!(join.exactly_one_row_for_surviving_pair);
        assert!(join.six_unordered_comparisons_exact);
        assert!(join.all_six_pairwise_row_derivation_hashes_valid);
        assert!(join.exact_five_five_three_inequivalence_signature);
        assert!(certificate.no_branch_continuation_capability);
        assert!(certificate.no_branch_executed);
        assert!(certificate.theorem_issuer_has_no_artifact_write_capability);
        assert_eq!(
            certificate.outcome,
            Stage4SemanticParsimonyOutcomeV1::TwoSemanticMinimizersRt2InequivalentEnactedRootNonminimalAdjudicationRequired
        );

        let mut testimony_only_mutation = certificate.roots.clone();
        testimony_only_mutation[0].non_authoritative_legacy_v5_sequence_derivation_hash =
            "blake3:non-authoritative-testimony-mutation".to_owned();
        testimony_only_mutation[0].authoritative_nu_matches_legacy_v5_comparison = false;
        testimony_only_mutation[0].derivation_hash = root_hash(&testimony_only_mutation[0]);
        let selection_projection = testimony_only_mutation
            .iter()
            .map(semantic_selection_root)
            .collect::<Vec<_>>();
        assert_eq!(
            semantic_parsimony_seal(
                &selection_projection,
                (certificate.minimum_kappa, certificate.minimum_semantic_nu),
                &certificate.semantic_minimizer_hashes,
            ),
            certificate.semantic_parsimony_seal
        );
    }

    #[test]
    fn fully_rehashed_mutations_cannot_rewrite_the_semantic_result() {
        let expected = cached_certificate();
        let mut forgeries = Vec::new();

        let mut forged = expected.clone();
        forged.minimum_semantic_nu += 1;
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.semantic_minimizer_hashes = vec![expected.enacted_root_hash.clone()];
        forged.semantic_minimizer_count = 1;
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.enacted_root_nonminimal = false;
        forged.enacted_root_is_semantic_minimizer = true;
        forgeries.push(forged);

        let mut forged = expected.clone();
        let join = forged
            .survivor_r_t2_join
            .as_mut()
            .expect("two-survivor join");
        join.full_scheme_sets_equivalent = true;
        join.derivation_hash = survivor_join_hash(join);
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.operation_capability_rows[0]
            .forbidden_capabilities
            .push(Stage4SemanticCapabilityV1::BranchContinuation);
        forged.operation_capability_rows[0].isolated = false;
        forged.operation_capability_rows[0].derivation_hash =
            capability_row_hash(&forged.operation_capability_rows[0]);
        forgeries.push(forged);

        let mut forged = expected.clone();
        forged.preseal_live_strict_cone_geometry.roots[0].kappa = 4;
        forged.preseal_live_strict_cone_geometry.derivation_hash =
            strict_cone_geometry_hash(&forged.preseal_live_strict_cone_geometry);
        forged.preseal_geometry_projection_hash = forged
            .preseal_live_strict_cone_geometry
            .derivation_hash
            .clone();
        forgeries.push(forged);

        for forged in &mut forgeries {
            forged.result_digest = certificate_hash(forged);
            let errors = validate_against_expected(forged, &expected);
            assert!(errors.iter().any(|error| error.contains("reissuance")));
            assert!(!errors.iter().any(|error| error.contains("digest mismatch")));
        }
    }
}
