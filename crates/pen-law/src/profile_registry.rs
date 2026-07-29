//! Immutable experiment-profile registry for the Law-V2 research lane.
//!
//! The registry classifies completed experiments and reserves identifiers for
//! proposed successors.  It carries no candidate grammar and grants no later
//! profile authority to import an earlier result as a privileged fixture.

use std::collections::BTreeSet;

use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const LAW_V2_PROFILE_REGISTRY_SCHEMA_VERSION: u16 = 1;
pub const LAW_V2_PROFILE_REGISTRY_ID: &str = "law-v2-profile-registry-v1";

const EMBEDDED_LAW_V2_PROFILE_REGISTRY: &[u8] =
    include_bytes!("../assets/law_v2_profile_registry_v1.json");

const PROFILE_A_ID: &str = "gsc-inductive-completion-core-v1";
const PROFILE_A_CONTINUATION_ID: &str = "law-v2-owner-specific-inductive-continuation-v1";
const CONTEXTUAL_PROFILE_ID: &str = "gsc-contextual-internalization-v1";
const PRODUCTIVE_PROFILE_ID: &str = "selective-productive-discharge-v1";

const PROFILE_A_BASE_SEMANTIC_DIGEST: &str =
    "blake3:d61458ebd47036861e48af9ef458df1b2b3dc890194069958ef2f14d4afdd11e";
const PROFILE_A_CONTINUATION_SEMANTIC_DIGEST: &str =
    "blake3:00d97ce3446442d91b8572557c16dd0576f7281260b5c000e64f41323323c7e2";
const PROFILE_A_H3_RESULT_DIGEST: &str =
    "blake3:f43acaf6f0b0b9e51dc9a55829eedbc1fb2f7cf1090260ac2d244d1616a27d03";
const PROFILE_A_H4_RESULT_DIGEST: &str =
    "blake3:20f8940f305a71802728af3c9206b7f0d0c126c4528459e9274f878a7dd324b8";

/// Exhaustive final-status vocabulary for a structural run.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum StructuralRunTermination {
    HaltedDebtFree,
    BlockedNoDischarger,
    BlockedProductivity,
    OutsideFragment,
    ResourceExhausted,
    Unknown,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileRegistryStatus {
    FrozenExecuted,
    ProposedNotAdopted,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileResultClassification {
    DirectEliminatorThenDebtFreeHalt,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedLawV2ProfileRegistryEntryV1 {
    pub id: String,
    pub execution_profile_ids: Vec<String>,
    pub status: ProfileRegistryStatus,
    pub semantic_manifest_digests: Vec<Digest>,
    pub result_digests: Vec<Digest>,
    pub result: Option<ProfileResultClassification>,
    pub termination: Option<StructuralRunTermination>,
    pub artifact_paths: Vec<String>,
    pub privileged_candidate_fixtures: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedLawV2ProfileRegistryV1 {
    pub schema_version: u16,
    pub registry_id: String,
    pub profiles: Vec<UncheckedLawV2ProfileRegistryEntryV1>,
}

impl CanonicalEncode for UncheckedLawV2ProfileRegistryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        let bytes = serde_json::to_vec(self).expect("profile registry serializes");
        encoder.bytes(&bytes);
    }
}

/// Opaque capability obtained only after every frozen/proposed invariant and
/// issued digest has replayed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedLawV2ProfileRegistryV1 {
    registry: UncheckedLawV2ProfileRegistryV1,
    canonical_digest: Digest,
}

impl VerifiedLawV2ProfileRegistryV1 {
    pub fn registry_id(&self) -> &str {
        &self.registry.registry_id
    }

    pub fn profiles(&self) -> &[UncheckedLawV2ProfileRegistryEntryV1] {
        &self.registry.profiles
    }

    pub fn canonical_digest(&self) -> &Digest {
        &self.canonical_digest
    }
}

/// Parse and validate the embedded registry against the issued Profile-A
/// semantic and result digests.
pub fn embedded_law_v2_profile_registry_v1() -> Result<VerifiedLawV2ProfileRegistryV1, String> {
    let registry: UncheckedLawV2ProfileRegistryV1 =
        serde_json::from_slice(EMBEDDED_LAW_V2_PROFILE_REGISTRY)
            .map_err(|error| format!("profile registry is not valid v1 JSON: {error}"))?;
    verify_law_v2_profile_registry_v1(registry)
}

