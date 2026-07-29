//! Concrete production-representation foundations for the lambda/unit
//! successor.
//!
//! This module supplies the Rust side of the finite-global correspondence:
//! an opaque slot table derived from a replayed [`VerifiedSignature`] in exact
//! declaration order, plus an explicit capability for the two primitive
//! public-sort successor checks `Sort 0 : Sort 1` and `Sort 1 : Sort 2`. The
//! capability records public levels `{0, 1}`, inferred successor levels
//! `{1, 2}`, and their union `{0, 1, 2}`. It does not claim structural closure
//! for compound terms, Rust/safe-Agda term-code agreement, or
//! conversion-typing refinement.

use crate::manifest::{
    AuditDecision, AuditUnknownReason, SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3,
    VerifiedSemanticAuditManifestV3,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, DependentContext, Digest, GlobalId, Kernel,
    KernelError, Term, VerifiedSignature,
};
use pen_kernel_synthesis::{
    SynthesisError, VerifiedSynthesizedJudgmentV1, synthesis_protocol_digest_v1,
    synthesize_lambda_unit_v1,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1: u16 = 1;
pub const LAMBDA_UNIT_PUBLIC_UNIVERSE_LEVELS_V1: &[u16] = &[0, 1];
pub const LAMBDA_UNIT_INFERRED_PUBLIC_SORT_SUCCESSOR_LEVELS_V1: &[u16] = &[1, 2];
pub const LAMBDA_UNIT_PUBLIC_AND_INFERRED_SORT_LEVELS_V1: &[u16] = &[0, 1, 2];

/// Zero-based declaration ordinal in a verified signature.
///
/// The wrapped ordinal is private and this type has no deserialization path.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlobalSlotV1(u64);

impl GlobalSlotV1 {
    pub fn ordinal(&self) -> u64 {
        self.0
    }
}

impl CanonicalEncode for GlobalSlotV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.0);
    }
}

/// One direct global dependency and its exact strict-prior slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedGlobalDependencySlotV1 {
    global: GlobalId,
    slot: GlobalSlotV1,
}

impl VerifiedGlobalDependencySlotV1 {
    pub fn global(&self) -> &GlobalId {
        &self.global
    }

    pub fn slot(&self) -> &GlobalSlotV1 {
        &self.slot
    }
}

impl CanonicalEncode for VerifiedGlobalDependencySlotV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.global.encode_canonical(encoder);
        self.slot.encode_canonical(encoder);
    }
}

/// One exact verified declaration at one production slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedGlobalSlotEntryV1 {
    slot: GlobalSlotV1,
    declaration: Declaration,
    direct_dependencies: Arc<[VerifiedGlobalDependencySlotV1]>,
    digest: Digest,
}

impl VerifiedGlobalSlotEntryV1 {
    pub fn slot(&self) -> &GlobalSlotV1 {
        &self.slot
    }

    pub fn global(&self) -> &GlobalId {
        &self.declaration.id
    }

    pub fn declaration(&self) -> &Declaration {
        &self.declaration
    }

    pub fn direct_dependencies(&self) -> &[VerifiedGlobalDependencySlotV1] {
        &self.direct_dependencies
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedGlobalSlotEntryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.slot.encode_canonical(encoder);
        self.declaration.encode_canonical(encoder);
        encoder.sequence(&self.direct_dependencies);
        self.digest.encode_canonical(encoder);
    }
}

mod private {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(super) struct CompleteGlobalSlotCoverage;

    #[derive(Clone, Debug)]
    pub(super) struct CompletePublicSortSuccessors;
}

/// Complete declaration-order slot authority for one verified signature.
///
/// Fields are private and there is no `Deserialize` implementation. The
/// auxiliary map is verifier-derived from `entries` and exists only for exact
/// GlobalId-to-slot round trips.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedGlobalSlotTableV1 {
    schema_version: u16,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    entries: Arc<[VerifiedGlobalSlotEntryV1]>,
    global_to_slot: BTreeMap<GlobalId, GlobalSlotV1>,
    coverage_digest: Digest,
    round_trip_digest: Digest,
    dependency_order_digest: Digest,
    _complete: private::CompleteGlobalSlotCoverage,
    digest: Digest,
}

impl VerifiedGlobalSlotTableV1 {
    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn entries(&self) -> &[VerifiedGlobalSlotEntryV1] {
        &self.entries
    }

