//! Finite, derivation-local substitutions for the lambda/unit V2 raw carrier.
//!
//! This module deliberately does not enumerate the category of typed
//! substitutions.  It reconstructs only the substitutions consumed by an
//! already enumerated rank-at-most-two carrier tuple.  General identity,
//! composition, lifting, and naturality remain theorem obligations.

use crate::carrier::{
    CarrierRuleV1, CarrierTupleDispositionV1, CarrierTupleOutcomeV1, ContextAmalgamationWitnessV1,
    PreQ0RawCarrierCertificateV1, rename_judgment_into,
};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, ConstructionSubstitutionRuleV2,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1, SubstitutionCensusScopeV2,
    VerifiedSemanticAuditManifestV1, VerifiedSemanticAuditManifestV2,
};
use crate::model::{FamilyConstructorV1, GenericJudgmentV1, RawFamilyIdV1, RawFamilyV1};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError, OpenJudgment,
    Term, VerifiedSignature,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// The leg of a registered context-amalgamation witness used by a tuple.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingSideV2 {
    Left,
    Right,
}

impl CanonicalEncode for EmbeddingSideV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Left => 0,
            Self::Right => 1,
        });
    }
}

/// Which side of an equation supplies a one-hole filler.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EquationFillerSideV2 {
    Left,
    Right,
}

impl CanonicalEncode for EquationFillerSideV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Left => 0,
            Self::Right => 1,
        });
    }
}

macro_rules! private_digest_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(Digest);

        impl $name {
            pub fn digest(&self) -> &Digest {
                &self.0
            }
        }

        impl CanonicalEncode for $name {
            fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                self.0.encode_canonical(encoder);
            }
        }
    };
}

private_digest_id!(CarrierTupleIdV2);
private_digest_id!(ConstructionContextIdV2);
private_digest_id!(OneHoleContextIdV2);
private_digest_id!(ConstructionSubstitutionIdV2);

/// Exactly the three direct construction forms admitted by the V2 census.
///
/// There is intentionally no constructor for a globally registered
/// composition.  If two maps are combined while reconstructing a tuple, only
/// their resulting image vector is bound to that tuple-local witness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ConstructionSubstitutionKindV2 {
    ContextEmbedding {
        witness: crate::model::ContextWitnessIdV1,
        side: EmbeddingSideV2,
    },
    ForcedNewestArgument {
        source_context: ConstructionContextIdV2,
        argument_family: RawFamilyIdV1,
    },
    OneHoleInstantiation {
        one_hole_context: OneHoleContextIdV2,
        filler_family: RawFamilyIdV1,
    },
}

impl CanonicalEncode for ConstructionSubstitutionKindV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::ContextEmbedding { witness, side } => {
                encoder.tag(0);
                witness.encode_canonical(encoder);
                side.encode_canonical(encoder);
            }
            Self::ForcedNewestArgument {
                source_context,
                argument_family,
            } => {
                encoder.tag(1);
                source_context.encode_canonical(encoder);
                argument_family.encode_canonical(encoder);
            }
            Self::OneHoleInstantiation {
                one_hole_context,
                filler_family,
            } => {
                encoder.tag(2);
                one_hole_context.encode_canonical(encoder);
                filler_family.encode_canonical(encoder);
            }
        }
    }
}

/// A verifier-minted simultaneous substitution used by one carrier tuple.
///
/// Images are complete and stored oldest declaration first.  The tuple
/// identity is part of the witness identity, so equal image vectors consumed
/// by different derivations are not collapsed into a global carrier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedDirectConstructionSubstitutionV2 {
    id: ConstructionSubstitutionIdV2,
    carrier_tuple: CarrierTupleIdV2,
    kind: ConstructionSubstitutionKindV2,
    source_context: DependentContext,
    target_context: DependentContext,
    images: Vec<Term>,
    one_hole_template: Option<Term>,
    equation_filler_side: Option<EquationFillerSideV2>,
    result_family: Option<RawFamilyIdV1>,
    kernel_replay_digest: Digest,
}

impl VerifiedDirectConstructionSubstitutionV2 {
    pub fn id(&self) -> &ConstructionSubstitutionIdV2 {
        &self.id
    }

    pub fn carrier_tuple(&self) -> &CarrierTupleIdV2 {
        &self.carrier_tuple
    }

    pub fn kind(&self) -> &ConstructionSubstitutionKindV2 {
        &self.kind
    }

    pub fn source_context(&self) -> &DependentContext {
        &self.source_context
    }

    pub fn target_context(&self) -> &DependentContext {
        &self.target_context
    }

    /// Complete source-variable images in oldest-first declaration order.
    pub fn images(&self) -> &[Term] {
        &self.images
    }

    /// Shifted one-hole template under the source context, when this is an
    /// equation-action instantiation.
    pub fn one_hole_template(&self) -> Option<&Term> {
        self.one_hole_template.as_ref()
    }

    pub fn equation_filler_side(&self) -> Option<EquationFillerSideV2> {
        self.equation_filler_side
    }

    pub fn result_family(&self) -> Option<&RawFamilyIdV1> {
        self.result_family.as_ref()
    }

    pub fn kernel_replay_digest(&self) -> &Digest {
        &self.kernel_replay_digest
    }
}

impl CanonicalEncode for VerifiedDirectConstructionSubstitutionV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.carrier_tuple.encode_canonical(encoder);
        self.kind.encode_canonical(encoder);
        self.source_context.encode_canonical(encoder);
        self.target_context.encode_canonical(encoder);
        encoder.sequence(&self.images);
        encoder.option(&self.one_hole_template);
        encoder.option(&self.equation_filler_side);
        encoder.option(&self.result_family);
        self.kernel_replay_digest.encode_canonical(encoder);
    }
}

