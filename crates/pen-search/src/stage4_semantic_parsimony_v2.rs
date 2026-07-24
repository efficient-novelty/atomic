//! BI-0-gated successor to the verdict-blind Stage-4 semantic audit.
//!
//! V1 proved a useful conditional theorem but issued its own prefix token.
//! That token did not satisfy the adopted BI-0-before-cone gate.  V2 has no
//! capability issuer.  Its only theorem entry point accepts the narrow token
//! exported by a separately completed BI-0 v6 run, replays that token, checks
//! its exact typed Stage-1-through-3 payload, and only then invokes the blind
//! V1 audit.  It still selects and executes no branch.

use crate::bi0_semantic_register_v6::{
    BI0_STAGE4_OPENING_CAPABILITY_V6_SCHEMA, Bi0Stage4OpeningCapabilityV6,
    replay_bi0_stage4_opening_capability_v6,
};
use crate::stage4_semantic_parsimony_v1::{
    Stage4SemanticParsimonyOutcomeV1, Stage4SemanticParsimonyV1Certificate,
    issue_stage4_semantic_parsimony_v1,
};
use pen_core::hash::blake3_hex;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const STAGE4_SEMANTIC_PARSIMONY_V2_SCHEMA: &str = "stage4-semantic-parsimony-bi0-gated-v2";
pub const STAGE4_SEMANTIC_PARSIMONY_V2_DATE: &str = "2026-07-22";
pub const STAGE4_SEMANTIC_PARSIMONY_V2_THEOREM_ID: &str =
    "T-SP4-v2-external-BI0-opening-before-blind-semantic-audit";
