//! Intrinsic registration and closed instantiation for the native dependent
//! core.
//!
//! A successful registration witnesses one checked constructor. A successful
//! instantiation witnesses one complete, ordered assignment. Neither result is
//! completeness evidence for a larger demand language or a theorem about an
//! external checker.

use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, CertificateClaim, DependentContext, Digest, Kernel,
    KernelError, OpenJudgment, ScopeInputs, Term, TrustedScope,
    UncheckedClosedSpecializationCertificate, VerifiedClosedSpecialization, VerifiedContext,
    VerifiedSignature,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const INTRINSIC_SCHEME_SCHEMA_VERSION: u16 = 1;

/// The native constructors that can be replayed by `pen-kernel`.
///
/// Each variant determines the judgment form; callers cannot supply an
/// unrelated open-judgment tag.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "rule", rename_all = "snake_case", deny_unknown_fields)]
pub enum SchemeRule {
    TypeFormationUse { formed_type: Term },
    TypedTermUse { term: Term, ty: Term },
    DefinitionalComputationClosure { left: Term, right: Term, ty: Term },
}

impl SchemeRule {
    fn open_judgment(&self, context: DependentContext) -> OpenJudgment {
        match self {
            Self::TypeFormationUse { formed_type } => OpenJudgment::TypeFormation {
                context,
                term: formed_type.clone(),
            },
            Self::TypedTermUse { term, ty } => OpenJudgment::HasType {
                context,
                term: term.clone(),
                ty: ty.clone(),
            },
            Self::DefinitionalComputationClosure { left, right, ty } => {
                OpenJudgment::DefinitionallyEqual {
                    context,
                    left: left.clone(),
                    right: right.clone(),
                    ty: ty.clone(),
                }
            }
        }
    }

    fn encode_tag(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::TypeFormationUse { .. } => 0,
            Self::TypedTermUse { .. } => 1,
            Self::DefinitionalComputationClosure { .. } => 2,
        });
    }

    fn encode_body(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::TypeFormationUse { formed_type } => {
                formed_type.encode_canonical(encoder);
            }
            Self::TypedTermUse { term, ty } => {
                term.encode_canonical(encoder);
                ty.encode_canonical(encoder);
            }
            Self::DefinitionalComputationClosure { left, right, ty } => {
                left.encode_canonical(encoder);
                right.encode_canonical(encoder);
                ty.encode_canonical(encoder);
            }
        }
    }
}

impl CanonicalEncode for SchemeRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.encode_tag(encoder);
        self.encode_body(encoder);
    }
}

/// Deserializable constructor input. Verification, rather than construction or
/// deserialization, grants the corresponding capability.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicScheme {
    pub schema_version: u16,
    pub context: DependentContext,
    pub rule: SchemeRule,
}

impl IntrinsicScheme {
    pub fn type_formation(context: DependentContext, formed_type: Term) -> Self {
        Self {
            schema_version: INTRINSIC_SCHEME_SCHEMA_VERSION,
            context,
            rule: SchemeRule::TypeFormationUse { formed_type },
        }
    }

    pub fn typed_term(context: DependentContext, term: Term, ty: Term) -> Self {
        Self {
            schema_version: INTRINSIC_SCHEME_SCHEMA_VERSION,
            context,
            rule: SchemeRule::TypedTermUse { term, ty },
        }
    }

    pub fn definitional_computation(
        context: DependentContext,
        left: Term,
        right: Term,
        ty: Term,
    ) -> Self {
        Self {
            schema_version: INTRINSIC_SCHEME_SCHEMA_VERSION,
            context,
            rule: SchemeRule::DefinitionalComputationClosure { left, right, ty },
        }
    }

    fn open_judgment(&self) -> OpenJudgment {
        self.rule.open_judgment(self.context.clone())
    }
}

impl CanonicalEncode for IntrinsicScheme {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.context.encode_canonical(encoder);
        self.rule.encode_canonical(encoder);
    }
}

struct RuleTag<'a>(&'a SchemeRule);

impl CanonicalEncode for RuleTag<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_tag(encoder);
    }
}

struct RuleBody<'a>(&'a SchemeRule);

impl CanonicalEncode for RuleBody<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_body(encoder);
    }
}

