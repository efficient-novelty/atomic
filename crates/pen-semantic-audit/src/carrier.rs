use crate::fragment::lambda_unit_judgment_syntax_violation;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1, VerifiedSemanticAuditManifestV1,
};
use crate::model::{
    ContextWitnessIdV1, FamilyConstructorV1, GenericJudgmentV1, HeadPresentationV1, LocalRoleV1,
    PublicAvailabilityV1, RawFamilyIdV1, RawFamilyV1, SeedIdV1, SemanticSchemaSeedV1,
    raw_family_id, semantic_seed_id,
};
use crate::normalizer::{
    VerifiedFreshConstructorComputationV1, normalize_generated_judgment_v1,
    verify_source_normalized_judgment_v1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError, OpenJudgment,
    Term, VerifiedSignature,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextWitnessKindV1 {
    IdenticalTelescope,
    DependencyRespectingDisjointInterleaving,
}

impl CanonicalEncode for ContextWitnessKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::IdenticalTelescope => 0,
            Self::DependencyRespectingDisjointInterleaving => 1,
        });
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextAmalgamationWitnessV1 {
    pub id: ContextWitnessIdV1,
    pub kind: ContextWitnessKindV1,
    pub left_context: DependentContext,
    pub right_context: DependentContext,
    pub target_context: DependentContext,
    /// Source declaration ordinal to target declaration ordinal.
    pub left_embedding: Vec<u32>,
    /// Source declaration ordinal to target declaration ordinal.
    pub right_embedding: Vec<u32>,
}

impl CanonicalEncode for ContextAmalgamationWitnessV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.kind.encode_canonical(encoder);
        self.left_context.encode_canonical(encoder);
        self.right_context.encode_canonical(encoder);
        self.target_context.encode_canonical(encoder);
        encode_u32_slice(encoder, &self.left_embedding);
        encode_u32_slice(encoder, &self.right_embedding);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CarrierRuleV1 {
    GenericPublicApplication,
    GenericEquationAction,
}

impl CanonicalEncode for CarrierRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::GenericPublicApplication => 0,
            Self::GenericEquationAction => 1,
        });
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedInapplicableReasonV1 {
    WrongJudgmentKind,
    FunctionConclusionIsNotPi,
    ArgumentTypeMismatch,
    ContextIsNotPublicApplication,
    HoleIsNotTypedForEquation,
}

impl CanonicalEncode for CertifiedInapplicableReasonV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::WrongJudgmentKind => 0,
            Self::FunctionConclusionIsNotPi => 1,
            Self::ArgumentTypeMismatch => 2,
            Self::ContextIsNotPublicApplication => 3,
            Self::HoleIsNotTypedForEquation => 4,
        });
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum CarrierTupleOutcomeV1 {
    Applicable {
        family: RawFamilyIdV1,
    },
    CertifiedInapplicable {
        reason: CertifiedInapplicableReasonV1,
    },
}

impl CanonicalEncode for CarrierTupleOutcomeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Applicable { family } => {
                encoder.tag(0);
                family.encode_canonical(encoder);
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
pub struct CarrierTupleDispositionV1 {
    pub rank: u16,
    pub rule: CarrierRuleV1,
    pub left: RawFamilyIdV1,
    pub right: RawFamilyIdV1,
    pub context_witness: ContextWitnessIdV1,
    pub hole_ordinal: Option<u32>,
    pub outcome: CarrierTupleOutcomeV1,
}

impl CanonicalEncode for CarrierTupleDispositionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.rank);
        self.rule.encode_canonical(encoder);
        self.left.encode_canonical(encoder);
        self.right.encode_canonical(encoder);
        self.context_witness.encode_canonical(encoder);
        match self.hole_ordinal {
            Some(ordinal) => {
                encoder.tag(1);
                encoder.u32(ordinal);
            }
            None => encoder.tag(0),
        }
        self.outcome.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedSemanticSeedV1 {
    pub id: SeedIdV1,
    pub seed: SemanticSchemaSeedV1,
    pub normalized_judgment: GenericJudgmentV1,
    pub derived_role: LocalRoleV1,
}

impl CanonicalEncode for VerifiedSemanticSeedV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.seed.encode_canonical(encoder);
        self.normalized_judgment.encode_canonical(encoder);
        self.derived_role.encode_canonical(encoder);
    }
}

/// Verifier-minted source seed used before any Q0 rewrite authority exists.
///
/// The capability retains only the source judgment.  A caller-supplied
/// `claimed_normalized` value is replaced by the source judgment before the
/// seed identity is minted, so it cannot suppress or merge a raw derivation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedPreQ0SemanticSeedV1 {
    id: SeedIdV1,
    seed: SemanticSchemaSeedV1,
    source_judgment: GenericJudgmentV1,
    derived_role: LocalRoleV1,
}

impl VerifiedPreQ0SemanticSeedV1 {
    pub fn id(&self) -> &SeedIdV1 {
        &self.id
    }

    pub fn seed(&self) -> &SemanticSchemaSeedV1 {
        &self.seed
    }

    pub fn source_judgment(&self) -> &GenericJudgmentV1 {
        &self.source_judgment
    }

    pub fn derived_role(&self) -> LocalRoleV1 {
        self.derived_role
    }
}

impl CanonicalEncode for VerifiedPreQ0SemanticSeedV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.seed.encode_canonical(encoder);
        self.source_judgment.encode_canonical(encoder);
        self.derived_role.encode_canonical(encoder);
    }
}

/// Complete syntactic rank-0/1/2 carrier before Q0 normalization or
/// deduplication.
///
/// Fields are private and the capability deliberately has no `Deserialize`
/// implementation.  Kernel checking establishes formation/typing only; it
/// does not treat a fresh public equation as definitional equality.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PreQ0RawCarrierCertificateV1 {
    manifest_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    q3_registry_verified_empty: bool,
    verified_seeds: Vec<VerifiedPreQ0SemanticSeedV1>,
    context_witnesses: Vec<ContextAmalgamationWitnessV1>,
    tuple_dispositions: Vec<CarrierTupleDispositionV1>,
    raw_families: Vec<RawFamilyV1>,
    digest: Digest,
}

impl PreQ0RawCarrierCertificateV1 {
    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn q3_registry_verified_empty(&self) -> bool {
        self.q3_registry_verified_empty
    }

    pub fn verified_seeds(&self) -> &[VerifiedPreQ0SemanticSeedV1] {
        &self.verified_seeds
    }

    pub fn context_witnesses(&self) -> &[ContextAmalgamationWitnessV1] {
        &self.context_witnesses
    }

    pub fn tuple_dispositions(&self) -> &[CarrierTupleDispositionV1] {
        &self.tuple_dispositions
    }

    pub fn raw_families(&self) -> &[RawFamilyV1] {
        &self.raw_families
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for PreQ0RawCarrierCertificateV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        encoder.tag(u8::from(self.q3_registry_verified_empty));
        encoder.sequence(&self.verified_seeds);
        encoder.sequence(&self.context_witnesses);
        encoder.sequence(&self.tuple_dispositions);
        encoder.sequence(&self.raw_families);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CarrierCertificateV1 {
    pub(crate) manifest_digest: Digest,
    pub(crate) signature_digest: Digest,
    pub(crate) normalizer_protocol_digest: Digest,
    pub(crate) fresh_program_digest: Option<Digest>,
    pub(crate) q3_registry_verified_empty: bool,
    pub(crate) verified_seeds: Vec<VerifiedSemanticSeedV1>,
    pub(crate) context_witnesses: Vec<ContextAmalgamationWitnessV1>,
    pub(crate) tuple_dispositions: Vec<CarrierTupleDispositionV1>,
    pub(crate) raw_families: Vec<RawFamilyV1>,
}

impl CarrierCertificateV1 {
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

    pub fn q3_registry_verified_empty(&self) -> bool {
        self.q3_registry_verified_empty
    }

    pub fn verified_seeds(&self) -> &[VerifiedSemanticSeedV1] {
        &self.verified_seeds
    }

    pub fn context_witnesses(&self) -> &[ContextAmalgamationWitnessV1] {
        &self.context_witnesses
    }

    pub fn tuple_dispositions(&self) -> &[CarrierTupleDispositionV1] {
        &self.tuple_dispositions
    }

    pub fn raw_families(&self) -> &[RawFamilyV1] {
        &self.raw_families
    }
}

impl CanonicalEncode for CarrierCertificateV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        encoder.option(&self.fresh_program_digest);
        encoder.tag(u8::from(self.q3_registry_verified_empty));
        encoder.sequence(&self.verified_seeds);
        encoder.sequence(&self.context_witnesses);
        encoder.sequence(&self.tuple_dispositions);
        encoder.sequence(&self.raw_families);
    }
}