    pub fn entry_for_slot(&self, slot: &GlobalSlotV1) -> Option<&VerifiedGlobalSlotEntryV1> {
        usize::try_from(slot.0)
            .ok()
            .and_then(|index| self.entries.get(index))
            .filter(|entry| entry.slot() == slot)
    }

    pub fn slot_for_global(&self, global: &GlobalId) -> Option<&GlobalSlotV1> {
        self.global_to_slot.get(global)
    }

    pub fn global_for_slot(&self, slot: &GlobalSlotV1) -> Option<&GlobalId> {
        self.entry_for_slot(slot)
            .map(VerifiedGlobalSlotEntryV1::global)
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn round_trip_digest(&self) -> &Digest {
        &self.round_trip_digest
    }

    pub fn dependency_order_digest(&self) -> &Digest {
        &self.dependency_order_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedGlobalSlotTableV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        encoder.sequence(&self.entries);
        self.coverage_digest.encode_canonical(encoder);
        self.round_trip_digest.encode_canonical(encoder);
        self.dependency_order_digest.encode_canonical(encoder);
        encoder.tag(1);
        self.digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductionRefinementFailureV1 {
    KernelReplay(KernelError),
    SignatureReplayMismatch,
    ResourceExhausted,
    DuplicateGlobal,
    NonPriorGlobalDependency {
        declaration: GlobalId,
        dependency: GlobalId,
    },
    IncompleteCoverage,
}

impl std::fmt::Display for ProductionRefinementFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KernelReplay(error) => write!(formatter, "signature replay failed: {error}"),
            Self::SignatureReplayMismatch => {
                formatter.write_str("replayed signature differs from the supplied capability")
            }
            Self::ResourceExhausted => {
                formatter.write_str("production-representation traversal exhausted its budget")
            }
            Self::DuplicateGlobal => {
                formatter.write_str("verified signature produced a duplicate global")
            }
            Self::NonPriorGlobalDependency {
                declaration,
                dependency,
            } => write!(
                formatter,
                "global dependency is not strict-prior: {declaration:?} -> {dependency:?}"
            ),
            Self::IncompleteCoverage => {
                formatter.write_str("global slot coverage or a round trip is incomplete")
            }
        }
    }
}

