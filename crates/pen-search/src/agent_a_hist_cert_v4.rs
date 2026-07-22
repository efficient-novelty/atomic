//! Agent A HIST-CERT v4 under adopted Phase-5b Branch (ii).
//!
//! The certificate consumes, but does not reinterpret, HIST-CERT v3 path
//! evidence and the E-2b ordinary-family/Generated-instance handoff.  Under
//! Branch (ii), F-T1 asks for certified semantic totals plus exact divergence
//! records, not blind equality with the sealed operational inventory.

use crate::e2b_quotient_closure::{
    E2B_QUOTIENT_CLOSURE_SCHEMA, E2bQuotientClosureCertificate, replay_e2b_quotient_closure_json,
};
use pen_core::hash::blake3_hex;
use pen_eval::tdc1_hist_cert_v3::{
    HIST_CERT_V3_SCHEMA, HistCertV3Certificate, replay_hist_cert_v3_json,
};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const AGENT_A_HIST_CERT_V4_SCHEMA: &str = "hist-cert-historical-hit-v4-branch-ii";
pub const AGENT_A_HIST_CERT_V4_DATE: &str = "2026-07-21";

const E2B_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e2b_quotient_closure_v1.json");
const HIST_V3_BYTES: &[u8] = include_bytes!("../../../docs/hist_cert_v3.json");
const FORK_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_fork_adjudication.md");
const AGENT_A_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_a_hist_cert_plan.md");
const E2B_SOURCE_BYTES: &[u8] = include_bytes!("e2b_quotient_closure.rs");
const HIST_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/tdc1_hist_cert_v3.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("agent_a_hist_cert_v4.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentASourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentAPackageCertification {
    pub step: u32,
    pub label: String,
    pub sealed_operational_total: u32,
    pub certified_path_family_count: u32,
    pub certified_independent_ordinary_family_count: u32,
    pub certified_generated_instance_count: u32,
    pub certified_semantic_total: u32,
    pub operational_row_coverage_total: u32,
    pub operational_inventory_fully_covered: bool,
    pub semantic_total_matches_seal: bool,
    pub signed_divergence: i32,
    pub divergence_cause: Option<String>,
    pub every_path_family_typed_and_marginal: bool,
    pub every_ordinary_row_typed_normalized_natural_and_resolved: bool,
    pub generated_instance_not_multiplied: bool,
    pub full_certified_total_issued: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FT1BranchIiAudit {
    pub falsifier: String,
    pub branch_semantics: String,
    pub certified_steps: Vec<u32>,
    pub certified_semantic_totals: Vec<u32>,
    pub sealed_testimonial_totals: Vec<u32>,
    pub certified_divergence_steps: Vec<u32>,
    pub all_registered_path_families_certified: bool,
    pub all_ordinary_rows_quotient_resolved: bool,
    pub every_difference_has_exact_divergence_record: bool,
    pub blind_reproduction_of_seal_required: bool,
    pub f_t1_discharged: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentADeferredSequence {
    pub remaining_history_certification_executed: bool,
    pub reselection_executed: bool,
    pub e5_f1_executed: bool,
    pub bridge_executed: bool,
    pub final_certificate_executed: bool,
}

impl AgentADeferredSequence {
    fn all_deferred(&self) -> bool {
        !self.remaining_history_certification_executed
            && !self.reselection_executed
            && !self.e5_f1_executed
            && !self.bridge_executed
            && !self.final_certificate_executed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentAHistCertV4Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<AgentASourceBinding>,
    pub branch_ii_adoption_replayed: bool,
    pub e2b_schema: String,
    pub e2b_digest: String,
    pub e2b_replayed: bool,
    pub hist_v3_schema: String,
    pub hist_v3_digest: String,
    pub hist_v3_replayed: bool,
    pub e2b_handoff_consumed_exactly_once: bool,
    pub packages: Vec<AgentAPackageCertification>,
    pub f_t1: FT1BranchIiAudit,
    pub full_certified_totals: Vec<u32>,
    pub f_t1_discharged: bool,
    pub downstream: AgentADeferredSequence,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentAHistCertV4Replay {
    pub valid: bool,
    pub package_steps: Vec<u32>,
    pub certified_totals: Vec<u32>,
    pub divergence_steps: Vec<u32>,
    pub f_t1_discharged: bool,
    pub downstream_deferred: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum AgentAHistCertV4Error {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(AGENT_A_HIST_CERT_V4_SCHEMA, domain, value))
        .expect("Agent A v4 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<AgentASourceBinding> {
    [
        (
            "docs/schema2_e2b_quotient_closure_v1.json",
            "ordinary_family_and_generated_instance_handoff",
            E2B_BYTES,
        ),
        (
            "docs/hist_cert_v3.json",
            "typed_marginal_historical_path_families",
            HIST_V3_BYTES,
        ),
        (
            "docs/phase5b_fork_adjudication.md",
            "adopted_branch_ii_ft1_semantics",
            FORK_BYTES,
        ),
        (
            "docs/agent_a_hist_cert_plan.md",
            "archival_agent_a_scope_and_falsifier",
            AGENT_A_PLAN_BYTES,
        ),
        (
            "crates/pen-search/src/e2b_quotient_closure.rs",
            "e2b_handoff_issuer",
            E2B_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/tdc1_hist_cert_v3.rs",
            "path_certificate_issuer",
            HIST_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/agent_a_hist_cert_v4.rs",
            "this_successor_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| AgentASourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

pub fn issue_agent_a_hist_cert_v4() -> Result<AgentAHistCertV4Certificate, AgentAHistCertV4Error> {
    let fork = std::str::from_utf8(FORK_BYTES)
        .map_err(|error| AgentAHistCertV4Error::Prerequisite(error.to_string()))?;
    let branch_ii_adoption_replayed = fork.contains("ADOPTED")
        && fork.contains("Branch (ii)")
        && fork.contains("semantic audit is the governing ledger")
        && fork.contains("certified semantic totals plus certified divergence records");
    if !branch_ii_adoption_replayed {
        return Err(AgentAHistCertV4Error::Prerequisite(
            "Branch-(ii) adoption markers are missing".to_owned(),
        ));
    }

    let e2b_json = std::str::from_utf8(E2B_BYTES)
        .map_err(|error| AgentAHistCertV4Error::Json(error.to_string()))?;
    let e2b_replay = replay_e2b_quotient_closure_json(e2b_json);
    if !e2b_replay.valid || !e2b_replay.e2b_complete || !e2b_replay.agent_a_handoff_issued {
        return Err(AgentAHistCertV4Error::Prerequisite(format!(
            "E-2b handoff did not replay: {:?}",
            e2b_replay.errors
        )));
    }
    let e2b: E2bQuotientClosureCertificate = serde_json::from_str(e2b_json)
        .map_err(|error| AgentAHistCertV4Error::Json(error.to_string()))?;
    if e2b.agent_a_handoff.agent_a_executed || e2b.agent_a_handoff.f_t1_evaluated {
        return Err(AgentAHistCertV4Error::Invariant(
            "E-2b predecessor already claims Agent A execution".to_owned(),
        ));
    }

    let hist_json = std::str::from_utf8(HIST_V3_BYTES)
        .map_err(|error| AgentAHistCertV4Error::Json(error.to_string()))?;
    let hist_replay = replay_hist_cert_v3_json(hist_json);
    if !hist_replay.valid
        || !hist_replay.all_registered_paths_typed_and_marginal
        || !hist_replay.full_certified_totals_partial
    {
        return Err(AgentAHistCertV4Error::Prerequisite(format!(
            "HIST-CERT v3 path handoff did not replay: {:?}",
            hist_replay.errors
        )));
    }
    let hist: HistCertV3Certificate = serde_json::from_str(hist_json)
        .map_err(|error| AgentAHistCertV4Error::Json(error.to_string()))?;

    let mut packages = Vec::new();
    for historical in &hist.packages {
        let output = e2b
            .historical_count_outputs
            .iter()
            .find(|output| output.step == historical.step)
            .ok_or_else(|| {
                AgentAHistCertV4Error::Invariant(format!(
                    "E-2b has no count output for step {}",
                    historical.step
                ))
            })?;
        let rows = e2b
            .ordinary_rows
            .iter()
            .filter(|row| row.step == historical.step)
            .collect::<Vec<_>>();
        let generated = rows
            .iter()
            .filter(|row| row.generated_instance_token.is_some())
            .count() as u32;
        let independent = rows.iter().filter(|row| row.family_token.is_some()).count() as u32;
        let certified_semantic_total = historical.typed_and_marginal_path_subtotal + independent;
        if certified_semantic_total != output.quotient_family_total
            || generated != output.generated_instance_row_count
            || output.operational_row_coverage_total != historical.recorded_total
        {
            return Err(AgentAHistCertV4Error::Invariant(format!(
                "handoff arithmetic mismatch at step {}",
                historical.step
            )));
        }
        let signed_divergence = certified_semantic_total as i32 - historical.recorded_total as i32;
        let divergence_cause = (signed_divergence != 0).then(|| {
            e2b.fq2
                .divergences
                .iter()
                .find(|divergence| divergence.stage == historical.step)
                .map(|divergence| divergence.exact_cause.clone())
                .unwrap_or_else(|| "missing_fq2_divergence_record".to_owned())
        });
        if divergence_cause.as_deref() == Some("missing_fq2_divergence_record") {
            return Err(AgentAHistCertV4Error::Invariant(format!(
                "semantic divergence at step {} lacks F-Q2 provenance",
                historical.step
            )));
        }
        let mut package = AgentAPackageCertification {
            step: historical.step,
            label: historical.label.clone(),
            sealed_operational_total: historical.recorded_total,
            certified_path_family_count: historical.typed_and_marginal_path_subtotal,
            certified_independent_ordinary_family_count: independent,
            certified_generated_instance_count: generated,
            certified_semantic_total,
            operational_row_coverage_total: output.operational_row_coverage_total,
            operational_inventory_fully_covered: output.every_ordinary_row_covered
                && output.operational_row_coverage_matches_seal,
            semantic_total_matches_seal: signed_divergence == 0,
            signed_divergence,
            divergence_cause,
            every_path_family_typed_and_marginal: historical
                .path_families
                .iter()
                .all(|family| family.typed_and_marginal),
            every_ordinary_row_typed_normalized_natural_and_resolved: rows.iter().all(|row| {
                row.row_typed
                    && row.row_normalized
                    && row.row_natural
                    && row.exactly_one_quotient_resolution_issued
            }),
            generated_instance_not_multiplied: rows.iter().all(|row| {
                row.generated_instance_token.as_ref().is_none_or(|token| {
                    !token.ordinary_family_issued && !token.counts_as_one_quotient_family
                })
            }),
            full_certified_total_issued: true,
            derivation_hash: String::new(),
        };
        package.derivation_hash = tagged_hash("agent-a-package-certification", &package);
        packages.push(package);
    }

    let certified_semantic_totals = packages
        .iter()
        .map(|package| package.certified_semantic_total)
        .collect::<Vec<_>>();
    let sealed_testimonial_totals = packages
        .iter()
        .map(|package| package.sealed_operational_total)
        .collect::<Vec<_>>();
    let certified_divergence_steps = packages
        .iter()
        .filter(|package| package.signed_divergence != 0)
        .map(|package| package.step)
        .collect::<Vec<_>>();
    let all_registered_path_families_certified = packages
        .iter()
        .all(|package| package.every_path_family_typed_and_marginal);
    let all_ordinary_rows_quotient_resolved = packages.iter().all(|package| {
        package.every_ordinary_row_typed_normalized_natural_and_resolved
            && package.generated_instance_not_multiplied
    });
    let every_difference_has_exact_divergence_record = packages.iter().all(|package| {
        (package.signed_divergence == 0 && package.divergence_cause.is_none())
            || (package.signed_divergence != 0 && package.divergence_cause.is_some())
    });
    let f_t1_discharged = all_registered_path_families_certified
        && all_ordinary_rows_quotient_resolved
        && every_difference_has_exact_divergence_record
        && packages.len() == 4;
    let mut f_t1 = FT1BranchIiAudit {
        falsifier: "F-T1".to_owned(),
        branch_semantics:
            "certified semantic totals plus certified divergence records; the seal remains testimonial"
                .to_owned(),
        certified_steps: packages.iter().map(|package| package.step).collect(),
        certified_semantic_totals: certified_semantic_totals.clone(),
        sealed_testimonial_totals,
        certified_divergence_steps,
        all_registered_path_families_certified,
        all_ordinary_rows_quotient_resolved,
        every_difference_has_exact_divergence_record,
        blind_reproduction_of_seal_required: false,
        f_t1_discharged,
        derivation_hash: String::new(),
    };
    f_t1.derivation_hash = tagged_hash("f-t1-branch-ii-audit", &f_t1);
    if !f_t1_discharged {
        return Err(AgentAHistCertV4Error::Invariant(
            "F-T1 Branch-(ii) criteria did not close".to_owned(),
        ));
    }

    let downstream = AgentADeferredSequence {
        remaining_history_certification_executed: false,
        reselection_executed: false,
        e5_f1_executed: false,
        bridge_executed: false,
        final_certificate_executed: false,
    };
    let mut certificate = AgentAHistCertV4Certificate {
        schema: AGENT_A_HIST_CERT_V4_SCHEMA.to_owned(),
        date: AGENT_A_HIST_CERT_V4_DATE.to_owned(),
        source_bindings: source_bindings(),
        branch_ii_adoption_replayed,
        e2b_schema: E2B_QUOTIENT_CLOSURE_SCHEMA.to_owned(),
        e2b_digest: e2b.result_digest.clone(),
        e2b_replayed: true,
        hist_v3_schema: HIST_CERT_V3_SCHEMA.to_owned(),
        hist_v3_digest: hist.result_digest.clone(),
        hist_v3_replayed: true,
        e2b_handoff_consumed_exactly_once: true,
        packages,
        f_t1,
        full_certified_totals: certified_semantic_totals,
        f_t1_discharged,
        downstream,
        outcome: "agent_a_branch_ii_full_hit_totals_certified_f_t1_discharged".to_owned(),
        permitted_conclusion: "The historical HIT packages have full semantic totals 7/8/10/17. Step 8 retains an exact -1 Generated-instance divergence from the sealed operational 18; F-T1 is discharged under adopted Branch-(ii) semantics.".to_owned(),
        required_successor_action: "Certify Steps 2-4 and 9-15 with the same completed-basis and divergence-record discipline before preregistering the reselection burn.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = tagged_hash("agent-a-hist-cert-v4", &certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> AgentAHistCertV4Replay {
    AgentAHistCertV4Replay {
        valid: false,
        package_steps: Vec::new(),
        certified_totals: Vec::new(),
        divergence_steps: Vec::new(),
        f_t1_discharged: false,
        downstream_deferred: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_agent_a_hist_cert_v4_certificate(
    certificate: &AgentAHistCertV4Certificate,
) -> AgentAHistCertV4Replay {
    let expected = match issue_agent_a_hist_cert_v4() {
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
    if observed != tagged_hash("agent-a-hist-cert-v4", &digest_projection) {
        errors.push("result digest mismatch".to_owned());
    }
    AgentAHistCertV4Replay {
        valid: errors.is_empty(),
        package_steps: certificate
            .packages
            .iter()
            .map(|package| package.step)
            .collect(),
        certified_totals: certificate.full_certified_totals.clone(),
        divergence_steps: certificate.f_t1.certified_divergence_steps.clone(),
        f_t1_discharged: certificate.f_t1_discharged,
        downstream_deferred: certificate.downstream.all_deferred(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_agent_a_hist_cert_v4_json(json: &str) -> AgentAHistCertV4Replay {
    match serde_json::from_str::<AgentAHistCertV4Certificate>(json) {
        Ok(certificate) => replay_agent_a_hist_cert_v4_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_agent_a_hist_cert_v4_create_new(
    path: &Path,
) -> Result<AgentAHistCertV4Replay, AgentAHistCertV4Error> {
    let certificate = issue_agent_a_hist_cert_v4()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| AgentAHistCertV4Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| AgentAHistCertV4Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| AgentAHistCertV4Error::Io(error.to_string()))?;
    let replay = replay_agent_a_hist_cert_v4_certificate(&certificate);
    if !replay.valid {
        return Err(AgentAHistCertV4Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_ii_certifies_hit_totals_and_discharges_ft1() {
        let certificate = issue_agent_a_hist_cert_v4().expect("Agent A v4");
        assert_eq!(certificate.full_certified_totals, vec![7, 8, 10, 17]);
        assert_eq!(certificate.f_t1.certified_divergence_steps, vec![8]);
        assert!(certificate.f_t1_discharged);
        assert!(certificate.downstream.all_deferred());
    }

    #[test]
    fn generated_cell_credit_and_divergence_suppression_are_rejected() {
        let certificate = issue_agent_a_hist_cert_v4().expect("Agent A v4");
        let mut forged = certificate.clone();
        forged.packages[3].certified_semantic_total = 18;
        assert!(!replay_agent_a_hist_cert_v4_certificate(&forged).valid);
        let mut suppressed = certificate;
        suppressed.f_t1.certified_divergence_steps.clear();
        assert!(!replay_agent_a_hist_cert_v4_certificate(&suppressed).valid);
    }
}
