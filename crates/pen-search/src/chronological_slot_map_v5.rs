//! F-SC1 chronological successor over canonical support comprehension.
//!
//! The 71 v4-derived rows remain literal v4 rows.  Only the unique named
//! cycle is replaced, and that replacement is admitted only after the
//! canonical support-comprehension derivation replays.  Consequently the
//! regression checks compare serialized row bytes as well as evidence and
//! row hashes, rather than merely comparing counts.

use crate::chronological_slot_map_v4::{
    ChronologicalCaseDispositionV4, ChronologicalCaseJoinV4, ChronologicalSlotMapV4Certificate,
    issue_chronological_slot_map_v4_certificate, replay_chronological_slot_map_v4_certificate,
};
use crate::support_comprehension_hardening_v6::{
    SupportComprehensionHardeningV6Token, issue_support_comprehension_hardening_v6,
    replay_support_comprehension_hardening_v6,
};
use crate::support_comprehension_v5::{
    SupportComprehensionDerivationV5, issue_support_comprehension_derivation_v5,
    replay_support_comprehension_derivation_v5,
};
use pen_core::hash::blake3_hex;
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

pub const CHRONOLOGICAL_SLOT_MAP_V5_SCHEMA: &str =
    "chronological-interface-slot-map-support-comprehension-v5";
pub const CHRONOLOGICAL_SLOT_MAP_V5_DATE: &str = "2026-07-22";
pub const SUPPORT_COMPREHENSION_CASE_ID: &str =
    "blake3:4c9c4272b11cb6aec0c332cd6697401ccd3f95b8bfa2520b6b6dceb6986d3571";

