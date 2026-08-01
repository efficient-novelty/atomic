use crate::carrier::{
    CarrierCertificateV1, curry_judgment, rename_judgment_by_permutation,
    substitution_action_digest,
};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, Q2RuleV1, VerifiedSemanticAuditManifestV1,
};
use crate::model::{
    FamilyClassIdV1, FamilyClassV1, FamilyConstructorV1, GenericJudgmentV1, HeadPresentationV1,
    PublicAvailabilityV1, PublicHeadSeedV1, PublicSupportV1, RawFamilyIdV1, RawFamilyV1, SeedIdV1,
    SemanticSchemaSeedV1,
};
use crate::normalizer::{VerifiedFreshConstructorComputationV1, normalize_generated_judgment_v1};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, Term, VerifiedSignature,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Q2CertifiedInapplicableReasonV1 {
    DifferentOriginatingFamily,
    DifferentJudgmentKind,
    DifferentRole,
    DifferentPublicSupport,
    DifferentNormalizedPresentation,
    ContextLengthsDiffer,
    DependencyOrderNotPreserved,
    NotPriorPublicAlias,
    NotDuplicateTransparentField,
}

impl CanonicalEncode for Q2CertifiedInapplicableReasonV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::DifferentOriginatingFamily => 0,
            Self::DifferentJudgmentKind => 1,
            Self::DifferentRole => 2,
            Self::DifferentPublicSupport => 3,
            Self::DifferentNormalizedPresentation => 4,
            Self::ContextLengthsDiffer => 5,
            Self::DependencyOrderNotPreserved => 6,
            Self::NotPriorPublicAlias => 7,
            Self::NotDuplicateTransparentField => 8,
        });
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum Q2PairOutcomeV1 {
    Applicable {
        edge_digest: Digest,
    },
    CertifiedInapplicable {
        reason: Q2CertifiedInapplicableReasonV1,
    },
}

impl CanonicalEncode for Q2PairOutcomeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Applicable { edge_digest } => {
                encoder.tag(0);
                edge_digest.encode_canonical(encoder);
            }
            Self::CertifiedInapplicable { reason } => {
                encoder.tag(1);
                reason.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Q2PairDispositionV1 {
    pub left: RawFamilyIdV1,
    pub right: RawFamilyIdV1,
    pub rule: Q2RuleV1,
    pub witness_code: Digest,
    pub outcome: Q2PairOutcomeV1,
}

impl CanonicalEncode for Q2PairDispositionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.left.encode_canonical(encoder);
        self.right.encode_canonical(encoder);
        self.rule.encode_canonical(encoder);
        self.witness_code.encode_canonical(encoder);
        self.outcome.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawToClassV1 {
    pub raw: RawFamilyIdV1,
    pub class: FamilyClassIdV1,
}

impl CanonicalEncode for RawToClassV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.raw.encode_canonical(encoder);
        self.class.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QuotientCertificateV1 {
    pub(crate) manifest_digest: Digest,
    pub(crate) signature_digest: Digest,
    pub(crate) normalizer_protocol_digest: Digest,
    pub(crate) fresh_program_digest: Option<Digest>,
    /// Q0 fixes this vertex set before any Q1/Q2/Q3 edge is considered.
    pub(crate) fixed_vertices: Vec<RawFamilyIdV1>,
    pub(crate) q1_identity_edges: Vec<Digest>,
    pub(crate) q2_pair_dispositions: Vec<Q2PairDispositionV1>,
    pub(crate) q3_registry_verified_empty: bool,
    pub(crate) q3_edges: Vec<Digest>,
    pub(crate) classes: Vec<FamilyClassV1>,
    pub(crate) raw_to_class: Vec<RawToClassV1>,
}

impl QuotientCertificateV1 {
    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    pub fn fresh_program_digest(&self) -> Option<&Digest> {
        self.fresh_program_digest.as_ref()
    }

    pub fn fixed_vertices(&self) -> &[RawFamilyIdV1] {
        &self.fixed_vertices
    }

    pub(crate) fn q1_identity_edges(&self) -> &[Digest] {
        &self.q1_identity_edges
    }

    pub fn q2_pair_dispositions(&self) -> &[Q2PairDispositionV1] {
        &self.q2_pair_dispositions
    }

    pub fn q3_registry_verified_empty(&self) -> bool {
        self.q3_registry_verified_empty
    }

    pub(crate) fn q3_edges(&self) -> &[Digest] {
        &self.q3_edges
    }

    pub fn classes(&self) -> &[FamilyClassV1] {
        &self.classes
    }

    pub fn raw_to_class(&self) -> &[RawToClassV1] {
        &self.raw_to_class
    }
}

impl CanonicalEncode for QuotientCertificateV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        encoder.option(&self.fresh_program_digest);
        encoder.sequence(&self.fixed_vertices);
        encoder.sequence(&self.q1_identity_edges);
        encoder.sequence(&self.q2_pair_dispositions);
        encoder.tag(u8::from(self.q3_registry_verified_empty));
        encoder.sequence(&self.q3_edges);
        encoder.sequence(&self.classes);
        encoder.sequence(&self.raw_to_class);
    }
}

