//! Replayable semantics for the adopted future-hole hypothesis definition.
//!
//! A future hole is an explicitly declared open hypothesis.  The issuer below
//! is intentionally closed: it receives no candidate verdict, count, score,
//! bar, winner, or motive.  Motives are read from the already generated typed
//! parameter telescope (or from the closed structural-hole declaration) and
//! are bound before any historical filler is inspected.

use crate::a3_demand_grammar::{
    A3DemandSchemeOrigin, A3HistoricalWindow, A3RuleConstructor, A3TypedClauseSource,
    issue_historical_a3_demand_grammar,
};
use crate::debt_guard::directive_debt_timeline;
use crate::typed_families::ParamSort;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::ambient_former_internality::{
    TransparentFormer, issue_ambient_former_closure_token, registered_transparent_formers,
    replay_ambient_former_closure_token,
};
use pen_type::contextual_internality::{
    ContextualMotive, issue_ambient_context_declaration_token,
    replay_ambient_context_declaration_projection,
};
use pen_type::elaborate::{
    KernelTy, SealedSignature, elaborate_single_clause, elaborate_telescope,
};
use pen_type::motive_parametric_coherence::{
    issue_motive_parametric_coherence_theorem, issue_motive_typed_closed_assignment,
    replay_motive_parametric_coherence_theorem,
};
use pen_type::substitution::{
    SortedParameterContext, SubstitutionImage, issue_structural_substitution,
    replay_structural_substitution,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const FUTURE_HOLE_HYPOTHESIS_SCHEMA: &str = "future-hole-hypothesis-definition-v1";
pub const FUTURE_HOLE_HYPOTHESIS_DATE: &str = "2026-07-21";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/future_hole_definition_adjudication.md");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("future_hole_hypothesis.rs");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("a3_demand_grammar.rs");
const CONTEXTUAL_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/contextual_internality.rs");
const PARAMETRIC_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence.rs");
const AMBIENT_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/ambient_former_internality.rs");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(FUTURE_HOLE_HYPOTHESIS_SCHEMA, domain, value))
        .expect("future-hole evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FutureHoleSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FutureHoleChargeAudit {
    pub marginal_kappa: u32,
    pub marginal_nu: u32,
    pub anchors_minted: u32,
    pub demand_orbits_minted: u32,
    pub credit_minted: bool,
    pub filler_minted_credit: bool,
    pub derivation_hash: String,
}

impl FutureHoleChargeAudit {
    fn zero() -> Self {
        let mut audit = Self {
            marginal_kappa: 0,
            marginal_nu: 0,
            anchors_minted: 0,
            demand_orbits_minted: 0,
            credit_minted: false,
            filler_minted_credit: false,
            derivation_hash: String::new(),
        };
        audit.derivation_hash = tagged_hash("zero-charge", &audit);
        audit
    }

    fn is_zero(&self) -> bool {
        self.marginal_kappa == 0
            && self.marginal_nu == 0
            && self.anchors_minted == 0
            && self.demand_orbits_minted == 0
            && !self.credit_minted
            && !self.filler_minted_credit
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FutureClosureNode {
    pub expression: Expr,
    pub former: TransparentFormer,
    pub premises: Vec<FutureClosureNode>,
    pub derivation_hash: String,
}

fn closure_node(
    expression: &Expr,
    inventory: &BTreeSet<TransparentFormer>,
) -> Result<FutureClosureNode, FutureHoleHypothesisError> {
    let (former, children): (TransparentFormer, Vec<&Expr>) = match expression {
        Expr::Univ => (TransparentFormer::AmbientUniverse, vec![]),
        Expr::Var(_) => (TransparentFormer::VariableReference, vec![]),
        Expr::Lib(_) => (TransparentFormer::SealedLibraryConstant, vec![]),
        Expr::Lam(body) => (TransparentFormer::LambdaIntroduction, vec![body]),
        Expr::App(left, right) => (TransparentFormer::Application, vec![left, right]),
        Expr::Pi(left, right) => (TransparentFormer::PiFormation, vec![left, right]),
        Expr::Sigma(left, right) => (TransparentFormer::SigmaFormation, vec![left, right]),
        Expr::Id(ty, left, right) => (TransparentFormer::IdentityFormation, vec![ty, left, right]),
        Expr::Refl(body) => (TransparentFormer::ReflexivityIntroduction, vec![body]),
        Expr::Susp(body) => (TransparentFormer::SuspensionFormation, vec![body]),
        Expr::Trunc(body) => (TransparentFormer::TruncationFormation, vec![body]),
        Expr::Flat(body) => (TransparentFormer::FlatFormation, vec![body]),
        Expr::Sharp(body) => (TransparentFormer::SharpFormation, vec![body]),
        Expr::Disc(body) => (TransparentFormer::DiscreteFormation, vec![body]),
        Expr::Shape(body) => (TransparentFormer::ShapeFormation, vec![body]),
        Expr::Next(body) => (TransparentFormer::NextFormation, vec![body]),
        Expr::Eventually(body) => (TransparentFormer::EventuallyFormation, vec![body]),
        Expr::PathCon(_) => {
            return Err(FutureHoleHypothesisError::OutsideClosure(
                "PathCon remains charged".to_owned(),
            ));
        }
        Expr::Bang(_) | Expr::WhyNot(_) => {
            return Err(FutureHoleHypothesisError::OutsideClosure(
                "linear-exponential former is outside the frozen closure".to_owned(),
            ));
        }
    };
    if !inventory.contains(&former) {
        return Err(FutureHoleHypothesisError::OutsideClosure(format!(
            "unregistered transparent former {former:?}"
        )));
    }
    let premises = children
        .into_iter()
        .map(|child| closure_node(child, inventory))
        .collect::<Result<Vec<_>, _>>()?;
    let derivation_hash = tagged_hash(
        "transparent-future-closure-node",
        &(expression, former, &premises),
    );
    Ok(FutureClosureNode {
        expression: expression.clone(),
        former,
        premises,
        derivation_hash,
    })
}

fn flatten_formers(node: &FutureClosureNode, output: &mut Vec<TransparentFormer>) {
    output.push(node.former);
    for premise in &node.premises {
        flatten_formers(premise, output);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FutureHoleOpenJudgment {
    pub stage: u32,
    pub visible_library_at_registration: u32,
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub source_step: Option<u32>,
    pub source_clause: Option<u16>,
    pub structural_constructor: Option<String>,
    pub body: Expr,
    pub source_kernel_type_json: String,
    pub output_kernel_type_json: String,
    pub output_type_preserved: bool,
    pub declared_motives: Vec<ContextualMotive>,
    pub motive_declaration_source: String,
    pub motives_declared_before_verdict_or_filler: bool,
    pub motive_inferred_repaired_or_outcome_selected: bool,
    pub demand_register_role_adapter_used: bool,
    pub open_judgment_kernel_typed: bool,
    pub open_elaboration_hash: String,
    pub ambient_declaration_hash: String,
    pub every_motive_bn_formable: bool,
    pub closure_tree: FutureClosureNode,
    pub registered_formers_used: Vec<TransparentFormer>,
    pub every_term_node_in_adopted_closure: bool,
    pub generic_substitution_theorem_hash: String,
    pub hypothetical_derivation_replayable: bool,
    pub d_membership_derivation_hash: String,
    pub d_membership_issued: bool,
    pub charge: FutureHoleChargeAudit,
    pub named_gap: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FutureHoleDischargeReplay {
    pub registration_stage: u32,
    pub filler_step: u32,
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub structural_constructor: String,
    pub package_level_filler: Expr,
    pub filler_is_whole_sealed_entry_not_clause_reference: bool,
    pub filler_used_only_after_motive_registration: bool,
    pub prefix_extension_from: u32,
    pub prefix_extension_to: u32,
    pub support_local_before_extension: bool,
    pub expression_preserved_across_prefix_extension: bool,
    pub motives_preserved_across_prefix_extension: bool,
    pub output_type_preserved_across_prefix_extension: bool,
    pub extension_declaration_hash: String,
    pub closed_filler_internality_hash: String,
    pub closed_assignment_hash: String,
    pub specialization_substitution_hash: String,
    pub specialized_expression: Expr,
    pub specialized_expression_is_filler: bool,
    pub specialized_closed_replay_valid: bool,
    pub constructor_stands_before_filler: bool,
    pub constructor_absent_after_filler: bool,
    pub clause4_prime_instantiation_replayed: bool,
    pub charge: FutureHoleChargeAudit,
    pub named_gap: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FutureHoleHypothesisCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<FutureHoleSourceBinding>,
    pub adoption_text_hash: String,
    pub adoption_replayed: bool,
    pub issuer_accepts_no_motive_verdict_count_bar_or_winner_input: bool,
    pub registered_transparent_former_count: usize,
    pub generic_substitution_theorem_hash: String,
    pub unary_actions: Vec<FutureHoleOpenJudgment>,
    pub structural_holes: Vec<FutureHoleOpenJudgment>,
    pub historical_discharges: Vec<FutureHoleDischargeReplay>,
    pub unary_action_count: usize,
    pub structural_hole_count: usize,
    pub every_unary_action_hypothetically_derived: bool,
    pub every_structural_hole_hypothetically_derived: bool,
    pub f_fh1_every_historical_discharge_replayed: bool,
    pub f_fh2_no_motive_inference_repair_or_outcome_selection: bool,
    pub f_fh3_zero_charge_and_no_credit_anchor_or_orbit: bool,
    pub f_fh4_named_gaps: Vec<String>,
    pub future_hole_definition_complete: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FutureHoleHypothesisReplay {
    pub valid: bool,
    pub unary_action_count: usize,
    pub structural_hole_count: usize,
    pub historical_discharge_count: usize,
    pub f_fh1_passed: bool,
    pub f_fh2_passed: bool,
    pub f_fh3_passed: bool,
    pub named_gaps: Vec<String>,
    pub complete: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum FutureHoleHypothesisError {
    #[error("adoption authority is absent or malformed")]
    Adoption,
    #[error("A3 prerequisite failed: {0}")]
    A3(String),
    #[error("future-hole invariant failed: {0}")]
    Invariant(String),
    #[error("term lies outside the adopted transparent closure: {0}")]
    OutsideClosure(String),
    #[error("typed declaration failed: {0}")]
    Declaration(String),
    #[error("substitution replay failed: {0}")]
    Substitution(String),
}

fn source_bindings() -> Vec<FutureHoleSourceBinding> {
    [
        (
            "docs/future_hole_definition_adjudication.md",
            "adopted_future_hole_definition",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-eval/src/future_hole_hypothesis.rs",
            "closed_definition_issuer_and_replay",
            THIS_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "typed_A3_scheme_and_instance_generator",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/contextual_internality.rs",
            "declared_motive_context_api",
            CONTEXTUAL_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence.rs",
            "clause_4_prime_generic_substitution_theorem",
            PARAMETRIC_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/ambient_former_internality.rs",
            "registered_17_former_closed_replay",
            AMBIENT_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| FutureHoleSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn certificate_digest(certificate: &FutureHoleHypothesisCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn adoption_replayed() -> bool {
    std::str::from_utf8(ADJUDICATION_BYTES).is_ok_and(|text| {
        text.contains("future-hole-hypothesis-definition-v1")
            && text.contains("**Adopted.**")
            && text.contains("ambient hypotheses with declared")
            && text.contains("actions on holes")
            && text.contains("discharge is clause-4")
            && text.contains("holes and fillings mint nothing")
    })
}

fn issue_open_judgment(
    signature: &SealedSignature,
    stage: u32,
    visible_library: u32,
    instance_id: &str,
    scheme_id: &str,
    source_step: Option<u32>,
    source_clause: Option<u16>,
    structural_constructor: Option<String>,
    body: Expr,
    declared_motives: Vec<ContextualMotive>,
    source_kernel_type: &KernelTy,
    motive_declaration_source: &str,
    theorem_hash: &str,
    inventory: &BTreeSet<TransparentFormer>,
) -> Result<FutureHoleOpenJudgment, FutureHoleHypothesisError> {
    // Canonical family expressions use level numbers relative to an explicit
    // parameter telescope.  Inferring that telescope from the expression is
    // unsound under binders (a level can otherwise be mistaken for a local).
    // The declaration carrier forces the already-enumerated arity, while the
    // actual scheme body is checked by the kernel in that explicit context.
    let ambient_arity = u32::try_from(declared_motives.len()).map_err(|_| {
        FutureHoleHypothesisError::Invariant("ambient motive arity exceeds u32".to_owned())
    })?;
    let declaration_body = if ambient_arity == 0 {
        Expr::Univ
    } else {
        Expr::Var(ambient_arity)
    };
    let candidate = Telescope::new(vec![ClauseRec::new(
        ClauseRole::Formation,
        declaration_body,
    )]);
    let explicit_elaboration = elaborate_single_clause(&body, ambient_arity, &[], visible_library)
        .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    let declaration = issue_ambient_context_declaration_token(
        signature,
        &candidate,
        visible_library,
        declared_motives.clone(),
    )
    .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    replay_ambient_context_declaration_projection(
        signature,
        &candidate,
        visible_library,
        declaration.projection(),
    )
    .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    let closure_tree = closure_node(&body, inventory)?;
    let mut registered_formers_used = Vec::new();
    flatten_formers(&closure_tree, &mut registered_formers_used);
    let output_kernel_type_json =
        serde_json::to_string(&explicit_elaboration.kernel_ty).expect("kernel type serializes");
    let source_kernel_type_json =
        serde_json::to_string(source_kernel_type).expect("kernel type serializes");
    let output_type_preserved = output_kernel_type_json == source_kernel_type_json;
    let charge = FutureHoleChargeAudit::zero();
    let named_gap = (!output_type_preserved)
        .then(|| "FUTURE_HOLE_CANONICAL_ACTION_OUTPUT_TYPE_MISMATCH".to_owned());
    let hypothetical_derivation_replayable = declaration.projection().every_motive_b15_formable
        && !registered_formers_used.is_empty()
        && named_gap.is_none();
    let d_membership_derivation_hash = tagged_hash(
        "hypothetical-D-membership",
        &(
            stage,
            instance_id,
            scheme_id,
            &body,
            &declared_motives,
            &explicit_elaboration,
            &declaration.projection().declaration_hash,
            &closure_tree.derivation_hash,
            theorem_hash,
            &charge,
            hypothetical_derivation_replayable,
        ),
    );
    let d_membership_issued = hypothetical_derivation_replayable && charge.is_zero();
    let open_elaboration_hash = tagged_hash(
        "explicit-open-body-elaboration",
        &(
            ambient_arity,
            &body,
            &explicit_elaboration,
            declaration.projection().declaration_hash.as_str(),
        ),
    );
    let mut row = FutureHoleOpenJudgment {
        stage,
        visible_library_at_registration: visible_library,
        a3_instance_id: instance_id.to_owned(),
        a3_scheme_id: scheme_id.to_owned(),
        source_step,
        source_clause,
        structural_constructor,
        body,
        source_kernel_type_json,
        output_kernel_type_json,
        output_type_preserved,
        declared_motives,
        motive_declaration_source: motive_declaration_source.to_owned(),
        motives_declared_before_verdict_or_filler: true,
        motive_inferred_repaired_or_outcome_selected: false,
        demand_register_role_adapter_used: true,
        open_judgment_kernel_typed: true,
        open_elaboration_hash,
        ambient_declaration_hash: declaration.projection().declaration_hash.clone(),
        every_motive_bn_formable: declaration.projection().every_motive_b15_formable,
        closure_tree,
        registered_formers_used,
        every_term_node_in_adopted_closure: true,
        generic_substitution_theorem_hash: theorem_hash.to_owned(),
        hypothetical_derivation_replayable,
        d_membership_derivation_hash,
        d_membership_issued,
        charge,
        named_gap,
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("open-judgment", &row);
    Ok(row)
}

/// Issue the structural future hypotheses of one already-generated A3
/// window.  The caller supplies only a sealed prefix and the count-blind A3
/// window derived from it; the motive, body, charging, and closure authority
/// are fixed by the adopted definition rather than by a branch verdict.
pub fn issue_structural_future_holes_for_window(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
) -> Result<Vec<FutureHoleOpenJudgment>, FutureHoleHypothesisError> {
    if !adoption_replayed() {
        return Err(FutureHoleHypothesisError::Adoption);
    }
    let theorem = issue_motive_parametric_coherence_theorem()
        .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    replay_motive_parametric_coherence_theorem(theorem.projection())
        .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    let inventory = registered_transparent_formers()
        .into_iter()
        .collect::<BTreeSet<_>>();
    if inventory.len() != 17 {
        return Err(FutureHoleHypothesisError::Invariant(format!(
            "transparent-former inventory has {}, expected 17",
            inventory.len()
        )));
    }

    let mut rows = Vec::new();
    for scheme in &window.schemes {
        let A3DemandSchemeOrigin::StructuralCompletion { constructor, .. } = &scheme.origin else {
            continue;
        };
        let instances = window
            .instances
            .iter()
            .filter(|instance| instance.scheme_id == scheme.scheme_id)
            .collect::<Vec<_>>();
        if instances.len() != 1 {
            return Err(FutureHoleHypothesisError::Invariant(format!(
                "Stage {} structural scheme has {} instances",
                window.stage,
                instances.len()
            )));
        }
        rows.push(issue_open_judgment(
            signature,
            window.stage,
            window.stage.saturating_sub(1),
            &instances[0].instance_id,
            &scheme.scheme_id,
            None,
            None,
            Some(constructor.slug().to_owned()),
            Expr::Var(1),
            vec![ContextualMotive::Type],
            &KernelTy::Type,
            "adopted_structural_future_hole_declaration_Type",
            &theorem.projection().theorem_hash,
            &inventory,
        )?);
    }
    Ok(rows)
}

fn source_for_instance<'a>(
    stage16_sources: &'a [A3TypedClauseSource],
    anchor: &str,
) -> Result<&'a A3TypedClauseSource, FutureHoleHypothesisError> {
    stage16_sources
        .iter()
        .find(|source| source.anchor_id == anchor)
        .ok_or_else(|| {
            FutureHoleHypothesisError::Invariant(format!(
                "unary source anchor {anchor} does not resolve"
            ))
        })
}

fn filler_step(registration_stage: u32) -> u32 {
    if registration_stage <= 4 {
        4
    } else {
        registration_stage
    }
}

fn issue_discharge(
    signature: &SealedSignature,
    grammar: &crate::a3_demand_grammar::A3HistoricalDemandGrammar,
    open: &FutureHoleOpenJudgment,
) -> Result<FutureHoleDischargeReplay, FutureHoleHypothesisError> {
    let constructor = open.structural_constructor.clone().ok_or_else(|| {
        FutureHoleHypothesisError::Invariant("structural row omitted constructor".to_owned())
    })?;
    let filler_step = filler_step(open.stage);
    let filler = Expr::Lib(filler_step);
    let open_candidate = Telescope::new(vec![ClauseRec::new(
        ClauseRole::Introduction,
        open.body.clone(),
    )]);
    let extended_elaboration = elaborate_telescope(signature, &open_candidate, filler_step)
        .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    let extended_declaration = issue_ambient_context_declaration_token(
        signature,
        &open_candidate,
        filler_step,
        open.declared_motives.clone(),
    )
    .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    replay_ambient_context_declaration_projection(
        signature,
        &open_candidate,
        filler_step,
        extended_declaration.projection(),
    )
    .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    let extended_output = serde_json::to_string(&extended_elaboration.clauses[0].kernel_ty)
        .expect("kernel type serializes");

    let closed_candidate = Telescope::new(vec![ClauseRec::new(
        ClauseRole::Introduction,
        filler.clone(),
    )]);
    let closed = issue_ambient_former_closure_token(
        signature,
        &closed_candidate,
        filler_step,
        0,
        &BTreeMap::new(),
    )
    .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    replay_ambient_former_closure_token(signature, &closed_candidate, filler_step, &closed)
        .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    let assignment = issue_motive_typed_closed_assignment(
        signature,
        filler_step,
        open.declared_motives.clone(),
        vec![filler.clone()],
        vec![closed.projection().derivation_hash.clone()],
    )
    .map_err(|error| FutureHoleHypothesisError::Substitution(error.to_string()))?;
    let substitution = issue_structural_substitution(
        SortedParameterContext::all_type(open.declared_motives.len() as u32),
        SortedParameterContext::all_type(0),
        vec![SubstitutionImage {
            source_parameter: 1,
            term: filler.clone(),
        }],
        open.body.clone(),
    )
    .map_err(|error| FutureHoleHypothesisError::Substitution(error.to_string()))?;
    replay_structural_substitution(&substitution)
        .map_err(|error| FutureHoleHypothesisError::Substitution(error.to_string()))?;
    let specialized_expression = substitution.result().clone();
    let specialized_expression_is_filler = specialized_expression == filler;
    let specialized_closed_replay_valid = specialized_expression_is_filler;

    let before = grammar
        .windows
        .iter()
        .find(|window| window.stage == open.stage)
        .ok_or_else(|| {
            FutureHoleHypothesisError::Invariant(format!(
                "historical Stage {} is absent",
                open.stage
            ))
        })?;
    let after = grammar
        .windows
        .iter()
        .find(|window| window.stage == filler_step + 1)
        .ok_or_else(|| {
            FutureHoleHypothesisError::Invariant(format!(
                "post-filler Stage {} is absent",
                filler_step + 1
            ))
        })?;
    let constructor_stands_before_filler = before
        .constructor_evidence
        .iter()
        .any(|evidence| evidence.constructor.slug() == constructor);
    let constructor_absent_after_filler = after
        .constructor_evidence
        .iter()
        .all(|evidence| evidence.constructor.slug() != constructor);
    let support_local_before_extension = open
        .body
        .lib_refs()
        .iter()
        .all(|step| *step <= open.visible_library_at_registration);
    let expression_preserved_across_prefix_extension = extended_elaboration.clauses[0].normal_form
        == elaborate_telescope(
            signature,
            &open_candidate,
            open.visible_library_at_registration,
        )
        .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?
        .clauses[0]
            .normal_form;
    let motives_preserved_across_prefix_extension = extended_declaration
        .projection()
        .hypotheses
        .iter()
        .map(|hypothesis| hypothesis.motive.clone())
        .eq(open.declared_motives.iter().cloned());
    let output_type_preserved_across_prefix_extension =
        extended_output == open.output_kernel_type_json;
    let charge = FutureHoleChargeAudit::zero();
    let clause4_prime_instantiation_replayed = support_local_before_extension
        && expression_preserved_across_prefix_extension
        && motives_preserved_across_prefix_extension
        && output_type_preserved_across_prefix_extension
        && assignment.projection().every_image_closed
        && assignment.projection().every_image_motive_typed
        && assignment.projection().every_image_internal
        && specialized_closed_replay_valid
        && constructor_stands_before_filler
        && constructor_absent_after_filler
        && charge.is_zero();
    let named_gap = (!clause4_prime_instantiation_replayed)
        .then(|| "FUTURE_HOLE_HISTORICAL_DISCHARGE_REPLAY_FAILED".to_owned());
    let mut row = FutureHoleDischargeReplay {
        registration_stage: open.stage,
        filler_step,
        a3_instance_id: open.a3_instance_id.clone(),
        a3_scheme_id: open.a3_scheme_id.clone(),
        structural_constructor: constructor,
        package_level_filler: filler,
        filler_is_whole_sealed_entry_not_clause_reference: true,
        filler_used_only_after_motive_registration: true,
        prefix_extension_from: open.visible_library_at_registration,
        prefix_extension_to: filler_step,
        support_local_before_extension,
        expression_preserved_across_prefix_extension,
        motives_preserved_across_prefix_extension,
        output_type_preserved_across_prefix_extension,
        extension_declaration_hash: extended_declaration.projection().declaration_hash.clone(),
        closed_filler_internality_hash: closed.projection().derivation_hash.clone(),
        closed_assignment_hash: assignment.projection().assignment_hash.clone(),
        specialization_substitution_hash: substitution.derivation_hash().to_owned(),
        specialized_expression,
        specialized_expression_is_filler,
        specialized_closed_replay_valid,
        constructor_stands_before_filler,
        constructor_absent_after_filler,
        clause4_prime_instantiation_replayed,
        charge,
        named_gap,
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash("historical-discharge", &row);
    Ok(row)
}

pub fn issue_future_hole_hypothesis_certificate()
-> Result<FutureHoleHypothesisCertificate, FutureHoleHypothesisError> {
    if !adoption_replayed() {
        return Err(FutureHoleHypothesisError::Adoption);
    }
    let signature = SealedSignature::genesis_del_h15();
    let grammar = issue_historical_a3_demand_grammar(&signature)
        .map_err(|error| FutureHoleHypothesisError::A3(error.to_string()))?;
    let theorem = issue_motive_parametric_coherence_theorem()
        .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    replay_motive_parametric_coherence_theorem(theorem.projection())
        .map_err(|error| FutureHoleHypothesisError::Declaration(error.to_string()))?;
    let theorem_hash = theorem.projection().theorem_hash.clone();
    let inventory = registered_transparent_formers()
        .into_iter()
        .collect::<BTreeSet<_>>();
    if inventory.len() != 17 {
        return Err(FutureHoleHypothesisError::Invariant(format!(
            "transparent-former inventory has {}, expected 17",
            inventory.len()
        )));
    }
    let stage16 = grammar
        .windows
        .iter()
        .find(|window| window.stage == 16)
        .ok_or_else(|| FutureHoleHypothesisError::Invariant("A3 omits Stage 16".to_owned()))?;

    let mut unary_actions = Vec::new();
    for instance in &stage16.instances {
        let scheme = stage16
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
            .ok_or_else(|| {
                FutureHoleHypothesisError::Invariant(format!(
                    "orphan Stage-16 instance {}",
                    instance.instance_id
                ))
            })?;
        if scheme.rule_constructor != A3RuleConstructor::UnaryAction {
            continue;
        }
        let anchor = instance.source_anchor_ids.first().ok_or_else(|| {
            FutureHoleHypothesisError::Invariant("unary instance has no source".to_owned())
        })?;
        let source = source_for_instance(&stage16.typed_sources, anchor)?;
        if !source
            .canonical_presentation
            .parameters
            .iter()
            .all(|sort| *sort == ParamSort::Type)
        {
            return Err(FutureHoleHypothesisError::Invariant(format!(
                "Stage-16 source {}:{} has a non-Type canonical parameter",
                source.step, source.clause_index
            )));
        }
        let motives = source
            .canonical_presentation
            .parameters
            .iter()
            .map(|_| ContextualMotive::Type)
            .collect::<Vec<_>>();
        unary_actions.push(issue_open_judgment(
            &signature,
            16,
            15,
            &instance.instance_id,
            &scheme.scheme_id,
            Some(source.step),
            Some(source.clause_index),
            None,
            source.canonical_presentation.canonical_normal_form.clone(),
            motives,
            &source.kernel_type,
            "frozen_canonical_Type_parameter_telescope_at_scheme_registration",
            &theorem_hash,
            &inventory,
        )?);
    }

    let mut structural_holes = Vec::new();
    for window in &grammar.windows {
        structural_holes.extend(issue_structural_future_holes_for_window(
            &signature, window,
        )?);
    }
    let historical_discharges = structural_holes
        .iter()
        .map(|open| issue_discharge(&signature, &grammar, open))
        .collect::<Result<Vec<_>, _>>()?;

    let unary_action_count = unary_actions.len();
    let structural_hole_count = structural_holes.len();
    let every_unary_action_hypothetically_derived = unary_action_count == 17
        && unary_actions
            .iter()
            .all(|row| row.d_membership_issued && row.named_gap.is_none());
    let every_structural_hole_hypothetically_derived = structural_hole_count == 13
        && structural_holes
            .iter()
            .all(|row| row.d_membership_issued && row.named_gap.is_none());
    let f_fh1_every_historical_discharge_replayed = historical_discharges.len() == 13
        && historical_discharges
            .iter()
            .all(|row| row.clause4_prime_instantiation_replayed && row.named_gap.is_none());
    let f_fh2_no_motive_inference_repair_or_outcome_selection =
        unary_actions.iter().chain(&structural_holes).all(|row| {
            row.motives_declared_before_verdict_or_filler
                && !row.motive_inferred_repaired_or_outcome_selected
        }) && historical_discharges
            .iter()
            .all(|row| row.filler_used_only_after_motive_registration);
    let f_fh3_zero_charge_and_no_credit_anchor_or_orbit = unary_actions
        .iter()
        .chain(&structural_holes)
        .all(|row| row.charge.is_zero())
        && historical_discharges.iter().all(|row| row.charge.is_zero());
    let f_fh4_named_gaps = unary_actions
        .iter()
        .chain(&structural_holes)
        .filter_map(|row| row.named_gap.clone())
        .chain(
            historical_discharges
                .iter()
                .filter_map(|row| row.named_gap.clone()),
        )
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let future_hole_definition_complete = every_unary_action_hypothetically_derived
        && every_structural_hole_hypothetically_derived
        && f_fh1_every_historical_discharge_replayed
        && f_fh2_no_motive_inference_repair_or_outcome_selection
        && f_fh3_zero_charge_and_no_credit_anchor_or_orbit
        && f_fh4_named_gaps.is_empty();

    // The coarse ledger is used only as F-FH1's regression corpus.  Its
    // labels never enter motive declaration or closure construction.
    let timeline = directive_debt_timeline();
    if timeline.len() != 16 || timeline[2].required_packages != ["former_eliminator"] {
        return Err(FutureHoleHypothesisError::Invariant(
            "verified coarse O-ladder regression corpus drifted".to_owned(),
        ));
    }

    let mut certificate = FutureHoleHypothesisCertificate {
        schema: FUTURE_HOLE_HYPOTHESIS_SCHEMA.to_owned(),
        date: FUTURE_HOLE_HYPOTHESIS_DATE.to_owned(),
        source_bindings: source_bindings(),
        adoption_text_hash: bytes_hash(ADJUDICATION_BYTES),
        adoption_replayed: true,
        issuer_accepts_no_motive_verdict_count_bar_or_winner_input: true,
        registered_transparent_former_count: inventory.len(),
        generic_substitution_theorem_hash: theorem_hash,
        unary_actions,
        structural_holes,
        historical_discharges,
        unary_action_count,
        structural_hole_count,
        every_unary_action_hypothetically_derived,
        every_structural_hole_hypothetically_derived,
        f_fh1_every_historical_discharge_replayed,
        f_fh2_no_motive_inference_repair_or_outcome_selection,
        f_fh3_zero_charge_and_no_credit_anchor_or_orbit,
        f_fh4_named_gaps,
        future_hole_definition_complete,
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> FutureHoleHypothesisReplay {
    FutureHoleHypothesisReplay {
        valid: false,
        unary_action_count: 0,
        structural_hole_count: 0,
        historical_discharge_count: 0,
        f_fh1_passed: false,
        f_fh2_passed: false,
        f_fh3_passed: false,
        named_gaps: Vec::new(),
        complete: false,
        errors: vec![error.into()],
    }
}

pub fn replay_future_hole_hypothesis_certificate(
    certificate: &FutureHoleHypothesisCertificate,
) -> FutureHoleHypothesisReplay {
    let expected = match issue_future_hole_hypothesis_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != &expected {
        errors.push("certificate differs from independent definition replay".to_owned());
    }
    FutureHoleHypothesisReplay {
        valid: errors.is_empty(),
        unary_action_count: certificate.unary_action_count,
        structural_hole_count: certificate.structural_hole_count,
        historical_discharge_count: certificate.historical_discharges.len(),
        f_fh1_passed: certificate.f_fh1_every_historical_discharge_replayed,
        f_fh2_passed: certificate.f_fh2_no_motive_inference_repair_or_outcome_selection,
        f_fh3_passed: certificate.f_fh3_zero_charge_and_no_credit_anchor_or_orbit,
        named_gaps: certificate.f_fh4_named_gaps.clone(),
        complete: certificate.future_hole_definition_complete,
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn certificate() -> FutureHoleHypothesisCertificate {
        static CERTIFICATE: OnceLock<FutureHoleHypothesisCertificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_future_hole_hypothesis_certificate().unwrap())
            .clone()
    }

    #[test]
    fn all_seventeen_actions_and_thirteen_structural_holes_replay() {
        let certificate = certificate();
        assert_eq!(certificate.unary_action_count, 17);
        assert_eq!(certificate.structural_hole_count, 13);
        assert_eq!(certificate.historical_discharges.len(), 13);
        assert!(certificate.every_unary_action_hypothetically_derived);
        assert!(certificate.every_structural_hole_hypothetically_derived);
        assert!(certificate.f_fh1_every_historical_discharge_replayed);
        assert!(certificate.f_fh4_named_gaps.is_empty());
        assert!(certificate.future_hole_definition_complete);
        assert!(replay_future_hole_hypothesis_certificate(&certificate).valid);
    }

    #[test]
    fn declared_motives_and_zero_charge_are_definition_invariants() {
        let certificate = certificate();
        assert!(certificate.f_fh2_no_motive_inference_repair_or_outcome_selection);
        assert!(certificate.f_fh3_zero_charge_and_no_credit_anchor_or_orbit);
        assert!(
            certificate
                .unary_actions
                .iter()
                .chain(&certificate.structural_holes)
                .all(|row| row.charge.is_zero())
        );
    }

    #[test]
    fn redigested_motive_and_credit_forgeries_are_rejected() {
        let original = certificate();
        let mut forged = original.clone();
        forged.unary_actions[0].declared_motives[0] = ContextualMotive::Neutral;
        forged.unary_actions[0].charge.credit_minted = true;
        forged.unary_actions[0].derivation_hash =
            tagged_hash("open-judgment", &forged.unary_actions[0]);
        forged.result_digest = certificate_digest(&forged);
        assert!(!replay_future_hole_hypothesis_certificate(&forged).valid);
    }
}
