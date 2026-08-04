//! Isolated JG1 through particular-instance JG2b2b1 foundations for Law V2
//! generative capabilities.
//!
//! Semantic novelty and generative gain are independent audits. This crate
//! therefore does not depend on `pen-semantic-audit` and does not alter its
//! carrier, role vocabulary, or negative SR2 result.
//!
//! [`VerifiedPreExposureGenerativeCapabilityGrammarV1`] certifies the exact
//! JG1 role, obligation, and receipt vocabulary. JG2a separately freezes a
//! finite constructor/interface grammar and verifies exact public stage
//! boundaries by independent kernel replay. JG2b1a derives a contiguous
//! normalized chain and an exhaustive declaration-birth index relative only
//! to the supplied chain. JG2b1b1 adds a process-local consuming linear writer;
//! JG2b1b2 consumes its closed state, freshly reconstructs every stage and
//! JG2b1a, and certifies completeness through that producer-finalized head.
//! JG2b2a freezes the entry protocol separating a finite derived birth-
//! occurrence census from the generic typed-substitution theorem required for
//! universal naturality. Its remintable token carries no factual occurrence,
//! substitution, or naturality authority.
//! JG2b2b0 additionally freezes an exact history-independent structural
//! occurrence grammar and executable unverified declaration traversal.  Its
//! remintable token still carries no factual occurrence, typing, classifier,
//! substitution, or naturality authority.
//! JG2b2b1 freezes the particular open typed substitution protocol and can
//! retain full evidence for one two-batch checked context morphism, identity,
//! composite, dependent binder lift, or reindexed open judgment. These finite
//! instances do not establish generic substitution metatheory, identity or
//! composition laws, functoriality, or universal naturality.
//! None of these authorities mints a globally-latest/EOF fact, capability
//! member, carrier, quotient, marginal, provenance, `gamma`, bootstrap, or
//! selection result.

#![forbid(unsafe_code)]

mod complete_sealed_history;
mod constructor_grammar;
mod contiguous_stage_chain;
mod linear_append_log;
mod particular_open_typed_substitution;
mod stage_surface;
mod structural_occurrence_grammar;
mod substitution_naturality_protocol;

pub use complete_sealed_history::*;
pub use constructor_grammar::*;
pub use contiguous_stage_chain::*;
pub use linear_append_log::*;
pub use particular_open_typed_substitution::*;
pub use stage_surface::*;
pub use structural_occurrence_grammar::*;
pub use substitution_naturality_protocol::*;

use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};

pub const GENERATIVE_CAPABILITY_GRAMMAR_SCHEMA_VERSION_V1: u16 = 1;
pub const GENERATIVE_CAPABILITY_ROLE_COUNT_V1: usize = 7;
pub const GENERATIVE_CAPABILITY_ADMISSION_OBLIGATION_COUNT_V1: usize = 8;
pub const GENERATIVE_RECEIPT_SOURCE_FORM_COUNT_V1: usize = 2;

const GENERATIVE_CAPABILITY_GRAMMAR_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg1/pre-exposure-generative-capability-grammar/v1";
const GENERATIVE_CAPABILITY_GRAMMAR_ROOT_TAG_V1: u8 = 0xa1;

/// The exact finite role vocabulary for irreducible reusable operational
/// capabilities. There is intentionally no extensible or `Other` variant.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeCapabilityRoleV1 {
    Formation,
    Abstraction,
    Aggregation,
    Transport,
    Comparison,
    Compiler,
    DischargeTransformer,
}

pub const GENERATIVE_CAPABILITY_ROLES_V1: [GenerativeCapabilityRoleV1;
    GENERATIVE_CAPABILITY_ROLE_COUNT_V1] = [
    GenerativeCapabilityRoleV1::Formation,
    GenerativeCapabilityRoleV1::Abstraction,
    GenerativeCapabilityRoleV1::Aggregation,
    GenerativeCapabilityRoleV1::Transport,
    GenerativeCapabilityRoleV1::Comparison,
    GenerativeCapabilityRoleV1::Compiler,
    GenerativeCapabilityRoleV1::DischargeTransformer,
];

