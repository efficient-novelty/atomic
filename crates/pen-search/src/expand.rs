use pen_core::canonical::{CanonKey, canonical_key_telescope};
use pen_core::clause::ClauseRec;
use pen_core::encode::telescope_bit_cost;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::Library;
use pen_core::rational::Rational;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::bar::{DiscoveryRecord, compute_rho};
use pen_eval::nu::{NativeNuResult, compute_native_nu};
use pen_type::check::{CheckError, CheckResult, check_telescope};
use pen_type::obligations::RetentionSignals;
use std::collections::BTreeSet;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StructuralSignals {
    pub eliminator_score: u16,
    pub former_score: u16,
    pub dependent_motive_density: u16,
    pub library_reference_density: u16,
    pub generic_binder_count: u16,
    pub closure_score: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpandedCandidate {
    pub telescope: Telescope,
    pub telescope_class: TelescopeClass,
    pub signals: StructuralSignals,
    pub canonical_key: CanonKey,
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub bit_kappa: u16,
    pub clause_kappa: u16,
    pub nu: u16,
    pub rho: Rational,
    pub trace: Vec<String>,
    pub shape_fingerprint: String,
    pub support_fingerprint: String,
}

impl ExpandedCandidate {
    pub fn retention_signals(&self) -> RetentionSignals {
        RetentionSignals {
            telescope_class: self.telescope_class,
            eliminator_score: self.signals.eliminator_score,
            former_score: self.signals.former_score,
            dependent_motive_density: self.signals.dependent_motive_density,
            library_reference_density: self.signals.library_reference_density,
            generic_binder_count: self.signals.generic_binder_count,
            closure_score: self.signals.closure_score,
        }
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ExpansionError {
    #[error("candidate failed conservative checking: {0}")]
    Check(#[from] CheckError),
    #[error("candidate has zero clause kappa")]
    ZeroClauseKappa,
}

pub fn expand_with_clause(
    prefix: &Telescope,
    clause: ClauseRec,
    library: &Library,
    history: &[DiscoveryRecord],
) -> Result<ExpandedCandidate, ExpansionError> {
    let mut telescope = prefix.clone();
    telescope.clauses.push(clause);
    evaluate_candidate(library, history, telescope)
}

pub fn evaluate_candidate(
    library: &Library,
    history: &[DiscoveryRecord],
    telescope: Telescope,
) -> Result<ExpandedCandidate, ExpansionError> {
    match check_telescope(library, &telescope) {
        CheckResult::Ok => {}
        CheckResult::Err(error) => return Err(ExpansionError::Check(error)),
    }

    evaluate_checked_candidate(library, history, telescope)
}

pub fn evaluate_checked_candidate(
    library: &Library,
    history: &[DiscoveryRecord],
    telescope: Telescope,
) -> Result<ExpandedCandidate, ExpansionError> {
    let native = compute_native_nu(
        &telescope,
        library,
        &history
            .iter()
            .map(|record| (record.step_index, record.nu))
            .collect::<Vec<_>>(),
    );
    build_candidate(telescope, native, library)
}

fn build_candidate(
    telescope: Telescope,
    native: NativeNuResult,
    library: &Library,
) -> Result<ExpandedCandidate, ExpansionError> {
    let clause_kappa = u16::try_from(telescope.kappa()).expect("kappa exceeded u16");
    if clause_kappa == 0 {
        return Err(ExpansionError::ZeroClauseKappa);
    }

    let bit_kappa = u16::try_from(telescope_bit_cost(&telescope)).expect("bit cost exceeded u16");
    let nu = u16::try_from(native.total).expect("nu exceeded u16");
    let rho = compute_rho(u32::from(nu), u32::from(clause_kappa)).expect("rho should exist");
    let candidate_json =
        serde_json::to_vec(&telescope).expect("candidate telescope should serialize");
    let candidate_hash = format!("blake3:{}", blake3_hex(&candidate_json));
    let canonical_key = canonical_key_telescope(&telescope);
    let canonical_hash = format!("blake3:{}", blake3_hex(canonical_key.0.as_bytes()));
    let telescope_class = telescope.classify(library);
    let signals = structural_signals(&telescope);

    Ok(ExpandedCandidate {
        shape_fingerprint: fingerprint_hex(candidate_hash.as_bytes()),
        support_fingerprint: fingerprint_hex(canonical_hash.as_bytes()),
        telescope,
        telescope_class,
        signals,
        canonical_key,
        candidate_hash,
        canonical_hash,
        bit_kappa,
        clause_kappa,
        nu,
        rho,
        trace: native.trace,
    })
}

fn fingerprint_hex(bytes: &[u8]) -> String {
    let hash = blake3_hex(bytes);
    format!("0x{}", &hash[..16])
}

fn structural_signals(telescope: &Telescope) -> StructuralSignals {
    let mut signals = StructuralSignals::default();
    let mut closure_refs = BTreeSet::new();

    for clause in &telescope.clauses {
        closure_refs.extend(clause.expr.var_refs());
        accumulate_expr_signals(&clause.expr, &mut signals);
    }

    signals.library_reference_density =
        u16::try_from(telescope.lib_refs().len()).expect("library ref count exceeded u16");
    signals.closure_score = u16::try_from(closure_refs.len()).expect("closure score exceeded u16");
    signals
}

fn accumulate_expr_signals(expr: &Expr, signals: &mut StructuralSignals) {
    match expr {
        Expr::App(function, argument) => {
            if matches!(function.as_ref(), Expr::Lam(_) | Expr::App(_, _)) {
                signals.eliminator_score = signals.eliminator_score.saturating_add(1);
            }
            accumulate_expr_signals(function, signals);
            accumulate_expr_signals(argument, signals);
        }
        Expr::Lam(body) => {
            signals.generic_binder_count = signals.generic_binder_count.saturating_add(1);
            accumulate_expr_signals(body, signals);
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            signals.former_score = signals.former_score.saturating_add(1);
            signals.generic_binder_count = signals.generic_binder_count.saturating_add(1);
            if !codomain.var_refs().is_empty() {
                signals.dependent_motive_density =
                    signals.dependent_motive_density.saturating_add(1);
            }
            accumulate_expr_signals(domain, signals);
            accumulate_expr_signals(codomain, signals);
        }
        Expr::Id(ty, left, right) => {
            accumulate_expr_signals(ty, signals);
            accumulate_expr_signals(left, signals);
            accumulate_expr_signals(right, signals);
        }
        Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body) => accumulate_expr_signals(body, signals),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate_candidate;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::telescope::Telescope;
    use pen_eval::bar::DiscoveryRecord;

    #[test]
    fn candidate_evaluation_is_structural_and_exact() {
        let mut library: Library = Vec::new();
        let mut history = Vec::new();
        for step in 1..=3 {
            let telescope = Telescope::reference(step);
            let evaluated = evaluate_candidate(&library, &history, telescope.clone())
                .expect("reference telescope should evaluate");
            history.push(DiscoveryRecord::new(
                step,
                u32::from(evaluated.nu),
                u32::from(evaluated.clause_kappa),
            ));
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }

        let pi = evaluate_candidate(&library, &history, Telescope::reference(4))
            .expect("pi telescope should evaluate");
        assert_eq!(pi.nu, 5);
        assert_eq!(pi.clause_kappa, 3);
        assert_eq!(pi.rho, pen_core::rational::Rational::new(5, 3));
        assert_eq!(
            pi.telescope_class,
            pen_core::telescope::TelescopeClass::Former
        );
        assert!(pi.signals.eliminator_score > 0);
        assert!(pi.signals.former_score > 0);
        assert!(pi.trace.iter().any(|line| line == "nu_total=5"));
    }
}
