//! Typed, evidence-bound transitive isolation for the T-BI pre-seal issuer.
//!
//! V1 recorded a useful closed capability diagram, but its intermediate
//! receipts were caller-supplied pairs of strings.  This additive successor
//! has no receipt input.  It derives every phase node from the exact v5
//! sequence and the candidate/prefix inputs that reissue that sequence,
//! recomputes the package and sequence commitments, and obtains capabilities
//! solely by exhaustive matching on the typed operation enum below.

use crate::act_local_semantic_provenance_v4::HISTORICAL_ROLE_KINDS;
use crate::act_local_semantic_provenance_v5::{
    ACT_LOCAL_SEMANTIC_PROVENANCE_V5_SCHEMA, ActLocalSemanticProvenanceV5Certificate,
    ActLocalSemanticSequenceV5, T_BI_B2_A3_CAPABILITY_ID, T_BI_B2_EQUIVALENCE_CLOSURE_ID,
    T_BI_B2_STAGE1_R1_ID, T_BI_B2_UNIFIED_QUOTIENT_ID, V5AnchorDisposition, V5ConstructorSurface,
    V5ExactA3OrbitDisposition, V5FreshMarginalityDisposition, V5PhaseReceipt, V5RelationRule,
    V5RoleResolution, V5SemanticFamilySource, V5UnifiedBaseEqualityDecision, V5UnifiedBaseRelation,
    V5UnifiedEqualityDecision, V5UnifiedEqualityRelation, V5UnifiedEquivalenceBaseRule,
    V5UnifiedFamilyPresentation, V5UnifiedMemberDisposition, V5UnifiedSurface,
    V5UnifiedSurfaceMember, replay_act_local_semantic_sequence_v5,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3DemandOutputType, A3HistoricalWindow, generate_a3_window_for_exact_prefix_unbounded,
};
use pen_eval::semantic_provenance::LocalRole;
use pen_eval::typed_families::{
    CandidateExtractionOutcome, extract_candidate_families, predecessor_closure,
};
use pen_type::elaborate::{SealedSignature, candidate_hash, elaborate_telescope};
use pen_type::equality::univalent_equality;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const T_BI_INTRINSIC_ISOLATION_V2_SCHEMA: &str = "t-bi-intrinsic-typed-phase-dag-isolation-v2";
pub const T_BI_INTRINSIC_ISOLATION_V2_DATE: &str = "2026-07-22";
pub const T_BI_B3_V2_THEOREM_ID: &str =
    "T-BI-B3-v2-typed-evidence-bound-transitive-pre-seal-isolation";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_INTRINSIC_ISOLATION_V2_SCHEMA, domain, value))
        .expect("T-BI-B3 v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn v5_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(ACT_LOCAL_SEMANTIC_PROVENANCE_V5_SCHEMA, domain, value))
        .expect("v5 commitment projection serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntrinsicCapabilityV2 {
    CandidateTerm,
    SealedPrefix,
    FrozenDeclarationGrammar,
    TermTypeEqualityKernel,
    CoreNaturalFamily,
    Schema2Normalization,
    UnifiedFamilyQuotient,
    PredecessorEqualityAndMarginality,
    GenericR1Package,
    R2GeneratedMembership,
    OrdinaryNaturalityKernel,
    CubicalPathKernel,
    ExactA3RequiredTypeGrammar,
    ExactA3NoOutputWitnessTheorem,
    ExactFamilyRoleRelation,
    FamilyAnchorInjection,
    SpecialCaseTheorem,
    ContentHashing,
    Archive,
    StructuralNu,
    Bar,
    Verdict,
    EnactedFuture,
}

impl IntrinsicCapabilityV2 {
    fn forbidden() -> BTreeSet<Self> {
        [
            Self::Archive,
            Self::StructuralNu,
            Self::Bar,
            Self::Verdict,
            Self::EnactedFuture,
        ]
        .into_iter()
        .collect()
    }
}

/// This enum is the capability boundary.  Adding an operation is a compile
/// error here until its capability set is classified.  Callers cannot supply
/// or override capabilities.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntrinsicPhaseOperationV2 {
    CandidateInput,
    ExactPrefix,
    FrozenDeclarationReissue,
    CandidateElaborationAndExtraction,
    CoreTermFamilyNaturality,
    ExprToSchema2Bridge,
    UnifiedRepresentationQuotient,
    PredecessorMarginalitySweep,
    GenericR1PackageTheorem,
    R2GeneratedInstanceMembership,
    OrdinaryRegistryProof,
    CubicalProjectionAndQuotient,
    ExactA3GeneratorCapability,
    RoleKindRelationInduction,
    FamilyAnchorResolution,
    StageSpecialCaseTheorems,
    PackageSeal,
    SequenceSeal,
    // These post-seal operations are part of the closed operation enum so a
    // mutation cannot smuggle their capability in as caller data.  Canonical
    // pre-seal issuance never emits them.
    ArchiveComparatorRead,
    StructuralNuComparatorRead,
    BarRead,
    VerdictRead,
    EnactedFutureRead,
}

impl IntrinsicPhaseOperationV2 {
    fn direct_capabilities(self) -> Vec<IntrinsicCapabilityV2> {
        use IntrinsicCapabilityV2 as C;
        use IntrinsicPhaseOperationV2 as O;
        match self {
            O::CandidateInput => vec![C::CandidateTerm],
            O::ExactPrefix => vec![C::SealedPrefix],
            O::FrozenDeclarationReissue => vec![C::FrozenDeclarationGrammar],
            O::CandidateElaborationAndExtraction => vec![C::TermTypeEqualityKernel],
            O::CoreTermFamilyNaturality => vec![C::CoreNaturalFamily],
            O::ExprToSchema2Bridge => vec![C::Schema2Normalization],
            O::UnifiedRepresentationQuotient => vec![C::UnifiedFamilyQuotient],
            O::PredecessorMarginalitySweep => vec![C::PredecessorEqualityAndMarginality],
            O::GenericR1PackageTheorem => vec![C::GenericR1Package],
            O::R2GeneratedInstanceMembership => vec![C::R2GeneratedMembership],
            O::OrdinaryRegistryProof => vec![C::OrdinaryNaturalityKernel],
            O::CubicalProjectionAndQuotient => vec![C::CubicalPathKernel],
            O::ExactA3GeneratorCapability => vec![
                C::ExactA3RequiredTypeGrammar,
                C::ExactA3NoOutputWitnessTheorem,
            ],
            O::RoleKindRelationInduction => vec![C::ExactFamilyRoleRelation],
            O::FamilyAnchorResolution => vec![C::FamilyAnchorInjection],
            O::StageSpecialCaseTheorems => vec![C::SpecialCaseTheorem],
            O::PackageSeal | O::SequenceSeal => vec![C::ContentHashing],
            O::ArchiveComparatorRead => vec![C::Archive],
            O::StructuralNuComparatorRead => vec![C::StructuralNu],
            O::BarRead => vec![C::Bar],
            O::VerdictRead => vec![C::Verdict],
            O::EnactedFutureRead => vec![C::EnactedFuture],
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum A3RequiredOutputConstructorV2 {
    ActionAt,
    ChronologicalInteraction,
    ContractibleOpenBox,
    StructuralCompletion,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3RequiredOutputCapabilityRowV2 {
    pub scheme_id: String,
    pub constructor: A3RequiredOutputConstructorV2,
    pub required_output: serde_json::Value,
    pub required_output_hash: String,
    pub carries_constructed_expr_witness: bool,
    pub carries_typed_realizer_token: bool,
    pub carries_semantic_family_witness: bool,
    pub row_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct A3GeneratorCapabilityProofV2 {
    pub stage: u32,
    pub prefix_signature_digest: String,
    pub window_derivation_hash: String,
    pub scheme_count: usize,
    pub instance_count: usize,
    pub orbit_count: usize,
    pub independently_exported_orbit_count: usize,
    pub rows: Vec<A3RequiredOutputCapabilityRowV2>,
    pub exact_window_field_surface_destructured: bool,
    pub exact_output_enum_surface_exhausted: bool,
    pub source_expressions_classified_as_inputs_not_outputs: bool,
    pub generator_api_has_no_constructed_output_witness_variant: bool,
    pub every_required_output_is_type_only: bool,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicIsolationGapV2 {
    pub stage: Option<u32>,
    pub operation: IntrinsicPhaseOperationV2,
    pub code: String,
    pub detail: String,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicPhaseReceiptV2 {
    pub node_id: String,
    pub stage: Option<u32>,
    pub operation: IntrinsicPhaseOperationV2,
    pub predecessor_node_ids: Vec<String>,
    pub predecessor_receipt_hashes: Vec<String>,
    pub predecessor_output_hashes: Vec<String>,
    pub evidence_hashes: Vec<String>,
    pub output_hash: String,
    pub direct_capabilities: Vec<IntrinsicCapabilityV2>,
    pub fully_bound: bool,
    pub named_gap_codes: Vec<String>,
    pub receipt_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicCapabilityClosureRowV2 {
    pub node_id: String,
    pub transitive_capabilities: Vec<IntrinsicCapabilityV2>,
    pub forbidden_capabilities: Vec<IntrinsicCapabilityV2>,
    pub isolated: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiIntrinsicIsolationV2Token {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub bound_intrinsic_sequence_schema: String,
    pub bound_sequence_derivation_hash: String,
    pub bound_package_derivation_hashes: Vec<String>,
    pub bound_intrinsic_sequence_seal: String,
    pub a3_generator_capability_proofs: Vec<A3GeneratorCapabilityProofV2>,
    pub phase_receipts: Vec<IntrinsicPhaseReceiptV2>,
    pub closure_rows: Vec<IntrinsicCapabilityClosureRowV2>,
    pub named_gaps: Vec<IntrinsicIsolationGapV2>,
    pub exact_fifteen_stage_surface: bool,
    pub exact_typed_phase_surface: bool,
    pub exact_package_commitments_recomputed: bool,
    pub exact_sequence_commitments_recomputed: bool,
    pub every_predecessor_hash_bound: bool,
    pub every_capability_derived_from_operation: bool,
    pub every_node_reaches_sequence_seal: bool,
    pub no_forbidden_capability_in_transitive_closure: bool,
    pub source_scan_used_as_proof: bool,
    pub runtime_self_report_used_as_proof: bool,
    pub synthetic_receipt_input_accepted: bool,
    pub transitive_call_graph_isolation_proved: bool,
    pub proof_scope: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiIntrinsicIsolationV2Error {
    #[error("T-BI-B3 v2 invalid input: {0}")]
    Input(String),
    #[error("T-BI-B3 v2 commitment binding failed: {0}")]
    Binding(String),
    #[error("T-BI-B3 v2 A3 capability proof failed: {0}")]
    A3(String),
}

fn receipt_hash(receipt: &IntrinsicPhaseReceiptV2) -> String {
    let mut projection = receipt.clone();
    projection.receipt_hash.clear();
    tagged_hash("typed-phase-receipt", &projection)
}

fn token_hash(token: &TBiIntrinsicIsolationV2Token) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("typed-transitive-isolation-token", &projection)
}

fn phase_receipt_hash(receipt: &V5PhaseReceipt) -> String {
    let mut projection = receipt.clone();
    projection.derivation_hash.clear();
    v5_hash("phase-receipt", &projection)
}

fn package_preseal_hash(package: &ActLocalSemanticProvenanceV5Certificate) -> String {
    v5_hash(
        "v5-package-preseal",
        &(
            package.stage,
            &package.candidate_hash,
            &package.predecessor_signature_digest,
            &package.bridges,
            &package.unified_quotient,
            &package.semantic_families,
            &package.role_resolutions,
            &package.exact_a3_capability,
            &package.finite_closure,
            &package.stage1_carrier_role_case_proofs,
            package.t_bi_b1_proved,
            package.t_bi_b2_proved,
        ),
    )
}

fn package_hash(package: &ActLocalSemanticProvenanceV5Certificate) -> String {
    let mut projection = package.clone();
    projection.derivation_hash.clear();
    v5_hash("act-local-semantic-v5-certificate", &projection)
}

fn sequence_seal_hash(package_hashes: &[String]) -> String {
    v5_hash("intrinsic-v5-package-sequence-seal", package_hashes)
}

fn sequence_hash(sequence: &ActLocalSemanticSequenceV5) -> String {
    let mut projection = sequence.clone();
    projection.derivation_hash.clear();
    v5_hash("semantic-v5-sequence", &projection)
}

fn stage_node_id(stage: u32, suffix: &str) -> String {
    format!("stage-{stage:02}/{suffix}")
}

fn phase<'a>(
    package: &'a ActLocalSemanticProvenanceV5Certificate,
    ordinal: u8,
    phase_id: &str,
) -> Result<&'a V5PhaseReceipt, TBiIntrinsicIsolationV2Error> {
    let matches = package
        .phase_receipts
        .iter()
        .filter(|receipt| receipt.ordinal == ordinal && receipt.phase_id == phase_id)
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
            "Stage {} has {} receipts for {ordinal}:{phase_id}",
            package.stage,
            matches.len()
        )));
    }
    let receipt = matches[0];
    if receipt.derivation_hash != phase_receipt_hash(receipt) {
        return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
            "Stage {} phase {phase_id} hash mismatch",
            package.stage
        )));
    }
    Ok(receipt)
}

fn gap(
    stage: Option<u32>,
    operation: IntrinsicPhaseOperationV2,
    code: &str,
    detail: &str,
    evidence: &[String],
) -> IntrinsicIsolationGapV2 {
    let evidence_hash = tagged_hash(
        "typed-isolation-gap-evidence",
        &(stage, operation, evidence),
    );
    IntrinsicIsolationGapV2 {
        stage,
        operation,
        code: code.to_owned(),
        detail: detail.to_owned(),
        evidence_hash,
    }
}

/// Exhaustive match over the required-output enum.  There is no wildcard:
/// adding a generator output constructor forces this theorem to be revisited.
fn output_constructor(output: &A3DemandOutputType) -> A3RequiredOutputConstructorV2 {
    match output {
        A3DemandOutputType::ActionAt {
            source_family: _,
            source_type: _,
        } => A3RequiredOutputConstructorV2::ActionAt,
        A3DemandOutputType::ChronologicalInteraction {
            older_family: _,
            older_type: _,
            newest_family: _,
            newest_type: _,
            interface_mode: _,
            interface_slot_map: _,
        } => A3RequiredOutputConstructorV2::ChronologicalInteraction,
        A3DemandOutputType::ContractibleOpenBox {
            dimension: _,
            boundary_families: _,
            boundary_types: _,
            path_witness_families: _,
        } => A3RequiredOutputConstructorV2::ContractibleOpenBox,
        A3DemandOutputType::StructuralCompletion {
            constructor: _,
            structural_snapshot: _,
        } => A3RequiredOutputConstructorV2::StructuralCompletion,
    }
}

