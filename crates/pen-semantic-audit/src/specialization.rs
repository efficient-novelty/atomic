//! Replay-checked family specialization.
//!
//! A serialized [`FamilyInstanceV1`] is only an untrusted claim.  This module
//! resolves its originating quotient class, asks the kernel to perform the
//! complete dependent substitution, and mints a private capability only when
//! the claimed specialized judgment is exactly the replayed normal result.

use crate::carrier::{generic_to_open, open_to_generic, substitution_action_digest};
use crate::manifest::{AuditDecision, AuditUnknownReason, VerifiedSemanticAuditManifestV1};
use crate::model::{FamilyClassIdV1, FamilyInstanceV1, GenericJudgmentV1};
use crate::quotient::QuotientCertificateV1;
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Digest, Kernel, KernelError, Term, VerifiedSignature,
};

/// A kernel-replayed specialization of exactly one quotient family.
///
/// Fields are private and this type deliberately has no `Deserialize`
/// implementation.  Callers can obtain it only through
/// [`verify_family_instance_v1`].
#[derive(Clone, Debug)]
pub struct VerifiedFamilyInstanceV1 {
    manifest_digest: Digest,
    signature_digest: Digest,
    quotient_digest: Digest,
    originating_class: FamilyClassIdV1,
    substitution: Vec<Term>,
    specialized_judgment: GenericJudgmentV1,
    instance_digest: Digest,
}

impl VerifiedFamilyInstanceV1 {
    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn quotient_digest(&self) -> &Digest {
        &self.quotient_digest
    }

    pub fn originating_class(&self) -> &FamilyClassIdV1 {
        &self.originating_class
    }

    pub fn substitution(&self) -> &[Term] {
        &self.substitution
    }

    pub fn specialized_judgment(&self) -> &GenericJudgmentV1 {
        &self.specialized_judgment
    }

    pub fn instance_digest(&self) -> &Digest {
        &self.instance_digest
    }
}

impl CanonicalEncode for VerifiedFamilyInstanceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.quotient_digest.encode_canonical(encoder);
        self.originating_class.encode_canonical(encoder);
        encoder.sequence(&self.substitution);
        self.specialized_judgment.encode_canonical(encoder);
        self.instance_digest.encode_canonical(encoder);
    }
}

