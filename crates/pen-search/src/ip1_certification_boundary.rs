//! IP-1: the Step-16 certification-boundary experiment.
//!
//! This module assembles, without changing the historical selector, the two
//! finite facts needed by `docs/ip1_tdc1_plan.md`:
//!
//! 1. the complete nine-way [`TelescopeClass`] classifier has a total
//!    evidence rule; and
//! 2. every conditional support-local ceiling lies below the sealed Step-16
//!    bar, while every token-gated branch names evidence that the frozen
//!    public API cannot construct.
//!
//! The selection clause A5 is deliberately an input.  The default artifact
//! keeps it `Undecided`; building or replaying this certificate cannot silently
//! adopt a new law.

#[path = "ip1_candidate_join.rs"]
pub mod candidate_join;

use crate::enumerate::{EnumerationContext, assess_raw_surface_membership};
use pen_core::canonical::canonical_key_telescope;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::bar::{clears_bar, compute_rho};
use pen_eval::certified_novelty::{
    CertifiedSurfaceCaps, ClassCeilings, FreshKernelCertificate, ReplayedP5LiftCapability,
    derive_linear_bound,
};
use pen_eval::halting::{accepted_canonical_keys, genesis_bar_16, genesis_history};
use pen_eval::minimality::analyze_semantic_minimality;
use pen_eval::nu::structural_nu;
use pen_type::admissibility::{
    AdmissibilityMode, assess_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::check::{CheckResult, check_telescope};
use pen_type::connectivity::{analyze_connectivity, passes_connectivity};
use pen_type::elaborate::{SealedSignature, issue_typed_lift_token};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const IP1_DATE: &str = "2026-07-18";
pub const IP1_SCHEMA_VERSION: u32 = 2;
const BAR_NUMERATOR: u64 = 354_333;
const BAR_DENOMINATOR: u64 = 39_040;
const EXPECTED_EXHAUSTION_DIGEST: &str =
    "blake3:5f2ff655260c7c5a7894557aef445d97bd45907e41a522f626c836aaf75bade8";
const EXHAUSTION_ARTIFACT: &str = include_str!("../../../docs/step16_semantic_exhaustion.json");
const CERTIFIED_NOVELTY_SOURCE: &str = include_str!("../../pen-eval/src/certified_novelty.rs");

const ALL_CLASSES: [TelescopeClass; 9] = [
    TelescopeClass::Foundation,
    TelescopeClass::Former,
    TelescopeClass::Hit,
    TelescopeClass::Suspension,
    TelescopeClass::Map,
    TelescopeClass::Modal,
    TelescopeClass::Axiomatic,
    TelescopeClass::Synthesis,
    TelescopeClass::Unknown,
];

const TRUSTED_TOKEN_TYPES: [&str; 6] = [
    "TrustedTransparentElaborationToken",
    "TrustedHFormEliminatorToken",
    "ReplayedP5LiftCapability",
    "TrustedP5RecordInternalityCapability",
    "TrustedSynthesisPolymorphicEliminatorToken",
    "TrustedSynthesisNaturalityToken",
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum A5Adjudication {
    Undecided,
    Adopted,
    Rejected,
}

impl A5Adjudication {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "undecided" => Some(Self::Undecided),
            "adopted" => Some(Self::Adopted),
            "rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct A5Boundary {
    pub adjudication: A5Adjudication,
    pub clause: String,
    pub adjudication_source: String,
    pub theorem_is_conditional_on_a5: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionalClassRuleKind {
    /// Typed normalization/weakening plus EGP gives a numeric verdict.
    EgpRankable,
    /// The class always reaches a named, publicly unconstructible token gate.
    RequiredTokenFailure,
    /// A syntactic predicate splits the class into the preceding two routes.
    EgpOrRequiredTokenFailure,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RequiredTokenFailure {
    pub token_type: String,
    pub outcome: String,
    pub condition: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClassCeilingCase {
    pub kappa: u16,
    pub conditional_ceiling_nu: u32,
    pub first_clearing_integer_nu: u32,
    pub cross_product_left: u64,
    pub cross_product_right: u64,
    pub strictly_below_bar: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClassBoundaryCase {
    pub class: TelescopeClass,
    pub class_label: String,
    /// A formula-level rule from `certified_novelty`, not an exhaustive
    /// candidate-wise verdict.  IP-1 specifically audits the missing join.
    pub conditional_rule_kind: ConditionalClassRuleKind,
    pub decision_rule: String,
    pub egp_outcome: Option<String>,
    pub required_token_failures: Vec<RequiredTokenFailure>,
    pub ceilings: Vec<ClassCeilingCase>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicTokenAudit {
    pub token_type: String,
    /// This audit covers only the disconnected `certified_novelty::Trusted*`
    /// wrapper.  It is not used to negate a public `pen_type` issuance route.
    pub audit_scope: String,
    pub struct_found: bool,
    pub all_fields_private: bool,
    pub public_associated_constructors: Vec<String>,
    pub constructible_from_frozen_public_api: bool,
}

/// Concrete F-IP2 witness replayed through the frozen Step-16 gate stack.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublicRequiredTokenWitness {
    pub name: String,
    pub candidate: Telescope,
    pub canonical_key: String,
    pub class: TelescopeClass,
    pub raw_surface_member: bool,
    pub raw_surface_rejections: Vec<String>,
    pub shallow_type_checks: bool,
    pub guarded_admitted: bool,
    pub admissibility_class: String,
    pub admissibility_reason: String,
    pub connectivity_passes: bool,
    pub references_active_window: bool,
    pub identified_with_sealed_structure: bool,
    pub semantically_minimal: bool,
    pub bar_clearing_detachable_subbundles: usize,
    pub structural_nu: u32,
    pub structural_rho: String,
    pub structural_clears_bar: bool,
    pub public_issuer: String,
    pub required_token_capability: String,
    pub token_issued: bool,
    pub token_subject_hash: Option<String>,
    pub token_signature_digest: Option<String>,
    pub token_derivation_hash: Option<String>,
    pub token_dominant_import: Option<u32>,
    pub token_lift_clauses: Vec<u16>,
    pub token_error: Option<String>,
    pub sidecar_adapter_succeeded: bool,
    pub sidecar_adapter_error: Option<String>,
    /// Independent minimal-record/API evidence required in addition to the
    /// replayed lift.  There is intentionally no production constructor yet.
    pub record_internality_capability_available: bool,
    pub complete_p5_route_succeeded: bool,
    pub public_fresh_kernel_assertion_available: bool,
    pub survives_frozen_candidate_gates: bool,
    pub falsifies_f_ip2: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RawPartitionStratum {
    pub kappa: u16,
    pub raw_total: String,
    pub internal: String,
    pub egp_marginal: String,
    pub invalid_named_bare_univ: String,
    pub unclassified: String,
    pub sum_matches_raw_total: bool,
    pub unclassified_is_zero: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoundaryCompleteness {
    pub raw_product_partition_complete: bool,
    pub classifier_total_by_exhaustive_match: bool,
    pub all_nine_classes_mapped_once: bool,
    pub every_class_has_conditional_formula_rule: bool,
    /// False in this burn: the raw exhaustion does not carry TelescopeClass,
    /// token outcomes, or certified nu, so it cannot be joined candidate-wise
    /// to the nine formula rows.
    pub candidate_wise_class_to_verdict_join_proved: bool,
    pub class_rows_are_conditional_formula_rules: bool,
    pub every_conditional_ceiling_strictly_below_bar: bool,
    pub every_token_failure_named: bool,
    pub no_public_required_token_route: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ip1Falsifiers {
    pub f_ip1_uncovered_raw_class: bool,
    pub f_ip2_public_required_token_route: bool,
    pub f_ip3_a5_rejected: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ip1CertificationBoundary {
    pub schema_version: u32,
    pub date: String,
    pub scope: String,
    pub a5: A5Boundary,
    pub bar_numerator: u64,
    pub bar_denominator: u64,
    pub bar_16: String,
    pub surface_caps: CertifiedSurfaceCaps,
    pub semantic_exhaustion_digest: String,
    pub semantic_exhaustion_signature_digest: String,
    pub semantic_exhaustion_closure_digest: String,
    pub raw_partition: Vec<RawPartitionStratum>,
    pub classes: Vec<ClassBoundaryCase>,
    pub token_source_digest: String,
    pub public_token_audit: Vec<PublicTokenAudit>,
    pub public_required_token_witnesses: Vec<PublicRequiredTokenWitness>,
    pub completeness: BoundaryCompleteness,
    pub falsifiers: Ip1Falsifiers,
    pub mechanical_boundary_complete: bool,
    pub no_certifiably_clearing_step16: bool,
    pub theorem_status: String,
    pub digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ip1Replay {
    pub valid: bool,
    pub mechanical_boundary_complete: bool,
    pub no_certifiably_clearing_step16: bool,
    pub theorem_status: String,
    pub errors: Vec<String>,
}

#[derive(Deserialize)]
struct ExhaustionArtifact {
    signature_digest: String,
    closure_digest: String,
    strata: Vec<ExhaustionArtifactStratum>,
    every_stratum_classified: bool,
    digest: String,
}

#[derive(Deserialize)]
struct ExhaustionArtifactStratum {
    kappa: u16,
    raw_total: String,
    internal: String,
    egp_marginal: String,
    invalid_named_bare_univ: String,
    unclassified: String,
    sums_match: bool,
    unclassified_is_zero: bool,
}

fn class_label(class: TelescopeClass) -> &'static str {
    match class {
        TelescopeClass::Foundation => "foundation",
        TelescopeClass::Former => "former",
        TelescopeClass::Hit => "hit",
        TelescopeClass::Suspension => "suspension",
        TelescopeClass::Map => "map",
        TelescopeClass::Modal => "modal",
        TelescopeClass::Axiomatic => "axiomatic",
        TelescopeClass::Synthesis => "synthesis",
        TelescopeClass::Unknown => "unknown",
    }
}

fn class_ceiling(ceilings: &ClassCeilings, class: TelescopeClass) -> u32 {
    match class {
        TelescopeClass::Foundation => ceilings.foundation,
        TelescopeClass::Former => ceilings.former,
        TelescopeClass::Hit => ceilings.hit,
        TelescopeClass::Suspension => ceilings.suspension,
        TelescopeClass::Map => ceilings.map,
        TelescopeClass::Modal => ceilings.modal,
        TelescopeClass::Axiomatic => ceilings.axiomatic,
        TelescopeClass::Synthesis => ceilings.synthesis,
        TelescopeClass::Unknown => ceilings.unknown,
    }
}

fn first_clearing_integer(kappa: u16) -> u32 {
    let numerator = BAR_NUMERATOR * u64::from(kappa);
    u32::try_from((numerator + BAR_DENOMINATOR - 1) / BAR_DENOMINATOR)
        .expect("Step-16 threshold fits u32")
}

fn class_rule(
    class: TelescopeClass,
) -> (
    ConditionalClassRuleKind,
    &'static str,
    Option<&'static str>,
    Vec<RequiredTokenFailure>,
) {
    let rankable = || {
        (
            ConditionalClassRuleKind::EgpRankable,
            "typed extraction decides internal-by-weakening versus EGP-marginal; no class amplification token is required",
            Some("normalized Marg2 families carry generator-clause EGP anchors"),
            Vec::new(),
        )
    };
    match class {
        TelescopeClass::Foundation
        | TelescopeClass::Former
        | TelescopeClass::Suspension
        | TelescopeClass::Map
        | TelescopeClass::Modal
        | TelescopeClass::Unknown => rankable(),
        TelescopeClass::Hit => (
            ConditionalClassRuleKind::RequiredTokenFailure,
            "certified_novelty::evaluate_opaque requires FrozenHFormCertificate for every Hit; complete() in turn requires its private typed-eliminator token",
            None,
            vec![RequiredTokenFailure {
                token_type: "TrustedHFormEliminatorToken".to_owned(),
                outcome: "no_public_constructor".to_owned(),
                condition: "TelescopeClass::Hit".to_owned(),
            }],
        ),
        TelescopeClass::Axiomatic => (
            ConditionalClassRuleKind::RequiredTokenFailure,
            "evaluate_opaque first rejects absence of a unique dominant import; otherwise FrozenP5Certificate requires both a non-vacuous TypedLiftToken replayed through ReplayedP5LiftCapability and independent TrustedP5RecordInternalityCapability evidence",
            None,
            vec![
                RequiredTokenFailure {
                    token_type: "P5UniqueDominantImportPremise".to_owned(),
                    outcome: "no_unique_dominant_import_or_continue_to_token_gate".to_owned(),
                    condition: "P5ImportAudit::unique_dominant_import.is_none()".to_owned(),
                },
                RequiredTokenFailure {
                    token_type: "ReplayedP5LiftCapability".to_owned(),
                    outcome:
                        "requires_successful_non_vacuous_kernel_issuance_and_definition_replay"
                            .to_owned(),
                    condition: "P5ImportAudit::unique_dominant_import.is_some()".to_owned(),
                },
                RequiredTokenFailure {
                    token_type: "TrustedP5RecordInternalityCapability".to_owned(),
                    outcome: "no_public_constructor".to_owned(),
                    condition: "P5ImportAudit::unique_dominant_import.is_some()".to_owned(),
                },
            ],
        ),
        TelescopeClass::Synthesis => (
            ConditionalClassRuleKind::EgpOrRequiredTokenFailure,
            "empty synthesis_sites is EGP-rankable at the base ceiling; a nonempty site set requires both private synthesis tokens",
            Some("synthesis_sites(candidate).is_empty() gives the normalized base-family verdict"),
            vec![
                RequiredTokenFailure {
                    token_type: "TrustedSynthesisPolymorphicEliminatorToken".to_owned(),
                    outcome: "no_public_constructor".to_owned(),
                    condition: "!synthesis_sites(candidate).is_empty()".to_owned(),
                },
                RequiredTokenFailure {
                    token_type: "TrustedSynthesisNaturalityToken".to_owned(),
                    outcome: "no_public_constructor".to_owned(),
                    condition: "!synthesis_sites(candidate).is_empty()".to_owned(),
                },
            ],
        ),
    }
}

fn class_cases(caps: &CertifiedSurfaceCaps) -> Vec<ClassBoundaryCase> {
    let theorem = derive_linear_bound(caps).expect("frozen IP-1 caps are valid");
    ALL_CLASSES
        .into_iter()
        .map(|class| {
            let (conditional_rule_kind, decision_rule, egp_outcome, required_token_failures) =
                class_rule(class);
            let ceilings = theorem
                .per_kappa
                .iter()
                .map(|row| {
                    let conditional_ceiling_nu = class_ceiling(row, class);
                    let cross_product_left = BAR_DENOMINATOR * u64::from(conditional_ceiling_nu);
                    let cross_product_right = BAR_NUMERATOR * u64::from(row.kappa);
                    ClassCeilingCase {
                        kappa: row.kappa,
                        conditional_ceiling_nu,
                        first_clearing_integer_nu: first_clearing_integer(row.kappa),
                        cross_product_left,
                        cross_product_right,
                        strictly_below_bar: cross_product_left < cross_product_right,
                    }
                })
                .collect();
            ClassBoundaryCase {
                class,
                class_label: class_label(class).to_owned(),
                conditional_rule_kind,
                decision_rule: decision_rule.to_owned(),
                egp_outcome: egp_outcome.map(str::to_owned),
                required_token_failures,
                ceilings,
            }
        })
        .collect()
}

fn braced_body_after<'a>(source: &'a str, marker: &str) -> Option<&'a str> {
    let marker_start = source.find(marker)?;
    let tail = &source[marker_start + marker.len()..];
    let open_relative = tail.find('{')?;
    let open = marker_start + marker.len() + open_relative;
    let mut depth = 0u32;
    for (offset, byte) in source.as_bytes()[open..].iter().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open + 1..open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn public_associated_constructors(impl_body: &str) -> Vec<String> {
    let mut constructors = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = impl_body[cursor..].find("pub ") {
        let start = cursor + relative;
        let tail = &impl_body[start..];
        let Some(fn_relative) = tail.find("fn ") else {
            break;
        };
        // A visibility item that is not a function should not borrow a later
        // function.  Restrict the scan to the current line prefix.
        if tail[..fn_relative].contains('\n') {
            cursor = start + 4;
            continue;
        }
        let signature_end = tail
            .find('{')
            .or_else(|| tail.find(';'))
            .unwrap_or(tail.len());
        let signature = &tail[..signature_end];
        if !signature.contains("&self") && !signature.contains("&mut self") {
            let name_tail = &tail[fn_relative + 3..];
            let name_end = name_tail
                .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
                .unwrap_or(name_tail.len());
            constructors.push(name_tail[..name_end].to_owned());
        }
        cursor = start + signature_end.max(4);
    }
    constructors.sort();
    constructors.dedup();
    constructors
}

fn audit_token(token_type: &str) -> PublicTokenAudit {
    let struct_marker = format!("pub struct {token_type}");
    let struct_body = braced_body_after(CERTIFIED_NOVELTY_SOURCE, &struct_marker);
    let all_fields_private = struct_body.is_some_and(|body| {
        body.lines()
            .filter(|line| !line.trim().is_empty())
            .all(|line| !line.trim_start().starts_with("pub "))
    });
    let impl_marker = format!("impl {token_type}");
    let constructors = braced_body_after(CERTIFIED_NOVELTY_SOURCE, &impl_marker)
        .map(public_associated_constructors)
        .unwrap_or_default();
    PublicTokenAudit {
        token_type: token_type.to_owned(),
        audit_scope: "certified_novelty::Trusted* wrapper only; public pen_type issuers are audited by concrete witnesses".to_owned(),
        struct_found: struct_body.is_some(),
        all_fields_private,
        constructible_from_frozen_public_api: !all_fields_private || !constructors.is_empty(),
        public_associated_constructors: constructors,
    }
}

fn p5_public_token_witness() -> PublicRequiredTokenWitness {
    // Found by the independent IP-1 audit after the plan was frozen.  Keep
    // the expression and declared roles literal: this is a replay witness,
    // not a schema selected to repair the theorem.
    let candidate = Telescope::new(vec![
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
        ),
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
        ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
    ]);
    let (library, pairs, _) = genesis_history();
    let admissibility = strict_admissibility_for_mode(16, 2, &library, AdmissibilityMode::Guarded);
    let context = EnumerationContext::from_admissibility(&library, admissibility);
    let raw = assess_raw_surface_membership(context, &candidate);
    let decision = assess_strict_admissibility(16, &library, &candidate, admissibility);
    let shallow_type_checks = matches!(check_telescope(&library, &candidate), CheckResult::Ok);
    let connectivity = analyze_connectivity(&library, &candidate);
    let connectivity_passes = passes_connectivity(&library, &candidate);
    let minimality = analyze_semantic_minimality(
        16,
        genesis_bar_16(),
        admissibility,
        &candidate,
        &library,
        &pairs,
    );
    let semantically_minimal = minimality.is_minimal();
    let bar_clearing_detachable_subbundles = minimality.admissible_bar_clear_subbundles.len();
    let identified_with_sealed_structure =
        accepted_canonical_keys().contains(&canonical_key_telescope(&candidate));
    let structural = structural_nu(&candidate, &library, &pairs);
    let rho = compute_rho(structural.total, candidate.kappa() as u32)
        .expect("the witness has positive kappa");
    let structural_clears_bar = clears_bar(rho, genesis_bar_16());
    let signature = SealedSignature::genesis_del_h15();
    let issued = issue_typed_lift_token(&signature, &candidate, 15);
    let (
        token_issued,
        token_subject_hash,
        token_signature_digest,
        token_derivation_hash,
        token_dominant_import,
        token_lift_clauses,
        token_error,
        sidecar_adapter_succeeded,
        sidecar_adapter_error,
    ) = match issued {
        Ok(token) => {
            let adapter =
                ReplayedP5LiftCapability::from_kernel_token(&signature, &candidate, 15, &token);
            let (sidecar_adapter_succeeded, sidecar_adapter_error) = match adapter {
                Ok(_) => (true, None),
                Err(error) => (false, Some(error.to_string())),
            };
            (
                true,
                Some(token.subject_hash().to_owned()),
                Some(token.signature_digest().to_owned()),
                Some(token.derivation_hash().to_owned()),
                Some(token.dominant_import()),
                token.lift_clauses().to_vec(),
                None,
                sidecar_adapter_succeeded,
                sidecar_adapter_error,
            )
        }
        Err(error) => (
            false,
            None,
            None,
            None,
            None,
            Vec::new(),
            Some(error.to_string()),
            false,
            Some("kernel token did not issue; sidecar replay was not attempted".to_owned()),
        ),
    };
    let fresh_kernel = FreshKernelCertificate::assert_all_clauses_opaque(&candidate, &library);
    let public_fresh_kernel_assertion_available =
        fresh_kernel.irreducible_clauses.len() == candidate.kappa();
    // Lift checking and record internality are deliberately independent.
    // The latter has no production constructor, so a successful public lift
    // adapter alone is not an end-to-end P5 route.
    let record_internality_capability_available = false;
    let complete_p5_route_succeeded =
        sidecar_adapter_succeeded && record_internality_capability_available;
    let survives_frozen_candidate_gates = raw.is_member
        && shallow_type_checks
        && decision.is_admitted()
        && connectivity_passes
        && connectivity.references_active_window
        && !identified_with_sealed_structure
        && semantically_minimal
        && bar_clearing_detachable_subbundles == 0;
    let falsifies_f_ip2 = survives_frozen_candidate_gates && complete_p5_route_succeeded;

    PublicRequiredTokenWitness {
        name: "p5_vacuous_public_typed_lift_rejected".to_owned(),
        canonical_key: canonical_key_telescope(&candidate).0,
        class: candidate.classify(&library),
        candidate,
        raw_surface_member: raw.is_member,
        raw_surface_rejections: raw.rejection_reasons,
        shallow_type_checks,
        guarded_admitted: decision.is_admitted(),
        admissibility_class: decision.class.as_str().to_owned(),
        admissibility_reason: decision.reason,
        connectivity_passes,
        references_active_window: connectivity.references_active_window,
        identified_with_sealed_structure,
        semantically_minimal,
        bar_clearing_detachable_subbundles,
        structural_nu: structural.total,
        structural_rho: rho.to_string(),
        structural_clears_bar,
        public_issuer: "pen_type::elaborate::issue_typed_lift_token".to_owned(),
        required_token_capability:
            "ReplayedP5LiftCapability plus TrustedP5RecordInternalityCapability".to_owned(),
        token_issued,
        token_subject_hash,
        token_signature_digest,
        token_derivation_hash,
        token_dominant_import,
        token_lift_clauses,
        token_error,
        sidecar_adapter_succeeded,
        sidecar_adapter_error,
        record_internality_capability_available,
        complete_p5_route_succeeded,
        public_fresh_kernel_assertion_available,
        survives_frozen_candidate_gates,
        falsifies_f_ip2,
    }
}

fn raw_partition() -> (String, String, String, Vec<RawPartitionStratum>, bool) {
    let artifact: ExhaustionArtifact =
        serde_json::from_str(EXHAUSTION_ARTIFACT).expect("frozen exhaustion artifact parses");
    assert_eq!(
        artifact.digest, EXPECTED_EXHAUSTION_DIGEST,
        "semantic exhaustion artifact changed without IP-1 re-registration"
    );
    let mut partition_complete = artifact.every_stratum_classified;
    let strata = artifact
        .strata
        .into_iter()
        .map(|row| {
            let raw = row.raw_total.parse::<u128>().expect("raw u128");
            let internal = row.internal.parse::<u128>().expect("internal u128");
            let marginal = row.egp_marginal.parse::<u128>().expect("marginal u128");
            let invalid = row
                .invalid_named_bare_univ
                .parse::<u128>()
                .expect("invalid u128");
            let unclassified = row.unclassified.parse::<u128>().expect("unclassified u128");
            let arithmetic_match = raw == internal + marginal + invalid + unclassified;
            let unclassified_is_zero = unclassified == 0;
            let sum_matches_raw_total = row.sums_match && arithmetic_match;
            partition_complete &=
                sum_matches_raw_total && row.unclassified_is_zero && unclassified_is_zero;
            RawPartitionStratum {
                kappa: row.kappa,
                raw_total: row.raw_total,
                internal: row.internal,
                egp_marginal: row.egp_marginal,
                invalid_named_bare_univ: row.invalid_named_bare_univ,
                unclassified: row.unclassified,
                sum_matches_raw_total,
                unclassified_is_zero,
            }
        })
        .collect();
    (
        artifact.digest,
        artifact.signature_digest,
        artifact.closure_digest,
        strata,
        partition_complete,
    )
}

fn certificate_digest(certificate: &Ip1CertificationBoundary) -> String {
    let mut payload = certificate.clone();
    payload.digest.clear();
    let bytes = serde_json::to_vec(&payload).expect("IP-1 certificate serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn build_certificate(adjudication: A5Adjudication) -> Ip1CertificationBoundary {
    let caps = CertifiedSurfaceCaps::genesis_step16();
    let classes = class_cases(&caps);
    let (exhaustion_digest, signature_digest, closure_digest, raw_partition, raw_complete) =
        raw_partition();
    let public_token_audit = TRUSTED_TOKEN_TYPES
        .into_iter()
        .map(audit_token)
        .collect::<Vec<_>>();
    let public_required_token_witnesses = vec![p5_public_token_witness()];
    let token_source_digest = format!("blake3:{}", blake3_hex(CERTIFIED_NOVELTY_SOURCE.as_bytes()));

    let mapped: BTreeSet<_> = classes.iter().map(|case| case.class).collect();
    let expected: BTreeSet<_> = ALL_CLASSES.into_iter().collect();
    let all_nine_classes_mapped_once = classes.len() == ALL_CLASSES.len() && mapped == expected;
    let every_class_has_rule = classes.iter().all(|case| {
        !case.decision_rule.is_empty()
            && match case.conditional_rule_kind {
                ConditionalClassRuleKind::EgpRankable => case.egp_outcome.is_some(),
                ConditionalClassRuleKind::RequiredTokenFailure => {
                    !case.required_token_failures.is_empty()
                }
                ConditionalClassRuleKind::EgpOrRequiredTokenFailure => {
                    case.egp_outcome.is_some() && !case.required_token_failures.is_empty()
                }
            }
    });
    let every_ceiling_below = classes
        .iter()
        .flat_map(|case| &case.ceilings)
        .all(|ceiling| ceiling.strictly_below_bar);
    let every_token_failure_named = classes
        .iter()
        .flat_map(|case| &case.required_token_failures)
        .all(|failure| {
            !failure.token_type.is_empty()
                && !failure.outcome.is_empty()
                && !failure.condition.is_empty()
        });
    // F-IP2 requires an end-to-end route, not merely a public function whose
    // result cannot discharge the sidecar premise.  P5 is audited through the
    // public issuer, replay adapter, AND independent record-internality
    // evidence; a failure at any boundary is a named token failure.  This
    // keeps the lift issuer public without turning it into full P5 evidence.
    let no_public_required_token_route = !public_required_token_witnesses
        .iter()
        .any(|witness| witness.falsifies_f_ip2);
    let completeness = BoundaryCompleteness {
        raw_product_partition_complete: raw_complete,
        // `class_label`, `class_ceiling`, and `class_rule` use exhaustive
        // matches with no wildcard. Rust compilation is the totality check.
        classifier_total_by_exhaustive_match: true,
        all_nine_classes_mapped_once,
        every_class_has_conditional_formula_rule: every_class_has_rule,
        candidate_wise_class_to_verdict_join_proved: false,
        class_rows_are_conditional_formula_rules: true,
        every_conditional_ceiling_strictly_below_bar: every_ceiling_below,
        every_token_failure_named,
        no_public_required_token_route,
    };
    let f_ip1_uncovered_raw_class = !(completeness.raw_product_partition_complete
        && completeness.classifier_total_by_exhaustive_match
        && completeness.all_nine_classes_mapped_once
        && completeness.every_class_has_conditional_formula_rule
        && completeness.candidate_wise_class_to_verdict_join_proved);
    let f_ip2_public_required_token_route = !completeness.no_public_required_token_route;
    let f_ip3_a5_rejected = adjudication == A5Adjudication::Rejected;
    let mechanical_boundary_complete = !f_ip1_uncovered_raw_class
        && !f_ip2_public_required_token_route
        && completeness.every_conditional_ceiling_strictly_below_bar
        && completeness.every_token_failure_named;
    let no_certifiably_clearing_step16 =
        mechanical_boundary_complete && adjudication == A5Adjudication::Adopted;
    let theorem_status = match adjudication {
        _ if !mechanical_boundary_complete => "blocked_by_ip1_falsifier",
        A5Adjudication::Undecided if mechanical_boundary_complete => {
            "conditional_boundary_complete_pending_a5"
        }
        A5Adjudication::Adopted if mechanical_boundary_complete => {
            "operational_halt_theorem_certified"
        }
        A5Adjudication::Rejected => "observation_only_a5_rejected",
        _ => "blocked_by_ip1_falsifier",
    }
    .to_owned();

    let mut certificate = Ip1CertificationBoundary {
        schema_version: IP1_SCHEMA_VERSION,
        date: IP1_DATE.to_owned(),
        scope: "frozen Step-16 raw surface: 2<=kappa<=4, r<=2, d<=1, six expression nodes; operational halt only".to_owned(),
        a5: A5Boundary {
            adjudication,
            clause: "On a debt-free field, acceptance requires a certified clearing; unrankable candidates cannot be accepted.".to_owned(),
            adjudication_source: "explicit caller/user authority; never inferred by the engine".to_owned(),
            theorem_is_conditional_on_a5: adjudication != A5Adjudication::Adopted,
        },
        bar_numerator: BAR_NUMERATOR,
        bar_denominator: BAR_DENOMINATOR,
        bar_16: "354333/39040".to_owned(),
        surface_caps: caps,
        semantic_exhaustion_digest: exhaustion_digest,
        semantic_exhaustion_signature_digest: signature_digest,
        semantic_exhaustion_closure_digest: closure_digest,
        raw_partition,
        classes,
        token_source_digest,
        public_token_audit,
        public_required_token_witnesses,
        completeness,
        falsifiers: Ip1Falsifiers {
            f_ip1_uncovered_raw_class,
            f_ip2_public_required_token_route,
            f_ip3_a5_rejected,
        },
        mechanical_boundary_complete,
        no_certifiably_clearing_step16,
        theorem_status,
        digest: String::new(),
    };
    certificate.digest = certificate_digest(&certificate);
    certificate
}

/// Build IP-1 under an explicit A5 adjudication.  Use `Undecided` unless the
/// law-maker has separately adopted or rejected A5.
pub fn run_ip1_certification_boundary(adjudication: A5Adjudication) -> Ip1CertificationBoundary {
    build_certificate(adjudication)
}

pub fn ip1_json_pretty(adjudication: A5Adjudication) -> String {
    serde_json::to_string_pretty(&run_ip1_certification_boundary(adjudication))
        .expect("IP-1 certificate serializes")
}

/// Full-definition replay.  Any flipped verdict, token outcome,
/// completeness bit, arithmetic row, or digest is rejected.
pub fn replay_ip1_certification_boundary(certificate: &Ip1CertificationBoundary) -> Ip1Replay {
    let mut errors = Vec::new();
    if certificate_digest(certificate) != certificate.digest {
        errors.push("IP-1 digest mismatch".to_owned());
    }
    if *certificate != build_certificate(certificate.a5.adjudication) {
        errors.push("IP-1 payload differs from fresh definition replay".to_owned());
    }
    if certificate.no_certifiably_clearing_step16
        && certificate.a5.adjudication != A5Adjudication::Adopted
    {
        errors.push("operational theorem claimed without adopted A5".to_owned());
    }
    Ip1Replay {
        valid: errors.is_empty(),
        mechanical_boundary_complete: certificate.mechanical_boundary_complete,
        no_certifiably_clearing_step16: certificate.no_certifiably_clearing_step16,
        theorem_status: certificate.theorem_status.clone(),
        errors,
    }
}

pub fn replay_ip1_json(json: &str) -> Ip1Replay {
    match serde_json::from_str::<Ip1CertificationBoundary>(json) {
        Ok(certificate) => replay_ip1_certification_boundary(&certificate),
        Err(error) => Ip1Replay {
            valid: false,
            mechanical_boundary_complete: false,
            no_certifiably_clearing_step16: false,
            theorem_status: "invalid_json".to_owned(),
            errors: vec![format!("invalid IP-1 JSON: {error}")],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formula_rows_reproduce_bounds_but_candidate_join_is_absent() {
        let certificate = run_ip1_certification_boundary(A5Adjudication::Undecided);
        assert!(!certificate.mechanical_boundary_complete);
        assert!(!certificate.no_certifiably_clearing_step16);
        assert_eq!(certificate.theorem_status, "blocked_by_ip1_falsifier");
        assert!(certificate.falsifiers.f_ip1_uncovered_raw_class);
        assert!(
            !certificate
                .completeness
                .candidate_wise_class_to_verdict_join_proved
        );
        assert!(
            certificate
                .completeness
                .class_rows_are_conditional_formula_rules
        );
        assert_eq!(certificate.classes.len(), 9);
        assert!(
            certificate
                .classes
                .iter()
                .flat_map(|case| &case.ceilings)
                .all(|ceiling| ceiling.strictly_below_bar)
        );
        assert_eq!(
            certificate
                .classes
                .iter()
                .flat_map(|case| &case.ceilings)
                .map(|ceiling| ceiling.conditional_ceiling_nu)
                .max(),
            Some(14)
        );
        assert_eq!(
            certificate
                .classes
                .first()
                .expect("class")
                .ceilings
                .iter()
                .map(|ceiling| ceiling.first_clearing_integer_nu)
                .collect::<Vec<_>>(),
            vec![19, 28, 37]
        );
    }

    #[test]
    fn adopted_a5_cannot_override_triggered_ip1_falsifiers() {
        let adopted = run_ip1_certification_boundary(A5Adjudication::Adopted);
        assert_eq!(adopted.a5.adjudication, A5Adjudication::Adopted);
        assert!(!adopted.a5.theorem_is_conditional_on_a5);
        assert!(!adopted.no_certifiably_clearing_step16);
        assert_eq!(adopted.theorem_status, "blocked_by_ip1_falsifier");

        let rejected = run_ip1_certification_boundary(A5Adjudication::Rejected);
        assert!(rejected.falsifiers.f_ip3_a5_rejected);
        assert!(!rejected.no_certifiably_clearing_step16);
        assert_eq!(rejected.theorem_status, "blocked_by_ip1_falsifier");
    }

    #[test]
    fn vacuous_public_p5_issuance_is_rejected_before_sidecar_adaptation() {
        let certificate = run_ip1_certification_boundary(A5Adjudication::Undecided);
        assert!(certificate.completeness.no_public_required_token_route);
        assert!(!certificate.falsifiers.f_ip2_public_required_token_route);
        assert_eq!(certificate.public_required_token_witnesses.len(), 1);
        let witness = &certificate.public_required_token_witnesses[0];
        assert!(witness.raw_surface_member);
        assert!(witness.shallow_type_checks);
        assert!(witness.guarded_admitted);
        assert!(witness.connectivity_passes);
        assert!(witness.references_active_window);
        assert!(!witness.identified_with_sealed_structure);
        assert!(witness.semantically_minimal);
        assert_eq!(witness.bar_clearing_detachable_subbundles, 0);
        assert!(!witness.token_issued);
        assert_eq!(witness.token_dominant_import, None);
        assert!(witness.token_lift_clauses.is_empty());
        assert!(
            witness
                .token_error
                .as_deref()
                .is_some_and(|error| error.contains("never directly applied"))
        );
        assert!(!witness.sidecar_adapter_succeeded);
        assert!(witness.sidecar_adapter_error.is_some());
        assert!(!witness.record_internality_capability_available);
        assert!(!witness.complete_p5_route_succeeded);
        assert!(witness.survives_frozen_candidate_gates);
        assert!(!witness.falsifies_f_ip2);
        assert_eq!(
            certificate.public_token_audit.len(),
            TRUSTED_TOKEN_TYPES.len()
        );
        for audit in &certificate.public_token_audit {
            assert!(audit.struct_found, "{}", audit.token_type);
            assert!(audit.all_fields_private, "{}", audit.token_type);
            if audit.token_type == "ReplayedP5LiftCapability" {
                assert_eq!(
                    audit.public_associated_constructors,
                    vec!["from_kernel_token"]
                );
                assert!(audit.constructible_from_frozen_public_api);
            } else {
                assert!(
                    audit.public_associated_constructors.is_empty(),
                    "{}",
                    audit.token_type
                );
                assert!(
                    !audit.constructible_from_frozen_public_api,
                    "{}",
                    audit.token_type
                );
            }
        }
        let internality = certificate
            .public_token_audit
            .iter()
            .find(|audit| audit.token_type == "TrustedP5RecordInternalityCapability")
            .expect("record-internality capability is audited independently");
        assert!(internality.struct_found);
        assert!(internality.all_fields_private);
        assert!(internality.public_associated_constructors.is_empty());
        assert!(!internality.constructible_from_frozen_public_api);
    }

    #[test]
    fn replay_rejects_flipped_rule_token_outcome_and_completeness_flag() {
        let certificate = run_ip1_certification_boundary(A5Adjudication::Undecided);
        assert!(replay_ip1_certification_boundary(&certificate).valid);

        let mut rule = certificate.clone();
        rule.classes[0].conditional_rule_kind = ConditionalClassRuleKind::RequiredTokenFailure;
        assert!(!replay_ip1_certification_boundary(&rule).valid);

        let mut token = certificate.clone();
        let hit = token
            .classes
            .iter_mut()
            .find(|case| case.class == TelescopeClass::Hit)
            .expect("Hit row");
        hit.required_token_failures[0].outcome = "ok".to_owned();
        assert!(!replay_ip1_certification_boundary(&token).valid);

        let mut completeness = certificate;
        completeness.completeness.raw_product_partition_complete = false;
        assert!(!replay_ip1_certification_boundary(&completeness).valid);
    }

    #[test]
    fn json_round_trip_and_malformed_json_fail_closed() {
        let json = ip1_json_pretty(A5Adjudication::Adopted);
        let replay = replay_ip1_json(&json);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(!replay.no_certifiably_clearing_step16);

        let malformed = replay_ip1_json("{not-json");
        assert!(!malformed.valid);
        assert!(!malformed.mechanical_boundary_complete);
        assert!(!malformed.no_certifiably_clearing_step16);
    }
}