/// Replay a verified signature and derive its exact finite global slot table.
pub fn diagnose_global_slot_table_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
) -> Result<VerifiedGlobalSlotTableV1, ProductionRefinementFailureV1> {
    let replayed = kernel
        .verify_signature(&signature.normalized_wire())
        .map_err(ProductionRefinementFailureV1::KernelReplay)?;
    if replayed.digest() != signature.digest()
        || replayed.declarations() != signature.declarations()
    {
        return Err(ProductionRefinementFailureV1::SignatureReplayMismatch);
    }

    let declarations = signature.declarations();
    let mut global_to_slot = BTreeMap::new();
    for (index, declaration) in declarations.iter().enumerate() {
        let ordinal =
            u64::try_from(index).map_err(|_| ProductionRefinementFailureV1::ResourceExhausted)?;
        if global_to_slot
            .insert(declaration.id.clone(), GlobalSlotV1(ordinal))
            .is_some()
        {
            return Err(ProductionRefinementFailureV1::DuplicateGlobal);
        }
    }

    let mut budget = TraversalBudget::new(kernel);
    let mut entries = Vec::with_capacity(declarations.len());
    for (index, declaration) in declarations.iter().enumerate() {
        let slot = GlobalSlotV1(
            u64::try_from(index).map_err(|_| ProductionRefinementFailureV1::ResourceExhausted)?,
        );
        let dependencies = direct_global_dependencies(declaration, &mut budget)?;
        let mut direct_dependencies = dependencies
            .into_iter()
            .map(|dependency| {
                let dependency_slot =
                    global_to_slot.get(&dependency).cloned().ok_or_else(|| {
                        ProductionRefinementFailureV1::NonPriorGlobalDependency {
                            declaration: declaration.id.clone(),
                            dependency: dependency.clone(),
                        }
                    })?;
                if dependency_slot >= slot {
                    return Err(ProductionRefinementFailureV1::NonPriorGlobalDependency {
                        declaration: declaration.id.clone(),
                        dependency,
                    });
                }
                Ok(VerifiedGlobalDependencySlotV1 {
                    global: dependency,
                    slot: dependency_slot,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        direct_dependencies.sort_by(|left, right| left.slot.cmp(&right.slot));
        let direct_dependencies =
            Arc::<[VerifiedGlobalDependencySlotV1]>::from(direct_dependencies.into_boxed_slice());
        let digest = Digest::of_canonical(
            "pen-semantic-audit/verified-global-slot-entry/v1",
            &GlobalSlotEntryMaterialV1 {
                signature: signature.digest(),
                slot: &slot,
                declaration,
                direct_dependencies: &direct_dependencies,
            },
        );
        entries.push(VerifiedGlobalSlotEntryV1 {
            slot,
            declaration: declaration.clone(),
            direct_dependencies,
            digest,
        });
    }
    let entries = Arc::<[VerifiedGlobalSlotEntryV1]>::from(entries.into_boxed_slice());

    verify_slot_coverage(declarations, &entries, &global_to_slot)?;
    let coverage_digest = Digest::of_canonical(
        "pen-semantic-audit/global-slot-table-coverage/v1",
        &GlobalSlotCoverageMaterialV1 {
            signature: signature.digest(),
            declarations,
            entries: &entries,
        },
    );
    let round_trip_digest = Digest::of_canonical(
        "pen-semantic-audit/global-slot-table-round-trips/v1",
        &GlobalSlotRoundTripMaterialV1 {
            entries: &entries,
            map: &global_to_slot,
        },
    );
    let dependency_order_digest = Digest::of_canonical(
        "pen-semantic-audit/global-slot-table-dependency-order/v1",
        &GlobalSlotDependencyOrderMaterialV1 { entries: &entries },
    );
    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let mut verified = VerifiedGlobalSlotTableV1 {
        schema_version: PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1,
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest,
        entries,
        global_to_slot,
        coverage_digest,
        round_trip_digest,
        dependency_order_digest,
        _complete: private::CompleteGlobalSlotCoverage,
        digest: Digest::of_bytes(b"pending global slot table"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-global-slot-table/v1",
        &GlobalSlotTableMaterialV1 {
            schema_version: verified.schema_version,
            signature: &verified.signature_digest,
            kernel: &verified.kernel_protocol_digest,
            entries: &verified.entries,
            coverage: &verified.coverage_digest,
            round_trips: &verified.round_trip_digest,
            dependency_order: &verified.dependency_order_digest,
        },
    );
    Ok(verified)
}

pub fn verify_global_slot_table_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
) -> AuditDecision<VerifiedGlobalSlotTableV1> {
    match diagnose_global_slot_table_v1(kernel, signature) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(
            ProductionRefinementFailureV1::KernelReplay(KernelError::ResourceExhausted(_))
            | ProductionRefinementFailureV1::ResourceExhausted,
        ) => AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted),
        Err(_) => AuditDecision::Unknown(AuditUnknownReason::MalformedInput),
    }
}

/// One checked universe-successor edge `Sort level : Sort inferred`.
#[derive(Clone, Debug)]
pub struct VerifiedUniverseSuccessorV1 {
    public_level: u16,
    inferred_level: u16,
    synthesis: VerifiedSynthesizedJudgmentV1,
    digest: Digest,
}

impl VerifiedUniverseSuccessorV1 {
    pub fn public_level(&self) -> u16 {
        self.public_level
    }

    pub fn inferred_level(&self) -> u16 {
        self.inferred_level
    }

    pub fn synthesis(&self) -> &VerifiedSynthesizedJudgmentV1 {
        &self.synthesis
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedUniverseSuccessorV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.public_level);
        encoder.u16(self.inferred_level);
        self.synthesis.digest().encode_canonical(encoder);
        self.digest.encode_canonical(encoder);
    }
}

/// The two kernel-checked primitive public-sort successor judgments.
///
/// This is deliberately not a structural universe-closure theorem for all
/// terms in the public grammar.
#[derive(Clone, Debug)]
pub struct VerifiedLambdaUnitPublicSortSuccessorsV1 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    signature_digest: Digest,
    global_slot_table_digest: Digest,
    kernel_protocol_digest: Digest,
    synthesis_protocol_digest: Digest,
    public_levels: Arc<[u16]>,
    inferred_successor_levels: Arc<[u16]>,
    public_and_inferred_levels: Arc<[u16]>,
    successors: Arc<[VerifiedUniverseSuccessorV1]>,
    coverage_digest: Digest,
    _complete: private::CompletePublicSortSuccessors,
    digest: Digest,
}