/// Complete direct-witness census relative to one existing pre-Q0 carrier.
///
/// This capability proves only that every stored carrier tuple has the direct
/// construction witnesses dictated by its derivation.  It does not prove that
/// caller-provided V1 seeds form the complete semantic seed census.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedConstructionSubstitutionCensusV2 {
    semantic_manifest_digest: Digest,
    source_carrier_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    tuple_ids: Vec<CarrierTupleIdV2>,
    witnesses: Vec<VerifiedDirectConstructionSubstitutionV2>,
    coverage_digest: Digest,
    digest: Digest,
}

impl VerifiedConstructionSubstitutionCensusV2 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn source_carrier_digest(&self) -> &Digest {
        &self.source_carrier_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn tuple_ids(&self) -> &[CarrierTupleIdV2] {
        &self.tuple_ids
    }

    pub fn witnesses(&self) -> &[VerifiedDirectConstructionSubstitutionV2] {
        &self.witnesses
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedConstructionSubstitutionCensusV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.source_carrier_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        encoder.sequence(&self.tuple_ids);
        encoder.sequence(&self.witnesses);
        self.coverage_digest.encode_canonical(encoder);
    }
}

/// Placeholder capability whose fields cannot be caller-constructed.
///
/// No function in this module mints it.  It exists to make the fail-closed
/// completion gate explicit without confusing a relative witness census with
/// a complete carrier theorem.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedRankInductiveCarrierV2 {
    semantic_seed_census_digest: Digest,
    construction_substitution_census_digest: Digest,
    carrier_digest: Digest,
}

impl VerifiedRankInductiveCarrierV2 {
    pub fn semantic_seed_census_digest(&self) -> &Digest {
        &self.semantic_seed_census_digest
    }

    pub fn construction_substitution_census_digest(&self) -> &Digest {
        &self.construction_substitution_census_digest
    }

    pub fn carrier_digest(&self) -> &Digest {
        &self.carrier_digest
    }
}

/// Legacy upgrade gate for a witness census built over the V1 carrier.
///
/// A complete V2 seed census now exists for compatible empty-demand
/// inventories, but this relative census still uses V1 seed/family identities.
/// A native V2 rank-inductive reconstruction (or an exact verified
/// correspondence) is required before full carrier authority can be minted.
pub fn attempt_rank_inductive_carrier_v2(
    _census: &VerifiedConstructionSubstitutionCensusV2,
) -> AuditDecision<VerifiedRankInductiveCarrierV2> {
    AuditDecision::Unknown(AuditUnknownReason::MissingRankInductiveCarrierTheorem)
}