/// Verify one seed against the exact proposed finite core.
///
/// The source judgment and its claimed Q0 normal form are both checked by
/// `pen-kernel`. The claim is accepted only when it is already canonical and
/// exactly equals the kernel's normalization of the source.
pub fn verify_semantic_seed_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    seed: &SemanticSchemaSeedV1,
) -> AuditDecision<VerifiedSemanticSeedV1> {
    verify_semantic_seed_with_q0_v1(kernel, signature, manifest, seed, None)
}

pub fn verify_semantic_seed_with_q0_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    seed: &SemanticSchemaSeedV1,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
) -> AuditDecision<VerifiedSemanticSeedV1> {
    let limits = manifest.manifest();
    let (judgment, claimed_role, derived_role) = match seed {
        SemanticSchemaSeedV1::PublicHead(seed) => {
            if !signature
                .declarations()
                .iter()
                .any(|declaration| declaration.id == seed.declaration)
            {
                return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
            }
            if !matches!(seed.judgment.source, GenericJudgmentV1::Term { .. })
                || !matches!(
                    seed.judgment.claimed_normalized,
                    GenericJudgmentV1::Term { .. }
                )
            {
                return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
            }
            if let HeadPresentationV1::TransparentAlias {
                target,
                availability,
            } = &seed.presentation
            {
                match validate_alias_availability(target, availability) {
                    Ok(()) => {}
                    Err(failure) => return failure.into_decision(),
                }
            }
            (&seed.judgment, seed.claimed_role, LocalRoleV1::KernelHead)
        }
        SemanticSchemaSeedV1::PublicEquation(seed) => {
            if !signature
                .declarations()
                .iter()
                .any(|declaration| declaration.id == seed.owner_head)
            {
                return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
            }
            if !matches!(seed.judgment.source, GenericJudgmentV1::Equation { .. })
                || !matches!(
                    seed.judgment.claimed_normalized,
                    GenericJudgmentV1::Equation { .. }
                )
            {
                return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
            }
            (&seed.judgment, seed.claimed_role, LocalRoleV1::Coherence)
        }
        SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
            return AuditDecision::OutsideFragment(OutsideFragmentReason::UniversalInterface);
        }
    };

    if claimed_role != derived_role {
        return AuditDecision::Unknown(AuditUnknownReason::RoleMismatch);
    }
    if judgment.source.context().0.len() > usize::from(limits.maximum_context_entries)
        || judgment.claimed_normalized.context().0.len()
            > usize::from(limits.maximum_context_entries)
    {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }
    if let Err(reason) = validate_supported_judgment(&judgment.source, manifest) {
        return AuditDecision::OutsideFragment(reason);
    }
    if let Err(reason) = validate_supported_judgment(&judgment.claimed_normalized, manifest) {
        return AuditDecision::OutsideFragment(reason);
    }

    let normalized =
        match verify_source_normalized_judgment_v1(manifest, kernel, signature, judgment, fresh) {
            AuditDecision::Proven(verified) => verified.normalized().clone(),
            AuditDecision::OutsideFragment(reason) => {
                return AuditDecision::OutsideFragment(reason);
            }
            AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
        };

    AuditDecision::Proven(VerifiedSemanticSeedV1 {
        id: semantic_seed_id(seed),
        seed: seed.clone(),
        normalized_judgment: normalized,
        derived_role,
    })
}

/// Verify a seed for syntactic carrier generation without granting Q0
/// normalization authority.
///
/// Term seeds are kernel-checked at their stated type.  Equation seeds check
/// both sides at the stated type but deliberately do not ask the kernel to
/// accept the candidate equation as definitional equality.
pub fn verify_pre_q0_semantic_seed_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    seed: &SemanticSchemaSeedV1,
) -> AuditDecision<VerifiedPreQ0SemanticSeedV1> {
    let canonical_seed = canonical_pre_q0_seed(seed);
    let limits = manifest.manifest();
    let (judgment, claimed_role, derived_role) = match &canonical_seed {
        SemanticSchemaSeedV1::PublicHead(seed) => {
            if !signature
                .declarations()
                .iter()
                .any(|declaration| declaration.id == seed.declaration)
                || !matches!(seed.judgment.source, GenericJudgmentV1::Term { .. })
            {
                return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
            }
            if let HeadPresentationV1::TransparentAlias {
                target,
                availability,
            } = &seed.presentation
                && let Err(failure) = validate_alias_availability(target, availability)
            {
                return failure.into_decision();
            }
            (
                &seed.judgment.source,
                seed.claimed_role,
                LocalRoleV1::KernelHead,
            )
        }
        SemanticSchemaSeedV1::PublicEquation(seed) => {
            if !signature
                .declarations()
                .iter()
                .any(|declaration| declaration.id == seed.owner_head)
                || !matches!(seed.judgment.source, GenericJudgmentV1::Equation { .. })
            {
                return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
            }
            (
                &seed.judgment.source,
                seed.claimed_role,
                LocalRoleV1::Coherence,
            )
        }
        SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
            return AuditDecision::OutsideFragment(OutsideFragmentReason::UniversalInterface);
        }
    };
    if claimed_role != derived_role {
        return AuditDecision::Unknown(AuditUnknownReason::RoleMismatch);
    }
    if judgment.context().0.len() > usize::from(limits.maximum_context_entries) {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }
    if let Err(reason) = validate_supported_judgment(judgment, manifest) {
        return AuditDecision::OutsideFragment(reason);
    }

    let checked = match judgment {
        GenericJudgmentV1::Term { .. } => kernel
            .verify_open_judgment(signature, &generic_to_open(judgment))
            .map(|_| ()),
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => {
            let left = OpenJudgment::HasType {
                context: context.clone(),
                term: left.clone(),
                ty: ty.clone(),
            };
            let right = OpenJudgment::HasType {
                context: context.clone(),
                term: right.clone(),
                ty: ty.clone(),
            };
            kernel
                .verify_open_judgments(signature, &[&left, &right])
                .map(|_| ())
        }
    };
    if let Err(error) = checked {
        return kernel_error_failure(error).into_decision();
    }

    let id = semantic_seed_id(&canonical_seed);
    let source_judgment = judgment.clone();
    AuditDecision::Proven(VerifiedPreQ0SemanticSeedV1 {
        id,
        seed: canonical_seed,
        source_judgment,
        derived_role,
    })
}

fn canonical_pre_q0_seed(seed: &SemanticSchemaSeedV1) -> SemanticSchemaSeedV1 {
    let mut seed = seed.clone();
    let judgment = match &mut seed {
        SemanticSchemaSeedV1::PublicHead(seed) => &mut seed.judgment,
        SemanticSchemaSeedV1::PublicEquation(seed) => &mut seed.judgment,
        SemanticSchemaSeedV1::PublicUniversalInterface { judgment, .. } => judgment,
    };
    judgment.claimed_normalized = judgment.source.clone();
    seed
}

/// Exhaust the seed, application, and equation-action carrier through rank 2.
///
/// `q3_origin_cutoff_edges` is a generic input contract. V1 accepts only a
/// positively supplied empty registry and never consults a live profile.
pub fn enumerate_raw_families_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    seeds: &[SemanticSchemaSeedV1],
    q3_origin_cutoff_edges: &[Digest],
) -> AuditDecision<CarrierCertificateV1> {
    enumerate_raw_families_with_q0_v1(
        kernel,
        signature,
        manifest,
        seeds,
        q3_origin_cutoff_edges,
        None,
    )
}

pub fn enumerate_raw_families_with_q0_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    seeds: &[SemanticSchemaSeedV1],
    q3_origin_cutoff_edges: &[Digest],
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
) -> AuditDecision<CarrierCertificateV1> {
    let limits = manifest.manifest();
    if !q3_origin_cutoff_edges.is_empty() {
        return AuditDecision::OutsideFragment(OutsideFragmentReason::NonEmptyQ3Registry);
    }
    if seeds.len() > usize::from(limits.maximum_seeds) {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }
    if limits.maximum_rank != 2 {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }

    let mut verified_seeds = Vec::with_capacity(seeds.len());
    for seed in seeds {
        match verify_semantic_seed_with_q0_v1(kernel, signature, manifest, seed, fresh) {
            AuditDecision::Proven(seed) => verified_seeds.push(seed),
            AuditDecision::OutsideFragment(reason) => {
                return AuditDecision::OutsideFragment(reason);
            }
            AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
        }
    }
    verified_seeds.sort_by(|left, right| left.id.cmp(&right.id));
    if verified_seeds
        .windows(2)
        .any(|pair| pair[0].id == pair[1].id)
    {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    }

    let mut raw_families = Vec::with_capacity(verified_seeds.len());
    for seed in &verified_seeds {
        let family = seed_raw_family(manifest, seed);
        raw_families.push(family);
    }
    if raw_families.len() > limits.maximum_raw_derivations as usize {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }

    let build_context = CarrierBuildContext {
        kernel,
        signature,
        manifest,
        fresh,
        normalization_mode: CarrierNormalizationMode::AuthorizedQ0,
    };
    let (context_witnesses, tuple_dispositions, raw_families) =
        match enumerate_family_closure(&build_context, raw_families) {
            Ok(result) => result,
            Err(failure) => return failure.into_decision(),
        };

    AuditDecision::Proven(CarrierCertificateV1 {
        manifest_digest: manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        fresh_program_digest: fresh.map(|program| program.program_digest().clone()),
        q3_registry_verified_empty: true,
        verified_seeds,
        context_witnesses,
        tuple_dispositions,
        raw_families,
    })
}