impl VerifiedLambdaUnitPublicSortSuccessorsV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn global_slot_table_digest(&self) -> &Digest {
        &self.global_slot_table_digest
    }

    pub fn public_levels(&self) -> &[u16] {
        &self.public_levels
    }

    pub fn inferred_successor_levels(&self) -> &[u16] {
        &self.inferred_successor_levels
    }

    pub fn public_and_inferred_levels(&self) -> &[u16] {
        &self.public_and_inferred_levels
    }

    pub fn successors(&self) -> &[VerifiedUniverseSuccessorV1] {
        &self.successors
    }

    pub fn coverage_digest(&self) -> &Digest {
        &self.coverage_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedLambdaUnitPublicSortSuccessorsV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.global_slot_table_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.synthesis_protocol_digest.encode_canonical(encoder);
        encode_u16_sequence(encoder, &self.public_levels);
        encode_u16_sequence(encoder, &self.inferred_successor_levels);
        encode_u16_sequence(encoder, &self.public_and_inferred_levels);
        encoder.sequence(&self.successors);
        self.coverage_digest.encode_canonical(encoder);
        encoder.tag(1);
        self.digest.encode_canonical(encoder);
    }
}

pub fn verify_lambda_unit_public_sort_successors_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    global_slots: &VerifiedGlobalSlotTableV1,
) -> AuditDecision<VerifiedLambdaUnitPublicSortSuccessorsV1> {
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3
        || manifest.manifest().universe_levels != LAMBDA_UNIT_PUBLIC_UNIVERSE_LEVELS_V1
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    if global_slots.signature_digest() != signature.digest()
        || global_slots.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedInput);
    }

    let mut inferred_successor_levels = BTreeSet::new();
    let mut public_and_inferred_levels =
        BTreeSet::from_iter(LAMBDA_UNIT_PUBLIC_UNIVERSE_LEVELS_V1.iter().copied());
    let mut successors = Vec::with_capacity(LAMBDA_UNIT_PUBLIC_UNIVERSE_LEVELS_V1.len());
    for public_level in LAMBDA_UNIT_PUBLIC_UNIVERSE_LEVELS_V1 {
        let term = Term::Sort {
            level: *public_level,
        };
        let synthesis =
            match synthesize_lambda_unit_v1(kernel, signature, &DependentContext::default(), &term)
            {
                Ok(synthesis) => synthesis,
                Err(error) => return public_sort_successor_synthesis_unknown(error),
            };
        let Some(inferred_level) = public_level.checked_add(1) else {
            return AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted);
        };
        if synthesis.normalized_type()
            != &(Term::Sort {
                level: inferred_level,
            })
            || synthesis.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
            || synthesis.synthesis_protocol_digest() != &synthesis_protocol_digest_v1()
        {
            return AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify);
        }
        inferred_successor_levels.insert(inferred_level);
        public_and_inferred_levels.insert(inferred_level);
        let digest = Digest::of_canonical(
            "pen-semantic-audit/verified-universe-successor/v1",
            &UniverseSuccessorMaterialV1 {
                manifest: manifest.candidate_digest(),
                signature: signature.digest(),
                public_level: *public_level,
                inferred_level,
                synthesis: synthesis.digest(),
            },
        );
        successors.push(VerifiedUniverseSuccessorV1 {
            public_level: *public_level,
            inferred_level,
            synthesis,
            digest,
        });
    }
    let public_levels = Arc::<[u16]>::from(
        LAMBDA_UNIT_PUBLIC_UNIVERSE_LEVELS_V1
            .to_vec()
            .into_boxed_slice(),
    );
    let inferred_successor_levels = Arc::<[u16]>::from(
        inferred_successor_levels
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let public_and_inferred_levels = Arc::<[u16]>::from(
        public_and_inferred_levels
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    if inferred_successor_levels.as_ref() != LAMBDA_UNIT_INFERRED_PUBLIC_SORT_SUCCESSOR_LEVELS_V1
        || public_and_inferred_levels.as_ref() != LAMBDA_UNIT_PUBLIC_AND_INFERRED_SORT_LEVELS_V1
    {
        return AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration);
    }
    let successors = Arc::<[VerifiedUniverseSuccessorV1]>::from(successors.into_boxed_slice());
    let coverage_digest = Digest::of_canonical(
        "pen-semantic-audit/lambda-unit-public-sort-successor-coverage/v1",
        &PublicSortSuccessorCoverageMaterialV1 {
            manifest: manifest.candidate_digest(),
            signature: signature.digest(),
            global_slots: global_slots.digest(),
            public_levels: &public_levels,
            inferred_successor_levels: &inferred_successor_levels,
            public_and_inferred_levels: &public_and_inferred_levels,
            successors: &successors,
        },
    );
    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let synthesis_protocol_digest = synthesis_protocol_digest_v1();
    let mut verified = VerifiedLambdaUnitPublicSortSuccessorsV1 {
        schema_version: PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1,
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        global_slot_table_digest: global_slots.digest().clone(),
        kernel_protocol_digest,
        synthesis_protocol_digest,
        public_levels,
        inferred_successor_levels,
        public_and_inferred_levels,
        successors,
        coverage_digest,
        _complete: private::CompletePublicSortSuccessors,
        digest: Digest::of_bytes(b"pending lambda/unit public sort successors"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-lambda-unit-public-sort-successors/v1",
        &PublicSortSuccessorsMaterialV1 {
            schema_version: verified.schema_version,
            manifest: &verified.semantic_manifest_digest,
            signature: &verified.signature_digest,
            global_slots: &verified.global_slot_table_digest,
            kernel: &verified.kernel_protocol_digest,
            synthesis: &verified.synthesis_protocol_digest,
            public_levels: &verified.public_levels,
            inferred_successor_levels: &verified.inferred_successor_levels,
            public_and_inferred_levels: &verified.public_and_inferred_levels,
            successors: &verified.successors,
            coverage: &verified.coverage_digest,
        },
    );
    AuditDecision::Proven(verified)
}

fn public_sort_successor_synthesis_unknown(
    error: SynthesisError,
) -> AuditDecision<VerifiedLambdaUnitPublicSortSuccessorsV1> {
    match error {
        SynthesisError::ResourceExhausted(_)
        | SynthesisError::Kernel(KernelError::ResourceExhausted(_))
        | SynthesisError::AggregateKernelReplay(KernelError::ResourceExhausted(_)) => {
            AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted)
        }
        _ => AuditDecision::Unknown(AuditUnknownReason::KernelCouldNotCertify),
    }
}

