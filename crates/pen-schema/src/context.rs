//! Operational E-1 schema contexts and dependent typed substitutions.
//!
//! Binders have stable identities rather than display names.  A context is
//! formed left-to-right, so every type, term, interval expression, and
//! cofibration can refer only to declarations in its prefix.  A substitution
//! is checked in the same order: the expected type of a later source element
//! is obtained by applying the already checked earlier images.  This is the
//! dependent typed-instance judgement absent from the legacy coarse
//! `Type`/`Opaque` variable-image audit.

use pen_type::cubical::{Cofibration as KernelCofibration, Endpoint};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const SCHEMA2_CONTEXT_FRAGMENT_VERSION: &str = "schema2-context-substitution-e1-v1";
pub const SCOPED_C1_THEOREM: &str =
    "dependent typed images over the complete current pen-schema E-1 TypeExpr/TermExpr inventory";
pub const PEN_CORE_TYPED_ELABORATION_GAP: &str =
    "C1_PEN_CORE_EXPR_TO_SCHEMA2_TYPED_EXPRESSION_BRIDGE";
pub const AGDA_RUST_CORRESPONDENCE_GAP: &str =
    "E1_AGDA_RUST_OPERATIONAL_CORRESPONDENCE_NOT_FORMALIZED";
pub const TYPE_EXPR_CONSTRUCTOR_COVERAGE: [&str; 3] = ["Parameter", "Trunc", "Path"];
pub const TERM_EXPR_CONSTRUCTOR_COVERAGE: [&str; 4] =
    ["Variable", "Library", "TruncPoint", "Reflexivity"];
pub const DIM_EXPR_CONSTRUCTOR_COVERAGE: [&str; 3] = ["Zero", "One", "Variable"];
pub const COFIBRATION_EXPR_CONSTRUCTOR_COVERAGE: [&str; 5] =
    ["False", "True", "Endpoint", "And", "Or"];
pub const DECLARATION_CONSTRUCTOR_COVERAGE: [&str; 5] = [
    "TypeParameter",
    "OpaqueElement",
    "LibraryReference",
    "IntervalVariable",
    "CofibrationAssumption",
];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BinderId(pub u32);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeExpr {
    Parameter {
        binder: BinderId,
    },
    Trunc {
        carrier: Box<TypeExpr>,
    },
    Path {
        carrier: Box<TypeExpr>,
        left: Box<TermExpr>,
        right: Box<TermExpr>,
    },
}

impl TypeExpr {
    pub fn parameter(binder: u32) -> Self {
        Self::Parameter {
            binder: BinderId(binder),
        }
    }