/// Strict wire record for a claimed native registration.
///
/// This record is never proof by itself. Replay against the separately
/// supplied scheme and verified signature is required.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedIntrinsicSchemeCertificate {
    pub schema_version: u16,
    pub scheme_digest: Digest,
    pub rule_digest: Digest,
    pub body_digest: Digest,
    pub context_digest: Digest,
    pub signature_digest: Digest,
    pub kernel_digest: Digest,
    pub normalizer_digest: Digest,
    pub source_digest: Digest,
    pub normalized_open_judgment: OpenJudgment,
}

impl CanonicalEncode for UncheckedIntrinsicSchemeCertificate {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.scheme_digest.encode_canonical(encoder);
        self.rule_digest.encode_canonical(encoder);
        self.body_digest.encode_canonical(encoder);
        self.context_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_digest.encode_canonical(encoder);
        self.normalizer_digest.encode_canonical(encoder);
        self.source_digest.encode_canonical(encoder);
        self.normalized_open_judgment.encode_canonical(encoder);
    }
}

/// Successful local registration. It cannot be deserialized or fabricated
/// outside this module.
#[derive(Clone, Debug)]
pub struct VerifiedIntrinsicScheme {
    certificate: UncheckedIntrinsicSchemeCertificate,
    scheme: IntrinsicScheme,
    verified_context: VerifiedContext,
    normalized_open_judgment: OpenJudgment,
}

impl VerifiedIntrinsicScheme {
    pub fn certificate(&self) -> &UncheckedIntrinsicSchemeCertificate {
        &self.certificate
    }

    pub fn scheme(&self) -> &IntrinsicScheme {
        &self.scheme
    }

    pub fn verified_context(&self) -> &VerifiedContext {
        &self.verified_context
    }

    pub fn normalized_open_judgment(&self) -> &OpenJudgment {
        &self.normalized_open_judgment
    }

    pub fn digest(&self) -> &Digest {
        &self.certificate.scheme_digest
    }
}

/// Successful replay of one complete ordered assignment.
///
/// The enclosed kernel capability is deliberately non-serializable.
#[derive(Clone, Debug)]
pub struct VerifiedIntrinsicSpecialization {
    scheme_digest: Digest,
    assignments_digest: Digest,
    kernel_capability: VerifiedClosedSpecialization,
}

impl VerifiedIntrinsicSpecialization {
    pub fn scheme_digest(&self) -> &Digest {
        &self.scheme_digest
    }

    pub fn assignments_digest(&self) -> &Digest {
        &self.assignments_digest
    }

    pub fn kernel_capability(&self) -> &VerifiedClosedSpecialization {
        &self.kernel_capability
    }

    pub fn normalized_judgment(&self) -> &OpenJudgment {
        self.kernel_capability.normalized_specialized_judgment()
    }
}

/// Closed failure boundary. Unsupported or malformed inputs remain unknown;
/// this API has no negative-proof disposition.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntrinsicSchemeUnknown {
    ResourceExhausted,
    Unsupported,
}

#[derive(Clone, Debug)]
pub enum IntrinsicSchemeOutcome<T> {
    Verified(T),
    Unknown(IntrinsicSchemeUnknown),
}

pub fn intrinsic_scheme_source_digest() -> Digest {
    Digest::of_domain_chunks(
        "pen-demand/intrinsic-scheme-source/v1",
        &[
            include_bytes!("scheme.rs"),
            include_bytes!("lib.rs"),
            include_bytes!("../Cargo.toml"),
        ],
    )
}

pub fn register_intrinsic_scheme(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    scheme: &IntrinsicScheme,
) -> IntrinsicSchemeOutcome<VerifiedIntrinsicScheme> {
    let certificate = match claim_intrinsic_scheme_certificate(kernel, signature, scheme) {
        IntrinsicSchemeOutcome::Verified(certificate) => certificate,
        IntrinsicSchemeOutcome::Unknown(reason) => return IntrinsicSchemeOutcome::Unknown(reason),
    };
    verify_intrinsic_scheme_certificate(kernel, signature, scheme, &certificate)
}