/// Reconstruct every direct construction substitution consumed by an
/// existing finite rank-0/1/2 raw-carrier derivation.
///
/// No compatible-pair loop or fixed-point closure occurs here.  Every witness
/// is emitted while visiting its unique carrier tuple.
pub fn verify_construction_substitution_census_v2(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    manifest: &VerifiedSemanticAuditManifestV2,
    carrier: &PreQ0RawCarrierCertificateV1,
) -> AuditDecision<VerifiedConstructionSubstitutionCensusV2> {
    if manifest.manifest().substitution_census_scope
        != SubstitutionCensusScopeV2::DirectConstructionWitnessesOnly
        || manifest.manifest().construction_substitution_rules
            != [
                ConstructionSubstitutionRuleV2::ContextEmbedding,
                ConstructionSubstitutionRuleV2::ForcedNewestArgument,
                ConstructionSubstitutionRuleV2::OneHoleInstantiation,
            ]
        || manifest.manifest().maximum_rank != 2
        || v1_manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1
        || carrier.manifest_digest() != v1_manifest.candidate_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    if signature.digest() != carrier.signature_digest()
        || kernel.kernel_protocol_digest() != *carrier.kernel_protocol_digest()
        || !carrier.q3_registry_verified_empty()
        || Digest::of_canonical("pen-semantic-audit/verified-pre-q0-raw-carrier/v1", carrier)
            != *carrier.digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    }

    let family_by_id = match unique_families(carrier.raw_families()) {
        Some(families) => families,
        None => return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision),
    };
    let witness_by_id = match unique_context_witnesses(carrier.context_witnesses()) {
        Some(witnesses) => witnesses,
        None => return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision),
    };
    if family_by_id
        .values()
        .any(|family| family.rank > manifest.manifest().maximum_rank)
    {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    }

    let mut tuple_ids = Vec::with_capacity(carrier.tuple_dispositions().len());
    let mut tuple_id_set = BTreeSet::new();
    let mut direct_witnesses = Vec::new();
    let mut witnessed_result_families = BTreeSet::new();

    for disposition in carrier.tuple_dispositions() {
        let tuple_id = CarrierTupleIdV2(Digest::of_canonical(
            "pen-semantic-audit/carrier-tuple/v2",
            disposition,
        ));
        if !tuple_id_set.insert(tuple_id.clone()) {
            return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
        }
        let Some(left) = family_by_id.get(&disposition.left).copied() else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        let Some(right) = family_by_id.get(&disposition.right).copied() else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        let Some(context_witness) = witness_by_id.get(&disposition.context_witness).copied() else {
            return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
        };
        if left.generic_judgment.context() != &context_witness.left_context
            || right.generic_judgment.context() != &context_witness.right_context
        {
            return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
        }

        let result_family = match &disposition.outcome {
            CarrierTupleOutcomeV1::Applicable { family } => {
                let Some(result) = family_by_id.get(family).copied() else {
                    return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
                };
                if result.rank != disposition.rank
                    || !witnessed_result_families.insert(family.clone())
                {
                    return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
                }
                Some(result)
            }
            CarrierTupleOutcomeV1::CertifiedInapplicable { .. } => None,
        };

        for side in [EmbeddingSideV2::Left, EmbeddingSideV2::Right] {
            let (source_context, embedding) = match side {
                EmbeddingSideV2::Left => (
                    context_witness.left_context.clone(),
                    context_witness.left_embedding.clone(),
                ),
                EmbeddingSideV2::Right => (
                    context_witness.right_context.clone(),
                    context_witness.right_embedding.clone(),
                ),
            };
            let Some(images) = embedding_images(&context_witness.target_context, &embedding) else {
                return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
            };
            if !embedding_respects_context(
                &source_context,
                &context_witness.target_context,
                &embedding,
                &images,
            ) {
                return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
            }
            if let Err(reason) = verify_simultaneous_substitution(
                kernel,
                signature,
                &source_context,
                &context_witness.target_context,
                &images,
            ) {
                return AuditDecision::Unknown(reason);
            }
            direct_witnesses.push(make_direct_witness(
                tuple_id.clone(),
                ConstructionSubstitutionKindV2::ContextEmbedding {
                    witness: context_witness.id.clone(),
                    side,
                },
                source_context,
                context_witness.target_context.clone(),
                images,
                None,
                None,
                result_family.map(|family| family.id.clone()),
                kernel.kernel_protocol_digest(),
            ));
        }

        if let Some(result_family) = result_family {
            let additional = match disposition.rule {
                CarrierRuleV1::GenericPublicApplication => reconstruct_application_witness(
                    kernel,
                    signature,
                    disposition,
                    &tuple_id,
                    (left, right),
                    result_family,
                    context_witness,
                ),
                CarrierRuleV1::GenericEquationAction => reconstruct_equation_witnesses(
                    kernel,
                    signature,
                    disposition,
                    &tuple_id,
                    (left, right),
                    result_family,
                    context_witness,
                ),
            };
            match additional {
                Ok(mut witnesses) => direct_witnesses.append(&mut witnesses),
                Err(reason) => return AuditDecision::Unknown(reason),
            }
        }
        tuple_ids.push(tuple_id);
    }

    let derived_families = family_by_id
        .values()
        .filter(|family| family.rank > 0)
        .map(|family| family.id.clone())
        .collect::<BTreeSet<_>>();
    if witnessed_result_families != derived_families {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }

    direct_witnesses.sort_by(|left, right| left.id.cmp(&right.id));
    if direct_witnesses
        .windows(2)
        .any(|pair| pair[0].id == pair[1].id)
    {
        return AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision);
    }
    tuple_ids.sort();
    let coverage_digest = Digest::of_canonical(
        "pen-semantic-audit/direct-construction-coverage/v2",
        &DirectCoverage {
            tuples: &tuple_ids,
            witnesses: &direct_witnesses,
        },
    );
    let mut census = VerifiedConstructionSubstitutionCensusV2 {
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        source_carrier_digest: carrier.digest().clone(),
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        tuple_ids,
        witnesses: direct_witnesses,
        coverage_digest,
        digest: Digest::of_bytes(b"pending-direct-construction-substitution-census-v2"),
    };
    census.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-construction-substitution-census/v2",
        &census,
    );
    AuditDecision::Proven(census)
}

struct DirectCoverage<'a> {
    tuples: &'a [CarrierTupleIdV2],
    witnesses: &'a [VerifiedDirectConstructionSubstitutionV2],
}

impl CanonicalEncode for DirectCoverage<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.tuples);
        encoder.sequence(self.witnesses);
    }
}

fn unique_families(families: &[RawFamilyV1]) -> Option<BTreeMap<RawFamilyIdV1, &RawFamilyV1>> {
    let mut by_id = BTreeMap::new();
    for family in families {
        if by_id.insert(family.id.clone(), family).is_some() {
            return None;
        }
    }
    Some(by_id)
}

fn unique_context_witnesses(
    witnesses: &[ContextAmalgamationWitnessV1],
) -> Option<BTreeMap<crate::model::ContextWitnessIdV1, &ContextAmalgamationWitnessV1>> {
    let mut by_id = BTreeMap::new();
    for witness in witnesses {
        if by_id.insert(witness.id.clone(), witness).is_some() {
            return None;
        }
    }
    Some(by_id)
}