fn prove_a3_generator_capability_v2(
    prefix: &SealedSignature,
    stage: u32,
) -> Result<A3GeneratorCapabilityProofV2, TBiIntrinsicIsolationV2Error> {
    let window = generate_a3_window_for_exact_prefix_unbounded(prefix, stage)
        .map_err(|error| TBiIntrinsicIsolationV2Error::A3(error.to_string()))?;

    // Exact destructuring deliberately has no `..`.  A new field on the
    // generator result therefore breaks compilation until its capability is
    // classified here.
    let A3HistoricalWindow {
        stage: window_stage,
        newest_step: _,
        older_step: _,
        orientation: _,
        structural_snapshot: _,
        typed_sources,
        constructor_evidence: _,
        base_rule_evidence: _,
        seed_dispositions: _,
        schemes,
        instances,
        orbits,
        focus_projection: _,
        finite_by_construction,
        every_instance_typed,
        window_derivation_hash,
    } = &window;

    let mut rows = Vec::new();
    for scheme in schemes {
        let constructor = output_constructor(&scheme.required_output);
        let required_output =
            serde_json::to_value(&scheme.required_output).expect("A3 required output serializes");
        let required_output_hash = tagged_hash(
            "a3-required-output-type-only",
            &(
                &scheme.scheme_id,
                &scheme.required_output,
                &scheme.formation_derivation_hash,
            ),
        );
        let mut row = A3RequiredOutputCapabilityRowV2 {
            scheme_id: scheme.scheme_id.clone(),
            constructor,
            required_output,
            required_output_hash,
            carries_constructed_expr_witness: false,
            carries_typed_realizer_token: false,
            carries_semantic_family_witness: false,
            row_hash: String::new(),
        };
        row.row_hash = tagged_hash("a3-required-output-capability-row", &row);
        rows.push(row);
    }
    rows.sort_by(|left, right| left.scheme_id.cmp(&right.scheme_id));
    let exact_window_field_surface_destructured = true;
    let exact_output_enum_surface_exhausted = true;
    let source_expressions_classified_as_inputs_not_outputs = typed_sources.iter().all(|source| {
        source.exported_public_clause
            && !source.anchor_id.is_empty()
            && !source.canonical_family_key.is_empty()
            && !source.public_eligibility_hash.is_empty()
            && !source.telescope_elaboration_hash.is_empty()
            && !source.typing_derivation_hash.is_empty()
    });
    let generator_api_has_no_constructed_output_witness_variant = rows.iter().all(|row| {
        !row.carries_constructed_expr_witness
            && !row.carries_typed_realizer_token
            && !row.carries_semantic_family_witness
    });
    let every_required_output_is_type_only =
        rows.len() == schemes.len() && generator_api_has_no_constructed_output_witness_variant;
    let proved = *window_stage == stage
        && *finite_by_construction
        && *every_instance_typed
        && exact_window_field_surface_destructured
        && exact_output_enum_surface_exhausted
        && source_expressions_classified_as_inputs_not_outputs
        && every_required_output_is_type_only;
    let independently_exported_orbit_count = orbits
        .iter()
        .filter(|orbit| orbit.independently_exported_demand_orbit)
        .count();
    let mut proof = A3GeneratorCapabilityProofV2 {
        stage,
        prefix_signature_digest: prefix.digest().to_owned(),
        window_derivation_hash: window_derivation_hash.clone(),
        scheme_count: schemes.len(),
        instance_count: instances.len(),
        orbit_count: orbits.len(),
        independently_exported_orbit_count,
        rows,
        exact_window_field_surface_destructured,
        exact_output_enum_surface_exhausted,
        source_expressions_classified_as_inputs_not_outputs,
        generator_api_has_no_constructed_output_witness_variant,
        every_required_output_is_type_only,
        proved,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("a3-generator-capability-proof", &proof);
    Ok(proof)
}

struct PhaseDagBuilderV2 {
    nodes: Vec<IntrinsicPhaseReceiptV2>,
    indices: BTreeMap<String, usize>,
    gaps: Vec<IntrinsicIsolationGapV2>,
}

impl PhaseDagBuilderV2 {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            indices: BTreeMap::new(),
            gaps: Vec::new(),
        }
    }

    fn add(
        &mut self,
        node_id: String,
        stage: Option<u32>,
        operation: IntrinsicPhaseOperationV2,
        predecessor_node_ids: Vec<String>,
        evidence_hashes: Vec<String>,
        bound_output_hash: Option<String>,
        gaps: Vec<IntrinsicIsolationGapV2>,
    ) -> Result<(), TBiIntrinsicIsolationV2Error> {
        if self.indices.contains_key(&node_id) {
            return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
                "duplicate typed phase node {node_id}"
            )));
        }
        if evidence_hashes
            .iter()
            .any(|hash| !hash.starts_with("blake3:"))
        {
            return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
                "node {node_id} has a non-derivation evidence hash"
            )));
        }
        let predecessors = predecessor_node_ids
            .iter()
            .map(|predecessor| {
                self.indices
                    .get(predecessor)
                    .copied()
                    .map(|index| &self.nodes[index])
                    .ok_or_else(|| {
                        TBiIntrinsicIsolationV2Error::Binding(format!(
                            "node {node_id} references non-predecessor {predecessor}"
                        ))
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let predecessor_receipt_hashes = predecessors
            .iter()
            .map(|predecessor| predecessor.receipt_hash.clone())
            .collect::<Vec<_>>();
        let predecessor_output_hashes = predecessors
            .iter()
            .map(|predecessor| predecessor.output_hash.clone())
            .collect::<Vec<_>>();
        let output_hash = bound_output_hash.unwrap_or_else(|| {
            tagged_hash(
                "typed-phase-output",
                &(
                    &node_id,
                    stage,
                    operation,
                    &predecessor_output_hashes,
                    &evidence_hashes,
                ),
            )
        });
        if !output_hash.starts_with("blake3:") {
            return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
                "node {node_id} has a non-derivation output"
            )));
        }
        let named_gap_codes = gaps.iter().map(|gap| gap.code.clone()).collect::<Vec<_>>();
        let mut receipt = IntrinsicPhaseReceiptV2 {
            node_id: node_id.clone(),
            stage,
            operation,
            predecessor_node_ids,
            predecessor_receipt_hashes,
            predecessor_output_hashes,
            evidence_hashes,
            output_hash,
            direct_capabilities: operation.direct_capabilities(),
            fully_bound: named_gap_codes.is_empty(),
            named_gap_codes,
            receipt_hash: String::new(),
        };
        receipt.receipt_hash = receipt_hash(&receipt);
        self.gaps.extend(gaps);
        self.indices.insert(node_id, self.nodes.len());
        self.nodes.push(receipt);
        Ok(())
    }
}

fn validate_v5_package_commitments(
    package: &ActLocalSemanticProvenanceV5Certificate,
) -> Result<(), TBiIntrinsicIsolationV2Error> {
    let expected_phases = [
        (1, "candidate-prefix"),
        (2, "frozen-declaration-and-v4-reissue"),
        (3, "elaboration-and-extraction"),
        (4, "expr-to-schema2"),
        (5, "exact-a3"),
        (6, "ordinary-and-cubical-theorem"),
        (7, "family-role-relation"),
        (8, "package-seal"),
    ];
    if package.phase_receipts.len() != expected_phases.len() {
        return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
            "Stage {} has {} phase receipts, expected {}",
            package.stage,
            package.phase_receipts.len(),
            expected_phases.len()
        )));
    }
    for (ordinal, phase_id) in expected_phases {
        phase(package, ordinal, phase_id)?;
    }
    let preseal = package_preseal_hash(package);
    if package.preseal_derivation_hash != preseal {
        return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
            "Stage {} pre-seal commitment mismatch",
            package.stage
        )));
    }
    let seal_phase = phase(package, 8, "package-seal")?;
    if seal_phase.evidence_hashes != vec![preseal] {
        return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
            "Stage {} package-seal phase is not bound to the pre-seal hash",
            package.stage
        )));
    }
    if package.derivation_hash != package_hash(package) {
        return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
            "Stage {} package derivation commitment mismatch",
            package.stage
        )));
    }
    Ok(())
}

fn validate_v5_sequence_commitments(
    sequence: &ActLocalSemanticSequenceV5,
) -> Result<(), TBiIntrinsicIsolationV2Error> {
    let package_hashes = sequence
        .packages
        .iter()
        .map(|package| package.derivation_hash.clone())
        .collect::<Vec<_>>();
    if sequence.exact_package_derivation_hashes != package_hashes {
        return Err(TBiIntrinsicIsolationV2Error::Binding(
            "sequence package-hash vector is not exact".to_owned(),
        ));
    }
    if sequence.intrinsic_sequence_seal != sequence_seal_hash(&package_hashes) {
        return Err(TBiIntrinsicIsolationV2Error::Binding(
            "intrinsic sequence seal mismatch".to_owned(),
        ));
    }
    if sequence.derivation_hash != sequence_hash(sequence) {
        return Err(TBiIntrinsicIsolationV2Error::Binding(
            "semantic sequence derivation commitment mismatch".to_owned(),
        ));
    }
    Ok(())
}

fn core_family_evidence(package: &ActLocalSemanticProvenanceV5Certificate) -> Vec<String> {
    package
        .semantic_families
        .iter()
        .filter(|family| matches!(family.source, V5SemanticFamilySource::FrozenV4 { .. }))
        .map(|family| {
            tagged_hash(
                "bound-core-family-evidence",
                &(
                    &family.family_id,
                    &family.source,
                    &family.term_or_bridge_derivation_hash,
                    family.typed_normalized_natural,
                    &family.derivation_hash,
                ),
            )
        })
        .collect()
}

fn unified_surface(presentation: &V5UnifiedFamilyPresentation) -> V5UnifiedSurface {
    match presentation {
        V5UnifiedFamilyPresentation::CoreExpr { .. } => V5UnifiedSurface::CoreExpr,
        V5UnifiedFamilyPresentation::OrdinarySchema2 { .. } => V5UnifiedSurface::OrdinarySchema2,
        V5UnifiedFamilyPresentation::CubicalPath { .. } => V5UnifiedSurface::CubicalPath,
    }
}

fn unified_equivalence_node_id(member: &V5UnifiedSurfaceMember) -> String {
    v5_hash(
        "unified-equivalence-node",
        &(
            member.stage,
            &member.family_id,
            &member.presentation_hash,
            &member.typing_normalization_naturality_hash,
        ),
    )
}

fn canonical_node_pair(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_owned(), right.to_owned())
    } else {
        (right.to_owned(), left.to_owned())
    }
}

fn base_rule_endpoints(rule: V5UnifiedEquivalenceBaseRule) -> (V5UnifiedSurface, V5UnifiedSurface) {
    match rule {
        V5UnifiedEquivalenceBaseRule::CoreUnivalentEquality => {
            (V5UnifiedSurface::CoreExpr, V5UnifiedSurface::CoreExpr)
        }
        V5UnifiedEquivalenceBaseRule::OrdinaryNormalizedShapeEquality => (
            V5UnifiedSurface::OrdinarySchema2,
            V5UnifiedSurface::OrdinarySchema2,
        ),
        V5UnifiedEquivalenceBaseRule::CubicalTypedEquality => {
            (V5UnifiedSurface::CubicalPath, V5UnifiedSurface::CubicalPath)
        }
        V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge => (
            V5UnifiedSurface::CoreExpr,
            V5UnifiedSurface::OrdinarySchema2,
        ),
    }
}

fn rule_accepts_surfaces(
    rule: V5UnifiedEquivalenceBaseRule,
    left: V5UnifiedSurface,
    right: V5UnifiedSurface,
) -> bool {
    let (expected_left, expected_right) = base_rule_endpoints(rule);
    (left == expected_left && right == expected_right)
        || (left == expected_right && right == expected_left)
}

fn cubical_pair_evidence_matches(evidence: &serde_json::Value, left: &str, right: &str) -> bool {
    let observed_left = evidence
        .get("left_family_hash")
        .and_then(serde_json::Value::as_str);
    let observed_right = evidence
        .get("right_family_hash")
        .and_then(serde_json::Value::as_str);
    (observed_left == Some(left) && observed_right == Some(right))
        || (observed_left == Some(right) && observed_right == Some(left))
}