impl CanonicalEncode for GenerativeCapabilityRoleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Formation => 0,
            Self::Abstraction => 1,
            Self::Aggregation => 2,
            Self::Transport => 3,
            Self::Comparison => 4,
            Self::Compiler => 5,
            Self::DischargeTransformer => 6,
        });
    }
}

/// Every later capability member must discharge every obligation in this
/// closed list. JG1 freezes the obligations but proves no member satisfies
/// them.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeCapabilityAdmissionObligationV1 {
    StageGeneric,
    NaturalUnderEveryAdmissibleSubstitution,
    FinitePublicTypedInterface,
    CheckableFromPresentSealedGrammar,
    AbsentFromPredecessorCapabilityClosure,
    PresentAfterSealing,
    SupportedByNewPaidClauseOrPriorLiveGenerativeOutput,
    IndependentOfLaterCandidatesAndHeldOutEvidence,
}

pub const GENERATIVE_CAPABILITY_ADMISSION_OBLIGATIONS_V1:
    [GenerativeCapabilityAdmissionObligationV1; GENERATIVE_CAPABILITY_ADMISSION_OBLIGATION_COUNT_V1] = [
    GenerativeCapabilityAdmissionObligationV1::StageGeneric,
    GenerativeCapabilityAdmissionObligationV1::NaturalUnderEveryAdmissibleSubstitution,
    GenerativeCapabilityAdmissionObligationV1::FinitePublicTypedInterface,
    GenerativeCapabilityAdmissionObligationV1::CheckableFromPresentSealedGrammar,
    GenerativeCapabilityAdmissionObligationV1::AbsentFromPredecessorCapabilityClosure,
    GenerativeCapabilityAdmissionObligationV1::PresentAfterSealing,
    GenerativeCapabilityAdmissionObligationV1::SupportedByNewPaidClauseOrPriorLiveGenerativeOutput,
    GenerativeCapabilityAdmissionObligationV1::IndependentOfLaterCandidatesAndHeldOutEvidence,
];

impl CanonicalEncode for GenerativeCapabilityAdmissionObligationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::StageGeneric => 0,
            Self::NaturalUnderEveryAdmissibleSubstitution => 1,
            Self::FinitePublicTypedInterface => 2,
            Self::CheckableFromPresentSealedGrammar => 3,
            Self::AbsentFromPredecessorCapabilityClosure => 4,
            Self::PresentAfterSealing => 5,
            Self::SupportedByNewPaidClauseOrPriorLiveGenerativeOutput => 6,
            Self::IndependentOfLaterCandidatesAndHeldOutEvidence => 7,
        });
    }
}

/// Atomic factors from which the frozen JG7 receipt-source sum is formed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeReceiptSourceFactorV1 {
    NewPaidClause,
    GenerativeCapabilityRole,
    PriorLiveGenerativeRequirementOutput,
}

impl CanonicalEncode for GenerativeReceiptSourceFactorV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::NewPaidClause => 0,
            Self::GenerativeCapabilityRole => 1,
            Self::PriorLiveGenerativeRequirementOutput => 2,
        });
    }
}

/// The two disjoint provenance-source forms available to the later JG7
/// injection. A product contains exactly two typed factors by construction;
/// an atom contains exactly one. Verification freezes the sole product as
/// `NewPaidClause × GenerativeCapabilityRole` and the sole atom as a prior
/// live generative-requirement output.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeReceiptSourceFormV1 {
    Product([GenerativeReceiptSourceFactorV1; 2]),
    Atom(GenerativeReceiptSourceFactorV1),
}

pub const GENERATIVE_RECEIPT_SOURCE_FORMS_V1: [GenerativeReceiptSourceFormV1;
    GENERATIVE_RECEIPT_SOURCE_FORM_COUNT_V1] = [
    GenerativeReceiptSourceFormV1::Product([
        GenerativeReceiptSourceFactorV1::NewPaidClause,
        GenerativeReceiptSourceFactorV1::GenerativeCapabilityRole,
    ]),
    GenerativeReceiptSourceFormV1::Atom(
        GenerativeReceiptSourceFactorV1::PriorLiveGenerativeRequirementOutput,
    ),
];