const EXPECTED_PREVIOUSLY_DERIVED: usize = 71;
const EXPECTED_SEALED: usize = 72;
const EXPECTED_FORMER: usize = 9;

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CHRONOLOGICAL_SLOT_MAP_V5_SCHEMA, domain, value))
        .expect("chronological v5 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn raw_bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn is_derived(disposition: &ChronologicalCaseDispositionV4) -> bool {
    matches!(disposition, ChronologicalCaseDispositionV4::Derived { .. })
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InheritedRowEqualityV5 {
    pub instance_id: String,
    pub baseline_row_hash: String,
    pub successor_row_hash: String,
    pub baseline_evidence_hash: String,
    pub successor_evidence_hash: String,
    pub baseline_serialized_bytes_hash: String,
    pub successor_serialized_bytes_hash: String,
    pub exact_row_equality: bool,
    pub exact_evidence_hash_equality: bool,
    pub exact_serialized_byte_equality: bool,
    pub equality_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapV5Certificate {
    pub schema: String,
    pub date: String,
    pub baseline_v4_result_digest: String,
    pub baseline_v4_replayed: bool,
    pub baseline_v4_derived_count: usize,
    pub baseline_v4_named_gap_count: usize,
    pub baseline_exact_54_plus_18_partition: bool,
    pub baseline_exact_sealed_72_id_join: bool,
    pub support_comprehension: SupportComprehensionDerivationV5,
    pub support_comprehension_replayed: bool,
    pub support_comprehension_hardening: SupportComprehensionHardeningV6Token,
    pub support_graph_normal_form_proved: bool,
    pub support_zero_mint_capability_proved: bool,
    pub support_comprehension_case_id: String,
    pub support_replacement_evidence_hash: String,
    pub support_replacement_evidence_bound: bool,
    pub cases: Vec<ChronologicalCaseJoinV4>,
    pub inherited_row_equalities: Vec<InheritedRowEqualityV5>,
    pub inherited_row_equality_count: usize,
    pub prior_71_baseline_bytes_hash: String,
    pub prior_71_successor_bytes_hash: String,
    pub prior_71_rows_byte_identical: bool,
    pub prior_71_evidence_hashes_identical: bool,
    pub prior_71_row_hashes_identical: bool,
    pub changed_case_ids: Vec<String>,
    pub exactly_one_case_changed: bool,
    pub sealed_discharge_count: usize,
    pub sealed_discharge_derived_count: usize,
    pub sealed_discharge_named_gap_count: usize,
    pub former_expected_instance_ids: Vec<String>,
    pub former_derived_instance_ids: Vec<String>,
    pub former_named_gap_instance_ids: Vec<String>,
    pub former_nine_exact: bool,
    pub support_parameter_kappa_charge: u32,
    pub support_parameter_nu_charge: u32,
    pub support_parameter_anchors_minted: u32,
    pub support_parameter_demand_orbits_minted: u32,
    pub realizer_kappa_charge: u32,
    pub realizer_nu_charge: u32,
    pub realizer_anchors_minted: u32,
    pub realizer_demand_orbits_minted: u32,
    pub f_sc3_zero_accounting_passed: bool,
    pub t_sm1b_surface_count: usize,
    pub t_sm1b_derived_count: usize,
    pub t_sm1b_named_gap_count: usize,
    pub f_sc1_regression_passed: bool,
    pub fixed_positive_gate_passed: bool,
    pub bi0_chronological_prerequisite_reopened: bool,
    pub outcome: String,
    pub conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ChronologicalSlotMapV5Error {
    #[error("v4 baseline failed: {0}")]
    Baseline(String),
    #[error("support-comprehension derivation failed: {0}")]
    Support(String),
    #[error("F-SC1 join failed: {0}")]
    Join(String),
    #[error("chronological v5 replay mismatch")]
    ReplayMismatch,
}

fn evidence_hash(row: &ChronologicalCaseJoinV4) -> Option<&str> {
    match &row.disposition {
        ChronologicalCaseDispositionV4::Derived { evidence_hash } => Some(evidence_hash),
        ChronologicalCaseDispositionV4::NamedBlocker { .. } => None,
    }
}

fn certificate_digest(certificate: &ChronologicalSlotMapV5Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn inherited_equality(
    baseline: &ChronologicalCaseJoinV4,
    successor: &ChronologicalCaseJoinV4,
) -> Result<InheritedRowEqualityV5, ChronologicalSlotMapV5Error> {
    let baseline_bytes = serde_json::to_vec(baseline)
        .map_err(|error| ChronologicalSlotMapV5Error::Join(error.to_string()))?;
    let successor_bytes = serde_json::to_vec(successor)
        .map_err(|error| ChronologicalSlotMapV5Error::Join(error.to_string()))?;
    let baseline_evidence_hash = evidence_hash(baseline)
        .ok_or_else(|| {
            ChronologicalSlotMapV5Error::Join("inherited F-SC1 row was not v4-derived".to_owned())
        })?
        .to_owned();
    let successor_evidence_hash = evidence_hash(successor)
        .ok_or_else(|| {
            ChronologicalSlotMapV5Error::Join("inherited F-SC1 row ceased to be derived".to_owned())
        })?
        .to_owned();
    let mut equality = InheritedRowEqualityV5 {
        instance_id: baseline.instance_id.clone(),
        baseline_row_hash: baseline.row_hash.clone(),
        successor_row_hash: successor.row_hash.clone(),
        baseline_evidence_hash: baseline_evidence_hash.clone(),
        successor_evidence_hash: successor_evidence_hash.clone(),
        baseline_serialized_bytes_hash: raw_bytes_hash(&baseline_bytes),
        successor_serialized_bytes_hash: raw_bytes_hash(&successor_bytes),
        exact_row_equality: baseline == successor,
        exact_evidence_hash_equality: baseline_evidence_hash == successor_evidence_hash,
        exact_serialized_byte_equality: baseline_bytes == successor_bytes,
        equality_hash: String::new(),
    };
    equality.equality_hash = tagged_hash("inherited-row-equality", &equality);
    Ok(equality)
}

/// Reissue v4, preserve its 71 positive rows literally, and replace only the
/// unique cycle after the support-comprehension theorem replays.
pub fn issue_chronological_slot_map_v5_certificate()
-> Result<ChronologicalSlotMapV5Certificate, ChronologicalSlotMapV5Error> {
    let baseline = issue_chronological_slot_map_v4_certificate()
        .map_err(|error| ChronologicalSlotMapV5Error::Baseline(error.to_string()))?;
    let baseline_replay = replay_chronological_slot_map_v4_certificate(&baseline);
    if !baseline_replay.valid {
        return Err(ChronologicalSlotMapV5Error::Baseline(
            baseline_replay.errors.join("; "),
        ));
    }
    let baseline_v4_replayed = true;
    let baseline_v4_derived_count = baseline
        .cases
        .iter()
        .filter(|row| is_derived(&row.disposition))
        .count();
    let baseline_v4_named_gap_count = baseline.cases.len() - baseline_v4_derived_count;
    if baseline_v4_derived_count != EXPECTED_PREVIOUSLY_DERIVED
        || baseline_v4_named_gap_count != 1
        || !baseline.exact_54_plus_18_partition
        || !baseline.exact_sealed_72_id_join
    {
        return Err(ChronologicalSlotMapV5Error::Baseline(format!(
            "v4 is not the fixed partitioned 71-derived/1-cycle surface: derived={baseline_v4_derived_count}, gaps={baseline_v4_named_gap_count}, partition={}, sealed_join={}",
            baseline.exact_54_plus_18_partition, baseline.exact_sealed_72_id_join,
        )));
    }
    let baseline_exact_54_plus_18_partition = baseline.exact_54_plus_18_partition;
    let baseline_exact_sealed_72_id_join = baseline.exact_sealed_72_id_join;

    let support_comprehension = issue_support_comprehension_derivation_v5()
        .map_err(|error| ChronologicalSlotMapV5Error::Support(error.to_string()))?;
    replay_support_comprehension_derivation_v5(&support_comprehension)
        .map_err(|error| ChronologicalSlotMapV5Error::Support(error.to_string()))?;
    let support_comprehension_replayed = true;
    let support_comprehension_hardening =
        issue_support_comprehension_hardening_v6(&support_comprehension)
            .map_err(|error| ChronologicalSlotMapV5Error::Support(error.to_string()))?;
    replay_support_comprehension_hardening_v6(
        &support_comprehension,
        &support_comprehension_hardening,
    )
    .map_err(|error| ChronologicalSlotMapV5Error::Support(error.to_string()))?;
    let support_graph_normal_form_proved = support_comprehension_hardening.graph_normal_form.proved;
    let support_zero_mint_capability_proved =
        support_comprehension_hardening.zero_mint_capability.proved;
    let support_comprehension_case_id = support_comprehension
        .canonical_analysis
        .sealed_instance
        .instance_id
        .clone();
    if support_comprehension_case_id != SUPPORT_COMPREHENSION_CASE_ID {
        return Err(ChronologicalSlotMapV5Error::Join(format!(
            "canonical analysis selected unexpected instance {support_comprehension_case_id}"
        )));
    }

    // Clone first: this is what makes all inherited row bytes, row hashes,
    // and evidence hashes definitionally fixed.  The unique negative row is
    // then replaced in place.
    let mut cases = baseline.cases.clone();
    let repaired = cases
        .iter_mut()
        .find(|row| row.instance_id == support_comprehension_case_id)
        .ok_or_else(|| {
            ChronologicalSlotMapV5Error::Join(
                "support-comprehension instance is absent from the fixed 72".to_owned(),
            )
        })?;
    match &repaired.disposition {
        ChronologicalCaseDispositionV4::NamedBlocker { blocker, .. }
            if blocker == "T_SM1B_DEPENDENT_TARGET_CYCLE" => {}
        _ => {
            return Err(ChronologicalSlotMapV5Error::Join(
                "the unique replacement row is not the exact inherited cycle".to_owned(),
            ));
        }
    }
    let support_replacement_evidence_hash = tagged_hash(
        "support-comprehension-portable-derived-evidence",
        &(
            &support_comprehension.specialization.derivation_hash,
            &support_comprehension.derivation_hash,
            &support_comprehension_hardening.derivation_hash,
        ),
    );
    repaired.disposition = ChronologicalCaseDispositionV4::Derived {
        evidence_hash: support_replacement_evidence_hash.clone(),
    };
    repaired.row_hash = tagged_hash("support-comprehension-replacement-row", repaired);
    let support_replacement_evidence_bound = matches!(
        &repaired.disposition,
        ChronologicalCaseDispositionV4::Derived { evidence_hash }
            if evidence_hash == &support_replacement_evidence_hash
    );

    let baseline_derived_rows = baseline
        .cases
        .iter()
        .filter(|row| is_derived(&row.disposition))
        .cloned()
        .collect::<Vec<_>>();
    let baseline_derived_ids = baseline_derived_rows
        .iter()
        .map(|row| row.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let successor_inherited_rows = cases
        .iter()
        .filter(|row| baseline_derived_ids.contains(&row.instance_id))
        .cloned()
        .collect::<Vec<_>>();
    let baseline_bytes = serde_json::to_vec(&baseline_derived_rows)
        .map_err(|error| ChronologicalSlotMapV5Error::Join(error.to_string()))?;
    let successor_bytes = serde_json::to_vec(&successor_inherited_rows)
        .map_err(|error| ChronologicalSlotMapV5Error::Join(error.to_string()))?;
    let prior_71_baseline_bytes_hash = raw_bytes_hash(&baseline_bytes);
    let prior_71_successor_bytes_hash = raw_bytes_hash(&successor_bytes);

    let mut inherited_row_equalities = Vec::new();
    for baseline_row in &baseline_derived_rows {
        let successor_row = successor_inherited_rows
            .iter()
            .find(|row| row.instance_id == baseline_row.instance_id)
            .ok_or_else(|| {
                ChronologicalSlotMapV5Error::Join(format!(
                    "inherited row {} disappeared",
                    baseline_row.instance_id
                ))
            })?;
        inherited_row_equalities.push(inherited_equality(baseline_row, successor_row)?);
    }
    inherited_row_equalities.sort_by(|left, right| left.instance_id.cmp(&right.instance_id));
    let inherited_row_equality_count = inherited_row_equalities.len();
    let prior_71_rows_byte_identical = baseline_bytes == successor_bytes
        && inherited_row_equality_count == EXPECTED_PREVIOUSLY_DERIVED
        && inherited_row_equalities
            .iter()
            .all(|equality| equality.exact_row_equality && equality.exact_serialized_byte_equality);
    let prior_71_evidence_hashes_identical = inherited_row_equalities
        .iter()
        .all(|equality| equality.exact_evidence_hash_equality);
    let prior_71_row_hashes_identical = inherited_row_equalities
        .iter()
        .all(|equality| equality.baseline_row_hash == equality.successor_row_hash);

    let changed_case_ids = baseline
        .cases
        .iter()
        .zip(cases.iter())
        .filter(|(before, after)| before != after)
        .map(|(_, after)| after.instance_id.clone())
        .collect::<Vec<_>>();
    let exactly_one_case_changed = changed_case_ids == vec![support_comprehension_case_id.clone()];
    let sealed_discharge_count = cases
        .iter()
        .filter(|row| row.independently_sealed_discharge)
        .count();
    let sealed_discharge_derived_count = cases
        .iter()
        .filter(|row| row.independently_sealed_discharge && is_derived(&row.disposition))
        .count();
    let sealed_discharge_named_gap_count = sealed_discharge_count - sealed_discharge_derived_count;

    let former_expected_set = baseline
        .former_expected_instance_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let former_expected_instance_ids = former_expected_set.iter().cloned().collect::<Vec<_>>();
    let former_derived_instance_ids = cases
        .iter()
        .filter(|row| row.former_gap_instance && is_derived(&row.disposition))
        .map(|row| row.instance_id.clone())
        .collect::<Vec<_>>();
    let former_named_gap_instance_ids = cases
        .iter()
        .filter(|row| row.former_gap_instance && !is_derived(&row.disposition))
        .map(|row| row.instance_id.clone())
        .collect::<Vec<_>>();
    let former_nine_exact = former_expected_instance_ids.len() == EXPECTED_FORMER
        && former_derived_instance_ids == former_expected_instance_ids
        && former_named_gap_instance_ids.is_empty();

    let support_parameter_kappa_charge = support_comprehension.support_parameters_kappa_charge;
    let support_parameter_nu_charge = support_comprehension.support_parameters_nu_charge;
    let support_parameter_anchors_minted = support_comprehension.support_parameters_anchors_minted;
    let support_parameter_demand_orbits_minted =
        support_comprehension.support_parameters_demand_orbits_minted;
    let realizer_kappa_charge = support_comprehension.realizers_kappa_charge;
    let realizer_nu_charge = support_comprehension.realizers_nu_charge;
    let realizer_anchors_minted = support_comprehension.realizers_anchors_minted;
    let realizer_demand_orbits_minted = support_comprehension.realizers_demand_orbits_minted;
    let f_sc3_zero_accounting_passed = support_comprehension.zero_accounting_proved
        && support_parameter_kappa_charge == 0
        && support_parameter_nu_charge == 0
        && support_parameter_anchors_minted == 0
        && support_parameter_demand_orbits_minted == 0
        && realizer_kappa_charge == 0
        && realizer_nu_charge == 0
        && realizer_anchors_minted == 0
        && realizer_demand_orbits_minted == 0;
    let t_sm1b_surface_count = cases
        .iter()
        .filter(|row| row.theorem_component == "T-SM1b")
        .count();
    let t_sm1b_derived_count = cases
        .iter()
        .filter(|row| row.theorem_component == "T-SM1b" && is_derived(&row.disposition))
        .count();
    let t_sm1b_named_gap_count = t_sm1b_surface_count - t_sm1b_derived_count;
    let f_sc1_regression_passed = baseline_v4_replayed
        && support_comprehension_replayed
        && support_graph_normal_form_proved
        && support_zero_mint_capability_proved
        && support_replacement_evidence_bound
        && prior_71_rows_byte_identical
        && prior_71_evidence_hashes_identical
        && prior_71_row_hashes_identical
        && baseline_exact_54_plus_18_partition
        && baseline_exact_sealed_72_id_join
        && exactly_one_case_changed
        && sealed_discharge_count == EXPECTED_SEALED
        && sealed_discharge_derived_count == EXPECTED_SEALED
        && sealed_discharge_named_gap_count == 0
        && former_nine_exact
        && f_sc3_zero_accounting_passed;
    let fixed_positive_gate_passed = f_sc1_regression_passed
        && t_sm1b_derived_count == t_sm1b_surface_count
        && t_sm1b_surface_count == 18
        && t_sm1b_named_gap_count == 0;
    if !fixed_positive_gate_passed {
        return Err(ChronologicalSlotMapV5Error::Join(
            "the support-comprehension successor did not pass the fixed positive gate".to_owned(),
        ));
    }

    let mut certificate = ChronologicalSlotMapV5Certificate {
        schema: CHRONOLOGICAL_SLOT_MAP_V5_SCHEMA.to_owned(),
        date: CHRONOLOGICAL_SLOT_MAP_V5_DATE.to_owned(),
        baseline_v4_result_digest: baseline.result_digest,
        baseline_v4_replayed,
        baseline_v4_derived_count,
        baseline_v4_named_gap_count,
        baseline_exact_54_plus_18_partition,
        baseline_exact_sealed_72_id_join,
        support_comprehension,
        support_comprehension_replayed,
        support_comprehension_hardening,
        support_graph_normal_form_proved,
        support_zero_mint_capability_proved,
        support_comprehension_case_id,
        support_replacement_evidence_hash,
        support_replacement_evidence_bound,
        cases,
        inherited_row_equalities,
        inherited_row_equality_count,
        prior_71_baseline_bytes_hash,
        prior_71_successor_bytes_hash,
        prior_71_rows_byte_identical,
        prior_71_evidence_hashes_identical,
        prior_71_row_hashes_identical,
        changed_case_ids,
        exactly_one_case_changed,
        sealed_discharge_count,
        sealed_discharge_derived_count,
        sealed_discharge_named_gap_count,
        former_expected_instance_ids,
        former_derived_instance_ids,
        former_named_gap_instance_ids,
        former_nine_exact,
        support_parameter_kappa_charge,
        support_parameter_nu_charge,
        support_parameter_anchors_minted,
        support_parameter_demand_orbits_minted,
        realizer_kappa_charge,
        realizer_nu_charge,
        realizer_anchors_minted,
        realizer_demand_orbits_minted,
        f_sc3_zero_accounting_passed,
        t_sm1b_surface_count,
        t_sm1b_derived_count,
        t_sm1b_named_gap_count,
        f_sc1_regression_passed,
        fixed_positive_gate_passed,
        bi0_chronological_prerequisite_reopened: fixed_positive_gate_passed,
        outcome: "F_SC1_SUPPORT_COMPREHENSION_FIXED_GATE_PASSED".to_owned(),
        conclusion: "The canonical comprehension derives the unique v4 cycle. All 71 inherited rows remain byte-, row-hash-, and evidence-hash-identical; the fixed chronological gate is 72/72 and the former-gap gate is 9/9. This reopens only the chronological prerequisite of BI-0.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn replay_chronological_slot_map_v5_certificate(
    claimed: &ChronologicalSlotMapV5Certificate,
) -> Result<(), ChronologicalSlotMapV5Error> {
    if claimed.result_digest != certificate_digest(claimed) {
        return Err(ChronologicalSlotMapV5Error::ReplayMismatch);
    }
    let reissued = issue_chronological_slot_map_v5_certificate()?;
    if reissued == *claimed {
        Ok(())
    } else {
        Err(ChronologicalSlotMapV5Error::ReplayMismatch)
    }
}

/// The inherited v4 certificate is included only through its replayed
/// digest and literal rows.  This helper makes the intended compatibility
/// boundary visible to downstream join code.
pub fn baseline_v4_for_v5() -> Result<ChronologicalSlotMapV4Certificate, ChronologicalSlotMapV5Error>
{
    issue_chronological_slot_map_v4_certificate()
        .map_err(|error| ChronologicalSlotMapV5Error::Baseline(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_sc1_preserves_all_seventy_one_rows_and_closes_the_fixed_gate() {
        let certificate = issue_chronological_slot_map_v5_certificate().expect("chronological v5");
        assert_eq!(certificate.inherited_row_equality_count, 71);
        assert!(certificate.prior_71_rows_byte_identical);
        assert!(certificate.prior_71_evidence_hashes_identical);
        assert!(certificate.prior_71_row_hashes_identical);
        assert_eq!(
            certificate.changed_case_ids,
            vec![SUPPORT_COMPREHENSION_CASE_ID]
        );
        assert_eq!(certificate.sealed_discharge_derived_count, 72);
        assert_eq!(certificate.sealed_discharge_named_gap_count, 0);
        assert_eq!(certificate.former_derived_instance_ids.len(), 9);
        assert!(certificate.former_nine_exact);
        assert!(
            certificate
                .support_comprehension_hardening
                .base_derivation_replayed
        );
        assert!(certificate.support_graph_normal_form_proved);
        assert!(certificate.support_zero_mint_capability_proved);
        assert!(certificate.support_replacement_evidence_bound);
        assert!(certificate.f_sc3_zero_accounting_passed);
        assert!(certificate.fixed_positive_gate_passed);
        replay_chronological_slot_map_v5_certificate(&certificate).expect("v5 replay");
        println!(
            "digest={} inherited={}/71 sealed={}/{} former={}/{} t_sm1b={}/{} zero={} gate={}",
            certificate.result_digest,
            certificate.inherited_row_equality_count,
            certificate.sealed_discharge_derived_count,
            certificate.sealed_discharge_count,
            certificate.former_derived_instance_ids.len(),
            certificate.former_expected_instance_ids.len(),
            certificate.t_sm1b_derived_count,
            certificate.t_sm1b_surface_count,
            certificate.f_sc3_zero_accounting_passed,
            certificate.fixed_positive_gate_passed,
        );
    }

    #[test]
    fn inherited_row_and_zero_charge_mutations_fail_replay() {
        let certificate = issue_chronological_slot_map_v5_certificate().expect("chronological v5");
        let mut row_mutation = certificate.clone();
        let inherited = row_mutation
            .cases
            .iter_mut()
            .find(|row| row.instance_id != SUPPORT_COMPREHENSION_CASE_ID)
            .expect("inherited row");
        inherited.row_hash.push_str("-mutated");
        row_mutation.result_digest = certificate_digest(&row_mutation);
        assert!(replay_chronological_slot_map_v5_certificate(&row_mutation).is_err());

        let mut replacement_mutation = certificate.clone();
        {
            let replacement = replacement_mutation
                .cases
                .iter_mut()
                .find(|row| row.instance_id == SUPPORT_COMPREHENSION_CASE_ID)
                .expect("replacement row");
            replacement.disposition = ChronologicalCaseDispositionV4::Derived {
                evidence_hash: "blake3:forged-specialization-only".to_owned(),
            };
            replacement.row_hash =
                tagged_hash("support-comprehension-replacement-row", replacement);
        }
        replacement_mutation.result_digest = certificate_digest(&replacement_mutation);
        assert!(replay_chronological_slot_map_v5_certificate(&replacement_mutation).is_err());

        let mut charge_mutation = certificate;
        charge_mutation.realizer_nu_charge = 1;
        charge_mutation.result_digest = certificate_digest(&charge_mutation);
        assert!(replay_chronological_slot_map_v5_certificate(&charge_mutation).is_err());
    }
}