fn reconstruct_application_witness(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    disposition: &CarrierTupleDispositionV1,
    tuple_id: &CarrierTupleIdV2,
    source_families: (&RawFamilyV1, &RawFamilyV1),
    result_family: &RawFamilyV1,
    context_witness: &ContextAmalgamationWitnessV1,
) -> Result<Vec<VerifiedDirectConstructionSubstitutionV2>, AuditUnknownReason> {
    let (function_family, argument_family) = source_families;
    if disposition.hole_ordinal.is_some()
        || result_family.constructor
            != (FamilyConstructorV1::GenericPublicApplication {
                function: function_family.id.clone(),
                argument: argument_family.id.clone(),
                context_witness: context_witness.id.clone(),
            })
    {
        return Err(AuditUnknownReason::MalformedInput);
    }
    let Some(GenericJudgmentV1::Term {
        term: function,
        ty: function_type,
        ..
    }) = rename_judgment_into(
        &function_family.generic_judgment,
        &context_witness.target_context,
        &context_witness.left_embedding,
    )
    else {
        return Err(AuditUnknownReason::MalformedInput);
    };
    let Some(GenericJudgmentV1::Term {
        term: argument,
        ty: _,
        ..
    }) = rename_judgment_into(
        &argument_family.generic_judgment,
        &context_witness.target_context,
        &context_witness.right_embedding,
    )
    else {
        return Err(AuditUnknownReason::MalformedInput);
    };
    let Term::Pi { parameter, body } = function_type else {
        return Err(AuditUnknownReason::MalformedInput);
    };
    if let Err(error) = kernel.verify_open_judgment(
        signature,
        &OpenJudgment::HasType {
            context: context_witness.target_context.clone(),
            term: argument.clone(),
            ty: (*parameter).clone(),
        },
    ) {
        return Err(kernel_unknown(error));
    }
    let mut source_context = context_witness.target_context.clone();
    source_context.0.push((*parameter).clone());
    let mut images = identity_images(&context_witness.target_context)?;
    images.push(argument.clone());
    verify_simultaneous_substitution(
        kernel,
        signature,
        &source_context,
        &context_witness.target_context,
        &images,
    )?;
    let Some(result_type) = instantiate_term_images(&body, source_context.0.len(), &images, 0)
    else {
        return Err(AuditUnknownReason::KernelCouldNotCertify);
    };
    let expected = GenericJudgmentV1::Term {
        context: context_witness.target_context.clone(),
        term: Term::Apply {
            function: Box::new(function),
            argument: Box::new(argument),
        },
        ty: result_type,
    };
    if result_family.generic_judgment != expected {
        return Err(AuditUnknownReason::MalformedInput);
    }
    let source_context_id = ConstructionContextIdV2(Digest::of_canonical(
        "pen-semantic-audit/construction-source-context/v2",
        &source_context,
    ));
    Ok(vec![make_direct_witness(
        tuple_id.clone(),
        ConstructionSubstitutionKindV2::ForcedNewestArgument {
            source_context: source_context_id,
            argument_family: argument_family.id.clone(),
        },
        source_context,
        context_witness.target_context.clone(),
        images,
        None,
        None,
        Some(result_family.id.clone()),
        kernel.kernel_protocol_digest(),
    )])
}

fn reconstruct_equation_witnesses(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    disposition: &CarrierTupleDispositionV1,
    tuple_id: &CarrierTupleIdV2,
    source_families: (&RawFamilyV1, &RawFamilyV1),
    result_family: &RawFamilyV1,
    context_witness: &ContextAmalgamationWitnessV1,
) -> Result<Vec<VerifiedDirectConstructionSubstitutionV2>, AuditUnknownReason> {
    let (equation_family, context_family) = source_families;
    let Some(hole_ordinal) = disposition.hole_ordinal else {
        return Err(AuditUnknownReason::MalformedInput);
    };
    if result_family.constructor
        != (FamilyConstructorV1::GenericEquationAction {
            equation: equation_family.id.clone(),
            context: context_family.id.clone(),
            hole_ordinal,
            context_witness: context_witness.id.clone(),
        })
    {
        return Err(AuditUnknownReason::MalformedInput);
    }
    let Some(GenericJudgmentV1::Equation {
        left, right, ty, ..
    }) = rename_judgment_into(
        &equation_family.generic_judgment,
        &context_witness.target_context,
        &context_witness.left_embedding,
    )
    else {
        return Err(AuditUnknownReason::MalformedInput);
    };
    let Some(GenericJudgmentV1::Term {
        term: context_term,
        ty: context_type,
        ..
    }) = rename_judgment_into(
        &context_family.generic_judgment,
        &context_witness.target_context,
        &context_witness.right_embedding,
    )
    else {
        return Err(AuditUnknownReason::MalformedInput);
    };
    let Some(left_result) = replace_term_node(&context_term, hole_ordinal, &left) else {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    };
    let Some(right_result) = replace_term_node(&context_term, hole_ordinal, &right) else {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    };
    let expected = GenericJudgmentV1::Equation {
        context: context_witness.target_context.clone(),
        left: left_result,
        right: right_result,
        ty: context_type.clone(),
    };
    if result_family.generic_judgment != expected {
        return Err(AuditUnknownReason::MalformedInput);
    }

    let mut source_context = context_witness.target_context.clone();
    source_context.0.push(ty);
    let Some(shifted_context_term) = shift_term(&context_term, 1, 0) else {
        return Err(AuditUnknownReason::ResourceExhausted);
    };
    let Some(one_hole_template) =
        replace_term_node_with_binder_hole(&shifted_context_term, hole_ordinal)
    else {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    };
    let Some(shifted_context_type) = shift_term(&context_type, 1, 0) else {
        return Err(AuditUnknownReason::ResourceExhausted);
    };
    if kernel
        .verify_open_judgment(
            signature,
            &OpenJudgment::HasType {
                context: source_context.clone(),
                term: one_hole_template.clone(),
                ty: shifted_context_type,
            },
        )
        .is_err()
    {
        return Err(AuditUnknownReason::MissingTypedOccurrenceCensus);
    }
    let one_hole_context = OneHoleContextIdV2(Digest::of_canonical(
        "pen-semantic-audit/one-hole-context/v2",
        &OneHoleSubject {
            context_family: &context_family.id,
            hole_ordinal,
            target_context: &context_witness.target_context,
            context_term: &context_term,
            source_context: &source_context,
            one_hole_template: &one_hole_template,
        },
    ));
    let mut output = Vec::with_capacity(2);
    for (side, filler) in [
        (EquationFillerSideV2::Left, left),
        (EquationFillerSideV2::Right, right),
    ] {
        let mut images = identity_images(&context_witness.target_context)?;
        images.push(filler);
        verify_simultaneous_substitution(
            kernel,
            signature,
            &source_context,
            &context_witness.target_context,
            &images,
        )?;
        let Some(instantiated_template) =
            instantiate_term_images(&one_hole_template, source_context.0.len(), &images, 0)
        else {
            return Err(AuditUnknownReason::KernelCouldNotCertify);
        };
        let expected_instantiation = match side {
            EquationFillerSideV2::Left => match &expected {
                GenericJudgmentV1::Equation { left, .. } => left,
                GenericJudgmentV1::Term { .. } => unreachable!("constructed equation"),
            },
            EquationFillerSideV2::Right => match &expected {
                GenericJudgmentV1::Equation { right, .. } => right,
                GenericJudgmentV1::Term { .. } => unreachable!("constructed equation"),
            },
        };
        if &instantiated_template != expected_instantiation {
            return Err(AuditUnknownReason::MissingTypedOccurrenceCensus);
        }
        output.push(make_direct_witness(
            tuple_id.clone(),
            ConstructionSubstitutionKindV2::OneHoleInstantiation {
                one_hole_context: one_hole_context.clone(),
                filler_family: equation_family.id.clone(),
            },
            source_context.clone(),
            context_witness.target_context.clone(),
            images,
            Some(one_hole_template.clone()),
            Some(side),
            Some(result_family.id.clone()),
            kernel.kernel_protocol_digest(),
        ));
    }
    Ok(output)
}

