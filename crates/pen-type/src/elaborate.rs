//! Phase 1 kernel of `docs/SEMANTIC_NORMALIZATION_PROGRAM.md`: a bounded
//! bidirectional elaborator over the sealed library signature `∂H_15`, with
//! weak-head reduction, kernel stuckness verdicts, a certified static fuel
//! bound, and derivation-backed token issuance.
//!
//! Scope fence (program §1): this is NOT a general dependent type checker.
//! It elaborates the finite fragment reachable within the frozen caps over
//! the sealed signature. Library entries enter as opaque constants with
//! their exported formation, eliminator, and computation clauses only; no
//! sealed entry exports a `computation`-role clause, so the only oriented
//! computation clause is generic beta, and every other application head is
//! stuck by construction — a kernel verdict, not an AST pattern.
//!
//! Kernel v1 conventions (frozen; see `normalize.rs` for binding):
//! - `Var` levels resolve against `[ambient parameters, prior fields,
//!   binders]`; the kernel elaborates each telescope in the SMALLEST
//!   ambient context that scopes it, capped at 2 (checker discipline).
//! - A field reference to a `Formation` clause has kernel type `Type`
//!   (the declared type as an opaque object); all other field references
//!   are opaque (`Neutral`).
//! - Coarse subsumptions (accepting an opaque classifier where `Type` or a
//!   domain match is demanded) are counted per clause and recorded in the
//!   derivation; token issuance requires zero coarse uses at the positions
//!   it certifies.
//!
//! Token discipline (program §0.4): token structs have private fields and
//! no public constructors. Issuance functions build tokens only from
//! successful kernel derivations; each token embeds the BLAKE3 hash of its
//! derivation and the signature digest it was issued against, and replay
//! re-derives instead of trusting.

use crate::equality::{EqualityWitness, KERNEL_EQUALITY_PROCEDURE, judgmental_equality};
use crate::normalize::{
    KERNEL_BINDING_CONVENTION, NormalizeError, normalize, substitute_level, whnf,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::stats::StructuralStats;
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

/// Version tag for the elaboration rules. Any change to the kernel rules
/// must bump this tag; the hash participates in the resume/replay compat
/// gate (program ground rule 3).
pub const ELABORATOR_VERSION_TAG: &str = "elaborator-v1";

/// Version tag for the token issuance rules (program ground rule 3).
pub const TOKEN_RULES_VERSION_TAG: &str = "token-rules-v2";

/// Checker discipline: at most two ambient parameters (mirrors
/// `check.rs`'s hardcoded maximum).
pub const MAX_AMBIENT_PARAMETERS: u32 = 2;

pub fn elaborator_hash() -> String {
    format!("blake3:{}", blake3_hex(ELABORATOR_VERSION_TAG.as_bytes()))
}

pub fn token_rules_hash() -> String {
    format!("blake3:{}", blake3_hex(TOKEN_RULES_VERSION_TAG.as_bytes()))
}

/// The sealed Genesis winners' candidate hashes, copied verbatim from
/// `runs/step15-live/checkpoints/steps/step-NN.json` (`accepted.candidate_hash`).
/// `SealedSignature::genesis_del_h15` re-derives each hash from
/// `Telescope::reference` and fails closed on any drift.
const SEALED_CANDIDATE_HASHES: [(u32, &str); 15] = [
    (
        1,
        "blake3:61631d63a5877aad1b32b1e71aa6f0e555317b65732f327e3f38e32c222e29e7",
    ),
    (
        2,
        "blake3:a2dfff0fb8ce1073119da3893e1d195247c234af66b9a7de17c85d5cf718f555",
    ),
    (
        3,
        "blake3:934b5599bb0f28abf0be9652982caec5e0ff6d7d204ddaa4e66f23c37155457e",
    ),
    (
        4,
        "blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407",
    ),
    (
        5,
        "blake3:043c7990d42d91f972544a3cbcae33ca65e48eaab82d605a15cb505ed0ad26d0",
    ),
    (
        6,
        "blake3:a2ff7bc69a678e8c257824465993f8aadf71c05676562887da12dce3ae6cfe1d",
    ),
    (
        7,
        "blake3:30f190b67aab5179a6de8dccee6c29c699b80c12bef676bf2e9f9377595a7906",
    ),
    (
        8,
        "blake3:e01de7b95b0dda56a1062add33387a37709e23d3d6ea6ce4efaf27adacb868c0",
    ),
    (
        9,
        "blake3:f57e3a5aa44003adb5f8054013e549e4f3065757d88922313478e8e445825491",
    ),
    (
        10,
        "blake3:93289041755cf4c4e0396029273822630028999d945359b10bb88e2376b9788e",
    ),
    (
        11,
        "blake3:03cc71839428ceab088e386e31e6c72b7ca4a1e12c9309557efc76826dcb88cd",
    ),
    (
        12,
        "blake3:0d06e3b14bfd7c1bd16d9f66039e016f84a6162ce85584f7c641753b833b1dcb",
    ),
    (
        13,
        "blake3:b09b3f832bef16953747c213e6b8a548f594339dc7db9148e7ae60a6c53a4cf1",
    ),
    (
        14,
        "blake3:1ff4820f2272c022a5fece1032aa9647e41167a3dd8f62765e84e22f5d90b19c",
    ),
    (
        15,
        "blake3:e919c8419bbafde89e3e99ff25f348f3c8b679ed22685e84d3cdec634ebf90d4",
    ),
];

/// Candidate hash exactly as the search engine computes it
/// (`pen-search/src/expand.rs`): BLAKE3 of the serde-JSON telescope.
pub fn candidate_hash(telescope: &Telescope) -> String {
    let bytes = serde_json::to_vec(telescope).expect("telescope serialization is infallible");
    format!("blake3:{}", blake3_hex(&bytes))
}

/// One opaque constant of the sealed signature with its derived exported
/// clause views. `computation`-role clauses never occur in sealed data;
/// the count is retained so the invariant is checked, not assumed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedEntry {
    pub step: u32,
    pub candidate_hash: String,
    pub telescope: Telescope,
    pub formation_clauses: Vec<u16>,
    pub beta_clauses: Vec<u16>,
    pub kan_dimensions: Vec<u32>,
    pub exported_computation_clauses: u16,
    pub direct_imports: Vec<u32>,
}

/// The sealed library signature. Entry `k` (0-based) is `Lib(k + 1)`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedSignature {
    entries: Vec<SealedEntry>,
    digest: String,
}

impl SealedSignature {
    /// The sealed Genesis signature `∂H_15`, rebuilt from the compiled-in
    /// reference telescopes and verified clause-for-clause against the
    /// frozen checkpoint candidate hashes. Panics on drift: a build whose
    /// reference data no longer matches the sealed snapshot must not limp.
    pub fn genesis_del_h15() -> Self {
        let telescopes: Vec<(u32, Telescope)> = Telescope::all_reference_telescopes();
        let signature = Self::from_telescopes(telescopes);
        for (step, frozen) in SEALED_CANDIDATE_HASHES {
            let entry = signature
                .entry(step)
                .expect("genesis signature has fifteen entries");
            assert_eq!(
                entry.candidate_hash, frozen,
                "sealed candidate hash drift at step {step}: reference telescope no \
                 longer matches the frozen checkpoint",
            );
        }
        signature
    }

    /// Build a signature from explicit telescopes. Tokens embed the
    /// signature digest, so a signature built from mutated telescopes
    /// yields tokens that fail replay against the genesis signature.
    pub fn from_telescopes(telescopes: Vec<(u32, Telescope)>) -> Self {
        let entries: Vec<SealedEntry> = telescopes
            .into_iter()
            .map(|(step, telescope)| derive_entry(step, telescope))
            .collect();
        let digest_payload = serde_json::json!({
            "binding": KERNEL_BINDING_CONVENTION,
            "equality": KERNEL_EQUALITY_PROCEDURE,
            "elaborator": ELABORATOR_VERSION_TAG,
            "entries": entries
                .iter()
                .map(|entry| (entry.step, entry.candidate_hash.clone()))
                .collect::<Vec<_>>(),
        });
        let digest = format!(
            "blake3:{}",
            blake3_hex(&serde_json::to_vec(&digest_payload).expect("digest payload serialization"),)
        );
        Self { entries, digest }
    }

    pub fn entry(&self, step: u32) -> Option<&SealedEntry> {
        self.entries.iter().find(|entry| entry.step == step)
    }

    pub fn entries(&self) -> &[SealedEntry] {
        &self.entries
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Reflexive-transitive closure of the direct-import relation,
    /// derived from the sealed telescopes themselves.
    pub fn reachable_from(&self, step: u32) -> BTreeSet<u32> {
        let mut reached = BTreeSet::new();
        let mut frontier = vec![step];
        while let Some(current) = frontier.pop() {
            if !reached.insert(current) {
                continue;
            }
            if let Some(entry) = self.entry(current) {
                for import in &entry.direct_imports {
                    if !reached.contains(import) {
                        frontier.push(*import);
                    }
                }
            }
        }
        reached
    }
}

fn derive_entry(step: u32, telescope: Telescope) -> SealedEntry {
    let hash = candidate_hash(&telescope);
    let mut formation_clauses = Vec::new();
    let mut beta_clauses = Vec::new();
    let mut kan_dimensions = Vec::new();
    let mut exported_computation_clauses = 0u16;
    for (index, clause) in telescope.clauses.iter().enumerate() {
        let index = u16::try_from(index).expect("clause index fits u16");
        if clause.role == ClauseRole::Computation {
            exported_computation_clauses += 1;
        }
        match &clause.expr {
            Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)) => {
                beta_clauses.push(index);
            }
            Expr::PathCon(dimension) => kan_dimensions.push(*dimension),
            _ => {}
        }
        if clause.role == ClauseRole::Formation {
            formation_clauses.push(index);
        }
    }
    let direct_imports: Vec<u32> = telescope.lib_refs().into_iter().collect();
    SealedEntry {
        step,
        candidate_hash: hash,
        telescope,
        formation_clauses,
        beta_clauses,
        kan_dimensions,
        exported_computation_clauses,
        direct_imports,
    }
}

/// Kernel classifier assigned to every elaborated expression.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum KernelTy {
    /// The classifier of types.
    Type,
    /// An inhabitant of the (weak-head normal) type expression.
    El(Expr),
    /// A function reading with unannotated pieces kept opaque.
    Fun(Box<KernelTy>, Box<KernelTy>),
    /// A path-constructor declaration of the given dimension.
    PathDecl { dimension: u32 },
    /// Opaque / unknown — every use as something more specific is a
    /// counted coarse subsumption.
    Neutral,
}

/// How a `Var` level resolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum ScopeRef {
    Ambient { parameter: u32 },
    Field { clause: u16 },
    Local { binder: u32 },
}

/// Kernel stuckness verdict for an application head.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum StuckHead {
    /// `Next` / `Eventually` — the temporal pair has no exported
    /// eliminator: stuck by construction (program §1).
    TemporalFormer { atom: String },
    /// `Flat` / `Sharp` / `Disc` / `Shape`.
    ModalFormer { atom: String },
    /// Any other type former used as a function head.
    OtherFormer { atom: String },
    /// `App(Lib n, _)` — the entry exports no computation clause.
    LibraryConstant { step: u32 },
    /// Applying an opaque field of the same telescope.
    FieldReference { clause: u16 },
    /// Neutral application on an ambient parameter (not fresh-stuck).
    AmbientParameter { parameter: u32 },
    /// Neutral application on a local binder variable (not fresh-stuck).
    LocalVariable { binder: u32 },
}

