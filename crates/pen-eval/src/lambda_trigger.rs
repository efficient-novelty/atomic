use crate::bar::compute_rho;
use crate::lambda_structure::frozen_structure_schema_variants;
use crate::nu::{NativeNuResult, structural_nu};
use crate::runtime_bar::{fib, fresh_stratum_bars_through, inherited_bars, post_15_ledger};
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use serde::Serialize;

pub const LAMBDA_TRIGGER_DATE: &str = "2026-07-04";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LambdaTriggerComputation {
    pub date: String,
    pub freeze_commit: String,
    pub eval_commit: String,
    pub task_a: TaskAOutput,
    pub variants: Vec<VariantOutput>,
    pub verdict: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TaskAOutput {
    pub inherited_bars_16_24: Vec<[String; 2]>,
    pub omega_frozen: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VariantOutput {
    pub name: String,
    pub rationale: String,
    pub nu: NuSplitOutput,
    pub kappa: u32,
    pub rho: String,
    pub c1_crossing_step: usize,
    pub c1_stratum_bars: Vec<String>,
    pub c2_capacity_clause_units: String,
    pub c2_crossing_step: usize,
    pub c3_exported_band: String,
    pub c3_exported_share: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct NuSplitOutput {
    #[serde(rename = "G")]
    pub g: u32,
    #[serde(rename = "C")]
    pub c: u32,
    #[serde(rename = "H")]
    pub h: u32,
}

pub fn build_lambda_trigger_computation(
    freeze_commit: impl Into<String>,
    eval_commit: impl Into<String>,
) -> LambdaTriggerComputation {
    let (library, history) = reference_library(15);
    let variants = frozen_structure_schema_variants()
        .into_iter()
        .map(|variant| {
            let native = crate::nu::compute_native_nu(&variant.telescope, &library, &history);
            let kappa = u32::try_from(variant.telescope.kappa()).expect("kappa should fit u32");
            let rho = compute_rho(native.total, kappa).expect("frozen variants have nonzero kappa");
            let stratum_steps = prefix_stratum_steps(&variant.telescope, &library, &history);
            let c1_bars = fresh_stratum_bars_through(&stratum_steps, 24);
            let c1_crossing_step = first_bar_crossing(&c1_bars, rho);
            let c2_capacity = plateau_clause_capacity(&stratum_steps);
            let c2_crossing_step = first_debt_crossing(c2_capacity);
            let c3_share = depth_two_share(c2_crossing_step);
            let c3_band = if c2_crossing_step == 16 {
                "depth-2"
            } else {
                "mixed"
            };

            VariantOutput {
                name: variant.name.to_owned(),
                rationale: variant.rationale.to_owned(),
                nu: NuSplitOutput {
                    g: native.nu_g,
                    c: native.nu_c,
                    h: native.nu_h,
                },
                kappa,
                rho: rational_string(rho),
                c1_crossing_step,
                c1_stratum_bars: c1_bars.into_iter().map(rational_string).collect(),
                c2_capacity_clause_units: rational_string(c2_capacity),
                c2_crossing_step,
                c3_exported_band: c3_band.to_owned(),
                c3_exported_share: rational_string(c3_share),
            }
        })
        .collect::<Vec<_>>();

    LambdaTriggerComputation {
        date: LAMBDA_TRIGGER_DATE.to_owned(),
        freeze_commit: freeze_commit.into(),
        eval_commit: eval_commit.into(),
        task_a: TaskAOutput {
            inherited_bars_16_24: inherited_bars(16, 24, &post_15_ledger())
                .into_iter()
                .map(rational_pair)
                .collect(),
            omega_frozen: rational_string(post_15_ledger().omega),
        },
        verdict: verdict(&variants).to_owned(),
        variants,
    }
}

fn reference_library(last_step: u32) -> (Library, Vec<(u32, u32)>) {
    let mut library = Vec::new();
    let mut history = Vec::new();

    for step in 1..=last_step {
        let telescope = Telescope::reference(step);
        let result = structural_nu(&telescope, &library, &history);
        library.push(LibraryEntry::from_telescope(&telescope, &library));
        history.push((step, result.total));
    }

    (library, history)
}

fn prefix_stratum_steps(
    telescope: &Telescope,
    library: &Library,
    history: &[(u32, u32)],
) -> Vec<(u64, u64)> {
    (1..=telescope.clauses.len())
        .map(|len| {
            let prefix = Telescope::new(telescope.clauses[..len].to_vec());
            let NativeNuResult { total, .. } =
                crate::nu::compute_native_nu(&prefix, library, history);
            (
                u64::from(total),
                u64::try_from(prefix.kappa()).expect("prefix kappa should fit u64"),
            )
        })
        .collect()
}

fn first_bar_crossing(bars: &[Rational], rho: Rational) -> usize {
    bars.iter()
        .position(|bar| *bar > rho)
        .map(|index| index + 1)
        .unwrap_or(0)
}

fn plateau_clause_capacity(stratum_steps: &[(u64, u64)]) -> Rational {
    let capacity = if stratum_steps.len() > 1 {
        stratum_steps[stratum_steps.len() - 2].1
    } else {
        stratum_steps[0].1
    };
    Rational::from_integer(capacity as i64)
}

fn first_debt_crossing(capacity: Rational) -> usize {
    (1..=64)
        .find(|n| Rational::from_integer(fib(*n) as i64) > capacity)
        .unwrap_or(0)
}

fn depth_two_share(crossing_step: usize) -> Rational {
    if crossing_step <= 2 {
        Rational::zero()
    } else {
        Rational::new(fib(crossing_step - 2) as i64, fib(crossing_step) as i64)
    }
}

fn verdict(variants: &[VariantOutput]) -> &'static str {
    let c1 = variants
        .iter()
        .any(|variant| variant.c1_crossing_step == 16);
    let c2 = variants
        .iter()
        .any(|variant| variant.c2_crossing_step == 16);
    let c3 = variants
        .iter()
        .any(|variant| variant.c3_exported_band == "depth-2");

    match (c1, c2, c3) {
        (true, true, true) => "close",
        (true, false, _) => "partial-C1",
        (false, true, _) => "partial-C2",
        _ => "miss",
    }
}

pub fn rational_string(value: Rational) -> String {
    format!("{}/{}", value.num(), value.den())
}

fn rational_pair(value: Rational) -> [String; 2] {
    [value.num().to_string(), value.den().to_string()]
}

#[cfg(test)]
mod tests {
    use super::{build_lambda_trigger_computation, first_debt_crossing, rational_string};
    use pen_core::rational::Rational;

    #[test]
    fn computation_reproduces_required_task_a_constants() {
        let computation = build_lambda_trigger_computation("freeze", "eval");
        assert_eq!(computation.task_a.omega_frozen, "359/64");
        assert_eq!(
            computation.task_a.inherited_bars_16_24[0],
            ["354333".to_owned(), "39040".to_owned()]
        );
    }

    #[test]
    fn debt_crossing_uses_strict_greater_than_fibonacci_capacity() {
        assert_eq!(first_debt_crossing(Rational::from_integer(1)), 3);
        assert_eq!(first_debt_crossing(Rational::from_integer(2)), 4);
        assert_eq!(first_debt_crossing(Rational::from_integer(610)), 16);
        assert_eq!(rational_string(Rational::new(377, 987)), "377/987");
    }
}
