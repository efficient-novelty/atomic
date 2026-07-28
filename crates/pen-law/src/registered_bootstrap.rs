//! Strict Law-V2A registration boundary.
//!
//! The embedded artifact discloses one fixed three-act initial condition. It
//! is defined directly in the kernel fragment and is not sliced from a legacy
//! target fixture. Verification establishes only exact syntax, normalization,
//! typing, provenance bindings, and digest chaining. It does not derive the
//! prefix from the empty context and does not establish a free-sealing
//! universal property, leastness, or uniqueness.

use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, Digest, GlobalId, Kernel, KernelError, Term,
    UncheckedSignature, VerifiedSignature,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

pub const REGISTERED_BOOTSTRAP_SCHEMA_VERSION: u16 = 1;
pub const REGISTERED_BOOTSTRAP_CODEC_VERSION: u16 = 1;
pub const REGISTERED_BOOTSTRAP_ACT_COUNT: usize = 3;
pub const REGISTERED_BOOTSTRAP_MAX_BYTES: usize = 16 * 1024;

const EMBEDDED_REGISTERED_BOOTSTRAP: &[u8] =
    include_bytes!("../assets/law_v2a_registered_bootstrap_v1.json");

const CONTRACT_DOMAIN: &str = "law-v2a-registered-bootstrap-contract-v1";
const GLOBAL_ID_DOMAIN: &str = "law-v2a-registered-bootstrap-global-v1";
const ACT_SOURCE_DOMAIN: &str = "law-v2a-registered-bootstrap-act-source-v1";
const ACT_BINDING_DOMAIN: &str = "law-v2a-registered-bootstrap-act-binding-v1";
const ARTIFACT_DOMAIN: &str = "law-v2a-registered-bootstrap-artifact-v1";

/// Trusted-protocol provenance carried by the unchecked registration.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredBootstrapBinding {
    pub schema_version: u16,
    pub codec_version: u16,
    pub kernel_digest: Digest,
    pub normalizer_digest: Digest,
    pub bootstrap_contract_digest: Digest,
}

impl CanonicalEncode for RegisteredBootstrapBinding {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.u16(self.codec_version);
        self.kernel_digest.encode_canonical(encoder);
        self.normalizer_digest.encode_canonical(encoder);
        self.bootstrap_contract_digest.encode_canonical(encoder);
    }
}

/// One unchecked act in the disclosed registration.
///
/// `extension` is the registered source declaration.
/// `normalized_extension` is checked afresh and must be the exact normal form
/// produced by appending that source to the verified predecessor.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedRegisteredBootstrapAct {
    pub ordinal: u16,
    pub predecessor_checked_boundary_digest: Digest,
    pub source_identity: Digest,
    pub binding_identity: Digest,
    pub extension: UncheckedSignature,
    pub normalized_extension: UncheckedSignature,
    pub checked_boundary_digest: Digest,
}

impl CanonicalEncode for UncheckedRegisteredBootstrapAct {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.ordinal);
        self.predecessor_checked_boundary_digest
            .encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        self.binding_identity.encode_canonical(encoder);
        self.extension.encode_canonical(encoder);
        self.normalized_extension.encode_canonical(encoder);
        self.checked_boundary_digest.encode_canonical(encoder);
    }
}

/// Deserializable Law-V2A registration input.
///
/// The claimed digest is an identity for the other fields, not evidence. Only
/// [`verify_registered_bootstrap_bytes`] can produce the private verified
/// capability.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedRegisteredBootstrap {
    pub binding: RegisteredBootstrapBinding,
    pub initial_checked_boundary_digest: Digest,
    pub acts: Vec<UncheckedRegisteredBootstrapAct>,
    pub final_boundary: UncheckedSignature,
    pub final_checked_boundary_digest: Digest,
    pub claimed_artifact_digest: Digest,
}

