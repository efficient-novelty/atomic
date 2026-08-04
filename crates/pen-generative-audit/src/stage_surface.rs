//! JG2a authority for one exact, public, kernel-verified stage boundary.
//!
//! This module binds an already verified JG1 grammar and constructor grammar
//! to one predecessor/candidate/successor boundary and one replayed free-
//! sealing certificate.  It deliberately does not classify capability
//! constructors or prove history, naturality, support, closure, marginality,
//! provenance, generative gain, or selection facts.

use crate::{
    VerifiedGenerativeCapabilityConstructorGrammarV1,
    VerifiedPreExposureGenerativeCapabilityGrammarV1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, CertificateClaim, Declaration, Digest, Kernel, KernelError,
    TrustedScope, UncheckedSignature, VerifiedFreeSealing, VerifiedSignature,
};

pub const GENERATIVE_CAPABILITY_STAGE_SURFACE_SCHEMA_VERSION_V1: u16 = 1;
pub const GENERATIVE_CAPABILITY_CANDIDATE_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2a/exact-public-stage-candidate/v1";

const GENERATIVE_CAPABILITY_STAGE_SURFACE_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2a/exact-public-stage-surface/v1";
const NORMALIZED_PREDECESSOR_DECLARATIONS_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2a/normalized-predecessor-declarations/v1";
const NORMALIZED_CANDIDATE_DECLARATIONS_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2a/normalized-candidate-declarations/v1";
const NORMALIZED_SUCCESSOR_DECLARATIONS_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2a/normalized-successor-declarations/v1";
const KERNEL_CONFIGURATION_DIGEST_DOMAIN_V1: &str = "law-v2/jg2a/kernel-configuration/v1";
const GENERATIVE_CAPABILITY_STAGE_SURFACE_ROOT_TAG_V1: u8 = 0xa3;

/// Canonical identity of the exact candidate syntax presented at a stage.
///
/// This digest is intentionally computed before normalization. Two candidates
/// that normalize to the same declarations remain different exact proposals.
pub fn generative_capability_candidate_digest_v1(candidate: &UncheckedSignature) -> Digest {
    Digest::of_canonical(GENERATIVE_CAPABILITY_CANDIDATE_DIGEST_DOMAIN_V1, candidate)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GenerativeCapabilityStageSurfaceBindingV1 {
    schema_version: u16,
    jg1_manifest_digest: Digest,
    constructor_grammar_manifest_digest: Digest,
    scope_grammar_digest: Digest,
    scope_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    kernel_configuration_digest: Digest,
    predecessor_digest: Digest,
    candidate_digest: Digest,
    sealing_subject_digest: Digest,
    successor_digest: Digest,
    predecessor_declaration_count: u64,
    candidate_declaration_count: u64,
    successor_declaration_count: u64,
    normalized_predecessor_declarations_digest: Digest,
    normalized_candidate_declarations_digest: Digest,
    normalized_successor_declarations_digest: Digest,
}

impl CanonicalEncode for GenerativeCapabilityStageSurfaceBindingV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_CAPABILITY_STAGE_SURFACE_ROOT_TAG_V1);
        encoder.u16(self.schema_version);
        self.jg1_manifest_digest.encode_canonical(encoder);
        self.constructor_grammar_manifest_digest
            .encode_canonical(encoder);
        self.scope_grammar_digest.encode_canonical(encoder);
        self.scope_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.kernel_configuration_digest.encode_canonical(encoder);
        self.predecessor_digest.encode_canonical(encoder);
        self.candidate_digest.encode_canonical(encoder);
        self.sealing_subject_digest.encode_canonical(encoder);
        self.successor_digest.encode_canonical(encoder);
        encoder.u64(self.predecessor_declaration_count);
        encoder.u64(self.candidate_declaration_count);
        encoder.u64(self.successor_declaration_count);
        self.normalized_predecessor_declarations_digest
            .encode_canonical(encoder);
        self.normalized_candidate_declarations_digest
            .encode_canonical(encoder);
        self.normalized_successor_declarations_digest
            .encode_canonical(encoder);
    }
}

