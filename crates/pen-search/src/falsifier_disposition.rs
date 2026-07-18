//! Phase 5a of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`: full typed-family
//! dispositions for adversarial candidates.
//!
//! For each candidate the disposition records, with kernel derivations:
//! - the typed family inventory (Phase 2 extraction: total families,
//!   marginal families with generator clauses and closure evidence);
//! - every token attempt (H-form typed eliminator, naturality squares,
//!   P5 typed lift) with the EXACT failure point named by the kernel
//!   (no formation clause; no oriented basis; stuck fresh head /
//!   untypable lift; no reachability-dominant import) — a failed token
//!   never becomes bounded opaque credit;
//! - the EGP anchor disposition (Phase 3 classifier): each remaining
//!   marginal family either occupies a valid local-role slot or the
//!   candidate stays outside debt-free territory.
//!
//! Everything is re-derivable: the disposition embeds the extraction and
//! classification derivation hashes, and rebuilding it from the sealed
//! signature reproduces it byte for byte.

use pen_core::telescope::Telescope;
use pen_eval::demand_orbits::KernelOrbitExtraction;
use pen_eval::egp::{assign_mechanism, classify_candidate};
use pen_eval::typed_families::{
    CandidateExtractionOutcome, PredecessorClosure, extract_candidate_families,
};
use pen_core::expr::Expr;
use pen_type::elaborate::{
    SealedSignature, TokenError, issue_naturality_token, issue_typed_eliminator_token,
    issue_typed_lift_token,
};
use serde::Serialize;

/// One token attempt with its kernel outcome label.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TokenAttempt {
    pub token: String,
    pub outcome: String,
    pub detail: String,
}

/// Summary of one marginal family and its anchor content.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MarginalFamilySummary {
    pub family_id: String,
    pub generator_clause: u16,
    pub mechanism: String,
    pub local_role: String,
}

/// The full Phase 5a typed disposition of one candidate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SemanticFalsifierDisposition {
    pub subject_hash: String,
    /// `Some` when the kernel rejected the candidate outright.
    pub kernel_invalid: Option<String>,
    pub families_total: usize,
    pub marginal_families: Vec<MarginalFamilySummary>,
    /// The token the candidate's structural mechanism requires
    /// (H-form eliminator for Hit/Synthesis shapes, P5 lift for
    /// Axiomatic shapes).
    pub required_token: String,
    pub token_attempts: Vec<TokenAttempt>,
    /// The classifier verdict over the kernel-backed submission.
    pub egp_marginal_nu: Option<u32>,
    pub egp_local_capacity: Option<u32>,
    pub egp_debt_free_bound_holds: Option<bool>,
    pub egp_rejection: Option<String>,
    pub extraction_derivation_hash: Option<String>,
}

fn token_error_label(error: &TokenError) -> (String, String) {
    let label = match error {
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
    };
    (label.to_string(), error.to_string())
}

