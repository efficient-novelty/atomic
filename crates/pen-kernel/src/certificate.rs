use crate::{
    CanonicalEncode, CanonicalEncoder, Digest, Kernel, KernelError, OpenJudgment, Term,
    UncheckedSignature, VerifiedSignature,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CERTIFICATE_SCHEMA_VERSION: u16 = 1;
pub const CANONICAL_CODEC_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificateClaim {
    Derivation,
    DefinitionalEquivalence,
    FreeSealingFragment,
    ClosedSpecialization,
}

impl CanonicalEncode for CertificateClaim {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Derivation => 0,
            Self::DefinitionalEquivalence => 1,
            Self::FreeSealingFragment => 2,
            Self::ClosedSpecialization => 3,
        });
    }
}

/// Every proof object is tied to the complete public verification context.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertificateBinding {
    pub schema_version: u16,
    pub codec_version: u16,
    pub claim: CertificateClaim,
    pub kernel_digest: Digest,
    pub normalizer_digest: Digest,
    pub law_digest: Digest,
    pub grammar_digest: Digest,
    pub scheme_calculus_digest: Digest,
    pub blindness_contract_digest: Digest,
    pub bootstrap_contract_digest: Digest,
    pub history_digest: Digest,
    pub public_boundary_digest: Digest,
    pub derivation_basis_digest: Digest,
    pub active_window_digest: Digest,
    pub candidate_digest: Digest,
}

impl CanonicalEncode for CertificateBinding {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.u16(self.codec_version);
        self.claim.encode_canonical(encoder);
        self.kernel_digest.encode_canonical(encoder);
        self.normalizer_digest.encode_canonical(encoder);
        self.law_digest.encode_canonical(encoder);
        self.grammar_digest.encode_canonical(encoder);
        self.scheme_calculus_digest.encode_canonical(encoder);
        self.blindness_contract_digest.encode_canonical(encoder);
        self.bootstrap_contract_digest.encode_canonical(encoder);
        self.history_digest.encode_canonical(encoder);
        self.public_boundary_digest.encode_canonical(encoder);
        self.derivation_basis_digest.encode_canonical(encoder);
        self.active_window_digest.encode_canonical(encoder);
        self.candidate_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopeInputs {
    pub law_digest: Digest,
    pub grammar_digest: Digest,
    pub scheme_calculus_digest: Digest,
    pub blindness_contract_digest: Digest,
    pub bootstrap_contract_digest: Digest,
    pub history_digest: Digest,
    pub public_boundary_digest: Digest,
    pub derivation_basis_digest: Digest,
    pub active_window_digest: Digest,
    pub candidate_digest: Digest,
}

/// Locally trusted expected scope. It cannot be obtained by deserializing a
/// certificate and is compared field-for-field during replay.
#[derive(Clone, Debug)]
pub struct TrustedScope {
    binding: CertificateBinding,
    digest: Digest,
}

impl TrustedScope {
    pub fn new(kernel: &Kernel, claim: CertificateClaim, inputs: ScopeInputs) -> Self {
        let binding = CertificateBinding {
            schema_version: CERTIFICATE_SCHEMA_VERSION,
            codec_version: CANONICAL_CODEC_VERSION,
            claim,
            kernel_digest: kernel.kernel_protocol_digest(),
            normalizer_digest: kernel.normalizer_protocol_digest(),
            law_digest: inputs.law_digest,
            grammar_digest: inputs.grammar_digest,
            scheme_calculus_digest: inputs.scheme_calculus_digest,
            blindness_contract_digest: inputs.blindness_contract_digest,
            bootstrap_contract_digest: inputs.bootstrap_contract_digest,
            history_digest: inputs.history_digest,
            public_boundary_digest: inputs.public_boundary_digest,
            derivation_basis_digest: inputs.derivation_basis_digest,
            active_window_digest: inputs.active_window_digest,
            candidate_digest: inputs.candidate_digest,
        };
        let digest = Digest::of_canonical("certificate-binding-v1", &binding);
        Self { binding, digest }
    }

