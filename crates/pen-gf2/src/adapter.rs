use crate::contract::{
    CANONICAL_ORDER_VERSION, CanonicalOrder, FINITE_FRAGMENT_SCHEMA_VERSION, FiniteFeature,
    FiniteFeatureDeclaration, FiniteFragmentLimits, FiniteResource, LAW_REQUEST_SCHEMA_VERSION,
    UncheckedFiniteFragmentManifest, UncheckedLawRequest, UncheckedLawSubject,
};
use pen_kernel::{
    Digest, Kernel, KernelError, OpenJudgment, ResourceKind, Term, UncheckedSignature,
};
use std::collections::BTreeSet;
use thiserror::Error;

/// A backend-independent interface for validating a fragment and deciding a
/// request within it.
///
/// The first native request schema supports only `KernelJudgment`: signature
/// checking, context checking, normalization, typing, and definitional
/// equality are replayed together by `pen-kernel`. Opaque fragment artifacts
/// are membership-checked and refused because this adapter has no verifier for
/// their derivations, equivalences, or closed specializations. Adding those
/// operations requires a new request schema and proof-producing backend.
pub trait LawKernel {
    type UncheckedManifest: ?Sized;
    type UncheckedRequest: ?Sized;
    type VerifiedFragment;
    type Proven;
    type Refuted;
    type FragmentError;

    fn verify_fragment(
        &self,
        manifest: &Self::UncheckedManifest,
    ) -> Result<Self::VerifiedFragment, Self::FragmentError>;

    fn decide(
        &self,
        fragment: &Self::VerifiedFragment,
        request: &Self::UncheckedRequest,
    ) -> LawDecision<Self::Proven, Self::Refuted>;
}