/// Fail-closed reasons why an exact JG2a public stage surface was not minted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenerativeCapabilityStageSurfaceFailureV1 {
    EmptyCandidate,
    ConstructorGrammarJg1Mismatch,
    CertificateClaimMismatch,
    KernelProtocolMismatch,
    NormalizerProtocolMismatch,
    ScopeGrammarMismatch,
    PublicBoundaryMismatch,
    CandidateDigestMismatch,
    CandidateKernelFailure(KernelError),
    SealingScopeMismatch,
    SealingSubjectMismatch,
    SealedSuccessorMismatch,
    DeclarationCountOverflow,
}

impl std::fmt::Display for GenerativeCapabilityStageSurfaceFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCandidate => formatter.write_str("the exact stage candidate is empty"),
            Self::ConstructorGrammarJg1Mismatch => formatter.write_str(
                "the verified constructor grammar is not bound to the supplied JG1 grammar",
            ),
            Self::CertificateClaimMismatch => formatter
                .write_str("the trusted scope does not claim an exact free-sealing fragment"),
            Self::KernelProtocolMismatch => formatter
                .write_str("the trusted scope is not bound to the supplied kernel protocol"),
            Self::NormalizerProtocolMismatch => formatter
                .write_str("the trusted scope is not bound to the supplied normalizer protocol"),
            Self::ScopeGrammarMismatch => formatter.write_str(
                "the trusted scope grammar is not the verified combined JG1/constructor grammar",
            ),
            Self::PublicBoundaryMismatch => formatter
                .write_str("the trusted public boundary is not the supplied verified predecessor"),
            Self::CandidateDigestMismatch => formatter
                .write_str("the trusted candidate identity is not the exact canonical candidate"),
            Self::CandidateKernelFailure(error) => {
                write!(
                    formatter,
                    "the exact candidate does not extend the predecessor: {error}"
                )
            }
            Self::SealingScopeMismatch => formatter.write_str(
                "the verified free-sealing capability belongs to a different trusted scope",
            ),
            Self::SealingSubjectMismatch => formatter.write_str(
                "the verified free-sealing capability has a different exact extension subject",
            ),
            Self::SealedSuccessorMismatch => formatter.write_str(
                "the independently replayed successor differs from the verified sealed successor",
            ),
            Self::DeclarationCountOverflow => formatter.write_str(
                "a public declaration count cannot be represented by the JG2a wire format",
            ),
        }
    }
}

impl std::error::Error for GenerativeCapabilityStageSurfaceFailureV1 {}

impl From<KernelError> for GenerativeCapabilityStageSurfaceFailureV1 {
    fn from(error: KernelError) -> Self {
        Self::CandidateKernelFailure(error)
    }
}

/// Opaque proof of one exact, typed public stage boundary.
///
/// The proof says only that the supplied candidate is nonempty, the complete
/// public scope bindings agree, and the kernel independently reconstructs the
/// same normalized successor as an existing verified free-sealing capability.
#[derive(Clone, Debug)]
pub struct VerifiedGenerativeCapabilityStageSurfaceV1 {
    binding: GenerativeCapabilityStageSurfaceBindingV1,
    manifest_digest: Digest,
    predecessor: VerifiedSignature,
    exact_candidate: UncheckedSignature,
    normalized_candidate: UncheckedSignature,
    successor: VerifiedSignature,
}

impl PartialEq for VerifiedGenerativeCapabilityStageSurfaceV1 {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding
            && self.manifest_digest == other.manifest_digest
            && self.predecessor.normalized_wire() == other.predecessor.normalized_wire()
            && self.exact_candidate == other.exact_candidate
            && self.normalized_candidate == other.normalized_candidate
            && self.successor.normalized_wire() == other.successor.normalized_wire()
    }
}

impl Eq for VerifiedGenerativeCapabilityStageSurfaceV1 {}

impl VerifiedGenerativeCapabilityStageSurfaceV1 {
    pub const fn schema_version(&self) -> u16 {
        self.binding.schema_version
    }

