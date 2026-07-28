//! Wire schemas for later Law V2 proof layers.
//!
//! Every type in this module is explicitly unchecked. These records preserve
//! the witnesses a future verifier must replay; they confer no acceptance,
//! completeness, discharge, or halt authority by deserialization alone.
//! Canonical ID and subject-digest helpers in this unchecked layer do not
//! perform resource accounting. Artifact loaders must impose byte, node, and
//! recursion bounds before calling them; the resulting digest is never a
//! verified capability.

use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, CertificateBinding, DependentContext, Digest, GlobalId,
    OpenJudgment, Term, UncheckedClosedSpecializationCertificate, UncheckedDerivationCertificate,
    UncheckedEquivalenceCertificate, UncheckedFreeSealingCertificate, UncheckedSignature,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const LAW_CERTIFICATE_SCHEMA_VERSION: u16 = 1;
pub const LAW_CERTIFICATE_CODEC_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LawArtifactKind {
    History,
    Candidate,
    DemandScheme,
    ClCore,
    DemandCensus,
    Discharge,
    Halt,
}

impl CanonicalEncode for LawArtifactKind {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::History => 0,
            Self::Candidate => 1,
            Self::DemandScheme => 2,
            Self::ClCore => 3,
            Self::DemandCensus => 4,
            Self::Discharge => 5,
            Self::Halt => 6,
        });
    }
}

/// Context binding repeated on every law-level wire certificate. A later
/// verifier must compare this with locally trusted inputs, not with another
/// field supplied by the artifact.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LawArtifactBinding {
    pub schema_version: u16,
    pub codec_version: u16,
    pub kind: LawArtifactKind,
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

impl CanonicalEncode for LawArtifactBinding {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.u16(self.codec_version);
        self.kind.encode_canonical(encoder);
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

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct EventId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct ObligationId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct RequiredOutputId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct SchemaFamilyId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct SchemaInstanceId(pub Digest);

macro_rules! encode_digest_id {
    ($($name:ty),+ $(,)?) => {
        $(
            impl CanonicalEncode for $name {
                fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                    self.0.encode_canonical(encoder);
                }
            }
        )+
    };
}

encode_digest_id!(
    EventId,
    ObligationId,
    RequiredOutputId,
    SchemaFamilyId,
    SchemaInstanceId,
);

impl EventId {
    pub fn for_sealed_extension(pre_extension_history: &Digest, sealing_subject: &Digest) -> Self {
        let mut encoder = CanonicalEncoder::new();
        pre_extension_history.encode_canonical(&mut encoder);
        sealing_subject.encode_canonical(&mut encoder);
        Self(Digest::of_domain_bytes(
            "law-v2-sealed-event-id-v1",
            encoder.as_bytes(),
        ))
    }
}

impl RequiredOutputId {
    pub fn for_judgment(judgment: &OpenJudgment) -> Self {
        Self(Digest::of_canonical(
            "law-v2-required-output-id-v1",
            judgment,
        ))
    }
}

impl SchemaFamilyId {
    pub fn for_subject(parameters: &DependentContext, body: &OpenJudgment) -> Self {
        let mut encoder = CanonicalEncoder::new();
        parameters.encode_canonical(&mut encoder);
        body.encode_canonical(&mut encoder);
        Self(Digest::of_domain_bytes(
            "law-v2-schema-family-id-v1",
            encoder.as_bytes(),
        ))
    }
}

