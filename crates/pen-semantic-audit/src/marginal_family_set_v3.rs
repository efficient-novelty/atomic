//! Exact marginal-family set for the V3 semantic-family quotient.
//!
//! Marginality is the class complement of the verified weakening image.
//! The complement is fixed before support is inspected: support is a theorem
//! about every member of the exact set and is never a filter. This stage
//! deliberately mints no cost, demand-orbit, realization, SR2, or selective
//! authority.

use crate::family_quotient_v3::{
    FamilyClassIdV3, NormalizedFamilyIdV3, VerifiedFamilyClassV3, VerifiedFamilyQuotientV3,
    derive_v3_family_ids,
};
use crate::family_weakening_v3::VerifiedFamilyWeakeningV3;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const MARGINAL_FAMILY_SET_SCHEMA_VERSION_V3: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MarginalFamilySetFailureV3 {
    ExactManifestIdentityMismatch,
    ChainBindingMismatch,
    WeakeningImageMismatch,
    QuotientCoverageMismatch,
    RawIdentityMismatch,
    IncompleteSupport,
}

impl std::fmt::Display for MarginalFamilySetFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifest is not the exact registered lambda/unit V3 proposal",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the family quotient and weakening theorem are not one verified chain",
            ),
            Self::WeakeningImageMismatch => formatter.write_str(
                "the weakening image, class map, and restriction do not define one exact image",
            ),
            Self::QuotientCoverageMismatch => formatter.write_str(
                "the quotient classes do not partition the authorized raw-family carrier exactly",
            ),
            Self::RawIdentityMismatch => formatter.write_str(
                "the authorized carrier cannot be reconstructed on the V3 raw-family identity surface",
            ),
            Self::IncompleteSupport => formatter.write_str(
                "a member of the exact marginal complement lacks the required new-event support",
            ),
        }
    }
}

impl std::error::Error for MarginalFamilySetFailureV3 {}

impl MarginalFamilySetFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        AuditDecision::Unknown(AuditUnknownReason::MissingWeakeningMarginalAuthority)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MarginalMemberSupportV3 {
    class: FamilyClassIdV3,
    member: NormalizedFamilyIdV3,
    support_digest: Digest,
}

impl CanonicalEncode for MarginalMemberSupportV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.class.encode_canonical(encoder);
        self.member.encode_canonical(encoder);
        self.support_digest.encode_canonical(encoder);
    }
}

/// Opaque authority for the exact post-quotient class complement. There is
/// no deserialization path and no constructor from caller-provided classes,
/// image IDs, raw members, support claims, or counts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedMarginalFamilySetV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    inventory_digest: Digest,
    exact_extension_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    new_event: crate::model::EventIdV1,
    rewrite_authority_digest: Digest,
    predecessor_reconstruction_digest: Digest,
    successor_quotient_digest: Digest,
    weakening_digest: Digest,
    semantic_set_digest: Digest,
    class_partition_digest: Digest,
    raw_partition_digest: Digest,
    support_touch_digest: Digest,
    successor_class_count: u64,
    image_class_count: u64,
    successor_raw_family_count: u64,
    image_raw_member_count: u64,
    marginal_ids: Arc<[FamilyClassIdV3]>,
    marginals: Arc<[VerifiedFamilyClassV3]>,
    marginal_member_support: Arc<[MarginalMemberSupportV3]>,
    digest: Digest,
}

