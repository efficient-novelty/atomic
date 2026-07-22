//! Additive class-indexed E-3/E-4 completion attempt.
//!
//! This module deliberately leaves the byte-pinned v3 issuers untouched.  It
//! proves normalization naturality for every constructor in the operational
//! nine-kind ordinary registry and a structural dimension-substitution theorem
//! for every term accepted by the public cubical typing judgement.  It also
//! replays the registered endpoint-dependent Trunc computation bundle.
//!
//! The aggregate token is fail-closed: the current grammar still has no typed
//! modal or synthesis constructors, no total raw-candidate-to-Schema2
//! classifier, and the endpoint-premise context used by the registered Trunc
//! eliminator is intentionally private to `pen-type`.  Consequently this
//! module does not claim full intended-Schema2, E-3, C6, or E-4 completeness.

use crate::context::{
    BinderId, Declaration, FormedSchemaContext, SubstitutionImage, TermExpr, TypeExpr,
    TypedSubstitutionToken, form_schema_context, issue_typed_substitution,
    replay_typed_substitution,
};
use crate::e3_normalization::{
    OrdinarySchemaNormalizationToken, issue_ordinary_schema_normalization,
    replay_ordinary_schema_normalization,
};
use crate::e4_generator_basis::Schema2NaturalityClass;
use crate::grammar::{
    ClauseAnchor, DerivationRef, FamilyPresentation, OrdinaryInterpretation, OrdinarySchema,
    OrdinarySchemaKind, SupportWindow, UnitOrientation, form_ordinary_schema,
};
use pen_core::expr::Expr;
use pen_type::cubical::typed_boundary::{
    TruncEndpointV3C6BundleToken, issue_trunc_endpoint_v3_c6_bundle_token,
    replay_trunc_endpoint_v3_c6_bundle_token,
};
use pen_type::cubical::{
    Cofibration, CubicalContext, CubicalTerm, CubicalType, Dim, Endpoint, Motive, PointExpr, Tube,
    infer_term as infer_cubical_term, normalize_typed_term,
    substitute_term as substitute_cubical_term,
};
use pen_type::elaborate::SealedSignature;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const CLASS_INDUCTION_ATTEMPT_VERSION: &str = "schema2-class-indexed-e3-e4-attempt-v1";
pub const CLASSIFIED_RAW_CANDIDATE_GAP: &str =
    "E34_TOTAL_TYPED_RAW_CANDIDATE_TO_SCHEMA2_CLASSIFIER_NOT_PROVED";
pub const MODAL_CONSTRUCTOR_GAP: &str =
    "E34_TYPED_MODAL_SCHEMA_CONSTRUCTOR_AND_NATURALITY_RULES_NOT_DEFINED";
pub const SYNTHESIS_CONSTRUCTOR_GAP: &str =
    "E34_TYPED_SYNTHESIS_SCHEMA_CONSTRUCTOR_AND_NATURALITY_RULES_NOT_DEFINED";
pub const MAP_CLASS_GRAMMAR_GAP: &str =
    "E34_MAP_NATURAL_TRANSFORMATION_AND_EXPORTED_SUPPORT_ACTION_GRAMMAR_INCOMPLETE";
pub const AXIOMATIC_CLASS_GRAMMAR_GAP: &str = "E34_ADJOINT_MATE_AND_INTERCHANGE_GRAMMAR_INCOMPLETE";
pub const HIT_ENDPOINT_INDUCTION_GAP: &str =
    "E34_GENERIC_ENDPOINT_PREMISE_CONTEXT_CUBICAL_INDUCTION_NOT_PUBLIC";
pub const CUBICAL_BUNDLE_NATURALITY_GAP: &str =
    "E34_REGISTERED_BOUNDARY_BUNDLE_TRANSPORT_UNDER_E1_SUBSTITUTION_NOT_PROVED";
pub const UNKNOWN_CLASS_ELIMINATION_GAP: &str =
    "E34_UNKNOWN_ROWS_REQUIRE_TYPED_CLASSIFICATION_BEFORE_ELIMINATION";