/// Build a wire record after native checking. Once this record crosses a wire
/// boundary it must still be treated as untrusted and replayed.
pub fn claim_intrinsic_scheme_certificate(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    scheme: &IntrinsicScheme,
) -> IntrinsicSchemeOutcome<UncheckedIntrinsicSchemeCertificate> {
    if scheme.schema_version != INTRINSIC_SCHEME_SCHEMA_VERSION {
        return IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported);
    }
    if let Err(error) = kernel.verify_context(signature, &scheme.context) {
        return IntrinsicSchemeOutcome::Unknown(kernel_reason(error));
    }
    let normalized_open_judgment =
        match kernel.verify_open_judgment(signature, &scheme.open_judgment()) {
            Ok(judgment) => judgment,
            Err(error) => return IntrinsicSchemeOutcome::Unknown(kernel_reason(error)),
        };
    IntrinsicSchemeOutcome::Verified(expected_certificate(
        kernel,
        signature,
        scheme,
        normalized_open_judgment,
    ))
}

pub fn verify_intrinsic_scheme_certificate(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    scheme: &IntrinsicScheme,
    certificate: &UncheckedIntrinsicSchemeCertificate,
) -> IntrinsicSchemeOutcome<VerifiedIntrinsicScheme> {
    if scheme.schema_version != INTRINSIC_SCHEME_SCHEMA_VERSION
        || certificate.schema_version != INTRINSIC_SCHEME_SCHEMA_VERSION
    {
        return IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported);
    }

    let verified_context = match kernel.verify_context(signature, &scheme.context) {
        Ok(context) => context,
        Err(error) => return IntrinsicSchemeOutcome::Unknown(kernel_reason(error)),
    };
    let normalized_open_judgment =
        match kernel.verify_open_judgment(signature, &scheme.open_judgment()) {
            Ok(judgment) => judgment,
            Err(error) => return IntrinsicSchemeOutcome::Unknown(kernel_reason(error)),
        };
    if let Err(error) =
        kernel.verify_open_judgment(signature, &certificate.normalized_open_judgment)
    {
        return IntrinsicSchemeOutcome::Unknown(kernel_reason(error));
    }

    let expected =
        expected_certificate(kernel, signature, scheme, normalized_open_judgment.clone());
    if *certificate != expected {
        return IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported);
    }

    IntrinsicSchemeOutcome::Verified(VerifiedIntrinsicScheme {
        certificate: expected,
        scheme: scheme.clone(),
        verified_context,
        normalized_open_judgment,
    })
}

/// Replay one exact, complete, left-to-right assignment through the registered
/// constructor. Assignment typing and the final closed judgment are checked
/// again by `pen-kernel`.
pub fn specialize_intrinsic_scheme(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    registered: &VerifiedIntrinsicScheme,
    assignments: &[Term],
) -> IntrinsicSchemeOutcome<VerifiedIntrinsicSpecialization> {
    if registered.certificate.kernel_digest != kernel.kernel_protocol_digest()
        || registered.certificate.normalizer_digest != kernel.normalizer_protocol_digest()
        || registered.certificate.source_digest != intrinsic_scheme_source_digest()
        || registered.certificate.signature_digest != *signature.digest()
        || assignments.len() != registered.verified_context.entries().len()
    {
        return IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported);
    }

    let subject_digest = match kernel.closed_specialization_subject_digest(
        signature,
        &registered.normalized_open_judgment,
        assignments,
    ) {
        Ok(digest) => digest,
        Err(error) => return IntrinsicSchemeOutcome::Unknown(kernel_reason(error)),
    };
    let normalized_specialized_judgment = match kernel.normalize_closed_specialization(
        signature,
        &registered.normalized_open_judgment,
        assignments,
    ) {
        Ok(judgment) => judgment,
        Err(error) => return IntrinsicSchemeOutcome::Unknown(kernel_reason(error)),
    };

    let assignments_digest = assignment_digest(assignments);
    let scope = specialization_scope(kernel, signature, registered, &assignments_digest);
    let certificate = UncheckedClosedSpecializationCertificate {
        binding: scope.binding().clone(),
        subject_digest,
        open_judgment: registered.normalized_open_judgment.clone(),
        assignments: assignments.to_vec(),
        normalized_specialized_judgment,
    };
    let kernel_capability = match kernel.verify_closed_specialization_certificate(
        &scope,
        signature,
        &registered.normalized_open_judgment,
        assignments,
        &certificate,
    ) {
        Ok(capability) => capability,
        Err(error) => {
            return IntrinsicSchemeOutcome::Unknown(match error {
                pen_kernel::CertificateError::Kernel(kernel_error) => kernel_reason(kernel_error),
                _ => IntrinsicSchemeUnknown::Unsupported,
            });
        }
    };

    IntrinsicSchemeOutcome::Verified(VerifiedIntrinsicSpecialization {
        scheme_digest: registered.certificate.scheme_digest.clone(),
        assignments_digest,
        kernel_capability,
    })
}

