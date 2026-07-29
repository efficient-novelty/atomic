//! Binder-local typed-occurrence reconstruction for the V2 semantic profile.
//!
//! This module records syntax positions only after replaying their local
//! typing judgment with `pen-kernel`.  Binder bodies are checked in the
//! context obtained by appending the checked, normalized parameter type; an
//! occurrence is never justified by shifting a type out of an outer context.
//!
//! The public kernel currently checks caller-supplied types but does not
//! expose an inferred type.  Consequently, this verifier is complete for
//! bidirectional positions and for synthesis headed by a global or another
//! synthesizable term.  A position that genuinely requires an unavailable
//! lookup result (for example, a variable in function position) fails closed
//! with [`AuditUnknownReason::MissingTypedOccurrenceCensus`].  No partial
//! census capability is minted.

use crate::fragment::{
    lambda_unit_context_syntax_violation, lambda_unit_judgment_syntax_violation,
    lambda_unit_term_syntax_violation,
};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2, TypedOccurrenceCensusProtocolV2,
    VerifiedSemanticAuditManifestV2,
};
use crate::model::GenericJudgmentV1;
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError, OpenJudgment,
    ResourceKind, Term, VerifiedSignature,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

pub const TYPED_OCCURRENCE_CENSUS_SCHEMA_VERSION_V1: u16 = 1;

/// A caller-supplied root to be checked and included in one exact census.
///
/// This is input data, not an authority object.  Root and occurrence
/// identities are minted only after kernel replay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "judgment", rename_all = "snake_case")]
pub enum TypedOccurrenceRootRequestV1 {
    HasType {
        context: DependentContext,
        term: Term,
        ty: Term,
    },
    Equation {
        context: DependentContext,
        left: Term,
        right: Term,
        ty: Term,
    },
    TypeFormation {
        context: DependentContext,
        term: Term,
    },
}

impl TypedOccurrenceRootRequestV1 {
    pub fn context(&self) -> &DependentContext {
        match self {
            Self::HasType { context, .. }
            | Self::Equation { context, .. }
            | Self::TypeFormation { context, .. } => context,
        }
    }
}

impl CanonicalEncode for TypedOccurrenceRootRequestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::HasType { context, term, ty } => {
                encoder.tag(0);
                context.encode_canonical(encoder);
                term.encode_canonical(encoder);
                ty.encode_canonical(encoder);
            }
            Self::Equation {
                context,
                left,
                right,
                ty,
            } => {
                encoder.tag(1);
                context.encode_canonical(encoder);
                left.encode_canonical(encoder);
                right.encode_canonical(encoder);
                ty.encode_canonical(encoder);
            }
            Self::TypeFormation { context, term } => {
                encoder.tag(2);
                context.encode_canonical(encoder);
                term.encode_canonical(encoder);
            }
        }
    }
}

/// Verifier-minted identity of one checked root.
///
/// The field is private and this type deliberately has no `Deserialize`
/// implementation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct TypedOccurrenceRootIdV1(Digest);

impl TypedOccurrenceRootIdV1 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for TypedOccurrenceRootIdV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// Verifier-minted identity of one typed syntax occurrence.
///
/// The field is private and this type deliberately has no `Deserialize`
/// implementation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct TypedOccurrenceIdV1(Digest);

impl TypedOccurrenceIdV1 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for TypedOccurrenceIdV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TypedBinderKindV1 {
    Pi,
    Lambda,
}

impl CanonicalEncode for TypedBinderKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Pi => 0,
            Self::Lambda => 1,
        });
    }
}

/// One typed step in a syntax path.
///
/// Equation side and judgment-type markers are part of the path rather than
/// out-of-band flags, so two equal subterms in distinct judgment positions do
/// not share an occurrence identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "component", rename_all = "snake_case")]
pub enum TypedOccurrencePathComponentV1 {
    JudgmentTerm,
    EquationLeft,
    EquationRight,
    TypeFormationSubject,
    JudgmentType,
    Function,
    Argument,
    BinderParameter { binder: TypedBinderKindV1 },
    BinderBody { binder: TypedBinderKindV1 },
}

impl CanonicalEncode for TypedOccurrencePathComponentV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::JudgmentTerm => encoder.tag(0),
            Self::EquationLeft => encoder.tag(1),
            Self::EquationRight => encoder.tag(2),
            Self::TypeFormationSubject => encoder.tag(3),
            Self::JudgmentType => encoder.tag(4),
            Self::Function => encoder.tag(5),
            Self::Argument => encoder.tag(6),
            Self::BinderParameter { binder } => {
                encoder.tag(7);
                binder.encode_canonical(encoder);
            }
            Self::BinderBody { binder } => {
                encoder.tag(8);
                binder.encode_canonical(encoder);
            }
        }
    }
}

/// Verifier-minted exact path from a checked root.
///
/// Callers can inspect but cannot construct or deserialize paths.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct TypedOccurrencePathV1(Vec<TypedOccurrencePathComponentV1>);

impl TypedOccurrencePathV1 {
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

impl CanonicalEncode for TypedOccurrencePathV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.0);
    }
}

