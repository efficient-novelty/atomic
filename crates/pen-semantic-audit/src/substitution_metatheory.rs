//! Generic simultaneous-substitution operations and the safe-Agda theorem
//! frontier for the lambda/unit successor.
//!
//! Arbitrary substitutions are values checked on demand; they are never
//! exhaustively enumerated.  The Agda package currently proves raw
//! capture-avoiding substitution algebra, beta/delta/fresh schema stability,
//! and the two family-constructor naturality laws.  It explicitly does not
//! claim the dependent typing substitution lemma. Consequently the full
//! [`VerifiedSubstitutionMetatheoryV1`] capability remains fail-closed.

use crate::agda_gate::{
    AgdaReferenceFailureV1, FixedAgdaSourceV1, VerifiedFixedAgdaPackageV1,
    diagnose_pinned_fixed_agda_package_v1,
};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2,
    VerifiedSemanticAuditManifestV2,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError, OpenJudgment,
    ResourceKind, Term, VerifiedSignature,
};

const AGDA_ENTRY_RELATIVE_PATH: &str = "LawV2/LambdaUnit/SubstitutionMetatheoryV1.agda";

const AGDA_SOURCES: &[FixedAgdaSourceV1] = &[
    FixedAgdaSourceV1 {
        relative_path: AGDA_ENTRY_RELATIVE_PATH,
        module_name: "LawV2.LambdaUnit.SubstitutionMetatheoryV1",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/SubstitutionMetatheoryV1.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/Substitution.agda",
        module_name: "LawV2.LambdaUnit.Substitution",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/Substitution.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/SubstitutionReduction.agda",
        module_name: "LawV2.LambdaUnit.SubstitutionReduction",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/SubstitutionReduction.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/FamilyNaturality.agda",
        module_name: "LawV2.LambdaUnit.FamilyNaturality",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/FamilyNaturality.agda"),
    },
    FixedAgdaSourceV1 {
        relative_path: "LawV2/LambdaUnit/SubstitutionTyping.agda",
        module_name: "LawV2.LambdaUnit.SubstitutionTyping",
        bytes: include_bytes!("../agda/LawV2/LambdaUnit/SubstitutionTyping.agda"),
    },
];

/// A kernel-checked simultaneous substitution from `source_context` to
/// `target_context`.
///
/// Images are stored in oldest-first source-declaration order. Fields are
/// private and this capability deliberately has no `Deserialize`
/// implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedSimultaneousSubstitutionV1 {
    source_context: DependentContext,
    target_context: DependentContext,
    images: Vec<Term>,
    digest: Digest,
}

impl TypedSimultaneousSubstitutionV1 {
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

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    /// Apply this substitution capture-avoidably to one source-scoped term.
    ///
    /// `None` indicates an out-of-scope variable or structural index overflow;
    /// it is never converted into theorem authority.
    pub fn apply(&self, term: &Term) -> Option<Term> {
        instantiate_term_images(term, self.source_context.0.len(), &self.images, 0)
    }
}

impl CanonicalEncode for TypedSimultaneousSubstitutionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source_context.encode_canonical(encoder);
        self.target_context.encode_canonical(encoder);
        encoder.sequence(&self.images);
    }
}

/// Opaque proof that the pinned safe-Agda package checked its exact raw
/// substitution-algebra and naturality theorem surface.
///
/// This is intentionally weaker than dependent typing preservation and cannot
/// authorize Q0 by itself.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedRawSubstitutionAlgebraV1 {
    agda_source_tree_digest: Digest,
    agda_checker_transcript_digest: Digest,
    pinned_agda_package_digest: Digest,
    digest: Digest,
}

impl VerifiedRawSubstitutionAlgebraV1 {
    pub fn agda_source_tree_digest(&self) -> &Digest {
        &self.agda_source_tree_digest
    }