fn verify_slot_coverage(
    declarations: &[Declaration],
    entries: &[VerifiedGlobalSlotEntryV1],
    global_to_slot: &BTreeMap<GlobalId, GlobalSlotV1>,
) -> Result<(), ProductionRefinementFailureV1> {
    if declarations.len() != entries.len() || entries.len() != global_to_slot.len() {
        return Err(ProductionRefinementFailureV1::IncompleteCoverage);
    }
    for (index, (declaration, entry)) in declarations.iter().zip(entries).enumerate() {
        let ordinal =
            u64::try_from(index).map_err(|_| ProductionRefinementFailureV1::ResourceExhausted)?;
        if entry.slot().ordinal() != ordinal
            || entry.declaration() != declaration
            || global_to_slot.get(&declaration.id) != Some(entry.slot())
            || entries.get(index).map(VerifiedGlobalSlotEntryV1::global) != Some(&declaration.id)
            || entry.direct_dependencies().iter().any(|dependency| {
                dependency.slot() >= entry.slot()
                    || global_to_slot.get(dependency.global()) != Some(dependency.slot())
            })
        {
            return Err(ProductionRefinementFailureV1::IncompleteCoverage);
        }
    }
    Ok(())
}

fn direct_global_dependencies(
    declaration: &Declaration,
    budget: &mut TraversalBudget,
) -> Result<BTreeSet<GlobalId>, ProductionRefinementFailureV1> {
    let mut dependencies = BTreeSet::new();
    collect_term_globals(&declaration.ty, budget, &mut dependencies)?;
    if let Some(body) = &declaration.body {
        collect_term_globals(body, budget, &mut dependencies)?;
    }
    Ok(dependencies)
}