struct OneHoleSubject<'a> {
    context_family: &'a RawFamilyIdV1,
    hole_ordinal: u32,
    target_context: &'a DependentContext,
    context_term: &'a Term,
    source_context: &'a DependentContext,
    one_hole_template: &'a Term,
}

impl CanonicalEncode for OneHoleSubject<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.context_family.encode_canonical(encoder);
        encoder.u32(self.hole_ordinal);
        self.target_context.encode_canonical(encoder);
        self.context_term.encode_canonical(encoder);
        self.source_context.encode_canonical(encoder);
        self.one_hole_template.encode_canonical(encoder);
    }
}

#[allow(clippy::too_many_arguments)]
fn make_direct_witness(
    carrier_tuple: CarrierTupleIdV2,
    kind: ConstructionSubstitutionKindV2,
    source_context: DependentContext,
    target_context: DependentContext,
    images: Vec<Term>,
    one_hole_template: Option<Term>,
    equation_filler_side: Option<EquationFillerSideV2>,
    result_family: Option<RawFamilyIdV1>,
    kernel_protocol_digest: Digest,
) -> VerifiedDirectConstructionSubstitutionV2 {
    let kernel_replay_digest = Digest::of_canonical(
        "pen-semantic-audit/direct-substitution-kernel-replay/v2",
        &KernelReplaySubject {
            kernel_protocol_digest: &kernel_protocol_digest,
            source_context: &source_context,
            target_context: &target_context,
            images: &images,
        },
    );
    let id = ConstructionSubstitutionIdV2(Digest::of_canonical(
        "pen-semantic-audit/direct-construction-substitution/v2",
        &DirectWitnessSubject {
            carrier_tuple: &carrier_tuple,
            kind: &kind,
            source_context: &source_context,
            target_context: &target_context,
            images: &images,
            one_hole_template: one_hole_template.as_ref(),
            equation_filler_side,
            result_family: result_family.as_ref(),
            kernel_replay_digest: &kernel_replay_digest,
        },
    ));
    VerifiedDirectConstructionSubstitutionV2 {
        id,
        carrier_tuple,
        kind,
        source_context,
        target_context,
        images,
        one_hole_template,
        equation_filler_side,
        result_family,
        kernel_replay_digest,
    }
}

struct KernelReplaySubject<'a> {
    kernel_protocol_digest: &'a Digest,
    source_context: &'a DependentContext,
    target_context: &'a DependentContext,
    images: &'a [Term],
}

impl CanonicalEncode for KernelReplaySubject<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.source_context.encode_canonical(encoder);
        self.target_context.encode_canonical(encoder);
        encoder.sequence(self.images);
    }
}

struct DirectWitnessSubject<'a> {
    carrier_tuple: &'a CarrierTupleIdV2,
    kind: &'a ConstructionSubstitutionKindV2,
    source_context: &'a DependentContext,
    target_context: &'a DependentContext,
    images: &'a [Term],
    one_hole_template: Option<&'a Term>,
    equation_filler_side: Option<EquationFillerSideV2>,
    result_family: Option<&'a RawFamilyIdV1>,
    kernel_replay_digest: &'a Digest,
}

impl CanonicalEncode for DirectWitnessSubject<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.carrier_tuple.encode_canonical(encoder);
        self.kind.encode_canonical(encoder);
        self.source_context.encode_canonical(encoder);
        self.target_context.encode_canonical(encoder);
        encoder.sequence(self.images);
        match self.one_hole_template {
            Some(one_hole_template) => {
                encoder.tag(1);
                one_hole_template.encode_canonical(encoder);
            }
            None => encoder.tag(0),
        }
        encoder.option(&self.equation_filler_side);
        match self.result_family {
            Some(result_family) => {
                encoder.tag(1);
                result_family.encode_canonical(encoder);
            }
            None => encoder.tag(0),
        }
        self.kernel_replay_digest.encode_canonical(encoder);
    }
}

fn identity_images(context: &DependentContext) -> Result<Vec<Term>, AuditUnknownReason> {
    (0..context.0.len())
        .map(|ordinal| {
            let free_index = context
                .0
                .len()
                .checked_sub(ordinal + 1)
                .ok_or(AuditUnknownReason::MalformedInput)?;
            Ok(Term::Var {
                index: u32::try_from(free_index)
                    .map_err(|_| AuditUnknownReason::ResourceExhausted)?,
            })
        })
        .collect()
}

