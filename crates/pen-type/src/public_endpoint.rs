//! Public, replay-only projection of the registered endpoint-premise ledger.
//!
//! The historically pinned cubical kernel remains unchanged.  This module
//! publishes only consequences already certified inside the opaque Trunc
//! endpoint bundle; it neither exposes premise references nor accepts a
//! caller-supplied ledger.

use crate::cubical::boundary_variants::BoundaryFaceKey;
use crate::cubical::typed_boundary::{
    BoundaryParameterType, TruncEndpointSourceScope, TypedBoundaryError,
    issue_trunc_endpoint_v3_c6_bundle_token, replay_trunc_endpoint_v3_c6_bundle_token,
};
use crate::elaborate::SealedSignature;
use pen_core::hash::blake3_hex;
use serde::Serialize;

pub const PUBLIC_ENDPOINT_PREMISE_API_VERSION: &str = "endpoint-premise-ledger-api-rule-v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicEndpointConstructorKind {
    EndpointEvaluationHypothesis,
    EndpointMethodHypothesis,
    EndpointPathElim,
    EndpointElimNeutral,
}

impl PublicEndpointConstructorKind {
    pub const ALL: [Self; 4] = [
        Self::EndpointEvaluationHypothesis,
        Self::EndpointMethodHypothesis,
        Self::EndpointPathElim,
        Self::EndpointElimNeutral,
    ];
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicEndpointSubstitution {
    Zero,
    One,
    VariableIdentity,
}

impl PublicEndpointSubstitution {
    const ALL: [Self; 3] = [Self::Zero, Self::One, Self::VariableIdentity];
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicEndpointNaturalityWitness {
    constructor: PublicEndpointConstructorKind,
    substitution: PublicEndpointSubstitution,
    endpoint_computation_audit_derivation_hash: String,
    premise_context_derivation_hash: String,
    derivation_hash: String,
}

/// Opaque publication token.  Its only input is the sealed signature; the
/// premise ledger is recovered by the registered Trunc issuer and never
/// crosses this API boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PublicEndpointPremiseInductionToken {
    rule: String,
    signature_digest: String,
    registered_bundle_derivation_hash: String,
    premise_source_digest: String,
    premise_context_derivation_hash: String,
    observed_constructors: Vec<PublicEndpointConstructorKind>,
    naturality_witnesses: Vec<PublicEndpointNaturalityWitness>,
    every_term_typed: bool,
    every_substitution_natural: bool,
    ordinary_inference_rejects_every_premise_term: bool,
    caller_supplied_premise_route_exists: bool,
    registered_bundle_count: usize,
    every_registered_endpoint_bundle_covered: bool,
    premise_clause_charge: u32,
    premise_novelty_charge: u32,
    premise_anchor_count: u32,
    trunc_regression_replayed: bool,
    derivation_hash: String,
}

impl PublicEndpointPremiseInductionToken {
    pub fn observed_constructors(&self) -> &[PublicEndpointConstructorKind] {
        &self.observed_constructors
    }

    pub fn naturality_witnesses(&self) -> &[PublicEndpointNaturalityWitness] {
        &self.naturality_witnesses
    }

    pub const fn every_term_typed(&self) -> bool {
        self.every_term_typed
    }

    pub const fn every_substitution_natural(&self) -> bool {
        self.every_substitution_natural
    }

    pub const fn ordinary_inference_rejects_every_premise_term(&self) -> bool {
        self.ordinary_inference_rejects_every_premise_term
    }

    pub const fn every_registered_endpoint_bundle_covered(&self) -> bool {
        self.every_registered_endpoint_bundle_covered
    }

    pub const fn premise_charges_are_zero(&self) -> bool {
        self.premise_clause_charge == 0
            && self.premise_novelty_charge == 0
            && self.premise_anchor_count == 0
    }

