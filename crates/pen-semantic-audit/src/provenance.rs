use crate::carrier::{CarrierCertificateV1, VerifiedSemanticSeedV1};
use crate::manifest::{AuditDecision, AuditUnknownReason, VerifiedSemanticAuditManifestV1};
use crate::model::{
    ClauseIdV1, DemandOrbitIdV1, DemandOutputIdV1, FamilyClassIdV1, FamilyClassV1,
    FamilyConstructorV1, LocalRoleV1, ProvenanceTagV1, RawFamilyIdV1, RawFamilyV1, SeedIdV1,
    SemanticSchemaSeedV1,
};
use crate::quotient::QuotientCertificateV1;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::collections::{BTreeMap, BTreeSet};

/// Exhaustive candidate-local support for one marginal family.
///
/// This is deliberately separate from `FamilyClassV1::source_clause`: a
/// quotient representative can depend on more than one candidate-local
/// clause. The verifier considers the complete eligible set and refuses to
/// pick one by iteration order or digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FamilyDependencySupportV1 {
    pub family: FamilyClassIdV1,
    pub candidate_local_clauses: BTreeSet<ClauseIdV1>,
    pub preexisting_demand_outputs: BTreeSet<(DemandOrbitIdV1, DemandOutputIdV1)>,
}

impl CanonicalEncode for FamilyDependencySupportV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.family.encode_canonical(encoder);
        encoder.u64(self.candidate_local_clauses.len() as u64);
        for clause in &self.candidate_local_clauses {
            clause.encode_canonical(encoder);
        }
        encoder.u64(self.preexisting_demand_outputs.len() as u64);
        for (orbit, output) in &self.preexisting_demand_outputs {
            orbit.encode_canonical(encoder);
            output.encode_canonical(encoder);
        }
    }
}

/// Reconstruct exhaustive clause/output support from the certified raw
/// derivation DAG. Quotient representative convenience fields are checked,
/// but never treated as exhaustive dependency evidence.
pub fn derive_family_dependency_support_v1(
    carrier: &CarrierCertificateV1,
    quotient: &QuotientCertificateV1,
    marginals: &[FamilyClassV1],
) -> AuditDecision<Vec<FamilyDependencySupportV1>> {
    if carrier.manifest_digest != quotient.manifest_digest
        || carrier.signature_digest != quotient.signature_digest
        || carrier.normalizer_protocol_digest != quotient.normalizer_protocol_digest
        || carrier.fresh_program_digest != quotient.fresh_program_digest
        || !carrier.q3_registry_verified_empty
        || !quotient.q3_registry_verified_empty
        || !quotient.q3_edges.is_empty()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }

    let raw_index = match unique_raw_index(&carrier.raw_families) {
        Some(index) => index,
        None => return AuditDecision::Unknown(AuditUnknownReason::MalformedInput),
    };
    let seed_index = match unique_seed_index(&carrier.verified_seeds) {
        Some(index) => index,
        None => return AuditDecision::Unknown(AuditUnknownReason::MalformedInput),
    };
    let class_index = match unique_class_index(&quotient.classes) {
        Some(index) => index,
        None => return AuditDecision::Unknown(AuditUnknownReason::MalformedInput),
    };

    let fixed = quotient
        .fixed_vertices
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if fixed.len() != quotient.fixed_vertices.len()
        || fixed != raw_index.keys().cloned().collect()
        || quotient.raw_to_class.len() != raw_index.len()
    {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }
    let mut assigned = BTreeMap::new();
    for assignment in &quotient.raw_to_class {
        let Some(class) = class_index.get(&assignment.class) else {
            return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
        };
        if !raw_index.contains_key(&assignment.raw)
            || !class.members.contains(&assignment.raw)
            || assigned
                .insert(assignment.raw.clone(), assignment.class.clone())
                .is_some()
        {
            return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
        }
    }

    let mut marginal_ids = BTreeSet::new();
    let mut memo = BTreeMap::new();
    let mut derived = Vec::with_capacity(marginals.len());
    for marginal in marginals {
        if !marginal_ids.insert(marginal.id.clone()) {
            return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
        }
        let Some(certified_class) = class_index.get(&marginal.id) else {
            return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
        };
        if *certified_class != marginal {
            return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
        }

        let mut clauses = BTreeSet::new();
        let mut outputs = BTreeSet::new();
        for member in &certified_class.members {
            if assigned.get(member) != Some(&certified_class.id) {
                return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
            }
            let support = match derive_raw_support(
                member,
                &raw_index,
                &seed_index,
                &mut memo,
                &mut BTreeSet::new(),
            ) {
                Ok(support) => support,
                Err(reason) => return AuditDecision::Unknown(reason),
            };
            let raw = raw_index
                .get(member)
                .expect("derivation support checked the raw member");
            if raw.role != certified_class.role {
                return AuditDecision::Unknown(AuditUnknownReason::RoleMismatch);
            }
            clauses.extend(support.clauses);
            outputs.extend(support.outputs);
        }
        if certified_class
            .source_clause
            .as_ref()
            .is_some_and(|clause| !clauses.contains(clause))
            || certified_class
                .demand_anchor
                .as_ref()
                .is_some_and(|anchor| !outputs.contains(anchor))
        {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport);
        }
        derived.push(FamilyDependencySupportV1 {
            family: certified_class.id.clone(),
            candidate_local_clauses: clauses,
            preexisting_demand_outputs: outputs,
        });
    }
    derived.sort_by(|left, right| left.family.cmp(&right.family));
    AuditDecision::Proven(derived)
}

