//! Proof-carrying semantic export index for the registered Law-V2A prefix.
//!
//! The index is a separate, digest-bound sidecar.  It does not modify the
//! registered bootstrap bytes and it does not pretend that a constructor
//! grouping follows from a flat signature alone.  Successful replay proves
//! exact binding, generic-code validity, native erasure, and one-use coverage
//! of the final registered boundary.

use crate::registered_bootstrap::registered_source_extensions;
use crate::{REGISTERED_BOOTSTRAP_ACT_COUNT, VerifiedRegisteredBootstrap};
use pen_demand::gsc::{
    ClosedInductiveCode, GscOutcome, VerifiedGscSemanticManifest, verify_closed_inductive_code,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, Digest, GlobalId, Kernel, KernelError, Term,
    UncheckedSignature,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const REGISTERED_BOOTSTRAP_EXPORT_SCHEMA_VERSION: u16 = 1;
pub const REGISTERED_BOOTSTRAP_EXPORT_CODEC_VERSION: u16 = 1;
pub const REGISTERED_BOOTSTRAP_EXPORT_MAX_BYTES: usize = 32 * 1024;

const EMBEDDED_REGISTERED_BOOTSTRAP_EXPORT_INDEX: &[u8] =
    include_bytes!("../assets/law_v2a_registered_bootstrap_export_index_v1.json");
const EXPORT_INDEX_DOMAIN: &str = "law-v2a-registered-bootstrap-export-index-v1";
const GROUP_DOMAIN: &str = "law-v2a-registered-bootstrap-export-group-v1";

/// The only native erasure admitted by this first registered sidecar.
///
/// The name describes a shape rather than a historical stage or public
/// display name.  Its verifier recognizes the kernel's primitive closed
/// singleton former and sole nullary introduction.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegisteredInductiveErasureRuleV1 {
    NativeClosedSingleton,
}

impl CanonicalEncode for RegisteredInductiveErasureRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::NativeClosedSingleton => encoder.tag(0),
        }
    }
}

/// Constructor-position to public-declaration alias.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredIntroductionAliasV1 {
    pub constructor_ordinal: u16,
    pub declaration: GlobalId,
}

impl CanonicalEncode for RegisteredIntroductionAliasV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.constructor_ordinal);
        self.declaration.encode_canonical(encoder);
    }
}

/// Replay material for exact erasure to declarations already in the
/// registered bootstrap.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedInductiveAliasErasureV1 {
    pub rule: RegisteredInductiveErasureRuleV1,
    /// Owner first, then introductions in constructor order.
    pub source_identities: Vec<Digest>,
    /// Normalized owner first, then introductions in constructor order.
    pub normalized_declarations: UncheckedSignature,
}

impl CanonicalEncode for UncheckedInductiveAliasErasureV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.rule.encode_canonical(encoder);
        encoder.sequence(&self.source_identities);
        self.normalized_declarations.encode_canonical(encoder);
    }
}

/// One disposition in the exhaustive final-boundary partition.
///
/// There is intentionally no unchecked `CertifiedIrrelevant` escape hatch in
/// this version.  Such a variant requires its own complete proof grammar.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "disposition", rename_all = "snake_case", deny_unknown_fields)]
pub enum UncheckedRegisteredGroupDispositionV1 {
    PlainPublicDeclaration {
        declaration: GlobalId,
    },
    InductiveAlias {
        owner: GlobalId,
        primitive_code: ClosedInductiveCode,
        introduction_aliases: Vec<RegisteredIntroductionAliasV1>,
        erasure: UncheckedInductiveAliasErasureV1,
    },
}

impl CanonicalEncode for UncheckedRegisteredGroupDispositionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::PlainPublicDeclaration { declaration } => {
                encoder.tag(0);
                declaration.encode_canonical(encoder);
            }
            Self::InductiveAlias {
                owner,
                primitive_code,
                introduction_aliases,
                erasure,
            } => {
                encoder.tag(1);
                owner.encode_canonical(encoder);
                primitive_code.encode_canonical(encoder);
                encoder.sequence(introduction_aliases);
                erasure.encode_canonical(encoder);
            }
        }
    }
}

impl UncheckedRegisteredGroupDispositionV1 {
    fn declarations(&self) -> Vec<&GlobalId> {
        match self {
            Self::PlainPublicDeclaration { declaration } => vec![declaration],
            Self::InductiveAlias {
                owner,
                introduction_aliases,
                ..
            } => std::iter::once(owner)
                .chain(introduction_aliases.iter().map(|alias| &alias.declaration))
                .collect(),
        }
    }
}

