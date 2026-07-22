//! Count-blind, finite historical-reference fragment of the ordinary
//! depth-two schema grammar.
//!
//! This module defines the *shape* of the E-2 ordinary grammar.  It does not
//! import any historical score or cardinality.  Formation is checked against
//! the operational E-1 context fragment, and every formed schema records a
//! replayable depth and two-step-support derivation.  The bridge from the
//! current `pen_core::Expr` language into this typed syntax remains the named
//! global C1 gap; consequently these objects are schema-level realizers, not
//! claims that arbitrary kernel terms have been elaborated.  In this E-2
//! batch, clause support is additionally bound to the fifteen registered
//! reference telescopes; arbitrary source-bound windows remain a named C2
//! successor obligation.

use crate::context::{
    BinderId, FormedSchemaContext, LibraryKey, SchemaContextError, SchemaSupport, TermExpr,
    TypeExpr, TypedExpression, check_typed_expression, expression_support, replay_formed_context,
};
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

pub const ORDINARY_SCHEMA2_GRAMMAR_VERSION: &str = "schema2-ordinary-grammar-e2-v1";
pub const FORMATION_COMPLETION_PACKAGE_RULE: &str = "formation-completion-package-family-rule-v1";
pub const DERIVED_ACTION_MEMBERSHIP_RULE: &str = "derived-action-generator-membership-rule-v1";
pub const GLOBAL_C1_BRIDGE_GAP: &str = "C1_PEN_CORE_EXPR_TO_SCHEMA2_TYPED_EXPRESSION_BRIDGE";
pub const E3_NORMAL_FORM_GAP: &str = "E3_FROZEN_TYPED_NORMALIZATION_AND_EQUALITY_NOT_YET_PROVED";
pub const E4_GENERATOR_MEMBERSHIP_GAP: &str =
    "E4_NATURALITY_GENERATOR_MEMBERSHIP_NOT_YET_DECIDABLE";
pub const E5_DEMAND_OUTPUT_GAP: &str = "E5_LIVE_DEMAND_ORBIT_OUTPUT_MEMBERSHIP_NOT_YET_DECIDABLE";
pub const E2_TYPED_INSTANCE_MEMBERSHIP_GAP: &str =
    "E2_TYPED_SPECIALIZATION_MEMBERSHIP_TOKEN_NOT_YET_AVAILABLE";
pub const ARBITRARY_SEALED_SUPPORT_WINDOW_GAP: &str =
    "C2_ARBITRARY_SEALED_SUPPORT_WINDOW_GRAMMAR_NOT_YET_IMPLEMENTED";

/// The finite ordinary fragment required by E-2.
///
/// This is an enum rather than a caller-provided label: adding another
/// constructor is a source change and changes the registry digest.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdinarySchemaKind {
    FreshFormation,
    PointOrUnitIntro,
    PathConstructorIntro,
    Recursor,
    Inductor,
    TruncParametricAction,
    PostPathOperation,
    PostPathCoherence,
    CellAction,
}

impl OrdinarySchemaKind {
    pub const ALL: [Self; 9] = [
        Self::FreshFormation,
        Self::PointOrUnitIntro,
        Self::PathConstructorIntro,
        Self::Recursor,
        Self::Inductor,
        Self::TruncParametricAction,
        Self::PostPathOperation,
        Self::PostPathCoherence,
        Self::CellAction,
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaDepth {
    Zero,
    One,
    Two,
}

impl SchemaDepth {
    pub const fn value(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticLocalRole {
    KernelHead,
    AdjointMate,
    SupportAction,
    Coherence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TypingRule {
    CarrierFormation,
    ElementIntroduction,
    PathIntroduction,
    NonDependentElimination,
    MotiveIndexedElimination,
    FunctorialTruncAction,
    NaryCarrierOperation,
    OrientedUnitCoherence,
    RegisteredCellAction,
}

/// Quotient rule attached to a constructor before looking at any stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotientRule {
    /// R1: a formation is carrier provenance for a completed package.  It is
    /// not a separate family unless E-4 later proves the exception.
    FormationPackageCarrier,
    /// The constructor denotes one schematic family; uniform specialization
    /// is an instance, not another family.
    StructuralNaturalFamily,
    /// R2: independence cannot be decided until generator membership exists.
    DerivedActionGeneratorMembership,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct OrdinaryConstructorDescriptor {
    pub kind: OrdinarySchemaKind,
    pub typing_rule: TypingRule,
    pub depth: SchemaDepth,
    pub role: SemanticLocalRole,
    pub quotient_rule: QuotientRule,
}

pub const ORDINARY_CONSTRUCTOR_REGISTRY: [OrdinaryConstructorDescriptor; 9] = [
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::FreshFormation,
        typing_rule: TypingRule::CarrierFormation,
        depth: SchemaDepth::Zero,
        role: SemanticLocalRole::KernelHead,
        quotient_rule: QuotientRule::FormationPackageCarrier,
    },
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::PointOrUnitIntro,
        typing_rule: TypingRule::ElementIntroduction,
        depth: SchemaDepth::One,
        role: SemanticLocalRole::KernelHead,
        quotient_rule: QuotientRule::StructuralNaturalFamily,
    },
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::PathConstructorIntro,
        typing_rule: TypingRule::PathIntroduction,
        depth: SchemaDepth::One,
        role: SemanticLocalRole::KernelHead,
        quotient_rule: QuotientRule::StructuralNaturalFamily,
    },
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::Recursor,
        typing_rule: TypingRule::NonDependentElimination,
        depth: SchemaDepth::Two,
        role: SemanticLocalRole::KernelHead,
        quotient_rule: QuotientRule::StructuralNaturalFamily,
    },
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::Inductor,
        typing_rule: TypingRule::MotiveIndexedElimination,
        depth: SchemaDepth::Two,
        role: SemanticLocalRole::KernelHead,
        quotient_rule: QuotientRule::StructuralNaturalFamily,
    },
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::TruncParametricAction,
        typing_rule: TypingRule::FunctorialTruncAction,
        depth: SchemaDepth::Two,
        role: SemanticLocalRole::SupportAction,
        quotient_rule: QuotientRule::StructuralNaturalFamily,
    },
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::PostPathOperation,
        typing_rule: TypingRule::NaryCarrierOperation,
        depth: SchemaDepth::Two,
        role: SemanticLocalRole::KernelHead,
        quotient_rule: QuotientRule::StructuralNaturalFamily,
    },
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::PostPathCoherence,
        typing_rule: TypingRule::OrientedUnitCoherence,
        depth: SchemaDepth::Two,
        role: SemanticLocalRole::Coherence,
        quotient_rule: QuotientRule::DerivedActionGeneratorMembership,
    },
    OrdinaryConstructorDescriptor {
        kind: OrdinarySchemaKind::CellAction,
        typing_rule: TypingRule::RegisteredCellAction,
        depth: SchemaDepth::Two,
        role: SemanticLocalRole::SupportAction,
        quotient_rule: QuotientRule::DerivedActionGeneratorMembership,
    },
];

pub fn ordinary_constructor_descriptor(
    kind: OrdinarySchemaKind,
) -> &'static OrdinaryConstructorDescriptor {
    ORDINARY_CONSTRUCTOR_REGISTRY
        .iter()
        .find(|descriptor| descriptor.kind == kind)
        .expect("OrdinarySchemaKind::ALL and the registry are definitionally aligned")
}

pub fn ordinary_constructor_registry_digest() -> String {
    tagged_hash(
        "ordinary-constructor-registry",
        &ORDINARY_CONSTRUCTOR_REGISTRY,
    )
}