impl UncheckedRegisteredBootstrap {
    /// Canonical identity of the registration fields other than the claimed
    /// identity itself. This helper does not verify the artifact.
    pub fn subject_digest(&self) -> Digest {
        let mut encoder = CanonicalEncoder::new();
        self.binding.encode_canonical(&mut encoder);
        self.initial_checked_boundary_digest
            .encode_canonical(&mut encoder);
        encoder.sequence(&self.acts);
        self.final_boundary.encode_canonical(&mut encoder);
        self.final_checked_boundary_digest
            .encode_canonical(&mut encoder);
        Digest::of_domain_bytes(ARTIFACT_DOMAIN, encoder.as_bytes())
    }
}

/// Reasons an unchecked registration cannot cross the Law-V2A boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegisteredBootstrapError {
    InputTooLarge { actual: usize, maximum: usize },
    InvalidJson(String),
    SchemaVersion,
    CodecVersion,
    KernelBinding,
    NormalizerBinding,
    ContractBinding,
    InitialBoundary,
    ActCount,
    ActOrdinal { expected: u16 },
    SourceDeclarationCount { ordinal: u16 },
    NormalizedDeclarationCount { ordinal: u16 },
    SourceShape { ordinal: u16 },
    PredecessorBoundary { ordinal: u16 },
    SourceIdentity { ordinal: u16 },
    NormalizedExtension { ordinal: u16 },
    CheckedBoundary { ordinal: u16 },
    ActBinding { ordinal: u16 },
    FinalBoundary,
    FinalBoundaryDigest,
    ArtifactDigest,
    Kernel(KernelError),
}

impl fmt::Display for RegisteredBootstrapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "registered bootstrap is {actual} bytes; maximum is {maximum}"
                )
            }
            Self::InvalidJson(message) => {
                write!(formatter, "invalid registered-bootstrap JSON: {message}")
            }
            Self::SchemaVersion => formatter.write_str("unsupported bootstrap schema version"),
            Self::CodecVersion => formatter.write_str("unsupported bootstrap codec version"),
            Self::KernelBinding => formatter.write_str("bootstrap kernel binding mismatch"),
            Self::NormalizerBinding => formatter.write_str("bootstrap normalizer binding mismatch"),
            Self::ContractBinding => formatter.write_str("bootstrap contract binding mismatch"),
            Self::InitialBoundary => formatter.write_str("bootstrap initial boundary mismatch"),
            Self::ActCount => formatter.write_str("bootstrap must contain exactly three acts"),
            Self::ActOrdinal { expected } => {
                write!(formatter, "bootstrap act ordinal must be {expected}")
            }
            Self::SourceDeclarationCount { ordinal } => {
                write!(
                    formatter,
                    "bootstrap act {ordinal} source must contain one declaration"
                )
            }
            Self::NormalizedDeclarationCount { ordinal } => {
                write!(
                    formatter,
                    "bootstrap act {ordinal} normal form must contain one declaration"
                )
            }
            Self::SourceShape { ordinal } => {
                write!(
                    formatter,
                    "bootstrap act {ordinal} source is not registered"
                )
            }
            Self::PredecessorBoundary { ordinal } => {
                write!(
                    formatter,
                    "bootstrap act {ordinal} predecessor boundary mismatch"
                )
            }
            Self::SourceIdentity { ordinal } => {
                write!(
                    formatter,
                    "bootstrap act {ordinal} source identity mismatch"
                )
            }
            Self::NormalizedExtension { ordinal } => {
                write!(
                    formatter,
                    "bootstrap act {ordinal} normalized extension mismatch"
                )
            }
            Self::CheckedBoundary { ordinal } => {
                write!(
                    formatter,
                    "bootstrap act {ordinal} checked boundary mismatch"
                )
            }
            Self::ActBinding { ordinal } => {
                write!(
                    formatter,
                    "bootstrap act {ordinal} binding identity mismatch"
                )
            }
            Self::FinalBoundary => formatter.write_str("bootstrap final boundary mismatch"),
            Self::FinalBoundaryDigest => {
                formatter.write_str("bootstrap final boundary digest mismatch")
            }
            Self::ArtifactDigest => formatter.write_str("bootstrap artifact digest mismatch"),
            Self::Kernel(error) => write!(formatter, "kernel rejected bootstrap: {error}"),
        }
    }
}

impl std::error::Error for RegisteredBootstrapError {}