/// The exact local judgment replayed for an occurrence.
///
/// A formation occurrence records the universe returned by a successful
/// bounded kernel search.  `universe` is therefore evidence-bearing data
/// inside a verified record, not a caller-supplied universe annotation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "judgment", rename_all = "snake_case")]
pub enum LocalJudgmentV1 {
    HasType { ty: Term },
    TypeFormation { universe: Term },
}

impl LocalJudgmentV1 {
    pub fn ty(&self) -> &Term {
        match self {
            Self::HasType { ty } => ty,
            Self::TypeFormation { universe } => universe,
        }
    }

    pub fn is_type_formation(&self) -> bool {
        matches!(self, Self::TypeFormation { .. })
    }
}

impl CanonicalEncode for LocalJudgmentV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::HasType { ty } => {
                encoder.tag(0);
                ty.encode_canonical(encoder);
            }
            Self::TypeFormation { universe } => {
                encoder.tag(1);
                universe.encode_canonical(encoder);
            }
        }
    }
}

/// One verifier-minted, binder-local typed syntax occurrence.
///
/// Every field is private, and the record deliberately has no `Deserialize`
/// implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedTypedOccurrenceV1 {
    id: TypedOccurrenceIdV1,
    root: TypedOccurrenceRootIdV1,
    path: TypedOccurrencePathV1,
    local_context: DependentContext,
    term: Term,
    judgment: LocalJudgmentV1,
    kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedTypedOccurrenceV1 {
    pub fn id(&self) -> &TypedOccurrenceIdV1 {
        &self.id
    }

    pub fn root(&self) -> &TypedOccurrenceRootIdV1 {
        &self.root
    }

    pub fn path(&self) -> &TypedOccurrencePathV1 {
        &self.path
    }

    pub fn local_context(&self) -> &DependentContext {
        &self.local_context
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn judgment(&self) -> &LocalJudgmentV1 {
        &self.judgment
    }

    pub fn kernel_replay_digest(&self) -> &Digest {
        &self.kernel_replay_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedTypedOccurrenceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.root.encode_canonical(encoder);
        self.path.encode_canonical(encoder);
        self.local_context.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.judgment.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// One checked root retained by the census.
///
/// The normalized context and judgment type are retained while term syntax is
/// deliberately not normalized away: rewrite roots may contain beta, delta,
/// or fresh-rule redexes.  Fields are private and the type is not
/// deserializable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedTypedOccurrenceRootV1 {
    id: TypedOccurrenceRootIdV1,
    subject: TypedOccurrenceRootRequestV1,
    kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedTypedOccurrenceRootV1 {
    pub fn id(&self) -> &TypedOccurrenceRootIdV1 {
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

impl CanonicalEncode for VerifiedTypedOccurrenceRootV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.subject.encode_canonical(encoder);
        self.kernel_replay_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Complete occurrence authority for the exact supplied root set.
///
/// Completeness is relative to `roots_digest`; downstream rewrite code must
/// compare that digest with its independently reconstructed root/reduct set.
/// No partial builder state can construct this type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedTypedOccurrenceCensusV1 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    signature_digest: Digest,
    roots_digest: Digest,
    roots: Vec<VerifiedTypedOccurrenceRootV1>,
    occurrences: Vec<VerifiedTypedOccurrenceV1>,
    coverage_digest: Digest,
    digest: Digest,
}

impl VerifiedTypedOccurrenceCensusV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn roots_digest(&self) -> &Digest {
        &self.roots_digest
    }

    pub fn roots(&self) -> &[VerifiedTypedOccurrenceRootV1] {
        &self.roots
    }

    pub fn occurrences(&self) -> &[VerifiedTypedOccurrenceV1] {
        &self.occurrences
    }

    pub fn occurrences_for_root(
        &self,
        root: &TypedOccurrenceRootIdV1,
    ) -> impl Iterator<Item = &VerifiedTypedOccurrenceV1> {
        self.occurrences
            .iter()
            .filter(move |occurrence| occurrence.root() == root)
    }

    pub fn occurrence(&self, id: &TypedOccurrenceIdV1) -> Option<&VerifiedTypedOccurrenceV1> {
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

impl CanonicalEncode for VerifiedTypedOccurrenceCensusV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.roots_digest.encode_canonical(encoder);
        encoder.sequence(&self.roots);
        encoder.sequence(&self.occurrences);
        self.coverage_digest.encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// Reconstruct a complete binder-local occurrence census for `roots`.
///
/// An unsynthesizable position returns
/// `Unknown(MissingTypedOccurrenceCensus)` and no capability.  In particular,
/// this function never substitutes a hand-shifted outer-context type for the
/// kernel's unavailable inferred variable-lookup result.
pub fn verify_typed_occurrence_census_v2(
    manifest: &VerifiedSemanticAuditManifestV2,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    roots: &[TypedOccurrenceRootRequestV1],
) -> AuditDecision<VerifiedTypedOccurrenceCensusV1> {
    let manifest_wire = manifest.manifest();
    if manifest_wire.profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2
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

    let mut builder = TypedOccurrenceBuilderV1 {
        manifest,
        kernel,
        signature,
        operations_left: kernel.limits().max_operations,
        occurrences: Vec::new(),
        occurrence_ids: BTreeSet::new(),
    };
    match builder.build(roots) {
        Ok(census) => AuditDecision::Proven(census),
        Err(failure) => failure.into_decision(),
    }
}

#[derive(Clone, Debug)]
enum BuildFailureV1 {
    Unknown(AuditUnknownReason),
    Outside(OutsideFragmentReason),
}

impl BuildFailureV1 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::Unknown(reason) => AuditDecision::Unknown(reason),
            Self::Outside(reason) => AuditDecision::OutsideFragment(reason),
        }
    }
}

impl From<KernelError> for BuildFailureV1 {
    fn from(error: KernelError) -> Self {
        match error {
            KernelError::ResourceExhausted(
                ResourceKind::Operations | ResourceKind::Depth | ResourceKind::Normalization,
            ) => Self::Unknown(AuditUnknownReason::ResourceExhausted),
            _ => Self::Unknown(AuditUnknownReason::KernelCouldNotCertify),
        }
    }
}

type BuildResultV1<T> = Result<T, BuildFailureV1>;

struct TypedOccurrenceBuilderV1<'a> {
    manifest: &'a VerifiedSemanticAuditManifestV2,
    kernel: &'a Kernel,
    signature: &'a VerifiedSignature,
    operations_left: u32,
    occurrences: Vec<VerifiedTypedOccurrenceV1>,
    occurrence_ids: BTreeSet<TypedOccurrenceIdV1>,
}

impl TypedOccurrenceBuilderV1<'_> {
    fn build(
        &mut self,
        requests: &[TypedOccurrenceRootRequestV1],
    ) -> BuildResultV1<VerifiedTypedOccurrenceCensusV1> {
        let mut roots_by_id = BTreeMap::new();
        for request in requests {
            self.charge()?;
            self.check_root_fragment(request)?;
            let root = self.verify_root(request)?;
            match roots_by_id.get(root.id()) {
                Some(prior) if prior != &root => {
                    return Err(BuildFailureV1::Unknown(
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
            "pen-semantic-audit/typed-occurrence-roots/v1",
            &CanonicalSliceV1(&roots),
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
            return Err(BuildFailureV1::Unknown(
                AuditUnknownReason::IncompleteEnumeration,
            ));
        }

        let coverage = self
            .occurrences
            .iter()
            .map(|occurrence| TypedOccurrenceCoverageEntryV1 {
                root: occurrence.root().clone(),
                path: occurrence.path().clone(),
                occurrence: occurrence.id().clone(),
            })
            .collect::<Vec<_>>();
        let coverage_digest = Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-coverage/v1",
            &CanonicalSliceV1(&coverage),
        );

        let mut census = VerifiedTypedOccurrenceCensusV1 {
            schema_version: TYPED_OCCURRENCE_CENSUS_SCHEMA_VERSION_V1,
            semantic_manifest_digest: self.manifest.candidate_digest().clone(),
            signature_digest: self.signature.digest().clone(),
            roots_digest,
            roots,
            occurrences: std::mem::take(&mut self.occurrences),
            coverage_digest,
            digest: Digest::of_bytes(b"typed-occurrence-census-pending"),
        };
        census.digest = Digest::of_canonical(
            "pen-semantic-audit/verified-typed-occurrence-census/v1",
            &VerifiedTypedOccurrenceCensusDigestInputV1(&census),
        );
        Ok(census)
    }

    fn charge(&mut self) -> BuildResultV1<()> {
        self.operations_left =
            self.operations_left
                .checked_sub(1)
                .ok_or(BuildFailureV1::Unknown(
                    AuditUnknownReason::ResourceExhausted,
                ))?;
        Ok(())
    }

    fn check_context_bound(&self, context: &DependentContext) -> BuildResultV1<()> {
        if context.0.len() > usize::from(self.manifest.manifest().maximum_context_entries) {
            return Err(BuildFailureV1::Unknown(
                AuditUnknownReason::ResourceExhausted,
            ));
        }
        Ok(())
    }

    fn check_root_fragment(&self, request: &TypedOccurrenceRootRequestV1) -> BuildResultV1<()> {
        let levels = &self.manifest.manifest().universe_levels;
        self.check_context_bound(request.context())?;
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
            return Err(BuildFailureV1::Outside(violation.outside_reason()));
        }
        Ok(())
    }

    fn verify_root(
        &mut self,
        request: &TypedOccurrenceRootRequestV1,
    ) -> BuildResultV1<VerifiedTypedOccurrenceRootV1> {
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
                    return Err(BuildFailureV1::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                }
                let digest = Digest::of_canonical(
                    "pen-semantic-audit/typed-occurrence-equation-root-replay/v1",
                    &ReplayDigestPairV1 {
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

        let id = TypedOccurrenceRootIdV1(Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-root-id/v1",
            &subject,
        ));
        let mut root = VerifiedTypedOccurrenceRootV1 {
            id,
            subject,
            kernel_replay_digest,
            digest: Digest::of_bytes(b"typed-occurrence-root-pending"),
        };
        root.digest = Digest::of_canonical(
            "pen-semantic-audit/verified-typed-occurrence-root/v1",
            &VerifiedTypedOccurrenceRootDigestInputV1(&root),
        );
        Ok(root)
    }

    fn visit_root(&mut self, root: &VerifiedTypedOccurrenceRootV1) -> BuildResultV1<()> {
        match root.subject() {
            TypedOccurrenceRootRequestV1::HasType { context, term, ty } => {
                self.visit_term(
                    root.id(),
                    context,
                    term,
                    ty,
                    &TypedOccurrencePathV1::root(TypedOccurrencePathComponentV1::JudgmentTerm),
                )?;
                self.visit_type(
                    root.id(),
                    context,
                    ty,
                    &TypedOccurrencePathV1::root(TypedOccurrencePathComponentV1::JudgmentType),
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
                    &TypedOccurrencePathV1::root(TypedOccurrencePathComponentV1::EquationLeft),
                )?;
                self.visit_term(
                    root.id(),
                    context,
                    right,
                    ty,
                    &TypedOccurrencePathV1::root(TypedOccurrencePathComponentV1::EquationRight),
                )?;
                self.visit_type(
                    root.id(),
                    context,
                    ty,
                    &TypedOccurrencePathV1::root(TypedOccurrencePathComponentV1::JudgmentType),
                )
            }
            TypedOccurrenceRootRequestV1::TypeFormation { context, term } => self.visit_type(
                root.id(),
                context,
                term,
                &TypedOccurrencePathV1::root(TypedOccurrencePathComponentV1::TypeFormationSubject),
            ),
        }
    }

    fn visit_term(
        &mut self,
        root: &TypedOccurrenceRootIdV1,
        context: &DependentContext,
        term: &Term,
        expected: &Term,
        path: &TypedOccurrencePathV1,
    ) -> BuildResultV1<()> {
        self.charge()?;
        self.check_context_bound(context)?;
        let replay = self.replay_has_type(context, term, expected)?;
        self.record(
            root,
            path,
            context,
            term,
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
            Term::Pi { parameter, body } => {
                self.visit_binder_type(root, context, parameter, body, path, TypedBinderKindV1::Pi)
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                let Term::Pi {
                    parameter,
                    body: expected_body,
                } = replay.ty
                else {
                    return Err(BuildFailureV1::Unknown(
                        AuditUnknownReason::MissingTypedOccurrenceCensus,
                    ));
                };
                let parameter_replay = self.replay_type_formation(context, parameter_type)?;
                if parameter_replay.term != *parameter {
                    return Err(BuildFailureV1::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                }
                self.visit_type(
                    root,
                    context,
                    parameter_type,
                    &path.child(TypedOccurrencePathComponentV1::BinderParameter {
                        binder: TypedBinderKindV1::Lambda,
                    }),
                )?;
                let extended = self.extend_context(context, parameter_replay.term)?;
                self.visit_term(
                    root,
                    &extended,
                    body,
                    &expected_body,
                    &path.child(TypedOccurrencePathComponentV1::BinderBody {
                        binder: TypedBinderKindV1::Lambda,
                    }),
                )
            }
            Term::Apply { function, argument } => {
                let function_type = self.synthesize_term_type(context, function)?;
                let Term::Pi { parameter, .. } = &function_type else {
                    return Err(BuildFailureV1::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                };
                self.visit_term(
                    root,
                    context,
                    function,
                    &function_type,
                    &path.child(TypedOccurrencePathComponentV1::Function),
                )?;
                self.visit_term(
                    root,
                    context,
                    argument,
                    parameter,
                    &path.child(TypedOccurrencePathComponentV1::Argument),
                )
            }
            Term::Sigma { .. } | Term::Pair { .. } => Err(BuildFailureV1::Outside(
                OutsideFragmentReason::UnsupportedTerm,
            )),
            Term::First { .. } | Term::Second { .. } => Err(BuildFailureV1::Outside(
                OutsideFragmentReason::DescriptorProjection,
            )),
        }
    }

    fn visit_type(
        &mut self,
        root: &TypedOccurrenceRootIdV1,
        context: &DependentContext,
        term: &Term,
        path: &TypedOccurrencePathV1,
    ) -> BuildResultV1<()> {
        self.charge()?;
        self.check_context_bound(context)?;
        let replay = self.replay_type_formation(context, term)?;
        self.record(
            root,
            path,
            context,
            term,
            LocalJudgmentV1::TypeFormation {
                universe: replay.universe,
            },
            replay.digest,
        )?;

        match term {
            Term::Sort { .. } | Term::Var { .. } | Term::Global { .. } | Term::UnitType => Ok(()),
            Term::Pi { parameter, body } => {
                self.visit_binder_type(root, context, parameter, body, path, TypedBinderKindV1::Pi)
            }
            Term::Apply { function, argument } => {
                let function_type = self.synthesize_term_type(context, function)?;
                let Term::Pi { parameter, .. } = &function_type else {
                    return Err(BuildFailureV1::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                };
                self.visit_term(
                    root,
                    context,
                    function,
                    &function_type,
                    &path.child(TypedOccurrencePathComponentV1::Function),
                )?;
                self.visit_term(
                    root,
                    context,
                    argument,
                    parameter,
                    &path.child(TypedOccurrencePathComponentV1::Argument),
                )
            }
            Term::Lambda { .. } | Term::Unit => Err(BuildFailureV1::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            )),
            Term::Sigma { .. } | Term::Pair { .. } => Err(BuildFailureV1::Outside(
                OutsideFragmentReason::UnsupportedTerm,
            )),
            Term::First { .. } | Term::Second { .. } => Err(BuildFailureV1::Outside(
                OutsideFragmentReason::DescriptorProjection,
            )),
        }
    }

    fn visit_binder_type(
        &mut self,
        root: &TypedOccurrenceRootIdV1,
        context: &DependentContext,
        parameter: &Term,
        body: &Term,
        path: &TypedOccurrencePathV1,
        binder: TypedBinderKindV1,
    ) -> BuildResultV1<()> {
        let parameter_replay = self.replay_type_formation(context, parameter)?;
        self.visit_type(
            root,
            context,
            parameter,
            &path.child(TypedOccurrencePathComponentV1::BinderParameter { binder }),
        )?;
        let extended = self.extend_context(context, parameter_replay.term)?;
        self.visit_type(
            root,
            &extended,
            body,
            &path.child(TypedOccurrencePathComponentV1::BinderBody { binder }),
        )
    }

    fn extend_context(
        &mut self,
        context: &DependentContext,
        parameter: Term,
    ) -> BuildResultV1<DependentContext> {
        let mut entries = context.0.clone();
        entries.push(parameter);
        let candidate = DependentContext(entries);
        self.check_context_bound(&candidate)?;
        let checked = self.kernel.verify_context(self.signature, &candidate)?;
        Ok(checked.normalized_wire())
    }

    fn synthesize_term_type(
        &mut self,
        context: &DependentContext,
        term: &Term,
    ) -> BuildResultV1<Term> {
        self.charge()?;
        let candidate = match term {
            Term::Sort { level } => Term::Sort {
                level: level.checked_add(1).ok_or(BuildFailureV1::Outside(
                    OutsideFragmentReason::UnsupportedUniverseLevel,
                ))?,
            },
            Term::Var { .. } => {
                // `pen-kernel` can check a supplied variable type but does not
                // expose the inferred lookup result.  Shifting an outer
                // context entry here would recreate the unsound approximation
                // this census is designed to eliminate.
                return Err(BuildFailureV1::Unknown(
                    AuditUnknownReason::MissingTypedOccurrenceCensus,
                ));
            }
            Term::Global { id } => self
                .signature
                .declarations()
                .iter()
                .find(|declaration| &declaration.id == id)
                .map(|declaration| declaration.ty.clone())
                .ok_or(BuildFailureV1::Unknown(
                    AuditUnknownReason::KernelCouldNotCertify,
                ))?,
            Term::Pi { .. } => self.replay_type_formation(context, term)?.universe,
            Term::Lambda {
                parameter_type,
                body,
            } => {
                let parameter_replay = self.replay_type_formation(context, parameter_type)?;
                let extended = self.extend_context(context, parameter_replay.term.clone())?;
                let body_type = self.synthesize_term_type(&extended, body)?;
                Term::Pi {
                    parameter: Box::new(parameter_replay.term),
                    body: Box::new(body_type),
                }
            }
            Term::Apply { function, argument } => {
                let function_type = self.synthesize_term_type(context, function)?;
                let Term::Pi { parameter, body } = function_type else {
                    return Err(BuildFailureV1::Unknown(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                };
                self.replay_has_type(context, argument, &parameter)?;
                substitute_top_v1(&body, argument).ok_or(BuildFailureV1::Unknown(
                    AuditUnknownReason::MissingTypedOccurrenceCensus,
                ))?
            }
            Term::UnitType => Term::Sort { level: 0 },
            Term::Unit => Term::UnitType,
            Term::Sigma { .. } | Term::Pair { .. } => {
                return Err(BuildFailureV1::Outside(
                    OutsideFragmentReason::UnsupportedTerm,
                ));
            }
            Term::First { .. } | Term::Second { .. } => {
                return Err(BuildFailureV1::Outside(
                    OutsideFragmentReason::DescriptorProjection,
                ));
            }
        };
        Ok(self.replay_has_type(context, term, &candidate)?.ty)
    }

    fn replay_has_type(
        &mut self,
        context: &DependentContext,
        term: &Term,
        ty: &Term,
    ) -> BuildResultV1<HasTypeReplayV1> {
        self.check_context_bound(context)?;
        let input = OpenJudgment::HasType {
            context: context.clone(),
            term: term.clone(),
            ty: ty.clone(),
        };
        let output = self.kernel.verify_open_judgment(self.signature, &input)?;
        let OpenJudgment::HasType {
            context,
            term: _,
            ty,
        } = &output
        else {
            return Err(BuildFailureV1::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        };
        let digest = Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-kernel-replay/v1",
            &KernelReplayBindingV1 {
                input,
                output: output.clone(),
            },
        );
        Ok(HasTypeReplayV1 {
            context: context.clone(),
            ty: ty.clone(),
            digest,
        })
    }

    fn replay_type_formation(
        &mut self,
        context: &DependentContext,
        term: &Term,
    ) -> BuildResultV1<TypeFormationReplayV1> {
        self.check_context_bound(context)?;
        let formation_input = OpenJudgment::TypeFormation {
            context: context.clone(),
            term: term.clone(),
        };
        let formation_output = self
            .kernel
            .verify_open_judgment(self.signature, &formation_input)?;
        let OpenJudgment::TypeFormation {
            context: normalized_context,
            term: normalized_term,
        } = &formation_output
        else {
            return Err(BuildFailureV1::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        };
        let normalized_context = normalized_context.clone();
        let normalized_term = normalized_term.clone();

        let mut universe_levels = BTreeSet::new();
        for level in &self.manifest.manifest().universe_levels {
            universe_levels.insert(*level);
            if let Some(successor) = level.checked_add(1) {
                universe_levels.insert(successor);
            }
        }
        let mut successful = None;
        for level in universe_levels {
            let candidate = Term::Sort { level };
            let typing_input = OpenJudgment::HasType {
                context: normalized_context.clone(),
                term: term.clone(),
                ty: candidate,
            };
            let Ok(typing_output) = self
                .kernel
                .verify_open_judgment(self.signature, &typing_input)
            else {
                continue;
            };
            let OpenJudgment::HasType { ty, .. } = &typing_output else {
                continue;
            };
            if successful.is_some() {
                return Err(BuildFailureV1::Unknown(
                    AuditUnknownReason::KernelCouldNotCertify,
                ));
            }
            successful = Some((ty.clone(), typing_input, typing_output));
        }
        let Some((universe, typing_input, typing_output)) = successful else {
            return Err(BuildFailureV1::Outside(
                OutsideFragmentReason::UnsupportedUniverseLevel,
            ));
        };

        let digest = Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-formation-replay/v1",
            &TypeFormationReplayBindingV1 {
                formation_input,
                formation_output,
                typing_input,
                typing_output,
            },
        );
        Ok(TypeFormationReplayV1 {
            context: normalized_context,
            term: normalized_term,
            universe,
            digest,
        })
    }

    fn record(
        &mut self,
        root: &TypedOccurrenceRootIdV1,
        path: &TypedOccurrencePathV1,
        local_context: &DependentContext,
        term: &Term,
        judgment: LocalJudgmentV1,
        kernel_replay_digest: Digest,
    ) -> BuildResultV1<()> {
        let id_input = TypedOccurrenceIdInputV1 {
            root: root.clone(),
            path: path.clone(),
        };
        let id = TypedOccurrenceIdV1(Digest::of_canonical(
            "pen-semantic-audit/typed-occurrence-id/v1",
            &id_input,
        ));
        if !self.occurrence_ids.insert(id.clone()) {
            return Err(BuildFailureV1::Unknown(
                AuditUnknownReason::IncompleteEnumeration,
            ));
        }
        let mut occurrence = VerifiedTypedOccurrenceV1 {
            id,
            root: root.clone(),
            path: path.clone(),
            local_context: local_context.clone(),
            term: term.clone(),
            judgment,
            kernel_replay_digest,
            digest: Digest::of_bytes(b"typed-occurrence-pending"),
        };
        occurrence.digest = Digest::of_canonical(
            "pen-semantic-audit/verified-typed-occurrence/v1",
            &VerifiedTypedOccurrenceDigestInputV1(&occurrence),
        );
        self.occurrences.push(occurrence);
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct HasTypeReplayV1 {
    context: DependentContext,
    ty: Term,
    digest: Digest,
}

#[derive(Clone, Debug)]
struct TypeFormationReplayV1 {
    context: DependentContext,
    term: Term,
    universe: Term,
    digest: Digest,
}

#[derive(Clone, Debug)]
struct KernelReplayBindingV1 {
    input: OpenJudgment,
    output: OpenJudgment,
}

impl CanonicalEncode for KernelReplayBindingV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.input.encode_canonical(encoder);
        self.output.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
struct TypeFormationReplayBindingV1 {
    formation_input: OpenJudgment,
    formation_output: OpenJudgment,
    typing_input: OpenJudgment,
    typing_output: OpenJudgment,
}

impl CanonicalEncode for TypeFormationReplayBindingV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.formation_input.encode_canonical(encoder);
        self.formation_output.encode_canonical(encoder);
        self.typing_input.encode_canonical(encoder);
        self.typing_output.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
struct ReplayDigestPairV1 {
    left: Digest,
    right: Digest,
}

impl CanonicalEncode for ReplayDigestPairV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.left.encode_canonical(encoder);
        self.right.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
struct TypedOccurrenceIdInputV1 {
    root: TypedOccurrenceRootIdV1,
    path: TypedOccurrencePathV1,
}

impl CanonicalEncode for TypedOccurrenceIdInputV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.root.encode_canonical(encoder);
        self.path.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
struct TypedOccurrenceCoverageEntryV1 {
    root: TypedOccurrenceRootIdV1,
    path: TypedOccurrencePathV1,
    occurrence: TypedOccurrenceIdV1,
}

impl CanonicalEncode for TypedOccurrenceCoverageEntryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.root.encode_canonical(encoder);
        self.path.encode_canonical(encoder);
        self.occurrence.encode_canonical(encoder);
    }
}

struct CanonicalSliceV1<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSliceV1<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct VerifiedTypedOccurrenceRootDigestInputV1<'a>(&'a VerifiedTypedOccurrenceRootV1);

impl CanonicalEncode for VerifiedTypedOccurrenceRootDigestInputV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.id.encode_canonical(encoder);
        self.0.subject.encode_canonical(encoder);
        self.0.kernel_replay_digest.encode_canonical(encoder);
    }
}

struct VerifiedTypedOccurrenceDigestInputV1<'a>(&'a VerifiedTypedOccurrenceV1);

impl CanonicalEncode for VerifiedTypedOccurrenceDigestInputV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.id.encode_canonical(encoder);
        self.0.root.encode_canonical(encoder);
        self.0.path.encode_canonical(encoder);
        self.0.local_context.encode_canonical(encoder);
        self.0.term.encode_canonical(encoder);
        self.0.judgment.encode_canonical(encoder);
        self.0.kernel_replay_digest.encode_canonical(encoder);
    }
}

struct VerifiedTypedOccurrenceCensusDigestInputV1<'a>(&'a VerifiedTypedOccurrenceCensusV1);

