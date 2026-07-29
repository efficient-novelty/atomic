//! Blind prospective continuation after the accepted one-nullary H3 response.
//!
//! The continuation adds no new demand rule.  It freely seals the unique H3
//! class, advances the width-two window, and decides the complete supported
//! inventory against the cumulative library.

use crate::h3_inductive_completion::{
    REGISTERED_BOOTSTRAP_ASSET_BYTES, REGISTERED_BOOTSTRAP_ASSET_CANONICAL_LF_DIGEST,
    canonical_lf_text_digest, canonical_source_digest, encode_canonical_json,
};
use pen_demand::gsc::{
    GscFamilyId, GscOutcome, GscReferenceVectorAgreement, GscRule, GscSemanticManifestV1,
    current_gsc_verifier_manifest_v1, frozen_gsc_semantic_manifest_v1,
    verify_gsc_reference_agreement_v1, verify_gsc_semantic_manifest_v1,
    verify_gsc_verifier_manifest_v1,
};
use pen_gf2::{
    Gf2SliceVerifierManifestV1, LawV2ContinuationSemanticManifestV1, VerifiedH4HaltCertificateV1,
    continue_one_nullary_inductive_completion_v1, frozen_law_v2_continuation_semantic_manifest_v1,
    run_registered_one_nullary_inductive_slice, verify_law_v2_continuation_semantic_manifest_v1,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Declaration, Digest, Kernel, KernelLimits};