    pub fn jg1_manifest_digest(&self) -> &Digest {
        &self.binding.jg1_manifest_digest
    }

    pub fn constructor_grammar_manifest_digest(&self) -> &Digest {
        &self.binding.constructor_grammar_manifest_digest
    }

    pub fn scope_grammar_digest(&self) -> &Digest {
        &self.binding.scope_grammar_digest
    }

    pub fn scope_digest(&self) -> &Digest {
        &self.binding.scope_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.binding.kernel_protocol_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.binding.normalizer_protocol_digest
    }

    pub fn kernel_configuration_digest(&self) -> &Digest {
        &self.binding.kernel_configuration_digest
    }

    pub fn predecessor_digest(&self) -> &Digest {
        &self.binding.predecessor_digest
    }

    pub fn candidate_digest(&self) -> &Digest {
        &self.binding.candidate_digest
    }

    pub fn sealing_subject_digest(&self) -> &Digest {
        &self.binding.sealing_subject_digest
    }

    pub fn successor_digest(&self) -> &Digest {
        &self.binding.successor_digest
    }

    pub const fn predecessor_declaration_count(&self) -> u64 {
        self.binding.predecessor_declaration_count
    }

    pub const fn candidate_declaration_count(&self) -> u64 {
        self.binding.candidate_declaration_count
    }

    pub const fn successor_declaration_count(&self) -> u64 {
        self.binding.successor_declaration_count
    }

    pub fn normalized_predecessor_declarations_digest(&self) -> &Digest {
        &self.binding.normalized_predecessor_declarations_digest
    }

    pub fn normalized_candidate_declarations_digest(&self) -> &Digest {
        &self.binding.normalized_candidate_declarations_digest
    }

