//! Replayable term-level `Internal` closure for the adopted transparent
//! ambient-former partition.
//!
//! The token proves a closed candidate clause by replaying the exact kernel
//! constructor tree.  Leaves must be the ambient universe, sealed predecessor
//! constants, local binders, or strictly-prior candidate fields whose
//! `Internal` certificate hashes are supplied by the ordered classifier.
//! Every non-leaf must belong to the registered transparent former inventory.
//! Candidate-fresh application heads, `PathCon`, candidate-declared
//! formations, ambient parameters, and uncertified dependencies fail closed.

use crate::elaborate::{DerivationNode, SealedSignature, candidate_hash, elaborate_telescope};
use crate::normalize::{normalize, substitute_level, whnf};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const AMBIENT_FORMER_INTERNALITY_VERSION: &str = "kernel-ambient-former-closure-internal-v1";
const NORMALIZATION_FUEL: u32 = 256;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransparentFormer {
    AmbientUniverse,
    VariableReference,
    SealedLibraryConstant,
    LambdaIntroduction,
    Application,
    PiFormation,
    SigmaFormation,
    IdentityFormation,
    ReflexivityIntroduction,
    SuspensionFormation,
    TruncationFormation,
    FlatFormation,
    SharpFormation,
    DiscreteFormation,
    ShapeFormation,
    NextFormation,
    EventuallyFormation,
}

