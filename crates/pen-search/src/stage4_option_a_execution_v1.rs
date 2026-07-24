//! Post-adjudication execution capability for the four-root Stage-4 cone.
//!
//! This module is deliberately narrower than a branch runner.  It replays the
//! exact cross-bound Stage-4 v3 certificate, pins the adopted Option-A block,
//! and binds all four roots to corroborating R-T1/R-T2 testimony.  Its sole
//! positive effect is permission to execute every certified root as part of a
//! complete BI-1 sweep.  It has no selector, branch implementation, UC-1
//! scorer, bridge, or artifact writer.

use crate::r_t2_future_hole_confluence_v2::{
    R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA, Rt2FutureHoleConfluenceV2Certificate,
    Rt2FutureHoleConfluenceV2Outcome, replay_archived_stage4_fork_projection,
};
use crate::stage4_semantic_parsimony_v1::{
    STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA, Stage4StructuralRt1JoinV1,
};
use crate::stage4_semantic_parsimony_v3::{
    STAGE4_SEMANTIC_PARSIMONY_V3_SCHEMA, Stage4SemanticParsimonyV3Certificate,
    replay_stage4_semantic_parsimony_v3,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const STAGE4_OPTION_A_EXECUTION_V1_SCHEMA: &str =
    "stage4-option-a-all-four-bi1-execution-capability-v1";
pub const STAGE4_OPTION_A_EXECUTION_V1_DATE: &str = "2026-07-23";
pub const STAGE4_OPTION_A_EXECUTION_V1_THEOREM_ID: &str =
    "T-S4-Option-A-cross-bound-four-root-execution-authority";

const V3_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/stage4_semantic_parsimony_v3.json");
const OPTION_A_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/stage4_semantic_divergence_adjudication.md");
const R_T2_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json");

const EXPECTED_OPTION_A_ADOPTION_BLOCK: &str = r#"## ADOPTION

**Option A adopted — value never legislates at guarded stages.**
Recorded verbatim from the user's adoption message of 2026-07-22
(replay pins this text):

> I adopt option A. Rationale: Real choice exists, even at the most
> fundamental level.
>
> — Halvor Lande, 22 July 2026

**Grounds audit (F-S4-1).** The stated ground is an ontological
commitment to real contingency: the law may not eliminate lawful
alternatives by economy. It cites no desire to keep the enacted world
law-selected (the ground would hold identically had a minimal act been
enacted), no sunk cost, no UC-1 outcome, no downstream result.
F-S4-1 is satisfied. Options B and C are not adopted and are retained
above for the record.

**Precision recorded at the adopter's request.** Option A secures
choice against *value*, not against *proof*: the law never prunes
lawful alternatives by cost, but a certified equivalence (e.g. a
proven UC-1b) may still show apparent alternatives to be one act —
in which case no choice is destroyed, because none existed. The
adopted ground in its exact form: *real choice exists wherever
genuinely distinct lawful acts exist, and the law will never eliminate
it by economy.* This commits to non-legislation of value, not to any
cone cardinality.

**Now in force.** Semantic ν joins the bar in the diagnostic register
at guarded stages (F-S4-3 armed: treating enacted nonminimality as a
defect, or reintroducing value selection at guarded stages, is
invalid). The lawful cone is the admissibility cone: four branches,
enacted branch indexical. The death of `demand-parsimony-law-v1`
(F-BF2, lawful register) stands recorded. BI-1 sweeps four branches;
BI-4, UC-1 scoring, and successor selective-law and bridge documents
quantify over the 4-cone.

Amendment goes through a versioned successor; no silent modification.
"#;

const EXPECTED_ROOT_SEMANTIC_NU: [(&str, u32); 4] = [
    (
        "blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407",
        3,
    ),
    (
        "blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308",
        2,
    ),
    (
        "blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b",
        2,
    ),
    (
        "blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4",
        3,
    ),
];

const FORBIDDEN_CAPABILITIES: [&str; 8] = [
    "select_root_by_semantic_nu",
    "select_root_by_bar",
    "select_or_privilege_enacted_root",
    "prune_nonminimal_root",
    "treat_partial_sweep_as_complete",
    "score_UC1",
    "execute_bridge",
    "write_artifact",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4OptionAStemEntryV1 {
    stage: u32,
    telescope: Telescope,
    candidate_hash: String,
    predecessor_signature_digest: String,
    derivation_hash: String,
}

impl Stage4OptionAStemEntryV1 {
    pub fn stage(&self) -> u32 {
        self.stage
    }

    pub fn telescope(&self) -> &Telescope {
        &self.telescope
    }

    pub fn candidate_hash(&self) -> &str {
        &self.candidate_hash
    }

    pub fn predecessor_signature_digest(&self) -> &str {
        &self.predecessor_signature_digest
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4OptionAExecutionRootV1 {
    candidate_hash: String,
    telescope: Telescope,
    prefix_signature_digest: String,
    kappa: u16,
    semantic_nu: u32,
    v3_semantic_root_derivation_hash: String,
    v3_root_extension_derivation_hash: String,
    v3_selection_join_derivation_hash: String,
    r_t1_class_id: String,
    r_t1_class_derivation_hash: String,
    r_t1_package_derivation_hash: String,
    r_t2_economy_geometry_key: String,
    r_t2_economy_class_size: usize,
    exact_v3_root_binding: bool,
    exact_r_t1_class_testimony: bool,
    exact_r_t2_economy_testimony: bool,
    execution_authorized: bool,
    derivation_hash: String,
}

impl Stage4OptionAExecutionRootV1 {
    pub fn candidate_hash(&self) -> &str {
        &self.candidate_hash
    }

    pub fn telescope(&self) -> &Telescope {
        &self.telescope
    }

    pub fn prefix_signature_digest(&self) -> &str {
        &self.prefix_signature_digest
    }

    pub fn kappa(&self) -> u16 {
        self.kappa
    }

    pub fn semantic_nu(&self) -> u32 {
        self.semantic_nu
    }

    pub fn r_t1_class_id(&self) -> &str {
        &self.r_t1_class_id
    }

    pub fn r_t2_economy_geometry_key(&self) -> &str {
        &self.r_t2_economy_geometry_key
    }

    pub fn r_t2_economy_class_size(&self) -> usize {
        self.r_t2_economy_class_size
    }

    pub fn execution_authorized(&self) -> bool {
        self.execution_authorized
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4OptionAExecutionCapabilityV1 {
    schema: String,
    date: String,
    theorem_id: String,
    source_v3_artifact_bytes_hash: String,
    source_v3_result_digest: String,
    source_option_a_adjudication_bytes_hash: String,
    source_r_t1_audit_derivation_hash: String,
    source_r_t2_artifact_bytes_hash: String,
    source_r_t2_result_digest: String,
    v3_fully_replayed_before_authorization: bool,
    option_a_adoption_block_exact: bool,
    v3_cross_binding_exact: bool,
    v3_cone_audit_authorized: bool,
    v3_pre_adjudication_branch_authority_was_absent: bool,
    common_stem: Vec<Stage4OptionAStemEntryV1>,
    common_stem_signature_digest: String,
    roots: Vec<Stage4OptionAExecutionRootV1>,
    authorized_root_hashes: Vec<String>,
    enacted_root_hash: String,
    enacted_root_is_indexical_testimony_only: bool,
    exact_four_root_surface: bool,
    exact_per_root_semantic_register: bool,
    exact_all_kappa_three: bool,
    r_t1_class_ids_derived_as_testimony_only: bool,
    r_t2_economy_geometry_derived_as_testimony_only: bool,
    r_t1_or_r_t2_value_used_as_authorization_input: bool,
    semantic_nu_diagnostic_only_at_guarded_stages: bool,
    bar_diagnostic_only_at_guarded_stages: bool,
    f_s4_3_armed: bool,
    all_four_execution_authorized: bool,
    complete_sweep_required_for_cone_verdict: bool,
    branch_selector_capability_present: bool,
    selected_root_hash: Option<String>,
    enacted_root_privilege_present: bool,
    ordering_used_only_for_serialization: bool,
    forbidden_capabilities: Vec<String>,
    artifact_write_capability_present: bool,
    permitted_conclusion: String,
    derivation_hash: String,
}

impl Stage4OptionAExecutionCapabilityV1 {
    pub fn schema(&self) -> &str {
        &self.schema
    }

    pub fn source_v3_result_digest(&self) -> &str {
        &self.source_v3_result_digest
    }

    pub fn common_stem(&self) -> &[Stage4OptionAStemEntryV1] {
        &self.common_stem
    }

    pub fn common_stem_signature_digest(&self) -> &str {
        &self.common_stem_signature_digest
    }

    pub fn roots(&self) -> &[Stage4OptionAExecutionRootV1] {
        &self.roots
    }

    pub fn root_by_hash(&self, candidate_hash: &str) -> Option<&Stage4OptionAExecutionRootV1> {
        self.roots
            .iter()
            .find(|root| root.candidate_hash == candidate_hash)
    }

    pub fn authorized_root_hashes(&self) -> &[String] {
        &self.authorized_root_hashes
    }

    /// Indexical testimony used only to schedule the enacted-first BI
    /// regression.  It grants no selection or comparative privilege.
    pub fn enacted_indexical_root_hash(&self) -> &str {
        &self.enacted_root_hash
    }

    pub fn enacted_root_is_indexical_testimony_only(&self) -> bool {
        self.enacted_root_is_indexical_testimony_only
    }

    pub fn all_four_execution_authorized(&self) -> bool {
        self.all_four_execution_authorized
    }

    pub fn complete_sweep_required_for_cone_verdict(&self) -> bool {
        self.complete_sweep_required_for_cone_verdict
    }

    pub fn has_no_selector(&self) -> bool {
        !self.branch_selector_capability_present
            && self.selected_root_hash.is_none()
            && !self.enacted_root_privilege_present
    }

    pub fn semantic_nu_is_diagnostic_only(&self) -> bool {
        self.semantic_nu_diagnostic_only_at_guarded_stages
    }

    pub fn bar_is_diagnostic_only(&self) -> bool {
        self.bar_diagnostic_only_at_guarded_stages
    }

    pub fn f_s4_3_armed(&self) -> bool {
        self.f_s4_3_armed
    }

    pub fn forbidden_capabilities(&self) -> &[String] {
        &self.forbidden_capabilities
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4OptionAExecutionReplayV1 {
    pub valid: bool,
    pub v3_replayed: bool,
    pub option_a_adoption_exact: bool,
    pub root_count: usize,
    pub all_four_execution_authorized: bool,
    pub selector_capability_present: bool,
    pub errors: Vec<String>,
}

impl Stage4OptionAExecutionReplayV1 {
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn join(&self, separator: &str) -> String {
        self.errors.join(separator)
    }
}

/// In-process proof object joining one issued capability to an independent
/// deterministic replay of its complete authority chain.  Its fields stay
/// private so downstream construction can reuse the expensive replay without
/// accepting a caller-authored `valid = true` projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stage4OptionAReplayedExecutionGrantV1 {
    capability: Stage4OptionAExecutionCapabilityV1,
    replay: Stage4OptionAExecutionReplayV1,
}

impl Stage4OptionAReplayedExecutionGrantV1 {
    pub fn capability(&self) -> &Stage4OptionAExecutionCapabilityV1 {
        &self.capability
    }

    pub fn replay(&self) -> &Stage4OptionAExecutionReplayV1 {
        &self.replay
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Stage4OptionAExecutionV1Error {
    #[error("Stage-4 Option-A v1 JSON failed: {0}")]
    Json(String),
    #[error("Stage-4 Option-A v1 prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("Stage-4 Option-A v1 exact binding failed: {0}")]
    Binding(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(STAGE4_OPTION_A_EXECUTION_V1_SCHEMA, domain, value))
        .expect("Stage-4 Option-A evidence serializes");
    bytes_hash(&bytes)
}

fn stage4_v1_postseal_r_t1_telescope_hash<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(&(
        STAGE4_SEMANTIC_PARSIMONY_V1_SCHEMA,
        "postseal-r-t1-package-telescope",
        value,
    ))
    .expect("Stage-4 v1 R-T1 telescope testimony serializes");
    bytes_hash(&bytes)
}

fn stem_entry_hash(entry: &Stage4OptionAStemEntryV1) -> String {
    let mut projection = entry.clone();
    projection.derivation_hash.clear();
    tagged_hash("cross-bound-common-stem-entry", &projection)
}

fn root_hash(root: &Stage4OptionAExecutionRootV1) -> String {
    let mut projection = root.clone();
    projection.derivation_hash.clear();
    tagged_hash("authorized-four-root-member", &projection)
}

fn capability_hash(capability: &Stage4OptionAExecutionCapabilityV1) -> String {
    let mut projection = capability.clone();
    projection.derivation_hash.clear();
    tagged_hash("all-four-execution-capability", &projection)
}

fn exact_option_a_adoption_block() -> Result<bool, Stage4OptionAExecutionV1Error> {
    let text = std::str::from_utf8(OPTION_A_ADJUDICATION_BYTES)
        .map_err(|error| Stage4OptionAExecutionV1Error::Prerequisite(error.to_string()))?;
    let start = text.find("## ADOPTION").ok_or_else(|| {
        Stage4OptionAExecutionV1Error::Prerequisite(
            "semantic-divergence adjudication has no ADOPTION block".to_owned(),
        )
    })?;
    Ok(&text[start..] == EXPECTED_OPTION_A_ADOPTION_BLOCK)
}

fn expected_semantic_register() -> BTreeMap<&'static str, u32> {
    EXPECTED_ROOT_SEMANTIC_NU.into_iter().collect()
}

fn derive_common_stem(
    v3: &Stage4SemanticParsimonyV3Certificate,
) -> Result<(Vec<Stage4OptionAStemEntryV1>, String), Stage4OptionAExecutionV1Error> {
    let entries = v3
        .replayed_v2_claim
        .external_bi0_opening_capability
        .exact_stage1_through3_entries();
    if entries.len() != 3 || entries.iter().map(|entry| entry.stage()).ne(1..=3) {
        return Err(Stage4OptionAExecutionV1Error::Binding(
            "v3 external BI-0 opening is not exactly Stages 1 through 3".to_owned(),
        ));
    }
    let telescopes = entries
        .iter()
        .map(|entry| (entry.stage(), entry.telescope().clone()))
        .collect::<Vec<_>>();
    let signature_digest = SealedSignature::from_telescopes(telescopes.clone())
        .digest()
        .to_owned();
    if signature_digest
        != v3
            .cross_binding
            .computed_external_full_prefix_signature_digest
        || signature_digest != v3.cross_binding.blind_opening_full_prefix_signature_digest
        || signature_digest != v3.cross_binding.preseal_common_prefix_signature_digest
        || signature_digest
            != v3
                .cross_binding
                .cone_geometry_common_prefix_signature_digest
    {
        return Err(Stage4OptionAExecutionV1Error::Binding(
            "v3 common-stem signature copies are not exact".to_owned(),
        ));
    }
    let mut stem = Vec::with_capacity(3);
    for (position, entry) in entries.iter().enumerate() {
        let predecessor = SealedSignature::from_telescopes(telescopes[..position].to_vec())
            .digest()
            .to_owned();
        let cross_row = v3.cross_binding.rows.get(position).ok_or_else(|| {
            Stage4OptionAExecutionV1Error::Binding(format!(
                "v3 cross-binding row {position} is absent"
            ))
        })?;
        if entry.candidate_hash() != candidate_hash(entry.telescope())
            || entry.predecessor_signature_digest() != predecessor
            || !cross_row.row_proved
            || cross_row.stage != entry.stage()
            || cross_row.external_candidate_hash != entry.candidate_hash()
            || cross_row.external_predecessor_signature_digest != predecessor
        {
            return Err(Stage4OptionAExecutionV1Error::Binding(format!(
                "v3 common-stem entry {} is not exactly cross-bound",
                entry.stage()
            )));
        }
        let mut row = Stage4OptionAStemEntryV1 {
            stage: entry.stage(),
            telescope: entry.telescope().clone(),
            candidate_hash: entry.candidate_hash().to_owned(),
            predecessor_signature_digest: predecessor,
            derivation_hash: String::new(),
        };
        row.derivation_hash = stem_entry_hash(&row);
        stem.push(row);
    }
    Ok((stem, signature_digest))
}

fn validate_embedded_r_t1_join(
    join: &Stage4StructuralRt1JoinV1,
) -> Result<(), Stage4OptionAExecutionV1Error> {
    let distinct_live_candidates = join
        .live_candidate_hashes
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>()
        .len();
    let distinct_package_candidates = join
        .package_bindings
        .iter()
        .map(|binding| binding.candidate_hash.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let distinct_class_ids = join
        .orbit_class_bindings
        .iter()
        .map(|binding| binding.class_id.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    if !join.replay_valid
        || !join.exact_candidate_multiset_join
        || !join.every_structural_derivation_bound
        || join.used_as_semantic_selector
        || join.structural_audit_derivation_hash.is_empty()
        || join.live_candidate_hashes.len() != 4
        || join.package_bindings.len() != 4
        || join.pairwise_bindings.len() != 6
        || join.orbit_class_bindings.len() != 4
        || distinct_live_candidates != 4
        || distinct_package_candidates != 4
        || distinct_class_ids != 4
    {
        return Err(Stage4OptionAExecutionV1Error::Prerequisite(
            "the fully replayed v3 certificate does not contain one exact selector-free four-root R-T1 join"
                .to_owned(),
        ));
    }
    Ok(())
}

fn embedded_r_t1_testimony_for<'a>(
    join: &'a Stage4StructuralRt1JoinV1,
    candidate: &str,
    telescope: &'a Telescope,
) -> Result<(&'a str, &'a str, &'a str, &'a Telescope), Stage4OptionAExecutionV1Error> {
    let live_matches = join
        .live_candidate_hashes
        .iter()
        .filter(|member| member.as_str() == candidate)
        .count();
    let classes = join
        .orbit_class_bindings
        .iter()
        .filter(|class| {
            class
                .member_candidate_hashes
                .iter()
                .any(|member| member == candidate)
        })
        .collect::<Vec<_>>();
    let packages = join
        .package_bindings
        .iter()
        .filter(|package| package.candidate_hash.as_str() == candidate)
        .collect::<Vec<_>>();
    if live_matches != 1 || classes.len() != 1 || packages.len() != 1 {
        return Err(Stage4OptionAExecutionV1Error::Binding(format!(
            "embedded R-T1 testimony has {live_matches}/{}/{} live/class/package matches for {candidate}",
            classes.len(),
            packages.len()
        )));
    }
    if candidate_hash(telescope) != candidate
        || packages[0].telescope_hash != stage4_v1_postseal_r_t1_telescope_hash(telescope)
        || packages[0].package_derivation_hash.is_empty()
        || classes[0].class_derivation_hash.is_empty()
    {
        return Err(Stage4OptionAExecutionV1Error::Binding(format!(
            "embedded R-T1 package/class testimony is not bound to the v3 geometry for {candidate}"
        )));
    }
    Ok((
        classes[0].class_id.as_str(),
        classes[0].class_derivation_hash.as_str(),
        packages[0].package_derivation_hash.as_str(),
        telescope,
    ))
}

fn r_t2_economy_key(
    branch: &crate::r_t2_future_hole_confluence_v2::Rt2V2BranchFormation,
) -> Result<&str, Stage4OptionAExecutionV1Error> {
    let rows = branch
        .future_hole_aggregate
        .registrations
        .iter()
        .filter(|row| row.dependent_context_declared_arity == Some(3))
        .collect::<Vec<_>>();
    if rows.len() != 1
        || rows[0].branch_independent_semantic_key.is_empty()
        || rows[0].branch_local_family_id_used_in_semantic_key
    {
        return Err(Stage4OptionAExecutionV1Error::Binding(format!(
            "R-T2 branch {} lacks one exact branch-independent arity-3 economy key",
            branch.stage4_candidate_hash
        )));
    }
    Ok(rows[0].branch_independent_semantic_key.as_str())
}

fn derive_capability_after_replays(
    v3: &Stage4SemanticParsimonyV3Certificate,
    option_a_adoption_block_exact: bool,
    r_t2: &Rt2FutureHoleConfluenceV2Certificate,
) -> Result<Stage4OptionAExecutionCapabilityV1, Stage4OptionAExecutionV1Error> {
    let v3_cross_binding_exact = v3
        .cross_binding
        .exact_bi0_to_blind_preseal_cross_binding_proved;
    let v3_cone_audit_authorized = v3.authorized_stage4_cone_audit;
    let v3_pre_adjudication_branch_authority_was_absent = !v3.authorized_bi1_branch_execution;
    if v3.schema != STAGE4_SEMANTIC_PARSIMONY_V3_SCHEMA
        || !option_a_adoption_block_exact
        || !v3.v2_claim_fully_replayed_before_cross_binding
        || !v3_cross_binding_exact
        || !v3_cone_audit_authorized
        || !v3_pre_adjudication_branch_authority_was_absent
        || !v3.no_branch_selected_or_executed
        || !v3.divergence_requires_versioned_adjudication
    {
        return Err(Stage4OptionAExecutionV1Error::Prerequisite(
            "v3 and the adopted Option-A block do not form the exact post-adjudication gate"
                .to_owned(),
        ));
    }

    let (common_stem, common_stem_signature_digest) = derive_common_stem(v3)?;
    let blind_audit = &v3.replayed_v2_claim.blind_audit;
    let preseal = &blind_audit.preseal;
    let r_t1 = &blind_audit.structural_r_t1_join;
    validate_embedded_r_t1_join(r_t1)?;
    if preseal.roots.len() != 4
        || preseal.live_strict_cone_geometry.roots.len() != 4
        || v3.cross_binding.root_prefix_extensions.len() != 4
        || v3.cross_binding.selection_root_joins.len() != 4
        || r_t1.package_bindings.len() != 4
        || r_t1.orbit_class_bindings.len() != 4
        || r_t2.branch_count != 4
        || r_t2.branches.len() != 4
    {
        return Err(Stage4OptionAExecutionV1Error::Binding(
            "one of the exact four-root surfaces has drifted".to_owned(),
        ));
    }

    let economy_keys = r_t2
        .branches
        .iter()
        .map(|branch| r_t2_economy_key(branch).map(str::to_owned))
        .collect::<Result<Vec<_>, _>>()?;
    let economy_class_sizes = economy_keys.iter().fold(BTreeMap::new(), |mut map, key| {
        *map.entry(key.clone()).or_insert(0usize) += 1;
        map
    });
    if economy_class_sizes.len() != 2 || economy_class_sizes.values().any(|count| *count != 2) {
        return Err(Stage4OptionAExecutionV1Error::Binding(
            "R-T2 economy geometry is not exactly two classes of two roots".to_owned(),
        ));
    }

    let expected_register = expected_semantic_register();
    let mut roots = Vec::with_capacity(4);
    for semantic_root in &preseal.roots {
        let expected_nu = expected_register
            .get(semantic_root.candidate_hash.as_str())
            .copied()
            .ok_or_else(|| {
                Stage4OptionAExecutionV1Error::Binding(format!(
                    "unexpected v3 root {}",
                    semantic_root.candidate_hash
                ))
            })?;
        let geometry = preseal
            .live_strict_cone_geometry
            .roots
            .iter()
            .filter(|row| row.candidate_hash == semantic_root.candidate_hash)
            .collect::<Vec<_>>();
        let extension = v3
            .cross_binding
            .root_prefix_extensions
            .iter()
            .filter(|row| row.candidate_hash == semantic_root.candidate_hash)
            .collect::<Vec<_>>();
        let selection = v3
            .cross_binding
            .selection_root_joins
            .iter()
            .filter(|row| row.candidate_hash == semantic_root.candidate_hash)
            .collect::<Vec<_>>();
        let r_t2_matches = r_t2
            .branches
            .iter()
            .filter(|branch| branch.stage4_candidate_hash == semantic_root.candidate_hash)
            .collect::<Vec<_>>();
        if geometry.len() != 1
            || extension.len() != 1
            || selection.len() != 1
            || r_t2_matches.len() != 1
        {
            return Err(Stage4OptionAExecutionV1Error::Binding(format!(
                "root {} lacks a unique v3/R-T2 join",
                semantic_root.candidate_hash
            )));
        }
        let geometry = geometry[0];
        let extension = extension[0];
        let selection = selection[0];
        let r_t2_branch = r_t2_matches[0];
        let economy_key = r_t2_economy_key(r_t2_branch)?.to_owned();
        let economy_class_size = economy_class_sizes[&economy_key];
        let (
            r_t1_class_id,
            r_t1_class_derivation_hash,
            r_t1_package_derivation_hash,
            r_t1_telescope,
        ) = embedded_r_t1_testimony_for(r_t1, &semantic_root.candidate_hash, &geometry.telescope)?;
        let exact_v3_root_binding = semantic_root.kappa == 3
            && semantic_root.stage4_semantic_nu == expected_nu
            && semantic_root.exact_candidate_prefix_and_package_bindings
            && semantic_root.root_semantic_audit_proved
            && candidate_hash(&geometry.telescope) == semantic_root.candidate_hash
            && geometry.kappa == 3
            && extension.row_proved
            && extension.exact_candidate_join
            && extension.exact_geometry_telescope_hash
            && extension.exact_stage4_extension_signature
            && extension.semantic_root_prefix_signature_digest
                == semantic_root.prefix_signature_digest
            && selection.row_proved
            && selection.exact_semantic_root_projection
            && selection.proved_root_extension_joined;
        let exact_r_t1_class_testimony = r_t1_telescope == &geometry.telescope
            && candidate_hash(r_t1_telescope) == semantic_root.candidate_hash
            && r_t1_class_id == r_t2_branch.r_t1_class_id;
        let exact_r_t2_economy_testimony = r_t2_branch.stage4_telescope == geometry.telescope
            && r_t2_branch.prefix_signature_digest == semantic_root.prefix_signature_digest
            && r_t2_branch.prefix_steps == vec![1, 2, 3, 4]
            && r_t2_branch.prefix_steps_exactly_one_through_four
            && economy_class_size == 2;
        if !exact_v3_root_binding || !exact_r_t1_class_testimony || !exact_r_t2_economy_testimony {
            return Err(Stage4OptionAExecutionV1Error::Binding(format!(
                "root {} failed exact v3/R-T1/R-T2 binding",
                semantic_root.candidate_hash
            )));
        }
        let mut row = Stage4OptionAExecutionRootV1 {
            candidate_hash: semantic_root.candidate_hash.clone(),
            telescope: geometry.telescope.clone(),
            prefix_signature_digest: semantic_root.prefix_signature_digest.clone(),
            kappa: semantic_root.kappa,
            semantic_nu: semantic_root.stage4_semantic_nu,
            v3_semantic_root_derivation_hash: semantic_root.derivation_hash.clone(),
            v3_root_extension_derivation_hash: extension.derivation_hash.clone(),
            v3_selection_join_derivation_hash: selection.derivation_hash.clone(),
            r_t1_class_id: r_t1_class_id.to_owned(),
            r_t1_class_derivation_hash: r_t1_class_derivation_hash.to_owned(),
            r_t1_package_derivation_hash: r_t1_package_derivation_hash.to_owned(),
            r_t2_economy_geometry_key: economy_key,
            r_t2_economy_class_size: economy_class_size,
            exact_v3_root_binding,
            exact_r_t1_class_testimony,
            exact_r_t2_economy_testimony,
            // Authorization is deliberately a v3 + Option-A fact.  The
            // class/economy rows corroborate identity but never choose.
            execution_authorized: option_a_adoption_block_exact && exact_v3_root_binding,
            derivation_hash: String::new(),
        };
        row.derivation_hash = root_hash(&row);
        roots.push(row);
    }
    roots.sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));
    let authorized_root_hashes = roots
        .iter()
        .map(|root| root.candidate_hash.clone())
        .collect::<Vec<_>>();
    let expected_hashes = EXPECTED_ROOT_SEMANTIC_NU
        .iter()
        .map(|(hash, _)| (*hash).to_owned())
        .collect::<Vec<_>>();
    let exact_four_root_surface = roots.len() == 4
        && authorized_root_hashes == expected_hashes
        && roots
            .iter()
            .all(|root| root.execution_authorized && root.derivation_hash == root_hash(root));
    let enacted_root_hash = v3.replayed_v2_claim.blind_audit.enacted_root_hash.clone();
    if !authorized_root_hashes
        .iter()
        .any(|root| root == &enacted_root_hash)
    {
        return Err(Stage4OptionAExecutionV1Error::Binding(
            "v3 enacted indexical root is outside the authorized four-root surface".to_owned(),
        ));
    }
    let exact_per_root_semantic_register = roots
        .iter()
        .all(|root| expected_register.get(root.candidate_hash.as_str()) == Some(&root.semantic_nu));
    let exact_all_kappa_three = roots.iter().all(|root| root.kappa == 3);
    let r_t1_class_ids_derived_as_testimony_only =
        roots.iter().all(|root| root.exact_r_t1_class_testimony)
            && roots
                .iter()
                .map(|root| root.r_t1_class_id.as_str())
                .collect::<BTreeSet<_>>()
                .len()
                == 4;
    let r_t2_economy_geometry_derived_as_testimony_only =
        roots.iter().all(|root| root.exact_r_t2_economy_testimony)
            && roots
                .iter()
                .map(|root| root.r_t2_economy_geometry_key.as_str())
                .collect::<BTreeSet<_>>()
                .len()
                == 2;
    let r_t1_or_r_t2_value_used_as_authorization_input = false;
    let semantic_nu_diagnostic_only_at_guarded_stages = true;
    let bar_diagnostic_only_at_guarded_stages = true;
    let f_s4_3_armed = true;
    let branch_selector_capability_present = false;
    let selected_root_hash = None;
    let enacted_root_privilege_present = false;
    let artifact_write_capability_present = false;
    let all_four_execution_authorized = option_a_adoption_block_exact
        && v3_cross_binding_exact
        && v3_cone_audit_authorized
        && v3_pre_adjudication_branch_authority_was_absent
        && exact_four_root_surface
        && exact_per_root_semantic_register
        && exact_all_kappa_three
        && semantic_nu_diagnostic_only_at_guarded_stages
        && bar_diagnostic_only_at_guarded_stages
        && f_s4_3_armed
        && !branch_selector_capability_present
        && selected_root_hash.is_none()
        && !enacted_root_privilege_present
        && !r_t1_or_r_t2_value_used_as_authorization_input
        && !artifact_write_capability_present;
    if !all_four_execution_authorized {
        return Err(Stage4OptionAExecutionV1Error::Binding(
            "the exact all-four no-selector authorization did not close".to_owned(),
        ));
    }

    let mut capability = Stage4OptionAExecutionCapabilityV1 {
        schema: STAGE4_OPTION_A_EXECUTION_V1_SCHEMA.to_owned(),
        date: STAGE4_OPTION_A_EXECUTION_V1_DATE.to_owned(),
        theorem_id: STAGE4_OPTION_A_EXECUTION_V1_THEOREM_ID.to_owned(),
        source_v3_artifact_bytes_hash: bytes_hash(V3_ARTIFACT_BYTES),
        source_v3_result_digest: v3.result_digest.clone(),
        source_option_a_adjudication_bytes_hash: bytes_hash(OPTION_A_ADJUDICATION_BYTES),
        source_r_t1_audit_derivation_hash: r_t1.structural_audit_derivation_hash.clone(),
        source_r_t2_artifact_bytes_hash: bytes_hash(R_T2_ARTIFACT_BYTES),
        source_r_t2_result_digest: r_t2.result_digest.clone(),
        v3_fully_replayed_before_authorization: true,
        option_a_adoption_block_exact,
        v3_cross_binding_exact,
        v3_cone_audit_authorized,
        v3_pre_adjudication_branch_authority_was_absent,
        common_stem,
        common_stem_signature_digest,
        roots,
        authorized_root_hashes,
        enacted_root_hash,
        enacted_root_is_indexical_testimony_only: true,
        exact_four_root_surface,
        exact_per_root_semantic_register,
        exact_all_kappa_three,
        r_t1_class_ids_derived_as_testimony_only,
        r_t2_economy_geometry_derived_as_testimony_only,
        r_t1_or_r_t2_value_used_as_authorization_input,
        semantic_nu_diagnostic_only_at_guarded_stages,
        bar_diagnostic_only_at_guarded_stages,
        f_s4_3_armed,
        all_four_execution_authorized,
        complete_sweep_required_for_cone_verdict: true,
        branch_selector_capability_present,
        selected_root_hash,
        enacted_root_privilege_present,
        ordering_used_only_for_serialization: true,
        forbidden_capabilities: FORBIDDEN_CAPABILITIES
            .iter()
            .map(|capability| (*capability).to_owned())
            .collect(),
        artifact_write_capability_present,
        permitted_conclusion: "The exact cross-bound Stage-4 cone may be executed root by root only as one complete four-root BI-1 sweep. Semantic nu and the bar are diagnostics; neither may select, prune, privilege, or repair a root. R-T1 classes and R-T2 economy geometry are testimony only. BI-4, UC-1, the bridge, and artifact writing are outside this capability."
            .to_owned(),
        derivation_hash: String::new(),
    };
    capability.derivation_hash = capability_hash(&capability);
    Ok(capability)
}

/// Issue the narrow post-adjudication BI-1 execution capability.  This fully
/// replays v3 before reading the adoption as an authority transition.  The
/// exact R-T1 package/class testimony is projected from v3's already-replayed
/// embedded v1 join; it is not reissued a second time.  This executes no root
/// and writes no artifact.
pub fn issue_stage4_option_a_execution_v1()
-> Result<Stage4OptionAExecutionCapabilityV1, Stage4OptionAExecutionV1Error> {
    let v3: Stage4SemanticParsimonyV3Certificate = serde_json::from_slice(V3_ARTIFACT_BYTES)
        .map_err(|error| Stage4OptionAExecutionV1Error::Json(error.to_string()))?;
    let v3_replay = replay_stage4_semantic_parsimony_v3(&v3);
    if !v3_replay.valid {
        return Err(Stage4OptionAExecutionV1Error::Prerequisite(format!(
            "Stage-4 semantic parsimony v3 failed full replay: {}",
            v3_replay.errors.join("; ")
        )));
    }
    let option_a_adoption_block_exact = exact_option_a_adoption_block()?;
    if !option_a_adoption_block_exact {
        return Err(Stage4OptionAExecutionV1Error::Prerequisite(
            "the adopted Option-A block is not byte-exact".to_owned(),
        ));
    }
    let r_t2: Rt2FutureHoleConfluenceV2Certificate = serde_json::from_slice(R_T2_ARTIFACT_BYTES)
        .map_err(|error| Stage4OptionAExecutionV1Error::Json(error.to_string()))?;
    let r_t2_errors = replay_archived_stage4_fork_projection(&r_t2);
    if !r_t2_errors.is_empty()
        || r_t2.schema != R_T2_FUTURE_HOLE_CONFLUENCE_V2_SCHEMA
        || r_t2.outcome != Rt2FutureHoleConfluenceV2Outcome::LawLevelConfluenceRefutedRungRt3Opened
        || r_t2.selected_candidate_hash.is_some()
        || r_t2.hash_or_enumeration_order_used_as_selector
    {
        return Err(Stage4OptionAExecutionV1Error::Prerequisite(format!(
            "R-T2 economy testimony did not replay without a selector: {}",
            r_t2_errors.join("; ")
        )));
    }
    derive_capability_after_replays(&v3, option_a_adoption_block_exact, &r_t2)
}

fn replay_against_expected(
    claimed: &Stage4OptionAExecutionCapabilityV1,
    expected: &Stage4OptionAExecutionCapabilityV1,
) -> Stage4OptionAExecutionReplayV1 {
    let mut errors = Vec::new();
    if claimed.derivation_hash != capability_hash(claimed) {
        errors.push("Stage-4 Option-A capability digest mismatch".to_owned());
    }
    if claimed != expected {
        errors.push("Stage-4 Option-A capability differs from deterministic reissuance".to_owned());
    }
    Stage4OptionAExecutionReplayV1 {
        valid: errors.is_empty(),
        v3_replayed: claimed.v3_fully_replayed_before_authorization,
        option_a_adoption_exact: claimed.option_a_adoption_block_exact,
        root_count: claimed.roots.len(),
        all_four_execution_authorized: claimed.all_four_execution_authorized,
        selector_capability_present: claimed.branch_selector_capability_present
            || claimed.selected_root_hash.is_some()
            || claimed.enacted_root_privilege_present,
        errors,
    }
}

/// Deterministically replay the complete authority chain and compare every
/// capability field.  Replay never executes a branch.
pub fn replay_stage4_option_a_execution_v1(
    claimed: &Stage4OptionAExecutionCapabilityV1,
) -> Stage4OptionAExecutionReplayV1 {
    match issue_stage4_option_a_execution_v1() {
        Ok(expected) => replay_against_expected(claimed, &expected),
        Err(error) => Stage4OptionAExecutionReplayV1 {
            valid: false,
            v3_replayed: false,
            option_a_adoption_exact: false,
            root_count: claimed.roots.len(),
            all_four_execution_authorized: claimed.all_four_execution_authorized,
            selector_capability_present: claimed.branch_selector_capability_present
                || claimed.selected_root_hash.is_some()
                || claimed.enacted_root_privilege_present,
            errors: vec![error.to_string()],
        },
    }
}

/// Issue once and independently replay once, yielding a sealed in-process
/// grant that may be projected into several branch-local artifacts.  This is
/// a performance boundary only: it removes redundant authority reissuance,
/// not either side of the issue/replay pair.
pub fn issue_replayed_stage4_option_a_execution_v1()
-> Result<Stage4OptionAReplayedExecutionGrantV1, Stage4OptionAExecutionV1Error> {
    let capability = issue_stage4_option_a_execution_v1()?;
    let replay = replay_stage4_option_a_execution_v1(&capability);
    if !replay.valid {
        return Err(Stage4OptionAExecutionV1Error::Prerequisite(format!(
            "issued Option-A execution capability failed independent replay: {}",
            replay.errors.join("; ")
        )));
    }
    Ok(Stage4OptionAReplayedExecutionGrantV1 { capability, replay })
}

pub fn replay_stage4_option_a_execution_v1_json(json: &str) -> Stage4OptionAExecutionReplayV1 {
    match serde_json::from_str::<Stage4OptionAExecutionCapabilityV1>(json) {
        Ok(capability) => replay_stage4_option_a_execution_v1(&capability),
        Err(error) => Stage4OptionAExecutionReplayV1 {
            valid: false,
            v3_replayed: false,
            option_a_adoption_exact: false,
            root_count: 0,
            all_four_execution_authorized: false,
            selector_capability_present: false,
            errors: vec![format!(
                "Stage-4 Option-A JSON did not deserialize: {error}"
            )],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn archived_v3() -> Stage4SemanticParsimonyV3Certificate {
        serde_json::from_slice(V3_ARTIFACT_BYTES).expect("archived Stage-4 v3 certificate")
    }

    fn issued() -> &'static Stage4OptionAExecutionCapabilityV1 {
        static CAPABILITY: OnceLock<Stage4OptionAExecutionCapabilityV1> = OnceLock::new();
        CAPABILITY.get_or_init(|| {
            issue_stage4_option_a_execution_v1().expect("exact Option-A execution capability")
        })
    }

    #[test]
    fn adopted_option_a_block_is_byte_exact() {
        assert_eq!(exact_option_a_adoption_block(), Ok(true));
    }

    #[test]
    fn production_surface_cannot_execute_select_or_emit() {
        let source = include_str!("stage4_option_a_execution_v1.rs");
        let production = source.split("#[cfg(test)]").next().unwrap_or(source);
        assert!(production.contains("pub fn issue_stage4_option_a_execution_v1"));
        assert!(production.contains("pub fn replay_stage4_option_a_execution_v1"));
        assert!(!production.contains("issue_stage4_r_t1_orbit_audit"));
        assert!(!production.contains("replay_stage4_r_t1_orbit_audit"));
        assert!(!production.contains("execute_branch_continuation"));
        assert!(!production.contains("issue_bi_program_after_bi0"));
        assert!(!production.contains("OpenOptions"));
        assert!(!production.contains("create_new("));
        assert!(!production.contains("pub fn emit_"));
    }

    #[test]
    fn embedded_r_t1_projection_preserves_capability_testimony_and_digest() {
        let v3 = archived_v3();
        let blind_audit = &v3.replayed_v2_claim.blind_audit;
        let join = &blind_audit.structural_r_t1_join;
        validate_embedded_r_t1_join(join).expect("exact embedded R-T1 join");

        let r_t2: Rt2FutureHoleConfluenceV2Certificate =
            serde_json::from_slice(R_T2_ARTIFACT_BYTES).expect("archived R-T2 certificate");
        let capability = derive_capability_after_replays(&v3, true, &r_t2)
            .expect("capability projected from replayed v3 testimony");
        assert_eq!(
            capability.source_r_t1_audit_derivation_hash,
            join.structural_audit_derivation_hash
        );
        assert_eq!(capability.derivation_hash, capability_hash(&capability));

        for root in capability.roots() {
            let package = join
                .package_bindings
                .iter()
                .find(|binding| binding.candidate_hash == root.candidate_hash())
                .expect("one embedded package binding");
            let class = join
                .orbit_class_bindings
                .iter()
                .find(|binding| {
                    binding
                        .member_candidate_hashes
                        .iter()
                        .any(|member| member == root.candidate_hash())
                })
                .expect("one embedded class binding");
            assert_eq!(
                root.r_t1_package_derivation_hash,
                package.package_derivation_hash
            );
            assert_eq!(root.r_t1_class_id, class.class_id);
            assert_eq!(root.r_t1_class_derivation_hash, class.class_derivation_hash);
            assert_eq!(
                package.telescope_hash,
                stage4_v1_postseal_r_t1_telescope_hash(root.telescope())
            );
            assert!(root.exact_r_t1_class_testimony);
        }
    }

    #[test]
    fn embedded_r_t1_projection_rejects_rehashed_binding_mutations() {
        let v3 = archived_v3();
        let preseal = &v3.replayed_v2_claim.blind_audit.preseal;
        let join = &v3.replayed_v2_claim.blind_audit.structural_r_t1_join;
        let semantic_root = &preseal.roots[0];
        let geometry = preseal
            .live_strict_cone_geometry
            .roots
            .iter()
            .find(|root| root.candidate_hash == semantic_root.candidate_hash)
            .expect("root geometry");

        let mut wrong_package = join.clone();
        wrong_package
            .package_bindings
            .iter_mut()
            .find(|binding| binding.candidate_hash == semantic_root.candidate_hash)
            .expect("package binding")
            .telescope_hash = "blake3:rehashed-wrong-telescope".to_owned();
        assert!(
            embedded_r_t1_testimony_for(
                &wrong_package,
                &semantic_root.candidate_hash,
                &geometry.telescope,
            )
            .is_err()
        );

        let mut duplicate_class = join.clone();
        duplicate_class
            .orbit_class_bindings
            .iter_mut()
            .find(|class| {
                !class
                    .member_candidate_hashes
                    .iter()
                    .any(|member| member == &semantic_root.candidate_hash)
            })
            .expect("different orbit class")
            .member_candidate_hashes
            .push(semantic_root.candidate_hash.clone());
        assert!(
            embedded_r_t1_testimony_for(
                &duplicate_class,
                &semantic_root.candidate_hash,
                &geometry.telescope,
            )
            .is_err()
        );

        let mut selector_tainted = join.clone();
        selector_tainted.used_as_semantic_selector = true;
        assert!(validate_embedded_r_t1_join(&selector_tainted).is_err());
    }

    #[test]
    fn capability_grants_exactly_all_four_without_selector() {
        let capability = issued();
        assert_eq!(capability.roots().len(), 4);
        assert_eq!(
            capability
                .roots()
                .iter()
                .map(|root| (root.candidate_hash(), root.semantic_nu(), root.kappa()))
                .collect::<Vec<_>>(),
            EXPECTED_ROOT_SEMANTIC_NU
                .iter()
                .map(|(hash, nu)| (*hash, *nu, 3))
                .collect::<Vec<_>>()
        );
        assert!(capability.all_four_execution_authorized());
        assert!(capability.complete_sweep_required_for_cone_verdict());
        assert!(capability.has_no_selector());
        assert!(capability.semantic_nu_is_diagnostic_only());
        assert!(capability.bar_is_diagnostic_only());
        assert!(capability.f_s4_3_armed());
        assert!(capability.r_t1_class_ids_derived_as_testimony_only);
        assert!(capability.r_t2_economy_geometry_derived_as_testimony_only);
        assert!(!capability.r_t1_or_r_t2_value_used_as_authorization_input);
        assert_eq!(
            capability.forbidden_capabilities(),
            FORBIDDEN_CAPABILITIES
                .iter()
                .map(|value| (*value).to_owned())
                .collect::<Vec<_>>()
        );
        assert!(
            capability
                .authorized_root_hashes()
                .iter()
                .any(|root| root == capability.enacted_indexical_root_hash())
        );
        assert!(capability.enacted_root_is_indexical_testimony_only());
    }

    #[test]
    fn deterministic_replay_reissues_the_complete_authority_chain() {
        let replay = replay_stage4_option_a_execution_v1(issued());
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(replay.v3_replayed);
        assert!(replay.option_a_adoption_exact);
        assert_eq!(replay.root_count, 4);
        assert!(replay.all_four_execution_authorized);
        assert!(!replay.selector_capability_present);
    }

    #[test]
    fn rehashed_semantic_value_mutation_fails_deterministic_comparison() {
        let expected = issued();
        let mut forged = expected.clone();
        forged.roots[0].semantic_nu += 1;
        forged.roots[0].derivation_hash = root_hash(&forged.roots[0]);
        forged.derivation_hash = capability_hash(&forged);
        let replay = replay_against_expected(&forged, expected);
        assert!(!replay.valid);
        assert!(
            replay
                .errors
                .iter()
                .any(|error| error.contains("deterministic reissuance"))
        );
    }

    #[test]
    fn rehashed_selector_or_root_omission_mutations_fail() {
        let expected = issued();

        let mut selector = expected.clone();
        selector.branch_selector_capability_present = true;
        selector.selected_root_hash = Some(selector.roots[0].candidate_hash.clone());
        selector.derivation_hash = capability_hash(&selector);
        let selector_replay = replay_against_expected(&selector, expected);
        assert!(!selector_replay.valid);
        assert!(selector_replay.selector_capability_present);

        let mut omitted = expected.clone();
        omitted.roots.pop();
        omitted.authorized_root_hashes.pop();
        omitted.derivation_hash = capability_hash(&omitted);
        let omitted_replay = replay_against_expected(&omitted, expected);
        assert!(!omitted_replay.valid);
        assert_eq!(omitted_replay.root_count, 3);
    }
}