fn collect_term_globals(
    root: &Term,
    budget: &mut TraversalBudget,
    dependencies: &mut BTreeSet<GlobalId>,
) -> Result<(), ProductionRefinementFailureV1> {
    let mut stack = vec![(root, 0_u16)];
    while let Some((term, depth)) = stack.pop() {
        budget.charge(depth)?;
        let child_depth = depth
            .checked_add(1)
            .ok_or(ProductionRefinementFailureV1::ResourceExhausted)?;
        match term {
            Term::Global { id } => {
                dependencies.insert(id.clone());
            }
            Term::Pi { parameter, body }
            | Term::Sigma { parameter, body }
            | Term::Apply {
                function: parameter,
                argument: body,
            } => {
                stack.push((body, child_depth));
                stack.push((parameter, child_depth));
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                stack.push((body, child_depth));
                stack.push((parameter_type, child_depth));
            }
            Term::Pair {
                sigma_type,
                first,
                second,
            } => {
                stack.push((second, child_depth));
                stack.push((first, child_depth));
                stack.push((sigma_type, child_depth));
            }
            Term::First { pair } | Term::Second { pair } => {
                stack.push((pair, child_depth));
            }
            Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => {}
        }
    }
    Ok(())
}

struct TraversalBudget {
    operations_left: u32,
    max_depth: u16,
}

impl TraversalBudget {
    fn new(kernel: &Kernel) -> Self {
        Self {
            operations_left: kernel.limits().max_operations,
            max_depth: kernel.limits().max_depth,
        }
    }

    fn charge(&mut self, depth: u16) -> Result<(), ProductionRefinementFailureV1> {
        if depth > self.max_depth {
            return Err(ProductionRefinementFailureV1::ResourceExhausted);
        }
        self.operations_left = self
            .operations_left
            .checked_sub(1)
            .ok_or(ProductionRefinementFailureV1::ResourceExhausted)?;
        Ok(())
    }
}

fn encode_u16_sequence(encoder: &mut CanonicalEncoder, values: &[u16]) {
    encoder.u64(values.len() as u64);
    for value in values {
        encoder.u16(*value);
    }
}

struct GlobalSlotEntryMaterialV1<'a> {
    signature: &'a Digest,
    slot: &'a GlobalSlotV1,
    declaration: &'a Declaration,
    direct_dependencies: &'a [VerifiedGlobalDependencySlotV1],
}

impl CanonicalEncode for GlobalSlotEntryMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.signature.encode_canonical(encoder);
        self.slot.encode_canonical(encoder);
        self.declaration.encode_canonical(encoder);
        encoder.sequence(self.direct_dependencies);
    }
}

struct GlobalSlotCoverageMaterialV1<'a> {
    signature: &'a Digest,
    declarations: &'a [Declaration],
    entries: &'a [VerifiedGlobalSlotEntryV1],
}

impl CanonicalEncode for GlobalSlotCoverageMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.signature.encode_canonical(encoder);
        encoder.sequence(self.declarations);
        encoder.sequence(self.entries);
        encoder.u64(self.declarations.len() as u64);
        encoder.u64(self.entries.len() as u64);
    }
}

struct GlobalSlotRoundTripMaterialV1<'a> {
    entries: &'a [VerifiedGlobalSlotEntryV1],
    map: &'a BTreeMap<GlobalId, GlobalSlotV1>,
}

impl CanonicalEncode for GlobalSlotRoundTripMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.entries.len() as u64);
        for entry in self.entries {
            entry.slot().encode_canonical(encoder);
            entry.global().encode_canonical(encoder);
            self.map
                .get(entry.global())
                .expect("verified complete global slot map")
                .encode_canonical(encoder);
        }
    }
}

struct GlobalSlotDependencyOrderMaterialV1<'a> {
    entries: &'a [VerifiedGlobalSlotEntryV1],
}

impl CanonicalEncode for GlobalSlotDependencyOrderMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.entries.len() as u64);
        for entry in self.entries {
            entry.slot().encode_canonical(encoder);
            encoder.sequence(entry.direct_dependencies());
        }
    }
}

struct GlobalSlotTableMaterialV1<'a> {
    schema_version: u16,
    signature: &'a Digest,
    kernel: &'a Digest,
    entries: &'a [VerifiedGlobalSlotEntryV1],
    coverage: &'a Digest,
    round_trips: &'a Digest,
    dependency_order: &'a Digest,
}

impl CanonicalEncode for GlobalSlotTableMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.signature.encode_canonical(encoder);
        self.kernel.encode_canonical(encoder);
        encoder.sequence(self.entries);
        self.coverage.encode_canonical(encoder);
        self.round_trips.encode_canonical(encoder);
        self.dependency_order.encode_canonical(encoder);
    }
}