    pub fn agda_checker_transcript_digest(&self) -> &Digest {
        &self.agda_checker_transcript_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedRawSubstitutionAlgebraV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.agda_source_tree_digest.encode_canonical(encoder);
        self.agda_checker_transcript_digest
            .encode_canonical(encoder);
        self.pinned_agda_package_digest.encode_canonical(encoder);
    }
}

/// Full authority requested by the lambda/unit V2 profile.
///
/// The constructor is private, the type is not deserializable, and the
/// verifier does not mint it while dependent typing preservation remains
/// unproved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSubstitutionMetatheoryV1 {
    semantic_manifest_digest: Digest,
    term_grammar_digest: Digest,
    context_grammar_digest: Digest,
    de_bruijn_protocol_digest: Digest,
    substitution_protocol_digest: Digest,
    q0_rule_inventory_digest: Digest,
    family_constructor_inventory_digest: Digest,
    agda_source_digest: Digest,
    agda_checker_transcript_digest: Digest,
    digest: Digest,
}

impl VerifiedSubstitutionMetatheoryV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn term_grammar_digest(&self) -> &Digest {
        &self.term_grammar_digest
    }

    pub fn context_grammar_digest(&self) -> &Digest {
        &self.context_grammar_digest
    }

    pub fn de_bruijn_protocol_digest(&self) -> &Digest {
        &self.de_bruijn_protocol_digest
    }

    pub fn substitution_protocol_digest(&self) -> &Digest {
        &self.substitution_protocol_digest
    }

    pub fn q0_rule_inventory_digest(&self) -> &Digest {
        &self.q0_rule_inventory_digest
    }

    pub fn family_constructor_inventory_digest(&self) -> &Digest {
        &self.family_constructor_inventory_digest
    }

    pub fn agda_source_digest(&self) -> &Digest {
        &self.agda_source_digest
    }

    pub fn agda_checker_transcript_digest(&self) -> &Digest {
        &self.agda_checker_transcript_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedSubstitutionMetatheoryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.term_grammar_digest.encode_canonical(encoder);
        self.context_grammar_digest.encode_canonical(encoder);
        self.de_bruijn_protocol_digest.encode_canonical(encoder);
        self.substitution_protocol_digest.encode_canonical(encoder);
        self.q0_rule_inventory_digest.encode_canonical(encoder);
        self.family_constructor_inventory_digest
            .encode_canonical(encoder);
        self.agda_source_digest.encode_canonical(encoder);
        self.agda_checker_transcript_digest
            .encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubstitutionMetatheoryFailureV1 {
    ManifestMismatch,
    AgdaGate(AgdaReferenceFailureV1),
    MissingDependentTypingPreservation,
    MissingTypedQ0ReductionStability,
}

impl std::fmt::Display for SubstitutionMetatheoryFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManifestMismatch => {
                formatter.write_str("semantic manifest is not the lambda/unit V2 successor")
            }
            Self::AgdaGate(failure) => write!(formatter, "pinned Agda gate failed: {failure}"),
            Self::MissingDependentTypingPreservation => formatter.write_str(
                "dependent typing preservation for arbitrary simultaneous substitution \
                 has not been mechanized",
            ),
            Self::MissingTypedQ0ReductionStability => formatter.write_str(
                "typed Q0 reduction stability cannot be derived before the dependent \
                 substitution lemma",
            ),
        }
    }
}

/// Check one arbitrary simultaneous substitution with the production kernel.
///
/// This verifies the source and target contexts and then checks image `i`
/// against the source entry type after substituting the already checked images
/// `0..i`. No finite global substitution inventory is consulted.
pub fn verify_typed_simultaneous_substitution_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    source_context: &DependentContext,
    target_context: &DependentContext,
    images: &[Term],
) -> AuditDecision<TypedSimultaneousSubstitutionV1> {
    match verify_typed_substitution(kernel, signature, source_context, target_context, images) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(reason) => AuditDecision::Unknown(reason),
    }
}

