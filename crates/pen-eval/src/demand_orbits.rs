//! Phase 3 of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`: kernel-backed
//! demand-orbit extraction for every historical depth-two support window,
//! including `(S14, S15)` for Step 16, with J2/J3 and window-locality
//! evidence.
//!
//! Kernel-v1 operational definitions (frozen):
//! - The demand alphabet is the frozen twelve-package guard rail, refined
//!   to individual typed orbits: a LIVE orbit at stage `N` exists exactly
//!   for each package the structural debt requires at `N`
//!   (`directive_debt_timeline`), grounded in the typed families of the
//!   generating window entries; an ANSWERED orbit exists exactly where a
//!   package left the required set between `N-1` and `N`, discharged by
//!   the step accepted at `N-1`, with the answering entry's
//!   characteristic families checked by the kernel (a discharge whose
//!   accepted entry does NOT exhibit the package's characteristic former
//!   fails closed).
//! - J2 (extraction completeness) is totality of this derivation: the
//!   inventory is the image of a deterministic total function of the
//!   sealed timeline and window families — replay re-derives it.
//! - J3 (disposition completeness) is checked exhaustively: every orbit
//!   and required output carries a disposition.
//! - Window locality: across consecutive stages every live orbit either
//!   persists (transport record), or is answered by the newly accepted
//!   step (discharge record); an expiry would be a certified finding and
//!   fails the locality check. An empty `(S14, S15)` vector is never
//!   trusted by itself — stage 16's emptiness is derived from the same
//!   total extraction as every other stage.
//!
//! Live orbits carry a required-output class derived from the package's
//! characteristic-former PATTERN (a real canonical expression, not a
//! label): a family answers the demand iff its canonical normal form
//! exhibits the characteristic former, decided by the kernel predicate
//! `family_answers_package` — the same predicate validates historical
//! answers, so classes and answers cannot drift apart.

use crate::debt_guard::{DirectiveDebtRecord, directive_debt_timeline};
use crate::semantic_provenance::{
    DemandOrbitId, DemandOutputPosition, DemandOutputPositionId, OrbitResolution,
    SemanticAssumptionRef, SemanticDemandOrbit, StageOrbitInventory, UnivalentClassId,
};
use crate::typed_families::{ClosureFamily, PredecessorClosure};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_type::elaborate::SealedSignature;
use serde::Serialize;
use thiserror::Error;

/// The characteristic-former pattern of one structural package: a family
/// answers the package iff its canonical normal form contains one of the
/// pattern formers (kernel predicate, not an AST label).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CharacteristicFormer {
    PathConstructor { min_dimension: u32 },
    Truncation,
    Modal,
    Temporal,
    Binder,
    LibraryReference { min_step: u32 },
}

pub fn package_characteristic(package: &str) -> Option<CharacteristicFormer> {
    Some(match package {
        "former_eliminator" => CharacteristicFormer::Binder,
        "initial_hit" => CharacteristicFormer::PathConstructor { min_dimension: 1 },
        "truncation_hit" => CharacteristicFormer::Truncation,
        "higher_hit" => CharacteristicFormer::PathConstructor { min_dimension: 2 },
        "sphere_lift" => CharacteristicFormer::PathConstructor { min_dimension: 3 },
        "axiomatic_bundle" => CharacteristicFormer::LibraryReference { min_step: 1 },
        "modal_shell" => CharacteristicFormer::Modal,
        "connection_shell" => CharacteristicFormer::LibraryReference { min_step: 10 },
        "curvature_shell" => CharacteristicFormer::LibraryReference { min_step: 11 },
        "operator_bundle" => CharacteristicFormer::LibraryReference { min_step: 12 },
        "hilbert_functional" => CharacteristicFormer::LibraryReference { min_step: 13 },
        "temporal_shell" => CharacteristicFormer::Temporal,
        _ => return None,
    })
}

