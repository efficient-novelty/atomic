//! SH-1 T2: exact-rational staircase iteration.
//!
//! The calculator is deliberately law-pluggable and consumes only the
//! registered post-Step-15 ledger seed.  It does not consult a target bar
//! when defining any credit law.  Selection is the frozen minimum-positive-
//! overshoot rule, followed by the lower-dimension tie-break.

use crate::sh1_capacity::DeclaredCapacityOracleV0;
use num_rational::Ratio;
use serde::{Serialize, Serializer};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use thiserror::Error;

pub const SH1_STAIRCASE_CALCULATOR_ID: &str = "sh1.staircase-calculator.t2.v0";
pub const SH1_QUADRATIC_LAW_ID: &str = "sh1.credit.formed-path.5-plus-d-squared";
pub const SH1_FALLBACK_LAW_ID: &str = "sh1.credit.fallback.4-plus-library-size";
pub const SH1_CAPACITY_GATED_LAW_ID: &str = "sh1.credit.capacity-gated.5-plus-d-squared.oracle-v0";
pub const SH1_REGISTERED_T2_THROUGH_STEP: u32 = 102;
pub const SH1_REGISTERED_T2_MAX_DIMENSION: u32 = 16;

/// Exact rational used by the SH-1 calculator.  It serializes as a single
/// normalized `numerator/denominator` string, avoiding JSON precision loss.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sh1ExactRational(Ratio<i128>);

impl Sh1ExactRational {
    pub fn new(numerator: i128, denominator: i128) -> Self {
        Self(Ratio::new(numerator, denominator))
    }

    pub fn numerator(&self) -> i128 {
        *self.0.numer()
    }

    pub fn denominator(&self) -> i128 {
        *self.0.denom()
    }

    fn from_ratio(value: Ratio<i128>) -> Self {
        Self(value)
    }
}

impl Display for Sh1ExactRational {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}/{}", self.numerator(), self.denominator())
    }
}