/// Compute the complete Q1/Q2 component quotient of a carrier whose Q0
/// vertices are already fixed. Q2 only adds edges between those vertices.
pub fn quotient_families_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    carrier: &CarrierCertificateV1,
) -> AuditDecision<QuotientCertificateV1> {
    quotient_families_with_q0_v1(kernel, signature, manifest, carrier, None)
}

/// Compute the finite Q1/Q2 component quotient while replaying every fixed
/// vertex through the same restricted Q0 program that minted the carrier.
///
/// This entry point binds an already verifier-minted fresh program to the
/// manifest, successor signature, and carrier program digest. It establishes
/// only exact per-vertex replay for this finite carrier. It does not prove the
/// still-open generic termination, confluence, substitution-stability,
/// conservativity, or Rust/Agda agreement theorems for the rewrite system.
pub fn quotient_families_with_q0_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    carrier: &CarrierCertificateV1,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
) -> AuditDecision<QuotientCertificateV1> {
    if carrier.manifest_digest != *manifest.candidate_digest()
        || carrier.signature_digest != *signature.digest()
        || carrier.normalizer_protocol_digest != kernel.normalizer_protocol_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    let supplied_fresh_digest = fresh.map(|program| program.program_digest().clone());
    if carrier.fresh_program_digest != supplied_fresh_digest {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    if let Some(program) = fresh
        && (program.manifest_digest() != manifest.candidate_digest()
            || program.extended_signature().digest() != signature.digest()
            || program
                .termination_certificate()
                .kernel_normalizer_protocol_digest()
                != &kernel.normalizer_protocol_digest()
            || program
                .confluence_certificate()
                .kernel_normalizer_protocol_digest()
                != &kernel.normalizer_protocol_digest())
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    if !carrier.q3_registry_verified_empty {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    }
    let vertices = &carrier.raw_families;
    let mut vertex_ids = BTreeSet::new();
    for vertex in vertices {
        if !vertex_ids.insert(vertex.id.clone()) {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        let normalized = match normalize_generated_judgment_v1(
            manifest,
            kernel,
            signature,
            &vertex.generic_judgment,
            fresh,
        ) {
            AuditDecision::Proven(normalized) => normalized,
            AuditDecision::OutsideFragment(reason) => {
                return AuditDecision::OutsideFragment(reason);
            }
            AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
        };
        if vertex.generic_judgment != normalized {
            return AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure);
        }
    }

    let rule_count = manifest.manifest().q2_rules.len();
    let minimum_dispositions = vertices
        .len()
        .checked_mul(vertices.len())
        .and_then(|pairs| pairs.checked_mul(rule_count));
    if minimum_dispositions
        .is_none_or(|count| count > manifest.manifest().maximum_q2_pair_witnesses as usize)
    {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }

    let seed_index = seed_index(carrier);
    let head_index = match public_head_index(carrier) {
        Ok(index) => index,
        Err(reason) => return AuditDecision::Unknown(reason),
    };
    let alias_targets = match verified_alias_targets(vertices, &seed_index, &head_index) {
        Ok(targets) => targets,
        Err(reason) => return AuditDecision::Unknown(reason),
    };

    let mut union_find = UnionFind::new(vertices.len());
    let q1_identity_edges = vertices
        .iter()
        .map(|vertex| Digest::of_canonical("pen-semantic-audit/q1-identity-edge/v1", &vertex.id))
        .collect::<Vec<_>>();
    let disposition_context = Q2DispositionContext {
        vertices,
        seed_index: &seed_index,
        alias_targets: &alias_targets,
    };
    let mut dispositions = Vec::with_capacity(minimum_dispositions.unwrap_or(0));
    for (left_index, left) in vertices.iter().enumerate() {
        for (right_index, right) in vertices.iter().enumerate() {
            for rule in &manifest.manifest().q2_rules {
                let remaining = (manifest.manifest().maximum_q2_pair_witnesses as usize)
                    .saturating_sub(dispositions.len());
                let generated = match dispositions_for_rule(
                    *rule,
                    Q2PairInput {
                        left,
                        right,
                        left_index,
                        right_index,
                        remaining,
                    },
                    &disposition_context,
                ) {
                    Ok(generated) => generated,
                    Err(reason) => return AuditDecision::Unknown(reason),
                };
                if generated.is_empty() || generated.len() > remaining {
                    return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
                }
                for disposition in generated {
                    if matches!(disposition.outcome, Q2PairOutcomeV1::Applicable { .. }) {
                        union_find.union(left_index, right_index);
                    }
                    dispositions.push(disposition);
                }
            }
        }
    }

    let mut components: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for index in 0..vertices.len() {
        let root = union_find.find(index);
        components.entry(root).or_default().push(index);
    }
    let mut classes = Vec::with_capacity(components.len());
    let mut class_for_index = vec![None; vertices.len()];
    let mut class_ids = BTreeSet::new();
    for members in components.into_values() {
        let representative_index =
            match canonical_representative(&members, vertices, &alias_targets) {
                Ok(index) => index,
                Err(reason) => return AuditDecision::Unknown(reason),
            };
        let representative = &vertices[representative_index];
        let canonical_support = representative.public_support.clone();
        let canonical_action =
            substitution_action_digest(&representative.generic_judgment, representative.role);
        let class_id = family_class_id(
            manifest.candidate_digest(),
            &representative.generic_judgment,
            representative.role,
            &canonical_support,
            &canonical_action,
        );
        if !class_ids.insert(class_id.clone()) {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        let mut member_ids = members
            .iter()
            .map(|index| vertices[*index].id.clone())
            .collect::<Vec<_>>();
        member_ids.sort();
        for index in &members {
            class_for_index[*index] = Some(class_id.clone());
        }
        classes.push(FamilyClassV1 {
            id: class_id,
            representative: representative.id.clone(),
            members: member_ids,
            generic_judgment: representative.generic_judgment.clone(),
            role: representative.role,
            canonical_support,
            source_clause: representative.source_clause.clone(),
            demand_anchor: representative.demand_anchor.clone(),
            substitution_action_digest: canonical_action,
        });
    }
    classes.sort_by(|left, right| left.id.cmp(&right.id));

    let raw_to_class = vertices
        .iter()
        .enumerate()
        .map(|(index, raw)| RawToClassV1 {
            raw: raw.id.clone(),
            class: class_for_index[index]
                .clone()
                .expect("every fixed vertex belongs to one component"),
        })
        .collect();
    AuditDecision::Proven(QuotientCertificateV1 {
        manifest_digest: manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        normalizer_protocol_digest: carrier.normalizer_protocol_digest.clone(),
        fresh_program_digest: carrier.fresh_program_digest.clone(),
        fixed_vertices: vertices.iter().map(|family| family.id.clone()).collect(),
        q1_identity_edges,
        q2_pair_dispositions: dispositions,
        q3_registry_verified_empty: true,
        q3_edges: Vec::new(),
        classes,
        raw_to_class,
    })
}

#[derive(Clone, Copy)]
struct Q2PairInput<'a> {
    left: &'a RawFamilyV1,
    right: &'a RawFamilyV1,
    left_index: usize,
    right_index: usize,
    remaining: usize,
}

#[derive(Clone, Copy)]
struct Q2DispositionContext<'a> {
    vertices: &'a [RawFamilyV1],
    seed_index: &'a BTreeMap<SeedIdV1, SemanticSchemaSeedV1>,
    alias_targets: &'a BTreeMap<usize, usize>,
}