/// Deserializable export-index sidecar.  Every digest is a claim until replay
/// returns [`VerifiedRegisteredBootstrapExportIndexV1`].
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedRegisteredBootstrapExportIndexV1 {
    pub schema_version: u16,
    pub codec_version: u16,
    pub semantic_manifest_digest: Digest,
    pub bootstrap_contract_digest: Digest,
    pub bootstrap_artifact_digest: Digest,
    pub final_checked_boundary_digest: Digest,
    pub boundary_chain: Vec<Digest>,
    pub registered_source_identities: Vec<Digest>,
    pub registered_binding_identities: Vec<Digest>,
    pub group_dispositions: Vec<UncheckedRegisteredGroupDispositionV1>,
    /// Explicit theorem registrations only.  The first profile admits none.
    pub registered_q3_theorems: Vec<GlobalId>,
    pub claimed_subject_digest: Digest,
}

impl UncheckedRegisteredBootstrapExportIndexV1 {
    pub fn subject_digest(&self) -> Digest {
        let mut encoder = CanonicalEncoder::new();
        encoder.u16(self.schema_version);
        encoder.u16(self.codec_version);
        self.semantic_manifest_digest.encode_canonical(&mut encoder);
        self.bootstrap_contract_digest
            .encode_canonical(&mut encoder);
        self.bootstrap_artifact_digest
            .encode_canonical(&mut encoder);
        self.final_checked_boundary_digest
            .encode_canonical(&mut encoder);
        encoder.sequence(&self.boundary_chain);
        encoder.sequence(&self.registered_source_identities);
        encoder.sequence(&self.registered_binding_identities);
        encoder.sequence(&self.group_dispositions);
        encoder.sequence(&self.registered_q3_theorems);
        Digest::of_domain_bytes(EXPORT_INDEX_DOMAIN, encoder.as_bytes())
    }
}

/// Strict sidecar rejection reasons.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegisteredBootstrapExportError {
    InputTooLarge { actual: usize, maximum: usize },
    InvalidJson(String),
    SchemaVersion,
    CodecVersion,
    SemanticManifestBinding,
    BootstrapContractBinding,
    BootstrapArtifactBinding,
    FinalBoundaryBinding,
    BoundaryChainBinding,
    SourceIdentityBinding,
    ActBinding,
    SubjectDigest,
    EmptyGroup,
    GroupOrder,
    UnknownDeclaration,
    DuplicateDeclaration,
    MissingDeclaration,
    IntroductionOrder,
    IntroductionCoverage,
    UnsupportedInductiveCode,
    ErasureSourceIdentity,
    ErasureNormalizedDeclaration,
    ErasureSourceShape,
    ErasureNormalizedShape,
    NonEmptyQ3Registry,
    Kernel(KernelError),
}

impl fmt::Display for RegisteredBootstrapExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "registered bootstrap export index is {actual} bytes; maximum is {maximum}"
                )
            }
            Self::InvalidJson(message) => {
                write!(formatter, "invalid bootstrap export-index JSON: {message}")
            }
            Self::SchemaVersion => formatter.write_str("unsupported export-index schema version"),
            Self::CodecVersion => formatter.write_str("unsupported export-index codec version"),
            Self::SemanticManifestBinding => {
                formatter.write_str("export index semantic-manifest binding mismatch")
            }
            Self::BootstrapContractBinding => {
                formatter.write_str("export index bootstrap-contract binding mismatch")
            }
            Self::BootstrapArtifactBinding => {
                formatter.write_str("export index bootstrap-artifact binding mismatch")
            }
            Self::FinalBoundaryBinding => {
                formatter.write_str("export index final-boundary binding mismatch")
            }
            Self::BoundaryChainBinding => {
                formatter.write_str("export index boundary-chain binding mismatch")
            }
            Self::SourceIdentityBinding => {
                formatter.write_str("export index source-identity binding mismatch")
            }
            Self::ActBinding => formatter.write_str("export index act-binding mismatch"),
            Self::SubjectDigest => formatter.write_str("export-index subject digest mismatch"),
            Self::EmptyGroup => formatter.write_str("export index contains an empty group"),
            Self::GroupOrder => {
                formatter.write_str("export groups are not in canonical boundary order")
            }
            Self::UnknownDeclaration => {
                formatter.write_str("export group names a declaration outside the bootstrap")
            }
            Self::DuplicateDeclaration => {
                formatter.write_str("a bootstrap declaration occurs in more than one group")
            }
            Self::MissingDeclaration => {
                formatter.write_str("a bootstrap declaration has no group disposition")
            }
            Self::IntroductionOrder => {
                formatter.write_str("constructor aliases are not in canonical ordinal order")
            }
            Self::IntroductionCoverage => {
                formatter.write_str("constructor aliases do not cover the code exactly")
            }
            Self::UnsupportedInductiveCode => {
                formatter.write_str("inductive code is unsupported or not verified")
            }
            Self::ErasureSourceIdentity => {
                formatter.write_str("inductive erasure source identities do not replay")
            }
            Self::ErasureNormalizedDeclaration => {
                formatter.write_str("inductive erasure normalized declarations do not replay")
            }
            Self::ErasureSourceShape => {
                formatter.write_str("inductive aliases do not erase to the exact source shapes")
            }
            Self::ErasureNormalizedShape => {
                formatter.write_str("inductive aliases do not erase to the exact normal forms")
            }
            Self::NonEmptyQ3Registry => formatter.write_str(
                "the first bootstrap export grammar admits no explicit Q3 theorem registrations",
            ),
            Self::Kernel(error) => write!(formatter, "kernel rejected export replay: {error}"),
        }
    }
}

