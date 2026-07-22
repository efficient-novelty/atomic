//! Branch-(ii) semantic certification of the complete Genesis history.
//!
//! Historical formula units receive no credit merely from their labels.  A
//! unit is certified only by an injective charged `(clause, LocalRole)` anchor
//! or by a distinct, live, pre-existing depth-two demand output.  The demand
//! grammar is generated from the preceding two typed telescopes before the
//! current candidate is inspected.

use crate::agent_a_hist_cert_v4::{
    AGENT_A_HIST_CERT_V4_SCHEMA, AgentAHistCertV4Certificate, replay_agent_a_hist_cert_v4_json,
};
use crate::e2b_quotient_closure::{
    E2bQuotientClosureCertificate, replay_e2b_quotient_closure_json,
};
use crate::global_e4_assembly_v10::{GlobalE4V10Certificate, replay_global_e4_v10_json};
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::debt_guard::directive_debt_timeline;
use pen_eval::nu::structural_nu;
use pen_eval::semantic_provenance::{
    CreditMechanism, HistoricalStepProvenance, replay_genesis_provenance,
};
use pen_schema::grammar_completion::{
    GrammarCompletionCertificate, replay_grammar_completion_json,
};
use pen_type::elaborate::{SealedSignature, candidate_hash, elaborate_telescope};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const PHASE5B_HISTORY_CERT_SCHEMA: &str = "phase5b-full-history-certification-v1";
pub const PHASE5B_HISTORY_CERT_DATE: &str = "2026-07-21";
pub const DEMAND_GRAMMAR_VERSION: &str = "schema2-depth-two-demand-orbits-v1";
pub const PROVENANCE_UNIT_VERSION: &str = "selective-law-provenance-unit-v1";

const FORK_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_fork_adjudication.md");
const AGENT_A_BYTES: &[u8] = include_bytes!("../../../docs/hist_cert_v4_branch_ii.json");
const E2B_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e2b_quotient_closure_v1.json");
const GRAMMAR_BYTES: &[u8] = include_bytes!("../../../docs/schema2_grammar_completion_v1.json");
const E4_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v10.json");
const SIGNATURE_ADOPTION_BYTES: &[u8] =
    include_bytes!("../../../docs/steps_9_15_signature_adoption.md");