impl StuckHead {
    /// Fresh-stuck means the head has no exported eliminator or
    /// computation clause; neutral variable heads are ordinary neutrals.
    pub fn is_fresh_stuck(&self) -> bool {
        !matches!(
            self,
            StuckHead::AmbientParameter { .. } | StuckHead::LocalVariable { .. }
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StuckApplication {
    pub clause_index: u16,
    pub head: StuckHead,
}

/// One node of an elaboration derivation. Serialized verbatim into the
/// derivation hash; replay re-runs the rules and compares.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DerivationNode {
    pub rule: String,
    pub kernel_ty: KernelTy,
    pub coarse: bool,
    pub children: Vec<DerivationNode>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum ElabError {
    #[error("variable level {level} exceeds scope size {scope_len}")]
    ScopeViolation { level: u32, scope_len: u32 },
    #[error("required ambient context {required} exceeds kernel maximum {max}")]
    AmbientContextTooLarge { required: u32, max: u32 },
    #[error("library reference {step} is outside the visible signature prefix {visible}")]
    LibOutOfSignature { step: u32, visible: u32 },
    #[error("bare Univ cannot be used as an application argument")]
    BareUnivArgument,
    #[error("linear-exponential constructors are outside the frozen v1 alphabet")]
    LinearExponentialOutsideFrozenAlphabet,
    #[error("normalization failed: {0}")]
    Normalize(NormalizeError),
}

impl From<NormalizeError> for ElabError {
    fn from(error: NormalizeError) -> Self {
        Self::Normalize(error)
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
#[error("clause {clause_index}: {error}")]
pub struct ClauseFailure {
    pub clause_index: u16,
    pub error: ElabError,
}

/// Per-clause elaboration record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClauseElaboration {
    pub clause_index: u16,
    pub declared_role: ClauseRole,
    pub kernel_role: ClauseRole,
    pub kernel_ty: KernelTy,
    pub normal_form: Expr,
    pub beta_steps: u32,
    /// Synthesis-time beta plus normalization steps for this clause.  This
    /// observation is kept out of the legacy serialized derivation payload so
    /// existing token/artifact hashes remain unchanged; the additive fuel
    /// composition token serializes it separately.
    #[serde(skip_serializing)]
    pub total_fuel_observed: u32,
    pub is_beta_redex_clause: bool,
    pub stuck_applications: Vec<StuckApplication>,
    pub coarse_assumptions: u32,
    pub derivation: DerivationNode,
}

/// The certified static fuel bound (program §1): measure is node count
/// times clause count; the grader checks `observed <= bound`.
/// `total_fuel_observed` counts every evaluation step the kernel performs
/// (both synthesis-time beta and clause normalization).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FuelCertificate {
    pub static_bound: u32,
    pub max_fuel_observed: u32,
    pub total_fuel_observed: u32,
    pub within_bound: bool,
}

/// Full elaboration of one telescope against a visible signature prefix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TelescopeElaboration {
    pub subject_hash: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub ambient_parameters: u32,
    pub clauses: Vec<ClauseElaboration>,
    pub fuel: FuelCertificate,
    pub derivation_hash: String,
}

impl TelescopeElaboration {
    pub fn stuck_applications(&self) -> Vec<&StuckApplication> {
        self.clauses
            .iter()
            .flat_map(|clause| clause.stuck_applications.iter())
            .collect()
    }
}

struct ElabState {
    visible_library: u32,
    ambient: u32,
    /// Exact classifiers for a versioned dependent ambient context.  Legacy
    /// callers leave this absent and retain the frozen all-`Type` ambient
    /// interpretation byte-for-byte.
    ambient_types: Option<Vec<KernelTy>>,
    prior_roles: Vec<ClauseRole>,
    locals: Vec<KernelTy>,
    clause_index: u16,
    stuck: Vec<StuckApplication>,
    coarse: u32,
    fuel_budget: u32,
    fuel_used: u32,
}

impl ElabState {
    fn scope_len(&self) -> u32 {
        self.ambient + self.prior_roles.len() as u32 + self.locals.len() as u32
    }

    fn resolve(&self, level: u32) -> Option<ScopeRef> {
        let priors = self.prior_roles.len() as u32;
        let locals = self.locals.len() as u32;
        if level == 0 {
            None
        } else if level <= self.ambient {
            Some(ScopeRef::Ambient { parameter: level })
        } else if level <= self.ambient + priors {
            Some(ScopeRef::Field {
                clause: (level - self.ambient - 1) as u16,
            })
        } else if level <= self.ambient + priors + locals {
            Some(ScopeRef::Local {
                binder: level - self.ambient - priors,
            })
        } else {
            None
        }
    }

    fn remaining_fuel(&self) -> u32 {
        self.fuel_budget.saturating_sub(self.fuel_used)
    }

    fn spend(&mut self, steps: u32) {
        self.fuel_used = self.fuel_used.saturating_add(steps);
    }

    fn note_coarse(&mut self) {
        self.coarse = self.coarse.saturating_add(1);
    }
}

/// Minimal ambient context required to scope `expr` at the given number of
/// prior fields, under the kernel binder discipline (`Lam` bodies and
/// `Pi`/`Sigma` codomains bind; unary operators do not).
fn required_ambient(expr: &Expr, priors: u32, locals: u32) -> u32 {
    match expr {
        Expr::Var(level) => level.saturating_sub(priors + locals),
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => 0,
        Expr::App(function, argument) => required_ambient(function, priors, locals)
            .max(required_ambient(argument, priors, locals)),
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => required_ambient(
            domain, priors, locals,
        )
        .max(required_ambient(codomain, priors, locals + 1)),
        Expr::Lam(body) => required_ambient(body, priors, locals + 1),
        Expr::Id(ty, left, right) => required_ambient(ty, priors, locals)
            .max(required_ambient(left, priors, locals))
            .max(required_ambient(right, priors, locals)),
        Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => required_ambient(inner, priors, locals),
    }
}

/// Minimal ambient context for a whole telescope (clause `k` sees `k`
/// prior fields).
pub fn minimal_ambient_parameters(telescope: &Telescope) -> u32 {
    telescope
        .clauses
        .iter()
        .enumerate()
        .map(|(index, clause)| required_ambient(&clause.expr, index as u32, 0))
        .max()
        .unwrap_or(0)
}

/// The ambient shortfall of one clause expression at `priors` prior
/// fields: the minimal ambient context it demands on its own.
pub fn required_clause_ambient(expr: &Expr, priors: u32) -> u32 {
    required_ambient(expr, priors, 0)
}

/// A single clause elaborated in an EXPLICIT context (ambient size and
/// prior kernel roles), for streaming counters that cannot afford whole
/// telescopes. Same rules as `elaborate_telescope`; the caller supplies
/// the ambient (which for a full telescope is the max clause shortfall).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SingleClauseElaboration {
    pub kernel_ty: KernelTy,
    pub kernel_role: ClauseRole,
    pub normal_form: Expr,
    pub beta_steps: u32,
    /// Synthesis-time beta plus normalization steps.
    #[serde(skip_serializing)]
    pub total_fuel_observed: u32,
    pub coarse_assumptions: u32,
    pub stuck_applications: Vec<StuckApplication>,
}

pub fn elaborate_single_clause(
    expr: &Expr,
    ambient: u32,
    prior_roles: &[ClauseRole],
    visible_library: u32,
) -> Result<SingleClauseElaboration, ElabError> {
    elaborate_single_clause_with_derivation(expr, ambient, prior_roles, visible_library)
        .map(|(summary, _)| summary)
}

/// Exact single-clause elaboration in a caller-declared ambient context,
/// retaining the kernel derivation for replayable contextual audits.  This is
/// additive: the legacy summary API above remains byte-for-byte compatible.
pub fn elaborate_single_clause_with_derivation(
    expr: &Expr,
    ambient: u32,
    prior_roles: &[ClauseRole],
    visible_library: u32,
) -> Result<(SingleClauseElaboration, DerivationNode), ElabError> {
    if ambient > MAX_AMBIENT_PARAMETERS {
        return Err(ElabError::AmbientContextTooLarge {
            required: ambient,
            max: MAX_AMBIENT_PARAMETERS,
        });
    }
    let node_count = StructuralStats::from_expr(expr).node_count;
    let fuel = node_count
        .saturating_mul((prior_roles.len() as u32 + 1).max(1))
        .max(16);
    let mut state = ElabState {
        visible_library,
        ambient,
        ambient_types: None,
        prior_roles: prior_roles.to_vec(),
        locals: Vec::new(),
        clause_index: prior_roles.len() as u16,
        stuck: Vec::new(),
        coarse: 0,
        fuel_budget: fuel,
        fuel_used: 0,
    };
    let (kernel_ty, derivation) = synth(&mut state, expr)?;
    let scope_len = ambient + prior_roles.len() as u32;
    let normalized = normalize(expr, scope_len, state.remaining_fuel())?;
    let is_beta_redex =
        matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)));
    let kernel_role = derive_kernel_role(expr, &kernel_ty, is_beta_redex);
    Ok((
        SingleClauseElaboration {
            kernel_ty,
            kernel_role,
            normal_form: normalized.expr,
            beta_steps: normalized.steps,
            total_fuel_observed: state.fuel_used.saturating_add(normalized.steps),
            coarse_assumptions: state.coarse,
            stuck_applications: state.stuck,
        },
        derivation,
    ))
}

/// Exact single-clause elaboration in a caller-declared typed ambient
/// context.  This is the versioned dependent-context entry point: unlike the
/// legacy API it has no arity-two implementation cap, and an ambient
/// projection synthesizes the exact classifier supplied for that hypothesis.
/// It does not alter whole-telescope elaboration or any archived kernel run.
pub fn elaborate_single_clause_with_typed_ambient(
    expr: &Expr,
    ambient_types: &[KernelTy],
    prior_roles: &[ClauseRole],
    visible_library: u32,
) -> Result<(SingleClauseElaboration, DerivationNode), ElabError> {
    let ambient =
        u32::try_from(ambient_types.len()).map_err(|_| ElabError::AmbientContextTooLarge {
            required: u32::MAX,
            max: u32::MAX,
        })?;
    let node_count = StructuralStats::from_expr(expr).node_count;
    let fuel = node_count
        .saturating_mul((prior_roles.len() as u32 + 1).max(1))
        .max(16);
    let mut state = ElabState {
        visible_library,
        ambient,
        ambient_types: Some(ambient_types.to_vec()),
        prior_roles: prior_roles.to_vec(),
        locals: Vec::new(),
        clause_index: prior_roles.len() as u16,
        stuck: Vec::new(),
        coarse: 0,
        fuel_budget: fuel,
        fuel_used: 0,
    };
    let (kernel_ty, derivation) = synth(&mut state, expr)?;
    let scope_len = ambient + prior_roles.len() as u32;
    let normalized = normalize(expr, scope_len, state.remaining_fuel())?;
    let is_beta_redex =
        matches!(expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)));
    let kernel_role = derive_kernel_role(expr, &kernel_ty, is_beta_redex);
    Ok((
        SingleClauseElaboration {
            kernel_ty,
            kernel_role,
            normal_form: normalized.expr,
            beta_steps: normalized.steps,
            total_fuel_observed: state.fuel_used.saturating_add(normalized.steps),
            coarse_assumptions: state.coarse,
            stuck_applications: state.stuck,
        },
        derivation,
    ))
}