/// Exhaust the syntactic seed/application/equation-action carrier through
/// rank two before any Q0 normalization or Q0-based deduplication.
pub fn enumerate_pre_q0_raw_families_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    seeds: &[SemanticSchemaSeedV1],
    q3_origin_cutoff_edges: &[Digest],
) -> AuditDecision<PreQ0RawCarrierCertificateV1> {
    let limits = manifest.manifest();
    if !q3_origin_cutoff_edges.is_empty() {
        return AuditDecision::OutsideFragment(OutsideFragmentReason::NonEmptyQ3Registry);
    }
    if seeds.len() > usize::from(limits.maximum_seeds) {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }
    if limits.maximum_rank != 2 {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }

    let mut verified_seeds = Vec::with_capacity(seeds.len());
    for seed in seeds {
        match verify_pre_q0_semantic_seed_v1(kernel, signature, manifest, seed) {
            AuditDecision::Proven(seed) => verified_seeds.push(seed),
            AuditDecision::OutsideFragment(reason) => {
                return AuditDecision::OutsideFragment(reason);
            }
            AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
        }
    }
    verified_seeds.sort_by(|left, right| left.id.cmp(&right.id));
    if verified_seeds
        .windows(2)
        .any(|pair| pair[0].id == pair[1].id)
    {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    }

    let raw_families = verified_seeds
        .iter()
        .map(|seed| pre_q0_seed_raw_family(manifest, seed))
        .collect::<Vec<_>>();
    if raw_families.len() > limits.maximum_raw_derivations as usize {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }
    let build_context = CarrierBuildContext {
        kernel,
        signature,
        manifest,
        fresh: None,
        normalization_mode: CarrierNormalizationMode::PreQ0Syntactic,
    };
    let (context_witnesses, tuple_dispositions, raw_families) =
        match enumerate_family_closure(&build_context, raw_families) {
            Ok(result) => result,
            Err(failure) => return failure.into_decision(),
        };

    let mut certificate = PreQ0RawCarrierCertificateV1 {
        manifest_digest: manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        q3_registry_verified_empty: true,
        verified_seeds,
        context_witnesses,
        tuple_dispositions,
        raw_families,
        digest: Digest::of_bytes(b"pending-pre-q0-raw-carrier"),
    };
    certificate.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-pre-q0-raw-carrier/v1",
        &certificate,
    );
    AuditDecision::Proven(certificate)
}

fn validate_alias_availability(
    target: &pen_kernel::GlobalId,
    availability: &PublicAvailabilityV1,
) -> Result<(), AuditFailure> {
    match availability {
        PublicAvailabilityV1::PredecessorPublicExport { target: available }
        | PublicAvailabilityV1::DependencyPriorExport { target: available }
            if available == target =>
        {
            Ok(())
        }
        PublicAvailabilityV1::DerivedFromPublicInterface { public_support, .. }
            if public_support.contains(target) =>
        {
            Ok(())
        }
        PublicAvailabilityV1::AmbientOnly => Ok(()),
        PublicAvailabilityV1::OutsideFragment => Err(AuditFailure::Outside(
            OutsideFragmentReason::UnsupportedTerm,
        )),
        PublicAvailabilityV1::Unknown => {
            Err(AuditFailure::Unknown(AuditUnknownReason::IncompleteSupport))
        }
        _ => Err(AuditFailure::Unknown(AuditUnknownReason::MalformedInput)),
    }
}

fn seed_raw_family(
    manifest: &VerifiedSemanticAuditManifestV1,
    seed: &VerifiedSemanticSeedV1,
) -> RawFamilyV1 {
    let constructor = match &seed.seed {
        SemanticSchemaSeedV1::PublicHead(_) => FamilyConstructorV1::PublicHeadSeed {
            seed: seed.id.clone(),
        },
        SemanticSchemaSeedV1::PublicEquation(_) => FamilyConstructorV1::PublicEquationSeed {
            seed: seed.id.clone(),
        },
        SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
            unreachable!("universal-interface seeds fail before carrier construction")
        }
    };
    let (support, source_clause, demand_anchor) = match &seed.seed {
        SemanticSchemaSeedV1::PublicHead(seed) => (
            seed.public_support.clone(),
            seed.source_clause.clone(),
            None,
        ),
        SemanticSchemaSeedV1::PublicEquation(seed) => (
            seed.public_support.clone(),
            seed.source_clause.clone(),
            seed.demand_anchor.clone(),
        ),
        SemanticSchemaSeedV1::PublicUniversalInterface { .. } => unreachable!(),
    };
    let action = substitution_action_digest(&seed.normalized_judgment, seed.derived_role);
    let id = raw_family_id(
        manifest.candidate_digest(),
        0,
        &constructor,
        &seed.normalized_judgment,
        seed.derived_role,
        &support,
        &action,
    );
    RawFamilyV1 {
        id,
        rank: 0,
        constructor,
        generic_judgment: seed.normalized_judgment.clone(),
        role: seed.derived_role,
        public_support: support,
        source_clause,
        demand_anchor,
        substitution_action_digest: action,
    }
}

fn pre_q0_seed_raw_family(
    manifest: &VerifiedSemanticAuditManifestV1,
    seed: &VerifiedPreQ0SemanticSeedV1,
) -> RawFamilyV1 {
    let constructor = match &seed.seed {
        SemanticSchemaSeedV1::PublicHead(_) => FamilyConstructorV1::PublicHeadSeed {
            seed: seed.id.clone(),
        },
        SemanticSchemaSeedV1::PublicEquation(_) => FamilyConstructorV1::PublicEquationSeed {
            seed: seed.id.clone(),
        },
        SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
            unreachable!("universal-interface seeds fail before carrier construction")
        }
    };
    let (support, source_clause, demand_anchor) = match &seed.seed {
        SemanticSchemaSeedV1::PublicHead(seed) => (
            seed.public_support.clone(),
            seed.source_clause.clone(),
            None,
        ),
        SemanticSchemaSeedV1::PublicEquation(seed) => (
            seed.public_support.clone(),
            seed.source_clause.clone(),
            seed.demand_anchor.clone(),
        ),
        SemanticSchemaSeedV1::PublicUniversalInterface { .. } => unreachable!(),
    };
    let judgment = seed.source_judgment.clone();
    let action = substitution_action_digest(&judgment, seed.derived_role);
    let id = raw_family_id(
        manifest.candidate_digest(),
        0,
        &constructor,
        &judgment,
        seed.derived_role,
        &support,
        &action,
    );
    RawFamilyV1 {
        id,
        rank: 0,
        constructor,
        generic_judgment: judgment,
        role: seed.derived_role,
        public_support: support,
        source_clause,
        demand_anchor,
        substitution_action_digest: action,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AuditFailure {
    Outside(OutsideFragmentReason),
    Unknown(AuditUnknownReason),
}

impl AuditFailure {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::Outside(reason) => AuditDecision::OutsideFragment(reason),
            Self::Unknown(reason) => AuditDecision::Unknown(reason),
        }
    }
}

enum BuildDisposition {
    CertifiedInapplicable(CertifiedInapplicableReasonV1),
    Abort(AuditFailure),
}

struct CarrierBuildContext<'a> {
    kernel: &'a Kernel,
    signature: &'a VerifiedSignature,
    manifest: &'a VerifiedSemanticAuditManifestV1,
    fresh: Option<&'a VerifiedFreshConstructorComputationV1>,
    normalization_mode: CarrierNormalizationMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CarrierNormalizationMode {
    PreQ0Syntactic,
    AuthorizedQ0,
}

type CarrierClosureV1 = (
    Vec<ContextAmalgamationWitnessV1>,
    Vec<CarrierTupleDispositionV1>,
    Vec<RawFamilyV1>,
);