#[derive(Clone)]
struct DerivedRawSupport {
    clauses: BTreeSet<ClauseIdV1>,
    outputs: BTreeSet<(DemandOrbitIdV1, DemandOutputIdV1)>,
}

fn derive_raw_support(
    id: &RawFamilyIdV1,
    raw_index: &BTreeMap<RawFamilyIdV1, &RawFamilyV1>,
    seed_index: &BTreeMap<SeedIdV1, &VerifiedSemanticSeedV1>,
    memo: &mut BTreeMap<RawFamilyIdV1, DerivedRawSupport>,
    visiting: &mut BTreeSet<RawFamilyIdV1>,
) -> Result<DerivedRawSupport, AuditUnknownReason> {
    if let Some(support) = memo.get(id) {
        return Ok(support.clone());
    }
    let raw = raw_index
        .get(id)
        .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
    if raw.rank > 2 || !visiting.insert(id.clone()) {
        return Err(AuditUnknownReason::MalformedInput);
    }

    let (expected_role, mut support) = match &raw.constructor {
        FamilyConstructorV1::PublicHeadSeed { seed }
        | FamilyConstructorV1::PublicEquationSeed { seed } => {
            if raw.rank != 0 {
                return Err(AuditUnknownReason::MalformedInput);
            }
            let verified = seed_index
                .get(seed)
                .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
            let expected_constructor = match &verified.seed {
                SemanticSchemaSeedV1::PublicHead(_) => {
                    FamilyConstructorV1::PublicHeadSeed { seed: seed.clone() }
                }
                SemanticSchemaSeedV1::PublicEquation(_) => {
                    FamilyConstructorV1::PublicEquationSeed { seed: seed.clone() }
                }
                SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
                    return Err(AuditUnknownReason::MalformedInput);
                }
            };
            if raw.constructor != expected_constructor {
                return Err(AuditUnknownReason::MalformedInput);
            }
            let mut clauses = BTreeSet::new();
            let mut outputs = BTreeSet::new();
            match &verified.seed {
                SemanticSchemaSeedV1::PublicHead(seed) => {
                    clauses.extend(seed.source_clause.iter().cloned());
                }
                SemanticSchemaSeedV1::PublicEquation(seed) => {
                    clauses.extend(seed.source_clause.iter().cloned());
                    outputs.extend(seed.demand_anchor.iter().cloned());
                }
                SemanticSchemaSeedV1::PublicUniversalInterface { .. } => unreachable!(),
            }
            (
                verified.derived_role,
                DerivedRawSupport { clauses, outputs },
            )
        }
        FamilyConstructorV1::GenericPublicApplication {
            function, argument, ..
        } => {
            let function_support =
                derive_child_support(raw, function, raw_index, seed_index, memo, visiting)?;
            let argument_support =
                derive_child_support(raw, argument, raw_index, seed_index, memo, visiting)?;
            (
                LocalRoleV1::SupportAction,
                union_raw_support(function_support, argument_support),
            )
        }
        FamilyConstructorV1::GenericEquationAction {
            equation, context, ..
        } => {
            let equation_support =
                derive_child_support(raw, equation, raw_index, seed_index, memo, visiting)?;
            let context_support =
                derive_child_support(raw, context, raw_index, seed_index, memo, visiting)?;
            (
                LocalRoleV1::Coherence,
                union_raw_support(equation_support, context_support),
            )
        }
    };
    visiting.remove(id);
    if raw.role != expected_role {
        return Err(AuditUnknownReason::RoleMismatch);
    }
    if raw
        .source_clause
        .as_ref()
        .is_some_and(|clause| !support.clauses.contains(clause))
        || raw
            .demand_anchor
            .as_ref()
            .is_some_and(|anchor| !support.outputs.contains(anchor))
    {
        return Err(AuditUnknownReason::IncompleteSupport);
    }
    support.clauses.extend(raw.source_clause.iter().cloned());
    support.outputs.extend(raw.demand_anchor.iter().cloned());
    memo.insert(id.clone(), support.clone());
    Ok(support)
}

