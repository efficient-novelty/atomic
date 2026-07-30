//! Verified history view for the registered Law-V2A founding prefix.
//!
//! The existing [`crate::UncheckedHistory`] wire type describes later sealed
//! extensions and is intentionally not reused here: the registered bootstrap
//! is an explicit initial condition, not a free-sealing certificate.  This
//! module derives a private history capability directly from the already
//! verified bootstrap and its separately verified semantic export index.

use crate::{
    DEMAND_WINDOW_WIDTH, EventId, REGISTERED_BOOTSTRAP_ACT_COUNT,
    UncheckedRegisteredGroupDispositionV1, VerifiedRegisteredBootstrap,
    VerifiedRegisteredBootstrapExportIndexV1,
};
use pen_demand::gsc::{
    CLOSED_FORMER_FRAME_SCHEMA_VERSION, ClosedFormerFrame, ConstructorPortId, EquationClauseCode,
    GscOriginEventId, GscOutcome, GscRule, GscUnknownReason, IntroductionAlias, OutputClause,
    OutputRole, PortKey, PortRef, PublicSourceId, VerifiedCanonicalDemandFamilyV2,
    VerifiedClosedFormerFrame, VerifiedGscSemanticManifest, compile_compute_v1, compile_use_v1,
    verify_closed_former_frame, verify_closed_inductive_code,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Digest, GlobalId, Kernel, KernelError, UncheckedSignature,
    VerifiedSignature,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

const EMPTY_HISTORY_DOMAIN: &str = "law-v2-registered-empty-history-v1";
const EVENT_EXPORT_DOMAIN: &str = "law-v2-registered-event-export-v1";
const EVENT_ID_DOMAIN: &str = "law-v2-registered-event-id-v1";
const HISTORY_STEP_DOMAIN: &str = "law-v2-registered-history-step-v1";
const NORMALIZED_EXTENSION_DOMAIN: &str = "law-v2-registered-normalized-extension-v1";
const ANCHOR_DOMAIN: &str = "law-v2-verified-anchor-v1";
const Q3_CUTOFF_DOMAIN: &str = "law-v2-origin-cutoff-q3-registry-v1";
const REGISTERED_FRAME_BINDING_DOMAIN: &str = "law-v2-registered-closed-former-binding-v1";
const REGISTERED_FAMILY_BINDING_DOMAIN: &str = "law-v2-registered-demand-family-binding-v1";
const REGISTERED_COMPUTE_TUPLE_BINDING_DOMAIN: &str =
    "law-v2-registered-compute-demand-tuple-binding-v1";
const REGISTERED_FRAME_TUPLE_BINDING_DOMAIN: &str =
    "law-v2-registered-frame-demand-tuple-binding-v1";
const REGISTERED_ACTIVE_INVENTORY_BINDING_DOMAIN: &str =
    "law-v2-registered-active-demand-inventory-binding-v1";
const REGISTERED_INVENTORY_PORT_DOMAIN: &str = "law-v2-registered-active-demand-inventory-port-v1";
const REGISTERED_INVENTORY_CONSTRUCTOR_DOMAIN: &str =
    "law-v2-registered-active-demand-inventory-constructor-v1";

/// Why the registered prefix could not be promoted to a verified history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegisteredHistoryError {
    BootstrapBinding,
    BoundaryCount,
    BoundarySnapshot { ordinal: u16 },
    BoundaryDigest { ordinal: u16 },
    DuplicateDeclarationOrigin,
    MissingDeclarationOrigin,
    InvalidGroupActivation { ordinal: u16 },
    NonEmptyQ3Registry,
    ActiveAnchor,
    Kernel(KernelError),
}

impl fmt::Display for RegisteredHistoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BootstrapBinding => {
                formatter.write_str("export index is bound to a different bootstrap")
            }
            Self::BoundaryCount => formatter.write_str(
                "registered boundary does not have one declaration for every founding act",
            ),
            Self::BoundarySnapshot { ordinal } => {
                write!(formatter, "cannot reconstruct boundary snapshot {ordinal}")
            }
            Self::BoundaryDigest { ordinal } => {
                write!(
                    formatter,
                    "boundary snapshot {ordinal} has the wrong digest"
                )
            }
            Self::DuplicateDeclarationOrigin => {
                formatter.write_str("a declaration has more than one registered origin")
            }
            Self::MissingDeclarationOrigin => {
                formatter.write_str("a final-boundary declaration has no registered origin")
            }
            Self::InvalidGroupActivation { ordinal } => {
                write!(
                    formatter,
                    "an export group has invalid activation ordinal {ordinal}"
                )
            }
            Self::NonEmptyQ3Registry => formatter
                .write_str("the registered bootstrap profile does not support explicit Q3 exports"),
            Self::ActiveAnchor => {
                formatter.write_str("registered history has no immediate width-two active anchor")
            }
            Self::Kernel(error) => write!(formatter, "kernel rejected history replay: {error}"),
        }
    }
}

impl std::error::Error for RegisteredHistoryError {}

impl From<KernelError> for RegisteredHistoryError {
    fn from(error: KernelError) -> Self {
        Self::Kernel(error)
    }
}

/// Complete groups that become usable at one event.
///
/// This is deliberately called an activation index, not an exhaustive
/// per-event partition.  A constructor group may span several founding acts;
/// it becomes available only when its last source declaration exists.
#[derive(Clone, Debug)]
pub struct VerifiedEventExportIndex {
    activated_group_digests: Vec<Digest>,
    finalized_bootstrap_index_digest: Option<Digest>,
    digest: Digest,
}

impl VerifiedEventExportIndex {
    pub fn activated_group_digests(&self) -> &[Digest] {
        &self.activated_group_digests
    }