pub fn replay_ordinary_constructor_registry(expected_digest: &str) -> bool {
    expected_digest == ordinary_constructor_registry_digest()
        && ORDINARY_CONSTRUCTOR_REGISTRY
            .iter()
            .map(|descriptor| descriptor.kind)
            .collect::<BTreeSet<_>>()
            == OrdinarySchemaKind::ALL.into_iter().collect::<BTreeSet<_>>()
        && ORDINARY_CONSTRUCTOR_REGISTRY
            .iter()
            .all(|descriptor| descriptor.depth.value() <= SchemaDepth::Two.value())
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SupportWindow {
    predecessor_step: u32,
    current_step: u32,
}

impl SupportWindow {
    pub fn new(predecessor_step: u32, current_step: u32) -> Result<Self, OrdinaryGrammarError> {
        if predecessor_step.checked_add(1) != Some(current_step) {
            return Err(OrdinaryGrammarError::NonAdjacentSupportWindow {
                predecessor_step,
                current_step,
            });
        }
        Ok(Self {
            predecessor_step,
            current_step,
        })
    }

    pub const fn predecessor_step(self) -> u32 {
        self.predecessor_step
    }

    pub const fn current_step(self) -> u32 {
        self.current_step
    }

    pub const fn contains(self, step: u32) -> bool {
        step == self.predecessor_step || step == self.current_step
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClauseAnchor {
    pub step: u32,
    pub clause: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DepthTwoProof {
    constructor: OrdinarySchemaKind,
    derived_depth: SchemaDepth,
    bound: u8,
    within_bound: bool,
    derivation_hash: String,
}

impl DepthTwoProof {
    pub const fn derived_depth(&self) -> SchemaDepth {
        self.derived_depth
    }

    pub const fn within_bound(&self) -> bool {
        self.within_bound
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SupportWindowProof {
    window: SupportWindow,
    anchors: Vec<ClauseAnchor>,
    source_telescopes: Vec<SupportSourceTelescope>,
    has_current_generator: bool,
    every_anchor_is_local: bool,
    derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SupportSourceTelescope {
    pub step: u32,
    pub candidate_hash: String,
    pub clause_count: u32,
}

impl SupportWindowProof {
    pub const fn window(&self) -> SupportWindow {
        self.window
    }

    pub fn anchors(&self) -> &[ClauseAnchor] {
        &self.anchors
    }

    pub fn source_telescopes(&self) -> &[SupportSourceTelescope] {
        &self.source_telescopes
    }

    pub const fn has_current_generator(&self) -> bool {
        self.has_current_generator
    }

    pub const fn every_anchor_is_local(&self) -> bool {
        self.every_anchor_is_local
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// A reference is always a derivation digest, never a display label.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DerivationRef(String);

impl DerivationRef {
    pub fn parse(value: impl Into<String>) -> Result<Self, OrdinaryGrammarError> {
        let value = value.into();
        let Some(hex) = value.strip_prefix("blake3:") else {
            return Err(OrdinaryGrammarError::InvalidDerivationReference);
        };
        if hex.len() != 64 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(OrdinaryGrammarError::InvalidDerivationReference);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Binder in the schema-level dependent function fragment.  These binders
/// are disjoint from E-1 `BinderId`s, so a schema lambda cannot capture an
/// ambient term accidentally.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SchemaBinderId(pub u32);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SchemaDimBinderId(pub u32);

/// The registered boundary evidence carried by a cube type is bound to its
/// source step, carrier, and dimension.  E-2 checks this binding, while the
/// external archive-token join remains an explicit later bridge.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RegisteredBoundaryBundleRef {
    pub derivation: DerivationRef,
    pub source_step: u32,
    pub carrier: TypeExpr,
    pub dimension: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SchemaDimExpr {
    Zero,
    One,
    Variable { binder: SchemaDimBinderId },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SchemaTypeExpr {
    Element {
        carrier: TypeExpr,
    },
    Product {
        left: Box<SchemaTypeExpr>,
        right: Box<SchemaTypeExpr>,
    },
    Pi {
        binder: SchemaBinderId,
        domain: Box<SchemaTypeExpr>,
        codomain: Box<SchemaTypeExpr>,
    },
    Path {
        carrier: TypeExpr,
        left: Box<SchemaTermExpr>,
        right: Box<SchemaTermExpr>,
    },
    Cube {
        carrier: TypeExpr,
        boundary: Box<SchemaTermExpr>,
        dimension: u32,
        source_bundle: RegisteredBoundaryBundleRef,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SchemaTermExpr {
    Ambient {
        term: TermExpr,
    },
    Variable {
        binder: SchemaBinderId,
    },
    Pair {
        left: Box<SchemaTermExpr>,
        right: Box<SchemaTermExpr>,
    },
    Lam {
        binder: SchemaBinderId,
        domain: Box<SchemaTypeExpr>,
        body: Box<SchemaTermExpr>,
    },
    App {
        function: Box<SchemaTermExpr>,
        argument: Box<SchemaTermExpr>,
    },
    CubeAt {
        cube: Box<SchemaTermExpr>,
        coordinates: Vec<SchemaDimExpr>,
    },
    /// Face-safe action on a registered cube.  The result boundary is
    /// derived from the source boundary and the checked function; callers
    /// cannot assert an unrelated boundary as they could with a raw cube
    /// lambda.
    MapCube {
        function: Box<SchemaTermExpr>,
        cube: Box<SchemaTermExpr>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SchemaLocalDeclaration {
    pub binder: SchemaBinderId,
    pub ty: SchemaTypeExpr,
}

fn check_bundle_binding(
    context: &FormedSchemaContext,
    window: SupportWindow,
    carrier: &TypeExpr,
    dimension: u32,
    bundle: &RegisteredBoundaryBundleRef,
) -> Result<(), OrdinaryGrammarError> {
    if dimension == 0 {
        return Err(OrdinaryGrammarError::ZeroCubicalDimension);
    }
    typecheck_type(context, carrier)?;
    if bundle.source_step != window.current_step()
        || bundle.carrier != *carrier
        || bundle.dimension != dimension
    {
        return Err(OrdinaryGrammarError::BoundaryBundleBindingMismatch);
    }
    Ok(())
}

fn substitute_schema_term(
    term: &SchemaTermExpr,
    binder: SchemaBinderId,
    image: &SchemaTermExpr,
) -> SchemaTermExpr {
    match term {
        SchemaTermExpr::Ambient { .. } => term.clone(),
        SchemaTermExpr::Variable { binder: found } if *found == binder => image.clone(),
        SchemaTermExpr::Variable { .. } => term.clone(),
        SchemaTermExpr::Pair { left, right } => SchemaTermExpr::Pair {
            left: Box::new(substitute_schema_term(left, binder, image)),
            right: Box::new(substitute_schema_term(right, binder, image)),
        },
        SchemaTermExpr::Lam {
            binder: bound,
            domain,
            body,
        } => SchemaTermExpr::Lam {
            binder: *bound,
            domain: Box::new(substitute_schema_type(domain, binder, image)),
            body: if *bound == binder {
                body.clone()
            } else {
                Box::new(substitute_schema_term(body, binder, image))
            },
        },
        SchemaTermExpr::App { function, argument } => SchemaTermExpr::App {
            function: Box::new(substitute_schema_term(function, binder, image)),
            argument: Box::new(substitute_schema_term(argument, binder, image)),
        },
        SchemaTermExpr::CubeAt { cube, coordinates } => SchemaTermExpr::CubeAt {
            cube: Box::new(substitute_schema_term(cube, binder, image)),
            coordinates: coordinates.clone(),
        },
        SchemaTermExpr::MapCube { function, cube } => SchemaTermExpr::MapCube {
            function: Box::new(substitute_schema_term(function, binder, image)),
            cube: Box::new(substitute_schema_term(cube, binder, image)),
        },
    }
}

fn substitute_schema_type(
    ty: &SchemaTypeExpr,
    binder: SchemaBinderId,
    image: &SchemaTermExpr,
) -> SchemaTypeExpr {
    match ty {
        SchemaTypeExpr::Element { .. } => ty.clone(),
        SchemaTypeExpr::Product { left, right } => SchemaTypeExpr::Product {
            left: Box::new(substitute_schema_type(left, binder, image)),
            right: Box::new(substitute_schema_type(right, binder, image)),
        },
        SchemaTypeExpr::Pi {
            binder: bound,
            domain,
            codomain,
        } => SchemaTypeExpr::Pi {
            binder: *bound,
            domain: Box::new(substitute_schema_type(domain, binder, image)),
            codomain: if *bound == binder {
                codomain.clone()
            } else {
                Box::new(substitute_schema_type(codomain, binder, image))
            },
        },
        SchemaTypeExpr::Path {
            carrier,
            left,
            right,
        } => SchemaTypeExpr::Path {
            carrier: carrier.clone(),
            left: Box::new(substitute_schema_term(left, binder, image)),
            right: Box::new(substitute_schema_term(right, binder, image)),
        },
        SchemaTypeExpr::Cube {
            carrier,
            boundary,
            dimension,
            source_bundle,
        } => SchemaTypeExpr::Cube {
            carrier: carrier.clone(),
            boundary: Box::new(substitute_schema_term(boundary, binder, image)),
            dimension: *dimension,
            source_bundle: source_bundle.clone(),
        },
    }
}

fn lookup_schema_binder(
    locals: &[SchemaLocalDeclaration],
    binder: SchemaBinderId,
) -> Result<SchemaTypeExpr, OrdinaryGrammarError> {
    locals
        .iter()
        .rev()
        .find(|declaration| declaration.binder == binder)
        .map(|declaration| declaration.ty.clone())
        .ok_or(OrdinaryGrammarError::UnboundSchemaVariable { binder })
}

pub fn check_schema_type(
    context: &FormedSchemaContext,
    window: SupportWindow,
    locals: &[SchemaLocalDeclaration],
    dimensions: &[SchemaDimBinderId],
    ty: &SchemaTypeExpr,
) -> Result<(), OrdinaryGrammarError> {
    match ty {
        SchemaTypeExpr::Element { carrier } => typecheck_type(context, carrier),
        SchemaTypeExpr::Product { left, right } => {
            check_schema_type(context, window, locals, dimensions, left)?;
            check_schema_type(context, window, locals, dimensions, right)
        }
        SchemaTypeExpr::Pi {
            binder,
            domain,
            codomain,
        } => {
            check_schema_type(context, window, locals, dimensions, domain)?;
            if locals
                .iter()
                .any(|declaration| declaration.binder == *binder)
            {
                return Err(OrdinaryGrammarError::DuplicateSchemaBinder { binder: *binder });
            }
            let mut extended = locals.to_vec();
            extended.push(SchemaLocalDeclaration {
                binder: *binder,
                ty: (**domain).clone(),
            });
            check_schema_type(context, window, &extended, dimensions, codomain)
        }
        SchemaTypeExpr::Path {
            carrier,
            left,
            right,
        } => {
            typecheck_type(context, carrier)?;
            let expected = SchemaTypeExpr::Element {
                carrier: carrier.clone(),
            };
            let left_ty = infer_schema_term(context, window, locals, dimensions, left)?;
            let right_ty = infer_schema_term(context, window, locals, dimensions, right)?;
            if left_ty != expected {
                return Err(schema_type_mismatch(expected, left_ty));
            }
            if right_ty
                != (SchemaTypeExpr::Element {
                    carrier: carrier.clone(),
                })
            {
                return Err(schema_type_mismatch(
                    SchemaTypeExpr::Element {
                        carrier: carrier.clone(),
                    },
                    right_ty,
                ));
            }
            Ok(())
        }
        SchemaTypeExpr::Cube {
            carrier,
            boundary,
            dimension,
            source_bundle,
        } => {
            check_bundle_binding(context, window, carrier, *dimension, source_bundle)?;
            let found = infer_schema_term(context, window, locals, dimensions, boundary)?;
            let expected = SchemaTypeExpr::Element {
                carrier: carrier.clone(),
            };
            if found != expected {
                return Err(schema_type_mismatch(expected, found));
            }
            Ok(())
        }
    }
}

pub fn infer_schema_term(
    context: &FormedSchemaContext,
    window: SupportWindow,
    locals: &[SchemaLocalDeclaration],
    dimensions: &[SchemaDimBinderId],
    term: &SchemaTermExpr,
) -> Result<SchemaTypeExpr, OrdinaryGrammarError> {
    match term {
        SchemaTermExpr::Ambient { term } => Ok(SchemaTypeExpr::Element {
            carrier: crate::context::infer_term(context, term)?,
        }),
        SchemaTermExpr::Variable { binder } => lookup_schema_binder(locals, *binder),
        SchemaTermExpr::Pair { left, right } => Ok(SchemaTypeExpr::Product {
            left: Box::new(infer_schema_term(
                context, window, locals, dimensions, left,
            )?),
            right: Box::new(infer_schema_term(
                context, window, locals, dimensions, right,
            )?),
        }),
        SchemaTermExpr::Lam {
            binder,
            domain,
            body,
        } => {
            check_schema_type(context, window, locals, dimensions, domain)?;
            if locals
                .iter()
                .any(|declaration| declaration.binder == *binder)
            {
                return Err(OrdinaryGrammarError::DuplicateSchemaBinder { binder: *binder });
            }
            let mut extended = locals.to_vec();
            extended.push(SchemaLocalDeclaration {
                binder: *binder,
                ty: (**domain).clone(),
            });
            let codomain = infer_schema_term(context, window, &extended, dimensions, body)?;
            Ok(SchemaTypeExpr::Pi {
                binder: *binder,
                domain: domain.clone(),
                codomain: Box::new(codomain),
            })
        }
        SchemaTermExpr::App { function, argument } => {
            let function_ty = infer_schema_term(context, window, locals, dimensions, function)?;
            let SchemaTypeExpr::Pi {
                binder,
                domain,
                codomain,
            } = function_ty
            else {
                return Err(OrdinaryGrammarError::ApplicationOfNonFunction);
            };
            let argument_ty = infer_schema_term(context, window, locals, dimensions, argument)?;
            if argument_ty != *domain {
                return Err(schema_type_mismatch(*domain, argument_ty));
            }
            Ok(substitute_schema_type(&codomain, binder, argument))
        }
        SchemaTermExpr::CubeAt { cube, coordinates } => {
            let cube_ty = infer_schema_term(context, window, locals, dimensions, cube)?;
            let SchemaTypeExpr::Cube {
                carrier, dimension, ..
            } = cube_ty
            else {
                return Err(OrdinaryGrammarError::CubeApplicationOfNonCube);
            };
            if coordinates.len() != usize::try_from(dimension).expect("u32 fits usize") {
                return Err(OrdinaryGrammarError::CubeCoordinateArityMismatch);
            }
            for coordinate in coordinates {
                if let SchemaDimExpr::Variable { binder } = coordinate
                    && !dimensions.contains(binder)
                {
                    return Err(OrdinaryGrammarError::UnboundSchemaDimension { binder: *binder });
                }
            }
            Ok(SchemaTypeExpr::Element { carrier })
        }
        SchemaTermExpr::MapCube { function, cube } => {
            let function_ty = infer_schema_term(context, window, locals, dimensions, function)?;
            let SchemaTypeExpr::Pi {
                binder,
                domain,
                codomain,
            } = function_ty
            else {
                return Err(OrdinaryGrammarError::ApplicationOfNonFunction);
            };
            let cube_ty = infer_schema_term(context, window, locals, dimensions, cube)?;
            let SchemaTypeExpr::Cube {
                carrier: source_carrier,
                boundary: source_boundary,
                dimension,
                source_bundle,
            } = cube_ty
            else {
                return Err(OrdinaryGrammarError::CubeApplicationOfNonCube);
            };
            let source_element = SchemaTypeExpr::Element {
                carrier: source_carrier,
            };
            if *domain != source_element {
                return Err(schema_type_mismatch(*domain, source_element));
            }
            let result_type = substitute_schema_type(&codomain, binder, &source_boundary);
            let SchemaTypeExpr::Element {
                carrier: result_carrier,
            } = result_type
            else {
                return Err(OrdinaryGrammarError::CubeMapCodomainIsNotElement);
            };
            check_bundle_binding(context, window, &result_carrier, dimension, &source_bundle)?;
            let mapped_boundary = match function.as_ref() {
                SchemaTermExpr::Lam {
                    binder: function_binder,
                    body,
                    ..
                } => substitute_schema_term(body, *function_binder, &source_boundary),
                _ => SchemaTermExpr::App {
                    function: function.clone(),
                    argument: source_boundary,
                },
            };
            Ok(SchemaTypeExpr::Cube {
                carrier: result_carrier,
                boundary: Box::new(mapped_boundary),
                dimension,
                source_bundle,
            })
        }
    }
}

/// Exact count-blind interpretation adopted for the three positional Step-8
/// rows.  Coherence and cell-action generator membership remain pending.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step8RegisteredSignatures {
    context: FormedSchemaContext,
    window: SupportWindow,
    carrier: TypeExpr,
    base: TermExpr,
    source_cell_bundle: RegisteredBoundaryBundleRef,
    path_cell_declaration: SchemaLocalDeclaration,
    operation_declaration: SchemaLocalDeclaration,
    left_unit_type: SchemaTypeExpr,
    cell_action_type: SchemaTypeExpr,
    cell_action_term: SchemaTermExpr,
    first_slot_orientation: bool,
    operation_signature_typed: bool,
    left_unit_signature_typed: bool,
    cell_action_term_typed: bool,
    cell_action_boundary_derived_by_map_cube: bool,
    coherence_generator_membership_decided: bool,
    cell_action_generator_membership_decided: bool,
    e4_gap: String,
    external_bundle_archive_join_proved: bool,
    external_bundle_archive_join_gap: String,
    derivation_hash: String,
}

impl Step8RegisteredSignatures {
    pub fn left_unit_type(&self) -> &SchemaTypeExpr {
        &self.left_unit_type
    }

    pub fn cell_action_type(&self) -> &SchemaTypeExpr {
        &self.cell_action_type
    }

    pub fn cell_action_term(&self) -> &SchemaTermExpr {
        &self.cell_action_term
    }

    pub const fn first_slot_orientation(&self) -> bool {
        self.first_slot_orientation
    }

    pub const fn cell_action_term_typed(&self) -> bool {
        self.cell_action_term_typed
    }

    pub const fn cell_action_boundary_derived_by_map_cube(&self) -> bool {
        self.cell_action_boundary_derived_by_map_cube
    }

    pub const fn operation_signature_typed(&self) -> bool {
        self.operation_signature_typed
    }

    pub const fn left_unit_signature_typed(&self) -> bool {
        self.left_unit_signature_typed
    }

    pub const fn coherence_generator_membership_decided(&self) -> bool {
        self.coherence_generator_membership_decided
    }

    pub const fn cell_action_generator_membership_decided(&self) -> bool {
        self.cell_action_generator_membership_decided
    }

    pub const fn external_bundle_archive_join_proved(&self) -> bool {
        self.external_bundle_archive_join_proved
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_step8_registered_signatures(
    context: FormedSchemaContext,
    carrier: TypeExpr,
    base: TermExpr,
    source_bundle_derivation: DerivationRef,
) -> Result<Step8RegisteredSignatures, OrdinaryGrammarError> {
    replay_formed_context(&context)?;
    let window = SupportWindow::new(7, 8)?;
    typecheck_type(&context, &carrier)?;
    typecheck_term(&context, &base, &carrier)?;
    let source_cell_bundle = RegisteredBoundaryBundleRef {
        derivation: source_bundle_derivation,
        source_step: 8,
        carrier: carrier.clone(),
        dimension: 3,
    };
    check_bundle_binding(&context, window, &carrier, 3, &source_cell_bundle)?;

    let element = SchemaTypeExpr::Element {
        carrier: carrier.clone(),
    };
    let base_term = SchemaTermExpr::Ambient { term: base.clone() };
    let path_cell = SchemaBinderId(0);
    let operation = SchemaBinderId(1);
    let pair_argument = SchemaBinderId(2);
    let family_argument = SchemaBinderId(3);
    let path_cell_declaration = SchemaLocalDeclaration {
        binder: path_cell,
        ty: SchemaTypeExpr::Cube {
            carrier: carrier.clone(),
            boundary: Box::new(base_term.clone()),
            dimension: 3,
            source_bundle: source_cell_bundle.clone(),
        },
    };
    let operation_declaration = SchemaLocalDeclaration {
        binder: operation,
        ty: SchemaTypeExpr::Pi {
            binder: pair_argument,
            domain: Box::new(SchemaTypeExpr::Product {
                left: Box::new(element.clone()),
                right: Box::new(element.clone()),
            }),
            codomain: Box::new(element.clone()),
        },
    };
    let locals = vec![path_cell_declaration.clone(), operation_declaration.clone()];
    check_schema_type(&context, window, &[], &[], &path_cell_declaration.ty)?;
    check_schema_type(
        &context,
        window,
        &locals[..1],
        &[],
        &operation_declaration.ty,
    )?;

    let variable = SchemaTermExpr::Variable {
        binder: family_argument,
    };
    let mu_base_x = SchemaTermExpr::App {
        function: Box::new(SchemaTermExpr::Variable { binder: operation }),
        argument: Box::new(SchemaTermExpr::Pair {
            left: Box::new(base_term.clone()),
            right: Box::new(variable.clone()),
        }),
    };
    let left_unit_type = SchemaTypeExpr::Pi {
        binder: family_argument,
        domain: Box::new(element.clone()),
        codomain: Box::new(SchemaTypeExpr::Path {
            carrier: carrier.clone(),
            left: Box::new(mu_base_x.clone()),
            right: Box::new(variable.clone()),
        }),
    };
    check_schema_type(&context, window, &locals, &[], &left_unit_type)?;

    let cell_action_type = SchemaTypeExpr::Pi {
        binder: family_argument,
        domain: Box::new(element.clone()),
        codomain: Box::new(SchemaTypeExpr::Cube {
            carrier: carrier.clone(),
            boundary: Box::new(mu_base_x.clone()),
            dimension: 3,
            source_bundle: source_cell_bundle.clone(),
        }),
    };
    check_schema_type(&context, window, &locals, &[], &cell_action_type)?;
    let translated_point = SchemaBinderId(4);
    let translation = SchemaTermExpr::Lam {
        binder: translated_point,
        domain: Box::new(element.clone()),
        body: Box::new(SchemaTermExpr::App {
            function: Box::new(SchemaTermExpr::Variable { binder: operation }),
            argument: Box::new(SchemaTermExpr::Pair {
                left: Box::new(SchemaTermExpr::Variable {
                    binder: translated_point,
                }),
                right: Box::new(variable.clone()),
            }),
        }),
    };
    let cell_action_term = SchemaTermExpr::Lam {
        binder: family_argument,
        domain: Box::new(element),
        body: Box::new(SchemaTermExpr::MapCube {
            function: Box::new(translation),
            cube: Box::new(SchemaTermExpr::Variable { binder: path_cell }),
        }),
    };
    let inferred_cell_action =
        infer_schema_term(&context, window, &locals, &[], &cell_action_term)?;
    if inferred_cell_action != cell_action_type {
        return Err(schema_type_mismatch(cell_action_type, inferred_cell_action));
    }

    let first_slot_orientation = true;
    let operation_signature_typed = true;
    let left_unit_signature_typed = true;
    let cell_action_term_typed = true;
    let cell_action_boundary_derived_by_map_cube = true;
    let coherence_generator_membership_decided = false;
    let cell_action_generator_membership_decided = false;
    let e4_gap = E4_GENERATOR_MEMBERSHIP_GAP.to_owned();
    let external_bundle_archive_join_proved = false;
    let external_bundle_archive_join_gap =
        "E2_REGISTERED_BOUNDARY_BUNDLE_REFERENCE_ARCHIVE_JOIN_NOT_IN_GENERIC_GRAMMAR".to_owned();
    let derivation_hash = tagged_hash(
        "step8-registered-signatures",
        &(
            (
                context.derivation_hash(),
                window,
                &carrier,
                &base,
                &source_cell_bundle,
                &path_cell_declaration,
                &operation_declaration,
                &left_unit_type,
                &cell_action_type,
                &cell_action_term,
            ),
            (
                first_slot_orientation,
                operation_signature_typed,
                left_unit_signature_typed,
                cell_action_term_typed,
                cell_action_boundary_derived_by_map_cube,
                coherence_generator_membership_decided,
                cell_action_generator_membership_decided,
                &e4_gap,
                external_bundle_archive_join_proved,
                &external_bundle_archive_join_gap,
            ),
        ),
    );
    Ok(Step8RegisteredSignatures {
        context,
        window,
        carrier,
        base,
        source_cell_bundle,
        path_cell_declaration,
        operation_declaration,
        left_unit_type,
        cell_action_type,
        cell_action_term,
        first_slot_orientation,
        operation_signature_typed,
        left_unit_signature_typed,
        cell_action_term_typed,
        cell_action_boundary_derived_by_map_cube,
        coherence_generator_membership_decided,
        cell_action_generator_membership_decided,
        e4_gap,
        external_bundle_archive_join_proved,
        external_bundle_archive_join_gap,
        derivation_hash,
    })
}

pub fn replay_step8_registered_signatures(
    token: &Step8RegisteredSignatures,
) -> Result<(), OrdinaryGrammarError> {
    let replay = issue_step8_registered_signatures(
        token.context.clone(),
        token.carrier.clone(),
        token.base.clone(),
        token.source_cell_bundle.derivation.clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(OrdinaryGrammarError::ReplayMismatch)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitOrientation {
    Left,
    Right,
}

/// A typed semantic signature in the finite E-2 schema language.
///
/// Function spaces and motives are represented as schema constructors here,
/// rather than smuggled into E-1's deliberately small `TermExpr`.  The C1
/// bridge constant records that these signatures are not yet arbitrary
/// `pen_core::Expr` elaborations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OrdinaryInterpretation {
    FreshFormation {
        carrier: TypeExpr,
    },
    PointOrUnitIntro {
        carrier: TypeExpr,
        point: TermExpr,
    },
    PathConstructorIntro {
        carrier: TypeExpr,
        left: TermExpr,
        right: TermExpr,
        cubical_dimension: u32,
    },
    Recursor {
        carrier: TypeExpr,
        codomain: TypeExpr,
    },
    Inductor {
        carrier: TypeExpr,
        motive_fiber: TypeExpr,
    },
    TruncParametricAction {
        source_carrier: TypeExpr,
        target_carrier: TypeExpr,
    },
    PostPathOperation {
        carrier: TypeExpr,
        arity: u32,
    },
    PostPathCoherence {
        carrier: TypeExpr,
        operation: DerivationRef,
        unit: TermExpr,
        variable: TermExpr,
        orientation: UnitOrientation,
    },
    CellAction {
        carrier: TypeExpr,
        operation: DerivationRef,
        cell_dimension: u32,
        registered_boundary_bundle: DerivationRef,
    },
}

impl OrdinaryInterpretation {
    pub const fn kind(&self) -> OrdinarySchemaKind {
        match self {
            Self::FreshFormation { .. } => OrdinarySchemaKind::FreshFormation,
            Self::PointOrUnitIntro { .. } => OrdinarySchemaKind::PointOrUnitIntro,
            Self::PathConstructorIntro { .. } => OrdinarySchemaKind::PathConstructorIntro,
            Self::Recursor { .. } => OrdinarySchemaKind::Recursor,
            Self::Inductor { .. } => OrdinarySchemaKind::Inductor,
            Self::TruncParametricAction { .. } => OrdinarySchemaKind::TruncParametricAction,
            Self::PostPathOperation { .. } => OrdinarySchemaKind::PostPathOperation,
            Self::PostPathCoherence { .. } => OrdinarySchemaKind::PostPathCoherence,
            Self::CellAction { .. } => OrdinarySchemaKind::CellAction,
        }
    }

    pub fn carrier(&self) -> &TypeExpr {
        match self {
            Self::FreshFormation { carrier }
            | Self::PointOrUnitIntro { carrier, .. }
            | Self::PathConstructorIntro { carrier, .. }
            | Self::Recursor { carrier, .. }
            | Self::Inductor { carrier, .. }
            | Self::PostPathOperation { carrier, .. }
            | Self::PostPathCoherence { carrier, .. }
            | Self::CellAction { carrier, .. } => carrier,
            Self::TruncParametricAction { source_carrier, .. } => source_carrier,
        }
    }
}

/// How an occurrence is presented to the family quotient.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FamilyPresentation {
    CanonicalFamily,
    UniformInstance {
        parent_family: DerivationRef,
    },
    ExportedDemandOutput {
        live_orbit: DerivationRef,
        required_output_position: u32,
    },
}

/// Count-blind output of the family/instance rules available at E-2.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FamilyDisposition {
    /// R1 carrier provenance: never independently exported at E-2.  The
    /// exception remains pending until E-4 can decide generator membership.
    FormationCarrier {
        package_rule: String,
        independent_role_exception_gap: String,
    },
    StructuralNaturalFamily,
    UniformInstancePending {
        parent_family: DerivationRef,
        gap: String,
    },
    ExportedDemandOutputPending {
        live_orbit: DerivationRef,
        required_output_position: u32,
        gap: String,
    },
    GeneratorMembershipPending {
        rule: String,
        role: SemanticLocalRole,
        gap: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrdinarySchema {
    context: FormedSchemaContext,
    constructor: OrdinarySchemaKind,
    interpretation: OrdinaryInterpretation,
    presentation: FamilyPresentation,
    family_disposition: FamilyDisposition,
    depth_proof: DepthTwoProof,
    support_proof: SupportWindowProof,
    semantic_support: SchemaSupport,
    provisional_schema_id: String,
    formation_derivation_hash: String,
    global_c1_bridge_retired: bool,
    global_c1_gap: String,
    frozen_normal_form_available: bool,
    frozen_normal_form_gap: String,
}

impl OrdinarySchema {
    pub fn context(&self) -> &FormedSchemaContext {
        &self.context
    }

    pub const fn constructor(&self) -> OrdinarySchemaKind {
        self.constructor
    }

    pub fn interpretation(&self) -> &OrdinaryInterpretation {
        &self.interpretation
    }

    pub fn presentation(&self) -> &FamilyPresentation {
        &self.presentation
    }

    pub fn family_disposition(&self) -> &FamilyDisposition {
        &self.family_disposition
    }

    pub fn depth_proof(&self) -> &DepthTwoProof {
        &self.depth_proof
    }

    pub fn support_proof(&self) -> &SupportWindowProof {
        &self.support_proof
    }

    pub fn semantic_support(&self) -> &SchemaSupport {
        &self.semantic_support
    }

    /// This identifier is explicitly provisional until E-3 supplies frozen
    /// normalization and univalent equality.
    pub fn provisional_schema_id(&self) -> &str {
        &self.provisional_schema_id
    }

    pub fn formation_derivation_hash(&self) -> &str {
        &self.formation_derivation_hash
    }

    pub const fn global_c1_bridge_retired(&self) -> bool {
        self.global_c1_bridge_retired
    }

    pub const fn frozen_normal_form_available(&self) -> bool {
        self.frozen_normal_form_available
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum OrdinaryGrammarError {
    #[error(transparent)]
    Context(#[from] SchemaContextError),
    #[error(
        "support window ({predecessor_step},{current_step}) is not an adjacent two-step window"
    )]
    NonAdjacentSupportWindow {
        predecessor_step: u32,
        current_step: u32,
    },
    #[error("ordinary schema has no clause anchor")]
    EmptyClauseSupport,
    #[error("ordinary schema must be generated by at least one current-step clause")]
    NoCurrentClauseAnchor,
    #[error("clause anchor {step}:{clause} lies outside the support window")]
    ClauseOutsideSupport { step: u32, clause: u32 },
    #[error("clause anchor {step}:{clause} is duplicated")]
    DuplicateClauseAnchor { step: u32, clause: u32 },
    #[error("support step {step} is outside the registered historical source range 1..=15")]
    UnregisteredSupportStep { step: u32 },
    #[error("clause anchor {step}:{clause} exceeds the registered clause count {clause_count}")]
    ClauseAnchorOutOfBounds {
        step: u32,
        clause: u32,
        clause_count: u32,
    },
    #[error("constructor {constructor:?} does not match interpretation {interpretation:?}")]
    InterpretationKindMismatch {
        constructor: OrdinarySchemaKind,
        interpretation: OrdinarySchemaKind,
    },
    #[error(
        "constructor {constructor:?} cannot bypass its pending R1/R2 quotient with a caller presentation"
    )]
    PresentationBypassesPendingQuotient { constructor: OrdinarySchemaKind },
    #[error("path/cell dimension must be positive")]
    ZeroCubicalDimension,
    #[error("operation arity must be positive")]
    ZeroOperationArity,
    #[error("typed term has type {found:?}, expected carrier {expected:?}")]
    CarrierTypeMismatch { expected: TypeExpr, found: TypeExpr },
    #[error("derivation references must be canonical blake3 digests")]
    InvalidDerivationReference,
    #[error("registered boundary bundle is not bound to the current step, carrier, and dimension")]
    BoundaryBundleBindingMismatch,
    #[error("schema binder {binder:?} is unbound")]
    UnboundSchemaVariable { binder: SchemaBinderId },
    #[error("schema binder {binder:?} is duplicated")]
    DuplicateSchemaBinder { binder: SchemaBinderId },
    #[error("schema dimension is unbound: {binder:?}")]
    UnboundSchemaDimension { binder: SchemaDimBinderId },
    #[error("schema cube dimensions are duplicated")]
    DuplicateSchemaDimension,
    #[error("schema term has type {found:?}, expected {expected:?}")]
    SchemaTermTypeMismatch {
        expected: Box<SchemaTypeExpr>,
        found: Box<SchemaTypeExpr>,
    },
    #[error("schema application head is not a Pi type")]
    ApplicationOfNonFunction,
    #[error("schema cube application head is not a Cube type")]
    CubeApplicationOfNonCube,
    #[error("schema cube mapping function does not return an element type")]
    CubeMapCodomainIsNotElement,
    #[error("schema cube coordinate arity does not match its dimension")]
    CubeCoordinateArityMismatch,
    #[error("formed schema replay mismatch")]
    ReplayMismatch,
}

fn schema_type_mismatch(expected: SchemaTypeExpr, found: SchemaTypeExpr) -> OrdinaryGrammarError {
    OrdinaryGrammarError::SchemaTermTypeMismatch {
        expected: Box::new(expected),
        found: Box::new(found),
    }
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(ORDINARY_SCHEMA2_GRAMMAR_VERSION, domain, payload))
        .expect("ordinary grammar proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn typecheck_type(
    context: &FormedSchemaContext,
    expression: &TypeExpr,
) -> Result<(), OrdinaryGrammarError> {
    check_typed_expression(
        context,
        &TypedExpression::Type {
            expression: expression.clone(),
        },
    )?;
    Ok(())
}

fn typecheck_term(
    context: &FormedSchemaContext,
    expression: &TermExpr,
    ty: &TypeExpr,
) -> Result<(), OrdinaryGrammarError> {
    check_typed_expression(
        context,
        &TypedExpression::Term {
            expression: expression.clone(),
            ty: ty.clone(),
        },
    )?;
    Ok(())
}

fn check_interpretation(
    context: &FormedSchemaContext,
    interpretation: &OrdinaryInterpretation,
) -> Result<(), OrdinaryGrammarError> {
    match interpretation {
        OrdinaryInterpretation::FreshFormation { carrier } => typecheck_type(context, carrier),
        OrdinaryInterpretation::PointOrUnitIntro { carrier, point } => {
            typecheck_type(context, carrier)?;
            typecheck_term(context, point, carrier)
        }
        OrdinaryInterpretation::PathConstructorIntro {
            carrier,
            left,
            right,
            cubical_dimension,
        } => {
            if *cubical_dimension == 0 {
                return Err(OrdinaryGrammarError::ZeroCubicalDimension);
            }
            typecheck_type(context, carrier)?;
            typecheck_term(context, left, carrier)?;
            typecheck_term(context, right, carrier)
        }
        OrdinaryInterpretation::Recursor { carrier, codomain } => {
            typecheck_type(context, carrier)?;
            typecheck_type(context, codomain)
        }
        OrdinaryInterpretation::Inductor {
            carrier,
            motive_fiber,
        } => {
            typecheck_type(context, carrier)?;
            typecheck_type(context, motive_fiber)
        }
        OrdinaryInterpretation::TruncParametricAction {
            source_carrier,
            target_carrier,
        } => {
            typecheck_type(context, source_carrier)?;
            typecheck_type(context, target_carrier)
        }
        OrdinaryInterpretation::PostPathOperation { carrier, arity } => {
            if *arity == 0 {
                return Err(OrdinaryGrammarError::ZeroOperationArity);
            }
            typecheck_type(context, carrier)
        }
        OrdinaryInterpretation::PostPathCoherence {
            carrier,
            unit,
            variable,
            ..
        } => {
            typecheck_type(context, carrier)?;
            typecheck_term(context, unit, carrier)?;
            typecheck_term(context, variable, carrier)
        }
        OrdinaryInterpretation::CellAction {
            carrier,
            cell_dimension,
            ..
        } => {
            if *cell_dimension == 0 {
                return Err(OrdinaryGrammarError::ZeroCubicalDimension);
            }
            typecheck_type(context, carrier)
        }
    }
}

fn support_of_interpretation(interpretation: &OrdinaryInterpretation) -> SchemaSupport {
    let mut binders = BTreeSet::<BinderId>::new();
    let mut libraries = BTreeSet::<LibraryKey>::new();
    let mut include = |expression: TypedExpression| {
        let support = expression_support(&expression);
        binders.extend(support.binders);
        libraries.extend(support.libraries);
    };
    match interpretation {
        OrdinaryInterpretation::FreshFormation { carrier }
        | OrdinaryInterpretation::PostPathOperation { carrier, .. }
        | OrdinaryInterpretation::CellAction { carrier, .. } => {
            include(TypedExpression::Type {
                expression: carrier.clone(),
            });
        }
        OrdinaryInterpretation::PointOrUnitIntro { carrier, point } => {
            include(TypedExpression::Term {
                expression: point.clone(),
                ty: carrier.clone(),
            });
        }
        OrdinaryInterpretation::PathConstructorIntro {
            carrier,
            left,
            right,
            ..
        } => {
            include(TypedExpression::Term {
                expression: left.clone(),
                ty: carrier.clone(),
            });
            include(TypedExpression::Term {
                expression: right.clone(),
                ty: carrier.clone(),
            });
        }
        OrdinaryInterpretation::Recursor { carrier, codomain } => {
            include(TypedExpression::Type {
                expression: carrier.clone(),
            });
            include(TypedExpression::Type {
                expression: codomain.clone(),
            });
        }
        OrdinaryInterpretation::Inductor {
            carrier,
            motive_fiber,
        } => {
            include(TypedExpression::Type {
                expression: carrier.clone(),
            });
            include(TypedExpression::Type {
                expression: motive_fiber.clone(),
            });
        }
        OrdinaryInterpretation::TruncParametricAction {
            source_carrier,
            target_carrier,
        } => {
            include(TypedExpression::Type {
                expression: source_carrier.clone(),
            });
            include(TypedExpression::Type {
                expression: target_carrier.clone(),
            });
        }
        OrdinaryInterpretation::PostPathCoherence {
            carrier,
            unit,
            variable,
            ..
        } => {
            include(TypedExpression::Term {
                expression: unit.clone(),
                ty: carrier.clone(),
            });
            include(TypedExpression::Term {
                expression: variable.clone(),
                ty: carrier.clone(),
            });
        }
    }
    SchemaSupport {
        binders: binders.into_iter().collect(),
        libraries: libraries.into_iter().collect(),
    }
}

fn family_disposition(
    descriptor: &OrdinaryConstructorDescriptor,
    presentation: &FamilyPresentation,
) -> FamilyDisposition {
    if let FamilyPresentation::UniformInstance { parent_family } = presentation {
        return FamilyDisposition::UniformInstancePending {
            parent_family: parent_family.clone(),
            gap: E2_TYPED_INSTANCE_MEMBERSHIP_GAP.to_owned(),
        };
    }
    if let FamilyPresentation::ExportedDemandOutput {
        live_orbit,
        required_output_position,
    } = presentation
    {
        return FamilyDisposition::ExportedDemandOutputPending {
            live_orbit: live_orbit.clone(),
            required_output_position: *required_output_position,
            gap: E5_DEMAND_OUTPUT_GAP.to_owned(),
        };
    }
    match descriptor.quotient_rule {
        QuotientRule::FormationPackageCarrier => FamilyDisposition::FormationCarrier {
            package_rule: FORMATION_COMPLETION_PACKAGE_RULE.to_owned(),
            independent_role_exception_gap: E4_GENERATOR_MEMBERSHIP_GAP.to_owned(),
        },
        QuotientRule::StructuralNaturalFamily => FamilyDisposition::StructuralNaturalFamily,
        QuotientRule::DerivedActionGeneratorMembership => {
            FamilyDisposition::GeneratorMembershipPending {
                rule: DERIVED_ACTION_MEMBERSHIP_RULE.to_owned(),
                role: descriptor.role,
                gap: E4_GENERATOR_MEMBERSHIP_GAP.to_owned(),
            }
        }
    }
}

pub fn form_ordinary_schema(
    context: FormedSchemaContext,
    constructor: OrdinarySchemaKind,
    interpretation: OrdinaryInterpretation,
    presentation: FamilyPresentation,
    window: SupportWindow,
    mut anchors: Vec<ClauseAnchor>,
) -> Result<OrdinarySchema, OrdinaryGrammarError> {
    replay_formed_context(&context)?;
    if constructor != interpretation.kind() {
        return Err(OrdinaryGrammarError::InterpretationKindMismatch {
            constructor,
            interpretation: interpretation.kind(),
        });
    }
    if anchors.is_empty() {
        return Err(OrdinaryGrammarError::EmptyClauseSupport);
    }
    anchors.sort_unstable();
    for pair in anchors.windows(2) {
        if pair[0] == pair[1] {
            return Err(OrdinaryGrammarError::DuplicateClauseAnchor {
                step: pair[0].step,
                clause: pair[0].clause,
            });
        }
    }
    for anchor in &anchors {
        if !window.contains(anchor.step) {
            return Err(OrdinaryGrammarError::ClauseOutsideSupport {
                step: anchor.step,
                clause: anchor.clause,
            });
        }
        if !(1..=15).contains(&anchor.step) {
            return Err(OrdinaryGrammarError::UnregisteredSupportStep { step: anchor.step });
        }
        let telescope = Telescope::reference(anchor.step);
        if usize::try_from(anchor.clause).expect("u32 fits usize") >= telescope.clauses.len() {
            return Err(OrdinaryGrammarError::ClauseAnchorOutOfBounds {
                step: anchor.step,
                clause: anchor.clause,
                clause_count: u32::try_from(telescope.clauses.len())
                    .expect("historical telescope is small"),
            });
        }
    }
    let has_current_generator = anchors
        .iter()
        .any(|anchor| anchor.step == window.current_step());
    if !has_current_generator {
        return Err(OrdinaryGrammarError::NoCurrentClauseAnchor);
    }
    check_interpretation(&context, &interpretation)?;

    let descriptor = ordinary_constructor_descriptor(constructor);
    if descriptor.quotient_rule != QuotientRule::StructuralNaturalFamily
        && !matches!(presentation, FamilyPresentation::CanonicalFamily)
    {
        return Err(OrdinaryGrammarError::PresentationBypassesPendingQuotient { constructor });
    }
    let within_bound = descriptor.depth.value() <= SchemaDepth::Two.value();
    let depth_derivation_hash = tagged_hash(
        "depth-two-proof",
        &(
            constructor,
            descriptor.depth,
            SchemaDepth::Two.value(),
            within_bound,
        ),
    );
    let depth_proof = DepthTwoProof {
        constructor,
        derived_depth: descriptor.depth,
        bound: SchemaDepth::Two.value(),
        within_bound,
        derivation_hash: depth_derivation_hash,
    };
    let every_anchor_is_local = anchors.iter().all(|anchor| window.contains(anchor.step));
    let source_steps = anchors
        .iter()
        .map(|anchor| anchor.step)
        .collect::<BTreeSet<_>>();
    let source_telescopes = source_steps
        .into_iter()
        .map(|step| {
            let telescope = Telescope::reference(step);
            SupportSourceTelescope {
                step,
                candidate_hash: pen_type::elaborate::candidate_hash(&telescope),
                clause_count: u32::try_from(telescope.clauses.len())
                    .expect("historical telescope is small"),
            }
        })
        .collect::<Vec<_>>();
    let support_derivation_hash = tagged_hash(
        "support-window-proof",
        &(
            window,
            &anchors,
            &source_telescopes,
            has_current_generator,
            every_anchor_is_local,
        ),
    );
    let support_proof = SupportWindowProof {
        window,
        anchors,
        source_telescopes,
        has_current_generator,
        every_anchor_is_local,
        derivation_hash: support_derivation_hash,
    };
    let semantic_support = support_of_interpretation(&interpretation);
    let disposition = family_disposition(descriptor, &presentation);
    let provisional_schema_id = tagged_hash(
        "provisional-schema-id",
        &(
            context.derivation_hash(),
            constructor,
            &interpretation,
            &presentation,
            window,
            support_proof.anchors(),
        ),
    );
    let formation_derivation_hash = tagged_hash(
        "formed-ordinary-schema",
        &(
            context.derivation_hash(),
            descriptor,
            &interpretation,
            &presentation,
            &disposition,
            &depth_proof,
            &support_proof,
            &semantic_support,
            &provisional_schema_id,
            false,
            GLOBAL_C1_BRIDGE_GAP,
            false,
            E3_NORMAL_FORM_GAP,
        ),
    );
    Ok(OrdinarySchema {
        context,
        constructor,
        interpretation,
        presentation,
        family_disposition: disposition,
        depth_proof,
        support_proof,
        semantic_support,
        provisional_schema_id,
        formation_derivation_hash,
        global_c1_bridge_retired: false,
        global_c1_gap: GLOBAL_C1_BRIDGE_GAP.to_owned(),
        frozen_normal_form_available: false,
        frozen_normal_form_gap: E3_NORMAL_FORM_GAP.to_owned(),
    })
}

pub fn replay_ordinary_schema(schema: &OrdinarySchema) -> Result<(), OrdinaryGrammarError> {
    let replay = form_ordinary_schema(
        schema.context.clone(),
        schema.constructor,
        schema.interpretation.clone(),
        schema.presentation.clone(),
        schema.support_proof.window,
        schema.support_proof.anchors.clone(),
    )?;
    if replay == *schema {
        Ok(())
    } else {
        Err(OrdinaryGrammarError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{Declaration, form_schema_context};

    fn context() -> FormedSchemaContext {
        form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(0),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(1),
                name: "a".to_owned(),
                ty: TypeExpr::parameter(0),
            },
        ])
        .expect("formed test context")
    }

    fn anchor() -> Vec<ClauseAnchor> {
        vec![ClauseAnchor { step: 8, clause: 3 }]
    }

    #[test]
    fn registry_is_total_unique_and_depth_two_without_count_inputs() {
        let registry_kinds = ORDINARY_CONSTRUCTOR_REGISTRY
            .iter()
            .map(|descriptor| descriptor.kind)
            .collect::<BTreeSet<_>>();
        let enum_kinds = OrdinarySchemaKind::ALL.into_iter().collect::<BTreeSet<_>>();
        assert_eq!(registry_kinds, enum_kinds);
        let digest = ordinary_constructor_registry_digest();
        assert!(replay_ordinary_constructor_registry(&digest));
        assert!(!replay_ordinary_constructor_registry("blake3:changed"));
        assert!(
            ORDINARY_CONSTRUCTOR_REGISTRY
                .iter()
                .all(|descriptor| descriptor.depth.value() <= 2)
        );
        let json = serde_json::to_string(&ORDINARY_CONSTRUCTOR_REGISTRY).unwrap();
        for forbidden in ["sealed", "score", "bar", "historical_count"] {
            assert!(!json.contains(forbidden));
        }
    }

    #[test]
    fn formed_schema_replays_and_records_global_gaps() {
        let schema = form_ordinary_schema(
            context(),
            OrdinarySchemaKind::PointOrUnitIntro,
            OrdinaryInterpretation::PointOrUnitIntro {
                carrier: TypeExpr::parameter(0),
                point: TermExpr::variable(1),
            },
            FamilyPresentation::CanonicalFamily,
            SupportWindow::new(7, 8).unwrap(),
            anchor(),
        )
        .unwrap();
        replay_ordinary_schema(&schema).unwrap();
        assert!(schema.depth_proof().within_bound());
        assert!(schema.support_proof().every_anchor_is_local());
        assert!(!schema.global_c1_bridge_retired());
        assert!(!schema.frozen_normal_form_available());
        assert_eq!(
            schema.family_disposition(),
            &FamilyDisposition::StructuralNaturalFamily
        );
    }

    #[test]
    fn uniform_specialization_requires_a_typed_membership_token() {
        let parent = DerivationRef::parse(format!("blake3:{}", "a".repeat(64))).unwrap();
        let schema = form_ordinary_schema(
            context(),
            OrdinarySchemaKind::Recursor,
            OrdinaryInterpretation::Recursor {
                carrier: TypeExpr::parameter(0),
                codomain: TypeExpr::parameter(0),
            },
            FamilyPresentation::UniformInstance {
                parent_family: parent.clone(),
            },
            SupportWindow::new(7, 8).unwrap(),
            anchor(),
        )
        .unwrap();
        assert_eq!(
            schema.family_disposition(),
            &FamilyDisposition::UniformInstancePending {
                parent_family: parent,
                gap: E2_TYPED_INSTANCE_MEMBERSHIP_GAP.to_owned(),
            }
        );
    }

    #[test]
    fn demand_exports_and_r2_fail_closed_at_their_later_deciders() {
        let digest = DerivationRef::parse(format!("blake3:{}", "b".repeat(64))).unwrap();
        let operation = form_ordinary_schema(
            context(),
            OrdinarySchemaKind::PostPathOperation,
            OrdinaryInterpretation::PostPathOperation {
                carrier: TypeExpr::parameter(0),
                arity: 2,
            },
            FamilyPresentation::ExportedDemandOutput {
                live_orbit: digest.clone(),
                required_output_position: 0,
            },
            SupportWindow::new(7, 8).unwrap(),
            anchor(),
        )
        .unwrap();
        assert!(matches!(
            operation.family_disposition(),
            FamilyDisposition::ExportedDemandOutputPending { gap, .. }
                if gap == E5_DEMAND_OUTPUT_GAP
        ));

        let coherence = form_ordinary_schema(
            context(),
            OrdinarySchemaKind::PostPathCoherence,
            OrdinaryInterpretation::PostPathCoherence {
                carrier: TypeExpr::parameter(0),
                operation: digest,
                unit: TermExpr::variable(1),
                variable: TermExpr::variable(1),
                orientation: UnitOrientation::Left,
            },
            FamilyPresentation::CanonicalFamily,
            SupportWindow::new(7, 8).unwrap(),
            anchor(),
        )
        .unwrap();
        assert!(matches!(
            coherence.family_disposition(),
            FamilyDisposition::GeneratorMembershipPending { rule, gap, .. }
                if rule == DERIVED_ACTION_MEMBERSHIP_RULE && gap == E4_GENERATOR_MEMBERSHIP_GAP
        ));

        let bypass = form_ordinary_schema(
            context(),
            OrdinarySchemaKind::CellAction,
            OrdinaryInterpretation::CellAction {
                carrier: TypeExpr::parameter(0),
                operation: DerivationRef::parse(format!("blake3:{}", "d".repeat(64))).unwrap(),
                cell_dimension: 3,
                registered_boundary_bundle: DerivationRef::parse(format!(
                    "blake3:{}",
                    "e".repeat(64)
                ))
                .unwrap(),
            },
            FamilyPresentation::UniformInstance {
                parent_family: DerivationRef::parse(format!("blake3:{}", "f".repeat(64))).unwrap(),
            },
            SupportWindow::new(7, 8).unwrap(),
            anchor(),
        );
        assert!(matches!(
            bypass,
            Err(OrdinaryGrammarError::PresentationBypassesPendingQuotient {
                constructor: OrdinarySchemaKind::CellAction
            })
        ));
    }

    #[test]
    fn malformed_typing_support_and_references_are_rejected() {
        let malformed = form_ordinary_schema(
            context(),
            OrdinarySchemaKind::PointOrUnitIntro,
            OrdinaryInterpretation::PointOrUnitIntro {
                carrier: TypeExpr::trunc(TypeExpr::parameter(0)),
                point: TermExpr::variable(1),
            },
            FamilyPresentation::CanonicalFamily,
            SupportWindow::new(7, 8).unwrap(),
            anchor(),
        );
        assert!(matches!(
            malformed,
            Err(OrdinaryGrammarError::Context(
                SchemaContextError::TermTypeMismatch { .. }
            ))
        ));
        assert!(SupportWindow::new(6, 8).is_err());
        assert!(DerivationRef::parse("human-label").is_err());
        assert!(matches!(
            form_ordinary_schema(
                context(),
                OrdinarySchemaKind::PostPathOperation,
                OrdinaryInterpretation::PostPathOperation {
                    carrier: TypeExpr::parameter(0),
                    arity: 2,
                },
                FamilyPresentation::CanonicalFamily,
                SupportWindow::new(7, 8).unwrap(),
                vec![ClauseAnchor {
                    step: 8,
                    clause: 999,
                }],
            ),
            Err(OrdinaryGrammarError::ClauseAnchorOutOfBounds { .. })
        ));
        assert!(
            form_ordinary_schema(
                context(),
                OrdinarySchemaKind::PostPathOperation,
                OrdinaryInterpretation::PostPathOperation {
                    carrier: TypeExpr::parameter(0),
                    arity: 2,
                },
                FamilyPresentation::CanonicalFamily,
                SupportWindow::new(7, 8).unwrap(),
                vec![ClauseAnchor { step: 6, clause: 0 }],
            )
            .is_err()
        );
    }

    #[test]
    fn step8_registered_product_pi_path_cube_and_first_slot_cell_action_replay() {
        let bundle = DerivationRef::parse(format!("blake3:{}", "c".repeat(64))).unwrap();
        let token = issue_step8_registered_signatures(
            context(),
            TypeExpr::parameter(0),
            TermExpr::variable(1),
            bundle,
        )
        .unwrap();
        replay_step8_registered_signatures(&token).unwrap();
        assert!(token.operation_signature_typed());
        assert!(token.left_unit_signature_typed());
        assert!(token.cell_action_term_typed());
        assert!(token.cell_action_boundary_derived_by_map_cube());
        assert!(token.first_slot_orientation());
        assert!(!token.coherence_generator_membership_decided());
        assert!(!token.cell_action_generator_membership_decided());
        assert!(!token.external_bundle_archive_join_proved());
        assert!(
            matches!(token.left_unit_type(), SchemaTypeExpr::Pi { codomain, .. }
            if matches!(codomain.as_ref(), SchemaTypeExpr::Path { .. }))
        );
        assert!(
            matches!(token.cell_action_type(), SchemaTypeExpr::Pi { codomain, .. }
            if matches!(codomain.as_ref(), SchemaTypeExpr::Cube { dimension: 3, .. }))
        );
        assert!(
            matches!(token.cell_action_term(), SchemaTermExpr::Lam { body, .. }
            if matches!(body.as_ref(), SchemaTermExpr::MapCube { .. }))
        );

        let mut changed = token;
        changed.first_slot_orientation = false;
        assert_eq!(
            replay_step8_registered_signatures(&changed),
            Err(OrdinaryGrammarError::ReplayMismatch)
        );
    }
}
