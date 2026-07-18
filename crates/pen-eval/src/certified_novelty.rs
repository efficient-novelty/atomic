//! Proof-carrying, support-local novelty accounting.
//!
//! The structural evaluator in [`crate::nu`] is intentionally a fast closed-
//! form evaluator.  Its historical formulas contain three amplifications that
//! are not justified by the shallow MBTT AST alone:
//!
//! * a HIT without a formation clause inherits the size of the whole library;
//! * an `Axiomatic` telescope inherits the novelty of one referenced entry;
//! * a temporal polymorphism pattern is multiplied by the whole library.
//!
//! This module is a fail-closed layer for theorem work.  It never infers one
//! of those semantic witnesses from a class label.  A declaration is either
//! transparent, with exact old-library realizers, or opaque, with an explicit
//! fresh-kernel certificate and the frozen finite bases needed by its class.
//! Positive credit is local to the normalized support of the candidate.
//!
//! This is deliberately separate from `structural_nu`: callers can audit the
//! corrected theorem boundary without silently changing the historical
//! Genesis replay.

use crate::p5_record::{ImportDag, P5ImportAudit};
use pen_core::canonical::{canonical_key_expr, canonical_key_telescope};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_type::elaborate::{
    SealedSignature, TokenReplayError, TypedLiftToken, candidate_hash as kernel_candidate_hash,
    replay_typed_lift_token,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

/// Frozen grammar and support caps for a finite theorem lane.
///
/// These are inputs to the bound derivation, not conclusions selected from a
/// desired bar.  `genesis_step16` is the raw open-band surface audited by the
/// search crate: two to four clauses, latest-two imports, dimension one, and
/// six expression nodes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CertifiedSurfaceCaps {
    pub min_kappa: u16,
    pub max_kappa: u16,
    pub max_direct_support: u16,
    pub max_path_dimension: u32,
    pub max_expr_nodes: u16,
    pub allowed_imports: BTreeSet<u32>,
    pub allow_truncation: bool,
    pub allow_modal: bool,
    pub allow_temporal: bool,
    pub allow_linear_exponential: bool,
}

impl CertifiedSurfaceCaps {
    pub fn genesis_step16() -> Self {
        Self {
            min_kappa: 2,
            max_kappa: 4,
            max_direct_support: 2,
            max_path_dimension: 1,
            max_expr_nodes: 6,
            allowed_imports: [14, 15].into_iter().collect(),
            allow_truncation: false,
            allow_modal: true,
            allow_temporal: true,
            allow_linear_exponential: false,
        }
    }

    pub fn validate(&self) -> Result<(), CertifiedNoveltyError> {
        if self.min_kappa == 0 || self.min_kappa > self.max_kappa {
            return Err(CertifiedNoveltyError::InvalidSurfaceCaps {
                reason: "kappa interval must be nonempty and positive".to_owned(),
            });
        }
        if self.max_expr_nodes == 0 {
            return Err(CertifiedNoveltyError::InvalidSurfaceCaps {
                reason: "expression-node cap must be positive".to_owned(),
            });
        }
        if usize::from(self.max_direct_support) > self.allowed_imports.len() {
            return Err(CertifiedNoveltyError::InvalidSurfaceCaps {
                reason: "max_direct_support exceeds the frozen import set".to_owned(),
            });
        }
        Ok(())
    }
}

/// One telescope field referenced by a normalized expression.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FreshFieldHead {
    pub introduced_at_clause: u16,
}

/// A fresh neutral application observed during normalization.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StuckFreshEliminator {
    pub used_at_clause: u16,
    pub head: FreshFieldHead,
}

/// Structural normal-form facts used by all certificate validators.
///
/// `canonical_expr` is presentation normalization.  Support and de Bruijn
/// head tracking are computed independently, rather than trusted from a
/// submitted certificate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NormalizedClauseFacts {
    pub clause_index: u16,
    pub role: ClauseRole,
    pub canonical_expr: String,
    pub node_count: u16,
    pub direct_support: BTreeSet<u32>,
    pub referenced_fresh_heads: BTreeSet<FreshFieldHead>,
    pub ambient_parameters: BTreeSet<u32>,
    pub stuck_fresh_eliminators: BTreeSet<StuckFreshEliminator>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NormalizedCandidate {
    pub canonical_key: String,
    pub class: TelescopeClass,
    pub clauses: Vec<NormalizedClauseFacts>,
    pub direct_support: BTreeSet<u32>,
    pub referenced_fresh_heads: BTreeSet<FreshFieldHead>,
    pub stuck_fresh_eliminators: BTreeSet<StuckFreshEliminator>,
}

impl NormalizedCandidate {
    pub fn from_telescope(candidate: &Telescope, library: &Library) -> Self {
        let clauses = candidate
            .clauses
            .iter()
            .enumerate()
            .map(|(index, clause)| normalize_clause(index, clause.role, &clause.expr))
            .collect::<Vec<_>>();
        let direct_support = clauses
            .iter()
            .flat_map(|clause| clause.direct_support.iter().copied())
            .collect();
        let referenced_fresh_heads = clauses
            .iter()
            .flat_map(|clause| clause.referenced_fresh_heads.iter().copied())
            .collect();
        let stuck_fresh_eliminators = clauses
            .iter()
            .flat_map(|clause| clause.stuck_fresh_eliminators.iter().copied())
            .collect();

        Self {
            canonical_key: canonical_key_telescope(candidate).0,
            class: candidate.classify(library),
            clauses,
            direct_support,
            referenced_fresh_heads,
            stuck_fresh_eliminators,
        }
    }
}

fn normalize_clause(index: usize, role: ClauseRole, expr: &Expr) -> NormalizedClauseFacts {
    let clause_index = u16::try_from(index).expect("telescope clause index exceeded u16");
    let mut facts = ExprFacts::default();
    collect_expr_facts(expr, clause_index, 0, clause_index, &mut facts);
    NormalizedClauseFacts {
        clause_index,
        role,
        canonical_expr: canonical_key_expr(expr).0,
        node_count: facts.node_count,
        direct_support: facts.direct_support,
        referenced_fresh_heads: facts.referenced_fresh_heads,
        ambient_parameters: facts.ambient_parameters,
        stuck_fresh_eliminators: facts.stuck_fresh_eliminators,
    }
}

#[derive(Default)]
struct ExprFacts {
    node_count: u16,
    direct_support: BTreeSet<u32>,
    referenced_fresh_heads: BTreeSet<FreshFieldHead>,
    ambient_parameters: BTreeSet<u32>,
    stuck_fresh_eliminators: BTreeSet<StuckFreshEliminator>,
}

fn collect_expr_facts(
    expr: &Expr,
    prior_fields: u16,
    local_binders: u16,
    used_at_clause: u16,
    facts: &mut ExprFacts,
) {
    facts.node_count = facts.node_count.saturating_add(1);
    match expr {
        Expr::Lib(step) => {
            facts.direct_support.insert(*step);
        }
        Expr::Var(index) => record_var(*index, prior_fields, local_binders, facts),
        Expr::App(function, argument) => {
            if let Some(head) = fresh_head_of(function, prior_fields, local_binders) {
                facts.stuck_fresh_eliminators.insert(StuckFreshEliminator {
                    used_at_clause,
                    head,
                });
            }
            collect_expr_facts(function, prior_fields, local_binders, used_at_clause, facts);
            collect_expr_facts(argument, prior_fields, local_binders, used_at_clause, facts);
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            collect_expr_facts(domain, prior_fields, local_binders, used_at_clause, facts);
            collect_expr_facts(
                codomain,
                prior_fields,
                local_binders.saturating_add(1),
                used_at_clause,
                facts,
            );
        }
        Expr::Lam(body) => collect_expr_facts(
            body,
            prior_fields,
            local_binders.saturating_add(1),
            used_at_clause,
            facts,
        ),
        Expr::Id(ty, left, right) => {
            collect_expr_facts(ty, prior_fields, local_binders, used_at_clause, facts);
            collect_expr_facts(left, prior_fields, local_binders, used_at_clause, facts);
            collect_expr_facts(right, prior_fields, local_binders, used_at_clause, facts);
        }
        Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => {
            collect_expr_facts(body, prior_fields, local_binders, used_at_clause, facts)
        }
        Expr::Univ | Expr::PathCon(_) => {}
    }
}

fn record_var(index: u32, prior_fields: u16, local_binders: u16, facts: &mut ExprFacts) {
    if index == 0 {
        return;
    }
    let local = u32::from(local_binders);
    if index <= local {
        return;
    }
    let outside_local = index - local;
    if outside_local <= u32::from(prior_fields) {
        let introduced = u32::from(prior_fields) - outside_local;
        let introduced_at_clause =
            u16::try_from(introduced).expect("fresh field index exceeded u16");
        facts.referenced_fresh_heads.insert(FreshFieldHead {
            introduced_at_clause,
        });
    } else {
        facts
            .ambient_parameters
            .insert(outside_local - u32::from(prior_fields));
    }
}

fn fresh_head_of(expr: &Expr, prior_fields: u16, local_binders: u16) -> Option<FreshFieldHead> {
    match expr {
        Expr::Var(index) if *index > u32::from(local_binders) => {
            let outside_local = *index - u32::from(local_binders);
            (outside_local <= u32::from(prior_fields)).then(|| FreshFieldHead {
                introduced_at_clause: prior_fields - outside_local as u16,
            })
        }
        Expr::App(function, _) => fresh_head_of(function, prior_fields, local_binders),
        _ => None,
    }
}

/// An exact old-library realizer for one transparent clause.
///
/// The current MBTT has no reduction kernel, so the only machine-checkable
/// definitional equality admitted here is equality of canonical normal forms.
/// A future elaborator can replace this narrow witness without weakening the
/// fail-closed boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransparentClauseCertificate {
    pub clause_index: u16,
    pub old_realizer: Expr,
    pub candidate_normal_form: String,
    pub realizer_normal_form: String,
}