impl SchemaInstanceId {
    pub fn for_subject(
        family: &SchemaFamilyId,
        assignments: &[Term],
        specialized_body: &OpenJudgment,
    ) -> Self {
        let mut encoder = CanonicalEncoder::new();
        family.encode_canonical(&mut encoder);
        encoder.sequence(assignments);
        specialized_body.encode_canonical(&mut encoder);
        Self(Digest::of_domain_bytes(
            "law-v2-schema-instance-id-v1",
            encoder.as_bytes(),
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NormalType {
    pub term: Term,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveWindow {
    /// Newest event first. The verifier derives and enforces the fixed width.
    pub events: Vec<EventId>,
    pub boundary_digests: Vec<Digest>,
}

impl CanonicalEncode for ActiveWindow {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.events);
        encoder.sequence(&self.boundary_digests);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DerivationSchema {
    pub family_id: SchemaFamilyId,
    pub parameters: DependentContext,
    pub conclusion: OpenJudgment,
}

impl CanonicalEncode for DerivationSchema {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.family_id.encode_canonical(encoder);
        self.parameters.encode_canonical(encoder);
        self.conclusion.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedSealedExtension {
    /// The seal must bind this predecessor history, never the containing
    /// history whose digest is being computed.
    pub pre_extension_history_digest: Digest,
    pub event_id: EventId,
    pub extension: UncheckedSignature,
    pub sealing: UncheckedFreeSealingCertificate,
}

impl CanonicalEncode for UncheckedSealedExtension {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.pre_extension_history_digest.encode_canonical(encoder);
        self.event_id.encode_canonical(encoder);
        self.extension.encode_canonical(encoder);
        encode_free_sealing_certificate(&self.sealing, encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedHistory {
    pub binding: LawArtifactBinding,
    pub extensions: Vec<UncheckedSealedExtension>,
    pub public_boundary: UncheckedSignature,
    pub derivation_basis: Vec<DerivationSchema>,
    pub active_window: ActiveWindow,
}

impl UncheckedHistory {
    /// Canonical history subject identity. The outer binding is intentionally
    /// excluded so `binding.history_digest` can be compared without a cycle.
    pub fn subject_digest(&self) -> Digest {
        let mut encoder = CanonicalEncoder::new();
        encoder.sequence(&self.extensions);
        self.public_boundary.encode_canonical(&mut encoder);
        encoder.sequence(&self.derivation_basis);
        self.active_window.encode_canonical(&mut encoder);
        Digest::of_domain_bytes("law-v2-history-subject-v1", encoder.as_bytes())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryEntry {
    pub old: GlobalId,
    pub extended: GlobalId,
}

impl CanonicalEncode for BoundaryEntry {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.old.encode_canonical(encoder);
        self.extended.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryInclusion {
    pub entries: Vec<BoundaryEntry>,
}

impl CanonicalEncode for BoundaryInclusion {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.entries);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RequiredOutput {
    pub id: RequiredOutputId,
    pub judgment: OpenJudgment,
}

impl CanonicalEncode for RequiredOutput {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.judgment.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrimitiveObligation {
    pub id: ObligationId,
    pub support: Vec<EventId>,
    pub required_outputs: Vec<RequiredOutput>,
}

impl CanonicalEncode for PrimitiveObligation {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        encoder.sequence(&self.support);
        encoder.sequence(&self.required_outputs);
    }
}

impl ObligationId {
    pub fn for_subject(support: &[EventId], required_outputs: &[RequiredOutput]) -> Self {
        let mut encoder = CanonicalEncoder::new();
        encoder.sequence(support);
        encoder.sequence(required_outputs);
        Self(Digest::of_domain_bytes(
            "law-v2-obligation-id-v1",
            encoder.as_bytes(),
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationLayer {
    pub obligations: Vec<PrimitiveObligation>,
}

impl CanonicalEncode for ObligationLayer {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.obligations);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObligationComplex {
    /// Depth is represented structurally. A future checker rejects a complex
    /// with more layers than the constitutive bound.
    pub layers: Vec<ObligationLayer>,
}

impl CanonicalEncode for ObligationComplex {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.layers);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedCandidate {
    pub binding: LawArtifactBinding,
    pub boundary: UncheckedSignature,
    pub realization: OpenJudgment,
    pub induced_obligations: ObligationComplex,
    pub boundary_inclusion: BoundaryInclusion,
}

impl UncheckedCandidate {
    /// Canonical candidate subject identity. The binding is excluded so its
    /// `candidate_digest` field can be verified without self-reference.
    pub fn subject_digest(&self) -> Digest {
        let mut encoder = CanonicalEncoder::new();
        self.boundary.encode_canonical(&mut encoder);
        self.realization.encode_canonical(&mut encoder);
        self.induced_obligations.encode_canonical(&mut encoder);
        self.boundary_inclusion.encode_canonical(&mut encoder);
        Digest::of_domain_bytes("law-v2-candidate-subject-v1", encoder.as_bytes())
    }
}

fn encode_free_sealing_certificate(
    certificate: &UncheckedFreeSealingCertificate,
    encoder: &mut CanonicalEncoder,
) {
    certificate.binding.encode_canonical(encoder);
    certificate.subject_digest.encode_canonical(encoder);
    certificate.extension.encode_canonical(encoder);
    certificate
        .normalized_sealed_signature
        .encode_canonical(encoder);
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "support_rule", deny_unknown_fields)]
pub enum SupportRule {
    ExactEvents { events: Vec<EventId> },
    ContextProjection { parameter_indices: Vec<u32> },
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedSubstitutionWitness {
    /// Entries are checked sequentially against the declared dependent context.
    pub assignments: Vec<Term>,
    pub typing: Vec<UncheckedDerivationCertificate>,
    pub specialized_body: OpenJudgment,
    /// Available only when the substitution closes the full dependent
    /// context. Open/support-context specialization remains unsupported.
    pub closed_replay: Option<UncheckedClosedSpecializationCertificate>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedTotalSpecializationCertificate {
    pub registration_derivation: UncheckedDerivationCertificate,
    pub rule_binding: CertificateBinding,
    pub generic_substitution: TypedSubstitutionWitness,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedDemandScheme {
    pub binding: LawArtifactBinding,
    pub family_id: SchemaFamilyId,
    pub parameters: DependentContext,
    pub motive: NormalType,
    pub open_body: OpenJudgment,
    pub support: SupportRule,
    pub total_specialization: UncheckedTotalSpecializationCertificate,
}

/// A family and a specialization are different wire types, so an instance
/// cannot be accidentally inserted into the family register.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedSchemaFamily {
    pub id: SchemaFamilyId,
    pub parameters: DependentContext,
    pub normal_body: OpenJudgment,
    pub naturality: Vec<UncheckedEquivalenceCertificate>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedSchemaInstance {
    pub id: SchemaInstanceId,
    pub family_id: SchemaFamilyId,
    pub substitution: TypedSubstitutionWitness,
    pub specialized_derivation: UncheckedDerivationCertificate,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InducedDischargeWitness {
    pub obligation: ObligationId,
    pub output: RequiredOutputId,
    /// The path length, not a claimed integer, determines constitutive depth.
    pub obligation_path: Vec<ObligationId>,
    pub derivation: UncheckedDerivationCertificate,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EquivarianceWitness {
    pub source_candidate_digest: Digest,
    pub transported_candidate_digest: Digest,
    pub transport: Vec<UncheckedEquivalenceCertificate>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DemandConnectionWitness {
    pub declaration: GlobalId,
    pub obligation_path: Vec<ObligationId>,
}

/// This is only the storage shape for the replayable CL components. There is
/// deliberately no verified CL, accepted candidate, or boolean shortcut.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedClCoreCertificate {
    pub binding: LawArtifactBinding,
    pub candidate: UncheckedCandidate,
    pub boundary_typing: Vec<UncheckedDerivationCertificate>,
    pub realization: UncheckedDerivationCertificate,
    pub equivalence_invariance: Vec<EquivarianceWitness>,
    pub local_satisfiability: Vec<InducedDischargeWitness>,
    pub sealing_fragment: UncheckedFreeSealingCertificate,
    pub exported_canonicity: Vec<UncheckedDerivationCertificate>,
    pub demand_connections: Vec<DemandConnectionWitness>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "disposition", deny_unknown_fields)]
pub enum UncheckedDerivabilityDisposition {
    Derived {
        derivation: Box<UncheckedDerivationCertificate>,
    },
    Undischarged {
        finite_universe: Vec<OpenJudgment>,
        saturation_layers: Vec<Vec<OpenJudgment>>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchemeGroundingWitness {
    pub scheme: SchemaFamilyId,
    pub substitution: TypedSubstitutionWitness,
    pub instance: UncheckedSchemaInstance,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtractionCoverageWitness {
    pub scheme: SchemaFamilyId,
    pub admissible_groundings: Vec<SchemeGroundingWitness>,
    pub rejected_assignments: Vec<TypedSubstitutionWitness>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstanceDecision {
    pub instance: UncheckedSchemaInstance,
    pub disposition: UncheckedDerivabilityDisposition,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "transition", deny_unknown_fields)]
pub enum ExpirationWitness {
    Transported {
        prior: SchemaInstanceId,
        current: SchemaInstanceId,
        weakening: UncheckedDerivationCertificate,
    },
    Discharged {
        prior: SchemaInstanceId,
        derivation: UncheckedDerivationCertificate,
    },
    Expired {
        prior: SchemaInstanceId,
        support_events: Vec<EventId>,
    },
}

/// Raw three-part census record. Completeness is not inferred from vector
/// length; a future verifier must replay all rule groundings and saturation.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedDemandCensusCertificate {
    pub binding: LawArtifactBinding,
    pub registered_schemes: Vec<UncheckedDemandScheme>,
    pub extraction_coverage: Vec<ExtractionCoverageWitness>,
    pub decisions: Vec<InstanceDecision>,
    pub locality_and_expiration: Vec<ExpirationWitness>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutputDischargeWitness {
    pub output: RequiredOutputId,
    pub substitution: TypedSubstitutionWitness,
    pub derivation: UncheckedDerivationCertificate,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedDischargeCertificate {
    pub binding: LawArtifactBinding,
    pub required_outputs: Vec<RequiredOutput>,
    pub coverage: Vec<OutputDischargeWitness>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FalsifierDisposition {
    pub challenge: OpenJudgment,
    pub decision: UncheckedDerivabilityDisposition,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticAuditReference {
    pub candidate_digest: Digest,
    pub audit_artifact_digest: Digest,
}

/// Storage schema only. No code in Phase 2 can verify or issue halt authority.
/// A later verifier must derive emptiness from the full census rather than
/// trust a count or flag.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedHaltCertificate {
    pub binding: LawArtifactBinding,
    pub census: UncheckedDemandCensusCertificate,
    pub decided_instances: Vec<InstanceDecision>,
    pub falsifier: FalsifierDisposition,
    pub semantic_provenance: SemanticAuditReference,
}

#[cfg(test)]
mod tests {
    use super::{
        BoundaryInclusion, LAW_CERTIFICATE_CODEC_VERSION, LAW_CERTIFICATE_SCHEMA_VERSION,
        LawArtifactBinding, LawArtifactKind, ObligationComplex, SchemaFamilyId, SchemaInstanceId,
        UncheckedCandidate, UncheckedSchemaFamily,
    };
    use pen_kernel::{DependentContext, Digest, OpenJudgment, Term, UncheckedSignature};

    fn digest(seed: &[u8]) -> Digest {
        Digest::of_bytes(seed)
    }

    fn binding() -> LawArtifactBinding {
        LawArtifactBinding {
            schema_version: LAW_CERTIFICATE_SCHEMA_VERSION,
            codec_version: LAW_CERTIFICATE_CODEC_VERSION,
            kind: LawArtifactKind::DemandCensus,
            kernel_digest: digest(b"kernel"),
            normalizer_digest: digest(b"normalizer"),
            law_digest: digest(b"law"),
            grammar_digest: digest(b"grammar"),
            scheme_calculus_digest: digest(b"schemes"),
            blindness_contract_digest: digest(b"blindness"),
            bootstrap_contract_digest: digest(b"bootstrap"),
            history_digest: digest(b"history"),
            public_boundary_digest: digest(b"boundary"),
            derivation_basis_digest: digest(b"basis"),
            active_window_digest: digest(b"window"),
            candidate_digest: digest(b"candidate"),
        }
    }

    #[test]
    fn law_bindings_reject_unknown_fields() {
        let mut value = serde_json::to_value(binding()).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("accepted".to_owned(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<LawArtifactBinding>(value).is_err());
    }

    #[test]
    fn family_wire_has_no_instance_count_or_validity_flag() {
        let family = UncheckedSchemaFamily {
            id: SchemaFamilyId(digest(b"family")),
            parameters: DependentContext::default(),
            normal_body: OpenJudgment::TypeFormation {
                context: DependentContext::default(),
                term: Term::UnitType,
            },
            naturality: Vec::new(),
        };
        let value = serde_json::to_value(family).expect("serialize");
        let object = value.as_object().expect("object");
        assert!(!object.contains_key("count"));
        assert!(!object.contains_key("valid"));
    }

    #[test]
    fn candidate_subject_digest_excludes_its_self_binding() {
        let mut candidate = UncheckedCandidate {
            binding: binding(),
            boundary: UncheckedSignature::default(),
            realization: OpenJudgment::TypeFormation {
                context: DependentContext::default(),
                term: Term::UnitType,
            },
            induced_obligations: ObligationComplex { layers: Vec::new() },
            boundary_inclusion: BoundaryInclusion {
                entries: Vec::new(),
            },
        };
        let subject = candidate.subject_digest();
        candidate.binding.candidate_digest = digest(b"other-binding");
        assert_eq!(candidate.subject_digest(), subject);

        candidate.realization = OpenJudgment::TypeFormation {
            context: DependentContext::default(),
            term: Term::Sort { level: 0 },
        };
        assert_ne!(candidate.subject_digest(), subject);
    }

    #[test]
    fn family_and_instance_ids_use_distinct_subject_domains() {
        let context = DependentContext::default();
        let body = OpenJudgment::TypeFormation {
            context: context.clone(),
            term: Term::UnitType,
        };
        let family = SchemaFamilyId::for_subject(&context, &body);
        let instance = SchemaInstanceId::for_subject(&family, &[], &body);
        assert_ne!(family.0, instance.0);
    }
}
