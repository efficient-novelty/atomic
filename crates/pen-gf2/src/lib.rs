//! Versioned, bounded contracts for a finite proof fragment.
//!
//! The manifest describes the complete finite language that a caller intends
//! to use. A backend may implement only a subset of that language. Unsupported
//! syntax and resource exhaustion are explicit decisions; neither is negative
//! evidence. Deserializable decisions are claims only, while successful replay
//! returns opaque, non-deserializable handles.

#![forbid(unsafe_code)]

mod adapter;
mod continuation;
mod contract;
mod operational;
mod quotient;
mod rewrite;
mod verifier_manifest;

#[cfg(test)]
mod tests;

use pen_demand::gsc::{
    GscOutcome, GscUnknownReason, VerifiedGscSemanticManifest, VerifiedGscVerifierManifest,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel};
use pen_law::{VerifiedOriginCutoffQ3Registry, VerifiedRegisteredActiveDemandInventory};

pub use adapter::{
    FragmentValidationError, LawDecision, LawKernel, NativeKernelAdapter, NativeVerifiedProof,
    NativeVerifiedRefutation, OutsideFragment, OutsideReason, ResourceExhausted,
    VerifiedFiniteFragment,
};
pub use continuation::{
    ContinuationExpirationRule, ContinuationExtractionRule, ContinuationHaltRule,
    ContinuationIntegrationRule, ContinuationLibraryRule,
    LAW_V2_CONTINUATION_SEMANTIC_MANIFEST_SCHEMA_VERSION, LawV2ContinuationSemanticManifestV1,
    VerifiedH4CensusV1, VerifiedH4DemandDecisionV1, VerifiedH4HaltCertificateV1,
    VerifiedLawV2ContinuationSemanticManifestV1, VerifiedOneNullaryFreeSealV1,
    continue_one_nullary_inductive_completion_v1, frozen_law_v2_continuation_semantic_manifest_v1,
    verify_law_v2_continuation_semantic_manifest_v1,
};
pub use contract::{
    CANONICAL_ORDER_VERSION, CanonicalOrder, DECISION_CLAIM_SCHEMA_VERSION,
    FINITE_FRAGMENT_SCHEMA_VERSION, FiniteFeature, FiniteFeatureDeclaration, FiniteFragmentLimits,
    FiniteResource, LAW_REQUEST_SCHEMA_VERSION, UncheckedDecisionClaim,
    UncheckedFiniteFragmentManifest, UncheckedLawRequest, UncheckedLawSubject,
};
pub use operational::{
    CertifiedInapplicabilityReason, ORDERED_OPERATIONAL_RULES, OperationalRule,
    VerifiedCanonicalFormAnalysis, VerifiedDemandConnectedResponse, VerifiedDerivationDag,
    VerifiedDerivationNode, VerifiedDerivedUse, VerifiedGroundRuleDisposition,
    VerifiedInapplicabilityEvidence, VerifiedPortDischarge, VerifiedUnderivedUse,
    operational_grammar_digest,
};
pub use quotient::{VerifiedCandidateCarrier, VerifiedFiniteQuotient, VerifiedQ0ResponseClass};
pub use rewrite::{
    VerifiedEquationExtensionSet, VerifiedFreshEliminatorBeta, VerifiedGeneratedPortSubstitution,
    VerifiedRewriteInvariants,
};
pub use verifier_manifest::{
    GF2_SLICE_VERIFIER_MANIFEST_SCHEMA_VERSION, Gf2SliceVerifierManifestV1,
    current_gf2_slice_verifier_manifest_v1, gf2_inductive_completion_verifier_source_digest,
};

/// Authoritative result of the registered one-nullary inductive-completion
/// vertical slice.
#[derive(Clone, Debug)]
pub struct VerifiedOneNullaryInductiveSlice {
    verifier_manifest: Gf2SliceVerifierManifestV1,
    registered_active_demand_inventory_binding_digest: Digest,
    registered_frame_binding_digest: Digest,
    pre_response: VerifiedUnderivedUse,
    equations: VerifiedEquationExtensionSet,
    response: VerifiedDemandConnectedResponse,
    candidate_carrier: VerifiedCandidateCarrier,
    quotient: VerifiedFiniteQuotient,
    digest: Digest,
}

impl VerifiedOneNullaryInductiveSlice {
    pub fn verifier_manifest(&self) -> &Gf2SliceVerifierManifestV1 {
        &self.verifier_manifest
    }

    pub fn registered_frame_binding_digest(&self) -> &Digest {
        &self.registered_frame_binding_digest
    }

    pub fn registered_active_demand_inventory_binding_digest(&self) -> &Digest {
        &self.registered_active_demand_inventory_binding_digest
    }

    pub fn pre_response(&self) -> &VerifiedUnderivedUse {
        &self.pre_response
    }

    pub fn equations(&self) -> &VerifiedEquationExtensionSet {
        &self.equations
    }

    pub fn response(&self) -> &VerifiedDemandConnectedResponse {
        &self.response
    }

    pub fn candidate_carrier(&self) -> &VerifiedCandidateCarrier {
        &self.candidate_carrier
    }