impl std::error::Error for RegisteredBootstrapExportError {}

impl From<KernelError> for RegisteredBootstrapExportError {
    fn from(error: KernelError) -> Self {
        Self::Kernel(error)
    }
}

/// One verified group plus its declaration-origin-derived activation event.
#[derive(Clone, Debug)]
pub struct VerifiedRegisteredGroupDispositionV1 {
    disposition: UncheckedRegisteredGroupDispositionV1,
    declarations: Vec<GlobalId>,
    activation_ordinal: u16,
    digest: Digest,
}

impl VerifiedRegisteredGroupDispositionV1 {
    pub fn disposition(&self) -> &UncheckedRegisteredGroupDispositionV1 {
        &self.disposition
    }

    pub fn declarations(&self) -> &[GlobalId] {
        &self.declarations
    }

    pub fn activation_ordinal(&self) -> u16 {
        self.activation_ordinal
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Non-deserializable capability for the exhaustive final-boundary partition.
#[derive(Debug)]
pub struct VerifiedRegisteredBootstrapExportIndexV1 {
    digest: Digest,
    semantic_manifest_digest: Digest,
    bootstrap_contract_digest: Digest,
    bootstrap_artifact_digest: Digest,
    final_boundary_digest: Digest,
    groups: Vec<VerifiedRegisteredGroupDispositionV1>,
}

impl VerifiedRegisteredBootstrapExportIndexV1 {
    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn bootstrap_contract_digest(&self) -> &Digest {
        &self.bootstrap_contract_digest
    }

    pub fn bootstrap_artifact_digest(&self) -> &Digest {
        &self.bootstrap_artifact_digest
    }

    pub fn final_boundary_digest(&self) -> &Digest {
        &self.final_boundary_digest
    }

    pub fn groups(&self) -> &[VerifiedRegisteredGroupDispositionV1] {
        &self.groups
    }

    /// True only relative to the exhaustive registered export grammar.
    pub fn registered_q3_theorems_is_empty(&self) -> bool {
        true
    }
}

/// Load the disclosed export-index sidecar.
pub fn load_embedded_registered_bootstrap_export_index_v1(
    kernel: &Kernel,
    bootstrap: &VerifiedRegisteredBootstrap,
    semantic_manifest: &VerifiedGscSemanticManifest,
) -> Result<VerifiedRegisteredBootstrapExportIndexV1, RegisteredBootstrapExportError> {
    verify_registered_bootstrap_export_index_bytes_v1(
        kernel,
        bootstrap,
        semantic_manifest,
        EMBEDDED_REGISTERED_BOOTSTRAP_EXPORT_INDEX,
    )
}

/// Parse and replay one export-index sidecar under a fixed byte cap.
pub fn verify_registered_bootstrap_export_index_bytes_v1(
    kernel: &Kernel,
    bootstrap: &VerifiedRegisteredBootstrap,
    semantic_manifest: &VerifiedGscSemanticManifest,
    bytes: &[u8],
) -> Result<VerifiedRegisteredBootstrapExportIndexV1, RegisteredBootstrapExportError> {
    if bytes.len() > REGISTERED_BOOTSTRAP_EXPORT_MAX_BYTES {
        return Err(RegisteredBootstrapExportError::InputTooLarge {
            actual: bytes.len(),
            maximum: REGISTERED_BOOTSTRAP_EXPORT_MAX_BYTES,
        });
    }
    let unchecked: UncheckedRegisteredBootstrapExportIndexV1 = serde_json::from_slice(bytes)
        .map_err(|error| RegisteredBootstrapExportError::InvalidJson(error.to_string()))?;
    verify_registered_bootstrap_export_index_v1(kernel, bootstrap, semantic_manifest, &unchecked)
}

fn verify_registered_bootstrap_export_index_v1(
    kernel: &Kernel,
    bootstrap: &VerifiedRegisteredBootstrap,
    semantic_manifest: &VerifiedGscSemanticManifest,
    unchecked: &UncheckedRegisteredBootstrapExportIndexV1,
) -> Result<VerifiedRegisteredBootstrapExportIndexV1, RegisteredBootstrapExportError> {
    if unchecked.schema_version != REGISTERED_BOOTSTRAP_EXPORT_SCHEMA_VERSION {
        return Err(RegisteredBootstrapExportError::SchemaVersion);
    }
    if unchecked.codec_version != REGISTERED_BOOTSTRAP_EXPORT_CODEC_VERSION {
        return Err(RegisteredBootstrapExportError::CodecVersion);
    }
    if unchecked.semantic_manifest_digest != *semantic_manifest.digest() {
        return Err(RegisteredBootstrapExportError::SemanticManifestBinding);
    }
    if unchecked.bootstrap_contract_digest != *bootstrap.bootstrap_contract_digest() {
        return Err(RegisteredBootstrapExportError::BootstrapContractBinding);
    }
    if unchecked.bootstrap_artifact_digest != *bootstrap.artifact_digest() {
        return Err(RegisteredBootstrapExportError::BootstrapArtifactBinding);
    }
    if unchecked.final_checked_boundary_digest != *bootstrap.final_boundary().digest() {
        return Err(RegisteredBootstrapExportError::FinalBoundaryBinding);
    }
    if unchecked.boundary_chain.as_slice() != bootstrap.boundary_chain() {
        return Err(RegisteredBootstrapExportError::BoundaryChainBinding);
    }
    if unchecked.registered_source_identities.as_slice() != bootstrap.source_identities() {
        return Err(RegisteredBootstrapExportError::SourceIdentityBinding);
    }
    if unchecked.registered_binding_identities.as_slice() != bootstrap.binding_identities() {
        return Err(RegisteredBootstrapExportError::ActBinding);
    }
    if unchecked.subject_digest() != unchecked.claimed_subject_digest {
        return Err(RegisteredBootstrapExportError::SubjectDigest);
    }
    if !unchecked.registered_q3_theorems.is_empty() {
        return Err(RegisteredBootstrapExportError::NonEmptyQ3Registry);
    }

    let final_declarations = bootstrap.final_boundary().declarations();
    if final_declarations.len() != REGISTERED_BOOTSTRAP_ACT_COUNT {
        return Err(RegisteredBootstrapExportError::MissingDeclaration);
    }
    let declaration_positions = final_declarations
        .iter()
        .enumerate()
        .map(|(index, declaration)| (declaration.id.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let source_extensions = registered_source_extensions();

    let mut covered = BTreeSet::new();
    let mut prior_group_minimum = None;
    let mut groups = Vec::with_capacity(unchecked.group_dispositions.len());
    for disposition in &unchecked.group_dispositions {
        let declaration_refs = disposition.declarations();
        if declaration_refs.is_empty() {
            return Err(RegisteredBootstrapExportError::EmptyGroup);
        }
        let mut positions = Vec::with_capacity(declaration_refs.len());
        for declaration in &declaration_refs {
            let position = declaration_positions
                .get(*declaration)
                .copied()
                .ok_or(RegisteredBootstrapExportError::UnknownDeclaration)?;
            if !covered.insert((*declaration).clone()) {
                return Err(RegisteredBootstrapExportError::DuplicateDeclaration);
            }
            positions.push(position);
        }
        let group_minimum = positions
            .iter()
            .copied()
            .min()
            .ok_or(RegisteredBootstrapExportError::EmptyGroup)?;
        if prior_group_minimum.is_some_and(|prior| prior >= group_minimum) {
            return Err(RegisteredBootstrapExportError::GroupOrder);
        }
        prior_group_minimum = Some(group_minimum);

        if let UncheckedRegisteredGroupDispositionV1::InductiveAlias {
            owner,
            primitive_code,
            introduction_aliases,
            erasure,
        } = disposition
        {
            verify_inductive_alias(
                kernel,
                semantic_manifest,
                primitive_code,
                owner,
                introduction_aliases,
                erasure,
                &declaration_positions,
                final_declarations,
                &source_extensions,
                bootstrap.source_identities(),
            )?;
        }

        let activation_index = positions
            .iter()
            .copied()
            .max()
            .ok_or(RegisteredBootstrapExportError::EmptyGroup)?;
        let activation_ordinal = u16::try_from(activation_index + 1)
            .map_err(|_| RegisteredBootstrapExportError::GroupOrder)?;
        groups.push(VerifiedRegisteredGroupDispositionV1 {
            disposition: disposition.clone(),
            declarations: declaration_refs.into_iter().cloned().collect(),
            activation_ordinal,
            digest: Digest::of_canonical(GROUP_DOMAIN, disposition),
        });
    }

    let expected = declaration_positions
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    if covered != expected {
        return Err(RegisteredBootstrapExportError::MissingDeclaration);
    }

    Ok(VerifiedRegisteredBootstrapExportIndexV1 {
        digest: unchecked.claimed_subject_digest.clone(),
        semantic_manifest_digest: unchecked.semantic_manifest_digest.clone(),
        bootstrap_contract_digest: unchecked.bootstrap_contract_digest.clone(),
        bootstrap_artifact_digest: unchecked.bootstrap_artifact_digest.clone(),
        final_boundary_digest: unchecked.final_checked_boundary_digest.clone(),
        groups,
    })
}

#[allow(clippy::too_many_arguments)]
fn verify_inductive_alias(
    kernel: &Kernel,
    semantic_manifest: &VerifiedGscSemanticManifest,
    primitive_code: &ClosedInductiveCode,
    owner: &GlobalId,
    introduction_aliases: &[RegisteredIntroductionAliasV1],
    erasure: &UncheckedInductiveAliasErasureV1,
    positions: &BTreeMap<GlobalId, usize>,
    final_declarations: &[Declaration],
    source_extensions: &[UncheckedSignature; REGISTERED_BOOTSTRAP_ACT_COUNT],
    registered_source_identities: &[Digest; REGISTERED_BOOTSTRAP_ACT_COUNT],
) -> Result<(), RegisteredBootstrapExportError> {
    let verified_code = match verify_closed_inductive_code(semantic_manifest, primitive_code) {
        GscOutcome::Proven(code) => code,
        GscOutcome::Unknown(_) => {
            return Err(RegisteredBootstrapExportError::UnsupportedInductiveCode);
        }
    };
    if !verified_code.is_one_nullary() || introduction_aliases.len() != 1 {
        return Err(RegisteredBootstrapExportError::IntroductionCoverage);
    }

    if introduction_aliases
        .iter()
        .enumerate()
        .any(|(index, alias)| usize::from(alias.constructor_ordinal) != index)
    {
        return Err(RegisteredBootstrapExportError::IntroductionOrder);
    }

    let ordered_ids = std::iter::once(owner)
        .chain(introduction_aliases.iter().map(|alias| &alias.declaration))
        .collect::<Vec<_>>();
    let ordered_positions = ordered_ids
        .iter()
        .map(|id| {
            positions
                .get(*id)
                .copied()
                .ok_or(RegisteredBootstrapExportError::UnknownDeclaration)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let expected_source_identities = ordered_positions
        .iter()
        .map(|position| registered_source_identities[*position].clone())
        .collect::<Vec<_>>();
    if erasure.source_identities != expected_source_identities {
        return Err(RegisteredBootstrapExportError::ErasureSourceIdentity);
    }
    let expected_normalized = ordered_positions
        .iter()
        .map(|position| final_declarations[*position].clone())
        .collect::<Vec<_>>();
    if erasure.normalized_declarations.declarations != expected_normalized {
        return Err(RegisteredBootstrapExportError::ErasureNormalizedDeclaration);
    }

    let owner_position = ordered_positions[0];
    let owner_source = &source_extensions[owner_position].declarations[0];
    if owner_source.id != *owner || owner_source.body != Some(Term::UnitType) {
        return Err(RegisteredBootstrapExportError::ErasureSourceShape);
    }
    let owner_normal = &final_declarations[owner_position];
    if owner_normal.id != *owner
        || owner_normal.ty
            != (Term::Sort {
                level: primitive_code.universe_level,
            })
        || owner_normal.body != Some(Term::UnitType)
    {
        return Err(RegisteredBootstrapExportError::ErasureNormalizedShape);
    }

    for (alias, position) in introduction_aliases
        .iter()
        .zip(ordered_positions.iter().skip(1))
    {
        let source = &source_extensions[*position].declarations[0];
        if source.id != alias.declaration
            || source.ty != (Term::Global { id: owner.clone() })
            || source.body != Some(Term::Unit)
        {
            return Err(RegisteredBootstrapExportError::ErasureSourceShape);
        }
        let normalized = &final_declarations[*position];
        if normalized.id != alias.declaration
            || normalized.ty != Term::UnitType
            || normalized.body != Some(Term::Unit)
        {
            return Err(RegisteredBootstrapExportError::ErasureNormalizedShape);
        }
    }

    // Rechecking the carried normalized slice ensures it remains a genuine
    // kernel extension in its original predecessor context.
    let first_position = ordered_positions[0];
    let predecessor = kernel.verify_signature(&UncheckedSignature {
        declarations: final_declarations[..first_position].to_vec(),
    })?;
    let _ = kernel.verify_extension(&predecessor, &erasure.normalized_declarations)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_embedded_registered_bootstrap;
    use pen_demand::gsc::{
        CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION, ComputationMode, ConstructorCode, TelescopeCode,
        frozen_gsc_semantic_manifest_v1, verify_gsc_semantic_manifest_v1,
    };
    use pen_kernel::KernelLimits;
    use serde_json::{Value, json};

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    fn manifest() -> VerifiedGscSemanticManifest {
        match verify_gsc_semantic_manifest_v1(&frozen_gsc_semantic_manifest_v1()) {
            GscOutcome::Proven(manifest) => manifest,
            GscOutcome::Unknown(reason) => panic!("frozen manifest is unknown: {reason:?}"),
        }
    }

    fn one_nullary_code() -> ClosedInductiveCode {
        ClosedInductiveCode {
            schema_version: CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
            universe_level: 0,
            parameters: TelescopeCode::default(),
            indices: TelescopeCode::default(),
            constructors: vec![ConstructorCode {
                arguments: TelescopeCode::default(),
                result_indices: Vec::new(),
                recursive_positions: Vec::new(),
                boundary_ports: Vec::new(),
            }],
            generated_computation_mode: ComputationMode::JudgmentalFreshHead,
        }
    }

    fn fresh_disclosure(
        bootstrap: &VerifiedRegisteredBootstrap,
        manifest: &VerifiedGscSemanticManifest,
    ) -> UncheckedRegisteredBootstrapExportIndexV1 {
        let declarations = bootstrap.final_boundary().declarations();
        let mut disclosure = UncheckedRegisteredBootstrapExportIndexV1 {
            schema_version: REGISTERED_BOOTSTRAP_EXPORT_SCHEMA_VERSION,
            codec_version: REGISTERED_BOOTSTRAP_EXPORT_CODEC_VERSION,
            semantic_manifest_digest: manifest.digest().clone(),
            bootstrap_contract_digest: bootstrap.bootstrap_contract_digest().clone(),
            bootstrap_artifact_digest: bootstrap.artifact_digest().clone(),
            final_checked_boundary_digest: bootstrap.final_boundary().digest().clone(),
            boundary_chain: bootstrap.boundary_chain().to_vec(),
            registered_source_identities: bootstrap.source_identities().to_vec(),
            registered_binding_identities: bootstrap.binding_identities().to_vec(),
            group_dispositions: vec![
                UncheckedRegisteredGroupDispositionV1::PlainPublicDeclaration {
                    declaration: declarations[0].id.clone(),
                },
                UncheckedRegisteredGroupDispositionV1::InductiveAlias {
                    owner: declarations[1].id.clone(),
                    primitive_code: one_nullary_code(),
                    introduction_aliases: vec![RegisteredIntroductionAliasV1 {
                        constructor_ordinal: 0,
                        declaration: declarations[2].id.clone(),
                    }],
                    erasure: UncheckedInductiveAliasErasureV1 {
                        rule: RegisteredInductiveErasureRuleV1::NativeClosedSingleton,
                        source_identities: bootstrap.source_identities()[1..].to_vec(),
                        normalized_declarations: UncheckedSignature {
                            declarations: declarations[1..].to_vec(),
                        },
                    },
                },
            ],
            registered_q3_theorems: Vec::new(),
            claimed_subject_digest: Digest::of_bytes(b"replaced below"),
        };
        disclosure.claimed_subject_digest = disclosure.subject_digest();
        disclosure
    }

    fn embedded_unchecked() -> UncheckedRegisteredBootstrapExportIndexV1 {
        serde_json::from_slice(EMBEDDED_REGISTERED_BOOTSTRAP_EXPORT_INDEX)
            .expect("embedded export-index JSON")
    }

    fn encode(index: &UncheckedRegisteredBootstrapExportIndexV1) -> Vec<u8> {
        serde_json::to_vec(index).expect("serialize export index")
    }

    #[test]
    fn embedded_sidecar_is_the_fresh_disclosure_and_replays() {
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        let manifest = manifest();
        assert_eq!(
            embedded_unchecked(),
            fresh_disclosure(&bootstrap, &manifest)
        );
        let verified =
            load_embedded_registered_bootstrap_export_index_v1(&kernel, &bootstrap, &manifest)
                .expect("embedded export index verifies");
        assert_eq!(verified.groups().len(), 2);
        assert_eq!(verified.groups()[0].activation_ordinal(), 1);
        assert_eq!(verified.groups()[1].activation_ordinal(), 3);
        assert!(verified.registered_q3_theorems_is_empty());
    }

    #[test]
    fn binding_and_claim_mutations_are_rejected() {
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        let manifest = manifest();
        let original = fresh_disclosure(&bootstrap, &manifest);

        let mut artifact = original.clone();
        artifact.bootstrap_artifact_digest = Digest::of_bytes(b"wrong bootstrap");
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&artifact),
            )
            .expect_err("bootstrap mutation"),
            RegisteredBootstrapExportError::BootstrapArtifactBinding
        );

        let mut semantic = original.clone();
        semantic.semantic_manifest_digest = Digest::of_bytes(b"wrong manifest");
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&semantic),
            )
            .expect_err("manifest mutation"),
            RegisteredBootstrapExportError::SemanticManifestBinding
        );

        let mut chain = original.clone();
        chain.boundary_chain[1] = Digest::of_bytes(b"wrong boundary");
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&chain),
            )
            .expect_err("boundary-chain mutation"),
            RegisteredBootstrapExportError::BoundaryChainBinding
        );

        let mut source = original.clone();
        source.registered_source_identities[1] = Digest::of_bytes(b"wrong source");
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&source),
            )
            .expect_err("source-list mutation"),
            RegisteredBootstrapExportError::SourceIdentityBinding
        );

        let mut claim = original;
        claim.claimed_subject_digest = Digest::of_bytes(b"wrong claim");
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&claim),
            )
            .expect_err("claim mutation"),
            RegisteredBootstrapExportError::SubjectDigest
        );
    }

    #[test]
    fn group_partition_must_be_canonical_complete_and_single_use() {
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        let manifest = manifest();
        let original = fresh_disclosure(&bootstrap, &manifest);

        let mut omitted = original.clone();
        omitted.group_dispositions.pop();
        omitted.claimed_subject_digest = omitted.subject_digest();
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&omitted),
            )
            .expect_err("omitted group"),
            RegisteredBootstrapExportError::MissingDeclaration
        );

        let mut duplicated = original.clone();
        duplicated.group_dispositions.push(
            UncheckedRegisteredGroupDispositionV1::PlainPublicDeclaration {
                declaration: bootstrap.final_boundary().declarations()[0].id.clone(),
            },
        );
        duplicated.claimed_subject_digest = duplicated.subject_digest();
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&duplicated),
            )
            .expect_err("duplicate declaration"),
            RegisteredBootstrapExportError::DuplicateDeclaration
        );

        let mut reordered = original;
        reordered.group_dispositions.swap(0, 1);
        reordered.claimed_subject_digest = reordered.subject_digest();
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&reordered),
            )
            .expect_err("group reorder"),
            RegisteredBootstrapExportError::GroupOrder
        );
    }

    #[test]
    fn alias_erasure_and_constructor_coverage_are_replayed() {
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        let manifest = manifest();
        let original = fresh_disclosure(&bootstrap, &manifest);

        let mut identity = original.clone();
        let UncheckedRegisteredGroupDispositionV1::InductiveAlias { erasure, .. } =
            &mut identity.group_dispositions[1]
        else {
            panic!("second group is inductive")
        };
        erasure.source_identities[0] = Digest::of_bytes(b"wrong source");
        identity.claimed_subject_digest = identity.subject_digest();
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&identity),
            )
            .expect_err("source identity mutation"),
            RegisteredBootstrapExportError::ErasureSourceIdentity
        );

        let mut normalized = original.clone();
        let UncheckedRegisteredGroupDispositionV1::InductiveAlias { erasure, .. } =
            &mut normalized.group_dispositions[1]
        else {
            panic!("second group is inductive")
        };
        erasure.normalized_declarations.declarations[1].body = None;
        normalized.claimed_subject_digest = normalized.subject_digest();
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&normalized),
            )
            .expect_err("normalized erasure mutation"),
            RegisteredBootstrapExportError::ErasureNormalizedDeclaration
        );

        let mut code = original.clone();
        let UncheckedRegisteredGroupDispositionV1::InductiveAlias { primitive_code, .. } =
            &mut code.group_dispositions[1]
        else {
            panic!("second group is inductive")
        };
        primitive_code.constructors.push(ConstructorCode {
            arguments: TelescopeCode::default(),
            result_indices: Vec::new(),
            recursive_positions: Vec::new(),
            boundary_ports: Vec::new(),
        });
        code.claimed_subject_digest = code.subject_digest();
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&code),
            )
            .expect_err("constructor mutation"),
            RegisteredBootstrapExportError::IntroductionCoverage
        );

        let mut universe = original;
        let UncheckedRegisteredGroupDispositionV1::InductiveAlias { primitive_code, .. } =
            &mut universe.group_dispositions[1]
        else {
            panic!("second group is inductive")
        };
        primitive_code.universe_level = 1;
        universe.claimed_subject_digest = universe.subject_digest();
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&universe),
            )
            .expect_err("universe mutation"),
            RegisteredBootstrapExportError::ErasureNormalizedShape
        );
    }

    #[test]
    fn q3_registration_fails_closed() {
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        let manifest = manifest();
        let mut q3 = fresh_disclosure(&bootstrap, &manifest);
        q3.registered_q3_theorems
            .push(bootstrap.final_boundary().declarations()[0].id.clone());
        q3.claimed_subject_digest = q3.subject_digest();
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest,
                &encode(&q3),
            )
            .expect_err("nonempty q3 registry"),
            RegisteredBootstrapExportError::NonEmptyQ3Registry
        );
    }

    #[test]
    fn unknown_fields_fail_closed_at_top_level_and_inside_groups() {
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        let manifest = manifest();
        let original = fresh_disclosure(&bootstrap, &manifest);

        let mut top: Value = serde_json::to_value(&original).expect("JSON value");
        top.as_object_mut()
            .expect("top-level object")
            .insert("future_hint".to_owned(), json!("forbidden"));
        let bytes = serde_json::to_vec(&top).expect("serialize");
        assert!(matches!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel, &bootstrap, &manifest, &bytes
            ),
            Err(RegisteredBootstrapExportError::InvalidJson(_))
        ));

        let mut nested: Value = serde_json::to_value(original).expect("JSON value");
        nested["group_dispositions"][1]["anticipated_output"] = json!(4);
        let bytes = serde_json::to_vec(&nested).expect("serialize");
        assert!(matches!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel, &bootstrap, &manifest, &bytes
            ),
            Err(RegisteredBootstrapExportError::InvalidJson(_))
        ));
    }

    #[test]
    fn oversized_input_is_rejected_before_decoding() {
        let bytes = vec![b' '; REGISTERED_BOOTSTRAP_EXPORT_MAX_BYTES + 1];
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        assert_eq!(
            verify_registered_bootstrap_export_index_bytes_v1(
                &kernel,
                &bootstrap,
                &manifest(),
                &bytes,
            )
            .expect_err("oversized sidecar"),
            RegisteredBootstrapExportError::InputTooLarge {
                actual: bytes.len(),
                maximum: REGISTERED_BOOTSTRAP_EXPORT_MAX_BYTES,
            }
        );
    }

    #[test]
    #[ignore = "maintenance helper: prints the canonical fresh export sidecar"]
    fn print_fresh_export_sidecar() {
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        println!(
            "{}",
            serde_json::to_string_pretty(&fresh_disclosure(&bootstrap, &manifest()))
                .expect("serialize sidecar")
        );
    }
}