/// Elaborate a telescope against the visible prefix of the sealed
/// signature. Total over the frozen fragment: success carries derivations
/// and verdicts (including stuckness); failure names the exact clause and
/// violation. `visible_library` is the number of signature steps the
/// telescope may reference (15 for Step-16 candidates; `step - 1` for a
/// sealed entry's own clauses).
pub fn elaborate_telescope(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
) -> Result<TelescopeElaboration, ClauseFailure> {
    let ambient = minimal_ambient_parameters(telescope);
    if ambient > MAX_AMBIENT_PARAMETERS {
        return Err(ClauseFailure {
            clause_index: 0,
            error: ElabError::AmbientContextTooLarge {
                required: ambient,
                max: MAX_AMBIENT_PARAMETERS,
            },
        });
    }

    let stats = StructuralStats::from_telescope(telescope);
    let kappa = telescope.kappa() as u32;
    let static_bound = stats.node_count.saturating_mul(kappa.max(1));

    let mut state = ElabState {
        visible_library,
        ambient,
        ambient_types: None,
        prior_roles: Vec::new(),
        locals: Vec::new(),
        clause_index: 0,
        stuck: Vec::new(),
        coarse: 0,
        fuel_budget: static_bound,
        fuel_used: 0,
    };

    let mut clauses = Vec::with_capacity(telescope.clauses.len());
    let mut max_clause_fuel = 0u32;
    for (index, clause) in telescope.clauses.iter().enumerate() {
        let clause_index = u16::try_from(index).expect("clause index fits u16");
        state.clause_index = clause_index;
        state.stuck = Vec::new();
        state.coarse = 0;
        let fuel_before = state.fuel_used;

        let (kernel_ty, derivation) =
            synth(&mut state, &clause.expr).map_err(|error| ClauseFailure {
                clause_index,
                error,
            })?;

        let scope_len = state.ambient + state.prior_roles.len() as u32;
        let normalized =
            normalize(&clause.expr, scope_len, state.remaining_fuel()).map_err(|error| {
                ClauseFailure {
                    clause_index,
                    error: error.into(),
                }
            })?;
        state.spend(normalized.steps);

        let clause_fuel = state.fuel_used - fuel_before;
        max_clause_fuel = max_clause_fuel.max(clause_fuel);

        let is_beta_redex_clause = matches!(&clause.expr, Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)));
        let kernel_role = derive_kernel_role(&clause.expr, &kernel_ty, is_beta_redex_clause);

        clauses.push(ClauseElaboration {
            clause_index,
            declared_role: clause.role,
            kernel_role,
            kernel_ty,
            normal_form: normalized.expr,
            beta_steps: normalized.steps,
            total_fuel_observed: clause_fuel,
            is_beta_redex_clause,
            stuck_applications: std::mem::take(&mut state.stuck),
            coarse_assumptions: state.coarse,
            derivation,
        });
        // Field typing must depend only on expressions: push the
        // kernel-derived role, never the caller-declared role byte, so a
        // role-bit perturbation cannot change any later clause's
        // classifier or coarse count (fail-open hazard).
        state.prior_roles.push(kernel_role);
    }

    let fuel = FuelCertificate {
        static_bound,
        max_fuel_observed: max_clause_fuel,
        total_fuel_observed: state.fuel_used,
        within_bound: state.fuel_used <= static_bound,
    };

    let derivation_payload = serde_json::json!({
        "binding": KERNEL_BINDING_CONVENTION,
        "equality": KERNEL_EQUALITY_PROCEDURE,
        "elaborator": ELABORATOR_VERSION_TAG,
        "signature_digest": signature.digest(),
        "visible_library": visible_library,
        "ambient": ambient,
        "clauses": clauses,
        "fuel": fuel,
    });
    let derivation_hash = format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&derivation_payload).expect("derivation serialization"))
    );

    Ok(TelescopeElaboration {
        subject_hash: candidate_hash(telescope),
        signature_digest: signature.digest().to_string(),
        visible_library,
        ambient_parameters: ambient,
        clauses,
        fuel,
        derivation_hash,
    })
}

fn derive_kernel_role(expr: &Expr, kernel_ty: &KernelTy, is_beta_redex: bool) -> ClauseRole {
    match expr {
        Expr::PathCon(_) => ClauseRole::PathAttach,
        Expr::App(_, _) if is_beta_redex => ClauseRole::Elimination,
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ) => ClauseRole::Formation,
        Expr::App(_, _) => ClauseRole::Introduction,
        Expr::Lam(_) | Expr::Var(_) | Expr::Refl(_) => ClauseRole::Introduction,
        _ => match kernel_ty {
            KernelTy::Type => ClauseRole::Formation,
            _ => ClauseRole::Formation,
        },
    }
}

fn node(
    rule: &str,
    kernel_ty: KernelTy,
    coarse: bool,
    children: Vec<DerivationNode>,
) -> DerivationNode {
    DerivationNode {
        rule: rule.to_string(),
        kernel_ty,
        coarse,
        children,
    }
}

/// Coerce a synthesized classifier to `Type` in a formation position,
/// counting a coarse subsumption when the classifier is opaque.
fn as_type(state: &mut ElabState, ty: &KernelTy) -> bool {
    match ty {
        KernelTy::Type => false,
        _ => {
            state.note_coarse();
            true
        }
    }
}

fn synth(state: &mut ElabState, expr: &Expr) -> Result<(KernelTy, DerivationNode), ElabError> {
    match expr {
        Expr::Univ => Ok((
            KernelTy::Type,
            node("univ-form", KernelTy::Type, false, vec![]),
        )),
        Expr::Var(level) => match state.resolve(*level) {
            Some(ScopeRef::Ambient { parameter }) => {
                let ty = state
                    .ambient_types
                    .as_ref()
                    .and_then(|types| types.get(parameter.saturating_sub(1) as usize))
                    .cloned()
                    .unwrap_or(KernelTy::Type);
                Ok((
                    ty.clone(),
                    node(&format!("ambient-param-{parameter}"), ty, false, vec![]),
                ))
            }
            Some(ScopeRef::Field { clause }) => {
                let ty = if state
                    .prior_roles
                    .get(clause as usize)
                    .is_some_and(|role| *role == ClauseRole::Formation)
                {
                    KernelTy::Type
                } else {
                    KernelTy::Neutral
                };
                Ok((
                    ty.clone(),
                    node(&format!("field-ref-{clause}"), ty, false, vec![]),
                ))
            }
            Some(ScopeRef::Local { binder }) => {
                let ty = state.locals[(binder - 1) as usize].clone();
                Ok((
                    ty.clone(),
                    node(&format!("local-var-{binder}"), ty, false, vec![]),
                ))
            }
            None => Err(ElabError::ScopeViolation {
                level: *level,
                scope_len: state.scope_len(),
            }),
        },
        Expr::Lib(step) => {
            if *step == 0 || *step > state.visible_library {
                Err(ElabError::LibOutOfSignature {
                    step: *step,
                    visible: state.visible_library,
                })
            } else {
                Ok((
                    KernelTy::Type,
                    node("library-constant", KernelTy::Type, false, vec![]),
                ))
            }
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            let rule = if matches!(expr, Expr::Pi(_, _)) {
                "pi-form"
            } else {
                "sigma-form"
            };
            let (domain_ty, domain_node) = synth(state, domain)?;
            let domain_coarse = as_type(state, &domain_ty);
            let domain_whnf = whnf(domain, state.scope_len(), state.remaining_fuel())?;
            state.spend(domain_whnf.steps);
            state.locals.push(KernelTy::El(domain_whnf.expr));
            let codomain_result = synth(state, codomain);
            state.locals.pop();
            let (codomain_ty, codomain_node) = codomain_result?;
            let codomain_coarse = as_type(state, &codomain_ty);
            Ok((
                KernelTy::Type,
                node(
                    rule,
                    KernelTy::Type,
                    domain_coarse || codomain_coarse,
                    vec![domain_node, codomain_node],
                ),
            ))
        }
        Expr::Lam(body) => {
            // Unannotated domain: the binder is opaque, which is itself a
            // counted coarse assumption.
            state.note_coarse();
            state.locals.push(KernelTy::Neutral);
            let body_result = synth(state, body);
            state.locals.pop();
            let (body_ty, body_node) = body_result?;
            let ty = KernelTy::Fun(Box::new(KernelTy::Neutral), Box::new(body_ty));
            Ok((ty.clone(), node("lam-intro", ty, true, vec![body_node])))
        }
        Expr::App(function, argument) => synth_app(state, function, argument),
        Expr::Id(ty, left, right) => {
            let (ty_ty, ty_node) = synth(state, ty)?;
            let coarse_ty = as_type(state, &ty_ty);
            let (_, left_node) = synth(state, left)?;
            let (_, right_node) = synth(state, right)?;
            Ok((
                KernelTy::Type,
                node(
                    "id-form",
                    KernelTy::Type,
                    coarse_ty,
                    vec![ty_node, left_node, right_node],
                ),
            ))
        }
        Expr::Refl(inner) => {
            let (_, inner_node) = synth(state, inner)?;
            Ok((
                KernelTy::Neutral,
                node("refl-intro", KernelTy::Neutral, false, vec![inner_node]),
            ))
        }
        Expr::PathCon(dimension) => {
            let ty = KernelTy::PathDecl {
                dimension: *dimension,
            };
            Ok((ty.clone(), node("path-attach", ty, false, vec![])))
        }
        Expr::Bang(_) | Expr::WhyNot(_) => Err(ElabError::LinearExponentialOutsideFrozenAlphabet),
        Expr::Susp(inner) => synth_unary_former(state, "susp-form", inner),
        Expr::Trunc(inner) => synth_unary_former(state, "trunc-form", inner),
        Expr::Flat(inner) => synth_unary_former(state, "flat-form", inner),
        Expr::Sharp(inner) => synth_unary_former(state, "sharp-form", inner),
        Expr::Disc(inner) => synth_unary_former(state, "disc-form", inner),
        Expr::Shape(inner) => synth_unary_former(state, "shape-form", inner),
        Expr::Next(inner) => synth_unary_former(state, "next-form", inner),
        Expr::Eventually(inner) => synth_unary_former(state, "eventually-form", inner),
    }
}

fn synth_unary_former(
    state: &mut ElabState,
    rule: &str,
    inner: &Expr,
) -> Result<(KernelTy, DerivationNode), ElabError> {
    let (inner_ty, inner_node) = synth(state, inner)?;
    let coarse = as_type(state, &inner_ty);
    Ok((
        KernelTy::Type,
        node(rule, KernelTy::Type, coarse, vec![inner_node]),
    ))
}