fn derive_child_support(
    parent: &RawFamilyV1,
    child_id: &RawFamilyIdV1,
    raw_index: &BTreeMap<RawFamilyIdV1, &RawFamilyV1>,
    seed_index: &BTreeMap<SeedIdV1, &VerifiedSemanticSeedV1>,
    memo: &mut BTreeMap<RawFamilyIdV1, DerivedRawSupport>,
    visiting: &mut BTreeSet<RawFamilyIdV1>,
) -> Result<DerivedRawSupport, AuditUnknownReason> {
    let child = raw_index
        .get(child_id)
        .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
    if child.rank >= parent.rank {
        return Err(AuditUnknownReason::MalformedInput);
    }
    derive_raw_support(child_id, raw_index, seed_index, memo, visiting)
}

fn union_raw_support(mut left: DerivedRawSupport, right: DerivedRawSupport) -> DerivedRawSupport {
    left.clauses.extend(right.clauses);
    left.outputs.extend(right.outputs);
    left
}

fn unique_raw_index(raw: &[RawFamilyV1]) -> Option<BTreeMap<RawFamilyIdV1, &RawFamilyV1>> {
    let mut index = BTreeMap::new();
    for family in raw {
        if index.insert(family.id.clone(), family).is_some() {
            return None;
        }
    }
    Some(index)
}

fn unique_seed_index(
    seeds: &[VerifiedSemanticSeedV1],
) -> Option<BTreeMap<SeedIdV1, &VerifiedSemanticSeedV1>> {
    let mut index = BTreeMap::new();
    for seed in seeds {
        if index.insert(seed.id.clone(), seed).is_some() {
            return None;
        }
    }
    Some(index)
}