fn enumerate_family_closure(
    build: &CarrierBuildContext<'_>,
    mut raw_families: Vec<RawFamilyV1>,
) -> Result<CarrierClosureV1, AuditFailure> {
    let limits = build.manifest.manifest();
    let mut context_witnesses = BTreeMap::new();
    let mut tuple_dispositions = Vec::new();
    for rank in 1..=limits.maximum_rank {
        let sources = raw_families.clone();
        for left in &sources {
            for right in &sources {
                if 1 + left.rank.max(right.rank) != rank {
                    continue;
                }
                let witnesses = enumerate_context_witnesses(
                    build.kernel,
                    build.signature,
                    limits.maximum_context_entries,
                    left.generic_judgment.context(),
                    right.generic_judgment.context(),
                    remaining_tuple_budget(limits.maximum_tuple_dispositions, &tuple_dispositions),
                )?;
                for witness in witnesses {
                    context_witnesses
                        .entry(witness.id.clone())
                        .or_insert_with(|| witness.clone());
                    match build_application(build, left, right, &witness, rank) {
                        Ok(Some(family)) => {
                            if !push_disposition(
                                &mut tuple_dispositions,
                                limits.maximum_tuple_dispositions,
                                CarrierTupleDispositionV1 {
                                    rank,
                                    rule: CarrierRuleV1::GenericPublicApplication,
                                    left: left.id.clone(),
                                    right: right.id.clone(),
                                    context_witness: witness.id.clone(),
                                    hole_ordinal: None,
                                    outcome: CarrierTupleOutcomeV1::Applicable {
                                        family: family.id.clone(),
                                    },
                                },
                            ) {
                                return Err(AuditFailure::Unknown(
                                    AuditUnknownReason::ResourceExhausted,
                                ));
                            }
                            if raw_families.len() >= limits.maximum_raw_derivations as usize {
                                return Err(AuditFailure::Unknown(
                                    AuditUnknownReason::ResourceExhausted,
                                ));
                            }
                            raw_families.push(family);
                        }
                        Ok(None) => {
                            unreachable!("inapplicable applications carry a typed reason")
                        }
                        Err(BuildDisposition::CertifiedInapplicable(reason)) => {
                            if !push_disposition(
                                &mut tuple_dispositions,
                                limits.maximum_tuple_dispositions,
                                CarrierTupleDispositionV1 {
                                    rank,
                                    rule: CarrierRuleV1::GenericPublicApplication,
                                    left: left.id.clone(),
                                    right: right.id.clone(),
                                    context_witness: witness.id.clone(),
                                    hole_ordinal: None,
                                    outcome: CarrierTupleOutcomeV1::CertifiedInapplicable {
                                        reason,
                                    },
                                },
                            ) {
                                return Err(AuditFailure::Unknown(
                                    AuditUnknownReason::ResourceExhausted,
                                ));
                            }
                        }
                        Err(BuildDisposition::Abort(failure)) => return Err(failure),
                    }

                    let equation_dispositions =
                        build_equation_actions(build, left, right, &witness, rank)?;
                    for (hole_ordinal, result) in equation_dispositions {
                        let outcome = match result {
                            Ok(family) => {
                                let id = family.id.clone();
                                if raw_families.len() >= limits.maximum_raw_derivations as usize {
                                    return Err(AuditFailure::Unknown(
                                        AuditUnknownReason::ResourceExhausted,
                                    ));
                                }
                                raw_families.push(family);
                                CarrierTupleOutcomeV1::Applicable { family: id }
                            }
                            Err(reason) => CarrierTupleOutcomeV1::CertifiedInapplicable { reason },
                        };
                        if !push_disposition(
                            &mut tuple_dispositions,
                            limits.maximum_tuple_dispositions,
                            CarrierTupleDispositionV1 {
                                rank,
                                rule: CarrierRuleV1::GenericEquationAction,
                                left: left.id.clone(),
                                right: right.id.clone(),
                                context_witness: witness.id.clone(),
                                hole_ordinal,
                                outcome,
                            },
                        ) {
                            return Err(AuditFailure::Unknown(
                                AuditUnknownReason::ResourceExhausted,
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok((
        context_witnesses.into_values().collect(),
        tuple_dispositions,
        raw_families,
    ))
}

fn build_application(
    build: &CarrierBuildContext<'_>,
    function: &RawFamilyV1,
    argument: &RawFamilyV1,
    witness: &ContextAmalgamationWitnessV1,
    rank: u16,
) -> Result<Option<RawFamilyV1>, BuildDisposition> {
    let function_judgment = rename_judgment_into(
        &function.generic_judgment,
        &witness.target_context,
        &witness.left_embedding,
    )
    .ok_or(BuildDisposition::Abort(AuditFailure::Unknown(
        AuditUnknownReason::KernelCouldNotCertify,
    )))?;
    let argument_judgment = rename_judgment_into(
        &argument.generic_judgment,
        &witness.target_context,
        &witness.right_embedding,
    )
    .ok_or(BuildDisposition::Abort(AuditFailure::Unknown(
        AuditUnknownReason::KernelCouldNotCertify,
    )))?;
    let (
        GenericJudgmentV1::Term {
            term: function_term,
            ty: function_type,
            ..
        },
        GenericJudgmentV1::Term {
            term: argument_term,
            ..
        },
    ) = (function_judgment, argument_judgment)
    else {
        return Err(BuildDisposition::CertifiedInapplicable(
            CertifiedInapplicableReasonV1::WrongJudgmentKind,
        ));
    };
    let Term::Pi { parameter: _, body } = function_type else {
        return Err(BuildDisposition::CertifiedInapplicable(
            CertifiedInapplicableReasonV1::FunctionConclusionIsNotPi,
        ));
    };
    let result_type = subst_top_unchecked(&body, &argument_term).ok_or(BuildDisposition::Abort(
        AuditFailure::Unknown(AuditUnknownReason::KernelCouldNotCertify),
    ))?;
    let candidate = GenericJudgmentV1::Term {
        context: witness.target_context.clone(),
        term: Term::Apply {
            function: Box::new(function_term),
            argument: Box::new(argument_term),
        },
        ty: result_type,
    };
    let ordinary = match build
        .kernel
        .verify_open_judgment(build.signature, &generic_to_open(&candidate))
    {
        Ok(normalized) => open_to_generic(normalized),
        Err(KernelError::TypeMismatch | KernelError::ExpectedFunction) => {
            return Err(BuildDisposition::CertifiedInapplicable(
                CertifiedInapplicableReasonV1::ArgumentTypeMismatch,
            ));
        }
        Err(error) => {
            return Err(BuildDisposition::Abort(kernel_error_failure(error)));
        }
    };
    let normalized = match build.normalization_mode {
        CarrierNormalizationMode::PreQ0Syntactic => candidate,
        CarrierNormalizationMode::AuthorizedQ0 => match normalize_generated_judgment_v1(
            build.manifest,
            build.kernel,
            build.signature,
            &ordinary,
            build.fresh,
        ) {
            AuditDecision::Proven(normalized) => normalized,
            AuditDecision::OutsideFragment(reason) => {
                return Err(BuildDisposition::Abort(AuditFailure::Outside(reason)));
            }
            AuditDecision::Unknown(reason) => {
                return Err(BuildDisposition::Abort(AuditFailure::Unknown(reason)));
            }
        },
    };
    if let Err(reason) = validate_supported_judgment(&normalized, build.manifest) {
        return Err(BuildDisposition::Abort(AuditFailure::Outside(reason)));
    }
    let constructor = FamilyConstructorV1::GenericPublicApplication {
        function: function.id.clone(),
        argument: argument.id.clone(),
        context_witness: witness.id.clone(),
    };
    let support = function.public_support.union(&argument.public_support);
    let role = LocalRoleV1::SupportAction;
    let action = substitution_action_digest(&normalized, role);
    let id = raw_family_id(
        build.manifest.candidate_digest(),
        rank,
        &constructor,
        &normalized,
        role,
        &support,
        &action,
    );
    Ok(Some(RawFamilyV1 {
        id,
        rank,
        constructor,
        generic_judgment: normalized,
        role,
        public_support: support,
        source_clause: common_option(&function.source_clause, &argument.source_clause),
        demand_anchor: common_option(&function.demand_anchor, &argument.demand_anchor),
        substitution_action_digest: action,
    }))
}

type EquationActionBuild = (
    Option<u32>,
    Result<RawFamilyV1, CertifiedInapplicableReasonV1>,
);

fn build_equation_actions(
    build: &CarrierBuildContext<'_>,
    equation: &RawFamilyV1,
    context_family: &RawFamilyV1,
    witness: &ContextAmalgamationWitnessV1,
    rank: u16,
) -> Result<Vec<EquationActionBuild>, AuditFailure> {
    let GenericJudgmentV1::Equation {
        left, right, ty: _, ..
    } = rename_judgment_into(
        &equation.generic_judgment,
        &witness.target_context,
        &witness.left_embedding,
    )
    .ok_or(AuditFailure::Unknown(
        AuditUnknownReason::KernelCouldNotCertify,
    ))?
    else {
        return Ok(vec![(
            None,
            Err(CertifiedInapplicableReasonV1::WrongJudgmentKind),
        )]);
    };
    if !matches!(
        context_family.constructor,
        FamilyConstructorV1::GenericPublicApplication { .. }
    ) {
        return Ok(vec![(
            None,
            Err(CertifiedInapplicableReasonV1::ContextIsNotPublicApplication),
        )]);
    }
    let GenericJudgmentV1::Term {
        term: context_term,
        ty: context_type,
        ..
    } = rename_judgment_into(
        &context_family.generic_judgment,
        &witness.target_context,
        &witness.right_embedding,
    )
    .ok_or(AuditFailure::Unknown(
        AuditUnknownReason::KernelCouldNotCertify,
    ))?
    else {
        return Ok(vec![(
            None,
            Err(CertifiedInapplicableReasonV1::WrongJudgmentKind),
        )]);
    };

    let node_count = term_node_count(&context_term)
        .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    let mut dispositions = Vec::with_capacity(node_count as usize);
    for hole_ordinal in 0..node_count {
        let left_term = replace_term_node(&context_term, hole_ordinal, &left).ok_or(
            AuditFailure::Unknown(AuditUnknownReason::IncompleteEnumeration),
        )?;
        let right_term = replace_term_node(&context_term, hole_ordinal, &right).ok_or(
            AuditFailure::Unknown(AuditUnknownReason::IncompleteEnumeration),
        )?;
        let candidate = GenericJudgmentV1::Equation {
            context: witness.target_context.clone(),
            left: left_term,
            right: right_term,
            ty: context_type.clone(),
        };
        let GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } = &candidate
        else {
            unreachable!("constructed an equation judgment");
        };
        let left_judgment = OpenJudgment::HasType {
            context: context.clone(),
            term: left.clone(),
            ty: ty.clone(),
        };
        let right_judgment = OpenJudgment::HasType {
            context: context.clone(),
            term: right.clone(),
            ty: ty.clone(),
        };
        match build
            .kernel
            .verify_open_judgments(build.signature, &[&left_judgment, &right_judgment])
        {
            Ok(_) => {
                let normalized = match build.normalization_mode {
                    CarrierNormalizationMode::PreQ0Syntactic => candidate,
                    CarrierNormalizationMode::AuthorizedQ0 => {
                        match normalize_generated_judgment_v1(
                            build.manifest,
                            build.kernel,
                            build.signature,
                            &candidate,
                            build.fresh,
                        ) {
                            AuditDecision::Proven(normalized) => normalized,
                            AuditDecision::OutsideFragment(reason) => {
                                return Err(AuditFailure::Outside(reason));
                            }
                            AuditDecision::Unknown(reason) => {
                                return Err(AuditFailure::Unknown(reason));
                            }
                        }
                    }
                };
                if let Err(reason) = validate_supported_judgment(&normalized, build.manifest) {
                    return Err(AuditFailure::Outside(reason));
                }
                let constructor = FamilyConstructorV1::GenericEquationAction {
                    equation: equation.id.clone(),
                    context: context_family.id.clone(),
                    hole_ordinal,
                    context_witness: witness.id.clone(),
                };
                let support = equation
                    .public_support
                    .union(&context_family.public_support);
                let role = LocalRoleV1::Coherence;
                let action = substitution_action_digest(&normalized, role);
                let id = raw_family_id(
                    build.manifest.candidate_digest(),
                    rank,
                    &constructor,
                    &normalized,
                    role,
                    &support,
                    &action,
                );
                dispositions.push((
                    Some(hole_ordinal),
                    Ok(RawFamilyV1 {
                        id,
                        rank,
                        constructor,
                        generic_judgment: normalized,
                        role,
                        public_support: support,
                        source_clause: equation.source_clause.clone(),
                        demand_anchor: equation.demand_anchor.clone(),
                        substitution_action_digest: action,
                    }),
                ));
            }
            Err(
                KernelError::ExpectedType
                | KernelError::ExpectedFunction
                | KernelError::ExpectedPair
                | KernelError::TypeMismatch,
            ) => dispositions.push((
                Some(hole_ordinal),
                Err(CertifiedInapplicableReasonV1::HoleIsNotTypedForEquation),
            )),
            Err(error) => return Err(kernel_error_failure(error)),
        }
    }
    Ok(dispositions)
}

fn enumerate_context_witnesses(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    maximum_context_entries: u16,
    left: &DependentContext,
    right: &DependentContext,
    remaining_dispositions: usize,
) -> Result<Vec<ContextAmalgamationWitnessV1>, AuditFailure> {
    if left.0.len() > usize::from(maximum_context_entries)
        || right.0.len() > usize::from(maximum_context_entries)
    {
        return Err(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted));
    }
    let mut witnesses = Vec::new();
    if left == right {
        let embedding = (0..left.0.len())
            .map(|ordinal| u32::try_from(ordinal).expect("context bound fits u32"))
            .collect::<Vec<_>>();
        witnesses.push(make_context_witness(
            ContextWitnessKindV1::IdenticalTelescope,
            left,
            right,
            left.clone(),
            embedding.clone(),
            embedding,
        ));
    }

    let combined_len = left
        .0
        .len()
        .checked_add(right.0.len())
        .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    if combined_len > usize::from(maximum_context_entries) {
        return Err(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted));
    }
    let interleaving_count = checked_binomial(combined_len, left.0.len())
        .filter(|count| *count <= remaining_dispositions)
        .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    let mut patterns = Vec::with_capacity(interleaving_count);
    enumerate_interleavings(
        left.0.len(),
        right.0.len(),
        &mut Vec::with_capacity(combined_len),
        &mut patterns,
    );
    if patterns.len() != interleaving_count {
        return Err(AuditFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ));
    }
    for pattern in patterns {
        let mut left_embedding = vec![0_u32; left.0.len()];
        let mut right_embedding = vec![0_u32; right.0.len()];
        let mut left_ordinal = 0;
        let mut right_ordinal = 0;
        for (target_ordinal, from_left) in pattern.iter().copied().enumerate() {
            let target_ordinal =
                u32::try_from(target_ordinal).expect("manifest context bound fits u32");
            if from_left {
                left_embedding[left_ordinal] = target_ordinal;
                left_ordinal += 1;
            } else {
                right_embedding[right_ordinal] = target_ordinal;
                right_ordinal += 1;
            }
        }
        let target_context =
            interleaved_context(left, right, &pattern, &left_embedding, &right_embedding).ok_or(
                AuditFailure::Unknown(AuditUnknownReason::KernelCouldNotCertify),
            )?;
        let normalized = match kernel.verify_context(signature, &target_context) {
            Ok(context) => context.normalized_wire(),
            Err(error) => return Err(kernel_error_failure(error)),
        };
        if normalized != target_context {
            return Err(AuditFailure::Unknown(
                AuditUnknownReason::NormalizationFailure,
            ));
        }
        witnesses.push(make_context_witness(
            ContextWitnessKindV1::DependencyRespectingDisjointInterleaving,
            left,
            right,
            target_context,
            left_embedding,
            right_embedding,
        ));
    }
    Ok(witnesses)
}

fn make_context_witness(
    kind: ContextWitnessKindV1,
    left_context: &DependentContext,
    right_context: &DependentContext,
    target_context: DependentContext,
    left_embedding: Vec<u32>,
    right_embedding: Vec<u32>,
) -> ContextAmalgamationWitnessV1 {
    struct Subject<'a> {
        kind: ContextWitnessKindV1,
        left_context: &'a DependentContext,
        right_context: &'a DependentContext,
        target_context: &'a DependentContext,
        left_embedding: &'a [u32],
        right_embedding: &'a [u32],
    }
    impl CanonicalEncode for Subject<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.kind.encode_canonical(encoder);
            self.left_context.encode_canonical(encoder);
            self.right_context.encode_canonical(encoder);
            self.target_context.encode_canonical(encoder);
            encode_u32_slice(encoder, self.left_embedding);
            encode_u32_slice(encoder, self.right_embedding);
        }
    }
    let id = ContextWitnessIdV1(Digest::of_canonical(
        "pen-semantic-audit/context-amalgamation/v1",
        &Subject {
            kind,
            left_context,
            right_context,
            target_context: &target_context,
            left_embedding: &left_embedding,
            right_embedding: &right_embedding,
        },
    ));
    ContextAmalgamationWitnessV1 {
        id,
        kind,
        left_context: left_context.clone(),
        right_context: right_context.clone(),
        target_context,
        left_embedding,
        right_embedding,
    }
}

fn interleaved_context(
    left: &DependentContext,
    right: &DependentContext,
    pattern: &[bool],
    left_embedding: &[u32],
    right_embedding: &[u32],
) -> Option<DependentContext> {
    let mut entries = Vec::with_capacity(pattern.len());
    let mut left_ordinal = 0;
    let mut right_ordinal = 0;
    for (target_ordinal, from_left) in pattern.iter().copied().enumerate() {
        let entry = if from_left {
            let entry = rename_term(
                &left.0[left_ordinal],
                left_ordinal,
                target_ordinal,
                left_embedding,
                0,
            )?;
            left_ordinal += 1;
            entry
        } else {
            let entry = rename_term(
                &right.0[right_ordinal],
                right_ordinal,
                target_ordinal,
                right_embedding,
                0,
            )?;
            right_ordinal += 1;
            entry
        };
        entries.push(entry);
    }
    Some(DependentContext(entries))
}

fn enumerate_interleavings(
    left_remaining: usize,
    right_remaining: usize,
    prefix: &mut Vec<bool>,
    output: &mut Vec<Vec<bool>>,
) {
    if left_remaining == 0 && right_remaining == 0 {
        output.push(prefix.clone());
        return;
    }
    if left_remaining > 0 {
        prefix.push(true);
        enumerate_interleavings(left_remaining - 1, right_remaining, prefix, output);
        prefix.pop();
    }
    if right_remaining > 0 {
        prefix.push(false);
        enumerate_interleavings(left_remaining, right_remaining - 1, prefix, output);
        prefix.pop();
    }
}

fn checked_binomial(n: usize, k: usize) -> Option<usize> {
    let k = k.min(n.checked_sub(k)?);
    let mut result = 1_u128;
    for i in 0..k {
        result = result
            .checked_mul((n - i) as u128)?
            .checked_div((i + 1) as u128)?;
        if result > usize::MAX as u128 {
            return None;
        }
    }
    usize::try_from(result).ok()
}

fn remaining_tuple_budget(maximum: u32, dispositions: &[CarrierTupleDispositionV1]) -> usize {
    (maximum as usize).saturating_sub(dispositions.len())
}

fn push_disposition(
    dispositions: &mut Vec<CarrierTupleDispositionV1>,
    maximum: u32,
    disposition: CarrierTupleDispositionV1,
) -> bool {
    if dispositions.len() >= maximum as usize {
        return false;
    }
    dispositions.push(disposition);
    true
}

fn common_option<T: Clone + Eq>(left: &Option<T>, right: &Option<T>) -> Option<T> {
    if left == right { left.clone() } else { None }
}

fn validate_supported_judgment(
    judgment: &GenericJudgmentV1,
    manifest: &VerifiedSemanticAuditManifestV1,
) -> Result<(), OutsideFragmentReason> {
    if manifest.manifest().profile_id == SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1 {
        return match lambda_unit_judgment_syntax_violation(
            judgment,
            &manifest.manifest().universe_levels,
        ) {
            Some(violation) => Err(violation.outside_reason()),
            None => Ok(()),
        };
    }
    for term in judgment_terms(judgment) {
        validate_supported_term(term, &manifest.manifest().universe_levels, false)?;
    }
    Ok(())
}

fn validate_supported_term(
    term: &Term,
    universe_levels: &[u16],
    projection_free_lambda_unit: bool,
) -> Result<(), OutsideFragmentReason> {
    match term {
        Term::Sort { level } => {
            if universe_levels.contains(level) {
                Ok(())
            } else {
                Err(OutsideFragmentReason::UnsupportedUniverseLevel)
            }
        }
        Term::Var { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => Ok(()),
        Term::Pi { parameter, body } => {
            validate_supported_term(parameter, universe_levels, projection_free_lambda_unit)?;
            validate_supported_term(body, universe_levels, projection_free_lambda_unit)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            validate_supported_term(parameter_type, universe_levels, projection_free_lambda_unit)?;
            validate_supported_term(body, universe_levels, projection_free_lambda_unit)
        }
        Term::Apply { function, argument } => {
            validate_supported_term(function, universe_levels, projection_free_lambda_unit)?;
            validate_supported_term(argument, universe_levels, projection_free_lambda_unit)
        }
        Term::First { .. } | Term::Second { .. } if projection_free_lambda_unit => {
            Err(OutsideFragmentReason::DescriptorProjection)
        }
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            Err(OutsideFragmentReason::UnsupportedTerm)
        }
    }
}

fn judgment_terms(judgment: &GenericJudgmentV1) -> Vec<&Term> {
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => {
            context.0.iter().chain([term, ty]).collect::<Vec<_>>()
        }
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => context
            .0
            .iter()
            .chain([left, right, ty])
            .collect::<Vec<_>>(),
    }
}

