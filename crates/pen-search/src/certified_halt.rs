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
    /// False by the certified outcome (B) of the semantic normalization
    /// program: the burned reselection run halts at stage 2 (first
    /// divergence at stage 1), so a completed revised history — and with
    /// it the anchored embedding §6 demands — does not exist under the
    /// strengthened law (docs/semantic_reselection.json).
    pub anchored_provenance_embedding_supplied: bool,
    /// True, earned by the Phase 4 semantic exhaustion: over the whole
    /// admitted Step-16 cone the partition internal | EGP-marginal is
    /// exhaustive with the unclassified part ZERO in every kappa stratum
    /// (docs/step16_semantic_exhaustion.json; the structural automaton's
    /// own honesty flags are unchanged).
    pub shipped_raw_telescope_quotient_complete: bool,
    /// False: no revised fifteen-stage history supplies a semantic
    /// Bar16, and the legacy Bar16 may not stand in for it.
    pub original_global_halt_proven: bool,
    pub outstanding_obligations: Vec<String>,
}

/// One kappa stratum of the frozen Phase 4 exhaustion constants (exact
/// u128 counts as decimal strings; reproduction path in the evidence
/// block re-derives them from source).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExhaustionStratumConstants {
    pub kappa: u16,
    pub raw_total: String,
    pub internal: String,
    pub egp_marginal: String,
    pub invalid_named_bare_univ: String,
    pub unclassified: String,
}

/// Summary of one falsifier's Phase 5a typed disposition, rebuilt from
/// the sealed signature on every certificate build.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FalsifierDispositionSummary {
    pub name: String,
    pub required_token: String,
    pub required_token_outcome: String,
    pub marginal_families: usize,
    pub egp_marginal_nu: Option<u32>,
    pub egp_local_capacity: Option<u32>,
}

/// The Phase 1-5 evidence block of the semantic normalization program
/// (SEMANTIC_NORMALIZATION_PROGRAM.md §6). Everything except the frozen
/// exhaustion constants is re-derived live on every build and therefore
/// participates in the definitional replay.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticNormalizationEvidence {
    pub program: String,
    pub elaborator_hash: String,
    pub token_rules_hash: String,
    pub binding_convention: String,
    pub equality_procedure: String,
    pub signature_digest: String,
    pub closure_digest: String,
    pub fuel_certificate_within_bound: bool,
    pub fuel_total_observed: u32,
    pub orbit_extraction_derivation_hash: String,
    pub semantic_o16_empty_if_assumptions: Option<bool>,
    pub j2_kernel_verified_at_16: bool,
    pub j3_kernel_verified_at_16: bool,
    pub window_locality_kernel_verified_at_16: bool,
    pub internality_derivation_hash: String,
    pub internality_marginal_nu: u32,
    pub internality_guarded_flows: usize,
    pub internality_excluded_flows: usize,
    pub falsifier_dispositions: Vec<FalsifierDispositionSummary>,
    pub exhaustion_strata: Vec<ExhaustionStratumConstants>,
    pub exhaustion_digest: String,
    pub exhaustion_every_stratum_classified: bool,
    pub exhaustion_reproduction: String,
    pub reselection_outcome: crate::semantic_reselection::ReselectionOutcome,
    pub reselection_first_divergence: Option<crate::semantic_reselection::FirstDivergence>,
    pub reselection_digest: String,
    pub reselection_lawful_continuations: [String; 2],
    pub agda_witness_module: String,
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
    pub semantic_normalization: SemanticNormalizationEvidence,
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

/// Frozen Phase 4 exhaustion constants (docs/step16_semantic_exhaustion.json,
/// regenerated by `cargo run -p pen-search --release --example
/// step16_semantic_exhaustion`); a unit test cross-checks them against the
/// committed artifact.
const EXHAUSTION_DIGEST: &str =
    "blake3:5f2ff655260c7c5a7894557aef445d97bd45907e41a522f626c836aaf75bade8";
