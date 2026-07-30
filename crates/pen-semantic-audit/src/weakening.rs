use crate::manifest::{AuditDecision, AuditUnknownReason, VerifiedSemanticAuditManifestV1};
use crate::model::{ConservativeExtensionV1, FamilyClassIdV1, FamilyClassV1};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeakeningCertificateV1 {
    manifest_candidate_digest: Digest,
    extension_digest: Digest,
    pre_family_set_digest: Digest,
    post_family_set_digest: Digest,
    weakening: Vec<(FamilyClassIdV1, FamilyClassIdV1)>,
    restriction_on_old_support: Vec<(FamilyClassIdV1, FamilyClassIdV1)>,
    image: Vec<FamilyClassIdV1>,
    marginals: Vec<FamilyClassIdV1>,
    support_touch_complete: bool,
    digest: Digest,
}

impl WeakeningCertificateV1 {
    pub fn weakening(&self) -> &[(FamilyClassIdV1, FamilyClassIdV1)] {
        &self.weakening
    }

    pub fn restriction_on_old_support(&self) -> &[(FamilyClassIdV1, FamilyClassIdV1)] {
        &self.restriction_on_old_support
    }

    pub fn image(&self) -> &[FamilyClassIdV1] {
        &self.image
    }

    pub fn marginals(&self) -> &[FamilyClassIdV1] {
        &self.marginals
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for WeakeningCertificateV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_candidate_digest.encode_canonical(encoder);
        self.extension_digest.encode_canonical(encoder);
        self.pre_family_set_digest.encode_canonical(encoder);
        self.post_family_set_digest.encode_canonical(encoder);
        encode_pairs(encoder, &self.weakening);
        encode_pairs(encoder, &self.restriction_on_old_support);
        encoder.sequence(&self.image);
        encoder.sequence(&self.marginals);
        encoder.tag(u8::from(self.support_touch_complete));
    }
}

pub fn verify_weakening_and_marginals_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    extension: &ConservativeExtensionV1,
    pre: &[FamilyClassV1],
    post: &[FamilyClassV1],
) -> AuditDecision<WeakeningCertificateV1> {
    if !extension.exact_and_conservative
        || extension.predecessor_boundary_digest == extension.successor_boundary_digest
    {
        return AuditDecision::Unknown(AuditUnknownReason::FailedRetraction);
    }

    let pre_map = match index_unique(pre) {
        Some(map) => map,
        None => return AuditDecision::Unknown(AuditUnknownReason::MalformedInput),
    };
    let post_map = match index_unique(post) {
        Some(map) => map,
        None => return AuditDecision::Unknown(AuditUnknownReason::MalformedInput),
    };

    let mut weakening = Vec::with_capacity(pre_map.len());
    for (id, old_family) in &pre_map {
        let Some(new_family) = post_map.get(id) else {
            return AuditDecision::Unknown(AuditUnknownReason::FailedRetraction);
        };
        if !same_transportable_family(old_family, new_family) {
            return AuditDecision::Unknown(AuditUnknownReason::FailedRetraction);
        }
        weakening.push(((*id).clone(), (*id).clone()));
    }

    let mut restriction = Vec::new();
    for (id, family) in &post_map {
        if family
            .canonical_support
            .is_old_support(&extension.predecessor_events)
        {
            let Some(old_family) = pre_map.get(id) else {
                return AuditDecision::Unknown(AuditUnknownReason::FailedRetraction);
            };
            if !same_transportable_family(old_family, family) {
                return AuditDecision::Unknown(AuditUnknownReason::FailedRetraction);
            }
            restriction.push(((*id).clone(), (*id).clone()));
        }
    }

    let image = weakening
        .iter()
        .map(|(_, target)| target.clone())
        .collect::<Vec<_>>();
    let image_set = image.iter().cloned().collect::<BTreeSet<_>>();
    let marginals = post_map
        .keys()
        .filter(|id| !image_set.contains(*id))
        .map(|id| (*id).clone())
        .collect::<Vec<_>>();

    for marginal in &marginals {
        let family = post_map
            .get(marginal)
            .expect("marginal identifiers came from post map");
        let touches_event = family
            .canonical_support
            .events
            .contains(&extension.new_event);
        let touches_discharge = !family
            .canonical_support
            .demand_outputs
            .is_disjoint(&extension.discharged_outputs);
        if !touches_event && !touches_discharge {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport);
        }
    }

    let extension_digest =
        Digest::of_canonical("pen-semantic-audit/conservative-extension/v1", extension);
    let pre_family_set_digest = family_set_digest("pre", pre);
    let post_family_set_digest = family_set_digest("post", post);
    let mut certificate = WeakeningCertificateV1 {
        manifest_candidate_digest: manifest.candidate_digest().clone(),
        extension_digest,
        pre_family_set_digest,
        post_family_set_digest,
        weakening,
        restriction_on_old_support: restriction,
        image,
        marginals,
        support_touch_complete: true,
        digest: Digest::of_bytes(b"pending weakening certificate"),
    };
    certificate.digest =
        Digest::of_canonical("pen-semantic-audit/weakening-certificate/v1", &certificate);
    AuditDecision::Proven(certificate)
}

