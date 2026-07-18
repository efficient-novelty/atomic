//! TDC-1: typed audit of the registered formed `PathCon(4)` package.
//!
//! The certificate built here is deliberately bar-independent.  It checks
//! the exact two-clause surface, enumerates the finite L1 index basis, asks
//! the typed kernel for a realization of every indexed family, checks
//! predecessor weakening only through evidence the kernel actually exposes,
//! and records EGP capacity without turning an untyped site into credit.
//! A separate comparison object maps a frozen certificate to Z1--Z4.

use crate::counting_lemmas::{L1SchemaKey, check_l1_finite_basis};
use crate::demand_orbits::kernel_stage_inventories;
use crate::semantic_provenance::{LocalRole, OrbitResolution, blind_local_role_coefficient};
use crate::typed_families::{
    CandidateExtractionOutcome, InstanceKind, extract_candidate_families, predecessor_closure,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{SealedSignature, TokenError, issue_typed_eliminator_token};
use pen_type::tdc1::{
    FormedPathTyping, PathSchemaKey, PathSchemaObligation, PathSchemaRealizationGap,
    elaborate_formed_path, elaborate_tdc1_package, enumerate_path_schema_obligations,
    realize_path_schema,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const TDC1_CERTIFICATE_SCHEMA: &str = "tdc1-typed-d4-certificate-v1";
pub const TDC1_COMPARISON_SCHEMA: &str = "tdc1-zone-comparison-v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SurfaceTypingSummary {
    pub typing: FormedPathTyping,
    pub fuel_within_bound: bool,
    pub formation_family_count: usize,
    pub shallow_surface_family_count: usize,
    pub shallow_surface_marginal_count: usize,
    pub extraction_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AggregateEliminatorAttempt {
    pub outcome: String,
    pub derivation_hash: Option<String>,
    pub beta_clause_count: usize,
    pub kan_dimensions: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticSiteDisposition {
    Weakening,
    Marginal,
    Undefined,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BasisSiteAudit {
    pub site: PathSchemaObligation,
    /// Equality of canonical INDEX presentations is a safe positive test,
    /// but its absence is not a no-weakening theorem without typed cubical
    /// realizers and univalent equality for them.
    pub exact_predecessor_index_matches: Vec<String>,
    pub realization: Result<(), PathSchemaRealizationGap>,
    pub semantic_disposition: SemanticSiteDisposition,
    pub provenance_anchor: Option<AnchorSlot>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AnchorSlot {
    pub clause: u16,
    pub role: LocalRole,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct L1BasisAudit {
    pub dimension: u32,
    pub beta_count: usize,
    pub diagonal_count: usize,
    pub off_diagonal_count: usize,
    pub raw_basis_count: usize,
    pub raw_ceiling_one_plus_d_squared: usize,
    pub exact_partition: bool,
    pub sites: Vec<BasisSiteAudit>,
    pub typed_realization_complete: bool,
    pub semantic_weakening_complete: bool,
    pub certified_path_nu: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProvenanceCapacityAudit {
    pub candidate_stage: u32,
    pub kappa: u16,
    pub local_roles_per_clause: u32,
    pub local_capacity: u32,
    pub live_demand_orbits: usize,
    pub live_demand_outputs: usize,
    pub orbit_derivation_hash: String,
    pub assigned_anchors: Vec<AnchorSlot>,
    pub anchors_injective: bool,
    pub all_marginal_sites_anchored: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistoricalHitRegression {
    pub step: u32,
    pub label: String,
    pub dimension: u32,
    pub recorded_total_nu: u32,
    pub recorded_path_nu: u32,
    pub surface_elaborated: bool,
    pub aggregate_eliminator: AggregateEliminatorAttempt,
    pub raw_basis_count: usize,
    pub certified_path_nu: Option<u32>,
    pub certified_total_nu: Option<u32>,
    pub rederived_recorded_path_score: bool,
    pub rederived_recorded_score: bool,
    pub blockers: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Tdc1Novelty {
    Certified { nu: u32 },
    Undefined { missing_obligations: Vec<String> },
}

impl Tdc1Novelty {
    pub fn certified_nu(&self) -> Option<u32> {
        match self {
            Self::Certified { nu } => Some(*nu),
            Self::Undefined { .. } => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MutationFalsifiers {
    pub dimension_mutation_rejected: bool,
    pub duplicate_basis_site_rejected: bool,
    pub missing_basis_site_rejected: bool,
    pub anchor_collision_rejected: bool,
    pub over_ceiling_basis_rejected: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tdc1Certificate {
    pub schema: String,
    pub bar_independent: bool,
    pub candidate: SurfaceTypingSummary,
    pub aggregate_eliminator: AggregateEliminatorAttempt,
    pub basis: L1BasisAudit,
    pub provenance: ProvenanceCapacityAudit,
    pub conditional_raw_ceiling_including_formation: usize,
    pub novelty: Tdc1Novelty,
    pub historical_regression: Vec<HistoricalHitRegression>,
    pub historical_regression_passed: bool,
    pub falsifiers: MutationFalsifiers,
    pub next_scope_increment: Vec<String>,
    pub certificate_digest: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Tdc1Zone {
    Z1,
    Z2,
    Z3,
    Z4,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tdc1Comparison {
    pub schema: String,
    pub certificate_digest: String,
    pub kappa: u16,
    pub certified_nu: Option<u32>,
    pub rho: Option<String>,
    pub bar: String,
    pub clearing_threshold: u32,
    pub clears_bar: Option<bool>,
    pub registered_zone: Tdc1Zone,
    pub historical_regression_passed: bool,
    pub step16_verdict_valid: bool,
    pub falsifier_f_t1: bool,
    pub falsifier_f_t2: bool,
    pub falsifier_f_t3: bool,
    pub meaning: String,
    pub comparison_digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Tdc1Error {
    #[error("typed predecessor closure failed: {0}")]
    Closure(String),
    #[error("TDC-1 surface failed: {0}")]
    Surface(String),
    #[error("candidate family extraction failed")]
    Extraction,
    #[error("demand-orbit extraction failed: {0}")]
    Orbits(String),
    #[error("L1 finite basis failed: {0}")]
    Basis(String),
    #[error("presented certificate does not replay byte-for-byte")]
    ReplayMismatch,
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(TDC1_CERTIFICATE_SCHEMA, domain, payload))
        .expect("TDC-1 proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

pub fn registered_d4_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Lib(15))),
        ),
        ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(4)),
    ])
}

fn token_attempt(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
) -> AggregateEliminatorAttempt {
    match issue_typed_eliminator_token(signature, telescope, visible_library) {
        Ok(token) => AggregateEliminatorAttempt {
            outcome: "issued".to_owned(),
            derivation_hash: Some(token.derivation_hash().to_owned()),
            beta_clause_count: token.basis().beta_clauses.len(),
            kan_dimensions: token.basis().kan_dimensions.clone(),
        },
        Err(error) => AggregateEliminatorAttempt {
            outcome: token_error_name(&error).to_owned(),
            derivation_hash: None,
            beta_clause_count: 0,
            kan_dimensions: Vec::new(),
        },
    }
}

fn token_error_name(error: &TokenError) -> &'static str {
    match error {
        TokenError::ElaborationFailed { .. } => "elaboration_failed",
        TokenError::NoFormationClause => "no_formation_clause",
        TokenError::NoOrientedBasis => "no_oriented_basis",
        TokenError::NoMotiveTypedEliminator { .. } => "no_motive_typed_eliminator",
        TokenError::NotANaturalitySquare { .. } => "not_a_naturality_square",
        TokenError::NaturalitySquareMismatch { .. } => "naturality_square_mismatch",
        TokenError::NoDirectImports => "no_direct_imports",
        TokenError::NoDominantImport { .. } => "no_dominant_import",
        TokenError::LiftNotTypedAgainstExportedFormation { .. } => {
            "lift_not_typed_against_exported_formation"
        }
    }
}

fn formation_family_count(extraction: &crate::typed_families::CandidateFamilyExtraction) -> usize {
    extraction
        .families
        .iter()
        .filter(|family| {
            family.instances.iter().any(|instance| {
                instance.clause_index == 0 && matches!(instance.kind, InstanceKind::Generator)
            })
        })
        .count()
}

fn historical_hit_specs() -> [(u32, &'static str, u32, u32, u32); 4] {
    [
        (5, "S1", 1, 7, 2),
        (6, "Trunc", 1, 8, 2),
        (7, "S2", 2, 10, 5),
        (8, "S3", 3, 18, 10),
    ]
}

fn historical_index_presentations() -> Result<Vec<(String, PathSchemaObligation)>, Tdc1Error> {
    let mut result = Vec::new();
    for (step, label, _, _, _) in historical_hit_specs() {
        let signature = SealedSignature::from_telescopes(
            (1..step)
                .map(|index| (index, Telescope::reference(index)))
                .collect(),
        );
        let telescope = Telescope::reference(step);
        let (typing, _) = elaborate_formed_path(&signature, &telescope, step - 1)
            .map_err(|error| Tdc1Error::Surface(error.to_string()))?;
        for site in enumerate_path_schema_obligations(&typing) {
            result.push((format!("step-{step}-{label}"), site));
        }
    }
    Ok(result)
}

fn site_inventory_valid(
    expected: &[PathSchemaObligation],
    candidate: &[PathSchemaObligation],
) -> bool {
    expected == candidate
        && candidate
            .iter()
            .map(|site| site.normal_form_digest.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            == candidate.len()
}

fn anchors_injective(anchors: &[AnchorSlot]) -> bool {
    anchors.iter().collect::<BTreeSet<_>>().len() == anchors.len()
}

fn historical_regression() -> Result<Vec<HistoricalHitRegression>, Tdc1Error> {
    let mut records = Vec::new();
    for (step, label, dimension, recorded_total_nu, recorded_path_nu) in historical_hit_specs() {
        let signature = SealedSignature::from_telescopes(
            (1..step)
                .map(|index| (index, Telescope::reference(index)))
                .collect(),
        );
        let telescope = Telescope::reference(step);
        let (typing, _) = elaborate_formed_path(&signature, &telescope, step - 1)
            .map_err(|error| Tdc1Error::Surface(error.to_string()))?;
        let obligations = enumerate_path_schema_obligations(&typing);
        let gaps: Vec<_> = obligations
            .iter()
            .filter_map(|site| realize_path_schema(site).err())
            .collect();
        let aggregate_eliminator = token_attempt(&signature, &telescope, step - 1);
        let certified_path_nu = gaps
            .is_empty()
            .then_some(u32::try_from(obligations.len()).expect("bounded L1 count"));
        // This minimal fragment has no typed reconstruction of the HIT's
        // formation/introduction/chaining channels.  Never fill the total
        // from the expected fixture: that would make the regression circular.
        let certified_total_nu = None;
        let path_rederived = certified_path_nu == Some(recorded_path_nu);
        let rederived = certified_total_nu == Some(recorded_total_nu);
        let mut blockers = Vec::new();
        if !gaps.is_empty() {
            blockers.push(format!(
                "{} of {} L1 sites lack typed cubical realization tokens",
                gaps.len(),
                obligations.len()
            ));
        }
        if aggregate_eliminator.outcome != "issued" {
            blockers.push(format!(
                "aggregate eliminator token: {}",
                aggregate_eliminator.outcome
            ));
        } else {
            blockers.push(
                "aggregate eliminator token has no per-site schema/basis isomorphism".to_owned(),
            );
        }
        blockers.push(
            "full historical non-path channels are not reconstructed by this minimal fragment"
                .to_owned(),
        );
        records.push(HistoricalHitRegression {
            step,
            label: label.to_owned(),
            dimension,
            recorded_total_nu,
            recorded_path_nu,
            surface_elaborated: true,
            aggregate_eliminator,
            raw_basis_count: obligations.len(),
            certified_path_nu,
            certified_total_nu,
            rederived_recorded_path_score: path_rederived,
            rederived_recorded_score: rederived,
            blockers,
        });
    }
    Ok(records)
}

pub fn build_tdc1_certificate() -> Result<Tdc1Certificate, Tdc1Error> {
    let signature = SealedSignature::genesis_del_h15();
    let closure =
        predecessor_closure(&signature).map_err(|error| Tdc1Error::Closure(error.to_string()))?;
    let candidate = registered_d4_candidate();
    let (typing, elaboration) = elaborate_tdc1_package(&signature, &candidate, 15)
        .map_err(|error| Tdc1Error::Surface(error.to_string()))?;
    let extraction = match extract_candidate_families(&signature, &closure, &candidate, 15) {
        CandidateExtractionOutcome::Extracted(extraction) => extraction,
        CandidateExtractionOutcome::KernelInvalid { .. } => return Err(Tdc1Error::Extraction),
    };
    let formation_family_count = formation_family_count(&extraction);
    let surface = SurfaceTypingSummary {
        typing: typing.clone(),
        fuel_within_bound: elaboration.fuel.within_bound,
        formation_family_count,
        shallow_surface_family_count: extraction.families.len(),
        shallow_surface_marginal_count: extraction.marginal_family_count,
        extraction_derivation_hash: extraction.derivation_hash.clone(),
    };

    let finite_basis =
        check_l1_finite_basis(4).map_err(|error| Tdc1Error::Basis(error.to_string()))?;
    let obligations = enumerate_path_schema_obligations(&typing);
    let predecessor_sites = historical_index_presentations()?;
    let sites = obligations
        .iter()
        .cloned()
        .map(|site| {
            let exact_predecessor_index_matches = predecessor_sites
                .iter()
                .filter(|(_, predecessor)| predecessor.normal_form == site.normal_form)
                .map(|(owner, _)| owner.clone())
                .collect();
            BasisSiteAudit {
                realization: realize_path_schema(&site),
                site,
                exact_predecessor_index_matches,
                semantic_disposition: SemanticSiteDisposition::Undefined,
                provenance_anchor: None,
            }
        })
        .collect::<Vec<_>>();
    let basis = L1BasisAudit {
        dimension: 4,
        beta_count: finite_basis.counts().beta,
        diagonal_count: finite_basis.counts().diagonal_monodromies,
        off_diagonal_count: finite_basis.counts().off_diagonal_variations,
        raw_basis_count: finite_basis.counts().total,
        raw_ceiling_one_plus_d_squared: 17,
        exact_partition: finite_basis.partition_is_exact(),
        typed_realization_complete: sites.iter().all(|site| site.realization.is_ok()),
        semantic_weakening_complete: sites
            .iter()
            .all(|site| site.semantic_disposition != SemanticSiteDisposition::Undefined),
        certified_path_nu: None,
        sites,
    };

    let orbits = kernel_stage_inventories(&signature, &closure)
        .map_err(|error| Tdc1Error::Orbits(error.to_string()))?;
    let stage16 = orbits
        .stage(16)
        .ok_or_else(|| Tdc1Error::Orbits("stage 16 absent".to_owned()))?;
    let live_demand_orbits = stage16
        .orbits
        .iter()
        .filter(|orbit| matches!(orbit.resolution, OrbitResolution::Live))
        .count();
    let live_demand_outputs = stage16
        .orbits
        .iter()
        .filter(|orbit| matches!(orbit.resolution, OrbitResolution::Live))
        .map(|orbit| orbit.required_outputs.len())
        .sum();
    let provenance = ProvenanceCapacityAudit {
        candidate_stage: 16,
        kappa: 2,
        local_roles_per_clause: blind_local_role_coefficient(),
        local_capacity: blind_local_role_coefficient() * 2,
        live_demand_orbits,
        live_demand_outputs,
        orbit_derivation_hash: orbits.derivation_hash.clone(),
        assigned_anchors: Vec::new(),
        anchors_injective: true,
        all_marginal_sites_anchored: false,
    };

    let historical_regression = historical_regression()?;
    let historical_regression_passed = historical_regression
        .iter()
        .all(|record| record.rederived_recorded_score);
    let missing_obligations = vec![
        "typed constructor-eliminator computation family for beta".to_owned(),
        "typed coe monodromy family for each diagonal Kan site".to_owned(),
        "typed hcom naturality family for each ordered off-diagonal site".to_owned(),
        "schema/basis isomorphism proving availability, independence, and exhaustiveness"
            .to_owned(),
        "semantic weakening/univalent-equality decisions for the realized cubical families"
            .to_owned(),
        "injective EGP anchors for every family surviving weakening".to_owned(),
        "historical HIT score regression through the same typed machinery".to_owned(),
    ];
    let novelty = Tdc1Novelty::Undefined {
        missing_obligations,
    };

    let expected_obligations = enumerate_path_schema_obligations(&typing);
    let mut duplicate = expected_obligations.clone();
    duplicate[1] = duplicate[0].clone();
    let missing = &expected_obligations[..expected_obligations.len() - 1];
    let mut over_ceiling = expected_obligations.clone();
    over_ceiling.push(expected_obligations[0].clone());
    let collision = vec![
        AnchorSlot {
            clause: 1,
            role: LocalRole::Coherence,
        },
        AnchorSlot {
            clause: 1,
            role: LocalRole::Coherence,
        },
    ];
    let dimension_mutation_rejected = {
        let mutated = Telescope::new(vec![
            candidate.clauses[0].clone(),
            ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(3)),
        ]);
        elaborate_tdc1_package(&signature, &mutated, 15).is_err()
    };
    let falsifiers = MutationFalsifiers {
        dimension_mutation_rejected,
        duplicate_basis_site_rejected: !site_inventory_valid(&expected_obligations, &duplicate),
        missing_basis_site_rejected: !site_inventory_valid(&expected_obligations, missing),
        anchor_collision_rejected: !anchors_injective(&collision),
        over_ceiling_basis_rejected: !site_inventory_valid(&expected_obligations, &over_ceiling),
    };

    // Keep these exact symbolic counts mutually checked.  This does not
    // upgrade them into semantic families.
    let finite_keys: BTreeSet<PathSchemaKey> = finite_basis
        .schemas()
        .iter()
        .map(|key| match key {
            L1SchemaKey::Beta => PathSchemaKey::Beta,
            L1SchemaKey::Kan { principal, probe } => PathSchemaKey::Kan {
                principal: *principal,
                probe: *probe,
            },
        })
        .collect();
    let typed_keys: BTreeSet<PathSchemaKey> = obligations
        .iter()
        .map(|site| site.normal_form.key.clone())
        .collect();
    if finite_keys != typed_keys {
        return Err(Tdc1Error::Basis(
            "pen-type and pen-eval L1 index enumerations diverged".to_owned(),
        ));
    }

    let mut certificate = Tdc1Certificate {
        schema: TDC1_CERTIFICATE_SCHEMA.to_owned(),
        bar_independent: true,
        candidate: surface,
        aggregate_eliminator: token_attempt(&signature, &candidate, 15),
        basis,
        provenance,
        conditional_raw_ceiling_including_formation: 17 + formation_family_count,
        novelty,
        historical_regression,
        historical_regression_passed,
        falsifiers,
        next_scope_increment: vec![
            "add interval variables and a cofibration lattice to pen-type".to_owned(),
            "add typed coe/hcom terms with oriented computation and union-staging replay"
                .to_owned(),
            "add a motive-typed PathCon eliminator and constructor beta rule".to_owned(),
            "mint opaque per-site realization tokens only from those derivations".to_owned(),
            "extend typed univalent equality/weakening to the new cubical family terms".to_owned(),
            "rerun historical HIT regression before reading the Step-16 score".to_owned(),
        ],
        certificate_digest: String::new(),
    };
    certificate.certificate_digest = tagged_hash("certificate", &certificate);
    Ok(certificate)
}

pub fn replay_tdc1_certificate(certificate: &Tdc1Certificate) -> Result<(), Tdc1Error> {
    let replay = build_tdc1_certificate()?;
    if &replay == certificate {
        Ok(())
    } else {
        Err(Tdc1Error::ReplayMismatch)
    }
}

pub fn compare_frozen_certificate(
    certificate: &Tdc1Certificate,
) -> Result<Tdc1Comparison, Tdc1Error> {
    replay_tdc1_certificate(certificate)?;
    let certified_nu = certificate.novelty.certified_nu();
    let bar = num_rational::Ratio::new(354_333i128, 39_040i128);
    let rho = certified_nu.map(|nu| num_rational::Ratio::new(i128::from(nu), 2));
    let zone = match certified_nu {
        Some(nu) if nu >= 19 => Tdc1Zone::Z1,
        Some(nu) if nu >= 9 => Tdc1Zone::Z2,
        Some(_) => Tdc1Zone::Z3,
        None => Tdc1Zone::Z4,
    };
    let f_t1 = !certificate.historical_regression_passed;
    let f_t2 = !certificate.provenance.anchors_injective;
    let f_t3 = certificate.basis.raw_basis_count > certificate.basis.raw_ceiling_one_plus_d_squared;
    let meaning = match zone {
        Tdc1Zone::Z1 => "certified continuation-level value".to_owned(),
        Tdc1Zone::Z2 => "halt survives, but the 4*kappa envelope is falsified".to_owned(),
        Tdc1Zone::Z3 => "typed support for the debt-free 4*kappa envelope".to_owned(),
        Tdc1Zone::Z4 => {
            "fragment insufficient: novelty remains undefined; no numeric verdict".to_owned()
        }
    };
    let mut comparison = Tdc1Comparison {
        schema: TDC1_COMPARISON_SCHEMA.to_owned(),
        certificate_digest: certificate.certificate_digest.clone(),
        kappa: certificate.candidate.typing.kappa,
        certified_nu,
        rho: rho
            .as_ref()
            .map(|value| format!("{}/{}", value.numer(), value.denom())),
        bar: format!("{}/{}", bar.numer(), bar.denom()),
        clearing_threshold: 19,
        clears_bar: rho.as_ref().map(|value| value >= &bar),
        registered_zone: zone,
        historical_regression_passed: certificate.historical_regression_passed,
        step16_verdict_valid: certified_nu.is_some() && !f_t1 && !f_t2 && !f_t3,
        falsifier_f_t1: f_t1,
        falsifier_f_t2: f_t2,
        falsifier_f_t3: f_t3,
        meaning,
        comparison_digest: String::new(),
    };
    comparison.comparison_digest = tagged_hash("comparison", &comparison);
    Ok(comparison)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Structural test only: it does not build the final certificate or map
    /// a result to a registered zone.  Burn execution owns that observation.
    #[test]
    fn candidate_shape_and_two_independent_index_enumerators_agree() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = registered_d4_candidate();
        let (typing, _) = elaborate_tdc1_package(&signature, &candidate, 15)
            .expect("registered package elaborates");
        let typed = enumerate_path_schema_obligations(&typing);
        let finite = check_l1_finite_basis(4).expect("finite index checker");
        assert_eq!(typed.len(), finite.counts().total);
        assert_eq!(typed.len(), 17);
        assert!(finite.partition_is_exact());
    }

    #[test]
    fn anchor_slot_injectivity_rejects_reuse() {
        let slot = AnchorSlot {
            clause: 1,
            role: LocalRole::Coherence,
        };
        assert!(!anchors_injective(&[slot.clone(), slot]));
    }
}
