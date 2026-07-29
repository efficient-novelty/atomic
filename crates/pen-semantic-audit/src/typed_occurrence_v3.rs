//! Synthesis-backed binder-local typed occurrences for the V3 successor.
//!
//! This module is additive: the V2 verifier and its fail-closed behavior are
//! unchanged. V3 uses the isolated `pen-kernel-synthesis` capability, checks
//! its protocol/signature/context bindings, and independently replays every
//! occurrence with the unchanged kernel.

use crate::fragment::{
    lambda_unit_context_syntax_violation, lambda_unit_judgment_syntax_violation,
    lambda_unit_term_syntax_violation,
};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3, TypedOccurrenceCensusProtocolV2,
    VerifiedSemanticAuditManifestV3,
};
use crate::model::GenericJudgmentV1;
use crate::typed_occurrence::{
    LocalJudgmentV1, TypedBinderKindV1, TypedOccurrencePathComponentV1,
    TypedOccurrenceRootRequestV1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError, OpenJudgment,
    ResourceKind, Term, VerifiedSignature,
};
use pen_kernel_synthesis::{
    SynthesisError, VerifiedSynthesizedJudgmentV1, synthesis_protocol_digest_v1,
    synthesize_lambda_unit_v1,
};
use std::collections::{BTreeMap, BTreeSet};

pub const SYNTHESIS_BACKED_OCCURRENCE_BATCH_SCHEMA_VERSION_V1: u16 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SynthesisBackedOccurrenceCensusPrerequisiteV3 {
    LambdaUnitTypingMetatheory,
    NativeRankInductiveCarrier,
    CarrierDerivedExhaustiveRootInventory,
}

pub const SYNTHESIS_BACKED_OCCURRENCE_CENSUS_PREREQUISITES_V3:
    [SynthesisBackedOccurrenceCensusPrerequisiteV3; 3] = [
    SynthesisBackedOccurrenceCensusPrerequisiteV3::LambdaUnitTypingMetatheory,
    SynthesisBackedOccurrenceCensusPrerequisiteV3::NativeRankInductiveCarrier,
    SynthesisBackedOccurrenceCensusPrerequisiteV3::CarrierDerivedExhaustiveRootInventory,
];

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypedOccurrenceRootIdV2(Digest);

impl TypedOccurrenceRootIdV2 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for TypedOccurrenceRootIdV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypedOccurrenceIdV2(Digest);

impl TypedOccurrenceIdV2 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for TypedOccurrenceIdV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// Exact path inside one fixed-root diagnostic batch.
///
/// This successor-owned representation keeps the preserved V1/V2 occurrence
/// module byte-for-byte unchanged.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypedOccurrencePathV2(Vec<TypedOccurrencePathComponentV1>);

impl TypedOccurrencePathV2 {
    pub fn components(&self) -> &[TypedOccurrencePathComponentV1] {
        &self.0
    }

    fn root(component: TypedOccurrencePathComponentV1) -> Self {
        Self(vec![component])
    }

    fn child(&self, component: TypedOccurrencePathComponentV1) -> Self {
        let mut components = self.0.clone();
        components.push(component);
        Self(components)
    }
}

impl CanonicalEncode for TypedOccurrencePathV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.0);
    }
}