pub fn identity_typed_substitution_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
) -> AuditDecision<TypedSimultaneousSubstitutionV1> {
    let images = identity_images(context);
    let Some(images) = images else {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    };
    verify_typed_simultaneous_substitution_v1(kernel, signature, context, context, &images)
}

/// Compose `first : Γ ⇒ Δ` and `second : Δ ⇒ Θ` on demand.
pub fn compose_typed_substitutions_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    first: &TypedSimultaneousSubstitutionV1,
    second: &TypedSimultaneousSubstitutionV1,
) -> AuditDecision<TypedSimultaneousSubstitutionV1> {
    if first.target_context != second.source_context {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    }
    let images = first
        .images
        .iter()
        .map(|image| second.apply(image))
        .collect::<Option<Vec<_>>>();
    let Some(images) = images else {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    };
    verify_typed_simultaneous_substitution_v1(
        kernel,
        signature,
        &first.source_context,
        &second.target_context,
        &images,
    )
}

/// Lift `σ : Γ ⇒ Δ` under one dependent binder `A`.
pub fn lift_typed_substitution_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    substitution: &TypedSimultaneousSubstitutionV1,
    source_parameter_type: &Term,
) -> AuditDecision<TypedSimultaneousSubstitutionV1> {
    let normalized_parameter = match kernel.verify_open_judgment(
        signature,
        &OpenJudgment::TypeFormation {
            context: substitution.source_context.clone(),
            term: source_parameter_type.clone(),
        },
    ) {
        Ok(OpenJudgment::TypeFormation { term, .. }) => term,
        Ok(_) => return AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify),
        Err(error) => return AuditDecision::Unknown(kernel_unknown(error)),
    };
    let Some(target_parameter) = substitution.apply(&normalized_parameter) else {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    };
    let normalized_target_parameter = match kernel.verify_open_judgment(
        signature,
        &OpenJudgment::TypeFormation {
            context: substitution.target_context.clone(),
            term: target_parameter,
        },
    ) {
        Ok(OpenJudgment::TypeFormation { term, .. }) => term,
        Ok(_) => return AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify),
        Err(error) => return AuditDecision::Unknown(kernel_unknown(error)),
    };

    let mut source_entries = substitution.source_context.0.clone();
    source_entries.push(normalized_parameter);
    let mut target_entries = substitution.target_context.0.clone();
    target_entries.push(normalized_target_parameter);
    let images = substitution
        .images
        .iter()
        .map(|image| shift_term(image, 1, 0))
        .chain(std::iter::once(Some(Term::Var { index: 0 })))
        .collect::<Option<Vec<_>>>();
    let Some(images) = images else {
        return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
    };
    verify_typed_simultaneous_substitution_v1(
        kernel,
        signature,
        &DependentContext(source_entries),
        &DependentContext(target_entries),
        &images,
    )
}

/// Weakening is the on-demand substitution from `Γ` into `Γ,A`.
pub fn weakening_typed_substitution_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
    parameter_type: &Term,
) -> AuditDecision<TypedSimultaneousSubstitutionV1> {
    let identity = match identity_typed_substitution_v1(kernel, signature, context) {
        AuditDecision::Proven(identity) => identity,
        AuditDecision::OutsideFragment(reason) => {
            return AuditDecision::OutsideFragment(reason);
        }
        AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
    };
    let lifted = match lift_typed_substitution_v1(kernel, signature, &identity, parameter_type) {
        AuditDecision::Proven(lifted) => lifted,
        AuditDecision::OutsideFragment(reason) => {
            return AuditDecision::OutsideFragment(reason);
        }
        AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
    };
    verify_typed_simultaneous_substitution_v1(
        kernel,
        signature,
        context,
        &lifted.target_context,
        &lifted.images[..context.0.len()],
    )
}

