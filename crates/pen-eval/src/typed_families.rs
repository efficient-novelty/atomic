//! Phase 2 of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`: typed semantic
//! families and canonical representatives.
//!
//! The extractor is kernel-backed end to end: every clause of a candidate
//! is elaborated and normalized by the Phase 1 kernel
//! (`pen_type::elaborate`), its normal form is abstracted over the
//! parameters it actually uses (a checked weakening/renaming map, recorded
//! for replay), and the canonical presentation is identified only through
//! the frozen univalent-equality decision procedure, whose witness is
//! retained. `NaturalFamilyId` has a private field and no deserializer:
//! a counted family cannot be manufactured from a string, token name, or
//! support-local bucket — the only mint is this extractor (family validity
//! as a dependent kernel judgment).
//!
//! Operational definitions (kernel v1, frozen):
//! - The parameter telescope of a clause family is the ordered list of
//!   free scope references (ambient parameters and prior fields) its
//!   normal form actually uses, renumbered by first use; unused scope
//!   entries are dropped (that IS the checked weakening quotient).
//! - Two presentations are the same family iff their canonical normal
//!   forms and parameter sorts agree under the frozen univalent equality.
//!   Canonicalization is invariant under bijective renamings of the free
//!   scope, and the recorded naturality derivation checks that square for
//!   the generating transposition.
//! - A uniform specialization (first-order instance) of a family is a
//!   member of that family, not additional credit.
//! - Marginality is decided against the typed predecessor closure: the
//!   families presented by the fifteen sealed entries themselves.
//!   Identical or instance-derivable families are internal with zero
//!   marginal credit; only the rest are marginal and proceed to the
//!   Phase 3 provenance classifier.

use crate::semantic_provenance::{
    SchemaFamilyId, SemanticAssumptionRef, TypedNormalFamily, TypedNormalJudgement,
    UnivalentClassId,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{
    ClauseFailure, KernelTy, SealedSignature, TelescopeElaboration, elaborate_telescope,
};
use pen_type::equality::{EqualityWitness, KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use pen_type::normalize::KERNEL_BINDING_CONVENTION;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

/// Canonical identifier of a natural family. Private field, no
/// deserializer: minted only by this extractor.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NaturalFamilyId {
    hash: String,
}

impl NaturalFamilyId {
    pub fn as_str(&self) -> &str {
        &self.hash
    }
}

/// The sort of one abstracted parameter.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ParamSort {
    /// An ambient type parameter, or a field reference to a formation
    /// clause (both enter the family telescope as type parameters).
    Type,
    /// A field reference to a non-formation clause: an opaque parameter.
    Opaque,
}

/// The checked renaming/weakening map from the clause's scope onto the
/// family's canonical parameter telescope. Replayable: `forward[i] =
/// (old_level, canonical_index)`; scope entries absent from `forward`
/// were weakened away.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RenamingMap {
    pub free_scope_len: u32,
    pub forward: Vec<(u32, u32)>,
}

/// The canonical presentation of one clause as a natural family.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CanonicalPresentation {
    pub canonical_normal_form: Expr,
    pub parameters: Vec<ParamSort>,
    pub renaming: RenamingMap,
}

/// Kernel-checked naturality: canonicalization is stable under bijective
/// renamings of the parameter telescope; the recorded square checks the
/// generating transposition (identity when arity < 2) through the frozen
/// univalent equality.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NaturalityDerivation {
    pub renaming_checked: Vec<(u32, u32)>,
    pub square: EqualityWitness,
}

/// How a clause relates to its family.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum InstanceKind {
    /// The clause is the family's canonical generator.
    Generator,
    /// The clause presents the identical canonical family again.
    RenamingInstance,
    /// The clause is a proper first-order specialization; the recorded
    /// substitution maps canonical parameters to subexpressions.
    Specialization { substitution: Vec<(u32, Expr)> },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InstanceRecord {
    pub clause_index: u16,
    pub kind: InstanceKind,
}

/// Marginality disposition against the typed predecessor closure.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum MarginalityDisposition {
    /// The identical canonical family is already presented by a sealed
    /// entry: internal, zero marginal credit.
    InternalIdentical {
        predecessor_step: u32,
        predecessor_clause: u16,
        equality: EqualityWitness,
    },
    /// The family is a first-order instance of a sealed family:
    /// derivable/inherited, internal, zero marginal credit.
    InternalDerivable {
        predecessor_step: u32,
        predecessor_clause: u16,
        substitution: Vec<(u32, Expr)>,
    },
    /// No preimage in the typed predecessor closure: the family is
    /// marginal and must obtain a valid EGP anchor in Phase 3.
    MarginalNoClosurePreimage { closure_digest: String },
}