impl From<KernelError> for RegisteredBootstrapError {
    fn from(error: KernelError) -> Self {
        Self::Kernel(error)
    }
}

/// Capability produced only after strict registration replay.
///
/// Its fields are private and it has no deserialization implementation.
#[derive(Debug)]
pub struct VerifiedRegisteredBootstrap {
    artifact_digest: Digest,
    bootstrap_contract_digest: Digest,
    kernel_digest: Digest,
    normalizer_digest: Digest,
    boundary_chain: [Digest; REGISTERED_BOOTSTRAP_ACT_COUNT + 1],
    source_identities: [Digest; REGISTERED_BOOTSTRAP_ACT_COUNT],
    binding_identities: [Digest; REGISTERED_BOOTSTRAP_ACT_COUNT],
    final_boundary: VerifiedSignature,
}

impl VerifiedRegisteredBootstrap {
    pub fn artifact_digest(&self) -> &Digest {
        &self.artifact_digest
    }

    pub fn bootstrap_contract_digest(&self) -> &Digest {
        &self.bootstrap_contract_digest
    }

    pub fn kernel_digest(&self) -> &Digest {
        &self.kernel_digest
    }

    pub fn normalizer_digest(&self) -> &Digest {
        &self.normalizer_digest
    }

    /// Empty predecessor followed by the three checked prefix boundaries.
    pub fn boundary_chain(&self) -> &[Digest; REGISTERED_BOOTSTRAP_ACT_COUNT + 1] {
        &self.boundary_chain
    }

    pub fn source_identities(&self) -> &[Digest; REGISTERED_BOOTSTRAP_ACT_COUNT] {
        &self.source_identities
    }

    pub fn binding_identities(&self) -> &[Digest; REGISTERED_BOOTSTRAP_ACT_COUNT] {
        &self.binding_identities
    }

    pub fn final_boundary(&self) -> &VerifiedSignature {
        &self.final_boundary
    }
}

/// Stable identity of the narrow Law-V2A registration contract.
pub fn registered_bootstrap_contract_digest() -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(REGISTERED_BOOTSTRAP_SCHEMA_VERSION);
    encoder.u16(REGISTERED_BOOTSTRAP_CODEC_VERSION);
    encoder.u64(REGISTERED_BOOTSTRAP_ACT_COUNT as u64);
    encoder.sequence(&registered_source_extensions());
    Digest::of_domain_bytes(CONTRACT_DOMAIN, encoder.as_bytes())
}

/// Load and replay the crate's disclosed registration under a fixed byte cap.
pub fn load_embedded_registered_bootstrap(
    kernel: &Kernel,
) -> Result<VerifiedRegisteredBootstrap, RegisteredBootstrapError> {
    verify_registered_bootstrap_bytes(kernel, EMBEDDED_REGISTERED_BOOTSTRAP)
}

/// Parse and replay a registration under the protocol's fixed byte cap.
///
/// Callers cannot raise the cap. Unknown fields are rejected at every
/// registration and kernel-wire object boundary.
pub fn verify_registered_bootstrap_bytes(
    kernel: &Kernel,
    bytes: &[u8],
) -> Result<VerifiedRegisteredBootstrap, RegisteredBootstrapError> {
    if bytes.len() > REGISTERED_BOOTSTRAP_MAX_BYTES {
        return Err(RegisteredBootstrapError::InputTooLarge {
            actual: bytes.len(),
            maximum: REGISTERED_BOOTSTRAP_MAX_BYTES,
        });
    }
    let unchecked: UncheckedRegisteredBootstrap = serde_json::from_slice(bytes)
        .map_err(|error| RegisteredBootstrapError::InvalidJson(error.to_string()))?;
    verify_registered_bootstrap(kernel, &unchecked)
}