/// Check the exact pinned raw theorem package.
pub fn verify_pinned_raw_substitution_algebra_v1() -> AuditDecision<VerifiedRawSubstitutionAlgebraV1>
{
    match diagnose_pinned_fixed_agda_package_v1(AGDA_SOURCES, AGDA_ENTRY_RELATIVE_PATH) {
        Ok(package) => AuditDecision::Proven(raw_algebra_from_package(package)),
        Err(_) => AuditDecision::Unknown(AuditUnknownReason::UnsupportedVerifier),
    }
}

/// Diagnose the full theorem frontier while retaining the precise local
/// reason. A successful raw-algebra check is still followed by the explicit
/// dependent-typing blocker.
pub fn diagnose_substitution_metatheory_v1(
    manifest: &VerifiedSemanticAuditManifestV2,
) -> Result<VerifiedSubstitutionMetatheoryV1, SubstitutionMetatheoryFailureV1> {
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2 {
        return Err(SubstitutionMetatheoryFailureV1::ManifestMismatch);
    }
    let _raw = diagnose_pinned_fixed_agda_package_v1(AGDA_SOURCES, AGDA_ENTRY_RELATIVE_PATH)
        .map(raw_algebra_from_package)
        .map_err(SubstitutionMetatheoryFailureV1::AgdaGate)?;

    // `SubstitutionTyping.agda` states and inhabits the exact frontier rather
    // than disguising scope preservation as dependent typing preservation.
    Err(SubstitutionMetatheoryFailureV1::MissingDependentTypingPreservation)
}

pub fn verify_substitution_metatheory_v1(
    manifest: &VerifiedSemanticAuditManifestV2,
) -> AuditDecision<VerifiedSubstitutionMetatheoryV1> {
    match diagnose_substitution_metatheory_v1(manifest) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(SubstitutionMetatheoryFailureV1::ManifestMismatch) => {
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        }
        Err(SubstitutionMetatheoryFailureV1::AgdaGate(_)) => {
            AuditDecision::Unknown(AuditUnknownReason::UnsupportedVerifier)
        }
        Err(
            SubstitutionMetatheoryFailureV1::MissingDependentTypingPreservation
            | SubstitutionMetatheoryFailureV1::MissingTypedQ0ReductionStability,
        ) => AuditDecision::Unknown(AuditUnknownReason::MissingSubstitutionMetatheory),
    }
}