    pub fn finalized_bootstrap_index_digest(&self) -> Option<&Digest> {
        self.finalized_bootstrap_index_digest.as_ref()
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// One verified event in the registered founding prefix.
#[derive(Clone, Debug)]
pub struct VerifiedEvent {
    event_id: EventId,
    ordinal: u16,
    predecessor: Option<EventId>,
    pre_history_digest: Digest,
    post_history_digest: Digest,
    pre_boundary_digest: Digest,
    post_boundary_digest: Digest,
    extension_digest: Digest,
    source_identity: Digest,
    binding_identity: Digest,
    export_index: VerifiedEventExportIndex,
}

impl VerifiedEvent {
    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    pub fn ordinal(&self) -> u16 {
        self.ordinal
    }

    pub fn predecessor(&self) -> Option<&EventId> {
        self.predecessor.as_ref()
    }

    pub fn pre_history_digest(&self) -> &Digest {
        &self.pre_history_digest
    }

    pub fn post_history_digest(&self) -> &Digest {
        &self.post_history_digest
    }

    pub fn pre_boundary_digest(&self) -> &Digest {
        &self.pre_boundary_digest
    }

    pub fn post_boundary_digest(&self) -> &Digest {
        &self.post_boundary_digest
    }

    /// Digest of the exact normalized declaration extension for this event.
    pub fn extension_digest(&self) -> &Digest {
        &self.extension_digest
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn binding_identity(&self) -> &Digest {
        &self.binding_identity
    }

    pub fn export_index(&self) -> &VerifiedEventExportIndex {
        &self.export_index
    }
}

/// An ordered width-two anchor with a stable prefix cutoff.
#[derive(Clone, Debug)]
pub struct VerifiedAnchor {
    newer: EventId,
    older: EventId,
    newer_ordinal: u16,
    older_ordinal: u16,
    cutoff_snapshot_index: usize,
    cutoff_boundary_digest: Digest,
    anchor_digest: Digest,
}

impl VerifiedAnchor {
    pub fn newer(&self) -> &EventId {
        &self.newer
    }

    pub fn older(&self) -> &EventId {
        &self.older
    }

    pub fn newer_ordinal(&self) -> u16 {
        self.newer_ordinal
    }

    pub fn older_ordinal(&self) -> u16 {
        self.older_ordinal
    }

    pub fn cutoff_boundary_digest(&self) -> &Digest {
        &self.cutoff_boundary_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.anchor_digest
    }
}

/// Verified empty Q3 registry at one event-origin cutoff.
///
/// This capability proves only that the exhaustive registered export grammar
/// contains no explicit Q3 theorem at or before this cutoff.  It does not say
/// that arbitrary ambient encodings contain no equivalences.
#[derive(Clone, Debug)]
pub struct VerifiedOriginCutoffQ3Registry {
    cutoff_event: EventId,
    cutoff_boundary_digest: Digest,
    export_index_digest: Digest,
    digest: Digest,
}

impl VerifiedOriginCutoffQ3Registry {
    pub fn cutoff_event(&self) -> &EventId {
        &self.cutoff_event
    }

    pub fn cutoff_boundary_digest(&self) -> &Digest {
        &self.cutoff_boundary_digest
    }

    pub fn export_index_digest(&self) -> &Digest {
        &self.export_index_digest
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Whether the registered group partition is complete at one historical
/// cutoff.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CutoffExportCoverage {
    Exhaustive,
    IncompleteRegisteredGroups { group_digests: Vec<Digest> },
}

/// One principal source, including its exact registered origin testimony.
///
/// Fields are private so callers cannot construct a source that can be
/// inserted into a registered frame capability.
#[derive(Clone, Debug)]
pub struct VerifiedRegisteredPrincipalSource {
    declaration: GlobalId,
    public_source: PublicSourceId,
    origin: GscOriginEventId,
    registered_source_identity: Digest,
}

impl VerifiedRegisteredPrincipalSource {
    pub fn declaration(&self) -> &GlobalId {
        &self.declaration
    }

    pub fn public_source(&self) -> &PublicSourceId {
        &self.public_source
    }

    pub fn origin(&self) -> &GscOriginEventId {
        &self.origin
    }

    pub fn registered_source_identity(&self) -> &Digest {
        &self.registered_source_identity
    }
}

impl CanonicalEncode for VerifiedRegisteredPrincipalSource {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.declaration.encode_canonical(encoder);
        self.public_source.encode_canonical(encoder);
        self.origin.encode_canonical(encoder);
        self.registered_source_identity.encode_canonical(encoder);
    }
}

/// Authoritative Law-V2A wrapper around a semantic closed-former frame.
///
/// `pen-demand` intentionally permits callers to check generic frames.
/// Authority enters only here, where the frame is tied to the verified
/// registered history, exhaustive export group, active anchor, cutoff, and
/// exact per-declaration origins.
#[derive(Clone, Debug)]
pub struct VerifiedRegisteredClosedFormerFrame {
    inner: VerifiedClosedFormerFrame,
    semantic_manifest_digest: Digest,
    export_group_digest: Digest,
    export_index_digest: Digest,
    history_digest: Digest,
    active_anchor_digest: Digest,
    cutoff_boundary_digest: Digest,
    q3_registry_digest: Digest,
    principal_source_bindings: Vec<VerifiedRegisteredPrincipalSource>,
    cutoff_boundary: VerifiedSignature,
    cutoff_coverage_exhaustive: bool,
    binding_digest: Digest,
}

impl VerifiedRegisteredClosedFormerFrame {
    pub fn inner(&self) -> &VerifiedClosedFormerFrame {
        &self.inner
    }

    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn export_group_digest(&self) -> &Digest {
        &self.export_group_digest
    }

    pub fn export_index_digest(&self) -> &Digest {
        &self.export_index_digest
    }

    pub fn history_digest(&self) -> &Digest {
        &self.history_digest
    }

    pub fn active_anchor_digest(&self) -> &Digest {
        &self.active_anchor_digest
    }

    pub fn cutoff_boundary_digest(&self) -> &Digest {
        &self.cutoff_boundary_digest
    }

    pub fn q3_registry_digest(&self) -> &Digest {
        &self.q3_registry_digest
    }

    pub fn principal_source_bindings(&self) -> &[VerifiedRegisteredPrincipalSource] {
        &self.principal_source_bindings
    }

    pub fn cutoff_boundary(&self) -> &VerifiedSignature {
        &self.cutoff_boundary
    }

    pub fn cutoff_coverage_is_exhaustive(&self) -> bool {
        self.cutoff_coverage_exhaustive
    }

    pub fn binding_digest(&self) -> &Digest {
        &self.binding_digest
    }

    pub fn binding_is_valid(&self) -> bool {
        self.inner.code().manifest_digest() == self.semantic_manifest_digest()
            && self.cutoff_coverage_exhaustive
            && self.cutoff_boundary.digest() == self.cutoff_boundary_digest()
            && frame_sources_match_registered_bindings(&self.inner, &self.principal_source_bindings)
            && registered_frame_binding_digest(
                &self.inner,
                &self.semantic_manifest_digest,
                &self.export_group_digest,
                &self.export_index_digest,
                &self.history_digest,
                &self.active_anchor_digest,
                &self.cutoff_boundary_digest,
                &self.q3_registry_digest,
                &self.principal_source_bindings,
            ) == self.binding_digest
    }

    /// Compile the use family from the registered frame and its bound cutoff.
    pub fn compile_use(
        &self,
        kernel: &Kernel,
        semantic_manifest: &VerifiedGscSemanticManifest,
    ) -> GscOutcome<VerifiedRegisteredDemandFamilyV2> {
        if semantic_manifest.digest() != self.semantic_manifest_digest() {
            return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
        }
        if !self.binding_is_valid() {
            return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
        }
        let inner = match compile_use_v1(
            semantic_manifest,
            kernel,
            &self.cutoff_boundary,
            &self.inner,
        ) {
            GscOutcome::Proven(family) => family,
            GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
        };
        if !family_support_matches_frame(&inner, self)
            || inner.rule() != GscRule::Use
            || inner.rank() != 1
            || inner.ports().len() != 1
        {
            return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
        }
        let registered = VerifiedRegisteredDemandFamilyV2::for_use(inner, self);
        if !registered.binding_is_valid_for(self) {
            return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
        }
        GscOutcome::Proven(registered)
    }

    /// Compile one computation family while reusing the exact generated use
    /// port from the supplied registered use-family capability.
    pub fn compile_compute(
        &self,
        kernel: &Kernel,
        semantic_manifest: &VerifiedGscSemanticManifest,
        use_family: &VerifiedRegisteredDemandFamilyV2,
        constructor_ordinal: usize,
    ) -> GscOutcome<VerifiedRegisteredDemandFamilyV2> {
        if semantic_manifest.digest() != self.semantic_manifest_digest() {
            return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
        }
        if !use_family.is_use()
            || !self.binding_is_valid()
            || !use_family.binding_is_valid_for(self)
            || use_family.registered_frame_binding_digest() != self.binding_digest()
            || use_family.history_digest() != self.history_digest()
            || use_family.active_anchor_digest() != self.active_anchor_digest()
            || use_family.cutoff_boundary_digest() != self.cutoff_boundary_digest()
        {
            return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
        }
        let Some(use_port) = use_family.exact_use_port() else {
            return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
        };
        let Some(constructor) = self.inner.code().constructor_id(constructor_ordinal) else {
            return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
        };
        let inner = match compile_compute_v1(
            semantic_manifest,
            kernel,
            &self.cutoff_boundary,
            &self.inner,
            use_family.inner(),
            use_port,
            constructor,
        ) {
            GscOutcome::Proven(family) => family,
            GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
        };
        if !family_support_matches_frame(&inner, self)
            || inner.rule() != GscRule::Compute
            || inner.rank() != 2
        {
            return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
        }
        let registered = VerifiedRegisteredDemandFamilyV2::for_compute(
            inner,
            self,
            use_family,
            use_port.clone(),
            constructor.clone(),
        );
        if !registered.binding_is_valid_for(self) {
            return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
        }
        GscOutcome::Proven(registered)
    }
}

/// History-bound wrapper around a compiler-produced use or computation
/// family.  It has no public constructor or deserialization implementation.
#[derive(Clone, Debug)]
pub struct VerifiedRegisteredDemandFamilyV2 {
    inner: VerifiedCanonicalDemandFamilyV2,
    semantic_manifest_digest: Digest,
    registered_frame_binding_digest: Digest,
    export_group_digest: Digest,
    history_digest: Digest,
    active_anchor_digest: Digest,
    cutoff_boundary_digest: Digest,
    q3_registry_digest: Digest,
    exact_use_port: Option<PortKey>,
    source_use_family_binding_digest: Option<Digest>,
    constructor: Option<pen_demand::gsc::ConstructorPortId>,
    binding_digest: Digest,
}

impl VerifiedRegisteredDemandFamilyV2 {
    pub fn inner(&self) -> &VerifiedCanonicalDemandFamilyV2 {
        &self.inner
    }

    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn registered_frame_binding_digest(&self) -> &Digest {
        &self.registered_frame_binding_digest
    }

    pub fn export_group_digest(&self) -> &Digest {
        &self.export_group_digest
    }

    pub fn history_digest(&self) -> &Digest {
        &self.history_digest
    }

    pub fn active_anchor_digest(&self) -> &Digest {
        &self.active_anchor_digest
    }

    pub fn cutoff_boundary_digest(&self) -> &Digest {
        &self.cutoff_boundary_digest
    }

    pub fn q3_registry_digest(&self) -> &Digest {
        &self.q3_registry_digest
    }

    /// For a use family, this is its sole generated use port.  For a compute
    /// family, this is the exact source port reused by its premise.
    pub fn exact_use_port(&self) -> Option<&PortKey> {
        self.exact_use_port.as_ref()
    }

    pub fn source_use_family_binding_digest(&self) -> Option<&Digest> {
        self.source_use_family_binding_digest.as_ref()
    }

    pub fn constructor(&self) -> Option<&pen_demand::gsc::ConstructorPortId> {
        self.constructor.as_ref()
    }

    pub fn binding_digest(&self) -> &Digest {
        &self.binding_digest
    }

    pub fn is_use(&self) -> bool {
        self.inner.rule() == GscRule::Use
            && self.inner.rank() == 1
            && self.exact_use_port.is_some()
            && self.source_use_family_binding_digest.is_none()
            && self.constructor.is_none()
    }

    pub fn is_compute(&self) -> bool {
        self.inner.rule() == GscRule::Compute
            && self.inner.rank() == 2
            && self.exact_use_port.is_some()
            && self.source_use_family_binding_digest.is_some()
            && self.constructor.is_some()
    }

    pub fn binding_is_valid_for(&self, frame: &VerifiedRegisteredClosedFormerFrame) -> bool {
        frame.binding_is_valid()
            && self.inner.semantic_manifest_digest() == frame.semantic_manifest_digest()
            && self.semantic_manifest_digest == *frame.semantic_manifest_digest()
            && self.registered_frame_binding_digest == *frame.binding_digest()
            && self.export_group_digest == *frame.export_group_digest()
            && self.history_digest == *frame.history_digest()
            && self.active_anchor_digest == *frame.active_anchor_digest()
            && self.cutoff_boundary_digest == *frame.cutoff_boundary_digest()
            && self.q3_registry_digest == *frame.q3_registry_digest()
            && family_support_matches_frame(&self.inner, frame)
            && registered_family_binding_digest(
                &self.inner,
                frame,
                self.exact_use_port.as_ref(),
                self.source_use_family_binding_digest.as_ref(),
                self.constructor.as_ref(),
            ) == self.binding_digest
            && if self.is_use() {
                registered_use_shape_is_exact(self, frame)
            } else {
                self.is_compute() && registered_compute_shape_is_exact(self, frame)
            }
    }

    fn for_use(
        inner: VerifiedCanonicalDemandFamilyV2,
        frame: &VerifiedRegisteredClosedFormerFrame,
    ) -> Self {
        let exact_use_port = inner
            .ports()
            .first()
            .expect("validated use family has one port")
            .key()
            .clone();
        Self::new(inner, frame, Some(exact_use_port), None, None)
    }

    fn for_compute(
        inner: VerifiedCanonicalDemandFamilyV2,
        frame: &VerifiedRegisteredClosedFormerFrame,
        use_family: &Self,
        exact_use_port: PortKey,
        constructor: pen_demand::gsc::ConstructorPortId,
    ) -> Self {
        Self::new(
            inner,
            frame,
            Some(exact_use_port),
            Some(use_family.binding_digest().clone()),
            Some(constructor),
        )
    }

    fn new(
        inner: VerifiedCanonicalDemandFamilyV2,
        frame: &VerifiedRegisteredClosedFormerFrame,
        exact_use_port: Option<PortKey>,
        source_use_family_binding_digest: Option<Digest>,
        constructor: Option<pen_demand::gsc::ConstructorPortId>,
    ) -> Self {
        let binding_digest = registered_family_binding_digest(
            &inner,
            frame,
            exact_use_port.as_ref(),
            source_use_family_binding_digest.as_ref(),
            constructor.as_ref(),
        );
        Self {
            semantic_manifest_digest: frame.semantic_manifest_digest.clone(),
            registered_frame_binding_digest: frame.binding_digest.clone(),
            export_group_digest: frame.export_group_digest.clone(),
            history_digest: frame.history_digest.clone(),
            active_anchor_digest: frame.active_anchor_digest.clone(),
            cutoff_boundary_digest: frame.cutoff_boundary_digest.clone(),
            q3_registry_digest: frame.q3_registry_digest.clone(),
            inner,
            exact_use_port,
            source_use_family_binding_digest,
            constructor,
            binding_digest,
        }
    }
}

/// One registered computation tuple, tied to the exact generated Use port
/// selected for its enclosing frame tuple.
#[derive(Clone, Debug)]
pub struct VerifiedRegisteredComputeDemandTuple {
    constructor_ordinal: u64,
    constructor: ConstructorPortId,
    family: VerifiedRegisteredDemandFamilyV2,
    output_port: PortKey,
    output_port_digest: Digest,
    binding_digest: Digest,
}

impl VerifiedRegisteredComputeDemandTuple {
    pub fn constructor_ordinal(&self) -> u64 {
        self.constructor_ordinal
    }

    pub fn constructor(&self) -> &ConstructorPortId {
        &self.constructor
    }

    pub fn family(&self) -> &VerifiedRegisteredDemandFamilyV2 {
        &self.family
    }

    pub fn output_port(&self) -> &PortKey {
        &self.output_port
    }

    pub fn output_port_digest(&self) -> &Digest {
        &self.output_port_digest
    }

    pub fn binding_digest(&self) -> &Digest {
        &self.binding_digest
    }

    pub fn binding_is_valid_for(
        &self,
        frame: &VerifiedRegisteredClosedFormerFrame,
        use_family: &VerifiedRegisteredDemandFamilyV2,
        exact_use_port: &PortKey,
    ) -> bool {
        let Some(constructor_ordinal) = usize::try_from(self.constructor_ordinal).ok() else {
            return false;
        };
        frame.inner().code().constructor_id(constructor_ordinal) == Some(&self.constructor)
            && use_family.is_use()
            && use_family.binding_is_valid_for(frame)
            && use_family.exact_use_port() == Some(exact_use_port)
            && self.family.is_compute()
            && self.family.binding_is_valid_for(frame)
            && self.family.exact_use_port() == Some(exact_use_port)
            && self.family.source_use_family_binding_digest() == Some(use_family.binding_digest())
            && self.family.constructor() == Some(&self.constructor)
            && matches!(
                self.family.inner().ports(),
                [port] if port.key() == &self.output_port
            )
            && self.output_port_digest == registered_inventory_port_digest(&self.output_port)
            && registered_compute_tuple_binding_digest(
                self.constructor_ordinal,
                &self.constructor,
                frame,
                use_family,
                exact_use_port,
                &self.family,
                &self.output_port,
                &self.output_port_digest,
            ) == self.binding_digest
    }

    fn new(
        constructor_ordinal: u64,
        constructor: ConstructorPortId,
        family: VerifiedRegisteredDemandFamilyV2,
        frame: &VerifiedRegisteredClosedFormerFrame,
        use_family: &VerifiedRegisteredDemandFamilyV2,
        exact_use_port: &PortKey,
        output_port: PortKey,
    ) -> Self {
        let output_port_digest = registered_inventory_port_digest(&output_port);
        let binding_digest = registered_compute_tuple_binding_digest(
            constructor_ordinal,
            &constructor,
            frame,
            use_family,
            exact_use_port,
            &family,
            &output_port,
            &output_port_digest,
        );
        Self {
            constructor_ordinal,
            constructor,
            family,
            output_port,
            output_port_digest,
            binding_digest,
        }
    }
}

/// The complete Use-plus-all-Compute compiler tuple for one active registered
/// former.  Compute tuples are in constructor ordinal order.
#[derive(Clone, Debug)]
pub struct VerifiedRegisteredFrameDemandTuple {
    active_frame_ordinal: u64,
    frame: VerifiedRegisteredClosedFormerFrame,
    use_family: VerifiedRegisteredDemandFamilyV2,
    exact_use_port: PortKey,
    compute_families: Vec<VerifiedRegisteredComputeDemandTuple>,
    candidate_compute_order: Vec<u64>,
    binding_digest: Digest,
}

impl VerifiedRegisteredFrameDemandTuple {
    pub fn active_frame_ordinal(&self) -> u64 {
        self.active_frame_ordinal
    }

    pub fn frame(&self) -> &VerifiedRegisteredClosedFormerFrame {
        &self.frame
    }

    pub fn use_family(&self) -> &VerifiedRegisteredDemandFamilyV2 {
        &self.use_family
    }

    pub fn exact_use_port(&self) -> &PortKey {
        &self.exact_use_port
    }

    pub fn compute_families(&self) -> &[VerifiedRegisteredComputeDemandTuple] {
        &self.compute_families
    }

    /// Indices into [`Self::compute_families`] sorted by exact Compute output
    /// port digest, as required by the frozen response-candidate grammar.
    pub fn candidate_compute_order(&self) -> &[u64] {
        &self.candidate_compute_order
    }

    pub fn candidate_compute_families_in_port_order(
        &self,
    ) -> impl Iterator<Item = &VerifiedRegisteredComputeDemandTuple> {
        self.candidate_compute_order
            .iter()
            .filter_map(|ordinal| usize::try_from(*ordinal).ok())
            .filter_map(|ordinal| self.compute_families.get(ordinal))
    }

    pub fn binding_digest(&self) -> &Digest {
        &self.binding_digest
    }

    pub fn binding_is_valid(&self) -> bool {
        self.frame.binding_is_valid()
            && self.use_family.is_use()
            && self.use_family.binding_is_valid_for(&self.frame)
            && self.use_family.exact_use_port() == Some(&self.exact_use_port)
            && self.compute_families.len() == self.frame.inner().code().constructor_ids().len()
            && canonical_candidate_compute_order(&self.compute_families)
                .is_some_and(|order| order == self.candidate_compute_order)
            && self
                .compute_families
                .iter()
                .enumerate()
                .all(|(ordinal, computation)| {
                    u64::try_from(ordinal) == Ok(computation.constructor_ordinal())
                        && computation.binding_is_valid_for(
                            &self.frame,
                            &self.use_family,
                            &self.exact_use_port,
                        )
                })
            && registered_frame_tuple_binding_digest(
                self.active_frame_ordinal,
                &self.frame,
                &self.use_family,
                &self.exact_use_port,
                &self.compute_families,
                &self.candidate_compute_order,
            ) == self.binding_digest
    }

    fn new(
        active_frame_ordinal: u64,
        frame: VerifiedRegisteredClosedFormerFrame,
        use_family: VerifiedRegisteredDemandFamilyV2,
        exact_use_port: PortKey,
        compute_families: Vec<VerifiedRegisteredComputeDemandTuple>,
    ) -> Result<Self, GscUnknownReason> {
        let candidate_compute_order = canonical_candidate_compute_order(&compute_families)
            .ok_or(GscUnknownReason::ResourceExhausted)?;
        let binding_digest = registered_frame_tuple_binding_digest(
            active_frame_ordinal,
            &frame,
            &use_family,
            &exact_use_port,
            &compute_families,
            &candidate_compute_order,
        );
        Ok(Self {
            active_frame_ordinal,
            frame,
            use_family,
            exact_use_port,
            compute_families,
            candidate_compute_order,
            binding_digest,
        })
    }
}

/// Opaque, exhaustive compiler inventory at the active registered cutoff.
///
/// The only producer is [`VerifiedHistory::active_demand_inventory`].  The
/// inventory contains one frame tuple per active registered closed former,
/// ordered by the verified export partition; each tuple contains its Use
/// family and every constructor Compute family.  Constructor order witnesses
/// exhaustive compiler coverage, while the separately bound output-port
/// digest order is the canonical response-candidate order.
#[derive(Clone, Debug)]
pub struct VerifiedRegisteredActiveDemandInventory {
    semantic_manifest_digest: Digest,
    export_index_digest: Digest,
    history_digest: Digest,
    active_anchor_digest: Digest,
    cutoff_boundary_digest: Digest,
    q3_registry_digest: Digest,
    cutoff_coverage_exhaustive: bool,
    tuples: Vec<VerifiedRegisteredFrameDemandTuple>,
    ordered_export_group_digests: Vec<Digest>,
    ordered_frame_binding_digests: Vec<Digest>,
    ordered_family_binding_digests: Vec<Digest>,
    ordered_port_digests: Vec<Digest>,
    ordered_constructor_digests: Vec<Digest>,
    frame_count: u64,
    use_family_count: u64,
    compute_family_count: u64,
    family_count: u64,
    port_count: u64,
    constructor_count: u64,
    binding_digest: Digest,
}

impl VerifiedRegisteredActiveDemandInventory {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn export_index_digest(&self) -> &Digest {
        &self.export_index_digest
    }

    pub fn history_digest(&self) -> &Digest {
        &self.history_digest
    }

    pub fn active_anchor_digest(&self) -> &Digest {
        &self.active_anchor_digest
    }

    pub fn cutoff_boundary_digest(&self) -> &Digest {
        &self.cutoff_boundary_digest
    }

    pub fn q3_registry_digest(&self) -> &Digest {
        &self.q3_registry_digest
    }

    pub fn cutoff_coverage_is_exhaustive(&self) -> bool {
        self.cutoff_coverage_exhaustive
    }

    pub fn tuples(&self) -> &[VerifiedRegisteredFrameDemandTuple] {
        &self.tuples
    }

    pub fn ordered_export_group_digests(&self) -> &[Digest] {
        &self.ordered_export_group_digests
    }

    pub fn ordered_frame_binding_digests(&self) -> &[Digest] {
        &self.ordered_frame_binding_digests
    }

    pub fn ordered_family_binding_digests(&self) -> &[Digest] {
        &self.ordered_family_binding_digests
    }

    pub fn ordered_port_digests(&self) -> &[Digest] {
        &self.ordered_port_digests
    }

    pub fn ordered_constructor_digests(&self) -> &[Digest] {
        &self.ordered_constructor_digests
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn use_family_count(&self) -> u64 {
        self.use_family_count
    }

    pub fn compute_family_count(&self) -> u64 {
        self.compute_family_count
    }

    pub fn family_count(&self) -> u64 {
        self.family_count
    }

    pub fn port_count(&self) -> u64 {
        self.port_count
    }

    pub fn constructor_count(&self) -> u64 {
        self.constructor_count
    }

    pub fn binding_digest(&self) -> &Digest {
        &self.binding_digest
    }

    pub fn binding_is_valid(&self) -> bool {
        if !self.cutoff_coverage_exhaustive {
            return false;
        }
        let Some(derived) = ActiveInventoryDerived::from_tuples(&self.tuples) else {
            return false;
        };
        all_unique(&derived.ordered_export_group_digests)
            && all_unique(&derived.ordered_frame_binding_digests)
            && all_unique(&derived.ordered_family_binding_digests)
            && all_unique(&derived.ordered_port_digests)
            && all_unique(&derived.ordered_tuple_binding_digests)
            && self.tuples.iter().enumerate().all(|(ordinal, tuple)| {
                tuple.binding_is_valid()
                    && u64::try_from(ordinal) == Ok(tuple.active_frame_ordinal())
                    && tuple.frame().semantic_manifest_digest() == &self.semantic_manifest_digest
                    && tuple.frame().export_index_digest() == &self.export_index_digest
                    && tuple.frame().history_digest() == &self.history_digest
                    && tuple.frame().active_anchor_digest() == &self.active_anchor_digest
                    && tuple.frame().cutoff_boundary_digest() == &self.cutoff_boundary_digest
                    && tuple.frame().q3_registry_digest() == &self.q3_registry_digest
            })
            && self.ordered_export_group_digests == derived.ordered_export_group_digests
            && self.ordered_frame_binding_digests == derived.ordered_frame_binding_digests
            && self.ordered_family_binding_digests == derived.ordered_family_binding_digests
            && self.ordered_port_digests == derived.ordered_port_digests
            && self.ordered_constructor_digests == derived.ordered_constructor_digests
            && self.frame_count == derived.frame_count
            && self.use_family_count == derived.use_family_count
            && self.compute_family_count == derived.compute_family_count
            && self.family_count == derived.family_count
            && self.port_count == derived.port_count
            && self.constructor_count == derived.constructor_count
            && registered_active_inventory_binding_digest(
                &self.semantic_manifest_digest,
                &self.export_index_digest,
                &self.history_digest,
                &self.active_anchor_digest,
                &self.cutoff_boundary_digest,
                &self.q3_registry_digest,
                &derived,
            ) == self.binding_digest
    }
}

/// Private, non-deserializable authority for the registered founding history.
#[derive(Debug)]
pub struct VerifiedHistory {
    events: Vec<VerifiedEvent>,
    boundary_snapshots: Vec<VerifiedSignature>,
    declaration_origins: BTreeMap<GlobalId, EventId>,
    anchors: Vec<VerifiedAnchor>,
    active_anchor: VerifiedAnchor,
    export_index: VerifiedRegisteredBootstrapExportIndexV1,
    origin_cutoff_q3_registry: VerifiedOriginCutoffQ3Registry,
    history_digest: Digest,
}

impl VerifiedHistory {
    /// Events are returned oldest first.
    pub fn events(&self) -> &[VerifiedEvent] {
        &self.events
    }

    /// Empty boundary followed by one snapshot after each event.
    pub fn boundary_snapshots(&self) -> &[VerifiedSignature] {
        &self.boundary_snapshots
    }

    pub fn declaration_origin(&self, declaration: &GlobalId) -> Option<&EventId> {
        self.declaration_origins.get(declaration)
    }

    pub fn active_anchor(&self) -> &VerifiedAnchor {
        &self.active_anchor
    }

    /// All adjacent width-two anchors, oldest first.
    pub fn anchors(&self) -> &[VerifiedAnchor] {
        &self.anchors
    }

    pub fn cutoff_boundary(&self, anchor: &VerifiedAnchor) -> Option<&VerifiedSignature> {
        self.anchors
            .iter()
            .find(|known| known.digest() == anchor.digest())
            .map(|known| &self.boundary_snapshots[known.cutoff_snapshot_index])
    }

    /// Report cross-event groups that have begun but are not yet complete.
    ///
    /// Consumers must map the incomplete case to an unsupported/unknown
    /// disposition.  It is never lawful to treat the already-present owner as
    /// a plain declaration merely because its introduction arrives later.
    pub fn cutoff_export_coverage(&self, anchor: &VerifiedAnchor) -> Option<CutoffExportCoverage> {
        let known = self
            .anchors
            .iter()
            .find(|known| known.digest() == anchor.digest())?;
        let cutoff = known.newer_ordinal();
        let mut incomplete = Vec::new();
        for group in self.export_index.groups() {
            if group.activation_ordinal() <= cutoff {
                continue;
            }
            let has_present_source = group.declarations().iter().any(|declaration| {
                let Some(origin) = self.declaration_origins.get(declaration) else {
                    return false;
                };
                self.events
                    .iter()
                    .find(|event| event.event_id() == origin)
                    .is_some_and(|event| event.ordinal() <= cutoff)
            });
            if has_present_source {
                incomplete.push(group.digest().clone());
            }
        }
        Some(if incomplete.is_empty() {
            CutoffExportCoverage::Exhaustive
        } else {
            CutoffExportCoverage::IncompleteRegisteredGroups {
                group_digests: incomplete,
            }
        })
    }

    pub fn export_index(&self) -> &VerifiedRegisteredBootstrapExportIndexV1 {
        &self.export_index
    }

    pub fn origin_cutoff_q3_registry(&self) -> &VerifiedOriginCutoffQ3Registry {
        &self.origin_cutoff_q3_registry
    }

    pub fn digest(&self) -> &Digest {
        &self.history_digest
    }

    /// Deterministically extract all registered closed-former frames.
    ///
    /// Public source IDs are derived from declaration origins.  Each verified
    /// registered event already binds its exact source identity, so that
    /// binding is inherited transitively rather than supplied again by a
    /// caller.  Plain declarations do not emit frames.
    pub fn closed_former_frames(
        &self,
        kernel: &Kernel,
        semantic_manifest: &VerifiedGscSemanticManifest,
    ) -> GscOutcome<Vec<VerifiedRegisteredClosedFormerFrame>> {
        if semantic_manifest.digest() != self.export_index.semantic_manifest_digest() {
            return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
        }
        if self.cutoff_export_coverage(self.active_anchor())
            != Some(CutoffExportCoverage::Exhaustive)
        {
            return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
        }
        let Some(cutoff_boundary) = self.cutoff_boundary(self.active_anchor()) else {
            return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
        };
        let mut frames = Vec::new();
        for group in self.export_index.groups() {
            let UncheckedRegisteredGroupDispositionV1::InductiveAlias {
                owner,
                primitive_code,
                introduction_aliases,
                ..
            } = group.disposition()
            else {
                continue;
            };
            let code = match verify_closed_inductive_code(semantic_manifest, primitive_code) {
                GscOutcome::Proven(code) => code,
                GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
            };
            let introductions = introduction_aliases
                .iter()
                .zip(code.constructor_ids())
                .map(|(alias, constructor)| IntroductionAlias {
                    constructor: constructor.clone(),
                    introduction: alias.declaration.clone(),
                })
                .collect::<Vec<_>>();
            let source_declarations = std::iter::once(owner.clone())
                .chain(
                    introduction_aliases
                        .iter()
                        .map(|alias| alias.declaration.clone()),
                )
                .collect::<Vec<_>>();
            let mut principal_sources = Vec::with_capacity(source_declarations.len());
            let mut birth_support = BTreeSet::new();
            let mut principal_source_bindings = Vec::with_capacity(source_declarations.len());
            for declaration in &source_declarations {
                let Some(origin) = self.declaration_origins.get(declaration) else {
                    return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
                };
                let Some(event) = self.events.iter().find(|event| event.event_id() == origin)
                else {
                    return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
                };
                let origin = GscOriginEventId(origin.0.clone());
                let public_source = PublicSourceId::for_declaration(&origin, declaration);
                principal_sources.push(public_source.clone());
                birth_support.insert(origin.clone());
                principal_source_bindings.push(VerifiedRegisteredPrincipalSource {
                    declaration: declaration.clone(),
                    public_source,
                    origin,
                    registered_source_identity: event.source_identity().clone(),
                });
            }
            let frame = ClosedFormerFrame {
                schema_version: CLOSED_FORMER_FRAME_SCHEMA_VERSION,
                code_id: code.id().clone(),
                owner: owner.clone(),
                introductions,
                source_declarations,
                principal_sources,
                birth_support: birth_support.into_iter().collect(),
            };
            match verify_closed_former_frame(
                semantic_manifest,
                kernel,
                cutoff_boundary,
                &code,
                &frame,
            ) {
                GscOutcome::Proven(frame) => {
                    if frame.principal_sources()
                        != principal_source_bindings
                            .iter()
                            .map(VerifiedRegisteredPrincipalSource::public_source)
                            .cloned()
                            .collect::<Vec<_>>()
                    {
                        return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
                    }
                    let binding_digest = registered_frame_binding_digest(
                        &frame,
                        semantic_manifest.digest(),
                        group.digest(),
                        self.export_index.digest(),
                        self.digest(),
                        self.active_anchor().digest(),
                        cutoff_boundary.digest(),
                        self.origin_cutoff_q3_registry().digest(),
                        &principal_source_bindings,
                    );
                    frames.push(VerifiedRegisteredClosedFormerFrame {
                        inner: frame,
                        semantic_manifest_digest: semantic_manifest.digest().clone(),
                        export_group_digest: group.digest().clone(),
                        export_index_digest: self.export_index.digest().clone(),
                        history_digest: self.digest().clone(),
                        active_anchor_digest: self.active_anchor().digest().clone(),
                        cutoff_boundary_digest: cutoff_boundary.digest().clone(),
                        q3_registry_digest: self.origin_cutoff_q3_registry().digest().clone(),
                        principal_source_bindings,
                        cutoff_boundary: cutoff_boundary.clone(),
                        cutoff_coverage_exhaustive: true,
                        binding_digest,
                    });
                }
                GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
            }
        }
        GscOutcome::Proven(frames)
    }

    /// Exhaustively compile the active registered frame/Use/Compute tuples.
    ///
    /// No caller supplies a frame, provenance set, Use port, or constructor
    /// identity.  The verified export order and constructor order determine
    /// compiler completeness; a separately bound output-port-digest order
    /// determines response-candidate assembly.
    pub fn active_demand_inventory(
        &self,
        kernel: &Kernel,
        semantic_manifest: &VerifiedGscSemanticManifest,
    ) -> GscOutcome<VerifiedRegisteredActiveDemandInventory> {
        if self.cutoff_export_coverage(self.active_anchor())
            != Some(CutoffExportCoverage::Exhaustive)
        {
            return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
        }
        let frames = match self.closed_former_frames(kernel, semantic_manifest) {
            GscOutcome::Proven(frames) => frames,
            GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
        };
        let mut tuples = Vec::new();
        if tuples.try_reserve(frames.len()).is_err() {
            return GscOutcome::Unknown(GscUnknownReason::ResourceExhausted);
        }
        for (active_frame_ordinal, frame) in frames.into_iter().enumerate() {
            let Ok(active_frame_ordinal) = u64::try_from(active_frame_ordinal) else {
                return GscOutcome::Unknown(GscUnknownReason::ResourceExhausted);
            };
            let use_family = match frame.compile_use(kernel, semantic_manifest) {
                GscOutcome::Proven(family) => family,
                GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
            };
            let Some(exact_use_port) = use_family.exact_use_port().cloned() else {
                return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
            };
            let constructor_count = frame.inner().code().constructor_ids().len();
            let mut compute_families = Vec::new();
            if compute_families.try_reserve(constructor_count).is_err() {
                return GscOutcome::Unknown(GscUnknownReason::ResourceExhausted);
            }
            for constructor_ordinal in 0..constructor_count {
                let Some(constructor) = frame
                    .inner()
                    .code()
                    .constructor_id(constructor_ordinal)
                    .cloned()
                else {
                    return GscOutcome::Unknown(GscUnknownReason::MalformedCode);
                };
                let family = match frame.compile_compute(
                    kernel,
                    semantic_manifest,
                    &use_family,
                    constructor_ordinal,
                ) {
                    GscOutcome::Proven(family) => family,
                    GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
                };
                let Ok(constructor_ordinal) = u64::try_from(constructor_ordinal) else {
                    return GscOutcome::Unknown(GscUnknownReason::ResourceExhausted);
                };
                let [output_port] = family.inner().ports() else {
                    return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
                };
                let output_port = output_port.key().clone();
                let computation = VerifiedRegisteredComputeDemandTuple::new(
                    constructor_ordinal,
                    constructor,
                    family,
                    &frame,
                    &use_family,
                    &exact_use_port,
                    output_port,
                );
                if !computation.binding_is_valid_for(&frame, &use_family, &exact_use_port) {
                    return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
                }
                compute_families.push(computation);
            }
            let tuple = match VerifiedRegisteredFrameDemandTuple::new(
                active_frame_ordinal,
                frame,
                use_family,
                exact_use_port,
                compute_families,
            ) {
                Ok(tuple) => tuple,
                Err(reason) => return GscOutcome::Unknown(reason),
            };
            if !tuple.binding_is_valid() {
                return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
            }
            tuples.push(tuple);
        }

        let Some(derived) = ActiveInventoryDerived::from_tuples(&tuples) else {
            return GscOutcome::Unknown(GscUnknownReason::ResourceExhausted);
        };
        let binding_digest = registered_active_inventory_binding_digest(
            semantic_manifest.digest(),
            self.export_index.digest(),
            self.digest(),
            self.active_anchor().digest(),
            self.active_anchor().cutoff_boundary_digest(),
            self.origin_cutoff_q3_registry().digest(),
            &derived,
        );
        let inventory = VerifiedRegisteredActiveDemandInventory {
            semantic_manifest_digest: semantic_manifest.digest().clone(),
            export_index_digest: self.export_index.digest().clone(),
            history_digest: self.digest().clone(),
            active_anchor_digest: self.active_anchor().digest().clone(),
            cutoff_boundary_digest: self.active_anchor().cutoff_boundary_digest().clone(),
            q3_registry_digest: self.origin_cutoff_q3_registry().digest().clone(),
            cutoff_coverage_exhaustive: true,
            tuples,
            ordered_export_group_digests: derived.ordered_export_group_digests,
            ordered_frame_binding_digests: derived.ordered_frame_binding_digests,
            ordered_family_binding_digests: derived.ordered_family_binding_digests,
            ordered_port_digests: derived.ordered_port_digests,
            ordered_constructor_digests: derived.ordered_constructor_digests,
            frame_count: derived.frame_count,
            use_family_count: derived.use_family_count,
            compute_family_count: derived.compute_family_count,
            family_count: derived.family_count,
            port_count: derived.port_count,
            constructor_count: derived.constructor_count,
            binding_digest,
        };
        if !inventory.binding_is_valid() {
            return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
        }
        GscOutcome::Proven(inventory)
    }
}

struct ActiveInventoryDerived {
    ordered_export_group_digests: Vec<Digest>,
    ordered_frame_binding_digests: Vec<Digest>,
    ordered_family_binding_digests: Vec<Digest>,
    ordered_port_digests: Vec<Digest>,
    ordered_constructor_digests: Vec<Digest>,
    ordered_tuple_binding_digests: Vec<Digest>,
    frame_count: u64,
    use_family_count: u64,
    compute_family_count: u64,
    family_count: u64,
    port_count: u64,
    constructor_count: u64,
}

impl ActiveInventoryDerived {
    fn from_tuples(tuples: &[VerifiedRegisteredFrameDemandTuple]) -> Option<Self> {
        let compute_count = tuples.iter().try_fold(0usize, |count, tuple| {
            count.checked_add(tuple.compute_families().len())
        })?;
        let family_count = tuples.len().checked_add(compute_count)?;
        let port_count = tuples.iter().try_fold(0usize, |count, tuple| {
            let count = count.checked_add(tuple.use_family().inner().ports().len())?;
            tuple
                .compute_families()
                .iter()
                .try_fold(count, |count, computation| {
                    count.checked_add(computation.family().inner().ports().len())
                })
        })?;

        let mut ordered_export_group_digests = Vec::new();
        let mut ordered_frame_binding_digests = Vec::new();
        let mut ordered_family_binding_digests = Vec::new();
        let mut ordered_port_digests = Vec::new();
        let mut ordered_constructor_digests = Vec::new();
        let mut ordered_tuple_binding_digests = Vec::new();
        ordered_export_group_digests
            .try_reserve(tuples.len())
            .ok()?;
        ordered_frame_binding_digests
            .try_reserve(tuples.len())
            .ok()?;
        ordered_family_binding_digests
            .try_reserve(family_count)
            .ok()?;
        ordered_port_digests.try_reserve(port_count).ok()?;
        ordered_constructor_digests
            .try_reserve(compute_count)
            .ok()?;
        ordered_tuple_binding_digests
            .try_reserve(tuples.len())
            .ok()?;

        for tuple in tuples {
            ordered_export_group_digests.push(tuple.frame().export_group_digest().clone());
            ordered_frame_binding_digests.push(tuple.frame().binding_digest().clone());
            ordered_tuple_binding_digests.push(tuple.binding_digest().clone());

            ordered_family_binding_digests.push(tuple.use_family().binding_digest().clone());
            ordered_port_digests.extend(
                tuple
                    .use_family()
                    .inner()
                    .ports()
                    .iter()
                    .map(|port| registered_inventory_port_digest(port.key())),
            );
            for computation in tuple.candidate_compute_families_in_port_order() {
                ordered_family_binding_digests.push(computation.family().binding_digest().clone());
                ordered_port_digests.extend(
                    computation
                        .family()
                        .inner()
                        .ports()
                        .iter()
                        .map(|port| registered_inventory_port_digest(port.key())),
                );
            }
            for computation in tuple.compute_families() {
                ordered_constructor_digests.push(registered_inventory_constructor_digest(
                    computation.constructor(),
                ));
            }
        }

        Some(Self {
            ordered_export_group_digests,
            ordered_frame_binding_digests,
            ordered_family_binding_digests,
            ordered_port_digests,
            ordered_constructor_digests,
            ordered_tuple_binding_digests,
            frame_count: u64::try_from(tuples.len()).ok()?,
            use_family_count: u64::try_from(tuples.len()).ok()?,
            compute_family_count: u64::try_from(compute_count).ok()?,
            family_count: u64::try_from(family_count).ok()?,
            port_count: u64::try_from(port_count).ok()?,
            constructor_count: u64::try_from(compute_count).ok()?,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn registered_compute_tuple_binding_digest(
    constructor_ordinal: u64,
    constructor: &ConstructorPortId,
    frame: &VerifiedRegisteredClosedFormerFrame,
    use_family: &VerifiedRegisteredDemandFamilyV2,
    exact_use_port: &PortKey,
    compute_family: &VerifiedRegisteredDemandFamilyV2,
    output_port: &PortKey,
    output_port_digest: &Digest,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    frame.binding_digest().encode_canonical(&mut encoder);
    use_family.binding_digest().encode_canonical(&mut encoder);
    exact_use_port.encode_canonical(&mut encoder);
    encoder.u64(constructor_ordinal);
    constructor.encode_canonical(&mut encoder);
    compute_family
        .binding_digest()
        .encode_canonical(&mut encoder);
    output_port.encode_canonical(&mut encoder);
    output_port_digest.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(REGISTERED_COMPUTE_TUPLE_BINDING_DOMAIN, encoder.as_bytes())
}

fn registered_frame_tuple_binding_digest(
    active_frame_ordinal: u64,
    frame: &VerifiedRegisteredClosedFormerFrame,
    use_family: &VerifiedRegisteredDemandFamilyV2,
    exact_use_port: &PortKey,
    compute_families: &[VerifiedRegisteredComputeDemandTuple],
    candidate_compute_order: &[u64],
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u64(active_frame_ordinal);
    frame.binding_digest().encode_canonical(&mut encoder);
    use_family.binding_digest().encode_canonical(&mut encoder);
    exact_use_port.encode_canonical(&mut encoder);
    encoder.u64(compute_families.len() as u64);
    for computation in compute_families {
        computation.binding_digest().encode_canonical(&mut encoder);
    }
    encoder.u64(candidate_compute_order.len() as u64);
    for ordinal in candidate_compute_order {
        encoder.u64(*ordinal);
    }
    Digest::of_domain_bytes(REGISTERED_FRAME_TUPLE_BINDING_DOMAIN, encoder.as_bytes())
}

fn canonical_candidate_compute_order(
    compute_families: &[VerifiedRegisteredComputeDemandTuple],
) -> Option<Vec<u64>> {
    let mut order = Vec::new();
    order.try_reserve(compute_families.len()).ok()?;
    for ordinal in 0..compute_families.len() {
        order.push(u64::try_from(ordinal).ok()?);
    }
    order.sort_by(|left, right| {
        let left = usize::try_from(*left)
            .ok()
            .and_then(|ordinal| compute_families.get(ordinal));
        let right = usize::try_from(*right)
            .ok()
            .and_then(|ordinal| compute_families.get(ordinal));
        match (left, right) {
            (Some(left), Some(right)) => left
                .output_port_digest()
                .cmp(right.output_port_digest())
                .then_with(|| left.constructor_ordinal().cmp(&right.constructor_ordinal())),
            _ => left.is_none().cmp(&right.is_none()),
        }
    });
    let has_duplicate_port_digest = order.windows(2).any(|pair| {
        let [left, right] = pair else {
            return true;
        };
        let (Ok(left), Ok(right)) = (usize::try_from(*left), usize::try_from(*right)) else {
            return true;
        };
        compute_families
            .get(left)
            .zip(compute_families.get(right))
            .is_none_or(|(left, right)| left.output_port_digest() == right.output_port_digest())
    });
    (!has_duplicate_port_digest).then_some(order)
}

fn registered_inventory_port_digest(port: &PortKey) -> Digest {
    Digest::of_canonical(REGISTERED_INVENTORY_PORT_DOMAIN, port)
}

fn registered_inventory_constructor_digest(constructor: &ConstructorPortId) -> Digest {
    Digest::of_canonical(REGISTERED_INVENTORY_CONSTRUCTOR_DOMAIN, constructor)
}

#[allow(clippy::too_many_arguments)]
fn registered_active_inventory_binding_digest(
    semantic_manifest_digest: &Digest,
    export_index_digest: &Digest,
    history_digest: &Digest,
    active_anchor_digest: &Digest,
    cutoff_boundary_digest: &Digest,
    q3_registry_digest: &Digest,
    derived: &ActiveInventoryDerived,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    semantic_manifest_digest.encode_canonical(&mut encoder);
    export_index_digest.encode_canonical(&mut encoder);
    history_digest.encode_canonical(&mut encoder);
    active_anchor_digest.encode_canonical(&mut encoder);
    cutoff_boundary_digest.encode_canonical(&mut encoder);
    q3_registry_digest.encode_canonical(&mut encoder);
    encoder.tag(1);
    encoder.u64(derived.frame_count);
    encoder.u64(derived.use_family_count);
    encoder.u64(derived.compute_family_count);
    encoder.u64(derived.family_count);
    encoder.u64(derived.port_count);
    encoder.u64(derived.constructor_count);
    encoder.sequence(&derived.ordered_export_group_digests);
    encoder.sequence(&derived.ordered_frame_binding_digests);
    encoder.sequence(&derived.ordered_family_binding_digests);
    encoder.sequence(&derived.ordered_port_digests);
    encoder.sequence(&derived.ordered_constructor_digests);
    encoder.sequence(&derived.ordered_tuple_binding_digests);
    Digest::of_domain_bytes(
        REGISTERED_ACTIVE_INVENTORY_BINDING_DOMAIN,
        encoder.as_bytes(),
    )
}

#[allow(clippy::too_many_arguments)]
fn registered_frame_binding_digest(
    frame: &VerifiedClosedFormerFrame,
    semantic_manifest_digest: &Digest,
    export_group_digest: &Digest,
    export_index_digest: &Digest,
    history_digest: &Digest,
    active_anchor_digest: &Digest,
    cutoff_boundary_digest: &Digest,
    q3_registry_digest: &Digest,
    principal_source_bindings: &[VerifiedRegisteredPrincipalSource],
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    semantic_manifest_digest.encode_canonical(&mut encoder);
    frame.id().encode_canonical(&mut encoder);
    export_group_digest.encode_canonical(&mut encoder);
    export_index_digest.encode_canonical(&mut encoder);
    history_digest.encode_canonical(&mut encoder);
    active_anchor_digest.encode_canonical(&mut encoder);
    cutoff_boundary_digest.encode_canonical(&mut encoder);
    q3_registry_digest.encode_canonical(&mut encoder);
    encoder.tag(1);
    encoder.sequence(principal_source_bindings);
    Digest::of_domain_bytes(REGISTERED_FRAME_BINDING_DOMAIN, encoder.as_bytes())
}

fn family_support_matches_frame(
    family: &VerifiedCanonicalDemandFamilyV2,
    frame: &VerifiedRegisteredClosedFormerFrame,
) -> bool {
    family.semantic_manifest_digest() == frame.semantic_manifest_digest()
        && family.principal_sources() == frame.inner().principal_sources()
        && family.birth_support() == frame.inner().birth_support()
        && family_sources_match_registered_bindings(
            family.principal_sources(),
            family.birth_support(),
            frame.principal_source_bindings(),
        )
}

fn frame_sources_match_registered_bindings(
    frame: &VerifiedClosedFormerFrame,
    bindings: &[VerifiedRegisteredPrincipalSource],
) -> bool {
    frame.frame().source_declarations.as_slice()
        == bindings
            .iter()
            .map(VerifiedRegisteredPrincipalSource::declaration)
            .cloned()
            .collect::<Vec<_>>()
        && bindings.iter().all(|binding| {
            &PublicSourceId::for_declaration(binding.origin(), binding.declaration())
                == binding.public_source()
        })
        && family_sources_match_registered_bindings(
            frame.principal_sources(),
            frame.birth_support(),
            bindings,
        )
}

fn family_sources_match_registered_bindings(
    principal_sources: &[PublicSourceId],
    birth_support: &[GscOriginEventId],
    bindings: &[VerifiedRegisteredPrincipalSource],
) -> bool {
    principal_sources
        == bindings
            .iter()
            .map(VerifiedRegisteredPrincipalSource::public_source)
            .cloned()
            .collect::<Vec<_>>()
        && birth_support
            == bindings
                .iter()
                .map(VerifiedRegisteredPrincipalSource::origin)
                .cloned()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
}

fn registered_use_shape_is_exact(
    family: &VerifiedRegisteredDemandFamilyV2,
    frame: &VerifiedRegisteredClosedFormerFrame,
) -> bool {
    family.inner().ports().len() == 1
        && family.inner().premise_refs().is_empty()
        && family
            .inner()
            .ports()
            .first()
            .is_some_and(|port| Some(port.key()) == family.exact_use_port())
        && matches!(
            family.inner().output_clauses(),
            [OutputClause::TermPort {
                role: OutputRole::UsePort { former },
                ..
            }] if former == frame.inner().id()
        )
}

fn registered_compute_shape_is_exact(
    family: &VerifiedRegisteredDemandFamilyV2,
    frame: &VerifiedRegisteredClosedFormerFrame,
) -> bool {
    let (Some(exact_use_port), Some(constructor)) = (family.exact_use_port(), family.constructor())
    else {
        return false;
    };
    matches!(
        family.inner().premise_refs(),
        [premise]
            if matches!(
                &premise.source,
                PortRef::Generated { port } if port == exact_use_port
            )
    ) && matches!(
        family.inner().output_clauses(),
        [OutputClause::EquationPort {
            equation_code:
                EquationClauseCode::GeneratedUseBeta {
                    former,
                    constructor: output_constructor,
                    use_port,
                },
            role:
                OutputRole::ComputationPort {
                    former: role_former,
                    constructor: role_constructor,
                    ..
                },
            ..
        }] if former == frame.inner().id()
            && role_former == frame.inner().id()
            && output_constructor == constructor
            && role_constructor == constructor
            && use_port == exact_use_port
    )
}

fn registered_family_binding_digest(
    family: &VerifiedCanonicalDemandFamilyV2,
    frame: &VerifiedRegisteredClosedFormerFrame,
    exact_use_port: Option<&PortKey>,
    source_use_family_binding_digest: Option<&Digest>,
    constructor: Option<&pen_demand::gsc::ConstructorPortId>,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    frame
        .semantic_manifest_digest()
        .encode_canonical(&mut encoder);
    family.id().encode_canonical(&mut encoder);
    family.rule().encode_canonical(&mut encoder);
    encoder.tag(family.rank());
    frame.binding_digest().encode_canonical(&mut encoder);
    frame.export_group_digest().encode_canonical(&mut encoder);
    frame.history_digest().encode_canonical(&mut encoder);
    frame.active_anchor_digest().encode_canonical(&mut encoder);
    frame
        .cutoff_boundary_digest()
        .encode_canonical(&mut encoder);
    frame.q3_registry_digest().encode_canonical(&mut encoder);
    encode_optional_canonical(&mut encoder, exact_use_port);
    encode_optional_canonical(&mut encoder, source_use_family_binding_digest);
    encode_optional_canonical(&mut encoder, constructor);
    Digest::of_domain_bytes(REGISTERED_FAMILY_BINDING_DOMAIN, encoder.as_bytes())
}

fn encode_optional_canonical<T: CanonicalEncode>(
    encoder: &mut CanonicalEncoder,
    value: Option<&T>,
) {
    match value {
        Some(value) => {
            encoder.tag(1);
            value.encode_canonical(encoder);
        }
        None => encoder.tag(0),
    }
}

fn all_unique<T: Ord + Clone>(values: &[T]) -> bool {
    values.iter().cloned().collect::<BTreeSet<_>>().len() == values.len()
}

/// Promote the exact registered prefix and final-boundary export partition to
/// a verified history.
pub fn verify_registered_bootstrap_history(
    kernel: &Kernel,
    bootstrap: &VerifiedRegisteredBootstrap,
    export_index: VerifiedRegisteredBootstrapExportIndexV1,
) -> Result<VerifiedHistory, RegisteredHistoryError> {
    if export_index.bootstrap_artifact_digest() != bootstrap.artifact_digest()
        || export_index.bootstrap_contract_digest() != bootstrap.bootstrap_contract_digest()
        || export_index.final_boundary_digest() != bootstrap.final_boundary().digest()
    {
        return Err(RegisteredHistoryError::BootstrapBinding);
    }
    if !export_index.registered_q3_theorems_is_empty() {
        return Err(RegisteredHistoryError::NonEmptyQ3Registry);
    }

    let declarations = bootstrap.final_boundary().declarations();
    if declarations.len() != REGISTERED_BOOTSTRAP_ACT_COUNT {
        return Err(RegisteredHistoryError::BoundaryCount);
    }

    let mut boundary_snapshots = Vec::with_capacity(REGISTERED_BOOTSTRAP_ACT_COUNT + 1);
    for prefix_len in 0..=REGISTERED_BOOTSTRAP_ACT_COUNT {
        let ordinal = u16::try_from(prefix_len).expect("registered act count fits u16");
        let snapshot = kernel.verify_signature(&UncheckedSignature {
            declarations: declarations[..prefix_len].to_vec(),
        })?;
        let expected = bootstrap
            .boundary_chain()
            .get(prefix_len)
            .ok_or(RegisteredHistoryError::BoundarySnapshot { ordinal })?;
        if snapshot.digest() != expected {
            return Err(RegisteredHistoryError::BoundaryDigest { ordinal });
        }
        boundary_snapshots.push(snapshot);
    }

    let mut initial_encoder = CanonicalEncoder::new();
    bootstrap
        .bootstrap_contract_digest()
        .encode_canonical(&mut initial_encoder);
    boundary_snapshots[0]
        .digest()
        .encode_canonical(&mut initial_encoder);
    let mut running_history =
        Digest::of_domain_bytes(EMPTY_HISTORY_DOMAIN, initial_encoder.as_bytes());

    let mut events = Vec::with_capacity(REGISTERED_BOOTSTRAP_ACT_COUNT);
    let mut declaration_origins = BTreeMap::new();
    for index in 0..REGISTERED_BOOTSTRAP_ACT_COUNT {
        let ordinal = u16::try_from(index + 1).expect("registered act count fits u16");
        let activated_group_digests = export_index
            .groups()
            .iter()
            .filter(|group| group.activation_ordinal() == ordinal)
            .map(|group| group.digest().clone())
            .collect::<Vec<_>>();
        if export_index
            .groups()
            .iter()
            .any(|group| group.activation_ordinal() == 0)
        {
            return Err(RegisteredHistoryError::InvalidGroupActivation { ordinal: 0 });
        }
        let finalized_bootstrap_index_digest =
            (index + 1 == REGISTERED_BOOTSTRAP_ACT_COUNT).then(|| export_index.digest().clone());
        let event_export_digest = event_export_digest(
            ordinal,
            export_index.semantic_manifest_digest(),
            &activated_group_digests,
            finalized_bootstrap_index_digest.as_ref(),
        );
        let event_export_index = VerifiedEventExportIndex {
            activated_group_digests,
            finalized_bootstrap_index_digest,
            digest: event_export_digest,
        };

        let predecessor = events
            .last()
            .map(|event: &VerifiedEvent| event.event_id.clone());
        let pre_boundary_digest = boundary_snapshots[index].digest().clone();
        let post_boundary_digest = boundary_snapshots[index + 1].digest().clone();
        let extension_digest = Digest::of_canonical(
            NORMALIZED_EXTENSION_DOMAIN,
            &UncheckedSignature {
                declarations: vec![declarations[index].clone()],
            },
        );
        let source_identity = bootstrap.source_identities()[index].clone();
        let binding_identity = bootstrap.binding_identities()[index].clone();
        let event_id = registered_event_id(
            &running_history,
            ordinal,
            predecessor.as_ref(),
            &pre_boundary_digest,
            &post_boundary_digest,
            &extension_digest,
            &source_identity,
            &binding_identity,
            event_export_index.digest(),
        );
        let post_history_digest = history_step_digest(
            &running_history,
            &event_id,
            &post_boundary_digest,
            &extension_digest,
            event_export_index.digest(),
        );
        let event = VerifiedEvent {
            event_id: event_id.clone(),
            ordinal,
            predecessor,
            pre_history_digest: running_history,
            post_history_digest: post_history_digest.clone(),
            pre_boundary_digest,
            post_boundary_digest,
            extension_digest,
            source_identity,
            binding_identity,
            export_index: event_export_index,
        };
        if declaration_origins
            .insert(declarations[index].id.clone(), event_id)
            .is_some()
        {
            return Err(RegisteredHistoryError::DuplicateDeclarationOrigin);
        }
        events.push(event);
        running_history = post_history_digest;
    }

    let final_ids = declarations
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect::<BTreeSet<_>>();
    let origin_ids = declaration_origins.keys().cloned().collect::<BTreeSet<_>>();
    if final_ids != origin_ids {
        return Err(RegisteredHistoryError::MissingDeclarationOrigin);
    }

    let anchors = verified_anchors(&events, &boundary_snapshots)?;
    let active_anchor = anchors
        .last()
        .cloned()
        .ok_or(RegisteredHistoryError::ActiveAnchor)?;
    let origin_cutoff_q3_registry =
        empty_origin_cutoff_q3_registry(&active_anchor, export_index.digest());

    Ok(VerifiedHistory {
        events,
        boundary_snapshots,
        declaration_origins,
        anchors,
        active_anchor,
        export_index,
        origin_cutoff_q3_registry,
        history_digest: running_history,
    })
}

fn event_export_digest(
    ordinal: u16,
    semantic_manifest_digest: &Digest,
    activated_group_digests: &[Digest],
    finalized_bootstrap_index_digest: Option<&Digest>,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(ordinal);
    semantic_manifest_digest.encode_canonical(&mut encoder);
    encoder.sequence(activated_group_digests);
    match finalized_bootstrap_index_digest {
        Some(digest) => {
            encoder.tag(1);
            digest.encode_canonical(&mut encoder);
        }
        None => encoder.tag(0),
    }
    Digest::of_domain_bytes(EVENT_EXPORT_DOMAIN, encoder.as_bytes())
}

#[allow(clippy::too_many_arguments)]
fn registered_event_id(
    pre_history_digest: &Digest,
    ordinal: u16,
    predecessor: Option<&EventId>,
    pre_boundary_digest: &Digest,
    post_boundary_digest: &Digest,
    extension_digest: &Digest,
    source_identity: &Digest,
    binding_identity: &Digest,
    event_export_digest: &Digest,
) -> EventId {
    let mut encoder = CanonicalEncoder::new();
    pre_history_digest.encode_canonical(&mut encoder);
    encoder.u16(ordinal);
    match predecessor {
        Some(event) => {
            encoder.tag(1);
            event.encode_canonical(&mut encoder);
        }
        None => encoder.tag(0),
    }
    pre_boundary_digest.encode_canonical(&mut encoder);
    post_boundary_digest.encode_canonical(&mut encoder);
    extension_digest.encode_canonical(&mut encoder);
    source_identity.encode_canonical(&mut encoder);
    binding_identity.encode_canonical(&mut encoder);
    event_export_digest.encode_canonical(&mut encoder);
    EventId(Digest::of_domain_bytes(EVENT_ID_DOMAIN, encoder.as_bytes()))
}

fn history_step_digest(
    pre_history_digest: &Digest,
    event_id: &EventId,
    post_boundary_digest: &Digest,
    extension_digest: &Digest,
    event_export_digest: &Digest,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    pre_history_digest.encode_canonical(&mut encoder);
    event_id.encode_canonical(&mut encoder);
    post_boundary_digest.encode_canonical(&mut encoder);
    extension_digest.encode_canonical(&mut encoder);
    event_export_digest.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(HISTORY_STEP_DOMAIN, encoder.as_bytes())
}

fn verified_anchors(
    events: &[VerifiedEvent],
    boundary_snapshots: &[VerifiedSignature],
) -> Result<Vec<VerifiedAnchor>, RegisteredHistoryError> {
    if DEMAND_WINDOW_WIDTH != 2 || events.len() < usize::from(DEMAND_WINDOW_WIDTH) {
        return Err(RegisteredHistoryError::ActiveAnchor);
    }
    events
        .windows(2)
        .map(|pair| {
            let [older, newer] = pair else {
                return Err(RegisteredHistoryError::ActiveAnchor);
            };
            if newer.predecessor() != Some(older.event_id())
                || newer.ordinal() != older.ordinal().saturating_add(1)
            {
                return Err(RegisteredHistoryError::ActiveAnchor);
            }
            let cutoff_snapshot_index = usize::from(newer.ordinal());
            let cutoff_boundary = boundary_snapshots
                .get(cutoff_snapshot_index)
                .ok_or(RegisteredHistoryError::ActiveAnchor)?;
            let mut encoder = CanonicalEncoder::new();
            newer.event_id().encode_canonical(&mut encoder);
            older.event_id().encode_canonical(&mut encoder);
            cutoff_boundary.digest().encode_canonical(&mut encoder);
            let anchor_digest = Digest::of_domain_bytes(ANCHOR_DOMAIN, encoder.as_bytes());
            Ok(VerifiedAnchor {
                newer: newer.event_id().clone(),
                older: older.event_id().clone(),
                newer_ordinal: newer.ordinal(),
                older_ordinal: older.ordinal(),
                cutoff_snapshot_index,
                cutoff_boundary_digest: cutoff_boundary.digest().clone(),
                anchor_digest,
            })
        })
        .collect()
}

fn empty_origin_cutoff_q3_registry(
    anchor: &VerifiedAnchor,
    export_index_digest: &Digest,
) -> VerifiedOriginCutoffQ3Registry {
    let mut encoder = CanonicalEncoder::new();
    anchor.newer().encode_canonical(&mut encoder);
    anchor
        .cutoff_boundary_digest()
        .encode_canonical(&mut encoder);
    export_index_digest.encode_canonical(&mut encoder);
    encoder.u64(0);
    VerifiedOriginCutoffQ3Registry {
        cutoff_event: anchor.newer().clone(),
        cutoff_boundary_digest: anchor.cutoff_boundary_digest().clone(),
        export_index_digest: export_index_digest.clone(),
        digest: Digest::of_domain_bytes(Q3_CUTOFF_DOMAIN, encoder.as_bytes()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        load_embedded_registered_bootstrap, load_embedded_registered_bootstrap_export_index_v1,
    };
    use pen_demand::gsc::{
        GscOutcome, VerifiedGscSemanticManifest, frozen_gsc_semantic_manifest_v1,
        verify_gsc_semantic_manifest_v1,
    };
    use pen_kernel::KernelLimits;

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    fn manifest() -> VerifiedGscSemanticManifest {
        match verify_gsc_semantic_manifest_v1(&frozen_gsc_semantic_manifest_v1()) {
            GscOutcome::Proven(manifest) => manifest,
            GscOutcome::Unknown(reason) => panic!("frozen manifest is unknown: {reason:?}"),
        }
    }

    fn history() -> VerifiedHistory {
        let kernel = kernel();
        let bootstrap =
            load_embedded_registered_bootstrap(&kernel).expect("registered bootstrap verifies");
        let export =
            load_embedded_registered_bootstrap_export_index_v1(&kernel, &bootstrap, &manifest())
                .expect("registered export index verifies");
        verify_registered_bootstrap_history(&kernel, &bootstrap, export)
            .expect("registered history verifies")
    }

    #[test]
    fn registered_history_reconstructs_events_origins_and_tip() {
        let history = history();
        assert_eq!(history.events().len(), REGISTERED_BOOTSTRAP_ACT_COUNT);
        assert_eq!(
            history.boundary_snapshots().len(),
            REGISTERED_BOOTSTRAP_ACT_COUNT + 1
        );
        assert_eq!(history.boundary_snapshots()[0].declarations().len(), 0);
        assert_eq!(history.boundary_snapshots()[3].declarations().len(), 3);

        for (index, declaration) in history.boundary_snapshots()[3]
            .declarations()
            .iter()
            .enumerate()
        {
            assert_eq!(
                history.declaration_origin(&declaration.id),
                Some(history.events()[index].event_id())
            );
        }
        assert_eq!(history.active_anchor().newer_ordinal(), 3);
        assert_eq!(history.active_anchor().older_ordinal(), 2);
        assert_eq!(
            history
                .cutoff_boundary(history.active_anchor())
                .expect("active cutoff")
                .digest(),
            history.active_anchor().cutoff_boundary_digest()
        );
        assert_eq!(
            history.digest(),
            history
                .events()
                .last()
                .expect("last event")
                .post_history_digest()
        );
        for (index, event) in history.events().iter().enumerate() {
            let expected = Digest::of_canonical(
                NORMALIZED_EXTENSION_DOMAIN,
                &UncheckedSignature {
                    declarations: vec![
                        history.boundary_snapshots()[3].declarations()[index].clone(),
                    ],
                },
            );
            assert_eq!(event.extension_digest(), &expected);
        }
    }

    #[test]
    fn cross_event_group_activates_only_when_complete() {
        let history = history();
        assert_eq!(
            history.events()[0]
                .export_index()
                .activated_group_digests()
                .len(),
            1
        );
        assert!(
            history.events()[1]
                .export_index()
                .activated_group_digests()
                .is_empty()
        );
        assert_eq!(
            history.events()[2]
                .export_index()
                .activated_group_digests()
                .len(),
            1
        );
        assert!(
            history.events()[0]
                .export_index()
                .finalized_bootstrap_index_digest()
                .is_none()
        );
        assert_eq!(
            history.events()[2]
                .export_index()
                .finalized_bootstrap_index_digest(),
            Some(history.export_index().digest())
        );
    }

    #[test]
    fn adjacent_anchors_retain_exact_prefix_cutoffs() {
        let history = history();
        assert_eq!(history.anchors().len(), 2);
        assert_eq!(history.anchors()[0].newer_ordinal(), 2);
        assert_eq!(history.anchors()[0].older_ordinal(), 1);
        assert_eq!(
            history
                .cutoff_boundary(&history.anchors()[0])
                .expect("older cutoff")
                .declarations()
                .len(),
            2
        );
        assert_eq!(
            history
                .cutoff_boundary(&history.anchors()[1])
                .expect("tip cutoff")
                .declarations()
                .len(),
            3
        );
        assert!(matches!(
            history.cutoff_export_coverage(&history.anchors()[0]),
            Some(CutoffExportCoverage::IncompleteRegisteredGroups {
                group_digests
            }) if group_digests.len() == 1
        ));
        assert_eq!(
            history.cutoff_export_coverage(&history.anchors()[1]),
            Some(CutoffExportCoverage::Exhaustive)
        );
    }

    #[test]
    fn h3_q3_registry_is_verified_empty_only_at_registered_cutoff() {
        let history = history();
        let registry = history.origin_cutoff_q3_registry();
        assert!(registry.is_empty());
        assert_eq!(registry.cutoff_event(), history.active_anchor().newer());
        assert_eq!(
            registry.cutoff_boundary_digest(),
            history.active_anchor().cutoff_boundary_digest()
        );
        assert_eq!(
            registry.export_index_digest(),
            history.export_index().digest()
        );
    }

    #[test]
    fn registered_index_extracts_the_closed_former_without_restipulation() {
        let history = history();
        let manifest = manifest();
        let frames = match history.closed_former_frames(&kernel(), &manifest) {
            GscOutcome::Proven(frames) => frames,
            GscOutcome::Unknown(reason) => panic!("registered frame is unknown: {reason:?}"),
        };
        assert_eq!(frames.len(), 1);
        let frame = &frames[0];
        assert!(frame.binding_is_valid());
        assert!(frame.cutoff_coverage_is_exhaustive());
        assert_eq!(frame.inner().introductions().len(), 1);
        assert_eq!(frame.inner().principal_sources().len(), 2);
        assert_eq!(frame.inner().birth_support().len(), 2);
        assert_eq!(frame.principal_source_bindings().len(), 2);
        assert_eq!(frame.history_digest(), history.digest());
        assert_eq!(
            frame.active_anchor_digest(),
            history.active_anchor().digest()
        );
        assert_eq!(
            frame.q3_registry_digest(),
            history.origin_cutoff_q3_registry().digest()
        );
        assert_eq!(
            frame.inner().owner(),
            &history.boundary_snapshots()[3].declarations()[1].id
        );
        assert_eq!(
            frame.inner().introductions()[0].introduction,
            history.boundary_snapshots()[3].declarations()[2].id
        );
    }

    #[test]
    fn registered_wrappers_compile_use_and_exact_port_compute() {
        let history = history();
        let manifest = manifest();
        let kernel = kernel();
        let frames = match history.closed_former_frames(&kernel, &manifest) {
            GscOutcome::Proven(frames) => frames,
            GscOutcome::Unknown(reason) => panic!("registered frame is unknown: {reason:?}"),
        };
        let frame = &frames[0];
        let use_family = match frame.compile_use(&kernel, &manifest) {
            GscOutcome::Proven(family) => family,
            GscOutcome::Unknown(reason) => panic!("registered use is unknown: {reason:?}"),
        };
        assert!(use_family.is_use());
        assert!(use_family.binding_is_valid_for(frame));
        assert_eq!(
            use_family.exact_use_port(),
            Some(use_family.inner().ports()[0].key())
        );

        let compute_family = match frame.compile_compute(&kernel, &manifest, &use_family, 0) {
            GscOutcome::Proven(family) => family,
            GscOutcome::Unknown(reason) => {
                panic!("registered computation is unknown: {reason:?}")
            }
        };
        assert!(compute_family.is_compute());
        assert!(compute_family.binding_is_valid_for(frame));
        assert_eq!(compute_family.exact_use_port(), use_family.exact_use_port());
        assert_eq!(
            compute_family.source_use_family_binding_digest(),
            Some(use_family.binding_digest())
        );
        assert_eq!(
            compute_family.constructor(),
            frame.inner().code().constructor_id(0)
        );
    }

    #[test]
    fn active_inventory_exhausts_registered_frames_and_compiler_tuples() {
        let history = history();
        let manifest = manifest();
        let inventory = match history.active_demand_inventory(&kernel(), &manifest) {
            GscOutcome::Proven(inventory) => inventory,
            GscOutcome::Unknown(reason) => {
                panic!("registered active demand inventory is unknown: {reason:?}")
            }
        };
        assert!(inventory.binding_is_valid());
        assert!(inventory.cutoff_coverage_is_exhaustive());
        assert_eq!(inventory.semantic_manifest_digest(), manifest.digest());
        assert_eq!(
            inventory.export_index_digest(),
            history.export_index().digest()
        );
        assert_eq!(inventory.history_digest(), history.digest());
        assert_eq!(
            inventory.active_anchor_digest(),
            history.active_anchor().digest()
        );
        assert_eq!(
            inventory.cutoff_boundary_digest(),
            history.active_anchor().cutoff_boundary_digest()
        );
        assert_eq!(
            inventory.q3_registry_digest(),
            history.origin_cutoff_q3_registry().digest()
        );

        assert_eq!(inventory.frame_count(), 1);
        assert_eq!(inventory.use_family_count(), 1);
        assert_eq!(inventory.compute_family_count(), 1);
        assert_eq!(inventory.family_count(), 2);
        assert_eq!(inventory.port_count(), 2);
        assert_eq!(inventory.constructor_count(), 1);
        assert_eq!(inventory.tuples().len(), 1);
        assert_eq!(inventory.ordered_export_group_digests().len(), 1);
        assert_eq!(inventory.ordered_frame_binding_digests().len(), 1);
        assert_eq!(inventory.ordered_family_binding_digests().len(), 2);
        assert_eq!(inventory.ordered_port_digests().len(), 2);
        assert_eq!(inventory.ordered_constructor_digests().len(), 1);

        let tuple = &inventory.tuples()[0];
        assert!(tuple.binding_is_valid());
        assert_eq!(tuple.active_frame_ordinal(), 0);
        assert_eq!(tuple.compute_families().len(), 1);
        assert_eq!(tuple.candidate_compute_order(), &[0]);
        assert_eq!(
            tuple
                .candidate_compute_families_in_port_order()
                .map(VerifiedRegisteredComputeDemandTuple::binding_digest)
                .collect::<Vec<_>>(),
            vec![tuple.compute_families()[0].binding_digest()]
        );
        let computation = &tuple.compute_families()[0];
        assert_eq!(computation.constructor_ordinal(), 0);
        assert_eq!(
            computation.constructor(),
            tuple.frame().inner().code().constructor_id(0).unwrap()
        );
        assert!(computation.binding_is_valid_for(
            tuple.frame(),
            tuple.use_family(),
            tuple.exact_use_port(),
        ));
    }

    #[test]
    fn active_inventory_tamper_order_count_and_omission_fail_closed() {
        let history = history();
        let manifest = manifest();
        let inventory = match history.active_demand_inventory(&kernel(), &manifest) {
            GscOutcome::Proven(inventory) => inventory,
            GscOutcome::Unknown(reason) => {
                panic!("registered active demand inventory is unknown: {reason:?}")
            }
        };

        let mut binding_tamper = inventory.clone();
        binding_tamper.history_digest = Digest::of_bytes(b"forged inventory history");
        assert!(!binding_tamper.binding_is_valid());

        let mut order_tamper = inventory.clone();
        order_tamper.ordered_family_binding_digests.swap(0, 1);
        assert!(!order_tamper.binding_is_valid());

        let mut tuple_order_tamper = inventory.clone();
        tuple_order_tamper.tuples[0].active_frame_ordinal = 1;
        tuple_order_tamper.tuples[0].binding_digest = registered_frame_tuple_binding_digest(
            tuple_order_tamper.tuples[0].active_frame_ordinal,
            &tuple_order_tamper.tuples[0].frame,
            &tuple_order_tamper.tuples[0].use_family,
            &tuple_order_tamper.tuples[0].exact_use_port,
            &tuple_order_tamper.tuples[0].compute_families,
            &tuple_order_tamper.tuples[0].candidate_compute_order,
        );
        assert!(tuple_order_tamper.tuples[0].binding_is_valid());
        assert!(!tuple_order_tamper.binding_is_valid());

        let mut candidate_order_tamper = inventory.clone();
        candidate_order_tamper.tuples[0].candidate_compute_order[0] = 1;
        candidate_order_tamper.tuples[0].binding_digest = registered_frame_tuple_binding_digest(
            candidate_order_tamper.tuples[0].active_frame_ordinal,
            &candidate_order_tamper.tuples[0].frame,
            &candidate_order_tamper.tuples[0].use_family,
            &candidate_order_tamper.tuples[0].exact_use_port,
            &candidate_order_tamper.tuples[0].compute_families,
            &candidate_order_tamper.tuples[0].candidate_compute_order,
        );
        assert!(!candidate_order_tamper.tuples[0].binding_is_valid());
        assert!(!candidate_order_tamper.binding_is_valid());

        let mut count_tamper = inventory.clone();
        count_tamper.family_count += 1;
        assert!(!count_tamper.binding_is_valid());

        let mut omission_tamper = inventory;
        omission_tamper.tuples[0].compute_families.clear();
        omission_tamper.tuples[0].candidate_compute_order.clear();
        omission_tamper.tuples[0].binding_digest = registered_frame_tuple_binding_digest(
            omission_tamper.tuples[0].active_frame_ordinal,
            &omission_tamper.tuples[0].frame,
            &omission_tamper.tuples[0].use_family,
            &omission_tamper.tuples[0].exact_use_port,
            &omission_tamper.tuples[0].compute_families,
            &omission_tamper.tuples[0].candidate_compute_order,
        );
        assert!(!omission_tamper.tuples[0].binding_is_valid());
        assert!(!omission_tamper.binding_is_valid());
    }

    #[test]
    fn candidate_compute_order_is_output_port_digest_order() {
        let history = history();
        let inventory = match history.active_demand_inventory(&kernel(), &manifest()) {
            GscOutcome::Proven(inventory) => inventory,
            GscOutcome::Unknown(reason) => {
                panic!("registered active demand inventory is unknown: {reason:?}")
            }
        };
        let base = inventory.tuples()[0].compute_families()[0].clone();
        let first_digest = Digest::of_bytes(b"candidate port one");
        let second_digest = Digest::of_bytes(b"candidate port two");
        let (lower, higher) = if first_digest < second_digest {
            (first_digest, second_digest)
        } else {
            (second_digest, first_digest)
        };
        let mut first = base.clone();
        first.constructor_ordinal = 0;
        first.output_port_digest = higher;
        let mut second = base;
        second.constructor_ordinal = 1;
        second.output_port_digest = lower.clone();
        assert_eq!(
            canonical_candidate_compute_order(&[first, second]),
            Some(vec![1, 0])
        );

        let mut duplicate = inventory.tuples()[0].compute_families()[0].clone();
        duplicate.output_port_digest = lower;
        let mut duplicate_again = duplicate.clone();
        duplicate_again.constructor_ordinal = 1;
        assert_eq!(
            canonical_candidate_compute_order(&[duplicate, duplicate_again]),
            None
        );
    }

    #[test]
    fn active_inventory_rejects_a_non_authoritative_active_cutoff() {
        let mut history = history();
        history.active_anchor.anchor_digest = Digest::of_bytes(b"forged active anchor");
        assert!(matches!(
            history.active_demand_inventory(&kernel(), &manifest()),
            GscOutcome::Unknown(GscUnknownReason::MalformedFrame)
        ));
    }

    #[test]
    fn forged_and_mismatched_registered_bindings_fail_closed() {
        let history = history();
        let manifest = manifest();
        let kernel = kernel();
        let frames = match history.closed_former_frames(&kernel, &manifest) {
            GscOutcome::Proven(frames) => frames,
            GscOutcome::Unknown(reason) => panic!("registered frame is unknown: {reason:?}"),
        };
        let frame = &frames[0];
        let use_family = match frame.compile_use(&kernel, &manifest) {
            GscOutcome::Proven(family) => family,
            GscOutcome::Unknown(reason) => panic!("registered use is unknown: {reason:?}"),
        };

        let mut forged_frame = frame.clone();
        forged_frame.history_digest = Digest::of_bytes(b"forged history");
        assert!(!forged_frame.binding_is_valid());
        assert!(matches!(
            forged_frame.compile_use(&kernel, &manifest),
            GscOutcome::Unknown(GscUnknownReason::MalformedFrame)
        ));

        let mut forged_origin = frame.clone();
        forged_origin.principal_source_bindings[0].origin =
            GscOriginEventId(Digest::of_bytes(b"forged origin"));
        assert!(!forged_origin.binding_is_valid());

        let mut forged_declaration = frame.clone();
        forged_declaration.principal_source_bindings[0].declaration =
            frame.principal_source_bindings[1].declaration().clone();
        forged_declaration.binding_digest = registered_frame_binding_digest(
            &forged_declaration.inner,
            &forged_declaration.semantic_manifest_digest,
            &forged_declaration.export_group_digest,
            &forged_declaration.export_index_digest,
            &forged_declaration.history_digest,
            &forged_declaration.active_anchor_digest,
            &forged_declaration.cutoff_boundary_digest,
            &forged_declaration.q3_registry_digest,
            &forged_declaration.principal_source_bindings,
        );
        assert!(!forged_declaration.binding_is_valid());

        let mut forged_coverage = frame.clone();
        forged_coverage.cutoff_coverage_exhaustive = false;
        assert!(!forged_coverage.binding_is_valid());

        let mut forged_use = use_family.clone();
        forged_use.active_anchor_digest = Digest::of_bytes(b"forged anchor");
        assert!(!forged_use.binding_is_valid_for(frame));
        assert!(matches!(
            frame.compile_compute(&kernel, &manifest, &forged_use, 0),
            GscOutcome::Unknown(GscUnknownReason::FamilyMismatch)
        ));
        assert!(matches!(
            frame.compile_compute(&kernel, &manifest, &use_family, 1),
            GscOutcome::Unknown(GscUnknownReason::PortMismatch)
        ));
    }
}