/// Trusted semantic capability asserting that the submitted old realizers
/// are well typed and elaborate the candidate declaration definitionally.
///
/// Canonical equality is checked independently, but syntax alone cannot say
/// whether a telescope clause is a definition body or an opaque signature.
/// Private fields and the absence of a public constructor keep that semantic
/// distinction fail closed until a typed elaborator is connected here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedTransparentElaborationToken {
    candidate_key: String,
    realizer_normal_forms: Vec<String>,
    theorem_id: String,
}

impl TrustedTransparentElaborationToken {
    pub fn theorem_id(&self) -> &str {
        &self.theorem_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransparentExtensionCertificate {
    pub candidate_key: String,
    pub clauses: Vec<TransparentClauseCertificate>,
    pub typed_elaboration: TrustedTransparentElaborationToken,
}

impl TransparentExtensionCertificate {
    pub fn exact(
        candidate: &Telescope,
        typed_elaboration: TrustedTransparentElaborationToken,
    ) -> Result<Self, TransparentCertificateError> {
        let candidate_key = canonical_key_telescope(candidate).0;
        let clauses = candidate
            .clauses
            .iter()
            .enumerate()
            .map(|(index, clause)| {
                let normal = canonical_key_expr(&clause.expr).0;
                TransparentClauseCertificate {
                    clause_index: index as u16,
                    old_realizer: clause.expr.clone(),
                    candidate_normal_form: normal.clone(),
                    realizer_normal_form: normal,
                }
            })
            .collect::<Vec<_>>();
        let realizer_normal_forms = clauses
            .iter()
            .map(|clause| clause.realizer_normal_form.clone())
            .collect::<Vec<_>>();
        if typed_elaboration.candidate_key != candidate_key
            || typed_elaboration.realizer_normal_forms != realizer_normal_forms
            || typed_elaboration.theorem_id.is_empty()
        {
            return Err(TransparentCertificateError::TypedElaborationTokenMismatch);
        }
        Ok(Self {
            candidate_key,
            clauses,
            typed_elaboration,
        })
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TransparentCertificateError {
    #[error("trusted transparent-elaboration token does not match the candidate realizers")]
    TypedElaborationTokenMismatch,
}

/// D3 kernel evidence for an opaque declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshKernelCertificate {
    pub candidate_key: String,
    /// Clauses asserted to introduce irreducible declaration heads.
    pub irreducible_clauses: BTreeSet<u16>,
    /// Must reproduce the normalizer's observed fresh stuck eliminators.
    pub observed_stuck_eliminators: BTreeSet<StuckFreshEliminator>,
    /// Must reproduce the normalized direct support exactly.
    pub normalized_support: BTreeSet<u32>,
}

impl FreshKernelCertificate {
    /// Submit the assertion that every clause is an opaque D3 head.
    ///
    /// This is deliberately named as an assertion: normalization verifies
    /// the support and stuck-head trace, but the present AST cannot derive
    /// semantic irreducibility.  The assertion never manufactures an
    /// H/P5/Synthesis amplification certificate, and its possible
    /// over-credit remains inside the independently derived linear bound.
    pub fn assert_all_clauses_opaque(candidate: &Telescope, library: &Library) -> Self {
        let normalized = NormalizedCandidate::from_telescope(candidate, library);
        Self {
            candidate_key: normalized.canonical_key,
            irreducible_clauses: (0..candidate.kappa()).map(|index| index as u16).collect(),
            observed_stuck_eliminators: normalized.stuck_fresh_eliminators,
            normalized_support: normalized.direct_support,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HSchemaTag {
    Beta {
        clause: u16,
    },
    Kan {
        clause: u16,
        principal: u32,
        probe: u32,
    },
}

/// Trusted semantic capability asserting that the submitted H formation has
/// a typed eliminator with the displayed beta/Kan family.
///
/// All fields are private and this module intentionally exposes no public
/// constructor.  The current AST cannot mint this token.  A future semantic
/// checker must be wired into this module to issue one after type checking.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedHFormEliminatorToken {
    candidate_key: String,
    formation_clause: u16,
    theorem_id: String,
}

impl TrustedHFormEliminatorToken {
    pub fn theorem_id(&self) -> &str {
        &self.theorem_id
    }
}

/// Frozen H-form basis: an actual formation clause plus exactly one beta tag
/// and the complete ordered `d x d` Kan matrix for every path constructor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenHFormCertificate {
    pub candidate_key: String,
    pub formation_clause: u16,
    pub schemas: BTreeSet<HSchemaTag>,
    pub typed_eliminator: TrustedHFormEliminatorToken,
}

impl FrozenHFormCertificate {
    pub fn complete(
        candidate: &Telescope,
        typed_eliminator: TrustedHFormEliminatorToken,
    ) -> Result<Self, HFormCertificateError> {
        let formation_clause = candidate
            .clauses
            .iter()
            .position(|clause| {
                clause.role == ClauseRole::Formation && is_h_formation_expr(&clause.expr)
            })
            .ok_or(HFormCertificateError::NoFormationClause)? as u16;
        let candidate_key = canonical_key_telescope(candidate).0;
        if typed_eliminator.candidate_key != candidate_key
            || typed_eliminator.formation_clause != formation_clause
            || typed_eliminator.theorem_id.is_empty()
        {
            return Err(HFormCertificateError::TypedEliminatorTokenMismatch);
        }
        let mut schemas = BTreeSet::new();
        for (index, clause) in candidate.clauses.iter().enumerate() {
            if let Expr::PathCon(dimension) = clause.expr {
                let clause = index as u16;
                schemas.insert(HSchemaTag::Beta { clause });
                for principal in 0..dimension {
                    for probe in 0..dimension {
                        schemas.insert(HSchemaTag::Kan {
                            clause,
                            principal,
                            probe,
                        });
                    }
                }
            }
        }
        if schemas.is_empty() {
            return Err(HFormCertificateError::NoPathConstructor);
        }
        Ok(Self {
            candidate_key,
            formation_clause,
            schemas,
            typed_eliminator,
        })
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum HFormCertificateError {
    #[error("an H-form certificate requires an explicit formation clause")]
    NoFormationClause,
    #[error("an H-form certificate requires at least one path constructor")]
    NoPathConstructor,
    #[error("trusted typed H-eliminator token does not match the candidate formation")]
    TypedEliminatorTokenMismatch,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct P5BridgeSchema {
    pub dominant_import: u32,
    pub target_import: u32,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct P5LocalLift {
    pub kernel_clause: u16,
    pub dominant_import: u32,
}

/// Sidecar capability produced only by replaying a kernel `TypedLiftToken`.
///
/// This capability deliberately asserts no record eliminator and contains no
/// free-form theorem label.  Its private fields bind the exact sealed
/// signature, candidate subject, kernel derivation, dominant import, visible
/// library prefix, and the clauses at which applications were actually
/// checked.  The public adapter below is the sole production constructor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayedP5LiftCapability {
    candidate_key: String,
    kernel_subject_hash: String,
    signature_digest: String,
    derivation_hash: String,
    visible_library: u32,
    direct_imports: BTreeSet<u32>,
    dominant_import: u32,
    lift_clauses: BTreeSet<u16>,
}

impl ReplayedP5LiftCapability {
    /// Replay a private-field kernel token and adapt exactly the fact it
    /// proves.  No token field, label, or caller-supplied digest is trusted.
    pub fn from_kernel_token(
        signature: &SealedSignature,
        candidate: &Telescope,
        visible_library: u32,
        token: &TypedLiftToken,
    ) -> Result<Self, P5KernelAdapterError> {
        replay_typed_lift_token(signature, candidate, visible_library, token)?;
        let lift_clauses = token.lift_clauses().iter().copied().collect::<BTreeSet<_>>();
        if lift_clauses.is_empty() || lift_clauses.len() != token.lift_clauses().len() {
            return Err(P5KernelAdapterError::InvalidLiftClauseSet);
        }
        Ok(Self {
            candidate_key: canonical_key_telescope(candidate).0,
            kernel_subject_hash: kernel_candidate_hash(candidate),
            signature_digest: token.signature_digest().to_owned(),
            derivation_hash: token.derivation_hash().to_owned(),
            visible_library,
            direct_imports: token.direct_imports().iter().copied().collect(),
            dominant_import: token.dominant_import(),
            lift_clauses,
        })
    }

    pub fn kernel_subject_hash(&self) -> &str {
        &self.kernel_subject_hash
    }

    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn visible_library(&self) -> u32 {
        self.visible_library
    }

    pub fn dominant_import(&self) -> u32 {
        self.dominant_import
    }

    pub fn direct_imports(&self) -> &BTreeSet<u32> {
        &self.direct_imports
    }

    pub fn lift_clauses(&self) -> &BTreeSet<u16> {
        &self.lift_clauses
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum P5KernelAdapterError {
    #[error("kernel P5 token failed definition replay: {0}")]
    Replay(#[from] TokenReplayError),
    #[error("kernel P5 token has an empty or duplicate lift-clause set")]
    InvalidLiftClauseSet,
}

/// Independent evidence that the opaque heads form an irreducible minimal
/// record API with a typed record eliminator.  A typed lift does not prove
/// this fact.  The fields are private and there is deliberately no public
/// constructor until the kernel can derive the required internality/API
/// theorem rather than accepting `assert_all_clauses_opaque` as evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedP5RecordInternalityCapability {
    candidate_key: String,
    signature_digest: String,
    api_clauses: BTreeSet<u16>,
    derivation_hash: String,
}

/// Frozen support-local P5 certificate.
///
/// It certifies the graph premise, a complete API (every opaque kernel
/// clause), one bridge to every non-dominant direct import, and at most one
/// shallow lift per fresh kernel head.  It intentionally has no field that
/// can claim `nu(Lmax)` as an undifferentiated integer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenP5Certificate {
    pub candidate_key: String,
    pub dominant_import: u32,
    pub minimal_complete_api: BTreeSet<u16>,
    pub bridges: BTreeSet<P5BridgeSchema>,
    pub local_lifts: BTreeSet<P5LocalLift>,
    pub replayed_typed_lift: ReplayedP5LiftCapability,
    pub record_internality: TrustedP5RecordInternalityCapability,
}

impl FrozenP5Certificate {
    /// Build the sidecar certificate through the replay boundary.  Keeping
    /// token adaptation inside this constructor prevents a caller from
    /// presenting strings or copying token metadata into a trusted wrapper.
    pub fn complete_local_from_kernel(
        signature: &SealedSignature,
        candidate: &Telescope,
        visible_library: u32,
        graph: &ImportDag,
        kernel: &FreshKernelCertificate,
        token: &TypedLiftToken,
        record_internality: TrustedP5RecordInternalityCapability,
    ) -> Result<Self, P5CertificateError> {
        let replayed_typed_lift =
            ReplayedP5LiftCapability::from_kernel_token(
                signature,
                candidate,
                visible_library,
                token,
            )?;
        let audit = P5ImportAudit::check(candidate, graph);
        let dominant_import =
            audit
                .unique_dominant_import
                .ok_or(P5CertificateError::NoUniqueDominantImport {
                    direct_imports: audit.direct_imports,
                    dominant_imports: audit.dominant_imports,
                })?;
        let candidate_key = canonical_key_telescope(candidate).0;
        if replayed_typed_lift.candidate_key != candidate_key
            || replayed_typed_lift.kernel_subject_hash != kernel_candidate_hash(candidate)
            || replayed_typed_lift.signature_digest != signature.digest()
            || replayed_typed_lift.derivation_hash != token.derivation_hash()
            || replayed_typed_lift.visible_library != visible_library
            || replayed_typed_lift.direct_imports != candidate.lib_refs()
            || replayed_typed_lift.dominant_import != dominant_import
            || !replayed_typed_lift
                .lift_clauses
                .is_subset(&kernel.irreducible_clauses)
        {
            return Err(P5CertificateError::KernelLiftCapabilityMismatch);
        }
        if record_internality.candidate_key != candidate_key
            || record_internality.signature_digest != signature.digest()
            || record_internality.api_clauses != kernel.irreducible_clauses
            || record_internality.derivation_hash.is_empty()
        {
            return Err(P5CertificateError::RecordInternalityCapabilityMismatch);
        }
        let bridges = candidate
            .lib_refs()
            .into_iter()
            .filter(|target| *target != dominant_import)
            .map(|target_import| P5BridgeSchema {
                dominant_import,
                target_import,
            })
            .collect();
        let local_lifts = replayed_typed_lift
            .lift_clauses
            .iter()
            .copied()
            .map(|kernel_clause| P5LocalLift {
                kernel_clause,
                dominant_import,
            })
            .collect();
        Ok(Self {
            candidate_key,
            dominant_import,
            minimal_complete_api: kernel.irreducible_clauses.clone(),
            bridges,
            local_lifts,
            replayed_typed_lift,
            record_internality,
        })
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum P5CertificateError {
    #[error(
        "P5 requires one reachability-dominant direct import; imports {direct_imports:?} have dominant set {dominant_imports:?}"
    )]
    NoUniqueDominantImport {
        direct_imports: Vec<u32>,
        dominant_imports: Vec<u32>,
    },
    #[error(transparent)]
    KernelAdapter(#[from] P5KernelAdapterError),
    #[error("replayed kernel P5 lift capability does not match the candidate API")]
    KernelLiftCapabilityMismatch,
    #[error("typed P5 record-internality capability does not match the candidate API")]
    RecordInternalityCapabilityMismatch,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SynthesisSiteKind {
    UniversePolymorphism,
    SpatialTemporalShift,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SynthesisSite {
    pub clause: u16,
    pub kind: SynthesisSiteKind,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SynthesisTransportSchema {
    pub site: SynthesisSite,
    pub supported_import: u32,
}

/// Trusted semantic capability for the typed polymorphic temporal
/// eliminators at the detected synthesis sites.  Private fields prevent an
/// AST-only caller from manufacturing the missing typing theorem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedSynthesisPolymorphicEliminatorToken {
    candidate_key: String,
    sites: BTreeSet<SynthesisSite>,
    theorem_id: String,
}

impl TrustedSynthesisPolymorphicEliminatorToken {
    pub fn theorem_id(&self) -> &str {
        &self.theorem_id
    }
}

/// Trusted semantic capability for naturality of the exact
/// site-by-direct-support transport matrix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedSynthesisNaturalityToken {
    candidate_key: String,
    transports: BTreeSet<SynthesisTransportSchema>,
    theorem_id: String,
}

impl TrustedSynthesisNaturalityToken {
    pub fn theorem_id(&self) -> &str {
        &self.theorem_id
    }
}

/// Frozen synthesis certificate: every detected amplification site is paired
/// only with direct normalized support.  No whole-library multiplier exists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenSynthesisCertificate {
    pub candidate_key: String,
    pub sites: BTreeSet<SynthesisSite>,
    pub transports: BTreeSet<SynthesisTransportSchema>,
    pub typed_polymorphic_eliminator: TrustedSynthesisPolymorphicEliminatorToken,
    pub naturality: TrustedSynthesisNaturalityToken,
}

impl FrozenSynthesisCertificate {
    pub fn complete_support_matrix(
        candidate: &Telescope,
        typed_polymorphic_eliminator: TrustedSynthesisPolymorphicEliminatorToken,
        naturality: TrustedSynthesisNaturalityToken,
    ) -> Result<Self, SynthesisCertificateError> {
        let sites = synthesis_sites(candidate);
        let direct_support = candidate.lib_refs();
        let transports = sites
            .iter()
            .copied()
            .flat_map(|site| {
                direct_support.iter().copied().map(move |supported_import| {
                    SynthesisTransportSchema {
                        site,
                        supported_import,
                    }
                })
            })
            .collect();
        let candidate_key = canonical_key_telescope(candidate).0;
        if typed_polymorphic_eliminator.candidate_key != candidate_key
            || typed_polymorphic_eliminator.sites != sites
            || typed_polymorphic_eliminator.theorem_id.is_empty()
        {
            return Err(SynthesisCertificateError::TypedEliminatorTokenMismatch);
        }
        if naturality.candidate_key != candidate_key
            || naturality.transports != transports
            || naturality.theorem_id.is_empty()
        {
            return Err(SynthesisCertificateError::NaturalityTokenMismatch);
        }
        Ok(Self {
            candidate_key,
            sites,
            transports,
            typed_polymorphic_eliminator,
            naturality,
        })
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SynthesisCertificateError {
    #[error("trusted typed synthesis-eliminator token does not match the detected sites")]
    TypedEliminatorTokenMismatch,
    #[error("trusted synthesis-naturality token does not match the support matrix")]
    NaturalityTokenMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpaqueExtensionCertificate {
    pub kernel: FreshKernelCertificate,
    pub h_form: Option<FrozenHFormCertificate>,
    pub p5: Option<FrozenP5Certificate>,
    pub synthesis: Option<FrozenSynthesisCertificate>,
}

impl OpaqueExtensionCertificate {
    pub fn local(candidate: &Telescope, library: &Library) -> Self {
        Self {
            kernel: FreshKernelCertificate::assert_all_clauses_opaque(candidate, library),
            h_form: None,
            p5: None,
            synthesis: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtensionCertificate {
    Transparent(TransparentExtensionCertificate),
    Opaque(OpaqueExtensionCertificate),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CertifiedDisposition {
    Transparent,
    CertifiedOpaque,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum AmplificationKind {
    WholeLibraryHitInheritance,
    HistoricalP5Inheritance,
    WholeLibrarySynthesisMultiplier,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SupportLocalComponents {
    pub native_kernel: u32,
    pub local_elimination: u32,
    pub support_interaction: u32,
    pub h_basis: u32,
    pub p5_bridges: u32,
    pub p5_local_lifts: u32,
    pub synthesis_transports: u32,
    pub total: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CertifiedNoveltyReport {
    pub disposition: CertifiedDisposition,
    pub class: TelescopeClass,
    pub kappa: u32,
    pub direct_support: BTreeSet<u32>,
    pub components: SupportLocalComponents,
    pub nu: u32,
    pub derived_linear_coefficient: u32,
    pub derived_linear_bound: u32,
    pub linear_theorem_holds: bool,
    /// Historical shortcuts deliberately not used by this evaluator.
    pub uncredited_amplifications: Vec<AmplificationKind>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClassCeilings {
    pub kappa: u16,
    pub foundation: u32,
    pub former: u32,
    pub hit: u32,
    pub suspension: u32,
    pub map: u32,
    pub modal: u32,
    pub axiomatic: u32,
    pub synthesis: u32,
    pub unknown: u32,
    pub maximum: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LinearBoundTheorem {
    pub caps: CertifiedSurfaceCaps,
    pub per_kappa: Vec<ClassCeilings>,
    pub coefficient: u32,
    pub every_ceiling_is_bounded: bool,
}

/// Derive the smallest integral `C` supported by the frozen class ceilings,
/// such that every support-local candidate on the surface has `nu <= C*k`.
pub fn derive_linear_bound(
    caps: &CertifiedSurfaceCaps,
) -> Result<LinearBoundTheorem, CertifiedNoveltyError> {
    caps.validate()?;
    let mut per_kappa = Vec::new();
    let mut coefficient = 0;
    for kappa in caps.min_kappa..=caps.max_kappa {
        let ceilings = class_ceilings(caps, kappa);
        coefficient = coefficient.max(ceil_div(ceilings.maximum, u32::from(kappa)));
        per_kappa.push(ceilings);
    }
    let every_ceiling_is_bounded = per_kappa
        .iter()
        .all(|ceilings| ceilings.maximum <= coefficient.saturating_mul(u32::from(ceilings.kappa)));
    Ok(LinearBoundTheorem {
        caps: caps.clone(),
        per_kappa,
        coefficient,
        every_ceiling_is_bounded,
    })
}

fn class_ceilings(caps: &CertifiedSurfaceCaps, kappa: u16) -> ClassCeilings {
    let k = u32::from(kappa);
    let r = u32::from(caps.max_direct_support);
    let d_payload = 1_u32.saturating_add(
        caps.max_path_dimension
            .saturating_mul(caps.max_path_dimension),
    );
    let modal_kinds = k.min(4);
    let modal_pairs = modal_kinds.saturating_mul(modal_kinds.saturating_sub(1)) / 2;

    let foundation = k;
    let former = 2 * k;
    // One local elimination per clause, direct support only, and a complete
    // beta/Kan payload per possible path clause.
    let hit = k
        .saturating_add(r)
        .saturating_add(k.saturating_mul(d_payload));
    let suspension = 5;
    let map = 2 * k + r * r;
    let modal = k + r + modal_pairs;
    // Native heads + local eliminators + one shallow dominant lift per head
    // + one bridge to each remaining direct support.
    let axiomatic = 3 * k + r.saturating_sub(1);
    // Base clauses plus the clause-by-direct-support transport matrix.
    let synthesis = k + k * r;
    let unknown = 2 * k;
    let maximum = [
        foundation, former, hit, suspension, map, modal, axiomatic, synthesis, unknown,
    ]
    .into_iter()
    .max()
    .unwrap_or(0);

    ClassCeilings {
        kappa,
        foundation,
        former,
        hit,
        suspension,
        map,
        modal,
        axiomatic,
        synthesis,
        unknown,
        maximum,
    }
}

fn ceil_div(numerator: u32, denominator: u32) -> u32 {
    numerator.saturating_add(denominator.saturating_sub(1)) / denominator
}

/// Validate a proof-carrying declaration and compute only support-local
/// novelty.  No bar is an input to this function.
pub fn evaluate_certified_novelty(
    candidate: &Telescope,
    certificate: &ExtensionCertificate,
    signature: &SealedSignature,
    library: &Library,
    graph: &ImportDag,
    caps: &CertifiedSurfaceCaps,
) -> Result<CertifiedNoveltyReport, CertifiedNoveltyError> {
    validate_signature_library_context(signature, library)?;
    validate_surface(candidate, caps)?;
    let theorem = derive_linear_bound(caps)?;
    let normalized = NormalizedCandidate::from_telescope(candidate, library);
    let kappa = candidate.kappa() as u32;

    let (disposition, components, uncredited_amplifications) = match certificate {
        ExtensionCertificate::Transparent(transparent) => {
            validate_transparent(candidate, transparent, &normalized, caps)?;
            (
                CertifiedDisposition::Transparent,
                zero_components(),
                Vec::new(),
            )
        }
        ExtensionCertificate::Opaque(opaque) => {
            validate_kernel(candidate, &opaque.kernel, &normalized)?;
            let (components, uncredited) = evaluate_opaque(
                candidate,
                opaque,
                &normalized,
                signature,
                library,
                graph,
                caps,
            )?;
            (
                CertifiedDisposition::CertifiedOpaque,
                components,
                uncredited,
            )
        }
    };

    let nu = components.total;
    let derived_linear_bound = theorem.coefficient.saturating_mul(kappa);
    let linear_theorem_holds = nu <= derived_linear_bound;
    if !linear_theorem_holds {
        return Err(CertifiedNoveltyError::LinearBoundViolation {
            nu,
            coefficient: theorem.coefficient,
            kappa,
        });
    }
    Ok(CertifiedNoveltyReport {
        disposition,
        class: normalized.class,
        kappa,
        direct_support: normalized.direct_support,
        components,
        nu,
        derived_linear_coefficient: theorem.coefficient,
        derived_linear_bound,
        linear_theorem_holds,
        uncredited_amplifications,
    })
}

/// The sidecar consumes a semantic token in the same sealed context in
/// which the kernel issued it.  `Library` is only a derived summary, so it
/// must replay exactly from the signature rather than being paired with a
/// token by convention.
fn validate_signature_library_context(
    signature: &SealedSignature,
    library: &Library,
) -> Result<(), CertifiedNoveltyError> {
    if signature.len() != library.len() {
        return Err(CertifiedNoveltyError::SignatureLibraryMismatch);
    }
    let mut replayed = Library::new();
    for (offset, entry) in signature.entries().iter().enumerate() {
        if entry.step != u32::try_from(offset + 1).expect("signature length fits u32") {
            return Err(CertifiedNoveltyError::SignatureLibraryMismatch);
        }
        let summary = LibraryEntry::from_telescope(&entry.telescope, &replayed);
        if library.get(offset) != Some(&summary) {
            return Err(CertifiedNoveltyError::SignatureLibraryMismatch);
        }
        replayed.push(summary);
    }
    Ok(())
}

fn zero_components() -> SupportLocalComponents {
    SupportLocalComponents {
        native_kernel: 0,
        local_elimination: 0,
        support_interaction: 0,
        h_basis: 0,
        p5_bridges: 0,
        p5_local_lifts: 0,
        synthesis_transports: 0,
        total: 0,
    }
}

fn evaluate_opaque(
    candidate: &Telescope,
    opaque: &OpaqueExtensionCertificate,
    normalized: &NormalizedCandidate,
    signature: &SealedSignature,
    _library: &Library,
    graph: &ImportDag,
    caps: &CertifiedSurfaceCaps,
) -> Result<(SupportLocalComponents, Vec<AmplificationKind>), CertifiedNoveltyError> {
    let k = candidate.kappa() as u32;
    let r = normalized.direct_support.len() as u32;
    let kernel = opaque.kernel.irreducible_clauses.len() as u32;
    let mut uncredited = Vec::new();

    let components = match normalized.class {
        TelescopeClass::Foundation => components(kernel, 0, 0, 0, 0, 0, 0),
        TelescopeClass::Former | TelescopeClass::Unknown => components(kernel, k, 0, 0, 0, 0, 0),
        TelescopeClass::Suspension => {
            let local = 5_u32.saturating_sub(kernel);
            components(kernel, local, 0, 0, 0, 0, 0)
        }
        TelescopeClass::Map => {
            let target = 2 * k + r * r;
            components(kernel, k, target.saturating_sub(kernel + k), 0, 0, 0, 0)
        }
        TelescopeClass::Modal => {
            let kinds = count_top_level_modal_kinds(candidate);
            let pairwise = kinds.saturating_mul(kinds.saturating_sub(1)) / 2;
            components(kernel, k.saturating_sub(kernel), r + pairwise, 0, 0, 0, 0)
        }
        TelescopeClass::Hit => {
            uncredited.push(AmplificationKind::WholeLibraryHitInheritance);
            let h_form = opaque.h_form.as_ref().ok_or(
                CertifiedNoveltyError::MissingAmplificationCertificate {
                    amplification: AmplificationKind::WholeLibraryHitInheritance,
                    requirement: "FrozenHFormCertificate",
                },
            )?;
            let h_count = validate_h_form(candidate, h_form, caps)?;
            components(kernel, k.saturating_sub(kernel), r, h_count, 0, 0, 0)
        }
        TelescopeClass::Axiomatic => {
            uncredited.push(AmplificationKind::HistoricalP5Inheritance);
            let audit = P5ImportAudit::check(candidate, graph);
            let Some(dominant) = audit.unique_dominant_import else {
                return Err(CertifiedNoveltyError::P5NoUniqueDominantImport {
                    direct_imports: audit.direct_imports,
                    dominant_imports: audit.dominant_imports,
                });
            };
            let p5 = opaque.p5.as_ref().ok_or(
                CertifiedNoveltyError::MissingAmplificationCertificate {
                    amplification: AmplificationKind::HistoricalP5Inheritance,
                    requirement: "FrozenP5Certificate",
                },
            )?;
            let (bridges, lifts) =
                validate_p5(candidate, &opaque.kernel, p5, dominant, signature)?;
            components(kernel, k, 0, 0, bridges, lifts, 0)
        }
        TelescopeClass::Synthesis => {
            let sites = synthesis_sites(candidate);
            let transports = if sites.is_empty() {
                0
            } else {
                uncredited.push(AmplificationKind::WholeLibrarySynthesisMultiplier);
                let synthesis = opaque.synthesis.as_ref().ok_or(
                    CertifiedNoveltyError::MissingAmplificationCertificate {
                        amplification: AmplificationKind::WholeLibrarySynthesisMultiplier,
                        requirement: "FrozenSynthesisCertificate",
                    },
                )?;
                validate_synthesis(candidate, synthesis)?
            };
            components(kernel, k.saturating_sub(kernel), 0, 0, 0, 0, transports)
        }
    };

    Ok((components, uncredited))
}

fn components(
    native_kernel: u32,
    local_elimination: u32,
    support_interaction: u32,
    h_basis: u32,
    p5_bridges: u32,
    p5_local_lifts: u32,
    synthesis_transports: u32,
) -> SupportLocalComponents {
    let total = native_kernel
        .saturating_add(local_elimination)
        .saturating_add(support_interaction)
        .saturating_add(h_basis)
        .saturating_add(p5_bridges)
        .saturating_add(p5_local_lifts)
        .saturating_add(synthesis_transports);
    SupportLocalComponents {
        native_kernel,
        local_elimination,
        support_interaction,
        h_basis,
        p5_bridges,
        p5_local_lifts,
        synthesis_transports,
        total,
    }
}

fn validate_surface(
    candidate: &Telescope,
    caps: &CertifiedSurfaceCaps,
) -> Result<(), CertifiedNoveltyError> {
    caps.validate()?;
    let kappa = candidate.kappa() as u16;
    if !(caps.min_kappa..=caps.max_kappa).contains(&kappa) {
        return Err(CertifiedNoveltyError::SurfaceViolation {
            reason: format!(
                "kappa {kappa} lies outside {}..={}",
                caps.min_kappa, caps.max_kappa
            ),
        });
    }
    let support = candidate.lib_refs();
    if support.len() > usize::from(caps.max_direct_support) {
        return Err(CertifiedNoveltyError::SurfaceViolation {
            reason: format!(
                "direct support {} exceeds cap {}",
                support.len(),
                caps.max_direct_support
            ),
        });
    }
    if !support.is_subset(&caps.allowed_imports) {
        return Err(CertifiedNoveltyError::SurfaceViolation {
            reason: format!(
                "direct support {support:?} is not contained in frozen imports {:?}",
                caps.allowed_imports
            ),
        });
    }
    for (index, clause) in candidate.clauses.iter().enumerate() {
        let nodes = expr_node_count(&clause.expr);
        if nodes > u32::from(caps.max_expr_nodes) {
            return Err(CertifiedNoveltyError::SurfaceViolation {
                reason: format!(
                    "clause {index} has {nodes} nodes, exceeding {}",
                    caps.max_expr_nodes
                ),
            });
        }
        validate_expr_surface(&clause.expr, caps).map_err(|reason| {
            CertifiedNoveltyError::SurfaceViolation {
                reason: format!("clause {index}: {reason}"),
            }
        })?;
    }
    Ok(())
}

fn validate_expr_surface(expr: &Expr, caps: &CertifiedSurfaceCaps) -> Result<(), String> {
    match expr {
        Expr::PathCon(dimension) if *dimension == 0 || *dimension > caps.max_path_dimension => {
            Err(format!(
                "path dimension {dimension} lies outside 1..={}",
                caps.max_path_dimension
            ))
        }
        Expr::Trunc(_) if !caps.allow_truncation => Err("truncation is disabled".to_owned()),
        Expr::Flat(_) | Expr::Sharp(_) | Expr::Disc(_) | Expr::Shape(_) if !caps.allow_modal => {
            Err("modal constructors are disabled".to_owned())
        }
        Expr::Next(_) | Expr::Eventually(_) if !caps.allow_temporal => {
            Err("temporal constructors are disabled".to_owned())
        }
        Expr::Bang(_) | Expr::WhyNot(_) if !caps.allow_linear_exponential => {
            Err("linear exponentials are disabled".to_owned())
        }
        Expr::Id(_, _, _) | Expr::Refl(_) | Expr::Susp(_) => {
            Err("constructor is absent from the frozen generic raw grammar".to_owned())
        }
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            validate_expr_surface(left, caps)?;
            validate_expr_surface(right, caps)
        }
        Expr::Lam(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => validate_expr_surface(body, caps),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => Ok(()),
    }
}

fn validate_transparent(
    candidate: &Telescope,
    certificate: &TransparentExtensionCertificate,
    normalized: &NormalizedCandidate,
    caps: &CertifiedSurfaceCaps,
) -> Result<(), CertifiedNoveltyError> {
    if certificate.candidate_key != normalized.canonical_key {
        return Err(CertifiedNoveltyError::CandidateKeyMismatch);
    }
    if certificate.typed_elaboration.candidate_key != certificate.candidate_key
        || certificate.typed_elaboration.theorem_id.is_empty()
    {
        return Err(CertifiedNoveltyError::InvalidTransparentElaborationToken);
    }
    if certificate.clauses.len() != candidate.kappa() {
        return Err(CertifiedNoveltyError::IncompleteTransparentCertificate);
    }
    let mut seen = BTreeSet::new();
    let mut realizer_normal_forms = vec![String::new(); candidate.kappa()];
    for witness in &certificate.clauses {
        if !seen.insert(witness.clause_index) {
            return Err(CertifiedNoveltyError::IncompleteTransparentCertificate);
        }
        let Some(clause) = candidate.clauses.get(usize::from(witness.clause_index)) else {
            return Err(CertifiedNoveltyError::IncompleteTransparentCertificate);
        };
        let clause_facts = &normalized.clauses[usize::from(witness.clause_index)];
        if !clause_facts.referenced_fresh_heads.is_empty()
            || !clause_facts.stuck_fresh_eliminators.is_empty()
        {
            return Err(CertifiedNoveltyError::TransparentRealizerUsesFreshHead {
                clause: witness.clause_index,
            });
        }
        let candidate_normal = canonical_key_expr(&clause.expr).0;
        let realizer_normal = canonical_key_expr(&witness.old_realizer).0;
        if candidate_normal != witness.candidate_normal_form
            || realizer_normal != witness.realizer_normal_form
            || candidate_normal != realizer_normal
        {
            return Err(CertifiedNoveltyError::TransparentNormalFormsDiffer {
                clause: witness.clause_index,
            });
        }
        realizer_normal_forms[usize::from(witness.clause_index)] = realizer_normal;
        let realizer = normalize_clause(0, clause.role, &witness.old_realizer);
        if !realizer.referenced_fresh_heads.is_empty()
            || !realizer.stuck_fresh_eliminators.is_empty()
        {
            return Err(CertifiedNoveltyError::TransparentRealizerUsesFreshHead {
                clause: witness.clause_index,
            });
        }
        if !realizer.direct_support.is_subset(&caps.allowed_imports) {
            return Err(CertifiedNoveltyError::TransparentRealizerOutsideLibrary {
                clause: witness.clause_index,
            });
        }
    }
    if seen.len() != candidate.kappa() {
        return Err(CertifiedNoveltyError::IncompleteTransparentCertificate);
    }
    if certificate.typed_elaboration.realizer_normal_forms != realizer_normal_forms {
        return Err(CertifiedNoveltyError::InvalidTransparentElaborationToken);
    }
    Ok(())
}

fn validate_kernel(
    candidate: &Telescope,
    kernel: &FreshKernelCertificate,
    normalized: &NormalizedCandidate,
) -> Result<(), CertifiedNoveltyError> {
    if kernel.candidate_key != normalized.canonical_key {
        return Err(CertifiedNoveltyError::CandidateKeyMismatch);
    }
    if kernel.irreducible_clauses.is_empty() {
        return Err(CertifiedNoveltyError::EmptyFreshKernel);
    }
    if kernel
        .irreducible_clauses
        .iter()
        .any(|index| usize::from(*index) >= candidate.kappa())
    {
        return Err(CertifiedNoveltyError::InvalidFreshKernelClause);
    }
    if kernel.observed_stuck_eliminators != normalized.stuck_fresh_eliminators {
        return Err(CertifiedNoveltyError::StuckEliminatorTraceMismatch);
    }
    if kernel.normalized_support != normalized.direct_support {
        return Err(CertifiedNoveltyError::NormalizedSupportMismatch);
    }
    Ok(())
}

fn validate_h_form(
    candidate: &Telescope,
    certificate: &FrozenHFormCertificate,
    caps: &CertifiedSurfaceCaps,
) -> Result<u32, CertifiedNoveltyError> {
    if certificate.candidate_key != canonical_key_telescope(candidate).0 {
        return Err(CertifiedNoveltyError::CandidateKeyMismatch);
    }
    let Some(formation) = candidate
        .clauses
        .get(usize::from(certificate.formation_clause))
    else {
        return Err(CertifiedNoveltyError::InvalidHFormCertificate {
            reason: "formation-clause index is out of range".to_owned(),
        });
    };
    if formation.role != ClauseRole::Formation || !is_h_formation_expr(&formation.expr) {
        return Err(CertifiedNoveltyError::InvalidHFormCertificate {
            reason: "submitted clause is not a Formation-role H formation".to_owned(),
        });
    }
    if certificate.typed_eliminator.candidate_key != certificate.candidate_key
        || certificate.typed_eliminator.formation_clause != certificate.formation_clause
        || certificate.typed_eliminator.theorem_id.is_empty()
    {
        return Err(CertifiedNoveltyError::InvalidHFormCertificate {
            reason: "trusted typed-eliminator token does not match the formation".to_owned(),
        });
    }
    let mut expected = BTreeSet::new();
    for (index, clause) in candidate.clauses.iter().enumerate() {
        if let Expr::PathCon(dimension) = clause.expr {
            if dimension == 0 || dimension > caps.max_path_dimension {
                return Err(CertifiedNoveltyError::InvalidHFormCertificate {
                    reason: "path dimension exceeds the frozen surface".to_owned(),
                });
            }
            let clause = index as u16;
            expected.insert(HSchemaTag::Beta { clause });
            for principal in 0..dimension {
                for probe in 0..dimension {
                    expected.insert(HSchemaTag::Kan {
                        clause,
                        principal,
                        probe,
                    });
                }
            }
        }
    }
    if expected.is_empty() || certificate.schemas != expected {
        return Err(CertifiedNoveltyError::InvalidHFormCertificate {
            reason: "beta/Kan basis is not the exact frozen finite basis".to_owned(),
        });
    }
    Ok(expected.len() as u32)
}

fn validate_p5(
    candidate: &Telescope,
    kernel: &FreshKernelCertificate,
    certificate: &FrozenP5Certificate,
    dominant: u32,
    signature: &SealedSignature,
) -> Result<(u32, u32), CertifiedNoveltyError> {
    if certificate.candidate_key != canonical_key_telescope(candidate).0 {
        return Err(CertifiedNoveltyError::CandidateKeyMismatch);
    }
    if certificate.dominant_import != dominant {
        return Err(CertifiedNoveltyError::InvalidP5Certificate {
            reason: "certificate dominant import differs from graph audit".to_owned(),
        });
    }
    if certificate.minimal_complete_api != kernel.irreducible_clauses {
        return Err(CertifiedNoveltyError::InvalidP5Certificate {
            reason: "minimal complete API does not cover the fresh kernel exactly".to_owned(),
        });
    }
    if certificate.replayed_typed_lift.candidate_key != certificate.candidate_key
        || certificate.replayed_typed_lift.kernel_subject_hash
            != kernel_candidate_hash(candidate)
        || certificate.replayed_typed_lift.signature_digest != signature.digest()
        || certificate.replayed_typed_lift.derivation_hash.is_empty()
        || certificate.replayed_typed_lift.visible_library != signature.len() as u32
        || certificate.replayed_typed_lift.direct_imports != candidate.lib_refs()
        || certificate.replayed_typed_lift.dominant_import != dominant
        || certificate.replayed_typed_lift.lift_clauses.is_empty()
        || !certificate
            .replayed_typed_lift
            .lift_clauses
            .is_subset(&kernel.irreducible_clauses)
    {
        return Err(CertifiedNoveltyError::InvalidP5Certificate {
            reason: "replayed kernel lift binding does not match the candidate API".to_owned(),
        });
    }
    if certificate.record_internality.candidate_key != certificate.candidate_key
        || certificate.record_internality.signature_digest != signature.digest()
        || certificate.record_internality.api_clauses != kernel.irreducible_clauses
        || certificate.record_internality.derivation_hash.is_empty()
    {
        return Err(CertifiedNoveltyError::InvalidP5Certificate {
            reason: "typed record-internality binding does not match the candidate API".to_owned(),
        });
    }
    let expected_bridges = candidate
        .lib_refs()
        .into_iter()
        .filter(|target| *target != dominant)
        .map(|target_import| P5BridgeSchema {
            dominant_import: dominant,
            target_import,
        })
        .collect::<BTreeSet<_>>();
    if certificate.bridges != expected_bridges {
        return Err(CertifiedNoveltyError::InvalidP5Certificate {
            reason: "bridge basis is not exactly one per non-dominant import".to_owned(),
        });
    }
    if certificate.local_lifts.len() > kernel.irreducible_clauses.len()
        || certificate.local_lifts.iter().any(|lift| {
            lift.dominant_import != dominant
                || !kernel.irreducible_clauses.contains(&lift.kernel_clause)
        })
    {
        return Err(CertifiedNoveltyError::InvalidP5Certificate {
            reason: "local lifts must be a partial one-per-kernel-head map to Lmax".to_owned(),
        });
    }
    let distinct_lift_heads = certificate
        .local_lifts
        .iter()
        .map(|lift| lift.kernel_clause)
        .collect::<BTreeSet<_>>();
    if distinct_lift_heads.len() != certificate.local_lifts.len() {
        return Err(CertifiedNoveltyError::InvalidP5Certificate {
            reason: "multiple inherited lifts were assigned to one fresh head".to_owned(),
        });
    }
    if distinct_lift_heads != certificate.replayed_typed_lift.lift_clauses {
        return Err(CertifiedNoveltyError::InvalidP5Certificate {
            reason: "local lifts differ from the replayed kernel lift clauses".to_owned(),
        });
    }
    Ok((
        certificate.bridges.len() as u32,
        certificate.local_lifts.len() as u32,
    ))
}

fn validate_synthesis(
    candidate: &Telescope,
    certificate: &FrozenSynthesisCertificate,
) -> Result<u32, CertifiedNoveltyError> {
    if certificate.candidate_key != canonical_key_telescope(candidate).0 {
        return Err(CertifiedNoveltyError::CandidateKeyMismatch);
    }
    let sites = synthesis_sites(candidate);
    if certificate.sites != sites {
        return Err(CertifiedNoveltyError::InvalidSynthesisCertificate {
            reason: "certificate does not cover exactly the detected synthesis sites".to_owned(),
        });
    }
    if certificate.typed_polymorphic_eliminator.candidate_key != certificate.candidate_key
        || certificate.typed_polymorphic_eliminator.sites != sites
        || certificate
            .typed_polymorphic_eliminator
            .theorem_id
            .is_empty()
    {
        return Err(CertifiedNoveltyError::InvalidSynthesisCertificate {
            reason: "trusted typed polymorphic-eliminator token does not match the sites"
                .to_owned(),
        });
    }
    let support = candidate.lib_refs();
    let expected = sites
        .iter()
        .copied()
        .flat_map(|site| {
            support
                .iter()
                .copied()
                .map(move |supported_import| SynthesisTransportSchema {
                    site,
                    supported_import,
                })
        })
        .collect::<BTreeSet<_>>();
    if certificate.transports != expected {
        return Err(CertifiedNoveltyError::InvalidSynthesisCertificate {
            reason: "transport basis is not the exact site-by-direct-support matrix".to_owned(),
        });
    }
    if certificate.naturality.candidate_key != certificate.candidate_key
        || certificate.naturality.transports != expected
        || certificate.naturality.theorem_id.is_empty()
    {
        return Err(CertifiedNoveltyError::InvalidSynthesisCertificate {
            reason: "trusted naturality token does not match the support matrix".to_owned(),
        });
    }
    Ok(expected.len() as u32)
}

fn synthesis_sites(candidate: &Telescope) -> BTreeSet<SynthesisSite> {
    candidate
        .clauses
        .iter()
        .enumerate()
        .flat_map(|(index, clause)| {
            let mut sites = Vec::new();
            if is_polymorphic_temporal_elim(&clause.expr) {
                sites.push(SynthesisSite {
                    clause: index as u16,
                    kind: SynthesisSiteKind::UniversePolymorphism,
                });
            }
            if is_spatial_temporal_clause(&clause.expr) {
                sites.push(SynthesisSite {
                    clause: index as u16,
                    kind: SynthesisSiteKind::SpatialTemporalShift,
                });
            }
            sites
        })
        .collect()
}

fn is_h_formation_expr(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Trunc(_))
        || matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ))
}

fn is_polymorphic_temporal_elim(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::App(function, _) if matches!(function.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))))
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Next(inner2) | Expr::Bang(inner2) if matches!(inner2.as_ref(), Expr::Var(_))))
                    && matches!(codomain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                    && matches!(codomain.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
}

fn is_spatial_temporal_clause(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(_))
                        && matches!(
                            argument.as_ref(),
                            Expr::Next(inner)
                                | Expr::Eventually(inner)
                                | Expr::Bang(inner)
                                | Expr::WhyNot(inner)
                                if matches!(inner.as_ref(), Expr::Var(_))
                        )
            )
    )
}

fn count_top_level_modal_kinds(candidate: &Telescope) -> u32 {
    let mut kinds = BTreeSet::new();
    for clause in &candidate.clauses {
        match clause.expr {
            Expr::Flat(_) => {
                kinds.insert(0_u8);
            }
            Expr::Sharp(_) => {
                kinds.insert(1_u8);
            }
            Expr::Disc(_) => {
                kinds.insert(2_u8);
            }
            Expr::Shape(_) => {
                kinds.insert(3_u8);
            }
            _ => {}
        }
    }
    kinds.len() as u32
}

fn expr_node_count(expr: &Expr) -> u32 {
    match expr {
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            1 + expr_node_count(left) + expr_node_count(right)
        }
        Expr::Id(ty, left, right) => {
            1 + expr_node_count(ty) + expr_node_count(left) + expr_node_count(right)
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => 1 + expr_node_count(body),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => 1,
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CertifiedNoveltyError {
    #[error("invalid frozen surface caps: {reason}")]
    InvalidSurfaceCaps { reason: String },
    #[error("candidate is outside the frozen surface: {reason}")]
    SurfaceViolation { reason: String },
    #[error("sealed signature and evaluation library do not replay to the same context")]
    SignatureLibraryMismatch,
    #[error("certificate belongs to a different canonical candidate")]
    CandidateKeyMismatch,
    #[error("transparent certificate must contain exactly one witness per clause")]
    IncompleteTransparentCertificate,
    #[error("trusted transparent-elaboration token does not match the candidate realizers")]
    InvalidTransparentElaborationToken,
    #[error("transparent clause {clause} does not have equal canonical normal forms")]
    TransparentNormalFormsDiffer { clause: u16 },
    #[error("transparent realizer for clause {clause} uses a fresh telescope head")]
    TransparentRealizerUsesFreshHead { clause: u16 },
    #[error("transparent realizer for clause {clause} uses support outside the frozen library")]
    TransparentRealizerOutsideLibrary { clause: u16 },
    #[error("opaque declarations require a nonempty D3 fresh kernel")]
    EmptyFreshKernel,
    #[error("fresh-kernel certificate names a clause outside the candidate")]
    InvalidFreshKernelClause,
    #[error("submitted stuck-eliminator trace differs from normalization")]
    StuckEliminatorTraceMismatch,
    #[error("submitted support differs from normalized direct support")]
    NormalizedSupportMismatch,
    #[error("{amplification:?} requires an explicit {requirement}")]
    MissingAmplificationCertificate {
        amplification: AmplificationKind,
        requirement: &'static str,
    },
    #[error("invalid frozen H-form certificate: {reason}")]
    InvalidHFormCertificate { reason: String },
    #[error(
        "P5 has no unique dominant import; imports {direct_imports:?} have dominant set {dominant_imports:?}"
    )]
    P5NoUniqueDominantImport {
        direct_imports: Vec<u32>,
        dominant_imports: Vec<u32>,
    },
    #[error("invalid frozen P5 certificate: {reason}")]
    InvalidP5Certificate { reason: String },
    #[error("invalid frozen synthesis certificate: {reason}")]
    InvalidSynthesisCertificate { reason: String },
    #[error("certified score {nu} exceeds derived bound {coefficient}*{kappa}")]
    LinearBoundViolation {
        nu: u32,
        coefficient: u32,
        kappa: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::halting::genesis_history;
    use pen_core::clause::{ClauseRec, ClauseRole};

    fn caps() -> CertifiedSurfaceCaps {
        CertifiedSurfaceCaps::genesis_step16()
    }

    fn graph() -> ImportDag {
        ImportDag::genesis_prefix(15)
    }

    fn library_for_signature(signature: &SealedSignature) -> Library {
        let mut library = Library::new();
        for entry in signature.entries() {
            let summary = LibraryEntry::from_telescope(&entry.telescope, &library);
            library.push(summary);
        }
        library
    }

    fn p5_survivor() -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Var(1))),
            ),
        ])
    }

    fn no_formation_hit() -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(1)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ])
    }

    fn formed_hit() -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(1)),
        ])
    }