/// Verify that an instance is a complete typed substitution of its claimed
/// originating family.  Specialization never creates a new family identity.
pub fn verify_family_instance_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    quotient: &QuotientCertificateV1,
    claimed: &FamilyInstanceV1,
) -> AuditDecision<VerifiedFamilyInstanceV1> {
    if quotient.manifest_digest != *manifest.candidate_digest()
        || quotient.signature_digest != *signature.digest()
        || quotient.normalizer_protocol_digest != kernel.normalizer_protocol_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }

    let Some(class) = quotient
        .classes
        .iter()
        .find(|class| class.id == claimed.originating_class)
    else {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    };
    if quotient
        .classes
        .iter()
        .filter(|candidate| candidate.id == claimed.originating_class)
        .count()
        != 1
        || class.substitution_action_digest
            != substitution_action_digest(&class.generic_judgment, class.role)
    {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }

    let replayed = match kernel.normalize_closed_specialization(
        signature,
        &generic_to_open(&class.generic_judgment),
        &claimed.substitution,
    ) {
        Ok(judgment) => open_to_generic(judgment),
        Err(KernelError::ResourceExhausted(_)) => {
            return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
        }
        Err(KernelError::SubstitutionArity) => {
            return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
        }
        Err(_) => {
            return AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify);
        }
    };
    if replayed != claimed.specialized_judgment {
        return AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure);
    }

    let quotient_digest =
        Digest::of_canonical("pen-semantic-audit/quotient-certificate/v1", quotient);
    let instance_digest =
        Digest::of_canonical("pen-semantic-audit/verified-family-instance/v1", claimed);
    AuditDecision::Proven(VerifiedFamilyInstanceV1 {
        manifest_digest: manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        quotient_digest,
        originating_class: claimed.originating_class.clone(),
        substitution: claimed.substitution.clone(),
        specialized_judgment: replayed,
        instance_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::carrier::enumerate_raw_families_v1;
    use crate::manifest::{proposed_semantic_audit_manifest_v1, verify_semantic_audit_manifest_v1};
    use crate::model::{
        EventIdV1, HeadPresentationV1, LocalRoleV1, PublicHeadSeedV1, PublicSupportV1,
        SemanticSchemaSeedV1, SourceNormalizedJudgmentV1,
    };
    use crate::quotient::quotient_families_v1;
    use pen_kernel::{Declaration, DependentContext, GlobalId, KernelLimits, UncheckedSignature};

    fn digest_id(label: &[u8]) -> Digest {
        Digest::of_bytes(label)
    }

    fn fixture() -> (
        Kernel,
        VerifiedSignature,
        VerifiedSemanticAuditManifestV1,
        QuotientCertificateV1,
    ) {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let head = GlobalId(digest_id(b"specialization/head"));
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
        let generic = GenericJudgmentV1::Term {
            context: DependentContext(vec![Term::Sort { level: 0 }]),
            term: Term::Apply {
                function: Box::new(Term::Global { id: head.clone() }),
                argument: Box::new(Term::Var { index: 0 }),
            },
            ty: Term::Sort { level: 0 },
        };
        let seed = SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
            declaration: head,
            origin_event: EventIdV1(digest_id(b"specialization/event")),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: digest_id(b"specialization/source"),
                source: generic.clone(),
                claimed_normalized: generic,
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
        let AuditDecision::Proven(quotient) =
            quotient_families_v1(&kernel, &signature, &manifest, &carrier)
        else {
            panic!("quotient");
        };
        (kernel, signature, manifest, quotient)
    }

    fn claimed_instance(
        kernel: &Kernel,
        signature: &VerifiedSignature,
        quotient: &QuotientCertificateV1,
        assignment: Term,
    ) -> FamilyInstanceV1 {
        let class = quotient.classes().first().expect("one class");
        let specialized = kernel
            .normalize_closed_specialization(
                signature,
                &generic_to_open(&class.generic_judgment),
                std::slice::from_ref(&assignment),
            )
            .expect("specialization");
        FamilyInstanceV1 {
            originating_class: class.id.clone(),
            substitution: vec![assignment],
            specialized_judgment: open_to_generic(specialized),
        }
    }

    #[test]
    fn distinct_specializations_retain_one_originating_family() {
        let (kernel, signature, manifest, quotient) = fixture();
        let first = claimed_instance(&kernel, &signature, &quotient, Term::UnitType);
        let second = claimed_instance(
            &kernel,
            &signature,
            &quotient,
            Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            },
        );

        let AuditDecision::Proven(first_verified) =
            verify_family_instance_v1(&kernel, &signature, &manifest, &quotient, &first)
        else {
            panic!("first instance");
        };
        let AuditDecision::Proven(second_verified) =
            verify_family_instance_v1(&kernel, &signature, &manifest, &quotient, &second)
        else {
            panic!("second instance");
        };
        assert_eq!(
            first_verified.originating_class(),
            second_verified.originating_class()
        );
        assert_ne!(
            first_verified.specialized_judgment(),
            second_verified.specialized_judgment()
        );
    }

    #[test]
    fn wrong_class_arity_and_specialized_payload_fail_closed() {
        let (kernel, signature, manifest, quotient) = fixture();
        let mut claimed = claimed_instance(&kernel, &signature, &quotient, Term::UnitType);

        claimed.originating_class = FamilyClassIdV1(digest_id(b"specialization/not-a-class"));
        assert!(matches!(
            verify_family_instance_v1(&kernel, &signature, &manifest, &quotient, &claimed),
            AuditDecision::Unknown(AuditUnknownReason::MalformedInput)
        ));

        claimed.originating_class = quotient.classes()[0].id.clone();
        claimed.substitution.clear();
        assert!(matches!(
            verify_family_instance_v1(&kernel, &signature, &manifest, &quotient, &claimed),
            AuditDecision::Unknown(AuditUnknownReason::MalformedInput)
        ));

        claimed.substitution.push(Term::UnitType);
        claimed.specialized_judgment = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        };
        assert!(matches!(
            verify_family_instance_v1(&kernel, &signature, &manifest, &quotient, &claimed),
            AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure)
        ));
    }
}
