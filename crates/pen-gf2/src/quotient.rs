//! Minimal complete quotient for the deterministic one-response profile.

use crate::operational::VerifiedDemandConnectedResponse;
use crate::rewrite::VerifiedEquationExtensionSet;
use pen_demand::gsc::{
    FreshEliminatorFillerGrammar, FreshEliminatorHeadGrammar, FreshHeadIdentityGrammar,
    GeneratedEquationCoverage, GscOutcome, GscUnknownReason, LiveUseFamilyShape, Q0RewriteGrammar,
    Q2PresentationGrammar, Q3EdgeGrammar, ResponseCandidateCanonicalOrder,
    VerifiedGscSemanticManifest,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, GlobalId, VerifiedSignature};
use pen_law::{VerifiedOriginCutoffQ3Registry, VerifiedRegisteredActiveDemandInventory};

/// Exhaustive, canonically ordered response carrier for one registered
/// one-nullary family tuple.
#[derive(Clone, Debug)]
pub struct VerifiedCandidateCarrier {
    semantic_manifest_digest: Digest,
    registered_active_demand_inventory_binding_digest: Digest,
    registered_frame_binding_digest: Digest,
    registered_frame_tuple_binding_digest: Digest,
    use_family_binding_digest: Digest,
    compute_family_binding_digests: Vec<Digest>,
    canonical_fresh_head: GlobalId,
    equation_set_digest: Digest,
    response_digest: Digest,
    candidate_count: u32,
    frame_count: u32,
    live_use_family_count: u32,
    compute_family_count: u32,
    family_count: u32,
    port_count: u32,
    constructor_count: u32,
    exhaustion_evidence_digest: Digest,
    digest: Digest,
}

impl VerifiedCandidateCarrier {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn registered_active_demand_inventory_binding_digest(&self) -> &Digest {
        &self.registered_active_demand_inventory_binding_digest
    }

    pub fn registered_frame_binding_digest(&self) -> &Digest {
        &self.registered_frame_binding_digest
    }

    pub fn registered_frame_tuple_binding_digest(&self) -> &Digest {
        &self.registered_frame_tuple_binding_digest
    }

    pub fn use_family_binding_digest(&self) -> &Digest {
        &self.use_family_binding_digest
    }

    pub fn compute_family_binding_digests(&self) -> &[Digest] {
        &self.compute_family_binding_digests
    }

    pub fn canonical_fresh_head(&self) -> &GlobalId {
        &self.canonical_fresh_head
    }

    pub fn equation_set_digest(&self) -> &Digest {
        &self.equation_set_digest
    }

    pub fn response_digest(&self) -> &Digest {
        &self.response_digest
    }

    pub fn candidate_count(&self) -> u32 {
        self.candidate_count
    }

    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn live_use_family_count(&self) -> u32 {
        self.live_use_family_count
    }

    pub fn compute_family_count(&self) -> u32 {
        self.compute_family_count
    }

    pub fn family_count(&self) -> u32 {
        self.family_count
    }

    pub fn port_count(&self) -> u32 {
        self.port_count
    }

    pub fn constructor_count(&self) -> u32 {
        self.constructor_count
    }

    pub fn is_exhaustive(&self) -> bool {
        self.candidate_count == self.live_use_family_count
            && self.frame_count == self.live_use_family_count
            && self.family_count
                == self
                    .live_use_family_count
                    .saturating_add(self.compute_family_count)
            && self.compute_family_count == self.constructor_count
            && self.port_count == self.family_count
    }