const EXHAUSTION_STRATA: [(u16, &str, &str, &str, &str, &str); 3] = [
    (
        2,
        "602707957488",
        "134405441711",
        "436447700533",
        "31854815244",
        "0",
    ),
    (
        3,
        "727994066026945536",
        "85462881238554189",
        "587614074893363379",
        "54917109895027968",
        "0",
    ),
    (
        4,
        "1135046124093383916890112",
        "70452049326669225329863",
        "955581751062132991906361",
        "109012323704581699653888",
        "0",
    ),
];

/// The four falsifier witnesses, in the probe's frozen order.
fn falsifier_telescopes() -> Vec<(String, Telescope)> {
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    let pi = |a: Expr, b: Expr| Expr::Pi(Box::new(a), Box::new(b));
    let app = |a: Expr, b: Expr| Expr::App(Box::new(a), Box::new(b));
    vec![
        (
            "hit_no_formation_d1".to_string(),
            Telescope::new(vec![
                ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(1)),
                ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
            ]),
        ),
        (
            "temporal_polymorphic_kappa2".to_string(),
            Telescope::new(vec![
                ClauseRec::new(
                    ClauseRole::Formation,
                    pi(
                        Expr::Next(Box::new(Expr::Var(1))),
                        Expr::Eventually(Box::new(Expr::Var(1))),
                    ),
                ),
                ClauseRec::new(
                    ClauseRole::Formation,
                    pi(
                        Expr::Next(Box::new(Expr::Var(1))),
                        Expr::Eventually(Box::new(Expr::Var(1))),
                    ),
                ),
            ]),
        ),
        (
            "axiomatic_single_l15_kappa3".to_string(),
            Telescope::new(vec![
                ClauseRec::new(ClauseRole::Formation, pi(Expr::Lib(15), Expr::Var(1))),
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(ClauseRole::Introduction, app(Expr::Lib(15), Expr::Var(1))),
            ]),
        ),
        (
            "axiomatic_inheritance_kappa3".to_string(),
            Telescope::new(vec![
                ClauseRec::new(ClauseRole::Formation, pi(Expr::Lib(15), Expr::Var(1))),
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
                ),
                ClauseRec::new(ClauseRole::Introduction, app(Expr::Lib(14), Expr::Var(1))),
            ]),
        ),
    ]
}