pub fn registered_transparent_formers() -> Vec<TransparentFormer> {
    vec![
        TransparentFormer::AmbientUniverse,
        TransparentFormer::VariableReference,
        TransparentFormer::SealedLibraryConstant,
        TransparentFormer::LambdaIntroduction,
        TransparentFormer::Application,
        TransparentFormer::PiFormation,
        TransparentFormer::SigmaFormation,
        TransparentFormer::IdentityFormation,
        TransparentFormer::ReflexivityIntroduction,
        TransparentFormer::SuspensionFormation,
        TransparentFormer::TruncationFormation,
        TransparentFormer::FlatFormation,
        TransparentFormer::SharpFormation,
        TransparentFormer::DiscreteFormation,
        TransparentFormer::ShapeFormation,
        TransparentFormer::NextFormation,
        TransparentFormer::EventuallyFormation,
    ]
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InternalTermProvenance {
    AmbientUniverse,
    SealedLibraryConstant {
        step: u32,
    },
    LocalBinder {
        binder: u32,
    },
    CertifiedPriorClause {
        clause_index: u16,
        certificate_hash: String,
    },
    TransparentFormer {
        former: TransparentFormer,
        premise_hashes: Vec<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientInternalTermProjection {
    pub expression: Expr,
    pub scope_len: u32,
    pub derivation_rule: String,
    pub former: TransparentFormer,
    pub provenance: InternalTermProvenance,
    pub premises: Vec<AmbientInternalTermProjection>,
    pub certified_field_dependencies: Vec<u16>,
    pub normal_form: Expr,
    pub normalization_steps: u32,
    pub registered_transparent_former: bool,
    pub all_premises_internal: bool,
    pub no_candidate_fresh_head: bool,
    pub typed_result_preserved: bool,
    pub full_provenance_retained: bool,
    pub independent_demand_orbit_exported: bool,
    pub internal_term_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientFormerClosureProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub candidate_hash: String,
    pub elaboration_hash: String,
    pub clause_index: u16,
    pub declared_role: ClauseRole,
    pub kernel_role: ClauseRole,
    pub outer_scope: u32,
    pub expression: Expr,
    pub clause_normal_form: Expr,
    pub certified_prior_clauses: BTreeMap<u16, String>,
    pub term_evidence: AmbientInternalTermProjection,
    pub candidate_declared_formation_excluded: bool,
    pub charged_path_constructor_excluded: bool,
    pub every_dependency_strictly_prior: bool,
    pub every_premise_replayed_internal: bool,
    pub typed_result_preserved: bool,
    pub full_provenance_retained: bool,
    pub standing_orbit_exception_preserved: bool,
    pub closed_candidate: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

/// Opaque proof object.  Persisted artifacts carry the public replay
/// projection; production issuance remains tied to kernel re-elaboration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AmbientFormerClosureToken {
    projection: AmbientFormerClosureProjection,
}

impl AmbientFormerClosureToken {
    pub fn projection(&self) -> &AmbientFormerClosureProjection {
        &self.projection
    }

    pub fn derivation_hash(&self) -> &str {
        &self.projection.derivation_hash
    }

    pub fn marginal_nu(&self) -> u32 {
        self.projection.marginal_nu
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum AmbientFormerInternalityError {
    #[error("candidate failed elaboration: {0}")]
    Elaboration(String),
    #[error("normalization failed: {0}")]
    Normalization(String),
    #[error("clause index {clause_index} is out of range")]
    ClauseOutOfRange { clause_index: u16 },
    #[error("ambient-former closure is closed-only; candidate ambient arity is {ambient}")]
    GuardedCandidate { ambient: u32 },
    #[error("candidate-declared formation at clause {clause_index} is charged, not Internal")]
    CandidateDeclaredFormation { clause_index: u16 },
    #[error("charged PathCon is excluded from ambient-former closure")]
    ChargedPathConstructor,
    #[error("linear-exponential former is outside the frozen v1 grammar")]
    OutsideFrozenGrammar,
    #[error("ambient parameter premise {parameter} has no guarded Internal certificate")]
    UncertifiedAmbientParameter { parameter: u32 },
    #[error("candidate field {clause_index} lacks a strictly-prior Internal certificate")]
    UncertifiedCandidateField { clause_index: u16 },
    #[error("candidate-fresh application head is excluded")]
    CandidateFreshApplicationHead,
    #[error("transparent former derivation shape mismatch for {expression:?} under rule {rule}")]
    DerivationShape { expression: Expr, rule: String },
    #[error("typed result normalization differs from the clause normal form")]
    TypedResultMismatch,
    #[error("ambient-former closure projection replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(AMBIENT_FORMER_INTERNALITY_VERSION, domain, value))
        .expect("ambient-former Internal evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn parse_rule_index(rule: &str, prefix: &str) -> Option<u32> {
    rule.strip_prefix(prefix)?.parse().ok()
}

fn candidate_field_head(expression: &Expr, derivation: &DerivationNode) -> bool {
    match expression {
        Expr::Var(_) => derivation.rule.starts_with("field-ref-"),
        Expr::App(function, _) => derivation
            .children
            .first()
            .is_some_and(|child| candidate_field_head(function, child)),
        _ => false,
    }
}

fn finish_node(
    expression: &Expr,
    derivation: &DerivationNode,
    scope_len: u32,
    former: TransparentFormer,
    provenance: InternalTermProvenance,
    premises: Vec<AmbientInternalTermProjection>,
    direct_dependencies: Vec<u16>,
    no_candidate_fresh_head: bool,
) -> Result<AmbientInternalTermProjection, AmbientFormerInternalityError> {
    let normalized = normalize(expression, scope_len, NORMALIZATION_FUEL)
        .map_err(|error| AmbientFormerInternalityError::Normalization(error.to_string()))?;
    let mut dependency_set = direct_dependencies.into_iter().collect::<BTreeSet<_>>();
    for premise in &premises {
        dependency_set.extend(premise.certified_field_dependencies.iter().copied());
    }
    let certified_field_dependencies = dependency_set.into_iter().collect::<Vec<_>>();
    let registered_transparent_former = registered_transparent_formers().contains(&former);
    let all_premises_internal = premises.iter().all(|premise| premise.internal_term_issued);
    let typed_result_preserved = true;
    let full_provenance_retained = premises
        .iter()
        .all(|premise| premise.full_provenance_retained);
    let independent_demand_orbit_exported = false;
    let internal_term_issued = registered_transparent_former
        && all_premises_internal
        && no_candidate_fresh_head
        && typed_result_preserved
        && full_provenance_retained
        && !independent_demand_orbit_exported;
    let marginal_nu = 0;
    let mut projection = AmbientInternalTermProjection {
        expression: expression.clone(),
        scope_len,
        derivation_rule: derivation.rule.clone(),
        former,
        provenance,
        premises,
        certified_field_dependencies,
        normal_form: normalized.expr,
        normalization_steps: normalized.steps,
        registered_transparent_former,
        all_premises_internal,
        no_candidate_fresh_head,
        typed_result_preserved,
        full_provenance_retained,
        independent_demand_orbit_exported,
        internal_term_issued,
        marginal_nu,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("ambient-internal-term", &projection);
    Ok(projection)
}

fn issue_unary_node(
    expression: &Expr,
    inner: &Expr,
    derivation: &DerivationNode,
    expected_rule: &str,
    former: TransparentFormer,
    scope_len: u32,
    visible_library: u32,
    certified_fields: &BTreeMap<u16, String>,
) -> Result<AmbientInternalTermProjection, AmbientFormerInternalityError> {
    if derivation.rule != expected_rule || derivation.children.len() != 1 {
        return Err(AmbientFormerInternalityError::DerivationShape {
            expression: expression.clone(),
            rule: derivation.rule.clone(),
        });
    }
    let premise = issue_internal_term(
        inner,
        &derivation.children[0],
        scope_len,
        visible_library,
        certified_fields,
    )?;
    let premise_hashes = vec![premise.derivation_hash.clone()];
    finish_node(
        expression,
        derivation,
        scope_len,
        former,
        InternalTermProvenance::TransparentFormer {
            former,
            premise_hashes,
        },
        vec![premise],
        Vec::new(),
        true,
    )
}

fn issue_internal_term(
    expression: &Expr,
    derivation: &DerivationNode,
    scope_len: u32,
    visible_library: u32,
    certified_fields: &BTreeMap<u16, String>,
) -> Result<AmbientInternalTermProjection, AmbientFormerInternalityError> {
    match expression {
        Expr::Univ if derivation.rule == "univ-form" && derivation.children.is_empty() => {
            finish_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::AmbientUniverse,
                InternalTermProvenance::AmbientUniverse,
                Vec::new(),
                Vec::new(),
                true,
            )
        }
        Expr::Lib(step)
            if (1..=visible_library).contains(step)
                && derivation.rule == "library-constant"
                && derivation.children.is_empty() =>
        {
            finish_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::SealedLibraryConstant,
                InternalTermProvenance::SealedLibraryConstant { step: *step },
                Vec::new(),
                Vec::new(),
                true,
            )
        }
        Expr::Var(_) if derivation.children.is_empty() => {
            if let Some(binder) = parse_rule_index(&derivation.rule, "local-var-") {
                return finish_node(
                    expression,
                    derivation,
                    scope_len,
                    TransparentFormer::VariableReference,
                    InternalTermProvenance::LocalBinder { binder },
                    Vec::new(),
                    Vec::new(),
                    true,
                );
            }
            if let Some(parameter) = parse_rule_index(&derivation.rule, "ambient-param-") {
                return Err(AmbientFormerInternalityError::UncertifiedAmbientParameter {
                    parameter,
                });
            }
            if let Some(clause) = parse_rule_index(&derivation.rule, "field-ref-") {
                let clause_index = u16::try_from(clause).map_err(|_| {
                    AmbientFormerInternalityError::UncertifiedCandidateField {
                        clause_index: u16::MAX,
                    }
                })?;
                let certificate_hash = certified_fields.get(&clause_index).ok_or(
                    AmbientFormerInternalityError::UncertifiedCandidateField { clause_index },
                )?;
                return finish_node(
                    expression,
                    derivation,
                    scope_len,
                    TransparentFormer::VariableReference,
                    InternalTermProvenance::CertifiedPriorClause {
                        clause_index,
                        certificate_hash: certificate_hash.clone(),
                    },
                    Vec::new(),
                    vec![clause_index],
                    true,
                );
            }
            Err(AmbientFormerInternalityError::DerivationShape {
                expression: expression.clone(),
                rule: derivation.rule.clone(),
            })
        }
        Expr::Lam(body) => {
            if derivation.rule != "lam-intro" || derivation.children.len() != 1 {
                return Err(AmbientFormerInternalityError::DerivationShape {
                    expression: expression.clone(),
                    rule: derivation.rule.clone(),
                });
            }
            let premise = issue_internal_term(
                body,
                &derivation.children[0],
                scope_len + 1,
                visible_library,
                certified_fields,
            )?;
            let premise_hashes = vec![premise.derivation_hash.clone()];
            finish_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::LambdaIntroduction,
                InternalTermProvenance::TransparentFormer {
                    former: TransparentFormer::LambdaIntroduction,
                    premise_hashes,
                },
                vec![premise],
                Vec::new(),
                true,
            )
        }
        Expr::App(function, argument) => {
            if candidate_field_head(function, derivation.children.first().unwrap_or(derivation)) {
                return Err(AmbientFormerInternalityError::CandidateFreshApplicationHead);
            }
            let (expected_children, reduced) = match derivation.rule.as_str() {
                "app-beta" => {
                    let function_whnf =
                        whnf(function, scope_len, NORMALIZATION_FUEL).map_err(|error| {
                            AmbientFormerInternalityError::Normalization(error.to_string())
                        })?;
                    let Expr::Lam(body) = function_whnf.expr else {
                        return Err(AmbientFormerInternalityError::DerivationShape {
                            expression: expression.clone(),
                            rule: derivation.rule.clone(),
                        });
                    };
                    (3, Some(substitute_level(&body, scope_len + 1, argument)))
                }
                "univ-app-form" | "app-fun" | "app-el-pi" | "app-stuck" => (2, None),
                _ => {
                    return Err(AmbientFormerInternalityError::DerivationShape {
                        expression: expression.clone(),
                        rule: derivation.rule.clone(),
                    });
                }
            };
            if derivation.children.len() != expected_children {
                return Err(AmbientFormerInternalityError::DerivationShape {
                    expression: expression.clone(),
                    rule: derivation.rule.clone(),
                });
            }
            let mut premises = vec![
                issue_internal_term(
                    function,
                    &derivation.children[0],
                    scope_len,
                    visible_library,
                    certified_fields,
                )?,
                issue_internal_term(
                    argument,
                    &derivation.children[1],
                    scope_len,
                    visible_library,
                    certified_fields,
                )?,
            ];
            if let Some(reduced) = reduced {
                premises.push(issue_internal_term(
                    &reduced,
                    &derivation.children[2],
                    scope_len,
                    visible_library,
                    certified_fields,
                )?);
            }
            let premise_hashes = premises
                .iter()
                .map(|premise| premise.derivation_hash.clone())
                .collect();
            finish_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::Application,
                InternalTermProvenance::TransparentFormer {
                    former: TransparentFormer::Application,
                    premise_hashes,
                },
                premises,
                Vec::new(),
                true,
            )
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            let (expected_rule, former) = if matches!(expression, Expr::Pi(_, _)) {
                ("pi-form", TransparentFormer::PiFormation)
            } else {
                ("sigma-form", TransparentFormer::SigmaFormation)
            };
            if derivation.rule != expected_rule || derivation.children.len() != 2 {
                return Err(AmbientFormerInternalityError::DerivationShape {
                    expression: expression.clone(),
                    rule: derivation.rule.clone(),
                });
            }
            let premises = vec![
                issue_internal_term(
                    domain,
                    &derivation.children[0],
                    scope_len,
                    visible_library,
                    certified_fields,
                )?,
                issue_internal_term(
                    codomain,
                    &derivation.children[1],
                    scope_len + 1,
                    visible_library,
                    certified_fields,
                )?,
            ];
            let premise_hashes = premises
                .iter()
                .map(|premise| premise.derivation_hash.clone())
                .collect();
            finish_node(
                expression,
                derivation,
                scope_len,
                former,
                InternalTermProvenance::TransparentFormer {
                    former,
                    premise_hashes,
                },
                premises,
                Vec::new(),
                true,
            )
        }
        Expr::Id(ty, left, right) => {
            if derivation.rule != "id-form" || derivation.children.len() != 3 {
                return Err(AmbientFormerInternalityError::DerivationShape {
                    expression: expression.clone(),
                    rule: derivation.rule.clone(),
                });
            }
            let premises = vec![
                issue_internal_term(
                    ty,
                    &derivation.children[0],
                    scope_len,
                    visible_library,
                    certified_fields,
                )?,
                issue_internal_term(
                    left,
                    &derivation.children[1],
                    scope_len,
                    visible_library,
                    certified_fields,
                )?,
                issue_internal_term(
                    right,
                    &derivation.children[2],
                    scope_len,
                    visible_library,
                    certified_fields,
                )?,
            ];
            let premise_hashes = premises
                .iter()
                .map(|premise| premise.derivation_hash.clone())
                .collect();
            finish_node(
                expression,
                derivation,
                scope_len,
                TransparentFormer::IdentityFormation,
                InternalTermProvenance::TransparentFormer {
                    former: TransparentFormer::IdentityFormation,
                    premise_hashes,
                },
                premises,
                Vec::new(),
                true,
            )
        }
        Expr::Refl(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "refl-intro",
            TransparentFormer::ReflexivityIntroduction,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::Susp(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "susp-form",
            TransparentFormer::SuspensionFormation,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::Trunc(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "trunc-form",
            TransparentFormer::TruncationFormation,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::Flat(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "flat-form",
            TransparentFormer::FlatFormation,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::Sharp(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "sharp-form",
            TransparentFormer::SharpFormation,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::Disc(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "disc-form",
            TransparentFormer::DiscreteFormation,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::Shape(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "shape-form",
            TransparentFormer::ShapeFormation,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::Next(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "next-form",
            TransparentFormer::NextFormation,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::Eventually(inner) => issue_unary_node(
            expression,
            inner,
            derivation,
            "eventually-form",
            TransparentFormer::EventuallyFormation,
            scope_len,
            visible_library,
            certified_fields,
        ),
        Expr::PathCon(_) => Err(AmbientFormerInternalityError::ChargedPathConstructor),
        Expr::Bang(_) | Expr::WhyNot(_) => Err(AmbientFormerInternalityError::OutsideFrozenGrammar),
        _ => Err(AmbientFormerInternalityError::DerivationShape {
            expression: expression.clone(),
            rule: derivation.rule.clone(),
        }),
    }
}

pub fn issue_ambient_former_closure_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    certified_prior_clauses: &BTreeMap<u16, String>,
) -> Result<AmbientFormerClosureToken, AmbientFormerInternalityError> {
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| AmbientFormerInternalityError::Elaboration(error.to_string()))?;
    if elaboration.ambient_parameters != 0 {
        return Err(AmbientFormerInternalityError::GuardedCandidate {
            ambient: elaboration.ambient_parameters,
        });
    }
    let index = usize::from(clause_index);
    let clause = candidate
        .clauses
        .get(index)
        .ok_or(AmbientFormerInternalityError::ClauseOutOfRange { clause_index })?;
    let clause_elaboration = elaboration
        .clauses
        .get(index)
        .ok_or(AmbientFormerInternalityError::ClauseOutOfRange { clause_index })?;
    if clause.role == ClauseRole::Formation {
        return Err(AmbientFormerInternalityError::CandidateDeclaredFormation { clause_index });
    }
    let every_dependency_strictly_prior = certified_prior_clauses
        .keys()
        .all(|dependency| *dependency < clause_index);
    if !every_dependency_strictly_prior {
        let bad = certified_prior_clauses
            .keys()
            .find(|dependency| **dependency >= clause_index)
            .copied()
            .unwrap_or(clause_index);
        return Err(AmbientFormerInternalityError::UncertifiedCandidateField { clause_index: bad });
    }
    let term_evidence = issue_internal_term(
        &clause.expr,
        &clause_elaboration.derivation,
        u32::from(clause_index),
        visible_library,
        certified_prior_clauses,
    )?;
    let charged_path_constructor_excluded = !contains_path_constructor(&clause.expr);
    if !charged_path_constructor_excluded {
        return Err(AmbientFormerInternalityError::ChargedPathConstructor);
    }
    let candidate_declared_formation_excluded = true;
    let every_premise_replayed_internal = term_evidence.internal_term_issued;
    let typed_result_preserved = term_evidence.typed_result_preserved
        && term_evidence.normal_form == clause_elaboration.normal_form;
    if !typed_result_preserved {
        return Err(AmbientFormerInternalityError::TypedResultMismatch);
    }
    let full_provenance_retained = term_evidence.full_provenance_retained;
    let standing_orbit_exception_preserved = !term_evidence.independent_demand_orbit_exported;
    let closed_candidate = true;
    let internal_closure_issued = candidate_declared_formation_excluded
        && charged_path_constructor_excluded
        && every_dependency_strictly_prior
        && every_premise_replayed_internal
        && typed_result_preserved
        && full_provenance_retained
        && standing_orbit_exception_preserved
        && closed_candidate;
    let marginal_nu = 0;
    let mut projection = AmbientFormerClosureProjection {
        version: AMBIENT_FORMER_INTERNALITY_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        candidate_hash: candidate_hash(candidate),
        elaboration_hash: elaboration.derivation_hash,
        clause_index,
        declared_role: clause.role,
        kernel_role: clause_elaboration.kernel_role,
        outer_scope: u32::from(clause_index),
        expression: clause.expr.clone(),
        clause_normal_form: clause_elaboration.normal_form.clone(),
        certified_prior_clauses: certified_prior_clauses.clone(),
        term_evidence,
        candidate_declared_formation_excluded,
        charged_path_constructor_excluded,
        every_dependency_strictly_prior,
        every_premise_replayed_internal,
        typed_result_preserved,
        full_provenance_retained,
        standing_orbit_exception_preserved,
        closed_candidate,
        internal_closure_issued,
        marginal_nu,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("ambient-former-closure-token", &projection);
    Ok(AmbientFormerClosureToken { projection })
}

fn contains_path_constructor(expression: &Expr) -> bool {
    match expression {
        Expr::PathCon(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            contains_path_constructor(left) || contains_path_constructor(right)
        }
        Expr::Id(ty, left, right) => {
            contains_path_constructor(ty)
                || contains_path_constructor(left)
                || contains_path_constructor(right)
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
        | Expr::WhyNot(inner) => contains_path_constructor(inner),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) => false,
    }
}

pub fn replay_ambient_former_closure_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    token: &AmbientFormerClosureToken,
) -> Result<(), AmbientFormerInternalityError> {
    let reissued = issue_ambient_former_closure_token(
        signature,
        candidate,
        visible_library,
        token.projection.clause_index,
        &token.projection.certified_prior_clauses,
    )?;
    if reissued == *token {
        Ok(())
    } else {
        Err(AmbientFormerInternalityError::ReplayMismatch)
    }
}

pub fn replay_ambient_former_closure_projection(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    projection: &AmbientFormerClosureProjection,
) -> Result<(), AmbientFormerInternalityError> {
    let token = issue_ambient_former_closure_token(
        signature,
        candidate,
        visible_library,
        projection.clause_index,
        &projection.certified_prior_clauses,
    )?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(AmbientFormerInternalityError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::ClauseRec;

    fn introduction_candidate(expression: Expr) -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(ClauseRole::Introduction, expression),
        ])
    }

    fn certified_field_zero() -> BTreeMap<u16, String> {
        BTreeMap::from([(0, "blake3:certified-field-zero".to_owned())])
    }

    #[test]
    fn app_stuck_over_sealed_premises_replays_at_zero_credit() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = introduction_candidate(Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Lib(14)),
            Box::new(Expr::Lib(15)),
        ))));
        let token = issue_ambient_former_closure_token(
            &signature,
            &candidate,
            15,
            1,
            &certified_field_zero(),
        )
        .expect("ambient former token");
        assert!(token.projection().internal_closure_issued);
        assert_eq!(token.projection().marginal_nu, 0);
        assert_eq!(
            token.projection().term_evidence.premises[0].former,
            TransparentFormer::Application
        );
        replay_ambient_former_closure_token(&signature, &candidate, 15, &token)
            .expect("token replay");
    }

    #[test]
    fn transparent_former_inventory_replays_at_term_level() {
        let signature = SealedSignature::genesis_del_h15();
        let bodies = [
            Expr::Lam(Box::new(Expr::Univ)),
            Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Lib(15))),
            Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            Expr::Sigma(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            Expr::Id(
                Box::new(Expr::Univ),
                Box::new(Expr::Lib(14)),
                Box::new(Expr::Lib(15)),
            ),
            Expr::Refl(Box::new(Expr::Lib(14))),
            Expr::Susp(Box::new(Expr::Lib(14))),
            Expr::Trunc(Box::new(Expr::Lib(14))),
            Expr::Flat(Box::new(Expr::Lib(14))),
            Expr::Sharp(Box::new(Expr::Lib(14))),
            Expr::Disc(Box::new(Expr::Lib(14))),
            Expr::Shape(Box::new(Expr::Lib(14))),
            Expr::Next(Box::new(Expr::Lib(14))),
            Expr::Eventually(Box::new(Expr::Lib(14))),
        ];
        for body in bodies {
            let candidate = introduction_candidate(Expr::Lam(Box::new(body)));
            let token = issue_ambient_former_closure_token(
                &signature,
                &candidate,
                15,
                1,
                &certified_field_zero(),
            )
            .expect("registered transparent former");
            assert!(token.projection().internal_closure_issued);
            assert_eq!(token.projection().marginal_nu, 0);
        }
    }

    #[test]
    fn charged_and_fresh_heads_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let fresh_head = introduction_candidate(Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Lib(15)),
        ))));
        assert!(matches!(
            issue_ambient_former_closure_token(
                &signature,
                &fresh_head,
                15,
                1,
                &certified_field_zero(),
            ),
            Err(AmbientFormerInternalityError::CandidateFreshApplicationHead)
        ));

        let path = introduction_candidate(Expr::Lam(Box::new(Expr::PathCon(1))));
        assert!(matches!(
            issue_ambient_former_closure_token(&signature, &path, 15, 1, &certified_field_zero(),),
            Err(AmbientFormerInternalityError::ChargedPathConstructor)
        ));

        let formation = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(ClauseRole::Formation, Expr::Flat(Box::new(Expr::Lib(15)))),
        ]);
        assert!(matches!(
            issue_ambient_former_closure_token(
                &signature,
                &formation,
                15,
                1,
                &certified_field_zero(),
            ),
            Err(AmbientFormerInternalityError::CandidateDeclaredFormation { .. })
        ));
    }

    #[test]
    fn uncertified_dependencies_and_projection_mutations_fail() {
        let signature = SealedSignature::genesis_del_h15();
        let field_premise =
            introduction_candidate(Expr::Lam(Box::new(Expr::Refl(Box::new(Expr::Var(1))))));
        assert!(matches!(
            issue_ambient_former_closure_token(&signature, &field_premise, 15, 1, &BTreeMap::new(),),
            Err(AmbientFormerInternalityError::UncertifiedCandidateField { clause_index: 0 })
        ));
        let token = issue_ambient_former_closure_token(
            &signature,
            &field_premise,
            15,
            1,
            &certified_field_zero(),
        )
        .expect("certified field premise");
        let mut projection = token.projection().clone();
        projection.marginal_nu = 1;
        assert_eq!(
            replay_ambient_former_closure_projection(&signature, &field_premise, 15, &projection,),
            Err(AmbientFormerInternalityError::ReplayMismatch)
        );
    }
}