impl MarginalityDisposition {
    pub fn is_marginal(&self) -> bool {
        matches!(self, Self::MarginalNoClosurePreimage { .. })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExtractedFamily {
    pub id: NaturalFamilyId,
    pub presentation: CanonicalPresentation,
    pub naturality: NaturalityDerivation,
    pub generator_kernel_ty: KernelTy,
    pub generator_role: ClauseRole,
    pub instances: Vec<InstanceRecord>,
    pub marginality: MarginalityDisposition,
}

/// Total extraction outcome for one candidate. Kernel-invalid candidates
/// receive an explicit disposition rather than an error path.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum CandidateExtractionOutcome {
    Extracted(CandidateFamilyExtraction),
    KernelInvalid { failure: ClauseFailure },
}

impl CandidateExtractionOutcome {
    pub fn extraction(&self) -> Option<&CandidateFamilyExtraction> {
        match self {
            Self::Extracted(extraction) => Some(extraction),
            Self::KernelInvalid { .. } => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CandidateFamilyExtraction {
    pub subject_hash: String,
    pub signature_digest: String,
    pub closure_digest: String,
    pub families: Vec<ExtractedFamily>,
    pub marginal_family_count: usize,
    pub derivation_hash: String,
}

impl CandidateFamilyExtraction {
    pub fn marginal_families(&self) -> impl Iterator<Item = &ExtractedFamily> {
        self.families
            .iter()
            .filter(|family| family.marginality.is_marginal())
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum FamilyExtractionError {
    #[error("predecessor closure entry failed to elaborate: {0}")]
    ClosureEntryInvalid(ClauseFailure),
}

/// One family of the typed predecessor closure.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClosureFamily {
    pub step: u32,
    pub clause_index: u16,
    pub id: NaturalFamilyId,
    pub presentation: CanonicalPresentation,
}

/// The typed predecessor closure: every family presented by the sealed
/// entries themselves, extracted by the same kernel pipeline.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PredecessorClosure {
    pub signature_digest: String,
    pub families: Vec<ClosureFamily>,
    pub digest: String,
}

impl PredecessorClosure {
    pub fn find_identical(&self, id: &NaturalFamilyId) -> Option<&ClosureFamily> {
        self.families.iter().find(|family| &family.id == id)
    }
}

fn family_id(
    signature: &SealedSignature,
    presentation: &CanonicalPresentation,
) -> NaturalFamilyId {
    let payload = serde_json::json!({
        "binding": KERNEL_BINDING_CONVENTION,
        "equality": KERNEL_EQUALITY_PROCEDURE,
        "signature_digest": signature.digest(),
        "canonical_normal_form": presentation.canonical_normal_form,
        "parameters": presentation.parameters,
    });
    NaturalFamilyId {
        hash: format!(
            "blake3:{}",
            blake3_hex(&serde_json::to_vec(&payload).expect("family id serialization"))
        ),
    }
}

/// Canonicalize a clause normal form over its free scope: parameters are
/// the free levels in first-use order; unused scope entries weaken away;
/// binder levels shift down onto the compacted telescope.
fn canonicalize(
    normal_form: &Expr,
    free_scope_len: u32,
    prior_roles: &[ClauseRole],
    ambient: u32,
) -> CanonicalPresentation {
    let mut order: Vec<u32> = Vec::new();
    collect_free_levels(normal_form, free_scope_len, &mut order);
    let forward: Vec<(u32, u32)> = order
        .iter()
        .enumerate()
        .map(|(index, level)| (*level, index as u32 + 1))
        .collect();
    let map: BTreeMap<u32, u32> = forward.iter().copied().collect();
    let params_used = forward.len() as u32;
    let canonical_normal_form =
        renumber_levels(normal_form, free_scope_len, &map, params_used);
    let parameters = order
        .iter()
        .map(|level| {
            if *level <= ambient {
                ParamSort::Type
            } else {
                let clause = (*level - ambient - 1) as usize;
                match prior_roles.get(clause) {
                    Some(ClauseRole::Formation) => ParamSort::Type,
                    _ => ParamSort::Opaque,
                }
            }
        })
        .collect();
    CanonicalPresentation {
        canonical_normal_form,
        parameters,
        renaming: RenamingMap {
            free_scope_len,
            forward,
        },
    }
}

fn collect_free_levels(expr: &Expr, free_scope_len: u32, order: &mut Vec<u32>) {
    match expr {
        Expr::Var(level) => {
            if *level <= free_scope_len && !order.contains(level) {
                order.push(*level);
            }
        }
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => {}
        Expr::App(a, b) | Expr::Pi(a, b) | Expr::Sigma(a, b) => {
            collect_free_levels(a, free_scope_len, order);
            collect_free_levels(b, free_scope_len, order);
        }
        Expr::Id(a, b, c) => {
            collect_free_levels(a, free_scope_len, order);
            collect_free_levels(b, free_scope_len, order);
            collect_free_levels(c, free_scope_len, order);
        }
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => collect_free_levels(inner, free_scope_len, order),
    }
}

fn renumber_levels(
    expr: &Expr,
    free_scope_len: u32,
    map: &BTreeMap<u32, u32>,
    params_used: u32,
) -> Expr {
    let rebuild = |inner: &Expr| renumber_levels(inner, free_scope_len, map, params_used);
    match expr {
        Expr::Var(level) => {
            if *level <= free_scope_len {
                Expr::Var(*map.get(level).expect("used level is in the renaming map"))
            } else {
                Expr::Var(params_used + (*level - free_scope_len))
            }
        }
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(a, b) => Expr::App(Box::new(rebuild(a)), Box::new(rebuild(b))),
        Expr::Pi(a, b) => Expr::Pi(Box::new(rebuild(a)), Box::new(rebuild(b))),
        Expr::Sigma(a, b) => Expr::Sigma(Box::new(rebuild(a)), Box::new(rebuild(b))),
        Expr::Id(a, b, c) => Expr::Id(
            Box::new(rebuild(a)),
            Box::new(rebuild(b)),
            Box::new(rebuild(c)),
        ),
        Expr::Lam(inner) => Expr::Lam(Box::new(rebuild(inner))),
        Expr::Refl(inner) => Expr::Refl(Box::new(rebuild(inner))),
        Expr::Susp(inner) => Expr::Susp(Box::new(rebuild(inner))),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(rebuild(inner))),
        Expr::Flat(inner) => Expr::Flat(Box::new(rebuild(inner))),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(rebuild(inner))),
        Expr::Disc(inner) => Expr::Disc(Box::new(rebuild(inner))),
        Expr::Shape(inner) => Expr::Shape(Box::new(rebuild(inner))),
        Expr::Next(inner) => Expr::Next(Box::new(rebuild(inner))),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(rebuild(inner))),
        Expr::Bang(inner) => Expr::Bang(Box::new(rebuild(inner))),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(rebuild(inner))),
    }
}