fn index_unique(families: &[FamilyClassV1]) -> Option<BTreeMap<&FamilyClassIdV1, &FamilyClassV1>> {
    let mut result = BTreeMap::new();
    for family in families {
        if result.insert(&family.id, family).is_some() {
            return None;
        }
    }
    Some(result)
}

fn same_transportable_family(left: &FamilyClassV1, right: &FamilyClassV1) -> bool {
    left.id == right.id
        && left.generic_judgment == right.generic_judgment
        && left.role == right.role
        && left.canonical_support == right.canonical_support
        && left.source_clause == right.source_clause
        && left.demand_anchor == right.demand_anchor
        && left.substitution_action_digest == right.substitution_action_digest
}

fn family_set_digest(label: &str, families: &[FamilyClassV1]) -> Digest {
    let mut sorted = families.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| left.id.cmp(&right.id));
    let mut encoder = CanonicalEncoder::new();
    encoder.text(label);
    encoder.u64(sorted.len() as u64);
    for family in sorted {
        family.encode_canonical(&mut encoder);
    }
    Digest::of_domain_bytes("pen-semantic-audit/family-class-set/v1", encoder.as_bytes())
}

fn encode_pairs(encoder: &mut CanonicalEncoder, pairs: &[(FamilyClassIdV1, FamilyClassIdV1)]) {
    encoder.u64(pairs.len() as u64);
    for (left, right) in pairs {
        left.encode_canonical(encoder);
        right.encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::verify_weakening_and_marginals_v1;
    use crate::manifest::{AuditDecision, AuditUnknownReason, verify_core_manifests_v1};
    use crate::model::{
        ClauseIdV1, ConservativeExtensionV1, EventIdV1, FamilyClassIdV1, FamilyClassV1,
        GenericJudgmentV1, LocalRoleV1, PublicSupportV1, RawFamilyIdV1,
    };
    use pen_kernel::{DependentContext, Digest, Term};
    use std::collections::BTreeSet;

    fn id(label: &[u8]) -> Digest {
        Digest::of_domain_bytes("weakening-test", label)
    }

    fn family(label: &[u8], events: &[EventIdV1]) -> FamilyClassV1 {
        FamilyClassV1 {
            id: FamilyClassIdV1(id(label)),
            representative: RawFamilyIdV1(id(&[label, b"-raw"].concat())),
            members: vec![RawFamilyIdV1(id(&[label, b"-raw"].concat()))],
            generic_judgment: GenericJudgmentV1::Term {
                context: DependentContext::default(),
                term: Term::Unit,
                ty: Term::UnitType,
            },
            role: LocalRoleV1::KernelHead,
            canonical_support: PublicSupportV1 {
                events: events.iter().cloned().collect(),
                ..PublicSupportV1::default()
            },
            source_clause: Some(ClauseIdV1(id(&[label, b"-clause"].concat()))),
            demand_anchor: None,
            substitution_action_digest: id(b"substitution-action"),
        }
    }

    fn extension(old: &EventIdV1, new: &EventIdV1) -> ConservativeExtensionV1 {
        ConservativeExtensionV1 {
            predecessor_boundary_digest: id(b"pre"),
            successor_boundary_digest: id(b"post"),
            new_event: new.clone(),
            predecessor_events: BTreeSet::from([old.clone()]),
            discharged_outputs: BTreeSet::new(),
            exact_and_conservative: true,
        }
    }

    #[test]
    fn restriction_is_a_retraction_and_marginals_are_exact_difference() {
        let AuditDecision::Proven((semantic, _)) = verify_core_manifests_v1() else {
            panic!("manifest");
        };
        let old = EventIdV1(id(b"old"));
        let new = EventIdV1(id(b"new"));
        let inherited = family(b"inherited", std::slice::from_ref(&old));
        let marginal = family(b"marginal", std::slice::from_ref(&new));
        let AuditDecision::Proven(certificate) = verify_weakening_and_marginals_v1(
            &semantic,
            &extension(&old, &new),
            std::slice::from_ref(&inherited),
            &[inherited.clone(), marginal.clone()],
        ) else {
            panic!("weakening should verify");
        };
        assert_eq!(
            certificate.restriction_on_old_support(),
            &[(inherited.id.clone(), inherited.id.clone())]
        );
        assert_eq!(certificate.marginals(), std::slice::from_ref(&marginal.id));
    }

    #[test]
    fn outside_image_without_new_support_aborts_instead_of_filtering() {
        let AuditDecision::Proven((semantic, _)) = verify_core_manifests_v1() else {
            panic!("manifest");
        };
        let old = EventIdV1(id(b"old"));
        let new = EventIdV1(id(b"new"));
        let foreign = EventIdV1(id(b"foreign"));
        let inherited = family(b"inherited", std::slice::from_ref(&old));
        let unsupported = family(b"unsupported", &[foreign]);
        assert!(matches!(
            verify_weakening_and_marginals_v1(
                &semantic,
                &extension(&old, &new),
                std::slice::from_ref(&inherited),
                &[inherited.clone(), unsupported],
            ),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport)
        ));
    }
}