impl VerifiedMarginalFamilySetV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn exact_extension_digest(&self) -> &Digest {
        &self.exact_extension_digest
    }

    pub fn predecessor_boundary_digest(&self) -> &Digest {
        &self.predecessor_boundary_digest
    }

    pub fn successor_boundary_digest(&self) -> &Digest {
        &self.successor_boundary_digest
    }

    pub fn new_event(&self) -> &crate::model::EventIdV1 {
        &self.new_event
    }

    pub fn rewrite_authority_digest(&self) -> &Digest {
        &self.rewrite_authority_digest
    }

    pub fn predecessor_reconstruction_digest(&self) -> &Digest {
        &self.predecessor_reconstruction_digest
    }

    pub fn successor_quotient_digest(&self) -> &Digest {
        &self.successor_quotient_digest
    }

    pub fn weakening_digest(&self) -> &Digest {
        &self.weakening_digest
    }

    pub fn semantic_set_digest(&self) -> &Digest {
        &self.semantic_set_digest
    }

    pub fn successor_class_count(&self) -> usize {
        self.successor_class_count as usize
    }

    pub fn image_class_count(&self) -> usize {
        self.image_class_count as usize
    }

    pub fn successor_raw_family_count(&self) -> usize {
        self.successor_raw_family_count as usize
    }

    pub fn image_raw_member_count(&self) -> usize {
        self.image_raw_member_count as usize
    }

    pub fn marginal_ids(&self) -> &[FamilyClassIdV3] {
        &self.marginal_ids
    }

    pub fn marginals(&self) -> &[VerifiedFamilyClassV3] {
        &self.marginals
    }

    /// Structural cardinality of the exact marginal set. This accessor does
    /// not issue a final value register before demand/SR2 provenance exists.
    pub fn marginal_count(&self) -> usize {
        self.marginals.len()
    }

    pub fn marginal_raw_member_count(&self) -> usize {
        self.marginal_member_support.len()
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedMarginalFamilySetV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.exact_extension_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.new_event.encode_canonical(encoder);
        self.rewrite_authority_digest.encode_canonical(encoder);
        self.predecessor_reconstruction_digest
            .encode_canonical(encoder);
        self.successor_quotient_digest.encode_canonical(encoder);
        self.weakening_digest.encode_canonical(encoder);
        self.semantic_set_digest.encode_canonical(encoder);
        self.class_partition_digest.encode_canonical(encoder);
        self.raw_partition_digest.encode_canonical(encoder);
        self.support_touch_digest.encode_canonical(encoder);
        encoder.u64(self.successor_class_count);
        encoder.u64(self.image_class_count);
        encoder.u64(self.successor_raw_family_count);
        encoder.u64(self.image_raw_member_count);
        encoder.sequence(&self.marginal_ids);
        encoder.sequence(&self.marginals);
        encoder.sequence(&self.marginal_member_support);
    }
}