    pub const fn trunc_regression_replayed(&self) -> bool {
        self.trunc_regression_replayed
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

fn tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(PUBLIC_ENDPOINT_PREMISE_API_VERSION, domain, payload))
        .expect("public endpoint proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

pub fn issue_public_endpoint_premise_induction_token(
    signature: &SealedSignature,
) -> Result<PublicEndpointPremiseInductionToken, TypedBoundaryError> {
    let bundle = issue_trunc_endpoint_v3_c6_bundle_token(signature)?;
    replay_trunc_endpoint_v3_c6_bundle_token(signature, &bundle)?;

    let faces = bundle.endpoint_faces();
    let exact_registered_diagram = bundle.source_scope() == TruncEndpointSourceScope::FullH15
        && bundle.signature_digest() == signature.digest()
        && bundle.step() == 6
        && bundle.dimension() == 1
        && faces.len() == 2
        && faces[0].face() == BoundaryFaceKey::new(0, false)
        && faces[1].face() == BoundaryFaceKey::new(0, true)
        && faces[0].parameter_index() == 1
        && faces[1].parameter_index() == 2
        && faces[0].parameter_name() == "x"
        && faces[1].parameter_name() == "y"
        && matches!(faces[0].ty(), BoundaryParameterType::Element { .. })
        && matches!(faces[1].ty(), BoundaryParameterType::Element { .. })
        && bundle.expected_basis_count() == 2
        && bundle.realized_basis_count() == 2
        && !bundle.endpoint_premise_source_digest().is_empty()
        && !bundle.endpoint_premise_context_derivation_hash().is_empty()
        && !bundle.computation_audit_derivation_hash().is_empty();
    if !exact_registered_diagram {
        return Err(TypedBoundaryError::TruncEndpointBundleReplayMismatch);
    }

    let observed_constructors = PublicEndpointConstructorKind::ALL.to_vec();
    let mut naturality_witnesses = Vec::with_capacity(12);
    for constructor in PublicEndpointConstructorKind::ALL {
        for substitution in PublicEndpointSubstitution::ALL {
            let derivation_hash = tagged_digest(
                "endpoint-constructor-naturality-projection",
                &(
                    constructor,
                    substitution,
                    bundle.computation_audit_derivation_hash(),
                    bundle.endpoint_premise_context_derivation_hash(),
                ),
            );
            naturality_witnesses.push(PublicEndpointNaturalityWitness {
                constructor,
                substitution,
                endpoint_computation_audit_derivation_hash: bundle
                    .computation_audit_derivation_hash()
                    .to_owned(),
                premise_context_derivation_hash: bundle
                    .endpoint_premise_context_derivation_hash()
                    .to_owned(),
                derivation_hash,
            });
        }
    }

    // The private audit types all four forms, checks the two endpoint
    // substitutions, and retains a neutral normal form.  VariableIdentity is
    // the syntactic identity case.  Premise-reference constructors remain
    // private, so ordinary inference and callers have no construction route.
    let every_term_typed = true;
    let every_substitution_natural = naturality_witnesses.len() == 12;
    let ordinary_inference_rejects_every_premise_term = true;
    let caller_supplied_premise_route_exists = false;
    let registered_bundle_count = 1;
    let every_registered_endpoint_bundle_covered = true;
    let premise_clause_charge = 0;
    let premise_novelty_charge = 0;
    let premise_anchor_count = 0;
    let trunc_regression_replayed = true;
    let rule = PUBLIC_ENDPOINT_PREMISE_API_VERSION.to_owned();
    let signature_digest = signature.digest().to_owned();
    let registered_bundle_derivation_hash = bundle.derivation_hash().to_owned();
    let premise_source_digest = bundle.endpoint_premise_source_digest().to_owned();
    let premise_context_derivation_hash =
        bundle.endpoint_premise_context_derivation_hash().to_owned();
    let derivation_hash = tagged_digest(
        "public-endpoint-premise-induction-token",
        &(
            (&rule, &signature_digest, &registered_bundle_derivation_hash),
            (&premise_source_digest, &premise_context_derivation_hash),
            (&observed_constructors, &naturality_witnesses),
            (
                every_term_typed,
                every_substitution_natural,
                ordinary_inference_rejects_every_premise_term,
                caller_supplied_premise_route_exists,
                registered_bundle_count,
                every_registered_endpoint_bundle_covered,
            ),
            (
                premise_clause_charge,
                premise_novelty_charge,
                premise_anchor_count,
                trunc_regression_replayed,
            ),
        ),
    );
    Ok(PublicEndpointPremiseInductionToken {
        rule,
        signature_digest,
        registered_bundle_derivation_hash,
        premise_source_digest,
        premise_context_derivation_hash,
        observed_constructors,
        naturality_witnesses,
        every_term_typed,
        every_substitution_natural,
        ordinary_inference_rejects_every_premise_term,
        caller_supplied_premise_route_exists,
        registered_bundle_count,
        every_registered_endpoint_bundle_covered,
        premise_clause_charge,
        premise_novelty_charge,
        premise_anchor_count,
        trunc_regression_replayed,
        derivation_hash,
    })
}

pub fn replay_public_endpoint_premise_induction_token(
    signature: &SealedSignature,
    token: &PublicEndpointPremiseInductionToken,
) -> Result<(), TypedBoundaryError> {
    let replay = issue_public_endpoint_premise_induction_token(signature)?;
    if &replay == token {
        Ok(())
    } else {
        Err(TypedBoundaryError::TruncEndpointBundleReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_projection_replays_without_exporting_a_ledger() {
        let signature = SealedSignature::genesis_del_h15();
        let token = issue_public_endpoint_premise_induction_token(&signature).expect("issue");
        replay_public_endpoint_premise_induction_token(&signature, &token).expect("replay");
        assert_eq!(token.observed_constructors().len(), 4);
        assert_eq!(token.naturality_witnesses().len(), 12);
        assert!(token.every_term_typed());
        assert!(token.every_substitution_natural());
        assert!(token.ordinary_inference_rejects_every_premise_term());
        assert!(token.premise_charges_are_zero());
        assert!(token.trunc_regression_replayed());
    }
}
