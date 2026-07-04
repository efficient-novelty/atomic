use pen_core::rational::Rational;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LedgerState {
    pub omega: Rational,
}

impl LedgerState {
    pub const fn new(omega: Rational) -> Self {
        Self { omega }
    }
}

pub fn post_15_ledger() -> LedgerState {
    LedgerState::new(Rational::new(359, 64))
}

pub fn fib(n: usize) -> u64 {
    assert!(n >= 1, "Fibonacci indexing starts at F_1");
    if n <= 2 {
        return 1;
    }

    let mut prev = 1_u64;
    let mut curr = 1_u64;
    for _ in 3..=n {
        let next = prev
            .checked_add(curr)
            .expect("Fibonacci value exceeded u64 range");
        prev = curr;
        curr = next;
    }
    curr
}

pub fn phi(n: usize) -> Rational {
    if n <= 1 {
        Rational::one()
    } else {
        Rational::new(fib(n) as i64, fib(n - 1) as i64)
    }
}

/// Inherited reading: Bar_n = (F_n / F_{n-1}) * Omega, with Omega frozen.
pub fn inherited_bar(n: usize, omega: &LedgerState) -> Rational {
    phi(n) * omega.omega
}

pub fn inherited_bars(start_n: usize, end_n: usize, omega: &LedgerState) -> Vec<Rational> {
    assert!(start_n <= end_n, "bar range must be non-empty and ordered");
    (start_n..=end_n).map(|n| inherited_bar(n, omega)).collect()
}

/// Fresh-stratum reading over already accepted stratum steps.
///
/// Each emitted bar uses the stratum's cumulative Omega after that local step.
/// For consumers that need bars beyond the frozen schema, use
/// `fresh_stratum_bars_through`, which keeps the final Omega fixed.
pub fn fresh_stratum_bars(steps: &[(u64, u64)]) -> Vec<Rational> {
    let mut sum_nu = 0_u64;
    let mut sum_kappa = 0_u64;
    steps
        .iter()
        .enumerate()
        .map(|(index, &(nu, kappa))| {
            sum_nu = sum_nu
                .checked_add(nu)
                .expect("fresh-stratum nu sum exceeded u64 range");
            sum_kappa = sum_kappa
                .checked_add(kappa)
                .expect("fresh-stratum kappa sum exceeded u64 range");
            let omega = LedgerState::new(Rational::new(sum_nu as i64, sum_kappa as i64));
            inherited_bar(index + 1, &omega)
        })
        .collect()
}

/// Emits bars 1..=max_n, freezing Omega after the last supplied stratum step.
pub fn fresh_stratum_bars_through(steps: &[(u64, u64)], max_n: usize) -> Vec<Rational> {
    assert!(!steps.is_empty(), "fresh stratum needs at least one step");
    let mut sum_nu = 0_u64;
    let mut sum_kappa = 0_u64;
    let mut bars = Vec::with_capacity(max_n);

    for n in 1..=max_n {
        if let Some(&(nu, kappa)) = steps.get(n - 1) {
            sum_nu = sum_nu
                .checked_add(nu)
                .expect("fresh-stratum nu sum exceeded u64 range");
            sum_kappa = sum_kappa
                .checked_add(kappa)
                .expect("fresh-stratum kappa sum exceeded u64 range");
        }
        let omega = LedgerState::new(Rational::new(sum_nu as i64, sum_kappa as i64));
        bars.push(inherited_bar(n, &omega));
    }

    bars
}

#[cfg(test)]
mod tests {
    use super::{
        LedgerState, fib, fresh_stratum_bars, inherited_bar, inherited_bars, phi, post_15_ledger,
    };
    use pen_core::rational::Rational;

    #[test]
    fn fibonacci_indexing_matches_runtime_handoff() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(15), 610);
        assert_eq!(fib(16), 987);
        assert_eq!(fib(17), 1597);
        assert_eq!(phi(16), Rational::new(987, 610));
    }

    #[test]
    fn inherited_runtime_bar_reproduces_step_sixteen_constant() {
        assert_eq!(post_15_ledger().omega, Rational::new(359, 64));
        assert_eq!(
            inherited_bar(16, &post_15_ledger()),
            Rational::new(354333, 39040)
        );
        assert_eq!(
            inherited_bars(16, 16, &LedgerState::new(Rational::new(359, 64))),
            vec![Rational::new(354333, 39040)]
        );
    }

    #[test]
    fn fresh_stratum_bars_accumulate_local_omega_exactly() {
        let bars = fresh_stratum_bars(&[(1, 2), (2, 1), (5, 3)]);
        assert_eq!(bars[0], Rational::new(1, 2));
        assert_eq!(bars[1], Rational::new(3, 3));
        assert_eq!(bars[2], Rational::new(2, 1) * Rational::new(8, 6));
    }
}