fn embedding_images(target: &DependentContext, embedding: &[u32]) -> Option<Vec<Term>> {
    embedding
        .iter()
        .map(|target_ordinal| {
            let target_ordinal = usize::try_from(*target_ordinal).ok()?;
            let free_index = target.0.len().checked_sub(target_ordinal.checked_add(1)?)?;
            Some(Term::Var {
                index: u32::try_from(free_index).ok()?,
            })
        })
        .collect()
}

fn embedding_respects_context(
    source: &DependentContext,
    target: &DependentContext,
    embedding: &[u32],
    images: &[Term],
) -> bool {
    if embedding.len() != source.0.len()
        || images.len() != source.0.len()
        || embedding.windows(2).any(|window| window[0] >= window[1])
        || embedding
            .last()
            .is_some_and(|last| usize::try_from(*last).map_or(true, |last| last >= target.0.len()))
    {
        return false;
    }
    for (source_ordinal, source_entry) in source.0.iter().enumerate() {
        let Ok(target_ordinal) = usize::try_from(embedding[source_ordinal]) else {
            return false;
        };
        let source_prefix = DependentContext(source.0[..source_ordinal].to_vec());
        let target_prefix = DependentContext(target.0[..target_ordinal].to_vec());
        let judgment = GenericJudgmentV1::Term {
            context: source_prefix,
            term: source_entry.clone(),
            ty: Term::UnitType,
        };
        let Some(GenericJudgmentV1::Term { term: renamed, .. }) =
            rename_judgment_into(&judgment, &target_prefix, &embedding[..source_ordinal])
        else {
            return false;
        };
        if target.0.get(target_ordinal) != Some(&renamed) {
            return false;
        }
    }
    embedding_images(target, embedding).as_deref() == Some(images)
}

fn verify_simultaneous_substitution(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    source: &DependentContext,
    target: &DependentContext,
    images: &[Term],
) -> Result<(), AuditUnknownReason> {
    kernel
        .verify_context(signature, source)
        .map_err(kernel_unknown)?;
    kernel
        .verify_context(signature, target)
        .map_err(kernel_unknown)?;
    if source.0.len() != images.len() {
        return Err(AuditUnknownReason::MalformedInput);
    }
    for (ordinal, (source_type, image)) in source.0.iter().zip(images).enumerate() {
        let expected_type = instantiate_term_images(source_type, ordinal, &images[..ordinal], 0)
            .ok_or(AuditUnknownReason::KernelCouldNotCertify)?;
        kernel
            .verify_open_judgment(
                signature,
                &OpenJudgment::HasType {
                    context: target.clone(),
                    term: image.clone(),
                    ty: expected_type,
                },
            )
            .map_err(kernel_unknown)?;
    }
    Ok(())
}