fn dispositions_for_rule(
    rule: Q2RuleV1,
    pair: Q2PairInput<'_>,
    context: &Q2DispositionContext<'_>,
) -> Result<Vec<Q2PairDispositionV1>, AuditUnknownReason> {
    let Q2PairInput {
        left,
        right,
        left_index,
        right_index,
        remaining,
    } = pair;
    let Q2DispositionContext {
        vertices,
        seed_index,
        alias_targets,
    } = context;
    match rule {
        Q2RuleV1::BinderRenaming => Ok(vec![single_disposition(
            left,
            right,
            rule,
            &[],
            if left.generic_judgment == right.generic_judgment
                && left.role == right.role
                && left.public_support == right.public_support
                && left.substitution_action_digest == right.substitution_action_digest
            {
                Ok(())
            } else {
                Err(Q2CertifiedInapplicableReasonV1::DifferentNormalizedPresentation)
            },
        )]),
        Q2RuleV1::IndependentDeclarationExchange => {
            exchange_dispositions(left, right, rule, remaining)
        }
        Q2RuleV1::ExplicitTelescopeCurryIsomorphism => Ok(vec![single_disposition(
            left,
            right,
            rule,
            &[],
            if left.role != right.role {
                Err(Q2CertifiedInapplicableReasonV1::DifferentRole)
            } else if left.public_support != right.public_support {
                Err(Q2CertifiedInapplicableReasonV1::DifferentPublicSupport)
            } else if curry_judgment(&left.generic_judgment)
                == curry_judgment(&right.generic_judgment)
            {
                Ok(())
            } else {
                Err(Q2CertifiedInapplicableReasonV1::DifferentNormalizedPresentation)
            },
        )]),
        Q2RuleV1::PriorPublicTransparentAlias => {
            let aliases = alias_targets.get(&left_index) == Some(&right_index)
                || alias_targets.get(&right_index) == Some(&left_index);
            Ok(vec![single_disposition(
                left,
                right,
                rule,
                &[],
                if aliases
                    && left.role == right.role
                    && curry_judgment(&left.generic_judgment)
                        == curry_judgment(&right.generic_judgment)
                {
                    Ok(())
                } else {
                    Err(Q2CertifiedInapplicableReasonV1::NotPriorPublicAlias)
                },
            )])
        }
        Q2RuleV1::DuplicateTransparentField => {
            let duplicate = duplicate_alias_target(left, right, seed_index)
                .and_then(|target| {
                    let target_vertex = vertices.iter().position(|candidate| {
                        seed_head(candidate, seed_index)
                            .is_some_and(|head| head.declaration == target)
                    })?;
                    (alias_targets.get(&left_index) == Some(&target_vertex)
                        && alias_targets.get(&right_index) == Some(&target_vertex))
                    .then_some(())
                })
                .is_some();
            Ok(vec![single_disposition(
                left,
                right,
                rule,
                &[],
                if duplicate
                    && left.role == right.role
                    && curry_judgment(&left.generic_judgment)
                        == curry_judgment(&right.generic_judgment)
                {
                    Ok(())
                } else {
                    Err(Q2CertifiedInapplicableReasonV1::NotDuplicateTransparentField)
                },
            )])
        }
    }
}