fn synth_app(
    state: &mut ElabState,
    function: &Expr,
    argument: &Expr,
) -> Result<(KernelTy, DerivationNode), ElabError> {
    if matches!(argument, Expr::Univ) {
        return Err(ElabError::BareUnivArgument);
    }
    let (function_ty, function_node) = synth(state, function)?;
    let (argument_ty, argument_node) = synth(state, argument)?;

    let function_whnf = whnf(function, state.scope_len(), state.remaining_fuel())?;
    state.spend(function_whnf.steps);

    match function_whnf.expr {
        Expr::Lam(body) => {
            // Oriented beta: reduce and synthesize the reduct so the
            // application's classifier reflects the computation clause.
            let reduced = substitute_level(&body, state.scope_len() + 1, argument);
            state.spend(1);
            let (reduced_ty, reduced_node) = synth(state, &reduced)?;
            Ok((
                reduced_ty.clone(),
                node(
                    "app-beta",
                    reduced_ty,
                    false,
                    vec![function_node, argument_node, reduced_node],
                ),
            ))
        }
        Expr::Univ => Ok((
            KernelTy::Type,
            node(
                "univ-app-form",
                KernelTy::Type,
                false,
                vec![function_node, argument_node],
            ),
        )),
        head => {
            // No further computation clause can fire. Classify the head:
            // typed function readings apply; everything else is neutral or
            // fresh-stuck (kernel verdict).
            match &function_ty {
                KernelTy::Fun(domain, codomain) => {
                    if **domain != argument_ty {
                        state.note_coarse();
                    }
                    let ty = (**codomain).clone();
                    return Ok((
                        ty.clone(),
                        node("app-fun", ty, true, vec![function_node, argument_node]),
                    ));
                }
                KernelTy::El(type_expr) => {
                    if let Expr::Pi(domain, codomain) = type_expr {
                        let expected = KernelTy::El((**domain).clone());
                        if expected != argument_ty {
                            state.note_coarse();
                        }
                        let instantiated =
                            substitute_level(codomain, state.scope_len() + 1, argument);
                        let ty = KernelTy::El(instantiated);
                        return Ok((
                            ty.clone(),
                            node("app-el-pi", ty, false, vec![function_node, argument_node]),
                        ));
                    }
                }
                _ => {}
            }

            let stuck_head = classify_stuck_head(state, &head);
            state.stuck.push(StuckApplication {
                clause_index: state.clause_index,
                head: stuck_head,
            });
            Ok((
                KernelTy::Neutral,
                node(
                    "app-stuck",
                    KernelTy::Neutral,
                    false,
                    vec![function_node, argument_node],
                ),
            ))
        }
    }
}

fn classify_stuck_head(state: &ElabState, head: &Expr) -> StuckHead {
    match head {
        Expr::Next(_) | Expr::Eventually(_) => StuckHead::TemporalFormer {
            atom: atom_name(head).to_string(),
        },
        Expr::Flat(_) | Expr::Sharp(_) | Expr::Disc(_) | Expr::Shape(_) => StuckHead::ModalFormer {
            atom: atom_name(head).to_string(),
        },
        Expr::Lib(step) => StuckHead::LibraryConstant { step: *step },
        Expr::Var(level) => match state.resolve(*level) {
            Some(ScopeRef::Field { clause }) => StuckHead::FieldReference { clause },
            Some(ScopeRef::Ambient { parameter }) => StuckHead::AmbientParameter { parameter },
            Some(ScopeRef::Local { binder }) => StuckHead::LocalVariable { binder },
            None => StuckHead::OtherFormer {
                atom: "out-of-scope-var".to_string(),
            },
        },
        Expr::App(inner, _) => classify_stuck_head(state, inner),
        other => StuckHead::OtherFormer {
            atom: atom_name(other).to_string(),
        },
    }
}

fn atom_name(expr: &Expr) -> &'static str {
    match expr {
        Expr::App(_, _) => "App",
        Expr::Lam(_) => "Lam",
        Expr::Pi(_, _) => "Pi",
        Expr::Sigma(_, _) => "Sigma",
        Expr::Univ => "Univ",
        Expr::Var(_) => "Var",
        Expr::Lib(_) => "Lib",
        Expr::Id(_, _, _) => "Id",
        Expr::Refl(_) => "Refl",
        Expr::Susp(_) => "Susp",
        Expr::Trunc(_) => "Trunc",
        Expr::PathCon(_) => "PathCon",
        Expr::Flat(_) => "Flat",
        Expr::Sharp(_) => "Sharp",
        Expr::Disc(_) => "Disc",
        Expr::Shape(_) => "Shape",
        Expr::Next(_) => "Next",
        Expr::Eventually(_) => "Eventually",
        Expr::Bang(_) => "Bang",
        Expr::WhyNot(_) => "WhyNot",
    }
}

// ---------------------------------------------------------------------------
// Tokens (program §1). Private fields, no public constructors: production
// code outside this module cannot mint semantic typing evidence. Issuance
// builds tokens from kernel derivations only; replay re-derives.
// ---------------------------------------------------------------------------

mod sealed {
    pub trait TokenSealed {}
}

/// Common face of the kernel tokens. The `sealed::TokenSealed` supertrait
/// makes the set closed: no type outside this module can implement it.
pub trait KernelToken: sealed::TokenSealed {
    fn derivation_hash(&self) -> &str;
    fn signature_digest(&self) -> &str;
}

/// Transport a term formed at scope size `from` WITHIN the same telescope
/// to scope size `to`. Free references (levels <= `from`) keep their
/// meaning — levels are absolute over [ambient, fields] — but must also
/// be in scope at `to`; binder-internal levels re-base. Returns `None`
/// (fail closed) when a free reference has no counterpart at `to`.
fn transport_within_telescope(expr: &Expr, from: u32, to: u32) -> Option<Expr> {
    transport_levels(expr, from, to, true)
}

/// Transport a term ACROSS telescopes: `from` is the source telescope's
/// scope size, `to` the destination clause scope. Free references denote
/// the source telescope's ambient parameters and fields, which have no
/// counterpart in the destination context — any free reference fails
/// closed. Only binder-internal structure transports (re-based levels).
fn transport_across_telescopes(expr: &Expr, from: u32, to: u32) -> Option<Expr> {
    transport_levels(expr, from, to, false)
}

fn transport_levels(expr: &Expr, from: u32, to: u32, keep_shared_free: bool) -> Option<Expr> {
    let rebuild = |inner: &Expr| transport_levels(inner, from, to, keep_shared_free);
    Some(match expr {
        Expr::Var(level) => {
            if *level <= from {
                if keep_shared_free && *level <= to {
                    Expr::Var(*level)
                } else {
                    return None;
                }
            } else {
                Expr::Var(level - from + to)
            }
        }
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(a, b) => Expr::App(Box::new(rebuild(a)?), Box::new(rebuild(b)?)),
        Expr::Pi(a, b) => Expr::Pi(Box::new(rebuild(a)?), Box::new(rebuild(b)?)),
        Expr::Sigma(a, b) => Expr::Sigma(Box::new(rebuild(a)?), Box::new(rebuild(b)?)),
        Expr::Id(a, b, c) => Expr::Id(
            Box::new(rebuild(a)?),
            Box::new(rebuild(b)?),
            Box::new(rebuild(c)?),
        ),
        Expr::Lam(inner) => Expr::Lam(Box::new(rebuild(inner)?)),
        Expr::Refl(inner) => Expr::Refl(Box::new(rebuild(inner)?)),
        Expr::Susp(inner) => Expr::Susp(Box::new(rebuild(inner)?)),
        Expr::Trunc(inner) => Expr::Trunc(Box::new(rebuild(inner)?)),
        Expr::Flat(inner) => Expr::Flat(Box::new(rebuild(inner)?)),
        Expr::Sharp(inner) => Expr::Sharp(Box::new(rebuild(inner)?)),
        Expr::Disc(inner) => Expr::Disc(Box::new(rebuild(inner)?)),
        Expr::Shape(inner) => Expr::Shape(Box::new(rebuild(inner)?)),
        Expr::Next(inner) => Expr::Next(Box::new(rebuild(inner)?)),
        Expr::Eventually(inner) => Expr::Eventually(Box::new(rebuild(inner)?)),
        Expr::Bang(inner) => Expr::Bang(Box::new(rebuild(inner)?)),
        Expr::WhyNot(inner) => Expr::WhyNot(Box::new(rebuild(inner)?)),
    })
}

/// The oriented basis backing a typed eliminator: generic beta redex
/// clauses and/or Kan (path-constructor) dimensions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrientedBasis {
    pub beta_clauses: Vec<u16>,
    pub kan_dimensions: Vec<u32>,
}

impl OrientedBasis {
    pub fn is_empty(&self) -> bool {
        self.beta_clauses.is_empty() && self.kan_dimensions.is_empty()
    }
}

/// Evidence that a telescope exhibits a real formation clause, an oriented
/// beta/Kan basis, and an eliminator judgmentally typed against its motive.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedEliminatorToken {
    subject_hash: String,
    signature_digest: String,
    formation_clause: u16,
    eliminator_clause: u16,
    motive_normal_form: Expr,
    basis: OrientedBasis,
    derivation_hash: String,
}

impl sealed::TokenSealed for TypedEliminatorToken {}

impl KernelToken for TypedEliminatorToken {
    fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
    fn signature_digest(&self) -> &str {
        &self.signature_digest
    }
}

impl TypedEliminatorToken {
    pub fn subject_hash(&self) -> &str {
        &self.subject_hash
    }
    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }
    pub fn formation_clause(&self) -> u16 {
        self.formation_clause
    }
    pub fn eliminator_clause(&self) -> u16 {
        self.eliminator_clause
    }
    pub fn motive_normal_form(&self) -> &Expr {
        &self.motive_normal_form
    }
    pub fn basis(&self) -> &OrientedBasis {
        &self.basis
    }
    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// Evidence that a clause is a naturality square between two distinct
/// unary operators, closing up to the frozen judgmental equality.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NaturalityToken {
    subject_hash: String,
    signature_digest: String,
    clause_index: u16,
    outer_operator: String,
    inner_operator: String,
    argument_normal_form: Expr,
    equality_witness: EqualityWitness,
    derivation_hash: String,
}

impl sealed::TokenSealed for NaturalityToken {}

impl KernelToken for NaturalityToken {
    fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
    fn signature_digest(&self) -> &str {
        &self.signature_digest
    }
}

impl NaturalityToken {
    pub fn subject_hash(&self) -> &str {
        &self.subject_hash
    }
    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }
    pub fn clause_index(&self) -> u16 {
        self.clause_index
    }
    pub fn outer_operator(&self) -> &str {
        &self.outer_operator
    }
    pub fn inner_operator(&self) -> &str {
        &self.inner_operator
    }
    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// Evidence for the P5 route: a unique reachability-dominant direct
/// import plus a typed lift derivation for every direct application of
/// the dominant import.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedLiftToken {
    subject_hash: String,
    signature_digest: String,
    direct_imports: Vec<u32>,
    dominant_import: u32,
    lift_clauses: Vec<u16>,
    derivation_hash: String,
}

impl sealed::TokenSealed for TypedLiftToken {}

impl KernelToken for TypedLiftToken {
    fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
    fn signature_digest(&self) -> &str {
        &self.signature_digest
    }
}

impl TypedLiftToken {
    pub fn subject_hash(&self) -> &str {
        &self.subject_hash
    }
    pub fn signature_digest(&self) -> &str {
        &self.signature_digest
    }
    pub fn direct_imports(&self) -> &[u32] {
        &self.direct_imports
    }
    pub fn dominant_import(&self) -> u32 {
        self.dominant_import
    }
    pub fn lift_clauses(&self) -> &[u16] {
        &self.lift_clauses
    }
    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum TokenError {
    #[error("elaboration failed at clause {clause_index}: {error}")]
    ElaborationFailed { clause_index: u16, error: ElabError },
    #[error("no coarse-free formation clause")]
    NoFormationClause,
    #[error("no oriented beta/Kan basis")]
    NoOrientedBasis,
    #[error("no eliminator clause typed against a formation motive")]
    NoMotiveTypedEliminator { stuck_heads: Vec<StuckApplication> },
    #[error("clause {clause_index} is not a naturality square: {reason}")]
    NotANaturalitySquare { clause_index: u16, reason: String },
    #[error("naturality square at clause {clause_index} does not close judgmentally")]
    NaturalitySquareMismatch {
        clause_index: u16,
        domain_normal_form: Expr,
        codomain_normal_form: Expr,
    },
    #[error("candidate has no direct imports")]
    NoDirectImports,
    #[error("no unique reachability-dominant direct import")]
    NoDominantImport {
        direct_imports: Vec<u32>,
        dominant_imports: Vec<u32>,
    },
    #[error("dominant import L{dominant_import} is never directly applied")]
    NoDominantApplications { dominant_import: u32 },
    #[error(
        "lift at clause {clause_index} is not typed against any exported formation of the dominant import"
    )]
    LiftNotTypedAgainstExportedFormation {
        clause_index: u16,
        argument_normal_form: Expr,
    },
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum TokenReplayError {
    #[error("token was issued against a different signature digest")]
    SignatureDigestMismatch { expected: String, found: String },
    #[error("token subject does not match the presented telescope")]
    SubjectHashMismatch { expected: String, found: String },
    #[error("re-issuance failed: {0}")]
    ReissueFailed(TokenError),
    #[error("re-derived token diverges from the presented token")]
    TokenFieldsDiverged,
}