impl GenerativeReceiptSourceFormV1 {
    pub fn capability_role_factor_count(self) -> usize {
        match self {
            Self::Product(factors) => factors
                .into_iter()
                .filter(|factor| {
                    *factor == GenerativeReceiptSourceFactorV1::GenerativeCapabilityRole
                })
                .count(),
            Self::Atom(factor) => {
                usize::from(factor == GenerativeReceiptSourceFactorV1::GenerativeCapabilityRole)
            }
        }
    }
}

impl CanonicalEncode for GenerativeReceiptSourceFormV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Product([left, right]) => {
                encoder.tag(0);
                left.encode_canonical(encoder);
                right.encode_canonical(encoder);
            }
            Self::Atom(factor) => {
                encoder.tag(1);
                factor.encode_canonical(encoder);
            }
        }
    }
}

/// Unverified JG1 input. Verification accepts only the exact proposal returned
/// by [`proposed_pre_exposure_generative_capability_grammar_v1`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreExposureGenerativeCapabilityGrammarManifestV1 {
    pub schema_version: u16,
    pub roles: Vec<GenerativeCapabilityRoleV1>,
    pub admission_obligations: Vec<GenerativeCapabilityAdmissionObligationV1>,
    pub receipt_source_forms: Vec<GenerativeReceiptSourceFormV1>,
}

impl CanonicalEncode for PreExposureGenerativeCapabilityGrammarManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_CAPABILITY_GRAMMAR_ROOT_TAG_V1);
        encoder.u16(self.schema_version);
        encoder.sequence(&self.roles);
        encoder.sequence(&self.admission_obligations);
        encoder.sequence(&self.receipt_source_forms);
    }
}

pub fn proposed_pre_exposure_generative_capability_grammar_v1()
-> PreExposureGenerativeCapabilityGrammarManifestV1 {
    PreExposureGenerativeCapabilityGrammarManifestV1 {
        schema_version: GENERATIVE_CAPABILITY_GRAMMAR_SCHEMA_VERSION_V1,
        roles: GENERATIVE_CAPABILITY_ROLES_V1.to_vec(),
        admission_obligations: GENERATIVE_CAPABILITY_ADMISSION_OBLIGATIONS_V1.to_vec(),
        receipt_source_forms: GENERATIVE_RECEIPT_SOURCE_FORMS_V1.to_vec(),
    }
}

/// Exact JG1 transcript. Pinning the bytes makes order and discriminants part
/// of the authority rather than a presentation convention.
pub const CANONICAL_PRE_EXPOSURE_GENERATIVE_CAPABILITY_GRAMMAR_BYTES_V1: &[u8] = &[
    0xa1, 0x01, 0x00, // root tag and schema version
    0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // seven roles
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // eight obligations
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // two receipt forms
    0x00, 0x00, 0x01, // product: new paid clause × exactly one capability role
    0x01, 0x02, // atom: prior live generative-requirement output
];

/// Frozen domain-separated digest of the canonical JG1 transcript.
pub const CANONICAL_PRE_EXPOSURE_GENERATIVE_CAPABILITY_GRAMMAR_DIGEST_V1: &str =
    "blake3:eee311f5f8c6f73d41e8b2d3dca0bc4bcc9c2e736c63a730073cc75acc1e0192";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerativeCapabilityGrammarFailureV1 {
    SchemaVersionMismatch,
    RoleVocabularyMismatch,
    AdmissionObligationVocabularyMismatch,
    ReceiptSourceVocabularyMismatch,
    CanonicalTranscriptMismatch,
    CanonicalDigestMismatch,
}

impl std::fmt::Display for GenerativeCapabilityGrammarFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SchemaVersionMismatch => "the JG1 schema version is not the frozen version",
            Self::RoleVocabularyMismatch => {
                "the generative-capability roles are not the exact frozen vocabulary and order"
            }
            Self::AdmissionObligationVocabularyMismatch => {
                "the admission obligations are not the exact frozen vocabulary and order"
            }
            Self::ReceiptSourceVocabularyMismatch => {
                "the receipt-source forms are not the exact frozen vocabulary and order"
            }
            Self::CanonicalTranscriptMismatch => {
                "the typed JG1 grammar does not encode to the frozen canonical transcript"
            }
            Self::CanonicalDigestMismatch => {
                "the canonical JG1 transcript does not have the frozen domain-separated digest"
            }
        })
    }
}

