//! Versioned TDC cubical-path regression.
//!
//! This evaluator consumes opaque tokens from `pen-type::cubical`.  It checks
//! the registered `1 + d^2` path basis for historical Steps 5--8 and for the
//! registered `d=4` candidate.  It does not claim that this registered basis
//! exhausts intended semantic schemas, and it intentionally does **not** turn
//! the local replay into a total historical-score or Step-16 novelty verdict.

use crate::demand_orbits::kernel_stage_inventories;
use crate::semantic_provenance::OrbitResolution;
use crate::tdc1::Tdc1Zone;
use crate::typed_families::predecessor_closure;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::cubical::{
    CUBICAL_FRAGMENT_VERSION, PATHCON_ATTACHMENT_AXIOM_VERSION, PathRealizationToken,
    realize_path_basis, replay_path_realization,
};
use pen_type::elaborate::SealedSignature;
use pen_type::tdc1::{
    PathSchemaKey, elaborate_formed_path, elaborate_tdc1_package, enumerate_path_schema_obligations,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const TDC1_CUBICAL_REGRESSION_SCHEMA: &str = "tdc1-cubical-step5-8-regression-v3";
pub const SUPERSEDED_TDC1_SCHEMAS: [&str; 2] = [
    "tdc1-typed-d4-certificate-v1",
    "tdc1-cubical-step5-8-regression-v2",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RealizationTokenAudit {
    pub key: PathSchemaKey,
    pub subject_hash: String,
    pub signature_digest: String,
    pub source_digest: String,
    pub term_hash: String,
    pub normal_form_hash: String,
    pub derivation_hash: String,
}

impl From<&PathRealizationToken> for RealizationTokenAudit {
    fn from(token: &PathRealizationToken) -> Self {
        Self {
            key: token.key().clone(),
            subject_hash: token.subject_hash().to_owned(),
            signature_digest: token.signature_digest().to_owned(),
            source_digest: token.source_digest().to_owned(),
            term_hash: token.term_hash().to_owned(),
            normal_form_hash: token.normal_form_hash().to_owned(),
            derivation_hash: token.derivation_hash().to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PathRegressionRow {
    pub step: Option<u32>,
    pub label: String,
    pub dimension: u32,
    pub recorded_path_nu: u32,
    pub realized_registered_basis_count: u32,
    pub certified_path_nu: Option<u32>,
    pub exact_index_bijection: bool,
    pub unique_source_bound_tokens: bool,
    pub basis_derivation_hash: String,
    pub token_audit: Vec<RealizationTokenAudit>,
    pub recorded_total_nu: Option<u32>,
    pub certified_total_nu: Option<u32>,
    pub registered_basis_replay_passed: bool,
    pub intended_schema_exhaustiveness_proved: bool,
    pub full_total_regression_passed: bool,
    pub blockers: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProvenanceObstruction {
    pub subject: String,
    pub inventory_stage: u32,
    pub dimension_squared_kan_families: u32,
    pub local_telescope_kappa: u32,
    pub local_role_capacity_rule: String,
    pub local_coherence_slots_upper_bound: u32,
    pub live_demand_outputs: u32,
    pub available_distinct_anchors_upper_bound: u32,
    pub closure_digest: String,
    pub orbit_derivation_hash: String,
    pub kernel_inventory_j2_verified: bool,
    pub kernel_inventory_j3_verified: bool,
    pub intended_semantic_j2_complete: bool,
    pub intended_semantic_j3_complete: bool,
    pub conditional_injective_assignment_possible: bool,
    pub certified_provenance_obstruction: bool,
    pub conclusion: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CubicalRegressionResult {
    pub schema: String,
    pub supersedes_without_overwriting: Vec<String>,
    pub cubical_fragment_version: String,
    pub pathcon_attachment_axiom_version: String,
    pub pathcon_attachment_axiom_status: String,
    pub pathcon_attachment_derived_from_shallow_kernel: bool,
    pub pathcon_attachment_user_adopted: bool,
    pub complete_union_staging: bool,
    pub intended_path_schema_exhaustiveness_proved: bool,
    pub historical_steps: Vec<PathRegressionRow>,
    pub registered_d4: PathRegressionRow,
    pub conditional_typed_basis_replay_passed: bool,
    pub full_historical_total_regression_passed: bool,
    pub provenance_obstructions: Vec<ProvenanceObstruction>,
    pub semantic_weakening_complete: bool,
    pub novelty_numeric: Option<u32>,
    pub registered_zone: Tdc1Zone,
    pub falsifier_f_t1: bool,
    pub step16_verdict_valid: bool,
    pub d5_attempted: bool,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CubicalRegressionError {
    #[error("formed-path elaboration failed: {0}")]
    Elaboration(String),
    #[error("cubical basis realization failed: {0}")]
    Realization(String),
    #[error("historical dimension mismatch at Step {step}: found {found}, expected {expected}")]
    DimensionMismatch {
        step: u32,
        found: u32,
        expected: u32,
    },
    #[error("cubical regression replay mismatch")]
    ReplayMismatch,
    #[error("predecessor closure failed: {0}")]
    Closure(String),
    #[error("demand-orbit extraction failed: {0}")]
    Orbits(String),
    #[error("demand-orbit inventory is missing stage {0}")]
    MissingInventory(u32),
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(TDC1_CUBICAL_REGRESSION_SCHEMA, domain, payload))
        .expect("cubical regression data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn historical_specs() -> [(u32, &'static str, u32, u32, u32); 4] {
    [
        (5, "S1", 1, 2, 7),
        (6, "Trunc", 1, 2, 8),
        (7, "S2", 2, 5, 10),
        (8, "S3", 3, 10, 18),
    ]
}

fn audit_row(
    step: Option<u32>,
    label: &str,
    typing: &pen_type::tdc1::FormedPathTyping,
    recorded_path_nu: u32,
    recorded_total_nu: Option<u32>,
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
) -> Result<PathRegressionRow, CubicalRegressionError> {
    let realization = realize_path_basis(signature, telescope, visible_library, typing)
        .map_err(|error| CubicalRegressionError::Realization(error.to_string()))?;
    let obligations = enumerate_path_schema_obligations(typing);
    let expected_keys = obligations
        .iter()
        .map(|obligation| obligation.normal_form.key.clone())
        .collect::<BTreeSet<_>>();
    let token_keys = realization
        .tokens()
        .iter()
        .map(|token| token.key().clone())
        .collect::<BTreeSet<_>>();
    let all_tokens_replay = realization.tokens().iter().all(|token| {
        replay_path_realization(signature, telescope, visible_library, typing, token).is_ok()
    });
    let unique_bindings = realization
        .tokens()
        .iter()
        .map(|token| {
            (
                token.source_digest(),
                token.signature_digest(),
                token.term_hash(),
                token.derivation_hash(),
            )
        })
        .collect::<BTreeSet<_>>()
        .len()
        == realization.tokens().len()
        && realization
            .tokens()
            .iter()
            .all(|token| token.signature_digest() == signature.digest());
    let exact_index_bijection = all_tokens_replay
        && expected_keys == token_keys
        && obligations.len() == realization.tokens().len();
    let realized_registered_basis_count =
        u32::try_from(realization.tokens().len()).expect("formed-path basis count fits u32");
    let registered_basis_replay_passed = exact_index_bijection
        && unique_bindings
        && realized_registered_basis_count == recorded_path_nu;

    let mut blockers = vec![
        "the registered 1+d^2 basis is replayed but no theorem classifies all intended path schemas by that basis"
            .to_owned(),
        "the conditional closed cubical fragment checks only the registered PathCon beta/Kan terms; it does not reconstruct every formation, introduction, elimination, or chaining family in the historical total"
            .to_owned(),
        "cubical family weakening and univalent equality have not been integrated into the global semantic-family quotient"
            .to_owned(),
    ];
    match step {
        Some(6) => blockers.push(
            "the shallow Trunc telescope does not type a Trunc functoriality schema in this fragment"
                .to_owned(),
        ),
        Some(8) => blockers.push(
            "the shallow S3 telescope does not type the recorded mu/unit coherence and cell-action schemas"
                .to_owned(),
        ),
        None => blockers.push(
            "under the frozen extracted-orbit inventory, the conditional capacity diagnostic has no injection for the sixteen registered Kan terms; intended semantic J2/J3 remain open"
                .to_owned(),
        ),
        _ => {}
    }

    Ok(PathRegressionRow {
        step,
        label: label.to_owned(),
        dimension: typing.dimension,
        recorded_path_nu,
        realized_registered_basis_count,
        certified_path_nu: None,
        exact_index_bijection,
        unique_source_bound_tokens: unique_bindings,
        basis_derivation_hash: realization.derivation_hash().to_owned(),
        token_audit: realization
            .tokens()
            .iter()
            .map(RealizationTokenAudit::from)
            .collect(),
        recorded_total_nu,
        certified_total_nu: None,
        registered_basis_replay_passed,
        intended_schema_exhaustiveness_proved: false,
        full_total_regression_passed: false,
        blockers,
    })
}

fn historical_rows() -> Result<Vec<PathRegressionRow>, CubicalRegressionError> {
    let mut rows = Vec::new();
    for (step, label, dimension, recorded_path_nu, recorded_total_nu) in historical_specs() {
        let signature = SealedSignature::from_telescopes(
            (1..step)
                .map(|index| (index, Telescope::reference(index)))
                .collect(),
        );
        let telescope = Telescope::reference(step);
        let (typing, _) = elaborate_formed_path(&signature, &telescope, step - 1)
            .map_err(|error| CubicalRegressionError::Elaboration(error.to_string()))?;
        if typing.dimension != dimension {
            return Err(CubicalRegressionError::DimensionMismatch {
                step,
                found: typing.dimension,
                expected: dimension,
            });
        }
        rows.push(audit_row(
            Some(step),
            label,
            &typing,
            recorded_path_nu,
            Some(recorded_total_nu),
            &signature,
            &telescope,
            step - 1,
        )?);
    }
    Ok(rows)
}

fn registered_d4_telescope() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Lib(15))),
        ),
        ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(4)),
    ])
}

fn registered_d4_row() -> Result<PathRegressionRow, CubicalRegressionError> {
    let signature = SealedSignature::genesis_del_h15();
    let telescope = registered_d4_telescope();
    let (typing, _) = elaborate_tdc1_package(&signature, &telescope, 15)
        .map_err(|error| CubicalRegressionError::Elaboration(error.to_string()))?;
    audit_row(
        None,
        "registered-d4",
        &typing,
        17,
        None,
        &signature,
        &telescope,
        15,
    )
}

fn provenance_obstructions() -> Result<Vec<ProvenanceObstruction>, CubicalRegressionError> {
    let signature = SealedSignature::genesis_del_h15();
    let closure = predecessor_closure(&signature)
        .map_err(|error| CubicalRegressionError::Closure(error.to_string()))?;
    let extraction = kernel_stage_inventories(&signature, &closure)
        .map_err(|error| CubicalRegressionError::Orbits(error.to_string()))?;

    let inputs = [
        ("Step 8 (S3)", 8_u32, 3_u32, Telescope::reference(8)),
        (
            "registered d=4 candidate",
            16_u32,
            4_u32,
            registered_d4_telescope(),
        ),
    ];
    inputs
        .into_iter()
        .map(|(subject, inventory_stage, dimension, telescope)| {
            let inventory = extraction
                .stage(inventory_stage)
                .ok_or(CubicalRegressionError::MissingInventory(inventory_stage))?;
            let local_kappa = u32::try_from(telescope.kappa())
                .expect("telescope clause count fits in u32");
            // Blind local-role bound: every local clause is granted at most
            // one independently chargeable coherence anchor.  This is an
            // upper bound derived from the actual telescope, not a label.
            let local_capacity = local_kappa;
            let live_outputs = inventory
                .orbits
                .iter()
                .filter(|orbit| matches!(orbit.resolution, OrbitResolution::Live))
                .map(|orbit| orbit.required_outputs.len())
                .sum::<usize>();
            let live_outputs =
                u32::try_from(live_outputs).expect("live output count fits in u32");
            let kan = dimension.pow(2);
            let capacity = local_capacity + live_outputs;
            let conditional_injective = kan <= capacity;
            let kernel_j2 = inventory
                .extraction_completeness_assumption
                .is_kernel_verified();
            let kernel_j3 = inventory
                .derivability_completeness_assumption
                .is_kernel_verified();
            // The kernel inventory is complete for its frozen package
            // extractor.  It is not a classification theorem for all
            // intended depth-two semantic demand schemas.
            let intended_j2 = false;
            let intended_j3 = false;
            let certified_obstruction = kernel_j2
                && kernel_j3
                && intended_j2
                && intended_j3
                && !conditional_injective;
            Ok(ProvenanceObstruction {
                subject: subject.to_owned(),
                inventory_stage,
                dimension_squared_kan_families: kan,
                local_telescope_kappa: local_kappa,
                local_role_capacity_rule:
                    "blind upper bound: at most one independently chargeable local coherence anchor per telescope clause"
                        .to_owned(),
                local_coherence_slots_upper_bound: local_capacity,
                live_demand_outputs: live_outputs,
                available_distinct_anchors_upper_bound: capacity,
                closure_digest: closure.digest.clone(),
                orbit_derivation_hash: extraction.derivation_hash.clone(),
                kernel_inventory_j2_verified: kernel_j2,
                kernel_inventory_j3_verified: kernel_j3,
                intended_semantic_j2_complete: intended_j2,
                intended_semantic_j3_complete: intended_j3,
                conditional_injective_assignment_possible: conditional_injective,
                certified_provenance_obstruction: certified_obstruction,
                conclusion: format!(
                    "Conditional diagnostic only: under the frozen extracted-orbit inventory and one-anchor-per-clause bound, {kan} separately counted Kan families do not inject into {capacity} local/live anchors; intended semantic J2/J3 completeness is not proved"
                ),
            })
        })
        .collect()
}

pub fn build_cubical_regression() -> Result<CubicalRegressionResult, CubicalRegressionError> {
    let historical_steps = historical_rows()?;
    let registered_d4 = registered_d4_row()?;
    let conditional_typed_basis_replay_passed = historical_steps
        .iter()
        .all(|row| row.registered_basis_replay_passed)
        && registered_d4.registered_basis_replay_passed;
    let provenance_obstructions = provenance_obstructions()?;

    let mut result = CubicalRegressionResult {
        schema: TDC1_CUBICAL_REGRESSION_SCHEMA.to_owned(),
        supersedes_without_overwriting: SUPERSEDED_TDC1_SCHEMAS
            .iter()
            .map(|schema| (*schema).to_owned())
            .collect(),
        cubical_fragment_version: CUBICAL_FRAGMENT_VERSION.to_owned(),
        pathcon_attachment_axiom_version: PATHCON_ATTACHMENT_AXIOM_VERSION.to_owned(),
        pathcon_attachment_axiom_status: "conditional_theory_relative".to_owned(),
        pathcon_attachment_derived_from_shallow_kernel: false,
        pathcon_attachment_user_adopted: false,
        // Only a restricted top-face nested instance is checked.  A general
        // CCHM union-staging theorem is not present and is not used to infer
        // exhaustiveness of the registered basis.
        complete_union_staging: false,
        intended_path_schema_exhaustiveness_proved: false,
        historical_steps,
        registered_d4,
        conditional_typed_basis_replay_passed,
        full_historical_total_regression_passed: false,
        provenance_obstructions,
        semantic_weakening_complete: false,
        novelty_numeric: None,
        registered_zone: Tdc1Zone::Z4,
        // F-T1 is the full Step 5--8 score regression, not merely its path
        // channel.  It remains triggered even though the path sub-regression
        // now passes.
        falsifier_f_t1: true,
        step16_verdict_valid: false,
        d5_attempted: false,
        remaining_obligations: vec![
            "type and replay the non-path families in the Step 5--8 historical totals"
                .to_owned(),
            "extend semantic weakening/univalent equality to normalized cubical family terms"
                .to_owned(),
            "prove intended semantic J2/J3 completeness before promoting the derived provenance-capacity arithmetic from a conditional diagnostic"
                .to_owned(),
            "prove general typed hcom union staging and a semantic classification theorem before treating the registered 1+d^2 basis as exhaustive"
                .to_owned(),
            "supply an injective provenance discipline that does not multiply uniform Kan specializations without independent export or live demand"
                .to_owned(),
            "only after those gates pass may a numeric Step-16 verdict or a d=5 experiment be attempted"
                .to_owned(),
        ],
        result_digest: String::new(),
    };
    result.result_digest = tagged_hash("result", &result);
    Ok(result)
}

pub fn replay_cubical_regression(
    presented: &CubicalRegressionResult,
) -> Result<(), CubicalRegressionError> {
    let replay = build_cubical_regression()?;
    if &replay == presented {
        Ok(())
    } else {
        Err(CubicalRegressionError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_basis_replays_but_path_nu_and_totals_remain_undefined() {
        let result = build_cubical_regression().expect("cubical regression");
        assert_eq!(
            result
                .historical_steps
                .iter()
                .map(|row| row.realized_registered_basis_count)
                .collect::<Vec<_>>(),
            vec![2, 2, 5, 10]
        );
        assert_eq!(result.registered_d4.realized_registered_basis_count, 17);
        assert!(result.conditional_typed_basis_replay_passed);
        assert!(
            result
                .historical_steps
                .iter()
                .all(|row| row.certified_path_nu.is_none())
        );
        assert!(result.registered_d4.certified_path_nu.is_none());
        assert!(!result.intended_path_schema_exhaustiveness_proved);
        assert!(!result.complete_union_staging);
        assert!(!result.full_historical_total_regression_passed);
        assert!(
            result
                .historical_steps
                .iter()
                .all(|row| row.certified_total_nu.is_none())
        );
    }

    #[test]
    fn provenance_keeps_result_in_z4_and_stops_before_d5() {
        let result = build_cubical_regression().expect("cubical regression");
        assert_eq!(result.registered_zone, Tdc1Zone::Z4);
        assert!(result.falsifier_f_t1);
        assert!(!result.step16_verdict_valid);
        assert!(!result.d5_attempted);
        assert_eq!(result.novelty_numeric, None);
        assert!(result.provenance_obstructions.iter().all(|audit| {
            audit.kernel_inventory_j2_verified
                && audit.kernel_inventory_j3_verified
                && !audit.intended_semantic_j2_complete
                && !audit.intended_semantic_j3_complete
                && !audit.conditional_injective_assignment_possible
                && !audit.certified_provenance_obstruction
                && audit.conclusion.starts_with("Conditional diagnostic only")
        }));
        assert!(!result.pathcon_attachment_derived_from_shallow_kernel);
        assert!(!result.pathcon_attachment_user_adopted);
    }

    #[test]
    fn result_replays_and_mutations_fail_closed() {
        let result = build_cubical_regression().expect("cubical regression");
        replay_cubical_regression(&result).expect("exact replay");
        let mut mutation = result;
        mutation.d5_attempted = true;
        assert_eq!(
            replay_cubical_regression(&mutation),
            Err(CubicalRegressionError::ReplayMismatch)
        );
    }
}