pub const STAGE4_SEMANTIC_PARSIMONY_V2_CERTIFICATE_NAME: &str = "stage4_semantic_parsimony_v2.json";
pub const STAGE4_SEMANTIC_PARSIMONY_V2_REPORT_NAME: &str = "STAGE4_SEMANTIC_PARSIMONY_V2_RESULT.md";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(STAGE4_SEMANTIC_PARSIMONY_V2_SCHEMA, domain, value))
        .expect("Stage-4 v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4Bi0GateBindingV2 {
    pub opening_capability_schema: String,
    pub opening_capability_derivation_hash: String,
    pub granting_bi0_result_digest: String,
    pub granting_bi0_pass_proof_hash: String,
    pub exact_prefix_steps: Vec<u32>,
    pub exact_prefix_candidate_hashes: Vec<String>,
    pub exact_prefix_predecessor_signature_digests: Vec<String>,
    pub exact_prefix_payload_hashes: Vec<String>,
    pub exact_prefix_t_bi_digest: String,
    pub exact_prefix_bi0_digest: String,
    pub capability_replayed_before_stage4_audit: bool,
    pub exact_stage1_through3_payload_replayed: bool,
    pub capability_has_no_cone_or_branch_power: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticParsimonyV2Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub external_bi0_opening_capability: Bi0Stage4OpeningCapabilityV6,
    pub gate_binding: Stage4Bi0GateBindingV2,
    pub gate_sealed_before_blind_audit: bool,
    pub blind_audit: Stage4SemanticParsimonyV1Certificate,
    pub blind_audit_issued_after_gate: bool,
    pub authorized_stage4_cone_audit: bool,
    pub authorized_bi1_branch_execution: bool,
    pub no_branch_selected_or_executed: bool,
    pub divergence_requires_versioned_adjudication: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticParsimonyV2Replay {
    pub valid: bool,
    pub bi0_gate_valid: bool,
    pub authorized_stage4_cone_audit: bool,
    pub branch_execution_authorized: bool,
    pub divergence_requires_adjudication: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Stage4SemanticParsimonyV2Error {
    #[error("Stage-4 v2 BI-0 opening failed: {0}")]
    Gate(String),
    #[error("Stage-4 v2 blind audit failed: {0}")]
    Audit(String),
    #[error("Stage-4 v2 invariant failed: {0}")]
    Invariant(String),
    #[error("Stage-4 v2 JSON failed: {0}")]
    Json(String),
    #[error("Stage-4 v2 I/O failed: {0}")]
    Io(String),
    #[error("emitted Stage-4 v2 artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn gate_binding_hash(binding: &Stage4Bi0GateBindingV2) -> String {
    let mut projection = binding.clone();
    projection.derivation_hash.clear();
    tagged_hash("external-BI0-gate-binding", &projection)
}

fn certificate_hash(certificate: &Stage4SemanticParsimonyV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("authorized-stage4-semantic-audit", &projection)
}

fn bind_external_gate(
    opening: &Bi0Stage4OpeningCapabilityV6,
) -> Result<Stage4Bi0GateBindingV2, Stage4SemanticParsimonyV2Error> {
    let replay_errors = replay_bi0_stage4_opening_capability_v6(opening);
    if !replay_errors.is_empty() {
        return Err(Stage4SemanticParsimonyV2Error::Gate(
            replay_errors.join("; "),
        ));
    }
    let entries = opening.exact_stage1_through3_entries();
    // The successful current-issuer replay above is the authority for which
    // typed prefix is current.  These checks establish only the capability's
    // own sequential/hash coherence; Stage 4 must not compare it with the
    // compiled historical reference payload.
    let exact_stage1_through3_payload_replayed = entries.len() == 3
        && entries.iter().map(|entry| entry.stage()).eq(1..=3)
        && entries.iter().enumerate().all(|(index, entry)| {
            entry.candidate_hash() == candidate_hash(entry.telescope())
                && entry.predecessor_signature_digest()
                    == SealedSignature::from_telescopes(
                        entries[..index]
                            .iter()
                            .map(|predecessor| {
                                (predecessor.stage(), predecessor.telescope().clone())
                            })
                            .collect(),
                    )
                    .digest()
                && !entry.payload_hash().is_empty()
        });
    let capability_has_no_cone_or_branch_power = opening.has_no_cone_or_branch_capability();
    let capability_replayed_before_stage4_audit = true;
    let proved = opening.schema() == BI0_STAGE4_OPENING_CAPABILITY_V6_SCHEMA
        && !opening.granting_bi0_result_digest().is_empty()
        && !opening.granting_bi0_pass_proof_hash().is_empty()
        && !opening
            .exact_stage1_through3_t_bi_prefix_digest()
            .is_empty()
        && !opening.exact_stage1_through3_bi0_prefix_digest().is_empty()
        && capability_replayed_before_stage4_audit
        && exact_stage1_through3_payload_replayed
        && capability_has_no_cone_or_branch_power;
    if !proved {
        return Err(Stage4SemanticParsimonyV2Error::Gate(
            "the replayed BI-0 capability did not bind the exact typed Stage-1-through-3 payload and no-branch boundary"
                .to_owned(),
        ));
    }
    let mut binding = Stage4Bi0GateBindingV2 {
        opening_capability_schema: opening.schema().to_owned(),
        opening_capability_derivation_hash: opening.derivation_hash().to_owned(),
        granting_bi0_result_digest: opening.granting_bi0_result_digest().to_owned(),
        granting_bi0_pass_proof_hash: opening.granting_bi0_pass_proof_hash().to_owned(),
        exact_prefix_steps: entries.iter().map(|entry| entry.stage()).collect(),
        exact_prefix_candidate_hashes: entries
            .iter()
            .map(|entry| entry.candidate_hash().to_owned())
            .collect(),
        exact_prefix_predecessor_signature_digests: entries
            .iter()
            .map(|entry| entry.predecessor_signature_digest().to_owned())
            .collect(),
        exact_prefix_payload_hashes: entries
            .iter()
            .map(|entry| entry.payload_hash().to_owned())
            .collect(),
        exact_prefix_t_bi_digest: opening
            .exact_stage1_through3_t_bi_prefix_digest()
            .to_owned(),
        exact_prefix_bi0_digest: opening.exact_stage1_through3_bi0_prefix_digest().to_owned(),
        capability_replayed_before_stage4_audit,
        exact_stage1_through3_payload_replayed,
        capability_has_no_cone_or_branch_power,
        proved,
        derivation_hash: String::new(),
    };
    binding.derivation_hash = gate_binding_hash(&binding);
    Ok(binding)
}

/// The sole V2 theorem entry point.  It accepts an externally issued BI-0
/// capability and has no API for constructing one.
pub fn issue_stage4_semantic_parsimony_v2(
    opening: &Bi0Stage4OpeningCapabilityV6,
) -> Result<Stage4SemanticParsimonyV2Certificate, Stage4SemanticParsimonyV2Error> {
    // This statement is intentionally first: no Stage-4 geometry or semantic
    // evidence exists until the external BI-0 capability has replayed.
    let gate_binding = bind_external_gate(opening)?;
    issue_stage4_semantic_parsimony_v2_after_gate(opening, gate_binding)
}

fn issue_stage4_semantic_parsimony_v2_after_gate(
    opening: &Bi0Stage4OpeningCapabilityV6,
    gate_binding: Stage4Bi0GateBindingV2,
) -> Result<Stage4SemanticParsimonyV2Certificate, Stage4SemanticParsimonyV2Error> {
    let gate_sealed_before_blind_audit = gate_binding.proved;

    let blind_audit = issue_stage4_semantic_parsimony_v1()
        .map_err(|error| Stage4SemanticParsimonyV2Error::Audit(error.to_string()))?;
    // V1 issuance is itself deterministic.  V2 replay deterministically
    // reissues the whole gated certificate, so replaying V1 again here would
    // duplicate the expensive semantic audit without adding authority.
    let blind_audit_issued_after_gate = true;
    let no_branch_selected_or_executed = blind_audit.no_branch_executed;
    let divergence_requires_versioned_adjudication = matches!(
        blind_audit.outcome,
        Stage4SemanticParsimonyOutcomeV1::TwoSemanticMinimizersRt2InequivalentEnactedRootNonminimalAdjudicationRequired
    ) && blind_audit.exact_two_minimizer_enacted_nonminimal_divergence;
    let authorized_stage4_cone_audit = gate_sealed_before_blind_audit
        && blind_audit_issued_after_gate
        && no_branch_selected_or_executed
        && !blind_audit.desired_verdict_count_score_or_bar_used_as_semantic_premise;
    let authorized_bi1_branch_execution = false;
    if !authorized_stage4_cone_audit {
        return Err(Stage4SemanticParsimonyV2Error::Invariant(
            "the gated Stage-4 audit did not close its blind no-branch theorem".to_owned(),
        ));
    }

    let mut certificate = Stage4SemanticParsimonyV2Certificate {
        schema: STAGE4_SEMANTIC_PARSIMONY_V2_SCHEMA.to_owned(),
        date: STAGE4_SEMANTIC_PARSIMONY_V2_DATE.to_owned(),
        theorem_id: STAGE4_SEMANTIC_PARSIMONY_V2_THEOREM_ID.to_owned(),
        external_bi0_opening_capability: opening.clone(),
        gate_binding,
        gate_sealed_before_blind_audit,
        blind_audit,
        blind_audit_issued_after_gate,
        authorized_stage4_cone_audit,
        authorized_bi1_branch_execution,
        no_branch_selected_or_executed,
        divergence_requires_versioned_adjudication,
        outcome: if divergence_requires_versioned_adjudication {
            "STAGE4_V2_AUTHORIZED_SEMANTIC_DIVERGENCE_ADJUDICATION_REQUIRED"
        } else {
            "STAGE4_V2_AUTHORIZED_BLIND_AUDIT_RECORDED"
        }
        .to_owned(),
        permitted_conclusion: "A passing current BI-0 v6 capability was replayed before any Stage-4 work, so the four-root semantic parsimony computation is an authorized cone audit. It selects and executes no branch."
            .to_owned(),
        required_successor_action: if divergence_requires_versioned_adjudication {
            "Adjudicate the certified semantic divergence (two nu-minimal, R-T2-inequivalent roots; enacted root nonminimal) before any BI-1 branch execution, UC-1 scoring, bridge, or final certificate."
        } else {
            "Continue only through the successor action named by the enclosed blind audit."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_hash(&certificate);
    Ok(certificate)
}

pub fn replay_stage4_semantic_parsimony_v2(
    claimed: &Stage4SemanticParsimonyV2Certificate,
) -> Stage4SemanticParsimonyV2Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_hash(claimed) {
        errors.push("Stage-4 v2 certificate digest mismatch".to_owned());
    }
    let bi0_gate_valid = match bind_external_gate(&claimed.external_bi0_opening_capability) {
        Ok(gate_binding) => {
            match issue_stage4_semantic_parsimony_v2_after_gate(
                &claimed.external_bi0_opening_capability,
                gate_binding,
            ) {
                Ok(expected) if expected == *claimed => {}
                Ok(_) => errors.push(
                    "Stage-4 v2 certificate differs from deterministic externally gated reissuance"
                        .to_owned(),
                ),
                Err(error) => errors.push(error.to_string()),
            }
            claimed.gate_binding.proved
        }
        Err(error) => {
            errors.push(error.to_string());
            false
        }
    };
    Stage4SemanticParsimonyV2Replay {
        valid: errors.is_empty(),
        bi0_gate_valid,
        authorized_stage4_cone_audit: claimed.authorized_stage4_cone_audit,
        branch_execution_authorized: claimed.authorized_bi1_branch_execution,
        divergence_requires_adjudication: claimed.divergence_requires_versioned_adjudication,
        errors,
    }
}

pub fn replay_stage4_semantic_parsimony_v2_json(json: &str) -> Stage4SemanticParsimonyV2Replay {
    match serde_json::from_str::<Stage4SemanticParsimonyV2Certificate>(json) {
        Ok(certificate) => replay_stage4_semantic_parsimony_v2(&certificate),
        Err(error) => Stage4SemanticParsimonyV2Replay {
            valid: false,
            bi0_gate_valid: false,
            authorized_stage4_cone_audit: false,
            branch_execution_authorized: false,
            divergence_requires_adjudication: false,
            errors: vec![format!("Stage-4 v2 JSON did not deserialize: {error}")],
        },
    }
}

pub fn render_stage4_semantic_parsimony_v2_report(
    certificate: &Stage4SemanticParsimonyV2Certificate,
) -> String {
    let rows = certificate
        .blind_audit
        .preseal
        .roots
        .iter()
        .map(|root| {
            let status = if certificate
                .blind_audit
                .preseal
                .semantic_minimizer_hashes
                .contains(&root.candidate_hash)
            {
                "minimum"
            } else if root.candidate_hash == certificate.blind_audit.enacted_root_hash {
                "enacted; nonminimal"
            } else {
                "nonminimal"
            };
            format!(
                "| `{}` | {} | {} | {} |",
                root.candidate_hash, root.kappa, root.stage4_semantic_nu, status
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# Stage-4 semantic parsimony v2 result\n\n**Date:** {}. **Outcome:** `{}`. **BI-0 gate replayed before cone audit:** **{}**. **Authorized branch execution:** **{}**.\n\n| Candidate | kappa | semantic nu | Result |\n|---|---:|---:|---|\n{}\n\nMinimum: `({}, {})`; minimizers: **{}**. Enacted root `{}` has semantic nu **{}** and is minimal: **{}**. Frozen R-T2 survivor pair inequivalent: **{}**. No branch selected or executed: **{}**.\n\n{}\n\nNext: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.gate_sealed_before_blind_audit,
        certificate.authorized_bi1_branch_execution,
        rows,
        certificate.blind_audit.preseal.minimum_kappa,
        certificate.blind_audit.preseal.minimum_semantic_nu,
        certificate.blind_audit.preseal.semantic_minimizer_count,
        certificate.blind_audit.enacted_root_hash,
        certificate.blind_audit.enacted_root_semantic_nu,
        certificate.blind_audit.enacted_root_is_semantic_minimizer,
        certificate
            .blind_audit
            .surviving_pair_frozen_r_t2_inequivalent,
        certificate.no_branch_selected_or_executed,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.result_digest,
    )
}

pub fn emit_stage4_semantic_parsimony_v2_create_new(
    opening: &Bi0Stage4OpeningCapabilityV6,
    directory: &Path,
) -> Result<Stage4SemanticParsimonyV2Certificate, Stage4SemanticParsimonyV2Error> {
    let certificate = issue_stage4_semantic_parsimony_v2(opening)?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| Stage4SemanticParsimonyV2Error::Json(error.to_string()))?;
    let replay = replay_stage4_semantic_parsimony_v2_json(&json);
    if !replay.valid {
        return Err(Stage4SemanticParsimonyV2Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let report = render_stage4_semantic_parsimony_v2_report(&certificate);
    let json_path = directory.join(STAGE4_SEMANTIC_PARSIMONY_V2_CERTIFICATE_NAME);
    let report_path = directory.join(STAGE4_SEMANTIC_PARSIMONY_V2_REPORT_NAME);
    let mut json_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&json_path)
        .map_err(|error| Stage4SemanticParsimonyV2Error::Io(error.to_string()))?;
    json_file
        .write_all(json.as_bytes())
        .map_err(|error| Stage4SemanticParsimonyV2Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| Stage4SemanticParsimonyV2Error::Io(error.to_string()))?;
    report_file
        .write_all(report.as_bytes())
        .map_err(|error| Stage4SemanticParsimonyV2Error::Io(error.to_string()))?;
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bi0_semantic_register_v6::issue_bi0_stage4_opening_capability_v6;

    #[test]
    fn externally_gated_audit_records_divergence_without_executing_a_branch() {
        let opening = issue_bi0_stage4_opening_capability_v6().expect("passing BI-0 v6 gate");
        let certificate =
            issue_stage4_semantic_parsimony_v2(&opening).expect("gated Stage-4 audit");
        assert!(certificate.gate_binding.proved);
        assert!(certificate.gate_sealed_before_blind_audit);
        assert!(certificate.authorized_stage4_cone_audit);
        assert!(!certificate.authorized_bi1_branch_execution);
        assert!(certificate.no_branch_selected_or_executed);
        assert!(certificate.divergence_requires_versioned_adjudication);
        assert_eq!(certificate.blind_audit.preseal.semantic_minimizer_count, 2);
        assert!(!certificate.blind_audit.enacted_root_is_semantic_minimizer);
        assert!(replay_stage4_semantic_parsimony_v2(&certificate).valid);
    }

    #[test]
    fn stage4_v2_production_surface_has_no_bi0_capability_issuer() {
        let source = include_str!("stage4_semantic_parsimony_v2.rs");
        let production = source.split("#[cfg(test)]").next().unwrap_or(source);
        let forbidden = ["issue_", "bi0_stage4_opening_capability_v6"].concat();
        assert!(!production.contains(&forbidden));
        assert!(production.contains("replay_bi0_stage4_opening_capability_v6"));
        assert!(!production.contains("Telescope::reference"));
    }
}