impl std::error::Error for GenerativeCapabilityGrammarFailureV1 {}

/// Opaque proof that the exact target-neutral JG1 grammar was verified.
///
/// This type cannot be constructed by callers. Its authority stops at the
/// grammar boundary described in the crate-level documentation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPreExposureGenerativeCapabilityGrammarV1 {
    schema_version: u16,
    manifest_digest: Digest,
}

impl VerifiedPreExposureGenerativeCapabilityGrammarV1 {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn roles(&self) -> &'static [GenerativeCapabilityRoleV1] {
        &GENERATIVE_CAPABILITY_ROLES_V1
    }

    pub fn admission_obligations(&self) -> &'static [GenerativeCapabilityAdmissionObligationV1] {
        &GENERATIVE_CAPABILITY_ADMISSION_OBLIGATIONS_V1
    }

    pub fn receipt_source_forms(&self) -> &'static [GenerativeReceiptSourceFormV1] {
        &GENERATIVE_RECEIPT_SOURCE_FORMS_V1
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }
}

pub fn verify_pre_exposure_generative_capability_grammar_v1(
    manifest: &PreExposureGenerativeCapabilityGrammarManifestV1,
) -> Result<VerifiedPreExposureGenerativeCapabilityGrammarV1, GenerativeCapabilityGrammarFailureV1>
{
    if manifest.schema_version != GENERATIVE_CAPABILITY_GRAMMAR_SCHEMA_VERSION_V1 {
        return Err(GenerativeCapabilityGrammarFailureV1::SchemaVersionMismatch);
    }
    if manifest.roles.as_slice() != GENERATIVE_CAPABILITY_ROLES_V1 {
        return Err(GenerativeCapabilityGrammarFailureV1::RoleVocabularyMismatch);
    }
    if manifest.admission_obligations.as_slice() != GENERATIVE_CAPABILITY_ADMISSION_OBLIGATIONS_V1 {
        return Err(GenerativeCapabilityGrammarFailureV1::AdmissionObligationVocabularyMismatch);
    }
    if manifest.receipt_source_forms.as_slice() != GENERATIVE_RECEIPT_SOURCE_FORMS_V1 {
        return Err(GenerativeCapabilityGrammarFailureV1::ReceiptSourceVocabularyMismatch);
    }

    let mut encoder = CanonicalEncoder::new();
    manifest.encode_canonical(&mut encoder);
    if encoder.as_bytes() != CANONICAL_PRE_EXPOSURE_GENERATIVE_CAPABILITY_GRAMMAR_BYTES_V1 {
        return Err(GenerativeCapabilityGrammarFailureV1::CanonicalTranscriptMismatch);
    }

    let manifest_digest = Digest::of_domain_bytes(
        GENERATIVE_CAPABILITY_GRAMMAR_DIGEST_DOMAIN_V1,
        encoder.as_bytes(),
    );
    if manifest_digest.as_str() != CANONICAL_PRE_EXPOSURE_GENERATIVE_CAPABILITY_GRAMMAR_DIGEST_V1 {
        return Err(GenerativeCapabilityGrammarFailureV1::CanonicalDigestMismatch);
    }

    Ok(VerifiedPreExposureGenerativeCapabilityGrammarV1 {
        schema_version: manifest.schema_version,
        manifest_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn verify_proposal() -> VerifiedPreExposureGenerativeCapabilityGrammarV1 {
        verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("the exact JG1 proposal must verify")
    }

    #[test]
    fn exact_closed_vocabularies_verify_and_are_exhaustive() {
        let verified = verify_proposal();

        assert_eq!(verified.schema_version(), 1);
        assert_eq!(verified.roles(), GENERATIVE_CAPABILITY_ROLES_V1);
        assert_eq!(
            verified.admission_obligations(),
            GENERATIVE_CAPABILITY_ADMISSION_OBLIGATIONS_V1
        );
        assert_eq!(
            verified.receipt_source_forms(),
            GENERATIVE_RECEIPT_SOURCE_FORMS_V1
        );
        assert_eq!(
            verified
                .roles()
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            GENERATIVE_CAPABILITY_ROLE_COUNT_V1
        );
        assert_eq!(
            verified
                .admission_obligations()
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            GENERATIVE_CAPABILITY_ADMISSION_OBLIGATION_COUNT_V1
        );
    }

    #[test]
    fn receipt_sum_roles_only_the_paid_clause_summand() {
        assert_eq!(
            GENERATIVE_RECEIPT_SOURCE_FORMS_V1[0],
            GenerativeReceiptSourceFormV1::Product([
                GenerativeReceiptSourceFactorV1::NewPaidClause,
                GenerativeReceiptSourceFactorV1::GenerativeCapabilityRole,
            ])
        );
        assert_eq!(
            GENERATIVE_RECEIPT_SOURCE_FORMS_V1[0].capability_role_factor_count(),
            1
        );
        assert_eq!(
            GENERATIVE_RECEIPT_SOURCE_FORMS_V1[1],
            GenerativeReceiptSourceFormV1::Atom(
                GenerativeReceiptSourceFactorV1::PriorLiveGenerativeRequirementOutput
            )
        );
        assert_eq!(
            GENERATIVE_RECEIPT_SOURCE_FORMS_V1[1].capability_role_factor_count(),
            0
        );
    }

    #[test]
    fn canonical_transcript_and_digest_are_deterministic() {
        let proposal = proposed_pre_exposure_generative_capability_grammar_v1();
        let mut encoder = CanonicalEncoder::new();
        proposal.encode_canonical(&mut encoder);
        assert_eq!(
            encoder.as_bytes(),
            CANONICAL_PRE_EXPOSURE_GENERATIVE_CAPABILITY_GRAMMAR_BYTES_V1
        );

        let expected_digest = Digest::of_domain_bytes(
            GENERATIVE_CAPABILITY_GRAMMAR_DIGEST_DOMAIN_V1,
            CANONICAL_PRE_EXPOSURE_GENERATIVE_CAPABILITY_GRAMMAR_BYTES_V1,
        );
        assert_eq!(
            expected_digest.as_str(),
            CANONICAL_PRE_EXPOSURE_GENERATIVE_CAPABILITY_GRAMMAR_DIGEST_V1
        );

        let first = verify_proposal();
        let second = verify_proposal();
        assert_eq!(first.manifest_digest(), second.manifest_digest());
        assert_eq!(first.manifest_digest(), &expected_digest);
    }

    #[test]
    fn every_exact_vocabulary_rejects_mutation() {
        let proposal = proposed_pre_exposure_generative_capability_grammar_v1();

        let mut schema_mutation = proposal.clone();
        schema_mutation.schema_version += 1;
        assert_eq!(
            verify_pre_exposure_generative_capability_grammar_v1(&schema_mutation),
            Err(GenerativeCapabilityGrammarFailureV1::SchemaVersionMismatch)
        );

        let mut role_mutation = proposal.clone();
        role_mutation.roles.swap(0, 1);
        assert_eq!(
            verify_pre_exposure_generative_capability_grammar_v1(&role_mutation),
            Err(GenerativeCapabilityGrammarFailureV1::RoleVocabularyMismatch)
        );

        let mut obligation_mutation = proposal.clone();
        obligation_mutation.admission_obligations.pop();
        assert_eq!(
            verify_pre_exposure_generative_capability_grammar_v1(&obligation_mutation),
            Err(GenerativeCapabilityGrammarFailureV1::AdmissionObligationVocabularyMismatch)
        );

        let mut receipt_mutation = proposal;
        receipt_mutation.receipt_source_forms[0] =
            GenerativeReceiptSourceFormV1::Atom(GenerativeReceiptSourceFactorV1::NewPaidClause);
        assert_eq!(
            verify_pre_exposure_generative_capability_grammar_v1(&receipt_mutation),
            Err(GenerativeCapabilityGrammarFailureV1::ReceiptSourceVocabularyMismatch)
        );
    }
}
