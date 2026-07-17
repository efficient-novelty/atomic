//! Definition-derived runtime semantics and the runtime-clock no-go.
//!
//! This module is deliberately separate from the frozen 2026-07-05 runtime
//! experiment in `runtime_dedup`, `runtime_field`, and `runtime_report`.  That
//! experiment remains immutable evidence.  The semantics here are the direct
//! executable reading of Appendix A's formal contract:
//!
//! - `nu_H(x)` and `kappa_H(x)` are marginal audits relative to realized
//!   history `H`;
//! - the ledger index advances only when `I(H, x)` seals a selected extension;
//! - an idle search tick widens the active cost horizon but does not create a
//!   new layer, renew Fibonacci debt, or update `Omega`;
//! - an exact repeat has an empty kernel over the enlarged history and is a
//!   theorem-readout, not a candidate continuation.
//! - quantitative debt `Delta_n` must not be conflated with the Guard-Rail
//!   Theorem's directive debt `O(n)`: both are functions of realized windows,
//!   while only the latter can vanish.
//!
//! These rules determine a realization-order semantics.  They do *not*
//! determine a physical cadence or throughput unit.  The temporal shell may
//! supply an internal continuous parameter `t`, but the ignition chapter
//! explicitly distinguishes it from the structural realization order.  Any
//! monotone assignment of physical timestamps to the same realization trace
//! preserves every `nu`, `kappa`, `rho`, `Omega`, and bar comparison.  A map
//! from realization order to time, scale factor, or redshift is therefore an
//! additional bridge, not a theorem of the two audit definitions.

use crate::runtime_bar::phi;
use pen_core::rational::Rational;

/// History-relative audit counts from the formal definitions.
///
/// The caller must obtain these from the normalized kernel-clause set and the
/// normalized derivation-schema basis.  The current structural evaluator
/// computes closed-form totals but does not expose either witness set, so this
/// type intentionally does not pretend to reconstruct them from an AST.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MarginalAudit {
    /// `|L(I(H,x)) \ L(H)|`.
    pub nu: u64,
    /// `|KernelClauses(x | H)|`.
    pub kappa: u64,
}

impl MarginalAudit {
    pub const fn new(nu: u64, kappa: u64) -> Self {
        Self { nu, kappa }
    }