struct UniverseSuccessorMaterialV1<'a> {
    manifest: &'a Digest,
    signature: &'a Digest,
    public_level: u16,
    inferred_level: u16,
    synthesis: &'a Digest,
}

impl CanonicalEncode for UniverseSuccessorMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.signature.encode_canonical(encoder);
        encoder.u16(self.public_level);
        encoder.u16(self.inferred_level);
        self.synthesis.encode_canonical(encoder);
    }
}

struct PublicSortSuccessorCoverageMaterialV1<'a> {
    manifest: &'a Digest,
    signature: &'a Digest,
    global_slots: &'a Digest,
    public_levels: &'a [u16],
    inferred_successor_levels: &'a [u16],
    public_and_inferred_levels: &'a [u16],
    successors: &'a [VerifiedUniverseSuccessorV1],
}

impl CanonicalEncode for PublicSortSuccessorCoverageMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest.encode_canonical(encoder);
        self.signature.encode_canonical(encoder);
        self.global_slots.encode_canonical(encoder);
        encode_u16_sequence(encoder, self.public_levels);
        encode_u16_sequence(encoder, self.inferred_successor_levels);
        encode_u16_sequence(encoder, self.public_and_inferred_levels);
        encoder.sequence(self.successors);
        encoder.u64(self.successors.len() as u64);
    }
}

struct PublicSortSuccessorsMaterialV1<'a> {
    schema_version: u16,
    manifest: &'a Digest,
    signature: &'a Digest,
    global_slots: &'a Digest,
    kernel: &'a Digest,
    synthesis: &'a Digest,
    public_levels: &'a [u16],
    inferred_successor_levels: &'a [u16],
    public_and_inferred_levels: &'a [u16],
    successors: &'a [VerifiedUniverseSuccessorV1],
    coverage: &'a Digest,
}