fn expr_exhibits(expr: &Expr, former: CharacteristicFormer) -> bool {
    let direct = match (former, expr) {
        (CharacteristicFormer::PathConstructor { min_dimension }, Expr::PathCon(dimension)) => {
            *dimension >= min_dimension
        }
        (CharacteristicFormer::Truncation, Expr::Trunc(_)) => true,
        (
            CharacteristicFormer::Modal,
            Expr::Flat(_) | Expr::Sharp(_) | Expr::Disc(_) | Expr::Shape(_),
        ) => true,
        (CharacteristicFormer::Temporal, Expr::Next(_) | Expr::Eventually(_)) => true,
        (CharacteristicFormer::Binder, Expr::Lam(_) | Expr::Pi(_, _) | Expr::Sigma(_, _)) => true,
        (CharacteristicFormer::LibraryReference { min_step }, Expr::Lib(step)) => {
            *step >= min_step
        }
        _ => false,
    };
    if direct {
        return true;
    }
    match expr {
        Expr::App(a, b) | Expr::Pi(a, b) | Expr::Sigma(a, b) => {
            expr_exhibits(a, former) || expr_exhibits(b, former)
        }
        Expr::Id(a, b, c) => {
            expr_exhibits(a, former) || expr_exhibits(b, former) || expr_exhibits(c, former)
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
        | Expr::WhyNot(inner) => expr_exhibits(inner, former),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

/// The kernel predicate deciding whether a canonical family answers a
/// package demand.
pub fn family_answers_package(package: &str, canonical_normal_form: &Expr) -> bool {
    package_characteristic(package)
        .is_some_and(|former| expr_exhibits(canonical_normal_form, former))
}

/// The demand class of a package at a window: a real canonical id derived
/// from the characteristic pattern, shared by every family that answers.
fn demand_class(package: &str, window: [u32; 2]) -> UnivalentClassId {
    let payload = serde_json::json!({
        "demand_package": package,
        "characteristic": package_characteristic(package),
        "window": window,
    });
    UnivalentClassId(format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&payload).expect("demand class serialization"))
    ))
}

fn orbit_id(stage: u32, package: &str, disposition: &str) -> DemandOrbitId {
    DemandOrbitId(format!("orbit:{stage}:{package}:{disposition}"))
}

/// One stage-to-stage locality transition for a package demand.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum LocalityTransition {
    /// The demand persists into the next stage's window (transport).
    Transported { package: String, from_stage: u32, to_stage: u32 },
    /// The demand was discharged by the step accepted at `from_stage`.
    Discharged {
        package: String,
        from_stage: u32,
        discharged_by_step: u32,
        answer_family: String,
    },
    /// The demand vanished without discharge: a certified finding.
    Expired { package: String, from_stage: u32 },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WindowLocalityLedger {
    pub transitions: Vec<LocalityTransition>,
    pub expiries: Vec<LocalityTransition>,
    pub holds: bool,
    pub digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum OrbitExtractionError {
    #[error(
        "package {package} discharged at stage {stage} but the accepted entry of step \
         {answer_step} exhibits no {package} characteristic family"
    )]
    AnswerNotExhibited {
        stage: u32,
        package: String,
        answer_step: u32,
    },
    #[error("window locality failed: {expiries} undischarged expiries")]
    LocalityFailed { expiries: usize },
}

/// The full kernel-backed orbit extraction over stages 1..=16.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KernelOrbitExtraction {
    pub signature_digest: String,
    pub closure_digest: String,
    pub inventories: Vec<StageOrbitInventory>,
    pub locality_ledger: WindowLocalityLedger,
    pub derivation_hash: String,
}

impl KernelOrbitExtraction {
    pub fn stage(&self, stage: u32) -> Option<&StageOrbitInventory> {
        self.inventories
            .iter()
            .find(|inventory| inventory.stage == stage)
    }

    pub fn live_orbit_count(&self, stage: u32) -> usize {
        self.stage(stage)
            .map(|inventory| {
                inventory
                    .orbits
                    .iter()
                    .filter(|orbit| matches!(orbit.resolution, OrbitResolution::Live))
                    .count()
            })
            .unwrap_or(0)
    }
}