fn exchange_dispositions(
    left: &RawFamilyV1,
    right: &RawFamilyV1,
    rule: Q2RuleV1,
    remaining: usize,
) -> Result<Vec<Q2PairDispositionV1>, AuditUnknownReason> {
    let left_context = left.generic_judgment.context();
    let right_context = right.generic_judgment.context();
    if left_context.0.len() != right_context.0.len() {
        return Ok(vec![single_disposition(
            left,
            right,
            rule,
            &[],
            Err(Q2CertifiedInapplicableReasonV1::ContextLengthsDiffer),
        )]);
    }
    let length = left_context.0.len();
    let permutation_count =
        checked_factorial(length).ok_or(AuditUnknownReason::ResourceExhausted)?;
    if permutation_count > remaining {
        return Err(AuditUnknownReason::ResourceExhausted);
    }
    let mut permutations = Vec::with_capacity(permutation_count);
    enumerate_permutations(
        length,
        &mut Vec::with_capacity(length),
        &mut vec![false; length],
        &mut permutations,
    );
    if permutations.len() != permutation_count {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    }
    let mut dispositions = Vec::with_capacity(permutation_count);
    for permutation in permutations {
        let encoded = permutation
            .iter()
            .map(|value| u32::try_from(*value).expect("context bound fits u32"))
            .collect::<Vec<_>>();
        let decision = if left.role != right.role {
            Err(Q2CertifiedInapplicableReasonV1::DifferentRole)
        } else if left.public_support != right.public_support {
            Err(Q2CertifiedInapplicableReasonV1::DifferentPublicSupport)
        } else if !permutation_preserves_context(right_context, left_context, &encoded) {
            Err(Q2CertifiedInapplicableReasonV1::DependencyOrderNotPreserved)
        } else if rename_judgment_by_permutation(&right.generic_judgment, left_context, &encoded)
            .is_some_and(|renamed| renamed == left.generic_judgment)
        {
            Ok(())
        } else {
            Err(Q2CertifiedInapplicableReasonV1::DifferentNormalizedPresentation)
        };
        dispositions.push(single_disposition(left, right, rule, &encoded, decision));
    }
    Ok(dispositions)
}

fn permutation_preserves_context(
    source: &DependentContext,
    target: &DependentContext,
    source_to_target: &[u32],
) -> bool {
    if source.0.len() != target.0.len() || source.0.len() != source_to_target.len() {
        return false;
    }
    for (source_ordinal, source_type) in source.0.iter().enumerate() {
        let Some(&target_ordinal) = source_to_target.get(source_ordinal) else {
            return false;
        };
        let Ok(target_ordinal) = usize::try_from(target_ordinal) else {
            return false;
        };
        if target_ordinal >= target.0.len() {
            return false;
        }
        let prefix_source = GenericJudgmentV1::Term {
            context: DependentContext(source.0[..source_ordinal].to_vec()),
            term: Term::Unit,
            ty: source_type.clone(),
        };
        let prefix_target = DependentContext(target.0[..target_ordinal].to_vec());
        let Some(GenericJudgmentV1::Term { ty: renamed, .. }) = rename_judgment_by_permutation(
            &prefix_source,
            &prefix_target,
            &source_to_target[..source_ordinal],
        ) else {
            return false;
        };
        if renamed != target.0[target_ordinal] {
            return false;
        }
    }
    true
}

