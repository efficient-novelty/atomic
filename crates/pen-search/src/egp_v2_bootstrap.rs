//! Score-blind EGP-v2 bootstrap reselection through Stage 4.
//!
//! The semantic pass and the legacy comparison are deliberately separate:
//! [`run_raw_bootstrap`] freezes a digest of the complete semantic result
//! before [`compare_with_sealed_legacy`] is allowed to construct or inspect
//! any reference telescope or legacy score.  Semantic variants are expanded
//! over the admitted cone by declaration state, never by stage label or
//! reference hash.
//!
//! The burned run and its adversarial proof-gate audit are reported in
//! `docs/EGP_V2_BOOTSTRAP_RESULT.md`.  Replayability of this experiment must
//! not be confused with discharge of the open semantic bridge obligations.

use crate::accept::acceptance_rank_for_telescope;
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use pen_core::canonical::canonical_key_telescope;
use pen_core::declaration::{
    ConstructorCompletionOverlay, ConstructorDeclaration, DeclarationType, FreshExportSite,
    InductiveFormationOverlay, InductiveHeadDeclaration,
};
use pen_core::encode::telescope_bit_cost;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_eval::egp_v2::{
    BasisCertificate, CoverageRule, DeclarationEvidence, ProvenanceTag, RegisteredCompletionRule,
    SemanticTargetKind, certify_independent_basis,
};
use pen_eval::typed_families::{
    CandidateExtractionOutcome, PredecessorClosure, extract_candidate_families, predecessor_closure,
};
use pen_type::admissibility::{
    AdmissibilityMode, passes_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::elaborate::{ELABORATOR_VERSION_TAG, SealedSignature, candidate_hash};
use pen_type::equality::KERNEL_EQUALITY_PROCEDURE;
use pen_type::fresh_declaration::{
    FRESH_DECLARATION_RULES, InductiveFormationEvidence, issue_constructor_completion,
    issue_inductive_formation,
};
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

pub const EGP_V2_BOOTSTRAP_SCHEMA_VERSION: u32 = 2;
pub const EGP_V2_LAW_ID: &str = pen_eval::egp_v2::EGP_V2_LAW_ID;
pub const EGP_V2_BOOTSTRAP_RUNNER_RULES: &str = "egp-v2-bootstrap-runner-v1";
pub const EGP_V2_BOOTSTRAP_DATE: &str = "2026-07-18";
const WINDOW_DEPTH: u16 = 2;
const LAST_BOOTSTRAP_STAGE: u32 = 4;

fn digest(domain: &str, value: &impl Serialize) -> String {
    let bytes =
        serde_json::to_vec(&(domain, value)).expect("EGP-v2 certificate material is serializable");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn rational_string(value: Rational) -> String {
    value.to_string()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FrozenRuleHashes {
    pub source_revision: String,
    pub ast_schema: String,
    pub declaration_rules: String,
    pub type_rules: String,
    pub normalization_equality: String,
    pub free_completion_rules: String,
    pub family_extractor: String,
    pub basis_algorithm_and_ordering: String,
    pub demand_rules: String,
    pub egp_anchor_rules: String,
    pub candidate_enumerator: String,
    pub initial_state: String,
    /// A preregistered opaque commitment.  Its preimage is reconstructed
    /// only in the post-run comparison pass.
    pub sealed_legacy_comparison_trace: String,
}

impl FrozenRuleHashes {
    fn for_initial_signature(signature: &SealedSignature) -> Self {
        let tagged = |name: &str, version: &str| digest(name, &version);
        Self {
            source_revision: option_env!("PEN_SOURCE_REVISION")
                .unwrap_or(concat!("pen-search-", env!("CARGO_PKG_VERSION")))
                .to_owned(),
            ast_schema: tagged(
                "ast-schema",
                pen_core::declaration::FRESH_DECLARATION_SCHEMA,
            ),
            declaration_rules: tagged("declaration-rules", FRESH_DECLARATION_RULES),
            type_rules: tagged("type-rules", ELABORATOR_VERSION_TAG),
            normalization_equality: tagged("normalization-equality", KERNEL_EQUALITY_PROCEDURE),
            free_completion_rules: tagged("free-completion", EGP_V2_LAW_ID),
            family_extractor: tagged("family-extractor", "typed-families-v1"),
            basis_algorithm_and_ordering: tagged("basis", EGP_V2_LAW_ID),
            demand_rules: tagged("demand", EGP_V2_BOOTSTRAP_RUNNER_RULES),
            egp_anchor_rules: tagged("anchors", EGP_V2_LAW_ID),
            candidate_enumerator: tagged("enumerator", "guarded-enumerator-v1"),
            initial_state: digest(
                "initial-semantic-state",
                &(signature.digest(), EGP_V2_LAW_ID),
            ),
            sealed_legacy_comparison_trace: sealed_legacy_commitment().to_owned(),
        }
    }
}

/// Full semantic input retained in the artifact.  Evidence tokens themselves
/// need not deserialize: these overlays plus the prior accepted prefix are
/// enough to reissue them.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BootstrapVariant {
    Plain {
        presentation: Telescope,
    },
    OneNullaryFormation {
        overlay: InductiveFormationOverlay,
    },
    ExactConstructorCompletion {
        formation_overlay: InductiveFormationOverlay,
        overlay: ConstructorCompletionOverlay,
    },
}

impl BootstrapVariant {
    pub fn presentation(&self) -> &Telescope {
        match self {
            Self::Plain { presentation } => presentation,
            Self::OneNullaryFormation { overlay } => overlay.presentation(),
            Self::ExactConstructorCompletion { overlay, .. } => overlay.presentation(),
        }
    }

    pub fn variant_kind(&self) -> &'static str {
        match self {
            Self::Plain { .. } => "plain",
            Self::OneNullaryFormation { .. } => "one_nullary_formation",
            Self::ExactConstructorCompletion { .. } => "exact_constructor_completion",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LiveConstructorDemand {
    pub demand_id: String,
    pub owner_head: String,
    pub slot_ordinal: u16,
    pub formation_subject_digest: String,
    pub formation_evidence_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AnsweredConstructorDemand {
    pub demand_id: String,
    pub completion_subject_digest: String,
    pub completion_evidence_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedInductiveSignature {
    pub owner_head: String,
    pub formation_evidence_digest: String,
    pub completion_evidence_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticLedgerSnapshot {
    pub prior_digest: Option<String>,
    pub live_constructor: Option<LiveConstructorDemand>,
    pub answered_constructors: Vec<AnsweredConstructorDemand>,
    pub sealed_signatures: Vec<SealedInductiveSignature>,
    pub digest: String,
}

impl SemanticLedgerSnapshot {
    fn initial(signature: &SealedSignature) -> Self {
        let mut snapshot = Self {
            prior_digest: None,
            live_constructor: None,
            answered_constructors: Vec::new(),
            sealed_signatures: Vec::new(),
            digest: String::new(),
        };
        snapshot.digest = digest(
            "egp-v2-ledger-genesis",
            &(EGP_V2_LAW_ID, signature.digest()),
        );
        snapshot
    }
}

#[derive(Clone, Debug)]
struct LiveFormationRuntime {
    overlay: InductiveFormationOverlay,
    evidence: InductiveFormationEvidence,
    demand: LiveConstructorDemand,
}

#[derive(Clone, Debug)]
struct RuntimeLedger {
    snapshot: SemanticLedgerSnapshot,
    live: Option<LiveFormationRuntime>,
}

impl RuntimeLedger {
    fn initial(signature: &SealedSignature) -> Self {
        Self {
            snapshot: SemanticLedgerSnapshot::initial(signature),
            live: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CandidateRejection {
    KernelInvalid { detail: String },
    FormationEvidenceInvalid { detail: String },
    CompletionEvidenceInvalid { detail: String },
    BasisInvalid { detail: String },
    LedgerInvalid { detail: String },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapCandidate {
    pub candidate_id: String,
    pub presentation_hash: String,
    pub canonical_key: String,
    pub variant: BootstrapVariant,
    pub prior_semantic_ledger_digest: String,
    pub basis_certificate: Option<BasisCertificate>,
    pub basis_certificate_digest: Option<String>,
    pub kappa: u16,
    pub revised_nu: Option<u32>,
    pub rho: Option<String>,
    pub clears_bar: bool,
    pub overshoot: Option<String>,
    pub rejection: Option<CandidateRejection>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapWinner {
    pub candidate_id: String,
    pub presentation_hash: String,
    pub canonical_key: String,
    pub variant: BootstrapVariant,
    pub basis_certificate_digest: String,
    pub kappa: u16,
    pub revised_nu: u32,
    pub rho: String,
    pub overshoot: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapStage {
    pub stage: u32,
    pub prior_stage_certificate_digest: Option<String>,
    pub semantic_ledger_before: SemanticLedgerSnapshot,
    pub bar_before: String,
    pub cone_enumerated: usize,
    pub cone_admitted: usize,
    pub cone_deduped: usize,
    pub semantic_variants: usize,
    pub cone_digest: String,
    pub candidates: Vec<BootstrapCandidate>,
    pub order: Vec<String>,
    pub winner: Option<BootstrapWinner>,
    pub semantic_ledger_after: SemanticLedgerSnapshot,
    pub bar_after: Option<String>,
    pub stage_certificate_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BootstrapOutcome {
    CompletedThroughStage4,
    HaltedNoClearingCandidate { stage: u32, bar: String },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RawBootstrapRun {
    pub law_id: String,
    pub schema_version: u32,
    pub date: String,
    pub burned_run: bool,
    pub frozen_hashes: FrozenRuleHashes,
    pub outcome: BootstrapOutcome,
    pub stages: Vec<BootstrapStage>,
    pub revised_history: Vec<(u32, u32, u32)>,
    pub final_semantic_ledger: SemanticLedgerSnapshot,
    pub raw_run_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StagePosthocComparison {
    pub stage: u32,
    pub legacy_winner_id: String,
    pub winner_semantically_equivalent: bool,
    pub legacy_nu: u32,
    pub score_matches_legacy: bool,
    pub legacy_bar_before: String,
    pub bar_before_matches_legacy: bool,
    pub legacy_bar_after: String,
    pub bar_after_matches_legacy: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FirstBootstrapDivergence {
    pub stage: u32,
    pub field: String,
    pub legacy: String,
    pub revised: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapInvariants {
    pub stage1_one_atom_covers_universe_package: Option<bool>,
    pub stage2_fresh_unit_atom: Option<bool>,
    pub stage2_one_live_constructor_orbit: Option<bool>,
    pub stage3_star_answers_that_orbit: Option<bool>,
    pub stage3_signature_sealed: Option<bool>,
    pub stage3_generated_unit_ind_and_beta_covered: Option<bool>,
    pub alpha_renaming_invariant: Option<bool>,
    pub univalent_representations_do_not_mint_atoms: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EgpV2BootstrapCertificate {
    pub raw: RawBootstrapRun,
    pub posthoc_comparison: Vec<StagePosthocComparison>,
    pub first_divergence: Option<FirstBootstrapDivergence>,
    pub bootstrap_invariants: BootstrapInvariants,
    pub run_digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum BootstrapError {
    #[error("stage {stage}: predecessor closure failed: {detail}")]
    Closure { stage: u32, detail: String },
    #[error("stage {stage}: semantic ledger transition failed: {detail}")]
    Ledger { stage: u32, detail: String },
    #[error("raw EGP-v2 digest is missing")]
    RawDigestMissing,
    #[error("sealed legacy comparison commitment drifted")]
    LegacyCommitmentDrift,
}

fn sealed_legacy_commitment() -> &'static str {
    // Opaque during the semantic pass.  The post-hoc grader reconstructs its
    // preimage and rejects drift before exposing any comparison as valid.
    "blake3:0d747c8ba5e898a74f4bbe1777da7fb44b30c21445e0d71556e2dc95e134b3b8"
}

fn enumerate_base_cone(stage: u32, structural_library: &Library) -> (usize, usize, Vec<Telescope>) {
    let admissibility = strict_admissibility_for_mode(
        stage,
        WINDOW_DEPTH,
        structural_library,
        AdmissibilityMode::Guarded,
    );
    let context = EnumerationContext::from_admissibility(structural_library, admissibility);
    let mut enumerated = Vec::new();
    for kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        enumerated.extend(enumerate_telescopes(structural_library, context, kappa));
    }
    let cone_enumerated = enumerated.len();
    let admitted = enumerated
        .into_iter()
        .filter(|telescope| {
            passes_strict_admissibility(stage, structural_library, telescope, admissibility)
        })
        .collect::<Vec<_>>();
    let cone_admitted = admitted.len();

    let mut seen = BTreeSet::new();
    let mut cone = Vec::new();
    for telescope in admitted {
        if seen.insert(canonical_key_telescope(&telescope).0) {
            cone.push(telescope);
        }
    }
    (cone_enumerated, cone_admitted, cone)
}

/// Expand semantics over the complete structural cone.  The only predicate
/// is declaration state plus the generic one-export surface (`kappa == 1`).
/// There is intentionally no stage index in this function.
fn expand_semantic_variants(
    signature: &SealedSignature,
    ledger: &RuntimeLedger,
    cone: &[Telescope],
) -> Vec<BootstrapVariant> {
    let mut variants = Vec::new();
    for telescope in cone {
        variants.push(BootstrapVariant::Plain {
            presentation: telescope.clone(),
        });
        if telescope.kappa() != 1 {
            continue;
        }

        match &ledger.live {
            None => {
                let declaration = InductiveHeadDeclaration::one_nullary(
                    FreshExportSite::new(signature.digest(), 0),
                    DeclarationType::existing(Expr::Univ),
                );
                variants.push(BootstrapVariant::OneNullaryFormation {
                    overlay: InductiveFormationOverlay::new(telescope.clone(), declaration),
                });
            }
            Some(live) => {
                let Some(slot) = live.evidence.pending_slots().first().cloned() else {
                    // A malformed runtime ledger cannot manufacture a broad
                    // completion variant.  The transition grader rejects it.
                    continue;
                };
                let declaration = ConstructorDeclaration::fill_nullary(
                    FreshExportSite::new(live.evidence.resulting_context_digest(), 0),
                    slot,
                );
                variants.push(BootstrapVariant::ExactConstructorCompletion {
                    formation_overlay: live.overlay.clone(),
                    overlay: ConstructorCompletionOverlay::new(telescope.clone(), declaration),
                });
            }
        }
    }
    variants
}

fn variant_id(variant: &BootstrapVariant, ledger_digest: &str) -> String {
    digest(
        "egp-v2-semantic-candidate",
        &(EGP_V2_LAW_ID, ledger_digest, variant),
    )
}

fn score_basis_certificate(
    extraction: &pen_eval::typed_families::CandidateFamilyExtraction,
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    ledger: &RuntimeLedger,
    variant: &BootstrapVariant,
) -> Result<BasisCertificate, CandidateRejection> {
    match variant {
        BootstrapVariant::Plain { presentation } => certify_independent_basis(
            signature,
            closure,
            presentation,
            extraction,
            DeclarationEvidence::None,
        )
        .map_err(|error| CandidateRejection::BasisInvalid {
            detail: error.to_string(),
        }),
        BootstrapVariant::OneNullaryFormation { overlay } => {
            if ledger.live.is_some() {
                return Err(CandidateRejection::LedgerInvalid {
                    detail: "a formation cannot open while a constructor demand is live".to_owned(),
                });
            }
            let evidence = issue_inductive_formation(signature, overlay).map_err(|error| {
                CandidateRejection::FormationEvidenceInvalid {
                    detail: error.to_string(),
                }
            })?;
            certify_independent_basis(
                signature,
                closure,
                overlay.presentation(),
                extraction,
                DeclarationEvidence::Formation {
                    overlay,
                    evidence: &evidence,
                },
            )
            .map_err(|error| CandidateRejection::BasisInvalid {
                detail: error.to_string(),
            })
        }
        BootstrapVariant::ExactConstructorCompletion {
            formation_overlay,
            overlay,
        } => {
            let Some(live) = &ledger.live else {
                return Err(CandidateRejection::LedgerInvalid {
                    detail: "a constructor completion has no live demand".to_owned(),
                });
            };
            if formation_overlay != &live.overlay {
                return Err(CandidateRejection::LedgerInvalid {
                    detail: "completion does not retain the live formation overlay".to_owned(),
                });
            }
            if ledger.snapshot.live_constructor.as_ref() != Some(&live.demand) {
                return Err(CandidateRejection::LedgerInvalid {
                    detail: "runtime demand and serialized ledger disagree".to_owned(),
                });
            }
            let evidence = issue_constructor_completion(signature, &live.evidence, overlay)
                .map_err(|error| CandidateRejection::CompletionEvidenceInvalid {
                    detail: error.to_string(),
                })?;
            certify_independent_basis(
                signature,
                closure,
                overlay.presentation(),
                extraction,
                DeclarationEvidence::Completion {
                    formation_overlay,
                    formation: &live.evidence,
                    overlay,
                    evidence: &evidence,
                },
            )
            .map_err(|error| CandidateRejection::BasisInvalid {
                detail: error.to_string(),
            })
        }
    }
}

fn score_variant(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    ledger: &RuntimeLedger,
    variant: BootstrapVariant,
    bar: Rational,
) -> BootstrapCandidate {
    let presentation = variant.presentation();
    let presentation_hash = candidate_hash(presentation);
    let canonical_key = canonical_key_telescope(presentation).0;
    let candidate_id = variant_id(&variant, &ledger.snapshot.digest);
    let kappa = u16::try_from(presentation.kappa()).expect("bootstrap kappa fits u16");

    let (basis_certificate, rejection) = match extract_candidate_families(
        signature,
        closure,
        presentation,
        u32::try_from(signature.len()).expect("bootstrap signature length fits u32"),
    ) {
        CandidateExtractionOutcome::KernelInvalid { failure } => (
            None,
            Some(CandidateRejection::KernelInvalid {
                detail: failure.to_string(),
            }),
        ),
        CandidateExtractionOutcome::Extracted(extraction) => {
            match score_basis_certificate(&extraction, signature, closure, ledger, &variant) {
                Ok(certificate) => (Some(certificate), None),
                Err(rejection) => (None, Some(rejection)),
            }
        }
    };

    let revised_nu = basis_certificate
        .as_ref()
        .map(|certificate| certificate.revised_nu);
    let rho = revised_nu.map(|nu| Rational::new(i64::from(nu), i64::from(kappa.max(1))));
    let clears_bar = rho.is_some_and(|rho| rho >= bar);
    let basis_certificate_digest = basis_certificate
        .as_ref()
        .map(|certificate| digest("egp-v2-basis-certificate", certificate));

    BootstrapCandidate {
        candidate_id,
        presentation_hash,
        canonical_key,
        variant,
        prior_semantic_ledger_digest: ledger.snapshot.digest.clone(),
        basis_certificate,
        basis_certificate_digest,
        kappa,
        revised_nu,
        rho: rho.map(rational_string),
        clears_bar,
        overshoot: clears_bar.then(|| rational_string(rho.expect("clearer has rho") - bar)),
        rejection,
    }
}

fn live_demand(
    overlay: &InductiveFormationOverlay,
    evidence: &InductiveFormationEvidence,
) -> Result<LiveConstructorDemand, String> {
    let [slot] = evidence.pending_slots() else {
        return Err(format!(
            "one-nullary formation issued {} pending slots",
            evidence.pending_slots().len()
        ));
    };
    Ok(LiveConstructorDemand {
        demand_id: digest(
            "egp-v2-constructor-demand",
            &(
                evidence.derivation_hash(),
                slot.owner().as_str(),
                slot.ordinal(),
            ),
        ),
        owner_head: slot.owner().as_str().to_owned(),
        slot_ordinal: slot.ordinal(),
        formation_subject_digest: overlay.semantic_subject_digest(),
        formation_evidence_digest: evidence.derivation_hash().to_owned(),
    })
}

fn chain_unchanged_ledger(
    before: &SemanticLedgerSnapshot,
    winner: &BootstrapWinner,
) -> SemanticLedgerSnapshot {
    let mut after = SemanticLedgerSnapshot {
        prior_digest: Some(before.digest.clone()),
        live_constructor: before.live_constructor.clone(),
        answered_constructors: before.answered_constructors.clone(),
        sealed_signatures: before.sealed_signatures.clone(),
        digest: String::new(),
    };
    after.digest = digest(
        "egp-v2-ledger-plain-transition",
        &(
            before.digest.as_str(),
            winner.candidate_id.as_str(),
            winner.basis_certificate_digest.as_str(),
            &after.live_constructor,
            &after.answered_constructors,
            &after.sealed_signatures,
        ),
    );
    after
}

fn apply_winner_to_ledger(
    stage: u32,
    signature: &SealedSignature,
    ledger: &mut RuntimeLedger,
    winner: &BootstrapWinner,
) -> Result<(), BootstrapError> {
    match &winner.variant {
        BootstrapVariant::Plain { .. } => {
            ledger.snapshot = chain_unchanged_ledger(&ledger.snapshot, winner);
            Ok(())
        }
        BootstrapVariant::OneNullaryFormation { overlay } => {
            if ledger.live.is_some() || ledger.snapshot.live_constructor.is_some() {
                return Err(BootstrapError::Ledger {
                    stage,
                    detail: "formation winner would overwrite a live constructor demand".to_owned(),
                });
            }
            let evidence = issue_inductive_formation(signature, overlay).map_err(|error| {
                BootstrapError::Ledger {
                    stage,
                    detail: format!("accepted formation evidence did not reissue: {error}"),
                }
            })?;
            let demand = live_demand(overlay, &evidence)
                .map_err(|detail| BootstrapError::Ledger { stage, detail })?;
            if ledger
                .snapshot
                .answered_constructors
                .iter()
                .any(|answered| answered.demand_id == demand.demand_id)
            {
                return Err(BootstrapError::Ledger {
                    stage,
                    detail: "formation attempted to reopen an answered demand".to_owned(),
                });
            }

            let before = ledger.snapshot.clone();
            let mut after = SemanticLedgerSnapshot {
                prior_digest: Some(before.digest.clone()),
                live_constructor: Some(demand.clone()),
                answered_constructors: before.answered_constructors,
                sealed_signatures: before.sealed_signatures,
                digest: String::new(),
            };
            after.digest = digest(
                "egp-v2-ledger-open-one-nullary",
                &(
                    before.digest.as_str(),
                    winner.candidate_id.as_str(),
                    winner.basis_certificate_digest.as_str(),
                    &after.live_constructor,
                    &after.answered_constructors,
                    &after.sealed_signatures,
                ),
            );
            ledger.snapshot = after;
            ledger.live = Some(LiveFormationRuntime {
                overlay: overlay.clone(),
                evidence,
                demand,
            });
            Ok(())
        }
        BootstrapVariant::ExactConstructorCompletion {
            formation_overlay,
            overlay,
        } => {
            let Some(live) = ledger.live.take() else {
                return Err(BootstrapError::Ledger {
                    stage,
                    detail: "completion winner has no live constructor demand".to_owned(),
                });
            };
            if formation_overlay != &live.overlay
                || ledger.snapshot.live_constructor.as_ref() != Some(&live.demand)
            {
                return Err(BootstrapError::Ledger {
                    stage,
                    detail: "completion winner does not match the exact live formation".to_owned(),
                });
            }
            if ledger
                .snapshot
                .answered_constructors
                .iter()
                .any(|answered| answered.demand_id == live.demand.demand_id)
            {
                return Err(BootstrapError::Ledger {
                    stage,
                    detail: "constructor demand was already consumed".to_owned(),
                });
            }
            let evidence = issue_constructor_completion(signature, &live.evidence, overlay)
                .map_err(|error| BootstrapError::Ledger {
                    stage,
                    detail: format!("accepted completion evidence did not reissue: {error}"),
                })?;
            let answered = evidence.answered_slot();
            if answered.owner().as_str() != live.demand.owner_head
                || answered.ordinal() != live.demand.slot_ordinal
                || !evidence.is_inductive_signature_closed()
            {
                return Err(BootstrapError::Ledger {
                    stage,
                    detail: "completion failed exact-slot or sealed-after ledger check".to_owned(),
                });
            }

            let before = ledger.snapshot.clone();
            let completion_subject_digest = overlay.semantic_subject_digest();
            let completion_evidence_digest = evidence.derivation_hash().to_owned();
            let mut answered_constructors = before.answered_constructors;
            answered_constructors.push(AnsweredConstructorDemand {
                demand_id: live.demand.demand_id.clone(),
                completion_subject_digest,
                completion_evidence_digest: completion_evidence_digest.clone(),
            });
            let mut sealed_signatures = before.sealed_signatures;
            sealed_signatures.push(SealedInductiveSignature {
                owner_head: live.demand.owner_head,
                formation_evidence_digest: live.demand.formation_evidence_digest,
                completion_evidence_digest,
            });
            let mut after = SemanticLedgerSnapshot {
                prior_digest: Some(before.digest.clone()),
                live_constructor: None,
                answered_constructors,
                sealed_signatures,
                digest: String::new(),
            };
            after.digest = digest(
                "egp-v2-ledger-close-one-nullary",
                &(
                    before.digest.as_str(),
                    winner.candidate_id.as_str(),
                    winner.basis_certificate_digest.as_str(),
                    &after.live_constructor,
                    &after.answered_constructors,
                    &after.sealed_signatures,
                ),
            );
            ledger.snapshot = after;
            Ok(())
        }
    }
}

fn stage_digest(stage: &BootstrapStage) -> String {
    let mut payload = stage.clone();
    payload.stage_certificate_digest.clear();
    digest("egp-v2-bootstrap-stage", &payload)
}

/// Execute and freeze the EGP-v2 semantic run.  This function contains no
/// construction of `Telescope::reference`, no legacy novelty call, and no
/// legacy bar calculation.  Comparison is a separate API below.
pub fn run_raw_bootstrap() -> Result<RawBootstrapRun, BootstrapError> {
    let mut structural_library: Library = Vec::new();
    let mut accepted_telescopes: Vec<(u32, Telescope)> = Vec::new();
    let mut revised_records: Vec<DiscoveryRecord> = Vec::new();
    let initial_signature = SealedSignature::from_telescopes(Vec::new());
    let frozen_hashes = FrozenRuleHashes::for_initial_signature(&initial_signature);
    let mut ledger = RuntimeLedger::initial(&initial_signature);
    let mut stages = Vec::new();
    let mut outcome = BootstrapOutcome::CompletedThroughStage4;

    for stage in 1..=LAST_BOOTSTRAP_STAGE {
        let signature = SealedSignature::from_telescopes(accepted_telescopes.clone());
        let closure = predecessor_closure(&signature).map_err(|error| BootstrapError::Closure {
            stage,
            detail: error.to_string(),
        })?;
        let bar = compute_bar(usize::from(WINDOW_DEPTH), stage, &revised_records).bar;
        let ledger_before = ledger.snapshot.clone();
        let prior_stage_certificate_digest = stages
            .last()
            .map(|prior: &BootstrapStage| prior.stage_certificate_digest.clone());

        let (cone_enumerated, cone_admitted, cone) =
            enumerate_base_cone(stage, &structural_library);
        let cone_deduped = cone.len();
        let variants = expand_semantic_variants(&signature, &ledger, &cone);
        let cone_digest = digest(
            "egp-v2-bootstrap-cone",
            &(signature.digest(), ledger_before.digest.as_str(), &variants),
        );

        let mut candidates = variants
            .into_iter()
            .map(|variant| score_variant(&signature, &closure, &ledger, variant, bar))
            .collect::<Vec<_>>();

        let mut ranked = Vec::new();
        for (index, candidate) in candidates.iter().enumerate() {
            let Some(nu) = candidate.revised_nu else {
                continue;
            };
            if !candidate.clears_bar {
                continue;
            }
            let bit_kappa = u16::try_from(telescope_bit_cost(candidate.variant.presentation()))
                .expect("bootstrap bit cost fits u16");
            if let Some(rank) = acceptance_rank_for_telescope(
                bar,
                candidate.variant.presentation(),
                u16::try_from(nu).expect("EGP-v2 bootstrap nu fits u16"),
                bit_kappa,
                candidate.kappa,
            ) {
                ranked.push((rank, candidate.candidate_id.clone(), index));
            }
        }
        ranked.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
        let order = ranked
            .iter()
            .map(|(_, candidate_id, _)| candidate_id.clone())
            .collect::<Vec<_>>();

        let winner = ranked.first().map(|(rank, _, index)| {
            let candidate = &candidates[*index];
            BootstrapWinner {
                candidate_id: candidate.candidate_id.clone(),
                presentation_hash: candidate.presentation_hash.clone(),
                canonical_key: candidate.canonical_key.clone(),
                variant: candidate.variant.clone(),
                basis_certificate_digest: candidate
                    .basis_certificate_digest
                    .clone()
                    .expect("a ranked candidate has a basis certificate"),
                kappa: candidate.kappa,
                revised_nu: candidate
                    .revised_nu
                    .expect("a ranked candidate has a score"),
                rho: candidate.rho.clone().expect("a ranked candidate has rho"),
                overshoot: rational_string(rank.overshoot),
            }
        });

        let (ledger_after, bar_after) = if let Some(winner) = &winner {
            apply_winner_to_ledger(stage, &signature, &mut ledger, winner)?;
            revised_records.push(DiscoveryRecord::new(
                stage,
                winner.revised_nu,
                u32::from(winner.kappa),
            ));
            accepted_telescopes.push((stage, winner.variant.presentation().clone()));
            structural_library.push(LibraryEntry::from_telescope(
                winner.variant.presentation(),
                &structural_library,
            ));
            (
                ledger.snapshot.clone(),
                Some(rational_string(
                    compute_bar(usize::from(WINDOW_DEPTH), stage + 1, &revised_records).bar,
                )),
            )
        } else {
            outcome = BootstrapOutcome::HaltedNoClearingCandidate {
                stage,
                bar: rational_string(bar),
            };
            (ledger.snapshot.clone(), None)
        };

        let mut stage_certificate = BootstrapStage {
            stage,
            prior_stage_certificate_digest,
            semantic_ledger_before: ledger_before,
            bar_before: rational_string(bar),
            cone_enumerated,
            cone_admitted,
            cone_deduped,
            semantic_variants: candidates.len(),
            cone_digest,
            candidates: std::mem::take(&mut candidates),
            order,
            winner,
            semantic_ledger_after: ledger_after,
            bar_after,
            stage_certificate_digest: String::new(),
        };
        stage_certificate.stage_certificate_digest = stage_digest(&stage_certificate);
        let halted = stage_certificate.winner.is_none();
        stages.push(stage_certificate);
        if halted {
            break;
        }
    }

    let revised_history = revised_records
        .iter()
        .map(|record| (record.step_index, record.nu, record.kappa))
        .collect();
    let mut raw = RawBootstrapRun {
        law_id: EGP_V2_LAW_ID.to_owned(),
        schema_version: EGP_V2_BOOTSTRAP_SCHEMA_VERSION,
        date: EGP_V2_BOOTSTRAP_DATE.to_owned(),
        burned_run: true,
        frozen_hashes,
        outcome,
        stages,
        revised_history,
        final_semantic_ledger: ledger.snapshot,
        raw_run_digest: String::new(),
    };
    raw.raw_run_digest = digest("egp-v2-raw-bootstrap-run", &raw);
    Ok(raw)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct LegacyComparisonStage {
    stage: u32,
    winner_id: String,
    telescope: Telescope,
    nu: u32,
    kappa: u32,
    bar_before: String,
    bar_after: String,
}

/// This is the only function in this module that constructs the sealed
/// reference telescopes or asks the shipped evaluator for their scores.
fn reconstruct_sealed_legacy_comparison() -> Vec<LegacyComparisonStage> {
    let mut library: Library = Vec::new();
    let mut history: Vec<(u32, u32)> = Vec::new();
    let mut records: Vec<DiscoveryRecord> = Vec::new();
    let mut stages = Vec::new();
    for stage in 1..=LAST_BOOTSTRAP_STAGE {
        let telescope = Telescope::reference(stage);
        let bar_before = compute_bar(usize::from(WINDOW_DEPTH), stage, &records).bar;
        let result = pen_eval::nu::structural_nu(&telescope, &library, &history);
        let kappa = u32::try_from(telescope.kappa()).expect("legacy bootstrap kappa fits u32");
        records.push(DiscoveryRecord::new(stage, result.total, kappa));
        let bar_after = compute_bar(usize::from(WINDOW_DEPTH), stage + 1, &records).bar;
        stages.push(LegacyComparisonStage {
            stage,
            winner_id: candidate_hash(&telescope),
            telescope: telescope.clone(),
            nu: result.total,
            kappa,
            bar_before: rational_string(bar_before),
            bar_after: rational_string(bar_after),
        });
        history.push((stage, result.total));
        library.push(LibraryEntry::from_telescope(&telescope, &library));
    }
    stages
}

fn legacy_comparison_digest(stages: &[LegacyComparisonStage]) -> String {
    digest("egp-v2-sealed-legacy-comparison", &stages)
}

fn first_divergence(
    raw_stage: &BootstrapStage,
    comparison: &StagePosthocComparison,
) -> Option<FirstBootstrapDivergence> {
    if !comparison.bar_before_matches_legacy {
        return Some(FirstBootstrapDivergence {
            stage: raw_stage.stage,
            field: "bar_before".to_owned(),
            legacy: comparison.legacy_bar_before.clone(),
            revised: raw_stage.bar_before.clone(),
        });
    }
    let Some(winner) = &raw_stage.winner else {
        return Some(FirstBootstrapDivergence {
            stage: raw_stage.stage,
            field: "no_clearing_candidate".to_owned(),
            legacy: comparison.legacy_winner_id.clone(),
            revised: "none".to_owned(),
        });
    };
    if !comparison.winner_semantically_equivalent {
        return Some(FirstBootstrapDivergence {
            stage: raw_stage.stage,
            field: "winner".to_owned(),
            legacy: comparison.legacy_winner_id.clone(),
            revised: winner.candidate_id.clone(),
        });
    }
    if !comparison.score_matches_legacy {
        return Some(FirstBootstrapDivergence {
            stage: raw_stage.stage,
            field: "score".to_owned(),
            legacy: comparison.legacy_nu.to_string(),
            revised: winner.revised_nu.to_string(),
        });
    }
    if !comparison.bar_after_matches_legacy {
        return Some(FirstBootstrapDivergence {
            stage: raw_stage.stage,
            field: "bar_after".to_owned(),
            legacy: comparison.legacy_bar_after.clone(),
            revised: raw_stage
                .bar_after
                .clone()
                .unwrap_or_else(|| "none".to_owned()),
        });
    }
    None
}

fn winner_at(raw: &RawBootstrapRun, stage: u32) -> Option<&BootstrapWinner> {
    raw.stages
        .iter()
        .find(|record| record.stage == stage)
        .and_then(|record| record.winner.as_ref())
}

fn stage_at(raw: &RawBootstrapRun, stage: u32) -> Option<&BootstrapStage> {
    raw.stages.iter().find(|record| record.stage == stage)
}

fn winning_basis(raw: &RawBootstrapRun, stage: u32) -> Option<&BasisCertificate> {
    let record = stage_at(raw, stage)?;
    let winner = record.winner.as_ref()?;
    record
        .candidates
        .iter()
        .find(|candidate| candidate.candidate_id == winner.candidate_id)
        .and_then(|candidate| candidate.basis_certificate.as_ref())
}

fn completion_basis(raw: &RawBootstrapRun, stage: u32) -> Option<&BasisCertificate> {
    stage_at(raw, stage)?
        .candidates
        .iter()
        .find(|candidate| {
            matches!(
                candidate.variant,
                BootstrapVariant::ExactConstructorCompletion { .. }
            ) && candidate.rejection.is_none()
        })
        .and_then(|candidate| candidate.basis_certificate.as_ref())
}

fn universe_package_is_one_covered_atom(certificate: &BasisCertificate) -> bool {
    let natural_targets = certificate
        .targets
        .iter()
        .filter(|target| target.kind == SemanticTargetKind::NaturalFamily)
        .count();
    certificate.revised_nu == 1
        && certificate.basis_atoms.len() == 1
        && certificate.minimum_basis.minimum_cardinality == 1
        && natural_targets == 2
        && certificate.coverage_derivations.iter().any(|derivation| {
            matches!(
                derivation.rule,
                CoverageRule::RegisteredCompletion {
                    rule: RegisteredCompletionRule::UniverseDecodeFromSamePackageFormation,
                    ..
                }
            )
        })
}

fn fresh_formation_is_one_atom(certificate: &BasisCertificate) -> bool {
    certificate.revised_nu == 1
        && certificate.basis_atoms.len() == 1
        && certificate
            .targets
            .iter()
            .filter(|target| target.kind == SemanticTargetKind::FreshInductiveDeclaration)
            .count()
            == 1
        && matches!(
            certificate.basis_atoms[0].provenance,
            ProvenanceTag::Local { .. }
        )
}

fn generated_unit_completion_is_covered(certificate: &BasisCertificate) -> bool {
    let has_eliminator = certificate
        .targets
        .iter()
        .any(|target| target.kind == SemanticTargetKind::GeneratedOneConstructorEliminator);
    let has_beta = certificate
        .targets
        .iter()
        .any(|target| target.kind == SemanticTargetKind::GeneratedOneConstructorBeta);
    let covers_eliminator = certificate.coverage_derivations.iter().any(|derivation| {
        matches!(
            derivation.rule,
            CoverageRule::RegisteredCompletion {
                rule: RegisteredCompletionRule::OneConstructorEliminator,
                ..
            }
        )
    });
    let covers_beta = certificate.coverage_derivations.iter().any(|derivation| {
        matches!(
            derivation.rule,
            CoverageRule::RegisteredCompletion {
                rule: RegisteredCompletionRule::OneConstructorBeta,
                ..
            }
        )
    });
    let generated_targets_are_not_atoms = certificate.basis_atoms.iter().all(|atom| {
        !certificate.targets.iter().any(|target| {
            target.id == atom.target
                && matches!(
                    target.kind,
                    SemanticTargetKind::GeneratedOneConstructorEliminator
                        | SemanticTargetKind::GeneratedOneConstructorBeta
                )
        })
    });
    let completion_is_one_demand_tag = certificate.revised_nu == 1
        && certificate.basis_atoms.len() == 1
        && matches!(
            certificate.basis_atoms[0].provenance,
            ProvenanceTag::LiveDemand { .. }
        );
    has_eliminator
        && has_beta
        && covers_eliminator
        && covers_beta
        && generated_targets_are_not_atoms
        && completion_is_one_demand_tag
}

fn alpha_reconstruction_is_invariant(raw: &RawBootstrapRun) -> Option<bool> {
    let winner = winner_at(raw, 2)?;
    let BootstrapVariant::OneNullaryFormation { overlay } = &winner.variant else {
        return Some(false);
    };
    let declaration = overlay.declaration();
    let reconstructed = InductiveHeadDeclaration::one_nullary(
        declaration.site().clone(),
        declaration.classifier().clone(),
    );
    let reconstructed_overlay =
        InductiveFormationOverlay::new(overlay.presentation().clone(), reconstructed.clone());
    Some(
        reconstructed.fresh_head() == declaration.fresh_head()
            && reconstructed_overlay.semantic_subject_digest() == overlay.semantic_subject_digest(),
    )
}

fn normalized_representations_do_not_mint_atoms(raw: &RawBootstrapRun) -> Option<bool> {
    let certificates = raw
        .stages
        .iter()
        .flat_map(|stage| &stage.candidates)
        .filter_map(|candidate| candidate.basis_certificate.as_ref())
        .collect::<Vec<_>>();
    if certificates.is_empty() {
        return None;
    }
    Some(certificates.into_iter().all(|certificate| {
        let targets = certificate
            .targets
            .iter()
            .map(|target| target.id.as_str())
            .collect::<BTreeSet<_>>();
        let basis_targets = certificate
            .basis_atoms
            .iter()
            .map(|atom| atom.target.as_str())
            .collect::<BTreeSet<_>>();
        let provenance = certificate
            .basis_atoms
            .iter()
            .map(|atom| &atom.provenance)
            .collect::<BTreeSet<_>>();
        targets.len() == certificate.targets.len()
            && basis_targets.len() == certificate.basis_atoms.len()
            && provenance.len() == certificate.basis_atoms.len()
            && certificate.provenance_injective
            && certificate.minimum_basis.minimum_cardinality == certificate.basis_atoms.len()
            && certificate.coverage_derivations.len() == certificate.targets.len()
    }))
}

fn computed_bootstrap_invariants(raw: &RawBootstrapRun) -> BootstrapInvariants {
    let stage2 = raw.stages.iter().find(|stage| stage.stage == 2);
    let stage3 = raw.stages.iter().find(|stage| stage.stage == 3);

    let stage1_one_atom = winning_basis(raw, 1).map(universe_package_is_one_covered_atom);
    let stage2_fresh = winner_at(raw, 2).map(|winner| {
        winning_basis(raw, 2).is_some_and(fresh_formation_is_one_atom)
            && matches!(winner.variant, BootstrapVariant::OneNullaryFormation { .. })
    });
    let stage2_one_live = stage2.map(|record| {
        record.semantic_ledger_after.live_constructor.is_some()
            && record
                .semantic_ledger_after
                .answered_constructors
                .is_empty()
    });
    let stage3_answers = stage3.map(|record| {
        matches!(
            record.winner.as_ref().map(|winner| &winner.variant),
            Some(BootstrapVariant::ExactConstructorCompletion { .. })
        ) && record
            .semantic_ledger_before
            .live_constructor
            .as_ref()
            .is_some_and(|live| {
                record
                    .semantic_ledger_after
                    .answered_constructors
                    .iter()
                    .any(|answered| answered.demand_id == live.demand_id)
            })
    });
    let stage3_sealed = stage3.map(|record| {
        record.semantic_ledger_after.live_constructor.is_none()
            && record.semantic_ledger_after.sealed_signatures.len()
                == record.semantic_ledger_before.sealed_signatures.len() + 1
    });

    BootstrapInvariants {
        stage1_one_atom_covers_universe_package: stage1_one_atom,
        stage2_fresh_unit_atom: stage2_fresh,
        stage2_one_live_constructor_orbit: stage2_one_live,
        stage3_star_answers_that_orbit: stage3_answers,
        stage3_signature_sealed: stage3_sealed,
        stage3_generated_unit_ind_and_beta_covered: completion_basis(raw, 3)
            .map(generated_unit_completion_is_covered),
        alpha_renaming_invariant: alpha_reconstruction_is_invariant(raw),
        univalent_representations_do_not_mint_atoms: normalized_representations_do_not_mint_atoms(
            raw,
        ),
    }
}

/// Attach the post-hoc comparison to an already frozen semantic run.
pub fn compare_with_sealed_legacy(
    raw: RawBootstrapRun,
) -> Result<EgpV2BootstrapCertificate, BootstrapError> {
    if raw.raw_run_digest.is_empty() {
        return Err(BootstrapError::RawDigestMissing);
    }
    let legacy = reconstruct_sealed_legacy_comparison();
    if legacy_comparison_digest(&legacy) != raw.frozen_hashes.sealed_legacy_comparison_trace {
        return Err(BootstrapError::LegacyCommitmentDrift);
    }

    let mut posthoc = Vec::new();
    let mut divergence = None;
    for raw_stage in &raw.stages {
        let legacy_stage = &legacy[raw_stage.stage as usize - 1];
        let winner = raw_stage.winner.as_ref();
        let comparison = StagePosthocComparison {
            stage: raw_stage.stage,
            legacy_winner_id: legacy_stage.winner_id.clone(),
            winner_semantically_equivalent: winner
                .is_some_and(|winner| winner.variant.presentation() == &legacy_stage.telescope),
            legacy_nu: legacy_stage.nu,
            score_matches_legacy: winner.is_some_and(|winner| winner.revised_nu == legacy_stage.nu),
            legacy_bar_before: legacy_stage.bar_before.clone(),
            bar_before_matches_legacy: raw_stage.bar_before == legacy_stage.bar_before,
            legacy_bar_after: legacy_stage.bar_after.clone(),
            bar_after_matches_legacy: raw_stage.bar_after.as_deref()
                == Some(legacy_stage.bar_after.as_str()),
        };
        if divergence.is_none() {
            divergence = first_divergence(raw_stage, &comparison);
        }
        posthoc.push(comparison);
    }

    let bootstrap_invariants = computed_bootstrap_invariants(&raw);
    let mut certificate = EgpV2BootstrapCertificate {
        raw,
        posthoc_comparison: posthoc,
        first_divergence: divergence,
        bootstrap_invariants,
        run_digest: String::new(),
    };
    certificate.run_digest = digest("egp-v2-bootstrap-certificate", &certificate);
    Ok(certificate)
}

pub fn run_egp_v2_bootstrap() -> Result<EgpV2BootstrapCertificate, BootstrapError> {
    let raw = run_raw_bootstrap()?;
    compare_with_sealed_legacy(raw)
}

pub fn replay_egp_v2_bootstrap(certificate: &EgpV2BootstrapCertificate) -> Result<(), String> {
    let replayed = run_egp_v2_bootstrap().map_err(|error| error.to_string())?;
    if &replayed == certificate {
        Ok(())
    } else {
        Err("EGP-v2 bootstrap replay diverged from the burned certificate".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stage_one_signature() -> SealedSignature {
        SealedSignature::from_telescopes(vec![(1, Telescope::reference(1))])
    }

    fn dummy_winner(variant: BootstrapVariant) -> BootstrapWinner {
        BootstrapWinner {
            candidate_id: digest("test-candidate", &variant),
            presentation_hash: candidate_hash(variant.presentation()),
            canonical_key: canonical_key_telescope(variant.presentation()).0,
            variant,
            basis_certificate_digest: "blake3:test-basis".to_owned(),
            kappa: 1,
            revised_nu: 1,
            rho: "1/1".to_owned(),
            overshoot: "0/1".to_owned(),
        }
    }

    #[test]
    fn variants_expand_from_state_not_stage_or_reference_identity() {
        let signature = SealedSignature::from_telescopes(Vec::new());
        let ledger = RuntimeLedger::initial(&signature);
        let single = Telescope::reference(2);
        let multiple = Telescope::reference(1);
        let variants =
            expand_semantic_variants(&signature, &ledger, &[single.clone(), multiple.clone()]);
        assert_eq!(variants.len(), 3);
        assert!(matches!(variants[0], BootstrapVariant::Plain { .. }));
        assert!(matches!(
            variants[1],
            BootstrapVariant::OneNullaryFormation { .. }
        ));
        assert!(matches!(variants[2], BootstrapVariant::Plain { .. }));

        let predecessor = stage_one_signature();
        let formation_overlay = InductiveFormationOverlay::new(
            single.clone(),
            InductiveHeadDeclaration::one_nullary(
                FreshExportSite::new(predecessor.digest(), 0),
                DeclarationType::existing(Expr::Univ),
            ),
        );
        let formation_evidence =
            issue_inductive_formation(&predecessor, &formation_overlay).expect("formation issues");
        let demand = live_demand(&formation_overlay, &formation_evidence).expect("one slot");
        let live_ledger = RuntimeLedger {
            snapshot: SemanticLedgerSnapshot {
                prior_digest: None,
                live_constructor: Some(demand.clone()),
                answered_constructors: Vec::new(),
                sealed_signatures: Vec::new(),
                digest: "blake3:live-test-ledger".to_owned(),
            },
            live: Some(LiveFormationRuntime {
                overlay: formation_overlay,
                evidence: formation_evidence,
                demand,
            }),
        };
        let current =
            SealedSignature::from_telescopes(vec![(1, Telescope::reference(1)), (2, single)]);
        let completion_variants =
            expand_semantic_variants(&current, &live_ledger, &[Telescope::reference(3)]);
        assert_eq!(completion_variants.len(), 2);
        assert!(matches!(
            completion_variants[1],
            BootstrapVariant::ExactConstructorCompletion { .. }
        ));
    }

    #[test]
    fn ledger_consumes_exact_slot_once_and_seals() {
        let predecessor = stage_one_signature();
        let mut ledger = RuntimeLedger::initial(&predecessor);
        let formation = BootstrapVariant::OneNullaryFormation {
            overlay: InductiveFormationOverlay::new(
                Telescope::reference(2),
                InductiveHeadDeclaration::one_nullary(
                    FreshExportSite::new(predecessor.digest(), 0),
                    DeclarationType::existing(Expr::Univ),
                ),
            ),
        };
        apply_winner_to_ledger(2, &predecessor, &mut ledger, &dummy_winner(formation))
            .expect("formation opens one demand");
        assert!(ledger.snapshot.live_constructor.is_some());

        let current = SealedSignature::from_telescopes(vec![
            (1, Telescope::reference(1)),
            (2, Telescope::reference(2)),
        ]);
        let completion = expand_semantic_variants(&current, &ledger, &[Telescope::reference(3)])
            .into_iter()
            .find(|variant| matches!(variant, BootstrapVariant::ExactConstructorCompletion { .. }))
            .expect("exact completion variant");
        let completion_winner = dummy_winner(completion);
        apply_winner_to_ledger(3, &current, &mut ledger, &completion_winner)
            .expect("completion consumes exact slot");
        assert!(ledger.snapshot.live_constructor.is_none());
        assert_eq!(ledger.snapshot.answered_constructors.len(), 1);
        assert_eq!(ledger.snapshot.sealed_signatures.len(), 1);

        let second_fill = apply_winner_to_ledger(4, &current, &mut ledger, &completion_winner);
        assert!(matches!(second_fill, Err(BootstrapError::Ledger { .. })));
    }

    #[test]
    fn burned_bootstrap_replays_and_pins_stage_three_halt() {
        let certificate = run_egp_v2_bootstrap().expect("burned EGP-v2 bootstrap publishes");
        assert_eq!(
            certificate.raw.outcome,
            BootstrapOutcome::HaltedNoClearingCandidate {
                stage: 3,
                bar: "4/3".to_owned(),
            }
        );
        assert_eq!(certificate.raw.stages.len(), 3);
        assert!(matches!(
            winner_at(&certificate.raw, 1).map(|winner| &winner.variant),
            Some(BootstrapVariant::Plain { .. })
        ));
        assert_eq!(
            winner_at(&certificate.raw, 1).map(|winner| winner.revised_nu),
            Some(1)
        );
        assert!(matches!(
            winner_at(&certificate.raw, 2).map(|winner| &winner.variant),
            Some(BootstrapVariant::OneNullaryFormation { .. })
        ));
        assert_eq!(
            winner_at(&certificate.raw, 2).map(|winner| winner.revised_nu),
            Some(1)
        );
        assert!(winner_at(&certificate.raw, 3).is_none());

        let completion = stage_at(&certificate.raw, 3)
            .expect("halted stage is published")
            .candidates
            .iter()
            .find(|candidate| {
                matches!(
                    candidate.variant,
                    BootstrapVariant::ExactConstructorCompletion { .. }
                )
            })
            .expect("completion variant is in the complete cone");
        assert_eq!(completion.revised_nu, Some(1));
        assert!(!completion.clears_bar);
        assert!(completion.rejection.is_none());
        assert!(
            completion
                .basis_certificate
                .as_ref()
                .is_some_and(generated_unit_completion_is_covered)
        );
        assert_eq!(
            certificate
                .first_divergence
                .as_ref()
                .map(|d| (d.stage, d.field.as_str())),
            Some((3, "no_clearing_candidate"))
        );
        assert_eq!(
            certificate
                .bootstrap_invariants
                .stage1_one_atom_covers_universe_package,
            Some(true)
        );
        assert_eq!(
            certificate.bootstrap_invariants.stage2_fresh_unit_atom,
            Some(true)
        );
        assert_eq!(
            certificate
                .bootstrap_invariants
                .stage2_one_live_constructor_orbit,
            Some(true)
        );
        assert_eq!(
            certificate
                .bootstrap_invariants
                .stage3_star_answers_that_orbit,
            Some(false)
        );
        assert_eq!(
            certificate.bootstrap_invariants.stage3_signature_sealed,
            Some(false)
        );
        assert_eq!(
            certificate
                .bootstrap_invariants
                .stage3_generated_unit_ind_and_beta_covered,
            Some(true)
        );
        replay_egp_v2_bootstrap(&certificate).expect("burned artifact replays byte-for-byte");
    }

    #[test]
    fn sealed_legacy_commitment_reconstructs_only_in_posthoc_grader() {
        assert_eq!(
            legacy_comparison_digest(&reconstruct_sealed_legacy_comparison()),
            sealed_legacy_commitment()
        );
    }
}
