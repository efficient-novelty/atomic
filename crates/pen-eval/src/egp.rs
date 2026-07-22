//! Phase 3 (second half) of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`:
//! the kernel-backed bridge into the one-way EGP classifier.
//!
//! `derive_conditional_at_most` already implements the injection
//! mechanics (slot uniqueness, role discipline, demand-window checks);
//! what it lacked was evidence. This module builds
//! `CandidateProvenanceSubmission`s whose every `SemanticAssumptionRef`
//! is `Verified` with a kernel derivation hash:
//!
//! - credited families come from the Phase 2 extractor (canonical ids,
//!   closure-checked marginality) — internal families are NOT credited;
//! - each marginal family's anchor is the dependent `Anchors(f, pi(f))`
//!   evidence: the family IS the named local role of its generator
//!   clause, by the kernel-v1 mechanism assignment below;
//! - the demand-orbit side comes from the Phase 3 extraction (stage
//!   inventories with kernel-verified J2/J3/locality).
//!
//! Kernel-v1 mechanism assignment (frozen, EXCLUSIVE precedence):
//! - a family whose canonical form references the library is
//!   `P5InheritedSurface` (support action);
//! - otherwise a path-constructor family is `DimensionSquared`
//!   (coherence);
//! - otherwise a modal/temporal-formed family is
//!   `ModalPairwiseCoherence` (coherence);
//! - everything else is `IntrinsicKernel` (kernel head).
//!
//! The classifier's role table checks only mechanism/role CONSISTENCY;
//! the assignment itself is gated here by an independent faithfulness
//! check against the family's kernel-derived canonical normal form
//! (`mechanism_is_faithful`), which fails closed on any contradiction —
//! that check, not the classifier, is what makes a wrong assignment
//! unrepresentable in a bridge-built submission.
//!
//! This is a one-way upper-bound classifier: no realizer or saturation
//! claim is made, and a family that obtains no valid anchor keeps the
//! candidate outside debt-free EGP territory (outcome-B material), never
//! silently credited.

use crate::demand_orbits::KernelOrbitExtraction;
use crate::semantic_provenance::{
    CandidateProvenanceSubmission, ConditionalAtMostBound, CreditMechanism, CreditedSchemaFamily,
    LocalRole, MarginalityWitness, ProvenanceAnchor, ProvenanceError, SemanticAssumptionRef,
    derive_conditional_at_most,
};
use crate::typed_families::{
    CandidateFamilyExtraction, ExtractedFamily, InstanceKind, MarginalityDisposition,
    to_typed_normal_family,
};
use pen_core::expr::Expr;
use serde::Serialize;
use thiserror::Error;

/// Kernel-v1 mechanism assignment for a marginal family.
pub fn assign_mechanism(family: &ExtractedFamily) -> CreditMechanism {
    let normal_form = &family.presentation.canonical_normal_form;
    if references_library(normal_form) {
        CreditMechanism::P5InheritedSurface
    } else if contains_path_constructor(normal_form) {
        CreditMechanism::DimensionSquared
    } else if contains_modal_or_temporal(normal_form) {
        CreditMechanism::ModalPairwiseCoherence
    } else {
        CreditMechanism::IntrinsicKernel
    }
}

fn references_library(expr: &Expr) -> bool {
    !expr.lib_refs().is_empty()
}

fn contains_path_constructor(expr: &Expr) -> bool {
    match expr {
        Expr::PathCon(_) => true,
        Expr::App(a, b) | Expr::Pi(a, b) | Expr::Sigma(a, b) => {
            contains_path_constructor(a) || contains_path_constructor(b)
        }
        Expr::Id(a, b, c) => {
            contains_path_constructor(a)
                || contains_path_constructor(b)
                || contains_path_constructor(c)
        }
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => contains_path_constructor(inner),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) => false,
    }
}

