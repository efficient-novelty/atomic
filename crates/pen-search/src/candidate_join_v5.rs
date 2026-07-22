//! Additive schema-5 rebind of the schema-4 kernel bridge under the adopted
//! historical element overlay.
//!
//! Schema 3 and schema 4 are immutable inputs.  This module first replays the
//! exact schema-4 bytes against the exact schema-3 bytes, then records only
//! the evidence newly earned by the V3 element-overlay tokens.  In
//! particular, registered historical base binding is proved, while general
//! C6 completion and candidate-level C8 provenance remain named gaps.
//!
//! This module contains no candidate ordering, acceptance, threshold, or
//! score comparison.

use crate::candidate_join::{
    C8_CANDIDATE_JOIN_GAP, C8_REFERENCE_ONLY_PROOF, CandidateJoinV4Certificate,
    RowObligationStatus, run_candidate_join_v4,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::cubical::typed_boundary::{
    ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION, ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION,
    BOUNDARY_CHARGE_POLICY_VERSION, C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP,
    HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION, HISTORICAL_POINT_TYPING_OBSTRUCTION_ID,
    HISTORICAL_PREFIX_V3_C6_BRIDGE_VERSION, HistoricalBaseBindingAudit,
    REGISTERED_HISTORICAL_DIAGRAMS_VERSION, RegisteredBoundaryKind,
    TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION, audit_historical_base_binding,
    issue_adopted_boundary_basis_charge_token, issue_historical_base_binding_token_v3,
    issue_historical_base_binding_token_v3_for_historical_prefix,
    issue_historical_element_overlay_token,
    issue_historical_element_overlay_token_for_historical_prefix,
    issue_historical_prefix_v3_c6_bridge_token, issue_operational_role_witness_v3,
    issue_operational_role_witness_v3_for_historical_prefix, issue_reference_only_charge_token,
    issue_typed_declared_boundary_token, issue_typed_declared_boundary_token_v3,
    issue_typed_declared_boundary_token_v3_for_historical_prefix, registered_boundary_diagram,
    registered_boundary_diagram_for_historical_prefix,
    registered_historical_element_overlay_entries, replay_adopted_boundary_basis_charge_token,
    replay_historical_base_binding_token_v3,
    replay_historical_base_binding_token_v3_for_historical_prefix,
    replay_historical_element_overlay_token,
    replay_historical_element_overlay_token_for_historical_prefix,
    replay_historical_prefix_v3_c6_bridge_token, replay_operational_role_witness_v3,
    replay_operational_role_witness_v3_for_historical_prefix, replay_reference_only_charge_token,
    replay_typed_declared_boundary_token, replay_typed_declared_boundary_token_v3,
    replay_typed_declared_boundary_token_v3_for_historical_prefix,
};
use pen_type::elaborate::{KernelTy, SealedSignature};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::fmt::Write as _;
use thiserror::Error;

pub const CANDIDATE_JOIN_V5_SCHEMA_VERSION: u32 = 5;
pub const CANDIDATE_JOIN_V5_DATE: &str = "2026-07-19";

pub const SCHEMA4_ARCHIVE_LENGTH: usize = 535_972;
pub const SCHEMA4_ARCHIVE_SHA256: &str =
    "4094B2E65E36507157029760C71FF62BA241E82D066CF590F9184DE66BEDB5F4";
pub const SCHEMA4_ARCHIVE_INTERNAL_DIGEST: &str =
    "blake3:6a6c3ccfe4731080bfaa944b98f50afe2bde1eb29fdd679b8f62e3a42f72d1e2";
pub const SCHEMA4_ARCHIVE_GIT_BLOB: &str = "5904e99c6de7a4047de1f66bae7cb7b0d9d81e07";

pub const ELEMENT_OVERLAY_ADJUDICATION_LENGTH: usize = 5_856;
pub const ELEMENT_OVERLAY_ADJUDICATION_SHA256: &str =
    "F7C6642B277C2AEE03B1129E33DBD3B1859F622F5CE3C1D200AF5E48A1DFE89B";
pub const ELEMENT_OVERLAY_ADJUDICATION_GIT_BLOB: &str = "a644c255f6a79597176f22c91645498175bf02fd";

pub const C6_V3_GENERAL_GAP: &str = "C6_V3_GENERAL_HISTORICAL_TERM_LEVEL_COMPLETION";
pub const C6_V3_REGISTERED_CONSTANT_PROOF: &str = "C6_V3_REGISTERED_CONSTANT_BOUNDARY_TERMS_TYPED";
pub const C7_V3_ALL_REGISTERED_PROOF: &str = "C7_ALL_REGISTERED_HISTORICAL_BASE_BINDINGS_UNDER_V3";
pub const C6_TRUNC_REALIZER_GAP: &str = C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER_GAP;
pub const F_B2_PENDING_GAP: &str = "F-B2-HIST-CERT-V3-CREATE-NEW-RERUN-PENDING";

const V2_S1_REFERENCE_HASH: &str =
    "blake3:14553e160a3daf841bf85cdab3f491e600286dcd944223152cff156f029c5baf";
const V2_S1_BASIS_HASH: &str =
    "blake3:3deede85f8bf3a530989490a76da765341ee298b178dec6971a9822f09aa480d";
const V2_TRUNC_REFERENCE_HASH: &str =
    "blake3:2abdb6bb7a834c039721bd3a13c57d23929446eced113fec34bc8c1a1fe14acc";
const V2_TRUNC_BASIS_HASH: &str =
    "blake3:2264fa85d67d406f660cf0a241b9bb1a1869cc8fe90a1fc72c40bbb49fe243e6";
const V2_TRUNC_TYPED_HASH: &str =
    "blake3:f34fd0d2660bb44b2f699cae6e0babb2f7bf9c9cd496c6abe616d99934d0b055";
const V2_S2_REFERENCE_HASH: &str =
    "blake3:ae6a58ad107157914a3b110c45e1131634d5f5fc57288a8850ac1425d7b9bbaa";
const V2_S2_BASIS_HASH: &str =
    "blake3:22bc0bda5518e2f8a6adae520970c7e3d43976dd98e4cf3420eef803b3fbbc3e";
const V2_S3_REFERENCE_HASH: &str =
    "blake3:7347676469ba1d3831913e1fc754b1d88621eb7740cadcccfe7733b180b2570f";
const V2_S3_BASIS_HASH: &str =
    "blake3:ff323cdaf4eb58f34ae96aeeb0f931d320f4939d39bc32ffb30c5ac6d9776f42";

const EMBEDDED_SCHEMA3_ARCHIVE: &[u8] =
    include_bytes!("../../../docs/ip1_candidate_verdict_join_v3.json");
const EMBEDDED_SCHEMA4_ARCHIVE: &[u8] =
    include_bytes!("../../../docs/ip1_candidate_verdict_join_v4.json");
const EMBEDDED_ELEMENT_OVERLAY_ADJUDICATION: &[u8] =
    include_bytes!("../../../docs/element_overlay_adjudication.md");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CandidateJoinV5Error {
    #[error("schema-4 archive has length {found}, expected {expected}")]
    ArchiveLength { expected: usize, found: usize },
    #[error("schema-4 archive SHA-256 mismatch: expected {expected}, got {found}")]
    ArchiveSha256 { expected: String, found: String },
    #[error("runtime schema-4 bytes differ from the compile-time archive")]
    ArchiveBytesDiffer,
    #[error("runtime schema-3 bytes differ from the compile-time archive")]
    Schema3BytesDiffer,
    #[error("invalid schema-4 JSON: {0}")]
    InvalidSchema4Json(String),
    #[error("invalid schema-5 JSON: {0}")]
    InvalidSchema5Json(String),
    #[error("schema-4 internal digest mismatch: expected {expected}, got {found}")]
    ArchiveInternalDigest { expected: String, found: String },
    #[error("schema-4 definition replay failed: {0}")]
    Schema4Replay(String),
    #[error("element-overlay adjudication binding mismatch: {0}")]
    Adjudication(String),
    #[error("V3 boundary evidence failed for {obligation}: {detail}")]
    Evidence { obligation: String, detail: String },
    #[error("schema-4 row join failed: {0}")]
    RowJoin(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum V5EvidenceStatus {
    Proved { proof_id: String },
    Gap { gap_id: String },
}

impl V5EvidenceStatus {
    fn is_proved(&self) -> bool {
        matches!(self, Self::Proved { .. })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct V5ScopedEvidence {
    pub evidence_id: String,
    pub implementation_version: String,
    pub scope: String,
    pub status: V5EvidenceStatus,
    pub kernel_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct V5ObligationEvidence {
    pub obligation_id: String,
    pub status: V5EvidenceStatus,
    pub scoped_evidence: Vec<V5ScopedEvidence>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NamedDigest {
    pub name: String,
    pub digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExactFileBinding {
    pub byte_length: usize,
    pub sha256: String,
    pub git_blob: String,
    pub internal_digest: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Schema4ArchiveBinding {
    pub exact_file: ExactFileBinding,
    pub schema_version: u32,
    pub date: String,
    pub schema3_sha256: String,
    pub schema3_internal_digest: String,
    pub stratum_row_counts: Vec<usize>,
    pub total_rows: usize,
    pub exact_definition_replay: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OverlayRegistryEntry {
    pub kind: String,
    pub step: u32,
    pub formation_clause: u16,
    pub element_clause: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoundaryAxiomV3Binding {
    pub status: String,
    pub predecessor_axiom_version: String,
    pub adopted_axiom_version: String,
    pub overlay_version: String,
    pub implementation_version: String,
    pub registered_diagrams_version: String,
    pub charging_policy_version: String,
    pub registered_entries: Vec<OverlayRegistryEntry>,
    pub adjudication: ExactFileBinding,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OverlayEntryReplayEvidence {
    pub kind: String,
    pub step: u32,
    pub formation_clause: u16,
    pub element_clause: u16,
    pub path_clause: u16,
    pub recorded_type: String,
    pub overlay_owner_digest: String,
    pub operational_role_derivation_hash: String,
    pub element_overlay_derivation_hash: String,
    pub base_binding_derivation_hash: String,
    pub typed_boundary_derivation_hash: String,
    pub typed_face_count: usize,
    pub historical_prefix_signature_digest: String,
    pub prefix_operational_role_derivation_hash: String,
    pub prefix_element_overlay_derivation_hash: String,
    pub prefix_base_binding_derivation_hash: String,
    pub prefix_typed_boundary_derivation_hash: String,
    pub prefix_c6_bridge_derivation_hash: String,
    pub prefix_path_basis_derivation_hash: String,
    pub prefix_boundary_basis_presentation_derivation_hash: String,
    pub prefix_key_bijection_digest: String,
    pub prefix_key_correspondence_count: usize,
    pub prefix_path_realization_root: String,
    pub prefix_expected_basis_count: u64,
    pub prefix_realized_basis_count: u64,
    pub f_o1_operational_role_replayed: bool,
    pub f_o2_term_typing_replayed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct V4ConservativityEvidence {
    pub exact_schema4_bytes_replayed: bool,
    pub all_213_parent_rows_preserved: bool,
    pub unchanged_parent_evidence: Vec<NamedDigest>,
    pub v2_obstruction_remains_available: bool,
    pub scope: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FalsifierLedgerV5 {
    pub f_o1_overlay_role_witnesses: V5EvidenceStatus,
    pub f_o2_registered_term_typing: V5EvidenceStatus,
    pub f_o3_exact_schema4_conservativity: V5EvidenceStatus,
    pub f_b2_hist_cert_v3_rerun: V5EvidenceStatus,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum V5RowObligationStatus {
    Proved {
        proof_id: String,
        evidence_hash: String,
    },
    Gap {
        gap_id: String,
        evidence_hash: String,
    },
    NotApplicable {
        reason_id: String,
        evidence_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateBridgeV5Row {
    pub stratum_index: u16,
    pub row_index: u16,
    pub kappa: u16,
    pub class: String,
    pub schema3_row_digest: String,
    pub schema4_row_derivation_hash: String,
    pub prior_c6_boundary_terms: RowObligationStatus,
    pub prior_c7_base_bindings: RowObligationStatus,
    pub prior_c8_charging: RowObligationStatus,
    pub c6_boundary_terms_under_v3: V5RowObligationStatus,
    pub c7_registered_historical_base_bindings_under_v3: V5RowObligationStatus,
    pub c8_charging_under_v3: V5RowObligationStatus,
    pub all_required_evidence_proved: bool,
    pub implemented_egp_bridge_returns_bound: bool,
    pub intended_typed_egp_proved: bool,
    pub full_candidate_extraction_join_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BridgeStratumV5 {
    pub stratum_index: u16,
    pub kappa: u16,
    pub rows: Vec<CandidateBridgeV5Row>,
    pub all_parent_rows_joined: bool,
    pub no_row_promoted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateJoinV5Completeness {
    pub exact_schema4_archive_bound_and_replayed: bool,
    pub all_213_schema4_rows_joined_by_row_digest: bool,
    pub f_o1_discharged: bool,
    pub f_o2_discharged: bool,
    pub f_o3_exact_schema4_scope_discharged: bool,
    pub f_b2_hist_cert_v3_rerun_discharged: bool,
    pub c6_registered_constant_boundary_terms_typed: bool,
    pub c6_general_historical_term_completion_proved: bool,
    pub c7_all_registered_historical_base_bindings_proved: bool,
    pub c8_registered_reference_only_charging_proved: bool,
    pub c8_candidate_boundary_provenance_join_proved: bool,
    pub no_row_promoted: bool,
    pub full_candidate_extraction_join_proved: bool,
    pub intended_semantic_candidate_join_proved: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateJoinV5Certificate {
    pub schema_version: u32,
    pub date: String,
    pub schema4_archive: Schema4ArchiveBinding,
    pub boundary_axiom_v3: BoundaryAxiomV3Binding,
    pub overlay_entries: Vec<OverlayEntryReplayEvidence>,
    pub v4_conservativity: V4ConservativityEvidence,
    pub c6_boundary_terms: V5ObligationEvidence,
    pub c7_base_bindings: V5ObligationEvidence,
    pub c8_charging: V5ObligationEvidence,
    pub falsifiers: FalsifierLedgerV5,
    pub strata: Vec<BridgeStratumV5>,
    pub completeness: CandidateJoinV5Completeness,
    pub digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateJoinV5Replay {
    pub valid: bool,
    pub exact_schema4_archive_bound_and_replayed: bool,
    pub all_213_schema4_rows_joined_by_row_digest: bool,
    pub c7_all_registered_historical_base_bindings_proved: bool,
    pub c6_general_historical_term_completion_proved: bool,
    pub c8_candidate_boundary_provenance_join_proved: bool,
    pub no_row_promoted: bool,
    pub errors: Vec<String>,
}

fn tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(CANDIDATE_JOIN_V5_SCHEMA_VERSION, domain, payload))
        .expect("schema-5 proof payload serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut encoded = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(&mut encoded, "{byte:02X}").expect("writing to String cannot fail");
    }
    encoded
}

fn kind_name(kind: RegisteredBoundaryKind) -> &'static str {
    match kind {
        RegisteredBoundaryKind::S1 => "s1",
        RegisteredBoundaryKind::Trunc => "trunc",
        RegisteredBoundaryKind::S2 => "s2",
        RegisteredBoundaryKind::S3 => "s3",
    }
}

fn proved(id: impl Into<String>) -> V5EvidenceStatus {
    V5EvidenceStatus::Proved {
        proof_id: id.into(),
    }
}

fn gap(id: impl Into<String>) -> V5EvidenceStatus {
    V5EvidenceStatus::Gap { gap_id: id.into() }
}

fn scoped(
    id: impl Into<String>,
    version: impl Into<String>,
    scope: impl Into<String>,
    status: V5EvidenceStatus,
    kernel_hash: impl Into<String>,
) -> V5ScopedEvidence {
    let evidence_id = id.into();
    let implementation_version = version.into();
    let scope = scope.into();
    let kernel_derivation_hash = kernel_hash.into();
    let derivation_hash = tagged_digest(
        "v5-scoped-evidence",
        &(
            &evidence_id,
            &implementation_version,
            &scope,
            &status,
            &kernel_derivation_hash,
        ),
    );
    V5ScopedEvidence {
        evidence_id,
        implementation_version,
        scope,
        status,
        kernel_derivation_hash,
        derivation_hash,
    }
}

fn obligation(
    id: impl Into<String>,
    status: V5EvidenceStatus,
    scoped_evidence: Vec<V5ScopedEvidence>,
) -> V5ObligationEvidence {
    let obligation_id = id.into();
    let derivation_hash = tagged_digest(
        "v5-obligation-evidence",
        &(&obligation_id, &status, &scoped_evidence),
    );
    V5ObligationEvidence {
        obligation_id,
        status,
        scoped_evidence,
        derivation_hash,
    }
}

fn exact_file_binding(
    bytes: &[u8],
    git_blob: &str,
    internal_digest: Option<&str>,
) -> ExactFileBinding {
    let byte_length = bytes.len();
    let sha256 = sha256_hex(bytes);
    let git_blob = git_blob.to_owned();
    let internal_digest = internal_digest.map(str::to_owned);
    let derivation_hash = tagged_digest(
        "exact-file-binding",
        &(byte_length, &sha256, &git_blob, &internal_digest),
    );
    ExactFileBinding {
        byte_length,
        sha256,
        git_blob,
        internal_digest,
        derivation_hash,
    }
}

fn validate_parent(
    schema3_json: &[u8],
    schema4_json: &[u8],
) -> Result<(CandidateJoinV4Certificate, Schema4ArchiveBinding), CandidateJoinV5Error> {
    if schema3_json != EMBEDDED_SCHEMA3_ARCHIVE {
        return Err(CandidateJoinV5Error::Schema3BytesDiffer);
    }
    if EMBEDDED_SCHEMA4_ARCHIVE.len() != SCHEMA4_ARCHIVE_LENGTH {
        return Err(CandidateJoinV5Error::ArchiveLength {
            expected: SCHEMA4_ARCHIVE_LENGTH,
            found: EMBEDDED_SCHEMA4_ARCHIVE.len(),
        });
    }
    let embedded_sha = sha256_hex(EMBEDDED_SCHEMA4_ARCHIVE);
    if embedded_sha != SCHEMA4_ARCHIVE_SHA256 {
        return Err(CandidateJoinV5Error::ArchiveSha256 {
            expected: SCHEMA4_ARCHIVE_SHA256.to_owned(),
            found: embedded_sha,
        });
    }
    if schema4_json != EMBEDDED_SCHEMA4_ARCHIVE {
        return Err(CandidateJoinV5Error::ArchiveBytesDiffer);
    }
    let archived: CandidateJoinV4Certificate = serde_json::from_slice(schema4_json)
        .map_err(|error| CandidateJoinV5Error::InvalidSchema4Json(error.to_string()))?;
    if archived.digest != SCHEMA4_ARCHIVE_INTERNAL_DIGEST {
        return Err(CandidateJoinV5Error::ArchiveInternalDigest {
            expected: SCHEMA4_ARCHIVE_INTERNAL_DIGEST.to_owned(),
            found: archived.digest,
        });
    }
    let recomputed = run_candidate_join_v4(schema3_json)
        .map_err(|error| CandidateJoinV5Error::Schema4Replay(error.to_string()))?;
    if archived != recomputed {
        return Err(CandidateJoinV5Error::Schema4Replay(
            "archived certificate differs from definition recomputation".to_owned(),
        ));
    }
    let row_counts = archived
        .strata
        .iter()
        .map(|stratum| stratum.rows.len())
        .collect::<Vec<_>>();
    let total_rows = row_counts.iter().sum();
    let exact_file = exact_file_binding(
        schema4_json,
        SCHEMA4_ARCHIVE_GIT_BLOB,
        Some(SCHEMA4_ARCHIVE_INTERNAL_DIGEST),
    );
    let schema_version = archived.schema_version;
    let date = archived.date.clone();
    let schema3_sha256 = archived.schema3_archive.sha256.clone();
    let schema3_internal_digest = archived.schema3_archive.internal_digest.clone();
    let exact_definition_replay = true;
    let derivation_hash = tagged_digest(
        "schema4-archive-binding",
        &(
            &exact_file,
            schema_version,
            &date,
            &schema3_sha256,
            &schema3_internal_digest,
            &row_counts,
            total_rows,
            exact_definition_replay,
        ),
    );
    Ok((
        archived,
        Schema4ArchiveBinding {
            exact_file,
            schema_version,
            date,
            schema3_sha256,
            schema3_internal_digest,
            stratum_row_counts: row_counts,
            total_rows,
            exact_definition_replay,
            derivation_hash,
        },
    ))
}

fn derive_axiom_binding() -> Result<BoundaryAxiomV3Binding, CandidateJoinV5Error> {
    if EMBEDDED_ELEMENT_OVERLAY_ADJUDICATION.len() != ELEMENT_OVERLAY_ADJUDICATION_LENGTH {
        return Err(CandidateJoinV5Error::Adjudication(format!(
            "length {}, expected {}",
            EMBEDDED_ELEMENT_OVERLAY_ADJUDICATION.len(),
            ELEMENT_OVERLAY_ADJUDICATION_LENGTH
        )));
    }
    let sha = sha256_hex(EMBEDDED_ELEMENT_OVERLAY_ADJUDICATION);
    if sha != ELEMENT_OVERLAY_ADJUDICATION_SHA256 {
        return Err(CandidateJoinV5Error::Adjudication(format!(
            "SHA-256 {sha}, expected {ELEMENT_OVERLAY_ADJUDICATION_SHA256}"
        )));
    }
    let registered_entries = registered_historical_element_overlay_entries()
        .into_iter()
        .map(|entry| OverlayRegistryEntry {
            kind: kind_name(entry.kind()).to_owned(),
            step: entry.step(),
            formation_clause: entry.formation_clause(),
            element_clause: entry.element_clause(),
        })
        .collect::<Vec<_>>();
    let expected = [
        ("s1", 5_u32, 0_u16, 1_u16),
        ("s2", 7_u32, 0_u16, 1_u16),
        ("s3", 8_u32, 0_u16, 1_u16),
    ];
    if registered_entries
        .iter()
        .map(|entry| {
            (
                entry.kind.as_str(),
                entry.step,
                entry.formation_clause,
                entry.element_clause,
            )
        })
        .collect::<Vec<_>>()
        != expected
    {
        return Err(CandidateJoinV5Error::Adjudication(
            "closed overlay registry is not exactly Steps 5, 7, and 8".to_owned(),
        ));
    }
    let adjudication = exact_file_binding(
        EMBEDDED_ELEMENT_OVERLAY_ADJUDICATION,
        ELEMENT_OVERLAY_ADJUDICATION_GIT_BLOB,
        None,
    );
    let status = "adopted".to_owned();
    let predecessor_axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION.to_owned();
    let adopted_axiom_version = ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION.to_owned();
    let overlay_version = HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION.to_owned();
    let implementation_version = TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION.to_owned();
    let registered_diagrams_version = REGISTERED_HISTORICAL_DIAGRAMS_VERSION.to_owned();
    let charging_policy_version = BOUNDARY_CHARGE_POLICY_VERSION.to_owned();
    let derivation_hash = tagged_digest(
        "boundary-axiom-v3-binding",
        &(
            &status,
            &predecessor_axiom_version,
            &adopted_axiom_version,
            &overlay_version,
            &implementation_version,
            &registered_diagrams_version,
            &charging_policy_version,
            &registered_entries,
            &adjudication,
        ),
    );
    Ok(BoundaryAxiomV3Binding {
        status,
        predecessor_axiom_version,
        adopted_axiom_version,
        overlay_version,
        implementation_version,
        registered_diagrams_version,
        charging_policy_version,
        registered_entries,
        adjudication,
        derivation_hash,
    })
}

fn evidence_error(obligation: &str, detail: impl ToString) -> CandidateJoinV5Error {
    CandidateJoinV5Error::Evidence {
        obligation: obligation.to_owned(),
        detail: detail.to_string(),
    }
}

fn derive_overlay_entries(
    signature: &SealedSignature,
) -> Result<Vec<OverlayEntryReplayEvidence>, CandidateJoinV5Error> {
    let mut evidence = Vec::new();
    for kind in [
        RegisteredBoundaryKind::S1,
        RegisteredBoundaryKind::S2,
        RegisteredBoundaryKind::S3,
    ] {
        let role = issue_operational_role_witness_v3(signature, kind)
            .map_err(|error| evidence_error("F-O1", error))?;
        replay_operational_role_witness_v3(signature, &role)
            .map_err(|error| evidence_error("F-O1", error))?;
        let overlay = issue_historical_element_overlay_token(signature, kind)
            .map_err(|error| evidence_error("F-O1", error))?;
        replay_historical_element_overlay_token(signature, &overlay)
            .map_err(|error| evidence_error("F-O1", error))?;
        let base = issue_historical_base_binding_token_v3(signature, kind)
            .map_err(|error| evidence_error("C7", error))?;
        replay_historical_base_binding_token_v3(signature, &base)
            .map_err(|error| evidence_error("C7", error))?;
        let map = registered_boundary_diagram(signature, kind)
            .map_err(|error| evidence_error("C6", error))?;
        let typed = issue_typed_declared_boundary_token_v3(signature, kind, map)
            .map_err(|error| evidence_error("F-O2", error))?;
        replay_typed_declared_boundary_token_v3(signature, &typed)
            .map_err(|error| evidence_error("F-O2", error))?;

        if role.kind() != kind
            || role.step() != kind.step()
            || role.formation_clause() != 0
            || role.element_clause() != 1
            || role.overlay_version() != HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION
            || overlay.kind() != kind
            || overlay.step() != kind.step()
            || overlay.formation_clause() != 0
            || overlay.element_clause() != 1
            || overlay.axiom_version() != ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION
            || overlay.overlay_version() != HISTORICAL_ELEMENT_DECLARATION_OVERLAY_VERSION
            || overlay.recorded_type() != &KernelTy::Type
            || overlay.overlay_type() != &KernelTy::El(overlay.owner().clone())
            || base.axiom_version() != ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION
            || base.element_overlay_derivation_hash() != Some(overlay.derivation_hash())
            || typed.axiom_version() != ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION
            || typed.element_overlay_derivation_hash() != Some(overlay.derivation_hash())
            || typed.faces().len() != (2 * kind.dimension()) as usize
        {
            return Err(evidence_error(
                "C6/C7",
                format!("{} V3 token linkage mismatch", kind_name(kind)),
            ));
        }

        let predecessor_signature = SealedSignature::from_telescopes(
            (1..kind.step())
                .map(|step| (step, Telescope::reference(step)))
                .collect(),
        );
        let current_telescope = Telescope::reference(kind.step());
        let prefix_role = issue_operational_role_witness_v3_for_historical_prefix(
            &predecessor_signature,
            kind,
            &current_telescope,
        )
        .map_err(|error| evidence_error("F-O1-prefix", error))?;
        replay_operational_role_witness_v3_for_historical_prefix(
            &predecessor_signature,
            &current_telescope,
            &prefix_role,
        )
        .map_err(|error| evidence_error("F-O1-prefix", error))?;
        let prefix_overlay = issue_historical_element_overlay_token_for_historical_prefix(
            &predecessor_signature,
            kind,
            &current_telescope,
        )
        .map_err(|error| evidence_error("F-O1-prefix", error))?;
        replay_historical_element_overlay_token_for_historical_prefix(
            &predecessor_signature,
            &current_telescope,
            &prefix_overlay,
        )
        .map_err(|error| evidence_error("F-O1-prefix", error))?;
        let prefix_base = issue_historical_base_binding_token_v3_for_historical_prefix(
            &predecessor_signature,
            kind,
            &current_telescope,
        )
        .map_err(|error| evidence_error("C7-prefix", error))?;
        replay_historical_base_binding_token_v3_for_historical_prefix(
            &predecessor_signature,
            &current_telescope,
            &prefix_base,
        )
        .map_err(|error| evidence_error("C7-prefix", error))?;
        let prefix_map = registered_boundary_diagram_for_historical_prefix(
            &predecessor_signature,
            kind,
            &current_telescope,
        )
        .map_err(|error| evidence_error("C6-prefix", error))?;
        let prefix_typed = issue_typed_declared_boundary_token_v3_for_historical_prefix(
            &predecessor_signature,
            kind,
            &current_telescope,
            prefix_map,
        )
        .map_err(|error| evidence_error("F-O2-prefix", error))?;
        replay_typed_declared_boundary_token_v3_for_historical_prefix(
            &predecessor_signature,
            &current_telescope,
            &prefix_typed,
        )
        .map_err(|error| evidence_error("F-O2-prefix", error))?;
        let prefix_bridge = issue_historical_prefix_v3_c6_bridge_token(
            &predecessor_signature,
            kind,
            &current_telescope,
        )
        .map_err(|error| evidence_error("C6-prefix-bridge", error))?;
        replay_historical_prefix_v3_c6_bridge_token(
            &predecessor_signature,
            &current_telescope,
            &prefix_bridge,
        )
        .map_err(|error| evidence_error("C6-prefix-bridge", error))?;
        let expected_basis_count = match kind {
            RegisteredBoundaryKind::S1 => 2,
            RegisteredBoundaryKind::S2 => 5,
            RegisteredBoundaryKind::S3 => 10,
            RegisteredBoundaryKind::Trunc => unreachable!(),
        };
        if prefix_role.step() != kind.step()
            || prefix_overlay.step() != kind.step()
            || prefix_base.element_overlay_derivation_hash()
                != Some(prefix_overlay.derivation_hash())
            || prefix_typed.element_overlay_derivation_hash()
                != Some(prefix_overlay.derivation_hash())
            || prefix_bridge.typed_boundary_derivation_hash() != prefix_typed.derivation_hash()
            || prefix_bridge.expected_basis_count() != expected_basis_count
            || prefix_bridge.realized_basis_count() != expected_basis_count
            || prefix_bridge.path_realization_derivation_hashes().len()
                != expected_basis_count as usize
            || prefix_bridge.key_correspondence().len() != expected_basis_count as usize
        {
            return Err(evidence_error(
                "C6/C7-prefix",
                format!(
                    "{} historical prefix token linkage mismatch",
                    kind_name(kind)
                ),
            ));
        }

        let kind_string = kind_name(kind).to_owned();
        let step = kind.step();
        let formation_clause = role.formation_clause();
        let element_clause = role.element_clause();
        let path_clause = 2;
        let recorded_type = "KernelTy::Type".to_owned();
        let overlay_owner_digest = tagged_digest("overlay-owner", overlay.owner());
        let operational_role_derivation_hash = role.derivation_hash().to_owned();
        let element_overlay_derivation_hash = overlay.derivation_hash().to_owned();
        let base_binding_derivation_hash = base.derivation_hash().to_owned();
        let typed_boundary_derivation_hash = typed.derivation_hash().to_owned();
        let typed_face_count = typed.faces().len();
        let historical_prefix_signature_digest = predecessor_signature.digest().to_owned();
        let prefix_operational_role_derivation_hash = prefix_role.derivation_hash().to_owned();
        let prefix_element_overlay_derivation_hash = prefix_overlay.derivation_hash().to_owned();
        let prefix_base_binding_derivation_hash = prefix_base.derivation_hash().to_owned();
        let prefix_typed_boundary_derivation_hash = prefix_typed.derivation_hash().to_owned();
        let prefix_c6_bridge_derivation_hash = prefix_bridge.derivation_hash().to_owned();
        let prefix_path_basis_derivation_hash =
            prefix_bridge.path_basis_derivation_hash().to_owned();
        let prefix_boundary_basis_presentation_derivation_hash = prefix_bridge
            .boundary_basis_presentation_derivation_hash()
            .to_owned();
        let prefix_key_bijection_digest = prefix_bridge.key_bijection_digest().to_owned();
        let prefix_key_correspondence_count = prefix_bridge.key_correspondence().len();
        let prefix_path_realization_root = tagged_digest(
            "prefix-path-realization-root",
            &prefix_bridge.path_realization_derivation_hashes(),
        );
        let prefix_expected_basis_count = prefix_bridge.expected_basis_count();
        let prefix_realized_basis_count = prefix_bridge.realized_basis_count();
        let f_o1_operational_role_replayed = true;
        let f_o2_term_typing_replayed = true;
        let derivation_hash = tagged_digest(
            "overlay-entry-replay-evidence",
            &(
                (
                    &kind_string,
                    step,
                    formation_clause,
                    element_clause,
                    path_clause,
                    &recorded_type,
                    &overlay_owner_digest,
                    &operational_role_derivation_hash,
                    &element_overlay_derivation_hash,
                    &base_binding_derivation_hash,
                    &typed_boundary_derivation_hash,
                    typed_face_count,
                ),
                (
                    &historical_prefix_signature_digest,
                    &prefix_operational_role_derivation_hash,
                    &prefix_element_overlay_derivation_hash,
                    &prefix_base_binding_derivation_hash,
                    &prefix_typed_boundary_derivation_hash,
                    &prefix_c6_bridge_derivation_hash,
                    &prefix_path_basis_derivation_hash,
                    &prefix_boundary_basis_presentation_derivation_hash,
                    &prefix_key_bijection_digest,
                    prefix_key_correspondence_count,
                    &prefix_path_realization_root,
                    prefix_expected_basis_count,
                    prefix_realized_basis_count,
                    f_o1_operational_role_replayed,
                    f_o2_term_typing_replayed,
                ),
            ),
        );
        evidence.push(OverlayEntryReplayEvidence {
            kind: kind_string,
            step,
            formation_clause,
            element_clause,
            path_clause,
            recorded_type,
            overlay_owner_digest,
            operational_role_derivation_hash,
            element_overlay_derivation_hash,
            base_binding_derivation_hash,
            typed_boundary_derivation_hash,
            typed_face_count,
            historical_prefix_signature_digest,
            prefix_operational_role_derivation_hash,
            prefix_element_overlay_derivation_hash,
            prefix_base_binding_derivation_hash,
            prefix_typed_boundary_derivation_hash,
            prefix_c6_bridge_derivation_hash,
            prefix_path_basis_derivation_hash,
            prefix_boundary_basis_presentation_derivation_hash,
            prefix_key_bijection_digest,
            prefix_key_correspondence_count,
            prefix_path_realization_root,
            prefix_expected_basis_count,
            prefix_realized_basis_count,
            f_o1_operational_role_replayed,
            f_o2_term_typing_replayed,
            derivation_hash,
        });
    }
    Ok(evidence)
}

fn derive_c6(
    signature: &SealedSignature,
    overlay_entries: &[OverlayEntryReplayEvidence],
) -> Result<V5ObligationEvidence, CandidateJoinV5Error> {
    let mut scoped_evidence = Vec::new();
    for kind in RegisteredBoundaryKind::ALL {
        let map = registered_boundary_diagram(signature, kind)
            .map_err(|error| evidence_error("C6", error))?;
        let token = issue_typed_declared_boundary_token_v3(signature, kind, map)
            .map_err(|error| evidence_error("C6", error))?;
        replay_typed_declared_boundary_token_v3(signature, &token)
            .map_err(|error| evidence_error("C6", error))?;
        let id = format!("C6_V3_REGISTERED_{}_BOUNDARY_TYPED", kind_name(kind));
        scoped_evidence.push(scoped(
            id.clone(),
            TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION,
            format!(
                "registered {} face and overlap terms type-check under the adopted V3 axiom; this is not general coe/hcom or method exhaustiveness",
                kind_name(kind)
            ),
            proved(id),
            token.derivation_hash(),
        ));
    }
    if overlay_entries.len() != 3 {
        return Err(evidence_error(
            "C6",
            "registered constant-boundary set is incomplete",
        ));
    }
    let mut constant_hashes = Vec::new();
    for entry in overlay_entries {
        let id = format!(
            "C6_V3_STEP_{}_PREFIX_BOUND_CONSTANT_REALIZER_BUNDLE_{}",
            entry.step, entry.prefix_realized_basis_count
        );
        scoped_evidence.push(scoped(
            id.clone(),
            HISTORICAL_PREFIX_V3_C6_BRIDGE_VERSION,
            format!(
                "the exact B{} predecessor prefix and Step {} package replay the typed constant boundary and all {} existing path-basis realizers; independence and exhaustiveness are not claimed",
                entry.step - 1,
                entry.step,
                entry.prefix_realized_basis_count
            ),
            proved(id),
            entry.prefix_c6_bridge_derivation_hash.clone(),
        ));
        constant_hashes.push(entry.prefix_c6_bridge_derivation_hash.clone());
    }
    let constant_root = tagged_digest("c6-v3-constant-boundary-root", &constant_hashes);
    scoped_evidence.push(scoped(
        C6_V3_REGISTERED_CONSTANT_PROOF,
        TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION,
        "the exact B4/B6/B7 predecessor prefixes and Steps 5/7/8 packages replay the registered constant-boundary typing and existing 2/5/10 path-basis realizer bundles",
        proved(C6_V3_REGISTERED_CONSTANT_PROOF),
        constant_root,
    ));
    scoped_evidence.push(scoped(
        C6_TRUNC_REALIZER_GAP,
        TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION,
        "Trunc's declared endpoint-dependent x/y boundary is term-typed, but no endpoint-sensitive PathCon beta/Kan realizer bundle is implemented",
        gap(C6_TRUNC_REALIZER_GAP),
        tagged_digest("c6-trunc-declared-endpoint-realizer-gap", &C6_TRUNC_REALIZER_GAP),
    ));
    scoped_evidence.push(scoped(
        C6_V3_GENERAL_GAP,
        TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION,
        "general boundary-aware coe/hcom, motive methods, substitution/weakening, independence, and exhaustiveness are not established by the registered tokens",
        gap(C6_V3_GENERAL_GAP),
        tagged_digest("c6-v3-general-gap", &C6_V3_GENERAL_GAP),
    ));
    Ok(obligation(
        "C6_V3_TERM_LEVEL_COMPLETION",
        gap(C6_V3_GENERAL_GAP),
        scoped_evidence,
    ))
}

fn derive_c7(entries: &[OverlayEntryReplayEvidence]) -> V5ObligationEvidence {
    let mut scoped_evidence = entries
        .iter()
        .map(|entry| {
            let id = format!("C7_V3_STEP_{}_BASE_BINDING_REPLAYED", entry.step);
            scoped(
                id.clone(),
                TYPED_BOUNDARY_ELEMENT_OVERLAY_FRAGMENT_VERSION,
                format!(
                    "registered {} clause {} is read as an element of formation clause {} through both the full-H15 check and the exact B{} predecessor-prefix replay",
                    entry.kind, entry.element_clause, entry.formation_clause, entry.step - 1
                ),
                proved(id),
                entry.prefix_base_binding_derivation_hash.clone(),
            )
        })
        .collect::<Vec<_>>();
    let entry_hashes = entries
        .iter()
        .map(|entry| entry.derivation_hash.clone())
        .collect::<Vec<_>>();
    scoped_evidence.push(scoped(
        C7_V3_ALL_REGISTERED_PROOF,
        ADOPTED_DECLARED_BOUNDARY_AXIOM_V3_VERSION,
        "exactly the registered Steps 5, 7, and 8 historical base bindings replay under the adopted element overlay; no candidate-generic overlay is inferred",
        proved(C7_V3_ALL_REGISTERED_PROOF),
        tagged_digest("c7-all-registered-v3", &entry_hashes),
    ));
    obligation(
        "C7_V3_HISTORICAL_BASE_BINDINGS",
        proved(C7_V3_ALL_REGISTERED_PROOF),
        scoped_evidence,
    )
}

fn derive_c8(
    parent: &CandidateJoinV4Certificate,
) -> Result<V5ObligationEvidence, CandidateJoinV5Error> {
    let registered = parent
        .theorem_bundle
        .c8_charging
        .scoped_evidence
        .iter()
        .find(|evidence| evidence.evidence_id == C8_REFERENCE_ONLY_PROOF)
        .ok_or_else(|| evidence_error("C8", "parent registered reference-only proof is absent"))?;
    if !matches!(
        registered.status,
        crate::candidate_join::EvidenceStatus::Proved { .. }
    ) {
        return Err(evidence_error(
            "C8",
            "parent registered reference-only proof is not proved",
        ));
    }
    Ok(obligation(
        "C8_V3_REFERENCE_ONLY_CHARGING",
        gap(C8_CANDIDATE_JOIN_GAP),
        vec![
            scoped(
                C8_REFERENCE_ONLY_PROOF,
                BOUNDARY_CHARGE_POLICY_VERSION,
                "the exact schema-4 registered reference-only charging proof replays unchanged under the additive overlay",
                proved(C8_REFERENCE_ONLY_PROOF),
                registered.derivation_hash.clone(),
            ),
            scoped(
                C8_CANDIDATE_JOIN_GAP,
                BOUNDARY_CHARGE_POLICY_VERSION,
                "the overlay adds registered historical bindings but no boundary-provenance inventory to schema-4 candidate rows",
                gap(C8_CANDIDATE_JOIN_GAP),
                parent.theorem_bundle.c8_charging.derivation_hash.clone(),
            ),
        ],
    ))
}

fn expected_v2_hashes(kind: RegisteredBoundaryKind) -> (&'static str, &'static str) {
    match kind {
        RegisteredBoundaryKind::S1 => (V2_S1_REFERENCE_HASH, V2_S1_BASIS_HASH),
        RegisteredBoundaryKind::Trunc => (V2_TRUNC_REFERENCE_HASH, V2_TRUNC_BASIS_HASH),
        RegisteredBoundaryKind::S2 => (V2_S2_REFERENCE_HASH, V2_S2_BASIS_HASH),
        RegisteredBoundaryKind::S3 => (V2_S3_REFERENCE_HASH, V2_S3_BASIS_HASH),
    }
}

fn derive_conservativity(
    signature: &SealedSignature,
    parent: &CandidateJoinV4Certificate,
) -> Result<V4ConservativityEvidence, CandidateJoinV5Error> {
    let trunc = parent
        .theorem_bundle
        .c6_boundary_terms
        .scoped_evidence
        .iter()
        .find(|evidence| evidence.evidence_id == "C6_TRUNC_REGISTERED_BOUNDARY_TYPED")
        .ok_or_else(|| evidence_error("F-O3", "parent Trunc typed witness is absent"))?;
    let mut unchanged_parent_evidence = vec![
        NamedDigest {
            name: "c1_substitution".to_owned(),
            digest: parent
                .theorem_bundle
                .c1_substitution
                .derivation_hash
                .clone(),
        },
        NamedDigest {
            name: "c2_naturality".to_owned(),
            digest: parent.theorem_bundle.c2_naturality.derivation_hash.clone(),
        },
        NamedDigest {
            name: "c3_fuel".to_owned(),
            digest: parent.theorem_bundle.c3_fuel.derivation_hash.clone(),
        },
        NamedDigest {
            name: "c5_window".to_owned(),
            digest: parent.theorem_bundle.c5_window.derivation_hash.clone(),
        },
        NamedDigest {
            name: "p5_window_reaudit".to_owned(),
            digest: parent
                .theorem_bundle
                .p5_window_reaudit
                .derivation_hash
                .clone(),
        },
        NamedDigest {
            name: "c6_trunc_registered_boundary_typed".to_owned(),
            digest: trunc.derivation_hash.clone(),
        },
        NamedDigest {
            name: "c8_charging".to_owned(),
            digest: parent.theorem_bundle.c8_charging.derivation_hash.clone(),
        },
    ];
    for kind in RegisteredBoundaryKind::ALL {
        let map = registered_boundary_diagram(signature, kind)
            .map_err(|error| evidence_error("F-O3", error))?;
        let reference = issue_reference_only_charge_token(signature, kind, map.clone())
            .map_err(|error| evidence_error("F-O3", error))?;
        replay_reference_only_charge_token(signature, &reference)
            .map_err(|error| evidence_error("F-O3", error))?;
        let basis = issue_adopted_boundary_basis_charge_token(signature, kind, map.clone())
            .map_err(|error| evidence_error("F-O3", error))?;
        replay_adopted_boundary_basis_charge_token(signature, &basis)
            .map_err(|error| evidence_error("F-O3", error))?;
        let (expected_reference, expected_basis) = expected_v2_hashes(kind);
        if reference.derivation_hash() != expected_reference
            || basis.derivation_hash() != expected_basis
        {
            return Err(evidence_error(
                "F-O3",
                format!("{} archival V2 charge hash drift", kind_name(kind)),
            ));
        }
        unchanged_parent_evidence.push(NamedDigest {
            name: format!("v2_{}_reference_only_token", kind_name(kind)),
            digest: reference.derivation_hash().to_owned(),
        });
        unchanged_parent_evidence.push(NamedDigest {
            name: format!("v2_{}_basis_charge_token", kind_name(kind)),
            digest: basis.derivation_hash().to_owned(),
        });
        if kind == RegisteredBoundaryKind::Trunc {
            let typed = issue_typed_declared_boundary_token(signature, kind, map)
                .map_err(|error| evidence_error("F-O3", error))?;
            replay_typed_declared_boundary_token(signature, &typed)
                .map_err(|error| evidence_error("F-O3", error))?;
            if typed.derivation_hash() != V2_TRUNC_TYPED_HASH {
                return Err(evidence_error("F-O3", "Trunc archival V2 typed hash drift"));
            }
            unchanged_parent_evidence.push(NamedDigest {
                name: "v2_trunc_typed_boundary_token".to_owned(),
                digest: typed.derivation_hash().to_owned(),
            });
        }
    }
    let exact_schema4_bytes_replayed = true;
    let all_213_parent_rows_preserved =
        parent.strata.iter().map(|s| s.rows.len()).sum::<usize>() == 213;
    let v2_obstruction_remains_available = [
        RegisteredBoundaryKind::S1,
        RegisteredBoundaryKind::S2,
        RegisteredBoundaryKind::S3,
    ]
    .into_iter()
    .all(|kind| {
        matches!(
            audit_historical_base_binding(signature, kind),
            Ok(HistoricalBaseBindingAudit::Obstructed(obstruction))
                if obstruction.obstruction_id() == HISTORICAL_POINT_TYPING_OBSTRUCTION_ID
        )
    });
    let scope = "exact schema-4 bytes, all 213 parent rows, C1/C2/C3/C5/P5, the V2 Trunc control, and C8; the separate HIST-CERT V3 rerun remains F-B2".to_owned();
    let derivation_hash = tagged_digest(
        "v4-conservativity-evidence",
        &(
            exact_schema4_bytes_replayed,
            all_213_parent_rows_preserved,
            &unchanged_parent_evidence,
            v2_obstruction_remains_available,
            &scope,
            &parent.digest,
        ),
    );
    Ok(V4ConservativityEvidence {
        exact_schema4_bytes_replayed,
        all_213_parent_rows_preserved,
        unchanged_parent_evidence,
        v2_obstruction_remains_available,
        scope,
        derivation_hash,
    })
}

fn v5_row_status(obligation: &V5ObligationEvidence, row_digest: &str) -> V5RowObligationStatus {
    let evidence_hash = tagged_digest(
        "v5-row-obligation",
        &(
            row_digest,
            &obligation.obligation_id,
            &obligation.status,
            &obligation.derivation_hash,
        ),
    );
    match &obligation.status {
        V5EvidenceStatus::Proved { proof_id } => V5RowObligationStatus::Proved {
            proof_id: proof_id.clone(),
            evidence_hash,
        },
        V5EvidenceStatus::Gap { gap_id } => V5RowObligationStatus::Gap {
            gap_id: gap_id.clone(),
            evidence_hash,
        },
    }
}

fn v5_not_applicable(id: &str, row_digest: &str) -> V5RowObligationStatus {
    let reason_id = "NON_HIT_SCHEMA4_ROW".to_owned();
    let evidence_hash = tagged_digest("v5-row-not-applicable", &(row_digest, id, &reason_id));
    V5RowObligationStatus::NotApplicable {
        reason_id,
        evidence_hash,
    }
}

fn build_rows(
    parent: &CandidateJoinV4Certificate,
    c6: &V5ObligationEvidence,
    c7: &V5ObligationEvidence,
    c8: &V5ObligationEvidence,
) -> Result<Vec<BridgeStratumV5>, CandidateJoinV5Error> {
    let mut strata = Vec::new();
    for (stratum_index, parent_stratum) in parent.strata.iter().enumerate() {
        let mut rows = Vec::new();
        for (row_index, parent_row) in parent_stratum.rows.iter().enumerate() {
            if usize::from(parent_row.stratum_index) != stratum_index
                || usize::from(parent_row.row_index) != row_index
                || parent_row.kappa != parent_stratum.kappa
            {
                return Err(CandidateJoinV5Error::RowJoin(format!(
                    "parent row index mismatch at {stratum_index}/{row_index}"
                )));
            }
            if parent_row.all_required_evidence_proved
                || parent_row.implemented_egp_bridge_returns_bound
                || parent_row.intended_typed_egp_proved
                || parent_row.full_candidate_extraction_join_proved
            {
                return Err(CandidateJoinV5Error::RowJoin(format!(
                    "parent row {stratum_index}/{row_index} is unexpectedly promoted"
                )));
            }
            let is_hit = parent_row.class == "Hit";
            let c6_status = if is_hit {
                v5_row_status(c6, &parent_row.schema3_row_digest)
            } else {
                v5_not_applicable("C6", &parent_row.schema3_row_digest)
            };
            let c7_status = if is_hit {
                v5_row_status(c7, &parent_row.schema3_row_digest)
            } else {
                v5_not_applicable("C7", &parent_row.schema3_row_digest)
            };
            let c8_status = if is_hit {
                v5_row_status(c8, &parent_row.schema3_row_digest)
            } else {
                v5_not_applicable("C8", &parent_row.schema3_row_digest)
            };
            let all_required_evidence_proved = false;
            let implemented_egp_bridge_returns_bound = false;
            let intended_typed_egp_proved = false;
            let full_candidate_extraction_join_proved = false;
            let derivation_hash = tagged_digest(
                "candidate-bridge-v5-row",
                &(
                    stratum_index,
                    row_index,
                    &parent_row.schema3_row_digest,
                    &parent_row.derivation_hash,
                    &parent_row.c6_boundary_terms,
                    &parent_row.c7_base_bindings,
                    &parent_row.c8_charging,
                    &c6_status,
                    &c7_status,
                    &c8_status,
                    all_required_evidence_proved,
                    implemented_egp_bridge_returns_bound,
                    intended_typed_egp_proved,
                    full_candidate_extraction_join_proved,
                ),
            );
            rows.push(CandidateBridgeV5Row {
                stratum_index: parent_row.stratum_index,
                row_index: parent_row.row_index,
                kappa: parent_row.kappa,
                class: parent_row.class.clone(),
                schema3_row_digest: parent_row.schema3_row_digest.clone(),
                schema4_row_derivation_hash: parent_row.derivation_hash.clone(),
                prior_c6_boundary_terms: parent_row.c6_boundary_terms.clone(),
                prior_c7_base_bindings: parent_row.c7_base_bindings.clone(),
                prior_c8_charging: parent_row.c8_charging.clone(),
                c6_boundary_terms_under_v3: c6_status,
                c7_registered_historical_base_bindings_under_v3: c7_status,
                c8_charging_under_v3: c8_status,
                all_required_evidence_proved,
                implemented_egp_bridge_returns_bound,
                intended_typed_egp_proved,
                full_candidate_extraction_join_proved,
                derivation_hash,
            });
        }
        let all_parent_rows_joined = rows.len() == parent_stratum.rows.len()
            && rows.iter().zip(&parent_stratum.rows).all(|(row, parent)| {
                row.schema3_row_digest == parent.schema3_row_digest
                    && row.schema4_row_derivation_hash == parent.derivation_hash
            });
        let no_row_promoted = rows.iter().all(|row| {
            !row.all_required_evidence_proved
                && !row.implemented_egp_bridge_returns_bound
                && !row.intended_typed_egp_proved
                && !row.full_candidate_extraction_join_proved
        });
        let derivation_hash = tagged_digest(
            "bridge-stratum-v5",
            &(
                stratum_index,
                parent_stratum.kappa,
                &rows,
                all_parent_rows_joined,
                no_row_promoted,
            ),
        );
        strata.push(BridgeStratumV5 {
            stratum_index: parent_stratum.stratum_index,
            kappa: parent_stratum.kappa,
            rows,
            all_parent_rows_joined,
            no_row_promoted,
            derivation_hash,
        });
    }
    Ok(strata)
}

fn certificate_digest(certificate: &CandidateJoinV5Certificate) -> String {
    tagged_digest(
        "candidate-join-v5-certificate",
        &(
            certificate.schema_version,
            &certificate.date,
            &certificate.schema4_archive,
            &certificate.boundary_axiom_v3,
            &certificate.overlay_entries,
            &certificate.v4_conservativity,
            &certificate.c6_boundary_terms,
            &certificate.c7_base_bindings,
            &certificate.c8_charging,
            &certificate.falsifiers,
            &certificate.strata,
            &certificate.completeness,
        ),
    )
}

pub fn run_candidate_join_v5(
    schema3_json: &[u8],
    schema4_json: &[u8],
) -> Result<CandidateJoinV5Certificate, CandidateJoinV5Error> {
    let (parent, schema4_archive) = validate_parent(schema3_json, schema4_json)?;
    let boundary_axiom_v3 = derive_axiom_binding()?;
    let signature = SealedSignature::genesis_del_h15();
    let overlay_entries = derive_overlay_entries(&signature)?;
    let c6_boundary_terms = derive_c6(&signature, &overlay_entries)?;
    let c7_base_bindings = derive_c7(&overlay_entries);
    let c8_charging = derive_c8(&parent)?;
    let v4_conservativity = derive_conservativity(&signature, &parent)?;
    let falsifiers = {
        let f_o1_overlay_role_witnesses =
            proved("F-O1-DISCHARGED-BY-FULL-H15-AND-B4-B6-B7-PREFIX-ROLE-REPLAYS");
        let f_o2_registered_term_typing =
            proved("F-O2-DISCHARGED-BY-FULL-H15-AND-B4-B6-B7-PREFIX-TYPED-REPLAYS");
        let f_o3_exact_schema4_conservativity =
            proved("F-O3-EXACT-SCHEMA4-CONSERVATIVITY-DISCHARGED");
        let f_b2_hist_cert_v3_rerun = gap(F_B2_PENDING_GAP);
        let derivation_hash = tagged_digest(
            "falsifier-ledger-v5",
            &(
                &f_o1_overlay_role_witnesses,
                &f_o2_registered_term_typing,
                &f_o3_exact_schema4_conservativity,
                &f_b2_hist_cert_v3_rerun,
                &overlay_entries,
                &v4_conservativity,
            ),
        );
        FalsifierLedgerV5 {
            f_o1_overlay_role_witnesses,
            f_o2_registered_term_typing,
            f_o3_exact_schema4_conservativity,
            f_b2_hist_cert_v3_rerun,
            derivation_hash,
        }
    };
    let strata = build_rows(&parent, &c6_boundary_terms, &c7_base_bindings, &c8_charging)?;
    let all_213_schema4_rows_joined_by_row_digest = strata
        .iter()
        .map(|stratum| stratum.rows.len())
        .sum::<usize>()
        == 213
        && strata.iter().all(|stratum| stratum.all_parent_rows_joined);
    let no_row_promoted = strata.iter().all(|stratum| stratum.no_row_promoted);
    let completeness = CandidateJoinV5Completeness {
        exact_schema4_archive_bound_and_replayed: true,
        all_213_schema4_rows_joined_by_row_digest,
        f_o1_discharged: falsifiers.f_o1_overlay_role_witnesses.is_proved(),
        f_o2_discharged: falsifiers.f_o2_registered_term_typing.is_proved(),
        f_o3_exact_schema4_scope_discharged: falsifiers
            .f_o3_exact_schema4_conservativity
            .is_proved(),
        f_b2_hist_cert_v3_rerun_discharged: falsifiers.f_b2_hist_cert_v3_rerun.is_proved(),
        c6_registered_constant_boundary_terms_typed: c6_boundary_terms.scoped_evidence.iter().any(
            |evidence| {
                evidence.evidence_id == C6_V3_REGISTERED_CONSTANT_PROOF
                    && evidence.status.is_proved()
            },
        ),
        c6_general_historical_term_completion_proved: c6_boundary_terms.status.is_proved(),
        c7_all_registered_historical_base_bindings_proved: c7_base_bindings.status.is_proved(),
        c8_registered_reference_only_charging_proved: c8_charging.scoped_evidence.iter().any(
            |evidence| {
                evidence.evidence_id == C8_REFERENCE_ONLY_PROOF && evidence.status.is_proved()
            },
        ),
        c8_candidate_boundary_provenance_join_proved: c8_charging.status.is_proved(),
        no_row_promoted,
        full_candidate_extraction_join_proved: false,
        intended_semantic_candidate_join_proved: false,
    };
    let mut certificate = CandidateJoinV5Certificate {
        schema_version: CANDIDATE_JOIN_V5_SCHEMA_VERSION,
        date: CANDIDATE_JOIN_V5_DATE.to_owned(),
        schema4_archive,
        boundary_axiom_v3,
        overlay_entries,
        v4_conservativity,
        c6_boundary_terms,
        c7_base_bindings,
        c8_charging,
        falsifiers,
        strata,
        completeness,
        digest: String::new(),
    };
    certificate.digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn candidate_join_v5_json_pretty(
    schema3_json: &[u8],
    schema4_json: &[u8],
) -> Result<String, CandidateJoinV5Error> {
    serde_json::to_string_pretty(&run_candidate_join_v5(schema3_json, schema4_json)?)
        .map_err(|error| CandidateJoinV5Error::InvalidSchema5Json(error.to_string()))
}

fn replay_against_expected(
    certificate: &CandidateJoinV5Certificate,
    expected: Result<CandidateJoinV5Certificate, CandidateJoinV5Error>,
) -> CandidateJoinV5Replay {
    let mut errors = Vec::new();
    match expected {
        Ok(expected) => {
            if certificate != &expected {
                errors.push("certificate differs from definition recomputation".to_owned());
            }
            if certificate.digest != certificate_digest(certificate) {
                errors.push("certificate digest mismatch".to_owned());
            }
        }
        Err(error) => errors.push(error.to_string()),
    }
    CandidateJoinV5Replay {
        valid: errors.is_empty(),
        exact_schema4_archive_bound_and_replayed: certificate
            .completeness
            .exact_schema4_archive_bound_and_replayed,
        all_213_schema4_rows_joined_by_row_digest: certificate
            .completeness
            .all_213_schema4_rows_joined_by_row_digest,
        c7_all_registered_historical_base_bindings_proved: certificate
            .completeness
            .c7_all_registered_historical_base_bindings_proved,
        c6_general_historical_term_completion_proved: certificate
            .completeness
            .c6_general_historical_term_completion_proved,
        c8_candidate_boundary_provenance_join_proved: certificate
            .completeness
            .c8_candidate_boundary_provenance_join_proved,
        no_row_promoted: certificate.completeness.no_row_promoted,
        errors,
    }
}

pub fn replay_candidate_join_v5(
    schema3_json: &[u8],
    schema4_json: &[u8],
    certificate: &CandidateJoinV5Certificate,
) -> CandidateJoinV5Replay {
    replay_against_expected(
        certificate,
        run_candidate_join_v5(schema3_json, schema4_json),
    )
}

pub fn replay_candidate_join_v5_json(
    schema3_json: &[u8],
    schema4_json: &[u8],
    schema5_json: &str,
) -> CandidateJoinV5Replay {
    let invalid = |detail: String| CandidateJoinV5Replay {
        valid: false,
        exact_schema4_archive_bound_and_replayed: false,
        all_213_schema4_rows_joined_by_row_digest: false,
        c7_all_registered_historical_base_bindings_proved: false,
        c6_general_historical_term_completion_proved: false,
        c8_candidate_boundary_provenance_join_proved: false,
        no_row_promoted: false,
        errors: vec![format!("invalid schema-5 JSON: {detail}")],
    };
    let raw = match serde_json::from_str::<serde_json::Value>(schema5_json) {
        Ok(raw) => raw,
        Err(error) => return invalid(error.to_string()),
    };
    // Decode the original token stream as well as the Value projection.
    // Struct decoding rejects duplicate recognized fields, while Value-first
    // decoding alone would collapse them before the duplicate could be seen.
    let certificate = match serde_json::from_str::<CandidateJoinV5Certificate>(schema5_json) {
        Ok(certificate) => certificate,
        Err(error) => return invalid(error.to_string()),
    };
    let projected = match serde_json::to_value(&certificate) {
        Ok(projected) => projected,
        Err(error) => return invalid(error.to_string()),
    };
    // Several rows intentionally embed archival schema-4 enums. Serde's
    // default struct decoding ignores unknown object fields, including fields
    // nested inside those imported types. Requiring lossless projection makes
    // every submitted JSON field replay-bound without changing the archival
    // schema-4 definitions or bytes.
    if raw != projected {
        return invalid("unknown or ignored field changes the raw JSON projection".to_owned());
    }
    replay_candidate_join_v5(schema3_json, schema4_json, &certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected() -> CandidateJoinV5Certificate {
        run_candidate_join_v5(EMBEDDED_SCHEMA3_ARCHIVE, EMBEDDED_SCHEMA4_ARCHIVE).expect("schema 5")
    }

    fn reject(expected: &CandidateJoinV5Certificate, mut mutation: CandidateJoinV5Certificate) {
        mutation.digest = certificate_digest(&mutation);
        assert!(!replay_against_expected(&mutation, Ok(expected.clone())).valid);
    }

    fn flip(status: &mut V5EvidenceStatus) {
        *status = match status {
            V5EvidenceStatus::Proved { .. } => gap("mutated-gap"),
            V5EvidenceStatus::Gap { .. } => proved("mutated-proof"),
        };
    }

    fn obligation_mut(
        certificate: &mut CandidateJoinV5Certificate,
        index: usize,
    ) -> &mut V5ObligationEvidence {
        match index {
            0 => &mut certificate.c6_boundary_terms,
            1 => &mut certificate.c7_base_bindings,
            2 => &mut certificate.c8_charging,
            _ => panic!("obligation index out of range"),
        }
    }

    #[test]
    fn exact_parent_overlay_statuses_and_rows_replay() {
        let certificate = expected();
        assert_eq!(certificate.schema4_archive.total_rows, 213);
        assert_eq!(certificate.overlay_entries.len(), 3);
        assert_eq!(
            certificate
                .overlay_entries
                .iter()
                .map(|entry| entry.step)
                .collect::<Vec<_>>(),
            vec![5, 7, 8]
        );
        assert_eq!(
            certificate
                .overlay_entries
                .iter()
                .map(|entry| entry.prefix_realized_basis_count)
                .collect::<Vec<_>>(),
            vec![2, 5, 10]
        );
        assert!(certificate.overlay_entries.iter().all(|entry| {
            entry.prefix_key_correspondence_count == entry.prefix_realized_basis_count as usize
                && !entry.prefix_key_bijection_digest.is_empty()
        }));
        assert!(
            certificate
                .c6_boundary_terms
                .scoped_evidence
                .iter()
                .any(|evidence| evidence.evidence_id == C6_TRUNC_REALIZER_GAP
                    && matches!(evidence.status, V5EvidenceStatus::Gap { .. }))
        );
        assert!(certificate.c7_base_bindings.status.is_proved());
        assert!(!certificate.c6_boundary_terms.status.is_proved());
        assert!(!certificate.c8_charging.status.is_proved());
        assert!(certificate.completeness.f_o1_discharged);
        assert!(certificate.completeness.f_o2_discharged);
        assert!(certificate.completeness.f_o3_exact_schema4_scope_discharged);
        assert!(!certificate.completeness.f_b2_hist_cert_v3_rerun_discharged);
        assert!(
            certificate
                .completeness
                .c6_registered_constant_boundary_terms_typed
        );
        assert!(
            !certificate
                .completeness
                .c6_general_historical_term_completion_proved
        );
        assert!(
            certificate
                .completeness
                .c7_all_registered_historical_base_bindings_proved
        );
        assert!(
            certificate
                .completeness
                .c8_registered_reference_only_charging_proved
        );
        assert!(
            !certificate
                .completeness
                .c8_candidate_boundary_provenance_join_proved
        );
        assert!(certificate.completeness.no_row_promoted);
        assert!(
            !certificate
                .completeness
                .full_candidate_extraction_join_proved
        );
        assert!(
            !certificate
                .completeness
                .intended_semantic_candidate_join_proved
        );
        let rows = certificate
            .strata
            .iter()
            .flat_map(|stratum| &stratum.rows)
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 213);
        assert_eq!(rows.iter().filter(|row| row.class == "Hit").count(), 27);
        assert!(rows.iter().all(|row| {
            !row.all_required_evidence_proved
                && !row.implemented_egp_bridge_returns_bound
                && !row.intended_typed_egp_proved
                && !row.full_candidate_extraction_join_proved
        }));
        assert!(
            replay_candidate_join_v5(
                EMBEDDED_SCHEMA3_ARCHIVE,
                EMBEDDED_SCHEMA4_ARCHIVE,
                &certificate
            )
            .valid
        );
    }

    #[test]
    fn runtime_parent_bytes_are_exact() {
        let mut schema4 = EMBEDDED_SCHEMA4_ARCHIVE.to_vec();
        schema4[0] ^= 1;
        assert!(matches!(
            run_candidate_join_v5(EMBEDDED_SCHEMA3_ARCHIVE, &schema4),
            Err(CandidateJoinV5Error::ArchiveBytesDiffer)
        ));
        let mut schema3 = EMBEDDED_SCHEMA3_ARCHIVE.to_vec();
        schema3[0] ^= 1;
        assert!(matches!(
            run_candidate_join_v5(&schema3, EMBEDDED_SCHEMA4_ARCHIVE),
            Err(CandidateJoinV5Error::Schema3BytesDiffer)
        ));
    }

    #[test]
    fn every_overlay_entry_field_is_replay_bound() {
        let expected = expected();
        for entry_index in 0..3 {
            for field in 0..28 {
                let mut mutation = expected.clone();
                let entry = &mut mutation.overlay_entries[entry_index];
                match field {
                    0 => entry.kind.push_str(":mutated"),
                    1 => entry.step += 1,
                    2 => entry.formation_clause += 1,
                    3 => entry.element_clause += 1,
                    4 => entry.path_clause += 1,
                    5 => entry.recorded_type.push_str(":mutated"),
                    6 => entry.overlay_owner_digest.push_str(":mutated"),
                    7 => entry.operational_role_derivation_hash.push_str(":mutated"),
                    8 => entry.element_overlay_derivation_hash.push_str(":mutated"),
                    9 => entry.base_binding_derivation_hash.push_str(":mutated"),
                    10 => entry.typed_boundary_derivation_hash.push_str(":mutated"),
                    11 => entry.typed_face_count += 1,
                    12 => entry
                        .historical_prefix_signature_digest
                        .push_str(":mutated"),
                    13 => entry
                        .prefix_operational_role_derivation_hash
                        .push_str(":mutated"),
                    14 => entry
                        .prefix_element_overlay_derivation_hash
                        .push_str(":mutated"),
                    15 => entry
                        .prefix_base_binding_derivation_hash
                        .push_str(":mutated"),
                    16 => entry
                        .prefix_typed_boundary_derivation_hash
                        .push_str(":mutated"),
                    17 => entry.prefix_c6_bridge_derivation_hash.push_str(":mutated"),
                    18 => entry.prefix_path_basis_derivation_hash.push_str(":mutated"),
                    19 => entry
                        .prefix_boundary_basis_presentation_derivation_hash
                        .push_str(":mutated"),
                    20 => entry.prefix_key_bijection_digest.push_str(":mutated"),
                    21 => entry.prefix_key_correspondence_count += 1,
                    22 => entry.prefix_path_realization_root.push_str(":mutated"),
                    23 => entry.prefix_expected_basis_count += 1,
                    24 => entry.prefix_realized_basis_count += 1,
                    25 => entry.f_o1_operational_role_replayed = false,
                    26 => entry.f_o2_term_typing_replayed = false,
                    27 => entry.derivation_hash.push_str(":mutated"),
                    _ => unreachable!(),
                }
                reject(&expected, mutation);
            }
        }
    }

    #[test]
    fn axiom_parent_conservativity_and_falsifier_fields_are_replay_bound() {
        let expected = expected();
        for field in 0..18 {
            let mut mutation = expected.clone();
            match field {
                0 => mutation.schema4_archive.exact_file.byte_length += 1,
                1 => mutation
                    .schema4_archive
                    .exact_file
                    .sha256
                    .push_str(":mutated"),
                2 => mutation
                    .schema4_archive
                    .exact_file
                    .git_blob
                    .push_str(":mutated"),
                3 => mutation
                    .schema4_archive
                    .exact_file
                    .internal_digest
                    .as_mut()
                    .expect("schema-4 internal digest")
                    .push_str(":mutated"),
                4 => mutation
                    .schema4_archive
                    .exact_file
                    .derivation_hash
                    .push_str(":mutated"),
                5 => mutation.schema4_archive.schema_version += 1,
                6 => mutation.schema4_archive.date.push_str(":mutated"),
                7 => mutation.schema4_archive.schema3_sha256.push_str(":mutated"),
                8 => mutation
                    .schema4_archive
                    .schema3_internal_digest
                    .push_str(":mutated"),
                9 => mutation.schema4_archive.stratum_row_counts[0] += 1,
                10 => mutation.schema4_archive.total_rows += 1,
                11 => mutation.schema4_archive.exact_definition_replay = false,
                12 => mutation
                    .schema4_archive
                    .derivation_hash
                    .push_str(":mutated"),
                13 => mutation.boundary_axiom_v3.status.push_str(":mutated"),
                14 => mutation
                    .boundary_axiom_v3
                    .predecessor_axiom_version
                    .push_str(":mutated"),
                15 => mutation
                    .boundary_axiom_v3
                    .adopted_axiom_version
                    .push_str(":mutated"),
                16 => mutation
                    .boundary_axiom_v3
                    .overlay_version
                    .push_str(":mutated"),
                17 => mutation
                    .boundary_axiom_v3
                    .implementation_version
                    .push_str(":mutated"),
                _ => unreachable!(),
            }
            reject(&expected, mutation);
        }

        for field in 0..17 {
            let mut mutation = expected.clone();
            match field {
                0 => mutation
                    .boundary_axiom_v3
                    .registered_diagrams_version
                    .push_str(":mutated"),
                1 => mutation
                    .boundary_axiom_v3
                    .charging_policy_version
                    .push_str(":mutated"),
                2 => mutation.boundary_axiom_v3.registered_entries[0]
                    .kind
                    .push_str(":mutated"),
                3 => mutation.boundary_axiom_v3.registered_entries[0].step += 1,
                4 => mutation.boundary_axiom_v3.registered_entries[0].formation_clause += 1,
                5 => mutation.boundary_axiom_v3.registered_entries[0].element_clause += 1,
                6 => {
                    mutation.boundary_axiom_v3.registered_entries.pop();
                }
                7 => mutation.boundary_axiom_v3.adjudication.byte_length += 1,
                8 => mutation
                    .boundary_axiom_v3
                    .adjudication
                    .sha256
                    .push_str(":mutated"),
                9 => mutation
                    .boundary_axiom_v3
                    .adjudication
                    .git_blob
                    .push_str(":mutated"),
                10 => {
                    mutation.boundary_axiom_v3.adjudication.internal_digest =
                        Some("mutated".to_owned())
                }
                11 => mutation
                    .boundary_axiom_v3
                    .adjudication
                    .derivation_hash
                    .push_str(":mutated"),
                12 => mutation
                    .boundary_axiom_v3
                    .derivation_hash
                    .push_str(":mutated"),
                13 => mutation.v4_conservativity.exact_schema4_bytes_replayed = false,
                14 => mutation.v4_conservativity.all_213_parent_rows_preserved = false,
                15 => mutation.v4_conservativity.v2_obstruction_remains_available = false,
                16 => mutation.v4_conservativity.scope.push_str(":mutated"),
                _ => unreachable!(),
            }
            reject(&expected, mutation);
        }

        for evidence_index in 0..expected.v4_conservativity.unchanged_parent_evidence.len() {
            for field in 0..2 {
                let mut mutation = expected.clone();
                let evidence =
                    &mut mutation.v4_conservativity.unchanged_parent_evidence[evidence_index];
                match field {
                    0 => evidence.name.push_str(":mutated"),
                    1 => evidence.digest.push_str(":mutated"),
                    _ => unreachable!(),
                }
                reject(&expected, mutation);
            }
        }
        let mut conservativity_hash = expected.clone();
        conservativity_hash
            .v4_conservativity
            .derivation_hash
            .push_str(":mutated");
        reject(&expected, conservativity_hash);

        for field in 0..5 {
            let mut mutation = expected.clone();
            match field {
                0 => flip(&mut mutation.falsifiers.f_o1_overlay_role_witnesses),
                1 => flip(&mut mutation.falsifiers.f_o2_registered_term_typing),
                2 => flip(&mut mutation.falsifiers.f_o3_exact_schema4_conservativity),
                3 => flip(&mut mutation.falsifiers.f_b2_hist_cert_v3_rerun),
                4 => mutation.falsifiers.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            reject(&expected, mutation);
        }
    }

    #[test]
    fn every_obligation_and_scoped_evidence_field_is_replay_bound() {
        let expected = expected();
        for obligation_index in 0..3 {
            for field in 0..3 {
                let mut mutation = expected.clone();
                match field {
                    0 => obligation_mut(&mut mutation, obligation_index)
                        .obligation_id
                        .push_str(":mutated"),
                    1 => flip(&mut obligation_mut(&mut mutation, obligation_index).status),
                    2 => obligation_mut(&mut mutation, obligation_index)
                        .derivation_hash
                        .push_str(":mutated"),
                    _ => unreachable!(),
                }
                reject(&expected, mutation);
            }
            let count = obligation_mut(&mut expected.clone(), obligation_index)
                .scoped_evidence
                .len();
            for evidence_index in 0..count {
                for field in 0..6 {
                    let mut mutation = expected.clone();
                    let evidence = &mut obligation_mut(&mut mutation, obligation_index)
                        .scoped_evidence[evidence_index];
                    match field {
                        0 => evidence.evidence_id.push_str(":mutated"),
                        1 => evidence.implementation_version.push_str(":mutated"),
                        2 => evidence.scope.push_str(":mutated"),
                        3 => flip(&mut evidence.status),
                        4 => evidence.kernel_derivation_hash.push_str(":mutated"),
                        5 => evidence.derivation_hash.push_str(":mutated"),
                        _ => unreachable!(),
                    }
                    reject(&expected, mutation);
                }
            }
        }
    }

    #[test]
    fn every_row_status_hash_and_promotion_flag_is_replay_bound() {
        let expected = expected();
        for stratum_index in 0..expected.strata.len() {
            for row_index in 0..expected.strata[stratum_index].rows.len() {
                for field in 0..17 {
                    let mut mutation = expected.clone();
                    let row = &mut mutation.strata[stratum_index].rows[row_index];
                    match field {
                        0 => row.stratum_index += 1,
                        1 => row.row_index += 1,
                        2 => row.kappa += 1,
                        3 => row.class.push_str(":mutated"),
                        4 => row.schema3_row_digest.push_str(":mutated"),
                        5 => row.schema4_row_derivation_hash.push_str(":mutated"),
                        6 => {
                            row.prior_c6_boundary_terms = RowObligationStatus::NotApplicable {
                                reason_id: "mutated".to_owned(),
                                evidence_hash: "mutated".to_owned(),
                            }
                        }
                        7 => {
                            row.prior_c7_base_bindings = RowObligationStatus::NotApplicable {
                                reason_id: "mutated".to_owned(),
                                evidence_hash: "mutated".to_owned(),
                            }
                        }
                        8 => {
                            row.prior_c8_charging = RowObligationStatus::NotApplicable {
                                reason_id: "mutated".to_owned(),
                                evidence_hash: "mutated".to_owned(),
                            }
                        }
                        9 => {
                            row.c6_boundary_terms_under_v3 = V5RowObligationStatus::Proved {
                                proof_id: "mutated".to_owned(),
                                evidence_hash: "mutated".to_owned(),
                            }
                        }
                        10 => {
                            row.c7_registered_historical_base_bindings_under_v3 =
                                V5RowObligationStatus::Gap {
                                    gap_id: "mutated".to_owned(),
                                    evidence_hash: "mutated".to_owned(),
                                }
                        }
                        11 => {
                            row.c8_charging_under_v3 = V5RowObligationStatus::Proved {
                                proof_id: "mutated".to_owned(),
                                evidence_hash: "mutated".to_owned(),
                            }
                        }
                        12 => row.all_required_evidence_proved = true,
                        13 => row.implemented_egp_bridge_returns_bound = true,
                        14 => row.intended_typed_egp_proved = true,
                        15 => row.full_candidate_extraction_join_proved = true,
                        16 => row.derivation_hash.push_str(":mutated"),
                        _ => unreachable!(),
                    }
                    reject(&expected, mutation);
                }
            }
        }

        for stratum_index in 0..expected.strata.len() {
            for field in 0..5 {
                let mut mutation = expected.clone();
                let stratum = &mut mutation.strata[stratum_index];
                match field {
                    0 => stratum.stratum_index += 1,
                    1 => stratum.kappa += 1,
                    2 => stratum.all_parent_rows_joined = false,
                    3 => stratum.no_row_promoted = false,
                    4 => stratum.derivation_hash.push_str(":mutated"),
                    _ => unreachable!(),
                }
                reject(&expected, mutation);
            }
        }
    }

    #[test]
    fn completeness_and_outer_digest_mutations_fail() {
        let expected = expected();
        for field in 0..16 {
            let mut mutation = expected.clone();
            let c = &mut mutation.completeness;
            match field {
                0 => c.exact_schema4_archive_bound_and_replayed = false,
                1 => c.all_213_schema4_rows_joined_by_row_digest = false,
                2 => c.f_o1_discharged = false,
                3 => c.f_o2_discharged = false,
                4 => c.f_o3_exact_schema4_scope_discharged = false,
                5 => c.f_b2_hist_cert_v3_rerun_discharged = true,
                6 => c.c6_registered_constant_boundary_terms_typed = false,
                7 => c.c6_general_historical_term_completion_proved = true,
                8 => c.c7_all_registered_historical_base_bindings_proved = false,
                9 => c.c8_registered_reference_only_charging_proved = false,
                10 => c.c8_candidate_boundary_provenance_join_proved = true,
                11 => c.no_row_promoted = false,
                12 => c.full_candidate_extraction_join_proved = true,
                13 => c.intended_semantic_candidate_join_proved = true,
                14 => mutation.schema_version += 1,
                15 => mutation.date.push_str(":mutated"),
                _ => unreachable!(),
            }
            reject(&expected, mutation);
        }
        let mut mutation = expected.clone();
        mutation.digest.push_str(":mutated");
        assert!(!replay_against_expected(&mutation, Ok(expected)).valid);
    }

    #[test]
    fn json_replay_rejects_unknown_top_level_and_deep_archival_fields() {
        let certificate = expected();
        let canonical = serde_json::to_value(&certificate).expect("schema-5 JSON value");

        let mut top_level = canonical.clone();
        top_level
            .as_object_mut()
            .expect("certificate object")
            .insert(
                "unexpected_unbound_claim".to_owned(),
                serde_json::Value::Bool(true),
            );
        let top_level_json = serde_json::to_string(&top_level).expect("mutated JSON");
        assert!(
            !replay_candidate_join_v5_json(
                EMBEDDED_SCHEMA3_ARCHIVE,
                EMBEDDED_SCHEMA4_ARCHIVE,
                &top_level_json,
            )
            .valid
        );

        let mut deep = canonical;
        deep["strata"][0]["rows"][0]["prior_c7_base_bindings"]
            .as_object_mut()
            .expect("archival row status object")
            .insert(
                "unexpected_nested_claim".to_owned(),
                serde_json::Value::Bool(true),
            );
        let deep_json = serde_json::to_string(&deep).expect("deeply mutated JSON");
        assert!(
            !replay_candidate_join_v5_json(
                EMBEDDED_SCHEMA3_ARCHIVE,
                EMBEDDED_SCHEMA4_ARCHIVE,
                &deep_json,
            )
            .valid
        );

        let canonical_json = serde_json::to_string(&certificate).expect("canonical JSON");
        let duplicate_top = canonical_json.replacen('{', "{\"schema_version\":999,", 1);
        assert!(
            !replay_candidate_join_v5_json(
                EMBEDDED_SCHEMA3_ARCHIVE,
                EMBEDDED_SCHEMA4_ARCHIVE,
                &duplicate_top,
            )
            .valid
        );
        let duplicate_deep = canonical_json.replacen(
            "\"prior_c7_base_bindings\":{\"status\":",
            "\"prior_c7_base_bindings\":{\"status\":\"gap\",\"status\":",
            1,
        );
        assert_ne!(
            duplicate_deep, canonical_json,
            "deep mutation must be inserted"
        );
        assert!(
            !replay_candidate_join_v5_json(
                EMBEDDED_SCHEMA3_ARCHIVE,
                EMBEDDED_SCHEMA4_ARCHIVE,
                &duplicate_deep,
            )
            .valid
        );
    }
}
