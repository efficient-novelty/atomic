//! DEMAND-COMPLETE v1: an independent pre-instance demand rule-seed audit.
//!
//! The intended side is generated solely from the two sealed window
//! telescopes and the potential depth-two rule shapes stated by the Guard-Rail note and
//! Appendix K: unary action, binary comparison, and higher open-box
//! reduction. It does not consume package labels or extractor output.
//!
//! Those sources do not provide an operational grammar of typed hole
//! instances, a quotient into demand orbits, or a decision procedure for
//! membership in definitional closure D. Therefore this module records a
//! named D-4 gap instead of shrinking C(W) to the current extractor.

use crate::debt_guard::{directive_debt_timeline, o16_emptiness_report};
use crate::demand_orbits::{kernel_stage_inventories, stage_inventories_for_timeline};
use crate::semantic_provenance::OrbitResolution;
use crate::tdc1_cubical::{CubicalRegressionResult, replay_cubical_regression};
use crate::typed_families::predecessor_closure;
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use thiserror::Error;

pub const DEMAND_COMPLETENESS_SCHEMA: &str = "demand-completeness-v1";
pub const DEMAND_COMPLETENESS_DATE: &str = "2026-07-19";
pub const INTENDED_RULE_SEED_GENERATOR_VERSION: &str = "independent-depth-two-window-rule-seeds-v1";
pub const C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS: &str =
    "C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS";
pub const D1_INTENDED_CW_INSTANCE_GRAMMAR_NOT_OPERATIONAL: &str =
    "D1_INTENDED_CW_INSTANCE_GRAMMAR_NOT_OPERATIONAL";
pub const D2_TYPED_HOLE_INSTANTIATION_DOMAIN_NOT_OPERATIONAL: &str =
    "D2_TYPED_HOLE_INSTANTIATION_DOMAIN_NOT_OPERATIONAL";
pub const D3_UNIVALENT_DEMAND_ORBIT_QUOTIENT_NOT_OPERATIONAL: &str =
    "D3_UNIVALENT_DEMAND_ORBIT_QUOTIENT_NOT_OPERATIONAL";
pub const D4_DEFINITIONAL_CLOSURE_MEMBERSHIP_NOT_DECIDABLE: &str =
    "D4_DEFINITIONAL_CLOSURE_MEMBERSHIP_NOT_DECIDABLE";
pub const D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED: &str =
    "D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED";