    pub fn exhaustion_evidence_digest(&self) -> &Digest {
        &self.exhaustion_evidence_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

pub(crate) fn verify_candidate_carrier(
    semantic: &VerifiedGscSemanticManifest,
    inventory: &VerifiedRegisteredActiveDemandInventory,
    equation_set: &VerifiedEquationExtensionSet,
    response: &VerifiedDemandConnectedResponse,
) -> GscOutcome<VerifiedCandidateCarrier> {
    let grammar = &semantic.manifest().response_candidate_grammar;
    if grammar.schema_version != 1
        || grammar.live_use_family_shape
            != LiveUseFamilyShape::EveryLiveSingleTermOutputUseFamilyV1
        || grammar.fresh_head_grammar
            != FreshEliminatorHeadGrammar::OneBodylessHeadClosedOverCanonicalUseContextV1
        || grammar.filler_grammar
            != FreshEliminatorFillerGrammar::HeadAppliedToAllContextVariablesInBinderOrderV1
        || grammar.generated_equation_coverage
            != GeneratedEquationCoverage::EveryComputeEquationReferencingExactUsePortV1
        || grammar.fresh_head_identity
            != FreshHeadIdentityGrammar::SemanticBoundaryFrameUsePortOrderedComputePortsAndClosedTypeV1
        || grammar.canonical_order
            != ResponseCandidateCanonicalOrder::UseFamilyUsePortThenComputeFamilyOutputPortDigestV1
        || grammar.max_live_use_families < 1
        || grammar.max_compute_families_per_use < 1
        || grammar.max_equations_per_candidate < 1
        || grammar.max_response_candidates < 1
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
    }
    if !inventory.binding_is_valid()
        || !inventory.cutoff_coverage_is_exhaustive()
        || inventory.semantic_manifest_digest() != semantic.digest()
        || inventory.use_family_count() > u64::from(grammar.max_live_use_families)
        || inventory.frame_count() > u64::from(grammar.max_live_use_families)
        || inventory.tuples().iter().any(|tuple| {
            tuple.compute_families().len() > usize::from(grammar.max_compute_families_per_use)
        })
        || inventory.compute_family_count()
            > u64::from(grammar.max_live_use_families)
                .saturating_mul(u64::from(grammar.max_compute_families_per_use))
        || inventory.use_family_count() > u64::from(grammar.max_response_candidates)
        || inventory.compute_family_count()
            > u64::from(grammar.max_live_use_families)
                .saturating_mul(u64::from(grammar.max_equations_per_candidate))
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
    }
    let [tuple] = inventory.tuples() else {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    };
    let candidate_compute_families = tuple
        .candidate_compute_families_in_port_order()
        .collect::<Vec<_>>();
    let [compute_tuple] = candidate_compute_families.as_slice() else {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    };
    let frame = tuple.frame();
    let use_family = tuple.use_family();
    let compute_family = compute_tuple.family();
    let one_constructor = frame.inner().code().constructor_id(0);
    if !tuple.binding_is_valid()
        || !compute_tuple.binding_is_valid_for(frame, use_family, tuple.exact_use_port())
        || !frame.binding_is_valid()
        || !frame.inner().code().is_one_nullary()
        || frame.semantic_manifest_digest() != semantic.digest()
        || !use_family.binding_is_valid_for(frame)
        || !compute_family.binding_is_valid_for(frame)
        || !use_family.is_use()
        || !compute_family.is_compute()
        || compute_family.source_use_family_binding_digest() != Some(use_family.binding_digest())
        || use_family.exact_use_port() != compute_family.exact_use_port()
        || compute_family.constructor() != one_constructor
        || use_family.inner().ports().len() != 1
        || compute_family.inner().ports().len() != 1
        || compute_tuple.output_port() != compute_family.inner().ports()[0].key()
        || equation_set.clauses_len() != 1
        || equation_set.digest() != response.equation_set_digest()
        || response.use_derivation().port() != use_family.inner().ports()[0].key()
    {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }

    let counts = match CandidateCarrierCounts::from_inventory(inventory) {
        Some(counts) => counts,
        None => return GscOutcome::Unknown(GscUnknownReason::ResourceExhausted),
    };
    if counts.frame_count != 1
        || counts.live_use_family_count != 1
        || counts.compute_family_count != 1
        || counts.candidate_count != 1
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    }
    let exhaustion_evidence_digest = {
        let mut encoder = CanonicalEncoder::new();
        semantic.digest().encode_canonical(&mut encoder);
        grammar.encode_canonical(&mut encoder);
        inventory.binding_digest().encode_canonical(&mut encoder);
        tuple.binding_digest().encode_canonical(&mut encoder);
        frame.binding_digest().encode_canonical(&mut encoder);
        use_family.binding_digest().encode_canonical(&mut encoder);
        compute_family
            .binding_digest()
            .encode_canonical(&mut encoder);
        use_family.inner().ports()[0]
            .key()
            .encode_canonical(&mut encoder);
        compute_family.inner().ports()[0]
            .key()
            .encode_canonical(&mut encoder);
        encode_inventory_order(inventory, &mut encoder);
        counts.encode_canonical(&mut encoder);
        Digest::of_domain_bytes(
            "pen-gf2/candidate-carrier-exhaustion/v1",
            encoder.as_bytes(),
        )
    };
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        exhaustion_evidence_digest.encode_canonical(&mut encoder);
        equation_set
            .clause()
            .fresh_head()
            .encode_canonical(&mut encoder);
        equation_set.digest().encode_canonical(&mut encoder);
        response.digest().encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/candidate-carrier/v1", encoder.as_bytes())
    };
    GscOutcome::Proven(VerifiedCandidateCarrier {
        semantic_manifest_digest: semantic.digest().clone(),
        registered_active_demand_inventory_binding_digest: inventory.binding_digest().clone(),
        registered_frame_binding_digest: frame.binding_digest().clone(),
        registered_frame_tuple_binding_digest: tuple.binding_digest().clone(),
        use_family_binding_digest: use_family.binding_digest().clone(),
        compute_family_binding_digests: candidate_compute_families
            .iter()
            .map(|tuple| tuple.family().binding_digest().clone())
            .collect(),
        canonical_fresh_head: equation_set.clause().fresh_head().clone(),
        equation_set_digest: equation_set.digest().clone(),
        response_digest: response.digest().clone(),
        candidate_count: counts.candidate_count,
        frame_count: counts.frame_count,
        live_use_family_count: counts.live_use_family_count,
        compute_family_count: counts.compute_family_count,
        family_count: counts.family_count,
        port_count: counts.port_count,
        constructor_count: counts.constructor_count,
        exhaustion_evidence_digest,
        digest,
    })
}

#[derive(Clone, Copy)]
struct CandidateCarrierCounts {
    candidate_count: u32,
    frame_count: u32,
    live_use_family_count: u32,
    compute_family_count: u32,
    family_count: u32,
    port_count: u32,
    constructor_count: u32,
}

impl CandidateCarrierCounts {
    fn from_inventory(inventory: &VerifiedRegisteredActiveDemandInventory) -> Option<Self> {
        Some(Self {
            candidate_count: u32::try_from(inventory.use_family_count()).ok()?,
            frame_count: u32::try_from(inventory.frame_count()).ok()?,
            live_use_family_count: u32::try_from(inventory.use_family_count()).ok()?,
            compute_family_count: u32::try_from(inventory.compute_family_count()).ok()?,
            family_count: u32::try_from(inventory.family_count()).ok()?,
            port_count: u32::try_from(inventory.port_count()).ok()?,
            constructor_count: u32::try_from(inventory.constructor_count()).ok()?,
        })
    }

    fn encode_canonical(self, encoder: &mut CanonicalEncoder) {
        encoder.u32(self.candidate_count);
        encoder.u32(self.frame_count);
        encoder.u32(self.live_use_family_count);
        encoder.u32(self.compute_family_count);
        encoder.u32(self.family_count);
        encoder.u32(self.port_count);
        encoder.u32(self.constructor_count);
    }
}

fn encode_inventory_order(
    inventory: &VerifiedRegisteredActiveDemandInventory,
    encoder: &mut CanonicalEncoder,
) {
    for digests in [
        inventory.ordered_export_group_digests(),
        inventory.ordered_frame_binding_digests(),
        inventory.ordered_family_binding_digests(),
        inventory.ordered_port_digests(),
        inventory.ordered_constructor_digests(),
    ] {
        encoder.u64(digests.len() as u64);
        for digest in digests {
            digest.encode_canonical(encoder);
        }
    }
}

/// The sole Q0-normalized response class.
#[derive(Clone, Debug)]
pub struct VerifiedQ0ResponseClass {
    representative_digest: Digest,
    equation_digest: Digest,
    digest: Digest,
}

impl VerifiedQ0ResponseClass {
    pub fn representative_digest(&self) -> &Digest {
        &self.representative_digest
    }