fn token_derivation_hash(kind: &str, payload: &serde_json::Value) -> String {
    let wrapped = serde_json::json!({
        "token_rules": TOKEN_RULES_VERSION_TAG,
        "kind": kind,
        "payload": payload,
    });
    format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&wrapped).expect("token payload serialization"))
    )
}

/// Issue a `TypedEliminatorToken` for `telescope` against the signature.
pub fn issue_typed_eliminator_token(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
) -> Result<TypedEliminatorToken, TokenError> {
    let elaboration =
        elaborate_telescope(signature, telescope, visible_library).map_err(|failure| {
            TokenError::ElaborationFailed {
                clause_index: failure.clause_index,
                error: failure.error,
            }
        })?;

    // (1) A real formation clause: kernel role Formation, classifier Type,
    // zero coarse subsumptions in its derivation.
    let formations: Vec<&ClauseElaboration> = elaboration
        .clauses
        .iter()
        .filter(|clause| {
            clause.kernel_role == ClauseRole::Formation
                && clause.kernel_ty == KernelTy::Type
                && clause.coarse_assumptions == 0
        })
        .collect();
    if formations.is_empty() {
        return Err(TokenError::NoFormationClause);
    }

    // (2) An oriented beta/Kan basis.
    let basis = OrientedBasis {
        beta_clauses: elaboration
            .clauses
            .iter()
            .filter(|clause| clause.is_beta_redex_clause)
            .map(|clause| clause.clause_index)
            .collect(),
        kan_dimensions: telescope
            .clauses
            .iter()
            .filter_map(|clause| match clause.expr {
                Expr::PathCon(dimension) => Some(dimension),
                _ => None,
            })
            .collect(),
    };
    if basis.is_empty() {
        return Err(TokenError::NoOrientedBasis);
    }

    // (3) An eliminator typed against its motive: a top-level application
    // whose weak-head-normal head is judgmentally equal to a formation
    // clause's normal form, TRANSPORTED into the eliminator's clause
    // scope. Levels are absolute over [ambient, fields], so shared free
    // references keep their meaning; binder-internal levels re-base; a
    // formation whose free references are not in scope at the eliminator
    // fails closed (skipped).
    let mut found: Option<(u16, u16, Expr)> = None;
    'outer: for clause in &elaboration.clauses {
        if let Expr::App(function, _) = &telescope.clauses[clause.clause_index as usize].expr {
            let scope_len = elaboration.ambient_parameters + u32::from(clause.clause_index);
            for formation in &formations {
                let formation_scope =
                    elaboration.ambient_parameters + u32::from(formation.clause_index);
                let Some(transported_motive) =
                    transport_within_telescope(&formation.normal_form, formation_scope, scope_len)
                else {
                    continue;
                };
                let witness = judgmental_equality(
                    function,
                    &transported_motive,
                    scope_len,
                    elaboration.fuel.static_bound,
                )
                .map_err(|error| TokenError::ElaborationFailed {
                    clause_index: clause.clause_index,
                    error: error.into(),
                })?;
                if witness.equal {
                    found = Some((
                        clause.clause_index,
                        formation.clause_index,
                        witness.left_normal_form,
                    ));
                    break 'outer;
                }
            }
        }
    }
    let Some((eliminator_clause, formation_clause, motive_normal_form)) = found else {
        return Err(TokenError::NoMotiveTypedEliminator {
            stuck_heads: elaboration
                .stuck_applications()
                .into_iter()
                .filter(|stuck| stuck.head.is_fresh_stuck())
                .cloned()
                .collect(),
        });
    };

    let payload = serde_json::json!({
        "elaboration": elaboration.derivation_hash,
        "formation_clause": formation_clause,
        "eliminator_clause": eliminator_clause,
        "motive": motive_normal_form,
        "basis": basis,
    });
    Ok(TypedEliminatorToken {
        subject_hash: elaboration.subject_hash,
        signature_digest: elaboration.signature_digest,
        formation_clause,
        eliminator_clause,
        motive_normal_form,
        basis,
        derivation_hash: token_derivation_hash("typed-eliminator", &payload),
    })
}

/// Replay a `TypedEliminatorToken`: re-derive from source and require the
/// presented token to match field-for-field.
pub fn replay_typed_eliminator_token(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    token: &TypedEliminatorToken,
) -> Result<(), TokenReplayError> {
    check_binding(
        signature,
        telescope,
        &token.signature_digest,
        &token.subject_hash,
    )?;
    let reissued = issue_typed_eliminator_token(signature, telescope, visible_library)
        .map_err(TokenReplayError::ReissueFailed)?;
    if &reissued == token {
        Ok(())
    } else {
        Err(TokenReplayError::TokenFieldsDiverged)
    }
}

/// Issue a `NaturalityToken` for one clause of `telescope`.
pub fn issue_naturality_token(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    clause_index: u16,
) -> Result<NaturalityToken, TokenError> {
    let elaboration =
        elaborate_telescope(signature, telescope, visible_library).map_err(|failure| {
            TokenError::ElaborationFailed {
                clause_index: failure.clause_index,
                error: failure.error,
            }
        })?;
    let Some(clause) = telescope.clauses.get(clause_index as usize) else {
        return Err(TokenError::NotANaturalitySquare {
            clause_index,
            reason: "clause index out of range".to_string(),
        });
    };
    let Expr::Pi(domain, codomain) = &clause.expr else {
        return Err(TokenError::NotANaturalitySquare {
            clause_index,
            reason: "clause is not a Pi square".to_string(),
        });
    };

    let scope_len = elaboration.ambient_parameters + u32::from(clause_index);
    let fuel = elaboration.fuel.static_bound;
    let domain_whnf =
        whnf(domain, scope_len, fuel).map_err(|error| TokenError::ElaborationFailed {
            clause_index,
            error: error.into(),
        })?;
    let codomain_whnf =
        whnf(codomain, scope_len + 1, fuel).map_err(|error| TokenError::ElaborationFailed {
            clause_index,
            error: error.into(),
        })?;

    let Some((outer, domain_inner)) = split_unary(&domain_whnf.expr) else {
        return Err(TokenError::NotANaturalitySquare {
            clause_index,
            reason: "domain is not an operator pair".to_string(),
        });
    };
    let Some((domain_second, domain_argument)) = split_unary(domain_inner) else {
        return Err(TokenError::NotANaturalitySquare {
            clause_index,
            reason: "domain is not an operator pair".to_string(),
        });
    };
    let Some((codomain_first, codomain_inner)) = split_unary(&codomain_whnf.expr) else {
        return Err(TokenError::NotANaturalitySquare {
            clause_index,
            reason: "codomain is not an operator pair".to_string(),
        });
    };
    let Some((codomain_second, codomain_argument)) = split_unary(codomain_inner) else {
        return Err(TokenError::NotANaturalitySquare {
            clause_index,
            reason: "codomain is not an operator pair".to_string(),
        });
    };

    // The square: outer(inner(X)) -> inner(outer(X)) with the SAME X.
    if outer == domain_second {
        return Err(TokenError::NotANaturalitySquare {
            clause_index,
            reason: "operators are not distinct".to_string(),
        });
    }
    if codomain_first != domain_second || codomain_second != outer {
        return Err(TokenError::NaturalitySquareMismatch {
            clause_index,
            domain_normal_form: domain_whnf.expr.clone(),
            codomain_normal_form: codomain_whnf.expr.clone(),
        });
    }
    // The codomain argument was formed under the Pi binder (scope + 1);
    // it must transport down to the domain's scope, which fails closed if
    // it references the binder itself.
    let Some(codomain_argument) =
        transport_within_telescope(codomain_argument, scope_len + 1, scope_len)
    else {
        return Err(TokenError::NaturalitySquareMismatch {
            clause_index,
            domain_normal_form: domain_whnf.expr.clone(),
            codomain_normal_form: codomain_whnf.expr.clone(),
        });
    };
    let witness = judgmental_equality(domain_argument, &codomain_argument, scope_len, fuel)
        .map_err(|error| TokenError::ElaborationFailed {
            clause_index,
            error: error.into(),
        })?;
    if !witness.equal {
        return Err(TokenError::NaturalitySquareMismatch {
            clause_index,
            domain_normal_form: domain_whnf.expr,
            codomain_normal_form: codomain_whnf.expr,
        });
    }

    let payload = serde_json::json!({
        "elaboration": elaboration.derivation_hash,
        "clause_index": clause_index,
        "outer": outer,
        "inner": domain_second,
        "argument": witness.left_normal_form,
        "witness": witness,
    });
    Ok(NaturalityToken {
        subject_hash: elaboration.subject_hash,
        signature_digest: elaboration.signature_digest,
        clause_index,
        outer_operator: outer.to_string(),
        inner_operator: domain_second.to_string(),
        argument_normal_form: witness.left_normal_form.clone(),
        equality_witness: witness,
        derivation_hash: token_derivation_hash("naturality", &payload),
    })
}

/// Replay a `NaturalityToken` by re-issuing and comparing.
pub fn replay_naturality_token(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    token: &NaturalityToken,
) -> Result<(), TokenReplayError> {
    check_binding(
        signature,
        telescope,
        &token.signature_digest,
        &token.subject_hash,
    )?;
    let reissued =
        issue_naturality_token(signature, telescope, visible_library, token.clause_index)
            .map_err(TokenReplayError::ReissueFailed)?;
    if &reissued == token {
        Ok(())
    } else {
        Err(TokenReplayError::TokenFieldsDiverged)
    }
}

fn split_unary(expr: &Expr) -> Option<(&'static str, &Expr)> {
    match expr {
        Expr::Flat(inner) => Some(("Flat", inner)),
        Expr::Sharp(inner) => Some(("Sharp", inner)),
        Expr::Disc(inner) => Some(("Disc", inner)),
        Expr::Shape(inner) => Some(("Shape", inner)),
        Expr::Next(inner) => Some(("Next", inner)),
        Expr::Eventually(inner) => Some(("Eventually", inner)),
        Expr::Trunc(inner) => Some(("Trunc", inner)),
        Expr::Susp(inner) => Some(("Susp", inner)),
        _ => None,
    }
}