/// Checked V3 root. Private fields and no `Deserialize` implementation make
/// this a verifier capability rather than wire authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedTypedOccurrenceRootV2 {
    id: TypedOccurrenceRootIdV2,
    subject: TypedOccurrenceRootRequestV1,
    kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedTypedOccurrenceRootV2 {
    pub fn id(&self) -> &TypedOccurrenceRootIdV2 {
        &self.id
    }

    pub fn subject(&self) -> &TypedOccurrenceRootRequestV1 {
        &self.subject
    }

    pub fn kernel_replay_digest(&self) -> &Digest {
        &self.kernel_replay_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedTypedOccurrenceRootV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.subject.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Complete local occurrence evidence. The synthesis capability stays opaque
/// but is retained for downstream inspection, not replaced by a bare digest.
#[derive(Clone, Debug)]
pub struct VerifiedTypedOccurrenceV2 {
    id: TypedOccurrenceIdV2,
    root: TypedOccurrenceRootIdV2,
    path: TypedOccurrencePathV2,
    local_context: DependentContext,
    term: Term,
    synthesized: VerifiedSynthesizedJudgmentV1,
    checked_judgment: LocalJudgmentV1,
    kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedTypedOccurrenceV2 {
    pub fn id(&self) -> &TypedOccurrenceIdV2 {
        &self.id
    }

    pub fn root(&self) -> &TypedOccurrenceRootIdV2 {
        &self.root
    }

    pub fn path(&self) -> &TypedOccurrencePathV2 {
        &self.path
    }

    pub fn local_context(&self) -> &DependentContext {
        &self.local_context
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn synthesized(&self) -> &VerifiedSynthesizedJudgmentV1 {
        &self.synthesized
    }

    pub fn checked_judgment(&self) -> &LocalJudgmentV1 {
        &self.checked_judgment
    }

    pub fn kernel_replay_digest(&self) -> &Digest {
        &self.kernel_replay_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedTypedOccurrenceV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.root.encode_canonical(encoder);
        self.path.encode_canonical(encoder);
        self.local_context.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.synthesized.digest().encode_canonical(encoder);
        self.checked_judgment.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Diagnostic evidence for exactly the caller-supplied roots.
///
/// `roots_digest` and `coverage_digest` prove completeness only inside this
/// fixed batch. They do not prove that the roots exhaust a native carrier and
/// therefore cannot serve as the V3 typed-occurrence census authority.
#[derive(Clone, Debug)]
pub struct VerifiedSynthesisBackedOccurrenceBatchV1 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    synthesis_protocol_digest: Digest,
    roots_digest: Digest,
    roots: Vec<VerifiedTypedOccurrenceRootV2>,
    occurrences: Vec<VerifiedTypedOccurrenceV2>,
    coverage_digest: Digest,
    digest: Digest,
}

impl VerifiedSynthesisBackedOccurrenceBatchV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn synthesis_protocol_digest(&self) -> &Digest {
        &self.synthesis_protocol_digest
    }

    pub fn roots_digest(&self) -> &Digest {
        &self.roots_digest
    }

    pub fn roots(&self) -> &[VerifiedTypedOccurrenceRootV2] {
        &self.roots
    }

    pub fn occurrences(&self) -> &[VerifiedTypedOccurrenceV2] {
        &self.occurrences
    }

    pub fn occurrences_for_root(
        &self,
        root: &TypedOccurrenceRootIdV2,
    ) -> impl Iterator<Item = &VerifiedTypedOccurrenceV2> {
        self.occurrences
            .iter()
            .filter(move |occurrence| occurrence.root() == root)
    }

    pub fn occurrence(&self, id: &TypedOccurrenceIdV2) -> Option<&VerifiedTypedOccurrenceV2> {
        self.occurrences
            .binary_search_by(|occurrence| occurrence.id().cmp(id))
            .ok()
            .map(|index| &self.occurrences[index])
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedSynthesisBackedOccurrenceBatchV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.synthesis_protocol_digest.encode_canonical(encoder);
        self.roots_digest.encode_canonical(encoder);
        encoder.sequence(&self.roots);
        encoder.sequence(&self.occurrences);
        self.coverage_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Verify local synthesis-backed evidence for exactly `roots`.
///
/// This diagnostic function accepts caller-selected roots and deliberately
/// does not mint a complete V3 occurrence census.
pub fn verify_synthesis_backed_occurrence_batch_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    roots: &[TypedOccurrenceRootRequestV1],
) -> AuditDecision<VerifiedSynthesisBackedOccurrenceBatchV1> {
    let manifest_wire = manifest.manifest();
    if manifest_wire.profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3
        || manifest_wire.typed_occurrence_census_protocol
            != TypedOccurrenceCensusProtocolV2::BinderLocalTermsAndJudgmentTypes
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    let root_limit = usize::try_from(manifest_wire.maximum_raw_derivations)
        .unwrap_or(usize::MAX)
        .min(kernel.limits().max_operations as usize);
    if roots.len() > root_limit {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    }
    let mut builder = BatchBuilderV1 {
        manifest,
        kernel,
        signature,
        operations_left: kernel.limits().max_operations,
        occurrences: Vec::new(),
        occurrence_ids: BTreeSet::new(),
    };
    match builder.build(roots) {
        Ok(batch) => AuditDecision::Proven(batch),
        Err(failure) => failure.into_decision(),
    }
}

/// Report the authoritative V3 census frontier without accepting caller roots
/// as an exhaustiveness witness.
///
/// The eventual constructor must require all entries in
/// [`SYNTHESIS_BACKED_OCCURRENCE_CENSUS_PREREQUISITES_V3`]. No such
/// constructor exists while the metatheory and native carrier are unminted.
pub fn diagnose_synthesis_backed_typed_occurrence_census_v3(
    manifest: &VerifiedSemanticAuditManifestV3,
) -> AuditDecision<()> {
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3
        || manifest.manifest().typed_occurrence_census_protocol
            != TypedOccurrenceCensusProtocolV2::BinderLocalTermsAndJudgmentTypes
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    AuditDecision::Unknown(AuditUnknownReason::MissingSynthesisBackedTypedOccurrenceCensus)
}

#[derive(Clone, Debug)]
enum BuildFailureV3 {
    Unknown(AuditUnknownReason),
    Outside(OutsideFragmentReason),
}

impl BuildFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::Unknown(reason) => AuditDecision::Unknown(reason),
            Self::Outside(reason) => AuditDecision::OutsideFragment(reason),
        }
    }
}

impl From<KernelError> for BuildFailureV3 {
    fn from(error: KernelError) -> Self {
        match error {
            KernelError::ResourceExhausted(
                ResourceKind::Operations | ResourceKind::Depth | ResourceKind::Normalization,
            ) => Self::Unknown(AuditUnknownReason::ResourceExhausted),
            _ => Self::Unknown(AuditUnknownReason::KernelCouldNotCertify),
        }
    }
}

impl From<SynthesisError> for BuildFailureV3 {
    fn from(error: SynthesisError) -> Self {
        match error {
            SynthesisError::OutsideLambdaUnitFragment => {
                Self::Outside(OutsideFragmentReason::UnsupportedTerm)
            }
            SynthesisError::ResourceExhausted(_)
            | SynthesisError::Kernel(KernelError::ResourceExhausted(_))
            | SynthesisError::AggregateKernelReplay(KernelError::ResourceExhausted(_)) => {
                Self::Unknown(AuditUnknownReason::ResourceExhausted)
            }
            _ => Self::Unknown(AuditUnknownReason::KernelCouldNotCertify),
        }
    }
}

type BuildResultV3<T> = Result<T, BuildFailureV3>;

struct BatchBuilderV1<'a> {
    manifest: &'a VerifiedSemanticAuditManifestV3,
    kernel: &'a Kernel,
    signature: &'a VerifiedSignature,
    operations_left: u32,
    occurrences: Vec<VerifiedTypedOccurrenceV2>,
    occurrence_ids: BTreeSet<TypedOccurrenceIdV2>,
}

impl BatchBuilderV1<'_> {
    fn build(
        &mut self,
        requests: &[TypedOccurrenceRootRequestV1],
    ) -> BuildResultV3<VerifiedSynthesisBackedOccurrenceBatchV1> {
        let mut roots_by_id = BTreeMap::new();
        for request in requests {
            self.charge()?;
            self.check_root_fragment(request)?;
            let root = self.verify_root(request)?;
            match roots_by_id.get(root.id()) {
                Some(prior) if prior != &root => {
                    return Err(BuildFailureV3::Unknown(
                        AuditUnknownReason::IncompleteEnumeration,
                    ));
                }
                Some(_) => {}
                None => {
                    roots_by_id.insert(root.id().clone(), root);
                }
            }
        }
        let roots = roots_by_id.into_values().collect::<Vec<_>>();
        let roots_digest = Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-roots/v2",
            &CanonicalSlice(&roots),
        );
        for root in &roots {
            self.visit_root(root)?;
        }
        self.occurrences
            .sort_by(|left, right| left.id().cmp(right.id()));
        if self
            .occurrences
            .windows(2)
            .any(|pair| pair[0].id() == pair[1].id())
        {
            return Err(BuildFailureV3::Unknown(
                AuditUnknownReason::IncompleteEnumeration,
            ));
        }
        let coverage = self
            .occurrences
            .iter()
            .map(|occurrence| CoverageEntry {
                root: occurrence.root().clone(),
                path: occurrence.path().clone(),
                occurrence: occurrence.id().clone(),
            })
            .collect::<Vec<_>>();
        let coverage_digest = Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-coverage/v2",
            &CanonicalSlice(&coverage),
        );
        let mut batch = VerifiedSynthesisBackedOccurrenceBatchV1 {
            schema_version: SYNTHESIS_BACKED_OCCURRENCE_BATCH_SCHEMA_VERSION_V1,
            semantic_manifest_digest: self.manifest.candidate_digest().clone(),
            signature_digest: self.signature.digest().clone(),
            kernel_protocol_digest: self.kernel.kernel_protocol_digest(),
            synthesis_protocol_digest: synthesis_protocol_digest_v1(),
            roots_digest,
            roots,
            occurrences: std::mem::take(&mut self.occurrences),
            coverage_digest,
            digest: Digest::of_bytes(b"synthesis-backed-occurrence-batch-v1-pending"),
        };
        batch.digest = Digest::of_canonical(
            "pen-semantic-audit/verified-synthesis-backed-occurrence-batch/v1",
            &BatchDigestInput(&batch),
        );
        Ok(batch)
    }

    fn charge(&mut self) -> BuildResultV3<()> {
        self.operations_left =
            self.operations_left
                .checked_sub(1)
                .ok_or(BuildFailureV3::Unknown(
                    AuditUnknownReason::ResourceExhausted,
                ))?;
        Ok(())
    }

    fn check_context_bound(&self, context: &DependentContext) -> BuildResultV3<()> {
        if context.0.len() > usize::from(self.manifest.manifest().maximum_context_entries) {
            return Err(BuildFailureV3::Unknown(
                AuditUnknownReason::ResourceExhausted,
            ));
        }
        Ok(())
    }

    fn check_root_fragment(&self, request: &TypedOccurrenceRootRequestV1) -> BuildResultV3<()> {
        self.check_context_bound(request.context())?;
        let levels = &self.manifest.manifest().universe_levels;
        let violation = match request {
            TypedOccurrenceRootRequestV1::HasType { context, term, ty } => {
                lambda_unit_judgment_syntax_violation(
                    &GenericJudgmentV1::Term {
                        context: context.clone(),
                        term: term.clone(),
                        ty: ty.clone(),
                    },
                    levels,
                )
            }
            TypedOccurrenceRootRequestV1::Equation {
                context,
                left,
                right,
                ty,
            } => lambda_unit_judgment_syntax_violation(
                &GenericJudgmentV1::Equation {
                    context: context.clone(),
                    left: left.clone(),
                    right: right.clone(),
                    ty: ty.clone(),
                },
                levels,
            ),
            TypedOccurrenceRootRequestV1::TypeFormation { context, term } => [
                lambda_unit_context_syntax_violation(context, levels),
                lambda_unit_term_syntax_violation(term, levels),
            ]
            .into_iter()
            .flatten()
            .max(),
        };
        if let Some(violation) = violation {
            return Err(BuildFailureV3::Outside(violation.outside_reason()));
        }
        Ok(())
    }

    fn verify_root(
        &mut self,
        request: &TypedOccurrenceRootRequestV1,
    ) -> BuildResultV3<VerifiedTypedOccurrenceRootV2> {
        let (subject, kernel_replay_digest) = match request {
            TypedOccurrenceRootRequestV1::HasType { context, term, ty } => {
                let replay = self.replay_has_type(context, term, ty)?;
                (
                    TypedOccurrenceRootRequestV1::HasType {
                        context: replay.context,
                        term: term.clone(),
                        ty: replay.ty,
                    },
                    replay.digest,
                )
            }
            TypedOccurrenceRootRequestV1::Equation {
                context,
                left,
                right,
                ty,
            } => {
                let left_replay = self.replay_has_type(context, left, ty)?;
                let right_replay = self.replay_has_type(context, right, ty)?;
                if left_replay.context != right_replay.context || left_replay.ty != right_replay.ty
                {
                    return Err(BuildFailureV3::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                }
                let digest = Digest::of_canonical(
                    "pen-semantic-audit/typed-occurrence-equation-root-replay/v2",
                    &DigestPair {
                        left: left_replay.digest,
                        right: right_replay.digest,
                    },
                );
                (
                    TypedOccurrenceRootRequestV1::Equation {
                        context: left_replay.context,
                        left: left.clone(),
                        right: right.clone(),
                        ty: left_replay.ty,
                    },
                    digest,
                )
            }
            TypedOccurrenceRootRequestV1::TypeFormation { context, term } => {
                let replay = self.replay_type_formation(context, term)?;
                (
                    TypedOccurrenceRootRequestV1::TypeFormation {
                        context: replay.context,
                        term: term.clone(),
                    },
                    replay.digest,
                )
            }
        };
        let id = TypedOccurrenceRootIdV2(Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-root-id/v2",
            &RootIdInput {
                manifest_digest: self.manifest.candidate_digest(),
                subject: &subject,
            },
        ));
        let digest = Digest::of_canonical(
            "pen-semantic-audit/verified-typed-occurrence-root/v2",
            &RootDigestInput {
                id: &id,
                subject: &subject,
                kernel_replay_digest: &kernel_replay_digest,
            },
        );
        Ok(VerifiedTypedOccurrenceRootV2 {
            id,
            subject,
            kernel_replay_digest,
            digest,
        })
    }

    fn visit_root(&mut self, root: &VerifiedTypedOccurrenceRootV2) -> BuildResultV3<()> {
        match root.subject() {
            TypedOccurrenceRootRequestV1::HasType { context, term, ty } => {
                self.visit_term(
                    root.id(),
                    context,
                    term,
                    ty,
                    &path_root(TypedOccurrencePathComponentV1::JudgmentTerm),
                )?;
                self.visit_type(
                    root.id(),
                    context,
                    ty,
                    &path_root(TypedOccurrencePathComponentV1::JudgmentType),
                )
            }
            TypedOccurrenceRootRequestV1::Equation {
                context,
                left,
                right,
                ty,
            } => {
                self.visit_term(
                    root.id(),
                    context,
                    left,
                    ty,
                    &path_root(TypedOccurrencePathComponentV1::EquationLeft),
                )?;
                self.visit_term(
                    root.id(),
                    context,
                    right,
                    ty,
                    &path_root(TypedOccurrencePathComponentV1::EquationRight),
                )?;
                self.visit_type(
                    root.id(),
                    context,
                    ty,
                    &path_root(TypedOccurrencePathComponentV1::JudgmentType),
                )
            }
            TypedOccurrenceRootRequestV1::TypeFormation { context, term } => self.visit_type(
                root.id(),
                context,
                term,
                &path_root(TypedOccurrencePathComponentV1::TypeFormationSubject),
            ),
        }
    }

    fn visit_term(
        &mut self,
        root: &TypedOccurrenceRootIdV2,
        context: &DependentContext,
        term: &Term,
        expected: &Term,
        path: &TypedOccurrencePathV2,
    ) -> BuildResultV3<()> {
        self.charge()?;
        let synthesis = self.synthesize(context, term)?;
        let replay = self.replay_has_type(context, term, expected)?;
        if synthesis.normalized_type() != &replay.ty {
            return Err(BuildFailureV3::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        }
        self.record(
            root,
            path,
            &replay.context,
            term,
            &synthesis,
            LocalJudgmentV1::HasType {
                ty: replay.ty.clone(),
            },
            replay.digest,
        )?;
        match term {
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit => Ok(()),
            Term::Pi { parameter, body } => self.visit_binder_type(
                root,
                &replay.context,
                parameter,
                body,
                path,
                TypedBinderKindV1::Pi,
            ),
            Term::Lambda {
                parameter_type,
                body,
            } => {
                let Term::Pi {
                    parameter,
                    body: expected_body,
                } = synthesis.normalized_type()
                else {
                    return Err(BuildFailureV3::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                };
                let parameter_replay =
                    self.replay_type_formation(&replay.context, parameter_type)?;
                if &parameter_replay.term != parameter.as_ref() {
                    return Err(BuildFailureV3::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                }
                self.visit_type(
                    root,
                    &replay.context,
                    parameter_type,
                    &path_child(
                        path,
                        TypedOccurrencePathComponentV1::BinderParameter {
                            binder: TypedBinderKindV1::Lambda,
                        },
                    ),
                )?;
                let extended = self.extend_context(&replay.context, parameter_replay.term)?;
                self.visit_term(
                    root,
                    &extended,
                    body,
                    expected_body,
                    &path_child(
                        path,
                        TypedOccurrencePathComponentV1::BinderBody {
                            binder: TypedBinderKindV1::Lambda,
                        },
                    ),
                )
            }
            Term::Apply { function, argument } => {
                let function_synthesis = self.synthesize(&replay.context, function)?;
                let Term::Pi { parameter, .. } = function_synthesis.normalized_type() else {
                    return Err(BuildFailureV3::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                };
                self.visit_term(
                    root,
                    &replay.context,
                    function,
                    function_synthesis.normalized_type(),
                    &path_child(path, TypedOccurrencePathComponentV1::Function),
                )?;
                self.visit_term(
                    root,
                    &replay.context,
                    argument,
                    parameter,
                    &path_child(path, TypedOccurrencePathComponentV1::Argument),
                )
            }
            Term::Sigma { .. } | Term::Pair { .. } => Err(BuildFailureV3::Outside(
                OutsideFragmentReason::UnsupportedTerm,
            )),
            Term::First { .. } | Term::Second { .. } => Err(BuildFailureV3::Outside(
                OutsideFragmentReason::DescriptorProjection,
            )),
        }
    }

    fn visit_type(
        &mut self,
        root: &TypedOccurrenceRootIdV2,
        context: &DependentContext,
        term: &Term,
        path: &TypedOccurrencePathV2,
    ) -> BuildResultV3<()> {
        self.charge()?;
        let synthesis = self.synthesize(context, term)?;
        if !matches!(synthesis.normalized_type(), Term::Sort { .. }) {
            return Err(BuildFailureV3::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        }
        let replay = self.replay_type_formation(context, term)?;
        self.record(
            root,
            path,
            &replay.context,
            term,
            &synthesis,
            LocalJudgmentV1::TypeFormation {
                universe: synthesis.normalized_type().clone(),
            },
            replay.digest,
        )?;
        match term {
            Term::Sort { .. } | Term::Var { .. } | Term::Global { .. } | Term::UnitType => Ok(()),
            Term::Pi { parameter, body } => self.visit_binder_type(
                root,
                &replay.context,
                parameter,
                body,
                path,
                TypedBinderKindV1::Pi,
            ),
            Term::Apply { function, argument } => {
                let function_synthesis = self.synthesize(&replay.context, function)?;
                let Term::Pi { parameter, .. } = function_synthesis.normalized_type() else {
                    return Err(BuildFailureV3::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                };
                self.visit_term(
                    root,
                    &replay.context,
                    function,
                    function_synthesis.normalized_type(),
                    &path_child(path, TypedOccurrencePathComponentV1::Function),
                )?;
                self.visit_term(
                    root,
                    &replay.context,
                    argument,
                    parameter,
                    &path_child(path, TypedOccurrencePathComponentV1::Argument),
                )
            }
            Term::Lambda { .. } | Term::Unit => Err(BuildFailureV3::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            )),
            Term::Sigma { .. } | Term::Pair { .. } => Err(BuildFailureV3::Outside(
                OutsideFragmentReason::UnsupportedTerm,
            )),
            Term::First { .. } | Term::Second { .. } => Err(BuildFailureV3::Outside(
                OutsideFragmentReason::DescriptorProjection,
            )),
        }
    }

    fn visit_binder_type(
        &mut self,
        root: &TypedOccurrenceRootIdV2,
        context: &DependentContext,
        parameter: &Term,
        body: &Term,
        path: &TypedOccurrencePathV2,
        binder: TypedBinderKindV1,
    ) -> BuildResultV3<()> {
        let parameter_replay = self.replay_type_formation(context, parameter)?;
        self.visit_type(
            root,
            context,
            parameter,
            &path_child(
                path,
                TypedOccurrencePathComponentV1::BinderParameter { binder },
            ),
        )?;
        let extended = self.extend_context(context, parameter_replay.term)?;
        self.visit_type(
            root,
            &extended,
            body,
            &path_child(path, TypedOccurrencePathComponentV1::BinderBody { binder }),
        )
    }

    fn synthesize(
        &self,
        context: &DependentContext,
        term: &Term,
    ) -> BuildResultV3<VerifiedSynthesizedJudgmentV1> {
        self.check_context_bound(context)?;
        let proof = synthesize_lambda_unit_v1(self.kernel, self.signature, context, term)?;
        if proof.signature_digest() != self.signature.digest()
            || proof.input_context() != context
            || proof.term() != term
            || proof.kernel_protocol_digest() != &self.kernel.kernel_protocol_digest()
            || proof.synthesis_protocol_digest() != &synthesis_protocol_digest_v1()
        {
            return Err(BuildFailureV3::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        }
        Ok(proof)
    }

    fn extend_context(
        &self,
        context: &DependentContext,
        parameter: Term,
    ) -> BuildResultV3<DependentContext> {
        let mut entries = context.0.clone();
        entries.push(parameter);
        let candidate = DependentContext(entries);
        self.check_context_bound(&candidate)?;
        Ok(self
            .kernel
            .verify_context(self.signature, &candidate)?
            .normalized_wire())
    }

    fn replay_has_type(
        &self,
        context: &DependentContext,
        term: &Term,
        ty: &Term,
    ) -> BuildResultV3<HasTypeReplay> {
        self.check_context_bound(context)?;
        let input = OpenJudgment::HasType {
            context: context.clone(),
            term: term.clone(),
            ty: ty.clone(),
        };
        let output = self.kernel.verify_open_judgment(self.signature, &input)?;
        let OpenJudgment::HasType { context, ty, .. } = &output else {
            return Err(BuildFailureV3::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        };
        let digest = Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-kernel-replay/v2",
            &ReplayBinding {
                input,
                output: output.clone(),
            },
        );
        Ok(HasTypeReplay {
            context: context.clone(),
            ty: ty.clone(),
            digest,
        })
    }

    fn replay_type_formation(
        &self,
        context: &DependentContext,
        term: &Term,
    ) -> BuildResultV3<TypeFormationReplay> {
        self.check_context_bound(context)?;
        let input = OpenJudgment::TypeFormation {
            context: context.clone(),
            term: term.clone(),
        };
        let output = self.kernel.verify_open_judgment(self.signature, &input)?;
        let OpenJudgment::TypeFormation { context, term, .. } = &output else {
            return Err(BuildFailureV3::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        };
        let digest = Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-formation-replay/v2",
            &ReplayBinding {
                input,
                output: output.clone(),
            },
        );
        Ok(TypeFormationReplay {
            context: context.clone(),
            term: term.clone(),
            digest,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn record(
        &mut self,
        root: &TypedOccurrenceRootIdV2,
        path: &TypedOccurrencePathV2,
        local_context: &DependentContext,
        term: &Term,
        synthesis: &VerifiedSynthesizedJudgmentV1,
        checked_judgment: LocalJudgmentV1,
        kernel_replay_digest: Digest,
    ) -> BuildResultV3<()> {
        let id = TypedOccurrenceIdV2(Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-id/v2",
            &OccurrenceIdInput { root, path },
        ));
        if !self.occurrence_ids.insert(id.clone()) {
            return Err(BuildFailureV3::Unknown(
                AuditUnknownReason::IncompleteEnumeration,
            ));
        }
        let digest = Digest::of_canonical(
            "pen-semantic-audit/verified-typed-occurrence/v2",
            &OccurrenceDigestInput {
                id: &id,
                root,
                path,
                local_context,
                term,
                synthesized_judgment_digest: synthesis.digest(),
                checked_judgment: &checked_judgment,
                kernel_replay_digest: &kernel_replay_digest,
            },
        );
        self.occurrences.push(VerifiedTypedOccurrenceV2 {
            id,
            root: root.clone(),
            path: path.clone(),
            local_context: local_context.clone(),
            term: term.clone(),
            synthesized: synthesis.clone(),
            checked_judgment,
            kernel_replay_digest,
            digest,
        });
        Ok(())
    }
}

fn path_root(component: TypedOccurrencePathComponentV1) -> TypedOccurrencePathV2 {
    TypedOccurrencePathV2::root(component)
}

fn path_child(
    path: &TypedOccurrencePathV2,
    component: TypedOccurrencePathComponentV1,
) -> TypedOccurrencePathV2 {
    path.child(component)
}

#[derive(Clone, Debug)]
struct HasTypeReplay {
    context: DependentContext,
    ty: Term,
    digest: Digest,
}

#[derive(Clone, Debug)]
struct TypeFormationReplay {
    context: DependentContext,
    term: Term,
    digest: Digest,
}

struct CanonicalSlice<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSlice<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct RootIdInput<'a> {
    manifest_digest: &'a Digest,
    subject: &'a TypedOccurrenceRootRequestV1,
}

impl CanonicalEncode for RootIdInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.subject.encode_canonical(encoder);
    }
}

struct RootDigestInput<'a> {
    id: &'a TypedOccurrenceRootIdV2,
    subject: &'a TypedOccurrenceRootRequestV1,
    kernel_replay_digest: &'a Digest,
}

impl CanonicalEncode for RootDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.subject.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
    }
}

struct OccurrenceIdInput<'a> {
    root: &'a TypedOccurrenceRootIdV2,
    path: &'a TypedOccurrencePathV2,
}