impl CanonicalEncode for PublicSortSuccessorsMaterialV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.manifest.encode_canonical(encoder);
        self.signature.encode_canonical(encoder);
        self.global_slots.encode_canonical(encoder);
        self.kernel.encode_canonical(encoder);
        self.synthesis.encode_canonical(encoder);
        encode_u16_sequence(encoder, self.public_levels);
        encode_u16_sequence(encoder, self.inferred_successor_levels);
        encode_u16_sequence(encoder, self.public_and_inferred_levels);
        encoder.sequence(self.successors);
        self.coverage.encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v3,
        verify_semantic_audit_lambda_unit_manifest_v3,
    };
    use pen_kernel::{KernelLimits, UncheckedSignature};

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("safe kernel")
    }

    fn global(label: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    fn dependency_signature(kernel: &Kernel) -> VerifiedSignature {
        let type_head = global(b"production-refinement/type-head");
        let unit_value = global(b"production-refinement/unit-value");
        let dependent_type = global(b"production-refinement/dependent-type");
        let body_alias = global(b"production-refinement/body-alias");
        kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: type_head.clone(),
                        ty: Term::Sort { level: 0 },
                        body: None,
                    },
                    Declaration {
                        id: unit_value.clone(),
                        ty: Term::UnitType,
                        body: None,
                    },
                    Declaration {
                        id: dependent_type,
                        ty: Term::Global { id: type_head },
                        body: None,
                    },
                    Declaration {
                        id: body_alias,
                        ty: Term::UnitType,
                        body: Some(Term::Global { id: unit_value }),
                    },
                ],
            })
            .expect("strict-prior dependency signature")
    }

    fn manifest() -> VerifiedSemanticAuditManifestV3 {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v3(
            &proposed_semantic_audit_lambda_unit_manifest_v3(),
        ) else {
            panic!("exact V3 manifest");
        };
        manifest
    }

    #[test]
    fn slot_table_covers_order_round_trips_and_strict_prior_dependencies() {
        let kernel = kernel();
        let signature = dependency_signature(&kernel);
        let AuditDecision::Proven(table) = verify_global_slot_table_v1(&kernel, &signature) else {
            panic!("verified signature must produce a complete slot table");
        };
        assert_eq!(table.entries().len(), 4);
        for (index, declaration) in signature.declarations().iter().enumerate() {
            let slot = table
                .slot_for_global(&declaration.id)
                .expect("global-to-slot round trip");
            assert_eq!(slot.ordinal(), index as u64);
            assert_eq!(table.global_for_slot(slot), Some(&declaration.id));
            assert_eq!(
                table.entry_for_slot(slot).map(|entry| entry.declaration()),
                Some(declaration)
            );
        }
        assert_eq!(
            table.entries()[2].direct_dependencies()[0].slot().ordinal(),
            0
        );
        assert_eq!(
            table.entries()[3].direct_dependencies()[0].slot().ordinal(),
            1
        );
        assert!(table.entries().iter().all(|entry| {
            entry
                .direct_dependencies()
                .iter()
                .all(|dependency| dependency.slot() < entry.slot())
        }));
    }

    #[test]
    fn independent_declaration_reordering_changes_slots_and_table_identity() {
        let kernel = kernel();
        let first = global(b"production-refinement/independent-a");
        let second = global(b"production-refinement/independent-b");
        let declaration = |id| Declaration {
            id,
            ty: Term::Sort { level: 0 },
            body: None,
        };
        let left = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![declaration(first.clone()), declaration(second.clone())],
            })
            .expect("left order");
        let right = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![declaration(second.clone()), declaration(first.clone())],
            })
            .expect("right order");
        let left_table = diagnose_global_slot_table_v1(&kernel, &left).expect("left slot table");
        let right_table = diagnose_global_slot_table_v1(&kernel, &right).expect("right slot table");
        assert_eq!(left_table.slot_for_global(&first).unwrap().ordinal(), 0);
        assert_eq!(right_table.slot_for_global(&first).unwrap().ordinal(), 1);
        assert_ne!(left_table.digest(), right_table.digest());
    }

    #[test]
    fn duplicate_and_forward_global_inputs_never_reach_slot_authority() {
        let kernel = kernel();
        let duplicate = global(b"production-refinement/duplicate");
        assert!(matches!(
            kernel.verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: duplicate.clone(),
                        ty: Term::Sort { level: 0 },
                        body: None,
                    },
                    Declaration {
                        id: duplicate,
                        ty: Term::Sort { level: 0 },
                        body: None,
                    },
                ],
            }),
            Err(KernelError::DuplicateGlobal)
        ));

        let earlier = global(b"production-refinement/forward-earlier");
        let later = global(b"production-refinement/forward-later");
        assert!(
            kernel
                .verify_signature(&UncheckedSignature {
                    declarations: vec![
                        Declaration {
                            id: earlier,
                            ty: Term::Global { id: later.clone() },
                            body: None,
                        },
                        Declaration {
                            id: later,
                            ty: Term::Sort { level: 0 },
                            body: None,
                        },
                    ],
                })
                .is_err()
        );
    }

    #[test]
    fn public_sort_successors_distinguish_public_inferred_and_union_levels() {
        let kernel = kernel();
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let table = diagnose_global_slot_table_v1(&kernel, &signature).expect("empty slot table");
        let AuditDecision::Proven(successors) =
            verify_lambda_unit_public_sort_successors_v1(&manifest(), &kernel, &signature, &table)
        else {
            panic!("exact primitive public-sort successors");
        };
        assert_eq!(
            successors.public_levels(),
            LAMBDA_UNIT_PUBLIC_UNIVERSE_LEVELS_V1
        );
        assert_eq!(
            successors.inferred_successor_levels(),
            LAMBDA_UNIT_INFERRED_PUBLIC_SORT_SUCCESSOR_LEVELS_V1
        );
        assert_eq!(
            successors.public_and_inferred_levels(),
            LAMBDA_UNIT_PUBLIC_AND_INFERRED_SORT_LEVELS_V1
        );
        assert_eq!(successors.successors().len(), 2);
        assert_eq!(successors.successors()[0].public_level(), 0);
        assert_eq!(successors.successors()[0].inferred_level(), 1);
        assert_eq!(successors.successors()[1].public_level(), 1);
        assert_eq!(successors.successors()[1].inferred_level(), 2);
    }

    #[test]
    fn public_sort_successors_reject_a_slot_table_for_another_signature() {
        let kernel = kernel();
        let empty = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let nonempty = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: global(b"production-refinement/mismatch"),
                    ty: Term::Sort { level: 0 },
                    body: None,
                }],
            })
            .expect("nonempty signature");
        let table = diagnose_global_slot_table_v1(&kernel, &empty).expect("empty table");
        assert!(matches!(
            verify_lambda_unit_public_sort_successors_v1(&manifest(), &kernel, &nonempty, &table,),
            AuditDecision::Unknown(AuditUnknownReason::MalformedInput)
        ));
    }
}