/// Issue a `TypedLiftToken` (P5 route) for `telescope`.
pub fn issue_typed_lift_token(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
) -> Result<TypedLiftToken, TokenError> {
    let elaboration =
        elaborate_telescope(signature, telescope, visible_library).map_err(|failure| {
            TokenError::ElaborationFailed {
                clause_index: failure.clause_index,
                error: failure.error,
            }
        })?;

    let direct_imports: Vec<u32> = telescope.lib_refs().into_iter().collect();
    if direct_imports.is_empty() {
        return Err(TokenError::NoDirectImports);
    }

    // Graph premise: a unique direct import from which every other direct
    // import is reachable in the sealed signature's import DAG.
    let dominant_imports: Vec<u32> = direct_imports
        .iter()
        .copied()
        .filter(|candidate| {
            let reach = signature.reachable_from(*candidate);
            direct_imports.iter().all(|other| reach.contains(other))
        })
        .collect();
    let [dominant_import] = dominant_imports[..] else {
        return Err(TokenError::NoDominantImport {
            direct_imports,
            dominant_imports,
        });
    };

    // Typed lift derivation: EVERY direct application of the dominant
    // import — top-level, nested under binders or unary formers, or
    // curried — must check against one of its exported Pi formations.
    // The entry's domains live in the entry's own scope; they transport
    // across telescopes only when they are free of entry-scope
    // references (levels are absolute per context, so a raw comparison
    // would conflate unrelated objects — fail closed instead).
    let dominant_entry = signature.entry(dominant_import);
    let entry_ambient = dominant_entry
        .map(|entry| minimal_ambient_parameters(&entry.telescope))
        .unwrap_or(0);
    let mut lift_clauses = Vec::new();
    for (index, clause) in telescope.clauses.iter().enumerate() {
        let clause_index = u16::try_from(index).expect("clause index fits u16");
        if !clause.expr.lib_refs().contains(&dominant_import) {
            continue;
        }
        let clause_scope = elaboration.ambient_parameters + u32::from(clause_index);
        let fuel = elaboration.fuel.static_bound;
        let mut applications = Vec::new();
        collect_dominant_applications(&clause.expr, dominant_import, 0, &mut applications);
        // A bare `Lib(dominant)` is an import, not a lift.  In particular,
        // it cannot discharge the P5 typing premise vacuously merely by
        // occurring as the domain or codomain of a formation.  Record a
        // lift clause only when there is at least one actual application
        // whose argument is checked below.
        if applications.is_empty() {
            continue;
        }
        lift_clauses.push(clause_index);
        for (binder_depth, argument) in applications {
            let argument_scope = clause_scope + binder_depth;
            let mut typed = false;
            if let Some(entry) = dominant_entry {
                for formation_index in &entry.formation_clauses {
                    let formation = &entry.telescope.clauses[*formation_index as usize];
                    let Expr::Pi(domain, _) = &formation.expr else {
                        continue;
                    };
                    let entry_scope = entry_ambient + u32::from(*formation_index);
                    let Some(transported_domain) =
                        transport_across_telescopes(domain, entry_scope, argument_scope)
                    else {
                        continue;
                    };
                    let witness =
                        judgmental_equality(argument, &transported_domain, argument_scope, fuel)
                            .map_err(|error| TokenError::ElaborationFailed {
                                clause_index,
                                error: error.into(),
                            })?;
                    if witness.equal {
                        typed = true;
                        break;
                    }
                }
            }
            if !typed {
                let argument_nf = normalize(argument, argument_scope, fuel)
                    .map_err(|error| TokenError::ElaborationFailed {
                        clause_index,
                        error: error.into(),
                    })?
                    .expr;
                return Err(TokenError::LiftNotTypedAgainstExportedFormation {
                    clause_index,
                    argument_normal_form: argument_nf,
                });
            }
        }
    }
    if lift_clauses.is_empty() {
        return Err(TokenError::NoDominantApplications { dominant_import });
    }

    let payload = serde_json::json!({
        "elaboration": elaboration.derivation_hash,
        "direct_imports": direct_imports,
        "dominant_import": dominant_import,
        "lift_clauses": lift_clauses,
    });
    Ok(TypedLiftToken {
        subject_hash: elaboration.subject_hash,
        signature_digest: elaboration.signature_digest,
        direct_imports,
        dominant_import,
        lift_clauses,
        derivation_hash: token_derivation_hash("typed-lift", &payload),
    })
}

/// Replay a `TypedLiftToken` by re-issuing and comparing.
pub fn replay_typed_lift_token(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
    token: &TypedLiftToken,
) -> Result<(), TokenReplayError> {
    check_binding(
        signature,
        telescope,
        &token.signature_digest,
        &token.subject_hash,
    )?;
    let reissued = issue_typed_lift_token(signature, telescope, visible_library)
        .map_err(TokenReplayError::ReissueFailed)?;
    if &reissued == token {
        Ok(())
    } else {
        Err(TokenReplayError::TokenFieldsDiverged)
    }
}

/// Collect every direct application of `dominant` in `expr`, with the
/// number of binders above each occurrence (Lam bodies and Pi/Sigma
/// codomains — the kernel binder discipline).
fn collect_dominant_applications<'a>(
    expr: &'a Expr,
    dominant: u32,
    binders: u32,
    out: &mut Vec<(u32, &'a Expr)>,
) {
    match expr {
        Expr::App(function, argument) => {
            if matches!(function.as_ref(), Expr::Lib(step) if *step == dominant) {
                out.push((binders, argument));
            }
            collect_dominant_applications(function, dominant, binders, out);
            collect_dominant_applications(argument, dominant, binders, out);
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            collect_dominant_applications(domain, dominant, binders, out);
            collect_dominant_applications(codomain, dominant, binders + 1, out);
        }
        Expr::Lam(body) => collect_dominant_applications(body, dominant, binders + 1, out),
        Expr::Id(ty, left, right) => {
            collect_dominant_applications(ty, dominant, binders, out);
            collect_dominant_applications(left, dominant, binders, out);
            collect_dominant_applications(right, dominant, binders, out);
        }
        Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => collect_dominant_applications(inner, dominant, binders, out),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => {}
    }
}

fn check_binding(
    signature: &SealedSignature,
    telescope: &Telescope,
    token_digest: &str,
    token_subject: &str,
) -> Result<(), TokenReplayError> {
    if signature.digest() != token_digest {
        return Err(TokenReplayError::SignatureDigestMismatch {
            expected: token_digest.to_string(),
            found: signature.digest().to_string(),
        });
    }
    let subject = candidate_hash(telescope);
    if subject != token_subject {
        return Err(TokenReplayError::SubjectHashMismatch {
            expected: token_subject.to_string(),
            found: subject,
        });
    }
    Ok(())
}

/// Aggregate fuel certificate over the fifteen sealed entries: the golden
/// grader's "fuel certificate emitted" artifact.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GenesisFuelCertificate {
    pub signature_digest: String,
    pub per_step: Vec<(u32, FuelCertificate)>,
    pub all_within_bound: bool,
}