/// Apply a parameter transposition to a canonical normal form.
fn transpose_params(expr: &Expr, first: u32, second: u32) -> Expr {
    let rebuild = |inner: &Expr| transpose_params(inner, first, second);
    match expr {
        Expr::Var(level) => {
            if *level == first {
                Expr::Var(second)
            } else if *level == second {
                Expr::Var(first)
            } else {
                Expr::Var(*level)
            }
        }
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(a, b) => Expr::App(Box::new(rebuild(a)), Box::new(rebuild(b))),
        Expr::Pi(a, b) => Expr::Pi(Box::new(rebuild(a)), Box::new(rebuild(b))),
        Expr::Sigma(a, b) => Expr::Sigma(Box::new(rebuild(a)), Box::new(rebuild(b))),
        Expr::Id(a, b, c) => Expr::Id(
            Box::new(rebuild(a)),
            Box::new(rebuild(b)),
            Box::new(rebuild(c)),
        ),
        Expr::Lam(inner) => Expr::Lam(Box::new(rebuild(inner))),
        Expr::Refl(inner) => Expr::Refl(Box::new(rebuild(inner))),
        Expr::Susp(inner) => Expr::Susp(Box::new(rebuild(inner))),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(rebuild(inner))),
        Expr::Flat(inner) => Expr::Flat(Box::new(rebuild(inner))),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(rebuild(inner))),
        Expr::Disc(inner) => Expr::Disc(Box::new(rebuild(inner))),
        Expr::Shape(inner) => Expr::Shape(Box::new(rebuild(inner))),
        Expr::Next(inner) => Expr::Next(Box::new(rebuild(inner))),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(rebuild(inner))),
        Expr::Bang(inner) => Expr::Bang(Box::new(rebuild(inner))),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(rebuild(inner))),
    }
}

fn naturality_derivation(
    presentation: &CanonicalPresentation,
) -> Result<NaturalityDerivation, pen_type::normalize::NormalizeError> {
    let params = presentation.parameters.len() as u32;
    let scope_len = params;
    let fuel = 64;
    if params < 2 {
        let square = univalent_equality(
            &presentation.canonical_normal_form,
            &presentation.canonical_normal_form,
            scope_len,
            fuel,
        )?;
        return Ok(NaturalityDerivation {
            renaming_checked: vec![],
            square,
        });
    }
    // Transpose the first two parameters and re-canonicalize: the square
    // closes iff the result is the same canonical presentation.
    let transposed = transpose_params(&presentation.canonical_normal_form, 1, 2);
    let recanonical = canonicalize(&transposed, params, &[], params);
    let square = univalent_equality(
        &presentation.canonical_normal_form,
        &recanonical.canonical_normal_form,
        scope_len,
        fuel,
    )?;
    Ok(NaturalityDerivation {
        renaming_checked: vec![(1, 2), (2, 1)],
        square,
    })
}

/// First-order matching of a family pattern (canonical parameters are
/// metavariables; binder levels are rigid) against a target canonical
/// normal form. Returns the substitution on success.
fn match_instance(
    pattern: &Expr,
    pattern_params: u32,
    target: &Expr,
    target_params: u32,
    bindings: &mut BTreeMap<u32, Expr>,
) -> bool {
    match (pattern, target) {
        (Expr::Var(level), _) if *level <= pattern_params => {
            if let Some(bound) = bindings.get(level) {
                bound == target
            } else if !is_parameter_scope_expr(target, target_params) {
                // The candidate subexpression references a TARGET binder
                // level: binding it would capture — the recorded
                // substitution could not replay in the pattern's scope
                // and a genuinely distinct dependent family would be
                // merged away. Fail closed.
                false
            } else {
                bindings.insert(*level, target.clone());
                true
            }
        }
        (Expr::Var(pattern_level), Expr::Var(target_level)) => {
            // Rigid binder levels must sit at the same depth above their
            // respective parameter telescopes; a free target parameter
            // (level <= target_params) can never match a rigid binder.
            *target_level > target_params
                && *pattern_level - pattern_params == *target_level - target_params
        }
        (Expr::Univ, Expr::Univ) => true,
        (Expr::Lib(a), Expr::Lib(b)) => a == b,
        (Expr::PathCon(a), Expr::PathCon(b)) => a == b,
        (Expr::App(a1, b1), Expr::App(a2, b2))
        | (Expr::Pi(a1, b1), Expr::Pi(a2, b2))
        | (Expr::Sigma(a1, b1), Expr::Sigma(a2, b2)) => {
            match_instance(a1, pattern_params, a2, target_params, bindings)
                && match_instance(b1, pattern_params, b2, target_params, bindings)
        }
        (Expr::Id(a1, b1, c1), Expr::Id(a2, b2, c2)) => {
            match_instance(a1, pattern_params, a2, target_params, bindings)
                && match_instance(b1, pattern_params, b2, target_params, bindings)
                && match_instance(c1, pattern_params, c2, target_params, bindings)
        }
        (Expr::Lam(a), Expr::Lam(b))
        | (Expr::Refl(a), Expr::Refl(b))
        | (Expr::Susp(a), Expr::Susp(b))
        | (Expr::Trunc(a), Expr::Trunc(b))
        | (Expr::Flat(a), Expr::Flat(b))
        | (Expr::Sharp(a), Expr::Sharp(b))
        | (Expr::Disc(a), Expr::Disc(b))
        | (Expr::Shape(a), Expr::Shape(b))
        | (Expr::Next(a), Expr::Next(b))
        | (Expr::Eventually(a), Expr::Eventually(b))
        | (Expr::Bang(a), Expr::Bang(b))
        | (Expr::WhyNot(a), Expr::WhyNot(b)) => {
            match_instance(a, pattern_params, b, target_params, bindings)
        }
        _ => false,
    }
}