/// The four possible verifier dispositions.
///
/// `Refuted` requires a backend-produced refutation handle. A failed proof
/// replay is not sufficient to construct one.
#[must_use]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LawDecision<P, R> {
    Proven(P),
    Refuted(R),
    OutsideFragment(OutsideFragment),
    ResourceExhausted(ResourceExhausted),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutsideReason {
    UnsupportedRequestVersion,
    InputExceedsFragmentLimit,
    SubjectAbsentFromCarrier,
    NativeBackendUnavailable,
    BackendCouldNotCertify,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutsideFragment {
    reason: OutsideReason,
    feature: Option<FiniteFeature>,
}

impl OutsideFragment {
    pub fn reason(&self) -> OutsideReason {
        self.reason
    }

    pub fn feature(&self) -> Option<FiniteFeature> {
        self.feature
    }

    fn new(reason: OutsideReason, feature: Option<FiniteFeature>) -> Self {
        Self { reason, feature }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceExhausted {
    resource: FiniteResource,
}

impl ResourceExhausted {
    pub fn resource(&self) -> FiniteResource {
        self.resource
    }

    fn new(resource: FiniteResource) -> Self {
        Self { resource }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum FragmentValidationError {
    #[error("unsupported finite-fragment schema version {0}")]
    UnsupportedSchemaVersion(u16),
    #[error("unsupported canonical-order version {0}")]
    UnsupportedCanonicalOrderVersion(u16),
    #[error("unsupported canonical-order declaration")]
    UnsupportedCanonicalOrder,
    #[error("finite feature declarations are missing, duplicated, or out of canonical order")]
    FeatureDeclarationOrder,
    #[error("entries for {0:?} are duplicated or out of canonical order")]
    EntryOrder(FiniteFeature),
    #[error("finite inventory exceeds its declared bound")]
    InventoryLimitExceeded,
    #[error("finite limit {0} is zero, inconsistent, or above its hard ceiling")]
    InvalidLimit(&'static str),
    #[error("kernel protocol digest does not match the native backend")]
    KernelProtocolDigestMismatch,
    #[error("normalizer protocol digest does not match the native backend")]
    NormalizerProtocolDigestMismatch,
}

/// Structurally validated fragment capability.
///
/// Its fields and constructor are private, and it deliberately has no
/// deserialization implementation. Possession proves manifest validation, not
/// semantic completeness of the declared inventories.
#[derive(Clone, Debug)]
pub struct VerifiedFiniteFragment {
    digest: Digest,
    limits: FiniteFragmentLimits,
    syntax: Vec<FiniteFeatureDeclaration>,
    kernel: Kernel,
}

impl VerifiedFiniteFragment {
    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn limits(&self) -> FiniteFragmentLimits {
        self.limits
    }

    pub fn feature_entries(&self, feature: FiniteFeature) -> &[Digest] {
        self.syntax
            .binary_search_by_key(&feature, |declaration| declaration.feature)
            .map_or(&[], |position| self.syntax[position].entries.as_slice())
    }

    pub fn contains_artifact(&self, feature: FiniteFeature, digest: &Digest) -> bool {
        self.feature_entries(feature).binary_search(digest).is_ok()
    }
}

/// Successful native replay. The normalized result may be inspected, but a
/// caller cannot construct or deserialize this capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeVerifiedProof {
    fragment_digest: Digest,
    signature_digest: Digest,
    subject_digest: Digest,
    normalized_judgment: OpenJudgment,
}

impl NativeVerifiedProof {
    pub fn fragment_digest(&self) -> &Digest {
        &self.fragment_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn subject_digest(&self) -> &Digest {
        &self.subject_digest
    }

    pub fn normalized_judgment(&self) -> &OpenJudgment {
        &self.normalized_judgment
    }
}

/// Reserved native refutation capability.
///
/// The current dependent-core kernel checks positive judgments but exposes no
/// proof object for their negations. Consequently this type has no public or
/// native constructor, and the adapter never emits `Refuted`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeVerifiedRefutation {
    _private: (),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NativeKernelAdapter;

impl NativeKernelAdapter {
    pub fn verify_fragment(
        &self,
        manifest: &UncheckedFiniteFragmentManifest,
    ) -> Result<VerifiedFiniteFragment, FragmentValidationError> {
        verify_native_fragment(manifest)
    }

    pub fn decide(
        &self,
        fragment: &VerifiedFiniteFragment,
        request: &UncheckedLawRequest,
    ) -> LawDecision<NativeVerifiedProof, NativeVerifiedRefutation> {
        decide_native(fragment, request)
    }
}

impl LawKernel for NativeKernelAdapter {
    type UncheckedManifest = UncheckedFiniteFragmentManifest;
    type UncheckedRequest = UncheckedLawRequest;
    type VerifiedFragment = VerifiedFiniteFragment;
    type Proven = NativeVerifiedProof;
    type Refuted = NativeVerifiedRefutation;
    type FragmentError = FragmentValidationError;

    fn verify_fragment(
        &self,
        manifest: &Self::UncheckedManifest,
    ) -> Result<Self::VerifiedFragment, Self::FragmentError> {
        verify_native_fragment(manifest)
    }

    fn decide(
        &self,
        fragment: &Self::VerifiedFragment,
        request: &Self::UncheckedRequest,
    ) -> LawDecision<Self::Proven, Self::Refuted> {
        decide_native(fragment, request)
    }
}

fn verify_native_fragment(
    manifest: &UncheckedFiniteFragmentManifest,
) -> Result<VerifiedFiniteFragment, FragmentValidationError> {
    if manifest.schema_version != FINITE_FRAGMENT_SCHEMA_VERSION {
        return Err(FragmentValidationError::UnsupportedSchemaVersion(
            manifest.schema_version,
        ));
    }
    if manifest.canonical_order_version != CANONICAL_ORDER_VERSION {
        return Err(FragmentValidationError::UnsupportedCanonicalOrderVersion(
            manifest.canonical_order_version,
        ));
    }
    if manifest.canonical_order != CanonicalOrder::FeatureThenDigestV1 {
        return Err(FragmentValidationError::UnsupportedCanonicalOrder);
    }
    if let Some(field) = manifest.limits.invalid_field() {
        return Err(FragmentValidationError::InvalidLimit(field));
    }
    if manifest.syntax.len() != FiniteFeature::ALL.len()
        || manifest
            .syntax
            .iter()
            .zip(FiniteFeature::ALL)
            .any(|(declaration, expected)| declaration.feature != expected)
    {
        return Err(FragmentValidationError::FeatureDeclarationOrder);
    }

    let mut inventory_entries = 0_u32;
    for declaration in &manifest.syntax {
        if declaration
            .entries
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        {
            return Err(FragmentValidationError::EntryOrder(declaration.feature));
        }
        inventory_entries = inventory_entries
            .checked_add(
                u32::try_from(declaration.entries.len())
                    .map_err(|_| FragmentValidationError::InventoryLimitExceeded)?,
            )
            .ok_or(FragmentValidationError::InventoryLimitExceeded)?;
        if inventory_entries > manifest.limits.max_inventory_entries {
            return Err(FragmentValidationError::InventoryLimitExceeded);
        }
    }

    let kernel = Kernel::new(manifest.limits.kernel_limits())
        .map_err(|_| FragmentValidationError::InvalidLimit("kernel_limits"))?;
    if manifest.kernel_protocol_digest != kernel.kernel_protocol_digest() {
        return Err(FragmentValidationError::KernelProtocolDigestMismatch);
    }
    if manifest.normalizer_protocol_digest != kernel.normalizer_protocol_digest() {
        return Err(FragmentValidationError::NormalizerProtocolDigestMismatch);
    }

    Ok(VerifiedFiniteFragment {
        digest: manifest.canonical_digest(),
        limits: manifest.limits,
        syntax: manifest.syntax.clone(),
        kernel,
    })
}

fn decide_native(
    fragment: &VerifiedFiniteFragment,
    request: &UncheckedLawRequest,
) -> LawDecision<NativeVerifiedProof, NativeVerifiedRefutation> {
    if request.schema_version != LAW_REQUEST_SCHEMA_VERSION {
        return outside(OutsideReason::UnsupportedRequestVersion, None);
    }

    match &request.subject {
        UncheckedLawSubject::FragmentArtifact {
            feature,
            subject_digest,
        } => {
            if !fragment.contains_artifact(*feature, subject_digest) {
                return outside(OutsideReason::SubjectAbsentFromCarrier, Some(*feature));
            }
            outside(OutsideReason::NativeBackendUnavailable, Some(*feature))
        }
        UncheckedLawSubject::KernelJudgment {
            signature,
            judgment,
        } => decide_kernel_judgment(fragment, signature, judgment),
    }
}

fn decide_kernel_judgment(
    fragment: &VerifiedFiniteFragment,
    signature: &UncheckedSignature,
    judgment: &OpenJudgment,
) -> LawDecision<NativeVerifiedProof, NativeVerifiedRefutation> {
    if let Err(failure) = preflight_kernel_input(fragment, signature, judgment) {
        return failure.into_decision();
    }

    let verified_signature = match fragment.kernel.verify_signature(signature) {
        Ok(signature) => signature,
        Err(error) => return map_kernel_error(error),
    };
    let normalized_judgment = match fragment
        .kernel
        .verify_open_judgment(&verified_signature, judgment)
    {
        Ok(judgment) => judgment,
        Err(error) => return map_kernel_error(error),
    };
    let subject_digest = match fragment
        .kernel
        .judgment_subject_digest(&verified_signature, &normalized_judgment)
    {
        Ok(digest) => digest,
        Err(error) => return map_kernel_error(error),
    };

    LawDecision::Proven(NativeVerifiedProof {
        fragment_digest: fragment.digest.clone(),
        signature_digest: verified_signature.digest().clone(),
        subject_digest,
        normalized_judgment,
    })
}

type NativeDecision = LawDecision<NativeVerifiedProof, NativeVerifiedRefutation>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PreflightFailure {
    Outside(OutsideReason, Option<FiniteFeature>),
    Exhausted(FiniteResource),
}

impl PreflightFailure {
    fn into_decision(self) -> NativeDecision {
        match self {
            Self::Outside(reason, feature) => outside(reason, feature),
            Self::Exhausted(resource) => exhausted(resource),
        }
    }
}

fn preflight_kernel_input(
    fragment: &VerifiedFiniteFragment,
    signature: &UncheckedSignature,
    judgment: &OpenJudgment,
) -> Result<(), PreflightFailure> {
    let limits = fragment.limits;
    if signature.declarations.len() > limits.max_signature_declarations as usize {
        return Err(PreflightFailure::Outside(
            OutsideReason::InputExceedsFragmentLimit,
            None,
        ));
    }
    if judgment.context().0.len() > limits.max_context_entries as usize {
        return Err(PreflightFailure::Outside(
            OutsideReason::InputExceedsFragmentLimit,
            None,
        ));
    }

    for declaration in &signature.declarations {
        if !fragment.contains_artifact(FiniteFeature::OpaqueConstants, &declaration.id.0) {
            return Err(PreflightFailure::Outside(
                OutsideReason::SubjectAbsentFromCarrier,
                Some(FiniteFeature::OpaqueConstants),
            ));
        }
    }

    let mut roots = Vec::new();
    for declaration in &signature.declarations {
        roots.push(&declaration.ty);
        if let Some(body) = &declaration.body {
            roots.push(body);
        }
    }
    roots.extend(judgment.context().0.iter());
    match judgment {
        OpenJudgment::TypeFormation { term, .. } => roots.push(term),
        OpenJudgment::HasType { term, ty, .. } => {
            roots.push(term);
            roots.push(ty);
        }
        OpenJudgment::DefinitionallyEqual {
            left, right, ty, ..
        } => {
            roots.push(left);
            roots.push(right);
            roots.push(ty);
        }
    }

    let mut stack: Vec<(&Term, u16)> = roots.into_iter().map(|term| (term, 0)).collect();
    let mut visited = 0_u32;
    let mut used_features = BTreeSet::from([
        FiniteFeature::DependentContextsAndTelescopes,
        FiniteFeature::FiniteUniverseLevels,
    ]);
    if !signature.declarations.is_empty() {
        used_features.insert(FiniteFeature::OpaqueConstants);
    }

    while let Some((term, depth)) = stack.pop() {
        if depth > limits.max_term_depth {
            return Err(PreflightFailure::Exhausted(FiniteResource::TermDepth));
        }
        visited = match visited.checked_add(1) {
            Some(count) if count <= limits.max_operations => count,
            _ => return Err(PreflightFailure::Exhausted(FiniteResource::Operations)),
        };
        let child_depth = match depth.checked_add(1) {
            Some(depth) => depth,
            None => return Err(PreflightFailure::Exhausted(FiniteResource::TermDepth)),
        };
        match term {
            Term::Sort { level } => {
                if *level > limits.max_universe_level {
                    return Err(PreflightFailure::Outside(
                        OutsideReason::InputExceedsFragmentLimit,
                        None,
                    ));
                }
            }
            Term::Var { .. } | Term::UnitType | Term::Unit => {}
            Term::Global { .. } => {
                used_features.insert(FiniteFeature::OpaqueConstants);
            }
            Term::Pi { parameter, body } => {
                used_features.insert(FiniteFeature::DependentProducts);
                stack.push((parameter, child_depth));
                stack.push((body, child_depth));
            }
            Term::Sigma { parameter, body } => {
                used_features.insert(FiniteFeature::DependentSums);
                stack.push((parameter, child_depth));
                stack.push((body, child_depth));
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                used_features.insert(FiniteFeature::DependentProducts);
                stack.push((parameter_type, child_depth));
                stack.push((body, child_depth));
            }
            Term::Apply { function, argument } => {
                used_features.insert(FiniteFeature::DependentProducts);
                stack.push((function, child_depth));
                stack.push((argument, child_depth));
            }
            Term::Pair {
                sigma_type,
                first,
                second,
            } => {
                used_features.insert(FiniteFeature::DependentSums);
                stack.push((sigma_type, child_depth));
                stack.push((first, child_depth));
                stack.push((second, child_depth));
            }
            Term::First { pair } | Term::Second { pair } => {
                used_features.insert(FiniteFeature::DependentSums);
                stack.push((pair, child_depth));
            }
        }
    }

    for feature in used_features {
        if fragment
            .syntax
            .binary_search_by_key(&feature, |declaration| declaration.feature)
            .is_err()
        {
            return Err(PreflightFailure::Outside(
                OutsideReason::NativeBackendUnavailable,
                Some(feature),
            ));
        }
    }
    Ok(())
}

fn map_kernel_error(error: KernelError) -> NativeDecision {
    match error {
        KernelError::ResourceExhausted(resource) => exhausted(map_resource(resource)),
        _ => outside(OutsideReason::BackendCouldNotCertify, None),
    }
}

fn map_resource(resource: ResourceKind) -> FiniteResource {
    match resource {
        ResourceKind::Operations => FiniteResource::Operations,
        ResourceKind::Depth => FiniteResource::TermDepth,
        ResourceKind::Normalization => FiniteResource::NormalizationFuel,
    }
}

fn outside(reason: OutsideReason, feature: Option<FiniteFeature>) -> NativeDecision {
    LawDecision::OutsideFragment(OutsideFragment::new(reason, feature))
}

fn exhausted(resource: FiniteResource) -> NativeDecision {
    LawDecision::ResourceExhausted(ResourceExhausted::new(resource))
}

#[cfg(test)]
mod tests {
    use super::{FragmentValidationError, LawDecision, NativeKernelAdapter, OutsideReason};
    use crate::{
        CANONICAL_ORDER_VERSION, CanonicalOrder, DECISION_CLAIM_SCHEMA_VERSION,
        FINITE_FRAGMENT_SCHEMA_VERSION, FiniteFeature, FiniteFeatureDeclaration,
        FiniteFragmentLimits, LAW_REQUEST_SCHEMA_VERSION, UncheckedDecisionClaim,
        UncheckedFiniteFragmentManifest, UncheckedLawRequest, UncheckedLawSubject,
    };
    use pen_kernel::{DependentContext, Digest, Kernel, OpenJudgment, Term, UncheckedSignature};

    fn manifest() -> UncheckedFiniteFragmentManifest {
        let limits = FiniteFragmentLimits::default();
        let kernel = Kernel::new(limits.kernel_limits()).expect("valid test limits");
        UncheckedFiniteFragmentManifest {
            schema_version: FINITE_FRAGMENT_SCHEMA_VERSION,
            canonical_order_version: CANONICAL_ORDER_VERSION,
            canonical_order: CanonicalOrder::FeatureThenDigestV1,
            limits,
            kernel_protocol_digest: kernel.kernel_protocol_digest(),
            normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
            syntax: FiniteFeature::ALL
                .into_iter()
                .map(|feature| FiniteFeatureDeclaration {
                    feature,
                    entries: Vec::new(),
                })
                .collect(),
        }
    }

    fn formation(term: Term) -> UncheckedLawRequest {
        UncheckedLawRequest {
            schema_version: LAW_REQUEST_SCHEMA_VERSION,
            subject: UncheckedLawSubject::KernelJudgment {
                signature: UncheckedSignature::default(),
                judgment: OpenJudgment::TypeFormation {
                    context: DependentContext::default(),
                    term,
                },
            },
        }
    }

    #[test]
    fn successful_replay_returns_an_opaque_proof_handle() {
        let adapter = NativeKernelAdapter;
        let fragment = adapter
            .verify_fragment(&manifest())
            .expect("valid fragment");
        let decision = adapter.decide(&fragment, &formation(Term::UnitType));
        let LawDecision::Proven(proof) = decision else {
            panic!("supported formation must replay");
        };
        assert_eq!(proof.fragment_digest(), fragment.digest());
        assert!(matches!(
            proof.normalized_judgment(),
            OpenJudgment::TypeFormation {
                term: Term::UnitType,
                ..
            }
        ));
    }

    #[test]
    fn native_backend_fails_closed_for_unimplemented_syntax_families() {
        let mut input = manifest();
        let subject = Digest::of_bytes(b"anonymous-subject");
        for feature in [
            FiniteFeature::FiniteSumsWithComputation,
            FiniteFeature::FinitePathOperations,
            FiniteFeature::FiniteCubicalOperations,
        ] {
            input
                .syntax
                .iter_mut()
                .find(|declaration| declaration.feature == feature)
                .expect("required declaration")
                .entries
                .push(subject.clone());
        }
        let adapter = NativeKernelAdapter;
        let fragment = adapter.verify_fragment(&input).expect("valid fragment");

        for feature in [
            FiniteFeature::FiniteSumsWithComputation,
            FiniteFeature::FinitePathOperations,
            FiniteFeature::FiniteCubicalOperations,
        ] {
            let decision = adapter.decide(
                &fragment,
                &UncheckedLawRequest {
                    schema_version: LAW_REQUEST_SCHEMA_VERSION,
                    subject: UncheckedLawSubject::FragmentArtifact {
                        feature,
                        subject_digest: subject.clone(),
                    },
                },
            );
            let LawDecision::OutsideFragment(outside) = decision else {
                panic!("unsupported syntax must remain outside");
            };
            assert_eq!(outside.reason(), OutsideReason::NativeBackendUnavailable);
            assert_eq!(outside.feature(), Some(feature));
        }
    }

    #[test]
    fn failed_positive_replay_is_not_a_refutation() {
        let adapter = NativeKernelAdapter;
        let fragment = adapter
            .verify_fragment(&manifest())
            .expect("valid fragment");
        let request = UncheckedLawRequest {
            schema_version: LAW_REQUEST_SCHEMA_VERSION,
            subject: UncheckedLawSubject::KernelJudgment {
                signature: UncheckedSignature::default(),
                judgment: OpenJudgment::HasType {
                    context: DependentContext::default(),
                    term: Term::Apply {
                        function: Box::new(Term::Unit),
                        argument: Box::new(Term::Unit),
                    },
                    ty: Term::UnitType,
                },
            },
        };
        let LawDecision::OutsideFragment(outside) = adapter.decide(&fragment, &request) else {
            panic!("a checker rejection is not a negative theorem");
        };
        assert_eq!(outside.reason(), OutsideReason::BackendCouldNotCertify);
    }

    #[test]
    fn bounded_preflight_reports_exhaustion_without_recursive_scanning() {
        let mut input = manifest();
        input.limits.max_term_depth = 8;
        let adapter = NativeKernelAdapter;
        let fragment = adapter.verify_fragment(&input).expect("valid fragment");
        let mut term = Term::Unit;
        for _ in 0..32 {
            term = Term::First {
                pair: Box::new(term),
            };
        }
        let LawDecision::ResourceExhausted(exhaustion) =
            adapter.decide(&fragment, &formation(term))
        else {
            panic!("deep input must exhaust its declared depth");
        };
        assert_eq!(exhaustion.resource(), crate::FiniteResource::TermDepth);
    }

    #[test]
    fn operational_exhaustion_is_distinct_from_refutation() {
        let mut input = manifest();
        input.limits.max_operations = 1;
        let adapter = NativeKernelAdapter;
        let fragment = adapter.verify_fragment(&input).expect("valid fragment");
        assert!(matches!(
            adapter.decide(&fragment, &formation(Term::UnitType)),
            LawDecision::ResourceExhausted(_)
        ));
    }

    #[test]
    fn manifest_requires_the_exact_feature_order_and_ordered_entries() {
        let adapter = NativeKernelAdapter;
        let mut missing = manifest();
        missing.syntax.pop();
        assert_eq!(
            adapter
                .verify_fragment(&missing)
                .expect_err("missing feature"),
            FragmentValidationError::FeatureDeclarationOrder
        );

        let mut unordered = manifest();
        let feature = FiniteFeature::OpaqueConstants;
        let declaration = unordered
            .syntax
            .iter_mut()
            .find(|declaration| declaration.feature == feature)
            .expect("declaration");
        declaration.entries = vec![Digest::of_bytes(b"left"), Digest::of_bytes(b"right")];
        declaration.entries.sort();
        declaration.entries.reverse();
        assert_eq!(
            adapter
                .verify_fragment(&unordered)
                .expect_err("unordered inventory"),
            FragmentValidationError::EntryOrder(feature)
        );
    }

    #[test]
    fn declared_inventory_and_hard_limits_are_enforced_before_hashing() {
        let adapter = NativeKernelAdapter;
        let mut inventory = manifest();
        inventory.limits.max_inventory_entries = 1;
        let declaration = inventory
            .syntax
            .iter_mut()
            .find(|declaration| declaration.feature == FiniteFeature::OpaqueConstants)
            .expect("declaration");
        declaration.entries = vec![Digest::of_bytes(b"one"), Digest::of_bytes(b"two")];
        declaration.entries.sort();
        assert_eq!(
            adapter
                .verify_fragment(&inventory)
                .expect_err("inventory bound"),
            FragmentValidationError::InventoryLimitExceeded
        );

        let mut invalid = manifest();
        invalid.limits.max_operations = u32::MAX;
        assert_eq!(
            adapter
                .verify_fragment(&invalid)
                .expect_err("hard operation ceiling"),
            FragmentValidationError::InvalidLimit("max_operations")
        );
    }

    #[test]
    fn canonical_digest_binds_limits_and_backend_identity() {
        let first = manifest();
        let mut second = first.clone();
        second.limits.max_context_entries -= 1;
        assert_ne!(first.canonical_digest(), second.canonical_digest());

        let mut forged = first;
        forged.kernel_protocol_digest = Digest::of_bytes(b"forged-backend");
        assert_eq!(
            NativeKernelAdapter
                .verify_fragment(&forged)
                .expect_err("identity mismatch"),
            FragmentValidationError::KernelProtocolDigestMismatch
        );
    }

    #[test]
    fn serde_rejects_unknown_fields_at_every_wire_boundary() {
        let mut manifest_json = serde_json::to_value(manifest()).expect("serialize manifest");
        manifest_json
            .as_object_mut()
            .expect("manifest object")
            .insert("extra".to_owned(), serde_json::json!(true));
        assert!(serde_json::from_value::<UncheckedFiniteFragmentManifest>(manifest_json).is_err());

        let mut request_json =
            serde_json::to_value(formation(Term::UnitType)).expect("serialize request");
        request_json
            .as_object_mut()
            .expect("request object")
            .insert("extra".to_owned(), serde_json::json!(true));
        assert!(serde_json::from_value::<UncheckedLawRequest>(request_json).is_err());

        let digest = Digest::of_bytes(b"claim");
        let claim = UncheckedDecisionClaim::Proven {
            schema_version: DECISION_CLAIM_SCHEMA_VERSION,
            fragment_digest: digest.clone(),
            subject_digest: digest.clone(),
            evidence_digest: digest,
        };
        let mut claim_json = serde_json::to_value(claim).expect("serialize claim");
        claim_json
            .as_object_mut()
            .expect("claim object")
            .insert("extra".to_owned(), serde_json::json!(true));
        assert!(serde_json::from_value::<UncheckedDecisionClaim>(claim_json).is_err());
    }

    #[test]
    fn malformed_and_unlisted_artifacts_remain_outside() {
        let adapter = NativeKernelAdapter;
        let fragment = adapter
            .verify_fragment(&manifest())
            .expect("valid fragment");
        let decision = adapter.decide(
            &fragment,
            &UncheckedLawRequest {
                schema_version: LAW_REQUEST_SCHEMA_VERSION,
                subject: UncheckedLawSubject::FragmentArtifact {
                    feature: FiniteFeature::FinitePathOperations,
                    subject_digest: Digest::of_bytes(b"not-listed"),
                },
            },
        );
        let LawDecision::OutsideFragment(outside) = decision else {
            panic!("unlisted subject must remain outside");
        };
        assert_eq!(outside.reason(), OutsideReason::SubjectAbsentFromCarrier);

        let mut wrong_version = formation(Term::UnitType);
        wrong_version.schema_version += 1;
        let LawDecision::OutsideFragment(outside) = adapter.decide(&fragment, &wrong_version)
        else {
            panic!("unknown request schema must remain outside");
        };
        assert_eq!(outside.reason(), OutsideReason::UnsupportedRequestVersion);
    }
}