impl Serialize for Sh1ExactRational {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct StaircaseLedgerSeed {
    pub accepted_through_step: u32,
    pub library_size: u32,
    pub sum_nu: u64,
    pub sum_kappa: u64,
}

impl StaircaseLedgerSeed {
    /// Frozen premise P2; no candidate or valuation information is embedded.
    pub const fn genesis_15() -> Self {
        Self {
            accepted_through_step: 15,
            library_size: 15,
            sum_nu: 359,
            sum_kappa: 64,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct StaircaseConfig {
    pub seed: StaircaseLedgerSeed,
    pub through_step: u32,
    pub kappa: u32,
}

impl StaircaseConfig {
    pub const fn genesis_through(through_step: u32) -> Self {
        Self {
            seed: StaircaseLedgerSeed::genesis_15(),
            through_step,
            kappa: 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaircaseCreditContext {
    pub step_index: u32,
    pub library_size_before: u32,
}

/// Pluggable credit law.  `candidate_dimensions` defines the raw rung domain;
/// returning `None` from `nu` rejects that dimension before bar comparison.
pub trait Sh1NuLaw {
    fn law_id(&self) -> &'static str;

    fn candidate_dimensions(&self, context: StaircaseCreditContext) -> Vec<u32>;

    fn nu(
        &self,
        dimension: u32,
        context: StaircaseCreditContext,
    ) -> Result<Option<u32>, StaircaseError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormedPathQuadraticLaw {
    max_dimension: u32,
}

impl FormedPathQuadraticLaw {
    pub const fn new(max_dimension: u32) -> Self {
        Self { max_dimension }
    }
}

impl Sh1NuLaw for FormedPathQuadraticLaw {
    fn law_id(&self) -> &'static str {
        SH1_QUADRATIC_LAW_ID
    }

    fn candidate_dimensions(&self, _context: StaircaseCreditContext) -> Vec<u32> {
        (1..=self.max_dimension).collect()
    }

    fn nu(
        &self,
        dimension: u32,
        context: StaircaseCreditContext,
    ) -> Result<Option<u32>, StaircaseError> {
        quadratic_nu(dimension, self.law_id(), context.step_index).map(Some)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StructuralFallbackLaw {
    max_dimension: u32,
}

impl StructuralFallbackLaw {
    pub const fn new(max_dimension: u32) -> Self {
        Self { max_dimension }
    }
}

impl Sh1NuLaw for StructuralFallbackLaw {
    fn law_id(&self) -> &'static str {
        SH1_FALLBACK_LAW_ID
    }

    fn candidate_dimensions(&self, _context: StaircaseCreditContext) -> Vec<u32> {
        (1..=self.max_dimension).collect()
    }

    fn nu(
        &self,
        _dimension: u32,
        context: StaircaseCreditContext,
    ) -> Result<Option<u32>, StaircaseError> {
        context
            .library_size_before
            .checked_add(4)
            .map(Some)
            .ok_or(StaircaseError::CreditOverflow {
                law_id: SH1_FALLBACK_LAW_ID,
                dimension: None,
                step_index: context.step_index,
            })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CapacityGatedQuadraticLaw<'a> {
    oracle: &'a DeclaredCapacityOracleV0,
}

impl<'a> CapacityGatedQuadraticLaw<'a> {
    pub const fn new(oracle: &'a DeclaredCapacityOracleV0) -> Self {
        Self { oracle }
    }
}

impl Sh1NuLaw for CapacityGatedQuadraticLaw<'_> {
    fn law_id(&self) -> &'static str {
        SH1_CAPACITY_GATED_LAW_ID
    }

    fn candidate_dimensions(&self, _context: StaircaseCreditContext) -> Vec<u32> {
        self.oracle.licensed_dimensions()
    }

    fn nu(
        &self,
        dimension: u32,
        context: StaircaseCreditContext,
    ) -> Result<Option<u32>, StaircaseError> {
        if self.oracle.licenses_dimension(dimension) {
            quadratic_nu(dimension, self.law_id(), context.step_index).map(Some)
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StaircaseStep {
    pub step_index: u32,
    pub library_size_before: u32,
    pub dimension: u32,
    pub nu: u32,
    pub kappa: u32,
    pub rho: Sh1ExactRational,
    pub bar: Sh1ExactRational,
    pub margin: Sh1ExactRational,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StaircaseJumpPoint {
    pub step_index: u32,
    pub from_dimension: Option<u32>,
    pub to_dimension: u32,
    pub bar: Sh1ExactRational,
    pub rho: Sh1ExactRational,
    pub margin: Sh1ExactRational,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StaircasePlateau {
    pub dimension: u32,
    pub start_step: u32,
    pub end_step: u32,
    pub length: u32,
    pub first_nu: u32,
    pub last_nu: u32,
    pub constant_nu: Option<u32>,
    pub first_rho: Sh1ExactRational,
    pub last_rho: Sh1ExactRational,
    pub constant_rho: Option<Sh1ExactRational>,
    pub final_bar: Sh1ExactRational,
    pub final_margin: Sh1ExactRational,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StaircaseReport {
    pub calculator_id: &'static str,
    pub exact_backend: &'static str,
    pub law_id: &'static str,
    pub config: StaircaseConfig,
    pub accepted_through_step: u32,
    pub halted_at: Option<u32>,
    pub steps: Vec<StaircaseStep>,
    pub jump_points: Vec<StaircaseJumpPoint>,
    pub plateaus: Vec<StaircasePlateau>,
    pub step_23_margin: Option<Sh1ExactRational>,
}

/// Canonical T2 experiment bundle.  It runs all three registered laws over
/// the same ledger seed and carries oracle v0 (including its trust boundary)
/// alongside the capacity-gated result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RegisteredT2Suite {
    pub candidate_dimension_ceiling: u32,
    pub capacity_oracle: DeclaredCapacityOracleV0,
    pub formed_quadratic: StaircaseReport,
    pub structural_fallback: StaircaseReport,
    pub capacity_gated_quadratic: StaircaseReport,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum StaircaseError {
    #[error("staircase kappa must be positive")]
    ZeroKappa,
    #[error("the ledger seed must have positive cumulative kappa")]
    EmptyLedgerDenominator,
    #[error("SH-1 iteration starts after Step 2")]
    UnsupportedBootstrapSeed,
    #[error("{law_id} overflowed its u32 credit at step {step_index}, dimension {dimension:?}")]
    CreditOverflow {
        law_id: &'static str,
        dimension: Option<u32>,
        step_index: u32,
    },
    #[error("exact i128 Fibonacci arithmetic overflowed at index {index}")]
    FibonacciOverflow { index: u32 },
    #[error("ledger arithmetic overflowed at step {step_index}")]
    LedgerOverflow { step_index: u32 },
}

pub fn calculate_registered_t2_suite() -> Result<RegisteredT2Suite, StaircaseError> {
    let config = StaircaseConfig::genesis_through(SH1_REGISTERED_T2_THROUGH_STEP);
    let oracle = DeclaredCapacityOracleV0::declared();
    let formed_quadratic = calculate_staircase(
        &FormedPathQuadraticLaw::new(SH1_REGISTERED_T2_MAX_DIMENSION),
        config,
    )?;
    let structural_fallback = calculate_staircase(
        &StructuralFallbackLaw::new(SH1_REGISTERED_T2_MAX_DIMENSION),
        config,
    )?;
    let capacity_gated_quadratic =
        calculate_staircase(&CapacityGatedQuadraticLaw::new(&oracle), config)?;

    Ok(RegisteredT2Suite {
        candidate_dimension_ceiling: SH1_REGISTERED_T2_MAX_DIMENSION,
        capacity_oracle: oracle,
        formed_quadratic,
        structural_fallback,
        capacity_gated_quadratic,
    })
}

pub fn calculate_staircase<L: Sh1NuLaw>(
    law: &L,
    config: StaircaseConfig,
) -> Result<StaircaseReport, StaircaseError> {
    if config.kappa == 0 {
        return Err(StaircaseError::ZeroKappa);
    }
    if config.seed.sum_kappa == 0 {
        return Err(StaircaseError::EmptyLedgerDenominator);
    }
    if config.seed.accepted_through_step < 2 {
        return Err(StaircaseError::UnsupportedBootstrapSeed);
    }

    let start_step = config.seed.accepted_through_step.saturating_add(1);
    let mut library_size = config.seed.library_size;
    let mut sum_nu = i128::from(config.seed.sum_nu);
    let mut sum_kappa = i128::from(config.seed.sum_kappa);
    let mut steps = Vec::new();
    let mut halted_at = None;

    for step_index in start_step..=config.through_step {
        let phi = fibonacci_ratio(step_index)?;
        let omega = Ratio::new(sum_nu, sum_kappa);
        let bar = phi * omega;
        let context = StaircaseCreditContext {
            step_index,
            library_size_before: library_size,
        };

        let dimensions: BTreeSet<_> = law
            .candidate_dimensions(context)
            .into_iter()
            .filter(|dimension| *dimension > 0)
            .collect();
        let mut best: Option<(Ratio<i128>, u32, u32, Ratio<i128>)> = None;

        for dimension in dimensions {
            let Some(nu) = law.nu(dimension, context)? else {
                continue;
            };
            let rho = Ratio::new(i128::from(nu), i128::from(config.kappa));
            if rho < bar {
                continue;
            }
            let margin = rho - bar;
            let replace = best
                .as_ref()
                .is_none_or(|(best_margin, best_dimension, best_nu, _)| {
                    (&margin, dimension, nu) < (best_margin, *best_dimension, *best_nu)
                });
            if replace {
                best = Some((margin, dimension, nu, rho));
            }
        }

        let Some((margin, dimension, nu, rho)) = best else {
            halted_at = Some(step_index);
            break;
        };

        steps.push(StaircaseStep {
            step_index,
            library_size_before: library_size,
            dimension,
            nu,
            kappa: config.kappa,
            rho: Sh1ExactRational::from_ratio(rho),
            bar: Sh1ExactRational::from_ratio(bar),
            margin: Sh1ExactRational::from_ratio(margin),
        });

        sum_nu = sum_nu
            .checked_add(i128::from(nu))
            .ok_or(StaircaseError::LedgerOverflow { step_index })?;
        sum_kappa = sum_kappa
            .checked_add(i128::from(config.kappa))
            .ok_or(StaircaseError::LedgerOverflow { step_index })?;
        library_size = library_size
            .checked_add(1)
            .ok_or(StaircaseError::LedgerOverflow { step_index })?;
    }

    let accepted_through_step = steps
        .last()
        .map_or(config.seed.accepted_through_step, |step| step.step_index);
    let jump_points = build_jump_points(&steps);
    let plateaus = build_plateaus(&steps);
    let step_23_margin = steps
        .iter()
        .find(|step| step.step_index == 23)
        .map(|step| step.margin.clone());

    Ok(StaircaseReport {
        calculator_id: SH1_STAIRCASE_CALCULATOR_ID,
        exact_backend: "num_rational::Ratio<i128>",
        law_id: law.law_id(),
        config,
        accepted_through_step,
        halted_at,
        steps,
        jump_points,
        plateaus,
        step_23_margin,
    })
}

fn quadratic_nu(
    dimension: u32,
    law_id: &'static str,
    step_index: u32,
) -> Result<u32, StaircaseError> {
    dimension
        .checked_mul(dimension)
        .and_then(|square| square.checked_add(5))
        .ok_or(StaircaseError::CreditOverflow {
            law_id,
            dimension: Some(dimension),
            step_index,
        })
}

fn fibonacci_ratio(step_index: u32) -> Result<Ratio<i128>, StaircaseError> {
    let numerator = fibonacci(step_index)?;
    let denominator = fibonacci(step_index - 1)?;
    Ok(Ratio::new(numerator, denominator))
}

fn fibonacci(index: u32) -> Result<i128, StaircaseError> {
    if index <= 2 {
        return Ok(1);
    }
    let mut previous = 1_i128;
    let mut current = 1_i128;
    for position in 3..=index {
        let next = previous
            .checked_add(current)
            .ok_or(StaircaseError::FibonacciOverflow { index: position })?;
        previous = current;
        current = next;
    }
    Ok(current)
}

fn build_jump_points(steps: &[StaircaseStep]) -> Vec<StaircaseJumpPoint> {
    let mut previous_dimension = None;
    let mut jumps = Vec::new();
    for step in steps {
        if previous_dimension != Some(step.dimension) {
            jumps.push(StaircaseJumpPoint {
                step_index: step.step_index,
                from_dimension: previous_dimension,
                to_dimension: step.dimension,
                bar: step.bar.clone(),
                rho: step.rho.clone(),
                margin: step.margin.clone(),
            });
        }
        previous_dimension = Some(step.dimension);
    }
    jumps
}

fn build_plateaus(steps: &[StaircaseStep]) -> Vec<StaircasePlateau> {
    let mut plateaus = Vec::new();
    let mut start = 0;
    while start < steps.len() {
        let dimension = steps[start].dimension;
        let mut end = start;
        while end + 1 < steps.len() && steps[end + 1].dimension == dimension {
            end += 1;
        }
        let first = &steps[start];
        let last = &steps[end];
        let plateau_steps = &steps[start..=end];
        let constant_nu = plateau_steps
            .iter()
            .all(|step| step.nu == first.nu)
            .then_some(first.nu);
        let constant_rho = plateau_steps
            .iter()
            .all(|step| step.rho == first.rho)
            .then(|| first.rho.clone());
        plateaus.push(StaircasePlateau {
            dimension,
            start_step: first.step_index,
            end_step: last.step_index,
            length: last.step_index - first.step_index + 1,
            first_nu: first.nu,
            last_nu: last.nu,
            constant_nu,
            first_rho: first.rho.clone(),
            last_rho: last.rho.clone(),
            constant_rho,
            final_bar: last.bar.clone(),
            final_margin: last.margin.clone(),
        });
        start = end + 1;
    }
    plateaus
}

#[cfg(test)]
mod tests {
    use super::{
        CapacityGatedQuadraticLaw, FormedPathQuadraticLaw, Sh1ExactRational, StaircaseConfig,
        StructuralFallbackLaw, calculate_registered_t2_suite, calculate_staircase,
    };
    use crate::sh1_capacity::DeclaredCapacityOracleV0;

    #[test]
    fn quadratic_law_reproduces_all_registered_plateaus_exactly() {
        let report = calculate_staircase(
            &FormedPathQuadraticLaw::new(16),
            StaircaseConfig::genesis_through(101),
        )
        .expect("quadratic staircase should calculate");

        let spans: Vec<_> = report
            .plateaus
            .iter()
            .map(|plateau| {
                (
                    plateau.dimension,
                    plateau.start_step,
                    plateau.end_step,
                    plateau.length,
                    plateau.constant_nu,
                    plateau.constant_rho.clone(),
                )
            })
            .collect();
        assert_eq!(
            spans,
            vec![
                (4, 16, 23, 8, Some(21), Some(Sh1ExactRational::new(21, 2))),
                (5, 24, 42, 19, Some(30), Some(Sh1ExactRational::new(15, 1))),
                (6, 43, 68, 26, Some(41), Some(Sh1ExactRational::new(41, 2))),
                (7, 69, 101, 33, Some(54), Some(Sh1ExactRational::new(27, 1))),
            ]
        );
        assert_eq!(
            report
                .jump_points
                .iter()
                .map(|jump| (jump.step_index, jump.from_dimension, jump.to_dimension))
                .collect::<Vec<_>>(),
            vec![
                (16, None, 4),
                (24, Some(4), 5),
                (43, Some(5), 6),
                (69, Some(6), 7),
            ]
        );
        assert_eq!(
            report.step_23_margin,
            Some(Sh1ExactRational::new(4_867, 1_381_458))
        );
        assert_eq!(report.halted_at, None);
    }

    #[test]
    fn capacity_gate_reproduces_rungs_then_halts_when_d8_is_needed() {
        let oracle = DeclaredCapacityOracleV0::declared();
        let report = calculate_staircase(
            &CapacityGatedQuadraticLaw::new(&oracle),
            StaircaseConfig::genesis_through(102),
        )
        .expect("capacity-gated staircase should calculate");

        assert_eq!(
            report
                .plateaus
                .iter()
                .map(|plateau| (plateau.dimension, plateau.start_step, plateau.end_step))
                .collect::<Vec<_>>(),
            vec![(4, 16, 23), (5, 24, 42), (6, 43, 68), (7, 69, 101)]
        );
        assert_eq!(report.accepted_through_step, 101);
        assert_eq!(report.halted_at, Some(102));
        assert_eq!(
            report.step_23_margin,
            Some(Sh1ExactRational::new(4_867, 1_381_458))
        );
    }

    #[test]
    fn fallback_law_exposes_the_repeated_d1_echo_and_growing_credit() {
        let report = calculate_staircase(
            &StructuralFallbackLaw::new(8),
            StaircaseConfig::genesis_through(20),
        )
        .expect("fallback staircase should calculate");

        assert_eq!(report.plateaus.len(), 1);
        let plateau = &report.plateaus[0];
        assert_eq!(
            (plateau.dimension, plateau.start_step, plateau.end_step),
            (1, 16, 20)
        );
        assert_eq!((plateau.first_nu, plateau.last_nu), (19, 23));
        assert_eq!(plateau.constant_nu, None);
        assert!(
            report
                .steps
                .windows(2)
                .all(|pair| pair[1].margin > pair[0].margin)
        );
        assert!(report.steps.iter().all(|step| step.dimension == 1));
        assert_eq!(report.halted_at, None);
    }

    #[test]
    fn exact_rationals_serialize_without_precision_loss() {
        let encoded = serde_json::to_string(&Sh1ExactRational::new(4_867, 1_381_458))
            .expect("exact rational should serialize");
        assert_eq!(encoded, "\"4867/1381458\"");
    }

    #[test]
    fn registered_suite_runs_all_three_laws_without_hiding_the_oracle_status() {
        let suite = calculate_registered_t2_suite().expect("registered T2 suite should calculate");

        assert_eq!(suite.formed_quadratic.halted_at, None);
        assert_eq!(suite.structural_fallback.halted_at, None);
        assert_eq!(suite.capacity_gated_quadratic.halted_at, Some(102));
        assert_eq!(
            suite.capacity_oracle.trust_boundary.epistemic_status,
            "declared_not_derived"
        );
        assert_eq!(
            suite.formed_quadratic.step_23_margin,
            suite.capacity_gated_quadratic.step_23_margin
        );
    }
}