/// A substitution is a RENAMING only when it is variable-for-variable and
/// injective: a diagonal map (two parameters sent to one variable) is a
/// proper uniform specialization and must be subsumed into its family,
/// not minted as separate credit.
fn is_renaming_substitution(bindings: &BTreeMap<u32, Expr>) -> bool {
    let mut seen = BTreeSet::new();
    bindings.values().all(|value| match value {
        Expr::Var(level) => seen.insert(*level),
        _ => false,
    })
}

/// True iff every free reference in `expr` stays within the target's
/// parameter telescope (no target binder levels escape into a binding).
fn is_parameter_scope_expr(expr: &Expr, target_params: u32) -> bool {
    match expr {
        Expr::Var(level) => *level <= target_params,
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => true,
        Expr::App(a, b) | Expr::Pi(a, b) | Expr::Sigma(a, b) => {
            is_parameter_scope_expr(a, target_params) && is_parameter_scope_expr(b, target_params)
        }
        Expr::Id(a, b, c) => {
            is_parameter_scope_expr(a, target_params)
                && is_parameter_scope_expr(b, target_params)
                && is_parameter_scope_expr(c, target_params)
        }
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => is_parameter_scope_expr(inner, target_params),
    }
}

/// A family pattern participates in instance subsumption only when it has
/// rigid structure. A bare-parameter pattern (canonical normal form
/// `Var(1)` — e.g. the beta-collapse clause of sealed step 4) would match
/// every expression and make marginality vacuous; it may still be matched
/// IDENTICALLY, but it subsumes nothing.
fn pattern_is_rigid(pattern: &Expr) -> bool {
    !matches!(pattern, Expr::Var(_))
}

/// Extract the typed predecessor closure from the sealed signature.
pub fn predecessor_closure(
    signature: &SealedSignature,
) -> Result<PredecessorClosure, FamilyExtractionError> {
    let mut families = Vec::new();
    for entry in signature.entries() {
        let elaboration =
            elaborate_telescope(signature, &entry.telescope, entry.step.saturating_sub(1))
                .map_err(FamilyExtractionError::ClosureEntryInvalid)?;
        // Kernel-derived roles only: caller-declared role bytes must not
        // influence parameter sorts (fail-open hazard).
        let prior_roles: Vec<ClauseRole> = elaboration
            .clauses
            .iter()
            .map(|clause| clause.kernel_role)
            .collect();
        for clause in &elaboration.clauses {
            let free_scope_len =
                elaboration.ambient_parameters + u32::from(clause.clause_index);
            let presentation = canonicalize(
                &clause.normal_form,
                free_scope_len,
                &prior_roles[..clause.clause_index as usize],
                elaboration.ambient_parameters,
            );
            let id = family_id(signature, &presentation);
            if families.iter().all(|existing: &ClosureFamily| existing.id != id) {
                families.push(ClosureFamily {
                    step: entry.step,
                    clause_index: clause.clause_index,
                    id,
                    presentation,
                });
            }
        }
    }
    let digest_payload = serde_json::json!({
        "signature_digest": signature.digest(),
        "family_ids": families.iter().map(|family| family.id.as_str()).collect::<Vec<_>>(),
    });
    let digest = format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&digest_payload).expect("closure digest serialization"))
    );
    Ok(PredecessorClosure {
        signature_digest: signature.digest().to_string(),
        families,
        digest,
    })
}

/// Total, kernel-backed family extraction for one candidate telescope.
pub fn extract_candidate_families(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    telescope: &Telescope,
    visible_library: u32,
) -> CandidateExtractionOutcome {
    let elaboration = match elaborate_telescope(signature, telescope, visible_library) {
        Ok(elaboration) => elaboration,
        Err(failure) => return CandidateExtractionOutcome::KernelInvalid { failure },
    };
    CandidateExtractionOutcome::Extracted(extract_from_elaboration(
        signature,
        closure,
        telescope,
        &elaboration,
    ))
}