fn build_semantic_normalization_evidence() -> SemanticNormalizationEvidence {
    use pen_eval::demand_orbits::kernel_stage_inventories;
    use pen_eval::internality::certify_guarded_step15_internality;
    use pen_eval::typed_families::predecessor_closure;
    use pen_type::elaborate::{
        SealedSignature, elaborator_hash, genesis_fuel_certificate, token_rules_hash,
    };
    use pen_type::equality::KERNEL_EQUALITY_PROCEDURE;
    use pen_type::normalize::KERNEL_BINDING_CONVENTION;

    let signature = SealedSignature::genesis_del_h15();
    let closure = predecessor_closure(&signature)
        .expect("the sealed corpus yields a predecessor closure");
    let orbits = kernel_stage_inventories(&signature, &closure)
        .expect("the sealed timeline yields kernel orbit inventories");
    let internality = certify_guarded_step15_internality(&signature, &closure)
        .expect("the guarded flow certifies");
    let fuel = genesis_fuel_certificate();
    let audit = pen_eval::semantic_provenance::audit_semantic_debt(&orbits.inventories);
    let stage_16 = orbits.stage(16).expect("stage 16 inventory");

    let falsifier_dispositions = falsifier_telescopes()
        .into_iter()
        .map(|(name, telescope)| {
            let disposition = crate::falsifier_disposition::build_semantic_disposition(
                &signature, &closure, &orbits, &telescope,
            );
            FalsifierDispositionSummary {
                name,
                required_token: disposition.required_token.clone(),
                required_token_outcome: disposition
                    .required_token_outcome()
                    .unwrap_or("missing")
                    .to_string(),
                marginal_families: disposition.marginal_families.len(),
                egp_marginal_nu: disposition.egp_marginal_nu,
                egp_local_capacity: disposition.egp_local_capacity,
            }
        })
        .collect();

    let reselection = crate::semantic_reselection::run_semantic_reselection()
        .expect("the burned run publishes");

    SemanticNormalizationEvidence {
        program: "docs/SEMANTIC_NORMALIZATION_PROGRAM.md".to_string(),
        elaborator_hash: elaborator_hash(),
        token_rules_hash: token_rules_hash(),
        binding_convention: KERNEL_BINDING_CONVENTION.to_string(),
        equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_string(),
        signature_digest: signature.digest().to_string(),
        closure_digest: closure.digest.clone(),
        fuel_certificate_within_bound: fuel.all_within_bound,
        fuel_total_observed: fuel
            .per_step
            .iter()
            .map(|(_, fuel)| fuel.total_fuel_observed)
            .sum(),
        orbit_extraction_derivation_hash: orbits.derivation_hash.clone(),
        semantic_o16_empty_if_assumptions: audit.semantic_o16_empty_if_assumptions,
        j2_kernel_verified_at_16: stage_16
            .extraction_completeness_assumption
            .is_kernel_verified(),
        j3_kernel_verified_at_16: stage_16
            .derivability_completeness_assumption
            .is_kernel_verified(),
        window_locality_kernel_verified_at_16: stage_16
            .window_locality_assumption
            .is_kernel_verified(),
        internality_derivation_hash: internality.derivation_hash.clone(),
        internality_marginal_nu: internality.conditional.marginal_nu,
        internality_guarded_flows: internality.guarded_flows.len(),
        internality_excluded_flows: internality.excluded_flows.len(),
        falsifier_dispositions,
        exhaustion_strata: EXHAUSTION_STRATA
            .iter()
            .map(
                |(kappa, raw, internal, egp, invalid, unclassified)| ExhaustionStratumConstants {
                    kappa: *kappa,
                    raw_total: (*raw).to_string(),
                    internal: (*internal).to_string(),
                    egp_marginal: (*egp).to_string(),
                    invalid_named_bare_univ: (*invalid).to_string(),
                    unclassified: (*unclassified).to_string(),
                },
            )
            .collect(),
        exhaustion_digest: EXHAUSTION_DIGEST.to_string(),
        exhaustion_every_stratum_classified: true,
        exhaustion_reproduction:
            "cargo run -p pen-search --release --example step16_semantic_exhaustion -- --out \
             docs/step16_semantic_exhaustion.json"
                .to_string(),
        reselection_outcome: reselection.outcome.clone(),
        reselection_first_divergence: reselection.first_divergence.clone(),
        reselection_digest: reselection.digest.clone(),
        reselection_lawful_continuations: reselection.lawful_continuations.clone(),
        agda_witness_module: "ProvenanceWitness16".to_string(),
    }
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

    let semantic_normalization = build_semantic_normalization_evidence();
    let exhaustion_classified = semantic_normalization.exhaustion_every_stratum_classified
        && semantic_normalization
            .exhaustion_strata
            .iter()
            .all(|stratum| stratum.unclassified == "0");
    let semantic_boundary = SemanticCompletenessBoundary {
        certified_calculus_class_partition_complete: true,
        finite_bound_cases_complete: finite_bounds.len() == 3
            && finite_bounds.iter().all(|bound| bound.all_classes_enumerated),
        // Outcome (B): the burned reselection halts before a completed
        // revised history exists, so the anchored embedding cannot be
        // supplied — the flag stays false BY DATA, not by omission.
        anchored_provenance_embedding_supplied: false,
        // Earned by the semantic exhaustion: internal | EGP-marginal
        // covers the whole admitted cone with zero unclassified.
        shipped_raw_telescope_quotient_complete: exhaustion_classified,
        original_global_halt_proven: false,
        outstanding_obligations: vec![
            "outcome (B) certified: the semantic reselection halts at stage 2 with a stage-1 \
             score divergence (docs/semantic_reselection.json); a completed revised history — \
             and with it the anchored embedding — does not exist under the strengthened law"
                .to_owned(),
            "the original global halt remains unproven: no revised fifteen-stage history \
             supplies a semantic Bar16, and the legacy 354333/39040 may not stand in for it"
                .to_owned(),
            "declared-choice fork (Two-Law bridge, book ledger): (i) treat the shipped \
             structural audit as a constitutive scoring bridge with a separately justified \
             semantic halt gate, or (ii) adopt the semantic audit and re-derive its history"
                .to_owned(),
        ],
    };

    let mut certificate = CertifiedHaltCertificate {
        schema_version: 3,
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
        semantic_normalization,
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
        // CONSCIOUS decision-gate update (SEMANTIC_NORMALIZATION_PROGRAM
        // §5b/§6): the quotient flag flipped TRUE because the Phase 4
        // semantic exhaustion proved internal | EGP-marginal covers the
        // whole admitted Step-16 cone with zero unclassified in every
        // stratum. The anchored-embedding and global-halt flags stay
        // FALSE by the certified outcome (B): the burned reselection
        // halts at stage 2, so no completed revised history — and no
        // semantic Bar16 — exists; the legacy bar may not stand in.
        let certificate = run_certified_halt_check();
        assert!(
            !certificate
                .semantic_boundary
                .anchored_provenance_embedding_supplied
        );
        assert!(
            certificate
                .semantic_boundary
                .shipped_raw_telescope_quotient_complete
        );
        assert!(!certificate.semantic_boundary.original_global_halt_proven);
        assert!(!certificate
            .semantic_boundary
            .outstanding_obligations
            .is_empty());
        // The evidence block carries the outcome as data.
        let evidence = &certificate.semantic_normalization;
        assert_eq!(evidence.semantic_o16_empty_if_assumptions, Some(true));
        assert!(evidence.j2_kernel_verified_at_16);
        assert!(evidence.j3_kernel_verified_at_16);
        assert!(evidence.window_locality_kernel_verified_at_16);
        assert!(evidence.fuel_certificate_within_bound);
        assert_eq!(evidence.internality_marginal_nu, 0);
        assert!(matches!(
            evidence.reselection_outcome,
            crate::semantic_reselection::ReselectionOutcome::HaltedNoClearingCandidate {
                stage: 2,
                ..
            }
        ));
        let divergence = evidence
            .reselection_first_divergence
            .as_ref()
            .expect("first divergence");
        assert_eq!((divergence.stage, divergence.field.as_str()), (1, "score"));
        // The four falsifier dispositions ride along, none obtaining its
        // required token.
        assert_eq!(evidence.falsifier_dispositions.len(), 4);
        for disposition in &evidence.falsifier_dispositions {
            assert_ne!(disposition.required_token_outcome, "ok");
        }
    }

    /// The frozen exhaustion constants embedded in the certificate must
    /// match the committed artifact byte for byte.
    #[test]
    fn exhaustion_constants_match_the_committed_artifact() {
        let artifact_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/step16_semantic_exhaustion.json"
        );
        let text = std::fs::read_to_string(artifact_path).expect("committed exhaustion artifact");
        let artifact: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
        assert_eq!(
            artifact["digest"].as_str().expect("digest"),
            EXHAUSTION_DIGEST
        );
        let strata = artifact["strata"].as_array().expect("strata");
        assert_eq!(strata.len(), EXHAUSTION_STRATA.len());
        for (stratum, (kappa, raw, internal, egp, invalid, unclassified)) in
            strata.iter().zip(EXHAUSTION_STRATA.iter())
        {
            assert_eq!(stratum["kappa"].as_u64(), Some(u64::from(*kappa)));
            assert_eq!(stratum["raw_total"].as_str(), Some(*raw));
            assert_eq!(stratum["internal"].as_str(), Some(*internal));
            assert_eq!(stratum["egp_marginal"].as_str(), Some(*egp));
            assert_eq!(
                stratum["invalid_named_bare_univ"].as_str(),
                Some(*invalid)
            );
            assert_eq!(stratum["unclassified"].as_str(), Some(*unclassified));
            assert_eq!(stratum["sums_match"].as_bool(), Some(true));
            assert_eq!(stratum["unclassified_is_zero"].as_bool(), Some(true));
        }
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