fn expected_certificate(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    scheme: &IntrinsicScheme,
    normalized_open_judgment: OpenJudgment,
) -> UncheckedIntrinsicSchemeCertificate {
    UncheckedIntrinsicSchemeCertificate {
        schema_version: INTRINSIC_SCHEME_SCHEMA_VERSION,
        scheme_digest: Digest::of_canonical("pen-demand/intrinsic-scheme/v1", scheme),
        rule_digest: Digest::of_canonical(
            "pen-demand/intrinsic-scheme-rule/v1",
            &RuleTag(&scheme.rule),
        ),
        body_digest: Digest::of_canonical(
            "pen-demand/intrinsic-scheme-body/v1",
            &RuleBody(&scheme.rule),
        ),
        context_digest: Digest::of_canonical(
            "pen-demand/intrinsic-scheme-context/v1",
            &scheme.context,
        ),
        signature_digest: signature.digest().clone(),
        kernel_digest: kernel.kernel_protocol_digest(),
        normalizer_digest: kernel.normalizer_protocol_digest(),
        source_digest: intrinsic_scheme_source_digest(),
        normalized_open_judgment,
    }
}

fn assignment_digest(assignments: &[Term]) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.sequence(assignments);
    Digest::of_domain_bytes(
        "pen-demand/intrinsic-scheme-assignments/v1",
        encoder.as_bytes(),
    )
}

fn specialization_scope(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    registered: &VerifiedIntrinsicScheme,
    assignments_digest: &Digest,
) -> TrustedScope {
    let source_namespace = Digest::of_domain_chunks(
        "pen-demand/intrinsic-scheme-kernel-scope/v1",
        &[
            registered.certificate.source_digest.as_str().as_bytes(),
            registered.certificate.scheme_digest.as_str().as_bytes(),
        ],
    );
    TrustedScope::new(
        kernel,
        CertificateClaim::ClosedSpecialization,
        ScopeInputs {
            law_digest: source_namespace.clone(),
            grammar_digest: registered.certificate.rule_digest.clone(),
            scheme_calculus_digest: registered.certificate.scheme_digest.clone(),
            blindness_contract_digest: source_namespace.clone(),
            bootstrap_contract_digest: source_namespace,
            history_digest: registered.certificate.signature_digest.clone(),
            public_boundary_digest: signature.digest().clone(),
            derivation_basis_digest: registered.certificate.body_digest.clone(),
            active_window_digest: registered.certificate.context_digest.clone(),
            candidate_digest: assignments_digest.clone(),
        },
    )
}