fn extract_from_elaboration(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    _telescope: &Telescope,
    elaboration: &TelescopeElaboration,
) -> CandidateFamilyExtraction {
    // Kernel-derived roles only (see predecessor_closure).
    let prior_roles: Vec<ClauseRole> = elaboration
        .clauses
        .iter()
        .map(|clause| clause.kernel_role)
        .collect();
    let mut families: Vec<ExtractedFamily> = Vec::new();

    for clause in &elaboration.clauses {
        let free_scope_len = elaboration.ambient_parameters + u32::from(clause.clause_index);
        let presentation = canonicalize(
            &clause.normal_form,
            free_scope_len,
            &prior_roles[..clause.clause_index as usize],
            elaboration.ambient_parameters,
        );
        let id = family_id(signature, &presentation);

        // Identical family already extracted from this candidate: the
        // clause is a renaming instance, not additional credit.
        if let Some(existing) = families.iter_mut().find(|family| family.id == id) {
            existing.instances.push(InstanceRecord {
                clause_index: clause.clause_index,
                kind: InstanceKind::RenamingInstance,
            });
            continue;
        }

        // Proper first-order specialization of an earlier candidate
        // family: a member of that family, not additional credit.
        let mut specialized = None;
        for (index, family) in families.iter().enumerate() {
            if !pattern_is_rigid(&family.presentation.canonical_normal_form) {
                continue;
            }
            let mut bindings = BTreeMap::new();
            if match_instance(
                &family.presentation.canonical_normal_form,
                family.presentation.parameters.len() as u32,
                &presentation.canonical_normal_form,
                presentation.parameters.len() as u32,
                &mut bindings,
            ) && !is_renaming_substitution(&bindings)
            {
                specialized = Some((index, bindings));
                break;
            }
        }
        if let Some((index, bindings)) = specialized {
            families[index].instances.push(InstanceRecord {
                clause_index: clause.clause_index,
                kind: InstanceKind::Specialization {
                    substitution: bindings.into_iter().collect(),
                },
            });
            continue;
        }

        // New family: decide marginality against the typed predecessor
        // closure through the frozen univalent equality.
        let marginality = decide_marginality(closure, &id, &presentation);
        let naturality = match naturality_derivation(&presentation) {
            Ok(naturality) => naturality,
            Err(_) => {
                // Fuel exhaustion during the naturality square: the
                // family is unusable; record it as marginal-unanchored
                // so nothing downstream can credit it silently.
                NaturalityDerivation {
                    renaming_checked: vec![],
                    square: EqualityWitness {
                        scope_len: presentation.parameters.len() as u32,
                        left_normal_form: presentation.canonical_normal_form.clone(),
                        right_normal_form: presentation.canonical_normal_form.clone(),
                        left_steps: 0,
                        right_steps: 0,
                        equal: false,
                    },
                }
            }
        };
        families.push(ExtractedFamily {
            id,
            presentation,
            naturality,
            generator_kernel_ty: clause.kernel_ty.clone(),
            generator_role: clause.kernel_role,
            instances: vec![InstanceRecord {
                clause_index: clause.clause_index,
                kind: InstanceKind::Generator,
            }],
            marginality,
        });
    }

    let marginal_family_count = families
        .iter()
        .filter(|family| family.marginality.is_marginal())
        .count();
    let derivation_payload = serde_json::json!({
        "binding": KERNEL_BINDING_CONVENTION,
        "equality": KERNEL_EQUALITY_PROCEDURE,
        "subject_hash": elaboration.subject_hash,
        "signature_digest": signature.digest(),
        "closure_digest": closure.digest,
        "elaboration_derivation": elaboration.derivation_hash,
        "families": families,
    });
    let derivation_hash = format!(
        "blake3:{}",
        blake3_hex(
            &serde_json::to_vec(&derivation_payload).expect("extraction derivation serialization"),
        )
    );
    CandidateFamilyExtraction {
        subject_hash: elaboration.subject_hash.clone(),
        signature_digest: signature.digest().to_string(),
        closure_digest: closure.digest.clone(),
        families,
        marginal_family_count,
        derivation_hash,
    }
}

fn decide_marginality(
    closure: &PredecessorClosure,
    id: &NaturalFamilyId,
    presentation: &CanonicalPresentation,
) -> MarginalityDisposition {
    if let Some(predecessor) = closure.find_identical(id) {
        let scope_len = presentation.parameters.len() as u32;
        let equality = univalent_equality(
            &presentation.canonical_normal_form,
            &predecessor.presentation.canonical_normal_form,
            scope_len,
            64,
        )
        .unwrap_or(EqualityWitness {
            scope_len,
            left_normal_form: presentation.canonical_normal_form.clone(),
            right_normal_form: predecessor.presentation.canonical_normal_form.clone(),
            left_steps: 0,
            right_steps: 0,
            equal: false,
        });
        if equality.equal {
            return MarginalityDisposition::InternalIdentical {
                predecessor_step: predecessor.step,
                predecessor_clause: predecessor.clause_index,
                equality,
            };
        }
    }
    for predecessor in &closure.families {
        if !pattern_is_rigid(&predecessor.presentation.canonical_normal_form) {
            continue;
        }
        let mut bindings = BTreeMap::new();
        if match_instance(
            &predecessor.presentation.canonical_normal_form,
            predecessor.presentation.parameters.len() as u32,
            &presentation.canonical_normal_form,
            presentation.parameters.len() as u32,
            &mut bindings,
        ) && !is_renaming_substitution(&bindings)
        {
            return MarginalityDisposition::InternalDerivable {
                predecessor_step: predecessor.step,
                predecessor_clause: predecessor.clause_index,
                substitution: bindings.into_iter().collect(),
            };
        }
    }
    MarginalityDisposition::MarginalNoClosurePreimage {
        closure_digest: closure.digest.clone(),
    }
}