pub fn verify_law_v2_profile_registry_v1(
    registry: UncheckedLawV2ProfileRegistryV1,
) -> Result<VerifiedLawV2ProfileRegistryV1, String> {
    if registry.schema_version != LAW_V2_PROFILE_REGISTRY_SCHEMA_VERSION {
        return Err("profile registry schema version is not v1".to_owned());
    }
    if registry.registry_id != LAW_V2_PROFILE_REGISTRY_ID {
        return Err("profile registry identity is not the frozen v1 identity".to_owned());
    }
    if registry.profiles.len() != 3 {
        return Err("profile registry must contain exactly the three reviewed entries".to_owned());
    }

    let ids = registry
        .profiles
        .iter()
        .map(|profile| profile.id.as_str())
        .collect::<BTreeSet<_>>();
    if ids.len() != registry.profiles.len() {
        return Err("profile registry identifiers must be unique".to_owned());
    }
    let ordered_ids = registry
        .profiles
        .iter()
        .map(|profile| profile.id.as_str())
        .collect::<Vec<_>>();
    if ordered_ids != [PROFILE_A_ID, CONTEXTUAL_PROFILE_ID, PRODUCTIVE_PROFILE_ID] {
        return Err("profile registry entries are not in canonical reviewed order".to_owned());
    }

    let profile_a = profile(&registry, PROFILE_A_ID)?;
    if profile_a.status != ProfileRegistryStatus::FrozenExecuted
        || profile_a.execution_profile_ids
            != [
                PROFILE_A_ID.to_owned(),
                PROFILE_A_CONTINUATION_ID.to_owned(),
            ]
        || profile_a.semantic_manifest_digests
            != [
                parse_digest(PROFILE_A_BASE_SEMANTIC_DIGEST)?,
                parse_digest(PROFILE_A_CONTINUATION_SEMANTIC_DIGEST)?,
            ]
        || profile_a.result_digests
            != [
                parse_digest(PROFILE_A_H3_RESULT_DIGEST)?,
                parse_digest(PROFILE_A_H4_RESULT_DIGEST)?,
            ]
        || profile_a.result != Some(ProfileResultClassification::DirectEliminatorThenDebtFreeHalt)
        || profile_a.termination != Some(StructuralRunTermination::HaltedDebtFree)
        || profile_a.artifact_paths
            != [
                "docs/law_v2_h3_inductive_completion_v1.json".to_owned(),
                "docs/law_v2_h4_continuation_v1.json".to_owned(),
            ]
    {
        return Err("frozen Profile A does not match its issued evidence".to_owned());
    }
    if !profile_a.privileged_candidate_fixtures.is_empty() {
        return Err("frozen Profile A must not export a privileged candidate fixture".to_owned());
    }

    verify_proposed(profile(&registry, CONTEXTUAL_PROFILE_ID)?)?;
    verify_proposed(profile(&registry, PRODUCTIVE_PROFILE_ID)?)?;
    let canonical_digest = Digest::of_canonical("pen-law/profile-registry/v1", &registry);
    Ok(VerifiedLawV2ProfileRegistryV1 {
        registry,
        canonical_digest,
    })
}

fn profile<'a>(
    registry: &'a UncheckedLawV2ProfileRegistryV1,
    id: &str,
) -> Result<&'a UncheckedLawV2ProfileRegistryEntryV1, String> {
    registry
        .profiles
        .iter()
        .find(|profile| profile.id == id)
        .ok_or_else(|| format!("profile registry is missing {id}"))
}

fn verify_proposed(profile: &UncheckedLawV2ProfileRegistryEntryV1) -> Result<(), String> {
    if profile.status != ProfileRegistryStatus::ProposedNotAdopted
        || profile.execution_profile_ids != [profile.id.clone()]
        || !profile.semantic_manifest_digests.is_empty()
        || !profile.result_digests.is_empty()
        || profile.result.is_some()
        || profile.termination.is_some()
        || !profile.artifact_paths.is_empty()
        || !profile.privileged_candidate_fixtures.is_empty()
    {
        return Err(format!(
            "proposed profile {} carries unissued execution authority",
            profile.id
        ));
    }
    Ok(())
}

fn parse_digest(value: &str) -> Result<Digest, String> {
    Digest::parse(value).map_err(|error| format!("invalid pinned digest {value}: {error}"))
}

#[cfg(test)]
mod tests {
    use super::{
        EMBEDDED_LAW_V2_PROFILE_REGISTRY, ProfileRegistryStatus, StructuralRunTermination,
        UncheckedLawV2ProfileRegistryV1, embedded_law_v2_profile_registry_v1,
        verify_law_v2_profile_registry_v1,
    };

    #[test]
    fn embedded_profile_a_is_frozen_as_a_debt_free_halt() {
        let registry =
            embedded_law_v2_profile_registry_v1().expect("embedded registry must verify");
        let profile_a = &registry.profiles()[0];
        assert_eq!(profile_a.status, ProfileRegistryStatus::FrozenExecuted);
        assert_eq!(
            profile_a.termination,
            Some(StructuralRunTermination::HaltedDebtFree)
        );
        assert!(profile_a.privileged_candidate_fixtures.is_empty());
    }

    #[test]
    fn a_proposed_profile_cannot_carry_live_result_authority() {
        let mut registry: UncheckedLawV2ProfileRegistryV1 =
            serde_json::from_slice(EMBEDDED_LAW_V2_PROFILE_REGISTRY)
                .expect("embedded registry JSON decodes");
        registry.profiles[1].termination = Some(StructuralRunTermination::Unknown);
        assert!(verify_law_v2_profile_registry_v1(registry).is_err());
    }

    #[test]
    fn registry_order_is_part_of_the_verified_identity() {
        let mut registry: UncheckedLawV2ProfileRegistryV1 =
            serde_json::from_slice(EMBEDDED_LAW_V2_PROFILE_REGISTRY)
                .expect("embedded registry JSON decodes");
        registry.profiles.swap(1, 2);
        assert!(verify_law_v2_profile_registry_v1(registry).is_err());
    }
}