fn kernel_reason(error: KernelError) -> IntrinsicSchemeUnknown {
    match error {
        KernelError::ResourceExhausted(_) => IntrinsicSchemeUnknown::ResourceExhausted,
        _ => IntrinsicSchemeUnknown::Unsupported,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        INTRINSIC_SCHEME_SCHEMA_VERSION, IntrinsicScheme, IntrinsicSchemeOutcome,
        IntrinsicSchemeUnknown, UncheckedIntrinsicSchemeCertificate,
        claim_intrinsic_scheme_certificate, register_intrinsic_scheme, specialize_intrinsic_scheme,
        verify_intrinsic_scheme_certificate,
    };
    use pen_kernel::{
        Declaration, DependentContext, Digest, GlobalId, Kernel, KernelLimits, OpenJudgment, Term,
        UncheckedSignature, VerifiedSignature,
    };

    fn id(label: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid limits")
    }

    fn toy_signature(kernel: &Kernel) -> VerifiedSignature {
        let carrier = id(b"toy-carrier");
        kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: carrier.clone(),
                        ty: Term::Sort { level: 0 },
                        body: None,
                    },
                    Declaration {
                        id: id(b"toy-item"),
                        ty: Term::Global { id: carrier },
                        body: None,
                    },
                ],
            })
            .expect("toy signature")
    }

    fn empty_signature(kernel: &Kernel) -> VerifiedSignature {
        kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature")
    }

    fn registered(
        kernel: &Kernel,
        signature: &VerifiedSignature,
        scheme: &IntrinsicScheme,
    ) -> super::VerifiedIntrinsicScheme {
        match register_intrinsic_scheme(kernel, signature, scheme) {
            IntrinsicSchemeOutcome::Verified(registered) => registered,
            IntrinsicSchemeOutcome::Unknown(reason) => {
                panic!("expected registration, got {reason:?}")
            }
        }
    }

    #[test]
    fn constructors_register_and_specialize_dependent_toys() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let context = DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]);

        let type_use = IntrinsicScheme::type_formation(
            DependentContext(vec![Term::Sort { level: 0 }]),
            Term::Var { index: 0 },
        );
        let type_registered = registered(&kernel, &signature, &type_use);
        let type_specialized = match specialize_intrinsic_scheme(
            &kernel,
            &signature,
            &type_registered,
            &[Term::UnitType],
        ) {
            IntrinsicSchemeOutcome::Verified(value) => value,
            IntrinsicSchemeOutcome::Unknown(reason) => {
                panic!("expected specialization, got {reason:?}")
            }
        };
        assert_eq!(
            type_specialized.normalized_judgment(),
            &OpenJudgment::TypeFormation {
                context: DependentContext::default(),
                term: Term::UnitType,
            }
        );

        let term_use = IntrinsicScheme::typed_term(
            context.clone(),
            Term::Var { index: 0 },
            Term::Var { index: 1 },
        );
        let term_registered = registered(&kernel, &signature, &term_use);
        let term_specialized = match specialize_intrinsic_scheme(
            &kernel,
            &signature,
            &term_registered,
            &[Term::UnitType, Term::Unit],
        ) {
            IntrinsicSchemeOutcome::Verified(value) => value,
            IntrinsicSchemeOutcome::Unknown(reason) => {
                panic!("expected specialization, got {reason:?}")
            }
        };
        assert_eq!(
            term_specialized.normalized_judgment(),
            &OpenJudgment::HasType {
                context: DependentContext::default(),
                term: Term::Unit,
                ty: Term::UnitType,
            }
        );

        let computation = IntrinsicScheme::definitional_computation(
            context,
            Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::Var { index: 1 }),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Var { index: 0 }),
            },
            Term::Var { index: 0 },
            Term::Var { index: 1 },
        );
        let computation_registered = registered(&kernel, &signature, &computation);
        let computation_specialized = match specialize_intrinsic_scheme(
            &kernel,
            &signature,
            &computation_registered,
            &[Term::UnitType, Term::Unit],
        ) {
            IntrinsicSchemeOutcome::Verified(value) => value,
            IntrinsicSchemeOutcome::Unknown(reason) => {
                panic!("expected specialization, got {reason:?}")
            }
        };
        assert_eq!(
            computation_specialized.normalized_judgment(),
            &OpenJudgment::DefinitionallyEqual {
                context: DependentContext::default(),
                left: Term::Unit,
                right: Term::Unit,
                ty: Term::UnitType,
            }
        );
    }

    #[test]
    fn closed_global_toy_is_checked_without_assignments() {
        let kernel = kernel();
        let signature = toy_signature(&kernel);
        let scheme = IntrinsicScheme::typed_term(
            DependentContext::default(),
            Term::Global {
                id: id(b"toy-item"),
            },
            Term::Global {
                id: id(b"toy-carrier"),
            },
        );
        let registered = registered(&kernel, &signature, &scheme);
        assert!(matches!(
            specialize_intrinsic_scheme(&kernel, &signature, &registered, &[]),
            IntrinsicSchemeOutcome::Verified(_)
        ));
    }

    #[test]
    fn malformed_rules_and_versions_remain_unknown() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let malformed = IntrinsicScheme::typed_term(
            DependentContext::default(),
            Term::Unit,
            Term::Sort { level: 0 },
        );
        assert!(matches!(
            register_intrinsic_scheme(&kernel, &signature, &malformed),
            IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported)
        ));

        let mut wrong_version =
            IntrinsicScheme::type_formation(DependentContext::default(), Term::UnitType);
        wrong_version.schema_version = INTRINSIC_SCHEME_SCHEMA_VERSION + 1;
        assert!(matches!(
            register_intrinsic_scheme(&kernel, &signature, &wrong_version),
            IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported)
        ));
    }

    #[test]
    fn every_registration_binding_is_replayed() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let scheme = IntrinsicScheme::type_formation(DependentContext::default(), Term::UnitType);
        let certificate = match claim_intrinsic_scheme_certificate(&kernel, &signature, &scheme) {
            IntrinsicSchemeOutcome::Verified(certificate) => certificate,
            IntrinsicSchemeOutcome::Unknown(reason) => {
                panic!("expected certificate, got {reason:?}")
            }
        };
        assert!(matches!(
            verify_intrinsic_scheme_certificate(&kernel, &signature, &scheme, &certificate),
            IntrinsicSchemeOutcome::Verified(_)
        ));

        let mutations: [fn(&mut UncheckedIntrinsicSchemeCertificate); 8] = [
            |value| value.scheme_digest = Digest::of_bytes(b"mutated-scheme"),
            |value| value.rule_digest = Digest::of_bytes(b"mutated-rule"),
            |value| value.body_digest = Digest::of_bytes(b"mutated-body"),
            |value| value.context_digest = Digest::of_bytes(b"mutated-context"),
            |value| value.signature_digest = Digest::of_bytes(b"mutated-signature"),
            |value| value.kernel_digest = Digest::of_bytes(b"mutated-kernel"),
            |value| value.normalizer_digest = Digest::of_bytes(b"mutated-normalizer"),
            |value| value.source_digest = Digest::of_bytes(b"mutated-source"),
        ];
        for mutate in mutations {
            let mut changed = certificate.clone();
            mutate(&mut changed);
            assert!(matches!(
                verify_intrinsic_scheme_certificate(&kernel, &signature, &scheme, &changed),
                IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported)
            ));
        }

        let mut changed_normal = certificate;
        changed_normal.normalized_open_judgment = OpenJudgment::TypeFormation {
            context: DependentContext::default(),
            term: Term::Sort { level: 0 },
        };
        assert!(matches!(
            verify_intrinsic_scheme_certificate(&kernel, &signature, &scheme, &changed_normal),
            IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported)
        ));
    }

    #[test]
    fn incomplete_or_misordered_assignments_are_not_negative_proofs() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let scheme = IntrinsicScheme::typed_term(
            DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]),
            Term::Var { index: 0 },
            Term::Var { index: 1 },
        );
        let registered = registered(&kernel, &signature, &scheme);
        assert!(matches!(
            specialize_intrinsic_scheme(&kernel, &signature, &registered, &[Term::UnitType]),
            IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported)
        ));
        assert!(matches!(
            specialize_intrinsic_scheme(
                &kernel,
                &signature,
                &registered,
                &[Term::Unit, Term::UnitType]
            ),
            IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::Unsupported)
        ));
    }

    #[test]
    fn strict_wire_format_rejects_unknown_fields() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let scheme = IntrinsicScheme::type_formation(DependentContext::default(), Term::UnitType);
        let registered = registered(&kernel, &signature, &scheme);
        let mut wire = serde_json::to_value(registered.certificate()).expect("serialize");
        wire.as_object_mut()
            .expect("certificate object")
            .insert("proof".to_owned(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<UncheckedIntrinsicSchemeCertificate>(wire).is_err());

        let mut scheme_wire = serde_json::to_value(&scheme).expect("serialize scheme");
        scheme_wire
            .as_object_mut()
            .expect("scheme object")
            .insert("trusted".to_owned(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<IntrinsicScheme>(scheme_wire).is_err());
    }

    #[test]
    fn verifier_resource_exhaustion_stays_distinct() {
        let full = kernel();
        let signature = empty_signature(&full);
        let scheme = IntrinsicScheme::type_formation(DependentContext::default(), Term::UnitType);
        let registered = registered(&full, &signature, &scheme);
        let limited = Kernel::new(KernelLimits {
            max_operations: 1,
            max_depth: 8,
            normalization_fuel: 1,
        })
        .expect("positive limits");

        assert!(matches!(
            register_intrinsic_scheme(&limited, &signature, &scheme),
            IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::ResourceExhausted)
        ));
        assert!(matches!(
            specialize_intrinsic_scheme(&limited, &signature, &registered, &[]),
            IntrinsicSchemeOutcome::Unknown(IntrinsicSchemeUnknown::ResourceExhausted)
        ));
    }
}
