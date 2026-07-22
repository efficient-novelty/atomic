//! Replayable E-2 ordinary-schema realizer tokens.
//!
//! A token in this module proves that a schema constructor, its E-1 types,
//! depth bound, support window, and count-blind quotient disposition replay.
//! It intentionally does **not** prove E-3 normalization, E-4 generator
//! completeness, E-5 demand liveness, E-6 marginality, or the global C1
//! `pen_core::Expr` bridge.

use crate::grammar::{
    E3_NORMAL_FORM_GAP, E4_GENERATOR_MEMBERSHIP_GAP, FORMATION_COMPLETION_PACKAGE_RULE,
    FamilyDisposition, FamilyPresentation, GLOBAL_C1_BRIDGE_GAP, OrdinaryGrammarError,
    OrdinarySchema, OrdinarySchemaKind, QuotientRule, ordinary_constructor_descriptor,
    replay_ordinary_schema,
};
use serde::Serialize;
use thiserror::Error;

pub const ORDINARY_REALIZER_TOKEN_VERSION: &str = "schema2-ordinary-realizer-e2-v1";
pub const STAGE1_R1_SOURCE_SHAPE_GAP: &str =
    "E2_STAGE1_UNIV_APP_SOURCE_SHAPE_NOT_REPRESENTED_BY_GENERIC_E1_TYPEEXPR";
pub const GENERIC_R1_PACKAGE_DEPENDENCY_GAP: &str =
    "E2_GENERIC_FORMATION_COMPLETION_SAME_PACKAGE_DEPENDENCY_NOT_PROVED";

/// What E-2 can decide about an otherwise well-typed ordinary realizer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RealizerFamilyVerdict {
    FormationCarrierProvenance {
        package_rule: String,
        carrier_exception_pending_e4: bool,
    },
    StructuralNaturalFamilySchema,
    UniformInstanceMembershipPending {
        parent_family_derivation: String,
        gap: String,
    },
    ExportedDemandOutputPendingE5 {
        orbit_derivation: String,
        required_output_position: u32,
    },
    GeneratorMembershipPendingE4 {
        rule: String,
    },
}

impl RealizerFamilyVerdict {
    pub const fn independent_family_decided(&self) -> bool {
        matches!(self, Self::StructuralNaturalFamilySchema)
    }

