use crate::coherence::d_bonacci_delta;
use pen_core::rational::Rational;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiscoveryRecord {
    pub step_index: u32,
    pub nu: u32,
    pub kappa: u32,
}

impl DiscoveryRecord {
    pub const fn new(step_index: u32, nu: u32, kappa: u32) -> Self {
        Self {
            step_index,
            nu,
            kappa,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BarComputation {
    pub phi: Rational,
    pub omega: Rational,
    pub bar: Rational,
}

pub fn compute_bar(
    window_depth: usize,
    step_index: u32,
    history: &[DiscoveryRecord],
) -> BarComputation {
    if step_index <= 2 {
        let omega = Rational::new(1, 2);
        return BarComputation {
            phi: Rational::one(),
            omega,
            bar: omega,
        };
    }

    let delta_n = i64::try_from(d_bonacci_delta(window_depth, step_index as usize))
        .expect("d-bonacci delta exceeded i64 range");
    let delta_nm1 = i64::try_from(d_bonacci_delta(window_depth, (step_index - 1) as usize))
        .expect("d-bonacci delta exceeded i64 range");
    let phi = Rational::new(delta_n, delta_nm1);

    let (sum_nu, sum_kappa) = history.iter().take((step_index - 1) as usize).fold(
        (0_i64, 0_i64),
        |(acc_nu, acc_kappa), record| {
            (
                acc_nu + i64::from(record.nu),
                acc_kappa + i64::from(record.kappa),
            )
        },
    );

    let omega = if sum_kappa > 0 {
        Rational::new(sum_nu, sum_kappa)
    } else {
        Rational::one()
    };

    BarComputation {
        phi,
        omega,
        bar: phi * omega,
    }
}

pub fn compute_rho(nu: u32, kappa: u32) -> Option<Rational> {
    if kappa == 0 {
        None
    } else {
        Some(Rational::new(i64::from(nu), i64::from(kappa)))
    }
}

pub fn clears_bar(rho: Rational, bar: Rational) -> bool {
    rho >= bar
}

pub fn positive_overshoot(rho: Rational, bar: Rational) -> Option<Rational> {
    if clears_bar(rho, bar) {
        Some(rho - bar)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BarComputation, DiscoveryRecord, clears_bar, compute_bar, compute_rho, positive_overshoot,
    };
    use pen_core::rational::Rational;

    #[test]
    fn early_steps_use_frozen_half_bar() {
        let bar = compute_bar(2, 2, &[]);
        assert_eq!(
            bar,
            BarComputation {
                phi: Rational::one(),
                omega: Rational::new(1, 2),
                bar: Rational::new(1, 2),
            }
        );
    }

    #[test]
    fn fibonacci_bar_uses_exact_history_ratio() {
        let history = [DiscoveryRecord::new(1, 1, 2), DiscoveryRecord::new(2, 1, 1)];
        let bar = compute_bar(2, 3, &history);
        assert_eq!(bar.phi, Rational::new(2, 1));
        assert_eq!(bar.omega, Rational::new(2, 3));
        assert_eq!(bar.bar, Rational::new(4, 3));
    }

    #[test]
    fn rho_and_overshoot_stay_exact() {
        let rho = compute_rho(17, 4).expect("rho should exist");
        let bar = Rational::new(401, 100);
        assert!(clears_bar(rho, bar));
        assert_eq!(positive_overshoot(rho, bar), Some(Rational::new(24, 100)));
    }
}
