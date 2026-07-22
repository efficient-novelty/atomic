//! Clause-local elaboration fuel witnesses and their whole-telescope sum.
//!
//! Legacy elaboration already executes a telescope with one shared budget.
//! This additive module records the missing compositional statement: each
//! clause is bounded by `node_count_i * kappa`, and those allocations sum
//! exactly to `total_node_count * kappa`.  Issuance fails closed if a clause
//! exceeds its allocation.

use crate::elaborate::{ClauseFailure, SealedSignature, candidate_hash, elaborate_telescope};
use pen_core::clause::ClauseRole;
use pen_core::hash::blake3_hex;
use pen_core::stats::StructuralStats;
use pen_core::telescope::Telescope;
use serde::Serialize;
use thiserror::Error;

pub const FUEL_COMPOSITION_VERSION: &str = "kernel-clause-fuel-composition-v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClauseFuelWitness {
    pub clause_index: u16,
    pub ambient: u32,
    pub prior_kernel_roles: Vec<ClauseRole>,
    pub node_count: u32,
    pub synth_and_normalize_steps: u32,
    pub allocated_bound: u32,
    pub within_local_bound: bool,
    pub witness_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TelescopeFuelCompositionToken {
    subject_hash: String,
    signature_digest: String,
    visible_library: u32,
    kappa: u32,
    clauses: Vec<ClauseFuelWitness>,
    sum_observed: u32,
    sum_allocated: u32,
    whole_static_bound: u32,
    allocations_sum_to_whole_bound: bool,
    observed_sum_matches_whole_run: bool,
    derivation_hash: String,
}

impl TelescopeFuelCompositionToken {
    pub fn clauses(&self) -> &[ClauseFuelWitness] {
        &self.clauses
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn sum_observed(&self) -> u32 {
        self.sum_observed
    }

    pub fn whole_static_bound(&self) -> u32 {
        self.whole_static_bound
    }

    pub fn composition_proved(&self) -> bool {
        self.allocations_sum_to_whole_bound
            && self.observed_sum_matches_whole_run
            && self.clauses.iter().all(|clause| clause.within_local_bound)
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum FuelCompositionError {
    #[error("whole-telescope elaboration failed: {0}")]
    Elaboration(ClauseFailure),
    #[error(
        "clause {clause_index} used {observed} fuel, exceeding its structural allocation {allocated}"
    )]
    LocalBoundExceeded {
        clause_index: u16,
        observed: u32,
        allocated: u32,
    },
    #[error("clause allocations do not sum to the whole static bound")]
    AllocationSumMismatch,
    #[error("clause observations do not sum to the whole run's observation")]
    ObservationSumMismatch,
    #[error("fuel-composition replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(FUEL_COMPOSITION_VERSION, domain, payload))
        .expect("fuel proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

pub fn issue_telescope_fuel_composition(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
) -> Result<TelescopeFuelCompositionToken, FuelCompositionError> {
    let elaboration = elaborate_telescope(signature, telescope, visible_library)
        .map_err(FuelCompositionError::Elaboration)?;
    let kappa = u32::try_from(telescope.kappa()).expect("telescope size fits u32");
    let mut prior_kernel_roles = Vec::new();
    let mut clauses = Vec::with_capacity(telescope.clauses.len());
    for (source, elaborated) in telescope.clauses.iter().zip(&elaboration.clauses) {
        let node_count = StructuralStats::from_expr(&source.expr).node_count;
        let allocated_bound = node_count.saturating_mul(kappa.max(1));
        let within_local_bound = elaborated.total_fuel_observed <= allocated_bound;
        if !within_local_bound {
            return Err(FuelCompositionError::LocalBoundExceeded {
                clause_index: elaborated.clause_index,
                observed: elaborated.total_fuel_observed,
                allocated: allocated_bound,
            });
        }
        let witness_hash = tagged_hash(
            "clause-local-fuel",
            &(
                elaborated.clause_index,
                elaboration.ambient_parameters,
                &prior_kernel_roles,
                &source.expr,
                node_count,
                elaborated.total_fuel_observed,
                allocated_bound,
            ),
        );
        clauses.push(ClauseFuelWitness {
            clause_index: elaborated.clause_index,
            ambient: elaboration.ambient_parameters,
            prior_kernel_roles: prior_kernel_roles.clone(),
            node_count,
            synth_and_normalize_steps: elaborated.total_fuel_observed,
            allocated_bound,
            within_local_bound,
            witness_hash,
        });
        prior_kernel_roles.push(elaborated.kernel_role);
    }
    let sum_observed = clauses.iter().fold(0u32, |sum, clause| {
        sum.saturating_add(clause.synth_and_normalize_steps)
    });
    let sum_allocated = clauses.iter().fold(0u32, |sum, clause| {
        sum.saturating_add(clause.allocated_bound)
    });
    let whole_static_bound = elaboration.fuel.static_bound;
    let allocations_sum_to_whole_bound = sum_allocated == whole_static_bound;
    if !allocations_sum_to_whole_bound {
        return Err(FuelCompositionError::AllocationSumMismatch);
    }
    let observed_sum_matches_whole_run = sum_observed == elaboration.fuel.total_fuel_observed;
    if !observed_sum_matches_whole_run {
        return Err(FuelCompositionError::ObservationSumMismatch);
    }
    let subject_hash = candidate_hash(telescope);
    let signature_digest = signature.digest().to_owned();
    let derivation_hash = tagged_hash(
        "whole-telescope-fuel-composition",
        &(
            &subject_hash,
            &signature_digest,
            visible_library,
            kappa,
            &clauses,
            sum_observed,
            sum_allocated,
            whole_static_bound,
            allocations_sum_to_whole_bound,
            observed_sum_matches_whole_run,
        ),
    );
    Ok(TelescopeFuelCompositionToken {
        subject_hash,
        signature_digest,
        visible_library,
        kappa,
        clauses,
        sum_observed,
        sum_allocated,
        whole_static_bound,
        allocations_sum_to_whole_bound,
        observed_sum_matches_whole_run,
        derivation_hash,
    })
}

pub fn replay_telescope_fuel_composition(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    token: &TelescopeFuelCompositionToken,
) -> Result<(), FuelCompositionError> {
    let replay = issue_telescope_fuel_composition(signature, telescope, visible_library)?;
    if replay == *token {
        Ok(())
    } else {
        Err(FuelCompositionError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_fifteen_sealed_telescopes_have_compositional_fuel_witnesses() {
        let signature = SealedSignature::genesis_del_h15();
        for step in 1..=15 {
            let telescope = Telescope::reference(step);
            let token =
                issue_telescope_fuel_composition(&signature, &telescope, step.saturating_sub(1))
                    .unwrap_or_else(|error| panic!("step {step} fuel composition failed: {error}"));
            assert!(token.composition_proved());
            replay_telescope_fuel_composition(
                &signature,
                &telescope,
                step.saturating_sub(1),
                &token,
            )
            .expect("fuel token replays");
        }
    }

    #[test]
    fn mutation_is_rejected() {
        let signature = SealedSignature::genesis_del_h15();
        let telescope = Telescope::reference(5);
        let mut token = issue_telescope_fuel_composition(&signature, &telescope, 4)
            .expect("historical telescope composes");
        token.derivation_hash.push('0');
        assert_eq!(
            replay_telescope_fuel_composition(&signature, &telescope, 4, &token),
            Err(FuelCompositionError::ReplayMismatch)
        );
    }
}