    pub fn equation_digest(&self) -> &Digest {
        &self.equation_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Complete finite quotient for this profile.
#[derive(Clone, Debug)]
pub struct VerifiedFiniteQuotient {
    class: VerifiedQ0ResponseClass,
    response_count: u32,
    q2_distinct_pair_decisions: u32,
    q3_edge_count: u32,
    q3_registry_digest: Digest,
    complete: bool,
    digest: Digest,
}

impl VerifiedFiniteQuotient {
    pub fn classes_len(&self) -> usize {
        1
    }

    pub fn class(&self) -> &VerifiedQ0ResponseClass {
        &self.class
    }

    pub fn response_count(&self) -> u32 {
        self.response_count
    }

    pub fn q2_distinct_pair_decisions(&self) -> u32 {
        self.q2_distinct_pair_decisions
    }

    pub fn q3_edge_count(&self) -> u32 {
        self.q3_edge_count
    }

    pub fn q3_registry_digest(&self) -> &Digest {
        &self.q3_registry_digest
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

pub(crate) fn verify_singleton_quotient(
    semantic: &VerifiedGscSemanticManifest,
    boundary: &VerifiedSignature,
    q3_registry: &VerifiedOriginCutoffQ3Registry,
    carrier: &VerifiedCandidateCarrier,
    response: &VerifiedDemandConnectedResponse,
) -> GscOutcome<VerifiedFiniteQuotient> {
    let q3 = Q3RegistryView {
        is_empty: q3_registry.is_empty(),
        cutoff_boundary_digest: q3_registry.cutoff_boundary_digest(),
        digest: q3_registry.digest(),
    };
    verify_bounded_quotient(semantic, boundary, q3, carrier, response, 0)
}

#[derive(Clone, Copy)]
struct Q3RegistryView<'a> {
    is_empty: bool,
    cutoff_boundary_digest: &'a Digest,
    digest: &'a Digest,
}

fn verify_bounded_quotient(
    semantic: &VerifiedGscSemanticManifest,
    boundary: &VerifiedSignature,
    q3_registry: Q3RegistryView<'_>,
    carrier: &VerifiedCandidateCarrier,
    response: &VerifiedDemandConnectedResponse,
    declared_q3_edges: usize,
) -> GscOutcome<VerifiedFiniteQuotient> {
    if semantic.manifest().q0_rewrites
        != Q0RewriteGrammar::DependentCoreAndAdmittedFreshComputationV1
        || semantic.manifest().q2_presentations
            != Q2PresentationGrammar::BinderTelescopeAndTransparentAliasV1
        || semantic.manifest().q3_edges != Q3EdgeGrammar::EmptyOriginCutoffV1
        || carrier.semantic_manifest_digest() != semantic.digest()
        || carrier.response_digest() != response.digest()
        || carrier.equation_set_digest() != response.equation_set_digest()
        || !carrier.is_exhaustive()
        || carrier.candidate_count() != 1
        || carrier.frame_count() != 1
        || carrier.live_use_family_count() != 1
        || carrier.compute_family_count() != 1
        || declared_q3_edges != 0
        || !q3_registry.is_empty
        || q3_registry.cutoff_boundary_digest != boundary.digest()
        || !response.all_outputs_filled()
        || !response.all_positive_clauses_connected()
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedBoundary);
    }

    let representative_digest = {
        let use_derivation = response.use_derivation();
        let mut encoder = CanonicalEncoder::new();
        semantic.digest().encode_canonical(&mut encoder);
        use_derivation.context().encode_canonical(&mut encoder);
        use_derivation.term().encode_canonical(&mut encoder);
        use_derivation.target().encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/q0-response-representative/v1", encoder.as_bytes())
    };
    let equation_digest = response.equation_set_digest().clone();
    let class_digest = {
        let mut encoder = CanonicalEncoder::new();
        representative_digest.encode_canonical(&mut encoder);
        equation_digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/q0-response-class/v1", encoder.as_bytes())
    };
    let class = VerifiedQ0ResponseClass {
        representative_digest,
        equation_digest,
        digest: class_digest,
    };
    let quotient_digest = {
        let mut encoder = CanonicalEncoder::new();
        semantic.digest().encode_canonical(&mut encoder);
        boundary.digest().encode_canonical(&mut encoder);
        response.digest().encode_canonical(&mut encoder);
        carrier.digest().encode_canonical(&mut encoder);
        class.digest().encode_canonical(&mut encoder);
        q3_registry.digest.encode_canonical(&mut encoder);
        encoder.u32(1);
        encoder.u32(0);
        encoder.u32(0);
        encoder.tag(1);
        Digest::of_domain_bytes("pen-gf2/finite-quotient/v1", encoder.as_bytes())
    };
    GscOutcome::Proven(VerifiedFiniteQuotient {
        class,
        response_count: carrier.candidate_count(),
        q2_distinct_pair_decisions: 0,
        q3_edge_count: 0,
        q3_registry_digest: q3_registry.digest.clone(),
        complete: true,
        digest: quotient_digest,
    })
}

#[cfg(test)]
pub(crate) fn exercise_quotient_guards_for_test(
    semantic: &VerifiedGscSemanticManifest,
    boundary: &VerifiedSignature,
    equation_set: &VerifiedEquationExtensionSet,
    response: &VerifiedDemandConnectedResponse,
) -> bool {
    let binding = Digest::of_bytes(b"synthetic registered binding");
    let exhaustion = Digest::of_bytes(b"synthetic exhaustion");
    let carrier_digest = Digest::of_bytes(b"synthetic carrier");
    let carrier = VerifiedCandidateCarrier {
        semantic_manifest_digest: semantic.digest().clone(),
        registered_active_demand_inventory_binding_digest: binding.clone(),
        registered_frame_binding_digest: binding.clone(),
        registered_frame_tuple_binding_digest: binding.clone(),
        use_family_binding_digest: binding.clone(),
        compute_family_binding_digests: vec![binding],
        canonical_fresh_head: equation_set.clause().fresh_head().clone(),
        equation_set_digest: equation_set.digest().clone(),
        response_digest: response.digest().clone(),
        candidate_count: 1,
        frame_count: 1,
        live_use_family_count: 1,
        compute_family_count: 1,
        family_count: 2,
        port_count: 2,
        constructor_count: 1,
        exhaustion_evidence_digest: exhaustion,
        digest: carrier_digest,
    };
    let q3_digest = Digest::of_bytes(b"synthetic empty cutoff registry");
    let good_q3 = Q3RegistryView {
        is_empty: true,
        cutoff_boundary_digest: boundary.digest(),
        digest: &q3_digest,
    };
    let good = matches!(
        verify_bounded_quotient(semantic, boundary, good_q3, &carrier, response, 0),
        GscOutcome::Proven(ref quotient)
            if quotient.is_complete()
                && quotient.classes_len() == 1
                && quotient.q2_distinct_pair_decisions() == 0
                && quotient.q3_edge_count() == 0
    );
    let nonempty_q3 = matches!(
        verify_bounded_quotient(semantic, boundary, good_q3, &carrier, response, 1),
        GscOutcome::Unknown(_)
    );
    let wrong_cutoff_digest = Digest::of_bytes(b"wrong cutoff");
    let wrong_cutoff = Q3RegistryView {
        is_empty: true,
        cutoff_boundary_digest: &wrong_cutoff_digest,
        digest: &q3_digest,
    };
    let cutoff_rejected = matches!(
        verify_bounded_quotient(semantic, boundary, wrong_cutoff, &carrier, response, 0),
        GscOutcome::Unknown(_)
    );
    let nonempty_view = Q3RegistryView {
        is_empty: false,
        cutoff_boundary_digest: boundary.digest(),
        digest: &q3_digest,
    };
    let forged_q3_rejected = matches!(
        verify_bounded_quotient(semantic, boundary, nonempty_view, &carrier, response, 0),
        GscOutcome::Unknown(_)
    );
    let mut unbound = carrier.clone();
    unbound.response_digest = Digest::of_bytes(b"unbound response");
    let unbound_rejected = matches!(
        verify_bounded_quotient(semantic, boundary, good_q3, &unbound, response, 0),
        GscOutcome::Unknown(_)
    );
    let mut multiple = carrier.clone();
    multiple.candidate_count = 2;
    let multiple_rejected = matches!(
        verify_bounded_quotient(semantic, boundary, good_q3, &multiple, response, 0),
        GscOutcome::Unknown(_)
    );
    good && nonempty_q3
        && cutoff_rejected
        && forged_q3_rejected
        && unbound_rejected
        && multiple_rejected
}