    pub fn binding(&self) -> &CertificateBinding {
        &self.binding
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedDerivationCertificate {
    pub binding: CertificateBinding,
    pub subject_digest: Digest,
    pub judgment: OpenJudgment,
    pub normalized_judgment: OpenJudgment,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedEquivalenceCertificate {
    pub binding: CertificateBinding,
    pub subject_digest: Digest,
    pub judgment: OpenJudgment,
    pub normalized_judgment: OpenJudgment,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedClosedSpecializationCertificate {
    pub binding: CertificateBinding,
    pub subject_digest: Digest,
    pub open_judgment: OpenJudgment,
    pub assignments: Vec<Term>,
    pub normalized_specialized_judgment: OpenJudgment,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedFreeSealingCertificate {
    pub binding: CertificateBinding,
    pub subject_digest: Digest,
    pub extension: UncheckedSignature,
    pub normalized_sealed_signature: UncheckedSignature,
}

/// Successful verification capability. It is intentionally not serializable.
#[derive(Clone, Debug)]
pub struct VerifiedDerivation {
    scope_digest: Digest,
    subject_digest: Digest,
    normalized_judgment: OpenJudgment,
}

impl VerifiedDerivation {
    pub fn scope_digest(&self) -> &Digest {
        &self.scope_digest
    }

    pub fn subject_digest(&self) -> &Digest {
        &self.subject_digest
    }

    pub fn normalized_judgment(&self) -> &OpenJudgment {
        &self.normalized_judgment
    }
}

/// Distinct capability: a derivation cannot be substituted for equivalence.
#[derive(Clone, Debug)]
pub struct VerifiedDefinitionalEquivalence {
    scope_digest: Digest,
    subject_digest: Digest,
    normalized_judgment: OpenJudgment,
}

/// A replayed particular closed substitution. It is not a theorem that every
/// admissible substitution specializes successfully.
#[derive(Clone, Debug)]
pub struct VerifiedClosedSpecialization {
    scope_digest: Digest,
    subject_digest: Digest,
    normalized_specialized_judgment: OpenJudgment,
}

impl VerifiedClosedSpecialization {
    pub fn scope_digest(&self) -> &Digest {
        &self.scope_digest
    }

    pub fn subject_digest(&self) -> &Digest {
        &self.subject_digest
    }

    pub fn normalized_specialized_judgment(&self) -> &OpenJudgment {
        &self.normalized_specialized_judgment
    }
}

impl VerifiedDefinitionalEquivalence {
    pub fn scope_digest(&self) -> &Digest {
        &self.scope_digest
    }

    pub fn subject_digest(&self) -> &Digest {
        &self.subject_digest
    }

    pub fn normalized_judgment(&self) -> &OpenJudgment {
        &self.normalized_judgment
    }
}

/// Exact extension capability. This does not authorize history mutation.
#[derive(Clone, Debug)]
pub struct VerifiedFreeSealing {
    scope_digest: Digest,
    subject_digest: Digest,
    sealed_signature: VerifiedSignature,
}

impl VerifiedFreeSealing {
    pub fn scope_digest(&self) -> &Digest {
        &self.scope_digest
    }

    pub fn subject_digest(&self) -> &Digest {
        &self.subject_digest
    }

    pub fn sealed_signature(&self) -> &VerifiedSignature {
        &self.sealed_signature
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum CertificateError {
    #[error("certificate scope does not match the trusted scope")]
    ScopeMismatch,
    #[error("trusted public boundary does not match the verified signature")]
    BoundaryMismatch,
    #[error("certificate subject differs from the expected subject")]
    SubjectMismatch,
    #[error("certificate contains a different claim than the expected subject")]
    ClaimMismatch,
    #[error("claimed normal form is not the replayed normal form")]
    NormalFormMismatch,
    #[error("claimed sealed signature is not the exact normalized extension")]
    SealedSignatureMismatch,
    #[error(transparent)]
    Kernel(#[from] KernelError),
}

impl Kernel {
    pub fn judgment_subject_digest(
        &self,
        signature: &VerifiedSignature,
        judgment: &OpenJudgment,
    ) -> Result<Digest, KernelError> {
        self.validate_certificate_judgments(&[judgment])?;
        let mut encoder = CanonicalEncoder::new();
        signature.digest().encode_canonical(&mut encoder);
        judgment.encode_canonical(&mut encoder);
        Ok(Digest::of_domain_bytes(
            "open-judgment-subject-v1",
            encoder.as_bytes(),
        ))
    }

    pub fn sealing_subject_digest(
        &self,
        base: &VerifiedSignature,
        extension: &UncheckedSignature,
    ) -> Result<Digest, KernelError> {
        self.validate_certificate_signatures(&[extension])?;
        let mut encoder = CanonicalEncoder::new();
        base.digest().encode_canonical(&mut encoder);
        extension.encode_canonical(&mut encoder);
        Ok(Digest::of_domain_bytes(
            "signature-extension-subject-v1",
            encoder.as_bytes(),
        ))
    }

    pub fn closed_specialization_subject_digest(
        &self,
        signature: &VerifiedSignature,
        open: &OpenJudgment,
        assignments: &[Term],
    ) -> Result<Digest, KernelError> {
        self.validate_certificate_judgments(&[open])?;
        self.validate_certificate_terms(assignments)?;
        let mut encoder = CanonicalEncoder::new();
        signature.digest().encode_canonical(&mut encoder);
        open.encode_canonical(&mut encoder);
        encoder.sequence(assignments);
        Ok(Digest::of_domain_bytes(
            "closed-specialization-subject-v1",
            encoder.as_bytes(),
        ))
    }

    pub fn verify_derivation_certificate(
        &self,
        scope: &TrustedScope,
        signature: &VerifiedSignature,
        expected: &OpenJudgment,
        certificate: &UncheckedDerivationCertificate,
    ) -> Result<VerifiedDerivation, CertificateError> {
        require_scope(
            scope,
            signature,
            CertificateClaim::Derivation,
            &certificate.binding,
        )?;
        self.validate_certificate_judgments(&[
            expected,
            &certificate.judgment,
            &certificate.normalized_judgment,
        ])?;
        if certificate.judgment != *expected {
            return Err(CertificateError::ClaimMismatch);
        }
        let subject_digest = self.judgment_subject_digest(signature, expected)?;
        if certificate.subject_digest != subject_digest {
            return Err(CertificateError::SubjectMismatch);
        }
        let normalized = self.verify_open_judgment(signature, expected)?;
        if certificate.normalized_judgment != normalized {
            return Err(CertificateError::NormalFormMismatch);
        }
        Ok(VerifiedDerivation {
            scope_digest: scope.digest().clone(),
            subject_digest,
            normalized_judgment: normalized,
        })
    }

    pub fn verify_equivalence_certificate(
        &self,
        scope: &TrustedScope,
        signature: &VerifiedSignature,
        expected: &OpenJudgment,
        certificate: &UncheckedEquivalenceCertificate,
    ) -> Result<VerifiedDefinitionalEquivalence, CertificateError> {
        require_scope(
            scope,
            signature,
            CertificateClaim::DefinitionalEquivalence,
            &certificate.binding,
        )?;
        self.validate_certificate_judgments(&[
            expected,
            &certificate.judgment,
            &certificate.normalized_judgment,
        ])?;
        if !matches!(expected, OpenJudgment::DefinitionallyEqual { .. })
            || certificate.judgment != *expected
        {
            return Err(CertificateError::ClaimMismatch);
        }
        let subject_digest = self.judgment_subject_digest(signature, expected)?;
        if certificate.subject_digest != subject_digest {
            return Err(CertificateError::SubjectMismatch);
        }
        let normalized = self.verify_open_judgment(signature, expected)?;
        if certificate.normalized_judgment != normalized {
            return Err(CertificateError::NormalFormMismatch);
        }
        Ok(VerifiedDefinitionalEquivalence {
            scope_digest: scope.digest().clone(),
            subject_digest,
            normalized_judgment: normalized,
        })
    }

    pub fn verify_closed_specialization_certificate(
        &self,
        scope: &TrustedScope,
        signature: &VerifiedSignature,
        expected_open: &OpenJudgment,
        expected_assignments: &[Term],
        certificate: &UncheckedClosedSpecializationCertificate,
    ) -> Result<VerifiedClosedSpecialization, CertificateError> {
        require_scope(
            scope,
            signature,
            CertificateClaim::ClosedSpecialization,
            &certificate.binding,
        )?;
        self.validate_certificate_judgments(&[
            expected_open,
            &certificate.open_judgment,
            &certificate.normalized_specialized_judgment,
        ])?;
        self.validate_certificate_terms(expected_assignments)?;
        self.validate_certificate_terms(&certificate.assignments)?;
        if certificate.open_judgment != *expected_open
            || certificate.assignments != expected_assignments
        {
            return Err(CertificateError::ClaimMismatch);
        }
        let subject_digest = self.closed_specialization_subject_digest(
            signature,
            expected_open,
            expected_assignments,
        )?;
        if certificate.subject_digest != subject_digest {
            return Err(CertificateError::SubjectMismatch);
        }
        let specialized =
            self.replay_closed_specialization(signature, expected_open, expected_assignments)?;
        if certificate.normalized_specialized_judgment != specialized {
            return Err(CertificateError::NormalFormMismatch);
        }
        Ok(VerifiedClosedSpecialization {
            scope_digest: scope.digest().clone(),
            subject_digest,
            normalized_specialized_judgment: specialized,
        })
    }

    pub fn verify_free_sealing_certificate(
        &self,
        scope: &TrustedScope,
        base: &VerifiedSignature,
        expected_extension: &UncheckedSignature,
        certificate: &UncheckedFreeSealingCertificate,
    ) -> Result<VerifiedFreeSealing, CertificateError> {
        require_scope(
            scope,
            base,
            CertificateClaim::FreeSealingFragment,
            &certificate.binding,
        )?;
        self.validate_certificate_signatures(&[
            expected_extension,
            &certificate.extension,
            &certificate.normalized_sealed_signature,
        ])?;
        if certificate.extension != *expected_extension {
            return Err(CertificateError::ClaimMismatch);
        }
        let subject_digest = self.sealing_subject_digest(base, expected_extension)?;
        if certificate.subject_digest != subject_digest {
            return Err(CertificateError::SubjectMismatch);
        }
        let sealed = self.verify_extension(base, expected_extension)?;
        if certificate.normalized_sealed_signature != sealed.normalized_wire() {
            return Err(CertificateError::SealedSignatureMismatch);
        }
        Ok(VerifiedFreeSealing {
            scope_digest: scope.digest().clone(),
            subject_digest,
            sealed_signature: sealed,
        })
    }
}

fn require_scope(
    expected: &TrustedScope,
    signature: &VerifiedSignature,
    claim: CertificateClaim,
    actual: &CertificateBinding,
) -> Result<(), CertificateError> {
    if expected.binding().claim != claim || actual != expected.binding() {
        return Err(CertificateError::ScopeMismatch);
    }
    if expected.binding().public_boundary_digest != *signature.digest() {
        return Err(CertificateError::BoundaryMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        CertificateClaim, ScopeInputs, TrustedScope, UncheckedClosedSpecializationCertificate,
        UncheckedDerivationCertificate, UncheckedEquivalenceCertificate,
        UncheckedFreeSealingCertificate,
    };
    use crate::{
        CertificateError, Declaration, DependentContext, Digest, GlobalId, Kernel, KernelError,
        KernelLimits, OpenJudgment, Term, UncheckedSignature,
    };

    fn digest(label: &[u8]) -> Digest {
        Digest::of_bytes(label)
    }

    fn scope(
        kernel: &Kernel,
        claim: CertificateClaim,
        boundary: &Digest,
        history: &[u8],
    ) -> TrustedScope {
        TrustedScope::new(
            kernel,
            claim,
            ScopeInputs {
                law_digest: digest(b"law"),
                grammar_digest: digest(b"grammar"),
                scheme_calculus_digest: digest(b"schemes"),
                blindness_contract_digest: digest(b"blindness"),
                bootstrap_contract_digest: digest(b"bootstrap"),
                history_digest: digest(history),
                public_boundary_digest: boundary.clone(),
                derivation_basis_digest: digest(b"basis"),
                active_window_digest: digest(b"window"),
                candidate_digest: digest(b"candidate"),
            },
        )
    }

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    fn empty_signature() -> crate::VerifiedSignature {
        kernel()
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature")
    }

    fn unit_judgment() -> OpenJudgment {
        OpenJudgment::HasType {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        }
    }

    #[test]
    fn derivation_replays_the_witness_not_just_its_digest() {
        let kernel = kernel();
        let signature = empty_signature();
        let expected = unit_judgment();
        let scope = scope(
            &kernel,
            CertificateClaim::Derivation,
            signature.digest(),
            b"history-a",
        );
        assert_eq!(
            scope.binding().kernel_digest,
            kernel.kernel_protocol_digest()
        );
        assert_eq!(
            scope.binding().normalizer_digest,
            kernel.normalizer_protocol_digest()
        );
        let normalized = kernel
            .verify_open_judgment(&signature, &expected)
            .expect("valid judgment");
        let mut certificate = UncheckedDerivationCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .judgment_subject_digest(&signature, &expected)
                .expect("subject digest"),
            judgment: expected.clone(),
            normalized_judgment: normalized,
        };
        kernel
            .verify_derivation_certificate(&scope, &signature, &expected, &certificate)
            .expect("certificate replays");

        certificate.normalized_judgment = OpenJudgment::HasType {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::Sort { level: 0 },
        };
        assert_eq!(
            kernel
                .verify_derivation_certificate(&scope, &signature, &expected, &certificate,)
                .expect_err("forged cached result"),
            CertificateError::NormalFormMismatch
        );
    }

    #[test]
    fn cross_history_replay_is_rejected() {
        let kernel = kernel();
        let signature = empty_signature();
        let expected = unit_judgment();
        let original = scope(
            &kernel,
            CertificateClaim::Derivation,
            signature.digest(),
            b"history-a",
        );
        let other = scope(
            &kernel,
            CertificateClaim::Derivation,
            signature.digest(),
            b"history-b",
        );
        let certificate = UncheckedDerivationCertificate {
            binding: original.binding().clone(),
            subject_digest: kernel
                .judgment_subject_digest(&signature, &expected)
                .expect("subject digest"),
            judgment: expected.clone(),
            normalized_judgment: kernel
                .verify_open_judgment(&signature, &expected)
                .expect("normal form"),
        };
        assert_eq!(
            kernel
                .verify_derivation_certificate(&other, &signature, &expected, &certificate,)
                .expect_err("wrong history"),
            CertificateError::ScopeMismatch
        );
    }

    #[test]
    fn recomputing_a_digest_does_not_change_the_expected_subject() {
        let kernel = kernel();
        let signature = empty_signature();
        let expected = unit_judgment();
        let scope = scope(
            &kernel,
            CertificateClaim::Derivation,
            signature.digest(),
            b"history",
        );
        let substituted = OpenJudgment::TypeFormation {
            context: DependentContext::default(),
            term: Term::UnitType,
        };
        let certificate = UncheckedDerivationCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .judgment_subject_digest(&signature, &substituted)
                .expect("subject digest"),
            judgment: substituted.clone(),
            normalized_judgment: kernel
                .verify_open_judgment(&signature, &substituted)
                .expect("normal substitute"),
        };
        assert_eq!(
            kernel
                .verify_derivation_certificate(&scope, &signature, &expected, &certificate,)
                .expect_err("subject substitution"),
            CertificateError::ClaimMismatch
        );
    }

    #[test]
    fn claim_kinds_cannot_be_substituted() {
        let kernel = kernel();
        let signature = empty_signature();
        let equality = OpenJudgment::DefinitionallyEqual {
            context: DependentContext::default(),
            left: Term::Unit,
            right: Term::Unit,
            ty: Term::UnitType,
        };
        let wrong_scope = scope(
            &kernel,
            CertificateClaim::Derivation,
            signature.digest(),
            b"history",
        );
        let certificate = UncheckedEquivalenceCertificate {
            binding: wrong_scope.binding().clone(),
            subject_digest: kernel
                .judgment_subject_digest(&signature, &equality)
                .expect("subject digest"),
            judgment: equality.clone(),
            normalized_judgment: kernel
                .verify_open_judgment(&signature, &equality)
                .expect("normal equality"),
        };
        assert_eq!(
            kernel
                .verify_equivalence_certificate(&wrong_scope, &signature, &equality, &certificate,)
                .expect_err("wrong capability kind"),
            CertificateError::ScopeMismatch
        );
    }

    #[test]
    fn closed_specialization_replays_dependent_substitution() {
        let kernel = kernel();
        let signature = empty_signature();
        let open = OpenJudgment::TypeFormation {
            context: DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]),
            term: Term::Var { index: 1 },
        };
        let assignments = vec![Term::UnitType, Term::Unit];
        let scope = scope(
            &kernel,
            CertificateClaim::ClosedSpecialization,
            signature.digest(),
            b"history",
        );
        let specialized = kernel
            .replay_closed_specialization(&signature, &open, &assignments)
            .expect("valid dependent specialization");
        assert_eq!(
            specialized,
            OpenJudgment::TypeFormation {
                context: DependentContext::default(),
                term: Term::UnitType,
            }
        );
        let certificate = UncheckedClosedSpecializationCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .closed_specialization_subject_digest(&signature, &open, &assignments)
                .expect("subject digest"),
            open_judgment: open.clone(),
            assignments: assignments.clone(),
            normalized_specialized_judgment: specialized,
        };
        kernel
            .verify_closed_specialization_certificate(
                &scope,
                &signature,
                &open,
                &assignments,
                &certificate,
            )
            .expect("certificate replay");

        let wrong_order = vec![Term::Unit, Term::UnitType];
        assert!(matches!(
            kernel.replay_closed_specialization(&signature, &open, &wrong_order),
            Err(KernelError::TypeMismatch)
        ));
    }

    #[test]
    fn exact_sealing_rejects_an_extra_declaration() {
        let kernel = kernel();
        let base = empty_signature();
        let extension = UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(b"owed")),
                ty: Term::UnitType,
                body: None,
            }],
        };
        let scope = scope(
            &kernel,
            CertificateClaim::FreeSealingFragment,
            base.digest(),
            b"history",
        );
        let mut claimed = kernel
            .verify_extension(&base, &extension)
            .expect("valid extension")
            .normalized_wire();
        claimed.declarations.push(Declaration {
            id: GlobalId(digest(b"extra")),
            ty: Term::UnitType,
            body: None,
        });
        let certificate = UncheckedFreeSealingCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .sealing_subject_digest(&base, &extension)
                .expect("subject digest"),
            extension: extension.clone(),
            normalized_sealed_signature: claimed,
        };
        assert_eq!(
            kernel
                .verify_free_sealing_certificate(&scope, &base, &extension, &certificate,)
                .expect_err("extra declaration"),
            CertificateError::SealedSignatureMismatch
        );
    }

