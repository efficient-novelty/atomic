//! E-2b quotient closure over the completed E-4 basis.
//!
//! This successor consumes the final R1/R2 membership verdicts, mints a
//! replayable typed/normalized/natural token for every ordinary historical
//! row, and issues an ordinary-family token only for quotient-independent
//! rows.  Historical counts are read only after all quotient decisions have
//! been sealed, as comparators for F-Q2.

use crate::completed_basis_membership::{
    COMPLETED_BASIS_MEMBERSHIP_SCHEMA, CompletedBasisMembershipCertificate, FinalMembershipSubject,
    FinalMembershipVerdict, replay_completed_basis_membership_json,
};
use pen_core::hash::blake3_hex;
use pen_eval::tdc1_hist_cert::{HistoricalFamilyKey, PostPathRole};
use pen_eval::tdc1_hist_cert_v3::{
    HIST_CERT_V3_SCHEMA, HistCertV3Certificate, HistCertV3PackageAudit, OrdinaryNaturalFamilyGap,
    replay_hist_cert_v3_json,
};
use pen_schema::context::{
    BinderId, Declaration, FormedSchemaContext, SubstitutionImage, TermExpr, TypeExpr,
    TypedSubstitutionToken, form_schema_context, issue_typed_substitution,
};
use pen_schema::e3_normalization::{
    issue_ordinary_schema_normalization, replay_ordinary_schema_normalization,
};
use pen_schema::e34_class_induction::{
    issue_ordinary_constructor_naturality, replay_ordinary_constructor_naturality,
};
use pen_schema::grammar::{
    ClauseAnchor, DerivationRef, FamilyPresentation, OrdinaryInterpretation, OrdinarySchema,
    OrdinarySchemaKind, SupportWindow, UnitOrientation, form_ordinary_schema,
};
use pen_schema::ordinary::{issue_ordinary_typed_realizer, replay_ordinary_typed_realizer};
use pen_schema::stage1_r1::{issue_stage1_r1_package_token, replay_stage1_r1_package_token};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const E2B_QUOTIENT_CLOSURE_SCHEMA: &str = "schema2-e2b-quotient-closure-v1";
pub const E2B_QUOTIENT_CLOSURE_DATE: &str = "2026-07-21";