pub(crate) fn generic_to_open(judgment: &GenericJudgmentV1) -> OpenJudgment {
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => OpenJudgment::HasType {
            context: context.clone(),
            term: term.clone(),
            ty: ty.clone(),
        },
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => OpenJudgment::DefinitionallyEqual {
            context: context.clone(),
            left: left.clone(),
            right: right.clone(),
            ty: ty.clone(),
        },
    }
}

pub(crate) fn open_to_generic(judgment: OpenJudgment) -> GenericJudgmentV1 {
    match judgment {
        OpenJudgment::HasType { context, term, ty } => {
            GenericJudgmentV1::Term { context, term, ty }
        }
        OpenJudgment::DefinitionallyEqual {
            context,
            left,
            right,
            ty,
        } => GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        },
        OpenJudgment::TypeFormation { context, term } => GenericJudgmentV1::Term {
            context,
            term: term.clone(),
            ty: Term::Sort { level: 0 },
        },
    }
}

pub(crate) fn curry_judgment(judgment: &GenericJudgmentV1) -> GenericJudgmentV1 {
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => {
            let mut curried_term = term.clone();
            let mut curried_type = ty.clone();
            for parameter in context.0.iter().rev() {
                curried_term = Term::Lambda {
                    parameter_type: Box::new(parameter.clone()),
                    body: Box::new(curried_term),
                };
                curried_type = Term::Pi {
                    parameter: Box::new(parameter.clone()),
                    body: Box::new(curried_type),
                };
            }
            GenericJudgmentV1::Term {
                context: DependentContext::default(),
                term: curried_term,
                ty: curried_type,
            }
        }
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => {
            let mut curried_left = left.clone();
            let mut curried_right = right.clone();
            let mut curried_type = ty.clone();
            for parameter in context.0.iter().rev() {
                curried_left = Term::Lambda {
                    parameter_type: Box::new(parameter.clone()),
                    body: Box::new(curried_left),
                };
                curried_right = Term::Lambda {
                    parameter_type: Box::new(parameter.clone()),
                    body: Box::new(curried_right),
                };
                curried_type = Term::Pi {
                    parameter: Box::new(parameter.clone()),
                    body: Box::new(curried_type),
                };
            }
            GenericJudgmentV1::Equation {
                context: DependentContext::default(),
                left: curried_left,
                right: curried_right,
                ty: curried_type,
            }
        }
    }
}