/// Derive the exact marginal-family complement from one verified quotient /
/// weakening chain. The complement is computed before support validation.
pub fn diagnose_marginal_family_set_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    successor_quotient: &VerifiedFamilyQuotientV3,
    weakening: &VerifiedFamilyWeakeningV3,
) -> Result<VerifiedMarginalFamilySetV3, MarginalFamilySetFailureV3> {
    if v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3() {
        return Err(MarginalFamilySetFailureV3::ExactManifestIdentityMismatch);
    }
    if successor_quotient.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || weakening.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || weakening.successor_quotient_digest() != successor_quotient.digest()
        || weakening.inventory_digest() != successor_quotient.inventory_digest()
        || weakening.successor_boundary_digest() != successor_quotient.signature_digest()
        || weakening.rewrite_authority_digest() != successor_quotient.rewrite_authority_digest()
    {
        return Err(MarginalFamilySetFailureV3::ChainBindingMismatch);
    }

    let class_index = index_successor_classes(successor_quotient.classes())?;
    let all_class_ids = class_index.keys().cloned().collect::<BTreeSet<_>>();
    let image = verify_exact_weakening_image(weakening, &all_class_ids)?;
    let marginal_ids = exact_complement(&all_class_ids, &image)
        .ok_or(MarginalFamilySetFailureV3::WeakeningImageMismatch)?;
    let marginal_id_set = marginal_ids.iter().cloned().collect::<BTreeSet<_>>();

    let raw_by_v3 = reconstruct_raw_identity(v3_manifest, successor_quotient)?;
    let class_members = verify_exact_class_coverage(&class_index, &raw_by_v3)?;
    let (image_members, marginal_members) =
        partition_members(&class_members, &image, &marginal_id_set)
            .ok_or(MarginalFamilySetFailureV3::QuotientCoverageMismatch)?;
    if image_members.len() + marginal_members.len() != raw_by_v3.len() {
        return Err(MarginalFamilySetFailureV3::QuotientCoverageMismatch);
    }
    verify_old_raw_targets(weakening, &class_members, &image, &image_members)?;

    let mut marginals = Vec::with_capacity(marginal_ids.len());
    let mut support_witnesses = Vec::with_capacity(marginal_members.len());
    for id in &marginal_ids {
        let class = class_index
            .get(id)
            .copied()
            .ok_or(MarginalFamilySetFailureV3::QuotientCoverageMismatch)?;
        for member in class.members() {
            let raw = raw_by_v3
                .get(member)
                .copied()
                .ok_or(MarginalFamilySetFailureV3::RawIdentityMismatch)?;
            if support_touches_new_event(&raw.public_support, weakening.new_event()) {
                support_witnesses.push(MarginalMemberSupportV3 {
                    class: id.clone(),
                    member: member.clone(),
                    support_digest: Digest::of_canonical(
                        "pen-semantic-audit/marginal-member-public-support/v3",
                        &raw.public_support,
                    ),
                });
            } else {
                return Err(MarginalFamilySetFailureV3::IncompleteSupport);
            }
        }
        let representative = raw_by_v3
            .get(class.representative())
            .copied()
            .ok_or(MarginalFamilySetFailureV3::RawIdentityMismatch)?;
        if representative.public_support != *class.canonical_support()
            || !class
                .canonical_support()
                .events
                .contains(weakening.new_event())
            || !class.canonical_support().demand_outputs.is_empty()
        {
            return Err(MarginalFamilySetFailureV3::IncompleteSupport);
        }
        marginals.push(class.clone());
    }
    support_witnesses
        .sort_by(|left, right| (&left.class, &left.member).cmp(&(&right.class, &right.member)));
    if support_witnesses.len() != marginal_members.len()
        || support_witnesses
            .iter()
            .map(|witness| witness.member.clone())
            .collect::<BTreeSet<_>>()
            != marginal_members
    {
        return Err(MarginalFamilySetFailureV3::IncompleteSupport);
    }

    let semantic_set_digest = Digest::of_canonical(
        "pen-semantic-audit/exact-marginal-family-set/v3",
        &CanonicalSequence(&marginal_ids),
    );
    let class_partition_digest = Digest::of_canonical(
        "pen-semantic-audit/marginal-class-partition/v3",
        &ClassPartitionMaterial {
            all: &all_class_ids,
            image: &image,
            marginal: &marginal_id_set,
        },
    );
    let raw_partition_digest = Digest::of_canonical(
        "pen-semantic-audit/marginal-raw-member-partition/v3",
        &RawPartitionMaterial {
            image: &image_members,
            marginal: &marginal_members,
        },
    );
    let support_touch_digest = Digest::of_canonical(
        "pen-semantic-audit/marginal-member-support-touch/v3",
        &CanonicalSequence(&support_witnesses),
    );

    let mut verified = VerifiedMarginalFamilySetV3 {
        schema_version: MARGINAL_FAMILY_SET_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        inventory_digest: weakening.inventory_digest().clone(),
        exact_extension_digest: weakening.exact_extension_digest().clone(),
        predecessor_boundary_digest: weakening.predecessor_boundary_digest().clone(),
        successor_boundary_digest: weakening.successor_boundary_digest().clone(),
        new_event: weakening.new_event().clone(),
        rewrite_authority_digest: weakening.rewrite_authority_digest().clone(),
        predecessor_reconstruction_digest: weakening.predecessor_reconstruction_digest().clone(),
        successor_quotient_digest: successor_quotient.digest().clone(),
        weakening_digest: weakening.digest().clone(),
        semantic_set_digest,
        class_partition_digest,
        raw_partition_digest,
        support_touch_digest,
        successor_class_count: all_class_ids.len() as u64,
        image_class_count: image.len() as u64,
        successor_raw_family_count: raw_by_v3.len() as u64,
        image_raw_member_count: image_members.len() as u64,
        marginal_ids: Arc::from(marginal_ids.into_boxed_slice()),
        marginals: Arc::from(marginals.into_boxed_slice()),
        marginal_member_support: Arc::from(support_witnesses.into_boxed_slice()),
        digest: Digest::of_bytes(b"pending verified marginal family set v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-marginal-family-set/v3",
        &verified,
    );
    Ok(verified)
}

pub fn verify_marginal_family_set_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    successor_quotient: &VerifiedFamilyQuotientV3,
    weakening: &VerifiedFamilyWeakeningV3,
) -> AuditDecision<VerifiedMarginalFamilySetV3> {
    match diagnose_marginal_family_set_v3(v3_manifest, successor_quotient, weakening) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

fn index_successor_classes(
    classes: &[VerifiedFamilyClassV3],
) -> Result<BTreeMap<FamilyClassIdV3, &VerifiedFamilyClassV3>, MarginalFamilySetFailureV3> {
    let mut index = BTreeMap::new();
    for class in classes {
        if index.insert(class.id().clone(), class).is_some() {
            return Err(MarginalFamilySetFailureV3::QuotientCoverageMismatch);
        }
    }
    Ok(index)
}

fn verify_exact_weakening_image(
    weakening: &VerifiedFamilyWeakeningV3,
    successor_classes: &BTreeSet<FamilyClassIdV3>,
) -> Result<BTreeSet<FamilyClassIdV3>, MarginalFamilySetFailureV3> {
    let image = weakening.image().iter().cloned().collect::<BTreeSet<_>>();
    let predecessor_classes = weakening
        .predecessor_classes()
        .iter()
        .map(|class| class.id().clone())
        .collect::<BTreeSet<_>>();
    let mut sources = BTreeSet::new();
    let mut targets = BTreeSet::new();
    for entry in weakening.weakening() {
        if !sources.insert(entry.source().clone()) || !targets.insert(entry.target().clone()) {
            return Err(MarginalFamilySetFailureV3::WeakeningImageMismatch);
        }
    }
    let mut restriction_sources = BTreeSet::new();
    let mut restriction_targets = BTreeSet::new();
    let restriction = weakening
        .restriction_on_image()
        .iter()
        .map(|entry| {
            restriction_sources.insert(entry.source().clone());
            restriction_targets.insert(entry.target().clone());
            (entry.source().clone(), entry.target().clone())
        })
        .collect::<BTreeMap<_, _>>();
    if image.len() != weakening.image().len()
        || restriction.len() != weakening.restriction_on_image().len()
        || restriction_targets.len() != restriction.len()
        || image != targets
        || image != restriction_sources
        || !image.is_subset(successor_classes)
        || predecessor_classes.len() != weakening.predecessor_classes().len()
        || sources != predecessor_classes
        || weakening
            .weakening()
            .iter()
            .any(|entry| restriction.get(entry.target()) != Some(entry.source()))
    {
        return Err(MarginalFamilySetFailureV3::WeakeningImageMismatch);
    }
    Ok(image)
}

fn reconstruct_raw_identity<'a>(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    quotient: &'a VerifiedFamilyQuotientV3,
) -> Result<BTreeMap<NormalizedFamilyIdV3, &'a crate::model::RawFamilyV1>, MarginalFamilySetFailureV3>
{
    let carrier = quotient.authorized_carrier_proof();
    let raw_ids = derive_v3_family_ids(v3_manifest, carrier, quotient.seed_images())
        .map_err(|_| MarginalFamilySetFailureV3::RawIdentityMismatch)?;
    let legacy = carrier
        .raw_families()
        .iter()
        .map(|family| (family.id.clone(), family))
        .collect::<BTreeMap<_, _>>();
    if legacy.len() != carrier.raw_families().len() || raw_ids.len() != legacy.len() {
        return Err(MarginalFamilySetFailureV3::RawIdentityMismatch);
    }
    let mut result = BTreeMap::new();
    for (legacy_id, v3_id) in raw_ids {
        let family = legacy
            .get(&legacy_id)
            .copied()
            .ok_or(MarginalFamilySetFailureV3::RawIdentityMismatch)?;
        if result.insert(v3_id, family).is_some() {
            return Err(MarginalFamilySetFailureV3::RawIdentityMismatch);
        }
    }
    Ok(result)
}