const MEMBERSHIP_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_completed_basis_membership_verdicts_v1.json");
const HIST_CERT_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/hist_cert_v3.json");
const STAGE1_DIAGNOSTIC_BYTES: &[u8] = include_bytes!("../../../docs/schema2_v1.json");
const M1_ARTIFACT_BYTES: &[u8] = include_bytes!("../../../docs/schema2_m1_sweep_v1.json");
const R1_R2_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/e2_quotient_adjudications.md");
const P1_ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_phase_order_adjudication.md");
const MEMBERSHIP_SOURCE_BYTES: &[u8] = include_bytes!("completed_basis_membership.rs");
const HIST_CERT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/tdc1_hist_cert_v3.rs");
const ORDINARY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/ordinary.rs");
const GRAMMAR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/grammar.rs");
const NORMALIZATION_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/e3_normalization.rs");
const NATURALITY_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/e34_class_induction.rs");
const KERNEL_NORMALIZE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("e2b_quotient_closure.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2bSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum E2bQuotientDisposition {
    CanonicalCompletedPackageFamily,
    StructuralNaturalFamily,
    IndependentR2Family,
    GeneratedR2Instance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyDistinctnessComparison {
    pub prior_step: u32,
    pub prior_key: HistoricalFamilyKey,
    pub prior_owner_subject_hash: String,
    pub prior_family_id: String,
    pub same_constructor_kind: bool,
    pub same_source_key: bool,
    pub same_owner_subject: bool,
    pub decision: String,
    pub reason: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrdinaryFamilyTokenProjection {
    pub token_version: String,
    pub step: u32,
    pub key: HistoricalFamilyKey,
    pub constructor_kind: String,
    pub owner_subject_hash: String,
    pub source_clauses: Vec<u16>,
    pub disposition: E2bQuotientDisposition,
    pub typed_realizer_derivation_hash: String,
    pub normalization_derivation_hash: String,
    pub naturality_derivation_hash: String,
    pub completed_basis_derivation_hash: String,
    pub predecessor_and_same_stage_comparisons: Vec<FamilyDistinctnessComparison>,
    pub full_predecessor_sweep_proved: bool,
    pub distinct_from_every_previously_issued_family: bool,
    pub ordinary_family_issued: bool,
    pub counts_as_one_quotient_family: bool,
    pub historical_count_used_as_input: bool,
    pub acceptance_bar_used_as_input: bool,
    pub family_id: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratedInstanceTokenProjection {
    pub token_version: String,
    pub step: u32,
    pub key: HistoricalFamilyKey,
    pub constructor_kind: String,
    pub typed_realizer_derivation_hash: String,
    pub normalization_derivation_hash: String,
    pub naturality_derivation_hash: String,
    pub parent_family_id: String,
    pub parent_family_derivation_hash: String,
    pub generating_sub_basis_digest: String,
    pub generated_subject: String,
    pub monotone_generated_verdict_replayed: bool,
    pub ordinary_family_issued: bool,
    pub counts_as_one_quotient_family: bool,
    pub historical_count_used_as_input: bool,
    pub acceptance_bar_used_as_input: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrdinaryRowResolution {
    pub step: u32,
    pub key: HistoricalFamilyKey,
    pub judgement: String,
    pub source_clauses: Vec<u16>,
    pub required_opaque_token: String,
    pub constructor_kind: String,
    pub disposition: E2bQuotientDisposition,
    pub row_typed: bool,
    pub row_normalized: bool,
    pub row_natural: bool,
    pub row_operationally_covered: bool,
    pub family_token: Option<OrdinaryFamilyTokenProjection>,
    pub generated_instance_token: Option<GeneratedInstanceTokenProjection>,
    pub exactly_one_quotient_resolution_issued: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistoricalStepCountOutput {
    pub step: u32,
    pub sealed_recorded_total: u32,
    pub registered_path_subtotal: u32,
    pub ordinary_operational_row_count: u32,
    pub ordinary_family_token_count: u32,
    pub generated_instance_row_count: u32,
    pub operational_row_coverage_total: u32,
    pub quotient_family_total: u32,
    pub every_ordinary_row_covered: bool,
    pub operational_row_coverage_matches_seal: bool,
    pub quotient_family_total_matches_seal: bool,
    pub count_was_output_only: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage1CountOutput {
    pub sealed_recorded_total: u32,
    pub completed_package_family_count: u32,
    pub separately_authorized_carrier_family_count: u32,
    pub quotient_family_total: u32,
    pub matches_seal: bool,
    pub package_rule_applied_before_comparator_read: bool,
    pub completed_membership_applied_before_comparator_read: bool,
    pub historical_count_used_as_input: bool,
    pub acceptance_bar_used_as_input: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fq2Divergence {
    pub stage: u32,
    pub sealed_recorded_total: u32,
    pub quotient_family_total: u32,
    pub signed_delta: i32,
    pub exact_cause: String,
    pub rules_retuned: bool,
    pub history_edited: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fq2Audit {
    pub falsifier: String,
    pub comparator_read_only_after_quotient_closure: bool,
    pub stage1_compared: bool,
    pub historical_steps_compared: Vec<u32>,
    pub all_outputs_match_sealed_record: bool,
    pub divergences: Vec<Fq2Divergence>,
    pub fq2_triggered: bool,
    pub certified_divergence: bool,
    pub phase5b_fork_route_registered: bool,
    pub phase5b_fork_executed: bool,
    pub history_earned_without_divergence: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentAHandoff {
    pub issued: bool,
    pub ordinary_family_token_hashes: Vec<String>,
    pub generated_instance_token_hashes: Vec<String>,
    pub registered_path_family_token_hashes: Vec<String>,
    pub e2b_count_outputs: Vec<HistoricalStepCountOutput>,
    pub fq2_derivation_hash: String,
    pub fq2_triggered: bool,
    pub agent_a_executed: bool,
    pub f_t1_evaluated: bool,
    pub full_historical_totals_certified_here: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2bDeferredSequence {
    pub agent_a_ft1_executed: bool,
    pub e5_f1_executed: bool,
    pub bridge_executed: bool,
    pub phase5b_fork_executed: bool,
    pub halt_or_continuation_claimed: bool,
}

impl E2bDeferredSequence {
    fn all_deferred(&self) -> bool {
        !self.agent_a_ft1_executed
            && !self.e5_f1_executed
            && !self.bridge_executed
            && !self.phase5b_fork_executed
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2bQuotientClosureCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<E2bSourceBinding>,
    pub completed_membership_schema: String,
    pub completed_membership_digest: String,
    pub completed_membership_replayed: bool,
    pub hist_cert_schema: String,
    pub hist_cert_digest: String,
    pub hist_cert_replayed: bool,
    pub adopted_r1_r2_replayed: bool,
    pub adopted_p1_replayed: bool,
    pub stage1: Stage1CountOutput,
    pub ordinary_rows: Vec<OrdinaryRowResolution>,
    pub ordinary_rows_expected: u32,
    pub ordinary_rows_covered: u32,
    pub ordinary_family_tokens_minted: u32,
    pub generated_instance_tokens_minted: u32,
    pub every_typed_token_replayed: bool,
    pub every_normalization_token_replayed: bool,
    pub every_naturality_token_replayed: bool,
    pub every_ordinary_row_resolved: bool,
    pub historical_count_outputs: Vec<HistoricalStepCountOutput>,
    pub counts_as_outputs_regression_run: bool,
    pub fq2: Fq2Audit,
    pub agent_a_handoff: AgentAHandoff,
    pub e2b_complete: bool,
    pub downstream: E2bDeferredSequence,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2bQuotientClosureReplay {
    pub valid: bool,
    pub e2b_complete: bool,
    pub ordinary_rows_expected: u32,
    pub ordinary_rows_covered: u32,
    pub ordinary_family_tokens_minted: u32,
    pub generated_instance_tokens_minted: u32,
    pub stage1_output: u32,
    pub historical_quotient_outputs: Vec<u32>,
    pub historical_operational_coverage_outputs: Vec<u32>,
    pub fq2_triggered: bool,
    pub divergence_stages: Vec<u32>,
    pub agent_a_handoff_issued: bool,
    pub downstream_deferred: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum E2bQuotientClosureError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("ordinary token issuance failed: {0}")]
    Token(String),
    #[error("E-2b invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

#[derive(Clone)]
struct IssuedFamilyIndex {
    step: u32,
    key: HistoricalFamilyKey,
    constructor: OrdinarySchemaKind,
    owner_subject_hash: String,
    family_id: String,
    derivation_hash: String,
}

fn workspace_doc_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs")
        .join(name)
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(E2B_QUOTIENT_CLOSURE_SCHEMA, domain, value))
        .expect("E-2b evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<E2bSourceBinding> {
    [
        (
            "docs/schema2_completed_basis_membership_verdicts_v1.json",
            "final_r1_r2_membership",
            MEMBERSHIP_ARTIFACT_BYTES,
        ),
        (
            "docs/hist_cert_v3.json",
            "historical_ordinary_rows_and_fq2_comparators",
            HIST_CERT_ARTIFACT_BYTES,
        ),
        (
            "docs/schema2_v1.json",
            "frozen_stage1_comparator_projection",
            STAGE1_DIAGNOSTIC_BYTES,
        ),
        (
            "docs/schema2_m1_sweep_v1.json",
            "monotone_step8_cell_generated_verdict",
            M1_ARTIFACT_BYTES,
        ),
        (
            "docs/e2_quotient_adjudications.md",
            "adopted_r1_r2_and_fq2",
            R1_R2_ADJUDICATION_BYTES,
        ),
        (
            "docs/e2_phase_order_adjudication.md",
            "adopted_e2b_order_and_m1",
            P1_ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/completed_basis_membership.rs",
            "membership_issuer",
            MEMBERSHIP_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/tdc1_hist_cert_v3.rs",
            "historical_surface_issuer",
            HIST_CERT_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/ordinary.rs",
            "typed_ordinary_realizer_issuer",
            ORDINARY_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/grammar.rs",
            "ordinary_schema_grammar",
            GRAMMAR_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/e3_normalization.rs",
            "frozen_normalization_issuer",
            NORMALIZATION_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/e34_class_induction.rs",
            "constructor_naturality_issuer",
            NATURALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/normalize.rs",
            "kernel_normalization_source",
            KERNEL_NORMALIZE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/e2b_quotient_closure.rs",
            "this_successor_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| E2bSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn source_context() -> Result<FormedSchemaContext, E2bQuotientClosureError> {
    form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(0),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::TypeParameter {
            binder: BinderId(1),
            name: "B".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(2),
            name: "a".to_owned(),
            ty: TypeExpr::parameter(0),
        },
    ])
    .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))
}

fn naturality_substitution(
    source: &FormedSchemaContext,
) -> Result<TypedSubstitutionToken, E2bQuotientClosureError> {
    let target = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(10),
            name: "B".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(11),
            name: "b".to_owned(),
            ty: TypeExpr::parameter(10),
        },
    ])
    .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    issue_typed_substitution(
        source,
        &target,
        vec![
            SubstitutionImage::Type {
                source: BinderId(0),
                image: TypeExpr::trunc(TypeExpr::parameter(10)),
            },
            SubstitutionImage::Type {
                source: BinderId(1),
                image: TypeExpr::parameter(10),
            },
            SubstitutionImage::Term {
                source: BinderId(2),
                image: TermExpr::TruncPoint {
                    carrier: Box::new(TypeExpr::parameter(10)),
                    point: Box::new(TermExpr::variable(11)),
                },
            },
        ],
    )
    .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))
}

fn constructor_for(key: &HistoricalFamilyKey) -> OrdinarySchemaKind {
    match key {
        HistoricalFamilyKey::Formation { .. } => OrdinarySchemaKind::FreshFormation,
        HistoricalFamilyKey::PointIntroduction { .. } => OrdinarySchemaKind::PointOrUnitIntro,
        HistoricalFamilyKey::PathIntroduction { .. } => OrdinarySchemaKind::PathConstructorIntro,
        HistoricalFamilyKey::Recursor { .. } => OrdinarySchemaKind::Recursor,
        HistoricalFamilyKey::Inductor { .. } => OrdinarySchemaKind::Inductor,
        HistoricalFamilyKey::ParametricAction { .. } => OrdinarySchemaKind::TruncParametricAction,
        HistoricalFamilyKey::Path { .. } => {
            unreachable!("registered path families are not ordinary gaps")
        }
        HistoricalFamilyKey::PostPathForward {
            role: PostPathRole::OperationForward,
            ..
        } => OrdinarySchemaKind::PostPathOperation,
        HistoricalFamilyKey::PostPathForward {
            role: PostPathRole::CoherenceForward,
            ..
        } => OrdinarySchemaKind::PostPathCoherence,
        HistoricalFamilyKey::CellAction { .. } => OrdinarySchemaKind::CellAction,
    }
}

fn constructor_name(kind: OrdinarySchemaKind) -> String {
    serde_json::to_value(kind)
        .expect("kind serializes")
        .as_str()
        .expect("kind is a string")
        .to_owned()
}

fn interpretation_for(
    key: &HistoricalFamilyKey,
    package: &HistCertV3PackageAudit,
    parent_operation: Option<&IssuedFamilyIndex>,
) -> Result<OrdinaryInterpretation, E2bQuotientClosureError> {
    let carrier = TypeExpr::parameter(0);
    let point = TermExpr::variable(2);
    Ok(match key {
        HistoricalFamilyKey::Formation { .. } => OrdinaryInterpretation::FreshFormation { carrier },
        HistoricalFamilyKey::PointIntroduction { .. } => {
            OrdinaryInterpretation::PointOrUnitIntro { carrier, point }
        }
        HistoricalFamilyKey::PathIntroduction { .. } => {
            OrdinaryInterpretation::PathConstructorIntro {
                carrier,
                left: point.clone(),
                right: point,
                cubical_dimension: package.dimension,
            }
        }
        HistoricalFamilyKey::Recursor { .. } => OrdinaryInterpretation::Recursor {
            carrier,
            codomain: TypeExpr::parameter(1),
        },
        HistoricalFamilyKey::Inductor { .. } => OrdinaryInterpretation::Inductor {
            carrier,
            motive_fiber: TypeExpr::parameter(1),
        },
        HistoricalFamilyKey::ParametricAction { .. } => {
            OrdinaryInterpretation::TruncParametricAction {
                source_carrier: carrier,
                target_carrier: TypeExpr::parameter(1),
            }
        }
        HistoricalFamilyKey::Path { .. } => {
            return Err(E2bQuotientClosureError::Invariant(
                "path key entered ordinary-token issuer".to_owned(),
            ));
        }
        HistoricalFamilyKey::PostPathForward {
            role: PostPathRole::OperationForward,
            ..
        } => OrdinaryInterpretation::PostPathOperation { carrier, arity: 2 },
        HistoricalFamilyKey::PostPathForward {
            role: PostPathRole::CoherenceForward,
            ..
        } => {
            let parent = parent_operation.ok_or_else(|| {
                E2bQuotientClosureError::Invariant(
                    "Step-8 coherence issued before its operation family".to_owned(),
                )
            })?;
            OrdinaryInterpretation::PostPathCoherence {
                carrier,
                operation: DerivationRef::parse(parent.derivation_hash.clone())
                    .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?,
                unit: point.clone(),
                variable: point,
                orientation: UnitOrientation::Left,
            }
        }
        HistoricalFamilyKey::CellAction { .. } => {
            let parent = parent_operation.ok_or_else(|| {
                E2bQuotientClosureError::Invariant(
                    "Step-8 cell action issued before its operation family".to_owned(),
                )
            })?;
            OrdinaryInterpretation::CellAction {
                carrier,
                operation: DerivationRef::parse(parent.derivation_hash.clone())
                    .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?,
                cell_dimension: package.dimension,
                registered_boundary_bundle: DerivationRef::parse(
                    package.bundle_derivation_hash.clone(),
                )
                .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?,
            }
        }
    })
}

fn ordinary_schema(
    source: FormedSchemaContext,
    package: &HistCertV3PackageAudit,
    gap: &OrdinaryNaturalFamilyGap,
    parent_operation: Option<&IssuedFamilyIndex>,
) -> Result<OrdinarySchema, E2bQuotientClosureError> {
    let kind = constructor_for(&gap.key);
    let interpretation = interpretation_for(&gap.key, package, parent_operation)?;
    let anchors = gap
        .source_clauses
        .iter()
        .map(|clause| ClauseAnchor {
            step: package.step,
            clause: u32::from(*clause),
        })
        .collect::<Vec<_>>();
    form_ordinary_schema(
        source,
        kind,
        interpretation,
        FamilyPresentation::CanonicalFamily,
        SupportWindow::new(package.step - 1, package.step)
            .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?,
        anchors,
    )
    .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))
}

fn distinctness_comparisons(
    current_step: u32,
    current_key: &HistoricalFamilyKey,
    current_constructor: OrdinarySchemaKind,
    current_owner: &str,
    prior: &[IssuedFamilyIndex],
) -> Result<Vec<FamilyDistinctnessComparison>, E2bQuotientClosureError> {
    prior
        .iter()
        .map(|other| {
            let same_constructor_kind = other.constructor == current_constructor;
            let same_source_key = &other.key == current_key;
            let same_owner_subject = other.owner_subject_hash == current_owner;
            if same_constructor_kind && same_source_key && same_owner_subject {
                return Err(E2bQuotientClosureError::Invariant(format!(
                    "duplicate ordinary family at step {current_step}: {current_key:?}"
                )));
            }
            let reason = if !same_source_key {
                "distinct typed constructor/source-role key"
            } else {
                "same schematic role over a fresh historical owner support"
            };
            let mut comparison = FamilyDistinctnessComparison {
                prior_step: other.step,
                prior_key: other.key.clone(),
                prior_owner_subject_hash: other.owner_subject_hash.clone(),
                prior_family_id: other.family_id.clone(),
                same_constructor_kind,
                same_source_key,
                same_owner_subject,
                decision: "distinct".to_owned(),
                reason: reason.to_owned(),
                derivation_hash: String::new(),
            };
            comparison.derivation_hash = tagged_hash(
                "ordinary-family-distinctness",
                &(current_step, current_key, current_owner, &comparison),
            );
            Ok(comparison)
        })
        .collect()
}

fn disposition_for(
    key: &HistoricalFamilyKey,
    membership: &CompletedBasisMembershipCertificate,
    m1_cell_generated: bool,
) -> Result<E2bQuotientDisposition, E2bQuotientClosureError> {
    match key {
        HistoricalFamilyKey::Formation { .. } => {
            Ok(E2bQuotientDisposition::CanonicalCompletedPackageFamily)
        }
        HistoricalFamilyKey::PostPathForward {
            role: PostPathRole::CoherenceForward,
            ..
        } => {
            let independent = membership.verdicts.iter().any(|record| {
                matches!(
                    record.subject,
                    FinalMembershipSubject::Step8LeftUnitCoherence
                ) && record.verdict == FinalMembershipVerdict::Independent
                    && record.independent_membership_issued
                    && record.separate_family_authorized
            });
            if !independent {
                return Err(E2bQuotientClosureError::Prerequisite(
                    "completed basis did not issue Independent for Step-8 left-unit".to_owned(),
                ));
            }
            Ok(E2bQuotientDisposition::IndependentR2Family)
        }
        HistoricalFamilyKey::CellAction { .. } => {
            if !m1_cell_generated {
                return Err(E2bQuotientClosureError::Prerequisite(
                    "M1 Step-8 cell-action Generated verdict is absent".to_owned(),
                ));
            }
            Ok(E2bQuotientDisposition::GeneratedR2Instance)
        }
        HistoricalFamilyKey::Path { .. } => Err(E2bQuotientClosureError::Invariant(
            "registered path family entered ordinary disposition".to_owned(),
        )),
        _ => Ok(E2bQuotientDisposition::StructuralNaturalFamily),
    }
}

fn read_u32(value: &Value, path: &[&str]) -> Result<u32, E2bQuotientClosureError> {
    let mut cursor = value;
    for segment in path {
        cursor = cursor.get(*segment).ok_or_else(|| {
            E2bQuotientClosureError::Prerequisite(format!(
                "missing frozen JSON field {}",
                path.join(".")
            ))
        })?;
    }
    cursor
        .as_u64()
        .and_then(|number| u32::try_from(number).ok())
        .ok_or_else(|| {
            E2bQuotientClosureError::Prerequisite(format!(
                "frozen JSON field {} is not u32",
                path.join(".")
            ))
        })
}

fn issue_row(
    package: &HistCertV3PackageAudit,
    gap: &OrdinaryNaturalFamilyGap,
    source: &FormedSchemaContext,
    substitution: &TypedSubstitutionToken,
    membership: &CompletedBasisMembershipCertificate,
    completed_basis_hash: &str,
    m1_cell_generated: bool,
    m1_sub_basis_digest: &str,
    issued_families: &[IssuedFamilyIndex],
    parent_operation: Option<&IssuedFamilyIndex>,
) -> Result<(OrdinaryRowResolution, Option<IssuedFamilyIndex>), E2bQuotientClosureError> {
    let schema = ordinary_schema(source.clone(), package, gap, parent_operation)?;
    let typed = issue_ordinary_typed_realizer(schema.clone())
        .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    replay_ordinary_typed_realizer(&typed)
        .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    let normalization = issue_ordinary_schema_normalization(schema.clone())
        .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    replay_ordinary_schema_normalization(&normalization)
        .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    let naturality = issue_ordinary_constructor_naturality(schema, substitution.clone())
        .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    replay_ordinary_constructor_naturality(&naturality)
        .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    if typed.count_inputs_used()
        || !normalization.typed_before_and_after()
        || !normalization.replay_stable()
        || !normalization.provenance_retained()
        || !naturality.holds()
    {
        return Err(E2bQuotientClosureError::Invariant(format!(
            "typed/normalized/natural replay failed at step {} for {:?}",
            package.step, gap.key
        )));
    }

    let kind = constructor_for(&gap.key);
    let disposition = disposition_for(&gap.key, membership, m1_cell_generated)?;
    let constructor_kind = constructor_name(kind);
    let (family_token, generated_instance_token, index) =
        if disposition == E2bQuotientDisposition::GeneratedR2Instance {
            let parent = parent_operation.ok_or_else(|| {
                E2bQuotientClosureError::Invariant(
                    "generated cell action has no issued parent operation".to_owned(),
                )
            })?;
            let mut token = GeneratedInstanceTokenProjection {
                token_version: "schema2-e2b-generated-instance-v1".to_owned(),
                step: package.step,
                key: gap.key.clone(),
                constructor_kind: constructor_kind.clone(),
                typed_realizer_derivation_hash: typed.derivation_hash().to_owned(),
                normalization_derivation_hash: normalization.derivation_hash().to_owned(),
                naturality_derivation_hash: naturality.derivation_hash().to_owned(),
                parent_family_id: parent.family_id.clone(),
                parent_family_derivation_hash: parent.derivation_hash.clone(),
                generating_sub_basis_digest: m1_sub_basis_digest.to_owned(),
                generated_subject: "step8_cell_action".to_owned(),
                monotone_generated_verdict_replayed: true,
                ordinary_family_issued: false,
                counts_as_one_quotient_family: false,
                historical_count_used_as_input: false,
                acceptance_bar_used_as_input: false,
                derivation_hash: String::new(),
            };
            token.derivation_hash = tagged_hash("generated-instance-token", &token);
            (None, Some(token), None)
        } else {
            let comparisons = distinctness_comparisons(
                package.step,
                &gap.key,
                kind,
                &package.current_subject_hash,
                issued_families,
            )?;
            let family_id = tagged_hash(
                "ordinary-quotient-family-id",
                &(
                    package.step,
                    &gap.key,
                    &package.current_subject_hash,
                    normalization.normal_form().semantic_equality_key.as_str(),
                ),
            );
            let mut token = OrdinaryFamilyTokenProjection {
                token_version: "schema2-e2b-ordinary-family-token-v1".to_owned(),
                step: package.step,
                key: gap.key.clone(),
                constructor_kind: constructor_kind.clone(),
                owner_subject_hash: package.current_subject_hash.clone(),
                source_clauses: gap.source_clauses.clone(),
                disposition,
                typed_realizer_derivation_hash: typed.derivation_hash().to_owned(),
                normalization_derivation_hash: normalization.derivation_hash().to_owned(),
                naturality_derivation_hash: naturality.derivation_hash().to_owned(),
                completed_basis_derivation_hash: completed_basis_hash.to_owned(),
                predecessor_and_same_stage_comparisons: comparisons,
                full_predecessor_sweep_proved: true,
                distinct_from_every_previously_issued_family: true,
                ordinary_family_issued: true,
                counts_as_one_quotient_family: true,
                historical_count_used_as_input: false,
                acceptance_bar_used_as_input: false,
                family_id: family_id.clone(),
                derivation_hash: String::new(),
            };
            token.derivation_hash = tagged_hash("ordinary-family-token", &token);
            let index = IssuedFamilyIndex {
                step: package.step,
                key: gap.key.clone(),
                constructor: kind,
                owner_subject_hash: package.current_subject_hash.clone(),
                family_id,
                derivation_hash: token.derivation_hash.clone(),
            };
            (Some(token), None, Some(index))
        };

    let mut row = OrdinaryRowResolution {
        step: package.step,
        key: gap.key.clone(),
        judgement: gap.judgement.clone(),
        source_clauses: gap.source_clauses.clone(),
        required_opaque_token: gap.required_opaque_token.clone(),
        constructor_kind,
        disposition,
        row_typed: true,
        row_normalized: true,
        row_natural: true,
        row_operationally_covered: true,
        family_token,
        generated_instance_token,
        exactly_one_quotient_resolution_issued: true,
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("ordinary-row-resolution", &row);
    Ok((row, index))
}

pub fn issue_e2b_quotient_closure_certificate()
-> Result<E2bQuotientClosureCertificate, E2bQuotientClosureError> {
    let membership_json = std::str::from_utf8(MEMBERSHIP_ARTIFACT_BYTES)
        .map_err(|error| E2bQuotientClosureError::Json(error.to_string()))?;
    let membership_replay = replay_completed_basis_membership_json(membership_json);
    if !membership_replay.valid
        || !membership_replay.e2b_authorized
        || membership_replay.pending_count_after != 0
    {
        return Err(E2bQuotientClosureError::Prerequisite(format!(
            "completed membership did not authorize E-2b: {:?}",
            membership_replay.errors
        )));
    }
    let membership: CompletedBasisMembershipCertificate = serde_json::from_str(membership_json)
        .map_err(|error| E2bQuotientClosureError::Json(error.to_string()))?;

    let hist_json = std::str::from_utf8(HIST_CERT_ARTIFACT_BYTES)
        .map_err(|error| E2bQuotientClosureError::Json(error.to_string()))?;
    let hist_replay = replay_hist_cert_v3_json(hist_json);
    if !hist_replay.valid
        || !hist_replay.raw_totals_7_8_10_18
        || !hist_replay.path_subtotals_2_2_5_10
        || !hist_replay.all_registered_paths_typed_and_marginal
    {
        return Err(E2bQuotientClosureError::Prerequisite(format!(
            "HIST-CERT v3 did not replay its registered surface: {:?}",
            hist_replay.errors
        )));
    }
    let hist: HistCertV3Certificate = serde_json::from_str(hist_json)
        .map_err(|error| E2bQuotientClosureError::Json(error.to_string()))?;

    let m1: Value = serde_json::from_slice(M1_ARTIFACT_BYTES)
        .map_err(|error| E2bQuotientClosureError::Json(error.to_string()))?;
    let prior_subjects = m1
        .pointer("/enlarged_sub_basis/prior_m1_generated_subjects")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            E2bQuotientClosureError::Prerequisite(
                "M1 generated-subject inventory is missing".to_owned(),
            )
        })?;
    let m1_cell_generated = prior_subjects
        .iter()
        .any(|subject| subject.as_str() == Some("step8_cell_action"));
    let m1_sub_basis_digest = m1
        .pointer("/enlarged_sub_basis/sub_basis_digest")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            E2bQuotientClosureError::Prerequisite("M1 sub-basis digest is missing".to_owned())
        })?
        .to_owned();

    let stage1_token = issue_stage1_r1_package_token()
        .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    replay_stage1_r1_package_token(&stage1_token)
        .map_err(|error| E2bQuotientClosureError::Token(error.to_string()))?;
    if membership.stage1_separate_carrier_family_authorized
        || !stage1_token.completed_package_family_issued
    {
        return Err(E2bQuotientClosureError::Invariant(
            "Stage-1 completed package/carrier verdict mismatch".to_owned(),
        ));
    }

    let source = source_context()?;
    let substitution = naturality_substitution(&source)?;
    let mut issued_families = Vec::<IssuedFamilyIndex>::new();
    let mut ordinary_rows = Vec::new();
    for package in &hist.packages {
        let mut parent_operation: Option<IssuedFamilyIndex> = None;
        for gap in &package.ordinary_family_gaps {
            let (row, issued) = issue_row(
                package,
                gap,
                &source,
                &substitution,
                &membership,
                &membership.completed_basis.derivation_hash,
                m1_cell_generated,
                &m1_sub_basis_digest,
                &issued_families,
                parent_operation.as_ref(),
            )?;
            if let Some(index) = issued {
                if index.constructor == OrdinarySchemaKind::PostPathOperation {
                    parent_operation = Some(index.clone());
                }
                issued_families.push(index);
            }
            ordinary_rows.push(row);
        }
    }

    let ordinary_rows_expected = hist
        .packages
        .iter()
        .map(|package| package.ordinary_family_gap_count)
        .sum::<u32>();
    let ordinary_rows_covered = ordinary_rows
        .iter()
        .filter(|row| row.row_operationally_covered)
        .count() as u32;
    let ordinary_family_tokens_minted = ordinary_rows
        .iter()
        .filter(|row| row.family_token.is_some())
        .count() as u32;
    let generated_instance_tokens_minted = ordinary_rows
        .iter()
        .filter(|row| row.generated_instance_token.is_some())
        .count() as u32;
    let every_ordinary_row_resolved = ordinary_rows_covered == ordinary_rows_expected
        && ordinary_rows.iter().all(|row| {
            row.exactly_one_quotient_resolution_issued
                && (row.family_token.is_some() ^ row.generated_instance_token.is_some())
        });
    if !every_ordinary_row_resolved {
        return Err(E2bQuotientClosureError::Invariant(
            "ordinary row quotient resolution is incomplete".to_owned(),
        ));
    }

    // Quotient decisions above are closed before either historical comparator
    // is read.  The following values can therefore affect only F-Q2 output.
    let stage1_diagnostic: Value = serde_json::from_slice(STAGE1_DIAGNOSTIC_BYTES)
        .map_err(|error| E2bQuotientClosureError::Json(error.to_string()))?;
    let stage1_sealed = read_u32(
        &stage1_diagnostic,
        &[
            "e2",
            "stage1_universe_diagnostic",
            "sealed_record_family_units",
        ],
    )?;
    let count_used_to_choose = stage1_diagnostic
        .pointer("/e2/stage1_universe_diagnostic/count_used_to_choose_package_boundary")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            E2bQuotientClosureError::Prerequisite(
                "Stage-1 count-use diagnostic is missing".to_owned(),
            )
        })?;
    if count_used_to_choose {
        return Err(E2bQuotientClosureError::Prerequisite(
            "Stage-1 predecessor used the count to choose its boundary".to_owned(),
        ));
    }
    let completed_package_family_count = u32::from(stage1_token.completed_package_family_issued);
    let separately_authorized_carrier_family_count =
        u32::from(membership.stage1_separate_carrier_family_authorized);
    let stage1_total = completed_package_family_count + separately_authorized_carrier_family_count;
    let mut stage1 = Stage1CountOutput {
        sealed_recorded_total: stage1_sealed,
        completed_package_family_count,
        separately_authorized_carrier_family_count,
        quotient_family_total: stage1_total,
        matches_seal: stage1_total == stage1_sealed,
        package_rule_applied_before_comparator_read: true,
        completed_membership_applied_before_comparator_read: true,
        historical_count_used_as_input: false,
        acceptance_bar_used_as_input: false,
        derivation_hash: String::new(),
    };
    stage1.derivation_hash = tagged_hash("stage1-count-output", &stage1);

    let mut historical_count_outputs = Vec::new();
    for package in &hist.packages {
        let rows = ordinary_rows
            .iter()
            .filter(|row| row.step == package.step)
            .collect::<Vec<_>>();
        let ordinary_operational_row_count = rows.len() as u32;
        let ordinary_family_token_count =
            rows.iter().filter(|row| row.family_token.is_some()).count() as u32;
        let generated_instance_row_count = rows
            .iter()
            .filter(|row| row.generated_instance_token.is_some())
            .count() as u32;
        let operational_row_coverage_total =
            package.typed_and_marginal_path_subtotal + ordinary_operational_row_count;
        let quotient_family_total =
            package.typed_and_marginal_path_subtotal + ordinary_family_token_count;
        let mut output = HistoricalStepCountOutput {
            step: package.step,
            sealed_recorded_total: package.recorded_total,
            registered_path_subtotal: package.typed_and_marginal_path_subtotal,
            ordinary_operational_row_count,
            ordinary_family_token_count,
            generated_instance_row_count,
            operational_row_coverage_total,
            quotient_family_total,
            every_ordinary_row_covered: rows.iter().all(|row| row.row_operationally_covered),
            operational_row_coverage_matches_seal: operational_row_coverage_total
                == package.recorded_total,
            quotient_family_total_matches_seal: quotient_family_total == package.recorded_total,
            count_was_output_only: true,
            derivation_hash: String::new(),
        };
        output.derivation_hash = tagged_hash("historical-count-output", &output);
        historical_count_outputs.push(output);
    }

    let mut divergences = Vec::new();
    if !stage1.matches_seal {
        let mut divergence = Fq2Divergence {
            stage: 1,
            sealed_recorded_total: stage1.sealed_recorded_total,
            quotient_family_total: stage1.quotient_family_total,
            signed_delta: stage1.quotient_family_total as i32 - stage1.sealed_recorded_total as i32,
            exact_cause: "stage1_r1_completed_package_and_carrier_exception_output".to_owned(),
            rules_retuned: false,
            history_edited: false,
            derivation_hash: String::new(),
        };
        divergence.derivation_hash = tagged_hash("fq2-divergence", &divergence);
        divergences.push(divergence);
    }
    for output in &historical_count_outputs {
        if !output.quotient_family_total_matches_seal {
            let exact_cause = if output.step == 8 && output.generated_instance_row_count == 1 {
                "step8_cell_action_is_a_generated_instance_of_the_parent_operation_and_does_not_mint_an_independent_ordinary_family"
            } else {
                "historical_operational_rows_do_not_biject_with_independent_quotient_families"
            };
            let mut divergence = Fq2Divergence {
                stage: output.step,
                sealed_recorded_total: output.sealed_recorded_total,
                quotient_family_total: output.quotient_family_total,
                signed_delta: output.quotient_family_total as i32
                    - output.sealed_recorded_total as i32,
                exact_cause: exact_cause.to_owned(),
                rules_retuned: false,
                history_edited: false,
                derivation_hash: String::new(),
            };
            divergence.derivation_hash = tagged_hash("fq2-divergence", &divergence);
            divergences.push(divergence);
        }
    }
    let fq2_triggered = !divergences.is_empty();
    let all_outputs_match_sealed_record = !fq2_triggered;
    let mut fq2 = Fq2Audit {
        falsifier: "F-Q2".to_owned(),
        comparator_read_only_after_quotient_closure: true,
        stage1_compared: true,
        historical_steps_compared: historical_count_outputs
            .iter()
            .map(|output| output.step)
            .collect(),
        all_outputs_match_sealed_record,
        divergences,
        fq2_triggered,
        certified_divergence: fq2_triggered,
        phase5b_fork_route_registered: fq2_triggered,
        phase5b_fork_executed: false,
        history_earned_without_divergence: all_outputs_match_sealed_record,
        derivation_hash: String::new(),
    };
    fq2.derivation_hash = tagged_hash("fq2-audit", &fq2);

    let ordinary_family_token_hashes = ordinary_rows
        .iter()
        .filter_map(|row| row.family_token.as_ref())
        .map(|token| token.derivation_hash.clone())
        .collect::<Vec<_>>();
    let generated_instance_token_hashes = ordinary_rows
        .iter()
        .filter_map(|row| row.generated_instance_token.as_ref())
        .map(|token| token.derivation_hash.clone())
        .collect::<Vec<_>>();
    let registered_path_family_token_hashes = hist
        .packages
        .iter()
        .flat_map(|package| package.path_families.iter())
        .map(|family| family.individual_join_derivation_hash.clone())
        .collect::<Vec<_>>();
    let mut agent_a_handoff = AgentAHandoff {
        issued: true,
        ordinary_family_token_hashes,
        generated_instance_token_hashes,
        registered_path_family_token_hashes,
        e2b_count_outputs: historical_count_outputs.clone(),
        fq2_derivation_hash: fq2.derivation_hash.clone(),
        fq2_triggered,
        agent_a_executed: false,
        f_t1_evaluated: false,
        full_historical_totals_certified_here: false,
        derivation_hash: String::new(),
    };
    agent_a_handoff.derivation_hash = tagged_hash("agent-a-handoff", &agent_a_handoff);

    let downstream = E2bDeferredSequence {
        agent_a_ft1_executed: false,
        e5_f1_executed: false,
        bridge_executed: false,
        phase5b_fork_executed: false,
        halt_or_continuation_claimed: false,
    };
    let e2b_complete = every_ordinary_row_resolved
        && ordinary_rows_expected == ordinary_rows_covered
        && ordinary_family_tokens_minted + generated_instance_tokens_minted
            == ordinary_rows_expected
        && historical_count_outputs.len() == hist.packages.len()
        && agent_a_handoff.issued
        && downstream.all_deferred();
    if !e2b_complete {
        return Err(E2bQuotientClosureError::Invariant(
            "E-2b did not reach its fail-closed terminal output".to_owned(),
        ));
    }
    let outcome = if fq2_triggered {
        "e2b_complete_fq2_certified_divergence".to_owned()
    } else {
        "e2b_complete_all_sealed_counts_earned".to_owned()
    };
    let permitted_conclusion = if fq2_triggered {
        "All ordinary rows are typed, normalized, natural, and quotient-resolved. F-Q2 reports the exact certified divergence; the adopted R1/R2 rules and sealed history remain unchanged."
    } else {
        "All ordinary rows are typed, normalized, natural, quotient-resolved, and the resulting count outputs match the sealed comparators."
    }
    .to_owned();
    let required_successor_action = if fq2_triggered {
        "Consume the issued Agent A handoff under the recorded F-Q2 divergence, then follow the registered Phase-5b route in adopted sequence."
    } else {
        "Consume the issued Agent A handoff and evaluate F-T1."
    }
    .to_owned();

    let mut certificate = E2bQuotientClosureCertificate {
        schema: E2B_QUOTIENT_CLOSURE_SCHEMA.to_owned(),
        date: E2B_QUOTIENT_CLOSURE_DATE.to_owned(),
        source_bindings: source_bindings(),
        completed_membership_schema: COMPLETED_BASIS_MEMBERSHIP_SCHEMA.to_owned(),
        completed_membership_digest: membership.result_digest.clone(),
        completed_membership_replayed: true,
        hist_cert_schema: HIST_CERT_V3_SCHEMA.to_owned(),
        hist_cert_digest: hist.result_digest.clone(),
        hist_cert_replayed: true,
        adopted_r1_r2_replayed: std::str::from_utf8(R1_R2_ADJUDICATION_BYTES)
            .is_ok_and(|text| text.contains("**R1 adopted**") && text.contains("**R2 adopted**")),
        adopted_p1_replayed: std::str::from_utf8(P1_ADJUDICATION_BYTES)
            .is_ok_and(|text| text.contains("**Adopted.**")),
        stage1,
        ordinary_rows,
        ordinary_rows_expected,
        ordinary_rows_covered,
        ordinary_family_tokens_minted,
        generated_instance_tokens_minted,
        every_typed_token_replayed: true,
        every_normalization_token_replayed: true,
        every_naturality_token_replayed: true,
        every_ordinary_row_resolved,
        historical_count_outputs,
        counts_as_outputs_regression_run: true,
        fq2,
        agent_a_handoff,
        e2b_complete,
        downstream,
        outcome,
        permitted_conclusion,
        required_successor_action,
        result_digest: String::new(),
    };
    certificate.result_digest = tagged_hash("e2b-quotient-closure-certificate", &certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> E2bQuotientClosureReplay {
    E2bQuotientClosureReplay {
        valid: false,
        e2b_complete: false,
        ordinary_rows_expected: 0,
        ordinary_rows_covered: 0,
        ordinary_family_tokens_minted: 0,
        generated_instance_tokens_minted: 0,
        stage1_output: 0,
        historical_quotient_outputs: Vec::new(),
        historical_operational_coverage_outputs: Vec::new(),
        fq2_triggered: false,
        divergence_stages: Vec::new(),
        agent_a_handoff_issued: false,
        downstream_deferred: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_e2b_quotient_closure_certificate(
    certificate: &E2bQuotientClosureCertificate,
) -> E2bQuotientClosureReplay {
    let expected = match issue_e2b_quotient_closure_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate != &expected {
        errors.push("certificate differs from count-blind create-new replay".to_owned());
    }
    let mut digest_projection = certificate.clone();
    let observed_digest = digest_projection.result_digest.clone();
    digest_projection.result_digest.clear();
    let expected_digest = tagged_hash("e2b-quotient-closure-certificate", &digest_projection);
    if observed_digest != expected_digest {
        errors.push("result digest mismatch".to_owned());
    }
    E2bQuotientClosureReplay {
        valid: errors.is_empty(),
        e2b_complete: certificate.e2b_complete,
        ordinary_rows_expected: certificate.ordinary_rows_expected,
        ordinary_rows_covered: certificate.ordinary_rows_covered,
        ordinary_family_tokens_minted: certificate.ordinary_family_tokens_minted,
        generated_instance_tokens_minted: certificate.generated_instance_tokens_minted,
        stage1_output: certificate.stage1.quotient_family_total,
        historical_quotient_outputs: certificate
            .historical_count_outputs
            .iter()
            .map(|output| output.quotient_family_total)
            .collect(),
        historical_operational_coverage_outputs: certificate
            .historical_count_outputs
            .iter()
            .map(|output| output.operational_row_coverage_total)
            .collect(),
        fq2_triggered: certificate.fq2.fq2_triggered,
        divergence_stages: certificate
            .fq2
            .divergences
            .iter()
            .map(|divergence| divergence.stage)
            .collect(),
        agent_a_handoff_issued: certificate.agent_a_handoff.issued,
        downstream_deferred: certificate.downstream.all_deferred(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_e2b_quotient_closure_json(json: &str) -> E2bQuotientClosureReplay {
    match serde_json::from_str::<E2bQuotientClosureCertificate>(json) {
        Ok(certificate) => replay_e2b_quotient_closure_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_e2b_quotient_closure_create_new(
    path: &Path,
) -> Result<E2bQuotientClosureReplay, E2bQuotientClosureError> {
    let certificate = issue_e2b_quotient_closure_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| E2bQuotientClosureError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| E2bQuotientClosureError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| E2bQuotientClosureError::Io(error.to_string()))?;
    let replay = replay_e2b_quotient_closure_certificate(&certificate);
    if !replay.valid {
        return Err(E2bQuotientClosureError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

pub fn default_e2b_artifact_path() -> PathBuf {
    workspace_doc_path("schema2_e2b_quotient_closure_v1.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_ordinary_row_receives_exactly_one_quotient_resolution() {
        let certificate = issue_e2b_quotient_closure_certificate().expect("E-2b closes");
        assert_eq!(certificate.ordinary_rows_expected, 24);
        assert_eq!(certificate.ordinary_rows_covered, 24);
        assert_eq!(certificate.ordinary_family_tokens_minted, 23);
        assert_eq!(certificate.generated_instance_tokens_minted, 1);
        assert!(certificate.every_ordinary_row_resolved);
        assert!(certificate.e2b_complete);
    }

    #[test]
    fn count_outputs_are_derived_after_verdicts_and_fq2_is_literal() {
        let certificate = issue_e2b_quotient_closure_certificate().expect("E-2b closes");
        assert_eq!(certificate.stage1.quotient_family_total, 1);
        assert_eq!(
            certificate
                .historical_count_outputs
                .iter()
                .map(|output| output.operational_row_coverage_total)
                .collect::<Vec<_>>(),
            vec![7, 8, 10, 18]
        );
        assert_eq!(
            certificate
                .historical_count_outputs
                .iter()
                .map(|output| output.quotient_family_total)
                .collect::<Vec<_>>(),
            vec![7, 8, 10, 17]
        );
        assert!(certificate.fq2.fq2_triggered);
        assert_eq!(certificate.fq2.divergences.len(), 1);
        assert_eq!(certificate.fq2.divergences[0].stage, 8);
        assert_eq!(certificate.fq2.divergences[0].signed_delta, -1);
        assert!(certificate.agent_a_handoff.issued);
        assert!(certificate.downstream.all_deferred());
    }

    #[test]
    fn forged_family_credit_or_suppressed_fq2_is_rejected() {
        let certificate = issue_e2b_quotient_closure_certificate().expect("E-2b closes");
        let generated_index = certificate
            .ordinary_rows
            .iter()
            .position(|row| row.generated_instance_token.is_some())
            .expect("one generated instance");
        let mut forged_credit = certificate.clone();
        forged_credit.ordinary_rows[generated_index]
            .generated_instance_token
            .as_mut()
            .expect("generated token")
            .counts_as_one_quotient_family = true;
        assert!(!replay_e2b_quotient_closure_certificate(&forged_credit).valid);

        let mut suppressed = certificate;
        suppressed.fq2.fq2_triggered = false;
        suppressed.fq2.certified_divergence = false;
        assert!(!replay_e2b_quotient_closure_certificate(&suppressed).valid);
    }
}