fn verify_registered_bootstrap(
    kernel: &Kernel,
    unchecked: &UncheckedRegisteredBootstrap,
) -> Result<VerifiedRegisteredBootstrap, RegisteredBootstrapError> {
    if unchecked.binding.schema_version != REGISTERED_BOOTSTRAP_SCHEMA_VERSION {
        return Err(RegisteredBootstrapError::SchemaVersion);
    }
    if unchecked.binding.codec_version != REGISTERED_BOOTSTRAP_CODEC_VERSION {
        return Err(RegisteredBootstrapError::CodecVersion);
    }
    if unchecked.binding.kernel_digest != kernel.kernel_protocol_digest() {
        return Err(RegisteredBootstrapError::KernelBinding);
    }
    if unchecked.binding.normalizer_digest != kernel.normalizer_protocol_digest() {
        return Err(RegisteredBootstrapError::NormalizerBinding);
    }
    if unchecked.binding.bootstrap_contract_digest != registered_bootstrap_contract_digest() {
        return Err(RegisteredBootstrapError::ContractBinding);
    }
    if unchecked.acts.len() != REGISTERED_BOOTSTRAP_ACT_COUNT {
        return Err(RegisteredBootstrapError::ActCount);
    }

    let mut checked = kernel.verify_signature(&UncheckedSignature::default())?;
    if unchecked.initial_checked_boundary_digest != *checked.digest() {
        return Err(RegisteredBootstrapError::InitialBoundary);
    }

    let expected_extensions = registered_source_extensions();
    let mut boundary_chain = Vec::with_capacity(REGISTERED_BOOTSTRAP_ACT_COUNT + 1);
    let mut source_identities = Vec::with_capacity(REGISTERED_BOOTSTRAP_ACT_COUNT);
    let mut binding_identities = Vec::with_capacity(REGISTERED_BOOTSTRAP_ACT_COUNT);
    boundary_chain.push(checked.digest().clone());

    for (index, (act, expected_extension)) in unchecked
        .acts
        .iter()
        .zip(expected_extensions.iter())
        .enumerate()
    {
        let ordinal = u16::try_from(index + 1).expect("three bootstrap positions fit u16");
        if act.ordinal != ordinal {
            return Err(RegisteredBootstrapError::ActOrdinal { expected: ordinal });
        }
        if act.extension.declarations.len() != 1 {
            return Err(RegisteredBootstrapError::SourceDeclarationCount { ordinal });
        }
        if act.normalized_extension.declarations.len() != 1 {
            return Err(RegisteredBootstrapError::NormalizedDeclarationCount { ordinal });
        }
        if act.extension != *expected_extension {
            return Err(RegisteredBootstrapError::SourceShape { ordinal });
        }
        if act.predecessor_checked_boundary_digest != *checked.digest() {
            return Err(RegisteredBootstrapError::PredecessorBoundary { ordinal });
        }

        let source_identity = act_source_identity(ordinal, &act.extension);
        if act.source_identity != source_identity {
            return Err(RegisteredBootstrapError::SourceIdentity { ordinal });
        }

        let source_checked = kernel.verify_extension(&checked, &act.extension)?;
        let normalized_extension = extension_tail(&checked, &source_checked);
        if act.normalized_extension != normalized_extension {
            return Err(RegisteredBootstrapError::NormalizedExtension { ordinal });
        }

        let normalized_checked = kernel.verify_extension(&checked, &act.normalized_extension)?;
        if normalized_checked.digest() != source_checked.digest()
            || normalized_checked.normalized_wire() != source_checked.normalized_wire()
        {
            return Err(RegisteredBootstrapError::NormalizedExtension { ordinal });
        }
        if act.checked_boundary_digest != *source_checked.digest() {
            return Err(RegisteredBootstrapError::CheckedBoundary { ordinal });
        }

        let binding_identity = act_binding_identity(
            &unchecked.binding,
            ordinal,
            &act.predecessor_checked_boundary_digest,
            &source_identity,
            &normalized_extension,
            source_checked.digest(),
        );
        if act.binding_identity != binding_identity {
            return Err(RegisteredBootstrapError::ActBinding { ordinal });
        }

        boundary_chain.push(source_checked.digest().clone());
        source_identities.push(source_identity);
        binding_identities.push(binding_identity);
        checked = source_checked;
    }

    if unchecked.final_boundary != checked.normalized_wire() {
        return Err(RegisteredBootstrapError::FinalBoundary);
    }
    let final_checked = kernel.verify_signature(&unchecked.final_boundary)?;
    if final_checked.normalized_wire() != unchecked.final_boundary {
        return Err(RegisteredBootstrapError::FinalBoundary);
    }
    if unchecked.final_checked_boundary_digest != *checked.digest()
        || final_checked.digest() != checked.digest()
    {
        return Err(RegisteredBootstrapError::FinalBoundaryDigest);
    }
    let artifact_digest = unchecked.subject_digest();
    if unchecked.claimed_artifact_digest != artifact_digest {
        return Err(RegisteredBootstrapError::ArtifactDigest);
    }

    Ok(VerifiedRegisteredBootstrap {
        artifact_digest,
        bootstrap_contract_digest: unchecked.binding.bootstrap_contract_digest.clone(),
        kernel_digest: unchecked.binding.kernel_digest.clone(),
        normalizer_digest: unchecked.binding.normalizer_digest.clone(),
        boundary_chain: boundary_chain
            .try_into()
            .expect("exact act count fixes boundary-chain length"),
        source_identities: source_identities
            .try_into()
            .expect("exact act count fixes source-identity length"),
        binding_identities: binding_identities
            .try_into()
            .expect("exact act count fixes binding-identity length"),
        final_boundary: final_checked,
    })
}