fn verify_typed_substitution(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    source_context: &DependentContext,
    target_context: &DependentContext,
    images: &[Term],
) -> Result<TypedSimultaneousSubstitutionV1, AuditUnknownReason> {
    let source = kernel
        .verify_context(signature, source_context)
        .map_err(kernel_unknown)?
        .normalized_wire();
    let target = kernel
        .verify_context(signature, target_context)
        .map_err(kernel_unknown)?
        .normalized_wire();
    if images.len() != source.0.len() {
        return Err(AuditUnknownReason::MalformedInput);
    }

    let mut normalized_images = Vec::with_capacity(images.len());
    for (ordinal, (source_entry, image)) in source.0.iter().zip(images).enumerate() {
        let expected_type = instantiate_term_images(source_entry, ordinal, &normalized_images, 0)
            .ok_or(AuditUnknownReason::MalformedInput)?;
        let normalized = kernel
            .verify_open_judgment(
                signature,
                &OpenJudgment::HasType {
                    context: target.clone(),
                    term: image.clone(),
                    ty: expected_type,
                },
            )
            .map_err(kernel_unknown)?;
        let OpenJudgment::HasType { term, .. } = normalized else {
            return Err(AuditUnknownReason::KernelCouldNotCertify);
        };
        normalized_images.push(term);
    }

    let mut verified = TypedSimultaneousSubstitutionV1 {
        source_context: source,
        target_context: target,
        images: normalized_images,
        digest: Digest::of_bytes(b"pending typed simultaneous substitution"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/typed-simultaneous-substitution/v1",
        &verified,
    );
    Ok(verified)
}

fn raw_algebra_from_package(
    package: VerifiedFixedAgdaPackageV1,
) -> VerifiedRawSubstitutionAlgebraV1 {
    let mut verified = VerifiedRawSubstitutionAlgebraV1 {
        agda_source_tree_digest: package.source_tree_digest().clone(),
        agda_checker_transcript_digest: package.checker_stdout_digest().clone(),
        pinned_agda_package_digest: package.digest().clone(),
        digest: Digest::of_bytes(b"pending raw substitution algebra"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-raw-substitution-algebra/v1",
        &verified,
    );
    verified
}

fn identity_images(context: &DependentContext) -> Option<Vec<Term>> {
    (0..context.0.len())
        .map(|ordinal| {
            let index = context.0.len().checked_sub(ordinal.checked_add(1)?)?;
            Some(Term::Var {
                index: u32::try_from(index).ok()?,
            })
        })
        .collect()
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
            let ordinal = source_context_len.checked_sub(free_index.checked_add(1)?)?;
            shift_term(images.get(ordinal)?, i64::from(binder_depth), 0)
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

fn kernel_unknown(error: KernelError) -> AuditUnknownReason {
    match error {
        KernelError::ResourceExhausted(
            ResourceKind::Operations | ResourceKind::Depth | ResourceKind::Normalization,
        ) => AuditUnknownReason::ResourceExhausted,
        _ => AuditUnknownReason::KernelCouldNotCertify,
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

    fn empty_signature(kernel: &Kernel) -> VerifiedSignature {
        kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature")
    }

    #[test]
    fn arbitrary_images_are_checked_sequentially_in_dependent_contexts() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = empty_signature(&kernel);
        let source = DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]);
        let target = DependentContext::default();
        let AuditDecision::Proven(substitution) = verify_typed_simultaneous_substitution_v1(
            &kernel,
            &signature,
            &source,
            &target,
            &[Term::UnitType, Term::Unit],
        ) else {
            panic!("dependent substitution");
        };
        assert_eq!(substitution.images(), &[Term::UnitType, Term::Unit]);
        assert_eq!(
            substitution.apply(&Term::Var { index: 0 }),
            Some(Term::Unit)
        );
    }

    #[test]
    fn composition_is_on_demand_and_capture_avoiding_under_lambda() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = empty_signature(&kernel);
        let one = DependentContext(vec![Term::UnitType]);
        let empty = DependentContext::default();
        let AuditDecision::Proven(first) = verify_typed_simultaneous_substitution_v1(
            &kernel,
            &signature,
            &one,
            &one,
            &[Term::Unit],
        ) else {
            panic!("first substitution");
        };
        let AuditDecision::Proven(second) = verify_typed_simultaneous_substitution_v1(
            &kernel,
            &signature,
            &one,
            &empty,
            &[Term::Unit],
        ) else {
            panic!("second substitution");
        };
        let AuditDecision::Proven(composed) =
            compose_typed_substitutions_v1(&kernel, &signature, &first, &second)
        else {
            panic!("composition");
        };
        let source_term = Term::Lambda {
            parameter_type: Box::new(Term::UnitType),
            body: Box::new(Term::Var { index: 1 }),
        };
        assert_eq!(
            composed.apply(&source_term),
            Some(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Unit),
            })
        );
    }

    #[test]
    fn verified_composition_associates_for_an_opaque_endomorphism_instance() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let function = GlobalId(Digest::of_domain_bytes(
            "pen-semantic-audit/substitution-test-global/v1",
            b"opaque-unit-endomorphism",
        ));
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
            .expect("opaque function signature");
        let context = DependentContext(vec![Term::UnitType]);
        let image = Term::Apply {
            function: Box::new(Term::Global { id: function }),
            argument: Box::new(Term::Var { index: 0 }),
        };
        let AuditDecision::Proven(substitution) = verify_typed_simultaneous_substitution_v1(
            &kernel,
            &signature,
            &context,
            &context,
            &[image],
        ) else {
            panic!("endomorphism");
        };
        let AuditDecision::Proven(square) =
            compose_typed_substitutions_v1(&kernel, &signature, &substitution, &substitution)
        else {
            panic!("square");
        };
        let AuditDecision::Proven(left_associated) =
            compose_typed_substitutions_v1(&kernel, &signature, &square, &substitution)
        else {
            panic!("left-associated cube");
        };
        let AuditDecision::Proven(right_associated) =
            compose_typed_substitutions_v1(&kernel, &signature, &substitution, &square)
        else {
            panic!("right-associated cube");
        };
        assert_eq!(left_associated, right_associated);
    }

    #[test]
    fn lifting_and_weakening_are_direct_typed_substitutions() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = empty_signature(&kernel);
        let context = DependentContext(vec![Term::UnitType]);
        let AuditDecision::Proven(identity) =
            identity_typed_substitution_v1(&kernel, &signature, &context)
        else {
            panic!("identity");
        };
        let AuditDecision::Proven(lifted) =
            lift_typed_substitution_v1(&kernel, &signature, &identity, &Term::UnitType)
        else {
            panic!("lift");
        };
        assert_eq!(
            lifted.images(),
            &[Term::Var { index: 1 }, Term::Var { index: 0 }]
        );

        let AuditDecision::Proven(weakening) =
            weakening_typed_substitution_v1(&kernel, &signature, &context, &Term::UnitType)
        else {
            panic!("weakening");
        };
        assert_eq!(weakening.source_context(), &context);
        assert_eq!(
            weakening.target_context(),
            &DependentContext(vec![Term::UnitType, Term::UnitType])
        );
        assert_eq!(weakening.images(), &[Term::Var { index: 1 }]);
    }

    #[test]
    fn ill_typed_image_fails_closed() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = empty_signature(&kernel);
        let source = DependentContext(vec![Term::UnitType]);
        assert!(matches!(
            verify_typed_simultaneous_substitution_v1(
                &kernel,
                &signature,
                &source,
                &DependentContext::default(),
                &[Term::UnitType],
            ),
            AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify)
        ));
    }

    #[test]
    fn fixed_package_marks_the_exact_unproved_typing_frontier() {
        let source = std::str::from_utf8(include_bytes!(
            "../agda/LawV2/LambdaUnit/SubstitutionTyping.agda"
        ))
        .expect("UTF-8");
        assert!(source.contains("{-# OPTIONS --safe --without-K #-}"));
        assert!(source.contains("MissingDependentTypingTheorem"));
        assert!(source.contains("dependent-typing-lemma-not-yet-mechanized"));
        for fixed in AGDA_SOURCES {
            let text = std::str::from_utf8(fixed.bytes).expect("fixed Agda source UTF-8");
            assert!(!text.contains("postulate"));
            assert!(!text.contains("{-# TERMINATING #-}"));
        }
    }

    #[test]
    #[ignore = "requires the independently pinned local Agda 2.8.0 runtime"]
    fn pinned_raw_substitution_algebra_checks() {
        let checked = diagnose_pinned_fixed_agda_package_v1(AGDA_SOURCES, AGDA_ENTRY_RELATIVE_PATH);
        assert!(checked.is_ok(), "{checked:?}");
        assert!(matches!(
            verify_pinned_raw_substitution_algebra_v1(),
            AuditDecision::Proven(_)
        ));
    }

    #[test]
    #[ignore = "requires the independently pinned local Agda 2.8.0 runtime"]
    fn full_capability_stops_at_dependent_typing_preservation() {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v2(
            &proposed_semantic_audit_lambda_unit_manifest_v2(),
        ) else {
            panic!("V2 manifest");
        };
        assert!(matches!(
            diagnose_substitution_metatheory_v1(&manifest),
            Err(SubstitutionMetatheoryFailureV1::MissingDependentTypingPreservation)
        ));
        assert!(matches!(
            verify_substitution_metatheory_v1(&manifest),
            AuditDecision::Unknown(AuditUnknownReason::MissingSubstitutionMetatheory)
        ));
    }
}
