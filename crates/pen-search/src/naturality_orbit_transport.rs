//! Shared naturality/orbit transport for the A3 and Stage-4 stops.
//!
//! This module is deliberately a quotient-and-transport module, not a
//! selector.  It never receives a desired count, a bar, a historical winner,
//! or an enumeration rank.  The A3 adapter proves concrete specializations
//! of registered Step-15 families; the Stage-4 adapter compares complete
//! typed packages under the frozen equality and exact parameter sorts.

use crate::e5_demand_projection::{
    E5_DEMAND_PROJECTION_SCHEMA, E5DemandInstance, E5DemandProjectionCertificate,
    E5MembershipDisposition, replay_e5_demand_projection_json,
};
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::t_bf1_prefix::{issue_t_bf1_prefix_certificate, replay_t_bf1_prefix_certificate};
use pen_core::canonical::canonical_key_telescope;
use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3DemandSchemeOrigin, A3HistoricalWindow, A3RuleConstructor, issue_historical_a3_demand_grammar,
};
use pen_eval::typed_families::clause_presentation;
use pen_schema::grammar_completion::{
    GRAMMAR_COMPLETION_SCHEMA, GrammarCompletionCertificate, RegisteredTraceSignature,
    replay_grammar_completion_json,
};
use pen_type::admissibility::{
    AdmissibilityMode, passes_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::elaborate::{SealedSignature, candidate_hash, elaborate_telescope};
use pen_type::equality::{EqualityWitness, KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use pen_type::substitution::{
    SortedParameterContext, SubstitutionImage, issue_structural_substitution,
    replay_structural_substitution,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const NATURALITY_ORBIT_TRANSPORT_SCHEMA: &str = "naturality-orbit-transport-v1";
pub const NATURALITY_ORBIT_TRANSPORT_DATE: &str = "2026-07-21";
pub const A3_UNARY_ACTION_OUTPUT_GAP: &str = "A3_UNARY_ACTION_OUTPUT_UNDEFINED";
pub const A3_STRUCTURAL_COMPLETION_OUTPUT_GAP: &str =
    "A3_STRUCTURAL_COMPLETION_TYPED_HOLE_UNDEFINED";

const PROTOCOL_BYTES: &[u8] = include_bytes!("../../../docs/tie_resolution_protocol.md");
const GRAMMAR_BYTES: &[u8] = include_bytes!("../../../docs/schema2_grammar_completion_v1.json");
const J3_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e5_demand_projection_v1.json");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const FAMILY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/typed_families.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const SUBSTITUTION_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/substitution.rs");
const ENUM_SOURCE_BYTES: &[u8] = include_bytes!("enumerate.rs");
const T_BF1_SOURCE_BYTES: &[u8] = include_bytes!("t_bf1_prefix.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransportSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenEqualityRecord {
    pub scope_len: u32,
    pub left_normal_form: Expr,
    pub right_normal_form: Expr,
    pub left_steps: u32,
    pub right_steps: u32,
    pub equal: bool,
}

impl From<EqualityWitness> for FrozenEqualityRecord {
    fn from(witness: EqualityWitness) -> Self {
        Self {
            scope_len: witness.scope_len,
            left_normal_form: witness.left_normal_form,
            right_normal_form: witness.right_normal_form,
            left_steps: witness.left_steps,
            right_steps: witness.right_steps,
            equal: witness.equal,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4PackageClause {
    pub clause_index: u16,
    pub kernel_role: String,
    pub kernel_type_json: String,
    pub canonical_normal_form: Expr,
    pub parameter_sorts: Vec<String>,
    pub presentation_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4SemanticPackage {
    pub candidate_hash: String,
    pub canonical_key: String,
    pub telescope: Telescope,
    pub clauses: Vec<Stage4PackageClause>,
    pub semantic_key: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4ClauseComparison {
    pub clause_index: u16,
    pub roles_equal: bool,
    pub kernel_types_equal: bool,
    pub parameter_sorts_equal: bool,
    pub frozen_equality: FrozenEqualityRecord,
    pub semantically_equal: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4PackageComparison {
    pub left_candidate_hash: String,
    pub right_candidate_hash: String,
    pub clauses: Vec<Stage4ClauseComparison>,
    pub packages_equal: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4OrbitClass {
    pub class_id: String,
    pub member_candidate_hashes: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage4OrbitAudit {
    pub cone_recomputed: bool,
    pub parsimony_certificate_replayed: bool,
    pub parsimony_minimizer_hashes_joined: bool,
    pub minimum_kappa: u16,
    pub minimum_certified_nu: u32,
    pub minimizer_count: usize,
    pub packages: Vec<Stage4SemanticPackage>,
    pub pairwise_comparisons: Vec<Stage4PackageComparison>,
    pub orbit_classes: Vec<Stage4OrbitClass>,
    pub semantic_class_count: usize,
    pub all_four_presentations_quotient_to_one: bool,
    pub r_t1_dissolves_tie: bool,
    pub r_t2_required: bool,
    pub no_selector_or_presentation_order_used: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3TransportedInstance {
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub mode: String,
    pub older_step: u32,
    pub older_clause: u16,
    pub newest_step: u32,
    pub newest_clause: u16,
    pub family_id: String,
    pub orbit_id: String,
    pub generic_family: Expr,
    pub typed_image: Expr,
    pub required_output: Expr,
    pub required_output_normal_form: Expr,
    pub required_output_kernel_type_json: String,
    pub structural_substitution_hash: String,
    pub typed_image_naturality_hash: String,
    pub normalization_equality: FrozenEqualityRecord,
    pub normalization_naturality_hash: String,
    pub d_membership_derivation_hash: String,
    pub exact_predecessor_instance_join: bool,
    pub uniform_specialization_not_new_family: bool,
    pub independently_exported_output_orbit: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3PendingOutput {
    pub stage: u32,
    pub clause_index: Option<u16>,
    pub a3_instance_id: Option<String>,
    pub a3_scheme_id: Option<String>,
    pub rule: String,
    pub source_registration_name: Option<String>,
    pub source_naturality_hash: Option<String>,
    pub gap_id: String,
    pub exact_reason: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3OrbitTransportAudit {
    pub grammar_archive_digest_valid: bool,
    pub grammar_live_replay_valid: bool,
    pub grammar_live_replay_errors: Vec<String>,
    pub j3_predecessor_archive_digest_valid: bool,
    pub j3_predecessor_live_replay_valid: bool,
    pub j3_predecessor_live_replay_errors: Vec<String>,
    pub frozen_surface_drift_bound_explicitly: bool,
    pub stage16_metadata_instance_count: usize,
    pub stage16_inventory_partition_exact: bool,
    pub chronological_instance_transport_bijection: bool,
    pub direct_instance_count: usize,
    pub direct_natural_family_count: usize,
    pub direct_instances: Vec<A3TransportedInstance>,
    pub j3_regression_64_to_8: bool,
    pub pointwise_instance_count: usize,
    pub pointwise_natural_family_count: usize,
    pub pointwise_instances: Vec<A3TransportedInstance>,
    pub pointwise_instances_join_existing_j3_families: bool,
    pub chronological_outputs_constructed_and_kernel_typed: bool,
    pub unary_seed_count: usize,
    pub unary_registration_join_count: usize,
    pub unary_pending_outputs: Vec<A3PendingOutput>,
    pub structural_completion_pending_outputs: Vec<A3PendingOutput>,
    pub remaining_non_j3_seeds_never_defaulted_to_singletons: bool,
    pub every_base_orbit_export_flag_derived_from_members: bool,
    pub full_a3_output_grammar_complete: bool,
    pub f1_executable: bool,
    pub semantic_o16_decided: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NaturalityOrbitTransportCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<TransportSourceBinding>,
    pub protocol_adoption_replayed: bool,
    pub equality_procedure: String,
    pub desired_count_used_as_definition_input: bool,
    pub bar_or_historical_winner_used: bool,
    pub enumeration_or_hash_order_used_as_selector: bool,
    pub a3: A3OrbitTransportAudit,
    pub stage4: Stage4OrbitAudit,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub bridge_authorized: bool,
    pub bar_free_adoption_authorized: bool,
    pub halt_claim_issued: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NaturalityOrbitTransportReplay {
    pub valid: bool,
    pub direct_regression_64_to_8: bool,
    pub pointwise_joined_to_eight: bool,
    pub unary_pending_count: usize,
    pub stage4_class_count: usize,
    pub r_t2_required: bool,
    pub f1_executable: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum NaturalityOrbitTransportError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("typing/transport failed: {0}")]
    Transport(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted replay failed: {0}")]
    EmittedReplay(String),
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(NATURALITY_ORBIT_TRANSPORT_SCHEMA, domain, value))
        .expect("transport evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value))
        .expect("external certificate evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn grammar_archive_digest_valid(certificate: &GrammarCompletionCertificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            GRAMMAR_COMPLETION_SCHEMA,
            "grammar-completion-certificate",
            &projection,
        )
}

fn j3_archive_digest_valid(certificate: &E5DemandProjectionCertificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            E5_DEMAND_PROJECTION_SCHEMA,
            "e5-demand-projection-certificate",
            &projection,
        )
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn source_bindings() -> Vec<TransportSourceBinding> {
    [
        (
            "docs/tie_resolution_protocol.md",
            "adopted_ladder_and_falsifiers",
            PROTOCOL_BYTES,
        ),
        (
            "docs/schema2_grammar_completion_v1.json",
            "registered_typed_natural_families",
            GRAMMAR_BYTES,
        ),
        (
            "docs/schema2_e5_demand_projection_v1.json",
            "certified_j3_64_to_8_regression_and_d_membership",
            J3_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "count_blind_full_window_seed_generator",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/typed_families.rs",
            "canonical_typed_family_presentations",
            FAMILY_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/equality.rs",
            "frozen_univalent_equality",
            EQUALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/substitution.rs",
            "capture_safe_structural_transport",
            SUBSTITUTION_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "live_stage4_cone_reconstruction",
            ENUM_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_bf1_prefix.rs",
            "live_stage4_parsimony_minimizer_certificate",
            T_BF1_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "shared_two_adapter_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| TransportSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn protocol_replayed() -> Result<(), NaturalityOrbitTransportError> {
    let protocol = std::str::from_utf8(PROTOCOL_BYTES)
        .map_err(|error| NaturalityOrbitTransportError::Prerequisite(error.to_string()))?;
    for clause in [
        "R-T1",
        "Quotient first",
        "R-T2",
        "Confluence second",
        "Enumeration order is not an admissible option",
        "I adopt the tie-resolution protocol",
    ] {
        if !protocol.contains(clause) {
            return Err(NaturalityOrbitTransportError::Prerequisite(format!(
                "tie protocol no longer contains {clause:?}"
            )));
        }
    }
    Ok(())
}

fn stage4_cone() -> Vec<Telescope> {
    let winners = (1..=3_u32)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    let mut library: Library = Vec::new();
    for (_, telescope) in &winners {
        library.push(LibraryEntry::from_telescope(telescope, &library));
    }
    let admissibility = strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::Guarded);
    let context = EnumerationContext::from_admissibility(&library, admissibility);
    let mut raw = Vec::new();
    for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        raw.extend(enumerate_telescopes(&library, context, kappa));
    }
    let mut seen = BTreeSet::new();
    raw.into_iter()
        .filter(|telescope| passes_strict_admissibility(4, &library, telescope, admissibility))
        .filter(|telescope| seen.insert(canonical_key_telescope(telescope).0))
        .collect()
}

fn stage4_package(
    signature: &SealedSignature,
    telescope: Telescope,
) -> Result<Stage4SemanticPackage, NaturalityOrbitTransportError> {
    let elaboration = elaborate_telescope(signature, &telescope, 3)
        .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
    let roles = elaboration
        .clauses
        .iter()
        .map(|clause| clause.kernel_role)
        .collect::<Vec<_>>();
    let clauses = elaboration
        .clauses
        .iter()
        .map(|clause| {
            let scope = elaboration.ambient_parameters + u32::from(clause.clause_index);
            let presentation = clause_presentation(
                &clause.normal_form,
                scope,
                &roles[..usize::from(clause.clause_index)],
                elaboration.ambient_parameters,
            );
            let parameter_sorts = presentation
                .parameters
                .iter()
                .map(|sort| format!("{sort:?}"))
                .collect::<Vec<_>>();
            let presentation_derivation_hash = tagged_hash(
                "stage4-clause-presentation",
                &(
                    clause.clause_index,
                    clause.kernel_role,
                    &clause.kernel_ty,
                    &presentation.canonical_normal_form,
                    &parameter_sorts,
                    &presentation.renaming,
                ),
            );
            Ok(Stage4PackageClause {
                clause_index: clause.clause_index,
                kernel_role: format!("{:?}", clause.kernel_role),
                kernel_type_json: serde_json::to_string(&clause.kernel_ty)
                    .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?,
                canonical_normal_form: presentation.canonical_normal_form,
                parameter_sorts,
                presentation_derivation_hash,
            })
        })
        .collect::<Result<Vec<_>, NaturalityOrbitTransportError>>()?;
    let semantic_key = tagged_hash(
        "stage4-semantic-package-key",
        &clauses
            .iter()
            .map(|clause| {
                (
                    &clause.kernel_role,
                    &clause.kernel_type_json,
                    &clause.canonical_normal_form,
                    &clause.parameter_sorts,
                )
            })
            .collect::<Vec<_>>(),
    );
    let hash = candidate_hash(&telescope);
    let key = canonical_key_telescope(&telescope).0;
    let derivation_hash = tagged_hash(
        "stage4-semantic-package",
        &(
            &hash,
            &key,
            &telescope,
            &clauses,
            &semantic_key,
            &elaboration.derivation_hash,
        ),
    );
    Ok(Stage4SemanticPackage {
        candidate_hash: hash,
        canonical_key: key,
        telescope,
        clauses,
        semantic_key,
        derivation_hash,
    })
}

fn compare_stage4_packages(
    left: &Stage4SemanticPackage,
    right: &Stage4SemanticPackage,
) -> Result<Stage4PackageComparison, NaturalityOrbitTransportError> {
    if left.clauses.len() != right.clauses.len() {
        return Err(NaturalityOrbitTransportError::Invariant(
            "Stage-4 package arity mismatch".to_owned(),
        ));
    }
    let clauses = left
        .clauses
        .iter()
        .zip(&right.clauses)
        .map(|(left_clause, right_clause)| {
            let scope_len =
                u32::try_from(left_clause.parameter_sorts.len()).expect("parameter arity fits u32");
            let equality = univalent_equality(
                &left_clause.canonical_normal_form,
                &right_clause.canonical_normal_form,
                scope_len,
                512,
            )
            .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
            let roles_equal = left_clause.kernel_role == right_clause.kernel_role;
            let kernel_types_equal = left_clause.kernel_type_json == right_clause.kernel_type_json;
            let parameter_sorts_equal = left_clause.parameter_sorts == right_clause.parameter_sorts;
            let semantically_equal =
                roles_equal && kernel_types_equal && parameter_sorts_equal && equality.equal;
            Ok(Stage4ClauseComparison {
                clause_index: left_clause.clause_index,
                roles_equal,
                kernel_types_equal,
                parameter_sorts_equal,
                frozen_equality: equality.into(),
                semantically_equal,
            })
        })
        .collect::<Result<Vec<_>, NaturalityOrbitTransportError>>()?;
    let packages_equal = clauses.iter().all(|clause| clause.semantically_equal);
    let derivation_hash = tagged_hash(
        "stage4-package-comparison",
        &(
            &left.candidate_hash,
            &right.candidate_hash,
            &clauses,
            packages_equal,
        ),
    );
    Ok(Stage4PackageComparison {
        left_candidate_hash: left.candidate_hash.clone(),
        right_candidate_hash: right.candidate_hash.clone(),
        clauses,
        packages_equal,
        derivation_hash,
    })
}

fn form_stage4_orbit_classes(
    packages: &[Stage4SemanticPackage],
    pairwise_comparisons: &[Stage4PackageComparison],
) -> Result<Vec<Stage4OrbitClass>, NaturalityOrbitTransportError> {
    // Form the quotient from the independently replayed pairwise semantic
    // judgments. `semantic_key` is audit data, never an equality proof.
    let mut parent = (0..packages.len()).collect::<Vec<_>>();
    fn root(parent: &mut [usize], mut node: usize) -> usize {
        while parent[node] != node {
            parent[node] = parent[parent[node]];
            node = parent[node];
        }
        node
    }
    for comparison in pairwise_comparisons {
        if !comparison.packages_equal {
            continue;
        }
        let left = packages
            .iter()
            .position(|package| package.candidate_hash == comparison.left_candidate_hash)
            .ok_or_else(|| {
                NaturalityOrbitTransportError::Invariant(
                    "Stage-4 comparison left member is absent from package batch".to_owned(),
                )
            })?;
        let right = packages
            .iter()
            .position(|package| package.candidate_hash == comparison.right_candidate_hash)
            .ok_or_else(|| {
                NaturalityOrbitTransportError::Invariant(
                    "Stage-4 comparison right member is absent from package batch".to_owned(),
                )
            })?;
        let left_root = root(&mut parent, left);
        let right_root = root(&mut parent, right);
        if left_root != right_root {
            parent[right_root] = left_root;
        }
    }
    let mut by_component = BTreeMap::<usize, Vec<String>>::new();
    for (index, package) in packages.iter().enumerate() {
        let component = root(&mut parent, index);
        by_component
            .entry(component)
            .or_default()
            .push(package.candidate_hash.clone());
    }
    let mut comparison_evidence_hashes = pairwise_comparisons
        .iter()
        .map(|comparison| comparison.derivation_hash.clone())
        .collect::<Vec<_>>();
    comparison_evidence_hashes.sort();
    let mut orbit_classes = by_component
        .into_values()
        .map(|mut member_candidate_hashes| {
            member_candidate_hashes.sort();
            let class_id = tagged_hash("stage4-orbit-class-id", &member_candidate_hashes);
            let derivation_hash = tagged_hash(
                "stage4-orbit-class",
                &(
                    &class_id,
                    &member_candidate_hashes,
                    &comparison_evidence_hashes,
                ),
            );
            Stage4OrbitClass {
                class_id,
                member_candidate_hashes,
                derivation_hash,
            }
        })
        .collect::<Vec<_>>();
    orbit_classes.sort_by(|left, right| left.class_id.cmp(&right.class_id));
    Ok(orbit_classes)
}

fn stage4_audit() -> Result<Stage4OrbitAudit, NaturalityOrbitTransportError> {
    let parsimony = issue_t_bf1_prefix_certificate().map_err(|error| {
        NaturalityOrbitTransportError::Prerequisite(format!(
            "live T-BF1 parsimony certificate failed: {error}"
        ))
    })?;
    let parsimony_replay = replay_t_bf1_prefix_certificate(&parsimony);
    if !parsimony_replay.valid
        || parsimony_replay.stopped_stage != Some(4)
        || parsimony_replay.minimizer_count_at_stop != Some(4)
    {
        return Err(NaturalityOrbitTransportError::Prerequisite(format!(
            "live T-BF1 Stage-4 minimizer replay failed: {:?}",
            parsimony_replay.errors
        )));
    }
    let parsimony_stage4 = parsimony
        .stages
        .iter()
        .find(|stage| stage.stage == 4)
        .ok_or_else(|| {
            NaturalityOrbitTransportError::Prerequisite(
                "live T-BF1 certificate omits Stage 4".to_owned(),
            )
        })?;
    let minimum_pair = parsimony_stage4.minimum_pair.as_ref().ok_or_else(|| {
        NaturalityOrbitTransportError::Prerequisite(
            "live T-BF1 Stage 4 omits its minimum parsimony pair".to_owned(),
        )
    })?;
    let signature = SealedSignature::from_telescopes(
        (1..=3_u32)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect(),
    );
    let mut packages = stage4_cone()
        .into_iter()
        .map(|telescope| stage4_package(&signature, telescope))
        .collect::<Result<Vec<_>, _>>()?;
    packages.sort_by(|left, right| left.candidate_hash.cmp(&right.candidate_hash));
    let live_package_hashes = packages
        .iter()
        .map(|package| package.candidate_hash.clone())
        .collect::<Vec<_>>();
    let mut parsimony_minimizer_hashes = parsimony_stage4.minimizer_hashes.clone();
    parsimony_minimizer_hashes.sort();
    let parsimony_minimizer_hashes_joined = live_package_hashes == parsimony_minimizer_hashes;
    if !parsimony_minimizer_hashes_joined
        || minimum_pair.kappa != 3
        || minimum_pair.certified_nu != 5
    {
        return Err(NaturalityOrbitTransportError::Invariant(format!(
            "Stage-4 quotient input is not the live (kappa,nu)=(3,5) parsimony minimizer cone: hashes_joined={parsimony_minimizer_hashes_joined}, pair=({},{})",
            minimum_pair.kappa, minimum_pair.certified_nu
        )));
    }
    let mut pairwise_comparisons = Vec::new();
    for left in 0..packages.len() {
        for right in (left + 1)..packages.len() {
            pairwise_comparisons.push(compare_stage4_packages(&packages[left], &packages[right])?);
        }
    }
    let orbit_classes = form_stage4_orbit_classes(&packages, &pairwise_comparisons)?;
    let semantic_class_count = orbit_classes.len();
    if packages.len() != 4 || pairwise_comparisons.len() != 6 {
        return Err(NaturalityOrbitTransportError::Invariant(format!(
            "Stage-4 minimizer quotient drifted: {} packages and {} pairwise comparisons",
            packages.len(),
            pairwise_comparisons.len()
        )));
    }
    let all_four_presentations_quotient_to_one = packages.len() == 4 && semantic_class_count == 1;
    let r_t1_dissolves_tie = all_four_presentations_quotient_to_one;
    let r_t2_required = packages.len() == 4 && semantic_class_count > 1;
    let no_selector_or_presentation_order_used = true;
    let derivation_hash = tagged_hash(
        "stage4-orbit-audit",
        &(
            &parsimony.result_digest,
            parsimony_minimizer_hashes_joined,
            minimum_pair,
            &packages,
            &pairwise_comparisons,
            &orbit_classes,
            semantic_class_count,
            all_four_presentations_quotient_to_one,
            r_t1_dissolves_tie,
            r_t2_required,
            no_selector_or_presentation_order_used,
        ),
    );
    Ok(Stage4OrbitAudit {
        cone_recomputed: true,
        parsimony_certificate_replayed: true,
        parsimony_minimizer_hashes_joined,
        minimum_kappa: minimum_pair.kappa,
        minimum_certified_nu: minimum_pair.certified_nu,
        minimizer_count: packages.len(),
        packages,
        pairwise_comparisons,
        orbit_classes,
        semantic_class_count,
        all_four_presentations_quotient_to_one,
        r_t1_dissolves_tie,
        r_t2_required,
        no_selector_or_presentation_order_used,
        derivation_hash,
    })
}

/// Issue only the branch-independent R-T1 Stage-4 orbit audit.
///
/// This narrow surface deliberately avoids constructing the unrelated
/// historical Stage-16/J3 transport carried by the aggregate certificate, so
/// a non-enacted branch continuation depends on no enacted suffix evidence.
pub fn issue_stage4_r_t1_orbit_audit() -> Result<Stage4OrbitAudit, NaturalityOrbitTransportError> {
    protocol_replayed()?;
    stage4_audit()
}

pub fn replay_stage4_r_t1_orbit_audit(audit: &Stage4OrbitAudit) -> Vec<String> {
    match issue_stage4_r_t1_orbit_audit() {
        Ok(expected) if &expected == audit => Vec::new(),
        Ok(_) => vec!["Stage-4 R-T1 audit differs from independent live reissuance".to_owned()],
        Err(error) => vec![error.to_string()],
    }
}

fn registration<'a>(
    grammar: &'a GrammarCompletionCertificate,
    step: u32,
    clause: u16,
) -> Result<&'a RegisteredTraceSignature, NaturalityOrbitTransportError> {
    grammar
        .registrations
        .iter()
        .find(|row| row.step == step && row.clause_index == clause)
        .ok_or_else(|| {
            NaturalityOrbitTransportError::Prerequisite(format!(
                "grammar registration missing Step {step} clause {clause}"
            ))
        })
}

fn predecessor_instance<'a>(
    predecessor: &'a E5DemandProjectionCertificate,
    older_clause: u16,
    newest_clause: u16,
) -> Result<&'a E5DemandInstance, NaturalityOrbitTransportError> {
    predecessor
        .restricted_window
        .instances
        .iter()
        .find(|instance| {
            instance.interface_step == 14
                && instance.interface_clause == older_clause
                && instance.scheme_clause == newest_clause
        })
        .ok_or_else(|| {
            NaturalityOrbitTransportError::Prerequisite(format!(
                "restricted J3 instance missing ({older_clause},{newest_clause})"
            ))
        })
}

fn predecessor_family(
    predecessor: &E5DemandProjectionCertificate,
    newest_clause: u16,
) -> Result<(String, String), NaturalityOrbitTransportError> {
    let orbit = predecessor
        .restricted_window
        .orbits
        .iter()
        .find(|orbit| orbit.scheme_clause == newest_clause)
        .ok_or_else(|| {
            NaturalityOrbitTransportError::Prerequisite(format!(
                "restricted J3 orbit missing scheme clause {newest_clause}"
            ))
        })?;
    Ok((orbit.family_id.clone(), orbit.orbit_id.clone()))
}

fn construct_transport(
    signature: &SealedSignature,
    predecessor: &E5DemandProjectionCertificate,
    generic: &RegisteredTraceSignature,
    a3_instance_id: &str,
    a3_scheme_id: &str,
    older_clause: u16,
    image: Expr,
    image_normal_form: Expr,
    typed_image_naturality_hash: &str,
    uniform_specialization_not_new_family: bool,
    independently_exported_output_orbit: bool,
    mode: &str,
) -> Result<A3TransportedInstance, NaturalityOrbitTransportError> {
    let substitution = issue_structural_substitution(
        SortedParameterContext::all_type(1),
        SortedParameterContext::all_type(1),
        vec![SubstitutionImage {
            source_parameter: 1,
            term: image.clone(),
        }],
        generic.raw_expr.clone(),
    )
    .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
    replay_structural_substitution(&substitution)
        .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
    let output = substitution.result().clone();
    let telescope = Telescope::new(vec![ClauseRec::new(generic.declared_role, output.clone())]);
    let elaboration = elaborate_telescope(signature, &telescope, 15)
        .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
    let clause = &elaboration.clauses[0];
    let normalized_substitution = issue_structural_substitution(
        SortedParameterContext::all_type(1),
        SortedParameterContext::all_type(1),
        vec![SubstitutionImage {
            source_parameter: 1,
            term: image_normal_form,
        }],
        generic.normal_form.clone(),
    )
    .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
    replay_structural_substitution(&normalized_substitution)
        .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
    let equality = univalent_equality(
        &clause.normal_form,
        normalized_substitution.result(),
        elaboration.ambient_parameters,
        4096,
    )
    .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
    if !equality.equal {
        return Err(NaturalityOrbitTransportError::Invariant(format!(
            "normalization transport failed for Step-15 clause {}",
            generic.clause_index
        )));
    }
    let (family_id, orbit_id) = predecessor_family(predecessor, generic.clause_index)?;
    let (d_membership_derivation_hash, exact_predecessor_instance_join) = if mode == "direct_type" {
        let archived = predecessor_instance(predecessor, older_clause, generic.clause_index)?;
        if archived.generic_scheme != generic.raw_expr
            || archived.typed_image != image
            || archived.required_output != output
            || archived.required_output_normal_form.as_ref() != Some(&clause.normal_form)
            || archived.required_output_kernel_type_json.as_deref()
                != Some(
                    serde_json::to_string(&clause.kernel_ty)
                        .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?
                        .as_str(),
                )
            || archived.family_id != family_id
            || archived.orbit_id != orbit_id
            || archived.natural_family_not_new_uniform_family
                != uniform_specialization_not_new_family
            || archived.independently_exported_output_orbit != independently_exported_output_orbit
        {
            return Err(NaturalityOrbitTransportError::Invariant(format!(
                "direct transport differs from archived instance ({older_clause},{})",
                generic.clause_index
            )));
        }
        let hash = match &archived.disposition {
            E5MembershipDisposition::Derivable {
                d_membership_derivation_hash,
                ..
            } => d_membership_derivation_hash.clone(),
            E5MembershipDisposition::Underdetermined { exact_reason } => {
                return Err(NaturalityOrbitTransportError::Prerequisite(format!(
                    "restricted predecessor became underdetermined: {exact_reason}"
                )));
            }
        };
        (hash, true)
    } else {
        (
            tagged_hash(
                "typed-kernel-scheme-specialization-in-d-b15-pointwise",
                &(
                    &family_id,
                    &orbit_id,
                    substitution.derivation_hash(),
                    &elaboration.derivation_hash,
                    &equality,
                    &generic.normalization_naturality_hash,
                    typed_image_naturality_hash,
                    "D(B15) is closed under replayed typed substitution",
                ),
            ),
            false,
        )
    };
    let required_output_kernel_type_json = serde_json::to_string(&clause.kernel_ty)
        .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?;
    let derivation_hash = tagged_hash(
        "a3-transported-instance",
        &(
            (
                a3_instance_id,
                a3_scheme_id,
                mode,
                older_clause,
                generic.clause_index,
                &family_id,
                &orbit_id,
                &generic.raw_expr,
                &image,
            ),
            (
                &output,
                &clause.normal_form,
                &required_output_kernel_type_json,
                substitution.derivation_hash(),
                &equality,
                &generic.normalization_naturality_hash,
                typed_image_naturality_hash,
                &d_membership_derivation_hash,
                exact_predecessor_instance_join,
                uniform_specialization_not_new_family,
                independently_exported_output_orbit,
            ),
        ),
    );
    Ok(A3TransportedInstance {
        a3_instance_id: a3_instance_id.to_owned(),
        a3_scheme_id: a3_scheme_id.to_owned(),
        mode: mode.to_owned(),
        older_step: 14,
        older_clause,
        newest_step: 15,
        newest_clause: generic.clause_index,
        family_id,
        orbit_id,
        generic_family: generic.raw_expr.clone(),
        typed_image: image,
        required_output: output,
        required_output_normal_form: clause.normal_form.clone(),
        required_output_kernel_type_json,
        structural_substitution_hash: substitution.derivation_hash().to_owned(),
        typed_image_naturality_hash: typed_image_naturality_hash.to_owned(),
        normalization_equality: equality.into(),
        normalization_naturality_hash: generic.normalization_naturality_hash.clone(),
        d_membership_derivation_hash,
        exact_predecessor_instance_join,
        uniform_specialization_not_new_family,
        independently_exported_output_orbit,
        derivation_hash,
    })
}

fn source_coordinates(
    window: &A3HistoricalWindow,
    anchor: &str,
) -> Result<(u32, u16), NaturalityOrbitTransportError> {
    window
        .typed_sources
        .iter()
        .find(|source| source.anchor_id == anchor)
        .map(|source| (source.step, source.clause_index))
        .ok_or_else(|| {
            NaturalityOrbitTransportError::Invariant(format!("A3 source anchor {anchor} is absent"))
        })
}

fn pending(
    stage: u32,
    clause_index: Option<u16>,
    a3_instance_id: Option<&str>,
    a3_scheme_id: Option<&str>,
    rule: &str,
    registration: Option<&RegisteredTraceSignature>,
    gap_id: &str,
    exact_reason: &str,
) -> A3PendingOutput {
    let source_registration_name = registration.map(|row| row.registered_name.clone());
    let source_naturality_hash = registration.map(|row| row.normalization_naturality_hash.clone());
    let a3_instance_id = a3_instance_id.map(str::to_owned);
    let a3_scheme_id = a3_scheme_id.map(str::to_owned);
    let derivation_hash = tagged_hash(
        "a3-pending-output",
        &(
            stage,
            clause_index,
            &a3_instance_id,
            &a3_scheme_id,
            rule,
            &source_registration_name,
            &source_naturality_hash,
            gap_id,
            exact_reason,
        ),
    );
    A3PendingOutput {
        stage,
        clause_index,
        a3_instance_id,
        a3_scheme_id,
        rule: rule.to_owned(),
        source_registration_name,
        source_naturality_hash,
        gap_id: gap_id.to_owned(),
        exact_reason: exact_reason.to_owned(),
        derivation_hash,
    }
}

pub fn prove_a3_orbit_transport_for_signature(
    signature: &SealedSignature,
) -> Result<A3OrbitTransportAudit, NaturalityOrbitTransportError> {
    let grammar_text = std::str::from_utf8(GRAMMAR_BYTES)
        .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?;
    let grammar_replay = replay_grammar_completion_json(grammar_text);
    let predecessor_text = std::str::from_utf8(J3_BYTES)
        .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?;
    let predecessor_replay = replay_e5_demand_projection_json(predecessor_text);
    let grammar_archive: GrammarCompletionCertificate = serde_json::from_slice(GRAMMAR_BYTES)
        .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?;
    let predecessor: E5DemandProjectionCertificate = serde_json::from_slice(J3_BYTES)
        .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?;
    let grammar_archive_digest_valid = grammar_archive_digest_valid(&grammar_archive);
    let j3_predecessor_archive_digest_valid = j3_archive_digest_valid(&predecessor);
    if !grammar_archive_digest_valid
        || !j3_predecessor_archive_digest_valid
        || !grammar_archive.step15_j3_exact
        || !grammar_archive.full_adopted_grammar_normalization_naturality_complete
        || predecessor.restricted_window.raw_instance_count != 64
        || predecessor.restricted_window.orbit_count != 8
        || !predecessor.restricted_window.every_instance_typed
        || !predecessor.restricted_window.every_orbit_derivable
    {
        return Err(NaturalityOrbitTransportError::Prerequisite(
            "grammar or restricted J3 theorem is not the certified 64-to-8 surface".to_owned(),
        ));
    }
    // The adopted E-4 result is explicitly relative to its frozen wrapped
    // surface.  Live drift is evidence recorded by this successor, not a
    // license to rewrite or silently reject the byte-stable predecessor.
    let grammar_live_replay_valid = grammar_replay.valid;
    let grammar_live_replay_errors = grammar_replay.errors;
    let j3_predecessor_live_replay_valid = predecessor_replay.valid;
    let j3_predecessor_live_replay_errors = predecessor_replay.errors;
    let frozen_surface_drift_bound_explicitly =
        !grammar_live_replay_valid || !j3_predecessor_live_replay_valid;
    let generated = issue_historical_a3_demand_grammar(signature)
        .map_err(|error| NaturalityOrbitTransportError::Transport(error.to_string()))?;
    let stage16 = generated
        .windows
        .iter()
        .find(|window| window.stage == 16)
        .ok_or_else(|| NaturalityOrbitTransportError::Invariant("A3 omits Stage 16".to_owned()))?;

    type ChronologicalCoordinate = (String, String, u32, u16, u32, u16);
    let mut direct_coordinates = Vec::<ChronologicalCoordinate>::new();
    let mut pointwise_coordinates = Vec::<ChronologicalCoordinate>::new();
    let mut unary_instance_ids = BTreeSet::new();
    let mut higher_instance_ids = BTreeSet::new();
    let mut structural_instance_ids = BTreeSet::new();
    let mut all_instance_ids = BTreeSet::new();
    for instance in &stage16.instances {
        if !all_instance_ids.insert(instance.instance_id.clone()) {
            return Err(NaturalityOrbitTransportError::Invariant(
                "A3 Stage-16 instance id is duplicated".to_owned(),
            ));
        }
        let scheme = stage16
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
            .ok_or_else(|| {
                NaturalityOrbitTransportError::Invariant("A3 orphan instance".to_owned())
            })?;
        match scheme.rule_constructor {
            A3RuleConstructor::UnaryAction => {
                if !matches!(
                    scheme.required_output,
                    pen_eval::a3_demand_grammar::A3DemandOutputType::ActionAt { .. }
                ) || instance.source_anchor_ids.len() != 1
                {
                    return Err(NaturalityOrbitTransportError::Invariant(
                        "A3 unary instance/output arity drifted".to_owned(),
                    ));
                }
                unary_instance_ids.insert(instance.instance_id.clone());
            }
            A3RuleConstructor::ChronologicalComparison => {
                if instance.source_anchor_ids.len() != 2 {
                    return Err(NaturalityOrbitTransportError::Invariant(
                        "chronological instance does not have two sources".to_owned(),
                    ));
                }
                let older = source_coordinates(stage16, &instance.source_anchor_ids[0])?;
                let newest = source_coordinates(stage16, &instance.source_anchor_ids[1])?;
                if older.0 != 14 || newest.0 != 15 {
                    return Err(NaturalityOrbitTransportError::Invariant(format!(
                        "chronological orientation drifted: older Step {}, newest Step {}",
                        older.0, newest.0
                    )));
                }
                let coordinate = (
                    instance.instance_id.clone(),
                    scheme.scheme_id.clone(),
                    older.0,
                    older.1,
                    newest.0,
                    newest.1,
                );
                match &scheme.required_output {
                    pen_eval::a3_demand_grammar::A3DemandOutputType::ChronologicalInteraction {
                        interface_mode:
                            pen_eval::a3_demand_grammar::A3ChronologicalInterfaceMode::DirectType,
                        ..
                    } => direct_coordinates.push(coordinate),
                    pen_eval::a3_demand_grammar::A3DemandOutputType::ChronologicalInteraction {
                        interface_mode:
                            pen_eval::a3_demand_grammar::A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. },
                        ..
                    } => pointwise_coordinates.push(coordinate),
                    _ => {
                        return Err(NaturalityOrbitTransportError::Invariant(
                            "chronological constructor has a non-chronological output".to_owned(),
                        ));
                    }
                }
            }
            A3RuleConstructor::HigherOpenBoxReduction => {
                if !matches!(
                    scheme.required_output,
                    pen_eval::a3_demand_grammar::A3DemandOutputType::ContractibleOpenBox { .. }
                ) {
                    return Err(NaturalityOrbitTransportError::Invariant(
                        "higher constructor has a non-open-box output".to_owned(),
                    ));
                }
                higher_instance_ids.insert(instance.instance_id.clone());
            }
            A3RuleConstructor::StructuralCompletionHole => {
                if !matches!(
                    scheme.required_output,
                    pen_eval::a3_demand_grammar::A3DemandOutputType::StructuralCompletion { .. }
                ) {
                    return Err(NaturalityOrbitTransportError::Invariant(
                        "completion constructor has a non-completion output".to_owned(),
                    ));
                }
                structural_instance_ids.insert(instance.instance_id.clone());
            }
        }
    }

    let stage16_inventory_partition_exact = all_instance_ids.len() == stage16.instances.len()
        && unary_instance_ids.len() == 17
        && direct_coordinates.len() == 64
        && pointwise_coordinates.len() == 8
        && higher_instance_ids.is_empty()
        && structural_instance_ids.is_empty()
        && unary_instance_ids.len() + direct_coordinates.len() + pointwise_coordinates.len()
            == stage16.instances.len();
    if !stage16_inventory_partition_exact {
        return Err(NaturalityOrbitTransportError::Invariant(format!(
            "Stage-16 A3 partition drifted: unary={}, direct={}, pointwise={}, higher={}, structural={}, total={}",
            unary_instance_ids.len(),
            direct_coordinates.len(),
            pointwise_coordinates.len(),
            higher_instance_ids.len(),
            structural_instance_ids.len(),
            stage16.instances.len()
        )));
    }

    let mut direct_instances = Vec::new();
    for (instance_id, scheme_id, older_step, older_clause, newest_step, newest_clause) in
        direct_coordinates
    {
        let a3_instance = stage16
            .instances
            .iter()
            .find(|instance| instance.instance_id == instance_id)
            .expect("direct coordinate came from Stage-16 A3 instance batch");
        let older = registration(&grammar_archive, older_step, older_clause)?;
        let generic = registration(&grammar_archive, newest_step, newest_clause)?;
        if older.kernel_type_json != "\"Type\"" {
            return Err(NaturalityOrbitTransportError::Invariant(
                "direct coordinate is not Type-valued".to_owned(),
            ));
        }
        direct_instances.push(construct_transport(
            signature,
            &predecessor,
            generic,
            &instance_id,
            &scheme_id,
            older_clause,
            older.raw_expr.clone(),
            older.normal_form.clone(),
            &older.normalization_naturality_hash,
            a3_instance.identity_or_uniform_specialization,
            a3_instance.independently_exported_demand,
            "direct_type",
        )?);
    }
    let direct_families = direct_instances
        .iter()
        .map(|instance| instance.family_id.clone())
        .collect::<BTreeSet<_>>();
    let j3_regression_64_to_8 = direct_instances.len() == 64 && direct_families.len() == 8;

    let pointwise_source = registration(&grammar_archive, 14, 8)?;
    let pointwise_image = Expr::App(
        Box::new(pointwise_source.raw_expr.clone()),
        Box::new(Expr::Var(1)),
    );
    let pointwise_image_nf = Expr::App(
        Box::new(pointwise_source.normal_form.clone()),
        Box::new(Expr::Var(1)),
    );
    let mut pointwise_instances = Vec::new();
    for (instance_id, scheme_id, older_step, older_clause, newest_step, newest_clause) in
        pointwise_coordinates
    {
        let a3_instance = stage16
            .instances
            .iter()
            .find(|instance| instance.instance_id == instance_id)
            .expect("pointwise coordinate came from Stage-16 A3 instance batch");
        if older_step != 14 || older_clause != 8 || newest_step != 15 {
            return Err(NaturalityOrbitTransportError::Invariant(
                "unexpected pointwise source".to_owned(),
            ));
        }
        let generic = registration(&grammar_archive, 15, newest_clause)?;
        pointwise_instances.push(construct_transport(
            signature,
            &predecessor,
            generic,
            &instance_id,
            &scheme_id,
            older_clause,
            pointwise_image.clone(),
            pointwise_image_nf.clone(),
            &pointwise_source.normalization_naturality_hash,
            a3_instance.identity_or_uniform_specialization,
            a3_instance.independently_exported_demand,
            "pointwise_type",
        )?);
    }
    let pointwise_families = pointwise_instances
        .iter()
        .map(|instance| instance.family_id.clone())
        .collect::<BTreeSet<_>>();
    let direct_family_ids = direct_families.iter().cloned().collect::<BTreeSet<_>>();
    let pointwise_instances_join_existing_j3_families = pointwise_instances.len() == 8
        && pointwise_families.len() == 8
        && pointwise_families == direct_family_ids;
    if !j3_regression_64_to_8 || !pointwise_instances_join_existing_j3_families {
        return Err(NaturalityOrbitTransportError::Invariant(format!(
            "A3 transport regression failed: direct {} instances/{} families, pointwise {} instances/{} families",
            direct_instances.len(),
            direct_families.len(),
            pointwise_instances.len(),
            pointwise_families.len()
        )));
    }
    let transported_instance_ids = direct_instances
        .iter()
        .chain(&pointwise_instances)
        .map(|instance| instance.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let expected_chronological_instance_ids = stage16
        .instances
        .iter()
        .filter(|instance| {
            stage16.schemes.iter().any(|scheme| {
                scheme.scheme_id == instance.scheme_id
                    && scheme.rule_constructor == A3RuleConstructor::ChronologicalComparison
            })
        })
        .map(|instance| instance.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let chronological_instance_transport_bijection = transported_instance_ids.len() == 72
        && transported_instance_ids == expected_chronological_instance_ids;
    if !chronological_instance_transport_bijection {
        return Err(NaturalityOrbitTransportError::Invariant(
            "typed chronological transports are not a bijection with the A3 instances".to_owned(),
        ));
    }

    let mut unary_pending_outputs = Vec::new();
    for instance in &stage16.instances {
        let scheme = stage16
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
            .ok_or_else(|| {
                NaturalityOrbitTransportError::Invariant("A3 orphan unary instance".to_owned())
            })?;
        if scheme.rule_constructor != A3RuleConstructor::UnaryAction {
            continue;
        }
        let source_anchor = instance.source_anchor_ids.first().ok_or_else(|| {
            NaturalityOrbitTransportError::Invariant("A3 unary source is absent".to_owned())
        })?;
        let source = stage16
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == *source_anchor)
            .ok_or_else(|| {
                NaturalityOrbitTransportError::Invariant(
                    "A3 unary source anchor does not resolve".to_owned(),
                )
            })?;
        let registered = registration(&grammar_archive, source.step, source.clause_index)?;
        if registered.normal_form != source.normal_form
            || registered.kernel_role != source.kernel_role
            || registered.kernel_type_json
                != serde_json::to_string(&source.kernel_type)
                    .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?
            || !registered.term_level_elaboration_succeeded
            || !registered.normalization_natural
        {
            return Err(NaturalityOrbitTransportError::Invariant(format!(
                "A3 unary source does not exactly join registration {}:{}",
                source.step, source.clause_index
            )));
        }
        unary_pending_outputs.push(pending(
            source.step,
            Some(source.clause_index),
            Some(&instance.instance_id),
            Some(&scheme.scheme_id),
            "unary_action",
            Some(registered),
            A3_UNARY_ACTION_OUTPUT_GAP,
            "the source family and its naturality token are typed, but A3 does not specify the action/operator whose output is demanded; source reflexivity is not an action proof",
        ));
    }

    let mut structural_completion_pending_outputs = Vec::new();
    for window in &generated.windows {
        for scheme in &window.schemes {
            if let A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } = &scheme.origin {
                let matching_instances = window
                    .instances
                    .iter()
                    .filter(|instance| instance.scheme_id == scheme.scheme_id)
                    .collect::<Vec<_>>();
                if matching_instances.len() != 1 {
                    return Err(NaturalityOrbitTransportError::Invariant(format!(
                        "completion scheme at Stage {} has {} instances, expected exactly one",
                        window.stage,
                        matching_instances.len()
                    )));
                }
                let instance = matching_instances[0];
                structural_completion_pending_outputs.push(pending(
                    window.stage,
                    None,
                    Some(&instance.instance_id),
                    Some(&scheme.scheme_id),
                    &format!("structural_completion::{constructor:?}"),
                    None,
                    A3_STRUCTURAL_COMPLETION_OUTPUT_GAP,
                    "a structural snapshot proves that a completion is required, but it does not determine an intrinsically typed object-level hole; importing the later answer would be retrospective",
                ));
            }
        }
    }

    let every_base_orbit_export_flag_derived_from_members =
        generated.windows.iter().all(|window| {
            window.orbits.iter().all(|orbit| {
                let scheme = window
                    .schemes
                    .iter()
                    .find(|scheme| scheme.scheme_id == orbit.scheme_id);
                let expected = window
                    .instances
                    .iter()
                    .filter(|instance| instance.scheme_id == orbit.scheme_id)
                    .any(|instance| instance.independently_exported_demand);
                scheme.is_some() && orbit.independently_exported_demand_orbit == expected
            })
        });
    if !every_base_orbit_export_flag_derived_from_members {
        return Err(NaturalityOrbitTransportError::Invariant(
            "A3 orbit export provenance is not derived from member provenance".to_owned(),
        ));
    }
    let chronological_outputs_constructed_and_kernel_typed = j3_regression_64_to_8
        && pointwise_instances_join_existing_j3_families
        && chronological_instance_transport_bijection
        && direct_instances
            .iter()
            .chain(&pointwise_instances)
            .all(|instance| {
                instance.normalization_equality.equal
                    && !instance.structural_substitution_hash.is_empty()
                    && !instance.typed_image_naturality_hash.is_empty()
                    && instance.uniform_specialization_not_new_family
                    && !instance.independently_exported_output_orbit
            });
    let unary_seed_count = unary_pending_outputs.len();
    let unary_registration_join_count = unary_pending_outputs
        .iter()
        .filter(|pending| pending.source_registration_name.is_some())
        .count();
    let full_a3_output_grammar_complete = unary_pending_outputs.is_empty()
        && structural_completion_pending_outputs.is_empty()
        && chronological_outputs_constructed_and_kernel_typed;
    let f1_executable = full_a3_output_grammar_complete;
    let semantic_o16_decided = false;
    let remaining_non_j3_seeds_never_defaulted_to_singletons = stage16_inventory_partition_exact
        && unary_pending_outputs.len() == 17
        && unary_registration_join_count == 17
        && pointwise_instances_join_existing_j3_families;
    if !remaining_non_j3_seeds_never_defaulted_to_singletons
        || !chronological_outputs_constructed_and_kernel_typed
    {
        return Err(NaturalityOrbitTransportError::Invariant(
            "A3 remaining-seed accounting or typed chronological transport is incomplete"
                .to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "a3-orbit-transport-audit",
        &(
            (
                grammar_archive_digest_valid,
                grammar_live_replay_valid,
                &grammar_live_replay_errors,
                j3_predecessor_archive_digest_valid,
                j3_predecessor_live_replay_valid,
                &j3_predecessor_live_replay_errors,
                frozen_surface_drift_bound_explicitly,
            ),
            stage16.instances.len(),
            stage16_inventory_partition_exact,
            chronological_instance_transport_bijection,
            &direct_instances,
            j3_regression_64_to_8,
            &pointwise_instances,
            pointwise_instances_join_existing_j3_families,
            &unary_pending_outputs,
            &structural_completion_pending_outputs,
            remaining_non_j3_seeds_never_defaulted_to_singletons,
            every_base_orbit_export_flag_derived_from_members,
            full_a3_output_grammar_complete,
        ),
    );
    Ok(A3OrbitTransportAudit {
        grammar_archive_digest_valid,
        grammar_live_replay_valid,
        grammar_live_replay_errors,
        j3_predecessor_archive_digest_valid,
        j3_predecessor_live_replay_valid,
        j3_predecessor_live_replay_errors,
        frozen_surface_drift_bound_explicitly,
        stage16_metadata_instance_count: stage16.instances.len(),
        stage16_inventory_partition_exact,
        chronological_instance_transport_bijection,
        direct_instance_count: direct_instances.len(),
        direct_natural_family_count: direct_families.len(),
        direct_instances,
        j3_regression_64_to_8,
        pointwise_instance_count: pointwise_instances.len(),
        pointwise_natural_family_count: pointwise_families.len(),
        pointwise_instances,
        pointwise_instances_join_existing_j3_families,
        chronological_outputs_constructed_and_kernel_typed,
        unary_seed_count,
        unary_registration_join_count,
        unary_pending_outputs,
        structural_completion_pending_outputs,
        remaining_non_j3_seeds_never_defaulted_to_singletons,
        every_base_orbit_export_flag_derived_from_members,
        full_a3_output_grammar_complete,
        f1_executable,
        semantic_o16_decided,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &NaturalityOrbitTransportCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

pub fn issue_naturality_orbit_transport_certificate()
-> Result<NaturalityOrbitTransportCertificate, NaturalityOrbitTransportError> {
    protocol_replayed()?;
    let signature = SealedSignature::genesis_del_h15();
    let a3 = prove_a3_orbit_transport_for_signature(&signature)?;
    let stage4 = stage4_audit()?;
    let outcome = match (
        stage4.r_t1_dissolves_tie,
        stage4.r_t2_required,
        a3.full_a3_output_grammar_complete,
    ) {
        (false, true, false) => "transport_regression_passed_r_t1_found_four_classes_a3_unary_and_structural_outputs_pending",
        (false, true, true) => "transport_complete_r_t1_found_four_classes",
        (true, false, true) => "transport_complete_r_t1_dissolved_tie",
        (true, false, false) => "transport_regression_passed_r_t1_dissolved_tie_a3_unary_and_structural_outputs_pending",
        state => {
            return Err(NaturalityOrbitTransportError::Invariant(format!(
                "non-exhaustive Stage-4 quotient/A3 completion state: {state:?}"
            )));
        }
    }
    .to_owned();
    let mut certificate = NaturalityOrbitTransportCertificate {
        schema: NATURALITY_ORBIT_TRANSPORT_SCHEMA.to_owned(),
        date: NATURALITY_ORBIT_TRANSPORT_DATE.to_owned(),
        source_bindings: source_bindings(),
        protocol_adoption_replayed: true,
        equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
        desired_count_used_as_definition_input: false,
        bar_or_historical_winner_used: false,
        enumeration_or_hash_order_used_as_selector: false,
        a3,
        stage4,
        outcome,
        permitted_conclusion: "The direct A3 surface now reproduces the certified 64 instances to eight J3 natural families, and the eight pointwise substitutions are typed members of those same families. The 17 unary seeds remain named pending because no action term is defined. At Stage 4 the four parsimony minimizers remain four frozen typed semantic packages, so R-T1 does not dissolve the tie.".to_owned(),
        required_successor_action: "Advance the Stage-4 ladder to R-T2 with a branching, count-blind confluence runner. Separately adjudicate/define the unary action output and an intrinsically typed structural-completion hole before rerunning full E-5. Do not run T-BF2, the bridge, adoption, or a halt certificate from this artifact.".to_owned(),
        bridge_authorized: false,
        bar_free_adoption_authorized: false,
        halt_claim_issued: false,
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: String) -> NaturalityOrbitTransportReplay {
    NaturalityOrbitTransportReplay {
        valid: false,
        direct_regression_64_to_8: false,
        pointwise_joined_to_eight: false,
        unary_pending_count: 0,
        stage4_class_count: 0,
        r_t2_required: false,
        f1_executable: false,
        errors: vec![error],
    }
}

pub fn replay_naturality_orbit_transport_certificate(
    certificate: &NaturalityOrbitTransportCertificate,
) -> NaturalityOrbitTransportReplay {
    let expected = match issue_naturality_orbit_transport_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate != &expected {
        errors.push("certificate differs from independent definition replay".to_owned());
    }
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("certificate digest mismatch".to_owned());
    }
    NaturalityOrbitTransportReplay {
        valid: errors.is_empty(),
        direct_regression_64_to_8: certificate.a3.j3_regression_64_to_8,
        pointwise_joined_to_eight: certificate.a3.pointwise_instances_join_existing_j3_families,
        unary_pending_count: certificate.a3.unary_pending_outputs.len(),
        stage4_class_count: certificate.stage4.semantic_class_count,
        r_t2_required: certificate.stage4.r_t2_required,
        f1_executable: certificate.a3.f1_executable,
        errors,
    }
}

pub fn emit_naturality_orbit_transport_create_new(
    path: &Path,
) -> Result<NaturalityOrbitTransportReplay, NaturalityOrbitTransportError> {
    let certificate = issue_naturality_orbit_transport_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| NaturalityOrbitTransportError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| NaturalityOrbitTransportError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| NaturalityOrbitTransportError::Io(error.to_string()))?;
    let replay = replay_naturality_orbit_transport_certificate(&certificate);
    if !replay.valid {
        return Err(NaturalityOrbitTransportError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn certificate() -> NaturalityOrbitTransportCertificate {
        static CERTIFICATE: OnceLock<NaturalityOrbitTransportCertificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_naturality_orbit_transport_certificate().unwrap())
            .clone()
    }

    #[test]
    fn signature_api_reproduces_the_genesis_certificate_a3_audit() {
        let signature = SealedSignature::genesis_del_h15();
        let audit = prove_a3_orbit_transport_for_signature(&signature).unwrap();
        assert_eq!(audit, certificate().a3);
    }

    #[test]
    fn non_enacted_r_t2_stage4_branch_preserves_the_stage16_transport_regression() {
        const NON_ENACTED_R_T2_STAGE4_HASH: &str =
            "blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308";

        let replacement = stage4_cone()
            .into_iter()
            .find(|telescope| candidate_hash(telescope) == NON_ENACTED_R_T2_STAGE4_HASH)
            .expect("the certified non-enacted R-T2 Stage-4 branch is in the live cone");
        assert_ne!(replacement, Telescope::reference(4));

        let mut telescopes = Telescope::all_reference_telescopes();
        telescopes
            .iter_mut()
            .find(|(step, _)| *step == 4)
            .expect("the fifteen-step reference signature contains Stage 4")
            .1 = replacement;
        let signature = SealedSignature::from_telescopes(telescopes);
        assert_eq!(
            signature
                .entry(4)
                .expect("the branch signature contains Stage 4")
                .candidate_hash,
            NON_ENACTED_R_T2_STAGE4_HASH
        );

        let audit = prove_a3_orbit_transport_for_signature(&signature).unwrap();
        assert_eq!(audit.stage16_metadata_instance_count, 89);
        assert!(audit.stage16_inventory_partition_exact);
        assert_eq!(audit.unary_seed_count, 17);
        assert_eq!(audit.direct_instance_count, 64);
        assert_eq!(audit.direct_natural_family_count, 8);
        assert!(audit.j3_regression_64_to_8);
        assert_eq!(audit.pointwise_instance_count, 8);
        assert_eq!(audit.pointwise_natural_family_count, 8);
        assert_eq!(
            audit.direct_instance_count + audit.pointwise_instance_count,
            72
        );
        assert!(audit.chronological_instance_transport_bijection);
    }

    #[test]
    fn a3_transport_reproduces_64_to_8_and_does_not_multiply_pointwise_instances() {
        let certificate = certificate();
        assert_eq!(certificate.a3.direct_instance_count, 64);
        assert_eq!(certificate.a3.direct_natural_family_count, 8);
        assert!(certificate.a3.j3_regression_64_to_8);
        assert_eq!(certificate.a3.pointwise_instance_count, 8);
        assert_eq!(certificate.a3.pointwise_natural_family_count, 8);
        assert!(certificate.a3.pointwise_instances_join_existing_j3_families);
        assert!(certificate.a3.grammar_archive_digest_valid);
        assert!(certificate.a3.j3_predecessor_archive_digest_valid);
        assert_eq!(
            certificate.a3.frozen_surface_drift_bound_explicitly,
            !certificate.a3.grammar_live_replay_valid
                || !certificate.a3.j3_predecessor_live_replay_valid
        );
        assert!(certificate.a3.stage16_inventory_partition_exact);
        assert!(certificate.a3.chronological_instance_transport_bijection);
        assert_eq!(certificate.a3.unary_pending_outputs.len(), 17);
        assert!(!certificate.a3.full_a3_output_grammar_complete);
        assert!(!certificate.a3.f1_executable);
    }

    #[test]
    fn stage4_r_t1_proves_four_semantic_classes_and_selects_nothing() {
        let certificate = certificate();
        assert_eq!(certificate.stage4.minimizer_count, 4);
        assert_eq!(certificate.stage4.pairwise_comparisons.len(), 6);
        assert_eq!(certificate.stage4.semantic_class_count, 4);
        assert!(!certificate.stage4.r_t1_dissolves_tie);
        assert!(certificate.stage4.r_t2_required);
        assert!(
            certificate
                .stage4
                .pairwise_comparisons
                .iter()
                .all(|comparison| !comparison.packages_equal)
        );
        assert!(!certificate.bridge_authorized);
        assert!(!certificate.bar_free_adoption_authorized);
        assert!(!certificate.halt_claim_issued);
    }

    #[test]
    fn stage4_quotient_is_invariant_under_package_and_pair_enumeration_order() {
        let certificate = certificate();
        let mut packages = certificate.stage4.packages.clone();
        let mut comparisons = certificate.stage4.pairwise_comparisons.clone();
        packages.reverse();
        comparisons.reverse();
        let reversed = form_stage4_orbit_classes(&packages, &comparisons).unwrap();
        assert_eq!(reversed, certificate.stage4.orbit_classes);
    }

    #[test]
    fn replay_rejects_a_redigested_false_promotion() {
        let mut forged = certificate();
        forged.a3.full_a3_output_grammar_complete = true;
        forged.a3.f1_executable = true;
        forged.bridge_authorized = true;
        forged.result_digest = certificate_digest(&forged);
        assert!(!replay_naturality_orbit_transport_certificate(&forged).valid);
    }
}
