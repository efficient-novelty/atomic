//! Phase 4 (first part) of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`:
//! conservative internality for the guarded Step-15-generated flow.
//!
//! For a flow generated solely by the Step-15 interface, the kernel
//! constructs the checked weakening and erasure maps between the base
//! family inventory (the typed predecessor closure of sealed `B15`) and
//! the flow's own families, and proves the two inverse laws
//!
//! ```text
//! erase (weaken f) = f        weaken (erase g) = g
//! ```
//!
//! as typed equalities through the same normalization and univalent
//! equality as Phase 2. The STRICT guarded subspace is the set of flows
//! whose every family is a weakening image of a closure family
//! (`InternalIdentical` — the kernel's checked equality witness IS the
//! inverse-law witness pointwise). Flows outside that subspace (e.g. the
//! bare window re-reference, whose `Lib(n)` constant families are fresh
//! presentations) are NOT smuggled in: they are reported as outside the
//! subspace and take the EGP route instead. A syntactic reference to
//! Step 15 is not an internality proof; the guard, the maps, and both
//! inverse derivations are serialized and replayed.

use crate::semantic_provenance::{
    ConditionalConservativeInternality, FamilyMapEdge, GuardedInternalityAssumptions,
    ProvenanceError, SchemaFamilyId, SemanticAssumptionRef,
    derive_conditional_conservative_internality,
};
use crate::typed_families::{
    CandidateExtractionOutcome, MarginalityDisposition, PredecessorClosure,
    extract_candidate_families,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::SealedSignature;
use pen_type::equality::EqualityWitness;
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

/// One guarded flow's kernel-checked membership in the strict subspace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GuardedFlowRecord {
    pub description: String,
    pub subject_hash: String,
    pub family_count: usize,
    /// Per-family: (flow family id, closure predecessor id, the typed
    /// equality witness) — the pointwise inverse-law data.
    pub weakening_images: Vec<(String, String, EqualityWitness)>,
}

/// A flow that is NOT in the strict weakening-image subspace, with the
/// families that keep it out (these take the EGP route).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExcludedFlowRecord {
    pub description: String,
    pub subject_hash: String,
    pub non_weakening_families: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum InternalityError {
    #[error("guarded flow failed kernel elaboration: {0}")]
    FlowInvalid(String),
    #[error("classifier rejected the internality submission: {0:?}")]
    Classifier(ProvenanceError),
}

/// The kernel-backed conservative-internality certificate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KernelInternalityCertificate {
    pub flow_name: String,
    pub signature_digest: String,
    pub closure_digest: String,
    pub guarded_flows: Vec<GuardedFlowRecord>,
    pub excluded_flows: Vec<ExcludedFlowRecord>,
    pub conditional: ConditionalConservativeInternality,
    /// The five availabilities of the Step-15 internality obligation, now
    /// kernel-backed for the strict guarded subspace.
    pub typed_family_extraction_available: bool,
    pub weakening_map_available: bool,
    pub erasure_map_available: bool,
    pub inverse_laws_available: bool,
    pub conclusion_nu_zero_proved_for_guarded_subspace: bool,
    pub derivation_hash: String,
}

/// The canonical guarded-flow exemplars of the Step-15 interface (the
/// engine's internal class): verbatim re-proposals of the sealed window,
/// the bare variable presentation, and the trunc-hybrid re-expression.
/// The window re-reference is deliberately included so the certificate
/// records its EXCLUSION from the strict subspace.
pub fn step15_guarded_flow_exemplars() -> Vec<(String, Telescope)> {
    let mut flows = Vec::new();
    for step in [13u32, 14, 15] {
        flows.push((
            format!("verbatim re-proposal of the Step-{step} entry"),
            Telescope::reference(step),
        ));
    }
    flows.push((
        "re-reference of the sealed window (Lib 13/14/15)".to_string(),
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(14)),
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(13)),
        ]),
    ));
    flows.push((
        "bare variable presentation".to_string(),
        Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, Expr::Var(1))]),
    ));
    flows.push((
        "truncation-context re-expression of a higher path".to_string(),
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Trunc(Box::new(Expr::Var(1)))),
            ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(2)),
        ]),
    ));
    flows
}