/// Independently replay one base edge from its typed endpoints.  In
/// particular, the two cubical/non-cubical cases are discharged by the
/// exhaustive absence of a base-rule constructor with those endpoints; mere
/// presence of typed payloads is never treated as a disequality proof.
fn base_decision_replays(
    decision: &V5UnifiedBaseEqualityDecision,
    left: &V5UnifiedSurfaceMember,
    right: &V5UnifiedSurfaceMember,
) -> bool {
    let left_surface = unified_surface(&left.presentation);
    let right_surface = unified_surface(&right.presentation);
    let relation_has_valid_endpoints = match decision.relation {
        V5UnifiedBaseRelation::Equal { rule } => {
            rule_accepts_surfaces(rule, left_surface, right_surface)
        }
        V5UnifiedBaseRelation::Distinct | V5UnifiedBaseRelation::NamedResidual { .. } => true,
    };
    if decision.left_node_id != unified_equivalence_node_id(left)
        || decision.right_node_id != unified_equivalence_node_id(right)
        || decision.left_surface != left_surface
        || decision.right_surface != right_surface
        || !relation_has_valid_endpoints
        || !decision.proved
        || matches!(
            decision.relation,
            V5UnifiedBaseRelation::NamedResidual { .. }
        )
    {
        return false;
    }

    let semantic_replay = match (&left.presentation, &right.presentation) {
        (
            V5UnifiedFamilyPresentation::CoreExpr {
                canonical_normal_form: left_nf,
                parameter_sorts: left_params,
                generator_kernel_type: left_kernel_type,
                generator_role: left_role,
                ..
            },
            V5UnifiedFamilyPresentation::CoreExpr {
                canonical_normal_form: right_nf,
                parameter_sorts: right_params,
                generator_kernel_type: right_kernel_type,
                generator_role: right_role,
                ..
            },
        ) => {
            if left_params != right_params
                || left_kernel_type != right_kernel_type
                || left_role != right_role
            {
                decision.relation == V5UnifiedBaseRelation::Distinct
                    && decision.evidence
                        == serde_json::json!({
                            "procedure": "typed-core-judgement-boundary-separation",
                            "left_parameters": left_params,
                            "right_parameters": right_params,
                            "left_kernel_type": left_kernel_type,
                            "right_kernel_type": right_kernel_type,
                            "left_generator_role": left_role,
                            "right_generator_role": right_role,
                        })
            } else {
                let scope = left_params
                    .as_array()
                    .and_then(|parameters| u32::try_from(parameters.len()).ok())
                    .unwrap_or(u32::MAX);
                match univalent_equality(left_nf, right_nf, scope, 4096) {
                    Ok(witness) => {
                        let expected_relation = if witness.equal {
                            V5UnifiedBaseRelation::Equal {
                                rule: V5UnifiedEquivalenceBaseRule::CoreUnivalentEquality,
                            }
                        } else {
                            V5UnifiedBaseRelation::Distinct
                        };
                        decision.relation == expected_relation
                            && decision.evidence
                                == serde_json::json!({
                                    "procedure": "fresh-core-univalent-equality",
                                    "term_equality": witness,
                                    "kernel_type_compatible": true,
                                    "generator_role_identical": true,
                                })
                    }
                    Err(_) => false,
                }
            }
        }
        (
            V5UnifiedFamilyPresentation::OrdinarySchema2 {
                ordinary_shape_key: left_key,
                ..
            },
            V5UnifiedFamilyPresentation::OrdinarySchema2 {
                ordinary_shape_key: right_key,
                ..
            },
        ) => {
            let expected_relation = if left_key == right_key {
                V5UnifiedBaseRelation::Equal {
                    rule: V5UnifiedEquivalenceBaseRule::OrdinaryNormalizedShapeEquality,
                }
            } else {
                V5UnifiedBaseRelation::Distinct
            };
            decision.relation == expected_relation
                && decision.evidence
                    == serde_json::json!({
                        "procedure": "ordinary-normalized-family-shape-equality",
                        "left_shape_key": left_key,
                        "right_shape_key": right_key,
                    })
        }
        (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        ) => {
            if left.family_id == right.family_id
                && left.presentation_hash == right.presentation_hash
            {
                decision.relation
                    == V5UnifiedBaseRelation::Equal {
                        rule: V5UnifiedEquivalenceBaseRule::CubicalTypedEquality,
                    }
                    && decision.evidence
                        == serde_json::json!({
                            "procedure": "identical-typed-cubical-presentation-reflexivity",
                            "family_id": left.family_id,
                            "presentation_hash": left.presentation_hash,
                        })
            } else {
                let expected_relation = match decision
                    .evidence
                    .get("decision")
                    .and_then(serde_json::Value::as_str)
                {
                    Some("equal") => Some(V5UnifiedBaseRelation::Equal {
                        rule: V5UnifiedEquivalenceBaseRule::CubicalTypedEquality,
                    }),
                    Some("distinct_within_fragment") => Some(V5UnifiedBaseRelation::Distinct),
                    _ => None,
                };
                expected_relation.as_ref() == Some(&decision.relation)
                    && cubical_pair_evidence_matches(
                        &decision.evidence,
                        &left.family_id,
                        &right.family_id,
                    )
            }
        }
        (
            V5UnifiedFamilyPresentation::CoreExpr {
                extracted_family_id,
                ..
            },
            V5UnifiedFamilyPresentation::OrdinarySchema2 {
                source_core_family_ids,
                ..
            },
        )
        | (
            V5UnifiedFamilyPresentation::OrdinarySchema2 {
                source_core_family_ids,
                ..
            },
            V5UnifiedFamilyPresentation::CoreExpr {
                extracted_family_id,
                ..
            },
        ) => {
            let typed = left.typed_normalized_natural
                && right.typed_normalized_natural
                && !left.typing_normalization_naturality_hash.is_empty()
                && !right.typing_normalization_naturality_hash.is_empty();
            if !typed {
                false
            } else {
                let bridge_membership = source_core_family_ids.contains(extracted_family_id);
                let expected_relation = if bridge_membership {
                    V5UnifiedBaseRelation::Equal {
                        rule: V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge,
                    }
                } else {
                    V5UnifiedBaseRelation::Distinct
                };
                decision.relation == expected_relation
                    && decision.evidence
                        == serde_json::json!({
                            "procedure": "exact-expr-schema2-bridge-constructor",
                            "core_family_id": extracted_family_id,
                            "ordinary_source_core_family_ids": source_core_family_ids,
                            "bridge_membership": bridge_membership,
                        })
            }
        }
        (
            V5UnifiedFamilyPresentation::CoreExpr { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        )
        | (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::CoreExpr { .. },
        )
        | (
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
            V5UnifiedFamilyPresentation::CubicalPath { .. },
        )
        | (
            V5UnifiedFamilyPresentation::CubicalPath { .. },
            V5UnifiedFamilyPresentation::OrdinarySchema2 { .. },
        ) => {
            let typed = left.typed_normalized_natural
                && right.typed_normalized_natural
                && !left.typing_normalization_naturality_hash.is_empty()
                && !right.typing_normalization_naturality_hash.is_empty();
            typed
                && decision.relation == V5UnifiedBaseRelation::Distinct
                && decision.evidence
                    == serde_json::json!({
                        "procedure": "closed-base-rule-endpoint-induction",
                        "left_surface": left_surface,
                        "right_surface": right_surface,
                        "applicable_cross_surface_base_constructors": [],
                    })
        }
    };

    let mut projection = decision.clone();
    projection.derivation_hash.clear();
    semantic_replay
        && decision.derivation_hash == v5_hash("unified-base-equality-decision", &projection)
}

fn recompute_components(
    node_ids: &[String],
    decisions: &[V5UnifiedBaseEqualityDecision],
) -> BTreeMap<String, String> {
    let mut adjacency = node_ids
        .iter()
        .cloned()
        .map(|node| (node, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for decision in decisions {
        if matches!(decision.relation, V5UnifiedBaseRelation::Equal { .. }) {
            if let Some(neighbours) = adjacency.get_mut(&decision.left_node_id) {
                neighbours.insert(decision.right_node_id.clone());
            }
            if let Some(neighbours) = adjacency.get_mut(&decision.right_node_id) {
                neighbours.insert(decision.left_node_id.clone());
            }
        }
    }
    let mut components = BTreeMap::new();
    for start in node_ids {
        if components.contains_key(start) {
            continue;
        }
        let component_id = start.clone();
        let mut pending = vec![start.clone()];
        while let Some(node) = pending.pop() {
            if components
                .insert(node.clone(), component_id.clone())
                .is_some()
            {
                continue;
            }
            if let Some(neighbours) = adjacency.get(&node) {
                for neighbour in neighbours.iter().rev() {
                    if !components.contains_key(neighbour) {
                        pending.push(neighbour.clone());
                    }
                }
            }
        }
    }
    components
}

fn equivalence_closure_replays<'a>(
    package: &'a ActLocalSemanticProvenanceV5Certificate,
    predecessor_members: &'a [V5UnifiedSurfaceMember],
) -> (
    BTreeMap<String, String>,
    BTreeMap<String, &'a V5UnifiedSurfaceMember>,
    bool,
) {
    let closure = &package.unified_quotient.equivalence_closure;
    let visible_predecessors = predecessor_members
        .iter()
        .filter(|member| member.visible_to_later_predecessor_closure);
    let members = package
        .unified_quotient
        .current_members
        .iter()
        .chain(visible_predecessors)
        .collect::<Vec<_>>();
    let member_by_node = members
        .iter()
        .map(|member| (unified_equivalence_node_id(member), *member))
        .collect::<BTreeMap<_, _>>();
    let mut expected_nodes = member_by_node.keys().cloned().collect::<Vec<_>>();
    expected_nodes.sort();
    let expected_registry = vec![
        V5UnifiedEquivalenceBaseRule::CoreUnivalentEquality,
        V5UnifiedEquivalenceBaseRule::OrdinaryNormalizedShapeEquality,
        V5UnifiedEquivalenceBaseRule::CubicalTypedEquality,
        V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge,
    ];
    let expected_pairs = expected_nodes
        .iter()
        .enumerate()
        .flat_map(|(left_index, left)| {
            expected_nodes
                .iter()
                .skip(left_index + 1)
                .map(move |right| canonical_node_pair(left, right))
        })
        .collect::<BTreeSet<_>>();
    let observed_pairs = closure
        .base_decisions
        .iter()
        .map(|decision| canonical_node_pair(&decision.left_node_id, &decision.right_node_id))
        .collect::<BTreeSet<_>>();
    let base_decisions_replay = closure.base_decisions.iter().all(|decision| {
        member_by_node
            .get(&decision.left_node_id)
            .zip(member_by_node.get(&decision.right_node_id))
            .is_some_and(|(left, right)| base_decision_replays(decision, left, right))
    });
    let components = recompute_components(&expected_nodes, &closure.base_decisions);
    let residual_count = closure
        .base_decisions
        .iter()
        .filter(|decision| {
            matches!(
                decision.relation,
                V5UnifiedBaseRelation::NamedResidual { .. }
            ) || !decision.proved
        })
        .count();
    let no_cubical_cross_edge = closure.base_decisions.iter().all(|decision| {
        !matches!(decision.relation, V5UnifiedBaseRelation::Equal { .. })
            || decision.left_surface == decision.right_surface
            || (decision.left_surface != V5UnifiedSurface::CubicalPath
                && decision.right_surface != V5UnifiedSurface::CubicalPath)
    });
    let registry_cross_rules = expected_registry
        .iter()
        .copied()
        .filter(|rule| {
            let (left, right) = base_rule_endpoints(*rule);
            left != right
        })
        .collect::<Vec<_>>();
    let mut projection = closure.clone();
    projection.derivation_hash.clear();
    let exact = closure.theorem_id == T_BI_B2_EQUIVALENCE_CLOSURE_ID
        && member_by_node.len() == members.len()
        && closure.member_universe_node_ids == expected_nodes
        && closure.member_universe_digest
            == v5_hash(
                "closed-unified-equivalence-member-universe",
                &expected_nodes,
            )
        && closure.base_rule_registry == expected_registry
        && closure.base_rule_registry_digest
            == v5_hash(
                "closed-unified-equivalence-base-rule-registry",
                &expected_registry,
            )
        && registry_cross_rules == vec![V5UnifiedEquivalenceBaseRule::ExactExprSchema2Bridge]
        && closure.expected_base_pair_count == expected_pairs.len()
        && closure.base_decisions.len() == expected_pairs.len()
        && observed_pairs == expected_pairs
        && base_decisions_replay
        && closure.component_by_node == components
        && closure.exact_rule_registry_replayed
        && closure.only_cross_surface_constructor_is_exact_expr_schema2_bridge
        && closure.no_base_equality_edge_touches_cubical_and_noncubical
        && no_cubical_cross_edge
        && closure.every_base_pair_decided
        && closure.reflexive_symmetric_transitive_closure_computed
        && closure.named_residual_count == residual_count
        && residual_count == 0
        && closure.proved
        && closure.derivation_hash == v5_hash("closed-unified-equivalence-closure", &projection);
    (components, member_by_node, exact)
}

fn closed_decision_replays(
    decision: &V5UnifiedEqualityDecision,
    left: &V5UnifiedSurfaceMember,
    right: &V5UnifiedSurfaceMember,
    components: &BTreeMap<String, String>,
    closure_hash: &str,
    member_universe_digest: &str,
    base_rule_registry_digest: &str,
) -> bool {
    let left_node_id = unified_equivalence_node_id(left);
    let right_node_id = unified_equivalence_node_id(right);
    let left_component = components.get(&left_node_id);
    let right_component = components.get(&right_node_id);
    let expected_relation = if left_component.is_some() && left_component == right_component {
        V5UnifiedEqualityRelation::Equal
    } else {
        V5UnifiedEqualityRelation::Distinct
    };
    let expected_evidence = serde_json::json!({
        "closure_theorem_id": T_BI_B2_EQUIVALENCE_CLOSURE_ID,
        "closure_derivation_hash": closure_hash,
        "member_universe_digest": member_universe_digest,
        "base_rule_registry_digest": base_rule_registry_digest,
        "left_node_id": left_node_id,
        "right_node_id": right_node_id,
        "left_component": left_component,
        "right_component": right_component,
        "only_cross_surface_constructor_is_exact_expr_schema2_bridge": true,
        "no_base_equality_edge_touches_cubical_and_noncubical": true,
    });
    let mut projection = decision.clone();
    projection.derivation_hash.clear();
    decision.left_stage == left.stage
        && decision.left_family_id == left.family_id
        && decision.right_stage == right.stage
        && decision.right_family_id == right.family_id
        && decision.left_surface == unified_surface(&left.presentation)
        && decision.right_surface == unified_surface(&right.presentation)
        && decision.relation == expected_relation
        && decision.procedure == "closed-unified-equivalence-rule-induction-v1"
        && decision.evidence == expected_evidence
        && !decision.v4_marginality_label_used_as_proof
        && decision.proved
        && decision.derivation_hash == v5_hash("unified-family-equality-decision", &projection)
}

fn quotient_evidence(
    package: &ActLocalSemanticProvenanceV5Certificate,
    predecessor_members: &[V5UnifiedSurfaceMember],
) -> (Vec<String>, bool) {
    let quotient = &package.unified_quotient;
    let current_ids = quotient
        .current_members
        .iter()
        .map(|member| member.family_id.as_str())
        .collect::<BTreeSet<_>>();
    let ledger_ids = quotient
        .current_members
        .iter()
        .filter(|member| member.included_in_semantic_ledger)
        .map(|member| member.family_id.as_str())
        .collect::<BTreeSet<_>>();
    let semantic_ids = package
        .semantic_families
        .iter()
        .map(|family| family.family_id.as_str())
        .collect::<BTreeSet<_>>();
    let core_ids = quotient
        .current_members
        .iter()
        .filter_map(|member| {
            (member.included_in_semantic_ledger
                && matches!(
                    member.presentation,
                    V5UnifiedFamilyPresentation::CoreExpr { .. }
                ))
            .then_some(member.family_id.clone())
        })
        .collect::<Vec<_>>();
    let ordinary_ids = quotient
        .current_members
        .iter()
        .filter_map(|member| {
            (member.included_in_semantic_ledger
                && matches!(
                    member.presentation,
                    V5UnifiedFamilyPresentation::OrdinarySchema2 { .. }
                ))
            .then_some(member.family_id.clone())
        })
        .collect::<Vec<_>>();
    let cubical_ids = quotient
        .current_members
        .iter()
        .filter_map(|member| {
            (member.included_in_semantic_ledger
                && matches!(
                    member.presentation,
                    V5UnifiedFamilyPresentation::CubicalPath { .. }
                ))
            .then_some(member.family_id.clone())
        })
        .collect::<Vec<_>>();
    let member_hashes_exact = quotient.current_members.iter().all(|member| {
        let mut projection = member.clone();
        projection.derivation_hash.clear();
        let disposition_exact = match &member.disposition {
            V5UnifiedMemberDisposition::Active => member.included_in_semantic_ledger,
            V5UnifiedMemberDisposition::CoreReplacedByOrdinaryBridge { bridge_family_ids } => {
                !member.included_in_semantic_ledger
                    && member.visible_to_later_predecessor_closure
                    && !bridge_family_ids.is_empty()
            }
            V5UnifiedMemberDisposition::GeneratedUniformInstance => {
                !member.included_in_semantic_ledger && !member.visible_to_later_predecessor_closure
            }
            V5UnifiedMemberDisposition::EqualWithinCandidate {
                representative_family_id,
            } => {
                !member.included_in_semantic_ledger
                    && !member.visible_to_later_predecessor_closure
                    && current_ids.contains(representative_family_id.as_str())
            }
        };
        member.stage == package.stage
            && member.typed_normalized_natural
            && !member.presentation_hash.is_empty()
            && !member.typing_normalization_naturality_hash.is_empty()
            && disposition_exact
            && member.derivation_hash == v5_hash("unified-surface-member", &projection)
    });
    let (components, _member_by_node, equivalence_closure_exact) =
        equivalence_closure_replays(package, predecessor_members);
    let visible_predecessors = predecessor_members
        .iter()
        .filter(|member| member.visible_to_later_predecessor_closure)
        .collect::<Vec<_>>();
    let member_by_identity = quotient
        .current_members
        .iter()
        .chain(visible_predecessors.iter().copied())
        .map(|member| ((member.stage, member.family_id.as_str()), member))
        .collect::<BTreeMap<_, _>>();
    let identity_count_exact =
        member_by_identity.len() == quotient.current_members.len() + visible_predecessors.len();
    let decision_exact = |decision: &V5UnifiedEqualityDecision| {
        member_by_identity
            .get(&(decision.left_stage, decision.left_family_id.as_str()))
            .zip(member_by_identity.get(&(decision.right_stage, decision.right_family_id.as_str())))
            .is_some_and(|(left, right)| {
                closed_decision_replays(
                    decision,
                    left,
                    right,
                    &components,
                    &quotient.equivalence_closure.derivation_hash,
                    &quotient.equivalence_closure.member_universe_digest,
                    &quotient.equivalence_closure.base_rule_registry_digest,
                )
            })
    };
    let within_pairs = quotient
        .within_candidate_decisions
        .iter()
        .map(|decision| canonical_node_pair(&decision.left_family_id, &decision.right_family_id))
        .collect::<BTreeSet<_>>();
    let visible_predecessor_ids = predecessor_members
        .iter()
        .filter(|member| member.visible_to_later_predecessor_closure)
        .map(|member| (member.stage, member.family_id.as_str()))
        .collect::<BTreeSet<_>>();
    let predecessor_pairs = quotient
        .predecessor_decisions
        .iter()
        .map(|decision| {
            (
                decision.left_family_id.clone(),
                decision.right_stage,
                decision.right_family_id.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    let current_id_vector = current_ids.iter().copied().collect::<Vec<_>>();
    let expected_within_pairs = current_id_vector
        .iter()
        .enumerate()
        .flat_map(|(left_index, left)| {
            current_id_vector
                .iter()
                .skip(left_index + 1)
                .map(move |right| canonical_node_pair(left, right))
        })
        .collect::<BTreeSet<_>>();
    let expected_predecessor_pairs = current_ids
        .iter()
        .flat_map(|current| {
            visible_predecessor_ids
                .iter()
                .map(move |(stage, predecessor)| {
                    ((*current).to_owned(), *stage, (*predecessor).to_owned())
                })
        })
        .collect::<BTreeSet<_>>();
    let expected_within = quotient
        .current_members
        .len()
        .saturating_mul(quotient.current_members.len().saturating_sub(1))
        / 2;
    let expected_predecessor = quotient.current_members.len() * visible_predecessor_ids.len();
    let decisions_exact = equivalence_closure_exact
        && identity_count_exact
        && quotient.within_candidate_decisions.len() == expected_within
        && quotient.expected_within_candidate_pair_count == expected_within
        && within_pairs == expected_within_pairs
        && quotient.within_candidate_decisions.iter().all(|decision| {
            decision.left_stage == package.stage
                && decision.right_stage == package.stage
                && current_ids.contains(decision.left_family_id.as_str())
                && current_ids.contains(decision.right_family_id.as_str())
                && decision_exact(decision)
        })
        && quotient.predecessor_decisions.len() == expected_predecessor
        && quotient.expected_predecessor_pair_count == expected_predecessor
        && predecessor_pairs == expected_predecessor_pairs
        && quotient.predecessor_decisions.iter().all(|decision| {
            decision.left_stage == package.stage
                && current_ids.contains(decision.left_family_id.as_str())
                && visible_predecessor_ids
                    .contains(&(decision.right_stage, decision.right_family_id.as_str()))
                && decision_exact(decision)
        });
    let fresh_keys = quotient
        .fresh_marginality
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let marginality_exact = fresh_keys == ledger_ids
        && quotient
            .fresh_marginality
            .values()
            .all(|disposition| match disposition {
                V5FreshMarginalityDisposition::InternalIdentical {
                    equality_derivation_hash,
                    ..
                }
                | V5FreshMarginalityDisposition::InternalUnifiedPreimage {
                    equality_derivation_hash,
                    ..
                } => !equality_derivation_hash.is_empty(),
                V5FreshMarginalityDisposition::InternalDerivable {
                    instance_derivation_hash,
                    ..
                } => !instance_derivation_hash.is_empty(),
                V5FreshMarginalityDisposition::InternalR1FormationCompletionPackage {
                    package_derivation_hash,
                    ..
                } => !package_derivation_hash.is_empty(),
                V5FreshMarginalityDisposition::MarginalNoPreimage {
                    predecessor_comparison_digest,
                } => !predecessor_comparison_digest.is_empty(),
                V5FreshMarginalityDisposition::NamedResidual { .. } => false,
            })
        && package.semantic_families.iter().all(|family| {
            quotient.fresh_marginality.get(&family.family_id) == Some(&family.fresh_marginality)
        });
    let extracted_core_ids = quotient
        .current_members
        .iter()
        .filter_map(|member| match &member.presentation {
            V5UnifiedFamilyPresentation::CoreExpr {
                extracted_family_id,
                ..
            } => Some(extracted_family_id.as_str()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let extraction_inventory = quotient
        .fresh_core_extraction_family_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let r2_quotiented = quotient
        .r2_quotiented_core_family_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let inventories_exact = quotient.fresh_core_inventory_exact
        && quotient.fresh_ordinary_inventory_exact
        && quotient.fresh_cubical_inventory_exact
        && !quotient.fresh_core_inventory_digest.is_empty()
        && !quotient.fresh_ordinary_bridge_inventory_digest.is_empty()
        && !quotient.fresh_cubical_inventory_digest.is_empty()
        && extracted_core_ids.is_disjoint(&r2_quotiented)
        && extracted_core_ids
            .union(&r2_quotiented)
            .copied()
            .collect::<BTreeSet<_>>()
            == extraction_inventory;
    let surface_digests_exact = quotient.core_family_ids == core_ids
        && quotient.ordinary_family_ids == ordinary_ids
        && quotient.cubical_family_ids == cubical_ids
        && quotient.core_surface_digest
            == v5_hash("unified-core-surface", &quotient.core_family_ids)
        && quotient.ordinary_surface_digest
            == v5_hash("unified-ordinary-surface", &quotient.ordinary_family_ids)
        && quotient.cubical_surface_digest
            == v5_hash("unified-cubical-surface", &quotient.cubical_family_ids)
        && quotient.current_surface_digest
            == v5_hash("unified-current-surface", &quotient.current_members)
        && quotient.predecessor_surface_digest
            == v5_hash("unified-predecessor-surface", predecessor_members);
    let mut quotient_projection = quotient.clone();
    quotient_projection.derivation_hash.clear();
    let quotient_hash_exact =
        quotient.derivation_hash == v5_hash("unified-equality-quotient", &quotient_projection);
    let mut closure_projection = package.finite_closure.clone();
    closure_projection.derivation_hash.clear();
    let closure_exact = package.finite_closure.derivation_hash
        == v5_hash("finite-semantic-closure-proof", &closure_projection)
        && package.finite_closure.unified_quotient_derivation_hash == quotient.derivation_hash
        && package.finite_closure.core_surface_digest == quotient.core_surface_digest
        && package.finite_closure.ordinary_surface_digest == quotient.ordinary_surface_digest
        && package.finite_closure.cubical_surface_digest == quotient.cubical_surface_digest
        && package.finite_closure.unified_quotient_complete;
    let exact = quotient.theorem_id == T_BI_B2_UNIFIED_QUOTIENT_ID
        && quotient.stage == package.stage
        && quotient.predecessor_signature_digest == package.predecessor_signature_digest
        && !quotient.predecessor_closure_digest.is_empty()
        && current_ids.len() == quotient.current_members.len()
        && ledger_ids == semantic_ids
        && quotient.every_current_member_enumerated_once
        && quotient.every_within_candidate_pair_decided
        && quotient.every_predecessor_pair_decided
        && quotient.every_active_member_has_fresh_marginality
        && quotient.generated_instances_mint_no_class
        && !quotient.v4_marginality_labels_used_as_proof
        && quotient.named_residual_count == 0
        && package.named_quotient_residual_count == 0
        && quotient.proved
        && member_hashes_exact
        && decisions_exact
        && marginality_exact
        && inventories_exact
        && surface_digests_exact
        && quotient_hash_exact
        && closure_exact;
    let evidence = vec![tagged_hash(
        "bound-unified-representation-quotient",
        &(
            package.stage,
            &quotient.derivation_hash,
            &quotient.equivalence_closure.derivation_hash,
            &quotient.predecessor_surface_digest,
            &quotient.current_surface_digest,
            &quotient.fresh_core_inventory_digest,
            &quotient.fresh_ordinary_bridge_inventory_digest,
            &quotient.fresh_cubical_inventory_digest,
            exact,
        ),
    )];
    (evidence, exact)
}

fn marginality_evidence(package: &ActLocalSemanticProvenanceV5Certificate) -> (Vec<String>, bool) {
    let evidence = package
        .semantic_families
        .iter()
        .map(|family| {
            tagged_hash(
                "bound-predecessor-marginality-row",
                &(
                    &family.family_id,
                    &family.source,
                    &family.shape_key,
                    family.marginal,
                    &family.marginality_proof_hash,
                ),
            )
        })
        .collect::<Vec<_>>();
    let exact = package
        .semantic_families
        .iter()
        .all(|family| !family.marginality_proof_hash.is_empty());
    (evidence, exact)
}

fn r1_evidence(package: &ActLocalSemanticProvenanceV5Certificate) -> (Vec<String>, bool) {
    let r1_families = package
        .semantic_families
        .iter()
        .filter(|family| {
            package.role_resolutions.iter().any(|resolution| {
                matches!(
                    &resolution.resolution,
                    V5RoleResolution::ProvedFamily { proof }
                        if proof.rule == V5RelationRule::GenericR1Completion
                            && proof.family_id == family.family_id
                )
            })
        })
        .map(|family| family.derivation_hash.clone())
        .collect::<Vec<_>>();
    let applicable = package.stage == 1;
    let exact = if applicable {
        package.stage1_r1_preserved && r1_families.len() == 1
    } else {
        package.stage1_r1_preserved
    };
    let evidence = vec![tagged_hash(
        "bound-generic-r1-package-theorem",
        &(
            package.stage,
            applicable,
            &r1_families,
            package.stage1_r1_preserved,
        ),
    )];
    (evidence, exact)
}

fn r2_evidence(package: &ActLocalSemanticProvenanceV5Certificate) -> (Vec<String>, bool) {
    let generated = package
        .bridges
        .iter()
        .filter(|bridge| bridge.generated_instance)
        .map(|bridge| (&bridge.semantic_family_id, &bridge.derivation_hash))
        .collect::<Vec<_>>();
    let represented = generated.iter().any(|(family_id, _)| {
        package
            .semantic_families
            .iter()
            .any(|family| &family.family_id == *family_id)
    });
    let exact = package.r2_generated_instance_not_exported && !represented;
    let evidence = vec![tagged_hash(
        "bound-r2-generated-membership-theorem",
        &(
            package.stage,
            &generated,
            represented,
            package.r2_generated_instance_not_exported,
        ),
    )];
    (evidence, exact)
}

fn exact_a3_evidence(
    package: &ActLocalSemanticProvenanceV5Certificate,
    api: &A3GeneratorCapabilityProofV2,
) -> (Vec<String>, bool) {
    let proof = &package.exact_a3_capability;
    let api_rows = api
        .rows
        .iter()
        .map(|row| (row.scheme_id.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let orbit_ids = proof
        .orbit_capabilities
        .iter()
        .map(|row| row.orbit_id.as_str())
        .collect::<BTreeSet<_>>();
    let v5_scheme_ids = proof
        .orbit_capabilities
        .iter()
        .map(|row| row.scheme_id.as_str())
        .collect::<BTreeSet<_>>();
    let api_scheme_ids = api_rows.keys().copied().collect::<BTreeSet<_>>();
    let orbit_rows_exact = proof.orbit_capabilities.iter().all(|row| {
        let mut projection = row.clone();
        projection.derivation_hash.clear();
        let row_hash_exact =
            row.derivation_hash == v5_hash("exact-A3-orbit-capability", &projection);
        let output_exact = api_rows
            .get(row.scheme_id.as_str())
            .is_some_and(|api_row| api_row.required_output == row.required_output);
        let disposition_exact = if row.independently_exported {
            matches!(
                &row.disposition,
                V5ExactA3OrbitDisposition::ZeroChargeOpenHypothesis {
                    registration_formation_hash,
                    registration_replay_digest,
                    hole_charge_hash,
                    no_candidate_term_constructed: true,
                    no_credit_anchor_or_orbit_minted: true,
                    proof_hash,
                } if !registration_formation_hash.is_empty()
                    && !registration_replay_digest.is_empty()
                    && !hole_charge_hash.is_empty()
                    && !proof_hash.is_empty()
            )
        } else {
            matches!(
                &row.disposition,
                V5ExactA3OrbitDisposition::NonExportedInstanceOrbit {
                    quotient_equality_hashes: _,
                    proof_hash,
                } if !proof_hash.is_empty()
            )
        };
        row.scheme_and_instance_join_replayed
            && row
                .member_instance_ids
                .contains(&row.representative_instance_id)
            && !row.rule_constructor.is_empty()
            && output_exact
            && disposition_exact
            && row_hash_exact
    });
    let exported_count = proof
        .orbit_capabilities
        .iter()
        .filter(|row| row.independently_exported)
        .count();
    let mut proof_projection = proof.clone();
    proof_projection.derivation_hash.clear();
    let proof_hash_exact =
        proof.derivation_hash == v5_hash("exact-prefix-A3-capability-proof", &proof_projection);
    let mut closure_projection = package.finite_closure.clone();
    closure_projection.derivation_hash.clear();
    let closure_hash_exact = package.finite_closure.derivation_hash
        == v5_hash("finite-semantic-closure-proof", &closure_projection);
    let exact = proof.theorem_id == T_BI_B2_A3_CAPABILITY_ID
        && proof.stage == package.stage
        && proof.stage == api.stage
        && proof.prefix_signature_digest == package.predecessor_signature_digest
        && proof.prefix_signature_digest == api.prefix_signature_digest
        && proof.window_derivation_hash == api.window_derivation_hash
        && !proof.inventory_exhaustiveness_derivation_hash.is_empty()
        && proof.fresh_scheme_count == api.scheme_count
        && proof.fresh_instance_count == api.instance_count
        && proof.fresh_orbit_count == api.orbit_count
        && proof.fresh_exported_orbit_count == api.independently_exported_orbit_count
        && proof.orbit_capabilities.len() == proof.fresh_orbit_count
        && orbit_ids.len() == proof.fresh_orbit_count
        && v5_scheme_ids == api_scheme_ids
        && exported_count == proof.fresh_exported_orbit_count
        && proof.exact_prefix_bound
        && proof.relative_inventory_exhaustive
        && proof.every_fresh_orbit_classified
        && proof.constructed_exported_output_count == 0
        && proof.named_live_export_residual_count == 0
        && proof.no_constructed_exported_a3_fallback
        && !proof.cached_v4_zero_used_as_evidence
        && proof.proved
        && proof_hash_exact
        && orbit_rows_exact
        && package.finite_closure.exact_a3_orbit_count == proof.fresh_orbit_count
        && package.finite_closure.exact_a3_usable_output_count == 0
        && package.finite_closure.exact_a3_capability_derivation_hash == proof.derivation_hash
        && package.finite_closure.exact_a3_fallback_exclusion_proved
        && package.finite_closure.exact_a3_complete
        && closure_hash_exact
        && api.proved;
    let evidence = vec![tagged_hash(
        "bound-exact-a3-package-and-generator-capability",
        &(
            package.stage,
            &proof.derivation_hash,
            &api.derivation_hash,
            &package.finite_closure.derivation_hash,
            &orbit_ids,
            exact,
        ),
    )];
    (evidence, exact)
}

fn role_evidence(package: &ActLocalSemanticProvenanceV5Certificate) -> (Vec<String>, bool) {
    let mut exact = package.every_role_declaration_resolved
        && package.role_resolutions.len() == package.role_declaration_count;
    let expected_surfaces = vec![
        V5ConstructorSurface::CoreExpr,
        V5ConstructorSurface::OrdinarySchema2,
        V5ConstructorSurface::CubicalPath,
        V5ConstructorSurface::ExactA3,
    ];
    let expected_a3_orbits = package
        .exact_a3_capability
        .orbit_capabilities
        .iter()
        .map(|row| row.orbit_id.clone())
        .collect::<Vec<_>>();
    let expected_core_families = package.unified_quotient.core_family_ids.clone();
    let expected_ordinary_families = package.unified_quotient.ordinary_family_ids.clone();
    let expected_cubical_families = package.unified_quotient.cubical_family_ids.clone();
    let family_by_id = package
        .semantic_families
        .iter()
        .map(|family| (family.family_id.as_str(), family))
        .collect::<BTreeMap<_, _>>();
    let all_family_ids = package
        .semantic_families
        .iter()
        .map(|family| family.family_id.clone())
        .collect::<Vec<_>>();
    let evidence = package
        .role_resolutions
        .iter()
        .map(|resolution| {
            let search = &resolution.constructor_search;
            let mut search_projection = search.clone();
            search_projection.derivation_hash.clear();
            let search_hash_exact = search.derivation_hash
                == v5_hash("four-surface-role-constructor-search", &search_projection);
            let search_exact = search.proved
                && search.declaration_id == resolution.declaration_id
                && search.registry_size == HISTORICAL_ROLE_KINDS.len()
                && search.registry_exactly_historical_27
                && HISTORICAL_ROLE_KINDS.get(search.registry_index).copied()
                    == Some(search.role_kind.as_str())
                && search.queried_surfaces == expected_surfaces
                && search.core_family_ids_checked == expected_core_families
                && search.ordinary_bridge_ids_checked == expected_ordinary_families
                && search.cubical_family_ids_checked == expected_cubical_families
                && search.a3_orbit_ids_checked == expected_a3_orbits
                && search.core_surface_derivation_hash
                    == v5_hash(
                        "role-core-surface-query",
                        &(
                            &search.declaration_id,
                            &expected_core_families,
                            &package.unified_quotient.core_surface_digest,
                        ),
                    )
                && search.ordinary_surface_derivation_hash
                    == v5_hash(
                        "role-ordinary-surface-query",
                        &(
                            &search.declaration_id,
                            &expected_ordinary_families,
                            &package.unified_quotient.ordinary_surface_digest,
                        ),
                    )
                && search.cubical_surface_derivation_hash
                    == v5_hash(
                        "role-cubical-surface-query",
                        &(
                            &search.declaration_id,
                            &expected_cubical_families,
                            &package.unified_quotient.cubical_surface_digest,
                        ),
                    )
                && search.a3_surface_derivation_hash
                    == v5_hash(
                        "role-exact-A3-surface-query",
                        &(
                            &search.declaration_id,
                            &expected_a3_orbits,
                            &package.exact_a3_capability.derivation_hash,
                        ),
                    )
                && search.core_constructor_search_complete
                && search.ordinary_constructor_search_complete
                && search.cubical_constructor_search_complete
                && search.a3_capability_search_complete
                && search.exact_coordinate_predicate_replayed
                && search.exact_mechanism_predicate_replayed
                && search.exact_local_role_predicate_replayed
                && search.no_wildcard_or_default_absence_rule
                && !search.used_v4_desired_label_equality
                && search.rule != V5RelationRule::NoFamilyConstructor
                && !search.owner_typing_derivation_hash.is_empty()
                && search.matching_family_ids.len()
                    == search.fresh_term_relation_derivation_hashes.len()
                && search
                    .matching_family_ids
                    .iter()
                    .all(|family_id| family_by_id.contains_key(family_id.as_str()))
                && search
                    .fresh_term_relation_derivation_hashes
                    .iter()
                    .all(|hash| !hash.is_empty())
                && (search.exhaustive_empty_search_proved == search.matching_family_ids.is_empty())
                && search_hash_exact;
            let mut resolution_projection = resolution.clone();
            resolution_projection.derivation_hash.clear();
            let resolution_hash_exact = resolution.derivation_hash
                == v5_hash("role-declaration-resolution", &resolution_projection);
            exact &= resolution.resolved
                && !resolution.silent_residue
                && search_exact
                && resolution_hash_exact;
            match &resolution.resolution {
                V5RoleResolution::ProvedFamily { proof } => {
                    let mut proof_projection = proof.clone();
                    proof_projection.derivation_hash.clear();
                    exact &= proof.proved
                        && proof.derivation_hash
                            == v5_hash("exact-family-role-relation", &proof_projection)
                        && proof.declaration_id == resolution.declaration_id
                        && proof.occurrence.kind == search.role_kind
                        && proof.occurrence.coordinate == search.exact_coordinate
                        && proof.rule == search.rule
                        && search.matching_family_ids == vec![proof.family_id.clone()]
                        && search
                            .fresh_term_relation_derivation_hashes
                            .contains(&proof.fresh_term_relation_derivation_hash)
                        && family_by_id
                            .get(proof.family_id.as_str())
                            .is_some_and(|family| {
                                family.term_or_bridge_derivation_hash
                                    == proof.family_term_or_bridge_derivation_hash
                            })
                        && proof.declaration_reissued_from_candidate_and_prefix
                        && proof.exact_constructor_coordinate_replayed
                        && proof.exact_owner_clause_replayed
                        && proof.exact_mechanism_replayed
                        && proof.exact_local_role_replayed
                        && !proof.label_candidate_used_as_proof
                        && !proof.v4_desired_label_equality_used_as_proof
                        && !proof.additional_credit_minted
                        && !proof.family_term_or_bridge_derivation_hash.is_empty()
                        && !proof.fresh_term_relation_derivation_hash.is_empty();
                }
                V5RoleResolution::TheoremBackedImpossibility { proof } => {
                    let mut proof_projection = proof.clone();
                    proof_projection.derivation_hash.clear();
                    exact &= proof.proved
                        && proof.derivation_hash
                            == v5_hash("role-relation-impossibility", &proof_projection)
                        && proof.declaration_id == resolution.declaration_id
                        && proof.occurrence.kind == search.role_kind
                        && proof.occurrence.coordinate == search.exact_coordinate
                        && proof.constructor_induction_rule == search.rule
                        && proof.finite_closure_derivation_hash
                            == package.finite_closure.derivation_hash
                        && proof.candidate_family_ids_checked == all_family_ids
                        && proof.exact_a3_usable_output_count == 0
                        && proof.exact_a3_capability_derivation_hash
                            == package.exact_a3_capability.derivation_hash
                        && proof.every_relation_constructor_checked
                        && proof.matching_constructed_family_ids == search.matching_family_ids
                        && proof.no_unique_family_relation
                        && (proof.no_registered_family_relation
                            == search.matching_family_ids.is_empty())
                        && proof.no_constructed_exported_a3_fallback
                        && package.finite_closure.exact_a3_fallback_exclusion_proved;
                }
                V5RoleResolution::NamedResidual { .. } => exact = false,
            }
            tagged_hash(
                "bound-role-kind-relation-resolution",
                &(
                    &resolution.declaration_id,
                    &resolution.resolution,
                    &resolution.derivation_hash,
                ),
            )
        })
        .collect::<Vec<_>>();
    (evidence, exact)
}

fn anchor_evidence(package: &ActLocalSemanticProvenanceV5Certificate) -> (Vec<String>, bool) {
    let mut exact = package.every_marginal_family_credited_or_theorem_impossible
        && package.local_anchor_nonreuse_holds;
    let evidence = package
        .semantic_families
        .iter()
        .map(|family| {
            exact &= if family.marginal {
                match &family.anchor {
                    V5AnchorDisposition::CreditedLocalRole {
                        relation_derivation_hashes,
                        injection_hash,
                        ..
                    } => {
                        family.credited
                            && !relation_derivation_hashes.is_empty()
                            && !injection_hash.is_empty()
                    }
                    V5AnchorDisposition::TheoremImpossibleNoRelation {
                        no_constructed_exported_a3_fallback,
                        proof_hash,
                        ..
                    }
                    | V5AnchorDisposition::TheoremImpossibleRelationCollision {
                        no_constructed_exported_a3_fallback,
                        proof_hash,
                        ..
                    }
                    | V5AnchorDisposition::TheoremImpossibleNonFunctionalRelation {
                        no_constructed_exported_a3_fallback,
                        proof_hash,
                        ..
                    } => {
                        !family.credited
                            && *no_constructed_exported_a3_fallback
                            && !proof_hash.is_empty()
                    }
                    V5AnchorDisposition::Internal => false,
                    V5AnchorDisposition::NamedResidual { .. } => false,
                }
            } else {
                !family.credited && matches!(family.anchor, V5AnchorDisposition::Internal)
            };
            tagged_hash(
                "bound-family-anchor-resolution",
                &(
                    &family.family_id,
                    family.marginal,
                    family.credited,
                    &family.anchor,
                    &family.derivation_hash,
                ),
            )
        })
        .collect::<Vec<_>>();
    (evidence, exact)
}

fn special_case_evidence(
    package: &ActLocalSemanticProvenanceV5Certificate,
    candidate: &Telescope,
    prefix: &SealedSignature,
) -> (Vec<String>, bool) {
    let generic_r1_relation_hashes = package
        .role_resolutions
        .iter()
        .filter_map(|resolution| match &resolution.resolution {
            V5RoleResolution::ProvedFamily { proof }
                if proof.rule == V5RelationRule::GenericR1Completion =>
            {
                Some(proof.derivation_hash.clone())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let generic_r1_term_relation_hashes = package
        .role_resolutions
        .iter()
        .filter_map(|resolution| match &resolution.resolution {
            V5RoleResolution::ProvedFamily { proof }
                if proof.rule == V5RelationRule::GenericR1Completion =>
            {
                Some(proof.fresh_term_relation_derivation_hash.clone())
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let exact_elaboration =
        elaborate_telescope(prefix, candidate, package.stage.saturating_sub(1)).ok();
    let expected_stage1_completion_hash = exact_elaboration.as_ref().and_then(|elaboration| {
        (package.stage == 1 && candidate.clauses.len() == 2 && elaboration.clauses.len() == 2).then(
            || {
                v5_hash(
                    "stage1-exact-formation-completion-equality",
                    &(
                        &candidate.clauses[0].expr,
                        &candidate.clauses[1].expr,
                        &elaboration.clauses[0],
                        &elaboration.clauses[1],
                        generic_r1_term_relation_hashes.first().map(String::as_str),
                    ),
                )
            },
        )
    });
    let expected_stage2_closure_hash = if package.stage == 2 {
        predecessor_closure(prefix).ok().and_then(|closure| {
            match extract_candidate_families(
                prefix,
                &closure,
                candidate,
                package.stage.saturating_sub(1),
            ) {
                CandidateExtractionOutcome::Extracted(extraction) => Some(v5_hash(
                    "stage2-fresh-exact-predecessor-closure",
                    &(
                        prefix.digest(),
                        &closure.digest,
                        &closure.families,
                        &extraction.derivation_hash,
                        &package.unified_quotient.derivation_hash,
                    ),
                )),
                CandidateExtractionOutcome::KernelInvalid { .. } => None,
            }
        })
    } else {
        None
    };
    let exact_stage1_roles = package
        .stage1_carrier_role_case_proofs
        .iter()
        .map(|proof| proof.role)
        .collect::<Vec<_>>();
    let stage1_case_proofs_exact = package.stage1_carrier_role_case_proofs.iter().all(|proof| {
        let mut projection = proof.clone();
        projection.derivation_hash.clear();
        let is_kernel_head = proof.role == LocalRole::KernelHead;
        proof.theorem_id == T_BI_B2_STAGE1_R1_ID
            && proof.carrier_clause == 0
            && proof.completion_clause == 1
            && proof.carrier_expr == Expr::Univ
            && proof.completion_expr == Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))
            && proof.carrier_normal_form == proof.carrier_expr
            && proof.completion_normal_form == proof.completion_expr
            && proof.candidate_clause_roles
                == candidate
                    .clauses
                    .iter()
                    .map(|clause| clause.role)
                    .collect::<Vec<_>>()
            && proof
                .candidate_clause_roles
                .iter()
                .all(|role| *role == ClauseRole::Formation)
            && proof.ordinary_bridge_count == package.bridges.len()
            && proof.ordinary_bridge_count == 0
            && proof.cubical_family_count == package.unified_quotient.cubical_family_ids.len()
            && proof.cubical_family_count == 0
            && proof.exact_a3_orbit_count == package.exact_a3_capability.fresh_orbit_count
            && proof.exact_a3_orbit_count == 0
            && proof.completion_relation_derivation_hash
                == generic_r1_relation_hashes.first().cloned()
            && proof.exact_formation_completion_package
            && (proof.role_generated_by_completion == is_kernel_head)
            && (proof.other_role_constructor_surface_empty == !is_kernel_head)
            && !proof.archive_or_desired_label_used
            && proof.proved
            && proof.derivation_hash == v5_hash("stage1-v5-carrier-role-case", &projection)
    });
    let stage1_exact = if package.stage == 1 {
        package.stage1_r1_preserved
            && candidate.clauses.len() == 2
            && candidate.clauses[0].role == ClauseRole::Formation
            && candidate.clauses[0].expr == Expr::Univ
            && candidate.clauses[1].role == ClauseRole::Formation
            && candidate.clauses[1].expr == Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))
            && generic_r1_relation_hashes.len() == 1
            && exact_stage1_roles == LocalRole::ALL
            && stage1_case_proofs_exact
            && package.stage1_carrier_role_case_derivation_hashes.len() == 4
            && package.stage1_carrier_role_case_derivation_hashes
                == package
                    .stage1_carrier_role_case_proofs
                    .iter()
                    .map(|proof| proof.derivation_hash.clone())
                    .collect::<Vec<_>>()
            && package
                .stage1_carrier_role_case_derivation_hashes
                .iter()
                .all(|hash| !hash.is_empty())
            && !package
                .stage1_exact_completion_equality_derivation_hash
                .is_empty()
            && Some(
                package
                    .stage1_exact_completion_equality_derivation_hash
                    .clone(),
            ) == expected_stage1_completion_hash
    } else {
        package.stage1_r1_preserved
            && package.stage1_carrier_role_case_proofs.is_empty()
            && package
                .stage1_carrier_role_case_derivation_hashes
                .is_empty()
            && package
                .stage1_exact_completion_equality_derivation_hash
                .is_empty()
    };
    let semantic_family_ids = package
        .semantic_families
        .iter()
        .map(|family| family.family_id.clone())
        .collect::<Vec<_>>();
    let stage2_exact = if package.stage == 2 {
        package.stage2_constitutive_question_not_assumed
            && !package.stage2_exact_family_surface_ids.is_empty()
            && package.stage2_exact_family_surface_ids == semantic_family_ids
            && package.stage2_internality_derivation_hashes.len()
                == package.stage2_exact_family_surface_ids.len()
            && package
                .stage2_internality_derivation_hashes
                .iter()
                .all(|hash| !hash.is_empty())
            && !package.stage2_exact_predecessor_closure_hash.is_empty()
            && Some(package.stage2_exact_predecessor_closure_hash.clone())
                == expected_stage2_closure_hash
            && package.unified_quotient.predecessor_closure_digest.len() > "blake3:".len()
            && package.semantic_families.iter().all(|family| {
                !family.marginal
                    && !family.credited
                    && matches!(family.anchor, V5AnchorDisposition::Internal)
                    && matches!(
                        &family.fresh_marginality,
                        V5FreshMarginalityDisposition::InternalIdentical { .. }
                            | V5FreshMarginalityDisposition::InternalDerivable { .. }
                            | V5FreshMarginalityDisposition::InternalUnifiedPreimage { .. }
                            | V5FreshMarginalityDisposition::InternalR1FormationCompletionPackage {
                                ..
                            }
                    )
                    && package
                        .stage2_internality_derivation_hashes
                        .contains(&family.marginality_proof_hash)
            })
            && package.stage2_no_ordinary_duplicate
    } else {
        package.stage2_constitutive_question_not_assumed
            && package.stage2_exact_family_surface_ids.is_empty()
            && package.stage2_internality_derivation_hashes.is_empty()
            && package.stage2_exact_predecessor_closure_hash.is_empty()
            && package.stage2_no_ordinary_duplicate
    };
    let expected_stage9_ids = package
        .role_resolutions
        .iter()
        .filter(|resolution| resolution.constructor_search.role_kind.starts_with("map_"))
        .map(|resolution| resolution.declaration_id.clone())
        .collect::<Vec<_>>();
    let stage9_search_hashes = package
        .role_resolutions
        .iter()
        .map(|resolution| resolution.constructor_search.derivation_hash.as_str())
        .collect::<Vec<_>>();
    let expected_stage9_theorem_hash = v5_hash(
        "stage9-exact-map-boundary-theorem",
        &(
            &package.candidate_hash,
            &package.stage9_map_declaration_ids,
            &stage9_search_hashes,
            &package.finite_closure.derivation_hash,
            &package.exact_a3_capability.derivation_hash,
        ),
    );
    let stage9_exact = if package.stage == 9 {
        package.stage9_boundary_decided_by_relation_theorem
            && package.stage9_map_declaration_ids == expected_stage9_ids
            && package.stage9_map_declaration_ids.len() == package.role_declaration_count
            && package
                .stage9_map_declaration_ids
                .iter()
                .collect::<BTreeSet<_>>()
                .len()
                == package.stage9_map_declaration_ids.len()
            && package.stage9_boundary_theorem_hash == expected_stage9_theorem_hash
    } else {
        package.stage9_boundary_decided_by_relation_theorem
            && package.stage9_map_declaration_ids.is_empty()
            && package.stage9_boundary_theorem_hash.is_empty()
    };
    let r2_exact = package.r2_generated_instance_not_exported;
    let exact = package.t_bi_b1_proved
        && package.t_bi_b2_proved
        && package.named_role_residual_count == 0
        && package.named_quotient_residual_count == 0
        && package.named_a3_residual_count == 0
        && package.total_named_residual_count
            == package.named_role_residual_count
                + package.named_quotient_residual_count
                + package.named_a3_residual_count
        && package.total_named_residual_count == 0
        && package.silent_residue_count == 0
        && stage1_exact
        && stage2_exact
        && stage9_exact
        && r2_exact;
    let evidence = vec![tagged_hash(
        "bound-stage-special-case-theorems",
        &(
            package.stage,
            (
                package.t_bi_b1_proved,
                package.t_bi_b2_proved,
                package.stage1_r1_preserved,
                &package.stage1_carrier_role_case_derivation_hashes,
                &package.stage1_carrier_role_case_proofs,
                &package.stage1_exact_completion_equality_derivation_hash,
            ),
            (
                package.stage2_constitutive_question_not_assumed,
                &package.stage2_exact_family_surface_ids,
                &package.stage2_internality_derivation_hashes,
                &package.stage2_exact_predecessor_closure_hash,
                package.stage2_no_ordinary_duplicate,
            ),
            (
                package.stage9_boundary_decided_by_relation_theorem,
                &package.stage9_map_declaration_ids,
                &package.stage9_boundary_theorem_hash,
            ),
            (
                package.r2_generated_instance_not_exported,
                package.named_role_residual_count,
                package.named_quotient_residual_count,
                package.named_a3_residual_count,
                package.total_named_residual_count,
                package.silent_residue_count,
            ),
            exact,
        ),
    )];
    (evidence, exact)
}

fn one_gap_unless(
    condition: bool,
    stage: Option<u32>,
    operation: IntrinsicPhaseOperationV2,
    code: &str,
    detail: &str,
    evidence: &[String],
) -> Vec<IntrinsicIsolationGapV2> {
    if condition {
        Vec::new()
    } else {
        vec![gap(stage, operation, code, detail, evidence)]
    }
}

fn derive_capability_closure(
    nodes: &[IntrinsicPhaseReceiptV2],
) -> Result<Vec<IntrinsicCapabilityClosureRowV2>, TBiIntrinsicIsolationV2Error> {
    let forbidden = IntrinsicCapabilityV2::forbidden();
    let mut closures = BTreeMap::<String, BTreeSet<IntrinsicCapabilityV2>>::new();
    let mut rows = Vec::with_capacity(nodes.len());
    for node in nodes {
        let expected_capabilities = node.operation.direct_capabilities();
        if node.direct_capabilities != expected_capabilities {
            return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
                "node {} carries caller-shaped capabilities",
                node.node_id
            )));
        }
        let mut capabilities = expected_capabilities.into_iter().collect::<BTreeSet<_>>();
        for predecessor in &node.predecessor_node_ids {
            let predecessor_closure = closures.get(predecessor).ok_or_else(|| {
                TBiIntrinsicIsolationV2Error::Binding(format!(
                    "node {} references unavailable predecessor {predecessor}",
                    node.node_id
                ))
            })?;
            capabilities.extend(predecessor_closure.iter().copied());
        }
        let forbidden_capabilities = capabilities
            .intersection(&forbidden)
            .copied()
            .collect::<Vec<_>>();
        let transitive_capabilities = capabilities.iter().copied().collect::<Vec<_>>();
        let isolated = forbidden_capabilities.is_empty();
        let derivation_hash = tagged_hash(
            "typed-capability-closure-row",
            &(
                &node.node_id,
                &node.receipt_hash,
                &transitive_capabilities,
                &forbidden_capabilities,
                isolated,
            ),
        );
        closures.insert(node.node_id.clone(), capabilities);
        rows.push(IntrinsicCapabilityClosureRowV2 {
            node_id: node.node_id.clone(),
            transitive_capabilities,
            forbidden_capabilities,
            isolated,
            derivation_hash,
        });
    }
    Ok(rows)
}

fn every_node_reaches_sequence_seal(nodes: &[IntrinsicPhaseReceiptV2]) -> bool {
    let Some(root) = nodes.last() else {
        return false;
    };
    if root.node_id != "intrinsic-sequence-seal" {
        return false;
    }
    let by_id = nodes
        .iter()
        .map(|node| (node.node_id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let mut reachable = BTreeSet::<String>::new();
    let mut stack = vec![root.node_id.clone()];
    while let Some(node_id) = stack.pop() {
        if !reachable.insert(node_id.clone()) {
            continue;
        }
        let Some(node) = by_id.get(node_id.as_str()) else {
            return false;
        };
        stack.extend(node.predecessor_node_ids.iter().cloned());
    }
    reachable.len() == nodes.len()
}

fn validate_claimed_node_bindings(nodes: &[IntrinsicPhaseReceiptV2]) -> Vec<String> {
    let mut errors = Vec::new();
    let mut prior = BTreeMap::<String, &IntrinsicPhaseReceiptV2>::new();
    for node in nodes {
        if prior.contains_key(&node.node_id) {
            errors.push(format!("duplicate claimed node {}", node.node_id));
            continue;
        }
        if node.direct_capabilities != node.operation.direct_capabilities() {
            errors.push(format!(
                "node {} capability set is not derived from its operation",
                node.node_id
            ));
        }
        let predecessors = node
            .predecessor_node_ids
            .iter()
            .filter_map(|id| prior.get(id).copied())
            .collect::<Vec<_>>();
        if predecessors.len() != node.predecessor_node_ids.len() {
            errors.push(format!(
                "node {} has a missing or forward predecessor",
                node.node_id
            ));
        } else {
            let receipt_hashes = predecessors
                .iter()
                .map(|predecessor| predecessor.receipt_hash.clone())
                .collect::<Vec<_>>();
            let output_hashes = predecessors
                .iter()
                .map(|predecessor| predecessor.output_hash.clone())
                .collect::<Vec<_>>();
            if receipt_hashes != node.predecessor_receipt_hashes
                || output_hashes != node.predecessor_output_hashes
            {
                errors.push(format!(
                    "node {} predecessor receipt/output binding mismatch",
                    node.node_id
                ));
            }
        }
        if receipt_hash(node) != node.receipt_hash {
            errors.push(format!("node {} receipt hash mismatch", node.node_id));
        }
        prior.insert(node.node_id.clone(), node);
    }
    match derive_capability_closure(nodes) {
        Ok(rows)
            if rows
                .iter()
                .all(|row| row.isolated && row.forbidden_capabilities.is_empty()) => {}
        Ok(_) => errors.push("claimed DAG contains a forbidden transitive capability".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

/// Issue B3 v2 from the exact act/prefix inputs and the v5 sequence they
/// reissue.  There is intentionally no receipt parameter.
pub fn issue_t_bi_intrinsic_isolation_v2(
    entries: &[(u32, Telescope)],
    sequence: &ActLocalSemanticSequenceV5,
) -> Result<TBiIntrinsicIsolationV2Token, TBiIntrinsicIsolationV2Error> {
    if entries.len() != 15
        || !entries.iter().map(|(stage, _)| *stage).eq(1..=15)
        || sequence.packages.len() != 15
        || !sequence
            .packages
            .iter()
            .map(|package| package.stage)
            .eq(1..=15)
    {
        return Err(TBiIntrinsicIsolationV2Error::Input(
            "B3 v2 requires the exact contiguous fifteen-act surface".to_owned(),
        ));
    }
    issue_t_bi_intrinsic_isolation_core_v2(entries, sequence, IsolationSurfaceV2::ExactFifteen)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IsolationSurfaceV2 {
    ExactFifteen,
    ContiguousPrefix,
}

/// Crate-private successor hook.  It deliberately returns the unchanged v2
/// evidence vocabulary so v3 can wrap the already-audited typed phase DAG,
/// while relaxing only the historical *surface* premise.  No receipt or
/// capability can be supplied by the caller.
pub(crate) fn issue_t_bi_intrinsic_isolation_prefix_core_v2(
    entries: &[(u32, Telescope)],
    sequence: &ActLocalSemanticSequenceV5,
) -> Result<TBiIntrinsicIsolationV2Token, TBiIntrinsicIsolationV2Error> {
    if entries.is_empty()
        || !entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=u32::try_from(entries.len()).unwrap_or(u32::MAX))
        || sequence.packages.len() != entries.len()
        || !sequence
            .packages
            .iter()
            .map(|package| package.stage)
            .eq(1..=u32::try_from(entries.len()).unwrap_or(u32::MAX))
    {
        return Err(TBiIntrinsicIsolationV2Error::Input(
            "B3 prefix core requires a nonempty contiguous Stage-1-through-N surface".to_owned(),
        ));
    }
    issue_t_bi_intrinsic_isolation_core_v2(entries, sequence, IsolationSurfaceV2::ContiguousPrefix)
}

fn issue_t_bi_intrinsic_isolation_core_v2(
    entries: &[(u32, Telescope)],
    sequence: &ActLocalSemanticSequenceV5,
    surface: IsolationSurfaceV2,
) -> Result<TBiIntrinsicIsolationV2Token, TBiIntrinsicIsolationV2Error> {
    let replay_errors = replay_act_local_semantic_sequence_v5(entries, sequence);
    if !replay_errors.is_empty() {
        return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
            "v5 sequence did not reissue from the supplied acts: {}",
            replay_errors.join("; ")
        )));
    }
    for package in &sequence.packages {
        validate_v5_package_commitments(package)?;
    }
    validate_v5_sequence_commitments(sequence)?;

    let mut builder = PhaseDagBuilderV2::new();
    let mut accepted = Vec::<(u32, Telescope)>::new();
    let mut predecessor_members = Vec::<V5UnifiedSurfaceMember>::new();
    let mut prior_package_nodes = Vec::<String>::new();
    let mut a3_generator_capability_proofs = Vec::new();
    let mut observed_role_kinds = BTreeSet::<String>::new();

    for (((stage, candidate), package), exact_package_hash) in entries
        .iter()
        .zip(&sequence.packages)
        .zip(&sequence.exact_package_derivation_hashes)
    {
        let prefix = SealedSignature::from_telescopes(accepted.clone());
        if package.stage != *stage
            || package.candidate_hash != candidate_hash(candidate)
            || package.predecessor_signature_digest != prefix.digest()
            || &package.derivation_hash != exact_package_hash
        {
            return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
                "Stage {stage} package is not bound to the exact candidate and prefix"
            )));
        }
        let stage = *stage;
        let candidate_node = stage_node_id(stage, "candidate-input");
        let prefix_node = stage_node_id(stage, "exact-prefix");
        let frozen_node = stage_node_id(stage, "frozen-declarations");
        let elaboration_node = stage_node_id(stage, "elaboration-extraction");
        let core_node = stage_node_id(stage, "core-term-family-naturality");
        let bridge_node = stage_node_id(stage, "expr-schema2-bridge");
        let quotient_node = stage_node_id(stage, "unified-representation-quotient");
        let marginality_node = stage_node_id(stage, "predecessor-marginality");
        let r1_node = stage_node_id(stage, "generic-r1-package");
        let r2_node = stage_node_id(stage, "r2-generated-membership");
        let ordinary_node = stage_node_id(stage, "ordinary-registry-proof");
        let cubical_node = stage_node_id(stage, "cubical-projection-quotient");
        let a3_node = stage_node_id(stage, "exact-a3-capability");
        let role_node = stage_node_id(stage, "role-kind-relation-induction");
        let anchor_node = stage_node_id(stage, "family-anchor-resolution");
        let special_node = stage_node_id(stage, "stage-special-case-theorems");
        let package_node = stage_node_id(stage, "package-seal");

        let phase1 = phase(package, 1, "candidate-prefix")?;
        let phase1_exact = phase1.evidence_hashes
            == vec![
                package.candidate_hash.clone(),
                package.predecessor_signature_digest.clone(),
            ];
        builder.add(
            candidate_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::CandidateInput,
            Vec::new(),
            vec![
                package.candidate_hash.clone(),
                phase1.derivation_hash.clone(),
            ],
            Some(package.candidate_hash.clone()),
            one_gap_unless(
                phase1_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::CandidateInput,
                "T_BI_B3_V2_CANDIDATE_PHASE_UNBOUND",
                "candidate-prefix phase does not contain the exact candidate and prefix",
                &phase1.evidence_hashes,
            ),
        )?;
        builder.add(
            prefix_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::ExactPrefix,
            prior_package_nodes.clone(),
            vec![
                package.predecessor_signature_digest.clone(),
                phase1.derivation_hash.clone(),
            ],
            Some(package.predecessor_signature_digest.clone()),
            Vec::new(),
        )?;

        let phase2 = phase(package, 2, "frozen-declaration-and-v4-reissue")?;
        let phase2_exact = phase2.evidence_hashes
            == vec![
                package.frozen_v3_package_hash.clone(),
                package.frozen_v4_package_hash.clone(),
            ];
        builder.add(
            frozen_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::FrozenDeclarationReissue,
            vec![candidate_node.clone(), prefix_node.clone()],
            vec![
                package.frozen_v3_package_hash.clone(),
                package.frozen_v4_package_hash.clone(),
                phase2.derivation_hash.clone(),
            ],
            None,
            one_gap_unless(
                phase2_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::FrozenDeclarationReissue,
                "T_BI_B3_V2_FROZEN_DECLARATION_PHASE_UNBOUND",
                "frozen declaration receipt is not the exact v3/v4 pair",
                &phase2.evidence_hashes,
            ),
        )?;

        let phase3 = phase(package, 3, "elaboration-and-extraction")?;
        let phase3_exact = phase3.evidence_hashes.len() == 2;
        builder.add(
            elaboration_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::CandidateElaborationAndExtraction,
            vec![
                candidate_node.clone(),
                prefix_node.clone(),
                frozen_node.clone(),
            ],
            phase3
                .evidence_hashes
                .iter()
                .cloned()
                .chain(std::iter::once(phase3.derivation_hash.clone()))
                .collect(),
            None,
            one_gap_unless(
                phase3_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::CandidateElaborationAndExtraction,
                "T_BI_B3_V2_ELABORATION_EXTRACTION_PHASE_UNBOUND",
                "elaboration/extraction receipt does not contain exactly both derivations",
                &phase3.evidence_hashes,
            ),
        )?;

        let core_evidence = core_family_evidence(package);
        let core_exact = package
            .semantic_families
            .iter()
            .filter(|family| matches!(family.source, V5SemanticFamilySource::FrozenV4 { .. }))
            .all(|family| {
                family.typed_normalized_natural
                    && !family.term_or_bridge_derivation_hash.is_empty()
                    && !family.derivation_hash.is_empty()
            });
        builder.add(
            core_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::CoreTermFamilyNaturality,
            vec![elaboration_node.clone(), frozen_node.clone()],
            core_evidence.clone(),
            None,
            one_gap_unless(
                core_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::CoreTermFamilyNaturality,
                "T_BI_B3_V2_CORE_FAMILY_EVIDENCE_UNBOUND",
                "a retained core/cubical family lacks typed normalized natural evidence",
                &core_evidence,
            ),
        )?;

        let phase4 = phase(package, 4, "expr-to-schema2")?;
        let bridge_hashes = package
            .bridges
            .iter()
            .map(|bridge| bridge.derivation_hash.clone())
            .collect::<Vec<_>>();
        let bridge_exact = phase4.evidence_hashes == bridge_hashes
            && package.bridges.iter().all(|bridge| {
                bridge.proved
                    && bridge.stage == stage
                    && bridge.candidate_hash == package.candidate_hash
                    && bridge.predecessor_signature_digest == package.predecessor_signature_digest
                    && !bridge.derivation_hash.is_empty()
            });
        let mut bridge_evidence = bridge_hashes.clone();
        bridge_evidence.push(phase4.derivation_hash.clone());
        builder.add(
            bridge_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::ExprToSchema2Bridge,
            vec![
                elaboration_node.clone(),
                core_node.clone(),
                frozen_node.clone(),
                prefix_node.clone(),
            ],
            bridge_evidence.clone(),
            None,
            one_gap_unless(
                bridge_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::ExprToSchema2Bridge,
                "T_BI_B3_V2_SCHEMA2_BRIDGE_UNBOUND",
                "an ordinary bridge is not bound to the exact act/prefix and phase receipt",
                &bridge_evidence,
            ),
        )?;

        let (quotient_evidence, quotient_exact) = quotient_evidence(package, &predecessor_members);
        builder.add(
            quotient_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::UnifiedRepresentationQuotient,
            vec![core_node.clone(), bridge_node.clone()],
            quotient_evidence.clone(),
            None,
            one_gap_unless(
                quotient_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::UnifiedRepresentationQuotient,
                "T_BI_B3_V2_UNIFIED_REPRESENTATION_QUOTIENT_GAP",
                "core, ordinary, and generated presentations do not form an exact disjoint quotient",
                &quotient_evidence,
            ),
        )?;

        let (marginality_evidence, marginality_exact) = marginality_evidence(package);
        let mut marginality_predecessors = vec![quotient_node.clone()];
        marginality_predecessors.extend(prior_package_nodes.iter().cloned());
        builder.add(
            marginality_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::PredecessorMarginalitySweep,
            marginality_predecessors,
            marginality_evidence.clone(),
            None,
            one_gap_unless(
                marginality_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::PredecessorMarginalitySweep,
                "T_BI_B3_V2_MARGINALITY_EVIDENCE_UNBOUND",
                "a family lacks a predecessor equality/marginality proof hash",
                &marginality_evidence,
            ),
        )?;

        let (r1_evidence, r1_exact) = r1_evidence(package);
        builder.add(
            r1_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::GenericR1PackageTheorem,
            vec![
                quotient_node.clone(),
                marginality_node.clone(),
                frozen_node.clone(),
            ],
            r1_evidence.clone(),
            None,
            one_gap_unless(
                r1_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::GenericR1PackageTheorem,
                "T_BI_B3_V2_GENERIC_R1_PACKAGE_GAP",
                "the applicable generic R1 package theorem is not uniquely bound",
                &r1_evidence,
            ),
        )?;

        let (r2_evidence, r2_exact) = r2_evidence(package);
        builder.add(
            r2_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::R2GeneratedInstanceMembership,
            vec![
                quotient_node.clone(),
                marginality_node.clone(),
                frozen_node.clone(),
            ],
            r2_evidence.clone(),
            None,
            one_gap_unless(
                r2_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::R2GeneratedInstanceMembership,
                "T_BI_B3_V2_R2_MEMBERSHIP_GAP",
                "a generated R2 child survives as a semantic family or lacks the M1 disposition",
                &r2_evidence,
            ),
        )?;

        let phase6 = phase(package, 6, "ordinary-and-cubical-theorem")?;
        if phase6.evidence_hashes.len() != 4
            || phase6.evidence_hashes[2] != package.unified_quotient.derivation_hash
            || phase6.evidence_hashes[3] != package.finite_closure.derivation_hash
        {
            return Err(TBiIntrinsicIsolationV2Error::Binding(format!(
                "Stage {stage} ordinary/cubical phase is not the exact four-proof bundle"
            )));
        }
        let ordinary_evidence = vec![
            phase6.evidence_hashes[0].clone(),
            phase6.derivation_hash.clone(),
        ];
        let ordinary_exact = package.finite_closure.every_ordinary_schema_bridged
            && package.finite_closure.ordinary_bridge_count
                == package.finite_closure.ordinary_applicable_schema_count;
        builder.add(
            ordinary_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::OrdinaryRegistryProof,
            vec![bridge_node.clone(), frozen_node.clone()],
            ordinary_evidence.clone(),
            None,
            one_gap_unless(
                ordinary_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::OrdinaryRegistryProof,
                "T_BI_B3_V2_ORDINARY_REGISTRY_GAP",
                "ordinary registry bridge count or proof is incomplete",
                &ordinary_evidence,
            ),
        )?;
        let cubical_evidence = vec![
            phase6.evidence_hashes[1].clone(),
            phase6.derivation_hash.clone(),
        ];
        builder.add(
            cubical_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::CubicalProjectionAndQuotient,
            vec![core_node.clone(), frozen_node.clone()],
            cubical_evidence.clone(),
            None,
            one_gap_unless(
                package.finite_closure.path_quotient_complete,
                Some(stage),
                IntrinsicPhaseOperationV2::CubicalProjectionAndQuotient,
                "T_BI_B3_V2_CUBICAL_QUOTIENT_GAP",
                "cubical realization/projection quotient is incomplete",
                &cubical_evidence,
            ),
        )?;

        let phase5 = phase(package, 5, "exact-a3")?;
        let a3_proof = prove_a3_generator_capability_v2(&prefix, stage)?;
        let (mut a3_evidence, package_a3_exact) = exact_a3_evidence(package, &a3_proof);
        let phase_a3_exact =
            phase5.evidence_hashes == vec![package.exact_a3_capability.derivation_hash.clone()];
        let a3_exact = phase_a3_exact && package_a3_exact;
        a3_evidence.extend([
            package.exact_a3_capability.derivation_hash.clone(),
            phase5.derivation_hash.clone(),
            a3_proof.derivation_hash.clone(),
        ]);
        let joined_a3_output = tagged_hash(
            "joined-exact-a3-capability-output",
            &(
                stage,
                &package.exact_a3_capability.derivation_hash,
                &a3_proof.derivation_hash,
            ),
        );
        builder.add(
            a3_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::ExactA3GeneratorCapability,
            vec![prefix_node.clone()],
            a3_evidence.clone(),
            Some(joined_a3_output),
            one_gap_unless(
                a3_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::ExactA3GeneratorCapability,
                "T_BI_B3_V2_A3_CAPABILITY_GAP",
                "exact A3 phase is not bound to the exhaustive required-type-only API theorem",
                &a3_evidence,
            ),
        )?;
        a3_generator_capability_proofs.push(a3_proof);

        let phase7 = phase(package, 7, "family-role-relation")?;
        let role_hashes = package
            .role_resolutions
            .iter()
            .map(|resolution| resolution.derivation_hash.clone())
            .collect::<Vec<_>>();
        let (mut role_evidence, role_exact) = role_evidence(package);
        let role_phase_exact = phase7.evidence_hashes == role_hashes;
        role_evidence.push(phase7.derivation_hash.clone());
        for resolution in &package.role_resolutions {
            let kind = match &resolution.resolution {
                V5RoleResolution::ProvedFamily { proof } => &proof.occurrence.kind,
                V5RoleResolution::TheoremBackedImpossibility { proof } => &proof.occurrence.kind,
                V5RoleResolution::NamedResidual { proof } => &proof.occurrence.kind,
            };
            observed_role_kinds.insert(kind.clone());
        }
        let mut role_gaps = one_gap_unless(
            role_exact && role_phase_exact,
            Some(stage),
            IntrinsicPhaseOperationV2::RoleKindRelationInduction,
            "T_BI_B3_V2_ROLE_RELATION_INDUCTION_GAP",
            "one or more role resolutions lack a proof-bearing total constructor search",
            &role_evidence,
        );
        if package.role_resolutions.iter().any(|resolution| {
            matches!(
                &resolution.resolution,
                V5RoleResolution::ProvedFamily { proof }
                    if proof.rule == V5RelationRule::NoFamilyConstructor
            ) || matches!(
                &resolution.resolution,
                V5RoleResolution::TheoremBackedImpossibility { proof }
                    if proof.constructor_induction_rule == V5RelationRule::NoFamilyConstructor
            )
        }) {
            role_gaps.push(gap(
                Some(stage),
                IntrinsicPhaseOperationV2::RoleKindRelationInduction,
                "T_BI_B3_V2_DEFAULT_ROLE_CONSTRUCTOR_FORBIDDEN",
                "a role kind reached the legacy NoFamilyConstructor default",
                &role_evidence,
            ));
        }
        builder.add(
            role_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::RoleKindRelationInduction,
            vec![
                core_node.clone(),
                bridge_node.clone(),
                quotient_node.clone(),
                marginality_node.clone(),
                r1_node.clone(),
                r2_node.clone(),
                ordinary_node.clone(),
                cubical_node.clone(),
                a3_node.clone(),
                frozen_node.clone(),
            ],
            role_evidence,
            None,
            role_gaps,
        )?;

        let (anchor_evidence, anchor_exact) = anchor_evidence(package);
        builder.add(
            anchor_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::FamilyAnchorResolution,
            vec![role_node.clone(), marginality_node.clone(), a3_node.clone()],
            anchor_evidence.clone(),
            None,
            one_gap_unless(
                anchor_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::FamilyAnchorResolution,
                "T_BI_B3_V2_ANCHOR_RESOLUTION_GAP",
                "a marginal family lacks an exact injection or theorem-backed impossibility",
                &anchor_evidence,
            ),
        )?;

        let (special_evidence, special_exact) = special_case_evidence(package, candidate, &prefix);
        builder.add(
            special_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::StageSpecialCaseTheorems,
            vec![
                r1_node.clone(),
                r2_node.clone(),
                role_node.clone(),
                anchor_node.clone(),
            ],
            special_evidence.clone(),
            None,
            one_gap_unless(
                special_exact,
                Some(stage),
                IntrinsicPhaseOperationV2::StageSpecialCaseTheorems,
                "T_BI_B3_V2_STAGE_SPECIAL_CASE_GAP",
                "Stage 1/2/8/9 theorem conditions or B1/B2 gate are not proved",
                &special_evidence,
            ),
        )?;

        let phase8 = phase(package, 8, "package-seal")?;
        let package_evidence = vec![
            package.preseal_derivation_hash.clone(),
            phase8.derivation_hash.clone(),
            package.derivation_hash.clone(),
        ];
        builder.add(
            package_node.clone(),
            Some(stage),
            IntrinsicPhaseOperationV2::PackageSeal,
            vec![
                candidate_node,
                prefix_node,
                frozen_node,
                elaboration_node,
                core_node,
                bridge_node,
                quotient_node,
                marginality_node,
                r1_node,
                r2_node,
                ordinary_node,
                cubical_node,
                a3_node,
                role_node,
                anchor_node,
                special_node,
            ],
            package_evidence,
            Some(package.derivation_hash.clone()),
            Vec::new(),
        )?;
        prior_package_nodes.push(package_node);
        predecessor_members.extend(package.unified_quotient.current_members.iter().cloned());
        accepted.push((stage, candidate.clone()));
    }

    let (role_surface_exact, role_surface_evidence_hash) = match surface {
        IsolationSurfaceV2::ExactFifteen => {
            let expected_role_kinds = HISTORICAL_ROLE_KINDS
                .into_iter()
                .map(str::to_owned)
                .collect::<BTreeSet<_>>();
            let exact = observed_role_kinds == expected_role_kinds;
            (
                exact,
                tagged_hash(
                    "exact-historical-role-kind-surface",
                    &(&observed_role_kinds, &expected_role_kinds, exact),
                ),
            )
        }
        IsolationSurfaceV2::ContiguousPrefix => (
            true,
            tagged_hash(
                "prefix-local-observed-role-kind-surface",
                &(&observed_role_kinds, true),
            ),
        ),
    };
    let sequence_aggregates_exact = sequence.role_declaration_count
        == sequence
            .packages
            .iter()
            .map(|package| package.role_declaration_count)
            .sum::<usize>()
        && sequence.proved_family_declaration_count
            == sequence
                .packages
                .iter()
                .map(|package| package.proved_family_declaration_count)
                .sum::<usize>()
        && sequence.theorem_impossibility_declaration_count
            == sequence
                .packages
                .iter()
                .map(|package| package.theorem_impossibility_declaration_count)
                .sum::<usize>()
        && sequence.named_role_residual_count
            == sequence
                .packages
                .iter()
                .map(|package| package.named_role_residual_count)
                .sum::<usize>()
        && sequence.named_quotient_residual_count
            == sequence
                .packages
                .iter()
                .map(|package| package.named_quotient_residual_count)
                .sum::<usize>()
        && sequence.named_a3_residual_count
            == sequence
                .packages
                .iter()
                .map(|package| package.named_a3_residual_count)
                .sum::<usize>()
        && sequence.total_named_residual_count
            == sequence.named_role_residual_count
                + sequence.named_quotient_residual_count
                + sequence.named_a3_residual_count
        && sequence.named_quotient_residual_count == 0
        && sequence.named_a3_residual_count == 0
        && sequence.total_named_residual_count == 0
        && sequence.silent_residue_count
            == sequence
                .packages
                .iter()
                .map(|package| package.silent_residue_count)
                .sum::<usize>()
        && sequence.named_role_residual_count == 0
        && sequence.silent_residue_count == 0
        && sequence.t_bi_b1_proved_on_sequence
        && sequence.t_bi_b2_proved_on_sequence
        && sequence.proved_family_declaration_count
            + sequence.theorem_impossibility_declaration_count
            == sequence.role_declaration_count;
    let sequence_evidence = vec![
        sequence.intrinsic_sequence_seal.clone(),
        sequence.derivation_hash.clone(),
        role_surface_evidence_hash,
        tagged_hash(
            "exact-v5-sequence-semantic-aggregates",
            &(
                sequence.role_declaration_count,
                sequence.proved_family_declaration_count,
                sequence.theorem_impossibility_declaration_count,
                sequence.named_role_residual_count,
                sequence.named_quotient_residual_count,
                sequence.named_a3_residual_count,
                sequence.total_named_residual_count,
                sequence.silent_residue_count,
                sequence.t_bi_b1_proved_on_sequence,
                sequence.t_bi_b2_proved_on_sequence,
                sequence_aggregates_exact,
            ),
        ),
    ];
    let mut sequence_gaps = one_gap_unless(
        role_surface_exact,
        None,
        IntrinsicPhaseOperationV2::SequenceSeal,
        "T_BI_B3_V2_ROLE_KIND_SURFACE_NOT_EXACT",
        "the typed relation induction does not cover exactly the frozen 27-kind surface",
        &sequence_evidence,
    );
    sequence_gaps.extend(one_gap_unless(
        sequence_aggregates_exact,
        None,
        IntrinsicPhaseOperationV2::SequenceSeal,
        "T_BI_B3_V2_SEQUENCE_AGGREGATE_GAP",
        "the v5 sequence has a named/silent residual or its exact B1/B2 aggregates do not close",
        &sequence_evidence,
    ));
    builder.add(
        "intrinsic-sequence-seal".to_owned(),
        None,
        IntrinsicPhaseOperationV2::SequenceSeal,
        prior_package_nodes,
        sequence_evidence.clone(),
        Some(sequence.intrinsic_sequence_seal.clone()),
        sequence_gaps,
    )?;

    let closure_rows = derive_capability_closure(&builder.nodes)?;
    let exact_fifteen_stage_surface = builder
        .nodes
        .iter()
        .filter_map(|node| node.stage)
        .collect::<BTreeSet<_>>()
        == (1..=15).collect::<BTreeSet<_>>();
    let prefix_len = u32::try_from(entries.len()).unwrap_or(u32::MAX);
    let exact_typed_phase_surface = builder.nodes.len() == entries.len() * 17 + 1
        && (1..=prefix_len).all(|stage| {
            builder
                .nodes
                .iter()
                .filter(|node| node.stage == Some(stage))
                .count()
                == 17
        });
    let exact_package_commitments_recomputed = sequence
        .packages
        .iter()
        .all(|package| package_hash(package) == package.derivation_hash);
    let exact_sequence_commitments_recomputed = sequence_hash(sequence) == sequence.derivation_hash
        && sequence_seal_hash(&sequence.exact_package_derivation_hashes)
            == sequence.intrinsic_sequence_seal;
    let every_predecessor_hash_bound = validate_claimed_node_bindings(&builder.nodes).is_empty();
    let every_capability_derived_from_operation = builder
        .nodes
        .iter()
        .all(|node| node.direct_capabilities == node.operation.direct_capabilities());
    let every_node_reaches_sequence_seal = every_node_reaches_sequence_seal(&builder.nodes);
    let no_forbidden_capability_in_transitive_closure = closure_rows
        .iter()
        .all(|row| row.isolated && row.forbidden_capabilities.is_empty());
    let source_scan_used_as_proof = false;
    let runtime_self_report_used_as_proof = false;
    let synthetic_receipt_input_accepted = false;
    let required_stage_surface_proved = match surface {
        IsolationSurfaceV2::ExactFifteen => exact_fifteen_stage_surface,
        IsolationSurfaceV2::ContiguousPrefix => {
            entries.iter().map(|(stage, _)| *stage).eq(1..=prefix_len)
        }
    };
    let transitive_call_graph_isolation_proved = required_stage_surface_proved
        && exact_typed_phase_surface
        && exact_package_commitments_recomputed
        && exact_sequence_commitments_recomputed
        && every_predecessor_hash_bound
        && every_capability_derived_from_operation
        && every_node_reaches_sequence_seal
        && no_forbidden_capability_in_transitive_closure
        && builder.gaps.is_empty()
        && !source_scan_used_as_proof
        && !runtime_self_report_used_as_proof
        && !synthetic_receipt_input_accepted;
    let mut token = TBiIntrinsicIsolationV2Token {
        schema: T_BI_INTRINSIC_ISOLATION_V2_SCHEMA.to_owned(),
        date: T_BI_INTRINSIC_ISOLATION_V2_DATE.to_owned(),
        theorem_id: T_BI_B3_V2_THEOREM_ID.to_owned(),
        bound_intrinsic_sequence_schema: sequence.schema.clone(),
        bound_sequence_derivation_hash: sequence.derivation_hash.clone(),
        bound_package_derivation_hashes: sequence.exact_package_derivation_hashes.clone(),
        bound_intrinsic_sequence_seal: sequence.intrinsic_sequence_seal.clone(),
        a3_generator_capability_proofs,
        phase_receipts: builder.nodes,
        closure_rows,
        named_gaps: builder.gaps,
        exact_fifteen_stage_surface,
        exact_typed_phase_surface,
        exact_package_commitments_recomputed,
        exact_sequence_commitments_recomputed,
        every_predecessor_hash_bound,
        every_capability_derived_from_operation,
        every_node_reaches_sequence_seal,
        no_forbidden_capability_in_transitive_closure,
        source_scan_used_as_proof,
        runtime_self_report_used_as_proof,
        synthetic_receipt_input_accepted,
        transitive_call_graph_isolation_proved,
        proof_scope: "Typed deterministic evidence DAG from exact candidate/prefix inputs through core extraction, Schema2, an independently replayed closed base-rule equivalence quotient, marginality, R1, R2, ordinary, cubical, exhaustive A3 capability, total role relation, anchoring, special cases, package commitments, and the sequence seal. Post-seal operations are absent from the canonical DAG and their typed capabilities are rejected transitively. This is a replay theorem relative to the purity of the linked Rust issuer functions; Rust's type system does not itself provide an effect system proving that those callees perform no hidden reads.".to_owned(),
        derivation_hash: String::new(),
    };
    token.derivation_hash = token_hash(&token);
    Ok(token)
}

pub fn issue_reference_t_bi_intrinsic_isolation_v2()
-> Result<TBiIntrinsicIsolationV2Token, TBiIntrinsicIsolationV2Error> {
    let entries = (1..=15)
        .map(|stage| (stage, Telescope::reference(stage)))
        .collect::<Vec<_>>();
    let sequence =
        crate::act_local_semantic_provenance_v5::issue_act_local_semantic_sequence_v5(&entries)
            .map_err(|error| TBiIntrinsicIsolationV2Error::Input(error.to_string()))?;
    issue_t_bi_intrinsic_isolation_v2(&entries, &sequence)
}

pub fn replay_t_bi_intrinsic_isolation_v2(
    entries: &[(u32, Telescope)],
    sequence: &ActLocalSemanticSequenceV5,
    claimed: &TBiIntrinsicIsolationV2Token,
) -> Vec<String> {
    let mut errors = validate_claimed_node_bindings(&claimed.phase_receipts);
    if claimed.derivation_hash != token_hash(claimed) {
        errors.push("T-BI-B3 v2 token digest mismatch".to_owned());
    }
    match issue_t_bi_intrinsic_isolation_v2(entries, sequence) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => {
            errors.push("T-BI-B3 v2 token differs from typed create-new reissuance".to_owned())
        }
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::act_local_semantic_provenance_v5::issue_act_local_semantic_sequence_v5;
    use std::sync::OnceLock;

    fn source() -> &'static (Vec<(u32, Telescope)>, ActLocalSemanticSequenceV5) {
        static SOURCE: OnceLock<(Vec<(u32, Telescope)>, ActLocalSemanticSequenceV5)> =
            OnceLock::new();
        SOURCE.get_or_init(|| {
            let entries = (1..=15)
                .map(|stage| (stage, Telescope::reference(stage)))
                .collect::<Vec<_>>();
            let sequence = issue_act_local_semantic_sequence_v5(&entries).expect("v5 sequence");
            (entries, sequence)
        })
    }

    fn token() -> TBiIntrinsicIsolationV2Token {
        let (entries, sequence) = source();
        issue_t_bi_intrinsic_isolation_v2(entries, sequence).expect("B3 v2 token")
    }

    fn predecessor_members(
        sequence: &ActLocalSemanticSequenceV5,
        package_index: usize,
    ) -> Vec<V5UnifiedSurfaceMember> {
        sequence.packages[..package_index]
            .iter()
            .flat_map(|package| package.unified_quotient.current_members.iter().cloned())
            .collect()
    }

    #[test]
    fn typed_dag_binds_real_phases_and_replays_without_receipt_input() {
        let token = token();
        let (entries, sequence) = source();
        assert_eq!(token.phase_receipts.len(), 256);
        assert_eq!(token.a3_generator_capability_proofs.len(), 15);
        assert!(token.named_gaps.is_empty(), "{:?}", token.named_gaps);
        assert!(token.exact_package_commitments_recomputed);
        assert!(token.exact_sequence_commitments_recomputed);
        assert!(token.every_predecessor_hash_bound);
        assert!(token.every_capability_derived_from_operation);
        assert!(token.every_node_reaches_sequence_seal);
        assert!(token.no_forbidden_capability_in_transitive_closure);
        assert!(!token.synthetic_receipt_input_accepted);
        assert!(token.transitive_call_graph_isolation_proved);
        assert!(replay_t_bi_intrinsic_isolation_v2(entries, sequence, &token).is_empty());
    }

    #[test]
    fn constructed_a3_output_cannot_hide_behind_resigned_zero_aggregates() {
        let (entries, sequence) = source();
        let index = sequence
            .packages
            .iter()
            .position(|package| {
                package
                    .exact_a3_capability
                    .orbit_capabilities
                    .iter()
                    .any(|row| row.independently_exported)
            })
            .expect("an exported historical A3 orbit");
        let stage = u32::try_from(index + 1).expect("historical stage");
        let prefix = SealedSignature::from_telescopes(entries[..index].to_vec());
        let api = prove_a3_generator_capability_v2(&prefix, stage).expect("A3 API proof");
        let mut forged = sequence.packages[index].clone();
        assert!(exact_a3_evidence(&forged, &api).1);
        let row = forged
            .exact_a3_capability
            .orbit_capabilities
            .iter_mut()
            .find(|row| row.independently_exported)
            .expect("exported orbit");
        row.disposition = V5ExactA3OrbitDisposition::ConstructedExportedOutput {
            family_id: "forged-exported-family".to_owned(),
            term_relation_derivation_hash: tagged_hash("forged-A3-term-relation", &stage),
            proof_hash: tagged_hash("forged-A3-output-proof", &stage),
        };
        row.derivation_hash.clear();
        row.derivation_hash = v5_hash("exact-A3-orbit-capability", row);
        // Re-sign the old scalar conclusions as if the constructed witness did
        // not exist.  B3 must inspect the closed disposition surface itself.
        forged.exact_a3_capability.constructed_exported_output_count = 0;
        forged
            .exact_a3_capability
            .no_constructed_exported_a3_fallback = true;
        forged.exact_a3_capability.proved = true;
        forged.exact_a3_capability.derivation_hash.clear();
        forged.exact_a3_capability.derivation_hash = v5_hash(
            "exact-prefix-A3-capability-proof",
            &forged.exact_a3_capability,
        );
        forged.finite_closure.exact_a3_capability_derivation_hash =
            forged.exact_a3_capability.derivation_hash.clone();
        forged.finite_closure.derivation_hash.clear();
        forged.finite_closure.derivation_hash =
            v5_hash("finite-semantic-closure-proof", &forged.finite_closure);
        assert!(!exact_a3_evidence(&forged, &api).1);
    }

    #[test]
    fn map_precomposition_cannot_fall_through_the_legacy_default() {
        let (_, sequence) = source();
        let mut forged = sequence.packages[8].clone();
        assert!(role_evidence(&forged).1);
        let resolution = forged
            .role_resolutions
            .iter_mut()
            .find(|resolution| resolution.constructor_search.role_kind == "map_precomposition")
            .expect("Stage 9 map_precomposition declaration");
        resolution.constructor_search.rule = V5RelationRule::NoFamilyConstructor;
        resolution.constructor_search.derivation_hash.clear();
        resolution.constructor_search.derivation_hash = v5_hash(
            "four-surface-role-constructor-search",
            &resolution.constructor_search,
        );
        resolution.derivation_hash.clear();
        resolution.derivation_hash = v5_hash("role-declaration-resolution", resolution);
        assert!(!role_evidence(&forged).1);
    }

    #[test]
    fn closed_equivalence_registry_and_components_are_recomputed() {
        let (_, sequence) = source();
        let index = sequence
            .packages
            .iter()
            .position(|package| {
                !package
                    .unified_quotient
                    .equivalence_closure
                    .base_decisions
                    .is_empty()
            })
            .expect("a nontrivial equivalence universe");
        let predecessors = predecessor_members(sequence, index);
        let package = &sequence.packages[index];
        assert!(equivalence_closure_replays(package, &predecessors).2);
        assert!(quotient_evidence(package, &predecessors).1);

        let mut missing_rule = package.clone();
        missing_rule
            .unified_quotient
            .equivalence_closure
            .base_rule_registry
            .pop();
        missing_rule
            .unified_quotient
            .equivalence_closure
            .base_rule_registry_digest = v5_hash(
            "closed-unified-equivalence-base-rule-registry",
            &missing_rule
                .unified_quotient
                .equivalence_closure
                .base_rule_registry,
        );
        missing_rule
            .unified_quotient
            .equivalence_closure
            .exact_rule_registry_replayed = true;
        missing_rule
            .unified_quotient
            .equivalence_closure
            .derivation_hash
            .clear();
        missing_rule
            .unified_quotient
            .equivalence_closure
            .derivation_hash = v5_hash(
            "closed-unified-equivalence-closure",
            &missing_rule.unified_quotient.equivalence_closure,
        );
        assert!(!equivalence_closure_replays(&missing_rule, &predecessors).2);

        let mut forged_component = package.clone();
        let node = forged_component
            .unified_quotient
            .equivalence_closure
            .member_universe_node_ids
            .first()
            .expect("equivalence node")
            .clone();
        forged_component
            .unified_quotient
            .equivalence_closure
            .component_by_node
            .insert(node, "blake3:forged-component".to_owned());
        forged_component
            .unified_quotient
            .equivalence_closure
            .derivation_hash
            .clear();
        forged_component
            .unified_quotient
            .equivalence_closure
            .derivation_hash = v5_hash(
            "closed-unified-equivalence-closure",
            &forged_component.unified_quotient.equivalence_closure,
        );
        assert!(!equivalence_closure_replays(&forged_component, &predecessors).2);
    }

    #[test]
    fn cubical_cross_surface_equality_edge_is_rejected_after_resigning() {
        let (_, sequence) = source();
        let (index, decision_index) = sequence
            .packages
            .iter()
            .enumerate()
            .find_map(|(package_index, package)| {
                package
                    .unified_quotient
                    .equivalence_closure
                    .base_decisions
                    .iter()
                    .position(|decision| {
                        decision.left_surface != decision.right_surface
                            && (decision.left_surface == V5UnifiedSurface::CubicalPath
                                || decision.right_surface == V5UnifiedSurface::CubicalPath)
                    })
                    .map(|decision_index| (package_index, decision_index))
            })
            .expect("a cubical/non-cubical base pair");
        let predecessors = predecessor_members(sequence, index);
        let mut forged = sequence.packages[index].clone();
        let decision =
            &mut forged.unified_quotient.equivalence_closure.base_decisions[decision_index];
        decision.relation = V5UnifiedBaseRelation::Equal {
            rule: V5UnifiedEquivalenceBaseRule::CubicalTypedEquality,
        };
        decision.proved = true;
        decision.derivation_hash.clear();
        decision.derivation_hash = v5_hash("unified-base-equality-decision", decision);
        forged
            .unified_quotient
            .equivalence_closure
            .derivation_hash
            .clear();
        forged.unified_quotient.equivalence_closure.derivation_hash = v5_hash(
            "closed-unified-equivalence-closure",
            &forged.unified_quotient.equivalence_closure,
        );
        assert!(!equivalence_closure_replays(&forged, &predecessors).2);
        assert!(!quotient_evidence(&forged, &predecessors).1);
    }

    #[test]
    fn role_surface_queries_are_not_unchecked_proof_strings() {
        let (_, sequence) = source();
        let mut forged = sequence
            .packages
            .iter()
            .find(|package| !package.role_resolutions.is_empty())
            .expect("a role-bearing package")
            .clone();
        assert!(role_evidence(&forged).1);
        let resolution = forged
            .role_resolutions
            .first_mut()
            .expect("role resolution");
        resolution.constructor_search.core_surface_derivation_hash =
            "blake3:forged-core-surface-query".to_owned();
        resolution.constructor_search.derivation_hash.clear();
        resolution.constructor_search.derivation_hash = v5_hash(
            "four-surface-role-constructor-search",
            &resolution.constructor_search,
        );
        resolution.derivation_hash.clear();
        resolution.derivation_hash = v5_hash("role-declaration-resolution", resolution);
        assert!(!role_evidence(&forged).1);
    }

    #[test]
    fn stage1_stage2_and_stage9_commitments_are_not_scalar_flags() {
        let (entries, sequence) = source();
        assert!(sequence.packages.iter().zip(entries).enumerate().all(
            |(index, (package, (_, candidate)))| {
                let prefix = SealedSignature::from_telescopes(entries[..index].to_vec());
                special_case_evidence(package, candidate, &prefix).1
            }
        ));

        let stage1_prefix = SealedSignature::from_telescopes(Vec::new());
        let mut stage1 = sequence.packages[0].clone();
        stage1.stage1_carrier_role_case_derivation_hashes.pop();
        assert!(!special_case_evidence(&stage1, &entries[0].1, &stage1_prefix).1);

        let mut stage1_body = sequence.packages[0].clone();
        let case = stage1_body
            .stage1_carrier_role_case_proofs
            .first_mut()
            .expect("Stage-1 role case");
        case.carrier_expr = Expr::Var(99);
        case.derivation_hash.clear();
        case.derivation_hash = v5_hash("stage1-v5-carrier-role-case", case);
        stage1_body.stage1_carrier_role_case_derivation_hashes = stage1_body
            .stage1_carrier_role_case_proofs
            .iter()
            .map(|proof| proof.derivation_hash.clone())
            .collect();
        assert!(!special_case_evidence(&stage1_body, &entries[0].1, &stage1_prefix).1);

        let stage2_prefix = SealedSignature::from_telescopes(entries[..1].to_vec());
        let mut stage2 = sequence.packages[1].clone();
        stage2.stage2_no_ordinary_duplicate = false;
        assert!(!special_case_evidence(&stage2, &entries[1].1, &stage2_prefix).1);

        let mut stage2_hash = sequence.packages[1].clone();
        stage2_hash.stage2_exact_predecessor_closure_hash =
            "blake3:forged-stage2-closure".to_owned();
        assert!(!special_case_evidence(&stage2_hash, &entries[1].1, &stage2_prefix).1);

        let stage9_prefix = SealedSignature::from_telescopes(entries[..8].to_vec());
        let mut stage9 = sequence.packages[8].clone();
        stage9.stage9_map_declaration_ids.pop();
        assert!(!special_case_evidence(&stage9, &entries[8].1, &stage9_prefix).1);
    }

    #[test]
    fn forbidden_capability_mutation_is_rejected_even_after_resigning() {
        let mut forged = token();
        let node = forged
            .phase_receipts
            .iter_mut()
            .find(|node| node.node_id == "stage-01/core-term-family-naturality")
            .expect("core node");
        node.operation = IntrinsicPhaseOperationV2::ArchiveComparatorRead;
        node.direct_capabilities = node.operation.direct_capabilities();
        node.receipt_hash = receipt_hash(node);
        forged.derivation_hash.clear();
        forged.derivation_hash = token_hash(&forged);
        let structural_errors = validate_claimed_node_bindings(&forged.phase_receipts);
        assert!(
            structural_errors
                .iter()
                .any(|error| error.contains("forbidden"))
        );
        let (entries, sequence) = source();
        assert!(!replay_t_bi_intrinsic_isolation_v2(entries, sequence, &forged).is_empty());
    }

    #[test]
    fn swapped_package_receipt_cannot_reuse_another_package_commitment() {
        let mut forged = token();
        let stage2_hash = forged.bound_package_derivation_hashes[1].clone();
        let node = forged
            .phase_receipts
            .iter_mut()
            .find(|node| node.node_id == "stage-01/package-seal")
            .expect("Stage 1 package node");
        node.output_hash = stage2_hash;
        node.receipt_hash = receipt_hash(node);
        forged.derivation_hash.clear();
        forged.derivation_hash = token_hash(&forged);
        let (entries, sequence) = source();
        let errors = replay_t_bi_intrinsic_isolation_v2(entries, sequence, &forged);
        assert!(!errors.is_empty());
    }

    #[test]
    fn synthetic_receipts_are_not_accepted_as_an_issuer_input() {
        let mut forged = token();
        for (index, node) in forged.phase_receipts.iter_mut().enumerate() {
            node.output_hash = tagged_hash("synthetic-unbound-receipt", &index);
            node.receipt_hash = receipt_hash(node);
        }
        forged.derivation_hash.clear();
        forged.derivation_hash = token_hash(&forged);
        let (entries, sequence) = source();
        let errors = replay_t_bi_intrinsic_isolation_v2(entries, sequence, &forged);
        assert!(!errors.is_empty());
    }
}