fn single_disposition(
    left: &RawFamilyV1,
    right: &RawFamilyV1,
    rule: Q2RuleV1,
    witness_data: &[u32],
    decision: Result<(), Q2CertifiedInapplicableReasonV1>,
) -> Q2PairDispositionV1 {
    struct Witness<'a> {
        left: &'a RawFamilyIdV1,
        right: &'a RawFamilyIdV1,
        rule: Q2RuleV1,
        data: &'a [u32],
    }
    impl CanonicalEncode for Witness<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.left.encode_canonical(encoder);
            self.right.encode_canonical(encoder);
            self.rule.encode_canonical(encoder);
            encoder.u64(self.data.len() as u64);
            for value in self.data {
                encoder.u32(*value);
            }
        }
    }
    let witness_code = Digest::of_canonical(
        "pen-semantic-audit/q2-witness/v1",
        &Witness {
            left: &left.id,
            right: &right.id,
            rule,
            data: witness_data,
        },
    );
    let outcome = match decision {
        Ok(()) => {
            struct Edge<'a> {
                witness: &'a Digest,
                left: &'a RawFamilyIdV1,
                right: &'a RawFamilyIdV1,
            }
            impl CanonicalEncode for Edge<'_> {
                fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                    self.witness.encode_canonical(encoder);
                    self.left.encode_canonical(encoder);
                    self.right.encode_canonical(encoder);
                }
            }
            Q2PairOutcomeV1::Applicable {
                edge_digest: Digest::of_canonical(
                    "pen-semantic-audit/q2-edge/v1",
                    &Edge {
                        witness: &witness_code,
                        left: &left.id,
                        right: &right.id,
                    },
                ),
            }
        }
        Err(reason) => Q2PairOutcomeV1::CertifiedInapplicable { reason },
    };
    Q2PairDispositionV1 {
        left: left.id.clone(),
        right: right.id.clone(),
        rule,
        witness_code,
        outcome,
    }
}

fn seed_index(carrier: &CarrierCertificateV1) -> BTreeMap<SeedIdV1, SemanticSchemaSeedV1> {
    carrier
        .verified_seeds
        .iter()
        .map(|seed| (seed.id.clone(), seed.seed.clone()))
        .collect()
}

fn public_head_index(
    carrier: &CarrierCertificateV1,
) -> Result<BTreeMap<pen_kernel::GlobalId, usize>, AuditUnknownReason> {
    let seed_index = seed_index(carrier);
    let mut heads = BTreeMap::new();
    for (index, family) in carrier.raw_families.iter().enumerate() {
        if let Some(head) = seed_head(family, &seed_index)
            && heads.insert(head.declaration.clone(), index).is_some()
        {
            return Err(AuditUnknownReason::MalformedInput);
        }
    }
    Ok(heads)
}

fn verified_alias_targets(
    vertices: &[RawFamilyV1],
    seed_index: &BTreeMap<SeedIdV1, SemanticSchemaSeedV1>,
    head_index: &BTreeMap<pen_kernel::GlobalId, usize>,
) -> Result<BTreeMap<usize, usize>, AuditUnknownReason> {
    let mut targets = BTreeMap::new();
    for (index, family) in vertices.iter().enumerate() {
        let Some(head) = seed_head(family, seed_index) else {
            continue;
        };
        let HeadPresentationV1::TransparentAlias {
            target,
            availability,
        } = &head.presentation
        else {
            continue;
        };
        if !alias_is_deletable(target, availability) {
            continue;
        }
        let Some(&target_index) = head_index.get(target) else {
            return Err(AuditUnknownReason::IncompleteSupport);
        };
        if index == target_index
            || family.role != vertices[target_index].role
            || curry_judgment(&family.generic_judgment)
                != curry_judgment(&vertices[target_index].generic_judgment)
        {
            return Err(AuditUnknownReason::IncompleteSupport);
        }
        targets.insert(index, target_index);
    }
    for start in targets.keys().copied() {
        let mut seen = BTreeSet::new();
        let mut current = start;
        while let Some(next) = targets.get(&current).copied() {
            if !seen.insert(current) {
                return Err(AuditUnknownReason::ProvenanceCollision);
            }
            current = next;
        }
    }
    Ok(targets)
}