    fn polymorphic_temporal_pair() -> Telescope {
        let clause = || {
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                    Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                ),
            )
        };
        Telescope::new(vec![clause(), clause()])
    }

    // Test-only semantic fixtures.  Production callers cannot construct
    // these private-field capabilities from an AST.
    fn trusted_transparent_token(candidate: &Telescope) -> TrustedTransparentElaborationToken {
        TrustedTransparentElaborationToken {
            candidate_key: canonical_key_telescope(candidate).0,
            realizer_normal_forms: candidate
                .clauses
                .iter()
                .map(|clause| canonical_key_expr(&clause.expr).0)
                .collect(),
            theorem_id: "test-only:typed-transparent-elaboration".to_owned(),
        }
    }

    fn trusted_h_token(
        candidate: &Telescope,
        formation_clause: u16,
    ) -> TrustedHFormEliminatorToken {
        TrustedHFormEliminatorToken {
            candidate_key: canonical_key_telescope(candidate).0,
            formation_clause,
            theorem_id: "test-only:typed-h-eliminator".to_owned(),
        }
    }

    fn trusted_p5_record_internality(
        signature: &SealedSignature,
        candidate: &Telescope,
        kernel: &FreshKernelCertificate,
    ) -> TrustedP5RecordInternalityCapability {
        TrustedP5RecordInternalityCapability {
            candidate_key: canonical_key_telescope(candidate).0,
            signature_digest: signature.digest().to_owned(),
            api_clauses: kernel.irreducible_clauses.clone(),
            derivation_hash: "test-only:typed-record-internality".to_owned(),
        }
    }

    fn trusted_synthesis_tokens(
        candidate: &Telescope,
    ) -> (
        TrustedSynthesisPolymorphicEliminatorToken,
        TrustedSynthesisNaturalityToken,
    ) {
        let candidate_key = canonical_key_telescope(candidate).0;
        let sites = synthesis_sites(candidate);
        let support = candidate.lib_refs();
        let transports = sites
            .iter()
            .copied()
            .flat_map(|site| {
                support
                    .iter()
                    .copied()
                    .map(move |supported_import| SynthesisTransportSchema {
                        site,
                        supported_import,
                    })
            })
            .collect();
        (
            TrustedSynthesisPolymorphicEliminatorToken {
                candidate_key: candidate_key.clone(),
                sites,
                theorem_id: "test-only:typed-synthesis-eliminator".to_owned(),
            },
            TrustedSynthesisNaturalityToken {
                candidate_key,
                transports,
                theorem_id: "test-only:synthesis-naturality".to_owned(),
            },
        )
    }

    #[test]
    fn normalizer_tracks_support_fresh_heads_and_stuck_eliminators() {
        let (library, _, _) = genesis_history();
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Lib(14))),
            ),
        ]);
        let normalized = NormalizedCandidate::from_telescope(&candidate, &library);

        assert_eq!(normalized.direct_support, [14, 15].into_iter().collect());
        assert_eq!(
            normalized.referenced_fresh_heads,
            [FreshFieldHead {
                introduced_at_clause: 0
            }]
            .into_iter()
            .collect()
        );
        assert_eq!(normalized.stuck_fresh_eliminators.len(), 1);
    }

    #[test]
    fn linear_constant_is_derived_from_caps_and_not_from_the_bar() {
        let theorem = derive_linear_bound(&caps()).expect("valid frozen surface");
        assert_eq!(theorem.coefficient, 4);
        assert!(theorem.every_ceiling_is_bounded);
        assert_eq!(
            theorem
                .per_kappa
                .iter()
                .map(|entry| (entry.kappa, entry.maximum))
                .collect::<Vec<_>>(),
            vec![(2, 8), (3, 11), (4, 14)]
        );
    }

    #[test]
    fn exact_old_library_presentations_are_transparent_and_score_zero() {
        let (library, _, _) = genesis_history();
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(14)),
        ]);
        let certificate = ExtensionCertificate::Transparent(
            TransparentExtensionCertificate::exact(
                &candidate,
                trusted_transparent_token(&candidate),
            )
            .expect("test elaboration token matches"),
        );
        let report =
            evaluate_certified_novelty(
                &candidate,
                &certificate,
                &SealedSignature::genesis_del_h15(),
                &library,
                &graph(),
                &caps(),
            )
                .expect("exact old-library presentation is transparent");

        assert_eq!(report.disposition, CertifiedDisposition::Transparent);
        assert_eq!(report.nu, 0);
        assert!(report.linear_theorem_holds);
    }

    #[test]
    fn transparent_witness_cannot_hide_a_fresh_field_dependency() {
        let (library, _, _) = genesis_history();
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let certificate = ExtensionCertificate::Transparent(
            TransparentExtensionCertificate::exact(
                &candidate,
                trusted_transparent_token(&candidate),
            )
            .expect("test elaboration token matches"),
        );
        assert_eq!(
            evaluate_certified_novelty(
                &candidate,
                &certificate,
                &SealedSignature::genesis_del_h15(),
                &library,
                &graph(),
                &caps(),
            ),
            Err(CertifiedNoveltyError::TransparentRealizerUsesFreshHead { clause: 1 })
        );
    }

    #[test]
    fn transparent_certificate_rejects_a_token_bound_to_other_realizers() {
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(14)),
        ]);
        let other = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(14)),
            ClauseRec::new(ClauseRole::Formation, Expr::Lib(15)),
        ]);
        assert_eq!(
            TransparentExtensionCertificate::exact(&candidate, trusted_transparent_token(&other),),
            Err(TransparentCertificateError::TypedElaborationTokenMismatch)
        );
    }

    #[test]
    fn no_formation_hit_cannot_claim_the_whole_library_bonus() {
        let (library, _, _) = genesis_history();
        let candidate = no_formation_hit();
        let certificate =
            ExtensionCertificate::Opaque(OpaqueExtensionCertificate::local(&candidate, &library));
        assert_eq!(
            evaluate_certified_novelty(
                &candidate,
                &certificate,
                &SealedSignature::genesis_del_h15(),
                &library,
                &graph(),
                &caps(),
            ),
            Err(CertifiedNoveltyError::MissingAmplificationCertificate {
                amplification: AmplificationKind::WholeLibraryHitInheritance,
                requirement: "FrozenHFormCertificate",
            })
        );
        assert_eq!(
            FrozenHFormCertificate::complete(&candidate, trusted_h_token(&candidate, 0)),
            Err(HFormCertificateError::NoFormationClause)
        );
    }

    #[test]
    fn h_formation_expression_with_wrong_clause_role_cannot_mint_a_certificate() {
        let candidate = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(1)),
        ]);
        assert_eq!(
            FrozenHFormCertificate::complete(&candidate, trusted_h_token(&candidate, 0)),
            Err(HFormCertificateError::NoFormationClause)
        );
    }

    #[test]
    fn a_complete_h_basis_gets_only_local_beta_and_kan_credit() {
        let (library, _, _) = genesis_history();
        let candidate = formed_hit();
        let mut opaque = OpaqueExtensionCertificate::local(&candidate, &library);
        opaque.h_form = Some(
            FrozenHFormCertificate::complete(&candidate, trusted_h_token(&candidate, 0))
                .expect("formed dimension-one HIT"),
        );
        let report = evaluate_certified_novelty(
            &candidate,
            &ExtensionCertificate::Opaque(opaque),
            &SealedSignature::genesis_del_h15(),
            &library,
            &graph(),
            &caps(),
        )
        .expect("complete local H basis");

        assert_eq!(report.components.h_basis, 2);
        assert_eq!(report.nu, 4);
        assert!(
            report
                .uncredited_amplifications
                .contains(&AmplificationKind::WholeLibraryHitInheritance)
        );
        assert!(report.linear_theorem_holds);
    }

    #[test]
    fn incomparable_step16_imports_fail_the_p5_domain_before_inheritance() {
        let (library, _, _) = genesis_history();
        let candidate = p5_survivor();
        let certificate =
            ExtensionCertificate::Opaque(OpaqueExtensionCertificate::local(&candidate, &library));
        assert_eq!(
            evaluate_certified_novelty(
                &candidate,
                &certificate,
                &SealedSignature::genesis_del_h15(),
                &library,
                &graph(),
                &caps(),
            ),
            Err(CertifiedNoveltyError::P5NoUniqueDominantImport {
                direct_imports: vec![14, 15],
                dominant_imports: vec![],
            })
        );
    }

    #[test]
    fn one_import_axiomatic_record_still_needs_an_explicit_p5_certificate() {
        let (library, _, _) = genesis_history();
        let candidate = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
            ),
        ]);
        let certificate =
            ExtensionCertificate::Opaque(OpaqueExtensionCertificate::local(&candidate, &library));
        assert_eq!(
            evaluate_certified_novelty(
                &candidate,
                &certificate,
                &SealedSignature::genesis_del_h15(),
                &library,
                &graph(),
                &caps(),
            ),
            Err(CertifiedNoveltyError::MissingAmplificationCertificate {
                amplification: AmplificationKind::HistoricalP5Inheritance,
                requirement: "FrozenP5Certificate",
            })
        );
    }

    #[test]
    fn replayed_kernel_p5_lift_builds_only_the_checked_local_lifts() {
        let signature = SealedSignature::from_telescopes(vec![(
            1,
            Telescope::new(vec![ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ))),
                    Box::new(Expr::Univ),
                ),
            )]),
        )]);
        let candidate = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(
                    Box::new(Expr::Lib(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ))),
                ),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let token = pen_type::elaborate::issue_typed_lift_token(&signature, &candidate, 1)
            .expect("non-vacuous typed lift");
        let kernel = FreshKernelCertificate::assert_all_clauses_opaque(&candidate, &Vec::new());
        let certificate = FrozenP5Certificate::complete_local_from_kernel(
            &signature,
            &candidate,
            1,
            &ImportDag::default(),
            &kernel,
            &token,
            trusted_p5_record_internality(&signature, &candidate, &kernel),
        )
        .expect("replayed kernel token adapts");

        assert_eq!(certificate.dominant_import, 1);
        assert_eq!(certificate.minimal_complete_api, [0, 1].into_iter().collect());
        assert_eq!(certificate.local_lifts.len(), 1);
        assert!(certificate.local_lifts.contains(&P5LocalLift {
            kernel_clause: 0,
            dominant_import: 1,
        }));
        assert_eq!(
            certificate.replayed_typed_lift.kernel_subject_hash(),
            pen_type::elaborate::candidate_hash(&candidate)
        );
        assert_eq!(
            certificate.replayed_typed_lift.signature_digest(),
            signature.digest()
        );
        assert_eq!(
            certificate.replayed_typed_lift.derivation_hash(),
            token.derivation_hash()
        );
        assert_eq!(certificate.replayed_typed_lift.visible_library(), 1);
        assert_eq!(certificate.replayed_typed_lift.dominant_import(), 1);
        assert_eq!(
            certificate.replayed_typed_lift.lift_clauses(),
            &[0].into_iter().collect()
        );
    }

    #[test]
    fn p5_kernel_adapter_rejects_candidate_and_signature_mutations() {
        let signature = SealedSignature::from_telescopes(vec![(
            1,
            Telescope::new(vec![ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ))),
                    Box::new(Expr::Univ),
                ),
            )]),
        )]);
        let candidate = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(
                    Box::new(Expr::Lib(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ))),
                ),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let token = pen_type::elaborate::issue_typed_lift_token(&signature, &candidate, 1)
            .expect("non-vacuous typed lift");

        let mut mutated_candidate = candidate.clone();
        mutated_candidate.clauses[1] =
            ClauseRec::new(ClauseRole::Formation, Expr::Var(1));
        assert!(matches!(
            ReplayedP5LiftCapability::from_kernel_token(
                &signature,
                &mutated_candidate,
                1,
                &token,
            ),
            Err(P5KernelAdapterError::Replay(
                TokenReplayError::SubjectHashMismatch { .. }
            ))
        ));

        let mutated_signature = SealedSignature::from_telescopes(vec![(
            1,
            Telescope::new(vec![ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sigma(Box::new(Expr::Univ), Box::new(Expr::Univ))),
                    Box::new(Expr::Univ),
                ),
            )]),
        )]);
        assert!(matches!(
            ReplayedP5LiftCapability::from_kernel_token(
                &mutated_signature,
                &candidate,
                1,
                &token,
            ),
            Err(P5KernelAdapterError::Replay(
                TokenReplayError::SignatureDigestMismatch { .. }
            ))
        ));
    }

    #[test]
    fn p5_consumer_rejects_signature_library_and_capability_drift() {
        let signature = SealedSignature::from_telescopes(vec![(
            1,
            Telescope::new(vec![ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ))),
                    Box::new(Expr::Univ),
                ),
            )]),
        )]);
        let library = library_for_signature(&signature);
        let candidate = Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(
                    Box::new(Expr::Lib(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ))),
                ),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let token = pen_type::elaborate::issue_typed_lift_token(&signature, &candidate, 1)
            .expect("non-vacuous typed lift");
        let kernel = FreshKernelCertificate::assert_all_clauses_opaque(&candidate, &library);
        let p5 = FrozenP5Certificate::complete_local_from_kernel(
            &signature,
            &candidate,
            1,
            &ImportDag::default(),
            &kernel,
            &token,
            trusted_p5_record_internality(&signature, &candidate, &kernel),
        )
        .expect("both independent test capabilities are present");
        let mut opaque = OpaqueExtensionCertificate::local(&candidate, &library);
        opaque.p5 = Some(p5);
        let certificate = ExtensionCertificate::Opaque(opaque);
        let caps = CertifiedSurfaceCaps {
            min_kappa: 3,
            max_kappa: 3,
            max_direct_support: 1,
            max_path_dimension: 1,
            max_expr_nodes: 6,
            allowed_imports: [1].into_iter().collect(),
            allow_truncation: false,
            allow_modal: false,
            allow_temporal: false,
            allow_linear_exponential: false,
        };

        evaluate_certified_novelty(
            &candidate,
            &certificate,
            &signature,
            &library,
            &ImportDag::default(),
            &caps,
        )
        .expect("matching signature/library context validates");

        assert_eq!(
            evaluate_certified_novelty(
                &candidate,
                &certificate,
                &signature,
                &Library::new(),
                &ImportDag::default(),
                &caps,
            ),
            Err(CertifiedNoveltyError::SignatureLibraryMismatch)
        );

        let mutated_signature = SealedSignature::from_telescopes(vec![(
            1,
            Telescope::new(vec![ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(
                    Box::new(Expr::Sigma(Box::new(Expr::Univ), Box::new(Expr::Univ))),
                    Box::new(Expr::Univ),
                ),
            )]),
        )]);
        let mutated_library = library_for_signature(&mutated_signature);
        assert!(matches!(
            evaluate_certified_novelty(
                &candidate,
                &certificate,
                &mutated_signature,
                &mutated_library,
                &ImportDag::default(),
                &caps,
            ),
            Err(CertifiedNoveltyError::InvalidP5Certificate { .. })
        ));
    }

    #[test]
    fn step13_and_step14_keep_graph_dominance_but_do_not_mint_vacuous_lifts() {
        let graph = graph();
        let signature = SealedSignature::genesis_del_h15();
        for (step, expected) in [(13, 12), (14, 13)] {
            let candidate = Telescope::reference(step);
            assert_eq!(
                P5ImportAudit::check(&candidate, &graph).unique_dominant_import,
                Some(expected)
            );
            assert!(matches!(
                pen_type::elaborate::issue_typed_lift_token(
                    &signature,
                    &candidate,
                    step - 1,
                ),
                Err(pen_type::elaborate::TokenError::NoDominantApplications {
                    dominant_import,
                }) if dominant_import == expected
            ));
        }
    }

    #[test]
    fn polymorphic_temporal_pair_cannot_multiply_by_the_whole_library() {
        let (library, _, _) = genesis_history();
        let candidate = polymorphic_temporal_pair();
        let certificate =
            ExtensionCertificate::Opaque(OpaqueExtensionCertificate::local(&candidate, &library));
        assert_eq!(
            evaluate_certified_novelty(
                &candidate,
                &certificate,
                &SealedSignature::genesis_del_h15(),
                &library,
                &graph(),
                &caps(),
            ),
            Err(CertifiedNoveltyError::MissingAmplificationCertificate {
                amplification: AmplificationKind::WholeLibrarySynthesisMultiplier,
                requirement: "FrozenSynthesisCertificate",
            })
        );
    }

    #[test]
    fn synthesis_certificate_has_no_transport_without_direct_support() {
        let (library, _, _) = genesis_history();
        let candidate = polymorphic_temporal_pair();
        let mut opaque = OpaqueExtensionCertificate::local(&candidate, &library);
        let (typed, naturality) = trusted_synthesis_tokens(&candidate);
        opaque.synthesis = Some(
            FrozenSynthesisCertificate::complete_support_matrix(&candidate, typed, naturality)
                .expect("test semantic tokens match"),
        );
        let report = evaluate_certified_novelty(
            &candidate,
            &ExtensionCertificate::Opaque(opaque),
            &SealedSignature::genesis_del_h15(),
            &library,
            &graph(),
            &caps(),
        )
        .expect("empty support matrix is exact");

        assert_eq!(report.components.synthesis_transports, 0);
        assert_eq!(report.nu, 2);
        assert!(report.linear_theorem_holds);
    }

    #[test]
    fn synthesis_credit_is_the_exact_site_by_direct_support_matrix() {
        let (library, _, _) = genesis_history();
        let poly = Expr::Pi(
            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
        );
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, poly),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(15)),
                    Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                ))),
            ),
        ]);
        let mut opaque = OpaqueExtensionCertificate::local(&candidate, &library);
        let (typed, naturality) = trusted_synthesis_tokens(&candidate);
        opaque.synthesis = Some(
            FrozenSynthesisCertificate::complete_support_matrix(&candidate, typed, naturality)
                .expect("test semantic tokens match"),
        );
        let report = evaluate_certified_novelty(
            &candidate,
            &ExtensionCertificate::Opaque(opaque),
            &SealedSignature::genesis_del_h15(),
            &library,
            &graph(),
            &caps(),
        )
        .expect("two sites by one direct import");

        assert_eq!(report.components.synthesis_transports, 2);
        assert_eq!(report.nu, 4);
        assert!(report.linear_theorem_holds);
    }

    #[test]
    fn synthesis_certificate_rejects_a_mismatched_naturality_token() {
        let candidate = polymorphic_temporal_pair();
        let (typed, mut naturality) = trusted_synthesis_tokens(&candidate);
        naturality.transports.insert(SynthesisTransportSchema {
            site: *synthesis_sites(&candidate)
                .iter()
                .next()
                .expect("polymorphic site"),
            supported_import: 15,
        });
        assert_eq!(
            FrozenSynthesisCertificate::complete_support_matrix(&candidate, typed, naturality,),
            Err(SynthesisCertificateError::NaturalityTokenMismatch)
        );
    }

    #[test]
    fn forged_support_or_stuck_trace_is_rejected() {
        let (library, _, _) = genesis_history();
        let candidate = p5_survivor();
        let mut opaque = OpaqueExtensionCertificate::local(&candidate, &library);
        opaque.kernel.normalized_support.clear();
        assert_eq!(
            evaluate_certified_novelty(
                &candidate,
                &ExtensionCertificate::Opaque(opaque),
                &SealedSignature::genesis_del_h15(),
                &library,
                &graph(),
                &caps(),
            ),
            Err(CertifiedNoveltyError::NormalizedSupportMismatch)
        );
    }

    #[test]
    fn every_successful_report_obeys_the_independently_derived_bound() {
        let (library, _, _) = genesis_history();
        let candidates = [
            Telescope::new(vec![
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Lib(14))),
                ),
                ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Pi(Box::new(Expr::Lib(14)), Box::new(Expr::Lib(15))),
                ),
            ]),
            formed_hit(),
        ];

        let map_certificate = ExtensionCertificate::Opaque(OpaqueExtensionCertificate::local(
            &candidates[0],
            &library,
        ));
        let mut hit_opaque = OpaqueExtensionCertificate::local(&candidates[1], &library);
        hit_opaque.h_form = Some(
            FrozenHFormCertificate::complete(&candidates[1], trusted_h_token(&candidates[1], 0))
                .expect("formed HIT"),
        );
        let hit_certificate = ExtensionCertificate::Opaque(hit_opaque);

        for (candidate, certificate) in candidates.iter().zip([map_certificate, hit_certificate]) {
            let report = evaluate_certified_novelty(
                candidate,
                &certificate,
                &SealedSignature::genesis_del_h15(),
                &library,
                &graph(),
                &caps(),
            )
            .expect("certified candidate");
            assert!(report.nu <= report.derived_linear_bound);
            assert_eq!(report.derived_linear_coefficient, 4);
        }
    }
}