pub(crate) fn substitution_action_digest(
    judgment: &GenericJudgmentV1,
    role: LocalRoleV1,
) -> Digest {
    struct Subject<'a> {
        judgment: &'a GenericJudgmentV1,
        role: LocalRoleV1,
    }
    impl CanonicalEncode for Subject<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.judgment.encode_canonical(encoder);
            self.role.encode_canonical(encoder);
        }
    }
    let curried = curry_judgment(judgment);
    Digest::of_canonical(
        "pen-semantic-audit/substitution-action/v1",
        &Subject {
            judgment: &curried,
            role,
        },
    )
}

pub(crate) fn rename_judgment_into(
    judgment: &GenericJudgmentV1,
    target_context: &DependentContext,
    embedding: &[u32],
) -> Option<GenericJudgmentV1> {
    let source_len = judgment.context().0.len();
    if embedding.len() != source_len {
        return None;
    }
    let target_len = target_context.0.len();
    match judgment {
        GenericJudgmentV1::Term { term, ty, .. } => Some(GenericJudgmentV1::Term {
            context: target_context.clone(),
            term: rename_term(term, source_len, target_len, embedding, 0)?,
            ty: rename_term(ty, source_len, target_len, embedding, 0)?,
        }),
        GenericJudgmentV1::Equation {
            left, right, ty, ..
        } => Some(GenericJudgmentV1::Equation {
            context: target_context.clone(),
            left: rename_term(left, source_len, target_len, embedding, 0)?,
            right: rename_term(right, source_len, target_len, embedding, 0)?,
            ty: rename_term(ty, source_len, target_len, embedding, 0)?,
        }),
    }
}

pub(crate) fn rename_judgment_by_permutation(
    judgment: &GenericJudgmentV1,
    target_context: &DependentContext,
    source_to_target: &[u32],
) -> Option<GenericJudgmentV1> {
    rename_judgment_into(judgment, target_context, source_to_target)
}

