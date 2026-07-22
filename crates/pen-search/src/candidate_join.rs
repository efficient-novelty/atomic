//! Schema-4 kernel-bridge sidecar for the archived IP-1 candidate quotient.
//!
//! This module deliberately does not import the schema-3 generator or its
//! comparison machinery.  It binds the exact schema-3 JSON bytes, projects
//! only the structural row fields needed for a row join, and records the
//! current C-1..C-8 proof/gap boundary. In particular, an open bridge premise
//! is never promoted beyond its recorded evidence status.

use crate::enumerate::{EnumerationContext, LateFamilySurface, assess_raw_surface_membership};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::certified_novelty::ReplayedP5LiftCapability;
use pen_eval::naturality_basis::{
    RAW_RENAMING_BASIS_VERSION, SEMANTIC_COMPLETENESS_GAP, issue_raw_renaming_basis,
    replay_raw_renaming_basis,
};
use pen_eval::typed_families::{CanonicalPresentation, ParamSort, RenamingMap};
use pen_type::cubical::typed_boundary::{
    ADOPTED_BASIS_CHARGE_BINDING_VERSION, ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION,
    BOUNDARY_CHARGE_POLICY_VERSION, HISTORICAL_POINT_TYPING_OBSTRUCTION_ID,
    HistoricalBaseBindingAudit, RegisteredBoundaryKind, TYPED_BOUNDARY_FRAGMENT_VERSION,
    TypedBoundaryError, audit_historical_base_binding, issue_adopted_boundary_basis_charge_token,
    issue_reference_only_charge_token, issue_typed_declared_boundary_token,
    registered_boundary_diagram, replay_adopted_boundary_basis_charge_token,
    replay_reference_only_charge_token, replay_typed_declared_boundary_token,
};
use pen_type::elaborate::{SealedSignature, issue_typed_lift_token};
use pen_type::fuel_composition::{
    FUEL_COMPOSITION_VERSION, issue_telescope_fuel_composition, replay_telescope_fuel_composition,
};
use pen_type::substitution::{
    EXPR_CONSTRUCTOR_COVERAGE, ParameterSort, SORT_PRESERVATION_SCOPE,
    SUBSTITUTION_FRAGMENT_VERSION, SortedParameterContext, identity_sort_preserving_substitution,
    issue_variable_image_support_preservation, replay_sort_preserving_substitution,
    replay_variable_image_support_preservation,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::fmt::Write as _;
use thiserror::Error;

pub const CANDIDATE_JOIN_V4_SCHEMA_VERSION: u32 = 4;
pub const CANDIDATE_JOIN_V4_DATE: &str = "2026-07-19";

pub const SCHEMA3_ARCHIVE_LENGTH: usize = 168_692;
pub const SCHEMA3_ARCHIVE_SHA256: &str =
    "4913E818C080D67C81CE1770F047E22DB9EA6975BFE704FB01352F268FA2B729";
pub const SCHEMA3_ARCHIVE_INTERNAL_DIGEST: &str =
    "blake3:e4cb89fe83476fb8a0eee43bcc976053bcbbff0bd6db35607d7e8154a3ef858b";
pub const SCHEMA3_ARCHIVE_GIT_BLOB: &str = "00789907410999f2c80e72b18465178c226153be";
pub const SCHEMA3_SIGNATURE_DIGEST: &str =
    "blake3:d51ffb32c2e6e18f45ee15a8c1af3edbf40759fda1696032ae6e97439c48f10a";
pub const SCHEMA3_CLOSURE_DIGEST: &str =
    "blake3:bad84e9cb1e6f0075af529ef0eb32d82b91292b2341e3b66d09a18d4dada9e70";
pub const SCHEMA3_ORBIT_DERIVATION_HASH: &str =
    "blake3:3c057f16a4c48a1b3d93554f5746768b98f4d7e0153b34f8b433f0dc3af3dc2a";

pub const C1_ARBITRARY_INSTANCE_GAP: &str = "C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION";
pub const C2_COMPLETENESS_GAP: &str = "C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS";
pub const C3_DOMAIN_WIDE_GAP: &str = "C3_DOMAIN_WIDE_WHOLE_TELESCOPE_FUEL_COMPOSITION";
pub const C5_CONTEXTUAL_CLOSURE_GAP: &str = "O-C5-ContextualSubstitutionClosure";
pub const C6_GENERAL_BOUNDARY_GAP: &str = "C6_V2_GENERAL_HISTORICAL_TERM_LEVEL_COMPLETION";
pub const C8_REFERENCE_ONLY_PROOF: &str =
    "C8_REFERENCE_ONLY_ZERO_CHARGE_AND_ONE_PLUS_D_SQUARED_ALL_REGISTERED_DIAGRAMS";
pub const C8_CANDIDATE_JOIN_GAP: &str = "C8_CANDIDATE_BOUNDARY_PROVENANCE_JOIN";

const EMBEDDED_SCHEMA3_ARCHIVE: &[u8] =
    include_bytes!("../../../docs/ip1_candidate_verdict_join_v3.json");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CandidateJoinV4Error {
    #[error("schema-3 archive has length {found}, expected {expected}")]
    ArchiveLength { expected: usize, found: usize },
    #[error("schema-3 archive SHA-256 mismatch: expected {expected}, got {found}")]
    ArchiveSha256 { expected: String, found: String },
    #[error("runtime schema-3 bytes differ from the compile-time archive")]
    ArchiveBytesDiffer,
    #[error("invalid schema-3 JSON: {0}")]
    InvalidSchema3Json(String),
    #[error("invalid schema-4 JSON: {0}")]
    InvalidSchema4Json(String),
    #[error("schema-3 projection mismatch: {0}")]
    Projection(String),
    #[error("kernel-bridge evidence construction failed for {obligation}: {detail}")]
    Evidence { obligation: String, detail: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Schema3Projection {
    schema_version: u32,
    signature_digest: String,
    closure_digest: String,
    orbit_derivation_hash: String,
    strata: Vec<Schema3StratumProjection>,
    digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Schema3StratumProjection {
    kappa: u16,
    rows: Vec<Schema3RowProjection>,
}

/// The only schema-3 row data allowed into the schema-4 derivation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Schema3RowProjection {
    kappa: u16,
    class: String,
    clause_local_extraction_proxy: String,
    direct_support: u8,
    amplification_route: String,
    count: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Schema3ArchiveBinding {
    pub byte_length: usize,
    pub sha256: String,
    pub git_blob: String,
    pub internal_digest: String,
    pub signature_digest: String,
    pub closure_digest: String,
    pub orbit_derivation_hash: String,
    pub stratum_row_counts: Vec<usize>,
    pub total_rows: usize,
    pub allowed_projection_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EvidenceStatus {
    Proved { proof_id: String },
    Gap { gap_id: String },
}

impl EvidenceStatus {
    fn is_proved(&self) -> bool {
        matches!(self, Self::Proved { .. })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScopedEvidence {
    pub evidence_id: String,
    pub implementation_version: String,
    pub scope: String,
    pub status: EvidenceStatus,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ObligationEvidence {
    pub obligation_id: String,
    pub status: EvidenceStatus,
    pub scoped_evidence: Vec<ScopedEvidence>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KernelBridgeTheoremBundle {
    pub c1_substitution: ObligationEvidence,
    pub c2_naturality: ObligationEvidence,
    pub c3_fuel: ObligationEvidence,
    pub c5_window: ObligationEvidence,
    pub c6_boundary_terms: ObligationEvidence,
    pub c7_base_bindings: ObligationEvidence,
    pub c8_charging: ObligationEvidence,
    pub p5_window_reaudit: P5WindowReaudit,
    pub derivation_hash: String,
}

/// Literal C-5 witness replay plus the exact boundary of the new
/// substitution result.  `None` is intentional: without a complete inventory
/// of arbitrary typed contextual substitutions, the checker neither asserts
/// nor denies that such a closure opens an in-surface route.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct P5WindowReaudit {
    pub candidate: Telescope,
    pub raw_surface_member: bool,
    pub raw_surface_rejections: Vec<String>,
    pub public_lift_token_issued: bool,
    pub token_dominant_import: Option<u32>,
    pub token_lift_clauses: Vec<u16>,
    pub token_derivation_hash: Option<String>,
    pub replay_adapter_succeeded: bool,
    pub record_internality_capability_available: bool,
    pub full_p5_route_constructible: bool,
    pub restricted_variable_images_preserve_library_support: bool,
    pub restricted_support_derivation_hash: String,
    pub arbitrary_contextual_substitution_inventory_complete: bool,
    pub in_surface_route_opened_under_complete_typed_substitution: Option<bool>,
    pub gap_id: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RowObligationStatus {
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

impl RowObligationStatus {
    fn is_proved_or_not_applicable(&self) -> bool {
        matches!(self, Self::Proved { .. } | Self::NotApplicable { .. })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateBridgeRow {
    pub stratum_index: u16,
    pub row_index: u16,
    pub kappa: u16,
    pub class: String,
    pub clause_local_extraction_proxy: String,
    pub direct_support: u8,
    pub amplification_route: String,
    pub count: String,
    pub schema3_row_digest: String,
    pub c1_substitution: RowObligationStatus,
    pub c2_naturality: RowObligationStatus,
    pub c3_fuel: RowObligationStatus,
    pub c5_window: RowObligationStatus,
    pub c6_boundary_terms: RowObligationStatus,
    pub c7_base_bindings: RowObligationStatus,
    pub c8_charging: RowObligationStatus,
    pub all_required_evidence_proved: bool,
    pub implemented_egp_bridge_returns_bound: bool,
    pub intended_typed_egp_proved: bool,
    pub full_candidate_extraction_join_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BridgeStratum {
    pub stratum_index: u16,
    pub kappa: u16,
    pub rows: Vec<CandidateBridgeRow>,
    pub all_archived_rows_joined: bool,
    pub no_row_promoted_across_open_gap: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateJoinV4Completeness {
    pub exact_schema3_archive_bound: bool,
    pub allowed_projection_only: bool,
    pub all_213_rows_joined: bool,
    pub every_applicable_c2_row_has_named_gap: bool,
    pub no_row_promoted_across_open_gap: bool,
    pub c1_arbitrary_instance_sort_preservation_proved: bool,
    pub c2_depth_two_schema_completeness_proved: bool,
    pub c3_domain_wide_fuel_compositionality_proved: bool,
    pub c5_contextual_substitution_closure_proved: bool,
    pub c6_v2_general_historical_term_completion_proved: bool,
    pub c7_all_historical_base_bindings_proved: bool,
    pub c8_registered_reference_only_charging_proved: bool,
    pub c8_candidate_boundary_provenance_join_proved: bool,
    pub full_candidate_extraction_join_proved: bool,
    pub intended_semantic_candidate_join_proved: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateJoinV4Certificate {
    pub schema_version: u32,
    pub date: String,
    pub schema3_archive: Schema3ArchiveBinding,
    pub theorem_bundle: KernelBridgeTheoremBundle,
    pub strata: Vec<BridgeStratum>,
    pub completeness: CandidateJoinV4Completeness,
    pub digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateJoinV4Replay {
    pub valid: bool,
    pub exact_schema3_archive_bound: bool,
    pub all_213_rows_joined: bool,
    pub no_row_promoted_across_open_gap: bool,
    pub full_candidate_extraction_join_proved: bool,
    pub intended_semantic_candidate_join_proved: bool,
    pub errors: Vec<String>,
}

fn tagged_digest(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(CANDIDATE_JOIN_V4_SCHEMA_VERSION, domain, payload))
        .expect("schema-4 proof payload serializes");
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

fn evidence_status_proved(proof_id: impl Into<String>) -> EvidenceStatus {
    EvidenceStatus::Proved {
        proof_id: proof_id.into(),
    }
}

fn evidence_status_gap(gap_id: impl Into<String>) -> EvidenceStatus {
    EvidenceStatus::Gap {
        gap_id: gap_id.into(),
    }
}

fn scoped_evidence(
    evidence_id: impl Into<String>,
    implementation_version: impl Into<String>,
    scope: impl Into<String>,
    status: EvidenceStatus,
    kernel_derivation_hash: impl Into<String>,
) -> ScopedEvidence {
    let evidence_id = evidence_id.into();
    let implementation_version = implementation_version.into();
    let scope = scope.into();
    let kernel_derivation_hash = kernel_derivation_hash.into();
    let derivation_hash = tagged_digest(
        "scoped-evidence",
        &(
            &evidence_id,
            &implementation_version,
            &scope,
            &status,
            &kernel_derivation_hash,
        ),
    );
    ScopedEvidence {
        evidence_id,
        implementation_version,
        scope,
        status,
        derivation_hash,
    }
}

fn obligation_evidence(
    obligation_id: &str,
    status: EvidenceStatus,
    scoped_evidence: Vec<ScopedEvidence>,
) -> ObligationEvidence {
    let derivation_hash = tagged_digest(
        "obligation-evidence",
        &(obligation_id, &status, &scoped_evidence),
    );
    ObligationEvidence {
        obligation_id: obligation_id.to_owned(),
        status,
        scoped_evidence,
        derivation_hash,
    }
}

fn evidence_error(obligation: &str, detail: impl ToString) -> CandidateJoinV4Error {
    CandidateJoinV4Error::Evidence {
        obligation: obligation.to_owned(),
        detail: detail.to_string(),
    }
}

fn derive_c1() -> Result<ObligationEvidence, CandidateJoinV4Error> {
    let context = SortedParameterContext::new(vec![ParameterSort::Type, ParameterSort::Opaque]);
    let body = Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)));
    let token = identity_sort_preserving_substitution(context, body)
        .map_err(|error| evidence_error("C1", error))?;
    replay_sort_preserving_substitution(&token).map_err(|error| evidence_error("C1", error))?;
    let restricted = scoped_evidence(
        "C1_RESTRICTED_SORT_IDENTICAL_VARIABLE_IMAGES",
        SUBSTITUTION_FRAGMENT_VERSION,
        format!(
            "{}; structural traversal explicitly covers {:?}",
            SORT_PRESERVATION_SCOPE, EXPR_CONSTRUCTOR_COVERAGE
        ),
        evidence_status_proved("C1_RESTRICTED_SORT_IDENTICAL_VARIABLE_IMAGES"),
        token.derivation_hash(),
    );
    let arbitrary_gap = scoped_evidence(
        C1_ARBITRARY_INSTANCE_GAP,
        SUBSTITUTION_FRAGMENT_VERSION,
        "non-variable images require a typed instance judgement in an arbitrary canonical-family context",
        evidence_status_gap(C1_ARBITRARY_INSTANCE_GAP),
        tagged_digest("c1-arbitrary-instance-gap", &SORT_PRESERVATION_SCOPE),
    );
    Ok(obligation_evidence(
        "C1_TYPED_SUBSTITUTION_AND_INSTANCE",
        evidence_status_gap(C1_ARBITRARY_INSTANCE_GAP),
        vec![restricted, arbitrary_gap],
    ))
}

fn derive_c2() -> Result<ObligationEvidence, CandidateJoinV4Error> {
    if SEMANTIC_COMPLETENESS_GAP != C2_COMPLETENESS_GAP {
        return Err(evidence_error(
            "C2",
            "naturality module and schema-4 gap identifiers differ",
        ));
    }
    let presentation = CanonicalPresentation {
        canonical_normal_form: Expr::Pi(
            Box::new(Expr::Var(1)),
            Box::new(Expr::App(Box::new(Expr::Var(2)), Box::new(Expr::Var(3)))),
        ),
        parameters: vec![ParamSort::Type, ParamSort::Opaque, ParamSort::Type],
        renaming: RenamingMap {
            free_scope_len: 3,
            forward: vec![(1, 1), (2, 2), (3, 3)],
        },
    };
    let token =
        issue_raw_renaming_basis(presentation).map_err(|error| evidence_error("C2", error))?;
    replay_raw_renaming_basis(&token).map_err(|error| evidence_error("C2", error))?;
    if !token.complete_for_raw_sort_preserving_renamings()
        || token.semantic_completeness_gap() != C2_COMPLETENESS_GAP
    {
        return Err(evidence_error(
            "C2",
            "raw renaming evidence attempted to promote semantic completeness",
        ));
    }
    let raw = scoped_evidence(
        "C2_RAW_SORT_PRESERVING_RENAMING_AND_UNUSED_WEAKENING",
        RAW_RENAMING_BASIS_VERSION,
        "finite sort-preserving parameter renamings and one unused-parameter weakening for the supplied canonical presentation only",
        evidence_status_proved("C2_RAW_SORT_PRESERVING_RENAMING_AND_UNUSED_WEAKENING"),
        token.derivation_hash(),
    );
    let gap = scoped_evidence(
        C2_COMPLETENESS_GAP,
        RAW_RENAMING_BASIS_VERSION,
        "intended depth-two schema grammar and a complete naturality-generator theorem are not implemented",
        evidence_status_gap(C2_COMPLETENESS_GAP),
        tagged_digest("c2-depth-two-gap", &C2_COMPLETENESS_GAP),
    );
    Ok(obligation_evidence(
        "C2_COMPLETE_NATURALITY_BASIS",
        evidence_status_gap(C2_COMPLETENESS_GAP),
        vec![raw, gap],
    ))
}

fn derive_c3() -> Result<ObligationEvidence, CandidateJoinV4Error> {
    let signature = SealedSignature::genesis_del_h15();
    let mut sealed_history_witnesses = Vec::new();
    for step in 1..=15 {
        let telescope = Telescope::reference(step);
        let token =
            issue_telescope_fuel_composition(&signature, &telescope, step.saturating_sub(1))
                .map_err(|error| evidence_error("C3", error))?;
        replay_telescope_fuel_composition(&signature, &telescope, step.saturating_sub(1), &token)
            .map_err(|error| evidence_error("C3", error))?;
        sealed_history_witnesses.push((step, token.derivation_hash().to_owned()));
    }
    let sealed = scoped_evidence(
        "C3_ALL_FIFTEEN_SEALED_TELESCOPES_COMPOSE",
        FUEL_COMPOSITION_VERSION,
        "each sealed telescope has replayable clause-local allocations summing exactly to its whole static fuel bound",
        evidence_status_proved("C3_ALL_FIFTEEN_SEALED_TELESCOPES_COMPOSE"),
        tagged_digest(
            "c3-sealed-history-fuel-witnesses",
            &sealed_history_witnesses,
        ),
    );
    let gap = scoped_evidence(
        C3_DOMAIN_WIDE_GAP,
        FUEL_COMPOSITION_VERSION,
        "the per-telescope token API and sealed-reference regressions do not establish a domain-wide theorem over every schema-3 quotient member",
        evidence_status_gap(C3_DOMAIN_WIDE_GAP),
        tagged_digest("c3-domain-wide-gap", &FUEL_COMPOSITION_VERSION),
    );
    Ok(obligation_evidence(
        "C3_WHOLE_TELESCOPE_FUEL_COMPOSITIONALITY",
        evidence_status_gap(C3_DOMAIN_WIDE_GAP),
        vec![sealed, gap],
    ))
}

fn c5_position_zero_context() -> EnumerationContext {
    EnumerationContext {
        library_size: 15,
        scope_size: 2,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes: 6,
        require_former_eliminator_clauses: false,
        require_initial_hit_clauses: false,
        require_truncation_hit_clauses: false,
        require_higher_hit_clauses: false,
        require_sphere_lift_clauses: false,
        require_axiomatic_bundle_clauses: false,
        require_modal_shell_clauses: false,
        require_connection_shell_clauses: false,
        require_curvature_shell_clauses: false,
        require_operator_bundle_clauses: false,
        require_hilbert_functional_clauses: false,
        require_temporal_shell_clauses: false,
        historical_anchor_ref: None,
        late_family_surface: LateFamilySurface::None,
    }
}

fn derive_c5() -> Result<(ObligationEvidence, P5WindowReaudit), CandidateJoinV4Error> {
    let candidate = Telescope::new(vec![
        ClauseRec::new(
            ClauseRole::Introduction,
            Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Lib(13))),
        ),
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
        ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
    ]);
    let raw = assess_raw_surface_membership(c5_position_zero_context(), &candidate);
    let signature = SealedSignature::genesis_del_h15();
    let lift = issue_typed_lift_token(&signature, &candidate, 15)
        .map_err(|error| evidence_error("C5", error))?;
    let replay_adapter_succeeded =
        ReplayedP5LiftCapability::from_kernel_token(&signature, &candidate, 15, &lift).is_ok();
    if !replay_adapter_succeeded {
        return Err(evidence_error(
            "C5",
            "the known public P5 lift witness no longer passes its replay adapter",
        ));
    }

    // This is the strongest support theorem justified by C1 today.  It shows
    // that a variable-only substitution cannot smuggle Lib(13) into a raw
    // `App(Lib(14), Var(_))`; it does not enumerate arbitrary typed images.
    let support_substitution = identity_sort_preserving_substitution(
        SortedParameterContext::all_type(1),
        Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Var(1))),
    )
    .map_err(|error| evidence_error("C5", error))?;
    let support = issue_variable_image_support_preservation(&support_substitution);
    replay_variable_image_support_preservation(&support_substitution, &support)
        .map_err(|error| evidence_error("C5", error))?;
    if !support.support_preserved_exactly() {
        return Err(evidence_error(
            "C5",
            "restricted variable-image support theorem did not preserve support",
        ));
    }

    let literal = scoped_evidence(
        "C5_LITERAL_PUBLIC_P5_WITNESS_REPLAYED",
        SUBSTITUTION_FRAGMENT_VERSION,
        "App(Lib(14),Lib(13)) still issues and replays a public lift token, but remains outside the raw leaf surface because Lib(13) is unavailable there",
        evidence_status_proved("C5_LITERAL_PUBLIC_P5_WITNESS_REPLAYED"),
        lift.derivation_hash(),
    );
    let support_evidence = scoped_evidence(
        "C5_VARIABLE_IMAGE_SUPPORT_PRESERVATION",
        SUBSTITUTION_FRAGMENT_VERSION,
        "restricted sort-preserving variable images introduce no hidden library leaf",
        evidence_status_proved("C5_VARIABLE_IMAGE_SUPPORT_PRESERVATION"),
        support.derivation_hash(),
    );
    let gap = scoped_evidence(
        C5_CONTEXTUAL_CLOSURE_GAP,
        SUBSTITUTION_FRAGMENT_VERSION,
        "the public-lift window must be re-audited after contextual substitution closure; no in-surface closure theorem is available",
        evidence_status_gap(C5_CONTEXTUAL_CLOSURE_GAP),
        tagged_digest("c5-contextual-substitution-gap", &C5_CONTEXTUAL_CLOSURE_GAP),
    );
    let obligation = obligation_evidence(
        "C5_P5_WINDOW_REAUDIT",
        evidence_status_gap(C5_CONTEXTUAL_CLOSURE_GAP),
        vec![literal, support_evidence, gap],
    );
    let public_lift_token_issued = true;
    let token_dominant_import = Some(lift.dominant_import());
    let token_lift_clauses = lift.lift_clauses().to_vec();
    let token_derivation_hash = Some(lift.derivation_hash().to_owned());
    let record_internality_capability_available = false;
    let full_p5_route_constructible = false;
    let restricted_variable_images_preserve_library_support = true;
    let restricted_support_derivation_hash = support.derivation_hash().to_owned();
    let arbitrary_contextual_substitution_inventory_complete = false;
    let in_surface_route_opened_under_complete_typed_substitution = None;
    let gap_id = C5_CONTEXTUAL_CLOSURE_GAP.to_owned();
    let derivation_hash = tagged_digest(
        "p5-window-reaudit",
        &(
            &candidate,
            raw.is_member,
            &raw.rejection_reasons,
            public_lift_token_issued,
            token_dominant_import,
            &token_lift_clauses,
            &token_derivation_hash,
            replay_adapter_succeeded,
            record_internality_capability_available,
            full_p5_route_constructible,
            restricted_variable_images_preserve_library_support,
            &restricted_support_derivation_hash,
            arbitrary_contextual_substitution_inventory_complete,
            in_surface_route_opened_under_complete_typed_substitution,
            &gap_id,
        ),
    );
    let reaudit = P5WindowReaudit {
        candidate,
        raw_surface_member: raw.is_member,
        raw_surface_rejections: raw.rejection_reasons,
        public_lift_token_issued,
        token_dominant_import,
        token_lift_clauses,
        token_derivation_hash,
        replay_adapter_succeeded,
        record_internality_capability_available,
        full_p5_route_constructible,
        restricted_variable_images_preserve_library_support,
        restricted_support_derivation_hash,
        arbitrary_contextual_substitution_inventory_complete,
        in_surface_route_opened_under_complete_typed_substitution,
        gap_id,
        derivation_hash,
    };
    Ok((obligation, reaudit))
}

fn boundary_kind_name(kind: RegisteredBoundaryKind) -> &'static str {
    match kind {
        RegisteredBoundaryKind::S1 => "s1",
        RegisteredBoundaryKind::Trunc => "trunc",
        RegisteredBoundaryKind::S2 => "s2",
        RegisteredBoundaryKind::S3 => "s3",
    }
}

fn derive_boundary_obligations()
-> Result<(ObligationEvidence, ObligationEvidence, ObligationEvidence), CandidateJoinV4Error> {
    let signature = SealedSignature::genesis_del_h15();
    let mut c6_scoped = Vec::new();
    let mut c7_scoped = Vec::new();
    let mut c8_scoped = Vec::new();

    for kind in RegisteredBoundaryKind::ALL {
        let name = boundary_kind_name(kind);
        let map = registered_boundary_diagram(&signature, kind)
            .map_err(|error| evidence_error("C6/C8", error))?;
        let charge = issue_reference_only_charge_token(&signature, kind, map.clone())
            .map_err(|error| evidence_error("C8", error))?;
        replay_reference_only_charge_token(&signature, &charge)
            .map_err(|error| evidence_error("C8", error))?;
        c8_scoped.push(scoped_evidence(
            format!("C8_REFERENCE_ONLY_{name}"),
            BOUNDARY_CHARGE_POLICY_VERSION,
            format!("registered {name} boundary; references resolve only to sealed clauses or declared context variables"),
            evidence_status_proved(format!("C8_REFERENCE_ONLY_{name}")),
            charge.derivation_hash(),
        ));
        let basis_charge = issue_adopted_boundary_basis_charge_token(&signature, kind, map.clone())
            .map_err(|error| evidence_error("C8", error))?;
        replay_adopted_boundary_basis_charge_token(&signature, &basis_charge)
            .map_err(|error| evidence_error("C8", error))?;
        c8_scoped.push(scoped_evidence(
            format!("C8_ADOPTED_BASIS_CHARGE_{name}"),
            ADOPTED_BASIS_CHARGE_BINDING_VERSION,
            format!(
                "registered {name} formula is 1 + d^2 with c(b) = 0 under {BOUNDARY_CHARGE_POLICY_VERSION}"
            ),
            evidence_status_proved(format!("C8_ADOPTED_BASIS_CHARGE_{name}")),
            basis_charge.derivation_hash(),
        ));

        match issue_typed_declared_boundary_token(&signature, kind, map) {
            Ok(token) if kind == RegisteredBoundaryKind::Trunc => {
                replay_typed_declared_boundary_token(&signature, &token)
                    .map_err(|error| evidence_error("C6", error))?;
                c6_scoped.push(scoped_evidence(
                    "C6_TRUNC_REGISTERED_BOUNDARY_TYPED",
                    TYPED_BOUNDARY_FRAGMENT_VERSION,
                    "the registered endpoint-dependent Trunc diagram is term-typed in its declared parameter context",
                    evidence_status_proved("C6_TRUNC_REGISTERED_BOUNDARY_TYPED"),
                    token.derivation_hash(),
                ));
            }
            Ok(_) => {
                return Err(evidence_error(
                    "C7",
                    format!("{name} unexpectedly received a historical base-binding token"),
                ));
            }
            Err(TypedBoundaryError::HistoricalPointTypingObstructed { obstruction })
                if kind.needs_historical_base_binding() =>
            {
                if obstruction.obstruction_id() != HISTORICAL_POINT_TYPING_OBSTRUCTION_ID {
                    return Err(evidence_error("C7", "unexpected obstruction identifier"));
                }
                let gap_id = format!("{HISTORICAL_POINT_TYPING_OBSTRUCTION_ID}:{name}");
                c6_scoped.push(scoped_evidence(
                    format!("C6_{name}_TERM_TYPING_BLOCKED_BY_C7"),
                    TYPED_BOUNDARY_FRAGMENT_VERSION,
                    format!("registered {name} diagram cannot be term-typed until its historical base is bound"),
                    evidence_status_gap(gap_id.clone()),
                    obstruction.derivation_hash(),
                ));
                c7_scoped.push(scoped_evidence(
                    gap_id.clone(),
                    ADOPTED_DECLARED_BOUNDARY_AXIOM_VERSION,
                    format!("historical {name} point clause is not derived as an element of its formed owner"),
                    evidence_status_gap(gap_id),
                    obstruction.derivation_hash(),
                ));
            }
            Err(error) => return Err(evidence_error("C6/C7", error)),
        }
    }

    for kind in [
        RegisteredBoundaryKind::S1,
        RegisteredBoundaryKind::S2,
        RegisteredBoundaryKind::S3,
    ] {
        match audit_historical_base_binding(&signature, kind)
            .map_err(|error| evidence_error("C7", error))?
        {
            HistoricalBaseBindingAudit::Obstructed(obstruction)
                if obstruction.obstruction_id() == HISTORICAL_POINT_TYPING_OBSTRUCTION_ID => {}
            HistoricalBaseBindingAudit::Obstructed(_) => {
                return Err(evidence_error("C7", "unexpected audit obstruction"));
            }
            HistoricalBaseBindingAudit::Bound(_) => {
                return Err(evidence_error("C7", "historical base unexpectedly bound"));
            }
        }
    }

    let c6 = obligation_evidence(
        "C6_V2_TERM_LEVEL_COMPLETION",
        evidence_status_gap(C6_GENERAL_BOUNDARY_GAP),
        c6_scoped,
    );
    let c7 = obligation_evidence(
        "C7_HISTORICAL_BASE_BINDINGS",
        evidence_status_gap(HISTORICAL_POINT_TYPING_OBSTRUCTION_ID),
        c7_scoped,
    );
    let registered_reference_hashes = c8_scoped
        .iter()
        .map(|evidence| evidence.derivation_hash.clone())
        .collect::<Vec<_>>();
    c8_scoped.push(scoped_evidence(
        C8_REFERENCE_ONLY_PROOF,
        ADOPTED_BASIS_CHARGE_BINDING_VERSION,
        "all four registered diagrams resolve every boundary leaf to a sealed clause or declared context variable and bind the formula 1 + d^2 with c(b) = 0 to the exact adopted charging policy",
        evidence_status_proved(C8_REFERENCE_ONLY_PROOF),
        tagged_digest("c8-all-registered-reference-only", &registered_reference_hashes),
    ));
    let c8 = obligation_evidence(
        "C8_REFERENCE_ONLY_CHARGING",
        evidence_status_gap(C8_CANDIDATE_JOIN_GAP),
        {
            c8_scoped.push(scoped_evidence(
                C8_CANDIDATE_JOIN_GAP,
                BOUNDARY_CHARGE_POLICY_VERSION,
                "registered diagrams are resolved, but schema-3 quotient rows carry no candidate boundary-provenance inventory to join",
                evidence_status_gap(C8_CANDIDATE_JOIN_GAP),
                tagged_digest("c8-candidate-join-gap", &C8_CANDIDATE_JOIN_GAP),
            ));
            c8_scoped
        },
    );
    Ok((c6, c7, c8))
}

fn derive_theorem_bundle() -> Result<KernelBridgeTheoremBundle, CandidateJoinV4Error> {
    let c1_substitution = derive_c1()?;
    let c2_naturality = derive_c2()?;
    let c3_fuel = derive_c3()?;
    let (c5_window, p5_window_reaudit) = derive_c5()?;
    let (c6_boundary_terms, c7_base_bindings, c8_charging) = derive_boundary_obligations()?;
    let derivation_hash = tagged_digest(
        "kernel-bridge-theorem-bundle",
        &(
            &c1_substitution,
            &c2_naturality,
            &c3_fuel,
            &c5_window,
            &c6_boundary_terms,
            &c7_base_bindings,
            &c8_charging,
            &p5_window_reaudit,
        ),
    );
    Ok(KernelBridgeTheoremBundle {
        c1_substitution,
        c2_naturality,
        c3_fuel,
        c5_window,
        c6_boundary_terms,
        c7_base_bindings,
        c8_charging,
        p5_window_reaudit,
        derivation_hash,
    })
}

fn validate_schema3_archive(
    schema3_json: &[u8],
) -> Result<(Schema3Projection, Schema3ArchiveBinding), CandidateJoinV4Error> {
    if EMBEDDED_SCHEMA3_ARCHIVE.len() != SCHEMA3_ARCHIVE_LENGTH {
        return Err(CandidateJoinV4Error::ArchiveLength {
            expected: SCHEMA3_ARCHIVE_LENGTH,
            found: EMBEDDED_SCHEMA3_ARCHIVE.len(),
        });
    }
    let embedded_sha = sha256_hex(EMBEDDED_SCHEMA3_ARCHIVE);
    if embedded_sha != SCHEMA3_ARCHIVE_SHA256 {
        return Err(CandidateJoinV4Error::ArchiveSha256 {
            expected: SCHEMA3_ARCHIVE_SHA256.to_owned(),
            found: embedded_sha,
        });
    }
    if schema3_json.len() != SCHEMA3_ARCHIVE_LENGTH {
        return Err(CandidateJoinV4Error::ArchiveLength {
            expected: SCHEMA3_ARCHIVE_LENGTH,
            found: schema3_json.len(),
        });
    }
    let runtime_sha = sha256_hex(schema3_json);
    if runtime_sha != SCHEMA3_ARCHIVE_SHA256 {
        return Err(CandidateJoinV4Error::ArchiveSha256 {
            expected: SCHEMA3_ARCHIVE_SHA256.to_owned(),
            found: runtime_sha,
        });
    }
    if schema3_json != EMBEDDED_SCHEMA3_ARCHIVE {
        return Err(CandidateJoinV4Error::ArchiveBytesDiffer);
    }
    let projection: Schema3Projection = serde_json::from_slice(schema3_json)
        .map_err(|error| CandidateJoinV4Error::InvalidSchema3Json(error.to_string()))?;
    if projection.schema_version != 3 {
        return Err(CandidateJoinV4Error::Projection(format!(
            "expected schema 3, got {}",
            projection.schema_version
        )));
    }
    for (field, found, expected) in [
        (
            "internal digest",
            projection.digest.as_str(),
            SCHEMA3_ARCHIVE_INTERNAL_DIGEST,
        ),
        (
            "signature digest",
            projection.signature_digest.as_str(),
            SCHEMA3_SIGNATURE_DIGEST,
        ),
        (
            "closure digest",
            projection.closure_digest.as_str(),
            SCHEMA3_CLOSURE_DIGEST,
        ),
        (
            "orbit derivation hash",
            projection.orbit_derivation_hash.as_str(),
            SCHEMA3_ORBIT_DERIVATION_HASH,
        ),
    ] {
        if found != expected {
            return Err(CandidateJoinV4Error::Projection(format!(
                "{field} mismatch: expected {expected}, got {found}"
            )));
        }
    }
    let expected_counts = [63usize, 72, 78];
    if projection.strata.len() != expected_counts.len() {
        return Err(CandidateJoinV4Error::Projection(format!(
            "expected three strata, got {}",
            projection.strata.len()
        )));
    }
    for (index, (stratum, expected_count)) in
        projection.strata.iter().zip(expected_counts).enumerate()
    {
        let expected_kappa = u16::try_from(index + 2).expect("small kappa");
        if stratum.kappa != expected_kappa || stratum.rows.len() != expected_count {
            return Err(CandidateJoinV4Error::Projection(format!(
                "stratum {index} expected kappa {expected_kappa} and {expected_count} rows, got kappa {} and {} rows",
                stratum.kappa,
                stratum.rows.len()
            )));
        }
        if stratum.rows.iter().any(|row| row.kappa != stratum.kappa) {
            return Err(CandidateJoinV4Error::Projection(format!(
                "stratum {index} contains a row with a mismatched kappa"
            )));
        }
    }
    let total_rows = projection
        .strata
        .iter()
        .map(|stratum| stratum.rows.len())
        .sum::<usize>();
    if total_rows != 213 {
        return Err(CandidateJoinV4Error::Projection(format!(
            "expected 213 rows, got {total_rows}"
        )));
    }
    let allowed_projection_digest = tagged_digest("schema3-allowed-projection", &projection);
    let binding = Schema3ArchiveBinding {
        byte_length: schema3_json.len(),
        sha256: runtime_sha,
        git_blob: SCHEMA3_ARCHIVE_GIT_BLOB.to_owned(),
        internal_digest: projection.digest.clone(),
        signature_digest: projection.signature_digest.clone(),
        closure_digest: projection.closure_digest.clone(),
        orbit_derivation_hash: projection.orbit_derivation_hash.clone(),
        stratum_row_counts: projection
            .strata
            .iter()
            .map(|stratum| stratum.rows.len())
            .collect(),
        total_rows,
        allowed_projection_digest,
    };
    Ok((projection, binding))
}

fn row_obligation_status(
    obligation: &ObligationEvidence,
    schema3_row_digest: &str,
) -> RowObligationStatus {
    let evidence_hash = tagged_digest(
        "row-obligation",
        &(
            schema3_row_digest,
            &obligation.obligation_id,
            &obligation.status,
            &obligation.derivation_hash,
        ),
    );
    match &obligation.status {
        EvidenceStatus::Proved { proof_id } => RowObligationStatus::Proved {
            proof_id: proof_id.clone(),
            evidence_hash,
        },
        EvidenceStatus::Gap { gap_id } => RowObligationStatus::Gap {
            gap_id: gap_id.clone(),
            evidence_hash,
        },
    }
}

fn not_applicable(
    obligation_id: &str,
    reason_id: &str,
    schema3_row_digest: &str,
) -> RowObligationStatus {
    RowObligationStatus::NotApplicable {
        reason_id: reason_id.to_owned(),
        evidence_hash: tagged_digest(
            "row-obligation-not-applicable",
            &(schema3_row_digest, obligation_id, reason_id),
        ),
    }
}

fn build_row(
    stratum_index: usize,
    row_index: usize,
    row: &Schema3RowProjection,
    bundle: &KernelBridgeTheoremBundle,
) -> CandidateBridgeRow {
    let schema3_row_digest =
        tagged_digest("schema3-projected-row", &(stratum_index, row_index, row));
    let kernel_invalid = row.clause_local_extraction_proxy == "kernel_invalid_bare_univ";
    let boundary_hit = row.class == "Hit";

    let c1_substitution = if kernel_invalid {
        not_applicable("C1", "ARCHIVED_KERNEL_INVALID_ROW", &schema3_row_digest)
    } else {
        row_obligation_status(&bundle.c1_substitution, &schema3_row_digest)
    };
    let c2_naturality = if kernel_invalid {
        not_applicable("C2", "ARCHIVED_KERNEL_INVALID_ROW", &schema3_row_digest)
    } else {
        row_obligation_status(&bundle.c2_naturality, &schema3_row_digest)
    };
    let c3_fuel = if kernel_invalid {
        not_applicable("C3", "ARCHIVED_KERNEL_INVALID_ROW", &schema3_row_digest)
    } else {
        row_obligation_status(&bundle.c3_fuel, &schema3_row_digest)
    };
    let c5_window = row_obligation_status(&bundle.c5_window, &schema3_row_digest);
    let c6_boundary_terms = if boundary_hit {
        row_obligation_status(&bundle.c6_boundary_terms, &schema3_row_digest)
    } else {
        not_applicable("C6", "NON_HIT_QUOTIENT_ROW", &schema3_row_digest)
    };
    let c7_base_bindings = if boundary_hit {
        row_obligation_status(&bundle.c7_base_bindings, &schema3_row_digest)
    } else {
        not_applicable("C7", "NON_HIT_QUOTIENT_ROW", &schema3_row_digest)
    };
    let c8_charging = if boundary_hit {
        row_obligation_status(&bundle.c8_charging, &schema3_row_digest)
    } else {
        not_applicable("C8", "NON_HIT_QUOTIENT_ROW", &schema3_row_digest)
    };
    let statuses = [
        &c1_substitution,
        &c2_naturality,
        &c3_fuel,
        &c5_window,
        &c6_boundary_terms,
        &c7_base_bindings,
        &c8_charging,
    ];
    let all_required_evidence_proved = !kernel_invalid
        && statuses
            .iter()
            .all(|status| status.is_proved_or_not_applicable());
    // C2 and C5 are definitionally open in this schema.  These assertions
    // make accidental promotion a construction-time failure as well as a
    // replay-time mismatch.
    assert!(!all_required_evidence_proved);
    let implemented_egp_bridge_returns_bound = false;
    let intended_typed_egp_proved = false;
    let full_candidate_extraction_join_proved = false;
    let derivation_hash = tagged_digest(
        "candidate-bridge-row",
        &(
            stratum_index,
            row_index,
            &schema3_row_digest,
            &c1_substitution,
            &c2_naturality,
            &c3_fuel,
            &c5_window,
            &c6_boundary_terms,
            &c7_base_bindings,
            &c8_charging,
            all_required_evidence_proved,
            implemented_egp_bridge_returns_bound,
            intended_typed_egp_proved,
            full_candidate_extraction_join_proved,
        ),
    );
    CandidateBridgeRow {
        stratum_index: u16::try_from(stratum_index).expect("three strata fit u16"),
        row_index: u16::try_from(row_index).expect("schema-3 row count fits u16"),
        kappa: row.kappa,
        class: row.class.clone(),
        clause_local_extraction_proxy: row.clause_local_extraction_proxy.clone(),
        direct_support: row.direct_support,
        amplification_route: row.amplification_route.clone(),
        count: row.count.clone(),
        schema3_row_digest,
        c1_substitution,
        c2_naturality,
        c3_fuel,
        c5_window,
        c6_boundary_terms,
        c7_base_bindings,
        c8_charging,
        all_required_evidence_proved,
        implemented_egp_bridge_returns_bound,
        intended_typed_egp_proved,
        full_candidate_extraction_join_proved,
        derivation_hash,
    }
}

fn certificate_digest(certificate: &CandidateJoinV4Certificate) -> String {
    let mut payload = certificate.clone();
    payload.digest.clear();
    tagged_digest("candidate-join-v4-certificate", &payload)
}

pub fn run_candidate_join_v4(
    schema3_json: &[u8],
) -> Result<CandidateJoinV4Certificate, CandidateJoinV4Error> {
    let (projection, schema3_archive) = validate_schema3_archive(schema3_json)?;
    let theorem_bundle = derive_theorem_bundle()?;
    let mut strata = Vec::with_capacity(projection.strata.len());
    for (stratum_index, source) in projection.strata.iter().enumerate() {
        let rows = source
            .rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| build_row(stratum_index, row_index, row, &theorem_bundle))
            .collect::<Vec<_>>();
        let all_archived_rows_joined = rows.len() == source.rows.len()
            && rows.iter().enumerate().all(|(row_index, row)| {
                usize::from(row.row_index) == row_index
                    && usize::from(row.stratum_index) == stratum_index
            });
        let no_row_promoted_across_open_gap = rows.iter().all(|row| {
            !row.all_required_evidence_proved
                && !row.implemented_egp_bridge_returns_bound
                && !row.intended_typed_egp_proved
                && !row.full_candidate_extraction_join_proved
        });
        let derivation_hash = tagged_digest(
            "bridge-stratum",
            &(
                stratum_index,
                source.kappa,
                &rows,
                all_archived_rows_joined,
                no_row_promoted_across_open_gap,
            ),
        );
        strata.push(BridgeStratum {
            stratum_index: u16::try_from(stratum_index).expect("three strata fit u16"),
            kappa: source.kappa,
            rows,
            all_archived_rows_joined,
            no_row_promoted_across_open_gap,
            derivation_hash,
        });
    }
    let all_213_rows_joined = strata
        .iter()
        .map(|stratum| stratum.rows.len())
        .sum::<usize>()
        == 213
        && strata
            .iter()
            .all(|stratum| stratum.all_archived_rows_joined);
    let every_applicable_c2_row_has_named_gap = strata.iter().flat_map(|s| &s.rows).all(|row| {
        if row.clause_local_extraction_proxy == "kernel_invalid_bare_univ" {
            matches!(row.c2_naturality, RowObligationStatus::NotApplicable { .. })
        } else {
            matches!(
                &row.c2_naturality,
                RowObligationStatus::Gap { gap_id, .. } if gap_id == C2_COMPLETENESS_GAP
            )
        }
    });
    let no_row_promoted_across_open_gap = strata
        .iter()
        .all(|stratum| stratum.no_row_promoted_across_open_gap);
    let completeness = CandidateJoinV4Completeness {
        exact_schema3_archive_bound: true,
        allowed_projection_only: true,
        all_213_rows_joined,
        every_applicable_c2_row_has_named_gap,
        no_row_promoted_across_open_gap,
        c1_arbitrary_instance_sort_preservation_proved: theorem_bundle
            .c1_substitution
            .status
            .is_proved(),
        c2_depth_two_schema_completeness_proved: theorem_bundle.c2_naturality.status.is_proved(),
        c3_domain_wide_fuel_compositionality_proved: theorem_bundle.c3_fuel.status.is_proved(),
        c5_contextual_substitution_closure_proved: theorem_bundle.c5_window.status.is_proved(),
        c6_v2_general_historical_term_completion_proved: theorem_bundle
            .c6_boundary_terms
            .status
            .is_proved(),
        c7_all_historical_base_bindings_proved: theorem_bundle.c7_base_bindings.status.is_proved(),
        c8_registered_reference_only_charging_proved: theorem_bundle
            .c8_charging
            .scoped_evidence
            .iter()
            .any(|evidence| {
                evidence.evidence_id == C8_REFERENCE_ONLY_PROOF && evidence.status.is_proved()
            }),
        c8_candidate_boundary_provenance_join_proved: theorem_bundle.c8_charging.status.is_proved(),
        full_candidate_extraction_join_proved: false,
        intended_semantic_candidate_join_proved: false,
    };
    let mut certificate = CandidateJoinV4Certificate {
        schema_version: CANDIDATE_JOIN_V4_SCHEMA_VERSION,
        date: CANDIDATE_JOIN_V4_DATE.to_owned(),
        schema3_archive,
        theorem_bundle,
        strata,
        completeness,
        digest: String::new(),
    };
    certificate.digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn candidate_join_v4_json_pretty(schema3_json: &[u8]) -> Result<String, CandidateJoinV4Error> {
    let certificate = run_candidate_join_v4(schema3_json)?;
    serde_json::to_string_pretty(&certificate)
        .map_err(|error| CandidateJoinV4Error::InvalidSchema4Json(error.to_string()))
}

fn replay_against_expected(
    certificate: &CandidateJoinV4Certificate,
    expected: Result<CandidateJoinV4Certificate, CandidateJoinV4Error>,
) -> CandidateJoinV4Replay {
    let mut errors = Vec::new();
    if certificate.schema_version != CANDIDATE_JOIN_V4_SCHEMA_VERSION {
        errors.push(format!(
            "schema mismatch: expected {}, got {}",
            CANDIDATE_JOIN_V4_SCHEMA_VERSION, certificate.schema_version
        ));
    }
    if certificate_digest(certificate) != certificate.digest {
        errors.push("schema-4 outer digest mismatch".to_owned());
    }
    match expected {
        Ok(expected) if expected != *certificate => {
            errors.push("schema-4 payload differs from definition replay".to_owned());
        }
        Ok(_) => {}
        Err(error) => errors.push(format!("schema-4 definition replay failed: {error}")),
    }
    CandidateJoinV4Replay {
        valid: errors.is_empty(),
        exact_schema3_archive_bound: certificate.completeness.exact_schema3_archive_bound,
        all_213_rows_joined: certificate.completeness.all_213_rows_joined,
        no_row_promoted_across_open_gap: certificate.completeness.no_row_promoted_across_open_gap,
        full_candidate_extraction_join_proved: certificate
            .completeness
            .full_candidate_extraction_join_proved,
        intended_semantic_candidate_join_proved: certificate
            .completeness
            .intended_semantic_candidate_join_proved,
        errors,
    }
}

pub fn replay_candidate_join_v4(
    schema3_json: &[u8],
    certificate: &CandidateJoinV4Certificate,
) -> CandidateJoinV4Replay {
    replay_against_expected(certificate, run_candidate_join_v4(schema3_json))
}

pub fn replay_candidate_join_v4_json(
    schema3_json: &[u8],
    schema4_json: &str,
) -> CandidateJoinV4Replay {
    match serde_json::from_str::<CandidateJoinV4Certificate>(schema4_json) {
        Ok(certificate) => replay_candidate_join_v4(schema3_json, &certificate),
        Err(error) => CandidateJoinV4Replay {
            valid: false,
            exact_schema3_archive_bound: false,
            all_213_rows_joined: false,
            no_row_promoted_across_open_gap: false,
            full_candidate_extraction_join_proved: false,
            intended_semantic_candidate_join_proved: false,
            errors: vec![format!("invalid schema-4 JSON: {error}")],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mutate_status_hash(status: &mut RowObligationStatus) {
        match status {
            RowObligationStatus::Proved { evidence_hash, .. }
            | RowObligationStatus::Gap { evidence_hash, .. }
            | RowObligationStatus::NotApplicable { evidence_hash, .. } => {
                evidence_hash.push_str(":mutated");
            }
        }
    }

    fn flip_status(status: &mut RowObligationStatus) {
        *status = match status {
            RowObligationStatus::Proved { evidence_hash, .. } => RowObligationStatus::Gap {
                gap_id: "mutated-gap".to_owned(),
                evidence_hash: evidence_hash.clone(),
            },
            RowObligationStatus::Gap { evidence_hash, .. }
            | RowObligationStatus::NotApplicable { evidence_hash, .. } => {
                RowObligationStatus::Proved {
                    proof_id: "mutated-proof".to_owned(),
                    evidence_hash: evidence_hash.clone(),
                }
            }
        };
    }

    fn row_status_mut(row: &mut CandidateBridgeRow, index: usize) -> &mut RowObligationStatus {
        match index {
            0 => &mut row.c1_substitution,
            1 => &mut row.c2_naturality,
            2 => &mut row.c3_fuel,
            3 => &mut row.c5_window,
            4 => &mut row.c6_boundary_terms,
            5 => &mut row.c7_base_bindings,
            6 => &mut row.c8_charging,
            _ => panic!("row obligation index out of range"),
        }
    }

    fn obligation_mut(
        bundle: &mut KernelBridgeTheoremBundle,
        index: usize,
    ) -> &mut ObligationEvidence {
        match index {
            0 => &mut bundle.c1_substitution,
            1 => &mut bundle.c2_naturality,
            2 => &mut bundle.c3_fuel,
            3 => &mut bundle.c5_window,
            4 => &mut bundle.c6_boundary_terms,
            5 => &mut bundle.c7_base_bindings,
            6 => &mut bundle.c8_charging,
            _ => panic!("bundle obligation index out of range"),
        }
    }

    fn flip_evidence_status(status: &mut EvidenceStatus) {
        *status = match status {
            EvidenceStatus::Proved { .. } => evidence_status_gap("mutated-gap"),
            EvidenceStatus::Gap { .. } => evidence_status_proved("mutated-proof"),
        };
    }

    fn assert_rejected(
        expected: &CandidateJoinV4Certificate,
        mut mutated: CandidateJoinV4Certificate,
    ) {
        mutated.digest = certificate_digest(&mutated);
        assert!(!replay_against_expected(&mutated, Ok(expected.clone())).valid);
    }

    #[test]
    fn archive_and_all_rows_are_joined_without_promotion() {
        let certificate = run_candidate_join_v4(EMBEDDED_SCHEMA3_ARCHIVE).expect("schema 4");
        assert_eq!(certificate.schema3_archive.total_rows, 213);
        assert_eq!(
            certificate.schema3_archive.stratum_row_counts,
            vec![63, 72, 78]
        );
        assert!(certificate.completeness.exact_schema3_archive_bound);
        assert!(certificate.completeness.allowed_projection_only);
        assert!(certificate.completeness.all_213_rows_joined);
        assert!(
            certificate
                .completeness
                .every_applicable_c2_row_has_named_gap
        );
        assert!(certificate.completeness.no_row_promoted_across_open_gap);
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
        assert!(
            !certificate
                .completeness
                .c6_v2_general_historical_term_completion_proved
        );
        assert!(
            !certificate
                .completeness
                .c7_all_historical_base_bindings_proved
        );
        let p5 = &certificate.theorem_bundle.p5_window_reaudit;
        assert!(!p5.raw_surface_member);
        assert!(p5.public_lift_token_issued);
        assert!(p5.replay_adapter_succeeded);
        assert!(p5.restricted_variable_images_preserve_library_support);
        assert!(!p5.arbitrary_contextual_substitution_inventory_complete);
        assert_eq!(
            p5.in_surface_route_opened_under_complete_typed_substitution,
            None
        );
        assert_eq!(p5.gap_id, C5_CONTEXTUAL_CLOSURE_GAP);
        let rows = certificate.strata.iter().flat_map(|stratum| &stratum.rows);
        assert_eq!(rows.count(), 213);
        assert!(certificate.strata.iter().flat_map(|s| &s.rows).all(|row| {
            !row.all_required_evidence_proved
                && !row.implemented_egp_bridge_returns_bound
                && !row.intended_typed_egp_proved
                && !row.full_candidate_extraction_join_proved
        }));
        assert!(replay_candidate_join_v4(EMBEDDED_SCHEMA3_ARCHIVE, &certificate).valid);
    }

    #[test]
    fn replay_rejects_every_row_status_hash_and_flag_mutation() {
        let expected = run_candidate_join_v4(EMBEDDED_SCHEMA3_ARCHIVE).expect("schema 4");
        for stratum_index in 0..expected.strata.len() {
            for row_index in 0..expected.strata[stratum_index].rows.len() {
                for obligation_index in 0..7 {
                    let mut hash_mutation = expected.clone();
                    mutate_status_hash(row_status_mut(
                        &mut hash_mutation.strata[stratum_index].rows[row_index],
                        obligation_index,
                    ));
                    assert_rejected(&expected, hash_mutation);

                    let mut status_mutation = expected.clone();
                    flip_status(row_status_mut(
                        &mut status_mutation.strata[stratum_index].rows[row_index],
                        obligation_index,
                    ));
                    assert_rejected(&expected, status_mutation);
                }
                for flag_index in 0..4 {
                    let mut mutation = expected.clone();
                    let row = &mut mutation.strata[stratum_index].rows[row_index];
                    match flag_index {
                        0 => row.all_required_evidence_proved = !row.all_required_evidence_proved,
                        1 => {
                            row.implemented_egp_bridge_returns_bound =
                                !row.implemented_egp_bridge_returns_bound;
                        }
                        2 => row.intended_typed_egp_proved = !row.intended_typed_egp_proved,
                        3 => {
                            row.full_candidate_extraction_join_proved =
                                !row.full_candidate_extraction_join_proved;
                        }
                        _ => unreachable!(),
                    }
                    assert_rejected(&expected, mutation);
                }
                let mut row_hash = expected.clone();
                row_hash.strata[stratum_index].rows[row_index]
                    .derivation_hash
                    .push_str(":mutated");
                assert_rejected(&expected, row_hash);

                let mut archive_row_hash = expected.clone();
                archive_row_hash.strata[stratum_index].rows[row_index]
                    .schema3_row_digest
                    .push_str(":mutated");
                assert_rejected(&expected, archive_row_hash);
            }
        }
    }

    #[test]
    fn replay_rejects_every_theorem_bundle_field_mutation() {
        let expected = run_candidate_join_v4(EMBEDDED_SCHEMA3_ARCHIVE).expect("schema 4");
        for obligation_index in 0..7 {
            let mut id = expected.clone();
            obligation_mut(&mut id.theorem_bundle, obligation_index)
                .obligation_id
                .push_str(":mutated");
            assert_rejected(&expected, id);

            let mut status = expected.clone();
            flip_evidence_status(
                &mut obligation_mut(&mut status.theorem_bundle, obligation_index).status,
            );
            assert_rejected(&expected, status);

            let mut hash = expected.clone();
            obligation_mut(&mut hash.theorem_bundle, obligation_index)
                .derivation_hash
                .push_str(":mutated");
            assert_rejected(&expected, hash);

            let evidence_count =
                obligation_mut(&mut expected.clone().theorem_bundle, obligation_index)
                    .scoped_evidence
                    .len();
            for evidence_index in 0..evidence_count {
                for field_index in 0..5 {
                    let mut mutation = expected.clone();
                    let evidence =
                        &mut obligation_mut(&mut mutation.theorem_bundle, obligation_index)
                            .scoped_evidence[evidence_index];
                    match field_index {
                        0 => evidence.evidence_id.push_str(":mutated"),
                        1 => evidence.implementation_version.push_str(":mutated"),
                        2 => evidence.scope.push_str(":mutated"),
                        3 => flip_evidence_status(&mut evidence.status),
                        4 => evidence.derivation_hash.push_str(":mutated"),
                        _ => unreachable!(),
                    }
                    assert_rejected(&expected, mutation);
                }
            }
        }
        let mut bundle_hash = expected.clone();
        bundle_hash
            .theorem_bundle
            .derivation_hash
            .push_str(":mutated");
        assert_rejected(&expected, bundle_hash);

        for field_index in 0..16 {
            let mut mutation = expected.clone();
            let p5 = &mut mutation.theorem_bundle.p5_window_reaudit;
            match field_index {
                0 => p5.candidate.clauses[0].expr = Expr::Univ,
                1 => p5.raw_surface_member = !p5.raw_surface_member,
                2 => p5.raw_surface_rejections.push("mutated".to_owned()),
                3 => p5.public_lift_token_issued = !p5.public_lift_token_issued,
                4 => p5.token_dominant_import = None,
                5 => p5.token_lift_clauses.push(1),
                6 => p5
                    .token_derivation_hash
                    .as_mut()
                    .expect("literal witness has token hash")
                    .push_str(":mutated"),
                7 => p5.replay_adapter_succeeded = !p5.replay_adapter_succeeded,
                8 => {
                    p5.record_internality_capability_available =
                        !p5.record_internality_capability_available
                }
                9 => p5.full_p5_route_constructible = !p5.full_p5_route_constructible,
                10 => {
                    p5.restricted_variable_images_preserve_library_support =
                        !p5.restricted_variable_images_preserve_library_support
                }
                11 => p5.restricted_support_derivation_hash.push_str(":mutated"),
                12 => {
                    p5.arbitrary_contextual_substitution_inventory_complete =
                        !p5.arbitrary_contextual_substitution_inventory_complete
                }
                13 => p5.in_surface_route_opened_under_complete_typed_substitution = Some(false),
                14 => p5.gap_id.push_str(":mutated"),
                15 => p5.derivation_hash.push_str(":mutated"),
                _ => unreachable!(),
            }
            assert_rejected(&expected, mutation);
        }
    }

    #[test]
    fn replay_rejects_archive_completeness_and_outer_digest_mutations() {
        let expected = run_candidate_join_v4(EMBEDDED_SCHEMA3_ARCHIVE).expect("schema 4");
        for field_index in 0..10 {
            let mut mutation = expected.clone();
            match field_index {
                0 => mutation.schema3_archive.byte_length += 1,
                1 => mutation.schema3_archive.sha256.push_str(":mutated"),
                2 => mutation.schema3_archive.git_blob.push_str(":mutated"),
                3 => mutation
                    .schema3_archive
                    .internal_digest
                    .push_str(":mutated"),
                4 => mutation
                    .schema3_archive
                    .signature_digest
                    .push_str(":mutated"),
                5 => mutation.schema3_archive.closure_digest.push_str(":mutated"),
                6 => mutation
                    .schema3_archive
                    .orbit_derivation_hash
                    .push_str(":mutated"),
                7 => mutation.schema3_archive.stratum_row_counts[0] += 1,
                8 => mutation.schema3_archive.total_rows += 1,
                9 => mutation
                    .schema3_archive
                    .allowed_projection_digest
                    .push_str(":mutated"),
                _ => unreachable!(),
            }
            assert_rejected(&expected, mutation);
        }

        for field_index in 0..16 {
            let mut mutation = expected.clone();
            let completeness = &mut mutation.completeness;
            match field_index {
                0 => {
                    completeness.exact_schema3_archive_bound =
                        !completeness.exact_schema3_archive_bound
                }
                1 => completeness.allowed_projection_only = !completeness.allowed_projection_only,
                2 => completeness.all_213_rows_joined = !completeness.all_213_rows_joined,
                3 => {
                    completeness.every_applicable_c2_row_has_named_gap =
                        !completeness.every_applicable_c2_row_has_named_gap
                }
                4 => {
                    completeness.no_row_promoted_across_open_gap =
                        !completeness.no_row_promoted_across_open_gap
                }
                5 => {
                    completeness.c1_arbitrary_instance_sort_preservation_proved =
                        !completeness.c1_arbitrary_instance_sort_preservation_proved
                }
                6 => {
                    completeness.c2_depth_two_schema_completeness_proved =
                        !completeness.c2_depth_two_schema_completeness_proved
                }
                7 => {
                    completeness.c3_domain_wide_fuel_compositionality_proved =
                        !completeness.c3_domain_wide_fuel_compositionality_proved
                }
                8 => {
                    completeness.c5_contextual_substitution_closure_proved =
                        !completeness.c5_contextual_substitution_closure_proved
                }
                9 => {
                    completeness.c6_v2_general_historical_term_completion_proved =
                        !completeness.c6_v2_general_historical_term_completion_proved
                }
                10 => {
                    completeness.c7_all_historical_base_bindings_proved =
                        !completeness.c7_all_historical_base_bindings_proved
                }
                11 => {
                    completeness.c8_registered_reference_only_charging_proved =
                        !completeness.c8_registered_reference_only_charging_proved
                }
                12 => {
                    completeness.c8_candidate_boundary_provenance_join_proved =
                        !completeness.c8_candidate_boundary_provenance_join_proved
                }
                13 => {
                    completeness.full_candidate_extraction_join_proved =
                        !completeness.full_candidate_extraction_join_proved
                }
                14 => {
                    completeness.intended_semantic_candidate_join_proved =
                        !completeness.intended_semantic_candidate_join_proved
                }
                15 => mutation.schema_version += 1,
                _ => unreachable!(),
            }
            assert_rejected(&expected, mutation);
        }

        let mut digest = expected.clone();
        digest.digest.push_str(":mutated");
        assert!(!replay_against_expected(&digest, Ok(expected)).valid);
    }

    #[test]
    fn runtime_schema3_must_match_compile_time_bytes() {
        let mut mutated = EMBEDDED_SCHEMA3_ARCHIVE.to_vec();
        mutated[0] ^= 1;
        assert!(run_candidate_join_v4(&mutated).is_err());
    }
}