/// Bridge one extracted family into the Phase 3 provenance vocabulary
/// with kernel-verified evidence references.
pub fn to_typed_normal_family(
    extraction: &CandidateFamilyExtraction,
    family: &ExtractedFamily,
) -> TypedNormalFamily {
    let judgement = TypedNormalJudgement {
        context_normal_form: serde_json::to_string(&family.presentation.parameters)
            .expect("parameter serialization"),
        term_normal_form: serde_json::to_string(&family.presentation.canonical_normal_form)
            .expect("normal form serialization"),
        type_normal_form: serde_json::to_string(&family.generator_kernel_ty)
            .expect("kernel ty serialization"),
        normalization: SemanticAssumptionRef::verified(
            "kernel-v1 typed normalization",
            extraction.derivation_hash.clone(),
        ),
        type_preservation: SemanticAssumptionRef::verified(
            "kernel-v1 clause classifier",
            extraction.derivation_hash.clone(),
        ),
    };
    // The naturality derivation certifies renaming-stability of the
    // canonicalization quotient (its square is true by construction of
    // the quotient and recorded as a replayable witness). A square that
    // failed to close (e.g. fuel exhaustion) must NOT become Verified
    // evidence: it fails closed and the classifier rejects the family.
    let naturality = if family.naturality.square.equal {
        SemanticAssumptionRef::verified(
            "kernel-v1 renaming-stability of the canonicalization quotient",
            extraction.derivation_hash.clone(),
        )
    } else {
        SemanticAssumptionRef::missing(
            "kernel-v1 renaming-stability square failed to close for this family",
        )
    };
    TypedNormalFamily {
        id: SchemaFamilyId(family.id.as_str().to_string()),
        judgement,
        parameters: family
            .presentation
            .parameters
            .iter()
            .enumerate()
            .map(|(index, sort)| format!("p{}:{:?}", index + 1, sort))
            .collect(),
        naturality,
        univalent_class: UnivalentClassId(family.id.as_str().to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::ClauseRec;

    fn app(function: Expr, argument: Expr) -> Expr {
        Expr::App(Box::new(function), Box::new(argument))
    }

    fn pi(domain: Expr, codomain: Expr) -> Expr {
        Expr::Pi(Box::new(domain), Box::new(codomain))
    }

    fn sigma(domain: Expr, codomain: Expr) -> Expr {
        Expr::Sigma(Box::new(domain), Box::new(codomain))
    }

    fn primary_role_for_test(expr: &Expr) -> ClauseRole {
        match expr {
            Expr::Susp(_) => ClauseRole::Formation,
            Expr::Univ | Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Id(_, _, _) => {
                ClauseRole::Formation
            }
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => ClauseRole::Formation,
            Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => ClauseRole::Introduction,
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Lib(_)) => {
                ClauseRole::Introduction
            }
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Lam(_)) => {
                ClauseRole::Elimination
            }
            Expr::App(_, _) => ClauseRole::Introduction,
            Expr::PathCon(_) => ClauseRole::PathAttach,
            _ => ClauseRole::Formation,
        }
    }

    fn tel(exprs: Vec<Expr>) -> Telescope {
        Telescope::new(
            exprs
                .into_iter()
                .map(|expr| ClauseRec::new(primary_role_for_test(&expr), expr))
                .collect(),
        )
    }

    fn genesis() -> (SealedSignature, PredecessorClosure) {
        let signature = SealedSignature::genesis_del_h15();
        let closure = predecessor_closure(&signature).expect("closure");
        (signature, closure)
    }

    #[test]
    fn predecessor_closure_is_deterministic_and_kernel_backed() {
        let (signature, closure) = genesis();
        let again = predecessor_closure(&signature).expect("closure again");
        assert_eq!(closure, again);
        assert_eq!(closure.signature_digest, signature.digest());
        assert!(closure.digest.starts_with("blake3:"));
        // The sealed corpus presents a substantial but deduplicated
        // family inventory: strictly fewer families than clauses.
        let total_clauses: usize = signature
            .entries()
            .iter()
            .map(|entry| entry.telescope.clauses.len())
            .sum();
        assert!(closure.families.len() < total_clauses);
        assert!(closure.families.len() > 10);
    }

    #[test]
    fn canonicalization_is_invariant_under_ambient_renaming() {
        // Pi(A2, A1) and Pi(A1, A2) canonicalize to the same family:
        // first-use order quotients bijective renamings away.
        let left = canonicalize(&pi(Expr::Var(2), Expr::Var(1)), 2, &[], 2);
        let right = canonicalize(&pi(Expr::Var(1), Expr::Var(2)), 2, &[], 2);
        assert_eq!(left.canonical_normal_form, right.canonical_normal_form);
        assert_eq!(left.parameters, right.parameters);
        // The recorded renaming maps differ — that is the point: the
        // quotient is checked, not assumed.
        assert_ne!(left.renaming, right.renaming);
    }

    #[test]
    fn weakening_drops_unused_parameters_from_the_telescope() {
        // Next(A1) in a two-parameter ambient scope uses only A1: the
        // family has arity 1 and the weakening is recorded.
        let presentation = canonicalize(&Expr::Next(Box::new(Expr::Var(1))), 2, &[], 2);
        assert_eq!(presentation.parameters.len(), 1);
        assert_eq!(
            presentation.canonical_normal_form,
            Expr::Next(Box::new(Expr::Var(1)))
        );
        assert_eq!(presentation.renaming.forward, vec![(1, 1)]);
    }

    #[test]
    fn binder_levels_shift_onto_the_compacted_telescope() {
        // Pi(A2, <binder>) at free scope 2 where the codomain uses the
        // binder (level 3): after weakening A1 away the binder sits at
        // level 2.
        let presentation = canonicalize(&pi(Expr::Var(2), Expr::Var(3)), 2, &[], 2);
        assert_eq!(presentation.parameters.len(), 1);
        assert_eq!(
            presentation.canonical_normal_form,
            pi(Expr::Var(1), Expr::Var(2))
        );
    }

    #[test]
    fn naturality_square_closes_for_symmetric_and_asymmetric_families() {
        let symmetric = canonicalize(&sigma(Expr::Var(1), Expr::Var(2)), 2, &[], 2);
        let naturality = naturality_derivation(&symmetric).expect("naturality");
        assert!(naturality.square.equal);
        // Asymmetric family: Pi(Next(A1), A2). Transposing and
        // re-canonicalizing returns the same canonical form — the
        // quotient is renaming-stable by construction.
        let asymmetric = canonicalize(
            &pi(Expr::Next(Box::new(Expr::Var(1))), Expr::Var(2)),
            2,
            &[],
            2,
        );
        let naturality = naturality_derivation(&asymmetric).expect("naturality");
        assert!(naturality.square.equal);
    }

    #[test]
    fn identical_clauses_are_one_family_with_renaming_instances() {
        let (signature, closure) = genesis();
        // temporal_polymorphic_kappa2: two copies of Pi(Next A, Eventually A).
        let candidate = tel(vec![
            pi(
                Expr::Next(Box::new(Expr::Var(1))),
                Expr::Eventually(Box::new(Expr::Var(1))),
            ),
            pi(
                Expr::Next(Box::new(Expr::Var(1))),
                Expr::Eventually(Box::new(Expr::Var(1))),
            ),
        ]);
        let outcome = extract_candidate_families(&signature, &closure, &candidate, 15);
        let extraction = outcome.extraction().expect("extracts");
        assert_eq!(extraction.families.len(), 1);
        let family = &extraction.families[0];
        assert_eq!(family.instances.len(), 2);
        assert!(matches!(family.instances[0].kind, InstanceKind::Generator));
        assert!(matches!(
            family.instances[1].kind,
            InstanceKind::RenamingInstance
        ));
        // Pi(Next A, Eventually A) is exactly the DCT bridge clause
        // (step 15 clause 3): internal, zero marginal credit.
        assert!(matches!(
            family.marginality,
            MarginalityDisposition::InternalIdentical {
                predecessor_step: 15,
                predecessor_clause: 2,
                ..
            }
        ));
        assert_eq!(extraction.marginal_family_count, 0);
    }

    #[test]
    fn falsifier_family_dispositions_are_pinned() {
        let (signature, closure) = genesis();

        // hit_no_formation_d1: [PathCon(1), Var(1)]. The path-attach
        // family is identical to the sealed step-5 one (internal). But
        // under the minimal-ambient kernel reading the second clause's
        // Var(1) refers to the PathCon FIELD (an Opaque parameter), a
        // different typed family from the sealed "point of an ambient
        // type" (Type parameter): exactly one marginal family, which
        // Phase 3 must anchor or leave unanchored.
        let hit = tel(vec![Expr::PathCon(1), Expr::Var(1)]);
        let outcome = extract_candidate_families(&signature, &closure, &hit, 15);
        let extraction = outcome.extraction().expect("extracts");
        assert_eq!(extraction.families.len(), 2);
        assert_eq!(extraction.marginal_family_count, 1);
        let marginal = extraction.marginal_families().next().expect("one marginal");
        assert_eq!(marginal.presentation.canonical_normal_form, Expr::Var(1));
        assert_eq!(marginal.presentation.parameters, vec![ParamSort::Opaque]);

        // axiomatic_single_l15_kappa3: Pi(Lib 15, A) and App(Lib 15, A)
        // reference the DCT constant, which no sealed entry can mention:
        // genuinely marginal families. Sigma(A, A) is a fresh formation
        // shape not presented by any sealed clause.
        let single = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            app(Expr::Lib(15), Expr::Var(1)),
        ]);
        let outcome = extract_candidate_families(&signature, &closure, &single, 15);
        let extraction = outcome.extraction().expect("extracts");
        let marginal: Vec<_> = extraction.marginal_families().collect();
        assert_eq!(extraction.families.len(), 3);
        assert_eq!(marginal.len(), 3);

        // axiomatic_inheritance_kappa3: same shape against Lib 14.
        let inheritance = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            app(Expr::Lib(14), Expr::Var(1)),
        ]);
        let outcome = extract_candidate_families(&signature, &closure, &inheritance, 15);
        let extraction = outcome.extraction().expect("extracts");
        assert_eq!(extraction.marginal_family_count, 3);
    }

    #[test]
    fn kernel_invalid_candidates_get_explicit_dispositions() {
        let (signature, closure) = genesis();
        let overflow = tel(vec![Expr::Var(9)]);
        let outcome = extract_candidate_families(&signature, &closure, &overflow, 15);
        assert!(matches!(
            outcome,
            CandidateExtractionOutcome::KernelInvalid { .. }
        ));
    }

    #[test]
    fn family_ids_are_stable_and_reextraction_replays() {
        let (signature, closure) = genesis();
        let candidate = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
        ]);
        let first = extract_candidate_families(&signature, &closure, &candidate, 15);
        let second = extract_candidate_families(&signature, &closure, &candidate, 15);
        assert_eq!(first, second);
    }

    #[test]
    fn specializations_join_their_family_instead_of_minting_credit() {
        let (signature, closure) = genesis();
        // Clause 1: App(A1, A2) — the general application family (App
        // forces two genuine ambient parameters; a Pi/Sigma codomain
        // would read its Var as the binder under minimal ambient).
        // Clause 2: App(A1, Next(A1)) — a proper first-order
        // specialization: a member of the family, not new credit.
        let candidate = tel(vec![
            app(Expr::Var(1), Expr::Var(2)),
            app(Expr::Var(1), Expr::Next(Box::new(Expr::Var(1)))),
        ]);
        let outcome = extract_candidate_families(&signature, &closure, &candidate, 15);
        let extraction = outcome.extraction().expect("extracts");
        assert_eq!(extraction.families.len(), 1);
        assert!(matches!(
            extraction.families[0].instances[1].kind,
            InstanceKind::Specialization { .. }
        ));
    }

    #[test]
    fn typed_normal_family_bridge_carries_kernel_verified_evidence() {
        let (signature, closure) = genesis();
        let candidate = tel(vec![pi(Expr::Lib(15), Expr::Var(1))]);
        let outcome = extract_candidate_families(&signature, &closure, &candidate, 15);
        let extraction = outcome.extraction().expect("extracts");
        let family = &extraction.families[0];
        let bridged = to_typed_normal_family(extraction, family);
        assert!(bridged.assumptions_are_supplied());
        assert!(bridged.judgement.normalization.is_kernel_verified());
        assert!(bridged.naturality.is_kernel_verified());
        assert_eq!(bridged.univalent_class.0, family.id.as_str());
    }

    #[test]
    fn mutated_closure_digest_shows_up_in_marginal_dispositions() {
        let (signature, closure) = genesis();
        let candidate = tel(vec![pi(Expr::Lib(15), Expr::Var(1))]);
        let outcome = extract_candidate_families(&signature, &closure, &candidate, 15);
        let extraction = outcome.extraction().expect("extracts");
        let MarginalityDisposition::MarginalNoClosurePreimage { closure_digest } =
            &extraction.families[0].marginality
        else {
            panic!("expected marginal disposition");
        };
        assert_eq!(closure_digest, &closure.digest);

        // A closure built over a forged signature has a different digest;
        // extractions against it are distinguishable by construction.
        let mut telescopes = Telescope::all_reference_telescopes();
        telescopes[5].1.clauses[2] = ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(2));
        let forged_signature = SealedSignature::from_telescopes(telescopes);
        let forged_closure = predecessor_closure(&forged_signature).expect("closure");
        assert_ne!(forged_closure.digest, closure.digest);
    }

    #[test]
    fn dependent_families_are_never_capture_merged_into_nondependent_patterns() {
        // Adversarial-review regression (in-cone witness): clause 3's
        // canonical form Pi(A, Next(<binder>)) is a DEPENDENT family; a
        // capture-incoherent metavariable binding used to absorb it as a
        // "specialization" of the non-dependent Pi(A1, A2) family from
        // clause 2, silently removing a marginal family from the AtMost
        // accounting. It must be minted as its own marginal family.
        let (signature, closure) = genesis();
        let witness = tel(vec![
            sigma(Expr::Var(1), Expr::Var(1)),
            pi(Expr::Var(1), Expr::Var(2)),
            pi(Expr::Var(1), Expr::Next(Box::new(Expr::Var(4)))),
        ]);
        let outcome = extract_candidate_families(&signature, &closure, &witness, 15);
        let extraction = outcome.extraction().expect("extracts");
        assert_eq!(extraction.families.len(), 3, "no capture-merge");
        // And the recorded substitutions of any legitimate
        // specialization stay within the target's parameter scope.
        for family in &extraction.families {
            for instance in &family.instances {
                if let InstanceKind::Specialization { substitution } = &instance.kind {
                    for (_, bound) in substitution {
                        assert!(
                            bound
                                .var_refs()
                                .iter()
                                .all(|level| *level
                                    <= family.presentation.parameters.len() as u32),
                            "capture-incoherent substitution recorded"
                        );
                    }
                }
            }
        }
        // Diagonal specializations, by contrast, ARE members of their
        // family (injective-renaming discipline): App(A1, A2) subsumes
        // App(A1, A1).
        let diagonal = tel(vec![
            app(Expr::Var(1), Expr::Var(2)),
            app(Expr::Var(1), Expr::Var(1)),
        ]);
        let outcome = extract_candidate_families(&signature, &closure, &diagonal, 15);
        let extraction = outcome.extraction().expect("extracts");
        assert_eq!(extraction.families.len(), 1, "diagonal must be subsumed");
        assert!(matches!(
            extraction.families[0].instances[1].kind,
            InstanceKind::Specialization { .. }
        ));
    }

    #[test]
    fn mutations_move_family_ids_and_derivation_hashes() {
        let (signature, closure) = genesis();
        let baseline = extract_candidate_families(
            &signature,
            &closure,
            &tel(vec![pi(Expr::Lib(15), Expr::Var(1))]),
            15,
        );
        let baseline = baseline.extraction().expect("extracts");

        // Altered type (a different former): different family ID and
        // extraction derivation hash — a tampered certificate cannot
        // replay.
        let altered = extract_candidate_families(
            &signature,
            &closure,
            &tel(vec![sigma(Expr::Lib(15), Expr::Var(1))]),
            15,
        );
        let altered = altered.extraction().expect("extracts");
        assert_ne!(
            baseline.families[0].id, altered.families[0].id,
            "altered former must move the family id"
        );
        assert_ne!(baseline.derivation_hash, altered.derivation_hash);

        // Altered parameter sort distinguishes families even with equal
        // shape: Var-of-Type-param vs Var-of-Opaque-field (pinned in the
        // falsifier disposition test) — here, check IDs are sort-aware
        // via the closure: the sealed Var family (Type param) is NOT the
        // preimage of the hit falsifier's Var family (Opaque param).
        let hit = tel(vec![Expr::PathCon(1), Expr::Var(1)]);
        let outcome = extract_candidate_families(&signature, &closure, &hit, 15);
        let extraction = outcome.extraction().expect("extracts");
        let marginal = extraction.marginal_families().next().expect("marginal");
        assert!(closure.find_identical(&marginal.id).is_none());

        // NaturalFamilyId cannot be deserialized or constructed: the
        // only mint is extraction (compile-time property; asserted here
        // by re-extraction equality).
        let replay = extract_candidate_families(
            &signature,
            &closure,
            &tel(vec![pi(Expr::Lib(15), Expr::Var(1))]),
            15,
        );
        assert_eq!(replay.extraction().expect("extracts"), baseline);
    }

    #[test]
    fn property_samples_across_kappa_strata_agree_on_canonical_keys() {
        let (signature, closure) = genesis();
        // Deterministic sample surface: small in-cap clause pool over the
        // raw leaf domain and frozen operators.
        let pool: Vec<Expr> = vec![
            pi(Expr::Next(Box::new(Expr::Var(1))), Expr::Var(1)),
            pi(Expr::Var(1), Expr::Eventually(Box::new(Expr::Var(1)))),
            sigma(Expr::Var(1), Expr::Var(1)),
            app(Expr::Lib(15), Expr::Var(1)),
            app(Expr::Lib(14), Expr::Var(1)),
            Expr::Flat(Box::new(Expr::Var(1))),
            Expr::PathCon(1),
            Expr::Var(1),
        ];
        // kappa strata 2, 3, 4: sliding windows over the pool.
        for kappa in 2..=4usize {
            for start in 0..=(pool.len() - kappa) {
                let clauses: Vec<Expr> = pool[start..start + kappa].to_vec();
                let candidate = tel(clauses.clone());
                let outcome =
                    extract_candidate_families(&signature, &closure, &candidate, 15);
                let Some(extraction) = outcome.extraction() else {
                    continue;
                };
                // Total: every clause is accounted for by exactly one
                // instance record.
                let accounted: usize = extraction
                    .families
                    .iter()
                    .map(|family| family.instances.len())
                    .sum();
                assert_eq!(accounted, kappa, "kappa {kappa} window {start}");
                // Replays byte-identically.
                let again =
                    extract_candidate_families(&signature, &closure, &candidate, 15);
                assert_eq!(outcome, again);
            }
        }
    }
}