    #[test]
    fn exact_sealing_accepts_only_the_reconstructed_normal_signature() {
        let kernel = kernel();
        let base = empty_signature();
        let extension = UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(b"owed")),
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
        let scope = scope(
            &kernel,
            CertificateClaim::FreeSealingFragment,
            base.digest(),
            b"history",
        );
        let normalized = kernel
            .verify_extension(&base, &extension)
            .expect("valid extension")
            .normalized_wire();
        let certificate = UncheckedFreeSealingCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .sealing_subject_digest(&base, &extension)
                .expect("subject digest"),
            extension: extension.clone(),
            normalized_sealed_signature: normalized,
        };
        let verified = kernel
            .verify_free_sealing_certificate(&scope, &base, &extension, &certificate)
            .expect("exact seal");
        assert_eq!(verified.sealed_signature().declarations().len(), 1);
    }

    #[test]
    fn unknown_certificate_fields_fail_deserialization() {
        let kernel = kernel();
        let signature = empty_signature();
        let expected = unit_judgment();
        let scope = scope(
            &kernel,
            CertificateClaim::Derivation,
            signature.digest(),
            b"history",
        );
        let certificate = UncheckedDerivationCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .judgment_subject_digest(&signature, &expected)
                .expect("subject digest"),
            judgment: expected.clone(),
            normalized_judgment: kernel
                .verify_open_judgment(&signature, &expected)
                .expect("normal"),
        };
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("trusted".to_owned(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<UncheckedDerivationCertificate>(value).is_err());
    }
}