fn rename_term(
    term: &Term,
    source_context_len: usize,
    target_context_len: usize,
    source_to_target: &[u32],
    binder_depth: u32,
) -> Option<Term> {
    match term {
        Term::Sort { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => Some(term.clone()),
        Term::Var { index } if *index < binder_depth => Some(term.clone()),
        Term::Var { index } => {
            let free_index = usize::try_from(index.checked_sub(binder_depth)?).ok()?;
            let source_position = source_context_len.checked_sub(free_index.checked_add(1)?)?;
            let target_position = usize::try_from(*source_to_target.get(source_position)?).ok()?;
            if target_position >= target_context_len {
                return None;
            }
            let target_free_index =
                target_context_len.checked_sub(target_position.checked_add(1)?)?;
            Some(Term::Var {
                index: binder_depth.checked_add(u32::try_from(target_free_index).ok()?)?,
            })
        }
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(rename_term(
                parameter,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
            body: Box::new(rename_term(
                body,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(rename_term(
                parameter,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
            body: Box::new(rename_term(
                body,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(rename_term(
                parameter_type,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
            body: Box::new(rename_term(
                body,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(rename_term(
                function,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
            argument: Box::new(rename_term(
                argument,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(rename_term(
                sigma_type,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
            first: Box::new(rename_term(
                first,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
            second: Box::new(rename_term(
                second,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(rename_term(
                pair,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(rename_term(
                pair,
                source_context_len,
                target_context_len,
                source_to_target,
                binder_depth,
            )?),
        }),
    }
}

fn subst_top_unchecked(body: &Term, replacement: &Term) -> Option<Term> {
    let lifted = shift_term(replacement, 1, 0)?;
    let replaced = substitute_term(body, 0, &lifted, 0)?;
    shift_term(&replaced, -1, 0)
}

fn substitute_term(
    term: &Term,
    target: u32,
    replacement: &Term,
    binder_depth: u32,
) -> Option<Term> {
    let sought = target.checked_add(binder_depth)?;
    match term {
        Term::Var { index } if *index == sought => {
            shift_term(replacement, i64::from(binder_depth), 0)
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Some(term.clone()),
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(substitute_term(
                parameter,
                target,
                replacement,
                binder_depth,
            )?),
            body: Box::new(substitute_term(
                body,
                target,
                replacement,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(substitute_term(
                parameter,
                target,
                replacement,
                binder_depth,
            )?),
            body: Box::new(substitute_term(
                body,
                target,
                replacement,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(substitute_term(
                parameter_type,
                target,
                replacement,
                binder_depth,
            )?),
            body: Box::new(substitute_term(
                body,
                target,
                replacement,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(substitute_term(
                function,
                target,
                replacement,
                binder_depth,
            )?),
            argument: Box::new(substitute_term(
                argument,
                target,
                replacement,
                binder_depth,
            )?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(substitute_term(
                sigma_type,
                target,
                replacement,
                binder_depth,
            )?),
            first: Box::new(substitute_term(first, target, replacement, binder_depth)?),
            second: Box::new(substitute_term(second, target, replacement, binder_depth)?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(substitute_term(pair, target, replacement, binder_depth)?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(substitute_term(pair, target, replacement, binder_depth)?),
        }),
    }
}

fn shift_term(term: &Term, amount: i64, cutoff: u32) -> Option<Term> {
    match term {
        Term::Var { index } if *index >= cutoff => {
            let shifted = i64::from(*index).checked_add(amount)?;
            Some(Term::Var {
                index: u32::try_from(shifted).ok()?,
            })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Some(term.clone()),
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(shift_term(parameter, amount, cutoff)?),
            body: Box::new(shift_term(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(shift_term(parameter, amount, cutoff)?),
            body: Box::new(shift_term(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(shift_term(parameter_type, amount, cutoff)?),
            body: Box::new(shift_term(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(shift_term(function, amount, cutoff)?),
            argument: Box::new(shift_term(argument, amount, cutoff)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(shift_term(sigma_type, amount, cutoff)?),
            first: Box::new(shift_term(first, amount, cutoff)?),
            second: Box::new(shift_term(second, amount, cutoff)?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(shift_term(pair, amount, cutoff)?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(shift_term(pair, amount, cutoff)?),
        }),
    }
}

fn term_node_count(term: &Term) -> Option<u32> {
    fn add(left: u32, right: u32) -> Option<u32> {
        left.checked_add(right)
    }
    match term {
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Some(1),
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            add(1, add(term_node_count(parameter)?, term_node_count(body)?)?)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => add(
            1,
            add(term_node_count(parameter_type)?, term_node_count(body)?)?,
        ),
        Term::Apply { function, argument } => add(
            1,
            add(term_node_count(function)?, term_node_count(argument)?)?,
        ),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => add(
            1,
            add(
                term_node_count(sigma_type)?,
                add(term_node_count(first)?, term_node_count(second)?)?,
            )?,
        ),
        Term::First { pair } | Term::Second { pair } => add(1, term_node_count(pair)?),
    }
}

fn replace_term_node(term: &Term, target: u32, replacement: &Term) -> Option<Term> {
    fn recurse(term: &Term, target: u32, replacement: &Term, ordinal: &mut u32) -> Option<Term> {
        let here = *ordinal;
        *ordinal = ordinal.checked_add(1)?;
        if here == target {
            return Some(replacement.clone());
        }
        match term {
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit => Some(term.clone()),
            Term::Pi { parameter, body } => Some(Term::Pi {
                parameter: Box::new(recurse(parameter, target, replacement, ordinal)?),
                body: Box::new(recurse(body, target, replacement, ordinal)?),
            }),
            Term::Sigma { parameter, body } => Some(Term::Sigma {
                parameter: Box::new(recurse(parameter, target, replacement, ordinal)?),
                body: Box::new(recurse(body, target, replacement, ordinal)?),
            }),
            Term::Lambda {
                parameter_type,
                body,
            } => Some(Term::Lambda {
                parameter_type: Box::new(recurse(parameter_type, target, replacement, ordinal)?),
                body: Box::new(recurse(body, target, replacement, ordinal)?),
            }),
            Term::Apply { function, argument } => Some(Term::Apply {
                function: Box::new(recurse(function, target, replacement, ordinal)?),
                argument: Box::new(recurse(argument, target, replacement, ordinal)?),
            }),
            Term::Pair {
                sigma_type,
                first,
                second,
            } => Some(Term::Pair {
                sigma_type: Box::new(recurse(sigma_type, target, replacement, ordinal)?),
                first: Box::new(recurse(first, target, replacement, ordinal)?),
                second: Box::new(recurse(second, target, replacement, ordinal)?),
            }),
            Term::First { pair } => Some(Term::First {
                pair: Box::new(recurse(pair, target, replacement, ordinal)?),
            }),
            Term::Second { pair } => Some(Term::Second {
                pair: Box::new(recurse(pair, target, replacement, ordinal)?),
            }),
        }
    }
    let mut ordinal = 0;
    let result = recurse(term, target, replacement, &mut ordinal)?;
    (target < ordinal).then_some(result)
}

fn kernel_error_failure(error: KernelError) -> AuditFailure {
    match error {
        KernelError::ResourceExhausted(_) => {
            AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted)
        }
        _ => AuditFailure::Unknown(AuditUnknownReason::KernelCouldNotCertify),
    }
}

fn encode_u32_slice(encoder: &mut CanonicalEncoder, values: &[u32]) {
    encoder.u64(values.len() as u64);
    for value in values {
        encoder.u32(*value);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CarrierRuleV1, CarrierTupleOutcomeV1, CertifiedInapplicableReasonV1,
        enumerate_pre_q0_raw_families_v1, enumerate_raw_families_v1, generic_to_open,
        validate_supported_term, verify_pre_q0_semantic_seed_v1,
    };
    use crate::manifest::{
        AuditDecision, OutsideFragmentReason, proposed_semantic_audit_lambda_unit_manifest_v1,
        proposed_semantic_audit_manifest_v1, verify_semantic_audit_lambda_unit_manifest_v1,
        verify_semantic_audit_manifest_v1,
    };
    use crate::model::{
        EquationIdV1, EventIdV1, GenericJudgmentV1, HeadPresentationV1, LocalRoleV1,
        PublicEquationSeedV1, PublicHeadSeedV1, PublicSupportV1, SemanticSchemaSeedV1,
        SourceNormalizedJudgmentV1,
    };
    use pen_kernel::{
        Declaration, DependentContext, Digest, GlobalId, Kernel, KernelLimits, Term,
        UncheckedSignature,
    };

    fn id(label: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    fn event(label: &[u8]) -> EventIdV1 {
        EventIdV1(Digest::of_bytes(label))
    }

    #[test]
    fn two_specializations_remain_instances_of_one_seed_family() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let head = id(b"type-operator");
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: head.clone(),
                    ty: Term::Pi {
                        parameter: Box::new(Term::Sort { level: 0 }),
                        body: Box::new(Term::Sort { level: 0 }),
                    },
                    body: None,
                }],
            })
            .expect("signature");
        let judgment = GenericJudgmentV1::Term {
            context: DependentContext(vec![Term::Sort { level: 0 }]),
            term: Term::Apply {
                function: Box::new(Term::Global { id: head.clone() }),
                argument: Box::new(Term::Var { index: 0 }),
            },
            ty: Term::Sort { level: 0 },
        };
        let seed = SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
            declaration: head,
            origin_event: event(b"event"),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: Digest::of_bytes(b"source"),
                source: judgment.clone(),
                claimed_normalized: judgment,
            },
            presentation: HeadPresentationV1::Opaque,
            claimed_role: LocalRoleV1::KernelHead,
            public_support: PublicSupportV1::default(),
            source_clause: None,
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
        let seed_family = carrier
            .raw_families
            .iter()
            .find(|family| family.rank == 0)
            .expect("seed family");
        let open = generic_to_open(&seed_family.generic_judgment);
        let first = kernel
            .normalize_closed_specialization(&signature, &open, &[Term::UnitType])
            .expect("first instance");
        let second = kernel
            .normalize_closed_specialization(
                &signature,
                &open,
                &[Term::Pi {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::UnitType),
                }],
            )
            .expect("second instance");
        assert_ne!(first, second);
        assert_eq!(
            carrier
                .raw_families
                .iter()
                .filter(|family| family.rank == 0)
                .count(),
            1
        );
    }

    #[test]
    fn nonempty_q3_and_reserved_universal_seed_fail_closed() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("signature");
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("manifest");
        };
        assert!(matches!(
            enumerate_raw_families_v1(
                &kernel,
                &signature,
                &manifest,
                &[],
                &[Digest::of_bytes(b"edge")]
            ),
            AuditDecision::OutsideFragment(OutsideFragmentReason::NonEmptyQ3Registry)
        ));

        let judgment = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        };
        let universal = SemanticSchemaSeedV1::PublicUniversalInterface {
            interface_digest: Digest::of_bytes(b"interface"),
            origin_event: event(b"event"),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: Digest::of_bytes(b"source"),
                source: judgment.clone(),
                claimed_normalized: judgment,
            },
        };
        assert!(matches!(
            enumerate_raw_families_v1(&kernel, &signature, &manifest, &[universal], &[]),
            AuditDecision::OutsideFragment(OutsideFragmentReason::UniversalInterface)
        ));
    }

    #[test]
    fn unsupported_path_surrogate_is_not_deserializable() {
        let json = r#"{
            "kind":"term",
            "context":[],
            "term":{"form":"path","left":{"form":"unit"},"right":{"form":"unit"}},
            "ty":{"form":"unit_type"}
        }"#;
        assert!(serde_json::from_str::<GenericJudgmentV1>(json).is_err());
    }

    #[test]
    fn pre_q0_carrier_retains_raw_source_and_ignores_claimed_normal_form() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let head = id(b"raw-head");
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: head.clone(),
                    ty: Term::UnitType,
                    body: None,
                }],
            })
            .expect("signature");
        let raw = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Unit),
            },
            ty: Term::UnitType,
        };
        let forged_claim = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Global { id: head.clone() },
            ty: Term::UnitType,
        };
        let seed = SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
            declaration: head,
            origin_event: event(b"raw-event"),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: Digest::of_bytes(b"raw-source"),
                source: raw.clone(),
                claimed_normalized: forged_claim,
            },
            presentation: HeadPresentationV1::Opaque,
            claimed_role: LocalRoleV1::KernelHead,
            public_support: PublicSupportV1::default(),
            source_clause: None,
        });
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("manifest");
        };
        let AuditDecision::Proven(carrier) =
            enumerate_pre_q0_raw_families_v1(&kernel, &signature, &manifest, &[seed], &[])
        else {
            panic!("pre-Q0 carrier");
        };
        assert_eq!(carrier.raw_families()[0].generic_judgment, raw);
        let SemanticSchemaSeedV1::PublicHead(stored) = carrier.verified_seeds()[0].seed() else {
            panic!("head seed");
        };
        assert_eq!(stored.judgment.claimed_normalized, stored.judgment.source);
    }

    #[test]
    fn pre_q0_equation_checks_typing_without_assuming_rewrite_authority() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let operation = id(b"opaque-operation");
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: operation.clone(),
                    ty: Term::Pi {
                        parameter: Box::new(Term::UnitType),
                        body: Box::new(Term::UnitType),
                    },
                    body: None,
                }],
            })
            .expect("signature");
        let equation = GenericJudgmentV1::Equation {
            context: DependentContext(vec![Term::UnitType]),
            left: Term::Apply {
                function: Box::new(Term::Global {
                    id: operation.clone(),
                }),
                argument: Box::new(Term::Var { index: 0 }),
            },
            right: Term::Var { index: 0 },
            ty: Term::UnitType,
        };
        let seed = SemanticSchemaSeedV1::PublicEquation(PublicEquationSeedV1 {
            equation: EquationIdV1(Digest::of_bytes(b"fresh-equation")),
            owner_head: operation,
            origin_event: event(b"equation-event"),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: Digest::of_bytes(b"equation-source"),
                source: equation.clone(),
                claimed_normalized: equation.clone(),
            },
            claimed_role: LocalRoleV1::Coherence,
            public_support: PublicSupportV1::default(),
            source_clause: None,
            demand_anchor: None,
        });
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("manifest");
        };
        let AuditDecision::Proven(verified) =
            verify_pre_q0_semantic_seed_v1(&kernel, &signature, &manifest, &seed)
        else {
            panic!("typed fresh equation should enter the raw carrier");
        };
        assert_eq!(verified.source_judgment(), &equation);
    }

    #[test]
    fn pre_q0_ill_typed_equation_hole_is_a_certified_negative() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let constructor = id(b"pre-q0-hole-constructor");
        let operation = id(b"pre-q0-hole-operation");
        let operation_type = Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            }),
        };
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: constructor.clone(),
                        ty: Term::UnitType,
                        body: None,
                    },
                    Declaration {
                        id: operation.clone(),
                        ty: operation_type.clone(),
                        body: None,
                    },
                ],
            })
            .expect("signature");
        let source_judgment = |term, ty, label: &[u8]| {
            let source = GenericJudgmentV1::Term {
                context: DependentContext::default(),
                term,
                ty,
            };
            SourceNormalizedJudgmentV1 {
                source_identity: Digest::of_bytes(label),
                source: source.clone(),
                claimed_normalized: source,
            }
        };
        let equation = GenericJudgmentV1::Equation {
            context: DependentContext(vec![Term::UnitType]),
            left: Term::Apply {
                function: Box::new(Term::Apply {
                    function: Box::new(Term::Global {
                        id: operation.clone(),
                    }),
                    argument: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Global {
                    id: constructor.clone(),
                }),
            },
            right: Term::Var { index: 0 },
            ty: Term::UnitType,
        };
        let seeds = vec![
            SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
                declaration: constructor.clone(),
                origin_event: event(b"pre-q0-hole-predecessor"),
                judgment: source_judgment(
                    Term::Global {
                        id: constructor.clone(),
                    },
                    Term::UnitType,
                    b"pre-q0-hole-constructor-source",
                ),
                presentation: HeadPresentationV1::Opaque,
                claimed_role: LocalRoleV1::KernelHead,
                public_support: PublicSupportV1::default(),
                source_clause: None,
            }),
            SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
                declaration: operation.clone(),
                origin_event: event(b"pre-q0-hole-successor"),
                judgment: source_judgment(
                    Term::Global {
                        id: operation.clone(),
                    },
                    operation_type,
                    b"pre-q0-hole-operation-source",
                ),
                presentation: HeadPresentationV1::Opaque,
                claimed_role: LocalRoleV1::KernelHead,
                public_support: PublicSupportV1::default(),
                source_clause: None,
            }),
            SemanticSchemaSeedV1::PublicEquation(PublicEquationSeedV1 {
                equation: EquationIdV1(Digest::of_bytes(b"pre-q0-hole-equation")),
                owner_head: operation,
                origin_event: event(b"pre-q0-hole-successor"),
                judgment: SourceNormalizedJudgmentV1 {
                    source_identity: Digest::of_bytes(b"pre-q0-hole-equation-source"),
                    source: equation.clone(),
                    claimed_normalized: equation,
                },
                claimed_role: LocalRoleV1::Coherence,
                public_support: PublicSupportV1::default(),
                source_clause: None,
                demand_anchor: None,
            }),
        ];
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("lambda/unit manifest");
        };
        let AuditDecision::Proven(carrier) =
            enumerate_pre_q0_raw_families_v1(&kernel, &signature, &manifest, &seeds, &[])
        else {
            panic!("ill-typed hole must not abort the complete carrier");
        };
        assert!(carrier.tuple_dispositions().iter().any(|disposition| {
            disposition.rank == 2
                && disposition.rule == CarrierRuleV1::GenericEquationAction
                && disposition.hole_ordinal == Some(1)
                && matches!(
                    &disposition.outcome,
                    CarrierTupleOutcomeV1::CertifiedInapplicable {
                        reason: CertifiedInapplicableReasonV1::HoleIsNotTypedForEquation
                    }
                )
        }));
    }

    #[test]
    fn lambda_unit_projection_syntax_has_the_exact_outside_reason() {
        let projection = Term::First {
            pair: Box::new(Term::Unit),
        };
        assert_eq!(
            validate_supported_term(&projection, &[0, 1], true),
            Err(OutsideFragmentReason::DescriptorProjection)
        );
        assert_eq!(
            validate_supported_term(&projection, &[0, 1], false),
            Err(OutsideFragmentReason::UnsupportedTerm)
        );
    }
}