fn registered_source_extensions() -> [UncheckedSignature; REGISTERED_BOOTSTRAP_ACT_COUNT] {
    let universe = registered_global_id(1);
    let registered_type = registered_global_id(2);
    let witness = registered_global_id(3);
    [
        one_declaration(
            universe.clone(),
            Term::Sort { level: 1 },
            Term::Sort { level: 0 },
        ),
        one_declaration(
            registered_type.clone(),
            Term::Global { id: universe },
            Term::UnitType,
        ),
        one_declaration(
            witness,
            Term::Global {
                id: registered_type,
            },
            Term::Unit,
        ),
    ]
}

fn registered_global_id(ordinal: u16) -> GlobalId {
    GlobalId(Digest::of_domain_bytes(
        GLOBAL_ID_DOMAIN,
        &ordinal.to_le_bytes(),
    ))
}

fn one_declaration(id: GlobalId, ty: Term, body: Term) -> UncheckedSignature {
    UncheckedSignature {
        declarations: vec![Declaration {
            id,
            ty,
            body: Some(body),
        }],
    }
}

fn extension_tail(
    predecessor: &VerifiedSignature,
    extended: &VerifiedSignature,
) -> UncheckedSignature {
    UncheckedSignature {
        declarations: extended.declarations()[predecessor.declarations().len()..].to_vec(),
    }
}

fn act_source_identity(ordinal: u16, extension: &UncheckedSignature) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(ordinal);
    extension.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(ACT_SOURCE_DOMAIN, encoder.as_bytes())
}