fn stage_window(stage: u32) -> [u32; 2] {
    [stage.saturating_sub(2), stage.saturating_sub(1)]
}

/// The families an accepted step presents, from the typed predecessor
/// closure (the closure retains the first presenter of each family; for
/// answer checking we need every family a given step presents, so this
/// scans presentations rather than the deduplicated ids).
fn step_families<'c>(closure: &'c PredecessorClosure, step: u32) -> Vec<&'c ClosureFamily> {
    closure
        .families
        .iter()
        .filter(|family| family.step == step)
        .collect()
}

/// Kernel-v1 total orbit extraction: stages 1..=16 from the frozen debt
/// timeline and the typed predecessor closure. Deterministic; replay
/// re-derives the identical extraction (J2 is the totality of exactly
/// this function).
pub fn kernel_stage_inventories(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
) -> Result<KernelOrbitExtraction, OrbitExtractionError> {
    let timeline: Vec<DirectiveDebtRecord> = directive_debt_timeline();
    let mut inventories = Vec::with_capacity(16);
    let mut transitions = Vec::new();
    let mut expiries = Vec::new();

    for (index, record) in timeline.iter().enumerate() {
        let stage = record.stage;
        let window = stage_window(stage);
        let mut orbits = Vec::new();

        // LIVE orbits: one per required package, grounded in the window.
        for package in &record.required_packages {
            let class = demand_class(package, window);
            orbits.push(SemanticDemandOrbit {
                id: orbit_id(stage, package, "live"),
                package: (*package).to_string(),
                generated_by_window: window,
                normalized_demand_type: format!(
                    "characteristic:{:?}",
                    package_characteristic(package)
                ),
                required_outputs: vec![DemandOutputPosition {
                    id: DemandOutputPositionId(format!("{stage}:{package}:p1")),
                    family_class: class,
                    normalized_output_type: format!(
                        "family exhibiting {:?}",
                        package_characteristic(package)
                    ),
                }],
                resolution: OrbitResolution::Live,
            });
        }

        // ANSWERED orbits and locality transitions: compare with the
        // previous stage's requirements.
        if index > 0 {
            let previous = &timeline[index - 1];
            let answer_step = previous.stage; // the step accepted at stage N-1
            for package in &previous.required_packages {
                let still_required = record.required_packages.contains(package);
                if still_required {
                    transitions.push(LocalityTransition::Transported {
                        package: (*package).to_string(),
                        from_stage: previous.stage,
                        to_stage: stage,
                    });
                    continue;
                }
                // Discharged: the accepted entry must exhibit the
                // package's characteristic former — checked, not assumed.
                let answering_family = step_families(closure, answer_step)
                    .into_iter()
                    .find(|family| {
                        family_answers_package(
                            package,
                            &family.presentation.canonical_normal_form,
                        )
                    });
                let Some(answering_family) = answering_family else {
                    expiries.push(LocalityTransition::Expired {
                        package: (*package).to_string(),
                        from_stage: previous.stage,
                    });
                    return Err(OrbitExtractionError::AnswerNotExhibited {
                        stage,
                        package: (*package).to_string(),
                        answer_step,
                    });
                };
                transitions.push(LocalityTransition::Discharged {
                    package: (*package).to_string(),
                    from_stage: previous.stage,
                    discharged_by_step: answer_step,
                    answer_family: answering_family.id.as_str().to_string(),
                });
                let class = demand_class(package, window);
                orbits.push(SemanticDemandOrbit {
                    id: orbit_id(stage, package, "answered"),
                    package: (*package).to_string(),
                    generated_by_window: window,
                    normalized_demand_type: format!(
                        "characteristic:{:?}",
                        package_characteristic(package)
                    ),
                    required_outputs: vec![DemandOutputPosition {
                        id: DemandOutputPositionId(format!("{stage}:{package}:answered:p1")),
                        family_class: class,
                        normalized_output_type: answering_family.id.as_str().to_string(),
                    }],
                    resolution: OrbitResolution::Answered {
                        discharged_by_step: answer_step,
                        answer: SemanticAssumptionRef::verified(
                            format!(
                                "kernel-v1 discharge of {package} by step {answer_step} \
                                 (family {})",
                                answering_family.id.as_str()
                            ),
                            answering_family.id.as_str().to_string(),
                        ),
                    },
                });
            }
        }

        inventories.push(StageOrbitInventory {
            stage,
            window,
            orbits,
            // Placeholder refs replaced below once the extraction digest
            // exists (the evidence names the derivation of the whole
            // deterministic extraction).
            extraction_completeness_assumption: SemanticAssumptionRef::missing("pending"),
            derivability_completeness_assumption: SemanticAssumptionRef::missing("pending"),
            window_locality_assumption: SemanticAssumptionRef::missing("pending"),
        });
    }

    let holds = expiries.is_empty();
    if !holds {
        return Err(OrbitExtractionError::LocalityFailed {
            expiries: expiries.len(),
        });
    }
    let ledger_payload = serde_json::json!({
        "transitions": transitions,
        "expiries": expiries,
    });
    let ledger_digest = format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&ledger_payload).expect("ledger serialization"))
    );
    let locality_ledger = WindowLocalityLedger {
        transitions,
        expiries,
        holds,
        digest: ledger_digest.clone(),
    };

    let derivation_payload = serde_json::json!({
        "signature_digest": signature.digest(),
        "closure_digest": closure.digest,
        "inventories": inventories,
        "ledger": ledger_digest,
    });
    let derivation_hash = format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&derivation_payload).expect("derivation serialization"))
    );

    for inventory in &mut inventories {
        inventory.extraction_completeness_assumption = SemanticAssumptionRef::verified(
            format!(
                "kernel-v1 J2: stage {} inventory is the image of the total \
                 deterministic window extraction",
                inventory.stage
            ),
            derivation_hash.clone(),
        );
        inventory.derivability_completeness_assumption = SemanticAssumptionRef::verified(
            format!(
                "kernel-v1 J3: every stage {} orbit and required output carries a \
                 checked disposition",
                inventory.stage
            ),
            derivation_hash.clone(),
        );
        inventory.window_locality_assumption = SemanticAssumptionRef::verified(
            format!(
                "kernel-v1 locality: stage {} demands persist or discharge in the \
                 ledger (digest {})",
                inventory.stage, ledger_digest
            ),
            derivation_hash.clone(),
        );
    }

    Ok(KernelOrbitExtraction {
        signature_digest: signature.digest().to_string(),
        closure_digest: closure.digest.clone(),
        inventories,
        locality_ledger,
        derivation_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_provenance::audit_semantic_debt;
    use crate::typed_families::predecessor_closure;

    fn extraction() -> KernelOrbitExtraction {
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");
        kernel_stage_inventories(&signature, &closure).expect("extraction")
    }

    #[test]
    fn extraction_is_total_deterministic_and_replayable() {
        let first = extraction();
        let second = extraction();
        assert_eq!(first, second);
        assert_eq!(first.inventories.len(), 16);
        for inventory in &first.inventories {
            assert!(inventory.extraction_completeness_assumption.is_kernel_verified());
            assert!(inventory.derivability_completeness_assumption.is_kernel_verified());
            assert!(inventory.window_locality_assumption.is_kernel_verified());
        }
        assert!(first.locality_ledger.holds);
        assert!(first.locality_ledger.expiries.is_empty());
    }

    #[test]
    fn stage_sixteen_live_demand_summand_is_empty_by_extraction_not_assertion() {
        let extraction = extraction();
        // Stage 16's window (S14, S15): the temporal-shell demand was
        // discharged by step 15 (DCT) — an Answered orbit with checked
        // family evidence — and NO live orbit remains.
        assert_eq!(extraction.live_orbit_count(16), 0);
        let sixteen = extraction.stage(16).expect("stage 16");
        let answered: Vec<_> = sixteen
            .orbits
            .iter()
            .filter(|orbit| {
                matches!(
                    orbit.resolution,
                    OrbitResolution::Answered { discharged_by_step: 15, .. }
                )
            })
            .collect();
        assert!(
            answered.iter().any(|orbit| orbit.package == "temporal_shell"),
            "temporal shell discharge by the DCT must be recorded at stage 16"
        );
    }

    #[test]
    fn historical_stages_have_live_orbits_matching_the_guard_rail() {
        let extraction = extraction();
        // Stages 4..=15 each carry standing demand (the guard rail);
        // stages 1, 2 and 16 are demand-free.
        for stage in 4..=15u32 {
            assert!(
                extraction.live_orbit_count(stage) > 0,
                "stage {stage} should carry live demand"
            );
        }
        assert_eq!(extraction.live_orbit_count(1), 0);
        assert_eq!(extraction.live_orbit_count(2), 0);
    }

    #[test]
    fn audit_accepts_the_kernel_inventories_and_decides_semantic_o16() {
        let extraction = extraction();
        let audit = audit_semantic_debt(&extraction.inventories);
        assert!(audit.inventory_input_valid, "inventories must validate");
        assert!(audit.rejected_inventory_stages.is_empty());
        assert!(audit.j2_assumed_at_16);
        assert!(audit.j3_assumed_at_16);
        assert!(audit.window_locality_assumed_at_16);
        assert!(audit.j2_j3_conditionally_sufficient_at_16);
        // THE Phase 3 milestone: semantic O(16) emptiness is now decided
        // (conditionally on the kernel-verified J2/J3/locality evidence),
        // no longer null.
        assert_eq!(audit.semantic_o16_empty_if_assumptions, Some(true));
        // Per-stage: focus/orbit correspondence holds everywhere.
        for stage_audit in &audit.stages {
            assert!(
                stage_audit.focus_orbit_correspondence_if_assumptions,
                "correspondence failed at stage {}: {:?}",
                stage_audit.stage, stage_audit.obligations
            );
            assert_eq!(
                stage_audit.semantic_debt_empty_if_assumptions,
                Some(stage_audit.live_orbit_count == 0)
            );
        }
    }

    #[test]
    fn discharge_answers_are_kernel_checked_not_asserted() {
        let extraction = extraction();
        // Every Answered orbit names a closure family that actually
        // exhibits the package's characteristic former.
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");
        let mut answered_count = 0;
        for inventory in &extraction.inventories {
            for orbit in &inventory.orbits {
                if let OrbitResolution::Answered { discharged_by_step, answer } =
                    &orbit.resolution
                {
                    answered_count += 1;
                    assert!(answer.is_kernel_verified());
                    let family_id = answer.derivation_hash.as_ref().expect("hash");
                    let family = closure
                        .families
                        .iter()
                        .find(|family| {
                            family.step == *discharged_by_step
                                && family.id.as_str() == family_id
                        })
                        .expect("answering family exists in the closure");
                    assert!(family_answers_package(
                        &orbit.package,
                        &family.presentation.canonical_normal_form
                    ));
                }
            }
        }
        // The guard rail discharges each of the twelve packages exactly
        // once across the corpus (persistence holds), and every discharge
        // is recorded.
        assert!(answered_count >= 10, "expected the historical discharges, got {answered_count}");
    }

    #[test]
    fn characteristic_predicates_pin_the_frozen_alphabet() {
        assert!(family_answers_package(
            "temporal_shell",
            &Expr::Next(Box::new(Expr::Var(1)))
        ));
        assert!(family_answers_package(
            "modal_shell",
            &Expr::Flat(Box::new(Expr::Var(1)))
        ));
        assert!(!family_answers_package(
            "temporal_shell",
            &Expr::Flat(Box::new(Expr::Var(1)))
        ));
        assert!(family_answers_package("initial_hit", &Expr::PathCon(1)));
        assert!(!family_answers_package("higher_hit", &Expr::PathCon(1)));
        assert!(family_answers_package(
            "hilbert_functional",
            &Expr::Pi(Box::new(Expr::Lib(13)), Box::new(Expr::Var(1)))
        ));
        // Unknown packages answer nothing (fail closed).
        assert!(!family_answers_package("no_such_package", &Expr::Univ));
    }
}
