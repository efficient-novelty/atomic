//! In-memory F-SM1 v3 sweep over the frozen chronological corpus.
//!
//! The v1/v2 issuers and their emitted artifacts are immutable inputs.  This
//! successor changes only the theorem used for membership: sealed contextual
//! Formation plus open ordered identity-tail specialization from
//! `contextual_formation_coherence_v3`.  The issuer is pure; the versioned
//! create-new wrapper may persist either a pass or an honest negative result.

use crate::contextual_formation_coherence_v3::{
    CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION, ContextualFormationCoherenceV3Error,
    FirstImageOriginV3, issue_contextual_formation_closure_v3,
    issue_contextual_type_family_closure_v3, issue_ordered_open_assignment_v3,
    kernel_context_from_parameter_sorts, replay_open_formation_specialization_v3,
    specialize_contextual_formation_v3,
};
use crate::e5_future_hole_finale_v2::{
    E5_FUTURE_HOLE_FINALE_V2_SCHEMA, E5FutureHoleFinaleV2Certificate,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3ChronologicalInterfaceMode, A3DemandOutputType, A3HistoricalWindow, A3RuleConstructor,
    A3TypedClauseSource, A3TypedDemandInstance, A3TypedDemandScheme,
    generate_a3_window_for_exact_prefix_unbounded, replay_chronological_interface_slot_map,
};
use pen_eval::typed_families::{extract_candidate_families, predecessor_closure};
use pen_type::elaborate::{KernelTy, SealedSignature};
use pen_type::substitution::{
    ParameterSort, SortedParameterContext, SubstitutionImage, issue_structural_substitution,
    replay_structural_substitution,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const CHRONOLOGICAL_SLOT_MAP_V3_SCHEMA: &str =
    "chronological-interface-slot-map-f-sm1-contextual-formation-v3";
pub const CHRONOLOGICAL_FORMATION_GAP_V3: &str = "BI_CHRONOLOGICAL_CONTEXTUAL_FORMATION_GAP_V3";
pub const CHRONOLOGICAL_OPEN_ASSIGNMENT_GAP_V3: &str = "BI_CHRONOLOGICAL_OPEN_ASSIGNMENT_GAP_V3";
pub const CHRONOLOGICAL_SPECIALIZATION_GAP_V3: &str = "BI_CHRONOLOGICAL_OPEN_SPECIALIZATION_GAP_V3";
pub const CHRONOLOGICAL_WRAPPER_GAP_V3: &str = "BI_CHRONOLOGICAL_WRAPPER_GAP_V3";
pub const CHRONOLOGICAL_SLOT_MAP_V3_CERTIFICATE_NAME: &str =
    "chronological_interface_slot_map_v3.json";
pub const CHRONOLOGICAL_SLOT_MAP_V3_REPORT_NAME: &str =
    "CHRONOLOGICAL_INTERFACE_SLOT_MAP_V3_RESULT.md";

const BI0_ARCHIVE_BYTES: &[u8] = include_bytes!("../../../docs/BI_REGRESSION_CERTIFICATE.json");
const E5_ARCHIVE_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_future_hole_finale_v2_dependent_context.json");
const V1_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/chronological_interface_slot_map_v1.json");
const V2_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/chronological_interface_slot_map_v2.json");
const FROZEN_V2_FINALE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_finale.rs");
const FROZEN_V2_SLOT_MAP_SOURCE_BYTES: &[u8] = include_bytes!("chronological_slot_map_v2.rs");
const FROZEN_V2_MOTIVE_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence_v2.rs");
const V3_THEOREM_SOURCE_BYTES: &[u8] = include_bytes!("contextual_formation_coherence_v3.rs");
const V3_WRAPPER_SOURCE_BYTES: &[u8] = include_bytes!("chronological_slot_map_v3.rs");
const FROZEN_BI0_ARCHIVE_SHA256: &str =
    "5601c530fe74eae2d5e1fecb49ec06c44d84d8f44b94e7f87384c4b973cf0fda";
const FROZEN_E5_ARCHIVE_SHA256: &str =
    "c60a28c82370e88829dfd6457ae52814d4523fcf2b2012378814a4cf0dd32be5";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case", tag = "membership")]
pub enum ChronologicalMembershipV3 {
    DeclarationOnly,
    Derived {
        theorem: String,
        source_formation_hash: String,
        assignment_hash: String,
        specialization_hash: String,
        exact_open_substitution_result_replayed: bool,
    },
    NamedGap {
        gap_id: String,
        reason: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenV2SourceBindingV3 {
    pub path: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapInstanceAuditV3 {
    pub stage: u32,
    pub instance_id: String,
    pub scheme_id: String,
    pub older_step: u32,
    pub older_clause: u16,
    pub newest_step: u32,
    pub newest_clause: u16,
    pub declared_arity: u32,
    pub declared_assignments: Vec<(u32, u32)>,
    pub declaration_hash: String,
    pub declaration_replayed: bool,
    pub zero_charge: bool,
    pub inferred_from_derivation_success: bool,
    pub instance_override_permitted: bool,
    pub independently_sealed_discharge: bool,
    pub membership: ChronologicalMembershipV3,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapWindowAuditV3 {
    pub stage: u32,
    pub prefix_signature_digest: String,
    pub instances: Vec<ChronologicalSlotMapInstanceAuditV3>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapAuditV3 {
    pub schema: String,
    pub theorem: String,
    pub v1_artifact_preserved: bool,
    pub v2_artifact_preserved: bool,
    pub frozen_v2_source_bindings: Vec<FrozenV2SourceBindingV3>,
    pub every_frozen_v2_source_bound_by_bytes: bool,
    pub v3_source_bindings: Vec<FrozenV2SourceBindingV3>,
    pub every_v3_source_bound_by_bytes: bool,
    pub issuance_is_pure_before_create_new_write: bool,
    pub windows: Vec<ChronologicalSlotMapWindowAuditV3>,
    pub live_declaration_sweep_hash_before_archive_read: String,
    pub historical_chronological_instance_count: usize,
    pub historical_declaration_replay_count: usize,
    pub historical_instance_ids_unique: bool,
    pub sealed_discharge_count: usize,
    pub sealed_discharge_derived_count: usize,
    pub sealed_discharge_named_gap_count: usize,
    pub every_sealed_exact_open_substitution_result_replayed: bool,
    pub named_gap_counts: BTreeMap<String, usize>,
    pub named_gap_reason_counts: BTreeMap<String, usize>,
    pub frozen_stage16_89_id_surface_exact: bool,
    pub formerly_gapped_expected_instance_ids: Vec<String>,
    pub formerly_gapped_derived_instance_ids: Vec<String>,
    pub formerly_gapped_named_gap_instance_ids: Vec<String>,
    pub every_declaration_order_preserving_identity: bool,
    pub every_declaration_zero_charge: bool,
    pub no_inference_or_override: bool,
    pub alternative_slot_map_permutation_trial_count: usize,
    pub f_sm1_passed: bool,
    pub bi0_prerequisite_side_reopened: bool,
    pub conclusion: String,
    pub audit_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapReplayV3 {
    pub valid: bool,
    pub f_sm1_passed: bool,
    pub sealed_discharge_derived_count: usize,
    pub sealed_discharge_named_gap_count: usize,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ChronologicalSlotMapV3Error {
    #[error("v3 chronological sweep invariant failed: {0}")]
    Invariant(String),
    #[error("v3 chronological archive failed: {0}")]
    Archive(String),
    #[error("v3 chronological JSON failed: {0}")]
    Json(String),
    #[error("v3 chronological I/O failed: {0}")]
    Io(String),
    #[error("emitted v3 chronological artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CHRONOLOGICAL_SLOT_MAP_V3_SCHEMA, domain, value))
        .expect("v3 chronological audit serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn frozen_v2_source_bindings() -> Vec<FrozenV2SourceBindingV3> {
    [
        (
            "crates/pen-search/src/branch_invariance_finale.rs",
            FROZEN_V2_FINALE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/chronological_slot_map_v2.rs",
            FROZEN_V2_SLOT_MAP_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence_v2.rs",
            FROZEN_V2_MOTIVE_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, bytes)| FrozenV2SourceBindingV3 {
        path: path.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: format!("blake3:{}", blake3_hex(bytes)),
    })
    .collect()
}

fn v3_source_bindings() -> Vec<FrozenV2SourceBindingV3> {
    [
        (
            "crates/pen-search/src/contextual_formation_coherence_v3.rs",
            V3_THEOREM_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/chronological_slot_map_v3.rs",
            V3_WRAPPER_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, bytes)| FrozenV2SourceBindingV3 {
        path: path.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: format!("blake3:{}", blake3_hex(bytes)),
    })
    .collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("archive serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn exact_prefix(full: &SealedSignature, stage: u32) -> SealedSignature {
    SealedSignature::from_telescopes(
        full.entries()
            .iter()
            .filter(|entry| entry.step < stage)
            .map(|entry| (entry.step, entry.telescope.clone()))
            .collect(),
    )
}

fn structural_context(types: &[KernelTy]) -> SortedParameterContext {
    SortedParameterContext::new(
        types
            .iter()
            .map(|ty| {
                if *ty == KernelTy::Type {
                    ParameterSort::Type
                } else {
                    ParameterSort::Opaque
                }
            })
            .collect(),
    )
}

fn chronological_sources<'a>(
    window: &'a A3HistoricalWindow,
    instance: &A3TypedDemandInstance,
) -> Result<(&'a A3TypedClauseSource, &'a A3TypedClauseSource), String> {
    if instance.source_anchor_ids.len() != 2 {
        return Err("chronological instance does not have exactly two sources".to_owned());
    }
    let older = window
        .typed_sources
        .iter()
        .find(|source| source.anchor_id == instance.source_anchor_ids[0])
        .ok_or_else(|| "older source is absent".to_owned())?;
    let newest = window
        .typed_sources
        .iter()
        .find(|source| source.anchor_id == instance.source_anchor_ids[1])
        .ok_or_else(|| "newest source is absent".to_owned())?;
    if older.step >= newest.step || !older.exported_public_clause || !newest.exported_public_clause
    {
        return Err("chronological source orientation/publicity failed".to_owned());
    }
    Ok((older, newest))
}

fn map_gap(error: ContextualFormationCoherenceV3Error) -> (String, String) {
    let gap = match error {
        ContextualFormationCoherenceV3Error::SealedFamily(_)
        | ContextualFormationCoherenceV3Error::Formation(_) => CHRONOLOGICAL_FORMATION_GAP_V3,
        ContextualFormationCoherenceV3Error::Assignment(_) => CHRONOLOGICAL_OPEN_ASSIGNMENT_GAP_V3,
        ContextualFormationCoherenceV3Error::Specialization(_)
        | ContextualFormationCoherenceV3Error::ReplayMismatch => {
            CHRONOLOGICAL_SPECIALIZATION_GAP_V3
        }
    };
    (gap.to_owned(), error.to_string())
}

fn prove_one(
    signature: &SealedSignature,
    visible_library: u32,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
) -> Result<ChronologicalMembershipV3, (String, String)> {
    let (older, newest) = chronological_sources(window, instance)
        .map_err(|reason| (CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(), reason))?;
    let (older_family, older_type, newest_family, newest_type, mode, slot_map) =
        match &scheme.required_output {
            A3DemandOutputType::ChronologicalInteraction {
                older_family,
                older_type,
                newest_family,
                newest_type,
                interface_mode,
                interface_slot_map,
            } => (
                older_family,
                older_type,
                newest_family,
                newest_type,
                interface_mode,
                interface_slot_map,
            ),
            _ => {
                return Err((
                    CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(),
                    "chronological scheme has a non-chronological output".to_owned(),
                ));
            }
        };
    if older_family != &older.canonical_family_key
        || older_type != &older.kernel_type
        || newest_family != &newest.canonical_family_key
        || newest_type != &newest.kernel_type
        || instance.source_family_keys
            != vec![
                older.canonical_family_key.clone(),
                newest.canonical_family_key.clone(),
            ]
    {
        return Err((
            CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(),
            "chronological scheme/source exact join failed".to_owned(),
        ));
    }
    let expected_mode = match &older.kernel_type {
        KernelTy::Type => A3ChronologicalInterfaceMode::DirectType,
        KernelTy::Fun(domain, codomain) if codomain.as_ref() == &KernelTy::Type => {
            A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction {
                domain: domain.as_ref().clone(),
            }
        }
        _ => {
            return Err((
                CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(),
                "older source is not Type-valued".to_owned(),
            ));
        }
    };
    if *mode != expected_mode {
        return Err((
            CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(),
            "chronological interface mode drifted".to_owned(),
        ));
    }
    let source_arity = newest.canonical_presentation.parameters.len() as u32;
    replay_chronological_interface_slot_map(slot_map, source_arity).map_err(|error| {
        (
            CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(),
            format!("slot-map replay failed: {error}"),
        )
    })?;
    if source_arity == 0 {
        return Err((
            CHRONOLOGICAL_OPEN_ASSIGNMENT_GAP_V3.to_owned(),
            "newest source has no parameter for the older interface".to_owned(),
        ));
    }
    let source_context =
        kernel_context_from_parameter_sorts(&newest.canonical_presentation.parameters);
    let target_arity = (older.canonical_presentation.parameters.len() as u32).max(source_arity);
    let target_parameters = (0..target_arity as usize)
        .map(|index| {
            older
                .canonical_presentation
                .parameters
                .get(index)
                .or_else(|| newest.canonical_presentation.parameters.get(index))
                .cloned()
                .ok_or_else(|| "target parameter inventory has a hole".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|reason| (CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(), reason))?;
    let target_context = kernel_context_from_parameter_sorts(&target_parameters);
    let first_origin = match mode {
        A3ChronologicalInterfaceMode::DirectType => FirstImageOriginV3::DirectSealedTypeFamily {
            source: older.clone(),
        },
        A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. } => {
            FirstImageOriginV3::PointwiseSealedTypeValuedFamily {
                source: older.clone(),
                argument_parameter: 1,
            }
        }
    };
    let formation = if newest.kernel_role == ClauseRole::Formation {
        issue_contextual_formation_closure_v3(signature, visible_library, newest)
    } else {
        issue_contextual_type_family_closure_v3(signature, visible_library, newest)
    }
    .map_err(map_gap)?;
    let assignment = issue_ordered_open_assignment_v3(
        signature,
        visible_library,
        source_context,
        target_context.clone(),
        first_origin,
        Vec::new(),
    )
    .map_err(map_gap)?;
    let specialization =
        specialize_contextual_formation_v3(signature, &formation, &assignment).map_err(map_gap)?;
    replay_open_formation_specialization_v3(signature, specialization.projection())
        .map_err(map_gap)?;

    let independent_images = assignment
        .projection()
        .images
        .iter()
        .map(|image| SubstitutionImage {
            source_parameter: image.source_parameter,
            term: image.term.clone(),
        })
        .collect::<Vec<_>>();
    let independent_substitution = issue_structural_substitution(
        structural_context(&assignment.projection().source_context),
        structural_context(&target_context),
        independent_images,
        newest.canonical_presentation.canonical_normal_form.clone(),
    )
    .map_err(|error| {
        (
            CHRONOLOGICAL_SPECIALIZATION_GAP_V3.to_owned(),
            error.to_string(),
        )
    })?;
    replay_structural_substitution(&independent_substitution).map_err(|error| {
        (
            CHRONOLOGICAL_SPECIALIZATION_GAP_V3.to_owned(),
            error.to_string(),
        )
    })?;
    let exact_open_substitution_result_replayed = independent_substitution.result()
        == &specialization.projection().substitution_result
        && independent_substitution.derivation_hash()
            == specialization.projection().substitution_derivation_hash
        && specialization.projection().specialized_relation.expression
            == specialization.projection().substitution_result
        && specialization
            .projection()
            .specialized_relation
            .internal_closure_issued;
    if !exact_open_substitution_result_replayed {
        return Err((
            CHRONOLOGICAL_SPECIALIZATION_GAP_V3.to_owned(),
            "independent exact substitution differs from the specialized relation".to_owned(),
        ));
    }
    let output_telescope = Telescope::new(vec![ClauseRec::new(
        newest.kernel_role,
        specialization.projection().substitution_result.clone(),
    )]);
    let closure = predecessor_closure(signature)
        .map_err(|error| (CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(), error.to_string()))?;
    let extraction =
        extract_candidate_families(signature, &closure, &output_telescope, visible_library);
    let extracted = extraction.extraction().ok_or_else(|| {
        (
            CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(),
            "specialized output failed live family extraction".to_owned(),
        )
    })?;
    if extracted.families.len() != 1 || !extracted.families[0].naturality.square.equal {
        return Err((
            CHRONOLOGICAL_WRAPPER_GAP_V3.to_owned(),
            "specialized output natural-family square did not replay".to_owned(),
        ));
    }
    Ok(ChronologicalMembershipV3::Derived {
        theorem: CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION.to_owned(),
        source_formation_hash: formation.projection().derivation_hash.clone(),
        assignment_hash: assignment.projection().assignment_hash.clone(),
        specialization_hash: specialization.projection().derivation_hash.clone(),
        exact_open_substitution_result_replayed,
    })
}

fn archive_ids(value: &Value, field: &str) -> Result<Vec<String>, ChronologicalSlotMapV3Error> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| ChronologicalSlotMapV3Error::Archive(format!("field {field} absent")))?
        .iter()
        .map(|row| {
            row.as_str().map(str::to_owned).ok_or_else(|| {
                ChronologicalSlotMapV3Error::Archive(format!("field {field} is non-string"))
            })
        })
        .collect()
}

pub fn issue_chronological_slot_map_audit_v3()
-> Result<ChronologicalSlotMapAuditV3, ChronologicalSlotMapV3Error> {
    let full = SealedSignature::genesis_del_h15();
    let mut generated = Vec::new();
    let mut windows = Vec::new();
    let mut all_ids = BTreeSet::new();
    for stage in 1..=16 {
        let prefix = exact_prefix(&full, stage);
        let window = generate_a3_window_for_exact_prefix_unbounded(&prefix, stage)
            .map_err(|error| ChronologicalSlotMapV3Error::Invariant(error.to_string()))?;
        let mut rows = Vec::new();
        for instance in &window.instances {
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == instance.scheme_id)
                .ok_or_else(|| {
                    ChronologicalSlotMapV3Error::Invariant("orphan A3 instance".to_owned())
                })?;
            if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
                continue;
            }
            let A3DemandOutputType::ChronologicalInteraction {
                interface_slot_map, ..
            } = &scheme.required_output
            else {
                return Err(ChronologicalSlotMapV3Error::Invariant(
                    "chronological scheme has non-chronological output".to_owned(),
                ));
            };
            replay_chronological_interface_slot_map(
                interface_slot_map,
                interface_slot_map.declared_arity,
            )
            .map_err(|error| ChronologicalSlotMapV3Error::Invariant(error.to_string()))?;
            let (older, newest) = chronological_sources(&window, instance)
                .map_err(ChronologicalSlotMapV3Error::Invariant)?;
            all_ids.insert(instance.instance_id.clone());
            let mut row = ChronologicalSlotMapInstanceAuditV3 {
                stage,
                instance_id: instance.instance_id.clone(),
                scheme_id: scheme.scheme_id.clone(),
                older_step: older.step,
                older_clause: older.clause_index,
                newest_step: newest.step,
                newest_clause: newest.clause_index,
                declared_arity: interface_slot_map.declared_arity,
                declared_assignments: interface_slot_map
                    .assignments
                    .iter()
                    .map(|assignment| (assignment.interface_slot, assignment.parameter))
                    .collect(),
                declaration_hash: interface_slot_map.declaration_hash.clone(),
                declaration_replayed: true,
                zero_charge: interface_slot_map.kappa_charge == 0
                    && interface_slot_map.nu_charge == 0,
                inferred_from_derivation_success: interface_slot_map
                    .inferred_from_derivation_success,
                instance_override_permitted: interface_slot_map.instance_override_permitted,
                independently_sealed_discharge: false,
                membership: ChronologicalMembershipV3::DeclarationOnly,
                derivation_hash: String::new(),
            };
            row.derivation_hash = tagged_hash("instance", &row);
            rows.push(row);
        }
        let mut audit = ChronologicalSlotMapWindowAuditV3 {
            stage,
            prefix_signature_digest: prefix.digest().to_owned(),
            instances: rows,
            derivation_hash: String::new(),
        };
        audit.derivation_hash = tagged_hash("window", &audit);
        windows.push(audit);
        generated.push((prefix, window));
    }
    let live_declaration_sweep_hash_before_archive_read =
        tagged_hash("live-declaration-sweep", &windows);

    // Historical artifacts are comparator inputs only and are opened after
    // the complete live declaration surface has been fixed.
    let e5: E5FutureHoleFinaleV2Certificate = serde_json::from_slice(E5_ARCHIVE_BYTES)
        .map_err(|error| ChronologicalSlotMapV3Error::Archive(error.to_string()))?;
    let mut e5_projection = e5.clone();
    let e5_observed = e5_projection.result_digest.clone();
    e5_projection.result_digest.clear();
    let e5_valid = sha256_hex(E5_ARCHIVE_BYTES) == FROZEN_E5_ARCHIVE_SHA256
        && e5_observed
            == external_tagged_hash(
                E5_FUTURE_HOLE_FINALE_V2_SCHEMA,
                "certificate",
                &e5_projection,
            );
    let sealed_ids = e5
        .stage16
        .membership_rows
        .iter()
        .filter(|row| row.rule.starts_with("chronological_comparison::"))
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    if !e5_valid || sealed_ids.len() != 72 {
        return Err(ChronologicalSlotMapV3Error::Archive(
            "frozen E-5 chronological corpus failed validation".to_owned(),
        ));
    }
    for (prefix, window) in &generated {
        for instance in &window.instances {
            if !sealed_ids.contains(&instance.instance_id) {
                continue;
            }
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == instance.scheme_id)
                .ok_or_else(|| {
                    ChronologicalSlotMapV3Error::Invariant("sealed orphan instance".to_owned())
                })?;
            let membership = match prove_one(
                prefix,
                window.stage.saturating_sub(1),
                window,
                scheme,
                instance,
            ) {
                Ok(membership) => membership,
                Err((gap_id, reason)) => ChronologicalMembershipV3::NamedGap { gap_id, reason },
            };
            let row = windows
                .iter_mut()
                .find(|row| row.stage == window.stage)
                .and_then(|row| {
                    row.instances
                        .iter_mut()
                        .find(|row| row.instance_id == instance.instance_id)
                })
                .ok_or_else(|| {
                    ChronologicalSlotMapV3Error::Invariant(
                        "sealed row absent from declaration sweep".to_owned(),
                    )
                })?;
            row.independently_sealed_discharge = true;
            row.membership = membership;
            row.derivation_hash.clear();
            row.derivation_hash = tagged_hash("instance", row);
        }
    }
    for window in &mut windows {
        window.derivation_hash.clear();
        window.derivation_hash = tagged_hash("window", window);
    }
    let sealed_rows = windows
        .iter()
        .flat_map(|window| &window.instances)
        .filter(|row| row.independently_sealed_discharge)
        .collect::<Vec<_>>();
    let sealed_discharge_count = sealed_rows.len();
    let sealed_discharge_derived_count = sealed_rows
        .iter()
        .filter(|row| matches!(row.membership, ChronologicalMembershipV3::Derived { .. }))
        .count();
    let sealed_discharge_named_gap_count = sealed_rows
        .iter()
        .filter(|row| matches!(row.membership, ChronologicalMembershipV3::NamedGap { .. }))
        .count();
    let every_sealed_exact_open_substitution_result_replayed = sealed_rows.iter().all(|row| {
        matches!(
            row.membership,
            ChronologicalMembershipV3::Derived {
                exact_open_substitution_result_replayed: true,
                ..
            }
        )
    });
    let mut named_gap_counts = BTreeMap::new();
    let mut named_gap_reason_counts = BTreeMap::new();
    for row in &sealed_rows {
        if let ChronologicalMembershipV3::NamedGap { gap_id, reason } = &row.membership {
            *named_gap_counts.entry(gap_id.clone()).or_insert(0) += 1;
            *named_gap_reason_counts.entry(reason.clone()).or_insert(0) += 1;
        }
    }
    drop(sealed_rows);

    let frozen_stage16_ids = e5
        .stage16
        .membership_rows
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let live_stage16_ids = generated[15]
        .1
        .instances
        .iter()
        .map(|row| row.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let frozen_stage16_89_id_surface_exact =
        frozen_stage16_ids.len() == 89 && frozen_stage16_ids == live_stage16_ids;
    if sha256_hex(BI0_ARCHIVE_BYTES) != FROZEN_BI0_ARCHIVE_SHA256 {
        return Err(ChronologicalSlotMapV3Error::Archive(
            "frozen BI-0 comparator digest drifted".to_owned(),
        ));
    }
    let bi0: Value = serde_json::from_slice(BI0_ARCHIVE_BYTES)
        .map_err(|error| ChronologicalSlotMapV3Error::Archive(error.to_string()))?;
    let mut formerly_gapped_expected_instance_ids =
        archive_ids(&bi0, "enacted_finale_issuer_gap_instance_ids")?;
    formerly_gapped_expected_instance_ids.sort();
    let former_set = formerly_gapped_expected_instance_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let stage16_rows = &windows[15].instances;
    let mut formerly_gapped_derived_instance_ids = stage16_rows
        .iter()
        .filter(|row| former_set.contains(&row.instance_id))
        .filter(|row| matches!(row.membership, ChronologicalMembershipV3::Derived { .. }))
        .map(|row| row.instance_id.clone())
        .collect::<Vec<_>>();
    let mut formerly_gapped_named_gap_instance_ids = stage16_rows
        .iter()
        .filter(|row| former_set.contains(&row.instance_id))
        .filter(|row| matches!(row.membership, ChronologicalMembershipV3::NamedGap { .. }))
        .map(|row| row.instance_id.clone())
        .collect::<Vec<_>>();
    formerly_gapped_derived_instance_ids.sort();
    formerly_gapped_named_gap_instance_ids.sort();

    let historical_chronological_instance_count =
        windows.iter().map(|window| window.instances.len()).sum();
    let historical_declaration_replay_count = windows
        .iter()
        .flat_map(|window| &window.instances)
        .filter(|row| row.declaration_replayed)
        .count();
    let historical_instance_ids_unique = all_ids.len() == historical_chronological_instance_count;
    let every_declaration_order_preserving_identity = windows.iter().all(|window| {
        window.instances.iter().all(|row| {
            row.declared_assignments
                .iter()
                .enumerate()
                .all(|(index, &(slot, parameter))| slot == index as u32 + 1 && parameter == slot)
        })
    });
    let every_declaration_zero_charge = windows
        .iter()
        .flat_map(|window| &window.instances)
        .all(|row| row.zero_charge);
    let no_inference_or_override = windows
        .iter()
        .flat_map(|window| &window.instances)
        .all(|row| !row.inferred_from_derivation_success && !row.instance_override_permitted);
    let f_sm1_passed = every_declaration_order_preserving_identity
        && every_declaration_zero_charge
        && no_inference_or_override
        && historical_declaration_replay_count == historical_chronological_instance_count
        && historical_instance_ids_unique
        && frozen_stage16_89_id_surface_exact
        && sealed_discharge_count == 72
        && sealed_discharge_derived_count == 72
        && sealed_discharge_named_gap_count == 0
        && every_sealed_exact_open_substitution_result_replayed
        && formerly_gapped_expected_instance_ids.len() == 9
        && formerly_gapped_derived_instance_ids == formerly_gapped_expected_instance_ids
        && formerly_gapped_named_gap_instance_ids.is_empty();
    let frozen_v2_source_bindings = frozen_v2_source_bindings();
    let every_frozen_v2_source_bound_by_bytes = frozen_v2_source_bindings.len() == 3
        && frozen_v2_source_bindings
            .iter()
            .all(|binding| binding.byte_length > 0 && binding.blake3.starts_with("blake3:"));
    let v3_source_bindings = v3_source_bindings();
    let every_v3_source_bound_by_bytes = v3_source_bindings.len() == 2
        && v3_source_bindings
            .iter()
            .all(|binding| binding.byte_length > 0 && binding.blake3.starts_with("blake3:"));
    let mut audit = ChronologicalSlotMapAuditV3 {
        schema: CHRONOLOGICAL_SLOT_MAP_V3_SCHEMA.to_owned(),
        theorem: CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION.to_owned(),
        v1_artifact_preserved: !V1_ARTIFACT_BYTES.is_empty(),
        v2_artifact_preserved: !V2_ARTIFACT_BYTES.is_empty(),
        frozen_v2_source_bindings,
        every_frozen_v2_source_bound_by_bytes,
        v3_source_bindings,
        every_v3_source_bound_by_bytes,
        issuance_is_pure_before_create_new_write: true,
        windows,
        live_declaration_sweep_hash_before_archive_read,
        historical_chronological_instance_count,
        historical_declaration_replay_count,
        historical_instance_ids_unique,
        sealed_discharge_count,
        sealed_discharge_derived_count,
        sealed_discharge_named_gap_count,
        every_sealed_exact_open_substitution_result_replayed,
        named_gap_counts,
        named_gap_reason_counts,
        frozen_stage16_89_id_surface_exact,
        formerly_gapped_expected_instance_ids,
        formerly_gapped_derived_instance_ids,
        formerly_gapped_named_gap_instance_ids,
        every_declaration_order_preserving_identity,
        every_declaration_zero_charge,
        no_inference_or_override,
        alternative_slot_map_permutation_trial_count: 0,
        f_sm1_passed: f_sm1_passed
            && every_frozen_v2_source_bound_by_bytes
            && every_v3_source_bound_by_bytes,
        bi0_prerequisite_side_reopened: f_sm1_passed
            && every_frozen_v2_source_bound_by_bytes
            && every_v3_source_bound_by_bytes,
        conclusion: if f_sm1_passed {
            "F-SM1 v3 passes: every sealed chronological discharge, including the former nine, replays by genuine Internal premises and exact open identity-tail specialization. This prerequisite alone authorizes no BI-0, BI-1, or BI-4 artifact."
        } else {
            "F-SM1 v3 remains false: named theorem or wrapper gaps survive. This prerequisite authorizes no BI-0, BI-1, or BI-4 artifact, and no alternate slot permutation was tried."
        }
        .to_owned(),
        audit_hash: String::new(),
    };
    audit.audit_hash = tagged_hash("audit", &audit);
    Ok(audit)
}

pub fn replay_chronological_slot_map_audit_v3(
    audit: &ChronologicalSlotMapAuditV3,
) -> ChronologicalSlotMapReplayV3 {
    let mut errors = Vec::new();
    let mut digest_projection = audit.clone();
    digest_projection.audit_hash.clear();
    if audit.audit_hash != tagged_hash("audit", &digest_projection) {
        errors.push("audit hash mismatch".to_owned());
    }
    match issue_chronological_slot_map_audit_v3() {
        Ok(expected) if expected == *audit => {}
        Ok(_) => errors.push("audit differs from independent v3 reissuance".to_owned()),
        Err(error) => errors.push(format!("independent v3 reissuance failed: {error}")),
    }
    ChronologicalSlotMapReplayV3 {
        valid: errors.is_empty(),
        f_sm1_passed: audit.f_sm1_passed,
        sealed_discharge_derived_count: audit.sealed_discharge_derived_count,
        sealed_discharge_named_gap_count: audit.sealed_discharge_named_gap_count,
        errors,
    }
}

pub fn replay_chronological_slot_map_v3_json(text: &str) -> ChronologicalSlotMapReplayV3 {
    match serde_json::from_str::<ChronologicalSlotMapAuditV3>(text) {
        Ok(audit) => replay_chronological_slot_map_audit_v3(&audit),
        Err(error) => ChronologicalSlotMapReplayV3 {
            valid: false,
            f_sm1_passed: false,
            sealed_discharge_derived_count: 0,
            sealed_discharge_named_gap_count: 0,
            errors: vec![format!("JSON parse failed: {error}")],
        },
    }
}

pub fn replay_chronological_slot_map_v3_directory(
    directory: &Path,
) -> ChronologicalSlotMapReplayV3 {
    let json_path = directory.join(CHRONOLOGICAL_SLOT_MAP_V3_CERTIFICATE_NAME);
    let report_path = directory.join(CHRONOLOGICAL_SLOT_MAP_V3_REPORT_NAME);
    let text = match read_to_string(&json_path) {
        Ok(text) => text,
        Err(error) => {
            return ChronologicalSlotMapReplayV3 {
                valid: false,
                f_sm1_passed: false,
                sealed_discharge_derived_count: 0,
                sealed_discharge_named_gap_count: 0,
                errors: vec![format!("certificate read failed: {error}")],
            };
        }
    };
    let mut replay = replay_chronological_slot_map_v3_json(&text);
    let audit = match serde_json::from_str::<ChronologicalSlotMapAuditV3>(&text) {
        Ok(audit) => audit,
        Err(_) => return replay,
    };
    match read_to_string(&report_path) {
        Ok(report) if report == render_chronological_slot_map_audit_v3(&audit) => {}
        Ok(_) => replay
            .errors
            .push("result report differs from replay render".to_owned()),
        Err(error) => replay
            .errors
            .push(format!("result report read failed: {error}")),
    }
    replay.valid = replay.errors.is_empty();
    replay
}

pub fn emit_chronological_slot_map_v3_create_new(
    directory: &Path,
) -> Result<ChronologicalSlotMapReplayV3, ChronologicalSlotMapV3Error> {
    let json_path = directory.join(CHRONOLOGICAL_SLOT_MAP_V3_CERTIFICATE_NAME);
    let report_path = directory.join(CHRONOLOGICAL_SLOT_MAP_V3_REPORT_NAME);
    if json_path.exists() || report_path.exists() {
        return Err(ChronologicalSlotMapV3Error::Io(
            "create-new target already exists".to_owned(),
        ));
    }
    let audit = issue_chronological_slot_map_audit_v3()?;
    let replay = replay_chronological_slot_map_audit_v3(&audit);
    if !replay.valid {
        return Err(ChronologicalSlotMapV3Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let json = serde_json::to_vec_pretty(&audit)
        .map_err(|error| ChronologicalSlotMapV3Error::Json(error.to_string()))?;
    let report = render_chronological_slot_map_audit_v3(&audit);
    let mut json_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&json_path)
        .map_err(|error| ChronologicalSlotMapV3Error::Io(error.to_string()))?;
    json_file
        .write_all(&json)
        .map_err(|error| ChronologicalSlotMapV3Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| ChronologicalSlotMapV3Error::Io(error.to_string()))?;
    report_file
        .write_all(report.as_bytes())
        .map_err(|error| ChronologicalSlotMapV3Error::Io(error.to_string()))?;
    let directory_replay = replay_chronological_slot_map_v3_directory(directory);
    if !directory_replay.valid {
        return Err(ChronologicalSlotMapV3Error::EmittedReplay(
            directory_replay.errors.join("; "),
        ));
    }
    Ok(directory_replay)
}

pub fn render_chronological_slot_map_audit_v3(audit: &ChronologicalSlotMapAuditV3) -> String {
    format!(
        "# Chronological interface slot-map F-SM1 v3 result\n\n- Digest: `{}`\n- Frozen v2 sources byte-bound: `{}`\n- V3 theorem/wrapper sources byte-bound: `{}`\n- Declaration replay: `{}/{}`\n- Sealed derived / named gap: `{}` / `{}`\n- Every sealed exact open substitution replayed: `{}`\n- Named gaps: `{:?}`\n- Exact reason classes: `{:?}`\n- Former nine derived / named gap: `{}` / `{}`\n- Alternative permutations trialed: `0`\n- F-SM1: `{}`\n- BI-0 prerequisite side reopened: `{}`\n\n{}\n",
        audit.audit_hash,
        audit.every_frozen_v2_source_bound_by_bytes,
        audit.every_v3_source_bound_by_bytes,
        audit.historical_declaration_replay_count,
        audit.historical_chronological_instance_count,
        audit.sealed_discharge_derived_count,
        audit.sealed_discharge_named_gap_count,
        audit.every_sealed_exact_open_substitution_result_replayed,
        audit.named_gap_counts,
        audit.named_gap_reason_counts,
        audit.formerly_gapped_derived_instance_ids.len(),
        audit.formerly_gapped_named_gap_instance_ids.len(),
        audit.f_sm1_passed,
        audit.bi0_prerequisite_side_reopened,
        audit.conclusion,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v3_audit_is_replayable_and_never_trials_a_permutation() {
        let audit = issue_chronological_slot_map_audit_v3().expect("v3 audit issues");
        let replay = replay_chronological_slot_map_audit_v3(&audit);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(audit.sealed_discharge_count, 72);
        assert_eq!(
            audit.sealed_discharge_derived_count + audit.sealed_discharge_named_gap_count,
            72
        );
        assert_eq!(audit.sealed_discharge_derived_count, 0);
        assert_eq!(audit.sealed_discharge_named_gap_count, 72);
        assert!(!audit.every_sealed_exact_open_substitution_result_replayed);
        assert_eq!(
            audit.named_gap_counts.get(CHRONOLOGICAL_FORMATION_GAP_V3),
            Some(&54)
        );
        assert_eq!(
            audit
                .named_gap_counts
                .get(CHRONOLOGICAL_SPECIALIZATION_GAP_V3),
            Some(&18)
        );
        assert_eq!(audit.formerly_gapped_named_gap_instance_ids.len(), 9);
        assert_eq!(audit.alternative_slot_map_permutation_trial_count, 0);
        assert!(audit.issuance_is_pure_before_create_new_write);
        assert!(!audit.f_sm1_passed);
        assert!(!audit.bi0_prerequisite_side_reopened);
    }

    #[test]
    fn rehashed_semantic_mutation_is_rejected() {
        let mut audit = issue_chronological_slot_map_audit_v3().expect("v3 audit issues");
        audit.conclusion.push_str(" forged");
        audit.audit_hash.clear();
        audit.audit_hash = tagged_hash("audit", &audit);
        assert!(!replay_chronological_slot_map_audit_v3(&audit).valid);
    }

    #[test]
    fn json_replay_rejects_unknown_and_rehashed_semantic_mutations() {
        let audit = issue_chronological_slot_map_audit_v3().expect("v3 audit issues");
        let json = serde_json::to_string_pretty(&audit).expect("serialize audit");
        assert!(replay_chronological_slot_map_v3_json(&json).valid);

        let mut unknown: Value = serde_json::from_str(&json).expect("JSON value");
        unknown
            .as_object_mut()
            .expect("audit object")
            .insert("forged_field".to_owned(), Value::Bool(true));
        assert!(!replay_chronological_slot_map_v3_json(&unknown.to_string()).valid);

        let mut forged = audit;
        forged.f_sm1_passed = true;
        forged.bi0_prerequisite_side_reopened = true;
        forged.audit_hash.clear();
        forged.audit_hash = tagged_hash("audit", &forged);
        let forged_json = serde_json::to_string(&forged).expect("forged JSON");
        assert!(!replay_chronological_slot_map_v3_json(&forged_json).valid);
    }
}