    pub fn trunc(carrier: TypeExpr) -> Self {
        Self::Trunc {
            carrier: Box::new(carrier),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TermExpr {
    Variable {
        binder: BinderId,
    },
    Library {
        step: u32,
        symbol: String,
    },
    TruncPoint {
        carrier: Box<TypeExpr>,
        point: Box<TermExpr>,
    },
    Reflexivity {
        carrier: Box<TypeExpr>,
        point: Box<TermExpr>,
    },
}

impl TermExpr {
    pub fn variable(binder: u32) -> Self {
        Self::Variable {
            binder: BinderId(binder),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", content = "binder", rename_all = "snake_case")]
pub enum DimExpr {
    Zero,
    One,
    Variable(BinderId),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CofibrationExpr {
    False,
    True,
    Endpoint { interval: BinderId, value: bool },
    And { terms: Vec<CofibrationExpr> },
    Or { terms: Vec<CofibrationExpr> },
}

impl CofibrationExpr {
    pub fn endpoint(interval: u32, value: bool) -> Self {
        Self::Endpoint {
            interval: BinderId(interval),
            value,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Declaration {
    TypeParameter {
        binder: BinderId,
        name: String,
        universe: u16,
    },
    OpaqueElement {
        binder: BinderId,
        name: String,
        ty: TypeExpr,
    },
    LibraryReference {
        binder: BinderId,
        name: String,
        step: u32,
        symbol: String,
        ty: TypeExpr,
    },
    IntervalVariable {
        binder: BinderId,
        name: String,
    },
    CofibrationAssumption {
        binder: BinderId,
        name: String,
        formula: CofibrationExpr,
    },
}

impl Declaration {
    pub const fn binder(&self) -> BinderId {
        match self {
            Self::TypeParameter { binder, .. }
            | Self::OpaqueElement { binder, .. }
            | Self::LibraryReference { binder, .. }
            | Self::IntervalVariable { binder, .. }
            | Self::CofibrationAssumption { binder, .. } => *binder,
        }
    }

    fn dependencies(&self) -> BTreeSet<BinderId> {
        let mut dependencies = BTreeSet::new();
        match self {
            Self::OpaqueElement { ty, .. } | Self::LibraryReference { ty, .. } => {
                collect_type_dependencies(ty, &mut dependencies)
            }
            Self::CofibrationAssumption { formula, .. } => {
                collect_cofibration_dependencies(formula, &mut dependencies)
            }
            Self::TypeParameter { .. } | Self::IntervalVariable { .. } => {}
        }
        dependencies
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FormedSchemaContext {
    declarations: Vec<Declaration>,
    derivation_hash: String,
}

impl FormedSchemaContext {
    pub fn declarations(&self) -> &[Declaration] {
        &self.declarations
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    pub fn declaration(&self, binder: BinderId) -> Option<&Declaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.binder() == binder)
    }

    pub fn interval_binders(&self) -> Vec<BinderId> {
        self.declarations
            .iter()
            .filter_map(|declaration| match declaration {
                Declaration::IntervalVariable { binder, .. } => Some(*binder),
                _ => None,
            })
            .collect()
    }

    fn library_declaration(&self, step: u32, symbol: &str) -> Option<&Declaration> {
        self.declarations.iter().find(|declaration| {
            matches!(
                declaration,
                Declaration::LibraryReference {
                    step: found_step,
                    symbol: found_symbol,
                    ..
                } if *found_step == step && found_symbol == symbol
            )
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SubstitutionImage {
    Type {
        source: BinderId,
        image: TypeExpr,
    },
    Term {
        source: BinderId,
        image: TermExpr,
    },
    RigidLibrary {
        source: BinderId,
        step: u32,
        symbol: String,
    },
    Dimension {
        source: BinderId,
        image: DimExpr,
    },
    Cofibration {
        source: BinderId,
        image: CofibrationExpr,
    },
}

impl SubstitutionImage {
    pub const fn source(&self) -> BinderId {
        match self {
            Self::Type { source, .. }
            | Self::Term { source, .. }
            | Self::RigidLibrary { source, .. }
            | Self::Dimension { source, .. }
            | Self::Cofibration { source, .. } => *source,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ImageMap {
    types: BTreeMap<BinderId, TypeExpr>,
    terms: BTreeMap<BinderId, TermExpr>,
    dimensions: BTreeMap<BinderId, DimExpr>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedSubstitutionToken {
    source: FormedSchemaContext,
    target: FormedSchemaContext,
    images: Vec<SubstitutionImage>,
    dependent_images_checked: usize,
    genuine_expression_images: usize,
    derivation_hash: String,
}

impl TypedSubstitutionToken {
    pub fn source(&self) -> &FormedSchemaContext {
        &self.source
    }

    pub fn target(&self) -> &FormedSchemaContext {
        &self.target
    }

    pub fn images(&self) -> &[SubstitutionImage] {
        &self.images
    }

    pub const fn dependent_images_checked(&self) -> usize {
        self.dependent_images_checked
    }

    pub const fn genuine_expression_images(&self) -> usize {
        self.genuine_expression_images
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }

    fn image_map(&self) -> ImageMap {
        image_map(&self.images)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypedExpression {
    Type { expression: TypeExpr },
    Term { expression: TermExpr, ty: TypeExpr },
    Dimension { expression: DimExpr },
    Cofibration { expression: CofibrationExpr },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LibraryKey {
    pub step: u32,
    pub symbol: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SchemaSupport {
    pub binders: Vec<BinderId>,
    pub libraries: Vec<LibraryKey>,
}

impl SchemaSupport {
    fn from_sets(binders: BTreeSet<BinderId>, libraries: BTreeSet<LibraryKey>) -> Self {
        Self {
            binders: binders.into_iter().collect(),
            libraries: libraries.into_iter().collect(),
        }
    }

    fn sets(&self) -> (BTreeSet<BinderId>, BTreeSet<LibraryKey>) {
        (
            self.binders.iter().copied().collect(),
            self.libraries.iter().cloned().collect(),
        )
    }

    pub fn is_subset_of(&self, other: &Self) -> bool {
        let (binders, libraries) = self.sets();
        let (other_binders, other_libraries) = other.sets();
        binders.is_subset(&other_binders) && libraries.is_subset(&other_libraries)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SubstitutionPreservationToken {
    substitution_derivation_hash: String,
    source_expression: TypedExpression,
    target_expression: TypedExpression,
    preserved: bool,
    derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SubstitutionSupportToken {
    substitution_derivation_hash: String,
    source_expression: TypedExpression,
    target_expression: TypedExpression,
    source_support: SchemaSupport,
    substituted_image_bound: SchemaSupport,
    target_support: SchemaSupport,
    target_support_within_image_bound: bool,
    derivation_hash: String,
}

impl SubstitutionSupportToken {
    pub fn source_support(&self) -> &SchemaSupport {
        &self.source_support
    }

    pub fn target_support(&self) -> &SchemaSupport {
        &self.target_support
    }

    pub fn substituted_image_bound(&self) -> &SchemaSupport {
        &self.substituted_image_bound
    }

    pub const fn target_support_within_image_bound(&self) -> bool {
        self.target_support_within_image_bound
    }
}

impl SubstitutionPreservationToken {
    pub fn target_expression(&self) -> &TypedExpression {
        &self.target_expression
    }

    pub const fn preserved(&self) -> bool {
        self.preserved
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompositionToken {
    first_derivation_hash: String,
    second_derivation_hash: String,
    composed: TypedSubstitutionToken,
    sequential_images_equal: bool,
    derivation_hash: String,
}

impl CompositionToken {
    pub fn composed(&self) -> &TypedSubstitutionToken {
        &self.composed
    }

    pub const fn sequential_images_equal(&self) -> bool {
        self.sequential_images_equal
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExchangeToken {
    before: FormedSchemaContext,
    after: FormedSchemaContext,
    left_index: usize,
    forward: TypedSubstitutionToken,
    backward: TypedSubstitutionToken,
    forward_then_backward: CompositionToken,
    backward_then_forward: CompositionToken,
    inverse_on_declarations: bool,
    derivation_hash: String,
}

impl ExchangeToken {
    pub fn after(&self) -> &FormedSchemaContext {
        &self.after
    }

    pub const fn inverse_on_declarations(&self) -> bool {
        self.inverse_on_declarations
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum SchemaContextError {
    #[error("binder {binder:?} is duplicated")]
    DuplicateBinder { binder: BinderId },
    #[error("declaration {binder:?} depends on unavailable binder {dependency:?}")]
    ForwardDependency {
        binder: BinderId,
        dependency: BinderId,
    },
    #[error("type variable {binder:?} is absent or is not a type parameter")]
    InvalidTypeVariable { binder: BinderId },
    #[error("term variable {binder:?} is absent or is not an element/library reference")]
    InvalidTermVariable { binder: BinderId },
    #[error("library reference {step}:{symbol} is absent")]
    MissingLibraryReference { step: u32, symbol: String },
    #[error("interval variable {binder:?} is absent or has the wrong declaration kind")]
    InvalidIntervalVariable { binder: BinderId },
    #[error("term type mismatch: expected {expected:?}, found {found:?}")]
    TermTypeMismatch { expected: TypeExpr, found: TypeExpr },
    #[error("substitution image inventory does not match the source context")]
    InvalidImageInventory,
    #[error("image kind for source binder {binder:?} does not match its declaration")]
    ImageKindMismatch { binder: BinderId },
    #[error("type image for {binder:?} lives in universe {found}, expected {expected}")]
    UniverseMismatch {
        binder: BinderId,
        expected: u16,
        found: u16,
    },
    #[error("rigid library image for {binder:?} does not match the source declaration")]
    RigidLibraryMismatch { binder: BinderId },
    #[error("target cofibration assumptions do not entail the image for {binder:?}")]
    CofibrationNotEntailed { binder: BinderId },
    #[error("substitution replay mismatch")]
    SubstitutionReplayMismatch,
    #[error("substitutions do not compose: first target differs from second source")]
    CompositionContextMismatch,
    #[error("composition is not extensionally equal on the source images")]
    CompositionMismatch,
    #[error("weakening target is not a proper prefix extension of the source")]
    InvalidWeakening,
    #[error("exchange index {left_index} has no adjacent declaration")]
    ExchangeOutOfRange { left_index: usize },
    #[error("declaration {right:?} depends on {left:?}, so the exchange is illegal")]
    DependentExchange { left: BinderId, right: BinderId },
    #[error("expression is not well typed in the source context")]
    IllTypedSourceExpression,
    #[error("substitution did not preserve the expression judgement")]
    PreservationMismatch,
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(SCHEMA2_CONTEXT_FRAGMENT_VERSION, domain, payload))
        .expect("schema context proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn collect_term_dependencies(term: &TermExpr, output: &mut BTreeSet<BinderId>) {
    match term {
        TermExpr::Variable { binder } => {
            output.insert(*binder);
        }
        TermExpr::Library { .. } => {}
        TermExpr::TruncPoint { carrier, point } | TermExpr::Reflexivity { carrier, point } => {
            collect_type_dependencies(carrier, output);
            collect_term_dependencies(point, output);
        }
    }
}

fn collect_type_dependencies(ty: &TypeExpr, output: &mut BTreeSet<BinderId>) {
    match ty {
        TypeExpr::Parameter { binder } => {
            output.insert(*binder);
        }
        TypeExpr::Trunc { carrier } => collect_type_dependencies(carrier, output),
        TypeExpr::Path {
            carrier,
            left,
            right,
        } => {
            collect_type_dependencies(carrier, output);
            collect_term_dependencies(left, output);
            collect_term_dependencies(right, output);
        }
    }
}

fn collect_cofibration_dependencies(formula: &CofibrationExpr, output: &mut BTreeSet<BinderId>) {
    match formula {
        CofibrationExpr::Endpoint { interval, .. } => {
            output.insert(*interval);
        }
        CofibrationExpr::And { terms } | CofibrationExpr::Or { terms } => {
            for term in terms {
                collect_cofibration_dependencies(term, output);
            }
        }
        CofibrationExpr::False | CofibrationExpr::True => {}
    }
}

fn collect_term_support(
    term: &TermExpr,
    binders: &mut BTreeSet<BinderId>,
    libraries: &mut BTreeSet<LibraryKey>,
) {
    match term {
        TermExpr::Variable { binder } => {
            binders.insert(*binder);
        }
        TermExpr::Library { step, symbol } => {
            libraries.insert(LibraryKey {
                step: *step,
                symbol: symbol.clone(),
            });
        }
        TermExpr::TruncPoint { carrier, point } | TermExpr::Reflexivity { carrier, point } => {
            collect_type_support(carrier, binders, libraries);
            collect_term_support(point, binders, libraries);
        }
    }
}

fn collect_type_support(
    ty: &TypeExpr,
    binders: &mut BTreeSet<BinderId>,
    libraries: &mut BTreeSet<LibraryKey>,
) {
    match ty {
        TypeExpr::Parameter { binder } => {
            binders.insert(*binder);
        }
        TypeExpr::Trunc { carrier } => collect_type_support(carrier, binders, libraries),
        TypeExpr::Path {
            carrier,
            left,
            right,
        } => {
            collect_type_support(carrier, binders, libraries);
            collect_term_support(left, binders, libraries);
            collect_term_support(right, binders, libraries);
        }
    }
}

fn collect_dim_support(dimension: DimExpr, binders: &mut BTreeSet<BinderId>) {
    if let DimExpr::Variable(binder) = dimension {
        binders.insert(binder);
    }
}

fn collect_cofibration_support(formula: &CofibrationExpr, binders: &mut BTreeSet<BinderId>) {
    match formula {
        CofibrationExpr::Endpoint { interval, .. } => {
            binders.insert(*interval);
        }
        CofibrationExpr::And { terms } | CofibrationExpr::Or { terms } => {
            for term in terms {
                collect_cofibration_support(term, binders);
            }
        }
        CofibrationExpr::False | CofibrationExpr::True => {}
    }
}

pub fn expression_support(expression: &TypedExpression) -> SchemaSupport {
    let mut binders = BTreeSet::new();
    let mut libraries = BTreeSet::new();
    match expression {
        TypedExpression::Type { expression } => {
            collect_type_support(expression, &mut binders, &mut libraries)
        }
        TypedExpression::Term { expression, ty } => {
            collect_term_support(expression, &mut binders, &mut libraries);
            collect_type_support(ty, &mut binders, &mut libraries);
        }
        TypedExpression::Dimension { expression } => collect_dim_support(*expression, &mut binders),
        TypedExpression::Cofibration { expression } => {
            collect_cofibration_support(expression, &mut binders)
        }
    }
    SchemaSupport::from_sets(binders, libraries)
}

fn image_support(image: &SubstitutionImage) -> SchemaSupport {
    let mut binders = BTreeSet::new();
    let mut libraries = BTreeSet::new();
    match image {
        SubstitutionImage::Type { image, .. } => {
            collect_type_support(image, &mut binders, &mut libraries)
        }
        SubstitutionImage::Term { image, .. } => {
            collect_term_support(image, &mut binders, &mut libraries)
        }
        SubstitutionImage::RigidLibrary { step, symbol, .. } => {
            libraries.insert(LibraryKey {
                step: *step,
                symbol: symbol.clone(),
            });
        }
        SubstitutionImage::Dimension { image, .. } => collect_dim_support(*image, &mut binders),
        SubstitutionImage::Cofibration { image, .. } => {
            collect_cofibration_support(image, &mut binders)
        }
    }
    SchemaSupport::from_sets(binders, libraries)
}

fn check_type(context: &FormedSchemaContext, ty: &TypeExpr) -> Result<u16, SchemaContextError> {
    match ty {
        TypeExpr::Parameter { binder } => match context.declaration(*binder) {
            Some(Declaration::TypeParameter { universe, .. }) => Ok(*universe),
            _ => Err(SchemaContextError::InvalidTypeVariable { binder: *binder }),
        },
        TypeExpr::Trunc { carrier } => check_type(context, carrier),
        TypeExpr::Path {
            carrier,
            left,
            right,
        } => {
            let universe = check_type(context, carrier)?;
            let left_type = infer_term(context, left)?;
            let right_type = infer_term(context, right)?;
            if left_type != **carrier {
                return Err(SchemaContextError::TermTypeMismatch {
                    expected: (**carrier).clone(),
                    found: left_type,
                });
            }
            if right_type != **carrier {
                return Err(SchemaContextError::TermTypeMismatch {
                    expected: (**carrier).clone(),
                    found: right_type,
                });
            }
            Ok(universe)
        }
    }
}

pub fn infer_term(
    context: &FormedSchemaContext,
    term: &TermExpr,
) -> Result<TypeExpr, SchemaContextError> {
    match term {
        TermExpr::Variable { binder } => match context.declaration(*binder) {
            Some(Declaration::OpaqueElement { ty, .. }) => Ok(ty.clone()),
            _ => Err(SchemaContextError::InvalidTermVariable { binder: *binder }),
        },
        TermExpr::Library { step, symbol } => {
            let Some(Declaration::LibraryReference { ty, .. }) =
                context.library_declaration(*step, symbol)
            else {
                return Err(SchemaContextError::MissingLibraryReference {
                    step: *step,
                    symbol: symbol.clone(),
                });
            };
            Ok(ty.clone())
        }
        TermExpr::TruncPoint { carrier, point } => {
            check_type(context, carrier)?;
            let found = infer_term(context, point)?;
            if found != **carrier {
                return Err(SchemaContextError::TermTypeMismatch {
                    expected: (**carrier).clone(),
                    found,
                });
            }
            Ok(TypeExpr::trunc((**carrier).clone()))
        }
        TermExpr::Reflexivity { carrier, point } => {
            check_type(context, carrier)?;
            let found = infer_term(context, point)?;
            if found != **carrier {
                return Err(SchemaContextError::TermTypeMismatch {
                    expected: (**carrier).clone(),
                    found,
                });
            }
            Ok(TypeExpr::Path {
                carrier: carrier.clone(),
                left: point.clone(),
                right: point.clone(),
            })
        }
    }
}

fn check_dim(context: &FormedSchemaContext, dimension: DimExpr) -> Result<(), SchemaContextError> {
    match dimension {
        DimExpr::Zero | DimExpr::One => Ok(()),
        DimExpr::Variable(binder) => match context.declaration(binder) {
            Some(Declaration::IntervalVariable { .. }) => Ok(()),
            _ => Err(SchemaContextError::InvalidIntervalVariable { binder }),
        },
    }
}

fn check_cofibration(
    context: &FormedSchemaContext,
    formula: &CofibrationExpr,
) -> Result<(), SchemaContextError> {
    match formula {
        CofibrationExpr::False | CofibrationExpr::True => Ok(()),
        CofibrationExpr::Endpoint { interval, .. } => {
            check_dim(context, DimExpr::Variable(*interval))
        }
        CofibrationExpr::And { terms } | CofibrationExpr::Or { terms } => {
            for term in terms {
                check_cofibration(context, term)?;
            }
            Ok(())
        }
    }
}

pub fn form_schema_context(
    declarations: Vec<Declaration>,
) -> Result<FormedSchemaContext, SchemaContextError> {
    let mut prefix = FormedSchemaContext {
        declarations: Vec::new(),
        derivation_hash: tagged_hash("empty-context", &()),
    };
    let mut seen_binders = BTreeSet::new();
    let mut seen_libraries = BTreeSet::new();
    for declaration in declarations {
        let binder = declaration.binder();
        if !seen_binders.insert(binder) {
            return Err(SchemaContextError::DuplicateBinder { binder });
        }
        for dependency in declaration.dependencies() {
            if prefix.declaration(dependency).is_none() {
                return Err(SchemaContextError::ForwardDependency { binder, dependency });
            }
        }
        match &declaration {
            Declaration::OpaqueElement { ty, .. } => {
                check_type(&prefix, ty)?;
            }
            Declaration::LibraryReference {
                step, symbol, ty, ..
            } => {
                check_type(&prefix, ty)?;
                if !seen_libraries.insert((*step, symbol.clone())) {
                    return Err(SchemaContextError::RigidLibraryMismatch { binder });
                }
            }
            Declaration::CofibrationAssumption { formula, .. } => {
                check_cofibration(&prefix, formula)?;
            }
            Declaration::TypeParameter { .. } | Declaration::IntervalVariable { .. } => {}
        }
        prefix.declarations.push(declaration);
        prefix.derivation_hash = tagged_hash("formed-context-prefix", &prefix.declarations);
    }
    Ok(prefix)
}

pub fn replay_formed_context(context: &FormedSchemaContext) -> Result<(), SchemaContextError> {
    let replay = form_schema_context(context.declarations.clone())?;
    if replay == *context {
        Ok(())
    } else {
        Err(SchemaContextError::SubstitutionReplayMismatch)
    }
}

fn image_map(images: &[SubstitutionImage]) -> ImageMap {
    let mut map = ImageMap::default();
    for image in images {
        match image {
            SubstitutionImage::Type { source, image } => {
                map.types.insert(*source, image.clone());
            }
            SubstitutionImage::Term { source, image } => {
                map.terms.insert(*source, image.clone());
            }
            SubstitutionImage::Dimension { source, image } => {
                map.dimensions.insert(*source, *image);
            }
            SubstitutionImage::RigidLibrary { .. } | SubstitutionImage::Cofibration { .. } => {}
        }
    }
    map
}

fn substitute_type(ty: &TypeExpr, map: &ImageMap) -> TypeExpr {
    match ty {
        TypeExpr::Parameter { binder } => {
            map.types.get(binder).cloned().unwrap_or_else(|| ty.clone())
        }
        TypeExpr::Trunc { carrier } => TypeExpr::trunc(substitute_type(carrier, map)),
        TypeExpr::Path {
            carrier,
            left,
            right,
        } => TypeExpr::Path {
            carrier: Box::new(substitute_type(carrier, map)),
            left: Box::new(substitute_term(left, map)),
            right: Box::new(substitute_term(right, map)),
        },
    }
}

fn substitute_term(term: &TermExpr, map: &ImageMap) -> TermExpr {
    match term {
        TermExpr::Variable { binder } => map
            .terms
            .get(binder)
            .cloned()
            .unwrap_or_else(|| term.clone()),
        TermExpr::Library { .. } => term.clone(),
        TermExpr::TruncPoint { carrier, point } => TermExpr::TruncPoint {
            carrier: Box::new(substitute_type(carrier, map)),
            point: Box::new(substitute_term(point, map)),
        },
        TermExpr::Reflexivity { carrier, point } => TermExpr::Reflexivity {
            carrier: Box::new(substitute_type(carrier, map)),
            point: Box::new(substitute_term(point, map)),
        },
    }
}

fn substitute_dim(dimension: DimExpr, map: &ImageMap) -> DimExpr {
    match dimension {
        DimExpr::Variable(binder) => map.dimensions.get(&binder).copied().unwrap_or(dimension),
        DimExpr::Zero | DimExpr::One => dimension,
    }
}

fn substitute_cofibration(formula: &CofibrationExpr, map: &ImageMap) -> CofibrationExpr {
    match formula {
        CofibrationExpr::False | CofibrationExpr::True => formula.clone(),
        CofibrationExpr::Endpoint { interval, value } => {
            match substitute_dim(DimExpr::Variable(*interval), map) {
                DimExpr::Zero => {
                    if !value {
                        CofibrationExpr::True
                    } else {
                        CofibrationExpr::False
                    }
                }
                DimExpr::One => {
                    if *value {
                        CofibrationExpr::True
                    } else {
                        CofibrationExpr::False
                    }
                }
                DimExpr::Variable(interval) => CofibrationExpr::Endpoint {
                    interval,
                    value: *value,
                },
            }
        }
        CofibrationExpr::And { terms } => CofibrationExpr::And {
            terms: terms
                .iter()
                .map(|term| substitute_cofibration(term, map))
                .collect(),
        },
        CofibrationExpr::Or { terms } => CofibrationExpr::Or {
            terms: terms
                .iter()
                .map(|term| substitute_cofibration(term, map))
                .collect(),
        },
    }
}

fn compile_cofibration(
    context: &FormedSchemaContext,
    formula: &CofibrationExpr,
) -> Result<KernelCofibration, SchemaContextError> {
    let intervals = context.interval_binders();
    let positions = intervals
        .iter()
        .enumerate()
        .map(|(index, binder)| (*binder, index as u16))
        .collect::<BTreeMap<_, _>>();
    fn compile(
        formula: &CofibrationExpr,
        positions: &BTreeMap<BinderId, u16>,
    ) -> Result<KernelCofibration, SchemaContextError> {
        Ok(match formula {
            CofibrationExpr::False => KernelCofibration::false_formula(),
            CofibrationExpr::True => KernelCofibration::true_formula(),
            CofibrationExpr::Endpoint { interval, value } => {
                let Some(variable) = positions.get(interval).copied() else {
                    return Err(SchemaContextError::InvalidIntervalVariable { binder: *interval });
                };
                KernelCofibration::endpoint(Endpoint {
                    variable,
                    value: *value,
                })
            }
            CofibrationExpr::And { terms } => terms
                .iter()
                .try_fold(KernelCofibration::true_formula(), |accumulator, term| {
                    Ok(accumulator.meet(&compile(term, positions)?))
                })?,
            CofibrationExpr::Or { terms } => terms
                .iter()
                .try_fold(KernelCofibration::false_formula(), |accumulator, term| {
                    Ok(accumulator.join(&compile(term, positions)?))
                })?,
        })
    }
    compile(formula, &positions)
}

fn context_assumption(
    context: &FormedSchemaContext,
) -> Result<KernelCofibration, SchemaContextError> {
    context.declarations.iter().try_fold(
        KernelCofibration::true_formula(),
        |assumption, declaration| match declaration {
            Declaration::CofibrationAssumption { formula, .. } => {
                Ok(assumption.meet(&compile_cofibration(context, formula)?))
            }
            _ => Ok(assumption),
        },
    )
}

fn identity_images(context: &FormedSchemaContext) -> Vec<SubstitutionImage> {
    context
        .declarations
        .iter()
        .map(|declaration| match declaration {
            Declaration::TypeParameter { binder, .. } => SubstitutionImage::Type {
                source: *binder,
                image: TypeExpr::Parameter { binder: *binder },
            },
            Declaration::OpaqueElement { binder, .. } => SubstitutionImage::Term {
                source: *binder,
                image: TermExpr::Variable { binder: *binder },
            },
            Declaration::LibraryReference {
                binder,
                step,
                symbol,
                ..
            } => SubstitutionImage::RigidLibrary {
                source: *binder,
                step: *step,
                symbol: symbol.clone(),
            },
            Declaration::IntervalVariable { binder, .. } => SubstitutionImage::Dimension {
                source: *binder,
                image: DimExpr::Variable(*binder),
            },
            Declaration::CofibrationAssumption {
                binder, formula, ..
            } => SubstitutionImage::Cofibration {
                source: *binder,
                image: formula.clone(),
            },
        })
        .collect()
}

pub fn issue_typed_substitution(
    source: &FormedSchemaContext,
    target: &FormedSchemaContext,
    images: Vec<SubstitutionImage>,
) -> Result<TypedSubstitutionToken, SchemaContextError> {
    replay_formed_context(source)?;
    replay_formed_context(target)?;
    if images.len() != source.declarations.len()
        || images
            .iter()
            .map(SubstitutionImage::source)
            .collect::<BTreeSet<_>>()
            != source
                .declarations
                .iter()
                .map(Declaration::binder)
                .collect()
    {
        return Err(SchemaContextError::InvalidImageInventory);
    }
    let supplied = images
        .into_iter()
        .map(|image| (image.source(), image))
        .collect::<BTreeMap<_, _>>();
    let mut checked_images = Vec::with_capacity(source.declarations.len());
    let mut partial = ImageMap::default();
    let mut dependent_images_checked = 0usize;
    let mut genuine_expression_images = 0usize;
    let target_assumption = context_assumption(target)?;
    for declaration in &source.declarations {
        let binder = declaration.binder();
        let image = supplied
            .get(&binder)
            .expect("complete inventory checked above")
            .clone();
        match (declaration, &image) {
            (
                Declaration::TypeParameter { universe, .. },
                SubstitutionImage::Type { image, .. },
            ) => {
                let found = check_type(target, image)?;
                if found != *universe {
                    return Err(SchemaContextError::UniverseMismatch {
                        binder,
                        expected: *universe,
                        found,
                    });
                }
                if !matches!(image, TypeExpr::Parameter { .. }) {
                    genuine_expression_images += 1;
                }
                partial.types.insert(binder, image.clone());
            }
            (Declaration::OpaqueElement { ty, .. }, SubstitutionImage::Term { image, .. }) => {
                let expected = substitute_type(ty, &partial);
                let found = infer_term(target, image)?;
                if found != expected {
                    return Err(SchemaContextError::TermTypeMismatch { expected, found });
                }
                if !matches!(image, TermExpr::Variable { .. }) {
                    genuine_expression_images += 1;
                }
                if !declaration.dependencies().is_empty() {
                    dependent_images_checked += 1;
                }
                partial.terms.insert(binder, image.clone());
            }
            (
                Declaration::LibraryReference {
                    step, symbol, ty, ..
                },
                SubstitutionImage::RigidLibrary {
                    step: image_step,
                    symbol: image_symbol,
                    ..
                },
            ) => {
                let expected = substitute_type(ty, &partial);
                let Some(Declaration::LibraryReference { ty: target_ty, .. }) =
                    target.library_declaration(*image_step, image_symbol)
                else {
                    return Err(SchemaContextError::RigidLibraryMismatch { binder });
                };
                if image_step != step || image_symbol != symbol || target_ty != &expected {
                    return Err(SchemaContextError::RigidLibraryMismatch { binder });
                }
                if !declaration.dependencies().is_empty() {
                    dependent_images_checked += 1;
                }
            }
            (Declaration::IntervalVariable { .. }, SubstitutionImage::Dimension { image, .. }) => {
                check_dim(target, *image)?;
                partial.dimensions.insert(binder, *image);
            }
            (
                Declaration::CofibrationAssumption { formula, .. },
                SubstitutionImage::Cofibration { image, .. },
            ) => {
                check_cofibration(target, image)?;
                let expected =
                    compile_cofibration(target, &substitute_cofibration(formula, &partial))?;
                let found = compile_cofibration(target, image)?;
                if expected != found || !target_assumption.implies(&found) {
                    return Err(SchemaContextError::CofibrationNotEntailed { binder });
                }
                if !declaration.dependencies().is_empty() {
                    dependent_images_checked += 1;
                }
            }
            _ => return Err(SchemaContextError::ImageKindMismatch { binder }),
        }
        checked_images.push(image);
    }
    let derivation_hash = tagged_hash(
        "dependent-typed-instance-substitution",
        &(
            source.derivation_hash(),
            target.derivation_hash(),
            &checked_images,
            dependent_images_checked,
            genuine_expression_images,
        ),
    );
    Ok(TypedSubstitutionToken {
        source: source.clone(),
        target: target.clone(),
        images: checked_images,
        dependent_images_checked,
        genuine_expression_images,
        derivation_hash,
    })
}

pub fn replay_typed_substitution(token: &TypedSubstitutionToken) -> Result<(), SchemaContextError> {
    let replay = issue_typed_substitution(&token.source, &token.target, token.images.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(SchemaContextError::SubstitutionReplayMismatch)
    }
}

pub fn identity_substitution(
    context: &FormedSchemaContext,
) -> Result<TypedSubstitutionToken, SchemaContextError> {
    issue_typed_substitution(context, context, identity_images(context))
}

pub fn weakening_substitution(
    source: &FormedSchemaContext,
    target: &FormedSchemaContext,
) -> Result<TypedSubstitutionToken, SchemaContextError> {
    if target.declarations.len() <= source.declarations.len()
        || target.declarations[..source.declarations.len()] != source.declarations
    {
        return Err(SchemaContextError::InvalidWeakening);
    }
    issue_typed_substitution(source, target, identity_images(source))
}

pub fn check_typed_expression(
    context: &FormedSchemaContext,
    expression: &TypedExpression,
) -> Result<(), SchemaContextError> {
    match expression {
        TypedExpression::Type { expression } => {
            check_type(context, expression)?;
        }
        TypedExpression::Term { expression, ty } => {
            check_type(context, ty)?;
            let found = infer_term(context, expression)?;
            if found != *ty {
                return Err(SchemaContextError::TermTypeMismatch {
                    expected: ty.clone(),
                    found,
                });
            }
        }
        TypedExpression::Dimension { expression } => check_dim(context, *expression)?,
        TypedExpression::Cofibration { expression } => check_cofibration(context, expression)?,
    }
    Ok(())
}

fn substitute_expression(expression: &TypedExpression, map: &ImageMap) -> TypedExpression {
    match expression {
        TypedExpression::Type { expression } => TypedExpression::Type {
            expression: substitute_type(expression, map),
        },
        TypedExpression::Term { expression, ty } => TypedExpression::Term {
            expression: substitute_term(expression, map),
            ty: substitute_type(ty, map),
        },
        TypedExpression::Dimension { expression } => TypedExpression::Dimension {
            expression: substitute_dim(*expression, map),
        },
        TypedExpression::Cofibration { expression } => TypedExpression::Cofibration {
            expression: substitute_cofibration(expression, map),
        },
    }
}

pub fn issue_substitution_preservation(
    substitution: &TypedSubstitutionToken,
    source_expression: TypedExpression,
) -> Result<SubstitutionPreservationToken, SchemaContextError> {
    replay_typed_substitution(substitution)?;
    check_typed_expression(&substitution.source, &source_expression)
        .map_err(|_| SchemaContextError::IllTypedSourceExpression)?;
    let target_expression = substitute_expression(&source_expression, &substitution.image_map());
    check_typed_expression(&substitution.target, &target_expression)
        .map_err(|_| SchemaContextError::PreservationMismatch)?;
    let preserved = true;
    let derivation_hash = tagged_hash(
        "substitution-preserves-typing",
        &(
            substitution.derivation_hash(),
            &source_expression,
            &target_expression,
            preserved,
        ),
    );
    Ok(SubstitutionPreservationToken {
        substitution_derivation_hash: substitution.derivation_hash().to_owned(),
        source_expression,
        target_expression,
        preserved,
        derivation_hash,
    })
}

pub fn replay_substitution_preservation(
    substitution: &TypedSubstitutionToken,
    token: &SubstitutionPreservationToken,
) -> Result<(), SchemaContextError> {
    let replay = issue_substitution_preservation(substitution, token.source_expression.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(SchemaContextError::PreservationMismatch)
    }
}

pub fn issue_substitution_support(
    substitution: &TypedSubstitutionToken,
    source_expression: TypedExpression,
) -> Result<SubstitutionSupportToken, SchemaContextError> {
    let preservation = issue_substitution_preservation(substitution, source_expression.clone())?;
    let target_expression = preservation.target_expression.clone();
    let source_support = expression_support(&source_expression);
    let target_support = expression_support(&target_expression);
    let images = substitution
        .images
        .iter()
        .map(|image| (image.source(), image))
        .collect::<BTreeMap<_, _>>();
    let (source_binders, source_libraries) = source_support.sets();
    let mut bound_binders = BTreeSet::new();
    let mut bound_libraries = source_libraries;
    for binder in source_binders {
        let image = images
            .get(&binder)
            .expect("well-typed source support belongs to the source context");
        let (image_binders, image_libraries) = image_support(image).sets();
        bound_binders.extend(image_binders);
        bound_libraries.extend(image_libraries);
    }
    let substituted_image_bound = SchemaSupport::from_sets(bound_binders, bound_libraries);
    let target_support_within_image_bound = target_support.is_subset_of(&substituted_image_bound);
    if !target_support_within_image_bound {
        return Err(SchemaContextError::PreservationMismatch);
    }
    let derivation_hash = tagged_hash(
        "substitution-support-behavior",
        &(
            substitution.derivation_hash(),
            &source_expression,
            &target_expression,
            &source_support,
            &substituted_image_bound,
            &target_support,
            target_support_within_image_bound,
        ),
    );
    Ok(SubstitutionSupportToken {
        substitution_derivation_hash: substitution.derivation_hash().to_owned(),
        source_expression,
        target_expression,
        source_support,
        substituted_image_bound,
        target_support,
        target_support_within_image_bound,
        derivation_hash,
    })
}

pub fn replay_substitution_support(
    substitution: &TypedSubstitutionToken,
    token: &SubstitutionSupportToken,
) -> Result<(), SchemaContextError> {
    let replay = issue_substitution_support(substitution, token.source_expression.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(SchemaContextError::PreservationMismatch)
    }
}

fn compose_image(image: &SubstitutionImage, second: &ImageMap) -> SubstitutionImage {
    match image {
        SubstitutionImage::Type { source, image } => SubstitutionImage::Type {
            source: *source,
            image: substitute_type(image, second),
        },
        SubstitutionImage::Term { source, image } => SubstitutionImage::Term {
            source: *source,
            image: substitute_term(image, second),
        },
        SubstitutionImage::RigidLibrary {
            source,
            step,
            symbol,
        } => SubstitutionImage::RigidLibrary {
            source: *source,
            step: *step,
            symbol: symbol.clone(),
        },
        SubstitutionImage::Dimension { source, image } => SubstitutionImage::Dimension {
            source: *source,
            image: substitute_dim(*image, second),
        },
        SubstitutionImage::Cofibration { source, image } => SubstitutionImage::Cofibration {
            source: *source,
            image: substitute_cofibration(image, second),
        },
    }
}

pub fn compose_typed_substitutions(
    first: &TypedSubstitutionToken,
    second: &TypedSubstitutionToken,
) -> Result<CompositionToken, SchemaContextError> {
    replay_typed_substitution(first)?;
    replay_typed_substitution(second)?;
    if first.target != second.source {
        return Err(SchemaContextError::CompositionContextMismatch);
    }
    let second_map = second.image_map();
    let images = first
        .images
        .iter()
        .map(|image| compose_image(image, &second_map))
        .collect::<Vec<_>>();
    let composed = issue_typed_substitution(&first.source, &second.target, images)?;
    let sequential_images_equal = first
        .images
        .iter()
        .zip(&composed.images)
        .all(|(image, composed_image)| compose_image(image, &second_map) == *composed_image);
    if !sequential_images_equal {
        return Err(SchemaContextError::CompositionMismatch);
    }
    let derivation_hash = tagged_hash(
        "typed-substitution-composition",
        &(
            first.derivation_hash(),
            second.derivation_hash(),
            &composed,
            sequential_images_equal,
        ),
    );
    Ok(CompositionToken {
        first_derivation_hash: first.derivation_hash().to_owned(),
        second_derivation_hash: second.derivation_hash().to_owned(),
        composed,
        sequential_images_equal,
        derivation_hash,
    })
}

pub fn replay_typed_substitution_composition(
    first: &TypedSubstitutionToken,
    second: &TypedSubstitutionToken,
    token: &CompositionToken,
) -> Result<(), SchemaContextError> {
    let replay = compose_typed_substitutions(first, second)?;
    if replay == *token {
        Ok(())
    } else {
        Err(SchemaContextError::CompositionMismatch)
    }
}

pub fn exchange_adjacent(
    context: &FormedSchemaContext,
    left_index: usize,
) -> Result<ExchangeToken, SchemaContextError> {
    let Some(right_index) = left_index.checked_add(1) else {
        return Err(SchemaContextError::ExchangeOutOfRange { left_index });
    };
    if right_index >= context.declarations.len() {
        return Err(SchemaContextError::ExchangeOutOfRange { left_index });
    }
    let left = context.declarations[left_index].binder();
    let right = context.declarations[right_index].binder();
    if context.declarations[right_index]
        .dependencies()
        .contains(&left)
    {
        return Err(SchemaContextError::DependentExchange { left, right });
    }
    let mut declarations = context.declarations.clone();
    declarations.swap(left_index, right_index);
    let after = form_schema_context(declarations)?;
    let forward = issue_typed_substitution(context, &after, identity_images(context))?;
    let backward = issue_typed_substitution(&after, context, identity_images(&after))?;
    let forward_then_backward = compose_typed_substitutions(&forward, &backward)?;
    let backward_then_forward = compose_typed_substitutions(&backward, &forward)?;
    let inverse_on_declarations = forward_then_backward.composed == identity_substitution(context)?
        && backward_then_forward.composed == identity_substitution(&after)?;
    let derivation_hash = tagged_hash(
        "legal-adjacent-exchange",
        &(
            context,
            &after,
            left_index,
            &forward,
            &backward,
            &forward_then_backward,
            &backward_then_forward,
            inverse_on_declarations,
        ),
    );
    Ok(ExchangeToken {
        before: context.clone(),
        after,
        left_index,
        forward,
        backward,
        forward_then_backward,
        backward_then_forward,
        inverse_on_declarations,
        derivation_hash,
    })
}

pub fn replay_adjacent_exchange(
    context: &FormedSchemaContext,
    token: &ExchangeToken,
) -> Result<(), SchemaContextError> {
    let replay = exchange_adjacent(context, token.left_index)?;
    if replay == *token {
        Ok(())
    } else {
        Err(SchemaContextError::CompositionMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn five_kind_context() -> FormedSchemaContext {
        let a = TypeExpr::parameter(1);
        form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(1),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(2),
                name: "x".to_owned(),
                ty: a.clone(),
            },
            Declaration::LibraryReference {
                binder: BinderId(3),
                name: "base".to_owned(),
                step: 1,
                symbol: "base".to_owned(),
                ty: a,
            },
            Declaration::IntervalVariable {
                binder: BinderId(4),
                name: "i".to_owned(),
            },
            Declaration::CofibrationAssumption {
                binder: BinderId(5),
                name: "phi".to_owned(),
                formula: CofibrationExpr::endpoint(4, false),
            },
        ])
        .expect("all five context entry classes form")
    }

    fn point_contexts() -> (FormedSchemaContext, FormedSchemaContext) {
        let source = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(1),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(2),
                name: "x".to_owned(),
                ty: TypeExpr::trunc(TypeExpr::parameter(1)),
            },
        ])
        .unwrap();
        let target = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(10),
                name: "B".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(11),
                name: "b".to_owned(),
                ty: TypeExpr::parameter(10),
            },
        ])
        .unwrap();
        (source, target)
    }

    #[test]
    fn formation_lookup_identity_and_preservation_cover_all_entry_classes() {
        let context = five_kind_context();
        replay_formed_context(&context).unwrap();
        let identity = identity_substitution(&context).unwrap();
        replay_typed_substitution(&identity).unwrap();
        assert_eq!(identity.dependent_images_checked(), 3);
        assert_eq!(identity.genuine_expression_images(), 0);
        for expression in [
            TypedExpression::Type {
                expression: TypeExpr::parameter(1),
            },
            TypedExpression::Term {
                expression: TermExpr::variable(2),
                ty: TypeExpr::parameter(1),
            },
            TypedExpression::Term {
                expression: TermExpr::Library {
                    step: 1,
                    symbol: "base".to_owned(),
                },
                ty: TypeExpr::parameter(1),
            },
            TypedExpression::Dimension {
                expression: DimExpr::Variable(BinderId(4)),
            },
            TypedExpression::Cofibration {
                expression: CofibrationExpr::endpoint(4, false),
            },
        ] {
            let token = issue_substitution_preservation(&identity, expression.clone()).unwrap();
            assert!(token.preserved());
            replay_substitution_preservation(&identity, &token).unwrap();
            let support = issue_substitution_support(&identity, expression).unwrap();
            assert!(support.target_support_within_image_bound());
            assert_eq!(support.target_support(), support.source_support());
            replay_substitution_support(&identity, &support).unwrap();
        }
    }

    #[test]
    fn genuine_dependent_expression_image_extends_the_restricted_c1_fragment() {
        let (source, target) = point_contexts();
        let substitution = issue_typed_substitution(
            &source,
            &target,
            vec![
                SubstitutionImage::Type {
                    source: BinderId(1),
                    image: TypeExpr::parameter(10),
                },
                SubstitutionImage::Term {
                    source: BinderId(2),
                    image: TermExpr::TruncPoint {
                        carrier: Box::new(TypeExpr::parameter(10)),
                        point: Box::new(TermExpr::variable(11)),
                    },
                },
            ],
        )
        .expect("a typed non-variable point image is admitted");
        assert_eq!(substitution.dependent_images_checked(), 1);
        assert_eq!(substitution.genuine_expression_images(), 1);
        let source_expression = TypedExpression::Term {
            expression: TermExpr::variable(2),
            ty: TypeExpr::trunc(TypeExpr::parameter(1)),
        };
        let preserved =
            issue_substitution_preservation(&substitution, source_expression.clone()).unwrap();
        assert!(matches!(
            preserved.target_expression(),
            TypedExpression::Term {
                expression: TermExpr::TruncPoint { .. },
                ty: TypeExpr::Trunc { .. },
            }
        ));
        let support = issue_substitution_support(&substitution, source_expression).unwrap();
        assert!(support.target_support_within_image_bound());
        assert_eq!(support.target_support(), support.substituted_image_bound());
        replay_substitution_support(&substitution, &support).unwrap();
        let mut mutated_support = support;
        mutated_support.target_support_within_image_bound = false;
        assert_eq!(
            replay_substitution_support(&substitution, &mutated_support),
            Err(SchemaContextError::PreservationMismatch)
        );

        let wrong = issue_typed_substitution(
            &source,
            &target,
            vec![
                SubstitutionImage::Type {
                    source: BinderId(1),
                    image: TypeExpr::parameter(10),
                },
                SubstitutionImage::Term {
                    source: BinderId(2),
                    image: TermExpr::variable(11),
                },
            ],
        );
        assert!(matches!(
            wrong,
            Err(SchemaContextError::TermTypeMismatch { .. })
        ));
    }

    #[test]
    fn composition_is_sequential_on_genuine_dependent_images() {
        let (source, middle) = point_contexts();
        let target = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(20),
                name: "C".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(21),
                name: "c".to_owned(),
                ty: TypeExpr::parameter(20),
            },
        ])
        .unwrap();
        let first = issue_typed_substitution(
            &source,
            &middle,
            vec![
                SubstitutionImage::Type {
                    source: BinderId(1),
                    image: TypeExpr::parameter(10),
                },
                SubstitutionImage::Term {
                    source: BinderId(2),
                    image: TermExpr::TruncPoint {
                        carrier: Box::new(TypeExpr::parameter(10)),
                        point: Box::new(TermExpr::variable(11)),
                    },
                },
            ],
        )
        .unwrap();
        let second = issue_typed_substitution(
            &middle,
            &target,
            vec![
                SubstitutionImage::Type {
                    source: BinderId(10),
                    image: TypeExpr::parameter(20),
                },
                SubstitutionImage::Term {
                    source: BinderId(11),
                    image: TermExpr::variable(21),
                },
            ],
        )
        .unwrap();
        let composition = compose_typed_substitutions(&first, &second).unwrap();
        assert!(composition.sequential_images_equal());
        assert_eq!(composition.composed().genuine_expression_images(), 1);
        replay_typed_substitution_composition(&first, &second, &composition).unwrap();
        let mut mutated_composition = composition.clone();
        mutated_composition.sequential_images_equal = false;
        assert_eq!(
            replay_typed_substitution_composition(&first, &second, &mutated_composition),
            Err(SchemaContextError::CompositionMismatch)
        );
        let preservation = issue_substitution_preservation(
            composition.composed(),
            TypedExpression::Term {
                expression: TermExpr::variable(2),
                ty: TypeExpr::trunc(TypeExpr::parameter(1)),
            },
        )
        .unwrap();
        assert!(preservation.preserved());
    }

    #[test]
    fn weakening_and_only_independent_exchange_are_issued() {
        let source = form_schema_context(vec![Declaration::TypeParameter {
            binder: BinderId(1),
            name: "A".to_owned(),
            universe: 0,
        }])
        .unwrap();
        let target = form_schema_context(vec![
            source.declarations()[0].clone(),
            Declaration::IntervalVariable {
                binder: BinderId(2),
                name: "i".to_owned(),
            },
        ])
        .unwrap();
        weakening_substitution(&source, &target).unwrap();
        let exchange = exchange_adjacent(&target, 0).unwrap();
        assert!(exchange.inverse_on_declarations());
        replay_adjacent_exchange(&target, &exchange).unwrap();
        let mut mutated_exchange = exchange.clone();
        mutated_exchange.inverse_on_declarations = false;
        assert_eq!(
            replay_adjacent_exchange(&target, &mutated_exchange),
            Err(SchemaContextError::CompositionMismatch)
        );
        assert!(matches!(
            exchange.after().declarations(),
            [
                Declaration::IntervalVariable { .. },
                Declaration::TypeParameter { .. }
            ]
        ));

        let dependent = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(1),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(2),
                name: "x".to_owned(),
                ty: TypeExpr::parameter(1),
            },
        ])
        .unwrap();
        assert_eq!(
            exchange_adjacent(&dependent, 0),
            Err(SchemaContextError::DependentExchange {
                left: BinderId(1),
                right: BinderId(2),
            })
        );
    }

    #[test]
    fn every_e1_expression_constructor_has_identity_preservation_and_support_replay() {
        assert_eq!(TYPE_EXPR_CONSTRUCTOR_COVERAGE.len(), 3);
        assert_eq!(TERM_EXPR_CONSTRUCTOR_COVERAGE.len(), 4);
        assert_eq!(DIM_EXPR_CONSTRUCTOR_COVERAGE.len(), 3);
        assert_eq!(COFIBRATION_EXPR_CONSTRUCTOR_COVERAGE.len(), 5);
        assert_eq!(DECLARATION_CONSTRUCTOR_COVERAGE.len(), 5);
        let context = five_kind_context();
        let identity = identity_substitution(&context).unwrap();
        let a = TypeExpr::parameter(1);
        let variable = TermExpr::variable(2);
        let library = TermExpr::Library {
            step: 1,
            symbol: "base".to_owned(),
        };
        let path = |point: TermExpr| TypeExpr::Path {
            carrier: Box::new(a.clone()),
            left: Box::new(point.clone()),
            right: Box::new(point),
        };
        let expressions = vec![
            TypedExpression::Type {
                expression: a.clone(),
            },
            TypedExpression::Type {
                expression: TypeExpr::trunc(a.clone()),
            },
            TypedExpression::Type {
                expression: path(variable.clone()),
            },
            TypedExpression::Term {
                expression: variable.clone(),
                ty: a.clone(),
            },
            TypedExpression::Term {
                expression: library.clone(),
                ty: a.clone(),
            },
            TypedExpression::Term {
                expression: TermExpr::TruncPoint {
                    carrier: Box::new(a.clone()),
                    point: Box::new(variable.clone()),
                },
                ty: TypeExpr::trunc(a.clone()),
            },
            TypedExpression::Term {
                expression: TermExpr::Reflexivity {
                    carrier: Box::new(a.clone()),
                    point: Box::new(variable.clone()),
                },
                ty: path(variable),
            },
            TypedExpression::Dimension {
                expression: DimExpr::Zero,
            },
            TypedExpression::Dimension {
                expression: DimExpr::One,
            },
            TypedExpression::Dimension {
                expression: DimExpr::Variable(BinderId(4)),
            },
            TypedExpression::Cofibration {
                expression: CofibrationExpr::False,
            },
            TypedExpression::Cofibration {
                expression: CofibrationExpr::True,
            },
            TypedExpression::Cofibration {
                expression: CofibrationExpr::endpoint(4, false),
            },
            TypedExpression::Cofibration {
                expression: CofibrationExpr::And {
                    terms: vec![CofibrationExpr::endpoint(4, false)],
                },
            },
            TypedExpression::Cofibration {
                expression: CofibrationExpr::Or {
                    terms: vec![
                        CofibrationExpr::endpoint(4, false),
                        CofibrationExpr::endpoint(4, true),
                    ],
                },
            },
        ];
        for expression in expressions {
            let preservation =
                issue_substitution_preservation(&identity, expression.clone()).unwrap();
            replay_substitution_preservation(&identity, &preservation).unwrap();
            let support = issue_substitution_support(&identity, expression).unwrap();
            assert!(support.target_support_within_image_bound());
            replay_substitution_support(&identity, &support).unwrap();
        }
        let library_path = TypedExpression::Type {
            expression: path(library),
        };
        let support = issue_substitution_support(&identity, library_path).unwrap();
        assert_eq!(support.target_support().libraries.len(), 1);
    }

    #[test]
    fn face_substitution_is_typed_and_entailment_is_fail_closed() {
        let source = form_schema_context(vec![
            Declaration::IntervalVariable {
                binder: BinderId(1),
                name: "i".to_owned(),
            },
            Declaration::CofibrationAssumption {
                binder: BinderId(2),
                name: "i0".to_owned(),
                formula: CofibrationExpr::endpoint(1, false),
            },
        ])
        .unwrap();
        let target = form_schema_context(vec![]).unwrap();
        let at_zero = issue_typed_substitution(
            &source,
            &target,
            vec![
                SubstitutionImage::Dimension {
                    source: BinderId(1),
                    image: DimExpr::Zero,
                },
                SubstitutionImage::Cofibration {
                    source: BinderId(2),
                    image: CofibrationExpr::True,
                },
            ],
        );
        assert!(at_zero.is_ok());
        let at_one = issue_typed_substitution(
            &source,
            &target,
            vec![
                SubstitutionImage::Dimension {
                    source: BinderId(1),
                    image: DimExpr::One,
                },
                SubstitutionImage::Cofibration {
                    source: BinderId(2),
                    image: CofibrationExpr::False,
                },
            ],
        );
        assert!(at_one.is_err());
    }

    #[test]
    fn rigid_libraries_are_stable_keys_not_retargetable_binder_variables() {
        let path_through_library = |carrier: u32| TypeExpr::Path {
            carrier: Box::new(TypeExpr::parameter(carrier)),
            left: Box::new(TermExpr::Library {
                step: 7,
                symbol: "base".to_owned(),
            }),
            right: Box::new(TermExpr::Library {
                step: 7,
                symbol: "base".to_owned(),
            }),
        };
        let context = |type_binder, library_binder, path_binder, collision| {
            let mut declarations = vec![Declaration::TypeParameter {
                binder: BinderId(type_binder),
                name: "T".to_owned(),
                universe: 0,
            }];
            if let Some(binder) = collision {
                declarations.push(Declaration::OpaqueElement {
                    binder: BinderId(binder),
                    name: "collision".to_owned(),
                    ty: TypeExpr::parameter(type_binder),
                });
            }
            declarations.extend([
                Declaration::LibraryReference {
                    binder: BinderId(library_binder),
                    name: "base".to_owned(),
                    step: 7,
                    symbol: "base".to_owned(),
                    ty: TypeExpr::parameter(type_binder),
                },
                Declaration::OpaqueElement {
                    binder: BinderId(path_binder),
                    name: "p".to_owned(),
                    ty: path_through_library(type_binder),
                },
            ]);
            form_schema_context(declarations).unwrap()
        };
        let source = context(1, 2, 3, None);
        let middle = context(10, 20, 30, None);
        // Binder 2 deliberately collides with the source library binder, but
        // denotes an unrelated opaque target element.
        let target = context(40, 50, 60, Some(2));

        let first = issue_typed_substitution(
            &source,
            &middle,
            vec![
                SubstitutionImage::Type {
                    source: BinderId(1),
                    image: TypeExpr::parameter(10),
                },
                SubstitutionImage::RigidLibrary {
                    source: BinderId(2),
                    step: 7,
                    symbol: "base".to_owned(),
                },
                SubstitutionImage::Term {
                    source: BinderId(3),
                    image: TermExpr::variable(30),
                },
            ],
        )
        .unwrap();
        let second = issue_typed_substitution(
            &middle,
            &target,
            vec![
                SubstitutionImage::Type {
                    source: BinderId(10),
                    image: TypeExpr::parameter(40),
                },
                SubstitutionImage::RigidLibrary {
                    source: BinderId(20),
                    step: 7,
                    symbol: "base".to_owned(),
                },
                SubstitutionImage::Term {
                    source: BinderId(30),
                    image: TermExpr::Reflexivity {
                        carrier: Box::new(TypeExpr::parameter(40)),
                        point: Box::new(TermExpr::Library {
                            step: 7,
                            symbol: "base".to_owned(),
                        }),
                    },
                },
            ],
        )
        .unwrap();
        let composed = compose_typed_substitutions(&first, &second).unwrap();
        let expression = TypedExpression::Term {
            expression: TermExpr::variable(3),
            ty: path_through_library(1),
        };
        let support = issue_substitution_support(composed.composed(), expression).unwrap();
        assert!(support.target_support_within_image_bound());
        assert_eq!(
            support.target_support().libraries,
            vec![LibraryKey {
                step: 7,
                symbol: "base".to_owned(),
            }]
        );
        assert!(!support.target_support().binders.contains(&BinderId(2)));

        let stale_library_variable = TypeExpr::Path {
            carrier: Box::new(TypeExpr::parameter(1)),
            left: Box::new(TermExpr::variable(2)),
            right: Box::new(TermExpr::variable(2)),
        };
        assert!(check_type(&source, &stale_library_variable).is_err());
    }
}