fn unique_class_index(
    classes: &[FamilyClassV1],
) -> Option<BTreeMap<FamilyClassIdV1, &FamilyClassV1>> {
    let mut index = BTreeMap::new();
    for class in classes {
        if index.insert(class.id.clone(), class).is_some() {
            return None;
        }
    }
    Some(index)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceCertificateV1 {
    manifest_candidate_digest: Digest,
    marginal_set_digest: Digest,
    first_irreducible_basis_digest: Digest,
    dependency_support_digest: Digest,
    injection: Vec<(FamilyClassIdV1, ProvenanceTagV1)>,
    digest: Digest,
}

impl ProvenanceCertificateV1 {
    pub fn injection(&self) -> &[(FamilyClassIdV1, ProvenanceTagV1)] {
        &self.injection
    }

    pub fn nu(&self) -> usize {
        self.injection.len()
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for ProvenanceCertificateV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_candidate_digest.encode_canonical(encoder);
        self.marginal_set_digest.encode_canonical(encoder);
        self.first_irreducible_basis_digest
            .encode_canonical(encoder);
        self.dependency_support_digest.encode_canonical(encoder);
        encoder.u64(self.injection.len() as u64);
        for (family, tag) in &self.injection {
            family.encode_canonical(encoder);
            tag.encode_canonical(encoder);
        }
    }
}

pub fn verify_sr2_injection_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    marginals: &[FamilyClassV1],
    first_irreducible_basis: &BTreeSet<ClauseIdV1>,
    preexisting_live_outputs: &BTreeSet<(DemandOrbitIdV1, DemandOutputIdV1)>,
) -> AuditDecision<ProvenanceCertificateV1> {
    let support = marginals
        .iter()
        .map(|family| FamilyDependencySupportV1 {
            family: family.id.clone(),
            candidate_local_clauses: family.source_clause.iter().cloned().collect(),
            preexisting_demand_outputs: family.demand_anchor.iter().cloned().collect(),
        })
        .collect::<Vec<_>>();
    verify_sr2_injection_with_support_v1(
        manifest,
        marginals,
        &support,
        first_irreducible_basis,
        preexisting_live_outputs,
    )
}

/// Verify SR2 from exhaustive dependency support.
///
/// A clause anchor is accepted only when the complete support intersects the
/// first-irreducible basis in exactly one clause. With no clause anchor, the
/// complete demand support must contain exactly one registered live output.
/// Multiple eligible anchors are `AmbiguousProvenance`; the verifier never
/// resolves them by hashing, traversal order, or a caller-selected principal.
pub fn verify_sr2_injection_with_support_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    marginals: &[FamilyClassV1],
    dependency_support: &[FamilyDependencySupportV1],
    first_irreducible_basis: &BTreeSet<ClauseIdV1>,
    preexisting_live_outputs: &BTreeSet<(DemandOrbitIdV1, DemandOutputIdV1)>,
) -> AuditDecision<ProvenanceCertificateV1> {
    let mut families = BTreeMap::new();
    for family in marginals {
        if families.insert(family.id.clone(), family).is_some() {
            return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
        }
    }

    let mut support_by_family = BTreeMap::new();
    for support in dependency_support {
        if !families.contains_key(&support.family)
            || support_by_family
                .insert(support.family.clone(), support)
                .is_some()
        {
            return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
        }
    }
    if support_by_family.len() != families.len() {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport);
    }

    let mut by_family = BTreeMap::new();
    let mut by_tag = BTreeMap::new();

    for (family_id, family) in &families {
        let support = support_by_family
            .get(family_id)
            .expect("support totality checked above");
        if family
            .source_clause
            .as_ref()
            .is_some_and(|clause| !support.candidate_local_clauses.contains(clause))
            || family
                .demand_anchor
                .as_ref()
                .is_some_and(|anchor| !support.preexisting_demand_outputs.contains(anchor))
        {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport);
        }

        let eligible_clauses = support
            .candidate_local_clauses
            .intersection(first_irreducible_basis)
            .cloned()
            .collect::<Vec<_>>();
        let tag = if eligible_clauses.len() == 1 {
            ProvenanceTagV1::ClauseRole {
                clause: eligible_clauses[0].clone(),
                role: family.role,
            }
        } else if eligible_clauses.len() > 1 {
            return AuditDecision::Unknown(AuditUnknownReason::AmbiguousProvenance);
        } else {
            if !support
                .preexisting_demand_outputs
                .is_subset(preexisting_live_outputs)
            {
                return AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport);
            }
            if support.preexisting_demand_outputs.len() > 1 {
                return AuditDecision::Unknown(AuditUnknownReason::AmbiguousProvenance);
            }
            let Some((orbit, output)) = support.preexisting_demand_outputs.iter().next() else {
                return AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport);
            };
            ProvenanceTagV1::DemandOutput {
                orbit: orbit.clone(),
                output: output.clone(),
            }
        };

        if by_tag.insert(tag.clone(), family_id.clone()).is_some() {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        by_family.insert(family_id.clone(), tag);
    }

    let injection = by_family.into_iter().collect::<Vec<_>>();

    let marginal_set_digest = {
        let mut encoder = CanonicalEncoder::new();
        encoder.u64(marginals.len() as u64);
        let mut ordered = marginals.iter().collect::<Vec<_>>();
        ordered.sort_by(|left, right| left.id.cmp(&right.id));
        for family in ordered {
            family.encode_canonical(&mut encoder);
        }
        Digest::of_domain_bytes("pen-semantic-audit/sr2-marginal-set/v1", encoder.as_bytes())
    };
    let first_irreducible_basis_digest = {
        let mut encoder = CanonicalEncoder::new();
        encoder.u64(first_irreducible_basis.len() as u64);
        for clause in first_irreducible_basis {
            clause.encode_canonical(&mut encoder);
        }
        Digest::of_domain_bytes(
            "pen-semantic-audit/sr2-first-irreducible-basis/v1",
            encoder.as_bytes(),
        )
    };
    let dependency_support_digest = {
        let mut encoder = CanonicalEncoder::new();
        encoder.u64(dependency_support.len() as u64);
        for support in support_by_family.values() {
            support.encode_canonical(&mut encoder);
        }
        Digest::of_domain_bytes(
            "pen-semantic-audit/sr2-dependency-support/v1",
            encoder.as_bytes(),
        )
    };
    let mut certificate = ProvenanceCertificateV1 {
        manifest_candidate_digest: manifest.candidate_digest().clone(),
        marginal_set_digest,
        first_irreducible_basis_digest,
        dependency_support_digest,
        injection,
        digest: Digest::of_bytes(b"pending provenance certificate"),
    };
    certificate.digest = Digest::of_canonical(
        "pen-semantic-audit/sr2-provenance-certificate/v1",
        &certificate,
    );
    AuditDecision::Proven(certificate)
}