    pub fn normalized_successor_declarations_digest(&self) -> &Digest {
        &self.binding.normalized_successor_declarations_digest
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    /// Verified normalized public boundary before the exact candidate.
    pub fn predecessor_boundary(&self) -> &VerifiedSignature {
        &self.predecessor
    }

    /// Exact checked candidate syntax presented to the stage verifier.
    /// Its canonical identity is already committed by [`Self::candidate_digest`].
    pub fn exact_candidate(&self) -> &UncheckedSignature {
        &self.exact_candidate
    }

    /// Normalized candidate declarations extracted from the replayed
    /// successor suffix, never copied from the caller's unchecked wire.
    pub fn normalized_candidate(&self) -> &UncheckedSignature {
        &self.normalized_candidate
    }

    /// Independently replayed and verified normalized public boundary after
    /// the exact candidate.
    pub fn successor_boundary(&self) -> &VerifiedSignature {
        &self.successor
    }
}

/// Verify and bind one exact public predecessor/candidate/successor surface.
///
/// `sealing` must already have been replayed by `pen-kernel`; this verifier
/// nevertheless replays `candidate` independently and compares the complete
/// normalized successor, rather than trusting only matching digests.
pub fn verify_generative_capability_stage_surface_v1(
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructor_grammar: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    scope: &TrustedScope,
    predecessor: &VerifiedSignature,
    candidate: &UncheckedSignature,
    sealing: &VerifiedFreeSealing,
) -> Result<VerifiedGenerativeCapabilityStageSurfaceV1, GenerativeCapabilityStageSurfaceFailureV1> {
    if candidate.declarations.is_empty() {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::EmptyCandidate);
    }
    if constructor_grammar.jg1_manifest_digest() != jg1.manifest_digest() {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::ConstructorGrammarJg1Mismatch);
    }

    let scope_binding = scope.binding();
    if scope_binding.claim != CertificateClaim::FreeSealingFragment {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::CertificateClaimMismatch);
    }

    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    if scope_binding.kernel_digest != kernel_protocol_digest {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::KernelProtocolMismatch);
    }
    let normalizer_protocol_digest = kernel.normalizer_protocol_digest();
    if scope_binding.normalizer_digest != normalizer_protocol_digest {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::NormalizerProtocolMismatch);
    }
    if scope_binding.grammar_digest != *constructor_grammar.scope_grammar_digest() {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::ScopeGrammarMismatch);
    }
    if scope_binding.public_boundary_digest != *predecessor.digest() {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::PublicBoundaryMismatch);
    }

    let candidate_digest = generative_capability_candidate_digest_v1(candidate);
    if scope_binding.candidate_digest != candidate_digest {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::CandidateDigestMismatch);
    }

    // Replay before comparing the supplied sealing token. This ensures a
    // malformed candidate fails for its own kernel reason and cannot be
    // obscured by an unrelated sealing token.
    let replayed_successor = kernel.verify_extension(predecessor, candidate)?;
    let sealing_subject_digest = kernel.sealing_subject_digest(predecessor, candidate)?;

    if sealing.scope_digest() != scope.digest() {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::SealingScopeMismatch);
    }
    if sealing.subject_digest() != &sealing_subject_digest {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::SealingSubjectMismatch);
    }
    if sealing.sealed_signature().digest() != replayed_successor.digest()
        || sealing.sealed_signature().declarations() != replayed_successor.declarations()
    {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::SealedSuccessorMismatch);
    }

    let predecessor_count = predecessor.declarations().len();
    let candidate_count = candidate.declarations.len();
    let successor_count = replayed_successor.declarations().len();
    let normalized_candidate_declarations = replayed_successor
        .declarations()
        .get(predecessor_count..)
        .ok_or(GenerativeCapabilityStageSurfaceFailureV1::SealedSuccessorMismatch)?;

    if normalized_candidate_declarations.len() != candidate_count
        || successor_count != predecessor_count.saturating_add(candidate_count)
    {
        return Err(GenerativeCapabilityStageSurfaceFailureV1::SealedSuccessorMismatch);
    }

    let binding = GenerativeCapabilityStageSurfaceBindingV1 {
        schema_version: GENERATIVE_CAPABILITY_STAGE_SURFACE_SCHEMA_VERSION_V1,
        jg1_manifest_digest: jg1.manifest_digest().clone(),
        constructor_grammar_manifest_digest: constructor_grammar.manifest_digest().clone(),
        scope_grammar_digest: constructor_grammar.scope_grammar_digest().clone(),
        scope_digest: scope.digest().clone(),
        kernel_protocol_digest,
        normalizer_protocol_digest,
        kernel_configuration_digest: generative_capability_kernel_configuration_digest_v1(kernel),
        predecessor_digest: predecessor.digest().clone(),
        candidate_digest,
        sealing_subject_digest,
        successor_digest: replayed_successor.digest().clone(),
        predecessor_declaration_count: u64::try_from(predecessor_count)
            .map_err(|_| GenerativeCapabilityStageSurfaceFailureV1::DeclarationCountOverflow)?,
        candidate_declaration_count: u64::try_from(candidate_count)
            .map_err(|_| GenerativeCapabilityStageSurfaceFailureV1::DeclarationCountOverflow)?,
        successor_declaration_count: u64::try_from(successor_count)
            .map_err(|_| GenerativeCapabilityStageSurfaceFailureV1::DeclarationCountOverflow)?,
        normalized_predecessor_declarations_digest: normalized_declarations_digest_v1(
            NORMALIZED_PREDECESSOR_DECLARATIONS_DIGEST_DOMAIN_V1,
            predecessor.declarations(),
        ),
        normalized_candidate_declarations_digest: normalized_declarations_digest_v1(
            NORMALIZED_CANDIDATE_DECLARATIONS_DIGEST_DOMAIN_V1,
            normalized_candidate_declarations,
        ),
        normalized_successor_declarations_digest: normalized_declarations_digest_v1(
            NORMALIZED_SUCCESSOR_DECLARATIONS_DIGEST_DOMAIN_V1,
            replayed_successor.declarations(),
        ),
    };
    let manifest_digest = Digest::of_canonical(
        GENERATIVE_CAPABILITY_STAGE_SURFACE_DIGEST_DOMAIN_V1,
        &binding,
    );
    let normalized_candidate = UncheckedSignature {
        declarations: normalized_candidate_declarations.to_vec(),
    };

    Ok(VerifiedGenerativeCapabilityStageSurfaceV1 {
        binding,
        manifest_digest,
        predecessor: predecessor.clone(),
        exact_candidate: candidate.clone(),
        normalized_candidate,
        successor: replayed_successor,
    })
}