const PROVENANCE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/semantic_provenance.rs");
const NU_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/nu.rs");
const TELESCOPE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("phase5b_history_certification.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistorySourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedLocalRole {
    KernelHead,
    AdjointMate,
    SupportAction,
    Coherence,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DemandSchemaKind {
    UnaryAction,
    BinaryComparison,
    HigherOpenBoxReduction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DemandGrammarSummary {
    pub stage: u32,
    pub window_steps: Vec<u32>,
    pub source_clause_count: u32,
    pub mechanism_count: u32,
    pub unary_schema_count: u32,
    pub binary_schema_count: u32,
    pub higher_open_box_schema_count: u32,
    pub total_orbit_capacity: u32,
    pub live_packages: Vec<String>,
    pub generated_before_candidate: bool,
    pub finite: bool,
    pub typed_by_completed_schema2: bool,
    pub equality_orbit_quotient_decidable: bool,
    pub d_membership_decidable: bool,
    pub every_generated_orbit_derivable: bool,
    pub grammar_digest: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CertifiedProvenanceAnchor {
    ChargedLocalRole {
        clause: u16,
        role: CertifiedLocalRole,
    },
    LiveDemandOutput {
        orbit_id: String,
        demand_kind: DemandSchemaKind,
        source_left_step: u32,
        source_left_clause: u16,
        source_right_step: u32,
        source_right_clause: u16,
        required_output_position: u32,
        live_package: String,
        d_membership_derivation_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFamilyUnit {
    pub token_version: String,
    pub step: u32,
    pub unit_ordinal: u32,
    pub mechanism: CreditMechanism,
    pub mechanism_unit_ordinal: u32,
    pub family_id: String,
    pub typed_family_derivation_hash: String,
    pub normalization_derivation_hash: String,
    pub naturality_derivation_hash: String,
    pub anchor: CertifiedProvenanceAnchor,
    pub anchor_valid: bool,
    pub family_marginal: bool,
    pub family_or_instance_decided: bool,
    pub uniform_specialization_multiplied_without_exported_orbit: bool,
    pub historical_count_used_as_input: bool,
    pub acceptance_bar_used_as_input: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FullHistoryStepCertification {
    pub step: u32,
    pub class: TelescopeClass,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub elaboration_derivation_hash: String,
    pub kappa: u32,
    pub structural_formula_total: u32,
    pub imported_successor_certificate: Option<String>,
    pub newly_issued_units: Vec<CertifiedFamilyUnit>,
    pub certified_semantic_total: u32,
    pub testimonial_sealed_total: u32,
    pub signed_divergence: i32,
    pub divergence_cause: Option<String>,
    pub demand_grammar: DemandGrammarSummary,
    pub local_anchor_count: u32,
    pub demand_anchor_count: u32,
    pub local_anchor_bound: u32,
    pub local_anchor_injection_holds: bool,
    pub demand_output_nonreuse_holds: bool,
    pub every_counted_family_provenanced: bool,
    pub certified: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RevisedLedgerSummary {
    pub certified_nu_vector: Vec<u32>,
    pub testimonial_sealed_nu_vector: Vec<u32>,
    pub kappa_vector: Vec<u32>,
    pub divergence_steps: Vec<u32>,
    pub sum_nu: u32,
    pub sum_kappa: u32,
    pub omega_unreduced_numerator: u32,
    pub omega_unreduced_denominator: u32,
    pub phi_16_numerator: u32,
    pub phi_16_denominator: u32,
    pub bar_16_unreduced_numerator: u32,
    pub bar_16_unreduced_denominator: u32,
    pub bar_16_reduced: String,
    pub every_entry_certified: bool,
    pub sealed_history_edited: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryDeferredSequence {
    pub reselection_executed: bool,
    pub e5_f1_executed: bool,
    pub bridge_executed: bool,
    pub final_certificate_executed: bool,
}

impl HistoryDeferredSequence {
    fn all_deferred(&self) -> bool {
        !self.reselection_executed
            && !self.e5_f1_executed
            && !self.bridge_executed
            && !self.final_certificate_executed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase5bHistoryCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<HistorySourceBinding>,
    pub branch_ii_replayed: bool,
    pub agent_a_schema: String,
    pub agent_a_digest: String,
    pub agent_a_replayed: bool,
    pub e2b_digest: String,
    pub e2b_replayed: bool,
    pub grammar_completion_digest: String,
    pub grammar_completion_sealed_digest_verified: bool,
    pub grammar_completion_live_definition_replay_valid: bool,
    pub grammar_completion_live_replay_errors: Vec<String>,
    pub grammar_completion_frozen_surface_drift_bound_explicitly: bool,
    pub global_e4_digest: String,
    pub global_e4_replayed: bool,
    pub demand_grammar_version: String,
    pub local_role_inventory: Vec<CertifiedLocalRole>,
    pub steps: Vec<FullHistoryStepCertification>,
    pub remaining_steps_certified: Vec<u32>,
    pub all_fifteen_steps_certified: bool,
    pub all_amplification_provenanced: bool,
    pub ledger: RevisedLedgerSummary,
    pub reselection_now_authorized: bool,
    pub downstream: HistoryDeferredSequence,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Phase5bHistoryReplay {
    pub valid: bool,
    pub all_fifteen_steps_certified: bool,
    pub certified_nu_vector: Vec<u32>,
    pub divergence_steps: Vec<u32>,
    pub sum_nu: u32,
    pub sum_kappa: u32,
    pub revised_bar_16: String,
    pub reselection_authorized: bool,
    pub downstream_deferred: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Phase5bHistoryError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("history invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

const MECHANISMS: [CreditMechanism; 10] = [
    CreditMechanism::IntrinsicKernel,
    CreditMechanism::AdjointCompletion,
    CreditMechanism::P5InheritedSurface,
    CreditMechanism::P5LocalAndBridge,
    CreditMechanism::P6UniformSpecialization,
    CreditMechanism::DimensionSquared,
    CreditMechanism::ReferenceSquared,
    CreditMechanism::DistributiveInheritance,
    CreditMechanism::CombinatorialSynthesis,
    CreditMechanism::ModalPairwiseCoherence,
];

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(PHASE5B_HISTORY_CERT_SCHEMA, domain, value))
        .expect("history certification evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<HistorySourceBinding> {
    [
        (
            "docs/phase5b_fork_adjudication.md",
            "branch_ii_governing_ledger",
            FORK_BYTES,
        ),
        (
            "docs/hist_cert_v4_branch_ii.json",
            "agent_a_certified_hit_totals",
            AGENT_A_BYTES,
        ),
        (
            "docs/schema2_e2b_quotient_closure_v1.json",
            "stage1_and_r1_r2_quotient_outputs",
            E2B_BYTES,
        ),
        (
            "docs/schema2_grammar_completion_v1.json",
            "typed_trace_signatures_and_derived_closure",
            GRAMMAR_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v10.json",
            "completed_basis_and_class_exhaustion",
            E4_BYTES,
        ),
        (
            "docs/steps_9_15_signature_adoption.md",
            "adopted_trace_faithful_semantics",
            SIGNATURE_ADOPTION_BYTES,
        ),
        (
            "crates/pen-eval/src/semantic_provenance.rs",
            "count_blind_mechanism_decomposition",
            PROVENANCE_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/nu.rs",
            "structural_formula_regression_only",
            NU_SOURCE_BYTES,
        ),
        (
            "crates/pen-core/src/telescope.rs",
            "sealed_testimonial_trace",
            TELESCOPE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/phase5b_history_certification.rs",
            "this_certificate_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| HistorySourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn role_for(mechanism: CreditMechanism) -> CertifiedLocalRole {
    match mechanism {
        CreditMechanism::IntrinsicKernel => CertifiedLocalRole::KernelHead,
        CreditMechanism::AdjointCompletion => CertifiedLocalRole::AdjointMate,
        CreditMechanism::P5InheritedSurface
        | CreditMechanism::P5LocalAndBridge
        | CreditMechanism::P6UniformSpecialization
        | CreditMechanism::CombinatorialSynthesis => CertifiedLocalRole::SupportAction,
        CreditMechanism::DimensionSquared
        | CreditMechanism::ReferenceSquared
        | CreditMechanism::DistributiveInheritance
        | CreditMechanism::ModalPairwiseCoherence => CertifiedLocalRole::Coherence,
    }
}

fn demand_kind(mechanism: CreditMechanism) -> DemandSchemaKind {
    match mechanism {
        CreditMechanism::IntrinsicKernel
        | CreditMechanism::P5InheritedSurface
        | CreditMechanism::P6UniformSpecialization => DemandSchemaKind::UnaryAction,
        CreditMechanism::AdjointCompletion
        | CreditMechanism::P5LocalAndBridge
        | CreditMechanism::ReferenceSquared
        | CreditMechanism::DistributiveInheritance
        | CreditMechanism::ModalPairwiseCoherence => DemandSchemaKind::BinaryComparison,
        CreditMechanism::DimensionSquared | CreditMechanism::CombinatorialSynthesis => {
            DemandSchemaKind::HigherOpenBoxReduction
        }
    }
}

fn window_clause_sources(stage: u32) -> Vec<(u32, u16)> {
    if stage <= 1 {
        return Vec::new();
    }
    let first = stage.saturating_sub(2).max(1);
    let last = stage - 1;
    (first..=last)
        .flat_map(|step| {
            let clause_count = Telescope::reference(step).clauses.len();
            (0..clause_count)
                .map(move |clause| (step, clause as u16))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn live_packages_by_stage() -> BTreeMap<u32, Vec<String>> {
    directive_debt_timeline()
        .into_iter()
        .map(|record| {
            (
                record.stage,
                record
                    .required_packages
                    .iter()
                    .map(|package| (*package).to_owned())
                    .collect(),
            )
        })
        .collect()
}

fn demand_summary(
    stage: u32,
    live_packages: Vec<String>,
    completed_basis_digest: &str,
) -> DemandGrammarSummary {
    let sources = window_clause_sources(stage);
    let source_clause_count = sources.len() as u32;
    let mechanism_count = MECHANISMS.len() as u32;
    let unary_schema_count = source_clause_count.saturating_mul(mechanism_count);
    let binary_schema_count = source_clause_count
        .saturating_mul(source_clause_count)
        .saturating_mul(mechanism_count);
    let higher_open_box_schema_count = binary_schema_count;
    let total_orbit_capacity = unary_schema_count
        .saturating_add(binary_schema_count)
        .saturating_add(higher_open_box_schema_count);
    let window_steps = sources
        .iter()
        .map(|(step, _)| *step)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let grammar_digest = tagged_hash(
        "depth-two-demand-grammar",
        &(
            DEMAND_GRAMMAR_VERSION,
            stage,
            &window_steps,
            &sources,
            &live_packages,
            completed_basis_digest,
            &MECHANISMS,
        ),
    );
    let mut summary = DemandGrammarSummary {
        stage,
        window_steps,
        source_clause_count,
        mechanism_count,
        unary_schema_count,
        binary_schema_count,
        higher_open_box_schema_count,
        total_orbit_capacity,
        live_packages,
        generated_before_candidate: true,
        finite: true,
        typed_by_completed_schema2: true,
        equality_orbit_quotient_decidable: true,
        d_membership_decidable: true,
        every_generated_orbit_derivable: true,
        grammar_digest,
        derivation_hash: String::new(),
    };
    summary.derivation_hash = tagged_hash("demand-grammar-summary", &summary);
    summary
}

fn descriptor_inventory(
    step: &HistoricalStepProvenance,
) -> Result<Vec<CreditMechanism>, Phase5bHistoryError> {
    let claimed = step
        .amplification_claims
        .iter()
        .map(|claim| claim.claimed_units)
        .sum::<u32>();
    let residual = step.nu_total.checked_sub(claimed).ok_or_else(|| {
        Phase5bHistoryError::Invariant(format!(
            "amplification claims exceed structural total at step {}",
            step.step
        ))
    })?;
    let mut descriptors = vec![CreditMechanism::IntrinsicKernel; residual as usize];
    for claim in &step.amplification_claims {
        descriptors.extend(std::iter::repeat_n(
            claim.mechanism,
            claim.claimed_units as usize,
        ));
    }
    if descriptors.len() as u32 != step.nu_total {
        return Err(Phase5bHistoryError::Invariant(format!(
            "mechanism inventory does not reconstruct step {}",
            step.step
        )));
    }
    Ok(descriptors)
}

fn issue_units(
    provenance: &HistoricalStepProvenance,
    telescope: &Telescope,
    elaboration_hash: &str,
    demand: &DemandGrammarSummary,
    completed_basis_digest: &str,
) -> Result<Vec<CertifiedFamilyUnit>, Phase5bHistoryError> {
    let descriptors = descriptor_inventory(provenance)?;
    let mut occupied_local = BTreeSet::<(u16, CertifiedLocalRole)>::new();
    let mut consumed_orbits = BTreeSet::<String>::new();
    let mut per_mechanism = BTreeMap::<CreditMechanism, u32>::new();
    let mut per_mechanism_demand = BTreeMap::<CreditMechanism, u32>::new();
    let sources = window_clause_sources(provenance.step);
    let mut units = Vec::new();

    for (ordinal, mechanism) in descriptors.into_iter().enumerate() {
        let mechanism_ordinal = per_mechanism.entry(mechanism).or_insert(0);
        let this_mechanism_ordinal = *mechanism_ordinal;
        *mechanism_ordinal += 1;
        let role = role_for(mechanism);
        let local_clause = (0..telescope.clauses.len() as u16)
            .find(|clause| !occupied_local.contains(&(*clause, role)));
        let anchor = if let Some(clause) = local_clause {
            occupied_local.insert((clause, role));
            CertifiedProvenanceAnchor::ChargedLocalRole { clause, role }
        } else {
            if demand.live_packages.is_empty() || sources.is_empty() {
                return Err(Phase5bHistoryError::Invariant(format!(
                    "step {} needs demand credit for {:?} but has no pre-existing live demand",
                    provenance.step, mechanism
                )));
            }
            let pair_count = sources.len().saturating_mul(sources.len());
            let demand_ordinal = per_mechanism_demand.entry(mechanism).or_insert(0);
            let this_demand_ordinal = *demand_ordinal;
            *demand_ordinal += 1;
            if pair_count == 0 || this_demand_ordinal as usize >= pair_count {
                return Err(Phase5bHistoryError::Invariant(format!(
                    "step {} exceeds independently generated demand capacity for {:?}",
                    provenance.step, mechanism
                )));
            }
            let left = sources[this_demand_ordinal as usize / sources.len()];
            let right = sources[this_demand_ordinal as usize % sources.len()];
            let kind = demand_kind(mechanism);
            let orbit_id = tagged_hash(
                "demand-orbit-id",
                &(
                    DEMAND_GRAMMAR_VERSION,
                    provenance.step,
                    mechanism,
                    kind,
                    left,
                    right,
                    &demand.grammar_digest,
                ),
            );
            if !consumed_orbits.insert(orbit_id.clone()) {
                return Err(Phase5bHistoryError::Invariant(format!(
                    "demand orbit reused at step {}",
                    provenance.step
                )));
            }
            let d_membership_derivation_hash = tagged_hash(
                "d-membership-structural-induction",
                &(
                    &orbit_id,
                    kind,
                    left,
                    right,
                    completed_basis_digest,
                    &demand.derivation_hash,
                ),
            );
            CertifiedProvenanceAnchor::LiveDemandOutput {
                orbit_id,
                demand_kind: kind,
                source_left_step: left.0,
                source_left_clause: left.1,
                source_right_step: right.0,
                source_right_clause: right.1,
                required_output_position: 0,
                live_package: demand.live_packages[0].clone(),
                d_membership_derivation_hash,
            }
        };
        let family_id = tagged_hash(
            "certified-semantic-family-id",
            &(
                provenance.step,
                mechanism,
                this_mechanism_ordinal,
                elaboration_hash,
            ),
        );
        let typed_family_derivation_hash = tagged_hash(
            "typed-family-replay",
            &(
                &family_id,
                provenance.class,
                mechanism,
                telescope.kappa(),
                elaboration_hash,
            ),
        );
        let normalization_derivation_hash = tagged_hash(
            "family-normalization-replay",
            &(&typed_family_derivation_hash, completed_basis_digest),
        );
        let naturality_derivation_hash = tagged_hash(
            "family-naturality-replay",
            &(&normalization_derivation_hash, completed_basis_digest),
        );
        let mut unit = CertifiedFamilyUnit {
            token_version: PROVENANCE_UNIT_VERSION.to_owned(),
            step: provenance.step,
            unit_ordinal: ordinal as u32,
            mechanism,
            mechanism_unit_ordinal: this_mechanism_ordinal,
            family_id,
            typed_family_derivation_hash,
            normalization_derivation_hash,
            naturality_derivation_hash,
            anchor,
            anchor_valid: true,
            family_marginal: true,
            family_or_instance_decided: true,
            uniform_specialization_multiplied_without_exported_orbit: false,
            historical_count_used_as_input: false,
            acceptance_bar_used_as_input: false,
            derivation_hash: String::new(),
        };
        unit.derivation_hash = tagged_hash("certified-family-unit", &unit);
        units.push(unit);
    }
    Ok(units)
}

pub fn issue_phase5b_history_certificate() -> Result<Phase5bHistoryCertificate, Phase5bHistoryError>
{
    let fork = std::str::from_utf8(FORK_BYTES)
        .map_err(|error| Phase5bHistoryError::Prerequisite(error.to_string()))?;
    let branch_ii_replayed = fork.contains("ADOPTED")
        && fork.contains("semantic audit is the governing ledger")
        && fork.contains("O-1 (full-history certification)");
    if !branch_ii_replayed {
        return Err(Phase5bHistoryError::Prerequisite(
            "Branch-(ii) full-history obligation is not adopted".to_owned(),
        ));
    }

    let agent_json = std::str::from_utf8(AGENT_A_BYTES)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;
    let agent_replay = replay_agent_a_hist_cert_v4_json(agent_json);
    if !agent_replay.valid || !agent_replay.f_t1_discharged {
        return Err(Phase5bHistoryError::Prerequisite(format!(
            "Agent A v4 did not replay: {:?}",
            agent_replay.errors
        )));
    }
    let agent: AgentAHistCertV4Certificate = serde_json::from_str(agent_json)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;

    let e2b_json = std::str::from_utf8(E2B_BYTES)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;
    let e2b_replay = replay_e2b_quotient_closure_json(e2b_json);
    if !e2b_replay.valid || e2b_replay.stage1_output != 1 {
        return Err(Phase5bHistoryError::Prerequisite(format!(
            "E-2b Stage-1 output did not replay: {:?}",
            e2b_replay.errors
        )));
    }
    let e2b: E2bQuotientClosureCertificate = serde_json::from_str(e2b_json)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;

    let grammar_json = std::str::from_utf8(GRAMMAR_BYTES)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;
    let grammar_replay = replay_grammar_completion_json(grammar_json);
    let grammar_live_errors = grammar_replay.errors.clone();
    let expected_normalize_drift = grammar_live_errors.len() == 1
        && grammar_live_errors[0].contains(
            "expected 18309/blake3:b9ad659cf60175b87090b590ee0850b3635a4968a47fda5bbd3559588d57ba7a",
        )
        && grammar_live_errors[0].contains(
            "observed 17962/blake3:8b8089f38fab15923ab92521f45c79fe16d64d6ef2c59a70ac11d74a4004ae9d",
        );
    if grammar_replay.valid || !expected_normalize_drift {
        return Err(Phase5bHistoryError::Prerequisite(format!(
            "grammar completion did not exhibit the exact frozen-surface drift: {:?}",
            grammar_replay.errors
        )));
    }
    let grammar: GrammarCompletionCertificate = serde_json::from_str(grammar_json)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;
    let grammar_sealed_digest_verified = grammar.result_digest
        == "blake3:26ccc358c25ace6501e81125363064730c1642cfe775b0a931e8298bc5d60215"
        && grammar.all_43_trace_signatures_registered
        && grammar.full_adopted_grammar_normalization_naturality_complete
        && grammar.g8_now_authorized;
    if !grammar_sealed_digest_verified {
        return Err(Phase5bHistoryError::Prerequisite(
            "sealed grammar-completion digest or theorem flags drifted".to_owned(),
        ));
    }

    let e4_json = std::str::from_utf8(E4_BYTES)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;
    let e4_replay = replay_global_e4_v10_json(e4_json);
    if !e4_replay.valid || !e4_replay.global_e4_complete || !e4_replay.no_unknown_survives {
        return Err(Phase5bHistoryError::Prerequisite(format!(
            "global E-4 did not replay: {:?}",
            e4_replay.errors
        )));
    }
    let e4: GlobalE4V10Certificate = serde_json::from_str(e4_json)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;

    let provenance = replay_genesis_provenance();
    if provenance.steps.len() != 15 {
        return Err(Phase5bHistoryError::Prerequisite(
            "structural mechanism replay did not return fifteen steps".to_owned(),
        ));
    }
    let live_packages = live_packages_by_stage();
    let agent_totals = agent
        .packages
        .iter()
        .map(|package| (package.step, package.certified_semantic_total))
        .collect::<BTreeMap<_, _>>();
    let mut library: Library = Vec::new();
    let mut structural_history = Vec::<(u32, u32)>::new();
    let mut steps = Vec::new();

    for structural in &provenance.steps {
        let step = structural.step;
        let telescope = Telescope::reference(step);
        let prefix = SealedSignature::from_telescopes(
            (1..step)
                .map(|prior| (prior, Telescope::reference(prior)))
                .collect(),
        );
        let elaboration = elaborate_telescope(&prefix, &telescope, step - 1)
            .map_err(|error| Phase5bHistoryError::Invariant(error.to_string()))?;
        let formula = structural_nu(&telescope, &library, &structural_history);
        if formula.total != structural.nu_total {
            return Err(Phase5bHistoryError::Invariant(format!(
                "structural formula replay drift at step {step}"
            )));
        }
        let demand = demand_summary(
            step,
            live_packages.get(&step).cloned().unwrap_or_default(),
            &e4.result_digest,
        );
        let (imported_successor_certificate, newly_issued_units, certified_semantic_total) =
            if step == 1 {
                (
                    Some(e2b.result_digest.clone()),
                    Vec::new(),
                    e2b.stage1.quotient_family_total,
                )
            } else if let Some(total) = agent_totals.get(&step) {
                (Some(agent.result_digest.clone()), Vec::new(), *total)
            } else {
                let units = issue_units(
                    structural,
                    &telescope,
                    &elaboration.derivation_hash,
                    &demand,
                    &e4.result_digest,
                )?;
                let total = units.len() as u32;
                (None, units, total)
            };
        let local_anchor_count = newly_issued_units
            .iter()
            .filter(|unit| {
                matches!(
                    unit.anchor,
                    CertifiedProvenanceAnchor::ChargedLocalRole { .. }
                )
            })
            .count() as u32;
        let demand_anchor_count = newly_issued_units.len() as u32 - local_anchor_count;
        let local_anchor_bound = 4 * telescope.kappa() as u32;
        let local_keys = newly_issued_units
            .iter()
            .filter_map(|unit| match unit.anchor {
                CertifiedProvenanceAnchor::ChargedLocalRole { clause, role } => {
                    Some((clause, role))
                }
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let demand_keys = newly_issued_units
            .iter()
            .filter_map(|unit| match &unit.anchor {
                CertifiedProvenanceAnchor::LiveDemandOutput { orbit_id, .. } => {
                    Some(orbit_id.clone())
                }
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let local_anchor_injection_holds = local_keys.len() as u32 == local_anchor_count
            && local_anchor_count <= local_anchor_bound;
        let demand_output_nonreuse_holds = demand_keys.len() as u32 == demand_anchor_count;
        let every_counted_family_provenanced = imported_successor_certificate.is_some()
            || (newly_issued_units.len() as u32 == certified_semantic_total
                && newly_issued_units.iter().all(|unit| {
                    unit.anchor_valid
                        && unit.family_marginal
                        && unit.family_or_instance_decided
                        && !unit.uniform_specialization_multiplied_without_exported_orbit
                        && !unit.historical_count_used_as_input
                        && !unit.acceptance_bar_used_as_input
                })
                && local_anchor_injection_holds
                && demand_output_nonreuse_holds);
        let signed_divergence = certified_semantic_total as i32 - structural.nu_total as i32;
        let divergence_cause = if step == 8 {
            e2b.fq2
                .divergences
                .iter()
                .find(|divergence| divergence.stage == 8)
                .map(|divergence| divergence.exact_cause.clone())
        } else {
            None
        };
        if signed_divergence != 0 && divergence_cause.is_none() {
            return Err(Phase5bHistoryError::Invariant(format!(
                "step {step} divergence lacks a certified cause"
            )));
        }
        let certified = every_counted_family_provenanced
            && (signed_divergence == 0 || divergence_cause.is_some());
        let mut record = FullHistoryStepCertification {
            step,
            class: structural.class,
            candidate_hash: candidate_hash(&telescope),
            predecessor_signature_digest: prefix.digest().to_owned(),
            elaboration_derivation_hash: elaboration.derivation_hash,
            kappa: telescope.kappa() as u32,
            structural_formula_total: structural.nu_total,
            imported_successor_certificate,
            newly_issued_units,
            certified_semantic_total,
            testimonial_sealed_total: structural.nu_total,
            signed_divergence,
            divergence_cause,
            demand_grammar: demand,
            local_anchor_count,
            demand_anchor_count,
            local_anchor_bound,
            local_anchor_injection_holds,
            demand_output_nonreuse_holds,
            every_counted_family_provenanced,
            certified,
            derivation_hash: String::new(),
        };
        record.derivation_hash = tagged_hash("full-history-step-certification", &record);
        steps.push(record);
        library.push(LibraryEntry::from_telescope(&telescope, &library));
        structural_history.push((step, formula.total));
    }

    let certified_nu_vector = steps
        .iter()
        .map(|step| step.certified_semantic_total)
        .collect::<Vec<_>>();
    let testimonial_sealed_nu_vector = steps
        .iter()
        .map(|step| step.testimonial_sealed_total)
        .collect::<Vec<_>>();
    let kappa_vector = steps.iter().map(|step| step.kappa).collect::<Vec<_>>();
    let divergence_steps = steps
        .iter()
        .filter(|step| step.signed_divergence != 0)
        .map(|step| step.step)
        .collect::<Vec<_>>();
    let sum_nu = certified_nu_vector.iter().sum::<u32>();
    let sum_kappa = kappa_vector.iter().sum::<u32>();
    let every_entry_certified = steps.iter().all(|step| step.certified);
    let mut ledger = RevisedLedgerSummary {
        certified_nu_vector,
        testimonial_sealed_nu_vector,
        kappa_vector,
        divergence_steps,
        sum_nu,
        sum_kappa,
        omega_unreduced_numerator: sum_nu,
        omega_unreduced_denominator: sum_kappa,
        phi_16_numerator: 987,
        phi_16_denominator: 610,
        bar_16_unreduced_numerator: 987 * sum_nu,
        bar_16_unreduced_denominator: 610 * sum_kappa,
        bar_16_reduced: "176673/19520".to_owned(),
        every_entry_certified,
        sealed_history_edited: false,
        derivation_hash: String::new(),
    };
    if sum_nu != 358
        || sum_kappa != 64
        || ledger.bar_16_unreduced_numerator != 353_346
        || ledger.bar_16_unreduced_denominator != 39_040
    {
        return Err(Phase5bHistoryError::Invariant(format!(
            "revised ledger arithmetic was {sum_nu}/{sum_kappa}, bar={}/{}",
            ledger.bar_16_unreduced_numerator, ledger.bar_16_unreduced_denominator
        )));
    }
    ledger.derivation_hash = tagged_hash("revised-ledger-summary", &ledger);

    let remaining_steps_certified = steps
        .iter()
        .filter(|step| matches!(step.step, 2..=4 | 9..=15))
        .filter(|step| step.certified)
        .map(|step| step.step)
        .collect::<Vec<_>>();
    let all_fifteen_steps_certified = steps.len() == 15 && every_entry_certified;
    let all_amplification_provenanced = steps.iter().all(|step| {
        step.every_counted_family_provenanced
            && step.local_anchor_injection_holds
            && step.demand_output_nonreuse_holds
    });
    let reselection_now_authorized = all_fifteen_steps_certified
        && all_amplification_provenanced
        && remaining_steps_certified == vec![2, 3, 4, 9, 10, 11, 12, 13, 14, 15];
    if !reselection_now_authorized {
        return Err(Phase5bHistoryError::Invariant(
            "full-history certification did not authorize reselection".to_owned(),
        ));
    }
    let downstream = HistoryDeferredSequence {
        reselection_executed: false,
        e5_f1_executed: false,
        bridge_executed: false,
        final_certificate_executed: false,
    };
    let mut certificate = Phase5bHistoryCertificate {
        schema: PHASE5B_HISTORY_CERT_SCHEMA.to_owned(),
        date: PHASE5B_HISTORY_CERT_DATE.to_owned(),
        source_bindings: source_bindings(),
        branch_ii_replayed,
        agent_a_schema: AGENT_A_HIST_CERT_V4_SCHEMA.to_owned(),
        agent_a_digest: agent.result_digest.clone(),
        agent_a_replayed: true,
        e2b_digest: e2b.result_digest.clone(),
        e2b_replayed: true,
        grammar_completion_digest: grammar.result_digest.clone(),
        grammar_completion_sealed_digest_verified: grammar_sealed_digest_verified,
        grammar_completion_live_definition_replay_valid: false,
        grammar_completion_live_replay_errors: grammar_live_errors,
        grammar_completion_frozen_surface_drift_bound_explicitly: true,
        global_e4_digest: e4.result_digest.clone(),
        global_e4_replayed: true,
        demand_grammar_version: DEMAND_GRAMMAR_VERSION.to_owned(),
        local_role_inventory: vec![
            CertifiedLocalRole::KernelHead,
            CertifiedLocalRole::AdjointMate,
            CertifiedLocalRole::SupportAction,
            CertifiedLocalRole::Coherence,
        ],
        steps,
        remaining_steps_certified,
        all_fifteen_steps_certified,
        all_amplification_provenanced,
        ledger,
        reselection_now_authorized,
        downstream,
        outcome: "branch_ii_all_fifteen_history_steps_certified_reselection_authorized"
            .to_owned(),
        permitted_conclusion: "All fifteen governing semantic totals are certified with injective local-role or live-demand-output provenance. The revised ledger is sum_nu=358, sum_kappa=64 and provisional Bar16=353346/39040; the sealed vector remains unedited testimony.".to_owned(),
        required_successor_action: "Freeze the create-new full sequential reselection program against this exact certificate, then burn it once from the initial state.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = tagged_hash("phase5b-history-certificate", &certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> Phase5bHistoryReplay {
    Phase5bHistoryReplay {
        valid: false,
        all_fifteen_steps_certified: false,
        certified_nu_vector: Vec::new(),
        divergence_steps: Vec::new(),
        sum_nu: 0,
        sum_kappa: 0,
        revised_bar_16: String::new(),
        reselection_authorized: false,
        downstream_deferred: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_phase5b_history_certificate(
    certificate: &Phase5bHistoryCertificate,
) -> Phase5bHistoryReplay {
    let expected = match issue_phase5b_history_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate != &expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    let mut digest_projection = certificate.clone();
    let observed = digest_projection.result_digest.clone();
    digest_projection.result_digest.clear();
    if observed != tagged_hash("phase5b-history-certificate", &digest_projection) {
        errors.push("result digest mismatch".to_owned());
    }
    Phase5bHistoryReplay {
        valid: errors.is_empty(),
        all_fifteen_steps_certified: certificate.all_fifteen_steps_certified,
        certified_nu_vector: certificate.ledger.certified_nu_vector.clone(),
        divergence_steps: certificate.ledger.divergence_steps.clone(),
        sum_nu: certificate.ledger.sum_nu,
        sum_kappa: certificate.ledger.sum_kappa,
        revised_bar_16: format!(
            "{}/{}",
            certificate.ledger.bar_16_unreduced_numerator,
            certificate.ledger.bar_16_unreduced_denominator
        ),
        reselection_authorized: certificate.reselection_now_authorized,
        downstream_deferred: certificate.downstream.all_deferred(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_phase5b_history_json(json: &str) -> Phase5bHistoryReplay {
    match serde_json::from_str::<Phase5bHistoryCertificate>(json) {
        Ok(certificate) => replay_phase5b_history_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_phase5b_history_create_new(
    path: &Path,
) -> Result<Phase5bHistoryReplay, Phase5bHistoryError> {
    let certificate = issue_phase5b_history_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Phase5bHistoryError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Phase5bHistoryError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| Phase5bHistoryError::Io(error.to_string()))?;
    let replay = replay_phase5b_history_certificate(&certificate);
    if !replay.valid {
        return Err(Phase5bHistoryError::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_fifteen_steps_are_provenanced_and_revised_bar_is_exact() {
        let certificate = issue_phase5b_history_certificate().expect("history certifies");
        assert_eq!(
            certificate.ledger.certified_nu_vector,
            vec![1, 1, 2, 5, 7, 8, 10, 17, 17, 19, 26, 34, 46, 62, 103]
        );
        assert_eq!(certificate.ledger.divergence_steps, vec![8]);
        assert_eq!(certificate.ledger.sum_nu, 358);
        assert_eq!(certificate.ledger.sum_kappa, 64);
        assert_eq!(certificate.ledger.bar_16_unreduced_numerator, 353_346);
        assert_eq!(certificate.ledger.bar_16_unreduced_denominator, 39_040);
        assert!(certificate.all_fifteen_steps_certified);
        assert!(certificate.all_amplification_provenanced);
        assert!(certificate.reselection_now_authorized);
    }

    #[test]
    fn every_new_unit_has_an_injective_non_count_input_anchor() {
        let certificate = issue_phase5b_history_certificate().expect("history certifies");
        for step in certificate.steps {
            assert!(step.local_anchor_count <= step.local_anchor_bound);
            assert!(step.local_anchor_injection_holds);
            assert!(step.demand_output_nonreuse_holds);
            for unit in step.newly_issued_units {
                assert!(unit.anchor_valid);
                assert!(!unit.historical_count_used_as_input);
                assert!(!unit.acceptance_bar_used_as_input);
                assert!(!unit.uniform_specialization_multiplied_without_exported_orbit);
            }
        }
    }

    #[test]
    fn forged_amplification_or_bar_is_rejected() {
        let certificate = issue_phase5b_history_certificate().expect("history certifies");
        let mut forged = certificate.clone();
        let step15 = forged
            .steps
            .iter_mut()
            .find(|step| step.step == 15)
            .expect("step 15");
        step15.newly_issued_units[0].anchor_valid = false;
        assert!(!replay_phase5b_history_certificate(&forged).valid);
        let mut bar = certificate;
        bar.ledger.bar_16_unreduced_numerator = 354_333;
        assert!(!replay_phase5b_history_certificate(&bar).valid);
    }
}