impl CanonicalEncode for OccurrenceIdInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.root.encode_canonical(encoder);
        self.path.encode_canonical(encoder);
    }
}

struct OccurrenceDigestInput<'a> {
    id: &'a TypedOccurrenceIdV2,
    root: &'a TypedOccurrenceRootIdV2,
    path: &'a TypedOccurrencePathV2,
    local_context: &'a DependentContext,
    term: &'a Term,
    synthesized_judgment_digest: &'a Digest,
    checked_judgment: &'a LocalJudgmentV1,
    kernel_replay_digest: &'a Digest,
}

impl CanonicalEncode for OccurrenceDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.root.encode_canonical(encoder);
        self.path.encode_canonical(encoder);
        self.local_context.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.synthesized_judgment_digest.encode_canonical(encoder);
        self.checked_judgment.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
struct CoverageEntry {
    root: TypedOccurrenceRootIdV2,
    path: TypedOccurrencePathV2,
    occurrence: TypedOccurrenceIdV2,
}

impl CanonicalEncode for CoverageEntry {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.root.encode_canonical(encoder);
        self.path.encode_canonical(encoder);
        self.occurrence.encode_canonical(encoder);
    }
}

struct BatchDigestInput<'a>(&'a VerifiedSynthesisBackedOccurrenceBatchV1);