fn seed_head<'a>(
    family: &RawFamilyV1,
    seed_index: &'a BTreeMap<SeedIdV1, SemanticSchemaSeedV1>,
) -> Option<&'a PublicHeadSeedV1> {
    let FamilyConstructorV1::PublicHeadSeed { seed } = &family.constructor else {
        return None;
    };
    match seed_index.get(seed)? {
        SemanticSchemaSeedV1::PublicHead(head) => Some(head),
        _ => None,
    }
}

fn alias_is_deletable(target: &pen_kernel::GlobalId, availability: &PublicAvailabilityV1) -> bool {
    match availability {
        PublicAvailabilityV1::PredecessorPublicExport { target: available }
        | PublicAvailabilityV1::DependencyPriorExport { target: available } => available == target,
        PublicAvailabilityV1::DerivedFromPublicInterface { public_support, .. } => {
            public_support.contains(target)
        }
        PublicAvailabilityV1::AmbientOnly
        | PublicAvailabilityV1::OutsideFragment
        | PublicAvailabilityV1::Unknown => false,
    }
}

fn duplicate_alias_target(
    left: &RawFamilyV1,
    right: &RawFamilyV1,
    seed_index: &BTreeMap<SeedIdV1, SemanticSchemaSeedV1>,
) -> Option<pen_kernel::GlobalId> {
    let left = seed_head(left, seed_index)?;
    let right = seed_head(right, seed_index)?;
    let (
        HeadPresentationV1::TransparentAlias {
            target: left_target,
            availability: left_availability,
        },
        HeadPresentationV1::TransparentAlias {
            target: right_target,
            availability: right_availability,
        },
    ) = (&left.presentation, &right.presentation)
    else {
        return None;
    };
    (left_target == right_target
        && alias_is_deletable(left_target, left_availability)
        && alias_is_deletable(right_target, right_availability))
    .then(|| left_target.clone())
}

fn canonical_representative(
    members: &[usize],
    vertices: &[RawFamilyV1],
    alias_targets: &BTreeMap<usize, usize>,
) -> Result<usize, AuditUnknownReason> {
    let mut candidates = BTreeSet::new();
    let alias_members = members
        .iter()
        .copied()
        .filter(|member| alias_targets.contains_key(member))
        .collect::<Vec<_>>();
    let starts = if alias_members.is_empty() {
        members
    } else {
        alias_members.as_slice()
    };
    for &member in starts {
        let mut current = member;
        let mut seen = BTreeSet::new();
        while let Some(next) = alias_targets.get(&current).copied() {
            if !seen.insert(current) {
                return Err(AuditUnknownReason::ProvenanceCollision);
            }
            current = next;
        }
        if members.contains(&current) {
            candidates.insert(current);
        }
    }
    candidates
        .into_iter()
        .min_by(|left, right| {
            presentation_digest(&vertices[*left])
                .cmp(&presentation_digest(&vertices[*right]))
                .then_with(|| vertices[*left].id.cmp(&vertices[*right].id))
        })
        .ok_or(AuditUnknownReason::IncompleteEnumeration)
}

fn presentation_digest(family: &RawFamilyV1) -> Digest {
    struct Subject<'a> {
        judgment: &'a GenericJudgmentV1,
        role: crate::model::LocalRoleV1,
        support: &'a PublicSupportV1,
    }
    impl CanonicalEncode for Subject<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.judgment.encode_canonical(encoder);
            self.role.encode_canonical(encoder);
            self.support.encode_canonical(encoder);
        }
    }
    Digest::of_canonical(
        "pen-semantic-audit/q2-presentation/v1",
        &Subject {
            judgment: &family.generic_judgment,
            role: family.role,
            support: &family.public_support,
        },
    )
}

fn family_class_id(
    manifest_digest: &Digest,
    judgment: &GenericJudgmentV1,
    role: crate::model::LocalRoleV1,
    support: &PublicSupportV1,
    substitution_action_digest: &Digest,
) -> FamilyClassIdV1 {
    struct Subject<'a> {
        manifest_digest: &'a Digest,
        judgment: &'a GenericJudgmentV1,
        role: crate::model::LocalRoleV1,
        support: &'a PublicSupportV1,
        action: &'a Digest,
    }
    impl CanonicalEncode for Subject<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.manifest_digest.encode_canonical(encoder);
            self.judgment.encode_canonical(encoder);
            self.role.encode_canonical(encoder);
            self.support.encode_canonical(encoder);
            self.action.encode_canonical(encoder);
        }
    }
    FamilyClassIdV1(Digest::of_canonical(
        "pen-semantic-audit/family-class/v1",
        &Subject {
            manifest_digest,
            judgment,
            role,
            support,
            action: substitution_action_digest,
        },
    ))
}