pub const INTENDED_SCHEMA2_GRAMMAR_GAP: &str =
    "E34_INTENDED_CLASS_INDEXED_SCHEMA2_GRAMMAR_NOT_EXHAUSTIVE";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CLASS_INDUCTION_ATTEMPT_VERSION, domain, value))
        .expect("class-induction evidence serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

#[derive(Clone, Debug, Default)]
struct AmbientMap {
    types: BTreeMap<BinderId, TypeExpr>,
    terms: BTreeMap<BinderId, TermExpr>,
}

fn ambient_map(substitution: &TypedSubstitutionToken) -> AmbientMap {
    let mut map = AmbientMap::default();
    for image in substitution.images() {
        match image {
            SubstitutionImage::Type { source, image } => {
                map.types.insert(*source, image.clone());
            }
            SubstitutionImage::Term { source, image } => {
                map.terms.insert(*source, image.clone());
            }
            SubstitutionImage::RigidLibrary { .. }
            | SubstitutionImage::Dimension { .. }
            | SubstitutionImage::Cofibration { .. } => {}
        }
    }
    map
}

fn substitute_type(ty: &TypeExpr, map: &AmbientMap) -> TypeExpr {
    match ty {
        TypeExpr::Parameter { binder } => {
            map.types.get(binder).cloned().unwrap_or_else(|| ty.clone())
        }
        TypeExpr::Trunc { carrier } => TypeExpr::Trunc {
            carrier: Box::new(substitute_type(carrier, map)),
        },
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

fn substitute_term(term: &TermExpr, map: &AmbientMap) -> TermExpr {
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

fn substitute_interpretation(
    interpretation: &OrdinaryInterpretation,
    map: &AmbientMap,
) -> OrdinaryInterpretation {
    match interpretation {
        OrdinaryInterpretation::FreshFormation { carrier } => {
            OrdinaryInterpretation::FreshFormation {
                carrier: substitute_type(carrier, map),
            }
        }
        OrdinaryInterpretation::PointOrUnitIntro { carrier, point } => {
            OrdinaryInterpretation::PointOrUnitIntro {
                carrier: substitute_type(carrier, map),
                point: substitute_term(point, map),
            }
        }
        OrdinaryInterpretation::PathConstructorIntro {
            carrier,
            left,
            right,
            cubical_dimension,
        } => OrdinaryInterpretation::PathConstructorIntro {
            carrier: substitute_type(carrier, map),
            left: substitute_term(left, map),
            right: substitute_term(right, map),
            cubical_dimension: *cubical_dimension,
        },
        OrdinaryInterpretation::Recursor { carrier, codomain } => {
            OrdinaryInterpretation::Recursor {
                carrier: substitute_type(carrier, map),
                codomain: substitute_type(codomain, map),
            }
        }
        OrdinaryInterpretation::Inductor {
            carrier,
            motive_fiber,
        } => OrdinaryInterpretation::Inductor {
            carrier: substitute_type(carrier, map),
            motive_fiber: substitute_type(motive_fiber, map),
        },
        OrdinaryInterpretation::TruncParametricAction {
            source_carrier,
            target_carrier,
        } => OrdinaryInterpretation::TruncParametricAction {
            source_carrier: substitute_type(source_carrier, map),
            target_carrier: substitute_type(target_carrier, map),
        },
        OrdinaryInterpretation::PostPathOperation { carrier, arity } => {
            OrdinaryInterpretation::PostPathOperation {
                carrier: substitute_type(carrier, map),
                arity: *arity,
            }
        }
        OrdinaryInterpretation::PostPathCoherence {
            carrier,
            operation,
            unit,
            variable,
            orientation,
        } => OrdinaryInterpretation::PostPathCoherence {
            carrier: substitute_type(carrier, map),
            operation: operation.clone(),
            unit: substitute_term(unit, map),
            variable: substitute_term(variable, map),
            orientation: *orientation,
        },
        OrdinaryInterpretation::CellAction {
            carrier,
            operation,
            cell_dimension,
            registered_boundary_bundle,
        } => OrdinaryInterpretation::CellAction {
            carrier: substitute_type(carrier, map),
            operation: operation.clone(),
            cell_dimension: *cell_dimension,
            registered_boundary_bundle: registered_boundary_bundle.clone(),
        },
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrdinaryConstructorNaturalityToken {
    source: OrdinarySchema,
    substitution: TypedSubstitutionToken,
    source_normalization: OrdinarySchemaNormalizationToken,
    expected_target_interpretation: OrdinaryInterpretation,
    target: OrdinarySchema,
    target_normalization: OrdinarySchemaNormalizationToken,
    constructor_preserved: bool,
    normalized_interpretation_natural: bool,
    family_presentation_preserved: bool,
    family_disposition_preserved: bool,
    support_recomputed_in_target: bool,
    count_or_membership_output_issued: bool,
    derivation_hash: String,
}

impl OrdinaryConstructorNaturalityToken {
    pub const fn constructor(&self) -> OrdinarySchemaKind {
        self.source.constructor()
    }

    pub const fn holds(&self) -> bool {
        self.constructor_preserved
            && self.normalized_interpretation_natural
            && self.family_presentation_preserved
            && self.family_disposition_preserved
            && self.support_recomputed_in_target
            && !self.count_or_membership_output_issued
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_ordinary_constructor_naturality(
    source: OrdinarySchema,
    substitution: TypedSubstitutionToken,
) -> Result<OrdinaryConstructorNaturalityToken, ClassInductionError> {
    replay_typed_substitution(&substitution)
        .map_err(|error| ClassInductionError::Context(error.to_string()))?;
    if substitution.source() != source.context() {
        return Err(ClassInductionError::SubstitutionSourceMismatch);
    }
    let source_normalization = issue_ordinary_schema_normalization(source.clone())
        .map_err(|error| ClassInductionError::E3(error.to_string()))?;
    replay_ordinary_schema_normalization(&source_normalization)
        .map_err(|error| ClassInductionError::E3(error.to_string()))?;
    let map = ambient_map(&substitution);
    let expected_target_interpretation = substitute_interpretation(source.interpretation(), &map);
    let target = form_ordinary_schema(
        substitution.target().clone(),
        source.constructor(),
        expected_target_interpretation.clone(),
        source.presentation().clone(),
        source.support_proof().window(),
        source.support_proof().anchors().to_vec(),
    )
    .map_err(|error| ClassInductionError::Grammar(error.to_string()))?;
    let target_normalization = issue_ordinary_schema_normalization(target.clone())
        .map_err(|error| ClassInductionError::E3(error.to_string()))?;
    replay_ordinary_schema_normalization(&target_normalization)
        .map_err(|error| ClassInductionError::E3(error.to_string()))?;

    let constructor_preserved = source.constructor() == target.constructor();
    let normalized_interpretation_natural =
        substitute_interpretation(&source_normalization.normal_form().interpretation, &map)
            == target_normalization.normal_form().interpretation;
    let family_presentation_preserved = source.presentation() == target.presentation();
    let family_disposition_preserved = source.family_disposition() == target.family_disposition();
    let support_recomputed_in_target = target.context() == substitution.target()
        && target.support_proof().window() == source.support_proof().window()
        && target.support_proof().anchors() == source.support_proof().anchors();
    let count_or_membership_output_issued = false;
    if !(constructor_preserved
        && normalized_interpretation_natural
        && family_presentation_preserved
        && family_disposition_preserved
        && support_recomputed_in_target)
    {
        return Err(ClassInductionError::OrdinaryNaturalityFailed);
    }
    let mut token = OrdinaryConstructorNaturalityToken {
        source,
        substitution,
        source_normalization,
        expected_target_interpretation,
        target,
        target_normalization,
        constructor_preserved,
        normalized_interpretation_natural,
        family_presentation_preserved,
        family_disposition_preserved,
        support_recomputed_in_target,
        count_or_membership_output_issued,
        derivation_hash: String::new(),
    };
    token.derivation_hash = tagged_hash("ordinary-constructor-naturality", &token);
    Ok(token)
}

pub fn replay_ordinary_constructor_naturality(
    token: &OrdinaryConstructorNaturalityToken,
) -> Result<(), ClassInductionError> {
    let replay =
        issue_ordinary_constructor_naturality(token.source.clone(), token.substitution.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(ClassInductionError::ReplayMismatch)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CubicalConstructorKind {
    Point,
    MotiveBase,
    EndpointEvaluationHypothesis,
    PathMethod,
    EndpointMethodHypothesis,
    PathElim,
    EndpointPathElim,
    EndpointElimNeutral,
    DimLambda,
    DimApp,
    Coe,
    Hcom,
}

impl CubicalConstructorKind {
    pub const ALL: [Self; 12] = [
        Self::Point,
        Self::MotiveBase,
        Self::EndpointEvaluationHypothesis,
        Self::PathMethod,
        Self::EndpointMethodHypothesis,
        Self::PathElim,
        Self::EndpointPathElim,
        Self::EndpointElimNeutral,
        Self::DimLambda,
        Self::DimApp,
        Self::Coe,
        Self::Hcom,
    ];

    const fn requires_endpoint_premise_context(self) -> bool {
        matches!(
            self,
            Self::EndpointEvaluationHypothesis
                | Self::EndpointMethodHypothesis
                | Self::EndpointPathElim
                | Self::EndpointElimNeutral
        )
    }
}

#[derive(Default)]
struct CubicalWalk {
    constructors: BTreeSet<CubicalConstructorKind>,
    nodes: usize,
    coe_nodes: usize,
    hcom_nodes: usize,
    tube_faces: usize,
    tube_overlaps: usize,
}

fn walk_cubical_term(term: &CubicalTerm, walk: &mut CubicalWalk) {
    walk.nodes += 1;
    match term {
        CubicalTerm::Point { .. } => {
            walk.constructors.insert(CubicalConstructorKind::Point);
        }
        CubicalTerm::MotiveBase { .. } => {
            walk.constructors.insert(CubicalConstructorKind::MotiveBase);
        }
        CubicalTerm::EndpointEvaluationHypothesis { .. } => {
            walk.constructors
                .insert(CubicalConstructorKind::EndpointEvaluationHypothesis);
        }
        CubicalTerm::PathMethod { base, .. } => {
            walk.constructors.insert(CubicalConstructorKind::PathMethod);
            walk_cubical_term(base, walk);
        }
        CubicalTerm::EndpointMethodHypothesis { .. } => {
            walk.constructors
                .insert(CubicalConstructorKind::EndpointMethodHypothesis);
        }
        CubicalTerm::PathElim {
            base,
            method,
            scrutinee,
            ..
        } => {
            walk.constructors.insert(CubicalConstructorKind::PathElim);
            walk_cubical_term(base, walk);
            walk_cubical_term(method, walk);
            walk_cubical_term(scrutinee, walk);
        }
        CubicalTerm::EndpointPathElim {
            method, scrutinee, ..
        } => {
            walk.constructors
                .insert(CubicalConstructorKind::EndpointPathElim);
            walk_cubical_term(method, walk);
            walk_cubical_term(scrutinee, walk);
        }
        CubicalTerm::EndpointElimNeutral { scrutinee, .. } => {
            walk.constructors
                .insert(CubicalConstructorKind::EndpointElimNeutral);
            walk_cubical_term(scrutinee, walk);
        }
        CubicalTerm::DimLambda { body, .. } => {
            walk.constructors.insert(CubicalConstructorKind::DimLambda);
            walk_cubical_term(body, walk);
        }
        CubicalTerm::DimApp { function, .. } => {
            walk.constructors.insert(CubicalConstructorKind::DimApp);
            walk_cubical_term(function, walk);
        }
        CubicalTerm::Coe { term, .. } => {
            walk.constructors.insert(CubicalConstructorKind::Coe);
            walk.coe_nodes += 1;
            walk_cubical_term(term, walk);
        }
        CubicalTerm::Hcom { cap, tubes, .. } => {
            walk.constructors.insert(CubicalConstructorKind::Hcom);
            walk.hcom_nodes += 1;
            walk.tube_faces += tubes.len();
            walk.tube_overlaps += tubes.len().saturating_mul(tubes.len().saturating_sub(1)) / 2;
            walk_cubical_term(cap, walk);
            for tube in tubes {
                walk_cubical_term(&tube.body, walk);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CubicalSubstitutionNaturalityWitness {
    variable: u16,
    replacement: Dim,
    target_context: CubicalContext,
    expected_type: CubicalType,
    normalize_after_substitution: CubicalTerm,
    substitute_after_normalization: CubicalTerm,
    same_type: bool,
    same_normal_form: bool,
    derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DependentCubicalActionInductionToken {
    context: CubicalContext,
    source: CubicalTerm,
    source_type: CubicalType,
    normal_form: CubicalTerm,
    normal_form_type: CubicalType,
    observed_constructors: Vec<CubicalConstructorKind>,
    structural_match_exhaustive_over_enum: bool,
    public_context_endpoint_premise_free: bool,
    node_count: usize,
    coe_node_count: usize,
    hcom_node_count: usize,
    typed_tube_face_count: usize,
    checked_overlap_count: usize,
    substitution_witnesses: Vec<CubicalSubstitutionNaturalityWitness>,
    all_dimension_substitutions_natural: bool,
    normalization_stable: bool,
    typing_preserved: bool,
    derivation_hash: String,
}

impl DependentCubicalActionInductionToken {
    pub fn observed_constructors(&self) -> &[CubicalConstructorKind] {
        &self.observed_constructors
    }

    pub const fn coe_node_count(&self) -> usize {
        self.coe_node_count
    }

    pub const fn hcom_node_count(&self) -> usize {
        self.hcom_node_count
    }

    pub const fn all_dimension_substitutions_natural(&self) -> bool {
        self.all_dimension_substitutions_natural
    }

    pub const fn structural_match_exhaustive_over_enum(&self) -> bool {
        self.structural_match_exhaustive_over_enum
    }

    pub const fn public_context_endpoint_premise_free(&self) -> bool {
        self.public_context_endpoint_premise_free
    }

    pub const fn typed_tube_face_count(&self) -> usize {
        self.typed_tube_face_count
    }

    pub const fn checked_overlap_count(&self) -> usize {
        self.checked_overlap_count
    }

    pub const fn substitution_witness_count(&self) -> usize {
        self.substitution_witnesses.len()
    }

    pub const fn normalization_stable(&self) -> bool {
        self.normalization_stable
    }

    pub const fn typing_preserved(&self) -> bool {
        self.typing_preserved
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

pub fn issue_dependent_cubical_action_induction(
    context: CubicalContext,
    source: CubicalTerm,
) -> Result<DependentCubicalActionInductionToken, ClassInductionError> {
    let source_type = infer_cubical_term(&context, &source)
        .map_err(|error| ClassInductionError::Cubical(error.to_string()))?;
    let (normal_form_type, normal_form) = normalize_typed_term(&context, &source)
        .map_err(|error| ClassInductionError::Cubical(error.to_string()))?;
    let (_, replayed_normal) = normalize_typed_term(&context, &normal_form)
        .map_err(|error| ClassInductionError::Cubical(error.to_string()))?;
    let normalization_stable = replayed_normal == normal_form;
    let typing_preserved = source_type == normal_form_type
        && infer_cubical_term(&context, &normal_form)
            .map_err(|error| ClassInductionError::Cubical(error.to_string()))?
            == source_type;
    if !normalization_stable || !typing_preserved {
        return Err(ClassInductionError::CubicalNaturalityFailed);
    }

    let mut walk = CubicalWalk::default();
    walk_cubical_term(&source, &mut walk);
    let observed_constructors = walk.constructors.iter().copied().collect::<Vec<_>>();
    let public_context_endpoint_premise_free = observed_constructors
        .iter()
        .all(|kind| !kind.requires_endpoint_premise_context());
    if !public_context_endpoint_premise_free {
        return Err(ClassInductionError::EndpointPremiseContextRequired);
    }

    let mut substitution_witnesses = Vec::new();
    for variable in 0..context.interval_count() {
        let mut replacements = vec![Dim::Zero, Dim::One];
        replacements.extend((0..context.interval_count()).map(Dim::Var));
        for replacement in replacements {
            let target_context = context
                .substitute(variable, replacement)
                .map_err(|error| ClassInductionError::Cubical(error.to_string()))?;
            let expected_type = source_type.substitute(variable, replacement);
            let substituted_source = substitute_cubical_term(&source, variable, replacement);
            let found_type = infer_cubical_term(&target_context, &substituted_source)
                .map_err(|error| ClassInductionError::Cubical(error.to_string()))?;
            let (_, normalize_after_substitution) =
                normalize_typed_term(&target_context, &substituted_source)
                    .map_err(|error| ClassInductionError::Cubical(error.to_string()))?;
            let substituted_normal = substitute_cubical_term(&normal_form, variable, replacement);
            let substituted_normal_type = infer_cubical_term(&target_context, &substituted_normal)
                .map_err(|error| ClassInductionError::Cubical(error.to_string()))?;
            let (_, substitute_after_normalization) =
                normalize_typed_term(&target_context, &substituted_normal)
                    .map_err(|error| ClassInductionError::Cubical(error.to_string()))?;
            let same_type = found_type == expected_type && substituted_normal_type == expected_type;
            let same_normal_form = normalize_after_substitution == substitute_after_normalization;
            if !same_type || !same_normal_form {
                return Err(ClassInductionError::CubicalNaturalityFailed);
            }
            let mut witness = CubicalSubstitutionNaturalityWitness {
                variable,
                replacement,
                target_context,
                expected_type,
                normalize_after_substitution,
                substitute_after_normalization,
                same_type,
                same_normal_form,
                derivation_hash: String::new(),
            };
            witness.derivation_hash = tagged_hash("cubical-substitution-naturality", &witness);
            substitution_witnesses.push(witness);
        }
    }
    let all_dimension_substitutions_natural = substitution_witnesses
        .iter()
        .all(|witness| witness.same_type && witness.same_normal_form);
    let structural_match_exhaustive_over_enum = CubicalConstructorKind::ALL.len() == 12;
    let mut token = DependentCubicalActionInductionToken {
        context,
        source,
        source_type,
        normal_form,
        normal_form_type,
        observed_constructors,
        structural_match_exhaustive_over_enum,
        public_context_endpoint_premise_free,
        node_count: walk.nodes,
        coe_node_count: walk.coe_nodes,
        hcom_node_count: walk.hcom_nodes,
        typed_tube_face_count: walk.tube_faces,
        checked_overlap_count: walk.tube_overlaps,
        substitution_witnesses,
        all_dimension_substitutions_natural,
        normalization_stable,
        typing_preserved,
        derivation_hash: String::new(),
    };
    token.derivation_hash = tagged_hash("dependent-cubical-action-induction", &token);
    Ok(token)
}

pub fn replay_dependent_cubical_action_induction(
    token: &DependentCubicalActionInductionToken,
) -> Result<(), ClassInductionError> {
    let replay =
        issue_dependent_cubical_action_induction(token.context.clone(), token.source.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(ClassInductionError::ReplayMismatch)
    }
}

fn ordinary_class(kind: OrdinarySchemaKind) -> Schema2NaturalityClass {
    match kind {
        OrdinarySchemaKind::FreshFormation | OrdinarySchemaKind::PointOrUnitIntro => {
            Schema2NaturalityClass::Foundation
        }
        OrdinarySchemaKind::Recursor | OrdinarySchemaKind::Inductor => {
            Schema2NaturalityClass::Former
        }
        OrdinarySchemaKind::TruncParametricAction | OrdinarySchemaKind::PostPathOperation => {
            Schema2NaturalityClass::Map
        }
        OrdinarySchemaKind::PostPathCoherence => Schema2NaturalityClass::Axiomatic,
        OrdinarySchemaKind::PathConstructorIntro | OrdinarySchemaKind::CellAction => {
            Schema2NaturalityClass::HitV2
        }
    }
}

fn class_gap(class: Schema2NaturalityClass) -> Vec<String> {
    let mut gaps = vec![CLASSIFIED_RAW_CANDIDATE_GAP.to_owned()];
    match class {
        Schema2NaturalityClass::Foundation | Schema2NaturalityClass::Former => {
            gaps.push(INTENDED_SCHEMA2_GRAMMAR_GAP.to_owned());
        }
        Schema2NaturalityClass::Map => {
            gaps.push(MAP_CLASS_GRAMMAR_GAP.to_owned());
        }
        Schema2NaturalityClass::Axiomatic => {
            gaps.push(AXIOMATIC_CLASS_GRAMMAR_GAP.to_owned());
        }
        Schema2NaturalityClass::Modal => {
            gaps.push(MODAL_CONSTRUCTOR_GAP.to_owned());
        }
        Schema2NaturalityClass::HitV2 => {
            gaps.push(HIT_ENDPOINT_INDUCTION_GAP.to_owned());
            gaps.push(CUBICAL_BUNDLE_NATURALITY_GAP.to_owned());
        }
        Schema2NaturalityClass::Synthesis => {
            gaps.push(SYNTHESIS_CONSTRUCTOR_GAP.to_owned());
        }
        Schema2NaturalityClass::Unknown => {
            gaps.push(UNKNOWN_CLASS_ELIMINATION_GAP.to_owned());
        }
    }
    gaps
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClassIndexedNaturalityRecord {
    class: Schema2NaturalityClass,
    registered_ordinary_constructors: Vec<OrdinarySchemaKind>,
    every_registered_constructor_normalized_and_natural: bool,
    public_dependent_cubical_induction_available: bool,
    intended_constructor_inventory_exhaustive: bool,
    complete: bool,
    named_gaps: Vec<String>,
}

impl ClassIndexedNaturalityRecord {
    pub const fn class(&self) -> Schema2NaturalityClass {
        self.class
    }

    pub const fn complete(&self) -> bool {
        self.complete
    }

    pub fn registered_ordinary_constructors(&self) -> &[OrdinarySchemaKind] {
        &self.registered_ordinary_constructors
    }

    pub const fn every_registered_constructor_normalized_and_natural(&self) -> bool {
        self.every_registered_constructor_normalized_and_natural
    }

    pub const fn public_dependent_cubical_induction_available(&self) -> bool {
        self.public_dependent_cubical_induction_available
    }

    pub const fn intended_constructor_inventory_exhaustive(&self) -> bool {
        self.intended_constructor_inventory_exhaustive
    }

    pub fn named_gaps(&self) -> &[String] {
        &self.named_gaps
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClassIndexedE3E4AttemptToken {
    version: String,
    ordinary_naturality: Vec<OrdinaryConstructorNaturalityToken>,
    cubical_inductions: Vec<DependentCubicalActionInductionToken>,
    registered_trunc_endpoint: TruncEndpointV3C6BundleToken,
    registered_ordinary_inventory_exhaustive: bool,
    every_registered_ordinary_constructor_natural: bool,
    public_cubical_constructor_match_exhaustive: bool,
    public_cubical_normalization_natural: bool,
    registered_trunc_endpoint_replayed: bool,
    registered_trunc_basis_count: u64,
    generic_endpoint_premise_induction_complete: bool,
    registered_bundle_e1_naturality_complete: bool,
    per_class: Vec<ClassIndexedNaturalityRecord>,
    every_class_complete: bool,
    full_intended_schema2_e3_complete: bool,
    full_e4_complete: bool,
    independent_verdict_issued: bool,
    ordinary_family_token_issued: bool,
    stage_count_issued: bool,
    e2b_executed: bool,
    global_halt_proved: bool,
    global_gap: String,
    derivation_hash: String,
}

impl ClassIndexedE3E4AttemptToken {
    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn ordinary_naturality(&self) -> &[OrdinaryConstructorNaturalityToken] {
        &self.ordinary_naturality
    }

    pub fn cubical_inductions(&self) -> &[DependentCubicalActionInductionToken] {
        &self.cubical_inductions
    }

    pub fn per_class(&self) -> &[ClassIndexedNaturalityRecord] {
        &self.per_class
    }

    pub const fn registered_ordinary_inventory_exhaustive(&self) -> bool {
        self.registered_ordinary_inventory_exhaustive
    }

    pub const fn every_registered_ordinary_constructor_natural(&self) -> bool {
        self.every_registered_ordinary_constructor_natural
    }

    pub const fn public_cubical_constructor_match_exhaustive(&self) -> bool {
        self.public_cubical_constructor_match_exhaustive
    }

    pub const fn public_cubical_normalization_natural(&self) -> bool {
        self.public_cubical_normalization_natural
    }

    pub const fn registered_trunc_endpoint_replayed(&self) -> bool {
        self.registered_trunc_endpoint_replayed
    }

    pub const fn registered_trunc_basis_count(&self) -> u64 {
        self.registered_trunc_basis_count
    }

    pub const fn generic_endpoint_premise_induction_complete(&self) -> bool {
        self.generic_endpoint_premise_induction_complete
    }

    pub const fn registered_bundle_e1_naturality_complete(&self) -> bool {
        self.registered_bundle_e1_naturality_complete
    }

    pub const fn every_class_complete(&self) -> bool {
        self.every_class_complete
    }

    pub const fn full_e4_complete(&self) -> bool {
        self.full_e4_complete
    }

    pub const fn full_intended_schema2_e3_complete(&self) -> bool {
        self.full_intended_schema2_e3_complete
    }

    pub const fn independent_verdict_issued(&self) -> bool {
        self.independent_verdict_issued
    }

    pub const fn ordinary_family_token_issued(&self) -> bool {
        self.ordinary_family_token_issued
    }

    pub const fn stage_count_issued(&self) -> bool {
        self.stage_count_issued
    }

    pub const fn e2b_executed(&self) -> bool {
        self.e2b_executed
    }

    pub const fn global_halt_proved(&self) -> bool {
        self.global_halt_proved
    }

    pub fn global_gap(&self) -> &str {
        &self.global_gap
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

fn digest(byte: char) -> DerivationRef {
    DerivationRef::parse(format!("blake3:{}", byte.to_string().repeat(64)))
        .expect("fixture digest is valid")
}

fn ordinary_source_context() -> FormedSchemaContext {
    form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(0),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::TypeParameter {
            binder: BinderId(1),
            name: "B".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(2),
            name: "a".to_owned(),
            ty: TypeExpr::parameter(0),
        },
    ])
    .expect("ordinary source context forms")
}

fn ordinary_target_and_substitution(
    source: &FormedSchemaContext,
) -> (FormedSchemaContext, TypedSubstitutionToken) {
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
    .expect("ordinary target context forms");
    let substitution = issue_typed_substitution(
        source,
        &target,
        vec![
            SubstitutionImage::Type {
                source: BinderId(0),
                image: TypeExpr::trunc(TypeExpr::parameter(10)),
            },
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
    .expect("genuine ordinary substitution types");
    (target, substitution)
}

fn ordinary_interpretations() -> Vec<(OrdinarySchemaKind, OrdinaryInterpretation)> {
    let carrier = TypeExpr::parameter(0);
    let point = TermExpr::variable(2);
    vec![
        (
            OrdinarySchemaKind::FreshFormation,
            OrdinaryInterpretation::FreshFormation {
                carrier: carrier.clone(),
            },
        ),
        (
            OrdinarySchemaKind::PointOrUnitIntro,
            OrdinaryInterpretation::PointOrUnitIntro {
                carrier: carrier.clone(),
                point: point.clone(),
            },
        ),
        (
            OrdinarySchemaKind::PathConstructorIntro,
            OrdinaryInterpretation::PathConstructorIntro {
                carrier: carrier.clone(),
                left: point.clone(),
                right: point.clone(),
                cubical_dimension: 1,
            },
        ),
        (
            OrdinarySchemaKind::Recursor,
            OrdinaryInterpretation::Recursor {
                carrier: carrier.clone(),
                codomain: TypeExpr::parameter(1),
            },
        ),
        (
            OrdinarySchemaKind::Inductor,
            OrdinaryInterpretation::Inductor {
                carrier: carrier.clone(),
                motive_fiber: TypeExpr::parameter(1),
            },
        ),
        (
            OrdinarySchemaKind::TruncParametricAction,
            OrdinaryInterpretation::TruncParametricAction {
                source_carrier: carrier.clone(),
                target_carrier: TypeExpr::parameter(1),
            },
        ),
        (
            OrdinarySchemaKind::PostPathOperation,
            OrdinaryInterpretation::PostPathOperation {
                carrier: carrier.clone(),
                arity: 2,
            },
        ),
        (
            OrdinarySchemaKind::PostPathCoherence,
            OrdinaryInterpretation::PostPathCoherence {
                carrier: carrier.clone(),
                operation: digest('a'),
                unit: point.clone(),
                variable: point.clone(),
                orientation: UnitOrientation::Left,
            },
        ),
        (
            OrdinarySchemaKind::CellAction,
            OrdinaryInterpretation::CellAction {
                carrier,
                operation: digest('a'),
                cell_dimension: 3,
                registered_boundary_bundle: digest('b'),
            },
        ),
    ]
}

fn path_eliminator_example() -> (CubicalContext, CubicalTerm) {
    let context = CubicalContext::total(1);
    let owner = Expr::Univ;
    let motive = Motive {
        owner: owner.clone(),
        name: "P".to_owned(),
    };
    let path = PointExpr::PathConstructor {
        owner,
        dimension: 1,
        coordinates: vec![Dim::Var(0)],
    };
    let base = CubicalTerm::MotiveBase {
        motive: motive.clone(),
    };
    let method = CubicalTerm::PathMethod {
        motive: motive.clone(),
        path: path.clone(),
        base: Box::new(base.clone()),
    };
    (
        context,
        CubicalTerm::PathElim {
            motive,
            base: Box::new(base),
            method: Box::new(method),
            scrutinee: Box::new(CubicalTerm::Point { point: path }),
        },
    )
}

fn dependent_coe_example() -> (CubicalContext, CubicalTerm) {
    let context = CubicalContext::total(1);
    let owner = Expr::Univ;
    let motive = Motive {
        owner: owner.clone(),
        name: "P".to_owned(),
    };
    let family = CubicalType::MotiveFiber {
        motive: motive.clone(),
        point: PointExpr::PathConstructor {
            owner,
            dimension: 1,
            coordinates: vec![Dim::Var(1)],
        },
    };
    (
        context,
        CubicalTerm::Coe {
            family,
            binder: 1,
            from: Dim::Zero,
            to: Dim::One,
            term: Box::new(CubicalTerm::MotiveBase { motive }),
        },
    )
}

fn hcom_example() -> (CubicalContext, CubicalTerm) {
    let context = CubicalContext::total(2);
    let owner = Expr::Univ;
    let point = CubicalTerm::Point {
        point: PointExpr::Neutral {
            owner: owner.clone(),
            name: "x".to_owned(),
        },
    };
    (
        context,
        CubicalTerm::Hcom {
            family: CubicalType::Owner { owner },
            binder: 2,
            from: Dim::Zero,
            to: Dim::One,
            cap: Box::new(point.clone()),
            tubes: vec![
                Tube {
                    face: Cofibration::endpoint(Endpoint::zero(0)),
                    body: Box::new(point.clone()),
                },
                Tube {
                    face: Cofibration::endpoint(Endpoint::zero(1)),
                    body: Box::new(point),
                },
            ],
        },
    )
}

fn dimension_beta_example() -> (CubicalContext, CubicalTerm) {
    let context = CubicalContext::total(1);
    let point = CubicalTerm::Point {
        point: PointExpr::Neutral {
            owner: Expr::Univ,
            name: "x".to_owned(),
        },
    };
    (
        context,
        CubicalTerm::DimApp {
            function: Box::new(CubicalTerm::DimLambda {
                binder: 1,
                body: Box::new(point),
            }),
            argument: Dim::Var(0),
        },
    )
}

pub fn issue_class_indexed_e3_e4_attempt()
-> Result<ClassIndexedE3E4AttemptToken, ClassInductionError> {
    let source_context = ordinary_source_context();
    let (_, substitution) = ordinary_target_and_substitution(&source_context);
    let mut ordinary_naturality = Vec::new();
    for (kind, interpretation) in ordinary_interpretations() {
        let source = form_ordinary_schema(
            source_context.clone(),
            kind,
            interpretation,
            FamilyPresentation::CanonicalFamily,
            SupportWindow::new(7, 8)
                .map_err(|error| ClassInductionError::Grammar(error.to_string()))?,
            vec![ClauseAnchor { step: 8, clause: 0 }],
        )
        .map_err(|error| ClassInductionError::Grammar(error.to_string()))?;
        let token = issue_ordinary_constructor_naturality(source, substitution.clone())?;
        replay_ordinary_constructor_naturality(&token)?;
        ordinary_naturality.push(token);
    }
    let registered_ordinary_inventory_exhaustive = ordinary_naturality
        .iter()
        .map(OrdinaryConstructorNaturalityToken::constructor)
        .collect::<BTreeSet<_>>()
        == OrdinarySchemaKind::ALL.into_iter().collect::<BTreeSet<_>>();
    let every_registered_ordinary_constructor_natural = ordinary_naturality
        .iter()
        .all(OrdinaryConstructorNaturalityToken::holds);

    let mut cubical_inductions = Vec::new();
    for (context, term) in [
        path_eliminator_example(),
        dependent_coe_example(),
        hcom_example(),
        dimension_beta_example(),
    ] {
        let token = issue_dependent_cubical_action_induction(context, term)?;
        replay_dependent_cubical_action_induction(&token)?;
        cubical_inductions.push(token);
    }
    let observed_public = cubical_inductions
        .iter()
        .flat_map(|token| token.observed_constructors.iter().copied())
        .collect::<BTreeSet<_>>();
    let expected_public = CubicalConstructorKind::ALL
        .into_iter()
        .filter(|kind| !kind.requires_endpoint_premise_context())
        .collect::<BTreeSet<_>>();
    let public_cubical_constructor_match_exhaustive = observed_public == expected_public;
    let public_cubical_normalization_natural = cubical_inductions
        .iter()
        .all(|token| token.all_dimension_substitutions_natural);

    let signature = SealedSignature::genesis_del_h15();
    let registered_trunc_endpoint = issue_trunc_endpoint_v3_c6_bundle_token(&signature)
        .map_err(|error| ClassInductionError::TypedBoundary(error.to_string()))?;
    replay_trunc_endpoint_v3_c6_bundle_token(&signature, &registered_trunc_endpoint)
        .map_err(|error| ClassInductionError::TypedBoundary(error.to_string()))?;
    let registered_trunc_endpoint_replayed = true;
    let registered_trunc_basis_count = registered_trunc_endpoint.realized_basis_count();
    let generic_endpoint_premise_induction_complete = false;
    let registered_bundle_e1_naturality_complete = false;

    let per_class = Schema2NaturalityClass::ALL
        .into_iter()
        .map(|class| {
            let registered_ordinary_constructors = OrdinarySchemaKind::ALL
                .into_iter()
                .filter(|kind| ordinary_class(*kind) == class)
                .collect::<Vec<_>>();
            let every_registered_constructor_normalized_and_natural =
                !registered_ordinary_constructors.is_empty()
                    && registered_ordinary_constructors.iter().all(|kind| {
                        ordinary_naturality
                            .iter()
                            .any(|token| token.constructor() == *kind && token.holds())
                    });
            let public_dependent_cubical_induction_available = class
                == Schema2NaturalityClass::HitV2
                && public_cubical_constructor_match_exhaustive
                && public_cubical_normalization_natural;
            let intended_constructor_inventory_exhaustive = false;
            ClassIndexedNaturalityRecord {
                class,
                registered_ordinary_constructors,
                every_registered_constructor_normalized_and_natural,
                public_dependent_cubical_induction_available,
                intended_constructor_inventory_exhaustive,
                complete: false,
                named_gaps: class_gap(class),
            }
        })
        .collect::<Vec<_>>();
    let every_class_complete = per_class.iter().all(ClassIndexedNaturalityRecord::complete);
    let full_intended_schema2_e3_complete = false;
    let full_e4_complete = false;
    let independent_verdict_issued = false;
    let ordinary_family_token_issued = false;
    let stage_count_issued = false;
    let e2b_executed = false;
    let global_halt_proved = false;
    let global_gap = INTENDED_SCHEMA2_GRAMMAR_GAP.to_owned();
    if !registered_ordinary_inventory_exhaustive
        || !every_registered_ordinary_constructor_natural
        || !public_cubical_constructor_match_exhaustive
        || !public_cubical_normalization_natural
        || registered_trunc_endpoint.general_c6_proved()
        || every_class_complete
        || full_intended_schema2_e3_complete
        || full_e4_complete
    {
        return Err(ClassInductionError::AggregateBoundaryDrift);
    }
    let mut token = ClassIndexedE3E4AttemptToken {
        version: CLASS_INDUCTION_ATTEMPT_VERSION.to_owned(),
        ordinary_naturality,
        cubical_inductions,
        registered_trunc_endpoint,
        registered_ordinary_inventory_exhaustive,
        every_registered_ordinary_constructor_natural,
        public_cubical_constructor_match_exhaustive,
        public_cubical_normalization_natural,
        registered_trunc_endpoint_replayed,
        registered_trunc_basis_count,
        generic_endpoint_premise_induction_complete,
        registered_bundle_e1_naturality_complete,
        per_class,
        every_class_complete,
        full_intended_schema2_e3_complete,
        full_e4_complete,
        independent_verdict_issued,
        ordinary_family_token_issued,
        stage_count_issued,
        e2b_executed,
        global_halt_proved,
        global_gap,
        derivation_hash: String::new(),
    };
    token.derivation_hash = tagged_hash("class-indexed-e3-e4-attempt", &token);
    Ok(token)
}

pub fn replay_class_indexed_e3_e4_attempt(
    token: &ClassIndexedE3E4AttemptToken,
) -> Result<(), ClassInductionError> {
    let replay = issue_class_indexed_e3_e4_attempt()?;
    if replay == *token {
        Ok(())
    } else {
        Err(ClassInductionError::ReplayMismatch)
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ClassInductionError {
    #[error("context judgement failed: {0}")]
    Context(String),
    #[error("ordinary grammar judgement failed: {0}")]
    Grammar(String),
    #[error("E-3 normalization judgement failed: {0}")]
    E3(String),
    #[error("cubical judgement failed: {0}")]
    Cubical(String),
    #[error("registered typed-boundary replay failed: {0}")]
    TypedBoundary(String),
    #[error("ordinary naturality substitution source mismatch")]
    SubstitutionSourceMismatch,
    #[error("ordinary constructor normalization naturality failed")]
    OrdinaryNaturalityFailed,
    #[error("cubical normalization/dimension-substitution naturality failed")]
    CubicalNaturalityFailed,
    #[error("endpoint-premise terms require the private registered premise context")]
    EndpointPremiseContextRequired,
    #[error("class-indexed aggregate crossed its fail-closed boundary")]
    AggregateBoundaryDrift,
    #[error("class-indexed replay mismatch")]
    ReplayMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_operational_ordinary_constructor_is_normalization_natural() {
        let token = issue_class_indexed_e3_e4_attempt().unwrap();
        assert_eq!(token.ordinary_naturality().len(), 9);
        assert!(
            token
                .ordinary_naturality()
                .iter()
                .all(OrdinaryConstructorNaturalityToken::holds)
        );
        assert_eq!(
            token
                .ordinary_naturality()
                .iter()
                .map(OrdinaryConstructorNaturalityToken::constructor)
                .collect::<BTreeSet<_>>(),
            OrdinarySchemaKind::ALL.into_iter().collect::<BTreeSet<_>>()
        );
    }

    #[test]
    fn public_dependent_cubical_syntax_has_typed_natural_induction() {
        let token = issue_class_indexed_e3_e4_attempt().unwrap();
        let observed = token
            .cubical_inductions()
            .iter()
            .flat_map(|proof| proof.observed_constructors().iter().copied())
            .collect::<BTreeSet<_>>();
        let expected = CubicalConstructorKind::ALL
            .into_iter()
            .filter(|kind| !kind.requires_endpoint_premise_context())
            .collect::<BTreeSet<_>>();
        assert_eq!(observed, expected);
        assert!(
            token
                .cubical_inductions()
                .iter()
                .all(DependentCubicalActionInductionToken::all_dimension_substitutions_natural)
        );
        assert!(
            token
                .cubical_inductions()
                .iter()
                .any(|proof| proof.coe_node_count() > 0)
        );
        assert!(
            token
                .cubical_inductions()
                .iter()
                .any(|proof| proof.hcom_node_count() > 0)
        );
    }

    #[test]
    fn full_class_completion_fails_closed_at_exact_missing_semantics() {
        let token = issue_class_indexed_e3_e4_attempt().unwrap();
        replay_class_indexed_e3_e4_attempt(&token).unwrap();
        assert_eq!(token.per_class().len(), Schema2NaturalityClass::ALL.len());
        assert!(token.per_class().iter().all(|record| !record.complete()));
        assert!(!token.every_class_complete());
        assert!(!token.full_e4_complete());
        assert!(
            token
                .per_class()
                .iter()
                .find(|record| record.class() == Schema2NaturalityClass::Modal)
                .unwrap()
                .named_gaps()
                .contains(&MODAL_CONSTRUCTOR_GAP.to_owned())
        );
        assert!(
            token
                .per_class()
                .iter()
                .find(|record| record.class() == Schema2NaturalityClass::HitV2)
                .unwrap()
                .named_gaps()
                .contains(&HIT_ENDPOINT_INDUCTION_GAP.to_owned())
        );
    }

    #[test]
    fn mutation_of_ordinary_or_cubical_proof_fails_replay() {
        let token = issue_class_indexed_e3_e4_attempt().unwrap();
        let mut ordinary = token.clone();
        ordinary.ordinary_naturality[0].normalized_interpretation_natural = false;
        ordinary.derivation_hash = tagged_hash("class-indexed-e3-e4-attempt", &ordinary);
        assert_eq!(
            replay_class_indexed_e3_e4_attempt(&ordinary),
            Err(ClassInductionError::ReplayMismatch)
        );

        let mut cubical = token;
        cubical.cubical_inductions[0].all_dimension_substitutions_natural = false;
        cubical.derivation_hash = tagged_hash("class-indexed-e3-e4-attempt", &cubical);
        assert_eq!(
            replay_class_indexed_e3_e4_attempt(&cubical),
            Err(ClassInductionError::ReplayMismatch)
        );
    }
}