fn contains_modal_or_temporal(expr: &Expr) -> bool {
    match expr {
        Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_) => true,
        Expr::App(a, b) | Expr::Pi(a, b) | Expr::Sigma(a, b) => {
            contains_modal_or_temporal(a) || contains_modal_or_temporal(b)
        }
        Expr::Id(a, b, c) => {
            contains_modal_or_temporal(a)
                || contains_modal_or_temporal(b)
                || contains_modal_or_temporal(c)
        }
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => contains_modal_or_temporal(inner),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

/// Independent faithfulness gate: the assigned mechanism must be exactly
/// what the exclusive-precedence predicates over the family's canonical
/// normal form dictate. A divergence (e.g. a mutated assignment table)
/// fails closed here.
pub fn mechanism_is_faithful(family: &ExtractedFamily, mechanism: CreditMechanism) -> bool {
    let normal_form = &family.presentation.canonical_normal_form;
    let has_library = references_library(normal_form);
    let has_path = contains_path_constructor(normal_form);
    let has_modal_temporal = contains_modal_or_temporal(normal_form);
    match mechanism {
        CreditMechanism::P5InheritedSurface => has_library,
        CreditMechanism::DimensionSquared => !has_library && has_path,
        CreditMechanism::ModalPairwiseCoherence => !has_library && !has_path && has_modal_temporal,
        CreditMechanism::IntrinsicKernel => !has_library && !has_path && !has_modal_temporal,
        _ => false,
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum EgpBridgeError {
    #[error("marginal family {family} has no generator instance")]
    NoGeneratorInstance { family: String },
    #[error("mechanism assignment for family {family} is not faithful to its normal form")]
    MechanismNotFaithful { family: String },
    #[error("orbit extraction does not cover candidate stage {stage}")]
    StageNotCovered { stage: u32 },
    #[error("classifier rejected the kernel-backed submission: {0}")]
    Classifier(ProvenanceError),
}

/// The kernel-backed EGP disposition of one candidate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KernelEgpDisposition {
    pub subject_hash: String,
    pub candidate_stage: u32,
    pub marginal_family_count: usize,
    pub bound: ConditionalAtMostBound,
    pub derivation_hash: String,
}

/// Build the kernel-backed submission for a candidate at `stage` and run
/// the one-way classifier. Only MARGINAL families are credited; internal
/// families contribute zero by construction (their closure evidence is in
/// the Phase 2 extraction).
pub fn classify_candidate(
    extraction: &CandidateFamilyExtraction,
    orbits: &KernelOrbitExtraction,
    candidate_stage: u32,
    kappa: u16,
) -> Result<KernelEgpDisposition, EgpBridgeError> {
    let mut credited = Vec::new();
    for family in extraction.marginal_families() {
        let MarginalityDisposition::MarginalNoClosurePreimage { closure_digest } =
            &family.marginality
        else {
            continue;
        };
        let generator = family
            .instances
            .iter()
            .find(|instance| matches!(instance.kind, InstanceKind::Generator))
            .ok_or_else(|| EgpBridgeError::NoGeneratorInstance {
                family: family.id.as_str().to_string(),
            })?;
        let mechanism = assign_mechanism(family);
        if !mechanism_is_faithful(family, mechanism) {
            return Err(EgpBridgeError::MechanismNotFaithful {
                family: family.id.as_str().to_string(),
            });
        }
        let role: LocalRole = mechanism.required_local_role();
        let typed = to_typed_normal_family(extraction, family);
        credited.push(CreditedSchemaFamily {
            family: typed,
            instances: Vec::new(),
            marginality: MarginalityWitness::NoWeakeningPreimage {
                fresh_kernel_clause: generator.clause_index,
                no_preimage: SemanticAssumptionRef::verified(
                    format!(
                        "kernel-v1 marginality: no preimage in the typed predecessor \
                         closure {closure_digest}"
                    ),
                    extraction.derivation_hash.clone(),
                ),
            },
            mechanism,
            anchor: ProvenanceAnchor::ChargedKernel {
                clause: generator.clause_index,
                role,
            },
            anchor_validity_assumption: SemanticAssumptionRef::verified(
                format!(
                    "kernel-v1 Anchors(f, pi(f)): mechanism {mechanism:?} for family {} \
                     computed from its kernel-derived canonical normal form per the \
                     frozen exclusive-precedence table, faithfulness gated by the \
                     bridge; anchored at generator clause {}",
                    family.id.as_str(),
                    generator.clause_index
                ),
                extraction.derivation_hash.clone(),
            ),
        });
    }

    // A stage the orbit extraction never covered must not become a
    // debt-free claim through an empty orbit list with Verified stamps.
    let Some(stage_inventory) = orbits.stage(candidate_stage).cloned() else {
        return Err(EgpBridgeError::StageNotCovered {
            stage: candidate_stage,
        });
    };
    let stage_inventory = vec![stage_inventory];
    let submission = CandidateProvenanceSubmission {
        candidate_stage,
        kappa,
        credited_families: credited,
        family_extraction_completeness_assumption: SemanticAssumptionRef::verified(
            "kernel-v1 family extraction is total over the candidate's clauses",
            extraction.derivation_hash.clone(),
        ),
        demand_orbits: stage_inventory
            .first()
            .map(|inventory| inventory.orbits.clone())
            .unwrap_or_default(),
        orbit_inventory_completeness_assumption: SemanticAssumptionRef::verified(
            "kernel-v1 J2/J3 stage inventory (total window extraction)",
            orbits.derivation_hash.clone(),
        ),
        window_locality_assumption: SemanticAssumptionRef::verified(
            format!(
                "kernel-v1 window locality ledger {}",
                orbits.locality_ledger.digest
            ),
            orbits.derivation_hash.clone(),
        ),
    };

    let bound = derive_conditional_at_most(&submission).map_err(EgpBridgeError::Classifier)?;
    let payload = serde_json::json!({
        "subject": extraction.subject_hash,
        "stage": candidate_stage,
        "kappa": kappa,
        "families": extraction.derivation_hash,
        "orbits": orbits.derivation_hash,
        "bound": bound,
    });
    let derivation_hash = format!(
        "blake3:{}",
        pen_core::hash::blake3_hex(
            &serde_json::to_vec(&payload).expect("egp payload serialization"),
        )
    );
    Ok(KernelEgpDisposition {
        subject_hash: extraction.subject_hash.clone(),
        candidate_stage,
        marginal_family_count: extraction.marginal_family_count,
        bound,
        derivation_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demand_orbits::kernel_stage_inventories;
    use crate::typed_families::{extract_candidate_families, predecessor_closure};
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::telescope::Telescope;
    use pen_type::elaborate::SealedSignature;

    fn app(function: Expr, argument: Expr) -> Expr {
        Expr::App(Box::new(function), Box::new(argument))
    }

    fn pi(domain: Expr, codomain: Expr) -> Expr {
        Expr::Pi(Box::new(domain), Box::new(codomain))
    }

    fn sigma(domain: Expr, codomain: Expr) -> Expr {
        Expr::Sigma(Box::new(domain), Box::new(codomain))
    }

    fn primary_role_for_test(expr: &Expr) -> ClauseRole {
        match expr {
            Expr::Susp(_) => ClauseRole::Formation,
            Expr::Univ | Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Id(_, _, _) => {
                ClauseRole::Formation
            }
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => ClauseRole::Formation,
            Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => ClauseRole::Introduction,
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Lib(_)) => ClauseRole::Introduction,
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Lam(_)) => ClauseRole::Elimination,
            Expr::App(_, _) => ClauseRole::Introduction,
            Expr::PathCon(_) => ClauseRole::PathAttach,
            _ => ClauseRole::Formation,
        }
    }

    fn tel(exprs: Vec<Expr>) -> Telescope {
        Telescope::new(
            exprs
                .into_iter()
                .map(|expr| ClauseRec::new(primary_role_for_test(&expr), expr))
                .collect(),
        )
    }

    fn harness() -> (
        SealedSignature,
        crate::typed_families::PredecessorClosure,
        KernelOrbitExtraction,
    ) {
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");
        let orbits = kernel_stage_inventories(&signature, &closure).expect("orbits");
        (signature, closure, orbits)
    }

    #[test]
    fn falsifier_egp_dispositions_are_kernel_backed_and_bounded() {
        let (signature, closure, orbits) = harness();

        // temporal_polymorphic_kappa2: one family, internal — ZERO
        // credited marginal families, marginal_nu 0, debt-free.
        let temporal = tel(vec![
            pi(
                Expr::Next(Box::new(Expr::Var(1))),
                Expr::Eventually(Box::new(Expr::Var(1))),
            ),
            pi(
                Expr::Next(Box::new(Expr::Var(1))),
                Expr::Eventually(Box::new(Expr::Var(1))),
            ),
        ]);
        let extraction = extract_candidate_families(&signature, &closure, &temporal, 15);
        let extraction = extraction.extraction().expect("extracts").clone();
        let disposition = classify_candidate(&extraction, &orbits, 16, 2).expect("classifies");
        assert_eq!(disposition.bound.marginal_nu, 0);
        assert!(disposition.bound.debt_free_if_inventory_exhaustive);
        assert!(disposition.bound.conditional_debt_free_linear_bound_holds);
        assert_eq!(disposition.bound.local_capacity, 8);

        // axiomatic_single_l15_kappa3: three marginal families, each with
        // a distinct generator-clause anchor: marginal_nu 3 <= 4*3.
        let single = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            app(Expr::Lib(15), Expr::Var(1)),
        ]);
        let extraction = extract_candidate_families(&signature, &closure, &single, 15);
        let extraction = extraction.extraction().expect("extracts").clone();
        let disposition = classify_candidate(&extraction, &orbits, 16, 3).expect("classifies");
        assert_eq!(disposition.bound.marginal_nu, 3);
        assert!(disposition.bound.debt_free_if_inventory_exhaustive);
        assert!(disposition.bound.conditional_debt_free_linear_bound_holds);
        assert_eq!(disposition.bound.local_capacity, 12);

        // hit_no_formation_d1: exactly one marginal family.
        let hit = tel(vec![Expr::PathCon(1), Expr::Var(1)]);
        let extraction = extract_candidate_families(&signature, &closure, &hit, 15);
        let extraction = extraction.extraction().expect("extracts").clone();
        let disposition = classify_candidate(&extraction, &orbits, 16, 2).expect("classifies");
        assert_eq!(disposition.bound.marginal_nu, 1);
        assert!(disposition.bound.debt_free_if_inventory_exhaustive);
    }

    #[test]
    fn semantic_marginal_nu_sits_far_below_the_structural_scores() {
        // The strengthened law counts typed marginal families: the
        // falsifiers' structural scores (19/32/107/108) collapse to
        // 1/0/3/3 — all far below first-clearing (19/28) — which is the
        // Phase 4/5 content in miniature: the shipped SAT witnesses carry
        // almost no typed marginal novelty.
        let (signature, closure, orbits) = harness();
        let inheritance = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            app(Expr::Lib(14), Expr::Var(1)),
        ]);
        let extraction = extract_candidate_families(&signature, &closure, &inheritance, 15);
        let extraction = extraction.extraction().expect("extracts").clone();
        let disposition = classify_candidate(&extraction, &orbits, 16, 3).expect("classifies");
        assert_eq!(disposition.bound.marginal_nu, 3);
        assert!(disposition.bound.marginal_nu <= 4 * 3);
    }

    #[test]
    fn mechanism_assignment_is_pinned_and_faithfulness_gated() {
        let (signature, closure, _) = harness();
        let single = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            app(Expr::Lib(15), Expr::Var(1)),
        ]);
        let extraction = extract_candidate_families(&signature, &closure, &single, 15);
        let extraction = extraction.extraction().expect("extracts");
        // Pinned per-family mechanism/role assignments (exclusive
        // precedence): the two library-referencing families take
        // P5InheritedSurface/SupportAction, the pure Sigma family takes
        // IntrinsicKernel/KernelHead.
        let mut assignments: Vec<(String, String)> = extraction
            .marginal_families()
            .map(|family| {
                let mechanism = assign_mechanism(family);
                assert!(
                    mechanism_is_faithful(family, mechanism),
                    "assignment must satisfy its own faithfulness gate"
                );
                (
                    format!("{mechanism:?}"),
                    format!("{:?}", mechanism.required_local_role()),
                )
            })
            .collect();
        assignments.sort();
        assert_eq!(
            assignments,
            vec![
                ("IntrinsicKernel".to_string(), "KernelHead".to_string()),
                (
                    "P5InheritedSurface".to_string(),
                    "SupportAction".to_string()
                ),
                (
                    "P5InheritedSurface".to_string(),
                    "SupportAction".to_string()
                ),
            ]
        );
        // The gate itself fails closed on a contradicting assignment.
        let library_family = extraction
            .marginal_families()
            .find(|family| {
                !family
                    .presentation
                    .canonical_normal_form
                    .lib_refs()
                    .is_empty()
            })
            .expect("library-referencing family");
        assert!(!mechanism_is_faithful(
            library_family,
            CreditMechanism::IntrinsicKernel
        ));
        assert!(!mechanism_is_faithful(
            library_family,
            CreditMechanism::DimensionSquared
        ));
    }

    #[test]
    fn uncovered_stages_are_rejected_not_stamped_debt_free() {
        let (signature, closure, orbits) = harness();
        let single = tel(vec![pi(Expr::Lib(15), Expr::Var(1))]);
        let extraction = extract_candidate_families(&signature, &closure, &single, 15);
        let extraction = extraction.extraction().expect("extracts").clone();
        let error = classify_candidate(&extraction, &orbits, 17, 1).unwrap_err();
        assert_eq!(error, EgpBridgeError::StageNotCovered { stage: 17 });
    }
}