const O16_BYTES: &[u8] = include_bytes!("../../../docs/o16_emptiness.json");
const T1_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/t1_result.md");
const TDC_CUBICAL_BYTES: &[u8] = include_bytes!("../../../docs/tdc1_cubical_regression_v3.json");
const SEMANTIC_PROVENANCE_BYTES: &[u8] =
    include_bytes!("../../../docs/semantic_provenance_audit.json");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum DemandCompletenessError {
    #[error("typed predecessor construction failed: {0}")]
    Closure(String),
    #[error("extractor replay failed: {0}")]
    Extractor(String),
    #[error("source artifact {name} failed exact binding: {reason}")]
    Archive { name: String, reason: String },
    #[error("sealed window source is missing step {0}")]
    MissingWindowStep(u32),
    #[error("historical ladder regression failed: {0}")]
    Ladder(String),
    #[error("certificate JSON failed: {0}")]
    Json(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBinding {
    pub name: String,
    pub path: String,
    pub byte_length: u64,
    pub sha256: String,
    pub exact_bytes_pinned: bool,
    pub definition_replayed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardRailPremiseAudit {
    pub a1_cumulativity: String,
    pub a2_monotone_derivability: String,
    pub a3_demand_schema: String,
    pub a4_depth_two_admissibility: String,
    pub theorem_12_claim: String,
    pub falsifier_f1: String,
    pub a3_asserts_finite_cw_exists: bool,
    pub a3_supplies_constructive_instance_generator: bool,
    pub theorem_12_instance_granularity_machine_checked: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WindowClauseSeed {
    pub step: u32,
    pub candidate_hash: String,
    pub clause_index: u16,
    pub role: ClauseRole,
    pub expression_digest: String,
    pub vocabulary_features: Vec<String>,
    pub seed_id: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntendedRuleSeedKind {
    UnaryAction,
    BinaryComparison,
    HigherOpenBoxReduction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntendedRuleSeed {
    pub kind: IntendedRuleSeedKind,
    pub source_clause_seeds: Vec<String>,
    pub interpretation: String,
    pub pre_instance_overapproximation: bool,
    pub exported_public_eligibility_enforced: bool,
    pub univalent_eligibility_enforced: bool,
    pub candidate_rule_seed_only: bool,
    pub typed_demand_instance_minted: bool,
    pub seed_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntendedDepthTwoWindow {
    /// The immediately preceding sealed stage, S_(n-1).
    pub newest_step: u32,
    /// The next preceding sealed stage, S_(n-2).
    pub older_step: u32,
    pub orientation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntendedWindowSpecification {
    pub stage: u32,
    pub window: IntendedDepthTwoWindow,
    pub clause_seeds: Vec<WindowClauseSeed>,
    pub rule_seeds: Vec<IntendedRuleSeed>,
    pub finite_rule_seed_inventory: bool,
    pub rule_seeds_are_pre_instance_overapproximation: bool,
    pub independent_of_extractor_labels: bool,
    pub typed_hole_instance_domain_constructed: bool,
    pub univalent_orbit_quotient_constructed: bool,
    pub definitional_closure_membership_decider_constructed: bool,
    pub generated_intended_demand_instances: Option<Vec<String>>,
    pub finite_complete_cw_constructed: bool,
    pub gap_ids: Vec<String>,
    pub specification_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoarseLadderRow {
    pub stage: u32,
    pub expected_packages: Vec<String>,
    pub engine_required_packages: Vec<String>,
    pub extractor_live_packages: Vec<String>,
    pub focus_family: String,
    pub pre_structural_band: bool,
    pub exact_regression_match: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtractorIdentificationRow {
    pub stage: u32,
    pub extractor_live_orbit_count: u32,
    pub intended_demand_instance_count: Option<u32>,
    pub extractor_equals_intended: Option<bool>,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtractorRelativityWitness {
    pub supplied_timeline_stages: u32,
    pub computed_inventory_count: u32,
    pub exactly_sixteen_inventories_computed: bool,
    pub all_supplied_package_sets_empty: bool,
    pub canonical_timeline_has_nonempty_package_sets: bool,
    pub supplied_timeline_differs_from_canonical: bool,
    pub extraction_succeeded: bool,
    pub all_orbit_inventories_empty: bool,
    pub all_j2_marked_kernel_verified: bool,
    pub all_j3_marked_kernel_verified: bool,
    pub all_locality_marked_kernel_verified: bool,
    pub locality_ledger_empty_and_holds: bool,
    pub demonstrates_evidence_is_relative_to_supplied_extractor_timeline: bool,
    pub proves_intended_schema_completeness: bool,
    pub exhibits_f1_demanded_but_underdetermined_instance: bool,
    pub canonical_extraction_derivation_hash: String,
    pub extraction_derivation_hash: String,
    pub witness_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct F1Audit {
    pub s15_s14_intended_instances_enumerated: bool,
    pub every_intended_instance_derivability_decided: bool,
    pub f1_executable: bool,
    pub demanded_but_underdetermined_instance: Option<String>,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub theorem_12_refuted: bool,
    pub theorem_12_proved: bool,
    pub focus_gating_restored: bool,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct F5Audit {
    pub a3_finiteness_asserted: bool,
    pub finite_complete_cw_machine_verified: bool,
    pub nontermination_or_nonfiniteness_witness: Option<String>,
    pub f5_triggered: bool,
    pub f5_excluded: bool,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceDiagnosticAudit {
    pub tdc_archive_definition_replayed: bool,
    pub tdc_archive_result_digest: String,
    pub step8_joined_to_tdc_subject: bool,
    pub registered_d4_joined_to_tdc_subject: bool,
    pub registered_d4_label: String,
    pub registered_d4_basis_derivation_hash: String,
    pub step8_extractor_live_outputs: u32,
    pub registered_d4_extractor_live_outputs: u32,
    pub tdc_live_output_counts_match_kernel_extractor: bool,
    pub extractor_relative_one_vs_zero_reproduced: bool,
    pub intended_demand_completeness_proved: bool,
    pub registered_d4_certified_demand_orphaned: bool,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DemandCompletenessCertificate {
    pub schema: String,
    pub date: String,
    pub intended_generator_version: String,
    pub source_bindings: Vec<SourceBinding>,
    pub primary_gap_id: String,
    pub premises: GuardRailPremiseAudit,
    pub intended_window_specifications: Vec<IntendedWindowSpecification>,
    pub coarse_ladder: Vec<CoarseLadderRow>,
    pub coarse_ladder_reproduced: bool,
    pub stage3_pre_jurisdiction_wrinkle_preserved: bool,
    pub extractor_identification: Vec<ExtractorIdentificationRow>,
    pub extractor_identified_with_intended_cw: bool,
    pub identification_gap_ids: Vec<String>,
    pub extractor_relativity_witness: ExtractorRelativityWitness,
    pub coarse_engine_o16_empty: bool,
    pub semantic_instance_granular_o16_empty: Option<bool>,
    pub f1: F1Audit,
    pub f5: F5Audit,
    pub provenance_diagnostic: ProvenanceDiagnosticAudit,
    pub outcome: String,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DemandCompletenessReplay {
    pub valid: bool,
    pub coarse_ladder_reproduced: bool,
    pub stage3_wrinkle_preserved: bool,
    pub intended_identification_proved: bool,
    pub semantic_o16_decided: bool,
    pub f1_triggered: bool,
    pub f5_triggered: bool,
    pub theorem_12_proved: bool,
    pub theorem_12_refuted: bool,
    pub d4_demand_orphan_certified: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(DEMAND_COMPLETENESS_SCHEMA, domain, value))
        .expect("demand-completeness proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:X}", Sha256::digest(bytes))
}

fn source_binding(
    name: &str,
    path: &str,
    bytes: &[u8],
    expected_length: u64,
    expected_sha256: &str,
    definition_replayed: bool,
) -> Result<SourceBinding, DemandCompletenessError> {
    let byte_length = bytes.len() as u64;
    let sha256 = sha256_hex(bytes);
    if byte_length != expected_length || sha256 != expected_sha256 {
        return Err(DemandCompletenessError::Archive {
            name: name.to_owned(),
            reason: format!(
                "expected {expected_length}/{expected_sha256}, found {byte_length}/{sha256}"
            ),
        });
    }
    Ok(SourceBinding {
        name: name.to_owned(),
        path: path.to_owned(),
        byte_length,
        sha256,
        exact_bytes_pinned: true,
        definition_replayed,
    })
}

fn expression_features(expression: &Expr, features: &mut BTreeSet<String>) {
    match expression {
        Expr::App(left, right) => {
            features.insert("application".to_owned());
            expression_features(left, features);
            expression_features(right, features);
        }
        Expr::Lam(body) => {
            features.insert("lambda".to_owned());
            expression_features(body, features);
        }
        Expr::Pi(left, right) => {
            features.insert("pi".to_owned());
            expression_features(left, features);
            expression_features(right, features);
        }
        Expr::Sigma(left, right) => {
            features.insert("sigma".to_owned());
            expression_features(left, features);
            expression_features(right, features);
        }
        Expr::Univ => {
            features.insert("universe".to_owned());
        }
        Expr::Var(_) => {
            features.insert("variable".to_owned());
        }
        Expr::Lib(_) => {
            features.insert("sealed_library_reference".to_owned());
        }
        Expr::Id(a, x, y) => {
            features.insert("identity".to_owned());
            expression_features(a, features);
            expression_features(x, features);
            expression_features(y, features);
        }
        Expr::Refl(body) => {
            features.insert("reflexivity".to_owned());
            expression_features(body, features);
        }
        Expr::Susp(body) => {
            features.insert("suspension".to_owned());
            expression_features(body, features);
        }
        Expr::Trunc(body) => {
            features.insert("truncation".to_owned());
            expression_features(body, features);
        }
        Expr::PathCon(dimension) => {
            features.insert(format!("path_constructor_dimension_{dimension}"));
        }
        Expr::Flat(body) | Expr::Sharp(body) | Expr::Disc(body) | Expr::Shape(body) => {
            features.insert("cohesive_modality".to_owned());
            expression_features(body, features);
        }
        Expr::Next(body) | Expr::Eventually(body) => {
            features.insert("temporal_operator".to_owned());
            expression_features(body, features);
        }
        Expr::Bang(body) | Expr::WhyNot(body) => {
            features.insert("linear_exponential".to_owned());
            expression_features(body, features);
        }
    }
}

fn clause_seed(
    step: u32,
    candidate_hash: &str,
    index: usize,
    role: ClauseRole,
    expr: &Expr,
) -> WindowClauseSeed {
    let expression_digest = tagged_hash("window-clause-expression", expr);
    let mut features = BTreeSet::new();
    expression_features(expr, &mut features);
    let clause_index = u16::try_from(index).expect("historical telescope index fits u16");
    let seed_id = tagged_hash(
        "window-clause-seed",
        &(
            step,
            candidate_hash,
            clause_index,
            role,
            &expression_digest,
            &features,
        ),
    );
    WindowClauseSeed {
        step,
        candidate_hash: candidate_hash.to_owned(),
        clause_index,
        role,
        expression_digest,
        vocabulary_features: features.into_iter().collect(),
        seed_id,
    }
}

fn rule_seed(
    stage: u32,
    kind: IntendedRuleSeedKind,
    sources: Vec<String>,
    interpretation: &str,
) -> IntendedRuleSeed {
    let seed_id = tagged_hash(
        "intended-rule-seed",
        &(stage, kind, &sources, interpretation),
    );
    IntendedRuleSeed {
        kind,
        source_clause_seeds: sources,
        interpretation: interpretation.to_owned(),
        pre_instance_overapproximation: true,
        exported_public_eligibility_enforced: false,
        univalent_eligibility_enforced: false,
        candidate_rule_seed_only: true,
        typed_demand_instance_minted: false,
        seed_id,
    }
}

/// Independent intended-side pre-instance generator. It accepts no package
/// timeline and calls no demand extractor. Its all-clause/cross-product seeds
/// deliberately overapproximate the source shapes: exported-public and
/// univalent eligibility cannot be enforced until the missing instance
/// grammar is supplied, so these seeds are not claimed to be `C(W)`.
fn intended_window_specification(
    signature: &SealedSignature,
    stage: u32,
) -> Result<IntendedWindowSpecification, DemandCompletenessError> {
    let older_step = stage - 2;
    let newest_step = stage - 1;
    let mut older = Vec::new();
    let mut newer = Vec::new();
    for (slot, step) in [older_step, newest_step].into_iter().enumerate() {
        let entry = signature
            .entries()
            .iter()
            .find(|entry| entry.step == step)
            .ok_or(DemandCompletenessError::MissingWindowStep(step))?;
        let target = if slot == 0 { &mut older } else { &mut newer };
        target.extend(
            entry
                .telescope
                .clauses
                .iter()
                .enumerate()
                .map(|(index, clause)| {
                    clause_seed(
                        step,
                        &entry.candidate_hash,
                        index,
                        clause.role,
                        &clause.expr,
                    )
                }),
        );
    }
    let mut clause_seeds = older.clone();
    clause_seeds.extend(newer.clone());
    let mut rule_seeds = clause_seeds
        .iter()
        .map(|clause| {
            rule_seed(
                stage,
                IntendedRuleSeedKind::UnaryAction,
                vec![clause.seed_id.clone()],
                "potential object-wise action on one exported public clause",
            )
        })
        .collect::<Vec<_>>();
    for left in &older {
        for right in &newer {
            rule_seeds.push(rule_seed(
                stage,
                IntendedRuleSeedKind::BinaryComparison,
                vec![left.seed_id.clone(), right.seed_id.clone()],
                "potential binary comparison across the two chronological window layers",
            ));
        }
    }
    rule_seeds.push(rule_seed(
        stage,
        IntendedRuleSeedKind::HigherOpenBoxReduction,
        clause_seeds.iter().map(|clause| clause.seed_id.clone()).collect(),
        "higher structural obligations, if generated, reduce to contractible open-box extension data",
    ));
    let gap_ids = vec![
        D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED.to_owned(),
        C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS.to_owned(),
        D1_INTENDED_CW_INSTANCE_GRAMMAR_NOT_OPERATIONAL.to_owned(),
        D2_TYPED_HOLE_INSTANTIATION_DOMAIN_NOT_OPERATIONAL.to_owned(),
        D3_UNIVALENT_DEMAND_ORBIT_QUOTIENT_NOT_OPERATIONAL.to_owned(),
        D4_DEFINITIONAL_CLOSURE_MEMBERSHIP_NOT_DECIDABLE.to_owned(),
    ];
    let mut specification = IntendedWindowSpecification {
        stage,
        window: IntendedDepthTwoWindow {
            newest_step,
            older_step,
            orientation: "W=(newest_step,older_step)".to_owned(),
        },
        clause_seeds,
        rule_seeds,
        finite_rule_seed_inventory: true,
        rule_seeds_are_pre_instance_overapproximation: true,
        independent_of_extractor_labels: true,
        typed_hole_instance_domain_constructed: false,
        univalent_orbit_quotient_constructed: false,
        definitional_closure_membership_decider_constructed: false,
        generated_intended_demand_instances: None,
        finite_complete_cw_constructed: false,
        gap_ids,
        specification_digest: String::new(),
    };
    specification.specification_digest =
        tagged_hash("intended-window-specification", &specification);
    Ok(specification)
}

fn expected_packages(stage: u32) -> Vec<String> {
    let package = match stage {
        3 | 4 => Some("former_eliminator"),
        5 => Some("initial_hit"),
        6 => Some("truncation_hit"),
        7 => Some("higher_hit"),
        8 => Some("sphere_lift"),
        9 => Some("axiomatic_bundle"),
        10 => Some("modal_shell"),
        11 => Some("connection_shell"),
        12 => Some("curvature_shell"),
        13 => Some("operator_bundle"),
        14 => Some("hilbert_functional"),
        15 => Some("temporal_shell"),
        _ => None,
    };
    package.into_iter().map(str::to_owned).collect()
}

fn expected_focus_and_band(stage: u32) -> (&'static str, bool) {
    match stage {
        1 | 2 | 3 => ("None", true),
        4 => ("Some(FormerEliminator)", false),
        5 => ("Some(InitialHit)", false),
        6 => ("Some(TruncationHit)", false),
        7 => ("Some(HigherHit)", false),
        8 => ("Some(SphereLift)", false),
        9 => ("Some(AxiomaticBundle)", false),
        10 => ("Some(ModalShell)", false),
        11 => ("Some(ConnectionShell)", false),
        12 => ("Some(CurvatureShell)", false),
        13 => ("Some(OperatorBundle)", false),
        14 => ("Some(HilbertFunctional)", false),
        15 => ("Some(TemporalShell)", false),
        16 => ("None", false),
        _ => ("OUT_OF_RANGE", false),
    }
}

fn certificate_digest(certificate: &DemandCompletenessCertificate) -> String {
    let mut payload = certificate.clone();
    payload.result_digest.clear();
    tagged_hash("result", &payload)
}

pub fn build_demand_completeness_certificate()
-> Result<DemandCompletenessCertificate, DemandCompletenessError> {
    let archived_o16: serde_json::Value =
        serde_json::from_slice(O16_BYTES).map_err(|error| DemandCompletenessError::Archive {
            name: "O16 engine-face artifact".to_owned(),
            reason: error.to_string(),
        })?;
    let live_o16 = o16_emptiness_report();
    let live_o16_value = serde_json::to_value(&live_o16).expect("O16 report serializes");
    if archived_o16 != live_o16_value {
        return Err(DemandCompletenessError::Archive {
            name: "O16 engine-face artifact".to_owned(),
            reason: "artifact differs from definition replay".to_owned(),
        });
    }
    let archived_tdc: CubicalRegressionResult =
        serde_json::from_slice(TDC_CUBICAL_BYTES).map_err(|error| {
            DemandCompletenessError::Archive {
                name: "TDC cubical regression v3".to_owned(),
                reason: error.to_string(),
            }
        })?;
    replay_cubical_regression(&archived_tdc).map_err(|error| DemandCompletenessError::Archive {
        name: "TDC cubical regression v3".to_owned(),
        reason: format!("definition replay failed: {error}"),
    })?;
    let source_bindings = vec![
        source_binding(
            "O16 engine-face artifact",
            "docs/o16_emptiness.json",
            O16_BYTES,
            3_679,
            "FDD67681F249E853A36ACB2C4DA1DCBD00AC33B03C0D657C238DAAC93FE2B83C",
            true,
        )?,
        source_binding(
            "TDC cubical regression v3",
            "docs/tdc1_cubical_regression_v3.json",
            TDC_CUBICAL_BYTES,
            36_033,
            "65CF20883420C5777BF487FFFCE046A447E097DDEFD0772E4B58BF3808F12753",
            true,
        )?,
        source_binding(
            "semantic provenance audit archive",
            "docs/semantic_provenance_audit.json",
            SEMANTIC_PROVENANCE_BYTES,
            71_763,
            "50979A76CE168B30F072BB1DC610A784EFBB3847CB3D6066191784054EB30E81",
            false,
        )?,
        source_binding(
            "T1 result and Guard-Rail addendum",
            "docs/t1_result.md",
            T1_RESULT_BYTES,
            22_100,
            "8B7E1FA78F4226D1FF424FF06C34E6A22AFEA0293C1655F16C2D3666760BD6E0",
            false,
        )?,
    ];

    let signature = SealedSignature::genesis_del_h15();
    let closure = predecessor_closure(&signature)
        .map_err(|error| DemandCompletenessError::Closure(error.to_string()))?;
    let extraction = kernel_stage_inventories(&signature, &closure)
        .map_err(|error| DemandCompletenessError::Extractor(error.to_string()))?;
    let timeline = directive_debt_timeline();
    let mut coarse_ladder = Vec::with_capacity(16);
    for record in &timeline {
        let extractor_inventory = extraction.stage(record.stage).ok_or_else(|| {
            DemandCompletenessError::Ladder(format!("missing stage {}", record.stage))
        })?;
        let extractor_live_packages = extractor_inventory
            .orbits
            .iter()
            .filter(|orbit| matches!(orbit.resolution, OrbitResolution::Live))
            .map(|orbit| orbit.package.clone())
            .collect::<Vec<_>>();
        let expected = expected_packages(record.stage);
        let engine = record
            .required_packages
            .iter()
            .map(|name| (*name).to_owned())
            .collect();
        let (expected_focus, expected_pre_structural_band) = expected_focus_and_band(record.stage);
        coarse_ladder.push(CoarseLadderRow {
            stage: record.stage,
            exact_regression_match: expected == engine
                && expected == extractor_live_packages
                && record.focus_family == expected_focus
                && record.pre_structural_band == expected_pre_structural_band,
            expected_packages: expected,
            engine_required_packages: engine,
            extractor_live_packages,
            focus_family: record.focus_family.clone(),
            pre_structural_band: record.pre_structural_band,
        });
    }
    let exact_ordered_unique_stage_domain =
        coarse_ladder.len() == 16 && coarse_ladder.iter().map(|row| row.stage).eq(1_u32..=16_u32);
    let coarse_ladder_reproduced = exact_ordered_unique_stage_domain
        && coarse_ladder.iter().all(|row| row.exact_regression_match)
        && coarse_ladder
            .iter()
            .filter(|row| (4..=15).contains(&row.stage))
            .all(|row| row.expected_packages.len() == 1)
        && coarse_ladder
            .iter()
            .find(|row| row.stage == 16)
            .is_some_and(|row| row.expected_packages.is_empty());
    let stage3_pre_jurisdiction_wrinkle_preserved = coarse_ladder
        .iter()
        .find(|row| row.stage == 3)
        .is_some_and(|row| {
            row.expected_packages == ["former_eliminator"]
                && row.pre_structural_band
                && row.focus_family == "None"
        })
        && coarse_ladder
            .iter()
            .find(|row| row.stage == 4)
            .is_some_and(|row| {
                row.expected_packages == ["former_eliminator"]
                    && !row.pre_structural_band
                    && row.focus_family == "Some(FormerEliminator)"
            });
    if !coarse_ladder_reproduced || !stage3_pre_jurisdiction_wrinkle_preserved {
        return Err(DemandCompletenessError::Ladder(
            "coarse ladder or stage-3 pre-jurisdiction wrinkle drifted".to_owned(),
        ));
    }

    let intended_window_specifications = (3..=16)
        .map(|stage| intended_window_specification(&signature, stage))
        .collect::<Result<Vec<_>, _>>()?;
    let extractor_identification = intended_window_specifications
        .iter()
        .map(|specification| ExtractorIdentificationRow {
            stage: specification.stage,
            extractor_live_orbit_count: u32::try_from(
                extraction.live_orbit_count(specification.stage),
            )
            .expect("historical orbit count fits u32"),
            intended_demand_instance_count: None,
            extractor_equals_intended: None,
            status: D1_INTENDED_CW_INSTANCE_GRAMMAR_NOT_OPERATIONAL.to_owned(),
        })
        .collect::<Vec<_>>();

    let empty_timeline = (1..=16)
        .map(|stage| (stage, Vec::<String>::new()))
        .collect::<Vec<_>>();
    let empty_extraction = stage_inventories_for_timeline(&signature, &closure, &empty_timeline)
        .map_err(|error| DemandCompletenessError::Extractor(error.to_string()))?;
    let computed_inventory_count = u32::try_from(empty_extraction.inventories.len())
        .expect("historical inventory count fits u32");
    let exactly_sixteen_inventories_computed = computed_inventory_count == 16;
    let supplied_timeline_stages =
        u32::try_from(empty_timeline.len()).expect("historical timeline count fits u32");
    let all_supplied_package_sets_empty = supplied_timeline_stages == 16
        && empty_timeline
            .iter()
            .all(|(_, packages)| packages.is_empty());
    // The cardinality guard on each universal prevents an empty result from
    // satisfying the witness vacuously.
    let all_orbit_inventories_empty = exactly_sixteen_inventories_computed
        && empty_extraction
            .inventories
            .iter()
            .all(|inventory| inventory.orbits.is_empty());
    let all_j2_marked_kernel_verified = exactly_sixteen_inventories_computed
        && empty_extraction.inventories.iter().all(|inventory| {
            inventory
                .extraction_completeness_assumption
                .is_kernel_verified()
        });
    let all_j3_marked_kernel_verified = exactly_sixteen_inventories_computed
        && empty_extraction.inventories.iter().all(|inventory| {
            inventory
                .derivability_completeness_assumption
                .is_kernel_verified()
        });
    let all_locality_marked_kernel_verified = exactly_sixteen_inventories_computed
        && empty_extraction
            .inventories
            .iter()
            .all(|inventory| inventory.window_locality_assumption.is_kernel_verified());
    let locality_ledger_empty_and_holds = empty_extraction.locality_ledger.holds
        && empty_extraction.locality_ledger.transitions.is_empty()
        && empty_extraction.locality_ledger.expiries.is_empty();
    let canonical_timeline_has_nonempty_package_sets = timeline
        .iter()
        .any(|record| !record.required_packages.is_empty());
    let supplied_timeline_differs_from_canonical = empty_timeline.len() == timeline.len()
        && all_supplied_package_sets_empty
        && canonical_timeline_has_nonempty_package_sets
        && empty_timeline
            .iter()
            .zip(&timeline)
            .all(|((stage, _), record)| *stage == record.stage);
    let demonstrates_extractor_relativity = exactly_sixteen_inventories_computed
        && all_orbit_inventories_empty
        && all_j2_marked_kernel_verified
        && all_j3_marked_kernel_verified
        && all_locality_marked_kernel_verified
        && locality_ledger_empty_and_holds
        && supplied_timeline_differs_from_canonical
        && empty_extraction.derivation_hash != extraction.derivation_hash;
    let witness_digest = tagged_hash(
        "all-empty-extractor-relativity-witness",
        &(
            &empty_timeline,
            &empty_extraction.derivation_hash,
            &extraction.derivation_hash,
            computed_inventory_count,
            all_supplied_package_sets_empty,
            all_orbit_inventories_empty,
            all_j2_marked_kernel_verified,
            all_j3_marked_kernel_verified,
            all_locality_marked_kernel_verified,
            locality_ledger_empty_and_holds,
            supplied_timeline_differs_from_canonical,
        ),
    );
    let extractor_relativity_witness = ExtractorRelativityWitness {
        supplied_timeline_stages,
        computed_inventory_count,
        exactly_sixteen_inventories_computed,
        all_supplied_package_sets_empty,
        canonical_timeline_has_nonempty_package_sets,
        supplied_timeline_differs_from_canonical,
        extraction_succeeded: true,
        all_orbit_inventories_empty,
        all_j2_marked_kernel_verified,
        all_j3_marked_kernel_verified,
        all_locality_marked_kernel_verified,
        locality_ledger_empty_and_holds,
        demonstrates_evidence_is_relative_to_supplied_extractor_timeline:
            demonstrates_extractor_relativity,
        proves_intended_schema_completeness: false,
        exhibits_f1_demanded_but_underdetermined_instance: false,
        canonical_extraction_derivation_hash: extraction.derivation_hash.clone(),
        extraction_derivation_hash: empty_extraction.derivation_hash,
        witness_digest,
    };

    let step8_outputs = extraction
        .stage(8)
        .expect("stage 8 exists")
        .orbits
        .iter()
        .filter(|orbit| matches!(orbit.resolution, OrbitResolution::Live))
        .map(|orbit| orbit.required_outputs.len() as u32)
        .sum();
    let d4_outputs = extraction
        .stage(16)
        .expect("stage 16 exists")
        .orbits
        .iter()
        .filter(|orbit| matches!(orbit.resolution, OrbitResolution::Live))
        .map(|orbit| orbit.required_outputs.len() as u32)
        .sum();
    let tdc_step8_rows = archived_tdc
        .provenance_obstructions
        .iter()
        .filter(|row| row.inventory_stage == 8 && row.subject == "Step 8 (S3)")
        .collect::<Vec<_>>();
    let tdc_d4_rows = archived_tdc
        .provenance_obstructions
        .iter()
        .filter(|row| row.inventory_stage == 16 && row.subject == "registered d=4 candidate")
        .collect::<Vec<_>>();
    if tdc_step8_rows.len() != 1 || tdc_d4_rows.len() != 1 {
        return Err(DemandCompletenessError::Archive {
            name: "TDC cubical regression v3".to_owned(),
            reason: format!(
                "expected unique Step-8 and registered-d4 provenance rows, found {}/{}",
                tdc_step8_rows.len(),
                tdc_d4_rows.len()
            ),
        });
    }
    let tdc_step8 = tdc_step8_rows[0];
    let tdc_d4 = tdc_d4_rows[0];
    let step8_joined_to_tdc_subject = tdc_step8.inventory_stage == 8;
    let registered_d4_joined_to_tdc_subject = archived_tdc.registered_d4.step.is_none()
        && archived_tdc.registered_d4.label == "registered-d4"
        && archived_tdc.registered_d4.dimension == 4
        && tdc_d4.inventory_stage == 16;
    let tdc_live_output_counts_match_kernel_extractor =
        tdc_step8.live_demand_outputs == step8_outputs && tdc_d4.live_demand_outputs == d4_outputs;
    let identification_gap_ids = vec![
        D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED.to_owned(),
        C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS.to_owned(),
        D1_INTENDED_CW_INSTANCE_GRAMMAR_NOT_OPERATIONAL.to_owned(),
        D2_TYPED_HOLE_INSTANTIATION_DOMAIN_NOT_OPERATIONAL.to_owned(),
        D3_UNIVALENT_DEMAND_ORBIT_QUOTIENT_NOT_OPERATIONAL.to_owned(),
        D4_DEFINITIONAL_CLOSURE_MEMBERSHIP_NOT_DECIDABLE.to_owned(),
    ];
    let mut certificate = DemandCompletenessCertificate {
        schema: DEMAND_COMPLETENESS_SCHEMA.to_owned(),
        date: DEMAND_COMPLETENESS_DATE.to_owned(),
        intended_generator_version: INTENDED_RULE_SEED_GENERATOR_VERSION.to_owned(),
        source_bindings,
        primary_gap_id:
            D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED.to_owned(),
        premises: GuardRailPremiseAudit {
            a1_cumulativity: "sealed kernels are never retracted".to_owned(),
            a2_monotone_derivability: "D is extensive, idempotent, and monotone".to_owned(),
            a3_demand_schema: "each depth-two window has a finite set C(W) of coherence demands, including clause schemes with future holes".to_owned(),
            a4_depth_two_admissibility: "an admissible candidate must discharge every undischarged demand of the current window".to_owned(),
            theorem_12_claim: "under A1-A4 and J1-J3, O(16) is empty".to_owned(),
            falsifier_f1: "an exhibited demanded-but-underdetermined (S15,S14) instance refutes Theorem 12 as used".to_owned(),
            a3_asserts_finite_cw_exists: true,
            a3_supplies_constructive_instance_generator: false,
            theorem_12_instance_granularity_machine_checked: false,
        },
        intended_window_specifications,
        coarse_ladder,
        coarse_ladder_reproduced,
        stage3_pre_jurisdiction_wrinkle_preserved,
        extractor_identification,
        extractor_identified_with_intended_cw: false,
        identification_gap_ids,
        extractor_relativity_witness,
        coarse_engine_o16_empty: live_o16.o16_empty,
        semantic_instance_granular_o16_empty: None,
        f1: F1Audit {
            s15_s14_intended_instances_enumerated: false,
            every_intended_instance_derivability_decided: false,
            f1_executable: false,
            demanded_but_underdetermined_instance: None,
            f1_triggered: false,
            f1_excluded: false,
            theorem_12_refuted: false,
            theorem_12_proved: false,
            focus_gating_restored: false,
            status: "not_triggered_not_excluded_and_not_executable: no intended demand instance was operationally generated; Theorem 12 is neither proved nor refuted and focus gating is not restored".to_owned(),
        },
        f5: F5Audit {
            a3_finiteness_asserted: true,
            finite_complete_cw_machine_verified: false,
            nontermination_or_nonfiniteness_witness: None,
            f5_triggered: false,
            f5_excluded: false,
            status: "not_triggered_and_not_excluded: A3 asserts finiteness, but no complete C(W) instance generator was available to execute a finiteness check".to_owned(),
        },
        provenance_diagnostic: ProvenanceDiagnosticAudit {
            tdc_archive_definition_replayed: true,
            tdc_archive_result_digest: archived_tdc.result_digest.clone(),
            step8_joined_to_tdc_subject,
            registered_d4_joined_to_tdc_subject,
            registered_d4_label: archived_tdc.registered_d4.label.clone(),
            registered_d4_basis_derivation_hash: archived_tdc
                .registered_d4
                .basis_derivation_hash
                .clone(),
            step8_extractor_live_outputs: step8_outputs,
            registered_d4_extractor_live_outputs: d4_outputs,
            tdc_live_output_counts_match_kernel_extractor,
            extractor_relative_one_vs_zero_reproduced: step8_joined_to_tdc_subject
                && registered_d4_joined_to_tdc_subject
                && tdc_live_output_counts_match_kernel_extractor
                && step8_outputs == 1
                && d4_outputs == 0,
            intended_demand_completeness_proved: false,
            registered_d4_certified_demand_orphaned: false,
            status: "conditional extractor-relative 1-vs-0 diagnostic; not an intended-schema obstruction".to_owned(),
        },
        outcome: "named_operational_cw_gap".to_owned(),
        remaining_obligations: vec![
            "define typed constructors for intended unary, binary, and future-hole demand instances from a historical depth-two window".to_owned(),
            "define weakening, substitution, and univalent equality on demand instances and prove a finite orbit quotient".to_owned(),
            "implement or prove a complete D-membership/derivability decision for every intended demand orbit".to_owned(),
            "only then compare the intended inventory with the package extractor and rerun F1 on (S15,S14)".to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> DemandCompletenessReplay {
    DemandCompletenessReplay {
        valid: false,
        coarse_ladder_reproduced: false,
        stage3_wrinkle_preserved: false,
        intended_identification_proved: false,
        semantic_o16_decided: false,
        f1_triggered: false,
        f5_triggered: false,
        theorem_12_proved: false,
        theorem_12_refuted: false,
        d4_demand_orphan_certified: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against(
    certificate: &DemandCompletenessCertificate,
    expected: &DemandCompletenessCertificate,
) -> DemandCompletenessReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    DemandCompletenessReplay {
        valid: errors.is_empty(),
        coarse_ladder_reproduced: certificate.coarse_ladder_reproduced,
        stage3_wrinkle_preserved: certificate.stage3_pre_jurisdiction_wrinkle_preserved,
        intended_identification_proved: certificate.extractor_identified_with_intended_cw,
        semantic_o16_decided: certificate.semantic_instance_granular_o16_empty.is_some(),
        f1_triggered: certificate.f1.f1_triggered,
        f5_triggered: certificate.f5.f5_triggered,
        theorem_12_proved: certificate.f1.theorem_12_proved,
        theorem_12_refuted: certificate.f1.theorem_12_refuted,
        d4_demand_orphan_certified: certificate
            .provenance_diagnostic
            .registered_d4_certified_demand_orphaned,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_demand_completeness_certificate(
    certificate: &DemandCompletenessCertificate,
) -> DemandCompletenessReplay {
    match build_demand_completeness_certificate() {
        Ok(expected) => replay_against(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn demand_completeness_json_pretty() -> Result<String, DemandCompletenessError> {
    serde_json::to_string_pretty(&build_demand_completeness_certificate()?)
        .map(|json| format!("{json}\n"))
        .map_err(|error| DemandCompletenessError::Json(error.to_string()))
}

pub fn replay_demand_completeness_json(json: &str) -> DemandCompletenessReplay {
    let raw: serde_json::Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(error) => return failed_replay(format!("invalid JSON: {error}")),
    };
    let certificate: DemandCompletenessCertificate = match serde_json::from_str(json) {
        Ok(certificate) => certificate,
        Err(error) => return failed_replay(format!("certificate shape error: {error}")),
    };
    let typed = serde_json::to_value(&certificate).expect("certificate serializes");
    if raw != typed {
        return failed_replay("JSON contains unknown, duplicate, or ignored structure");
    }
    replay_demand_completeness_certificate(&certificate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debt_guard::PACKAGE_NAMES;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ValuePathItem {
        Field(String),
        Index(usize),
    }

    fn redigest(certificate: &mut DemandCompletenessCertificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    fn collect_paths(
        value: &serde_json::Value,
        path: &mut Vec<ValuePathItem>,
        leaves: &mut Vec<Vec<ValuePathItem>>,
        arrays: &mut Vec<Vec<ValuePathItem>>,
    ) {
        match value {
            serde_json::Value::Object(object) => {
                for (field, child) in object {
                    path.push(ValuePathItem::Field(field.clone()));
                    collect_paths(child, path, leaves, arrays);
                    path.pop();
                }
            }
            serde_json::Value::Array(array) => {
                arrays.push(path.clone());
                for (index, child) in array.iter().enumerate() {
                    path.push(ValuePathItem::Index(index));
                    collect_paths(child, path, leaves, arrays);
                    path.pop();
                }
            }
            _ => leaves.push(path.clone()),
        }
    }

    fn value_at_path_mut<'a>(
        mut value: &'a mut serde_json::Value,
        path: &[ValuePathItem],
    ) -> &'a mut serde_json::Value {
        for item in path {
            value = match item {
                ValuePathItem::Field(field) => value
                    .as_object_mut()
                    .and_then(|object| object.get_mut(field))
                    .expect("recorded object path"),
                ValuePathItem::Index(index) => value
                    .as_array_mut()
                    .and_then(|array| array.get_mut(*index))
                    .expect("recorded array path"),
            };
        }
        value
    }

    fn assert_redigested_rejected(
        value: serde_json::Value,
        expected: &DemandCompletenessCertificate,
    ) {
        let Ok(mut mutation): Result<DemandCompletenessCertificate, _> =
            serde_json::from_value(value)
        else {
            return;
        };
        redigest(&mut mutation);
        assert!(!replay_against(&mutation, expected).valid);
    }

    fn assert_redigested_certificate_rejected(
        mut mutation: DemandCompletenessCertificate,
        expected: &DemandCompletenessCertificate,
    ) {
        redigest(&mut mutation);
        assert!(!replay_against(&mutation, expected).valid);
    }

    #[test]
    fn gap_outcome_preserves_ladder_wrinkle_and_conditionality() {
        let certificate = build_demand_completeness_certificate().expect("certificate");
        assert!(certificate.coarse_ladder_reproduced);
        assert!(certificate.stage3_pre_jurisdiction_wrinkle_preserved);
        assert!(!certificate.extractor_identified_with_intended_cw);
        assert_eq!(certificate.semantic_instance_granular_o16_empty, None);
        assert!(!certificate.f1.f1_triggered);
        assert!(!certificate.f1.f1_excluded);
        assert!(!certificate.f1.f1_executable);
        assert!(!certificate.f1.theorem_12_proved);
        assert!(!certificate.f1.theorem_12_refuted);
        assert!(!certificate.f1.focus_gating_restored);
        assert!(!certificate.f5.f5_triggered);
        assert!(!certificate.f5.f5_excluded);
        assert!(!certificate.f5.finite_complete_cw_machine_verified);
        assert_eq!(certificate.f5.nontermination_or_nonfiniteness_witness, None);
        assert_eq!(
            certificate.primary_gap_id,
            D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED
        );
        assert_eq!(certificate.outcome, "named_operational_cw_gap");
        assert!(
            certificate
                .coarse_ladder
                .iter()
                .map(|row| row.stage)
                .eq(1_u32..=16_u32)
        );
        assert!(
            certificate
                .intended_window_specifications
                .iter()
                .all(
                    |specification| specification.window.newest_step + 1 == specification.stage
                        && specification.window.older_step + 2 == specification.stage
                        && specification.rule_seeds_are_pre_instance_overapproximation
                )
        );
        assert!(
            certificate
                .intended_window_specifications
                .iter()
                .all(|specification| specification.generated_intended_demand_instances.is_none())
        );
        assert!(
            certificate
                .provenance_diagnostic
                .extractor_relative_one_vs_zero_reproduced
        );
        assert!(
            !certificate
                .provenance_diagnostic
                .registered_d4_certified_demand_orphaned
        );
    }

    #[test]
    fn intended_rule_seed_generator_contains_no_extractor_package_labels() {
        let certificate = build_demand_completeness_certificate().expect("certificate");
        let intended_json = serde_json::to_string(&certificate.intended_window_specifications)
            .expect("intended specs serialize");
        for package in PACKAGE_NAMES {
            assert!(
                !intended_json.contains(package),
                "intended generator mirrored {package}"
            );
        }
    }

    #[test]
    fn all_empty_timeline_is_a_machine_witness_of_extractor_relativity() {
        let certificate = build_demand_completeness_certificate().expect("certificate");
        let witness = &certificate.extractor_relativity_witness;
        assert!(witness.all_orbit_inventories_empty);
        assert_eq!(witness.computed_inventory_count, 16);
        assert!(witness.exactly_sixteen_inventories_computed);
        assert!(witness.canonical_timeline_has_nonempty_package_sets);
        assert!(witness.supplied_timeline_differs_from_canonical);
        assert!(witness.all_j2_marked_kernel_verified);
        assert!(witness.all_j3_marked_kernel_verified);
        assert!(witness.all_locality_marked_kernel_verified);
        assert!(witness.demonstrates_evidence_is_relative_to_supplied_extractor_timeline);
        assert!(!witness.proves_intended_schema_completeness);
        assert!(!witness.exhibits_f1_demanded_but_underdetermined_instance);
    }

    #[test]
    fn every_scalar_and_nonempty_vector_mutation_fails_after_redigest() {
        let expected = build_demand_completeness_certificate().expect("certificate");
        let original = serde_json::to_value(&expected).expect("certificate projects");
        let mut leaves = Vec::new();
        let mut arrays = Vec::new();
        collect_paths(&original, &mut Vec::new(), &mut leaves, &mut arrays);
        let digest_path = [ValuePathItem::Field("result_digest".to_owned())];
        for path in leaves {
            if path == digest_path {
                continue;
            }
            let mut mutation = original.clone();
            match value_at_path_mut(&mut mutation, &path) {
                serde_json::Value::Bool(value) => *value = !*value,
                serde_json::Value::Number(value) => {
                    let value = value.as_u64().expect("unsigned number");
                    *value_at_path_mut(&mut mutation, &path) = serde_json::Value::from(value + 1);
                }
                serde_json::Value::String(value) => value.push_str(":mutated"),
                value @ serde_json::Value::Null => *value = serde_json::Value::Bool(true),
                serde_json::Value::Array(_) | serde_json::Value::Object(_) => unreachable!(),
            }
            assert_redigested_rejected(mutation, &expected);
        }
        for path in arrays {
            let items = value_at_path_mut(&mut original.clone(), &path)
                .as_array()
                .expect("array")
                .clone();
            if items.is_empty() {
                continue;
            }
            let mut deletion = original.clone();
            value_at_path_mut(&mut deletion, &path)
                .as_array_mut()
                .unwrap()
                .remove(0);
            assert_redigested_rejected(deletion, &expected);
            let mut duplicate = original.clone();
            value_at_path_mut(&mut duplicate, &path)
                .as_array_mut()
                .unwrap()
                .push(items[0].clone());
            assert_redigested_rejected(duplicate, &expected);
            if let Some(different_index) = items.iter().position(|item| item != &items[0]) {
                let mut reorder = original.clone();
                value_at_path_mut(&mut reorder, &path)
                    .as_array_mut()
                    .unwrap()
                    .swap(0, different_index);
                assert_redigested_rejected(reorder, &expected);
            }
        }
    }

    #[test]
    fn important_empty_and_null_inventories_reject_typed_insertions_after_redigest() {
        let expected = build_demand_completeness_certificate().expect("certificate");

        let mut intended_instance = expected.clone();
        intended_instance.intended_window_specifications[0].generated_intended_demand_instances =
            Some(vec!["fabricated-typed-demand-instance".to_owned()]);
        assert_redigested_certificate_rejected(intended_instance, &expected);

        let mut identification = expected.clone();
        identification.extractor_identification[0].intended_demand_instance_count = Some(1);
        identification.extractor_identification[0].extractor_equals_intended = Some(true);
        assert_redigested_certificate_rejected(identification, &expected);

        let mut semantic_o16 = expected.clone();
        semantic_o16.semantic_instance_granular_o16_empty = Some(true);
        assert_redigested_certificate_rejected(semantic_o16, &expected);

        let mut f1_witness = expected.clone();
        f1_witness.f1.demanded_but_underdetermined_instance =
            Some("fabricated-S15-S14-instance".to_owned());
        assert_redigested_certificate_rejected(f1_witness, &expected);

        let mut f5_witness = expected.clone();
        f5_witness.f5.nontermination_or_nonfiniteness_witness =
            Some("fabricated-nontermination-witness".to_owned());
        assert_redigested_certificate_rejected(f5_witness, &expected);

        let stage16 = expected
            .coarse_ladder
            .iter()
            .position(|row| row.stage == 16)
            .expect("stage 16");
        for field in 0..3 {
            let mut empty_package_inventory = expected.clone();
            let row = &mut empty_package_inventory.coarse_ladder[stage16];
            match field {
                0 => row.expected_packages.push("fabricated-package".to_owned()),
                1 => row
                    .engine_required_packages
                    .push("fabricated-package".to_owned()),
                2 => row
                    .extractor_live_packages
                    .push("fabricated-package".to_owned()),
                _ => unreachable!(),
            }
            assert_redigested_certificate_rejected(empty_package_inventory, &expected);
        }
    }

    #[test]
    fn strict_json_and_explicit_promotions_fail_closed() {
        let json = demand_completeness_json_pretty().expect("json");
        assert!(replay_demand_completeness_json(&json).valid);
        let unknown = json.replacen("{\n", "{\n  \"unknown\": true,\n", 1);
        assert!(!replay_demand_completeness_json(&unknown).valid);
        let duplicate = json.replacen(
            "\"schema\": \"demand-completeness-v1\",",
            "\"schema\": \"demand-completeness-v1\",\n  \"schema\": \"demand-completeness-v1\",",
            1,
        );
        assert!(!replay_demand_completeness_json(&duplicate).valid);
        let unknown_deep = json.replacen(
            "\"premises\": {",
            "\"premises\": {\n    \"unknown\": true,",
            1,
        );
        assert!(!replay_demand_completeness_json(&unknown_deep).valid);
        let duplicate_deep = json.replacen(
            "\"a3_asserts_finite_cw_exists\": true,",
            "\"a3_asserts_finite_cw_exists\": true,\n    \"a3_asserts_finite_cw_exists\": true,",
            1,
        );
        assert!(!replay_demand_completeness_json(&duplicate_deep).valid);

        let expected = build_demand_completeness_certificate().expect("certificate");
        let mut promoted = expected.clone();
        promoted.extractor_identified_with_intended_cw = true;
        promoted.semantic_instance_granular_o16_empty = Some(true);
        promoted.f1.f1_excluded = true;
        promoted
            .provenance_diagnostic
            .registered_d4_certified_demand_orphaned = true;
        redigest(&mut promoted);
        assert!(!replay_against(&promoted, &expected).valid);
    }
}