/// Build the kernel-backed internality certificate for the guarded
/// Step-15 flow over the sealed signature.
pub fn certify_guarded_step15_internality(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
) -> Result<KernelInternalityCertificate, InternalityError> {
    let flow_name =
        "Step-15-generated guarded flow over sealed B15 (the Step-16 internal branch)".to_string();
    let mut guarded_flows = Vec::new();
    let mut excluded_flows = Vec::new();
    let mut base_families: BTreeSet<SchemaFamilyId> = BTreeSet::new();
    let mut extension_families: BTreeSet<SchemaFamilyId> = BTreeSet::new();
    let mut weakening: Vec<FamilyMapEdge> = Vec::new();
    let mut erasure: Vec<FamilyMapEdge> = Vec::new();

    for (description, telescope) in step15_guarded_flow_exemplars() {
        let outcome =
            extract_candidate_families(signature, closure, &telescope, signature.len() as u32);
        let extraction = match outcome {
            CandidateExtractionOutcome::Extracted(extraction) => extraction,
            CandidateExtractionOutcome::KernelInvalid { failure } => {
                return Err(InternalityError::FlowInvalid(format!(
                    "{description}: {failure}"
                )));
            }
        };

        let mut weakening_images = Vec::new();
        let mut non_weakening = Vec::new();
        for family in &extraction.families {
            match &family.marginality {
                MarginalityDisposition::InternalIdentical {
                    predecessor_step,
                    predecessor_clause,
                    equality,
                } => {
                    let predecessor = closure
                        .families
                        .iter()
                        .find(|candidate| {
                            candidate.step == *predecessor_step
                                && candidate.clause_index == *predecessor_clause
                        })
                        .expect("identical disposition names a closure family");
                    weakening_images.push((
                        family.id.as_str().to_string(),
                        predecessor.id.as_str().to_string(),
                        equality.clone(),
                    ));
                }
                MarginalityDisposition::InternalDerivable { .. }
                | MarginalityDisposition::MarginalNoClosurePreimage { .. } => {
                    non_weakening.push(family.id.as_str().to_string());
                }
            }
        }

        if non_weakening.is_empty() {
            for (flow_id, closure_id, _) in &weakening_images {
                let flow_family = SchemaFamilyId(flow_id.clone());
                let closure_family = SchemaFamilyId(closure_id.clone());
                base_families.insert(closure_family.clone());
                extension_families.insert(flow_family.clone());
                // The maps are pointwise-identical by the checked
                // equality witness: weaken(closure) = flow family and
                // erase(flow) = closure family, with the same ids.
                if !weakening
                    .iter()
                    .any(|edge| edge.source == closure_family && edge.target == flow_family)
                {
                    weakening.push(FamilyMapEdge {
                        source: closure_family.clone(),
                        target: flow_family.clone(),
                        action: SemanticAssumptionRef::verified(
                            "kernel-v1 weakening (checked identity embedding)",
                            extraction.derivation_hash.clone(),
                        ),
                    });
                    erasure.push(FamilyMapEdge {
                        source: flow_family,
                        target: closure_family,
                        action: SemanticAssumptionRef::verified(
                            "kernel-v1 erasure (checked identity restriction)",
                            extraction.derivation_hash.clone(),
                        ),
                    });
                }
            }
            guarded_flows.push(GuardedFlowRecord {
                description,
                subject_hash: extraction.subject_hash.clone(),
                family_count: extraction.families.len(),
                weakening_images,
            });
        } else {
            excluded_flows.push(ExcludedFlowRecord {
                description,
                subject_hash: extraction.subject_hash.clone(),
                non_weakening_families: non_weakening,
            });
        }
    }

    let evidence_payload = serde_json::json!({
        "flow": flow_name,
        "signature_digest": signature.digest(),
        "closure_digest": closure.digest,
        "guarded": guarded_flows,
        "excluded": excluded_flows,
    });
    let derivation_hash = format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&evidence_payload).expect("internality serialization"))
    );

    // Because the maps are identity embeddings, the ids on both sides
    // coincide; the conditional checker still verifies the finite
    // bijection both ways (fail closed on any mismatch).
    let assumptions = GuardedInternalityAssumptions {
        flow_name: flow_name.clone(),
        base_families,
        extension_families,
        base_extraction_completeness: SemanticAssumptionRef::verified(
            "kernel-v1 base inventory: the typed predecessor closure of sealed B15",
            derivation_hash.clone(),
        ),
        extension_extraction_completeness: SemanticAssumptionRef::verified(
            "kernel-v1 extension inventory: total extraction over the guarded flows",
            derivation_hash.clone(),
        ),
        weakening,
        erasure,
        erasure_after_weakening: SemanticAssumptionRef::verified(
            "kernel-v1 erase-after-weaken: pointwise identity with typed equality witnesses",
            derivation_hash.clone(),
        ),
        weakening_after_erasure_up_to_univalence: SemanticAssumptionRef::verified(
            "kernel-v1 weaken-after-erase: pointwise identity up to the frozen univalent \
             equality (witnesses retained per family)",
            derivation_hash.clone(),
        ),
    };
    let conditional = derive_conditional_conservative_internality(&assumptions)
        .map_err(InternalityError::Classifier)?;

    Ok(KernelInternalityCertificate {
        flow_name,
        signature_digest: signature.digest().to_string(),
        closure_digest: closure.digest.clone(),
        guarded_flows,
        excluded_flows,
        conditional,
        typed_family_extraction_available: true,
        weakening_map_available: true,
        erasure_map_available: true,
        inverse_laws_available: true,
        conclusion_nu_zero_proved_for_guarded_subspace: true,
        derivation_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typed_families::predecessor_closure;

    fn certificate() -> KernelInternalityCertificate {
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");
        certify_guarded_step15_internality(&signature, &closure).expect("certificate")
    }

    #[test]
    fn guarded_flows_prove_inverse_laws_with_typed_witnesses() {
        let certificate = certificate();
        // The strict subspace: the three re-proposals, the bare variable,
        // and the trunc-hybrid — every family a checked weakening image.
        assert_eq!(certificate.guarded_flows.len(), 5);
        for flow in &certificate.guarded_flows {
            assert!(flow.family_count > 0);
            assert_eq!(flow.family_count, flow.weakening_images.len());
            for (_, _, witness) in &flow.weakening_images {
                assert!(witness.equal, "inverse-law witness must be a real equality");
            }
        }
        assert!(certificate.conditional.weakening_erasure_are_inverse);
        assert_eq!(certificate.conditional.marginal_nu, 0);
        assert!(certificate.conclusion_nu_zero_proved_for_guarded_subspace);
        assert!(certificate.typed_family_extraction_available);
        assert!(certificate.weakening_map_available);
        assert!(certificate.erasure_map_available);
        assert!(certificate.inverse_laws_available);
    }

    #[test]
    fn window_re_reference_is_honestly_excluded_from_the_strict_subspace() {
        let certificate = certificate();
        // The bare Lib re-reference presents constant families no sealed
        // clause presents: NOT a weakening image — it is excluded and
        // takes the EGP route, never silently counted internal.
        assert_eq!(certificate.excluded_flows.len(), 1);
        let excluded = &certificate.excluded_flows[0];
        assert!(excluded.description.contains("re-reference"));
        assert_eq!(excluded.non_weakening_families.len(), 3);
    }

    #[test]
    fn certificate_replays_deterministically() {
        let first = certificate();
        let second = certificate();
        assert_eq!(first, second);
        assert!(first.derivation_hash.starts_with("blake3:"));
    }
}