    pub const fn pending_generator_membership(&self) -> bool {
        matches!(self, Self::GeneratorMembershipPendingE4 { .. })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrdinaryTypedRealizerToken {
    token_version: String,
    schema: OrdinarySchema,
    constructor_registry_digest: String,
    semantic_interpretation_digest: String,
    family_verdict: RealizerFamilyVerdict,
    formed_context_replayed: bool,
    typing_rule_replayed: bool,
    depth_two_replayed: bool,
    support_window_replayed: bool,
    count_inputs_used: bool,
    global_c1_bridge_retired: bool,
    global_c1_gap: String,
    frozen_normal_form_proved: bool,
    frozen_normal_form_gap: String,
    marginality_proved: bool,
    derivation_hash: String,
}

impl OrdinaryTypedRealizerToken {
    pub fn schema(&self) -> &OrdinarySchema {
        &self.schema
    }

    pub const fn constructor(&self) -> OrdinarySchemaKind {
        self.schema.constructor()
    }

    pub fn family_verdict(&self) -> &RealizerFamilyVerdict {
        &self.family_verdict
    }

    pub const fn count_inputs_used(&self) -> bool {
        self.count_inputs_used
    }

    pub const fn global_c1_bridge_retired(&self) -> bool {
        self.global_c1_bridge_retired
    }

    pub const fn marginality_proved(&self) -> bool {
        self.marginality_proved
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PackageCoverageProof {
    carrier_schema_derivation: String,
    completion_schema_derivation: String,
    same_parameter_context: bool,
    same_support_window: bool,
    same_carrier_type: bool,
    same_package_dependency_proved: bool,
    completion_covers_carrier: bool,
    derivation_hash: String,
}

impl PackageCoverageProof {
    pub const fn completion_covers_carrier(&self) -> bool {
        self.completion_covers_carrier
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// Fail-closed audit of a proposed generic formation/completion pair.  The
/// actual Stage-1 R1 issuer is `stage1_r1`; this generic token does not infer
/// a same-package dependency from a shared carrier type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FormationCompletionPackageToken {
    token_version: String,
    rule: String,
    carrier: OrdinaryTypedRealizerToken,
    completion: OrdinaryTypedRealizerToken,
    coverage: PackageCoverageProof,
    canonical_representative_schema: String,
    package_members_typed: bool,
    package_family_typed: bool,
    same_package_dependency_proved: bool,
    same_package_dependency_gap: String,
    separate_carrier_family_issued: bool,
    carrier_exception_generator_membership_decided: bool,
    carrier_exception_gap: String,
    frozen_normal_form_proved: bool,
    frozen_normal_form_gap: String,
    stage1_univ_app_source_shape_replayed: bool,
    stage1_source_shape_gap: String,
    count_inputs_used: bool,
    derivation_hash: String,
}

impl FormationCompletionPackageToken {
    pub fn carrier(&self) -> &OrdinaryTypedRealizerToken {
        &self.carrier
    }

    pub fn completion(&self) -> &OrdinaryTypedRealizerToken {
        &self.completion
    }

    pub fn coverage(&self) -> &PackageCoverageProof {
        &self.coverage
    }

    pub const fn package_family_typed(&self) -> bool {
        self.package_family_typed
    }

    pub const fn package_members_typed(&self) -> bool {
        self.package_members_typed
    }

    pub const fn same_package_dependency_proved(&self) -> bool {
        self.same_package_dependency_proved
    }

    pub const fn separate_carrier_family_issued(&self) -> bool {
        self.separate_carrier_family_issued
    }

    pub const fn carrier_exception_generator_membership_decided(&self) -> bool {
        self.carrier_exception_generator_membership_decided
    }

    pub const fn count_inputs_used(&self) -> bool {
        self.count_inputs_used
    }

    /// Generic R1 packages do not stand in for the actual
    /// `Univ` / `App(Univ, Var(1))` Stage-1 source pair.
    pub const fn stage1_univ_app_source_shape_replayed(&self) -> bool {
        self.stage1_univ_app_source_shape_replayed
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum OrdinaryRealizerError {
    #[error(transparent)]
    Grammar(#[from] OrdinaryGrammarError),
    #[error("ordinary realizer replay mismatch")]
    ReplayMismatch,
    #[error("R1 package carrier must be FreshFormation")]
    PackageCarrierIsNotFormation,
    #[error("R1 package completion cannot itself be FreshFormation")]
    PackageCompletionIsFormation,
    #[error("R1 package members do not share a parameter context")]
    PackageContextMismatch,
    #[error("R1 package members do not share a support window")]
    PackageWindowMismatch,
    #[error("R1 package completion is not typed over the carrier")]
    PackageCarrierTypeMismatch,
    #[error("R1 completion must be a canonical structural family before packaging")]
    PackageCompletionDispositionUndecided,
    #[error("R1 package replay mismatch")]
    PackageReplayMismatch,
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(ORDINARY_REALIZER_TOKEN_VERSION, domain, payload))
        .expect("ordinary realizer proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn verdict_for(schema: &OrdinarySchema) -> RealizerFamilyVerdict {
    match schema.family_disposition() {
        FamilyDisposition::FormationCarrier { package_rule, .. } => {
            RealizerFamilyVerdict::FormationCarrierProvenance {
                package_rule: package_rule.clone(),
                carrier_exception_pending_e4: true,
            }
        }
        FamilyDisposition::StructuralNaturalFamily => {
            RealizerFamilyVerdict::StructuralNaturalFamilySchema
        }
        FamilyDisposition::UniformInstancePending { parent_family, gap } => {
            RealizerFamilyVerdict::UniformInstanceMembershipPending {
                parent_family_derivation: parent_family.as_str().to_owned(),
                gap: gap.clone(),
            }
        }
        FamilyDisposition::ExportedDemandOutputPending {
            live_orbit,
            required_output_position,
            ..
        } => RealizerFamilyVerdict::ExportedDemandOutputPendingE5 {
            orbit_derivation: live_orbit.as_str().to_owned(),
            required_output_position: *required_output_position,
        },
        FamilyDisposition::GeneratorMembershipPending { rule, .. } => {
            RealizerFamilyVerdict::GeneratorMembershipPendingE4 { rule: rule.clone() }
        }
    }
}

pub fn issue_ordinary_typed_realizer(
    schema: OrdinarySchema,
) -> Result<OrdinaryTypedRealizerToken, OrdinaryRealizerError> {
    replay_ordinary_schema(&schema)?;
    let descriptor = ordinary_constructor_descriptor(schema.constructor());
    let constructor_registry_digest = tagged_hash("constructor-registry", &descriptor);
    let semantic_interpretation_digest =
        tagged_hash("semantic-interpretation", schema.interpretation());
    let family_verdict = verdict_for(&schema);
    let formed_context_replayed = true;
    let typing_rule_replayed = true;
    let depth_two_replayed = schema.depth_proof().within_bound();
    let support_window_replayed = schema.support_proof().every_anchor_is_local()
        && schema.support_proof().has_current_generator();
    let count_inputs_used = false;
    let global_c1_bridge_retired = false;
    let global_c1_gap = GLOBAL_C1_BRIDGE_GAP.to_owned();
    let frozen_normal_form_proved = false;
    let frozen_normal_form_gap = E3_NORMAL_FORM_GAP.to_owned();
    let marginality_proved = false;
    let derivation_hash = tagged_hash(
        "ordinary-typed-realizer",
        &(
            &schema,
            &constructor_registry_digest,
            &semantic_interpretation_digest,
            &family_verdict,
            formed_context_replayed,
            typing_rule_replayed,
            depth_two_replayed,
            support_window_replayed,
            count_inputs_used,
            global_c1_bridge_retired,
            &global_c1_gap,
            frozen_normal_form_proved,
            &frozen_normal_form_gap,
            marginality_proved,
        ),
    );
    Ok(OrdinaryTypedRealizerToken {
        token_version: ORDINARY_REALIZER_TOKEN_VERSION.to_owned(),
        schema,
        constructor_registry_digest,
        semantic_interpretation_digest,
        family_verdict,
        formed_context_replayed,
        typing_rule_replayed,
        depth_two_replayed,
        support_window_replayed,
        count_inputs_used,
        global_c1_bridge_retired,
        global_c1_gap,
        frozen_normal_form_proved,
        frozen_normal_form_gap,
        marginality_proved,
        derivation_hash,
    })
}

pub fn replay_ordinary_typed_realizer(
    token: &OrdinaryTypedRealizerToken,
) -> Result<(), OrdinaryRealizerError> {
    let replay = issue_ordinary_typed_realizer(token.schema.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(OrdinaryRealizerError::ReplayMismatch)
    }
}

pub fn issue_formation_completion_package(
    carrier_schema: OrdinarySchema,
    completion_schema: OrdinarySchema,
) -> Result<FormationCompletionPackageToken, OrdinaryRealizerError> {
    if carrier_schema.constructor() != OrdinarySchemaKind::FreshFormation {
        return Err(OrdinaryRealizerError::PackageCarrierIsNotFormation);
    }
    if completion_schema.constructor() == OrdinarySchemaKind::FreshFormation {
        return Err(OrdinaryRealizerError::PackageCompletionIsFormation);
    }
    if carrier_schema.context() != completion_schema.context() {
        return Err(OrdinaryRealizerError::PackageContextMismatch);
    }
    if carrier_schema.support_proof().window() != completion_schema.support_proof().window() {
        return Err(OrdinaryRealizerError::PackageWindowMismatch);
    }
    if carrier_schema.interpretation().carrier() != completion_schema.interpretation().carrier() {
        return Err(OrdinaryRealizerError::PackageCarrierTypeMismatch);
    }
    if !matches!(
        completion_schema.presentation(),
        FamilyPresentation::CanonicalFamily
    ) || ordinary_constructor_descriptor(completion_schema.constructor()).quotient_rule
        != QuotientRule::StructuralNaturalFamily
    {
        return Err(OrdinaryRealizerError::PackageCompletionDispositionUndecided);
    }

    let carrier = issue_ordinary_typed_realizer(carrier_schema)?;
    let completion = issue_ordinary_typed_realizer(completion_schema)?;
    let same_parameter_context = carrier.schema.context() == completion.schema.context();
    let same_support_window =
        carrier.schema.support_proof().window() == completion.schema.support_proof().window();
    let same_carrier_type =
        carrier.schema.interpretation().carrier() == completion.schema.interpretation().carrier();
    let same_package_dependency_proved = false;
    let same_package_dependency_gap = GENERIC_R1_PACKAGE_DEPENDENCY_GAP.to_owned();
    let completion_covers_carrier = same_parameter_context
        && same_support_window
        && same_carrier_type
        && same_package_dependency_proved;
    let coverage_derivation_hash = tagged_hash(
        "formation-completion-coverage",
        &(
            carrier.derivation_hash(),
            completion.derivation_hash(),
            same_parameter_context,
            same_support_window,
            same_carrier_type,
            same_package_dependency_proved,
            completion_covers_carrier,
        ),
    );
    let coverage = PackageCoverageProof {
        carrier_schema_derivation: carrier.derivation_hash().to_owned(),
        completion_schema_derivation: completion.derivation_hash().to_owned(),
        same_parameter_context,
        same_support_window,
        same_carrier_type,
        same_package_dependency_proved,
        completion_covers_carrier,
        derivation_hash: coverage_derivation_hash,
    };
    let rule = FORMATION_COMPLETION_PACKAGE_RULE.to_owned();
    let canonical_representative_schema = completion.schema.provisional_schema_id().to_owned();
    let package_members_typed = true;
    let package_family_typed = false;
    let separate_carrier_family_issued = false;
    let carrier_exception_generator_membership_decided = false;
    let carrier_exception_gap = E4_GENERATOR_MEMBERSHIP_GAP.to_owned();
    let frozen_normal_form_proved = false;
    let frozen_normal_form_gap = E3_NORMAL_FORM_GAP.to_owned();
    let stage1_univ_app_source_shape_replayed = false;
    let stage1_source_shape_gap = STAGE1_R1_SOURCE_SHAPE_GAP.to_owned();
    let count_inputs_used = false;
    let derivation_hash = tagged_hash(
        "formation-completion-package",
        &(
            &rule,
            &carrier,
            &completion,
            &coverage,
            &canonical_representative_schema,
            (
                package_members_typed,
                package_family_typed,
                same_package_dependency_proved,
                &same_package_dependency_gap,
                separate_carrier_family_issued,
                carrier_exception_generator_membership_decided,
                &carrier_exception_gap,
                frozen_normal_form_proved,
                &frozen_normal_form_gap,
                stage1_univ_app_source_shape_replayed,
                &stage1_source_shape_gap,
                count_inputs_used,
            ),
        ),
    );
    Ok(FormationCompletionPackageToken {
        token_version: ORDINARY_REALIZER_TOKEN_VERSION.to_owned(),
        rule,
        carrier,
        completion,
        coverage,
        canonical_representative_schema,
        package_members_typed,
        package_family_typed,
        same_package_dependency_proved,
        same_package_dependency_gap,
        separate_carrier_family_issued,
        carrier_exception_generator_membership_decided,
        carrier_exception_gap,
        frozen_normal_form_proved,
        frozen_normal_form_gap,
        stage1_univ_app_source_shape_replayed,
        stage1_source_shape_gap,
        count_inputs_used,
        derivation_hash,
    })
}

pub fn replay_formation_completion_package(
    token: &FormationCompletionPackageToken,
) -> Result<(), OrdinaryRealizerError> {
    replay_ordinary_typed_realizer(&token.carrier)?;
    replay_ordinary_typed_realizer(&token.completion)?;
    let replay = issue_formation_completion_package(
        token.carrier.schema.clone(),
        token.completion.schema.clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(OrdinaryRealizerError::PackageReplayMismatch)
    }
}

/// This predicate is the handoff boundary to E-4.  It never treats a typed
/// R2 signature as an independent family merely because a token exists.
pub fn requires_e4_generator_membership(token: &OrdinaryTypedRealizerToken) -> bool {
    token.family_verdict.pending_generator_membership()
        || matches!(
            token.family_verdict,
            RealizerFamilyVerdict::FormationCarrierProvenance {
                carrier_exception_pending_e4: true,
                ..
            }
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{
        BinderId, Declaration, FormedSchemaContext, TermExpr, TypeExpr, form_schema_context,
    };
    use crate::grammar::{
        ClauseAnchor, DERIVED_ACTION_MEMBERSHIP_RULE, DerivationRef, FamilyPresentation,
        OrdinaryInterpretation, SupportWindow, UnitOrientation, form_ordinary_schema,
    };

    fn context() -> FormedSchemaContext {
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
        .unwrap()
    }

    fn digest(byte: char) -> DerivationRef {
        DerivationRef::parse(format!("blake3:{}", byte.to_string().repeat(64))).unwrap()
    }

    fn formed(kind: OrdinarySchemaKind, interpretation: OrdinaryInterpretation) -> OrdinarySchema {
        form_ordinary_schema(
            context(),
            kind,
            interpretation,
            FamilyPresentation::CanonicalFamily,
            SupportWindow::new(7, 8).unwrap(),
            vec![ClauseAnchor { step: 8, clause: 0 }],
        )
        .unwrap()
    }

    fn interpretations() -> Vec<(OrdinarySchemaKind, OrdinaryInterpretation)> {
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

    #[test]
    fn every_generic_ordinary_constructor_has_a_replayable_typed_realizer() {
        for (kind, interpretation) in interpretations() {
            let token = issue_ordinary_typed_realizer(formed(kind, interpretation)).unwrap();
            replay_ordinary_typed_realizer(&token).unwrap();
            assert_eq!(token.constructor(), kind);
            assert!(!token.count_inputs_used());
            assert!(!token.global_c1_bridge_retired());
            assert!(!token.marginality_proved());
        }
    }

    #[test]
    fn r2_coherence_and_cell_tokens_do_not_decide_independence() {
        for (kind, interpretation) in interpretations().into_iter().filter(|(kind, _)| {
            matches!(
                kind,
                OrdinarySchemaKind::PostPathCoherence | OrdinarySchemaKind::CellAction
            )
        }) {
            let token = issue_ordinary_typed_realizer(formed(kind, interpretation)).unwrap();
            assert!(requires_e4_generator_membership(&token));
            assert_eq!(
                token.family_verdict(),
                &RealizerFamilyVerdict::GeneratorMembershipPendingE4 {
                    rule: DERIVED_ACTION_MEMBERSHIP_RULE.to_owned()
                }
            );
        }
    }

    #[test]
    fn generic_r1_pair_stays_pending_without_same_package_dependency() {
        let carrier = formed(
            OrdinarySchemaKind::FreshFormation,
            OrdinaryInterpretation::FreshFormation {
                carrier: TypeExpr::parameter(0),
            },
        );
        let completion = formed(
            OrdinarySchemaKind::PointOrUnitIntro,
            OrdinaryInterpretation::PointOrUnitIntro {
                carrier: TypeExpr::parameter(0),
                point: TermExpr::variable(2),
            },
        );
        let token = issue_formation_completion_package(carrier, completion).unwrap();
        replay_formation_completion_package(&token).unwrap();
        assert!(token.package_members_typed());
        assert!(!token.package_family_typed());
        assert!(!token.same_package_dependency_proved());
        assert!(!token.coverage().completion_covers_carrier());
        assert!(!token.separate_carrier_family_issued());
        assert!(!token.carrier_exception_generator_membership_decided());
        assert!(!token.stage1_univ_app_source_shape_replayed());
        assert!(!token.count_inputs_used());
    }

    #[test]
    fn r1_rejects_mismatched_carriers_and_pending_r2_completion() {
        let carrier = formed(
            OrdinarySchemaKind::FreshFormation,
            OrdinaryInterpretation::FreshFormation {
                carrier: TypeExpr::parameter(0),
            },
        );
        let mismatch = formed(
            OrdinarySchemaKind::Recursor,
            OrdinaryInterpretation::Recursor {
                carrier: TypeExpr::parameter(1),
                codomain: TypeExpr::parameter(0),
            },
        );
        assert_eq!(
            issue_formation_completion_package(carrier.clone(), mismatch),
            Err(OrdinaryRealizerError::PackageCarrierTypeMismatch)
        );
        let pending = formed(
            OrdinarySchemaKind::CellAction,
            OrdinaryInterpretation::CellAction {
                carrier: TypeExpr::parameter(0),
                operation: digest('a'),
                cell_dimension: 3,
                registered_boundary_bundle: digest('b'),
            },
        );
        assert_eq!(
            issue_formation_completion_package(carrier, pending),
            Err(OrdinaryRealizerError::PackageCompletionDispositionUndecided)
        );
    }

    #[test]
    fn token_mutations_fail_replay_even_after_rehash_is_not_available_to_callers() {
        let schema = formed(
            OrdinarySchemaKind::PostPathOperation,
            OrdinaryInterpretation::PostPathOperation {
                carrier: TypeExpr::parameter(0),
                arity: 2,
            },
        );
        let token = issue_ordinary_typed_realizer(schema).unwrap();
        let mut changed_rule = token.clone();
        changed_rule
            .constructor_registry_digest
            .push_str("-changed");
        assert_eq!(
            replay_ordinary_typed_realizer(&changed_rule),
            Err(OrdinaryRealizerError::ReplayMismatch)
        );
        let mut invented_count_input = token;
        invented_count_input.count_inputs_used = true;
        assert_eq!(
            replay_ordinary_typed_realizer(&invented_count_input),
            Err(OrdinaryRealizerError::ReplayMismatch)
        );
    }
}
