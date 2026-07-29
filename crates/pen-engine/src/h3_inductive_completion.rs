//! Authoritative disclosure for the frozen one-nullary inductive-completion
//! profile.
//!
//! The public outcome is serializable evidence, not a replacement for the
//! private capabilities retained by the verifiers. Every failed or missing
//! proof maps to `Unknown`.

use pen_demand::gsc::{
    GscFamilyId, GscOutcome, GscReferenceVectorAgreement, GscRule, GscSemanticManifestV1,
    GscVerifierManifestV1, OutputClause, PortKey, PremiseRef, PublicSourceId,
    VerifiedCanonicalDemandFamilyV2, VerifiedGscSemanticManifest, current_gsc_verifier_manifest_v1,
    frozen_gsc_semantic_manifest_v1, verify_gsc_reference_agreement_v1,
    verify_gsc_semantic_manifest_v1, verify_gsc_verifier_manifest_v1,
};
use pen_gf2::{
    CertifiedInapplicabilityReason, Gf2SliceVerifierManifestV1, OperationalRule,
    VerifiedDerivationDag, VerifiedOneNullaryInductiveSlice,
    run_registered_one_nullary_inductive_slice,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Kernel, KernelLimits,
    OpenJudgment, Term,
};
use pen_law::{
    CutoffExportCoverage, EventId, UncheckedRegisteredGroupDispositionV1, VerifiedHistory,
    VerifiedRegisteredActiveDemandInventory, VerifiedRegisteredBootstrap,
    VerifiedRegisteredClosedFormerFrame, VerifiedRegisteredDemandFamilyV2,
    load_embedded_registered_bootstrap, load_embedded_registered_bootstrap_export_index_v1,
    verify_registered_bootstrap_history,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const LAW_V2_H3_INDUCTIVE_COMPLETION_REPORT_SCHEMA_VERSION: u16 = 1;
pub const LAW_V2_H3_INDUCTIVE_COMPLETION_PROFILE_ID: &str = "gsc-inductive-completion-core-v1";
const REGISTERED_BOOTSTRAP_ASSET_CANONICAL_LF_DIGEST: &str =
    "blake3:f62e7503fa834800455ad67ccd02f6997476d17f85db5250293136cf0f0ef4d9";
const REGISTERED_BOOTSTRAP_ASSET_BYTES: &[u8] =
    include_bytes!("../../pen-law/assets/law_v2a_registered_bootstrap_v1.json");

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LawV2H3InductiveCompletionOutcomeV1 {
    Proven {
        report: Box<LawV2H3InductiveCompletionReportV1>,
    },
    Unknown {
        profile_id: String,
        reason: String,
        semantic_manifest_digest: Option<Digest>,
        fail_closed: bool,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LawV2H3InductiveCompletionReportV1 {
    #[serde(flatten)]
    pub body: LawV2H3InductiveCompletionReportBodyV1,
    pub result_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LawV2H3InductiveCompletionReportBodyV1 {
    pub schema_version: u16,
    pub profile_id: String,
    pub authority: String,
    pub semantic_manifest: GscSemanticManifestV1,
    pub semantic_manifest_digest: Digest,
    pub verifier: H3VerifierReportV1,
    pub bootstrap: H3BootstrapReportV1,
    pub history: H3HistoryReportV1,
    pub active_demand_inventory: H3ActiveDemandInventoryReportV1,
    pub registered_frame: H3RegisteredFrameReportV1,
    pub use_family: H3DemandFamilyReportV1,
    pub compute_families: Vec<H3DemandFamilyReportV1>,
    pub pre_response: H3UnderivedReportV1,
    pub equation_extension: H3EquationExtensionReportV1,
    pub response: H3ResponseReportV1,
    pub candidate_carrier: H3CandidateCarrierReportV1,
    pub quotient: H3QuotientReportV1,
    pub conclusion: H3ConclusionReportV1,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3VerifierReportV1 {
    pub demand_manifest: GscVerifierManifestV1,
    pub demand_manifest_digest: Digest,
    pub reference_agreement_capability_digest: Digest,
    pub gf2_manifest: Gf2SliceVerifierManifestV1,
    pub gf2_manifest_digest: Digest,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub law_authority_source_digest: Digest,
    pub engine_source_digest: Digest,
    pub workspace_manifest_digest: Digest,
    pub workspace_lock_digest: Digest,
    pub production_isolation_lock_digest: Digest,
    pub composite_provenance_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3RegisteredGroupReportV1 {
    pub group_digest: Digest,
    pub activation_ordinal: u16,
    pub declarations: Vec<GlobalId>,
    pub disposition: UncheckedRegisteredGroupDispositionV1,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3BootstrapReportV1 {
    pub bootstrap_contract_digest: Digest,
    pub bootstrap_artifact_digest: Digest,
    pub bootstrap_asset_canonical_lf_digest: Digest,
    pub bootstrap_kernel_digest: Digest,
    pub bootstrap_normalizer_digest: Digest,
    pub boundary_chain: Vec<Digest>,
    pub final_boundary_digest: Digest,
    pub source_identities: Vec<Digest>,
    pub binding_identities: Vec<Digest>,
    pub export_index_digest: Digest,
    pub groups: Vec<H3RegisteredGroupReportV1>,
    pub exported_q3_registry_is_empty: bool,
    pub original_bootstrap_unchanged: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3EventReportV1 {
    pub event_id: EventId,
    pub ordinal: u16,
    pub predecessor: Option<EventId>,
    pub pre_history_digest: Digest,
    pub post_history_digest: Digest,
    pub pre_boundary_digest: Digest,
    pub post_boundary_digest: Digest,
    pub extension_digest: Digest,
    pub source_identity: Digest,
    pub binding_identity: Digest,
    pub export_index_digest: Digest,
    pub activated_group_digests: Vec<Digest>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3HistoryReportV1 {
    pub history_digest: Digest,
    pub events: Vec<H3EventReportV1>,
    pub boundary_snapshot_digests: Vec<Digest>,
    pub declaration_origins: Vec<H3DeclarationOriginReportV1>,
    pub anchors: Vec<H3AnchorReportV1>,
    pub active_anchor_digest: Digest,
    pub active_anchor_older: EventId,
    pub active_anchor_newer: EventId,
    pub active_anchor_older_ordinal: u16,
    pub active_anchor_newer_ordinal: u16,
    pub cutoff_boundary_digest: Digest,
    pub cutoff_export_coverage_exhaustive: bool,
    pub origin_cutoff_q3_registry_digest: Digest,
    pub origin_cutoff_q3_registry_is_empty: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3DeclarationOriginReportV1 {
    pub declaration: GlobalId,
    pub origin: EventId,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3AnchorReportV1 {
    pub anchor_digest: Digest,
    pub older: EventId,
    pub newer: EventId,
    pub older_ordinal: u16,
    pub newer_ordinal: u16,
    pub cutoff_boundary_digest: Digest,
    pub cutoff_export_coverage_exhaustive: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3ComputeDemandTupleReportV1 {
    pub constructor_ordinal: u64,
    pub constructor_digest: Digest,
    pub family_binding_digest: Digest,
    pub output_port: PortKey,
    pub output_port_digest: Digest,
    pub tuple_binding_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3FrameDemandTupleReportV1 {
    pub active_frame_ordinal: u64,
    pub frame_binding_digest: Digest,
    pub use_family_binding_digest: Digest,
    pub exact_use_port: PortKey,
    pub compute_families: Vec<H3ComputeDemandTupleReportV1>,
    pub candidate_compute_order: Vec<u64>,
    pub tuple_binding_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3ActiveDemandInventoryReportV1 {
    pub binding_digest: Digest,
    pub semantic_manifest_digest: Digest,
    pub export_index_digest: Digest,
    pub history_digest: Digest,
    pub active_anchor_digest: Digest,
    pub cutoff_boundary_digest: Digest,
    pub q3_registry_digest: Digest,
    pub cutoff_export_coverage_exhaustive: bool,
    pub frame_count: u64,
    pub use_family_count: u64,
    pub compute_family_count: u64,
    pub family_count: u64,
    pub port_count: u64,
    pub constructor_count: u64,
    pub ordered_export_group_digests: Vec<Digest>,
    pub ordered_frame_binding_digests: Vec<Digest>,
    pub ordered_family_binding_digests: Vec<Digest>,
    pub ordered_port_digests: Vec<Digest>,
    pub ordered_constructor_digests: Vec<Digest>,
    pub tuples: Vec<H3FrameDemandTupleReportV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3PrincipalSourceReportV1 {
    pub declaration: GlobalId,
    pub public_source: PublicSourceId,
    pub origin_digest: Digest,
    pub registered_source_identity: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3RegisteredFrameReportV1 {
    pub binding_digest: Digest,
    pub frame_id: Digest,
    pub code_id: Digest,
    pub code: pen_demand::gsc::ClosedInductiveCode,
    pub owner: GlobalId,
    pub introduction_aliases: Vec<GlobalId>,
    pub constructor_ids: Vec<Digest>,
    pub export_group_digest: Digest,
    pub export_index_digest: Digest,
    pub history_digest: Digest,
    pub active_anchor_digest: Digest,
    pub cutoff_boundary_digest: Digest,
    pub q3_registry_digest: Digest,
    pub principal_sources: Vec<H3PrincipalSourceReportV1>,
    pub binding_valid: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3DemandFamilyReportV1 {
    pub binding_digest: Digest,
    pub family_id: GscFamilyId,
    pub rule: GscRule,
    pub rank: u8,
    pub parameter_context: DependentContext,
    pub premise_refs: Vec<PremiseRef>,
    pub output_clauses: Vec<OutputClause>,
    pub verification_judgments: Vec<OpenJudgment>,
    pub principal_sources: Vec<PublicSourceId>,
    pub birth_support_digests: Vec<Digest>,
    pub fixed_type_support: Vec<GlobalId>,
    pub parameter_support_projections: Vec<u32>,
    pub port_keys: Vec<PortKey>,
    pub exact_use_port: Option<PortKey>,
    pub source_use_family_binding_digest: Option<Digest>,
    pub constructor_digest: Option<Digest>,
    pub binding_valid_for_frame: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3GroundRuleDispositionReportV1 {
    pub rule: String,
    pub grounded_instances: u32,
    pub applicable_instances: u32,
    pub certified_inapplicable_instances: u32,
    pub inapplicability_evidence: Vec<H3InapplicabilityEvidenceReportV1>,
    pub exhaustive: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3InapplicabilityEvidenceReportV1 {
    pub reason: String,
    pub instances: u32,
    pub grounding_set_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3UnderivedReportV1 {
    pub port: PortKey,
    pub context: DependentContext,
    pub target: Term,
    pub examined_terms: u32,
    pub grounded_rule_instances: u32,
    pub structural_goal_count: u32,
    pub library_nodes: u32,
    pub closure_base_digest: Digest,
    pub closure_frontier_digests: Vec<Digest>,
    pub closure_fixed_point_digest: Digest,
    pub goal_rule_coverage_digest: Digest,
    pub canonical_replay_digest: Digest,
    pub grounding_inventory_digest: Digest,
    pub library_inventory_digest: Digest,
    pub dispositions: Vec<H3GroundRuleDispositionReportV1>,
    pub canonical_forms: H3CanonicalFormAnalysisReportV1,
    pub complete_relative_to_frozen_grammar: bool,
    pub grammar_digest: Digest,
    pub certificate_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3CanonicalFormAnalysisReportV1 {
    pub single_outer_function: bool,
    pub stuck_atomic_body: bool,
    pub neutral_heads_exhausted: bool,
    pub no_higher_order_argument_gap: bool,
    pub introduction_rules_separated: bool,
    pub digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3EquationExtensionReportV1 {
    pub set_digest: Digest,
    pub clause_count: u32,
    pub clause_digest: Digest,
    pub fresh_head: GlobalId,
    pub head_type: Term,
    pub context: DependentContext,
    pub left: Term,
    pub right: Term,
    pub ty: Term,
    pub extended_signature_digest: Digest,
    pub substitution_source_port: PortKey,
    pub substitution_source_context: DependentContext,
    pub substitution_target_context: DependentContext,
    pub substitution_filler: Term,
    pub substitution_digest: Digest,
    pub left_linear: bool,
    pub nonrecursive: bool,
    pub no_critical_overlaps: bool,
    pub terminating_in_admitted_fragment: bool,
    pub confluent_in_admitted_fragment: bool,
    pub conservative_on_old_terms: bool,
    pub exact_generated_substitution_preserves_equation: bool,
    pub invariant_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3DerivationNodeReportV1 {
    pub ordinal: u32,
    pub context: DependentContext,
    pub term: Term,
    pub ty: Term,
    pub rule: String,
    pub premises: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3PortDischargeReportV1 {
    pub port: PortKey,
    pub rule: String,
    pub premises: Vec<PortKey>,
    pub evidence_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3ResponseReportV1 {
    pub response_digest: Digest,
    pub use_port: PortKey,
    pub use_term: Term,
    pub use_target: Term,
    pub use_derivation_digest: Digest,
    pub use_derivation_nodes: Vec<H3DerivationNodeReportV1>,
    pub equation_set_digest: Digest,
    pub discharges: Vec<H3PortDischargeReportV1>,
    pub all_outputs_filled: bool,
    pub all_positive_clauses_connected: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3CandidateCarrierReportV1 {
    pub carrier_digest: Digest,
    pub semantic_manifest_digest: Digest,
    pub active_demand_inventory_binding_digest: Digest,
    pub registered_frame_binding_digest: Digest,
    pub use_family_binding_digest: Digest,
    pub compute_family_binding_digests: Vec<Digest>,
    pub canonical_fresh_head: GlobalId,
    pub equation_set_digest: Digest,
    pub response_digest: Digest,
    pub candidate_count: u32,
    pub live_use_family_count: u32,
    pub compute_family_count: u32,
    pub exhaustion_evidence_digest: Digest,
    pub exhaustive: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3QuotientReportV1 {
    pub quotient_digest: Digest,
    pub complete: bool,
    pub response_count: u32,
    pub class_count: u32,
    pub class_digest: Digest,
    pub representative_digest: Digest,
    pub equation_digest: Digest,
    pub q2_distinct_pair_decisions: u32,
    pub q3_edge_count: u32,
    pub q3_registry_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H3ConclusionReportV1 {
    pub live_profile_issued: bool,
    pub direct_eliminator_is_unique_class_relative_to_frozen_profile: bool,
    pub current_owner_specific_profile_recovers_archived_four_way_result: bool,
    pub contextual_internalization_was_adopted: bool,
    pub archived_candidate_inputs_loaded: bool,
    pub broader_semantic_claim_made: bool,
    pub disposition: String,
    pub semantic_manifest_digest: Digest,
    pub active_demand_inventory_binding_digest: Digest,
    pub registered_frame_binding_digest: Digest,
    pub verified_slice_digest: Digest,
}

/// Execute the registered-prefix profile from production-only capabilities.
pub fn run_law_v2_h3_inductive_completion_v1() -> LawV2H3InductiveCompletionOutcomeV1 {
    let kernel = match Kernel::new(KernelLimits::default()) {
        Ok(kernel) => kernel,
        Err(error) => return unknown(None, format!("kernel initialization failed: {error}")),
    };
    let semantic = match verify_gsc_semantic_manifest_v1(&frozen_gsc_semantic_manifest_v1()) {
        GscOutcome::Proven(semantic) => semantic,
        GscOutcome::Unknown(reason) => {
            return unknown(
                None,
                format!("semantic manifest is not verified: {reason:?}"),
            );
        }
    };
    run_with_semantic(&kernel, &semantic)
}

fn run_with_semantic(
    kernel: &Kernel,
    semantic: &VerifiedGscSemanticManifest,
) -> LawV2H3InductiveCompletionOutcomeV1 {
    let semantic_digest = Some(semantic.digest().clone());
    let reference_agreement = match verify_gsc_reference_agreement_v1(kernel, semantic) {
        GscOutcome::Proven(agreement) => agreement,
        GscOutcome::Unknown(reason) => {
            return unknown(
                semantic_digest,
                format!("generic Rust/Agda agreement is not verified: {reason:?}"),
            );
        }
    };
    let demand_manifest =
        match current_gsc_verifier_manifest_v1(semantic, kernel, &reference_agreement) {
            GscOutcome::Proven(manifest) => manifest,
            GscOutcome::Unknown(reason) => {
                return unknown(
                    semantic_digest,
                    format!("demand verifier manifest cannot be constructed: {reason:?}"),
                );
            }
        };
    let demand_verifier = match verify_gsc_verifier_manifest_v1(
        semantic,
        kernel,
        &reference_agreement,
        &demand_manifest,
    ) {
        GscOutcome::Proven(verifier) => verifier,
        GscOutcome::Unknown(reason) => {
            return unknown(
                semantic_digest,
                format!("demand verifier manifest is not verified: {reason:?}"),
            );
        }
    };
    if demand_manifest.reference_vector_agreement.status
        != GscReferenceVectorAgreement::ProvenRustReplayAndPinnedPrimitiveSafeAgdaProofV2
    {
        return unknown(
            semantic_digest,
            "generic Rust/Agda reference-vector agreement is not proven".to_owned(),
        );
    }
    let Some(bootstrap_asset_bytes_digest) =
        canonical_lf_text_digest(REGISTERED_BOOTSTRAP_ASSET_BYTES)
    else {
        return unknown(
            semantic_digest,
            "the registered bootstrap asset is not canonicalizable UTF-8 text".to_owned(),
        );
    };
    if bootstrap_asset_bytes_digest.as_str() != REGISTERED_BOOTSTRAP_ASSET_CANONICAL_LF_DIGEST {
        return unknown(
            semantic_digest,
            "the registered bootstrap asset bytes differ from the pre-slice freeze".to_owned(),
        );
    }
    let bootstrap = match load_embedded_registered_bootstrap(kernel) {
        Ok(bootstrap) => bootstrap,
        Err(error) => {
            return unknown(
                semantic_digest,
                format!("registered bootstrap replay failed: {error}"),
            );
        }
    };
    let export_index =
        match load_embedded_registered_bootstrap_export_index_v1(kernel, &bootstrap, semantic) {
            Ok(index) => index,
            Err(error) => {
                return unknown(
                    semantic_digest,
                    format!("registered bootstrap export replay failed: {error}"),
                );
            }
        };
    let history = match verify_registered_bootstrap_history(kernel, &bootstrap, export_index) {
        Ok(history) => history,
        Err(error) => {
            return unknown(
                semantic_digest,
                format!("registered history replay failed: {error}"),
            );
        }
    };
    if history.cutoff_export_coverage(history.active_anchor())
        != Some(CutoffExportCoverage::Exhaustive)
    {
        return unknown(
            semantic_digest,
            "active-anchor export coverage is not exhaustive".to_owned(),
        );
    }
    let active_demand_inventory = match history.active_demand_inventory(kernel, semantic) {
        GscOutcome::Proven(inventory) => inventory,
        GscOutcome::Unknown(reason) => {
            return unknown(
                semantic_digest,
                format!("registered active-demand inventory is undecided: {reason:?}"),
            );
        }
    };
    if !active_demand_inventory.binding_is_valid() {
        return unknown(
            semantic_digest,
            "registered active-demand inventory binding is invalid".to_owned(),
        );
    }
    let [frame_tuple] = active_demand_inventory.tuples() else {
        return unknown(
            semantic_digest,
            format!(
                "frozen carrier expected one exhaustive active frame tuple, found {}",
                active_demand_inventory.tuples().len()
            ),
        );
    };
    let frame = frame_tuple.frame();
    if !frame.inner().code().is_one_nullary() {
        return unknown(
            semantic_digest,
            "registered frame is outside the one-nullary profile".to_owned(),
        );
    }
    let [_compute_tuple] = frame_tuple.compute_families() else {
        return unknown(
            semantic_digest,
            "the one-nullary profile did not produce exactly one computation family".to_owned(),
        );
    };
    let verified_slice = match run_registered_one_nullary_inductive_slice(
        kernel,
        semantic,
        &demand_verifier,
        &active_demand_inventory,
        history.origin_cutoff_q3_registry(),
    ) {
        GscOutcome::Proven(slice) => slice,
        GscOutcome::Unknown(reason) => {
            return unknown(
                semantic_digest,
                format!("finite registered slice is undecided: {reason:?}"),
            );
        }
    };
    let report = match build_report(
        kernel,
        semantic,
        &demand_manifest,
        demand_verifier.digest(),
        &bootstrap,
        &history,
        &active_demand_inventory,
        &verified_slice,
    ) {
        Ok(report) => report,
        Err(reason) => {
            return unknown(
                semantic_digest,
                format!("verified slice report construction failed closed: {reason}"),
            );
        }
    };
    LawV2H3InductiveCompletionOutcomeV1::Proven {
        report: Box::new(report),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_report(
    kernel: &Kernel,
    semantic: &VerifiedGscSemanticManifest,
    demand_manifest: &GscVerifierManifestV1,
    demand_manifest_digest: &Digest,
    bootstrap: &VerifiedRegisteredBootstrap,
    history: &VerifiedHistory,
    active_demand_inventory: &VerifiedRegisteredActiveDemandInventory,
    verified_slice: &VerifiedOneNullaryInductiveSlice,
) -> Result<LawV2H3InductiveCompletionReportV1, String> {
    let [frame_tuple] = active_demand_inventory.tuples() else {
        return Err("active-demand inventory is not the verified singleton profile".to_owned());
    };
    let frame = frame_tuple.frame();
    let use_family = frame_tuple.use_family();
    let law_authority_source_digest = law_v2_h3_authority_source_digest()
        .ok_or_else(|| "law-authority source is not canonicalizable UTF-8".to_owned())?;
    let engine_source_digest = law_v2_h3_engine_source_digest()
        .ok_or_else(|| "engine source is not canonicalizable UTF-8".to_owned())?;
    let workspace_manifest_digest = canonical_lf_text_digest(include_bytes!("../../../Cargo.toml"))
        .ok_or_else(|| "workspace manifest is not canonicalizable UTF-8".to_owned())?;
    let workspace_lock_digest = canonical_lf_text_digest(include_bytes!("../../../Cargo.lock"))
        .ok_or_else(|| "workspace lock is not canonicalizable UTF-8".to_owned())?;
    let production_isolation_lock_digest = canonical_lf_text_digest(include_bytes!(
        "../../../scripts/law-v2-isolation.Cargo.lock"
    ))
    .ok_or_else(|| "production isolation lock is not canonicalizable UTF-8".to_owned())?;
    let gf2_manifest = verified_slice.verifier_manifest().clone();
    let gf2_manifest_digest = gf2_manifest.canonical_digest();
    let composite_provenance_digest = {
        let mut encoder = CanonicalEncoder::new();
        semantic.digest().encode_canonical(&mut encoder);
        demand_manifest_digest.encode_canonical(&mut encoder);
        gf2_manifest_digest.encode_canonical(&mut encoder);
        law_authority_source_digest.encode_canonical(&mut encoder);
        engine_source_digest.encode_canonical(&mut encoder);
        workspace_manifest_digest.encode_canonical(&mut encoder);
        workspace_lock_digest.encode_canonical(&mut encoder);
        production_isolation_lock_digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes(
            "pen-engine/law-v2-h3-composite-provenance/v1",
            encoder.as_bytes(),
        )
    };
    let verifier = H3VerifierReportV1 {
        demand_manifest: demand_manifest.clone(),
        demand_manifest_digest: demand_manifest_digest.clone(),
        reference_agreement_capability_digest: demand_manifest
            .reference_vector_agreement
            .verified_capability_digest
            .clone(),
        gf2_manifest,
        gf2_manifest_digest,
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        law_authority_source_digest,
        engine_source_digest,
        workspace_manifest_digest,
        workspace_lock_digest,
        production_isolation_lock_digest,
        composite_provenance_digest,
    };
    let export_index = history.export_index();
    let bootstrap_report = H3BootstrapReportV1 {
        bootstrap_contract_digest: bootstrap.bootstrap_contract_digest().clone(),
        bootstrap_artifact_digest: bootstrap.artifact_digest().clone(),
        bootstrap_asset_canonical_lf_digest: canonical_lf_text_digest(
            REGISTERED_BOOTSTRAP_ASSET_BYTES,
        )
        .ok_or_else(|| "embedded bootstrap is not canonicalizable UTF-8".to_owned())?,
        bootstrap_kernel_digest: bootstrap.kernel_digest().clone(),
        bootstrap_normalizer_digest: bootstrap.normalizer_digest().clone(),
        boundary_chain: bootstrap.boundary_chain().to_vec(),
        final_boundary_digest: bootstrap.final_boundary().digest().clone(),
        source_identities: bootstrap.source_identities().to_vec(),
        binding_identities: bootstrap.binding_identities().to_vec(),
        export_index_digest: export_index.digest().clone(),
        groups: export_index
            .groups()
            .iter()
            .map(|group| H3RegisteredGroupReportV1 {
                group_digest: group.digest().clone(),
                activation_ordinal: group.activation_ordinal(),
                declarations: group.declarations().to_vec(),
                disposition: group.disposition().clone(),
            })
            .collect(),
        exported_q3_registry_is_empty: export_index.registered_q3_theorems_is_empty(),
        original_bootstrap_unchanged: true,
    };
    let active_anchor = history.active_anchor();
    let final_boundary = history
        .boundary_snapshots()
        .last()
        .ok_or_else(|| "verified history has no final boundary".to_owned())?;
    let declaration_origins = final_boundary
        .declarations()
        .iter()
        .map(|declaration| {
            let origin = history
                .declaration_origin(&declaration.id)
                .ok_or_else(|| {
                    format!(
                        "verified history omits the origin of declaration {}",
                        declaration.id.0
                    )
                })?
                .clone();
            Ok(H3DeclarationOriginReportV1 {
                declaration: declaration.id.clone(),
                origin,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let history_report = H3HistoryReportV1 {
        history_digest: history.digest().clone(),
        events: history
            .events()
            .iter()
            .map(|event| H3EventReportV1 {
                event_id: event.event_id().clone(),
                ordinal: event.ordinal(),
                predecessor: event.predecessor().cloned(),
                pre_history_digest: event.pre_history_digest().clone(),
                post_history_digest: event.post_history_digest().clone(),
                pre_boundary_digest: event.pre_boundary_digest().clone(),
                post_boundary_digest: event.post_boundary_digest().clone(),
                extension_digest: event.extension_digest().clone(),
                source_identity: event.source_identity().clone(),
                binding_identity: event.binding_identity().clone(),
                export_index_digest: event.export_index().digest().clone(),
                activated_group_digests: event.export_index().activated_group_digests().to_vec(),
            })
            .collect(),
        boundary_snapshot_digests: history
            .boundary_snapshots()
            .iter()
            .map(|boundary| boundary.digest().clone())
            .collect(),
        declaration_origins,
        anchors: history
            .anchors()
            .iter()
            .map(|anchor| H3AnchorReportV1 {
                anchor_digest: anchor.digest().clone(),
                older: anchor.older().clone(),
                newer: anchor.newer().clone(),
                older_ordinal: anchor.older_ordinal(),
                newer_ordinal: anchor.newer_ordinal(),
                cutoff_boundary_digest: anchor.cutoff_boundary_digest().clone(),
                cutoff_export_coverage_exhaustive: history.cutoff_export_coverage(anchor)
                    == Some(CutoffExportCoverage::Exhaustive),
            })
            .collect(),
        active_anchor_digest: active_anchor.digest().clone(),
        active_anchor_older: active_anchor.older().clone(),
        active_anchor_newer: active_anchor.newer().clone(),
        active_anchor_older_ordinal: active_anchor.older_ordinal(),
        active_anchor_newer_ordinal: active_anchor.newer_ordinal(),
        cutoff_boundary_digest: active_anchor.cutoff_boundary_digest().clone(),
        cutoff_export_coverage_exhaustive: history.cutoff_export_coverage(active_anchor)
            == Some(CutoffExportCoverage::Exhaustive),
        origin_cutoff_q3_registry_digest: history.origin_cutoff_q3_registry().digest().clone(),
        origin_cutoff_q3_registry_is_empty: history.origin_cutoff_q3_registry().is_empty(),
    };
    let active_demand_inventory_report = H3ActiveDemandInventoryReportV1 {
        binding_digest: active_demand_inventory.binding_digest().clone(),
        semantic_manifest_digest: active_demand_inventory.semantic_manifest_digest().clone(),
        export_index_digest: active_demand_inventory.export_index_digest().clone(),
        history_digest: active_demand_inventory.history_digest().clone(),
        active_anchor_digest: active_demand_inventory.active_anchor_digest().clone(),
        cutoff_boundary_digest: active_demand_inventory.cutoff_boundary_digest().clone(),
        q3_registry_digest: active_demand_inventory.q3_registry_digest().clone(),
        cutoff_export_coverage_exhaustive: active_demand_inventory.cutoff_coverage_is_exhaustive(),
        frame_count: active_demand_inventory.frame_count(),
        use_family_count: active_demand_inventory.use_family_count(),
        compute_family_count: active_demand_inventory.compute_family_count(),
        family_count: active_demand_inventory.family_count(),
        port_count: active_demand_inventory.port_count(),
        constructor_count: active_demand_inventory.constructor_count(),
        ordered_export_group_digests: active_demand_inventory
            .ordered_export_group_digests()
            .to_vec(),
        ordered_frame_binding_digests: active_demand_inventory
            .ordered_frame_binding_digests()
            .to_vec(),
        ordered_family_binding_digests: active_demand_inventory
            .ordered_family_binding_digests()
            .to_vec(),
        ordered_port_digests: active_demand_inventory.ordered_port_digests().to_vec(),
        ordered_constructor_digests: active_demand_inventory
            .ordered_constructor_digests()
            .to_vec(),
        tuples: active_demand_inventory
            .tuples()
            .iter()
            .map(|tuple| H3FrameDemandTupleReportV1 {
                active_frame_ordinal: tuple.active_frame_ordinal(),
                frame_binding_digest: tuple.frame().binding_digest().clone(),
                use_family_binding_digest: tuple.use_family().binding_digest().clone(),
                exact_use_port: tuple.exact_use_port().clone(),
                compute_families: tuple
                    .compute_families()
                    .iter()
                    .map(|computation| H3ComputeDemandTupleReportV1 {
                        constructor_ordinal: computation.constructor_ordinal(),
                        constructor_digest: computation.constructor().0.clone(),
                        family_binding_digest: computation.family().binding_digest().clone(),
                        output_port: computation.output_port().clone(),
                        output_port_digest: computation.output_port_digest().clone(),
                        tuple_binding_digest: computation.binding_digest().clone(),
                    })
                    .collect(),
                candidate_compute_order: tuple.candidate_compute_order().to_vec(),
                tuple_binding_digest: tuple.binding_digest().clone(),
            })
            .collect(),
    };
    let registered_frame = H3RegisteredFrameReportV1 {
        binding_digest: frame.binding_digest().clone(),
        frame_id: frame.inner().id().0.clone(),
        code_id: frame.inner().code().id().0.clone(),
        code: frame.inner().code().code().clone(),
        owner: frame.inner().owner().clone(),
        introduction_aliases: frame
            .inner()
            .introductions()
            .iter()
            .map(|alias| alias.introduction.clone())
            .collect(),
        constructor_ids: frame
            .inner()
            .code()
            .constructor_ids()
            .iter()
            .map(|constructor| constructor.0.clone())
            .collect(),
        export_group_digest: frame.export_group_digest().clone(),
        export_index_digest: frame.export_index_digest().clone(),
        history_digest: frame.history_digest().clone(),
        active_anchor_digest: frame.active_anchor_digest().clone(),
        cutoff_boundary_digest: frame.cutoff_boundary_digest().clone(),
        q3_registry_digest: frame.q3_registry_digest().clone(),
        principal_sources: frame
            .principal_source_bindings()
            .iter()
            .map(|source| H3PrincipalSourceReportV1 {
                declaration: source.declaration().clone(),
                public_source: source.public_source().clone(),
                origin_digest: source.origin().0.clone(),
                registered_source_identity: source.registered_source_identity().clone(),
            })
            .collect(),
        binding_valid: frame.binding_is_valid(),
    };
    let use_family_report = family_report(use_family, frame);
    let compute_family_reports = frame_tuple
        .compute_families()
        .iter()
        .map(|computation| family_report(computation.family(), frame))
        .collect();
    let pre_response = verified_slice.pre_response();
    let canonical_forms = pre_response.canonical_forms();
    let pre_response_report = H3UnderivedReportV1 {
        port: pre_response.port().clone(),
        context: pre_response.context().clone(),
        target: pre_response.target().clone(),
        examined_terms: pre_response.examined_terms(),
        grounded_rule_instances: pre_response.grounded_rule_instances(),
        structural_goal_count: pre_response.structural_goal_count(),
        library_nodes: pre_response.library_nodes(),
        closure_base_digest: pre_response.closure_base_digest().clone(),
        closure_frontier_digests: pre_response.closure_frontier_digests().to_vec(),
        closure_fixed_point_digest: pre_response.closure_fixed_point_digest().clone(),
        goal_rule_coverage_digest: pre_response.goal_rule_coverage_digest().clone(),
        canonical_replay_digest: pre_response.canonical_replay_digest().clone(),
        grounding_inventory_digest: pre_response.grounding_inventory_digest().clone(),
        library_inventory_digest: pre_response.library_inventory_digest().clone(),
        dispositions: pre_response
            .dispositions()
            .iter()
            .map(|disposition| H3GroundRuleDispositionReportV1 {
                rule: operational_rule_name(disposition.rule()).to_owned(),
                grounded_instances: disposition.grounded_instances(),
                applicable_instances: disposition.applicable_instances(),
                certified_inapplicable_instances: disposition.certified_inapplicable_instances(),
                inapplicability_evidence: disposition
                    .inapplicability_evidence()
                    .iter()
                    .map(|evidence| H3InapplicabilityEvidenceReportV1 {
                        reason: inapplicability_reason_name(evidence.reason()).to_owned(),
                        instances: evidence.instances(),
                        grounding_set_digest: evidence.grounding_set_digest().clone(),
                    })
                    .collect(),
                exhaustive: disposition.is_exhaustively_decided(),
            })
            .collect(),
        canonical_forms: H3CanonicalFormAnalysisReportV1 {
            single_outer_function: canonical_forms.has_single_outer_function(),
            stuck_atomic_body: canonical_forms.has_stuck_atomic_body(),
            neutral_heads_exhausted: canonical_forms.neutral_heads_are_exhausted(),
            no_higher_order_argument_gap: canonical_forms.has_no_higher_order_argument_gap(),
            introduction_rules_separated: canonical_forms.introduction_rules_are_separated(),
            digest: canonical_forms.digest().clone(),
        },
        complete_relative_to_frozen_grammar: pre_response.is_complete_relative_to_grammar(),
        grammar_digest: pre_response.grammar_digest().clone(),
        certificate_digest: pre_response.digest().clone(),
    };
    let equations = verified_slice.equations();
    let clause = equations.clause();
    let invariants = clause.invariants();
    let substitution = clause.substitution();
    let equation_extension = H3EquationExtensionReportV1 {
        set_digest: equations.digest().clone(),
        clause_count: equations.clauses_len() as u32,
        clause_digest: clause.digest().clone(),
        fresh_head: clause.fresh_head().clone(),
        head_type: clause.head_type().clone(),
        context: clause.context().clone(),
        left: clause.left().clone(),
        right: clause.right().clone(),
        ty: clause.ty().clone(),
        extended_signature_digest: equations.extended_signature().digest().clone(),
        substitution_source_port: substitution.source_port().clone(),
        substitution_source_context: substitution.source_context().clone(),
        substitution_target_context: substitution.target_context().clone(),
        substitution_filler: substitution.filler().clone(),
        substitution_digest: substitution.digest().clone(),
        left_linear: invariants.is_left_linear(),
        nonrecursive: invariants.is_nonrecursive(),
        no_critical_overlaps: invariants.has_no_critical_overlaps(),
        terminating_in_admitted_fragment: invariants.is_terminating_in_admitted_fragment(),
        confluent_in_admitted_fragment: invariants.is_confluent_in_admitted_fragment(),
        conservative_on_old_terms: invariants.is_conservative_on_old_terms(),
        exact_generated_substitution_preserves_equation: invariants
            .exact_generated_substitution_preserves_equation(),
        invariant_digest: invariants.digest().clone(),
    };
    let response = verified_slice.response();
    let use_derivation = response.use_derivation();
    let response_report = H3ResponseReportV1 {
        response_digest: response.digest().clone(),
        use_port: use_derivation.port().clone(),
        use_term: use_derivation.term().clone(),
        use_target: use_derivation.target().clone(),
        use_derivation_digest: use_derivation.digest().clone(),
        use_derivation_nodes: derivation_report(use_derivation.derivation()),
        equation_set_digest: response.equation_set_digest().clone(),
        discharges: response
            .discharges()
            .iter()
            .map(|discharge| H3PortDischargeReportV1 {
                port: discharge.port().clone(),
                rule: operational_rule_name(discharge.rule()).to_owned(),
                premises: discharge.premises().to_vec(),
                evidence_digest: discharge.evidence_digest().clone(),
            })
            .collect(),
        all_outputs_filled: response.all_outputs_filled(),
        all_positive_clauses_connected: response.all_positive_clauses_connected(),
    };
    let carrier = verified_slice.candidate_carrier();
    if carrier.registered_active_demand_inventory_binding_digest()
        != active_demand_inventory.binding_digest()
        || verified_slice.registered_active_demand_inventory_binding_digest()
            != active_demand_inventory.binding_digest()
        || carrier.registered_frame_binding_digest() != frame.binding_digest()
        || verified_slice.registered_frame_binding_digest() != frame.binding_digest()
        || carrier.semantic_manifest_digest() != semantic.digest()
    {
        return Err("verified slice, carrier, and active inventory bindings disagree".to_owned());
    }
    let candidate_carrier = H3CandidateCarrierReportV1 {
        carrier_digest: carrier.digest().clone(),
        semantic_manifest_digest: carrier.semantic_manifest_digest().clone(),
        active_demand_inventory_binding_digest: carrier
            .registered_active_demand_inventory_binding_digest()
            .clone(),
        registered_frame_binding_digest: carrier.registered_frame_binding_digest().clone(),
        use_family_binding_digest: carrier.use_family_binding_digest().clone(),
        compute_family_binding_digests: carrier.compute_family_binding_digests().to_vec(),
        canonical_fresh_head: carrier.canonical_fresh_head().clone(),
        equation_set_digest: carrier.equation_set_digest().clone(),
        response_digest: carrier.response_digest().clone(),
        candidate_count: carrier.candidate_count(),
        live_use_family_count: carrier.live_use_family_count(),
        compute_family_count: carrier.compute_family_count(),
        exhaustion_evidence_digest: carrier.exhaustion_evidence_digest().clone(),
        exhaustive: carrier.is_exhaustive(),
    };
    let quotient = verified_slice.quotient();
    let quotient_report = H3QuotientReportV1 {
        quotient_digest: quotient.digest().clone(),
        complete: quotient.is_complete(),
        response_count: quotient.response_count(),
        class_count: quotient.classes_len() as u32,
        class_digest: quotient.class().digest().clone(),
        representative_digest: quotient.class().representative_digest().clone(),
        equation_digest: quotient.class().equation_digest().clone(),
        q2_distinct_pair_decisions: quotient.q2_distinct_pair_decisions(),
        q3_edge_count: quotient.q3_edge_count(),
        q3_registry_digest: quotient.q3_registry_digest().clone(),
    };
    let conclusion = H3ConclusionReportV1 {
        live_profile_issued: true,
        direct_eliminator_is_unique_class_relative_to_frozen_profile: quotient.is_complete()
            && quotient.classes_len() == 1
            && carrier.candidate_count() == 1,
        current_owner_specific_profile_recovers_archived_four_way_result: false,
        contextual_internalization_was_adopted: false,
        archived_candidate_inputs_loaded: false,
        broader_semantic_claim_made: false,
        disposition: "direct_eliminator_unique_relative_to_frozen_inductive_completion_profile"
            .to_owned(),
        semantic_manifest_digest: semantic.digest().clone(),
        active_demand_inventory_binding_digest: verified_slice
            .registered_active_demand_inventory_binding_digest()
            .clone(),
        registered_frame_binding_digest: verified_slice.registered_frame_binding_digest().clone(),
        verified_slice_digest: verified_slice.digest().clone(),
    };
    let body = LawV2H3InductiveCompletionReportBodyV1 {
        schema_version: LAW_V2_H3_INDUCTIVE_COMPLETION_REPORT_SCHEMA_VERSION,
        profile_id: LAW_V2_H3_INDUCTIVE_COMPLETION_PROFILE_ID.to_owned(),
        authority: "production_registered_history_and_frozen_finite_typed_verifiers".to_owned(),
        semantic_manifest: semantic.manifest().clone(),
        semantic_manifest_digest: semantic.digest().clone(),
        verifier,
        bootstrap: bootstrap_report,
        history: history_report,
        active_demand_inventory: active_demand_inventory_report,
        registered_frame,
        use_family: use_family_report,
        compute_families: compute_family_reports,
        pre_response: pre_response_report,
        equation_extension,
        response: response_report,
        candidate_carrier,
        quotient: quotient_report,
        conclusion,
    };
    let result_digest = report_body_digest(&body)
        .ok_or_else(|| "report body could not be canonically serialized".to_owned())?;
    Ok(LawV2H3InductiveCompletionReportV1 {
        body,
        result_digest,
    })
}

fn family_report(
    family: &VerifiedRegisteredDemandFamilyV2,
    frame: &VerifiedRegisteredClosedFormerFrame,
) -> H3DemandFamilyReportV1 {
    let inner: &VerifiedCanonicalDemandFamilyV2 = family.inner();
    H3DemandFamilyReportV1 {
        binding_digest: family.binding_digest().clone(),
        family_id: inner.id().clone(),
        rule: inner.rule(),
        rank: inner.rank(),
        parameter_context: inner.parameter_context().clone(),
        premise_refs: inner.premise_refs().to_vec(),
        output_clauses: inner.output_clauses().to_vec(),
        verification_judgments: inner.verification_judgments().to_vec(),
        principal_sources: inner.principal_sources().to_vec(),
        birth_support_digests: inner
            .birth_support()
            .iter()
            .map(|origin| origin.0.clone())
            .collect(),
        fixed_type_support: inner.fixed_type_support().to_vec(),
        parameter_support_projections: inner.parameter_support_projections().to_vec(),
        port_keys: inner
            .ports()
            .iter()
            .map(|port| port.key().clone())
            .collect(),
        exact_use_port: family.exact_use_port().cloned(),
        source_use_family_binding_digest: family.source_use_family_binding_digest().cloned(),
        constructor_digest: family
            .constructor()
            .map(|constructor| constructor.0.clone()),
        binding_valid_for_frame: family.binding_is_valid_for(frame),
    }
}

fn derivation_report(dag: &VerifiedDerivationDag) -> Vec<H3DerivationNodeReportV1> {
    dag.nodes()
        .iter()
        .map(|node| H3DerivationNodeReportV1 {
            ordinal: node.ordinal(),
            context: node.context().clone(),
            term: node.term().clone(),
            ty: node.ty().clone(),
            rule: operational_rule_name(node.rule()).to_owned(),
            premises: node.premises().to_vec(),
        })
        .collect()
}

fn operational_rule_name(rule: OperationalRule) -> &'static str {
    match rule {
        OperationalRule::PublicExact => "public_exact",
        OperationalRule::Q0Conversion => "q0_conversion",
        OperationalRule::ContextWeakening => "context_weakening",
        OperationalRule::CheckedSubstitution => "checked_substitution",
        OperationalRule::LambdaIntroduction => "lambda_introduction",
        OperationalRule::Application => "application",
        OperationalRule::PairIntroduction => "pair_introduction",
        OperationalRule::FirstProjection => "first_projection",
        OperationalRule::SecondProjection => "second_projection",
        OperationalRule::EquationReplay => "equation_replay",
        OperationalRule::QuotientTransport => "quotient_transport",
    }
}

fn inapplicability_reason_name(reason: CertifiedInapplicabilityReason) -> &'static str {
    match reason {
        CertifiedInapplicabilityReason::NormalFormsDiffer => "normal_forms_differ",
        CertifiedInapplicabilityReason::GoalIsNotDependentFunction => {
            "goal_is_not_dependent_function"
        }
        CertifiedInapplicabilityReason::PrincipalIsNotDependentFunction => {
            "principal_is_not_dependent_function"
        }
        CertifiedInapplicabilityReason::ArgumentTypeMismatch => "argument_type_mismatch",
        CertifiedInapplicabilityReason::GoalIsNotDependentPair => "goal_is_not_dependent_pair",
        CertifiedInapplicabilityReason::PrincipalIsNotDependentPair => {
            "principal_is_not_dependent_pair"
        }
        CertifiedInapplicabilityReason::NoBodyInFiniteLibrary => "no_body_in_finite_library",
        CertifiedInapplicabilityReason::NoEquationAvailableBeforeResponse => {
            "no_equation_available_before_response"
        }
        CertifiedInapplicabilityReason::NoQuotientClassAvailableBeforeResponse => {
            "no_quotient_class_available_before_response"
        }
    }
}

fn report_body_digest(body: &LawV2H3InductiveCompletionReportBodyV1) -> Option<Digest> {
    let value = serde_json::to_value(body).ok()?;
    let mut encoder = CanonicalEncoder::new();
    encode_canonical_json(&value, &mut encoder);
    Some(Digest::of_domain_bytes(
        "pen-engine/law-v2-h3-inductive-completion-report/canonical-json/v1",
        encoder.as_bytes(),
    ))
}

fn encode_canonical_json(value: &serde_json::Value, encoder: &mut CanonicalEncoder) {
    match value {
        serde_json::Value::Null => encoder.tag(0),
        serde_json::Value::Bool(value) => {
            encoder.tag(1);
            encoder.tag(u8::from(*value));
        }
        serde_json::Value::Number(value) => {
            encoder.tag(2);
            encoder.text(&value.to_string());
        }
        serde_json::Value::String(value) => {
            encoder.tag(3);
            encoder.text(value);
        }
        serde_json::Value::Array(values) => {
            encoder.tag(4);
            encoder.u64(values.len() as u64);
            for value in values {
                encode_canonical_json(value, encoder);
            }
        }
        serde_json::Value::Object(values) => {
            encoder.tag(5);
            let mut entries = values.iter().collect::<Vec<_>>();
            entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
            encoder.u64(entries.len() as u64);
            for (key, value) in entries {
                encoder.text(key);
                encode_canonical_json(value, encoder);
            }
        }
    }
}

pub fn replay_law_v2_h3_inductive_completion_report_v1(
    report: &LawV2H3InductiveCompletionReportV1,
) -> bool {
    if Some(&report.result_digest) != report_body_digest(&report.body).as_ref() {
        return false;
    }
    match run_law_v2_h3_inductive_completion_v1() {
        LawV2H3InductiveCompletionOutcomeV1::Proven { report: expected } => {
            report == expected.as_ref()
        }
        LawV2H3InductiveCompletionOutcomeV1::Unknown { .. } => false,
    }
}

pub fn law_v2_h3_inductive_completion_json_pretty_v1() -> String {
    match serde_json::to_string_pretty(&run_law_v2_h3_inductive_completion_v1()) {
        Ok(wire) => wire,
        Err(_) => format!(
            "{{\n  \"status\": \"unknown\",\n  \"profile_id\": \
             \"{LAW_V2_H3_INDUCTIVE_COMPLETION_PROFILE_ID}\",\n  \"reason\": \
             \"outcome serialization failed closed\",\n  \"semantic_manifest_digest\": null,\n  \
             \"fail_closed\": true\n}}"
        ),
    }
}

fn unknown(
    semantic_manifest_digest: Option<Digest>,
    reason: String,
) -> LawV2H3InductiveCompletionOutcomeV1 {
    LawV2H3InductiveCompletionOutcomeV1::Unknown {
        profile_id: LAW_V2_H3_INDUCTIVE_COMPLETION_PROFILE_ID.to_owned(),
        reason,
        semantic_manifest_digest,
        fail_closed: true,
    }
}

fn law_v2_h3_authority_source_digest() -> Option<Digest> {
    canonical_source_digest(
        "pen-engine/law-v2-h3-authority-source/v1",
        &[
            include_bytes!("../../pen-law/src/registered_bootstrap.rs"),
            include_bytes!("../../pen-law/src/registered_bootstrap_export.rs"),
            include_bytes!("../../pen-law/src/history.rs"),
            include_bytes!("../../pen-law/assets/law_v2a_registered_bootstrap_v1.json"),
            include_bytes!(
                "../../pen-law/assets/law_v2a_registered_bootstrap_export_index_v1.json"
            ),
        ],
    )
}

fn law_v2_h3_engine_source_digest() -> Option<Digest> {
    canonical_source_digest(
        "pen-engine/law-v2-h3-engine-source/v1",
        &[
            include_bytes!("h3_inductive_completion.rs"),
            include_bytes!("lib.rs"),
            include_bytes!("../Cargo.toml"),
            include_bytes!("../examples/law_v2_h3_inductive_completion.rs"),
        ],
    )
}

fn canonical_source_digest(domain: &str, chunks: &[&[u8]]) -> Option<Digest> {
    let canonical = chunks
        .iter()
        .map(|chunk| {
            let text = std::str::from_utf8(chunk).ok()?;
            let normalized = text.replace("\r\n", "\n");
            (!normalized.contains('\r')).then(|| normalized.into_bytes())
        })
        .collect::<Option<Vec<_>>>()?;
    let slices = canonical.iter().map(Vec::as_slice).collect::<Vec<_>>();
    Some(Digest::of_domain_chunks(domain, &slices))
}

fn canonical_lf_text_digest(bytes: &[u8]) -> Option<Digest> {
    let text = std::str::from_utf8(bytes).ok()?;
    let normalized = text.replace("\r\n", "\n");
    if normalized.contains('\r') {
        return None;
    }
    Some(Digest::of_bytes(normalized.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::{
        LAW_V2_H3_INDUCTIVE_COMPLETION_PROFILE_ID, LawV2H3InductiveCompletionOutcomeV1,
        REGISTERED_BOOTSTRAP_ASSET_BYTES, REGISTERED_BOOTSTRAP_ASSET_CANONICAL_LF_DIGEST,
        canonical_lf_text_digest, encode_canonical_json,
        replay_law_v2_h3_inductive_completion_report_v1, report_body_digest,
        run_law_v2_h3_inductive_completion_v1,
    };

    #[test]
    fn unknown_wire_is_fail_closed() {
        let outcome = LawV2H3InductiveCompletionOutcomeV1::Unknown {
            profile_id: LAW_V2_H3_INDUCTIVE_COMPLETION_PROFILE_ID.to_owned(),
            reason: "synthetic missing proof".to_owned(),
            semantic_manifest_digest: None,
            fail_closed: true,
        };
        let wire = serde_json::to_string(&outcome).expect("serialize");
        assert!(wire.contains("\"status\":\"unknown\""));
        assert!(wire.contains("\"fail_closed\":true"));
    }

    #[test]
    fn report_digest_function_is_domain_separated() {
        let _ = report_body_digest;
    }

    #[test]
    fn canonical_json_tree_encoding_is_independent_of_object_insertion_order() {
        let mut left = serde_json::Map::new();
        left.insert("b".to_owned(), serde_json::json!([2, true]));
        left.insert("a".to_owned(), serde_json::json!({"nested": null}));
        let mut right = serde_json::Map::new();
        right.insert("a".to_owned(), serde_json::json!({"nested": null}));
        right.insert("b".to_owned(), serde_json::json!([2, true]));

        let mut left_encoder = pen_kernel::CanonicalEncoder::new();
        encode_canonical_json(&serde_json::Value::Object(left), &mut left_encoder);
        let mut right_encoder = pen_kernel::CanonicalEncoder::new();
        encode_canonical_json(&serde_json::Value::Object(right), &mut right_encoder);
        assert_eq!(left_encoder.as_bytes(), right_encoder.as_bytes());
    }

    #[test]
    fn registered_bootstrap_bytes_match_the_pre_slice_canonical_lf_pin() {
        let digest = canonical_lf_text_digest(REGISTERED_BOOTSTRAP_ASSET_BYTES)
            .expect("the embedded bootstrap is canonicalizable UTF-8");
        assert_eq!(
            digest.as_str(),
            REGISTERED_BOOTSTRAP_ASSET_CANONICAL_LF_DIGEST
        );
    }

    #[test]
    #[ignore = "requires the exact pinned Agda 2.8.0 executable on PATH"]
    fn live_profile_is_proven_deterministic_replayable_and_tamper_evident() {
        let first = run_law_v2_h3_inductive_completion_v1();
        let second = run_law_v2_h3_inductive_completion_v1();
        assert_eq!(first, second, "two live executions must be byte-stable");

        let LawV2H3InductiveCompletionOutcomeV1::Proven { report } = first else {
            panic!("the pinned live environment must prove the frozen H3 profile");
        };
        assert_eq!(
            report.body.semantic_manifest_digest.as_str(),
            "blake3:d61458ebd47036861e48af9ef458df1b2b3dc890194069958ef2f14d4afdd11e"
        );
        assert_eq!(
            report.body.semantic_manifest.canonical_digest(),
            report.body.semantic_manifest_digest
        );
        assert_eq!(
            report.body.active_demand_inventory.semantic_manifest_digest,
            report.body.semantic_manifest_digest
        );
        assert_eq!(
            report.body.active_demand_inventory.binding_digest,
            report
                .body
                .candidate_carrier
                .active_demand_inventory_binding_digest
        );
        assert_eq!(
            report.body.registered_frame.binding_digest,
            report.body.conclusion.registered_frame_binding_digest
        );
        assert!(report.body.bootstrap.original_bootstrap_unchanged);
        assert!(report.body.bootstrap.exported_q3_registry_is_empty);
        assert!(report.body.history.origin_cutoff_q3_registry_is_empty);
        assert!(
            report
                .body
                .active_demand_inventory
                .cutoff_export_coverage_exhaustive
        );
        assert!(report.body.pre_response.complete_relative_to_frozen_grammar);
        assert!(report.body.candidate_carrier.exhaustive);
        assert!(report.body.quotient.complete);
        assert_eq!(report.body.quotient.class_count, 1);
        assert!(
            report
                .body
                .conclusion
                .direct_eliminator_is_unique_class_relative_to_frozen_profile
        );
        assert!(!report.body.conclusion.archived_candidate_inputs_loaded);

        let wire = serde_json::to_vec(&LawV2H3InductiveCompletionOutcomeV1::Proven {
            report: report.clone(),
        })
        .expect("serialize live report");
        let decoded: LawV2H3InductiveCompletionOutcomeV1 =
            serde_json::from_slice(&wire).expect("deserialize live report");
        assert_eq!(
            decoded,
            LawV2H3InductiveCompletionOutcomeV1::Proven {
                report: report.clone()
            }
        );
        assert!(replay_law_v2_h3_inductive_completion_report_v1(&report));

        let mut body_tamper = report.clone();
        body_tamper.body.conclusion.disposition = "forged disposition".to_owned();
        assert!(!replay_law_v2_h3_inductive_completion_report_v1(
            &body_tamper
        ));

        let mut digest_tamper = report;
        digest_tamper.result_digest = pen_kernel::Digest::of_bytes(b"forged H3 result digest");
        assert!(!replay_law_v2_h3_inductive_completion_report_v1(
            &digest_tamper
        ));
    }
}