use pen_law::{
    EventId, load_embedded_registered_bootstrap,
    load_embedded_registered_bootstrap_export_index_v1, verify_registered_bootstrap_history,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const LAW_V2_H4_CONTINUATION_REPORT_SCHEMA_VERSION: u16 = 1;
pub const LAW_V2_H4_CONTINUATION_PROFILE_ID: &str =
    "law-v2-owner-specific-inductive-continuation-v1";

const ACCEPTED_H3_RESULT_DIGEST: &str =
    "blake3:f43acaf6f0b0b9e51dc9a55829eedbc1fb2f7cf1090260ac2d244d1616a27d03";
const FROZEN_CONTINUATION_SEMANTIC_DIGEST: &str =
    "blake3:00d97ce3446442d91b8572557c16dd0576f7281260b5c000e64f41323323c7e2";
const ACCEPTED_H3_SEMANTIC_DIGEST: &str =
    "blake3:d61458ebd47036861e48af9ef458df1b2b3dc890194069958ef2f14d4afdd11e";
const ACCEPTED_H3_INVENTORY_DIGEST: &str =
    "blake3:3373e6a168e258a39384bef307c2fc91dbf257040e35ccef5450ccca1de04150";
const ACCEPTED_H3_FRAME_DIGEST: &str =
    "blake3:e23a5a4f86992163d0ad8ea7cc302c372cb5917156499856603c3cb108f7ac8f";
const ACCEPTED_H3_PRE_RESPONSE_DIGEST: &str =
    "blake3:7266d46172a975b00911c9608548e74b013738d82ba12b9e2544c0612215a15b";
const ACCEPTED_H3_EQUATION_DIGEST: &str =
    "blake3:6c680a188c72efd47a1f4a0a7cfa1b88018c3f345b09bd7eaa39d79dda0ecdc3";
const ACCEPTED_H3_RESPONSE_DIGEST: &str =
    "blake3:6670fc8f9477dfd7dda71cc539bcb5763c895bd0e3c5381fabe180c0ff19828d";
const ACCEPTED_H3_CARRIER_DIGEST: &str =
    "blake3:b98385b8405a06e2e76cc9412129fe2139f5f4a738abe199f829dfa308ea01cb";
const ACCEPTED_H3_QUOTIENT_DIGEST: &str =
    "blake3:758d478c94a4b1b9b8ac4ae17cf0017590df6077852697c4f565ac981acff950";

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LawV2H4ContinuationOutcomeV1 {
    Halted {
        report: Box<LawV2H4ContinuationReportV1>,
    },
    Unknown {
        profile_id: String,
        reason: String,
        base_semantic_manifest_digest: Option<Digest>,
        continuation_semantic_manifest_digest: Option<Digest>,
        fail_closed: bool,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LawV2H4ContinuationReportV1 {
    #[serde(flatten)]
    pub body: LawV2H4ContinuationReportBodyV1,
    pub result_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LawV2H4ContinuationReportBodyV1 {
    pub schema_version: u16,
    pub profile_id: String,
    pub authority: String,
    pub base_semantic_manifest: GscSemanticManifestV1,
    pub base_semantic_manifest_digest: Digest,
    pub continuation_semantic_manifest: LawV2ContinuationSemanticManifestV1,
    pub continuation_semantic_manifest_digest: Digest,
    pub accepted_h3: AcceptedH3BindingReportV1,
    pub verifier: H4VerifierReportV1,
    pub free_seal: H4FreeSealReportV1,
    pub prospective_census: H4CensusReportV1,
    pub halt: H4HaltReportV1,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptedH3BindingReportV1 {
    pub result_digest: Digest,
    pub semantic_manifest_digest: Digest,
    pub active_demand_inventory_binding_digest: Digest,
    pub registered_frame_binding_digest: Digest,
    pub pre_response_certificate_digest: Digest,
    pub equation_set_digest: Digest,
    pub response_digest: Digest,
    pub candidate_carrier_digest: Digest,
    pub quotient_digest: Digest,
    pub semantic_components_replayed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H4VerifierReportV1 {
    pub demand_verifier_manifest_digest: Digest,
    pub gf2_slice_verifier_manifest: Gf2SliceVerifierManifestV1,
    pub gf2_slice_verifier_manifest_digest: Digest,
    pub continuation_verifier_source_digest: Digest,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub workspace_manifest_digest: Digest,
    pub workspace_lock_digest: Digest,
    pub production_isolation_lock_digest: Digest,
    pub composite_provenance_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H4FreeSealReportV1 {
    pub ordinal: u16,
    pub predecessor: EventId,
    pub event_id: EventId,
    pub pre_history_digest: Digest,
    pub post_history_digest: Digest,
    pub pre_boundary_digest: Digest,
    pub post_boundary_digest: Digest,
    pub extension: Declaration,
    pub extension_digest: Digest,
    pub source_identity: Digest,
    pub binding_identity: Digest,
    pub event_export_digest: Digest,
    pub activated_closed_former_group_count: u32,
    pub equation_set_digest: Digest,
    pub response_digest: Digest,
    pub quotient_digest: Digest,
    pub active_anchor_digest: Digest,
    pub empty_q3_registry_digest: Digest,
    pub initiality_certificate_digest: Digest,
    pub free_seal_digest: Digest,
    pub exact_old_boundary_prefix_preserved: bool,
    pub exact_one_bodyless_generator: bool,
    pub exact_one_generated_equation: bool,
    pub no_additional_generator_or_equation: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H4DemandDecisionReportV1 {
    pub family_id: GscFamilyId,
    pub rule: GscRule,
    pub port: pen_demand::gsc::PortKey,
    pub disposition: String,
    pub evidence_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H4CensusReportV1 {
    pub active_older: EventId,
    pub active_newer: EventId,
    pub active_anchor_digest: Digest,
    pub registered_inventory_binding_digest: Digest,
    pub continuation_inventory_binding_digest: Digest,
    pub active_frame_binding_digests: Vec<Digest>,
    pub decisions: Vec<H4DemandDecisionReportV1>,
    pub extracted_family_count: u32,
    pub extracted_port_count: u32,
    pub derived_port_count: u32,
    pub live_orbit_count: u32,
    pub extraction_complete: bool,
    pub derivability_complete: bool,
    pub expiration_complete: bool,
    pub cumulative_weakening_replay_digest: Digest,
    pub extraction_digest: Digest,
    pub derivability_digest: Digest,
    pub expiration_digest: Digest,
    pub census_digest: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H4HaltReportV1 {
    pub halted_after_sealed_act_count: u16,
    pub prospective_stage_ordinal: u16,
    pub positive_cost_exclusion_digest: Digest,
    pub halt_certificate_digest: Digest,
    pub no_new_structure_generated: bool,
    pub disposition: String,
    pub scope: String,
}

pub fn law_v2_h4_continuation_manifest_json_pretty_v1() -> String {
    let kernel = match Kernel::new(KernelLimits::default()) {
        Ok(kernel) => kernel,
        Err(error) => {
            return format!(
                "{{\n  \"status\": \"unknown\",\n  \"reason\": \"kernel initialization failed: \
                 {error}\"\n}}"
            );
        }
    };
    let base = match verify_gsc_semantic_manifest_v1(&frozen_gsc_semantic_manifest_v1()) {
        GscOutcome::Proven(base) => base,
        GscOutcome::Unknown(reason) => {
            return format!(
                "{{\n  \"status\": \"unknown\",\n  \"reason\": \"base semantic manifest failed: \
                 {reason:?}\"\n}}"
            );
        }
    };
    if kernel.kernel_protocol_digest().as_str().is_empty() {
        return "{\"status\":\"unknown\",\"reason\":\"empty kernel identity\"}".to_owned();
    }
    let manifest = frozen_law_v2_continuation_semantic_manifest_v1(&base);
    let digest = manifest.canonical_digest();
    serde_json::to_string_pretty(&serde_json::json!({
        "status": "frozen",
        "manifest": manifest,
        "semantic_manifest_digest": digest,
    }))
    .unwrap_or_else(|_| {
        "{\"status\":\"unknown\",\"reason\":\"manifest serialization failed\"}".to_owned()
    })
}

pub fn run_law_v2_h4_continuation_v1() -> LawV2H4ContinuationOutcomeV1 {
    let kernel = match Kernel::new(KernelLimits::default()) {
        Ok(kernel) => kernel,
        Err(error) => return unknown(None, None, format!("kernel initialization failed: {error}")),
    };
    let base = match verify_gsc_semantic_manifest_v1(&frozen_gsc_semantic_manifest_v1()) {
        GscOutcome::Proven(base) => base,
        GscOutcome::Unknown(reason) => {
            return unknown(
                None,
                None,
                format!("base semantic manifest failed: {reason:?}"),
            );
        }
    };
    let base_digest = Some(base.digest().clone());
    if base.digest().as_str() != ACCEPTED_H3_SEMANTIC_DIGEST {
        return unknown(
            base_digest,
            None,
            "base semantic digest differs from the accepted H3 freeze".to_owned(),
        );
    }
    let continuation_raw = frozen_law_v2_continuation_semantic_manifest_v1(&base);
    let continuation =
        match verify_law_v2_continuation_semantic_manifest_v1(&base, &continuation_raw) {
            GscOutcome::Proven(manifest) => manifest,
            GscOutcome::Unknown(reason) => {
                return unknown(
                    base_digest,
                    None,
                    format!("continuation semantic manifest failed: {reason:?}"),
                );
            }
        };
    let continuation_digest = Some(continuation.digest().clone());
    if continuation.digest().as_str() != FROZEN_CONTINUATION_SEMANTIC_DIGEST {
        return unknown(
            base_digest,
            continuation_digest,
            "continuation semantic digest differs from its pre-run freeze".to_owned(),
        );
    }

    let agreement = match verify_gsc_reference_agreement_v1(&kernel, &base) {
        GscOutcome::Proven(agreement) => agreement,
        GscOutcome::Unknown(reason) => {
            return unknown(
                base_digest,
                continuation_digest,
                format!("generic Rust/Agda agreement failed: {reason:?}"),
            );
        }
    };
    let demand_manifest = match current_gsc_verifier_manifest_v1(&base, &kernel, &agreement) {
        GscOutcome::Proven(manifest) => manifest,
        GscOutcome::Unknown(reason) => {
            return unknown(
                base_digest,
                continuation_digest,
                format!("demand verifier manifest failed: {reason:?}"),
            );
        }
    };
    if demand_manifest.reference_vector_agreement.status
        != GscReferenceVectorAgreement::ProvenRustReplayAndPinnedPrimitiveSafeAgdaProofV2
    {
        return unknown(
            base_digest,
            continuation_digest,
            "generic Rust/Agda agreement is not proven".to_owned(),
        );
    }
    let demand_verifier =
        match verify_gsc_verifier_manifest_v1(&base, &kernel, &agreement, &demand_manifest) {
            GscOutcome::Proven(verifier) => verifier,
            GscOutcome::Unknown(reason) => {
                return unknown(
                    base_digest,
                    continuation_digest,
                    format!("demand verifier replay failed: {reason:?}"),
                );
            }
        };
    let Some(bootstrap_digest) = canonical_lf_text_digest(REGISTERED_BOOTSTRAP_ASSET_BYTES) else {
        return unknown(
            base_digest,
            continuation_digest,
            "bootstrap text is not canonicalizable".to_owned(),
        );
    };
    if bootstrap_digest.as_str() != REGISTERED_BOOTSTRAP_ASSET_CANONICAL_LF_DIGEST {
        return unknown(
            base_digest,
            continuation_digest,
            "bootstrap differs from its pre-H3 pin".to_owned(),
        );
    }
    let bootstrap = match load_embedded_registered_bootstrap(&kernel) {
        Ok(bootstrap) => bootstrap,
        Err(error) => {
            return unknown(
                base_digest,
                continuation_digest,
                format!("bootstrap replay failed: {error}"),
            );
        }
    };
    let export_index =
        match load_embedded_registered_bootstrap_export_index_v1(&kernel, &bootstrap, &base) {
            Ok(index) => index,
            Err(error) => {
                return unknown(
                    base_digest,
                    continuation_digest,
                    format!("bootstrap export replay failed: {error}"),
                );
            }
        };
    let history = match verify_registered_bootstrap_history(&kernel, &bootstrap, export_index) {
        Ok(history) => history,
        Err(error) => {
            return unknown(
                base_digest,
                continuation_digest,
                format!("registered history replay failed: {error}"),
            );
        }
    };
    let inventory = match history.active_demand_inventory(&kernel, &base) {
        GscOutcome::Proven(inventory) => inventory,
        GscOutcome::Unknown(reason) => {
            return unknown(
                base_digest,
                continuation_digest,
                format!("registered inventory failed: {reason:?}"),
            );
        }
    };
    let slice = match run_registered_one_nullary_inductive_slice(
        &kernel,
        &base,
        &demand_verifier,
        &inventory,
        history.origin_cutoff_q3_registry(),
    ) {
        GscOutcome::Proven(slice) => slice,
        GscOutcome::Unknown(reason) => {
            return unknown(
                base_digest,
                continuation_digest,
                format!("accepted H3 semantic slice failed replay: {reason:?}"),
            );
        }
    };
    if !accepted_h3_components_match(&inventory, &slice) {
        return unknown(
            base_digest,
            continuation_digest,
            "current verifier does not reproduce the accepted H3 semantic components".to_owned(),
        );
    }
    let halt = match continue_one_nullary_inductive_completion_v1(
        &kernel,
        &base,
        &continuation,
        &history,
        &inventory,
        &slice,
    ) {
        GscOutcome::Proven(halt) => halt,
        GscOutcome::Unknown(reason) => {
            return unknown(
                base_digest,
                continuation_digest,
                format!("prospective continuation is undecided: {reason:?}"),
            );
        }
    };
    let report = match build_report(
        &kernel,
        &base,
        &continuation_raw,
        continuation.digest(),
        demand_verifier.digest(),
        &slice,
        &halt,
    ) {
        Some(report) => report,
        None => {
            return unknown(
                base_digest,
                continuation_digest,
                "continuation report construction failed closed".to_owned(),
            );
        }
    };
    LawV2H4ContinuationOutcomeV1::Halted {
        report: Box::new(report),
    }
}

fn accepted_h3_components_match(
    inventory: &pen_law::VerifiedRegisteredActiveDemandInventory,
    slice: &pen_gf2::VerifiedOneNullaryInductiveSlice,
) -> bool {
    let Some(tuple) = inventory.tuples().first() else {
        return false;
    };
    digest_matches(inventory.binding_digest(), ACCEPTED_H3_INVENTORY_DIGEST)
        && digest_matches(tuple.frame().binding_digest(), ACCEPTED_H3_FRAME_DIGEST)
        && digest_matches(
            slice.pre_response().digest(),
            ACCEPTED_H3_PRE_RESPONSE_DIGEST,
        )
        && digest_matches(slice.equations().digest(), ACCEPTED_H3_EQUATION_DIGEST)
        && digest_matches(slice.response().digest(), ACCEPTED_H3_RESPONSE_DIGEST)
        && digest_matches(
            slice.candidate_carrier().digest(),
            ACCEPTED_H3_CARRIER_DIGEST,
        )
        && digest_matches(slice.quotient().digest(), ACCEPTED_H3_QUOTIENT_DIGEST)
}

fn digest_matches(actual: &Digest, expected: &str) -> bool {
    actual.as_str() == expected
}

fn build_report(
    kernel: &Kernel,
    base: &pen_demand::gsc::VerifiedGscSemanticManifest,
    continuation_manifest: &LawV2ContinuationSemanticManifestV1,
    continuation_manifest_digest: &Digest,
    demand_verifier_manifest_digest: &Digest,
    slice: &pen_gf2::VerifiedOneNullaryInductiveSlice,
    halt: &VerifiedH4HaltCertificateV1,
) -> Option<LawV2H4ContinuationReportV1> {
    let accepted_h3 = AcceptedH3BindingReportV1 {
        result_digest: Digest::parse(ACCEPTED_H3_RESULT_DIGEST).ok()?,
        semantic_manifest_digest: Digest::parse(ACCEPTED_H3_SEMANTIC_DIGEST).ok()?,
        active_demand_inventory_binding_digest: Digest::parse(ACCEPTED_H3_INVENTORY_DIGEST).ok()?,
        registered_frame_binding_digest: Digest::parse(ACCEPTED_H3_FRAME_DIGEST).ok()?,
        pre_response_certificate_digest: Digest::parse(ACCEPTED_H3_PRE_RESPONSE_DIGEST).ok()?,
        equation_set_digest: Digest::parse(ACCEPTED_H3_EQUATION_DIGEST).ok()?,
        response_digest: Digest::parse(ACCEPTED_H3_RESPONSE_DIGEST).ok()?,
        candidate_carrier_digest: Digest::parse(ACCEPTED_H3_CARRIER_DIGEST).ok()?,
        quotient_digest: Digest::parse(ACCEPTED_H3_QUOTIENT_DIGEST).ok()?,
        semantic_components_replayed: true,
    };
    let gf2_slice_verifier_manifest = slice.verifier_manifest().clone();
    let gf2_slice_verifier_manifest_digest = gf2_slice_verifier_manifest.canonical_digest();
    let continuation_verifier_source_digest = canonical_source_digest(
        "pen-engine/law-v2-h4-continuation-verifier-source/v1",
        &[
            include_bytes!("../../pen-gf2/src/continuation.rs"),
            include_bytes!("h4_inductive_continuation.rs"),
            include_bytes!("../Cargo.toml"),
        ],
    )?;
    let workspace_manifest_digest =
        canonical_lf_text_digest(include_bytes!("../../../Cargo.toml"))?;
    let workspace_lock_digest = canonical_lf_text_digest(include_bytes!("../../../Cargo.lock"))?;
    let production_isolation_lock_digest = canonical_lf_text_digest(include_bytes!(
        "../../../scripts/law-v2-isolation.Cargo.lock"
    ))?;
    let composite_provenance_digest = {
        let mut encoder = CanonicalEncoder::new();
        base.digest().encode_canonical(&mut encoder);
        continuation_manifest_digest.encode_canonical(&mut encoder);
        demand_verifier_manifest_digest.encode_canonical(&mut encoder);
        gf2_slice_verifier_manifest_digest.encode_canonical(&mut encoder);
        continuation_verifier_source_digest.encode_canonical(&mut encoder);
        workspace_manifest_digest.encode_canonical(&mut encoder);
        workspace_lock_digest.encode_canonical(&mut encoder);
        production_isolation_lock_digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes(
            "pen-engine/law-v2-h4-continuation-composite-provenance/v1",
            encoder.as_bytes(),
        )
    };
    let verifier = H4VerifierReportV1 {
        demand_verifier_manifest_digest: demand_verifier_manifest_digest.clone(),
        gf2_slice_verifier_manifest,
        gf2_slice_verifier_manifest_digest,
        continuation_verifier_source_digest,
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        workspace_manifest_digest,
        workspace_lock_digest,
        production_isolation_lock_digest,
        composite_provenance_digest,
    };
    let seal = halt.free_seal();
    let free_seal = H4FreeSealReportV1 {
        ordinal: seal.ordinal(),
        predecessor: seal.predecessor().clone(),
        event_id: seal.event_id().clone(),
        pre_history_digest: seal.pre_history_digest().clone(),
        post_history_digest: seal.post_history_digest().clone(),
        pre_boundary_digest: seal.pre_boundary_digest().clone(),
        post_boundary_digest: seal.post_boundary().digest().clone(),
        extension: seal.extension().clone(),
        extension_digest: seal.extension_digest().clone(),
        source_identity: seal.source_identity().clone(),
        binding_identity: seal.binding_identity().clone(),
        event_export_digest: seal.event_export_digest().clone(),
        activated_closed_former_group_count: seal.activated_closed_former_group_count(),
        equation_set_digest: seal.equation_set_digest().clone(),
        response_digest: seal.response_digest().clone(),
        quotient_digest: seal.quotient_digest().clone(),
        active_anchor_digest: seal.active_anchor_digest().clone(),
        empty_q3_registry_digest: seal.empty_q3_registry_digest().clone(),
        initiality_certificate_digest: seal.initiality_certificate_digest().clone(),
        free_seal_digest: seal.digest().clone(),
        exact_old_boundary_prefix_preserved: true,
        exact_one_bodyless_generator: true,
        exact_one_generated_equation: true,
        no_additional_generator_or_equation: true,
    };
    let census = halt.census();
    let prospective_census = H4CensusReportV1 {
        active_older: census.active_older().clone(),
        active_newer: census.active_newer().clone(),
        active_anchor_digest: census.active_anchor_digest().clone(),
        registered_inventory_binding_digest: census.registered_inventory_binding_digest().clone(),
        continuation_inventory_binding_digest: census
            .continuation_inventory_binding_digest()
            .clone(),
        active_frame_binding_digests: census.active_frame_binding_digests().to_vec(),
        decisions: census
            .decisions()
            .iter()
            .map(|decision| H4DemandDecisionReportV1 {
                family_id: decision.family_id().clone(),
                rule: decision.rule(),
                port: decision.port().clone(),
                disposition: "derived".to_owned(),
                evidence_digest: decision.evidence_digest().clone(),
            })
            .collect(),
        extracted_family_count: census.extracted_family_count(),
        extracted_port_count: census.extracted_port_count(),
        derived_port_count: census.derived_port_count(),
        live_orbit_count: census.live_orbit_count(),
        extraction_complete: census.extraction_is_complete(),
        derivability_complete: census.derivability_is_complete(),
        expiration_complete: census.expiration_is_complete(),
        cumulative_weakening_replay_digest: census.cumulative_weakening_replay_digest().clone(),
        extraction_digest: census.extraction_digest().clone(),
        derivability_digest: census.derivability_digest().clone(),
        expiration_digest: census.expiration_digest().clone(),
        census_digest: census.digest().clone(),
    };
    let halt_report = H4HaltReportV1 {
        halted_after_sealed_act_count: halt.halted_after_sealed_act_count(),
        prospective_stage_ordinal: halt.prospective_stage_ordinal(),
        positive_cost_exclusion_digest: halt.positive_cost_exclusion_digest().clone(),
        halt_certificate_digest: halt.digest().clone(),
        no_new_structure_generated: true,
        disposition: "halted_complete_empty_live_obligation_profile".to_owned(),
        scope: "relative_to_frozen_owner_specific_g_use_g_compute_continuation_profile".to_owned(),
    };
    let body = LawV2H4ContinuationReportBodyV1 {
        schema_version: LAW_V2_H4_CONTINUATION_REPORT_SCHEMA_VERSION,
        profile_id: LAW_V2_H4_CONTINUATION_PROFILE_ID.to_owned(),
        authority:
            "accepted_h3_semantic_components_plus_versioned_free_seal_and_complete_h4_census"
                .to_owned(),
        base_semantic_manifest: base.manifest().clone(),
        base_semantic_manifest_digest: base.digest().clone(),
        continuation_semantic_manifest: continuation_manifest.clone(),
        continuation_semantic_manifest_digest: continuation_manifest_digest.clone(),
        accepted_h3,
        verifier,
        free_seal,
        prospective_census,
        halt: halt_report,
    };
    let result_digest = report_body_digest(&body)?;
    Some(LawV2H4ContinuationReportV1 {
        body,
        result_digest,
    })
}

fn report_body_digest(body: &LawV2H4ContinuationReportBodyV1) -> Option<Digest> {
    let value = serde_json::to_value(body).ok()?;
    let mut encoder = CanonicalEncoder::new();
    encode_canonical_json(&value, &mut encoder);
    Some(Digest::of_domain_bytes(
        "pen-engine/law-v2-h4-continuation-report/canonical-json/v1",
        encoder.as_bytes(),
    ))
}

pub fn replay_law_v2_h4_continuation_report_v1(report: &LawV2H4ContinuationReportV1) -> bool {
    if Some(&report.result_digest) != report_body_digest(&report.body).as_ref() {
        return false;
    }
    match run_law_v2_h4_continuation_v1() {
        LawV2H4ContinuationOutcomeV1::Halted { report: expected } => report == expected.as_ref(),
        LawV2H4ContinuationOutcomeV1::Unknown { .. } => false,
    }
}

pub fn law_v2_h4_continuation_json_pretty_v1() -> String {
    serde_json::to_string_pretty(&run_law_v2_h4_continuation_v1()).unwrap_or_else(|_| {
        format!(
            "{{\n  \"status\": \"unknown\",\n  \"profile_id\": \
             \"{LAW_V2_H4_CONTINUATION_PROFILE_ID}\",\n  \"reason\": \"outcome serialization \
             failed closed\",\n  \"base_semantic_manifest_digest\": null,\n  \
             \"continuation_semantic_manifest_digest\": null,\n  \"fail_closed\": true\n}}"
        )
    })
}

fn unknown(
    base_semantic_manifest_digest: Option<Digest>,
    continuation_semantic_manifest_digest: Option<Digest>,
    reason: String,
) -> LawV2H4ContinuationOutcomeV1 {
    LawV2H4ContinuationOutcomeV1::Unknown {
        profile_id: LAW_V2_H4_CONTINUATION_PROFILE_ID.to_owned(),
        reason,
        base_semantic_manifest_digest,
        continuation_semantic_manifest_digest,
        fail_closed: true,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        FROZEN_CONTINUATION_SEMANTIC_DIGEST, LawV2H4ContinuationOutcomeV1,
        law_v2_h4_continuation_manifest_json_pretty_v1, replay_law_v2_h4_continuation_report_v1,
        run_law_v2_h4_continuation_v1,
    };

    #[test]
    fn continuation_manifest_can_be_frozen_without_executing_the_live_prefix() {
        let value: serde_json::Value =
            serde_json::from_str(&law_v2_h4_continuation_manifest_json_pretty_v1())
                .expect("manifest JSON");
        assert_eq!(value["status"], "frozen");
        assert!(
            value["semantic_manifest_digest"]
                .as_str()
                .is_some_and(|digest| digest.starts_with("blake3:"))
        );
        assert_eq!(
            value["semantic_manifest_digest"],
            FROZEN_CONTINUATION_SEMANTIC_DIGEST
        );
    }

    #[test]
    #[ignore = "requires the exact pinned Agda 2.8.0 executable on PATH"]
    fn live_continuation_halts_deterministically_and_replays() {
        let first = run_law_v2_h4_continuation_v1();
        let second = run_law_v2_h4_continuation_v1();
        assert_eq!(first, second);
        let LawV2H4ContinuationOutcomeV1::Halted { report } = first else {
            panic!("pinned continuation must decide its prospective census");
        };
        assert_eq!(report.body.halt.halted_after_sealed_act_count, 4);
        assert_eq!(report.body.halt.prospective_stage_ordinal, 5);
        assert_eq!(report.body.prospective_census.extracted_port_count, 2);
        assert_eq!(report.body.prospective_census.derived_port_count, 2);
        assert_eq!(report.body.prospective_census.live_orbit_count, 0);
        assert!(report.body.prospective_census.extraction_complete);
        assert!(report.body.prospective_census.derivability_complete);
        assert!(report.body.prospective_census.expiration_complete);
        assert!(report.body.halt.no_new_structure_generated);
        assert!(replay_law_v2_h4_continuation_report_v1(&report));
    }
}