fn verify_exact_class_coverage(
    classes: &BTreeMap<FamilyClassIdV3, &VerifiedFamilyClassV3>,
    raw: &BTreeMap<NormalizedFamilyIdV3, &crate::model::RawFamilyV1>,
) -> Result<BTreeMap<FamilyClassIdV3, BTreeSet<NormalizedFamilyIdV3>>, MarginalFamilySetFailureV3> {
    let mut result = BTreeMap::new();
    let mut covered = BTreeSet::new();
    for (id, class) in classes {
        let members = class.members().iter().cloned().collect::<BTreeSet<_>>();
        if members.is_empty()
            || members.len() != class.members().len()
            || !members.contains(class.representative())
            || members.iter().any(|member| !raw.contains_key(member))
            || members.iter().any(|member| !covered.insert(member.clone()))
        {
            return Err(MarginalFamilySetFailureV3::QuotientCoverageMismatch);
        }
        result.insert(id.clone(), members);
    }
    if covered != raw.keys().cloned().collect::<BTreeSet<_>>() {
        return Err(MarginalFamilySetFailureV3::QuotientCoverageMismatch);
    }
    Ok(result)
}

fn verify_old_raw_targets(
    weakening: &VerifiedFamilyWeakeningV3,
    class_members: &BTreeMap<FamilyClassIdV3, BTreeSet<NormalizedFamilyIdV3>>,
    image: &BTreeSet<FamilyClassIdV3>,
    image_members: &BTreeSet<NormalizedFamilyIdV3>,
) -> Result<(), MarginalFamilySetFailureV3> {
    let mut predecessor_owner = BTreeMap::new();
    let mut predecessor_classes = BTreeSet::new();
    for class in weakening.predecessor_classes() {
        if !predecessor_classes.insert(class.id().clone())
            || class.members().is_empty()
            || !class.members().contains(class.representative())
        {
            return Err(MarginalFamilySetFailureV3::WeakeningImageMismatch);
        }
        for member in class.members() {
            if predecessor_owner
                .insert(member.clone(), class.id().clone())
                .is_some()
            {
                return Err(MarginalFamilySetFailureV3::WeakeningImageMismatch);
            }
        }
    }
    let class_map = weakening
        .weakening()
        .iter()
        .map(|entry| (entry.source().clone(), entry.target().clone()))
        .collect::<BTreeMap<_, _>>();
    if class_map.len() != weakening.weakening().len()
        || class_map.keys().cloned().collect::<BTreeSet<_>>() != predecessor_classes
    {
        return Err(MarginalFamilySetFailureV3::WeakeningImageMismatch);
    }
    let successor_owner = class_members
        .iter()
        .flat_map(|(class, members)| {
            members
                .iter()
                .cloned()
                .map(|member| (member, class.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    if successor_owner.len() != class_members.values().map(BTreeSet::len).sum::<usize>() {
        return Err(MarginalFamilySetFailureV3::QuotientCoverageMismatch);
    }
    let mut sources = BTreeSet::new();
    let mut targets = BTreeSet::new();
    for entry in weakening.raw_weakening() {
        let predecessor_class = predecessor_owner
            .get(entry.source())
            .ok_or(MarginalFamilySetFailureV3::WeakeningImageMismatch)?;
        let expected_successor_class = class_map
            .get(predecessor_class)
            .ok_or(MarginalFamilySetFailureV3::WeakeningImageMismatch)?;
        if entry.source() != entry.target()
            || successor_owner.get(entry.target()) != Some(expected_successor_class)
            || !sources.insert(entry.source().clone())
            || !targets.insert(entry.target().clone())
        {
            return Err(MarginalFamilySetFailureV3::WeakeningImageMismatch);
        }
    }
    if sources != predecessor_owner.keys().cloned().collect::<BTreeSet<_>>()
        || !targets.is_subset(image_members)
        || image.iter().any(|class| {
            class_members
                .get(class)
                .is_none_or(|members| members.is_disjoint(&targets))
        })
    {
        return Err(MarginalFamilySetFailureV3::WeakeningImageMismatch);
    }
    Ok(())
}

fn exact_complement<T: Clone + Ord>(universe: &BTreeSet<T>, image: &BTreeSet<T>) -> Option<Vec<T>> {
    image
        .is_subset(universe)
        .then(|| universe.difference(image).cloned().collect())
}

fn support_touches_new_event(
    support: &crate::model::PublicSupportV1,
    new_event: &crate::model::EventIdV1,
) -> bool {
    support.events.contains(new_event) && support.demand_outputs.is_empty()
}

fn partition_members<K: Ord, V: Clone + Ord>(
    classes: &BTreeMap<K, BTreeSet<V>>,
    image: &BTreeSet<K>,
    marginal: &BTreeSet<K>,
) -> Option<(BTreeSet<V>, BTreeSet<V>)> {
    if !image.is_disjoint(marginal)
        || image.union(marginal).count() != classes.len()
        || image.iter().any(|id| !classes.contains_key(id))
        || marginal.iter().any(|id| !classes.contains_key(id))
    {
        return None;
    }
    let image_members = image
        .iter()
        .flat_map(|id| classes[id].iter().cloned())
        .collect::<BTreeSet<_>>();
    let marginal_members = marginal
        .iter()
        .flat_map(|id| classes[id].iter().cloned())
        .collect::<BTreeSet<_>>();
    image_members
        .is_disjoint(&marginal_members)
        .then_some((image_members, marginal_members))
}

struct CanonicalSequence<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSequence<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct ClassPartitionMaterial<'a> {
    all: &'a BTreeSet<FamilyClassIdV3>,
    image: &'a BTreeSet<FamilyClassIdV3>,
    marginal: &'a BTreeSet<FamilyClassIdV3>,
}

impl CanonicalEncode for ClassPartitionMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encode_set(encoder, self.all);
        encode_set(encoder, self.image);
        encode_set(encoder, self.marginal);
    }
}

struct RawPartitionMaterial<'a> {
    image: &'a BTreeSet<NormalizedFamilyIdV3>,
    marginal: &'a BTreeSet<NormalizedFamilyIdV3>,
}