#[cfg(test)]
mod tests {
    use super::{
        FamilyDependencySupportV1, derive_family_dependency_support_v1, verify_sr2_injection_v1,
        verify_sr2_injection_with_support_v1,
    };
    use crate::carrier::enumerate_raw_families_v1;
    use crate::manifest::{
        AuditDecision, AuditUnknownReason, proposed_semantic_audit_manifest_v1,
        verify_core_manifests_v1, verify_semantic_audit_manifest_v1,
    };
    use crate::model::{
        ClauseIdV1, EventIdV1, FamilyClassIdV1, FamilyClassV1, GenericJudgmentV1,
        HeadPresentationV1, LocalRoleV1, PublicHeadSeedV1, PublicSupportV1, RawFamilyIdV1,
        SemanticSchemaSeedV1, SourceNormalizedJudgmentV1,
    };
    use crate::quotient::quotient_families_v1;
    use pen_kernel::{
        Declaration, DependentContext, Digest, GlobalId, Kernel, KernelLimits, Term,
        UncheckedSignature,
    };
    use std::collections::BTreeSet;

    fn id(label: &[u8]) -> Digest {
        Digest::of_domain_bytes("provenance-test", label)
    }

    fn family(label: &[u8], clause: &ClauseIdV1, role: LocalRoleV1) -> FamilyClassV1 {
        let event = EventIdV1(id(b"event"));
        FamilyClassV1 {
            id: FamilyClassIdV1(id(label)),
            representative: RawFamilyIdV1(id(&[label, b"-raw"].concat())),
            members: vec![RawFamilyIdV1(id(&[label, b"-raw"].concat()))],
            generic_judgment: GenericJudgmentV1::Term {
                context: DependentContext::default(),
                term: Term::Unit,
                ty: Term::UnitType,
            },
            role,
            canonical_support: PublicSupportV1 {
                events: BTreeSet::from([event]),
                ..PublicSupportV1::default()
            },
            source_clause: Some(clause.clone()),
            demand_anchor: None,
            substitution_action_digest: id(b"substitution"),
        }
    }