impl CanonicalEncode for VerifiedTypedOccurrenceCensusDigestInputV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.0.schema_version);
        self.0.semantic_manifest_digest.encode_canonical(encoder);
        self.0.signature_digest.encode_canonical(encoder);
        self.0.roots_digest.encode_canonical(encoder);
        encoder.sequence(&self.0.roots);
        encoder.sequence(&self.0.occurrences);
        self.0.coverage_digest.encode_canonical(encoder);
    }
}

fn substitute_top_v1(body: &Term, replacement: &Term) -> Option<Term> {
    let lifted = shift_term_v1(replacement, 1, 0)?;
    let replaced = substitute_term_v1(body, 0, &lifted, 0)?;
    shift_term_v1(&replaced, -1, 0)
}

fn substitute_term_v1(
    term: &Term,
    target: u32,
    replacement: &Term,
    binder_depth: u32,
) -> Option<Term> {
    match term {
        Term::Sort { level } => Some(Term::Sort { level: *level }),
        Term::Var { index } => {
            let sought = target.checked_add(binder_depth)?;
            if *index == sought {
                shift_term_v1(replacement, i64::from(binder_depth), 0)
            } else {
                Some(Term::Var { index: *index })
            }
        }
        Term::Global { id } => Some(Term::Global { id: id.clone() }),
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(substitute_term_v1(
                parameter,
                target,
                replacement,
                binder_depth,
            )?),
            body: Box::new(substitute_term_v1(
                body,
                target,
                replacement,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(substitute_term_v1(
                parameter,
                target,
                replacement,
                binder_depth,
            )?),
            body: Box::new(substitute_term_v1(
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
            parameter_type: Box::new(substitute_term_v1(
                parameter_type,
                target,
                replacement,
                binder_depth,
            )?),
            body: Box::new(substitute_term_v1(
                body,
                target,
                replacement,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(substitute_term_v1(
                function,
                target,
                replacement,
                binder_depth,
            )?),
            argument: Box::new(substitute_term_v1(
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
            sigma_type: Box::new(substitute_term_v1(
                sigma_type,
                target,
                replacement,
                binder_depth,
            )?),
            first: Box::new(substitute_term_v1(
                first,
                target,
                replacement,
                binder_depth,
            )?),
            second: Box::new(substitute_term_v1(
                second,
                target,
                replacement,
                binder_depth,
            )?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(substitute_term_v1(pair, target, replacement, binder_depth)?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(substitute_term_v1(pair, target, replacement, binder_depth)?),
        }),
        Term::UnitType => Some(Term::UnitType),
        Term::Unit => Some(Term::Unit),
    }
}

fn shift_term_v1(term: &Term, amount: i64, cutoff: u32) -> Option<Term> {
    match term {
        Term::Sort { level } => Some(Term::Sort { level: *level }),
        Term::Var { index } => {
            if *index < cutoff {
                return Some(Term::Var { index: *index });
            }
            let shifted = i64::from(*index).checked_add(amount)?;
            Some(Term::Var {
                index: u32::try_from(shifted).ok()?,
            })
        }
        Term::Global { id } => Some(Term::Global { id: id.clone() }),
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(shift_term_v1(parameter, amount, cutoff)?),
            body: Box::new(shift_term_v1(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(shift_term_v1(parameter, amount, cutoff)?),
            body: Box::new(shift_term_v1(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(shift_term_v1(parameter_type, amount, cutoff)?),
            body: Box::new(shift_term_v1(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(shift_term_v1(function, amount, cutoff)?),
            argument: Box::new(shift_term_v1(argument, amount, cutoff)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(shift_term_v1(sigma_type, amount, cutoff)?),
            first: Box::new(shift_term_v1(first, amount, cutoff)?),
            second: Box::new(shift_term_v1(second, amount, cutoff)?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(shift_term_v1(pair, amount, cutoff)?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(shift_term_v1(pair, amount, cutoff)?),
        }),
        Term::UnitType => Some(Term::UnitType),
        Term::Unit => Some(Term::Unit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v2,
        verify_semantic_audit_lambda_unit_manifest_v2,
    };
    use pen_kernel::{Declaration, GlobalId, KernelLimits, UncheckedSignature};

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("safe test kernel")
    }

    fn manifest() -> VerifiedSemanticAuditManifestV2 {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v2(
            &proposed_semantic_audit_lambda_unit_manifest_v2(),
        ) else {
            panic!("exact V2 lambda/unit manifest should verify");
        };
        manifest
    }

    fn empty_signature(kernel: &Kernel) -> VerifiedSignature {
        kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature")
    }

    #[test]
    fn lambda_body_uses_checked_binder_local_context() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let request = TypedOccurrenceRootRequestV1::HasType {
            context: DependentContext::default(),
            term: Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Var { index: 0 }),
            },
            ty: Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            },
        };

        let AuditDecision::Proven(census) =
            verify_typed_occurrence_census_v2(&manifest(), &kernel, &signature, &[request])
        else {
            panic!("bidirectionally typed lambda should have a complete census");
        };
        assert_eq!(census.roots().len(), 1);
        assert_eq!(census.occurrences().len(), 6);

        let body = census
            .occurrences()
            .iter()
            .find(|occurrence| {
                occurrence.path().components()
                    == [
                        TypedOccurrencePathComponentV1::JudgmentTerm,
                        TypedOccurrencePathComponentV1::BinderBody {
                            binder: TypedBinderKindV1::Lambda,
                        },
                    ]
            })
            .expect("lambda body occurrence");
        assert_eq!(
            body.local_context(),
            &DependentContext(vec![Term::UnitType])
        );
        assert_eq!(body.term(), &Term::Var { index: 0 });
        assert_eq!(
            body.judgment(),
            &LocalJudgmentV1::HasType { ty: Term::UnitType }
        );
    }

    #[test]
    fn application_replays_global_function_and_argument_types() {
        let kernel = kernel();
        let function = GlobalId(Digest::of_bytes(b"typed-occurrence-test-function"));
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: function.clone(),
                    ty: Term::Pi {
                        parameter: Box::new(Term::UnitType),
                        body: Box::new(Term::UnitType),
                    },
                    body: None,
                }],
            })
            .expect("opaque unit endomorphism");
        let request = TypedOccurrenceRootRequestV1::HasType {
            context: DependentContext::default(),
            term: Term::Apply {
                function: Box::new(Term::Global { id: function }),
                argument: Box::new(Term::Unit),
            },
            ty: Term::UnitType,
        };

        let AuditDecision::Proven(census) =
            verify_typed_occurrence_census_v2(&manifest(), &kernel, &signature, &[request])
        else {
            panic!("global-headed application should have a complete census");
        };
        assert_eq!(census.occurrences().len(), 4);
        assert!(census.occurrences().iter().any(|occurrence| {
            occurrence.path().components()
                == [
                    TypedOccurrencePathComponentV1::JudgmentTerm,
                    TypedOccurrencePathComponentV1::Function,
                ]
        }));
        assert!(census.occurrences().iter().any(|occurrence| {
            occurrence.path().components()
                == [
                    TypedOccurrencePathComponentV1::JudgmentTerm,
                    TypedOccurrencePathComponentV1::Argument,
                ]
        }));
    }

    #[test]
    fn equation_sides_and_judgment_type_have_distinct_paths() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let request = TypedOccurrenceRootRequestV1::Equation {
            context: DependentContext::default(),
            left: Term::Unit,
            right: Term::Unit,
            ty: Term::UnitType,
        };

        let AuditDecision::Proven(census) =
            verify_typed_occurrence_census_v2(&manifest(), &kernel, &signature, &[request])
        else {
            panic!("typed equation root should have a complete census");
        };
        let root_components = census
            .occurrences()
            .iter()
            .map(|occurrence| occurrence.path().components()[0].clone())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            root_components,
            BTreeSet::from([
                TypedOccurrencePathComponentV1::EquationLeft,
                TypedOccurrencePathComponentV1::EquationRight,
                TypedOccurrencePathComponentV1::JudgmentType,
            ])
        );
    }

    #[test]
    fn variable_function_position_fails_closed_without_kernel_inference_api() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
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

        assert_eq!(
            verify_typed_occurrence_census_v2(&manifest(), &kernel, &signature, &[request]),
            AuditDecision::Unknown(AuditUnknownReason::MissingTypedOccurrenceCensus)
        );
    }

    #[test]
    fn unsupported_root_syntax_is_outside_fragment() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let request = TypedOccurrenceRootRequestV1::TypeFormation {
            context: DependentContext::default(),
            term: Term::Sigma {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            },
        };

        assert_eq!(
            verify_typed_occurrence_census_v2(&manifest(), &kernel, &signature, &[request]),
            AuditDecision::OutsideFragment(OutsideFragmentReason::UnsupportedTerm)
        );
    }
}