fn normalized_declarations_digest_v1(domain: &str, declarations: &[Declaration]) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.sequence(declarations);
    Digest::of_domain_bytes(domain, encoder.as_bytes())
}

pub(crate) fn generative_capability_kernel_configuration_digest_v1(kernel: &Kernel) -> Digest {
    let limits = kernel.limits();
    let mut encoder = CanonicalEncoder::new();
    encoder.u32(limits.max_operations);
    encoder.u16(limits.max_depth);
    encoder.u32(limits.normalization_fuel);
    Digest::of_domain_bytes(KERNEL_CONFIGURATION_DIGEST_DOMAIN_V1, encoder.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        proposed_generative_capability_constructor_grammar_v1,
        proposed_pre_exposure_generative_capability_grammar_v1,
        verify_generative_capability_constructor_grammar_v1,
        verify_pre_exposure_generative_capability_grammar_v1,
    };
    use pen_kernel::{
        Declaration, Digest, GlobalId, KernelLimits, ScopeInputs, Term,
        UncheckedFreeSealingCertificate,
    };

    fn digest(label: &[u8]) -> Digest {
        Digest::of_bytes(label)
    }

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    fn grammars() -> (
        VerifiedPreExposureGenerativeCapabilityGrammarV1,
        VerifiedGenerativeCapabilityConstructorGrammarV1,
    ) {
        let jg1 = verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("exact JG1 grammar");
        let constructors = verify_generative_capability_constructor_grammar_v1(
            &jg1,
            &proposed_generative_capability_constructor_grammar_v1(&jg1),
        )
        .expect("exact JG2a constructor grammar");
        (jg1, constructors)
    }

    fn empty_predecessor(kernel: &Kernel) -> VerifiedSignature {
        kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty predecessor")
    }

    fn candidate(label: &[u8]) -> UncheckedSignature {
        UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(label)),
                ty: Term::UnitType,
                body: None,
            }],
        }
    }

    fn scope(
        kernel: &Kernel,
        claim: CertificateClaim,
        grammar_digest: Digest,
        boundary_digest: Digest,
        candidate_digest: Digest,
        history_label: &[u8],
    ) -> TrustedScope {
        TrustedScope::new(
            kernel,
            claim,
            ScopeInputs {
                law_digest: digest(b"law"),
                grammar_digest,
                scheme_calculus_digest: digest(b"scheme-calculus"),
                blindness_contract_digest: digest(b"blindness"),
                bootstrap_contract_digest: digest(b"bootstrap"),
                history_digest: digest(history_label),
                public_boundary_digest: boundary_digest,
                derivation_basis_digest: digest(b"derivation-basis"),
                active_window_digest: digest(b"active-window"),
                candidate_digest,
            },
        )
    }

    fn exact_scope(
        kernel: &Kernel,
        grammar: &VerifiedGenerativeCapabilityConstructorGrammarV1,
        predecessor: &VerifiedSignature,
        candidate: &UncheckedSignature,
        history_label: &[u8],
    ) -> TrustedScope {
        scope(
            kernel,
            CertificateClaim::FreeSealingFragment,
            grammar.scope_grammar_digest().clone(),
            predecessor.digest().clone(),
            generative_capability_candidate_digest_v1(candidate),
            history_label,
        )
    }

    fn verified_sealing(
        kernel: &Kernel,
        scope: &TrustedScope,
        predecessor: &VerifiedSignature,
        candidate: &UncheckedSignature,
    ) -> VerifiedFreeSealing {
        let certificate = UncheckedFreeSealingCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .sealing_subject_digest(predecessor, candidate)
                .expect("sealing subject"),
            extension: candidate.clone(),
            normalized_sealed_signature: kernel
                .verify_extension(predecessor, candidate)
                .expect("valid candidate")
                .normalized_wire(),
        };
        kernel
            .verify_free_sealing_certificate(scope, predecessor, candidate, &certificate)
            .expect("verified exact sealing")
    }

    #[test]
    fn exact_nonempty_stage_surface_binds_every_public_boundary() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let predecessor = empty_predecessor(&kernel);
        let candidate = candidate(b"candidate");
        let scope = exact_scope(&kernel, &constructors, &predecessor, &candidate, b"history");
        let sealing = verified_sealing(&kernel, &scope, &predecessor, &candidate);

        let surface = verify_generative_capability_stage_surface_v1(
            &jg1,
            &constructors,
            &kernel,
            &scope,
            &predecessor,
            &candidate,
            &sealing,
        )
        .expect("exact stage surface");

        assert_eq!(surface.schema_version(), 1);
        assert_eq!(surface.jg1_manifest_digest(), jg1.manifest_digest());
        assert_eq!(
            surface.constructor_grammar_manifest_digest(),
            constructors.manifest_digest()
        );
        assert_eq!(surface.scope_digest(), scope.digest());
        assert_eq!(surface.predecessor_digest(), predecessor.digest());
        assert_eq!(
            surface.candidate_digest(),
            &generative_capability_candidate_digest_v1(&candidate)
        );
        assert_eq!(
            surface.successor_digest(),
            sealing.sealed_signature().digest()
        );
        assert_eq!(surface.predecessor_declaration_count(), 0);
        assert_eq!(surface.candidate_declaration_count(), 1);
        assert_eq!(surface.successor_declaration_count(), 1);
        assert_eq!(
            surface.predecessor_boundary().normalized_wire(),
            predecessor.normalized_wire()
        );
        assert_eq!(surface.exact_candidate(), &candidate);
        assert_eq!(surface.normalized_candidate(), &candidate);
        assert_eq!(
            surface.successor_boundary().normalized_wire(),
            sealing.sealed_signature().normalized_wire()
        );
        assert_eq!(
            surface.normalized_predecessor_declarations_digest(),
            &normalized_declarations_digest_v1(
                NORMALIZED_PREDECESSOR_DECLARATIONS_DIGEST_DOMAIN_V1,
                predecessor.declarations(),
            )
        );
        assert_eq!(
            surface.normalized_candidate_declarations_digest(),
            &normalized_declarations_digest_v1(
                NORMALIZED_CANDIDATE_DECLARATIONS_DIGEST_DOMAIN_V1,
                sealing.sealed_signature().declarations(),
            )
        );
        assert_eq!(
            surface.normalized_successor_declarations_digest(),
            &normalized_declarations_digest_v1(
                NORMALIZED_SUCCESSOR_DECLARATIONS_DIGEST_DOMAIN_V1,
                sealing.sealed_signature().declarations(),
            )
        );

        let repeated = verify_generative_capability_stage_surface_v1(
            &jg1,
            &constructors,
            &kernel,
            &scope,
            &predecessor,
            &candidate,
            &sealing,
        )
        .expect("deterministic replay");
        assert_eq!(surface, repeated);
        assert_eq!(surface.manifest_digest(), repeated.manifest_digest());
    }

    #[test]
    fn normalized_candidate_is_the_exact_successor_suffix() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let predecessor_wire = candidate(b"predecessor");
        let predecessor = kernel
            .verify_signature(&predecessor_wire)
            .expect("typed nonempty predecessor");
        let candidate = candidate(b"extension");
        let scope = exact_scope(&kernel, &constructors, &predecessor, &candidate, b"history");
        let sealing = verified_sealing(&kernel, &scope, &predecessor, &candidate);

        let surface = verify_generative_capability_stage_surface_v1(
            &jg1,
            &constructors,
            &kernel,
            &scope,
            &predecessor,
            &candidate,
            &sealing,
        )
        .expect("exact nonempty predecessor transition");

        assert_eq!(surface.predecessor_declaration_count(), 1);
        assert_eq!(surface.candidate_declaration_count(), 1);
        assert_eq!(surface.successor_declaration_count(), 2);
        assert_eq!(surface.normalized_candidate(), &candidate);
        assert_eq!(
            surface.successor_boundary().declarations()[..1],
            surface.predecessor_boundary().declarations()[..]
        );
        assert_eq!(
            surface.successor_boundary().declarations()[1..],
            surface.normalized_candidate().declarations[..]
        );
    }

    #[test]
    fn empty_mistyped_and_duplicate_candidates_fail_closed() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let predecessor = empty_predecessor(&kernel);
        let good_candidate = candidate(b"good");
        let good_scope = exact_scope(
            &kernel,
            &constructors,
            &predecessor,
            &good_candidate,
            b"history",
        );
        let good_sealing = verified_sealing(&kernel, &good_scope, &predecessor, &good_candidate);

        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &good_scope,
                &predecessor,
                &UncheckedSignature::default(),
                &good_sealing,
            ),
            Err(GenerativeCapabilityStageSurfaceFailureV1::EmptyCandidate)
        );

        let mistyped = UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(b"mistyped")),
                ty: Term::UnitType,
                body: Some(Term::UnitType),
            }],
        };
        let mistyped_scope =
            exact_scope(&kernel, &constructors, &predecessor, &mistyped, b"history");
        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &mistyped_scope,
                &predecessor,
                &mistyped,
                &good_sealing,
            ),
            Err(
                GenerativeCapabilityStageSurfaceFailureV1::CandidateKernelFailure(
                    KernelError::TypeMismatch
                )
            )
        );

        let duplicate_declaration = Declaration {
            id: GlobalId(digest(b"duplicate")),
            ty: Term::UnitType,
            body: None,
        };
        let duplicate = UncheckedSignature {
            declarations: vec![duplicate_declaration.clone(), duplicate_declaration],
        };
        let duplicate_scope =
            exact_scope(&kernel, &constructors, &predecessor, &duplicate, b"history");
        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &duplicate_scope,
                &predecessor,
                &duplicate,
                &good_sealing,
            ),
            Err(
                GenerativeCapabilityStageSurfaceFailureV1::CandidateKernelFailure(
                    KernelError::DuplicateGlobal
                )
            )
        );
    }

    #[test]
    fn claim_grammar_boundary_and_candidate_scope_mutations_fail_closed() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let predecessor = empty_predecessor(&kernel);
        let candidate = candidate(b"candidate");
        let exact_scope = exact_scope(&kernel, &constructors, &predecessor, &candidate, b"history");
        let exact_sealing = verified_sealing(&kernel, &exact_scope, &predecessor, &candidate);

        let wrong_claim = scope(
            &kernel,
            CertificateClaim::Derivation,
            constructors.scope_grammar_digest().clone(),
            predecessor.digest().clone(),
            generative_capability_candidate_digest_v1(&candidate),
            b"history",
        );
        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &wrong_claim,
                &predecessor,
                &candidate,
                &exact_sealing,
            ),
            Err(GenerativeCapabilityStageSurfaceFailureV1::CertificateClaimMismatch)
        );

        let wrong_grammar = scope(
            &kernel,
            CertificateClaim::FreeSealingFragment,
            digest(b"wrong-grammar"),
            predecessor.digest().clone(),
            generative_capability_candidate_digest_v1(&candidate),
            b"history",
        );
        let wrong_grammar_sealing =
            verified_sealing(&kernel, &wrong_grammar, &predecessor, &candidate);
        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &wrong_grammar,
                &predecessor,
                &candidate,
                &wrong_grammar_sealing,
            ),
            Err(GenerativeCapabilityStageSurfaceFailureV1::ScopeGrammarMismatch)
        );

        let wrong_boundary = scope(
            &kernel,
            CertificateClaim::FreeSealingFragment,
            constructors.scope_grammar_digest().clone(),
            digest(b"wrong-boundary"),
            generative_capability_candidate_digest_v1(&candidate),
            b"history",
        );
        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &wrong_boundary,
                &predecessor,
                &candidate,
                &exact_sealing,
            ),
            Err(GenerativeCapabilityStageSurfaceFailureV1::PublicBoundaryMismatch)
        );

        let wrong_candidate = scope(
            &kernel,
            CertificateClaim::FreeSealingFragment,
            constructors.scope_grammar_digest().clone(),
            predecessor.digest().clone(),
            digest(b"wrong-candidate"),
            b"history",
        );
        let wrong_candidate_sealing =
            verified_sealing(&kernel, &wrong_candidate, &predecessor, &candidate);
        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &wrong_candidate,
                &predecessor,
                &candidate,
                &wrong_candidate_sealing,
            ),
            Err(GenerativeCapabilityStageSurfaceFailureV1::CandidateDigestMismatch)
        );
    }

    #[test]
    fn sealing_scope_and_subject_mutations_fail_closed() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let predecessor = empty_predecessor(&kernel);
        let candidate_a = candidate(b"candidate-a");
        let candidate_b = candidate(b"candidate-b");
        let scope_a = exact_scope(
            &kernel,
            &constructors,
            &predecessor,
            &candidate_a,
            b"history-a",
        );
        let scope_b = exact_scope(
            &kernel,
            &constructors,
            &predecessor,
            &candidate_a,
            b"history-b",
        );
        let sealing_b = verified_sealing(&kernel, &scope_b, &predecessor, &candidate_a);
        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &scope_a,
                &predecessor,
                &candidate_a,
                &sealing_b,
            ),
            Err(GenerativeCapabilityStageSurfaceFailureV1::SealingScopeMismatch)
        );

        // pen-kernel deliberately treats candidate_digest as external scope
        // data. Minting a seal for B under A's otherwise valid scope exercises
        // JG2a's independent exact-subject comparison.
        let sealing_for_b_under_a = verified_sealing(&kernel, &scope_a, &predecessor, &candidate_b);
        assert_eq!(
            verify_generative_capability_stage_surface_v1(
                &jg1,
                &constructors,
                &kernel,
                &scope_a,
                &predecessor,
                &candidate_a,
                &sealing_for_b_under_a,
            ),
            Err(GenerativeCapabilityStageSurfaceFailureV1::SealingSubjectMismatch)
        );
    }

    #[test]
    fn candidate_identity_is_exact_and_domain_separated() {
        let syntactic_redex = UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(b"candidate")),
                ty: Term::UnitType,
                body: Some(Term::Apply {
                    function: Box::new(Term::Lambda {
                        parameter_type: Box::new(Term::UnitType),
                        body: Box::new(Term::Var { index: 0 }),
                    }),
                    argument: Box::new(Term::Unit),
                }),
            }],
        };
        let normal = UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(b"candidate")),
                ty: Term::UnitType,
                body: Some(Term::Unit),
            }],
        };

        assert_ne!(
            generative_capability_candidate_digest_v1(&syntactic_redex),
            generative_capability_candidate_digest_v1(&normal)
        );
        assert_ne!(
            generative_capability_candidate_digest_v1(&normal),
            Digest::of_canonical("another-domain", &normal)
        );

        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let predecessor = empty_predecessor(&kernel);
        let scope = exact_scope(
            &kernel,
            &constructors,
            &predecessor,
            &syntactic_redex,
            b"history",
        );
        let sealing = verified_sealing(&kernel, &scope, &predecessor, &syntactic_redex);
        let surface = verify_generative_capability_stage_surface_v1(
            &jg1,
            &constructors,
            &kernel,
            &scope,
            &predecessor,
            &syntactic_redex,
            &sealing,
        )
        .expect("redex candidate has an exact verified surface");

        assert_eq!(surface.exact_candidate(), &syntactic_redex);
        assert_ne!(surface.normalized_candidate(), &syntactic_redex);
        assert_eq!(surface.normalized_candidate(), &normal);
        assert_eq!(
            surface.successor_boundary().normalized_wire(),
            normal,
            "the retained successor and derived candidate suffix are normalized replay output"
        );
    }
}