/// Build the full typed disposition for one candidate at Step 16.
pub fn build_semantic_disposition(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    orbits: &KernelOrbitExtraction,
    telescope: &Telescope,
) -> SemanticFalsifierDisposition {
    let subject_hash = pen_type::elaborate::candidate_hash(telescope);
    let visible_library = signature.len() as u32;

    // Token attempts: the eliminator and lift attempts always run; a
    // naturality attempt runs per Pi clause.
    let mut token_attempts = Vec::new();
    match issue_typed_eliminator_token(signature, telescope, visible_library) {
        Ok(token) => token_attempts.push(TokenAttempt {
            token: "typed_eliminator".to_string(),
            outcome: "ok".to_string(),
            detail: format!(
                "formation clause {}, eliminator clause {}",
                token.formation_clause(),
                token.eliminator_clause()
            ),
        }),
        Err(error) => {
            let (outcome, detail) = token_error_label(&error);
            token_attempts.push(TokenAttempt {
                token: "typed_eliminator".to_string(),
                outcome,
                detail,
            });
        }
    }
    for (index, clause) in telescope.clauses.iter().enumerate() {
        if matches!(clause.expr, Expr::Pi(_, _)) {
            let clause_index = index as u16;
            match issue_naturality_token(signature, telescope, visible_library, clause_index) {
                Ok(token) => token_attempts.push(TokenAttempt {
                    token: format!("naturality[{clause_index}]"),
                    outcome: "ok".to_string(),
                    detail: format!(
                        "{} / {} square",
                        token.outer_operator(),
                        token.inner_operator()
                    ),
                }),
                Err(error) => {
                    let (outcome, detail) = token_error_label(&error);
                    token_attempts.push(TokenAttempt {
                        token: format!("naturality[{clause_index}]"),
                        outcome,
                        detail,
                    });
                }
            }
        }
    }
    match issue_typed_lift_token(signature, telescope, visible_library) {
        Ok(token) => token_attempts.push(TokenAttempt {
            token: "typed_lift".to_string(),
            outcome: "ok".to_string(),
            detail: format!("dominant import {}", token.dominant_import()),
        }),
        Err(error) => {
            let (outcome, detail) = token_error_label(&error);
            token_attempts.push(TokenAttempt {
                token: "typed_lift".to_string(),
                outcome,
                detail,
            });
        }
    }

    // The structurally required token: Axiomatic shapes (library-referencing,
    // non-modal) require the P5 lift; everything else requires the H-form
    // typed eliminator.
    let required_token = if telescope.lib_refs().is_empty() {
        "typed_eliminator".to_string()
    } else {
        "typed_lift".to_string()
    };

    // Phase 2 extraction and Phase 3 classification.
    let outcome = extract_candidate_families(signature, closure, telescope, visible_library);
    match outcome {
        CandidateExtractionOutcome::KernelInvalid { failure } => SemanticFalsifierDisposition {
            subject_hash,
            kernel_invalid: Some(failure.to_string()),
            families_total: 0,
            marginal_families: Vec::new(),
            required_token,
            token_attempts,
            egp_marginal_nu: None,
            egp_local_capacity: None,
            egp_debt_free_bound_holds: None,
            egp_rejection: None,
            extraction_derivation_hash: None,
        },
        CandidateExtractionOutcome::Extracted(extraction) => {
            let marginal_families = extraction
                .marginal_families()
                .map(|family| {
                    let generator_clause = family
                        .instances
                        .first()
                        .map(|instance| instance.clause_index)
                        .unwrap_or(0);
                    let mechanism = assign_mechanism(family);
                    MarginalFamilySummary {
                        family_id: family.id.as_str().to_string(),
                        generator_clause,
                        mechanism: format!("{mechanism:?}"),
                        local_role: format!("{:?}", mechanism.required_local_role()),
                    }
                })
                .collect();
            let kappa = telescope.kappa() as u16;
            let (egp_marginal_nu, egp_local_capacity, egp_debt_free_bound_holds, egp_rejection) =
                match classify_candidate(&extraction, orbits, 16, kappa) {
                    Ok(disposition) => (
                        Some(disposition.bound.marginal_nu),
                        Some(disposition.bound.local_capacity),
                        Some(disposition.bound.conditional_debt_free_linear_bound_holds),
                        None,
                    ),
                    Err(error) => (None, None, None, Some(error.to_string())),
                };
            SemanticFalsifierDisposition {
                subject_hash,
                kernel_invalid: None,
                families_total: extraction.families.len(),
                marginal_families,
                required_token,
                token_attempts,
                egp_marginal_nu,
                egp_local_capacity,
                egp_debt_free_bound_holds,
                egp_rejection,
                extraction_derivation_hash: Some(extraction.derivation_hash.clone()),
            }
        }
    }
}

impl SemanticFalsifierDisposition {
    pub fn required_token_outcome(&self) -> Option<&str> {
        self.token_attempts
            .iter()
            .find(|attempt| attempt.token == self.required_token)
            .map(|attempt| attempt.outcome.as_str())
    }
}