    pub fn quotient(&self) -> &VerifiedFiniteQuotient {
        &self.quotient
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Execute the frozen slice only from history-bound Law-V2A capabilities.
///
/// Generic checked frames and unchecked equation terms are intentionally not
/// accepted by this public entry point.
pub fn run_registered_one_nullary_inductive_slice(
    kernel: &Kernel,
    semantic: &VerifiedGscSemanticManifest,
    demand_verifier: &VerifiedGscVerifierManifest,
    inventory: &VerifiedRegisteredActiveDemandInventory,
    q3_registry: &VerifiedOriginCutoffQ3Registry,
) -> GscOutcome<VerifiedOneNullaryInductiveSlice> {
    if !inventory.binding_is_valid()
        || !inventory.cutoff_coverage_is_exhaustive()
        || inventory.semantic_manifest_digest() != semantic.digest()
        || inventory.q3_registry_digest() != q3_registry.digest()
        || inventory.cutoff_boundary_digest() != q3_registry.cutoff_boundary_digest()
        || inventory.tuples().len() != 1
        || inventory.compute_family_count() != 1
        || inventory.use_family_count() != 1
        || inventory.frame_count() != 1
    {
        return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
    }
    let tuple = &inventory.tuples()[0];
    let Some(compute_tuple) = tuple.compute_families().first() else {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    };
    let frame = tuple.frame();
    let use_family = tuple.use_family();
    let compute_family = compute_tuple.family();
    if !tuple.binding_is_valid()
        || !compute_tuple.binding_is_valid_for(frame, use_family, tuple.exact_use_port())
        || !frame.binding_is_valid()
        || frame.semantic_manifest_digest() != semantic.digest()
        || frame.q3_registry_digest() != q3_registry.digest()
        || frame.cutoff_boundary_digest() != q3_registry.cutoff_boundary_digest()
        || !q3_registry.is_empty()
        || !use_family.binding_is_valid_for(frame)
        || !compute_family.binding_is_valid_for(frame)
        || !use_family.is_use()
        || !compute_family.is_compute()
        || use_family.q3_registry_digest() != q3_registry.digest()
        || compute_family.q3_registry_digest() != q3_registry.digest()
        || compute_family.source_use_family_binding_digest() != Some(use_family.binding_digest())
    {
        return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
    }
    let boundary = frame.cutoff_boundary();

    let equations = match rewrite::verify_one_nullary_equation_extension(
        kernel,
        boundary,
        semantic,
        frame.inner(),
        use_family.inner(),
        compute_family.inner(),
    ) {
        GscOutcome::Proven(equations) => equations,
        GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
    };
    let derived = match operational::decide_one_nullary_use(
        kernel,
        equations.extended_signature(),
        semantic,
        use_family.inner(),
    ) {
        GscOutcome::Proven(operational::OneNullaryUseDecision::Derived(derived)) => derived,
        GscOutcome::Proven(operational::OneNullaryUseDecision::Underived(_)) => {
            return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
        }
        GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
    };
    let response = match operational::verify_demand_connected_response(
        use_family.inner(),
        compute_family.inner(),
        &equations,
        derived,
    ) {
        GscOutcome::Proven(response) => response,
        GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
    };
    let candidate_carrier =
        match quotient::verify_candidate_carrier(semantic, inventory, &equations, &response) {
            GscOutcome::Proven(carrier) => carrier,
            GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
        };
    let quotient = match quotient::verify_singleton_quotient(
        semantic,
        boundary,
        q3_registry,
        &candidate_carrier,
        &response,
    ) {
        GscOutcome::Proven(quotient) => quotient,
        GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
    };
    // Demand-family and response-candidate generation has no negative
    // premise.  Only after the exhaustive carrier has been constructed do we
    // decide whether the original Use port was live in the pre-response
    // library.
    let pre_response =
        match operational::decide_one_nullary_use(kernel, boundary, semantic, use_family.inner()) {
            GscOutcome::Proven(operational::OneNullaryUseDecision::Underived(certificate)) => {
                certificate
            }
            GscOutcome::Proven(operational::OneNullaryUseDecision::Derived(_)) => {
                return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
            }
            GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
        };
    if !pre_response.is_complete_relative_to_grammar() {
        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
    }
    let verifier_manifest =
        match current_gf2_slice_verifier_manifest_v1(semantic, kernel, demand_verifier) {
            GscOutcome::Proven(manifest) => manifest,
            GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
        };
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        semantic.digest().encode_canonical(&mut encoder);
        verifier_manifest
            .canonical_digest()
            .encode_canonical(&mut encoder);
        inventory.binding_digest().encode_canonical(&mut encoder);
        frame.binding_digest().encode_canonical(&mut encoder);
        use_family.binding_digest().encode_canonical(&mut encoder);
        compute_family
            .binding_digest()
            .encode_canonical(&mut encoder);
        q3_registry.digest().encode_canonical(&mut encoder);
        pre_response.digest().encode_canonical(&mut encoder);
        equations.digest().encode_canonical(&mut encoder);
        response.digest().encode_canonical(&mut encoder);
        candidate_carrier.digest().encode_canonical(&mut encoder);
        quotient.digest().encode_canonical(&mut encoder);
        Digest::of_domain_bytes(
            "pen-gf2/registered-one-nullary-inductive-slice/v1",
            encoder.as_bytes(),
        )
    };
    GscOutcome::Proven(VerifiedOneNullaryInductiveSlice {
        verifier_manifest,
        registered_active_demand_inventory_binding_digest: inventory.binding_digest().clone(),
        registered_frame_binding_digest: frame.binding_digest().clone(),
        pre_response,
        equations,
        response,
        candidate_carrier,
        quotient,
        digest,
    })
}