fn enumerate_permutations(
    length: usize,
    prefix: &mut Vec<usize>,
    used: &mut [bool],
    output: &mut Vec<Vec<usize>>,
) {
    if prefix.len() == length {
        output.push(prefix.clone());
        return;
    }
    for value in 0..length {
        if used[value] {
            continue;
        }
        used[value] = true;
        prefix.push(value);
        enumerate_permutations(length, prefix, used, output);
        prefix.pop();
        used[value] = false;
    }
}

fn checked_factorial(value: usize) -> Option<usize> {
    (1..=value).try_fold(1_usize, |accumulator, item| accumulator.checked_mul(item))
}

#[derive(Clone, Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(length: usize) -> Self {
        Self {
            parent: (0..length).collect(),
            rank: vec![0; length],
        }
    }

    fn find(&mut self, value: usize) -> usize {
        if self.parent[value] != value {
            self.parent[value] = self.find(self.parent[value]);
        }
        self.parent[value]
    }

    fn union(&mut self, left: usize, right: usize) {
        let left = self.find(left);
        let right = self.find(right);
        if left == right {
            return;
        }
        if self.rank[left] < self.rank[right] {
            self.parent[left] = right;
        } else if self.rank[left] > self.rank[right] {
            self.parent[right] = left;
        } else {
            self.parent[right] = left;
            self.rank[left] = self.rank[left].saturating_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{quotient_families_v1, quotient_families_with_q0_v1};
    use crate::carrier::{enumerate_raw_families_v1, enumerate_raw_families_with_q0_v1};
    use crate::manifest::{
        AuditDecision, AuditUnknownReason, Q2RuleV1, proposed_semantic_audit_manifest_v1,
        verify_semantic_audit_manifest_v1,
    };
    use crate::model::{
        EquationIdV1, EventIdV1, FamilyConstructorV1, GenericJudgmentV1, HeadPresentationV1,
        LocalRoleV1, PublicEquationSeedV1, PublicHeadSeedV1, PublicSupportV1, SemanticSchemaSeedV1,
        SourceNormalizedJudgmentV1,
    };
    use crate::normalizer::{
        FreshConstructorClauseV1, FreshConstructorComputationRequestV1,
        verify_fresh_constructor_computation_v1,
    };
    use pen_kernel::{
        Declaration, DependentContext, Digest, GlobalId, Kernel, KernelLimits, OpenJudgment, Term,
        UncheckedSignature,
    };

    fn id(label: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    fn seed(
        declaration: GlobalId,
        event: &[u8],
        source: GenericJudgmentV1,
        normalized: GenericJudgmentV1,
        presentation: HeadPresentationV1,
    ) -> SemanticSchemaSeedV1 {
        SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
            declaration,
            origin_event: EventIdV1(Digest::of_bytes(event)),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: Digest::of_bytes(event),
                source,
                claimed_normalized: normalized,
            },
            presentation,
            claimed_role: LocalRoleV1::KernelHead,
            public_support: PublicSupportV1::default(),
            source_clause: None,
        })
    }

    #[test]
    fn identity_and_disjoint_telescope_presentations_form_one_family_class() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let function = id(b"function");
        let argument = id(b"argument");
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: function.clone(),
                        ty: Term::Pi {
                            parameter: Box::new(Term::UnitType),
                            body: Box::new(Term::UnitType),
                        },
                        body: None,
                    },
                    Declaration {
                        id: argument.clone(),
                        ty: Term::UnitType,
                        body: Some(Term::Unit),
                    },
                ],
            })
            .expect("signature");
        let function_judgment = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Global {
                id: function.clone(),
            },
            ty: Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            },
        };
        let argument_source = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Global {
                id: argument.clone(),
            },
            ty: Term::UnitType,
        };
        let argument_normal = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        };
        let seeds = vec![
            seed(
                function,
                b"function-event",
                function_judgment.clone(),
                function_judgment,
                HeadPresentationV1::Opaque,
            ),
            seed(
                argument,
                b"argument-event",
                argument_source,
                argument_normal,
                HeadPresentationV1::TransparentDefinition,
            ),
        ];
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("manifest");
        };
        let AuditDecision::Proven(carrier) =
            enumerate_raw_families_v1(&kernel, &signature, &manifest, &seeds, &[])
        else {
            panic!("carrier");
        };
        let application_count = carrier
            .raw_families
            .iter()
            .filter(|family| {
                matches!(
                    family.constructor,
                    FamilyConstructorV1::GenericPublicApplication { .. }
                ) && family.rank == 1
            })
            .count();
        assert_eq!(application_count, 2);

        let AuditDecision::Proven(quotient) =
            quotient_families_v1(&kernel, &signature, &manifest, &carrier)
        else {
            panic!("quotient");
        };
        let application_classes = quotient
            .classes
            .iter()
            .filter(|class| {
                class.members.iter().any(|member| {
                    carrier.raw_families.iter().any(|family| {
                        &family.id == member
                            && matches!(
                                family.constructor,
                                FamilyConstructorV1::GenericPublicApplication { .. }
                            )
                            && family.rank == 1
                    })
                })
            })
            .count();
        assert_eq!(application_classes, 1);
        assert!(quotient.q3_edges.is_empty());
        assert!(quotient.q3_registry_verified_empty);
        assert!(quotient.q2_pair_dispositions.iter().any(|disposition| {
            disposition.rule == Q2RuleV1::BinderRenaming
                && matches!(
                    disposition.outcome,
                    super::Q2PairOutcomeV1::Applicable { .. }
                )
                && disposition.left != disposition.right
        }));
    }

    #[test]
    fn admitted_fresh_equation_reaches_quotient_only_with_its_bound_q0_program() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let constructor = id(b"fresh-equation-constructor");
        let other_constructor = id(b"other-fresh-equation-constructor");
        let head = id(b"fresh-equation-head");
        let boundary = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: constructor.clone(),
                        ty: Term::UnitType,
                        body: None,
                    },
                    Declaration {
                        id: other_constructor.clone(),
                        ty: Term::UnitType,
                        body: None,
                    },
                ],
            })
            .expect("boundary");
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("manifest");
        };
        let fresh_declaration = Declaration {
            id: head.clone(),
            ty: Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::Pi {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::UnitType),
                }),
            },
            body: None,
        };
        let request = |constructor: GlobalId| FreshConstructorComputationRequestV1 {
            fresh_declaration: fresh_declaration.clone(),
            clauses: vec![FreshConstructorClauseV1 {
                constructor,
                scrutinee_parameter_ordinal: 1,
            }],
        };
        let AuditDecision::Proven(program) = verify_fresh_constructor_computation_v1(
            &manifest,
            &kernel,
            &boundary,
            &request(constructor.clone()),
        ) else {
            panic!("fresh program");
        };
        let AuditDecision::Proven(other_program) = verify_fresh_constructor_computation_v1(
            &manifest,
            &kernel,
            &boundary,
            &request(other_constructor),
        ) else {
            panic!("other fresh program");
        };
        let signature = program.extended_signature().clone();
        assert_eq!(
            signature.digest(),
            other_program.extended_signature().digest()
        );
        assert_ne!(program.program_digest(), other_program.program_digest());

        let context = DependentContext(vec![Term::UnitType]);
        let source_left = Term::Apply {
            function: Box::new(Term::Apply {
                function: Box::new(Term::Global { id: head.clone() }),
                argument: Box::new(Term::Var { index: 0 }),
            }),
            argument: Box::new(Term::Global {
                id: constructor.clone(),
            }),
        };
        let source = GenericJudgmentV1::Equation {
            context: context.clone(),
            left: source_left.clone(),
            right: Term::Var { index: 0 },
            ty: Term::UnitType,
        };
        assert!(
            kernel
                .verify_open_judgment(
                    &signature,
                    &OpenJudgment::DefinitionallyEqual {
                        context: context.clone(),
                        left: source_left,
                        right: Term::Var { index: 0 },
                        ty: Term::UnitType,
                    },
                )
                .is_err(),
            "the base kernel must not silently know the admitted rewrite"
        );
        let normalized = GenericJudgmentV1::Equation {
            context,
            left: Term::Var { index: 0 },
            right: Term::Var { index: 0 },
            ty: Term::UnitType,
        };
        let seed = SemanticSchemaSeedV1::PublicEquation(PublicEquationSeedV1 {
            equation: EquationIdV1(Digest::of_bytes(b"fresh-equation")),
            owner_head: head,
            origin_event: EventIdV1(Digest::of_bytes(b"fresh-equation-event")),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: Digest::of_bytes(b"fresh-equation-source"),
                source,
                claimed_normalized: normalized,
            },
            claimed_role: LocalRoleV1::Coherence,
            public_support: PublicSupportV1::default(),
            source_clause: None,
            demand_anchor: None,
        });
        let AuditDecision::Proven(carrier) = enumerate_raw_families_with_q0_v1(
            &kernel,
            &signature,
            &manifest,
            &[seed],
            &[],
            Some(&program),
        ) else {
            panic!("q0-aware carrier");
        };

        assert!(matches!(
            quotient_families_with_q0_v1(&kernel, &signature, &manifest, &carrier, Some(&program),),
            AuditDecision::Proven(_)
        ));
        assert!(matches!(
            quotient_families_with_q0_v1(
                &kernel,
                &signature,
                &manifest,
                &carrier,
                Some(&other_program),
            ),
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        ));
        assert!(matches!(
            quotient_families_v1(&kernel, &signature, &manifest, &carrier),
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        ));
    }
}
