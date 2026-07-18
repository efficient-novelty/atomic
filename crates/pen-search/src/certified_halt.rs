//! Combined Step-16 certificate: shipped SAT, certified-calculus UNSAT.
//!
//! This module keeps two propositions deliberately separate.
//!
//! * The shipped structural evaluator is SAT.  Concrete raw-surface
//!   witnesses are replayed by [`crate::step16_automaton`].
//! * The proof-carrying, support-local calculus in
//!   [`pen_eval::certified_novelty`] is UNSAT on the frozen Step-16 surface.
//!
//! The second proposition becomes a proof of the mathematical Genesis halt
//! only after the semantic normalization/isomorphism described by
//! `agda/CertifiedHalt.agda` is supplied.  That obligation is intentionally a
//! machine-readable false flag here; generating this certificate cannot turn
//! a conditional theorem into an unconditional one.

use crate::step16_automaton::{
    replay_step16_certificate, run_step16_automaton, CertificateReplay as ShippedReplay,
    Step16AutomatonCertificate, Step16Decision,
};
use pen_core::hash::blake3_hex;
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::compute_bar;
use pen_eval::certified_novelty::{
    derive_linear_bound, CertifiedSurfaceCaps, ClassCeilings, LinearBoundTheorem,
};
use pen_eval::halting::{genesis_bar_16, genesis_history};
use pen_eval::p5_record::{ImportDag, P5ImportAudit};
use serde::{Deserialize, Serialize};