fn act_binding_identity(
    binding: &RegisteredBootstrapBinding,
    ordinal: u16,
    predecessor: &Digest,
    source_identity: &Digest,
    normalized_extension: &UncheckedSignature,
    checked_boundary: &Digest,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    binding.encode_canonical(&mut encoder);
    encoder.u16(ordinal);
    predecessor.encode_canonical(&mut encoder);
    source_identity.encode_canonical(&mut encoder);
    normalized_extension.encode_canonical(&mut encoder);
    checked_boundary.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(ACT_BINDING_DOMAIN, encoder.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_kernel::KernelLimits;
    use serde_json::{Value, json};

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    fn embedded_unchecked() -> UncheckedRegisteredBootstrap {
        serde_json::from_slice(EMBEDDED_REGISTERED_BOOTSTRAP).expect("embedded registration JSON")
    }

    fn encode(unchecked: &UncheckedRegisteredBootstrap) -> Vec<u8> {
        serde_json::to_vec(unchecked).expect("serialize registration")
    }

    fn fresh_disclosure(kernel: &Kernel) -> UncheckedRegisteredBootstrap {
        let binding = RegisteredBootstrapBinding {
            schema_version: REGISTERED_BOOTSTRAP_SCHEMA_VERSION,
            codec_version: REGISTERED_BOOTSTRAP_CODEC_VERSION,
            kernel_digest: kernel.kernel_protocol_digest(),
            normalizer_digest: kernel.normalizer_protocol_digest(),
            bootstrap_contract_digest: registered_bootstrap_contract_digest(),
        };
        let mut checked = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty boundary");
        let initial_checked_boundary_digest = checked.digest().clone();
        let mut acts = Vec::with_capacity(REGISTERED_BOOTSTRAP_ACT_COUNT);
        for (index, extension) in registered_source_extensions().into_iter().enumerate() {
            let ordinal = u16::try_from(index + 1).expect("small ordinal");
            let predecessor_checked_boundary_digest = checked.digest().clone();
            let source_identity = act_source_identity(ordinal, &extension);
            let next = kernel
                .verify_extension(&checked, &extension)
                .expect("registered act checks");
            let normalized_extension = extension_tail(&checked, &next);
            let checked_boundary_digest = next.digest().clone();
            let binding_identity = act_binding_identity(
                &binding,
                ordinal,
                &predecessor_checked_boundary_digest,
                &source_identity,
                &normalized_extension,
                &checked_boundary_digest,
            );
            acts.push(UncheckedRegisteredBootstrapAct {
                ordinal,
                predecessor_checked_boundary_digest,
                source_identity,
                binding_identity,
                extension,
                normalized_extension,
                checked_boundary_digest,
            });
            checked = next;
        }
        let mut disclosure = UncheckedRegisteredBootstrap {
            binding,
            initial_checked_boundary_digest,
            acts,
            final_boundary: checked.normalized_wire(),
            final_checked_boundary_digest: checked.digest().clone(),
            claimed_artifact_digest: Digest::of_bytes(b"replaced below"),
        };
        disclosure.claimed_artifact_digest = disclosure.subject_digest();
        disclosure
    }

    #[test]
    fn embedded_registration_replays_to_a_private_capability() {
        let kernel = kernel();
        let verified =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        assert_eq!(
            verified.bootstrap_contract_digest(),
            &registered_bootstrap_contract_digest()
        );
        assert_eq!(verified.kernel_digest(), &kernel.kernel_protocol_digest());
        assert_eq!(
            verified.normalizer_digest(),
            &kernel.normalizer_protocol_digest()
        );
        assert_eq!(
            verified.boundary_chain().last(),
            Some(verified.final_boundary().digest())
        );
        assert_eq!(verified.final_boundary().declarations().len(), 3);
        assert_eq!(verified.source_identities().len(), 3);
        assert_eq!(verified.binding_identities().len(), 3);
    }

    #[test]
    fn contract_identity_commits_to_the_exact_registered_extensions() {
        let mut encoder = CanonicalEncoder::new();
        encoder.u16(REGISTERED_BOOTSTRAP_SCHEMA_VERSION);
        encoder.u16(REGISTERED_BOOTSTRAP_CODEC_VERSION);
        encoder.u64(REGISTERED_BOOTSTRAP_ACT_COUNT as u64);
        encoder.sequence(&registered_source_extensions());
        assert_eq!(
            registered_bootstrap_contract_digest(),
            Digest::of_domain_bytes(CONTRACT_DOMAIN, encoder.as_bytes())
        );
        assert_ne!(
            registered_bootstrap_contract_digest(),
            Digest::of_domain_bytes(
                CONTRACT_DOMAIN,
                b"schema-1/codec-1/exact-three-anonymous-kernel-acts"
            )
        );
    }

    #[test]
    fn embedded_asset_is_the_fresh_disclosure_not_a_fixture_slice() {
        assert_eq!(embedded_unchecked(), fresh_disclosure(&kernel()));
    }

    #[test]
    fn source_mutations_are_rejected_at_every_act() {
        let kernel = kernel();
        for index in 0..REGISTERED_BOOTSTRAP_ACT_COUNT {
            let mut mutated = embedded_unchecked();
            mutated.acts[index].extension.declarations[0].body = None;
            assert_eq!(
                verify_registered_bootstrap_bytes(&kernel, &encode(&mutated))
                    .expect_err("source mutation"),
                RegisteredBootstrapError::SourceShape {
                    ordinal: u16::try_from(index + 1).expect("small ordinal")
                }
            );
        }
    }

    #[test]
    fn a_definitionally_equal_but_noncanonical_extension_is_rejected() {
        let kernel = kernel();
        let mut mutated = embedded_unchecked();
        mutated.acts[2].normalized_extension.declarations[0].body = Some(Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Var { index: 0 }),
            }),
            argument: Box::new(Term::Unit),
        });
        assert_eq!(
            verify_registered_bootstrap_bytes(&kernel, &encode(&mutated))
                .expect_err("noncanonical cached normal form"),
            RegisteredBootstrapError::NormalizedExtension { ordinal: 3 }
        );
    }

    #[test]
    fn chain_and_claim_mutations_are_rejected() {
        let kernel = kernel();
        let mut predecessor = embedded_unchecked();
        predecessor.acts[1].predecessor_checked_boundary_digest =
            Digest::of_bytes(b"wrong predecessor");
        assert_eq!(
            verify_registered_bootstrap_bytes(&kernel, &encode(&predecessor))
                .expect_err("broken chain"),
            RegisteredBootstrapError::PredecessorBoundary { ordinal: 2 }
        );

        let mut claim = embedded_unchecked();
        claim.claimed_artifact_digest = Digest::of_bytes(b"wrong artifact");
        assert_eq!(
            verify_registered_bootstrap_bytes(&kernel, &encode(&claim)).expect_err("wrong claim"),
            RegisteredBootstrapError::ArtifactDigest
        );
    }

    #[test]
    fn downstream_fields_and_labels_cannot_enter_the_wire_format() {
        for (field, value) in [
            ("downstream_slots", json!([])),
            ("display_names", json!(["anticipated"])),
            ("anticipated_total", json!(4)),
        ] {
            let mut artifact: Value =
                serde_json::from_slice(EMBEDDED_REGISTERED_BOOTSTRAP).expect("JSON value");
            artifact
                .as_object_mut()
                .expect("top-level object")
                .insert(field.to_owned(), value);
            let bytes = serde_json::to_vec(&artifact).expect("serialize mutation");
            assert!(matches!(
                verify_registered_bootstrap_bytes(&kernel(), &bytes),
                Err(RegisteredBootstrapError::InvalidJson(_))
            ));
        }

        let mut nested: Value =
            serde_json::from_slice(EMBEDDED_REGISTERED_BOOTSTRAP).expect("JSON value");
        nested["acts"][0]["external_annotation"] = json!("future");
        let bytes = serde_json::to_vec(&nested).expect("serialize nested mutation");
        assert!(matches!(
            verify_registered_bootstrap_bytes(&kernel(), &bytes),
            Err(RegisteredBootstrapError::InvalidJson(_))
        ));
    }

    #[test]
    fn oversized_input_is_rejected_before_decoding() {
        let bytes = vec![b' '; REGISTERED_BOOTSTRAP_MAX_BYTES + 1];
        assert_eq!(
            verify_registered_bootstrap_bytes(&kernel(), &bytes).expect_err("over byte cap"),
            RegisteredBootstrapError::InputTooLarge {
                actual: REGISTERED_BOOTSTRAP_MAX_BYTES + 1,
                maximum: REGISTERED_BOOTSTRAP_MAX_BYTES,
            }
        );
    }

    #[test]
    fn kernel_resource_exhaustion_is_not_a_bootstrap_refutation() {
        let constrained = Kernel::new(KernelLimits {
            max_operations: 1,
            ..KernelLimits::default()
        })
        .expect("positive limits");
        assert!(matches!(
            load_embedded_registered_bootstrap(&constrained),
            Err(RegisteredBootstrapError::Kernel(
                KernelError::ResourceExhausted(_)
            ))
        ));
    }

    #[test]
    #[ignore = "maintenance helper: prints the canonical fresh disclosure"]
    fn print_fresh_disclosure_asset() {
        println!(
            "{}",
            serde_json::to_string_pretty(&fresh_disclosure(&kernel()))
                .expect("serialize disclosure")
        );
    }
}