impl CanonicalEncode for BatchDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.0.schema_version);
        self.0.semantic_manifest_digest.encode_canonical(encoder);
        self.0.signature_digest.encode_canonical(encoder);
        self.0.kernel_protocol_digest.encode_canonical(encoder);
        self.0.synthesis_protocol_digest.encode_canonical(encoder);
        self.0.roots_digest.encode_canonical(encoder);
        encoder.sequence(&self.0.roots);
        encoder.sequence(&self.0.occurrences);
        self.0.coverage_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
struct ReplayBinding {
    input: OpenJudgment,
    output: OpenJudgment,
}

impl CanonicalEncode for ReplayBinding {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.input.encode_canonical(encoder);
        self.output.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
struct DigestPair {
    left: Digest,
    right: Digest,
}

impl CanonicalEncode for DigestPair {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.left.encode_canonical(encoder);
        self.right.encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v3,
        verify_semantic_audit_lambda_unit_manifest_v3,
    };
    use pen_kernel::{KernelLimits, UncheckedSignature};

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("safe kernel")
    }

    fn manifest() -> VerifiedSemanticAuditManifestV3 {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v3(
            &proposed_semantic_audit_lambda_unit_manifest_v3(),
        ) else {
            panic!("exact V3 manifest");
        };
        manifest
    }

    #[test]
    fn variable_headed_application_has_fixed_root_batch_evidence() {
        let kernel = kernel();
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let function_type = Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        };
        let request = TypedOccurrenceRootRequestV1::HasType {
            context: DependentContext(vec![function_type]),
            term: Term::Apply {
                function: Box::new(Term::Var { index: 0 }),
                argument: Box::new(Term::Unit),
            },
            ty: Term::UnitType,
        };
        let AuditDecision::Proven(batch) = verify_synthesis_backed_occurrence_batch_v1(
            &manifest(),
            &kernel,
            &signature,
            &[request],
        ) else {
            panic!("synthesis closes the local fixed-root V2 gap");
        };
        assert_eq!(batch.roots().len(), 1);
        assert_eq!(batch.occurrences().len(), 4);
        assert!(batch.occurrences().iter().any(|occurrence| {
            occurrence.path().components()
                == [
                    TypedOccurrencePathComponentV1::JudgmentTerm,
                    TypedOccurrencePathComponentV1::Function,
                ]
                && occurrence.term() == &Term::Var { index: 0 }
        }));
    }

    #[test]
    fn caller_roots_never_upgrade_to_authoritative_census() {
        let kernel = kernel();
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let AuditDecision::Proven(empty_batch) =
            verify_synthesis_backed_occurrence_batch_v1(&manifest(), &kernel, &signature, &[])
        else {
            panic!("empty fixed-root diagnostic batch");
        };
        assert!(empty_batch.roots().is_empty());
        assert!(matches!(
            diagnose_synthesis_backed_typed_occurrence_census_v3(&manifest()),
            AuditDecision::Unknown(AuditUnknownReason::MissingSynthesisBackedTypedOccurrenceCensus)
        ));
        assert_eq!(
            SYNTHESIS_BACKED_OCCURRENCE_CENSUS_PREREQUISITES_V3,
            [
                SynthesisBackedOccurrenceCensusPrerequisiteV3::LambdaUnitTypingMetatheory,
                SynthesisBackedOccurrenceCensusPrerequisiteV3::NativeRankInductiveCarrier,
                SynthesisBackedOccurrenceCensusPrerequisiteV3::CarrierDerivedExhaustiveRootInventory,
            ]
        );
    }

    #[test]
    fn mismatch_outside_and_resource_fail_without_partial_batch() {
        let kernel = kernel();
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let mismatch = TypedOccurrenceRootRequestV1::HasType {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::Sort { level: 0 },
        };
        assert!(matches!(
            verify_synthesis_backed_occurrence_batch_v1(
                &manifest(),
                &kernel,
                &signature,
                &[mismatch]
            ),
            AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify)
        ));

        let outside = TypedOccurrenceRootRequestV1::TypeFormation {
            context: DependentContext::default(),
            term: Term::Sigma {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            },
        };
        assert!(matches!(
            verify_synthesis_backed_occurrence_batch_v1(
                &manifest(),
                &kernel,
                &signature,
                &[outside]
            ),
            AuditDecision::OutsideFragment(OutsideFragmentReason::UnsupportedTerm)
        ));

        let tiny = Kernel::new(pen_kernel::KernelLimits {
            max_operations: 1,
            max_depth: 4,
            normalization_fuel: 4,
        })
        .expect("tiny kernel");
        let tiny_signature = tiny
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let request = TypedOccurrenceRootRequestV1::HasType {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        };
        assert!(matches!(
            verify_synthesis_backed_occurrence_batch_v1(
                &manifest(),
                &tiny,
                &tiny_signature,
                &[request]
            ),
            AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted)
        ));
    }
}