impl CanonicalEncode for RawPartitionMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encode_set(encoder, self.image);
        encode_set(encoder, self.marginal);
    }
}

fn encode_set<T: CanonicalEncode>(encoder: &mut CanonicalEncoder, set: &BTreeSet<T>) {
    encoder.u64(set.len() as u64);
    for value in set {
        value.encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::{exact_complement, partition_members, support_touches_new_event};
    use crate::model::{EventIdV1, PublicSupportV1};
    use pen_kernel::Digest;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn complement_is_exact_and_empty_is_lawful() {
        let all = BTreeSet::from([1, 2, 3]);
        assert_eq!(
            exact_complement(&all, &BTreeSet::from([1])),
            Some(vec![2, 3])
        );
        assert_eq!(exact_complement(&all, &all), Some(Vec::new()));
        assert_eq!(exact_complement(&all, &BTreeSet::from([4])), None);
    }

    #[test]
    fn new_alias_inside_an_image_class_is_not_marginal() {
        let classes = BTreeMap::from([(10, BTreeSet::from([1, 2])), (20, BTreeSet::from([3, 4]))]);
        let (image, marginal) =
            partition_members(&classes, &BTreeSet::from([10]), &BTreeSet::from([20]))
                .expect("exact class partition");
        assert_eq!(image, BTreeSet::from([1, 2]));
        assert_eq!(marginal, BTreeSet::from([3, 4]));
        assert!(!marginal.contains(&2));
    }

    #[test]
    fn incomplete_or_overlapping_class_partition_is_rejected() {
        let classes = BTreeMap::from([(10, BTreeSet::from([1, 2])), (20, BTreeSet::from([3, 4]))]);
        assert!(partition_members(&classes, &BTreeSet::from([10]), &BTreeSet::new()).is_none());
        assert!(
            partition_members(&classes, &BTreeSet::from([10]), &BTreeSet::from([10, 20]),)
                .is_none()
        );
    }

    #[test]
    fn support_is_checked_for_every_marginal_member_not_only_a_representative() {
        let new_event = EventIdV1(Digest::of_bytes(b"marginal-support-test/new-event"));
        let representative = PublicSupportV1 {
            events: BTreeSet::from([new_event.clone()]),
            ..PublicSupportV1::default()
        };
        let unsupported_member = PublicSupportV1::default();
        assert!(support_touches_new_event(&representative, &new_event));
        assert!(!support_touches_new_event(&unsupported_member, &new_event));
        assert!(
            ![representative, unsupported_member]
                .iter()
                .all(|support| support_touches_new_event(support, &new_event))
        );
    }
}