pub fn genesis_fuel_certificate() -> GenesisFuelCertificate {
    let signature = SealedSignature::genesis_del_h15();
    let mut per_step = Vec::with_capacity(signature.len());
    let mut all_within_bound = true;
    for entry in signature.entries() {
        let elaboration =
            elaborate_telescope(&signature, &entry.telescope, entry.step.saturating_sub(1))
                .unwrap_or_else(|failure| {
                    panic!("sealed entry {} must elaborate: {failure}", entry.step)
                });
        all_within_bound &= elaboration.fuel.within_bound;
        per_step.push((entry.step, elaboration.fuel));
    }
    GenesisFuelCertificate {
        signature_digest: signature.digest().to_string(),
        per_step,
        all_within_bound,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::ClauseRec;

    fn app(function: Expr, argument: Expr) -> Expr {
        Expr::App(Box::new(function), Box::new(argument))
    }

    fn lam(body: Expr) -> Expr {
        Expr::Lam(Box::new(body))
    }

    fn pi(domain: Expr, codomain: Expr) -> Expr {
        Expr::Pi(Box::new(domain), Box::new(codomain))
    }

    fn sigma(domain: Expr, codomain: Expr) -> Expr {
        Expr::Sigma(Box::new(domain), Box::new(codomain))
    }

    fn tel(exprs: Vec<Expr>) -> Telescope {
        Telescope::new(
            exprs
                .into_iter()
                .map(|expr| {
                    let role = Telescope::new(vec![]).clauses.first().map(|c| c.role);
                    let _ = role;
                    ClauseRec::new(primary_role_for_test(&expr), expr)
                })
                .collect(),
        )
    }

    // Mirror of pen-core's private primary_role, for building candidates
    // the way the enumerator does.
    fn primary_role_for_test(expr: &Expr) -> ClauseRole {
        match expr {
            Expr::Susp(_) => ClauseRole::Formation,
            Expr::Univ | Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Id(_, _, _) => {
                ClauseRole::Formation
            }
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => ClauseRole::Formation,
            Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => ClauseRole::Introduction,
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Lib(_)) => ClauseRole::Introduction,
            Expr::App(left, _) if matches!(left.as_ref(), Expr::Lam(_)) => ClauseRole::Elimination,
            Expr::App(_, _) => ClauseRole::Introduction,
            Expr::PathCon(_) => ClauseRole::PathAttach,
            _ => ClauseRole::Formation,
        }
    }

    #[test]
    fn genesis_signature_verifies_sealed_candidate_hashes() {
        let signature = SealedSignature::genesis_del_h15();
        assert_eq!(signature.len(), 15);
        assert!(signature.digest().starts_with("blake3:"));
        // No sealed entry exports a computation clause: the derived
        // oriented computation basis is generic beta only.
        for entry in signature.entries() {
            assert_eq!(
                entry.exported_computation_clauses, 0,
                "step {} unexpectedly exports a computation clause",
                entry.step
            );
        }
    }

    #[test]
    fn hash_tags_are_frozen() {
        assert_eq!(ELABORATOR_VERSION_TAG, "elaborator-v1");
        assert_eq!(TOKEN_RULES_VERSION_TAG, "token-rules-v2");
        assert!(elaborator_hash().starts_with("blake3:"));
        assert!(token_rules_hash().starts_with("blake3:"));
        assert_ne!(elaborator_hash(), token_rules_hash());
    }

    #[test]
    fn all_fifteen_sealed_entries_elaborate_and_reduce_to_frozen_normal_forms() {
        let signature = SealedSignature::genesis_del_h15();
        for entry in signature.entries() {
            let elaboration = elaborate_telescope(&signature, &entry.telescope, entry.step - 1)
                .unwrap_or_else(|failure| panic!("sealed step {} failed: {failure}", entry.step));
            assert!(elaboration.fuel.within_bound, "fuel at step {}", entry.step);
            assert!(
                elaboration.ambient_parameters <= MAX_AMBIENT_PARAMETERS,
                "ambient at step {}",
                entry.step
            );
            for (clause, elaborated) in entry
                .telescope
                .clauses
                .iter()
                .zip(elaboration.clauses.iter())
            {
                // The kernel's role reading agrees with the sealed roles.
                assert_eq!(
                    elaborated.kernel_role, clause.role,
                    "kernel role diverges at step {} clause {}",
                    entry.step, elaborated.clause_index
                );
                if entry.step == 4 && elaborated.clause_index == 2 {
                    // The single beta-redex clause in the sealed corpus:
                    // App(Lam(Var 1), Var 2) -> Var 1 (constant function).
                    assert!(elaborated.is_beta_redex_clause);
                    assert_eq!(elaborated.normal_form, Expr::Var(1));
                    assert_eq!(elaborated.beta_steps, 1);
                } else {
                    assert_eq!(
                        elaborated.normal_form, clause.expr,
                        "sealed clause not in normal form at step {} clause {}",
                        entry.step, elaborated.clause_index
                    );
                    assert_eq!(elaborated.beta_steps, 0);
                }
            }
        }
    }

    #[test]
    fn genesis_fuel_certificate_holds_the_static_bound() {
        let certificate = genesis_fuel_certificate();
        assert!(certificate.all_within_bound);
        assert_eq!(certificate.per_step.len(), 15);
        let total: u32 = certificate
            .per_step
            .iter()
            .map(|(_, fuel)| fuel.total_fuel_observed)
            .sum();
        // The sealed corpus holds exactly one beta redex (step 4, clause 3),
        // evaluated once during synthesis (app-beta) and once during clause
        // normalization: two observed evaluation steps in total.
        assert_eq!(total, 2);
        for (step, fuel) in &certificate.per_step {
            assert!(
                fuel.total_fuel_observed <= fuel.static_bound,
                "observed fuel exceeds static bound at step {step}"
            );
        }
    }

    #[test]
    fn temporal_pair_applications_are_stuck_kernel_verdicts() {
        let signature = SealedSignature::genesis_del_h15();
        let dct = &signature.entry(15).expect("step 15").telescope;
        let elaboration = elaborate_telescope(&signature, dct, 14).expect("dct elaborates");
        // Clause 7 (index 6): Lam(App(Eventually(Var 1), Var 2)) — the
        // Eventually head has no exported eliminator: stuck by construction.
        let clause_seven = &elaboration.clauses[6];
        assert!(clause_seven.stuck_applications.iter().any(|stuck| matches!(
            &stuck.head,
            StuckHead::TemporalFormer { atom } if atom == "Eventually"
        )));
        // Clause 4 (index 3): Lam(App(Lib 10, Next(Var 1))) — library
        // constant with no exported computation clause.
        let clause_four = &elaboration.clauses[3];
        assert!(
            clause_four
                .stuck_applications
                .iter()
                .any(|stuck| matches!(&stuck.head, StuckHead::LibraryConstant { step: 10 }))
        );
        // Every fresh-stuck verdict is a kernel verdict on the head.
        for stuck in elaboration.stuck_applications() {
            assert!(
                stuck.head.is_fresh_stuck()
                    || matches!(
                        stuck.head,
                        StuckHead::AmbientParameter { .. } | StuckHead::LocalVariable { .. }
                    )
            );
        }
    }

    #[test]
    fn naturality_tokens_issue_for_the_dct_commute_squares_and_replay() {
        let signature = SealedSignature::genesis_del_h15();
        let dct = signature.entry(15).expect("step 15").telescope.clone();
        // Clause 5 (index 4): Pi(Flat(Next A), Next(Flat A)).
        let flat_next = issue_naturality_token(&signature, &dct, 14, 4).expect("flat/next");
        assert_eq!(flat_next.outer_operator(), "Flat");
        assert_eq!(flat_next.inner_operator(), "Next");
        replay_naturality_token(&signature, &dct, 14, &flat_next).expect("replay");
        // Clause 6 (index 5): Pi(Sharp(Eventually A), Eventually(Sharp A)).
        let sharp_eventually =
            issue_naturality_token(&signature, &dct, 14, 5).expect("sharp/eventually");
        assert_eq!(sharp_eventually.outer_operator(), "Sharp");
        assert_eq!(sharp_eventually.inner_operator(), "Eventually");
        // A non-square clause fails with a named reason.
        let error = issue_naturality_token(&signature, &dct, 14, 0).unwrap_err();
        assert!(matches!(error, TokenError::NotANaturalitySquare { .. }));
    }

    #[test]
    fn mutated_naturality_square_fails_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let mut dct = signature.entry(15).expect("step 15").telescope.clone();
        // Insert an extra transport on the codomain side: the square no
        // longer closes judgmentally.
        dct.clauses[4] = ClauseRec::new(
            ClauseRole::Formation,
            pi(
                Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1))))),
                Expr::Next(Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(
                    Expr::Var(1),
                )))))),
            ),
        );
        let error = issue_naturality_token(&signature, &dct, 14, 4).unwrap_err();
        assert!(matches!(
            error,
            TokenError::NaturalitySquareMismatch {
                clause_index: 4,
                ..
            }
        ));
    }

    #[test]
    fn typed_eliminator_token_dispositions_across_sealed_entries() {
        let signature = SealedSignature::genesis_del_h15();
        let mut dispositions = Vec::new();
        for entry in signature.entries() {
            let result = issue_typed_eliminator_token(&signature, &entry.telescope, entry.step - 1);
            let label = match &result {
                Ok(_) => "ok".to_string(),
                Err(TokenError::NoFormationClause) => "no_formation_clause".to_string(),
                Err(TokenError::NoOrientedBasis) => "no_oriented_basis".to_string(),
                Err(TokenError::NoMotiveTypedEliminator { .. }) => {
                    "no_motive_typed_eliminator".to_string()
                }
                Err(other) => format!("other:{other:?}"),
            };
            dispositions.push((entry.step, label));
        }
        // Pinned kernel disposition table over the sealed corpus: exactly
        // one sealed entry (step 6, the truncation HIT) certifies a typed
        // eliminator against its motive with an oriented basis.
        let expected: Vec<(u32, String)> = vec![
            (1, "no_oriented_basis"),
            (2, "no_oriented_basis"),
            (3, "no_formation_clause"),
            (4, "no_formation_clause"),
            (5, "no_motive_typed_eliminator"),
            (6, "ok"),
            (7, "no_motive_typed_eliminator"),
            (8, "no_motive_typed_eliminator"),
            (9, "no_oriented_basis"),
            (10, "no_oriented_basis"),
            (11, "no_oriented_basis"),
            (12, "no_oriented_basis"),
            (13, "no_oriented_basis"),
            (14, "no_oriented_basis"),
            (15, "no_oriented_basis"),
        ]
        .into_iter()
        .map(|(step, label)| (step, label.to_string()))
        .collect();
        assert_eq!(dispositions, expected);

        // The one token that issues replays cleanly.
        let trunc = signature.entry(6).expect("step 6").telescope.clone();
        let token = issue_typed_eliminator_token(&signature, &trunc, 5).expect("step 6 token");
        assert_eq!(token.formation_clause(), 0);
        assert_eq!(token.eliminator_clause(), 1);
        assert_eq!(token.basis().kan_dimensions, vec![1]);
        replay_typed_eliminator_token(&signature, &trunc, 5, &token).expect("replay");
    }

    #[test]
    fn falsifier_shapes_fail_with_named_token_errors() {
        let signature = SealedSignature::genesis_del_h15();

        // hit_no_formation_d1: [PathCon(1), Var(1)] — Kan basis but no
        // formation clause.
        let hit = tel(vec![Expr::PathCon(1), Expr::Var(1)]);
        assert!(matches!(
            issue_typed_eliminator_token(&signature, &hit, 15),
            Err(TokenError::NoFormationClause)
        ));

        // temporal_polymorphic_kappa2: two copies of Pi(Next A, Eventually A)
        // — formations only, no oriented basis and no eliminator.
        let temporal = tel(vec![
            pi(
                Expr::Next(Box::new(Expr::Var(1))),
                Expr::Eventually(Box::new(Expr::Var(1))),
            ),
            pi(
                Expr::Next(Box::new(Expr::Var(1))),
                Expr::Eventually(Box::new(Expr::Var(1))),
            ),
        ]);
        assert!(matches!(
            issue_typed_eliminator_token(&signature, &temporal, 15),
            Err(TokenError::NoOrientedBasis)
        ));
    }

    #[test]
    fn p5_lift_dispositions_match_the_frozen_import_dag() {
        let signature = SealedSignature::genesis_del_h15();

        // Step 13 (Metric): direct imports {11, 12}; 12 reaches 11 — the
        // unique dominant import, matching the sealed P5 record.
        let metric = signature.entry(13).expect("step 13").telescope.clone();
        assert_eq!(
            issue_typed_lift_token(&signature, &metric, 12),
            Err(TokenError::NoDominantApplications {
                dominant_import: 12,
            })
        );

        // Step 14 (Hilbert): direct imports {11, 12, 13}; dominant 13.
        let hilbert = signature.entry(14).expect("step 14").telescope.clone();
        assert_eq!(
            issue_typed_lift_token(&signature, &hilbert, 13),
            Err(TokenError::NoDominantApplications {
                dominant_import: 13,
            })
        );

        // The two-import Step-16 survivor shape {14, 15}: reachability
        // incomparable — no dominant import (matches p5_record and the
        // Agda no-survivor-dominant theorem).
        let inheritance = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            app(Expr::Lib(14), Expr::Var(1)),
        ]);
        let error = issue_typed_lift_token(&signature, &inheritance, 15).unwrap_err();
        assert_eq!(
            error,
            TokenError::NoDominantImport {
                direct_imports: vec![14, 15],
                dominant_imports: vec![],
            }
        );

        // The single-import survivor {15}: graph premise holds, but the
        // direct application App(Lib 15, A) types against none of the
        // DCT's exported Pi formations — the typed lift fails.
        let single = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            app(Expr::Lib(15), Expr::Var(1)),
        ]);
        let error = issue_typed_lift_token(&signature, &single, 15).unwrap_err();
        assert!(matches!(
            error,
            TokenError::LiftNotTypedAgainstExportedFormation {
                clause_index: 2,
                ..
            }
        ));

        // A bare occurrence of L15 is an import but not a typed lift.  This
        // exact shape was the IP-1 public-issuance witness before the
        // non-vacuity requirement was enforced.
        let vacuous = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            Expr::Var(1),
        ]);
        assert_eq!(
            issue_typed_lift_token(&signature, &vacuous, 15),
            Err(TokenError::NoDominantApplications {
                dominant_import: 15,
            })
        );
    }

    #[test]
    fn cross_scope_lift_forgeries_fail_closed() {
        // Adversarial-review regression: the candidate argument
        // Lam(Var 1) at candidate scope 0 is the IDENTITY, while step
        // 14's clause-4 domain Lam(Var 1) at the entry's own scope is
        // the CONSTANT-A1 function. A raw cross-scope comparison
        // identifies them and mints a typed-lift token; the transport
        // discipline must reject the pairing and fail closed.
        let signature = SealedSignature::genesis_del_h15();
        let forgery = tel(vec![app(Expr::Lib(14), lam(Expr::Var(1))), Expr::Var(1)]);
        let error = issue_typed_lift_token(&signature, &forgery, 15).unwrap_err();
        assert!(matches!(
            error,
            TokenError::LiftNotTypedAgainstExportedFormation {
                clause_index: 0,
                ..
            }
        ));
    }

    #[test]
    fn nested_dominant_applications_are_typed_not_bypassed() {
        // Adversarial-review regression: a direct application of the
        // dominant import nested under a binder previously bypassed the
        // typed-lift check entirely.
        let signature = SealedSignature::genesis_del_h15();
        let nested = tel(vec![
            pi(Expr::Lib(15), Expr::Var(1)),
            sigma(Expr::Var(1), Expr::Var(1)),
            lam(app(Expr::Lib(15), Expr::Var(1))),
        ]);
        let error = issue_typed_lift_token(&signature, &nested, 15).unwrap_err();
        assert!(matches!(
            error,
            TokenError::LiftNotTypedAgainstExportedFormation {
                clause_index: 2,
                ..
            }
        ));
    }

    #[test]
    fn declared_role_perturbation_cannot_change_kernel_verdicts() {
        // Adversarial-review regression: field typing must depend only on
        // expressions. Flipping a declared role byte changes the subject
        // hash (tokens re-bind) but no kernel classifier, coarse count,
        // stuckness verdict, or normal form.
        let signature = SealedSignature::genesis_del_h15();
        let trunc = signature.entry(6).expect("step 6").telescope.clone();
        let baseline = elaborate_telescope(&signature, &trunc, 5).expect("baseline");

        let mut perturbed = trunc.clone();
        perturbed.clauses[0] =
            ClauseRec::new(ClauseRole::Introduction, perturbed.clauses[0].expr.clone());
        let elaborated = elaborate_telescope(&signature, &perturbed, 5).expect("perturbed");
        for (base, pert) in baseline.clauses.iter().zip(elaborated.clauses.iter()) {
            assert_eq!(base.kernel_role, pert.kernel_role);
            assert_eq!(base.kernel_ty, pert.kernel_ty);
            assert_eq!(base.normal_form, pert.normal_form);
            assert_eq!(base.coarse_assumptions, pert.coarse_assumptions);
            assert_eq!(base.stuck_applications, pert.stuck_applications);
        }
    }

    #[test]
    fn mutated_import_fails_lift_token_replay() {
        let signature = SealedSignature::from_telescopes(vec![(
            1,
            tel(vec![pi(pi(Expr::Univ, Expr::Univ), Expr::Univ)]),
        )]);
        let candidate = tel(vec![
            app(Expr::Lib(1), pi(Expr::Univ, Expr::Univ)),
            Expr::Var(1),
        ]);
        let token = issue_typed_lift_token(&signature, &candidate, 1).expect("typed lift");
        assert_eq!(token.lift_clauses(), &[0]);
        replay_typed_lift_token(&signature, &candidate, 1, &token).expect("replay");

        // Perturb the checked application argument.  Candidate binding is
        // checked before re-issuance, so the old derivation cannot migrate.
        let mut mutated = candidate.clone();
        mutated.clauses[0] = ClauseRec::new(
            ClauseRole::Introduction,
            app(Expr::Lib(1), sigma(Expr::Univ, Expr::Univ)),
        );
        let error = replay_typed_lift_token(&signature, &mutated, 1, &token).unwrap_err();
        assert!(matches!(
            error,
            TokenReplayError::SubjectHashMismatch { .. }
        ));
    }

    #[test]
    fn mutated_motive_fails_eliminator_token_replay() {
        let signature = SealedSignature::genesis_del_h15();
        let trunc = signature.entry(6).expect("step 6").telescope.clone();
        let token = issue_typed_eliminator_token(&signature, &trunc, 5).expect("token");

        // Perturb the motive: the eliminator head no longer matches the
        // formation clause.
        let mut mutated = trunc.clone();
        mutated.clauses[1] = ClauseRec::new(
            ClauseRole::Introduction,
            app(Expr::Trunc(Box::new(Expr::Var(2))), Expr::Var(2)),
        );
        let error = replay_typed_eliminator_token(&signature, &mutated, 5, &token).unwrap_err();
        assert!(matches!(
            error,
            TokenReplayError::SubjectHashMismatch { .. }
        ));

        // Re-issuing on the mutated telescope fails closed on its own.
        assert!(matches!(
            issue_typed_eliminator_token(&signature, &mutated, 5),
            Err(TokenError::NoMotiveTypedEliminator { .. })
        ));
    }

    #[test]
    fn tokens_bind_to_the_signature_digest() {
        let signature = SealedSignature::genesis_del_h15();
        let trunc = signature.entry(6).expect("step 6").telescope.clone();
        let token = issue_typed_eliminator_token(&signature, &trunc, 5).expect("token");

        // A signature with a perturbed beta/Kan surface at step 6 has a
        // different digest; the token fails replay against it.
        let mut telescopes = Telescope::all_reference_telescopes();
        telescopes[5].1.clauses[2] = ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(2));
        let forged = SealedSignature::from_telescopes(telescopes);
        assert_ne!(forged.digest(), signature.digest());
        let error = replay_typed_eliminator_token(&forged, &trunc, 5, &token).unwrap_err();
        assert!(matches!(
            error,
            TokenReplayError::SignatureDigestMismatch { .. }
        ));
    }

    #[test]
    fn scope_and_alphabet_violations_are_hard_errors() {
        let signature = SealedSignature::genesis_del_h15();

        // Ambient overflow: Var(9) with no priors demands 9 parameters.
        let overflow = tel(vec![Expr::Var(9)]);
        let failure = elaborate_telescope(&signature, &overflow, 15).unwrap_err();
        assert!(matches!(
            failure.error,
            ElabError::AmbientContextTooLarge {
                required: 9,
                max: 2
            }
        ));

        // Bare Univ as argument mirrors the checker.
        let bare = tel(vec![app(Expr::Var(1), Expr::Univ)]);
        let failure = elaborate_telescope(&signature, &bare, 15).unwrap_err();
        assert!(matches!(failure.error, ElabError::BareUnivArgument));

        // Linear exponentials are outside the frozen v1 alphabet.
        let bang = tel(vec![Expr::Bang(Box::new(Expr::Var(1)))]);
        let failure = elaborate_telescope(&signature, &bang, 15).unwrap_err();
        assert!(matches!(
            failure.error,
            ElabError::LinearExponentialOutsideFrozenAlphabet
        ));

        // Library reference beyond the visible prefix.
        let out_of_prefix = tel(vec![app(Expr::Lib(15), Expr::Var(1))]);
        let failure = elaborate_telescope(&signature, &out_of_prefix, 14).unwrap_err();
        assert!(matches!(
            failure.error,
            ElabError::LibOutOfSignature {
                step: 15,
                visible: 14
            }
        ));
    }

    #[test]
    fn perturbed_beta_rule_changes_the_derivation_and_normal_form() {
        // Perturb the oriented beta clause of step 4: the computation
        // clause App(Lam(Var 1), Var 2) becomes App(Lam(Var 1), Var 3),
        // whose reduct is a different normal form. The derivation hash
        // moves with it, so any token or certificate embedding the
        // original derivation fails closed on replay.
        let signature = SealedSignature::genesis_del_h15();
        let pi_former = signature.entry(4).expect("step 4").telescope.clone();
        let baseline = elaborate_telescope(&signature, &pi_former, 3).expect("baseline");

        let mut mutated = pi_former.clone();
        mutated.clauses[2] = ClauseRec::new(
            ClauseRole::Elimination,
            app(lam(Expr::Var(1)), Expr::Var(3)),
        );
        let perturbed = elaborate_telescope(&signature, &mutated, 3).expect("perturbed");

        assert_ne!(baseline.derivation_hash, perturbed.derivation_hash);
        assert_eq!(baseline.clauses[2].normal_form, Expr::Var(1));
        // The constant function still returns Var(1) — but the subject
        // hash and derivation differ, and a swapped orientation
        // (Lam(Var 3): the identity) yields a different reduct entirely.
        let mut swapped = pi_former.clone();
        swapped.clauses[2] = ClauseRec::new(
            ClauseRole::Elimination,
            app(lam(Expr::Var(5)), Expr::Var(2)),
        );
        let swapped_elab = elaborate_telescope(&signature, &swapped, 3).expect("swapped");
        // At clause index 2 the scope is ambient 2 + priors 2 = 4, so the
        // Lam binder is level 5: Lam(Var 5) is the identity and the
        // reduct is the argument Var(2).
        assert_eq!(swapped_elab.clauses[2].normal_form, Expr::Var(2));
        assert_ne!(swapped_elab.derivation_hash, baseline.derivation_hash);
    }

    #[test]
    fn token_seal_holds_across_the_crate_source() {
        // Ground rule 4 enforcement (mirrored in `cargo xtask
        // token-seal-check`; the repo has no CI, so this test is the
        // gate): whitespace-normalized scan of every pen-type source
        // file for public token constructors, public token fields,
        // Deserialize/Default derives, and unsanctioned fns returning a
        // token (wrapped returns included).
        const TOKEN_TYPES: [&str; 3] =
            ["TypedEliminatorToken", "NaturalityToken", "TypedLiftToken"];
        const SANCTIONED: [&str; 3] = [
            "issue_typed_eliminator_token",
            "issue_naturality_token",
            "issue_typed_lift_token",
        ];
        fn brace_block(source: &str) -> Option<&str> {
            let open = source.find('{')?;
            let mut depth = 0usize;
            for (index, character) in source[open..].char_indices() {
                match character {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(&source[..open + index + 1]);
                        }
                    }
                    _ => {}
                }
            }
            None
        }

        let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut violations: Vec<String> = Vec::new();
        for entry in std::fs::read_dir(&src_dir).expect("read src dir") {
            let path = entry.expect("dir entry").path();
            if path.extension().is_none_or(|ext| ext != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("read source");
            let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");

            let mut search = 0;
            while let Some(offset) = flat[search..].find("pub fn ") {
                let start = search + offset;
                let rest = &flat[start..];
                let signature_end = rest.find('{').unwrap_or(rest.len());
                let signature = &rest[..signature_end];
                let fn_name = signature
                    .trim_start_matches("pub fn ")
                    .split(['(', '<', ' '])
                    .next()
                    .unwrap_or("");
                if let Some(return_position) = signature.rfind("->") {
                    let return_type = &signature[return_position..];
                    if TOKEN_TYPES.iter().any(|token| return_type.contains(token))
                        && !SANCTIONED.contains(&fn_name)
                    {
                        violations.push(format!("unsanctioned token-returning fn {fn_name}"));
                    }
                }
                search = start + 7;
            }

            for token in TOKEN_TYPES {
                if let Some(struct_start) = flat.find(&format!("pub struct {token} {{")) {
                    if let Some(body) = brace_block(&flat[struct_start..]) {
                        let inner = &body[body.find('{').unwrap_or(0) + 1..];
                        assert!(!inner.contains("pub "), "{token} exposes a public field");
                    }
                    let preamble = &flat[struct_start.saturating_sub(600)..struct_start];
                    assert!(
                        !preamble.contains("Deserialize"),
                        "{token} derives Deserialize"
                    );
                    assert!(!preamble.contains("Default"), "{token} derives Default");
                }
                assert!(
                    !flat.contains(&format!("Deserialize for {token}"))
                        && !flat.contains(&format!("Deserialize<'de> for {token}")),
                    "manual Deserialize impl for {token}"
                );
                let mut impl_search = 0;
                while let Some(offset) = flat[impl_search..].find(&format!("impl {token} {{")) {
                    let impl_start = impl_search + offset;
                    let Some(body) = brace_block(&flat[impl_start..]) else {
                        break;
                    };
                    let mut fn_search = 0;
                    while let Some(fn_offset) = body[fn_search..].find("pub fn ") {
                        let fn_start = fn_search + fn_offset;
                        let signature_end =
                            body[fn_start..].find('{').unwrap_or(body.len() - fn_start);
                        let signature = &body[fn_start..fn_start + signature_end];
                        if let Some(return_position) = signature.rfind("->") {
                            assert!(
                                !signature[return_position..].contains("Self"),
                                "{token} exposes a public constructor"
                            );
                        }
                        fn_search = fn_start + 7;
                    }
                    impl_search = impl_start + body.len();
                }
            }
        }
        assert!(
            violations.is_empty(),
            "token seal violations: {violations:?}"
        );
    }

    #[test]
    fn derivation_hashes_are_deterministic_and_signature_bound() {
        let signature = SealedSignature::genesis_del_h15();
        let dct = &signature.entry(15).expect("step 15").telescope;
        let first = elaborate_telescope(&signature, dct, 14).expect("elaborates");
        let second = elaborate_telescope(&signature, dct, 14).expect("elaborates");
        assert_eq!(first.derivation_hash, second.derivation_hash);
        assert_eq!(first, second);
    }
}