fn instantiate_term_images(
    term: &Term,
    source_context_len: usize,
    images: &[Term],
    binder_depth: u32,
) -> Option<Term> {
    if images.len() != source_context_len {
        return None;
    }
    match term {
        Term::Sort { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => Some(term.clone()),
        Term::Var { index } if *index < binder_depth => Some(term.clone()),
        Term::Var { index } => {
            let free_index = usize::try_from(index.checked_sub(binder_depth)?).ok()?;
            let source_ordinal = source_context_len.checked_sub(free_index.checked_add(1)?)?;
            shift_term(images.get(source_ordinal)?, i64::from(binder_depth), 0)
        }
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(instantiate_term_images(
                parameter,
                source_context_len,
                images,
                binder_depth,
            )?),
            body: Box::new(instantiate_term_images(
                body,
                source_context_len,
                images,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(instantiate_term_images(
                parameter,
                source_context_len,
                images,
                binder_depth,
            )?),
            body: Box::new(instantiate_term_images(
                body,
                source_context_len,
                images,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(instantiate_term_images(
                parameter_type,
                source_context_len,
                images,
                binder_depth,
            )?),
            body: Box::new(instantiate_term_images(
                body,
                source_context_len,
                images,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(instantiate_term_images(
                function,
                source_context_len,
                images,
                binder_depth,
            )?),
            argument: Box::new(instantiate_term_images(
                argument,
                source_context_len,
                images,
                binder_depth,
            )?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(instantiate_term_images(
                sigma_type,
                source_context_len,
                images,
                binder_depth,
            )?),
            first: Box::new(instantiate_term_images(
                first,
                source_context_len,
                images,
                binder_depth,
            )?),
            second: Box::new(instantiate_term_images(
                second,
                source_context_len,
                images,
                binder_depth,
            )?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(instantiate_term_images(
                pair,
                source_context_len,
                images,
                binder_depth,
            )?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(instantiate_term_images(
                pair,
                source_context_len,
                images,
                binder_depth,
            )?),
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

fn replace_term_node(term: &Term, target: u32, replacement: &Term) -> Option<Term> {
    fn visit(term: &Term, target: u32, replacement: &Term, ordinal: &mut u32) -> Option<Term> {
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
                parameter: Box::new(visit(parameter, target, replacement, ordinal)?),
                body: Box::new(visit(body, target, replacement, ordinal)?),
            }),
            Term::Sigma { parameter, body } => Some(Term::Sigma {
                parameter: Box::new(visit(parameter, target, replacement, ordinal)?),
                body: Box::new(visit(body, target, replacement, ordinal)?),
            }),
            Term::Lambda {
                parameter_type,
                body,
            } => Some(Term::Lambda {
                parameter_type: Box::new(visit(parameter_type, target, replacement, ordinal)?),
                body: Box::new(visit(body, target, replacement, ordinal)?),
            }),
            Term::Apply { function, argument } => Some(Term::Apply {
                function: Box::new(visit(function, target, replacement, ordinal)?),
                argument: Box::new(visit(argument, target, replacement, ordinal)?),
            }),
            Term::Pair {
                sigma_type,
                first,
                second,
            } => Some(Term::Pair {
                sigma_type: Box::new(visit(sigma_type, target, replacement, ordinal)?),
                first: Box::new(visit(first, target, replacement, ordinal)?),
                second: Box::new(visit(second, target, replacement, ordinal)?),
            }),
            Term::First { pair } => Some(Term::First {
                pair: Box::new(visit(pair, target, replacement, ordinal)?),
            }),
            Term::Second { pair } => Some(Term::Second {
                pair: Box::new(visit(pair, target, replacement, ordinal)?),
            }),
        }
    }
    let mut ordinal = 0;
    let replaced = visit(term, target, replacement, &mut ordinal)?;
    (target < ordinal).then_some(replaced)
}

fn replace_term_node_with_binder_hole(term: &Term, target: u32) -> Option<Term> {
    fn visit(term: &Term, target: u32, ordinal: &mut u32, binder_depth: u32) -> Option<Term> {
        let here = *ordinal;
        *ordinal = ordinal.checked_add(1)?;
        if here == target {
            return Some(Term::Var {
                index: binder_depth,
            });
        }
        match term {
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit => Some(term.clone()),
            Term::Pi { parameter, body } => Some(Term::Pi {
                parameter: Box::new(visit(parameter, target, ordinal, binder_depth)?),
                body: Box::new(visit(body, target, ordinal, binder_depth.checked_add(1)?)?),
            }),
            Term::Sigma { parameter, body } => Some(Term::Sigma {
                parameter: Box::new(visit(parameter, target, ordinal, binder_depth)?),
                body: Box::new(visit(body, target, ordinal, binder_depth.checked_add(1)?)?),
            }),
            Term::Lambda {
                parameter_type,
                body,
            } => Some(Term::Lambda {
                parameter_type: Box::new(visit(parameter_type, target, ordinal, binder_depth)?),
                body: Box::new(visit(body, target, ordinal, binder_depth.checked_add(1)?)?),
            }),
            Term::Apply { function, argument } => Some(Term::Apply {
                function: Box::new(visit(function, target, ordinal, binder_depth)?),
                argument: Box::new(visit(argument, target, ordinal, binder_depth)?),
            }),
            Term::Pair {
                sigma_type,
                first,
                second,
            } => Some(Term::Pair {
                sigma_type: Box::new(visit(sigma_type, target, ordinal, binder_depth)?),
                first: Box::new(visit(first, target, ordinal, binder_depth)?),
                second: Box::new(visit(second, target, ordinal, binder_depth)?),
            }),
            Term::First { pair } => Some(Term::First {
                pair: Box::new(visit(pair, target, ordinal, binder_depth)?),
            }),
            Term::Second { pair } => Some(Term::Second {
                pair: Box::new(visit(pair, target, ordinal, binder_depth)?),
            }),
        }
    }
    let mut ordinal = 0;
    let replaced = visit(term, target, &mut ordinal, 0)?;
    (target < ordinal).then_some(replaced)
}

fn kernel_unknown(error: KernelError) -> AuditUnknownReason {
    match error {
        KernelError::ResourceExhausted(_) => AuditUnknownReason::ResourceExhausted,
        _ => AuditUnknownReason::KernelCouldNotCertify,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::carrier::enumerate_pre_q0_raw_families_v1;
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v1,
        proposed_semantic_audit_lambda_unit_manifest_v2, proposed_semantic_audit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v2, verify_semantic_audit_manifest_v1,
    };
    use crate::model::{
        EquationIdV1, EventIdV1, HeadPresentationV1, LocalRoleV1, PublicEquationSeedV1,
        PublicHeadSeedV1, PublicSupportV1, SemanticSchemaSeedV1, SourceNormalizedJudgmentV1,
    };
    use pen_kernel::{Declaration, GlobalId, KernelLimits, UncheckedSignature};

    fn kind_tag_for_exhaustiveness(kind: &ConstructionSubstitutionKindV2) -> u8 {
        match kind {
            ConstructionSubstitutionKindV2::ContextEmbedding { .. } => 0,
            ConstructionSubstitutionKindV2::ForcedNewestArgument { .. } => 1,
            ConstructionSubstitutionKindV2::OneHoleInstantiation { .. } => 2,
        }
    }

    #[test]
    fn kind_surface_is_exactly_the_three_direct_construction_forms() {
        let family = RawFamilyIdV1(Digest::of_bytes(b"family"));
        let kinds = [
            ConstructionSubstitutionKindV2::ContextEmbedding {
                witness: crate::model::ContextWitnessIdV1(Digest::of_bytes(b"context-witness")),
                side: EmbeddingSideV2::Left,
            },
            ConstructionSubstitutionKindV2::ForcedNewestArgument {
                source_context: ConstructionContextIdV2(Digest::of_bytes(b"source-context")),
                argument_family: family.clone(),
            },
            ConstructionSubstitutionKindV2::OneHoleInstantiation {
                one_hole_context: OneHoleContextIdV2(Digest::of_bytes(b"one-hole-context")),
                filler_family: family,
            },
        ];
        assert_eq!(
            kinds
                .iter()
                .map(kind_tag_for_exhaustiveness)
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn rank_inductive_authority_fails_at_the_native_v2_carrier_gate() {
        let census = VerifiedConstructionSubstitutionCensusV2 {
            semantic_manifest_digest: Digest::of_bytes(b"manifest"),
            source_carrier_digest: Digest::of_bytes(b"carrier"),
            signature_digest: Digest::of_bytes(b"signature"),
            kernel_protocol_digest: Digest::of_bytes(b"kernel"),
            tuple_ids: Vec::new(),
            witnesses: Vec::new(),
            coverage_digest: Digest::of_bytes(b"coverage"),
            digest: Digest::of_bytes(b"census"),
        };
        assert_eq!(
            attempt_rank_inductive_carrier_v2(&census),
            AuditDecision::Unknown(AuditUnknownReason::MissingRankInductiveCarrierTheorem)
        );
    }

    #[test]
    fn census_mints_only_tuple_local_direct_witnesses() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let function = GlobalId(Digest::of_bytes(b"direct-v2-function"));
        let argument = GlobalId(Digest::of_bytes(b"direct-v2-argument"));
        let function_type = Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        };
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: function.clone(),
                        ty: function_type.clone(),
                        body: None,
                    },
                    Declaration {
                        id: argument.clone(),
                        ty: Term::UnitType,
                        body: None,
                    },
                ],
            })
            .expect("signature");
        let head_seed = |declaration: GlobalId, ty: Term, label: &'static [u8]| {
            let judgment = GenericJudgmentV1::Term {
                context: DependentContext::default(),
                term: Term::Global {
                    id: declaration.clone(),
                },
                ty,
            };
            SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
                declaration,
                origin_event: EventIdV1(Digest::of_domain_bytes(
                    "pen-semantic-audit/direct-v2-test-event",
                    label,
                )),
                judgment: SourceNormalizedJudgmentV1 {
                    source_identity: Digest::of_domain_bytes(
                        "pen-semantic-audit/direct-v2-test-source",
                        label,
                    ),
                    source: judgment.clone(),
                    claimed_normalized: judgment,
                },
                presentation: HeadPresentationV1::Opaque,
                claimed_role: LocalRoleV1::KernelHead,
                public_support: PublicSupportV1::default(),
                source_clause: None,
            })
        };
        let equation_judgment = GenericJudgmentV1::Equation {
            context: DependentContext(vec![Term::UnitType]),
            left: Term::Apply {
                function: Box::new(Term::Global {
                    id: function.clone(),
                }),
                argument: Box::new(Term::Var { index: 0 }),
            },
            right: Term::Var { index: 0 },
            ty: Term::UnitType,
        };
        let seeds = vec![
            head_seed(function.clone(), function_type, b"function"),
            head_seed(argument, Term::UnitType, b"argument"),
            SemanticSchemaSeedV1::PublicEquation(PublicEquationSeedV1 {
                equation: EquationIdV1(Digest::of_bytes(b"direct-v2-equation")),
                owner_head: function,
                origin_event: EventIdV1(Digest::of_bytes(b"direct-v2-equation-event")),
                judgment: SourceNormalizedJudgmentV1 {
                    source_identity: Digest::of_bytes(b"direct-v2-equation-source"),
                    source: equation_judgment.clone(),
                    claimed_normalized: equation_judgment,
                },
                claimed_role: LocalRoleV1::Coherence,
                public_support: PublicSupportV1::default(),
                source_clause: None,
                demand_anchor: None,
            }),
        ];
        let AuditDecision::Proven(v1_manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("V1 manifest");
        };
        let AuditDecision::Proven(carrier) =
            enumerate_pre_q0_raw_families_v1(&kernel, &signature, &v1_manifest, &seeds, &[])
        else {
            panic!("pre-Q0 carrier");
        };
        let AuditDecision::Proven(v2_manifest) = verify_semantic_audit_lambda_unit_manifest_v2(
            &proposed_semantic_audit_lambda_unit_manifest_v2(),
        ) else {
            panic!("V2 manifest");
        };
        let AuditDecision::Proven(census) = verify_construction_substitution_census_v2(
            &kernel,
            &signature,
            &v1_manifest,
            &v2_manifest,
            &carrier,
        ) else {
            panic!("direct construction census");
        };

        assert_eq!(census.tuple_ids().len(), carrier.tuple_dispositions().len());
        assert!(census.witnesses().iter().any(|witness| matches!(
            witness.kind(),
            ConstructionSubstitutionKindV2::ForcedNewestArgument {
                argument_family: _,
                ..
            }
        )));
        assert!(census.witnesses().iter().any(|witness| matches!(
            witness.kind(),
            ConstructionSubstitutionKindV2::OneHoleInstantiation { .. }
        )));
        assert!(census.witnesses().iter().all(|witness| {
            matches!(
                witness.kind(),
                ConstructionSubstitutionKindV2::ContextEmbedding { .. }
                    | ConstructionSubstitutionKindV2::ForcedNewestArgument { .. }
                    | ConstructionSubstitutionKindV2::OneHoleInstantiation { .. }
            ) && census.tuple_ids().contains(witness.carrier_tuple())
        }));

        let AuditDecision::Proven(broader_v1_manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("broader V1 manifest");
        };
        let AuditDecision::Proven(broader_v1_carrier) = enumerate_pre_q0_raw_families_v1(
            &kernel,
            &signature,
            &broader_v1_manifest,
            &seeds,
            &[],
        ) else {
            panic!("broader V1 carrier");
        };
        assert_eq!(
            verify_construction_substitution_census_v2(
                &kernel,
                &signature,
                &v1_manifest,
                &v2_manifest,
                &broader_v1_carrier,
            ),
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        );
    }
}