pub const CERTIFIED_HALT_DATE: &str = "2026-07-18";
const BAR_NUMERATOR: u64 = 354_333;
const BAR_DENOMINATOR: u64 = 39_040;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AbstractClassCase {
    pub class: String,
    pub ceiling_nu: u32,
    pub cross_product_left: u64,
    pub cross_product_right: u64,
    pub strictly_below_bar: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KappaBoundCertificate {
    pub kappa: u16,
    pub class_cases: Vec<AbstractClassCase>,
    pub all_classes_enumerated: bool,
    pub maximum_nu: u32,
    pub maximizing_classes: Vec<String>,
    pub maximum_rho: String,
    pub first_clearing_integer_nu: u32,
    pub integer_margin_to_clear: u32,
    pub maximum_cross_product_left: u64,
    pub maximum_cross_product_right: u64,
    pub maximum_strictly_below_bar: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticCompletenessBoundary {
    /// Exact for the Rust calculus defined by the frozen class ceilings.
    pub certified_calculus_class_partition_complete: bool,
    /// Exact arithmetic enumeration of every class at kappa 2, 3, and 4.
    pub finite_bound_cases_complete: bool,
    /// Still false: the raw MBTT AST does not construct the typed marginal
    /// families, valid family/tag anchors, and injective classifier required
    /// by the extraction-guarded AtMost theorem.
    pub anchored_provenance_embedding_supplied: bool,
    /// Still false: the expression-signature automaton is not an exhaustive
    /// telescope-level quotient of every shipped gate.
    pub shipped_raw_telescope_quotient_complete: bool,
    /// Consequently false even though the corrected calculus is UNSAT.
    pub original_global_halt_proven: bool,
    pub outstanding_obligations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistoryRow {
    pub step: u32,
    pub nu: u32,
    pub kappa: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GenesisConservativityCertificate {
    pub structural_evaluator_left_unchanged: bool,
    pub history_identical_before_and_after_audit: bool,
    pub accepted_steps: usize,
    pub history: Vec<HistoryRow>,
    pub sum_nu: u32,
    pub sum_kappa: u32,
    pub replayed_bar_16: String,
    pub frozen_bar_16: String,
    pub bar_matches: bool,
    pub step13_unique_dominant_import: Option<u32>,
    pub step14_unique_dominant_import: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CertifiedHaltCertificate {
    pub schema_version: u32,
    pub date: String,
    pub theorem_scope: String,
    pub bar_numerator: u64,
    pub bar_denominator: u64,
    pub bar_16: String,
    pub shipped: Step16AutomatonCertificate,
    pub shipped_replay: ShippedReplay,
    pub shipped_sat: bool,
    pub certified_linear_theorem: LinearBoundTheorem,
    pub finite_bounds: Vec<KappaBoundCertificate>,
    pub corrected_calculus_unsat: bool,
    pub semantic_boundary: SemanticCompletenessBoundary,
    pub genesis_conservativity: GenesisConservativityCertificate,
    pub digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CertifiedHaltReplay {
    pub valid: bool,
    pub shipped_sat: bool,
    pub corrected_calculus_unsat: bool,
    pub original_global_halt_proven: bool,
    pub errors: Vec<String>,
}

fn class_values(ceilings: &ClassCeilings) -> [(&'static str, u32); 9] {
    [
        ("foundation", ceilings.foundation),
        ("former", ceilings.former),
        ("hit", ceilings.hit),
        ("suspension", ceilings.suspension),
        ("map", ceilings.map),
        ("modal", ceilings.modal),
        ("axiomatic", ceilings.axiomatic),
        ("synthesis", ceilings.synthesis),
        ("unknown", ceilings.unknown),
    ]
}

fn cross_products(nu: u32, kappa: u16) -> (u64, u64) {
    let left = BAR_DENOMINATOR
        .checked_mul(u64::from(nu))
        .expect("Step-16 left cross product fits u64");
    let right = BAR_NUMERATOR
        .checked_mul(u64::from(kappa))
        .expect("Step-16 right cross product fits u64");
    (left, right)
}

fn first_clearing_integer(kappa: u16) -> u32 {
    let numerator = BAR_NUMERATOR
        .checked_mul(u64::from(kappa))
        .expect("Step-16 clearing numerator fits u64");
    let rounded = numerator
        .checked_add(BAR_DENOMINATOR - 1)
        .expect("Step-16 ceiling addition fits u64")
        / BAR_DENOMINATOR;
    u32::try_from(rounded).expect("Step-16 clearing threshold fits u32")
}

fn bound_certificate(ceilings: &ClassCeilings) -> KappaBoundCertificate {
    let class_cases = class_values(ceilings)
        .into_iter()
        .map(|(class, ceiling_nu)| {
            let (left, right) = cross_products(ceiling_nu, ceilings.kappa);
            AbstractClassCase {
                class: class.to_owned(),
                ceiling_nu,
                cross_product_left: left,
                cross_product_right: right,
                strictly_below_bar: left < right,
            }
        })
        .collect::<Vec<_>>();
    let maximum_nu = class_cases
        .iter()
        .map(|case| case.ceiling_nu)
        .max()
        .unwrap_or(0);
    let maximizing_classes = class_cases
        .iter()
        .filter(|case| case.ceiling_nu == maximum_nu)
        .map(|case| case.class.clone())
        .collect();
    let (maximum_cross_product_left, maximum_cross_product_right) =
        cross_products(maximum_nu, ceilings.kappa);
    let first_clearing_integer_nu = first_clearing_integer(ceilings.kappa);

    KappaBoundCertificate {
        kappa: ceilings.kappa,
        all_classes_enumerated: class_cases.len() == 9,
        class_cases,
        maximum_nu,
        maximizing_classes,
        maximum_rho: Rational::new(i64::from(maximum_nu), i64::from(ceilings.kappa)).to_string(),
        first_clearing_integer_nu,
        integer_margin_to_clear: first_clearing_integer_nu
            .checked_sub(maximum_nu)
            .expect("certified maximum must lie below the clearing threshold"),
        maximum_cross_product_left,
        maximum_cross_product_right,
        maximum_strictly_below_bar: maximum_cross_product_left < maximum_cross_product_right,
    }
}

fn history_rows() -> Vec<HistoryRow> {
    let (_, _, records) = genesis_history();
    records
        .into_iter()
        .map(|record| HistoryRow {
            step: record.step_index,
            nu: record.nu,
            kappa: record.kappa,
        })
        .collect()
}

fn conservativity(before: &[HistoryRow], after: &[HistoryRow]) -> GenesisConservativityCertificate {
    let (_, _, records) = genesis_history();
    let replayed_bar = compute_bar(2, 16, &records).bar;
    let frozen_bar = genesis_bar_16();
    let graph = ImportDag::genesis_prefix(15);
    let step13 = P5ImportAudit::check(&Telescope::reference(13), &graph);
    let step14 = P5ImportAudit::check(&Telescope::reference(14), &graph);

    GenesisConservativityCertificate {
        // The certified evaluator is a sidecar module; `structural_nu` and
        // the fifteen reference telescopes are not rewritten.
        structural_evaluator_left_unchanged: true,
        history_identical_before_and_after_audit: before == after,
        accepted_steps: after.len(),
        history: after.to_vec(),
        sum_nu: after.iter().map(|row| row.nu).sum(),
        sum_kappa: after.iter().map(|row| row.kappa).sum(),
        replayed_bar_16: replayed_bar.to_string(),
        frozen_bar_16: frozen_bar.to_string(),
        bar_matches: replayed_bar == frozen_bar,
        step13_unique_dominant_import: step13.unique_dominant_import,
        step14_unique_dominant_import: step14.unique_dominant_import,
    }
}

fn certificate_digest(certificate: &CertifiedHaltCertificate) -> String {
    let mut payload = certificate.clone();
    payload.digest.clear();
    let bytes = serde_json::to_vec(&payload).expect("combined certificate must serialize");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn build_certificate() -> CertifiedHaltCertificate {
    let history_before = history_rows();
    let caps = CertifiedSurfaceCaps::genesis_step16();
    let certified_linear_theorem =
        derive_linear_bound(&caps).expect("the frozen Step-16 caps must be valid");
    let finite_bounds = certified_linear_theorem
        .per_kappa
        .iter()
        .map(bound_certificate)
        .collect::<Vec<_>>();
    let corrected_calculus_unsat = certified_linear_theorem.every_ceiling_is_bounded
        && finite_bounds.len() == usize::from(caps.max_kappa - caps.min_kappa + 1)
        && finite_bounds
            .iter()
            .all(|bound| bound.all_classes_enumerated && bound.maximum_strictly_below_bar);

    let shipped = run_step16_automaton();
    let shipped_replay = replay_step16_certificate(&shipped);
    let shipped_sat = shipped_replay.valid && shipped.decision == Step16Decision::Sat;
    let history_after = history_rows();

    let semantic_boundary = SemanticCompletenessBoundary {
        certified_calculus_class_partition_complete: true,
        finite_bound_cases_complete: finite_bounds.len() == 3
            && finite_bounds.iter().all(|bound| bound.all_classes_enumerated),
        anchored_provenance_embedding_supplied: false,
        shipped_raw_telescope_quotient_complete: shipped.completeness.exhaustive_telescope_quotient,
        original_global_halt_proven: false,
        outstanding_obligations: vec![
            "extract typed natural schema families and individual demand orbits modulo normalization, weakening, and univalent equality".to_owned(),
            "prove window locality/expiration and the J2/J3 disposition of every active demand orbit".to_owned(),
            "construct a valid anchored injection for each counted family, or weakening/erasure inverse evidence for an internal flow".to_owned(),
            "re-audit all fifteen candidate cones, selective orders, winners, and scores under the strengthened law".to_owned(),
            "prove that every Step-16 candidate is either certifiably internal or has a complete EGP-certified opaque normal form".to_owned(),
        ],
    };

    let mut certificate = CertifiedHaltCertificate {
        schema_version: 2,
        date: CERTIFIED_HALT_DATE.to_owned(),
        theorem_scope: "unconditional SAT for the shipped structural evaluator; UNSAT for the proof-carrying support-local calculus; conditional, not yet global, halt theorem for the intended semantics".to_owned(),
        bar_numerator: BAR_NUMERATOR,
        bar_denominator: BAR_DENOMINATOR,
        bar_16: genesis_bar_16().to_string(),
        shipped,
        shipped_replay,
        shipped_sat,
        certified_linear_theorem,
        finite_bounds,
        corrected_calculus_unsat,
        semantic_boundary,
        genesis_conservativity: conservativity(&history_before, &history_after),
        digest: String::new(),
    };
    certificate.digest = certificate_digest(&certificate);
    certificate
}

/// Build the complete dual-result Step-16 certificate.
pub fn run_certified_halt_check() -> CertifiedHaltCertificate {
    build_certificate()
}

/// Serialize a freshly replayed combined certificate for archival or
/// independent transport.
pub fn certified_halt_json_pretty() -> String {
    serde_json::to_string_pretty(&run_certified_halt_check())
        .expect("combined certificate must serialize")
}

/// Replay the entire payload from source definitions and reject any mutation.
pub fn replay_certified_halt_certificate(
    certificate: &CertifiedHaltCertificate,
) -> CertifiedHaltReplay {
    let mut errors = Vec::new();
    if certificate_digest(certificate) != certificate.digest {
        errors.push("combined certificate digest mismatch".to_owned());
    }
    if *certificate != build_certificate() {
        errors.push("combined certificate differs from a fresh definition replay".to_owned());
    }
    if certificate.semantic_boundary.original_global_halt_proven {
        errors.push("global halt flag cannot be true while semantic obligations remain".to_owned());
    }
    CertifiedHaltReplay {
        valid: errors.is_empty(),
        shipped_sat: certificate.shipped_sat,
        corrected_calculus_unsat: certificate.corrected_calculus_unsat,
        original_global_halt_proven: certificate.semantic_boundary.original_global_halt_proven,
        errors,
    }
}

/// Parse and replay a serialized certificate. Malformed input is an invalid
/// certificate and can never be interpreted as an UNSAT or halt result.
pub fn replay_certified_halt_certificate_json(json: &str) -> CertifiedHaltReplay {
    match serde_json::from_str::<CertifiedHaltCertificate>(json) {
        Ok(certificate) => replay_certified_halt_certificate(&certificate),
        Err(error) => CertifiedHaltReplay {
            valid: false,
            shipped_sat: false,
            corrected_calculus_unsat: false,
            original_global_halt_proven: false,
            errors: vec![format!("invalid combined certificate JSON: {error}")],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrected_calculus_is_exactly_below_the_step16_bar() {
        let certificate = run_certified_halt_check();
        assert!(certificate.shipped_sat);
        assert!(certificate.corrected_calculus_unsat);
        assert_eq!(certificate.certified_linear_theorem.coefficient, 4);
        assert_eq!(
            certificate
                .finite_bounds
                .iter()
                .map(|bound| (
                    bound.kappa,
                    bound.maximum_nu,
                    bound.first_clearing_integer_nu,
                    bound.integer_margin_to_clear,
                ))
                .collect::<Vec<_>>(),
            vec![(2, 8, 19, 11), (3, 11, 28, 17), (4, 14, 37, 23)]
        );
        assert!(certificate.finite_bounds.iter().all(|bound| {
            bound.class_cases.len() == 9
                && bound.class_cases.iter().all(|case| case.strictly_below_bar)
        }));
    }

    #[test]
    fn result_does_not_overclaim_the_original_halt() {
        let certificate = run_certified_halt_check();
        assert!(
            !certificate
                .semantic_boundary
                .anchored_provenance_embedding_supplied
        );
        assert!(
            !certificate
                .semantic_boundary
                .shipped_raw_telescope_quotient_complete
        );
        assert!(!certificate.semantic_boundary.original_global_halt_proven);
        assert!(!certificate
            .semantic_boundary
            .outstanding_obligations
            .is_empty());
    }

    #[test]
    fn genesis_history_and_p5_boundaries_are_conservative() {
        let certificate = run_certified_halt_check();
        let replay = &certificate.genesis_conservativity;
        assert!(replay.structural_evaluator_left_unchanged);
        assert!(replay.history_identical_before_and_after_audit);
        assert_eq!(replay.accepted_steps, 15);
        assert_eq!(replay.sum_nu, 359);
        assert_eq!(replay.sum_kappa, 64);
        assert!(replay.bar_matches);
        assert_eq!(replay.step13_unique_dominant_import, Some(12));
        assert_eq!(replay.step14_unique_dominant_import, Some(13));
    }

    #[test]
    fn combined_certificate_replay_rejects_mutation() {
        let certificate = run_certified_halt_check();
        let replay = replay_certified_halt_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);

        let mut mutated = certificate;
        mutated.finite_bounds[0].maximum_nu += 1;
        let replay = replay_certified_halt_certificate(&mutated);
        assert!(!replay.valid);
        assert!(!replay.errors.is_empty());
    }

    #[test]
    fn combined_json_round_trips_and_malformed_input_never_claims_unsat() {
        let json = certified_halt_json_pretty();
        let replay = replay_certified_halt_certificate_json(&json);
        assert!(replay.valid, "{:?}", replay.errors);
        assert!(replay.shipped_sat);
        assert!(replay.corrected_calculus_unsat);
        assert!(!replay.original_global_halt_proven);

        let malformed = replay_certified_halt_certificate_json("{not-json");
        assert!(!malformed.valid);
        assert!(!malformed.shipped_sat);
        assert!(!malformed.corrected_calculus_unsat);
        assert!(!malformed.original_global_halt_proven);
    }
}