    #[test]
    fn one_clause_can_anchor_two_distinct_typed_roles() {
        let AuditDecision::Proven((semantic, _)) = verify_core_manifests_v1() else {
            panic!("manifest");
        };
        let clause = ClauseIdV1(id(b"clause"));
        let basis = BTreeSet::from([clause.clone()]);
        let families = vec![
            family(b"head", &clause, LocalRoleV1::KernelHead),
            family(b"action", &clause, LocalRoleV1::SupportAction),
        ];
        let AuditDecision::Proven(certificate) =
            verify_sr2_injection_v1(&semantic, &families, &basis, &BTreeSet::new())
        else {
            panic!("roles make the tags distinct");
        };
        assert_eq!(certificate.nu(), 2);
    }

    #[test]
    fn duplicate_clause_role_is_a_lawful_collision_failure() {
        let AuditDecision::Proven((semantic, _)) = verify_core_manifests_v1() else {
            panic!("manifest");
        };
        let clause = ClauseIdV1(id(b"clause"));
        let basis = BTreeSet::from([clause.clone()]);
        let families = vec![
            family(b"one", &clause, LocalRoleV1::KernelHead),
            family(b"two", &clause, LocalRoleV1::KernelHead),
        ];
        assert!(matches!(
            verify_sr2_injection_v1(&semantic, &families, &basis, &BTreeSet::new()),
            AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision)
        ));
    }

    #[test]
    fn multiple_eligible_irreducible_anchors_are_not_order_resolved() {
        let AuditDecision::Proven((semantic, _)) = verify_core_manifests_v1() else {
            panic!("manifest");
        };
        let first = ClauseIdV1(id(b"first"));
        let second = ClauseIdV1(id(b"second"));
        let family = family(b"ambiguous", &first, LocalRoleV1::KernelHead);
        let support = FamilyDependencySupportV1 {
            family: family.id.clone(),
            candidate_local_clauses: BTreeSet::from([first.clone(), second.clone()]),
            preexisting_demand_outputs: BTreeSet::new(),
        };
        assert!(matches!(
            verify_sr2_injection_with_support_v1(
                &semantic,
                &[family],
                &[support],
                &BTreeSet::from([first, second]),
                &BTreeSet::new(),
            ),
            AuditDecision::Unknown(AuditUnknownReason::AmbiguousProvenance)
        ));
    }

    #[test]
    fn dependency_support_is_reconstructed_from_the_carrier_dag() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let head = GlobalId(id(b"head"));
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: head.clone(),
                    ty: Term::UnitType,
                    body: None,
                }],
            })
            .expect("signature");
        let clause = ClauseIdV1(id(b"clause"));
        let judgment = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Global { id: head.clone() },
            ty: Term::UnitType,
        };
        let seed = SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
            declaration: head,
            origin_event: EventIdV1(id(b"event")),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: id(b"source"),
                source: judgment.clone(),
                claimed_normalized: judgment,
            },
            presentation: HeadPresentationV1::Opaque,
            claimed_role: LocalRoleV1::KernelHead,
            public_support: PublicSupportV1::default(),
            source_clause: Some(clause.clone()),
        });
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("manifest");
        };
        let AuditDecision::Proven(carrier) =
            enumerate_raw_families_v1(&kernel, &signature, &manifest, &[seed], &[])
        else {
            panic!("carrier");
        };
        let AuditDecision::Proven(quotient) =
            quotient_families_v1(&kernel, &signature, &manifest, &carrier)
        else {
            panic!("quotient");
        };
        let AuditDecision::Proven(support) =
            derive_family_dependency_support_v1(&carrier, &quotient, &quotient.classes)
        else {
            panic!("dependency support");
        };
        assert_eq!(support.len(), 1);
        assert_eq!(support[0].candidate_local_clauses, BTreeSet::from([clause]));
    }
}