    /// Classifies the audit exactly as the formal continuation cone does.
    pub fn disposition(self) -> AuditDisposition {
        if self.kappa == 0 {
            AuditDisposition::TheoremReadout
        } else if self.nu == 0 {
            AuditDisposition::NonGenerative
        } else {
            AuditDisposition::Generative {
                rho: Rational::new(as_i64(self.nu), as_i64(self.kappa)),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditDisposition {
    /// Empty history-relative kernel: a derivation or redescription of `H`.
    TheoremReadout,
    /// A positive public burden that opens no new derivation schema.
    NonGenerative,
    /// A genuine candidate continuation, subject to the bar and prime filter.
    Generative { rho: Rational },
}

/// The part of PEN state fixed by the audited realization history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizedLedger {
    realized_steps: usize,
    active_horizon: u64,
    sum_nu: u64,
    sum_kappa: u64,
}

impl RealizedLedger {
    pub const fn new(initial_horizon: u64) -> Self {
        assert!(initial_horizon > 0, "the active horizon is positive");
        Self {
            realized_steps: 0,
            active_horizon: initial_horizon,
            sum_nu: 0,
            sum_kappa: 0,
        }
    }

    pub const fn realized_steps(&self) -> usize {
        self.realized_steps
    }

    pub const fn active_horizon(&self) -> u64 {
        self.active_horizon
    }

    pub const fn sum_nu(&self) -> u64 {
        self.sum_nu
    }

    pub const fn sum_kappa(&self) -> u64 {
        self.sum_kappa
    }

    pub fn omega(&self) -> Option<Rational> {
        (self.sum_kappa > 0).then(|| Rational::new(as_i64(self.sum_nu), as_i64(self.sum_kappa)))
    }

    /// The bar faced by the next realized layer.
    ///
    /// The empty-history seed convention is owned by the lane and is therefore
    /// not invented here.  Once history is nonempty, this is Appendix A's
    /// `Bar_n = Phi_n Omega_{n-1}` with `n = |H| + 1`.
    pub fn next_bar(&self) -> Option<Rational> {
        self.omega()
            .map(|omega| phi(self.realized_steps + 1) * omega)
    }

    /// PEN's idle update: widen the cost horizon without changing history.
    pub fn idle_search_tick(&mut self) {
        self.active_horizon = self
            .active_horizon
            .checked_add(1)
            .expect("active horizon exceeded u64");
    }

    /// Records an extension after constitution, bar, and prime filtering.
    ///
    /// A zero-kernel audit cannot be integrated as a new history step because
    /// the formal admissible cone excludes it.  Selection policy itself stays
    /// outside this small reference state machine.
    pub fn record_integration(&mut self, audit: MarginalAudit) -> Result<(), IntegrationError> {
        if audit.kappa == 0 {
            return Err(IntegrationError::ZeroKernelReadout);
        }
        self.sum_nu = self
            .sum_nu
            .checked_add(audit.nu)
            .expect("cumulative novelty exceeded u64");
        self.sum_kappa = self
            .sum_kappa
            .checked_add(audit.kappa)
            .expect("cumulative kernel cost exceeded u64");
        self.realized_steps = self
            .realized_steps
            .checked_add(1)
            .expect("realized step index exceeded usize");
        self.active_horizon = 2;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegrationError {
    ZeroKernelReadout,
}

/// A realization trace embedded in an arbitrary external clock.
///
/// Timestamps are required only to be strictly increasing.  Projecting them
/// away returns the identical audit trace, making the time-reparameterization
/// freedom executable rather than rhetorical.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimedRealization {
    pub timestamp: u64,
    pub audit: MarginalAudit,
}

pub fn embed_in_external_clock(
    trace: &[MarginalAudit],
    timestamps: &[u64],
) -> Result<Vec<TimedRealization>, ClockEmbeddingError> {
    if trace.len() != timestamps.len() {
        return Err(ClockEmbeddingError::LengthMismatch);
    }
    if timestamps.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(ClockEmbeddingError::NonMonotone);
    }
    Ok(trace
        .iter()
        .copied()
        .zip(timestamps.iter().copied())
        .map(|(audit, timestamp)| TimedRealization { timestamp, audit })
        .collect())
}

pub fn erase_external_clock(timed: &[TimedRealization]) -> Vec<MarginalAudit> {
    timed.iter().map(|entry| entry.audit).collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockEmbeddingError {
    LengthMismatch,
    NonMonotone,
}

fn as_i64(value: u64) -> i64 {
    i64::try_from(value).expect("audit count exceeded Rational's i64 range")
}

#[cfg(test)]
mod tests {
    use super::{
        embed_in_external_clock, erase_external_clock, AuditDisposition, ClockEmbeddingError,
        IntegrationError, MarginalAudit, RealizedLedger,
    };
    use crate::nu::structural_nu;
    use pen_core::{
        library::{Library, LibraryEntry},
        rational::Rational,
        telescope::Telescope,
    };

    fn genesis_audits() -> Vec<MarginalAudit> {
        let mut library: Library = Vec::new();
        let mut history = Vec::new();
        let mut audits = Vec::new();
        for step in 1..=15 {
            let telescope = Telescope::reference(step);
            let nu = structural_nu(&telescope, &library, &history).total;
            let kappa = u64::try_from(telescope.kappa()).expect("kappa fits u64");
            audits.push(MarginalAudit::new(u64::from(nu), kappa));
            library.push(LibraryEntry::from_telescope(&telescope, &library));
            history.push((step, nu));
        }
        audits
    }

    #[test]
    fn genesis_one_through_fifteen_is_value_conservative() {
        let mut ledger = RealizedLedger::new(2);
        for audit in genesis_audits() {
            ledger.record_integration(audit).unwrap();
        }

        assert_eq!(ledger.realized_steps(), 15);
        assert_eq!(ledger.sum_nu(), 359);
        assert_eq!(ledger.sum_kappa(), 64);
        assert_eq!(ledger.omega(), Some(Rational::new(359, 64)));
        assert_eq!(ledger.next_bar(), Some(Rational::new(354_333, 39_040)));
    }

    #[test]
    fn idle_search_widens_horizon_without_inventing_a_ledger_clock() {
        let mut ledger = RealizedLedger::new(2);
        ledger.record_integration(MarginalAudit::new(7, 3)).unwrap();
        let before = ledger.clone();

        for _ in 0..100 {
            ledger.idle_search_tick();
        }

        assert_eq!(ledger.realized_steps(), before.realized_steps());
        assert_eq!(ledger.sum_nu(), before.sum_nu());
        assert_eq!(ledger.sum_kappa(), before.sum_kappa());
        assert_eq!(ledger.omega(), before.omega());
        assert_eq!(ledger.next_bar(), before.next_bar());
        assert_eq!(ledger.active_horizon(), 102);
    }

    #[test]
    fn exact_repetition_is_a_readout_and_cannot_tick_history() {
        let duplicate = MarginalAudit::new(0, 0);
        assert_eq!(duplicate.disposition(), AuditDisposition::TheoremReadout);

        let mut ledger = RealizedLedger::new(2);
        assert_eq!(
            ledger.record_integration(duplicate),
            Err(IntegrationError::ZeroKernelReadout)
        );
        assert_eq!(ledger.realized_steps(), 0);
    }

    #[test]
    fn positive_cost_without_novelty_is_distinct_from_a_readout() {
        assert_eq!(
            MarginalAudit::new(0, 3).disposition(),
            AuditDisposition::NonGenerative
        );
        assert_eq!(
            MarginalAudit::new(7, 3).disposition(),
            AuditDisposition::Generative {
                rho: Rational::new(7, 3)
            }
        );
    }

    #[test]
    fn physical_cadence_is_not_identified_by_the_audit_trace() {
        let trace = [
            MarginalAudit::new(1, 2),
            MarginalAudit::new(1, 1),
            MarginalAudit::new(2, 1),
        ];
        let fast = embed_in_external_clock(&trace, &[1, 2, 3]).unwrap();
        let slow = embed_in_external_clock(&trace, &[1, 1_000, 1_000_000]).unwrap();

        assert_ne!(fast, slow);
        assert_eq!(erase_external_clock(&fast), trace);
        assert_eq!(erase_external_clock(&slow), trace);
        assert_eq!(
            embed_in_external_clock(&trace, &[1, 1, 2]),
            Err(ClockEmbeddingError::NonMonotone)
        );
    }
}
