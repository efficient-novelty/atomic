//! Versioned repair of the future-hole definition.
//!
//! Unlike the frozen v1 artifact, this layer binds the body and declared
//! motives at A3-scheme registration time.  Registration and realization
//! are total over semantic failures: an unsupported case is a named gap,
//! while `Err` is reserved for malformed A3 associations or replay damage.

use crate::a3_demand_grammar::{
    A3DemandConstructor, A3DemandOutputType, A3DemandSchemeOrigin, A3HistoricalDemandGrammar,
    A3HistoricalWindow, A3RuleConstructor, A3TypedClauseSource, A3TypedDemandInstance,
    A3TypedDemandScheme, generate_a3_window_for_exact_prefix_unbounded,
};
use crate::typed_families::ParamSort;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_type::admissibility::{
    AdmissibilityDecision, AdmissibilityMode, PackagePolicy, StrictAdmissibility, StructuralFamily,
    assess_strict_admissibility, strict_admissibility,
};
use pen_type::ambient_former_internality::{TransparentFormer, registered_transparent_formers};
use pen_type::contextual_internality::{
    ContextualMotive, ExplicitAmbientContextDeclarationToken,
    issue_explicit_ambient_context_declaration_token,
};
use pen_type::dependent_context::{
    DependentAmbientContextDeclarationProjection, DependentAmbientContextDeclarationToken,
    DependentContextMotive, DependentTotalSpecializationProjection,
    DependentTotalSpecializationToken, issue_dependent_ambient_context_declaration,
    issue_dependent_total_specialization_theorem, issue_sealed_prior_clause_type_reference,
    replay_dependent_ambient_context_declaration, replay_dependent_total_specialization_theorem,
};
use pen_type::elaborate::{
    KernelTy, SealedSignature, SingleClauseElaboration, candidate_hash, elaborate_telescope,
};
use pen_type::motive_parametric_coherence_v2::{
    CLOSURE_RULE_INVENTORY_V2, MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION,
    SpecializedClosureDerivationV2, VerifiedClosureDerivationV2, issue_closed_internal_evidence_v2,
    issue_exact_explicit_future_body_closure_derivation_v2,
    issue_exact_future_body_closure_derivation_v2, issue_motive_typed_closed_assignment_v2,
    replay_specialized_closure_derivation_v2, replay_verified_closure_derivation_v2,
    specialize_verified_closure_projection_v2,
};
use serde::Serialize;
use thiserror::Error;

pub const FUTURE_HOLE_HYPOTHESIS_V2_SCHEMA: &str = "future-hole-body-motive-registration-v2";
pub const GUARDED_PROVIDER_REGISTRY_V2: &str =
    "pen-type-guarded-strict-admissibility-provider-registry-v1";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/future_hole_definition_adjudication.md");
const ADMISSIBILITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/admissibility.rs");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(FUTURE_HOLE_HYPOTHESIS_V2_SCHEMA, domain, value))
        .expect("future-hole v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FutureHoleGapPhaseV2 {
    Registration,
    Realization,
    ChargeJoin,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NamedFutureHoleGapV2 {
    pub id: String,
    pub phase: FutureHoleGapPhaseV2,
    pub a3_scheme_id: String,
    pub a3_instance_id: String,
    pub occurrence_id: Option<String>,
    pub exact_error: String,
    pub input_digest: String,
    pub gap_hash: String,
}

impl NamedFutureHoleGapV2 {
    fn issue(
        id: impl Into<String>,
        phase: FutureHoleGapPhaseV2,
        scheme: &A3TypedDemandScheme,
        instance: &A3TypedDemandInstance,
        occurrence_id: Option<String>,
        exact_error: impl Into<String>,
        input: &impl Serialize,
    ) -> Self {
        let mut gap = Self {
            id: id.into(),
            phase,
            a3_scheme_id: scheme.scheme_id.clone(),
            a3_instance_id: instance.instance_id.clone(),
            occurrence_id,
            exact_error: exact_error.into(),
            input_digest: tagged_hash("gap-input", input),
            gap_hash: String::new(),
        };
        gap.gap_hash = gap_expected_hash(&gap);
        gap
    }

    pub fn replays(&self) -> bool {
        self.gap_hash == gap_expected_hash(self)
    }

    fn from_registration(
        id: impl Into<String>,
        phase: FutureHoleGapPhaseV2,
        registration: &RegisteredFutureHoleV2,
        exact_error: impl Into<String>,
        input: &impl Serialize,
    ) -> Self {
        let mut gap = Self {
            id: id.into(),
            phase,
            a3_scheme_id: registration.a3_scheme_id.clone(),
            a3_instance_id: registration.a3_instance_id.clone(),
            occurrence_id: Some(registration.occurrence_id.clone()),
            exact_error: exact_error.into(),
            input_digest: tagged_hash("gap-input", input),
            gap_hash: String::new(),
        };
        gap.gap_hash = gap_expected_hash(&gap);
        gap
    }
}

fn gap_expected_hash(gap: &NamedFutureHoleGapV2) -> String {
    tagged_hash(
        "named-gap",
        &(
            &gap.id,
            gap.phase,
            &gap.a3_scheme_id,
            &gap.a3_instance_id,
            &gap.occurrence_id,
            &gap.exact_error,
            &gap.input_digest,
        ),
    )
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ZeroMarginalChargeV2 {
    pub marginal_kappa: u32,
    pub marginal_nu: u32,
    pub anchors_minted: u32,
    pub demand_orbits_minted: u32,
    pub credit_minted: bool,
    pub charge_hash: String,
}

impl ZeroMarginalChargeV2 {
    fn issue() -> Self {
        let mut charge = Self {
            marginal_kappa: 0,
            marginal_nu: 0,
            anchors_minted: 0,
            demand_orbits_minted: 0,
            credit_minted: false,
            charge_hash: String::new(),
        };
        charge.charge_hash = zero_charge_expected_hash(&charge);
        charge
    }

    pub fn replays_as_zero(&self) -> bool {
        self.marginal_kappa == 0
            && self.marginal_nu == 0
            && self.anchors_minted == 0
            && self.demand_orbits_minted == 0
            && !self.credit_minted
            && self.charge_hash == zero_charge_expected_hash(self)
    }
}

fn zero_charge_expected_hash(charge: &ZeroMarginalChargeV2) -> String {
    tagged_hash(
        "zero-marginal-charge",
        &(
            charge.marginal_kappa,
            charge.marginal_nu,
            charge.anchors_minted,
            charge.demand_orbits_minted,
            charge.credit_minted,
        ),
    )
}

/// Evidence supplied by an upper layer for the sealed filler's already
/// existing ordinary charge.  Issuing a discharge never zeroes this record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FillerOrdinaryChargeProvenanceV2 {
    pub filler_step: u32,
    pub filler_candidate_hash: String,
    pub filler_telescope_digest: String,
    pub ordinary_kappa: u32,
    pub certified_nu: u32,
    pub bit_length: u32,
    pub upstream_certificate_hash: String,
    pub ordinary_family_token_hashes: Vec<String>,
    pub provenance_hash: String,
}

pub fn issue_filler_ordinary_charge_provenance_v2(
    filler_step: u32,
    filler: &Telescope,
    ordinary_kappa: u32,
    certified_nu: u32,
    bit_length: u32,
    upstream_certificate_hash: impl Into<String>,
    ordinary_family_token_hashes: Vec<String>,
) -> FillerOrdinaryChargeProvenanceV2 {
    let subject = candidate_hash(filler);
    let mut provenance = FillerOrdinaryChargeProvenanceV2 {
        filler_step,
        filler_candidate_hash: subject.clone(),
        filler_telescope_digest: subject,
        ordinary_kappa,
        certified_nu,
        bit_length,
        upstream_certificate_hash: upstream_certificate_hash.into(),
        ordinary_family_token_hashes,
        provenance_hash: String::new(),
    };
    provenance.provenance_hash = filler_charge_expected_hash(&provenance);
    provenance
}

fn filler_charge_expected_hash(charge: &FillerOrdinaryChargeProvenanceV2) -> String {
    tagged_hash(
        "filler-ordinary-charge-provenance",
        &(
            charge.filler_step,
            &charge.filler_candidate_hash,
            &charge.filler_telescope_digest,
            charge.ordinary_kappa,
            charge.certified_nu,
            charge.bit_length,
            &charge.upstream_certificate_hash,
            &charge.ordinary_family_token_hashes,
        ),
    )
}

pub fn replay_filler_ordinary_charge_provenance_v2(
    charge: &FillerOrdinaryChargeProvenanceV2,
) -> bool {
    !charge.upstream_certificate_hash.is_empty()
        && charge.filler_candidate_hash == charge.filler_telescope_digest
        && charge.provenance_hash == filler_charge_expected_hash(charge)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FreshUnaryHoleParameterV2 {
    pub source_parameter: u32,
    pub fresh_parameter: u32,
    pub declared_motive: DependentContextMotive,
    pub occurs_live_in_body: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "body_kind")]
pub enum FutureHoleBodyV2 {
    UnaryAction {
        sealed_operator_family_key: String,
        operator_template: Expr,
        fresh_hole_arguments: Vec<FreshUnaryHoleParameterV2>,
        instantiated_body: Expr,
        fresh_context_hash: String,
        typed_instantiation_hash: String,
    },
    StructuralHypothesis {
        parameter: u32,
        body: Expr,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "jurisdiction")]
pub enum StructuralJurisdictionV2 {
    Direct {
        stage: u32,
    },
    Stage3To4 {
        registration_stage: u32,
        jurisdiction_stage: u32,
        demand_precedes_jurisdiction_evidence_hash: String,
        transport_hash: String,
    },
}

impl StructuralJurisdictionV2 {
    pub fn jurisdiction_stage(&self) -> u32 {
        match self {
            Self::Direct { stage } => *stage,
            Self::Stage3To4 {
                jurisdiction_stage, ..
            } => *jurisdiction_stage,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StructuralProvidesContractV2 {
    pub constructor: A3DemandConstructor,
    pub provider_family: StructuralFamily,
    pub mode: AdmissibilityMode,
    pub required_policy: PackagePolicy,
    pub provider_registry_version: String,
    pub provider_registry_source_hash: String,
    pub jurisdiction: StructuralJurisdictionV2,
    pub guarded_profile: StrictAdmissibility,
    pub live_constructor_evidence_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "contract_kind")]
pub enum FutureHoleOutputContractV2 {
    UnaryActionAt {
        source_family_key: String,
        expected_kernel_type: KernelTy,
        no_reflexivity_rule: bool,
    },
    StructuralProvides(StructuralProvidesContractV2),
}

/// Fail-honest composition used when the legacy registered-probe precheck is
/// unavailable at an early historical prefix.  The generic theorem explicitly
/// records that probes are not evidence; this record supplies the missing
/// exact-body premises rather than pretending a contextual token was issued.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FutureHoleParametricInternalityV2 {
    pub explicit_ambient_arity: u32,
    pub exact_body_candidate_hash: String,
    pub historical_prefix_declaration_hash: String,
    pub typed_body_elaboration_hash: String,
    pub live_parameters: Vec<u32>,
    pub every_declared_parameter_live: bool,
    pub closure_formers: Vec<TransparentFormer>,
    pub registered_former_inventory: Vec<TransparentFormer>,
    pub every_body_former_registered: bool,
    pub eliminator_version: String,
    pub closure_rule_inventory: Vec<String>,
    pub source_derivation: Option<VerifiedClosureDerivationV2>,
    pub source_derivation_replayed: bool,
    pub source_bound_to_exact_body: bool,
    pub dependent_declaration_hash: String,
    pub dependent_totality_theorem: DependentTotalSpecializationProjection,
    pub dependent_totality_theorem_replayed: bool,
    pub total_specialization_authoritative: bool,
    pub hypothetical_internality_issued: bool,
    pub marginal_nu: u32,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RegisteredFutureHoleV2 {
    pub schema: String,
    pub definition_adoption_hash: String,
    pub a3_scheme_id: String,
    pub a3_instance_id: String,
    pub source_anchor_ids: Vec<String>,
    pub registration_stage: u32,
    pub visible_library_at_registration: u32,
    pub ambient_registration_prefix_last_step: u32,
    pub ambient_registration_prefix_signature_digest: String,
    pub ambient_declaration_bound_to_registration_prefix: bool,
    pub natural_family_id: String,
    pub occurrence_id: String,
    pub declared_ambient_arity: u32,
    /// Kept as a serialization-compatible field name for v2 consumers.  Its
    /// value is now the exact body telescope, never a synthetic carrier.
    pub declaration_carrier: Telescope,
    pub body_telescope: Telescope,
    pub explicit_body_elaboration: SingleClauseElaboration,
    pub body: FutureHoleBodyV2,
    pub declared_motives: Vec<DependentContextMotive>,
    pub expected_kernel_type: KernelTy,
    pub output_contract: FutureHoleOutputContractV2,
    pub ambient_declaration: DependentAmbientContextDeclarationProjection,
    pub parametric_internality: FutureHoleParametricInternalityV2,
    pub every_hole_live: bool,
    pub no_reflexivity_fallback: bool,
    pub local_semantic_scope_premises_satisfied: bool,
    /// Clean join point for the independently built A3-exhaustiveness token.
    pub external_exhaustiveness_evidence_hash: Option<String>,
    pub hole_marginal_charge: ZeroMarginalChargeV2,
    pub formation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "registration")]
pub enum FutureHoleRegistrationDispositionV2 {
    Registered(RegisteredFutureHoleV2),
    Gap(NamedFutureHoleGapV2),
}

impl FutureHoleRegistrationDispositionV2 {
    pub fn registered(&self) -> Option<&RegisteredFutureHoleV2> {
        match self {
            Self::Registered(value) => Some(value),
            Self::Gap(_) => None,
        }
    }

    pub fn gap(&self) -> Option<&NamedFutureHoleGapV2> {
        match self {
            Self::Registered(_) => None,
            Self::Gap(value) => Some(value),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StructuralFutureHoleRealizationV2 {
    pub schema: String,
    pub registration_formation_hash: String,
    pub occurrence_id: String,
    pub constructor: A3DemandConstructor,
    pub provider_family: StructuralFamily,
    pub filler_step: u32,
    pub filler_candidate_hash: String,
    pub filler_is_whole_sealed_entry: bool,
    pub filler_elaboration_hash: String,
    pub guarded_profile: StrictAdmissibility,
    pub provider_decision: AdmissibilityDecision,
    pub provider_relation_satisfied: bool,
    pub constructor_live_before_filler: bool,
    pub constructor_absent_after_filler: bool,
    pub extension_declaration_hash: String,
    pub extension_parametric_internality_hash: String,
    pub extension_source_derivation: VerifiedClosureDerivationV2,
    pub prefix_extension_preserved: bool,
    pub closed_assignment_hash: String,
    pub specialization_substitution_hash: String,
    pub specialization: SpecializedClosureDerivationV2,
    pub specialized_expression: Expr,
    pub specialized_expression_is_filler_reference: bool,
    pub clause4_prime_instantiation_replayed: bool,
    pub filler_ordinary_charge: FillerOrdinaryChargeProvenanceV2,
    pub discharge_marginal_charge: ZeroMarginalChargeV2,
    pub realization_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "realization")]
pub enum FutureHoleRealizationDispositionV2 {
    Realized(StructuralFutureHoleRealizationV2),
    Gap(NamedFutureHoleGapV2),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FutureHoleReplayV2 {
    pub valid: bool,
    pub expected_digest: String,
    pub claimed_digest: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum FutureHoleV2Error {
    #[error("A3 scheme/instance association is malformed: {0}")]
    MalformedAssociation(String),
    #[error("future-hole v2 replay mismatch: {0}")]
    ReplayMismatch(String),
}

fn constructor_family(constructor: A3DemandConstructor) -> StructuralFamily {
    match constructor {
        A3DemandConstructor::FormerEliminator => StructuralFamily::FormerEliminator,
        A3DemandConstructor::InitialHit => StructuralFamily::InitialHit,
        A3DemandConstructor::TruncationHit => StructuralFamily::TruncationHit,
        A3DemandConstructor::HigherHit => StructuralFamily::HigherHit,
        A3DemandConstructor::SphereLift => StructuralFamily::SphereLift,
        A3DemandConstructor::AxiomaticBundle => StructuralFamily::AxiomaticBundle,
        A3DemandConstructor::ModalShell => StructuralFamily::ModalShell,
        A3DemandConstructor::ConnectionShell => StructuralFamily::ConnectionShell,
        A3DemandConstructor::CurvatureShell => StructuralFamily::CurvatureShell,
        A3DemandConstructor::OperatorBundle => StructuralFamily::OperatorBundle,
        A3DemandConstructor::HilbertFunctional => StructuralFamily::HilbertFunctional,
        A3DemandConstructor::TemporalShell => StructuralFamily::TemporalShell,
    }
}

fn library_prefix(
    signature: &SealedSignature,
    visible_steps: u32,
    require_exact: bool,
) -> Result<Library, String> {
    if require_exact && signature.len() as u32 != visible_steps {
        return Err(format!(
            "expected an exact {visible_steps}-entry prefix, found {} entries",
            signature.len()
        ));
    }
    let available = signature
        .entries()
        .iter()
        .filter(|entry| entry.step <= visible_steps)
        .map(|entry| entry.step)
        .collect::<Vec<_>>();
    let expected = (1..=visible_steps).collect::<Vec<_>>();
    if available != expected {
        return Err(format!(
            "prefix inventory mismatch: expected {expected:?}, found {available:?}"
        ));
    }
    let mut library = Library::new();
    for step in 1..=visible_steps {
        let entry = signature
            .entry(step)
            .ok_or_else(|| format!("missing prefix entry {step}"))?;
        library.push(LibraryEntry::from_telescope(&entry.telescope, &library));
    }
    Ok(library)
}

fn extended_signature(
    prefix: &SealedSignature,
    filler_step: u32,
    filler: &Telescope,
) -> Result<SealedSignature, String> {
    if prefix.len() as u32 != filler_step.saturating_sub(1) {
        return Err(format!(
            "filler {filler_step} requires an exact {}-entry prefix, found {}",
            filler_step.saturating_sub(1),
            prefix.len()
        ));
    }
    let mut telescopes = prefix
        .entries()
        .iter()
        .map(|entry| (entry.step, entry.telescope.clone()))
        .collect::<Vec<_>>();
    telescopes.push((filler_step, filler.clone()));
    Ok(SealedSignature::from_telescopes(telescopes))
}

fn source_for_instance<'a>(
    window: &'a A3HistoricalWindow,
    instance: &A3TypedDemandInstance,
) -> Result<&'a A3TypedClauseSource, FutureHoleV2Error> {
    if instance.source_anchor_ids.len() != 1 {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "unary instance {} has {} source anchors",
            instance.instance_id,
            instance.source_anchor_ids.len()
        )));
    }
    window
        .typed_sources
        .iter()
        .find(|source| source.anchor_id == instance.source_anchor_ids[0])
        .ok_or_else(|| {
            FutureHoleV2Error::MalformedAssociation(format!(
                "source anchor {} is absent from Stage {}",
                instance.source_anchor_ids[0], window.stage
            ))
        })
}

fn check_scheme_instance(
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
) -> Result<(), FutureHoleV2Error> {
    if instance.scheme_id != scheme.scheme_id {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "instance {} names scheme {}, not {}",
            instance.instance_id, instance.scheme_id, scheme.scheme_id
        )));
    }
    Ok(())
}

fn registration_expected_hash(value: &RegisteredFutureHoleV2) -> String {
    let mut projection = value.clone();
    projection.formation_hash.clear();
    tagged_hash("registered-open-judgment", &projection)
}

fn exact_signature_prefix_through(
    signature: &SealedSignature,
    last_step: u32,
) -> Result<SealedSignature, String> {
    let mut telescopes = Vec::new();
    for step in 1..=last_step {
        let entry = signature.entry(step).ok_or_else(|| {
            format!(
                "signature omits Step {step}, required by the exact registration prefix through Step {last_step}"
            )
        })?;
        telescopes.push((step, entry.telescope.clone()));
    }
    Ok(SealedSignature::from_telescopes(telescopes))
}

fn collect_live_parameters(expression: &Expr, ambient_arity: u32, output: &mut Vec<u32>) {
    match expression {
        // Variables are one-based de Bruijn levels in this calculus:
        // ambient parameters remain levels `1..=ambient_arity` beneath a
        // binder, while newly bound variables occupy larger levels.  Thus
        // `Lam(Var(1))` is the ambient constant projection and
        // `Lam(Var(2))` is the identity in a one-parameter context.
        Expr::Var(level) if *level >= 1 && *level <= ambient_arity => output.push(*level),
        Expr::Var(_) | Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => {}
        Expr::App(left, right) => {
            collect_live_parameters(left, ambient_arity, output);
            collect_live_parameters(right, ambient_arity, output);
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            collect_live_parameters(domain, ambient_arity, output);
            collect_live_parameters(codomain, ambient_arity, output);
        }
        Expr::Lam(body) => {
            collect_live_parameters(body, ambient_arity, output);
        }
        Expr::Id(ty, left, right) => {
            collect_live_parameters(ty, ambient_arity, output);
            collect_live_parameters(left, ambient_arity, output);
            collect_live_parameters(right, ambient_arity, output);
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
        | Expr::WhyNot(inner) => {
            collect_live_parameters(inner, ambient_arity, output);
        }
    }
}

fn collect_closure_formers(
    expression: &Expr,
    output: &mut Vec<TransparentFormer>,
) -> Result<(), &'static str> {
    let (former, children): (TransparentFormer, Vec<&Expr>) = match expression {
        Expr::Univ => (TransparentFormer::AmbientUniverse, vec![]),
        Expr::Var(_) => (TransparentFormer::VariableReference, vec![]),
        Expr::Lib(_) => (TransparentFormer::SealedLibraryConstant, vec![]),
        Expr::Lam(body) => (TransparentFormer::LambdaIntroduction, vec![body]),
        Expr::App(left, right) => (TransparentFormer::Application, vec![left, right]),
        Expr::Pi(domain, codomain) => (TransparentFormer::PiFormation, vec![domain, codomain]),
        Expr::Sigma(domain, codomain) => {
            (TransparentFormer::SigmaFormation, vec![domain, codomain])
        }
        Expr::Id(ty, left, right) => (TransparentFormer::IdentityFormation, vec![ty, left, right]),
        Expr::Refl(inner) => (TransparentFormer::ReflexivityIntroduction, vec![inner]),
        Expr::Susp(inner) => (TransparentFormer::SuspensionFormation, vec![inner]),
        Expr::Trunc(inner) => (TransparentFormer::TruncationFormation, vec![inner]),
        Expr::Flat(inner) => (TransparentFormer::FlatFormation, vec![inner]),
        Expr::Sharp(inner) => (TransparentFormer::SharpFormation, vec![inner]),
        Expr::Disc(inner) => (TransparentFormer::DiscreteFormation, vec![inner]),
        Expr::Shape(inner) => (TransparentFormer::ShapeFormation, vec![inner]),
        Expr::Next(inner) => (TransparentFormer::NextFormation, vec![inner]),
        Expr::Eventually(inner) => (TransparentFormer::EventuallyFormation, vec![inner]),
        Expr::PathCon(_) => return Err("PathCon remains charged outside transparent closure"),
        Expr::Bang(_) | Expr::WhyNot(_) => {
            return Err("linear-exponential former is outside the frozen transparent closure");
        }
    };
    output.push(former);
    for child in children {
        collect_closure_formers(child, output)?;
    }
    Ok(())
}

fn parametric_evidence_expected_hash(value: &FutureHoleParametricInternalityV2) -> String {
    let mut projection = value.clone();
    projection.evidence_hash.clear();
    tagged_hash("future-hole-parametric-internality", &projection)
}

fn issue_fail_honest_parametric_evidence(
    signature: &SealedSignature,
    visible_library: u32,
    body_telescope: &Telescope,
    elaboration_hash: &str,
    declaration: &DependentAmbientContextDeclarationToken,
    totality: &DependentTotalSpecializationToken,
    legacy_declaration: Option<&ExplicitAmbientContextDeclarationToken>,
) -> Result<FutureHoleParametricInternalityV2, String> {
    replay_dependent_ambient_context_declaration(signature, declaration.projection())
        .map_err(|error| error.to_string())?;
    replay_dependent_total_specialization_theorem(
        signature,
        declaration.projection(),
        totality.projection(),
    )
    .map_err(|error| error.to_string())?;
    let ambient_arity = declaration.projection().declared_arity;
    let mut live_parameters = Vec::new();
    for clause in &body_telescope.clauses {
        collect_live_parameters(&clause.expr, ambient_arity, &mut live_parameters);
    }
    live_parameters.sort_unstable();
    live_parameters.dedup();
    let every_declared_parameter_live = live_parameters == (1..=ambient_arity).collect::<Vec<_>>();
    if !every_declared_parameter_live {
        return Err(format!(
            "de-Bruijn-level live-use audit found {live_parameters:?}, expected parameters 1..={ambient_arity}"
        ));
    }
    let mut closure_formers = Vec::new();
    for clause in &body_telescope.clauses {
        collect_closure_formers(&clause.expr, &mut closure_formers).map_err(str::to_owned)?;
    }
    let registered_former_inventory = registered_transparent_formers();
    let every_body_former_registered = closure_formers
        .iter()
        .all(|former| registered_former_inventory.contains(former));
    if !every_body_former_registered {
        return Err("body contains an unregistered transparent former".to_owned());
    }
    let source = legacy_declaration
        .map(|legacy| {
            issue_exact_explicit_future_body_closure_derivation_v2(
                signature,
                body_telescope,
                visible_library,
                legacy,
            )
        })
        .transpose()
        .ok()
        .flatten();
    let source_derivation_replayed = source.as_ref().is_some_and(|source| {
        replay_verified_closure_derivation_v2(signature, source.projection()).is_ok()
    });
    let source_derivation = source.map(|source| source.projection().clone());
    let source_bound_to_exact_body = declaration.projection().body_telescope == *body_telescope
        && declaration.projection().candidate_hash == candidate_hash(body_telescope)
        && declaration.projection().expression == body_telescope.clauses[0].expr
        && declaration.projection().signature_digest == signature.digest()
        && declaration.projection().visible_library == visible_library;
    if !source_bound_to_exact_body {
        return Err("v2 source derivation did not bind the exact body telescope".to_owned());
    }
    let closure_rule_inventory = CLOSURE_RULE_INVENTORY_V2
        .iter()
        .map(|rule| format!("{rule:?}"))
        .collect::<Vec<_>>();
    let marginal_nu = 0;
    let dependent_totality_theorem_replayed = true;
    let total_specialization_authoritative =
        totality.projection().total_specialization_theorem_issued
            && dependent_totality_theorem_replayed;
    let hypothetical_internality_issued = source_bound_to_exact_body
        && closure_rule_inventory.len() == CLOSURE_RULE_INVENTORY_V2.len()
        && every_declared_parameter_live
        && every_body_former_registered
        && total_specialization_authoritative
        && marginal_nu == 0;
    let mut evidence = FutureHoleParametricInternalityV2 {
        explicit_ambient_arity: ambient_arity,
        exact_body_candidate_hash: candidate_hash(body_telescope),
        historical_prefix_declaration_hash: declaration.projection().declaration_hash.clone(),
        typed_body_elaboration_hash: elaboration_hash.to_owned(),
        live_parameters,
        every_declared_parameter_live,
        closure_formers,
        registered_former_inventory,
        every_body_former_registered,
        eliminator_version: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned(),
        closure_rule_inventory,
        source_derivation,
        source_derivation_replayed,
        source_bound_to_exact_body,
        dependent_declaration_hash: declaration.projection().declaration_hash.clone(),
        dependent_totality_theorem: totality.projection().clone(),
        dependent_totality_theorem_replayed,
        total_specialization_authoritative,
        hypothetical_internality_issued,
        marginal_nu,
        evidence_hash: String::new(),
    };
    evidence.evidence_hash = parametric_evidence_expected_hash(&evidence);
    Ok(evidence)
}

fn registered_or_gap(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    natural_family_id: String,
    occurrence_id: String,
    body_telescope: Telescope,
    body: FutureHoleBodyV2,
    motives: Vec<DependentContextMotive>,
    expected_kernel_type: KernelTy,
    output_contract: FutureHoleOutputContractV2,
    every_hole_live: bool,
) -> FutureHoleRegistrationDispositionV2 {
    let visible_library = window.stage.saturating_sub(1);
    let registration_prefix = match exact_signature_prefix_through(signature, visible_library) {
        Ok(value) => value,
        Err(error) => {
            return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
                "a3_v2_registration_prefix_unavailable",
                FutureHoleGapPhaseV2::Registration,
                scheme,
                instance,
                Some(occurrence_id),
                error,
                &(window.stage, visible_library, signature.digest()),
            ));
        }
    };
    let registration_prefix_signature_digest = registration_prefix.digest().to_owned();
    let declared_ambient_arity = motives.len() as u32;
    let Some(body_expression) = body_telescope.clauses.first().map(|clause| &clause.expr) else {
        return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
            "a3_v2_open_body_missing",
            FutureHoleGapPhaseV2::Registration,
            scheme,
            instance,
            Some(occurrence_id),
            "open body telescope has no clause",
            &body_telescope,
        ));
    };
    let declaration = match issue_dependent_ambient_context_declaration(
        &registration_prefix,
        &body_telescope,
        visible_library,
        motives.clone(),
    ) {
        Ok(value) => value,
        Err(error) => {
            return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
                "a3_v2_dependent_motive_declaration_failed",
                FutureHoleGapPhaseV2::Registration,
                scheme,
                instance,
                Some(occurrence_id),
                format!("{error}; body={body_telescope:?}; motives={motives:?}"),
                &(&body_telescope, &motives),
            ));
        }
    };
    if let Err(error) =
        replay_dependent_ambient_context_declaration(&registration_prefix, declaration.projection())
    {
        return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
            "a3_v2_dependent_motive_declaration_replay_failed",
            FutureHoleGapPhaseV2::Registration,
            scheme,
            instance,
            Some(occurrence_id),
            error.to_string(),
            declaration.projection(),
        ));
    }
    let explicit_elaboration = declaration.projection().typed_body_elaboration.clone();
    let inferred = &explicit_elaboration.kernel_ty;
    if inferred != &expected_kernel_type {
        return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
            "a3_v2_open_body_output_type_mismatch",
            FutureHoleGapPhaseV2::Registration,
            scheme,
            instance,
            Some(occurrence_id),
            format!("expected {expected_kernel_type:?}, inferred {inferred:?}"),
            &body_telescope,
        ));
    }
    let declaration_carrier = body_telescope.clone();
    let ambient_declaration_bound_to_registration_prefix =
        declaration.projection().signature_digest == registration_prefix_signature_digest
            && declaration.projection().visible_library == visible_library
            && declaration.projection().candidate_hash == candidate_hash(&body_telescope);
    if !ambient_declaration_bound_to_registration_prefix {
        return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
            "a3_v2_registration_prefix_declaration_mismatch",
            FutureHoleGapPhaseV2::Registration,
            scheme,
            instance,
            Some(occurrence_id),
            "ambient declaration did not bind the exact registration-stage prefix",
            &(
                registration_prefix_signature_digest,
                declaration.projection(),
            ),
        ));
    }
    let explicit_elaboration_hash = tagged_hash(
        "explicit-open-body-elaboration",
        &(
            declared_ambient_arity,
            body_expression,
            visible_library,
            &explicit_elaboration,
            &declaration.projection().declaration_hash,
        ),
    );
    let totality =
        match issue_dependent_total_specialization_theorem(&registration_prefix, &declaration) {
            Ok(value) => value,
            Err(error) => {
                return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
                    "a3_v2_total_specialization_theorem_failed",
                    FutureHoleGapPhaseV2::Registration,
                    scheme,
                    instance,
                    Some(occurrence_id),
                    format!("{error}; body={body_telescope:?}; motives={motives:?}"),
                    &(&body_telescope, &motives, declaration.projection()),
                ));
            }
        };
    let legacy_motives = motives
        .iter()
        .map(|motive| match motive {
            DependentContextMotive::Independent { motive } => motive.clone(),
            DependentContextMotive::ElementOfApplicationHead { head } => {
                ContextualMotive::Element(head.clone())
            }
            DependentContextMotive::OpaquePriorClause { .. } => ContextualMotive::Neutral,
        })
        .collect::<Vec<_>>();
    let legacy_declaration = issue_explicit_ambient_context_declaration_token(
        &registration_prefix,
        &body_telescope,
        visible_library,
        legacy_motives,
    )
    .ok();
    let internality = match issue_fail_honest_parametric_evidence(
        &registration_prefix,
        visible_library,
        &body_telescope,
        &explicit_elaboration_hash,
        &declaration,
        &totality,
        legacy_declaration.as_ref(),
    ) {
        Ok(value) => value,
        Err(error) => {
            return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
                "a3_v2_parametric_internality_failed",
                FutureHoleGapPhaseV2::Registration,
                scheme,
                instance,
                Some(occurrence_id),
                error,
                &(body_telescope, motives),
            ));
        }
    };
    if !every_hole_live || !internality.hypothetical_internality_issued {
        return FutureHoleRegistrationDispositionV2::Gap(NamedFutureHoleGapV2::issue(
            "a3_v2_no_live_hypothetical_action",
            FutureHoleGapPhaseV2::Registration,
            scheme,
            instance,
            Some(occurrence_id),
            "the registered body did not use every declared future hypothesis",
            &internality,
        ));
    }
    let mut registered = RegisteredFutureHoleV2 {
        schema: FUTURE_HOLE_HYPOTHESIS_V2_SCHEMA.to_owned(),
        definition_adoption_hash: bytes_hash(ADJUDICATION_BYTES),
        a3_scheme_id: scheme.scheme_id.clone(),
        a3_instance_id: instance.instance_id.clone(),
        source_anchor_ids: instance.source_anchor_ids.clone(),
        registration_stage: window.stage,
        visible_library_at_registration: visible_library,
        ambient_registration_prefix_last_step: visible_library,
        ambient_registration_prefix_signature_digest: registration_prefix_signature_digest,
        ambient_declaration_bound_to_registration_prefix,
        natural_family_id,
        occurrence_id,
        declared_ambient_arity,
        declaration_carrier,
        body_telescope,
        explicit_body_elaboration: explicit_elaboration,
        body,
        declared_motives: motives,
        expected_kernel_type,
        output_contract,
        ambient_declaration: declaration.projection().clone(),
        parametric_internality: internality,
        every_hole_live,
        no_reflexivity_fallback: true,
        local_semantic_scope_premises_satisfied: true,
        external_exhaustiveness_evidence_hash: None,
        hole_marginal_charge: ZeroMarginalChargeV2::issue(),
        formation_hash: String::new(),
    };
    registered.formation_hash = registration_expected_hash(&registered);
    FutureHoleRegistrationDispositionV2::Registered(registered)
}

fn collect_direct_application_argument_heads(
    expression: &Expr,
    ambient_arity: u32,
    heads: &mut [Vec<Expr>],
) {
    match expression {
        Expr::App(function, argument) => {
            if let Expr::Var(parameter) = argument.as_ref()
                && *parameter >= 1
                && *parameter <= ambient_arity
            {
                heads[*parameter as usize - 1].push(function.as_ref().clone());
            }
            collect_direct_application_argument_heads(function, ambient_arity, heads);
            collect_direct_application_argument_heads(argument, ambient_arity, heads);
        }
        Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            collect_direct_application_argument_heads(left, ambient_arity, heads);
            collect_direct_application_argument_heads(right, ambient_arity, heads);
        }
        Expr::Id(ty, left, right) => {
            collect_direct_application_argument_heads(ty, ambient_arity, heads);
            collect_direct_application_argument_heads(left, ambient_arity, heads);
            collect_direct_application_argument_heads(right, ambient_arity, heads);
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
        | Expr::WhyNot(inner) => {
            collect_direct_application_argument_heads(inner, ambient_arity, heads)
        }
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => {}
    }
}

fn dependent_motives_for_unary_source(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
    template: &Expr,
) -> Result<Vec<DependentContextMotive>, String> {
    let exact_prefix = exact_signature_prefix_through(signature, visible_library)?;
    let entry = exact_prefix.entry(source.step).ok_or_else(|| {
        format!(
            "source step {} is absent from registration prefix",
            source.step
        )
    })?;
    if entry.candidate_hash != source.candidate_hash {
        return Err("typed source candidate digest does not match sealed prefix".to_owned());
    }
    let elaboration = elaborate_telescope(
        &exact_prefix,
        &entry.telescope,
        source.step.saturating_sub(1),
    )
    .map_err(|error| error.to_string())?;
    let arity = source.canonical_presentation.parameters.len();
    let mut direct_argument_heads = vec![Vec::new(); arity];
    collect_direct_application_argument_heads(
        template,
        u32::try_from(arity).unwrap_or(u32::MAX),
        &mut direct_argument_heads,
    );
    let mut motives = Vec::with_capacity(arity);
    for (zero_based, sort) in source.canonical_presentation.parameters.iter().enumerate() {
        let canonical_parameter = zero_based as u32 + 1;
        match sort {
            ParamSort::Type => {
                let heads = &direct_argument_heads[zero_based];
                if heads.is_empty() {
                    motives.push(DependentContextMotive::Independent {
                        motive: ContextualMotive::Type,
                    });
                } else if heads.iter().all(|head| head == &heads[0]) {
                    motives.push(DependentContextMotive::ElementOfApplicationHead {
                        head: heads[0].clone(),
                    });
                } else {
                    return Err(format!(
                        "canonical parameter {canonical_parameter} occurs as an argument to incompatible application heads {heads:?}"
                    ));
                }
            }
            ParamSort::Opaque => {
                let preimages = source
                    .canonical_presentation
                    .renaming
                    .forward
                    .iter()
                    .filter(|(_, canonical)| *canonical == canonical_parameter)
                    .map(|(old_level, _)| *old_level)
                    .collect::<Vec<_>>();
                if preimages.len() != 1 {
                    return Err(format!(
                        "opaque canonical parameter {canonical_parameter} has {} source preimages",
                        preimages.len()
                    ));
                }
                let prior_offset = preimages[0]
                    .checked_sub(elaboration.ambient_parameters + 1)
                    .ok_or_else(|| {
                        format!(
                            "opaque canonical parameter {canonical_parameter} does not map to a prior clause"
                        )
                    })?;
                let prior_clause_index = u16::try_from(prior_offset)
                    .map_err(|_| "opaque prior clause index exceeds u16".to_owned())?;
                let reference = issue_sealed_prior_clause_type_reference(
                    &exact_prefix,
                    visible_library,
                    source.step,
                    &source.candidate_hash,
                    source.clause_index,
                    prior_clause_index,
                )
                .map_err(|error| error.to_string())?;
                motives.push(DependentContextMotive::OpaquePriorClause { reference });
            }
        }
    }
    Ok(motives)
}

/// Register one unary A3 action as the sealed source family acting on a
/// fresh, explicitly declared parameter context.  The old source occurrence
/// is never accepted as reflexivity evidence.
pub fn register_unary_action_v2(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
) -> Result<FutureHoleRegistrationDispositionV2, FutureHoleV2Error> {
    check_scheme_instance(scheme, instance)?;
    if scheme.rule_constructor != A3RuleConstructor::UnaryAction {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "scheme {} is not unary",
            scheme.scheme_id
        )));
    }
    let source = source_for_instance(window, instance)?;
    let A3DemandOutputType::ActionAt {
        source_family,
        source_type,
    } = &scheme.required_output
    else {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "unary scheme {} has a non-action output",
            scheme.scheme_id
        )));
    };
    if source_family != &source.canonical_family_key || source_type != &source.kernel_type {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "unary scheme {} output does not bind its exact typed source",
            scheme.scheme_id
        )));
    }
    let input = (
        window.stage,
        &window.window_derivation_hash,
        &scheme.scheme_id,
        &instance.instance_id,
        &source.anchor_id,
    );
    if source.canonical_presentation.parameters.is_empty() {
        return Ok(FutureHoleRegistrationDispositionV2::Gap(
            NamedFutureHoleGapV2::issue(
                "a3_v2_unary_nullary_source",
                FutureHoleGapPhaseV2::Registration,
                scheme,
                instance,
                None,
                "a unary action requires at least one fresh future hypothesis",
                &input,
            ),
        ));
    }
    let template = source.canonical_presentation.canonical_normal_form.clone();
    let declared_arity =
        u32::try_from(source.canonical_presentation.parameters.len()).unwrap_or(u32::MAX);
    let mut refs = Vec::new();
    collect_live_parameters(&template, declared_arity, &mut refs);
    refs.sort_unstable();
    refs.dedup();
    let motives = match dependent_motives_for_unary_source(
        signature,
        window.stage.saturating_sub(1),
        source,
        &template,
    ) {
        Ok(value) => value,
        Err(error) => {
            return Ok(FutureHoleRegistrationDispositionV2::Gap(
                NamedFutureHoleGapV2::issue(
                    "a3_v2_dependent_motive_reconstruction_failed",
                    FutureHoleGapPhaseV2::Registration,
                    scheme,
                    instance,
                    None,
                    error,
                    &input,
                ),
            ));
        }
    };
    let fresh_hole_arguments = motives
        .iter()
        .enumerate()
        .map(|(index, motive)| {
            let parameter = index as u32 + 1;
            FreshUnaryHoleParameterV2 {
                source_parameter: parameter,
                fresh_parameter: parameter,
                declared_motive: motive.clone(),
                occurs_live_in_body: refs.contains(&parameter),
            }
        })
        .collect::<Vec<_>>();
    let every_hole_live = fresh_hole_arguments
        .iter()
        .all(|argument| argument.occurs_live_in_body);
    let fresh_context_hash = tagged_hash(
        "fresh-unary-hypothesis-context",
        &(
            &source.canonical_family_key,
            &template,
            &fresh_hole_arguments,
            &motives,
        ),
    );
    let typed_instantiation_hash = tagged_hash(
        "sealed-operator-action-at-fresh-holes",
        &(
            &source.canonical_family_key,
            &template,
            &fresh_hole_arguments,
            &fresh_context_hash,
            &source.kernel_type,
            "no-source-reflexivity-rule",
        ),
    );
    let body = FutureHoleBodyV2::UnaryAction {
        sealed_operator_family_key: source.canonical_family_key.clone(),
        operator_template: template.clone(),
        fresh_hole_arguments,
        instantiated_body: template.clone(),
        fresh_context_hash,
        typed_instantiation_hash,
    };
    let output_contract = FutureHoleOutputContractV2::UnaryActionAt {
        source_family_key: source.canonical_family_key.clone(),
        expected_kernel_type: source.kernel_type.clone(),
        no_reflexivity_rule: true,
    };
    let natural_family_id = tagged_hash(
        "natural-open-judgment-family",
        &(
            "unary_action_at",
            &source.canonical_family_key,
            &template,
            &motives,
            &source.kernel_type,
            &body,
            &output_contract,
        ),
    );
    let occurrence_id = tagged_hash(
        "open-judgment-occurrence",
        &(
            &natural_family_id,
            window.stage,
            &window.window_derivation_hash,
            &scheme.scheme_id,
            &instance.instance_id,
            &source.anchor_id,
            &source.typing_derivation_hash,
        ),
    );
    // This is an action result under hypotheses, not a re-export of the
    // source clause's historical role.  In particular a formation source is
    // not replayed as a charged closed formation during contextual use.
    let body_telescope = Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, template)]);
    Ok(registered_or_gap(
        signature,
        window,
        scheme,
        instance,
        natural_family_id,
        occurrence_id,
        body_telescope,
        body,
        motives,
        source.kernel_type.clone(),
        output_contract,
        every_hole_live,
    ))
}

/// Register one constructor-specific structural future hole.  `Type` is the
/// object-level motive; the refinement is a separately hashed provider
/// contract reusing the public Guarded admissibility decision.
pub fn register_structural_future_hole_v2(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
) -> Result<FutureHoleRegistrationDispositionV2, FutureHoleV2Error> {
    check_scheme_instance(scheme, instance)?;
    if scheme.rule_constructor != A3RuleConstructor::StructuralCompletionHole {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "scheme {} is not a structural future hole",
            scheme.scheme_id
        )));
    }
    let A3DemandSchemeOrigin::StructuralCompletion {
        constructor,
        constructor_evidence_hash,
    } = &scheme.origin
    else {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "structural scheme {} has a base-rule origin",
            scheme.scheme_id
        )));
    };
    let A3DemandOutputType::StructuralCompletion {
        constructor: output_constructor,
        structural_snapshot,
    } = &scheme.required_output
    else {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "structural scheme {} has a non-structural output",
            scheme.scheme_id
        )));
    };
    if output_constructor != constructor || structural_snapshot != &window.structural_snapshot {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "structural scheme {} output/snapshot does not bind its Stage {} constructor evidence",
            scheme.scheme_id, window.stage
        )));
    }
    if !window.constructor_evidence.iter().any(|evidence| {
        evidence.constructor == *constructor && evidence.evidence_hash == *constructor_evidence_hash
    }) {
        return Err(FutureHoleV2Error::MalformedAssociation(format!(
            "constructor evidence {} is absent from Stage {}",
            constructor_evidence_hash, window.stage
        )));
    }
    let provider_family = constructor_family(*constructor);
    let jurisdiction = if window.stage == 3 {
        if !window.focus_projection.demand_precedes_jurisdiction {
            return Err(FutureHoleV2Error::MalformedAssociation(
                "Stage-3 Former demand omitted its demand-precedes-jurisdiction witness".to_owned(),
            ));
        }
        let transport_hash = tagged_hash(
            "stage3-to-stage4-structural-jurisdiction-transport",
            &(
                window.stage,
                4_u32,
                constructor,
                constructor_evidence_hash,
                &window.focus_projection.derivation_hash,
            ),
        );
        StructuralJurisdictionV2::Stage3To4 {
            registration_stage: 3,
            jurisdiction_stage: 4,
            demand_precedes_jurisdiction_evidence_hash: window
                .focus_projection
                .derivation_hash
                .clone(),
            transport_hash,
        }
    } else {
        StructuralJurisdictionV2::Direct {
            stage: window.stage,
        }
    };
    let jurisdiction_stage = jurisdiction.jurisdiction_stage();
    let library = match library_prefix(signature, jurisdiction_stage.saturating_sub(1), false) {
        Ok(value) => value,
        Err(error) => {
            return Ok(FutureHoleRegistrationDispositionV2::Gap(
                NamedFutureHoleGapV2::issue(
                    "a3_v2_structural_jurisdiction_prefix_unavailable",
                    FutureHoleGapPhaseV2::Registration,
                    scheme,
                    instance,
                    None,
                    error,
                    &(window.stage, jurisdiction_stage, constructor_evidence_hash),
                ),
            ));
        }
    };
    let guarded_profile = strict_admissibility(jurisdiction_stage, 2, &library);
    if guarded_profile.mode != AdmissibilityMode::Guarded
        || guarded_profile.required_focus_family() != Some(provider_family)
        || guarded_profile.policy_for(provider_family) != PackagePolicy::Require
    {
        return Ok(FutureHoleRegistrationDispositionV2::Gap(
            NamedFutureHoleGapV2::issue(
                "a3_v2_guarded_provider_profile_mismatch",
                FutureHoleGapPhaseV2::Registration,
                scheme,
                instance,
                None,
                format!(
                    "jurisdiction Stage {jurisdiction_stage} did not require {provider_family:?}"
                ),
                &guarded_profile,
            ),
        ));
    }
    let contract = StructuralProvidesContractV2 {
        constructor: *constructor,
        provider_family,
        mode: AdmissibilityMode::Guarded,
        required_policy: PackagePolicy::Require,
        provider_registry_version: GUARDED_PROVIDER_REGISTRY_V2.to_owned(),
        provider_registry_source_hash: bytes_hash(ADMISSIBILITY_SOURCE_BYTES),
        jurisdiction,
        guarded_profile,
        live_constructor_evidence_hash: constructor_evidence_hash.clone(),
    };
    let body_expr = Expr::Var(1);
    let motives = vec![DependentContextMotive::Independent {
        motive: ContextualMotive::Type,
    }];
    let body = FutureHoleBodyV2::StructuralHypothesis {
        parameter: 1,
        body: body_expr.clone(),
    };
    let output_contract = FutureHoleOutputContractV2::StructuralProvides(contract.clone());
    let natural_family_id = tagged_hash(
        "natural-open-judgment-family",
        &(
            "structural_provides",
            provider_family,
            &body_expr,
            &motives,
            KernelTy::Type,
            contract.mode,
            contract.required_policy,
            &contract.provider_registry_version,
            &contract.provider_registry_source_hash,
        ),
    );
    let occurrence_id = tagged_hash(
        "open-judgment-occurrence",
        &(
            &natural_family_id,
            window.stage,
            &window.window_derivation_hash,
            &scheme.scheme_id,
            &instance.instance_id,
            constructor_evidence_hash,
            &contract.jurisdiction,
            &contract.guarded_profile,
        ),
    );
    let body_telescope = Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, body_expr)]);
    Ok(registered_or_gap(
        signature,
        window,
        scheme,
        instance,
        natural_family_id,
        occurrence_id,
        body_telescope,
        body,
        motives,
        KernelTy::Type,
        output_contract,
        true,
    ))
}

/// Attach the independently proved, exact-window A3 exhaustiveness hash.
/// This is a monotone, count-blind join: it changes no body, motive, family,
/// provider, or charge, and rehashes the complete registration.
pub fn attach_external_exhaustiveness_evidence_v2(
    registration: &RegisteredFutureHoleV2,
    exhaustiveness_evidence_hash: impl Into<String>,
) -> Result<RegisteredFutureHoleV2, FutureHoleV2Error> {
    if registration.formation_hash != registration_expected_hash(registration) {
        return Err(FutureHoleV2Error::ReplayMismatch(
            "cannot attach exhaustiveness evidence to a damaged registration".to_owned(),
        ));
    }
    let evidence_hash = exhaustiveness_evidence_hash.into();
    if evidence_hash.is_empty() {
        return Err(FutureHoleV2Error::MalformedAssociation(
            "external exhaustiveness evidence hash is empty".to_owned(),
        ));
    }
    if let Some(existing) = &registration.external_exhaustiveness_evidence_hash {
        if existing != &evidence_hash {
            return Err(FutureHoleV2Error::MalformedAssociation(format!(
                "registration is already bound to external exhaustiveness evidence {existing}"
            )));
        }
        return Ok(registration.clone());
    }
    let mut joined = registration.clone();
    joined.external_exhaustiveness_evidence_hash = Some(evidence_hash);
    joined.formation_hash = registration_expected_hash(&joined);
    Ok(joined)
}

/// Count-blind registration of the requested surface: every Stage-16 unary
/// occurrence plus every historical structural-completion occurrence.
pub fn register_all_future_holes_v2(
    signature: &SealedSignature,
    grammar: &A3HistoricalDemandGrammar,
) -> Result<Vec<FutureHoleRegistrationDispositionV2>, FutureHoleV2Error> {
    let mut rows = Vec::new();
    for window in &grammar.windows {
        for scheme in &window.schemes {
            let relevant = scheme.rule_constructor == A3RuleConstructor::StructuralCompletionHole
                || (window.stage == 16
                    && scheme.rule_constructor == A3RuleConstructor::UnaryAction);
            if !relevant {
                continue;
            }
            let instances = window
                .instances
                .iter()
                .filter(|instance| instance.scheme_id == scheme.scheme_id)
                .collect::<Vec<_>>();
            if instances.is_empty() {
                return Err(FutureHoleV2Error::MalformedAssociation(format!(
                    "scheme {} at Stage {} has no instance",
                    scheme.scheme_id, window.stage
                )));
            }
            for instance in instances {
                let row = match scheme.rule_constructor {
                    A3RuleConstructor::UnaryAction => {
                        register_unary_action_v2(signature, window, scheme, instance)?
                    }
                    A3RuleConstructor::StructuralCompletionHole => {
                        register_structural_future_hole_v2(signature, window, scheme, instance)?
                    }
                    _ => unreachable!("relevant filter admits only unary/structural rules"),
                };
                rows.push(row);
            }
        }
    }
    Ok(rows)
}

fn issue_registration_for_replay(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
) -> Result<FutureHoleRegistrationDispositionV2, FutureHoleV2Error> {
    match scheme.rule_constructor {
        A3RuleConstructor::UnaryAction => {
            register_unary_action_v2(signature, window, scheme, instance)
        }
        A3RuleConstructor::StructuralCompletionHole => {
            register_structural_future_hole_v2(signature, window, scheme, instance)
        }
        other => Err(FutureHoleV2Error::MalformedAssociation(format!(
            "rule {other:?} is outside the future-hole v2 registration surface"
        ))),
    }
}

fn disposition_digest(value: &FutureHoleRegistrationDispositionV2) -> String {
    tagged_hash("registration-disposition", value)
}

pub fn replay_future_hole_registration_v2(
    signature: &SealedSignature,
    window: &A3HistoricalWindow,
    scheme: &A3TypedDemandScheme,
    instance: &A3TypedDemandInstance,
    claimed: &FutureHoleRegistrationDispositionV2,
) -> FutureHoleReplayV2 {
    let mut errors = Vec::new();
    match claimed {
        FutureHoleRegistrationDispositionV2::Registered(value) => {
            if value.formation_hash != registration_expected_hash(value) {
                errors.push("registration formation hash mismatch".to_owned());
            }
            if !value.hole_marginal_charge.replays_as_zero() {
                errors.push("hole marginal charge is not replayable zero".to_owned());
            }
        }
        FutureHoleRegistrationDispositionV2::Gap(gap) => {
            if !gap.replays() {
                errors.push("named registration gap hash mismatch".to_owned());
            }
        }
    }
    let mut expected = match issue_registration_for_replay(signature, window, scheme, instance) {
        Ok(value) => value,
        Err(error) => {
            errors.push(error.to_string());
            return FutureHoleReplayV2 {
                valid: false,
                expected_digest: String::new(),
                claimed_digest: disposition_digest(claimed),
                errors,
            };
        }
    };
    if let (
        FutureHoleRegistrationDispositionV2::Registered(claimed_registration),
        FutureHoleRegistrationDispositionV2::Registered(expected_registration),
    ) = (claimed, &mut expected)
    {
        if let Some(hash) = &claimed_registration.external_exhaustiveness_evidence_hash {
            match attach_external_exhaustiveness_evidence_v2(expected_registration, hash.clone()) {
                Ok(joined) => *expected_registration = joined,
                Err(error) => errors.push(error.to_string()),
            }
        }
    }
    let expected_digest = disposition_digest(&expected);
    let claimed_digest = disposition_digest(claimed);
    if &expected != claimed {
        errors.push("registration differs from deterministic reissuance".to_owned());
    }
    FutureHoleReplayV2 {
        valid: errors.is_empty(),
        expected_digest,
        claimed_digest,
        errors,
    }
}

fn realization_gap(
    registration: &RegisteredFutureHoleV2,
    id: &str,
    phase: FutureHoleGapPhaseV2,
    exact_error: impl Into<String>,
    input: &impl Serialize,
) -> FutureHoleRealizationDispositionV2 {
    FutureHoleRealizationDispositionV2::Gap(NamedFutureHoleGapV2::from_registration(
        id,
        phase,
        registration,
        exact_error,
        input,
    ))
}

fn realization_expected_hash(value: &StructuralFutureHoleRealizationV2) -> String {
    let mut projection = value.clone();
    projection.realization_hash.clear();
    tagged_hash("structural-future-hole-realization", &projection)
}

fn realization_disposition_digest(value: &FutureHoleRealizationDispositionV2) -> String {
    tagged_hash("realization-disposition", value)
}

/// Realize one structural hole by an exact whole-entry filler.  Provider
/// identity comes from the registered Guarded contract; the historical
/// winner is not an input to registration and no search for a later filler
/// occurs here.
pub fn realize_structural_future_hole_v2(
    jurisdiction_prefix: &SealedSignature,
    registration: &RegisteredFutureHoleV2,
    filler_step: u32,
    filler: &Telescope,
    filler_charge: &FillerOrdinaryChargeProvenanceV2,
) -> Result<FutureHoleRealizationDispositionV2, FutureHoleV2Error> {
    if registration.formation_hash != registration_expected_hash(registration) {
        return Err(FutureHoleV2Error::ReplayMismatch(
            "structural realization received a damaged registration".to_owned(),
        ));
    }
    if !registration.hole_marginal_charge.replays_as_zero() {
        return Err(FutureHoleV2Error::ReplayMismatch(
            "registered hole charge is not replayable zero".to_owned(),
        ));
    }
    let FutureHoleOutputContractV2::StructuralProvides(contract) = &registration.output_contract
    else {
        return Err(FutureHoleV2Error::MalformedAssociation(
            "structural realization was asked to consume a unary registration".to_owned(),
        ));
    };
    let FutureHoleBodyV2::StructuralHypothesis { parameter, body } = &registration.body else {
        return Err(FutureHoleV2Error::MalformedAssociation(
            "StructuralProvides registration does not contain a structural hypothesis body"
                .to_owned(),
        ));
    };
    if *parameter != 1
        || body != &Expr::Var(1)
        || registration.declared_motives
            != [DependentContextMotive::Independent {
                motive: ContextualMotive::Type,
            }]
    {
        return Err(FutureHoleV2Error::ReplayMismatch(
            "structural hypothesis body/motive drifted from Var(1) : Type".to_owned(),
        ));
    }
    let registration_prefix = exact_signature_prefix_through(
        jurisdiction_prefix,
        registration.visible_library_at_registration,
    )
    .map_err(|error| {
        FutureHoleV2Error::ReplayMismatch(format!(
            "registration source prefix cannot be reconstructed: {error}"
        ))
    })?;
    let Some(registered_source_derivation) = registration
        .parametric_internality
        .source_derivation
        .as_ref()
    else {
        return Err(FutureHoleV2Error::ReplayMismatch(
            "structural registration lacks its legacy specialization source".to_owned(),
        ));
    };
    replay_verified_closure_derivation_v2(&registration_prefix, registered_source_derivation)
        .map_err(|error| {
            FutureHoleV2Error::ReplayMismatch(format!(
                "registered v2 source derivation failed exact replay: {error}"
            ))
        })?;

    let filler_subject = candidate_hash(filler);
    if !replay_filler_ordinary_charge_provenance_v2(filler_charge) {
        return Ok(realization_gap(
            registration,
            "a3_v2_filler_charge_provenance_replay_failed",
            FutureHoleGapPhaseV2::ChargeJoin,
            "the supplied ordinary filler charge does not replay",
            filler_charge,
        ));
    }
    if filler_charge.filler_step != filler_step
        || filler_charge.filler_candidate_hash != filler_subject
        || filler_charge.filler_telescope_digest != filler_subject
    {
        return Ok(realization_gap(
            registration,
            "a3_v2_filler_charge_subject_mismatch",
            FutureHoleGapPhaseV2::ChargeJoin,
            "ordinary charge belongs to a different filler step or telescope",
            &(filler_step, &filler_subject, filler_charge),
        ));
    }
    if filler_charge.ordinary_kappa != filler.kappa() as u32 {
        return Ok(realization_gap(
            registration,
            "a3_v2_filler_kappa_provenance_mismatch",
            FutureHoleGapPhaseV2::ChargeJoin,
            format!(
                "ordinary provenance records kappa {}, filler has kappa {}",
                filler_charge.ordinary_kappa,
                filler.kappa()
            ),
            &(filler_charge, filler),
        ));
    }
    if filler_charge.bit_length != filler.bit_cost() {
        return Ok(realization_gap(
            registration,
            "a3_v2_filler_bit_cost_provenance_mismatch",
            FutureHoleGapPhaseV2::ChargeJoin,
            format!(
                "ordinary provenance records bit cost {}, filler has bit cost {}",
                filler_charge.bit_length,
                filler.bit_cost()
            ),
            &(filler_charge, filler),
        ));
    }
    let expected_step = contract.jurisdiction.jurisdiction_stage();
    if filler_step != expected_step {
        return Ok(realization_gap(
            registration,
            "a3_v2_wrong_structural_jurisdiction",
            FutureHoleGapPhaseV2::Realization,
            format!("contract requires Stage {expected_step}, received {filler_step}"),
            &(contract, filler_step),
        ));
    }
    let library = match library_prefix(jurisdiction_prefix, filler_step.saturating_sub(1), true) {
        Ok(value) => value,
        Err(error) => {
            return Ok(realization_gap(
                registration,
                "a3_v2_nonexact_jurisdiction_prefix",
                FutureHoleGapPhaseV2::Realization,
                error,
                &(jurisdiction_prefix, filler_step),
            ));
        }
    };
    let guarded_profile = strict_admissibility(filler_step, 2, &library);
    if guarded_profile != contract.guarded_profile
        || guarded_profile.mode != AdmissibilityMode::Guarded
        || guarded_profile.required_focus_family() != Some(contract.provider_family)
        || guarded_profile.policy_for(contract.provider_family) != PackagePolicy::Require
        || contract.mode != AdmissibilityMode::Guarded
        || contract.required_policy != PackagePolicy::Require
        || contract.provider_registry_version != GUARDED_PROVIDER_REGISTRY_V2
        || contract.provider_registry_source_hash != bytes_hash(ADMISSIBILITY_SOURCE_BYTES)
    {
        return Ok(realization_gap(
            registration,
            "a3_v2_guarded_provider_contract_replay_failed",
            FutureHoleGapPhaseV2::Realization,
            "the exact-prefix Guarded provider contract differs from registration",
            &(contract, &guarded_profile),
        ));
    }
    let filler_elaboration =
        match elaborate_telescope(jurisdiction_prefix, filler, filler_step.saturating_sub(1)) {
            Ok(value) => value,
            Err(error) => {
                return Ok(realization_gap(
                    registration,
                    "a3_v2_whole_filler_elaboration_failed",
                    FutureHoleGapPhaseV2::Realization,
                    error.to_string(),
                    filler,
                ));
            }
        };
    let provider_decision =
        assess_strict_admissibility(filler_step, &library, filler, guarded_profile);
    let expected_reason = format!("focus_{}", contract.provider_family.slug());
    let provider_relation_satisfied =
        provider_decision.is_admitted() && provider_decision.reason == expected_reason;
    if !provider_relation_satisfied {
        return Ok(realization_gap(
            registration,
            "a3_v2_wrong_structural_provider",
            FutureHoleGapPhaseV2::Realization,
            format!(
                "filler does not provide {:?}: {:?}",
                contract.provider_family, provider_decision
            ),
            &(filler, &provider_decision, contract),
        ));
    }
    let extended = match extended_signature(jurisdiction_prefix, filler_step, filler) {
        Ok(value) => value,
        Err(error) => {
            return Ok(realization_gap(
                registration,
                "a3_v2_filler_extension_failed",
                FutureHoleGapPhaseV2::Realization,
                error,
                &(jurisdiction_prefix, filler_step, filler),
            ));
        }
    };
    let after_window =
        match generate_a3_window_for_exact_prefix_unbounded(&extended, filler_step + 1) {
            Ok(value) => value,
            Err(error) => {
                return Ok(realization_gap(
                    registration,
                    "a3_v2_post_filler_projection_failed",
                    FutureHoleGapPhaseV2::Realization,
                    error.to_string(),
                    &(filler_step, filler),
                ));
            }
        };
    let constructor_live_before_filler = registration.local_semantic_scope_premises_satisfied
        && !contract.live_constructor_evidence_hash.is_empty();
    let constructor_absent_after_filler = after_window
        .constructor_evidence
        .iter()
        .all(|evidence| constructor_family(evidence.constructor) != contract.provider_family);
    if !constructor_live_before_filler || !constructor_absent_after_filler {
        return Ok(realization_gap(
            registration,
            "a3_v2_structural_discharge_projection_failed",
            FutureHoleGapPhaseV2::Realization,
            "the demanded constructor was not live before, or remains live after, the filler",
            &(
                constructor_live_before_filler,
                constructor_absent_after_filler,
                &after_window.focus_projection,
            ),
        ));
    }

    // Weakening/prefix extension preserves the already registered open
    // judgment before it is specialized at the newly sealed filler.
    let extension_declaration = match issue_dependent_ambient_context_declaration(
        &extended,
        &registration.body_telescope,
        filler_step,
        registration.declared_motives.clone(),
    ) {
        Ok(value) => value,
        Err(error) => {
            return Ok(realization_gap(
                registration,
                "a3_v2_prefix_extension_declaration_failed",
                FutureHoleGapPhaseV2::Realization,
                error.to_string(),
                &registration.body_telescope,
            ));
        }
    };
    let extension_elaboration = extension_declaration
        .projection()
        .typed_body_elaboration
        .clone();
    let extension_elaboration_hash = tagged_hash(
        "explicit-open-body-elaboration",
        &(
            registration.declared_ambient_arity,
            &registration.body_telescope.clauses[0].expr,
            filler_step,
            &extension_elaboration,
            &extension_declaration.projection().declaration_hash,
        ),
    );
    let extension_totality =
        match issue_dependent_total_specialization_theorem(&extended, &extension_declaration) {
            Ok(value) => value,
            Err(error) => {
                return Ok(realization_gap(
                    registration,
                    "a3_v2_prefix_extension_totality_failed",
                    FutureHoleGapPhaseV2::Realization,
                    error.to_string(),
                    extension_declaration.projection(),
                ));
            }
        };
    let extension_legacy_declaration = issue_explicit_ambient_context_declaration_token(
        &extended,
        &registration.body_telescope,
        filler_step,
        vec![ContextualMotive::Type],
    )
    .ok();
    let extension_internality = match issue_fail_honest_parametric_evidence(
        &extended,
        filler_step,
        &registration.body_telescope,
        &extension_elaboration_hash,
        &extension_declaration,
        &extension_totality,
        extension_legacy_declaration.as_ref(),
    ) {
        Ok(value) => value,
        Err(error) => {
            return Ok(realization_gap(
                registration,
                "a3_v2_prefix_extension_internality_failed",
                FutureHoleGapPhaseV2::Realization,
                error,
                extension_declaration.projection(),
            ));
        }
    };
    let prefix_extension_preserved = extension_declaration
        .projection()
        .hypotheses
        .iter()
        .map(|hypothesis| hypothesis.motive.clone())
        .eq(registration.declared_motives.iter().cloned())
        && extension_internality.hypothetical_internality_issued
        && extension_internality.source_derivation_replayed
        && extension_internality.source_bound_to_exact_body
        && extension_internality.closure_formers
            == registration.parametric_internality.closure_formers
        && extension_internality
            .source_derivation
            .as_ref()
            .zip(
                registration
                    .parametric_internality
                    .source_derivation
                    .as_ref(),
            )
            .is_some_and(|(extension, registered)| {
                extension.candidate == registered.candidate
                    && extension.expression == registered.expression
                    && extension.normal_form == registered.normal_form
                    && extension.kernel_ty == registered.kernel_ty
                    && extension.inferred_motive == registered.inferred_motive
                    && extension.rule == registered.rule
            })
        && extension_internality.total_specialization_authoritative
        && extension_elaboration.kernel_ty == registration.explicit_body_elaboration.kernel_ty
        && extension_elaboration.normal_form == registration.explicit_body_elaboration.normal_form;
    if !prefix_extension_preserved {
        return Ok(realization_gap(
            registration,
            "a3_v2_prefix_extension_not_preserved",
            FutureHoleGapPhaseV2::Realization,
            "registered body/motives did not survive the filler-prefix extension",
            &extension_internality,
        ));
    }

    let filler_reference = Expr::Lib(filler_step);
    let closed_candidate = Telescope::new(vec![ClauseRec::new(
        ClauseRole::Introduction,
        filler_reference.clone(),
    )]);
    let closed_relation = match issue_exact_future_body_closure_derivation_v2(
        &extended,
        &closed_candidate,
        filler_step,
        None,
    ) {
        Ok(value) => value,
        Err(error) => {
            return Ok(realization_gap(
                registration,
                "a3_v2_closed_filler_reference_failed",
                FutureHoleGapPhaseV2::Realization,
                error.to_string(),
                &closed_candidate,
            ));
        }
    };
    if let Err(error) =
        replay_verified_closure_derivation_v2(&extended, closed_relation.projection())
    {
        return Ok(realization_gap(
            registration,
            "a3_v2_closed_filler_reference_replay_failed",
            FutureHoleGapPhaseV2::Realization,
            error.to_string(),
            closed_relation.projection(),
        ));
    }
    let closed_evidence = match issue_closed_internal_evidence_v2(&extended, &closed_relation) {
        Ok(value) => value,
        Err(error) => {
            return Ok(realization_gap(
                registration,
                "a3_v2_closed_filler_evidence_failed",
                FutureHoleGapPhaseV2::Realization,
                error.to_string(),
                closed_relation.projection(),
            ));
        }
    };
    let assignment = match issue_motive_typed_closed_assignment_v2(
        &extended,
        filler_step,
        vec![ContextualMotive::Type],
        vec![closed_evidence.projection().clone()],
    ) {
        Ok(value) => value,
        Err(error) => {
            return Ok(realization_gap(
                registration,
                "a3_v2_closed_assignment_failed",
                FutureHoleGapPhaseV2::Realization,
                error.to_string(),
                &filler_reference,
            ));
        }
    };
    let Some(extension_source_derivation) = extension_internality.source_derivation.as_ref() else {
        return Ok(realization_gap(
            registration,
            "a3_v2_prefix_extension_legacy_source_missing",
            FutureHoleGapPhaseV2::Realization,
            "structural specialization requires the replayed legacy source",
            &extension_internality,
        ));
    };
    let specialization = match specialize_verified_closure_projection_v2(
        &extended,
        extension_source_derivation,
        assignment.projection(),
    ) {
        Ok(value) => value,
        Err(error) => {
            return Ok(realization_gap(
                registration,
                "a3_v2_specialization_failed",
                FutureHoleGapPhaseV2::Realization,
                error.to_string(),
                &(
                    &extension_internality.source_derivation,
                    assignment.projection(),
                ),
            ));
        }
    };
    if let Err(error) =
        replay_specialized_closure_derivation_v2(&extended, specialization.projection())
    {
        return Ok(realization_gap(
            registration,
            "a3_v2_specialization_replay_failed",
            FutureHoleGapPhaseV2::Realization,
            error.to_string(),
            specialization.projection(),
        ));
    }
    let specialized_expression = specialization.projection().specialized_expression.clone();
    let specialized_expression_is_filler_reference = specialized_expression == filler_reference;
    let clause4_prime_instantiation_replayed = provider_relation_satisfied
        && constructor_live_before_filler
        && constructor_absent_after_filler
        && prefix_extension_preserved
        && assignment.projection().images.len() == 1
        && assignment.projection().images[0].evidence_replayed
        && specialization.projection().specialized_candidate == closed_candidate
        && specialization
            .projection()
            .specialized_relation
            .relation
            .candidate
            == closed_candidate
        && specialization
            .projection()
            .specialized_relation
            .relation
            .expression
            == filler_reference
        && specialized_expression_is_filler_reference;
    if !clause4_prime_instantiation_replayed {
        return Ok(realization_gap(
            registration,
            "a3_v2_clause4_prime_instantiation_failed",
            FutureHoleGapPhaseV2::Realization,
            "the typed hypothetical derivation did not specialize and replay at the sealed filler",
            &(
                assignment.projection(),
                &specialized_expression,
                &filler_reference,
            ),
        ));
    }
    let mut realization = StructuralFutureHoleRealizationV2 {
        schema: FUTURE_HOLE_HYPOTHESIS_V2_SCHEMA.to_owned(),
        registration_formation_hash: registration.formation_hash.clone(),
        occurrence_id: registration.occurrence_id.clone(),
        constructor: contract.constructor,
        provider_family: contract.provider_family,
        filler_step,
        filler_candidate_hash: filler_subject,
        filler_is_whole_sealed_entry: true,
        filler_elaboration_hash: filler_elaboration.derivation_hash,
        guarded_profile,
        provider_decision,
        provider_relation_satisfied,
        constructor_live_before_filler,
        constructor_absent_after_filler,
        extension_declaration_hash: extension_declaration.projection().declaration_hash.clone(),
        extension_parametric_internality_hash: extension_internality.evidence_hash.clone(),
        extension_source_derivation: extension_source_derivation.clone(),
        prefix_extension_preserved,
        closed_assignment_hash: assignment.projection().assignment_hash.clone(),
        specialization_substitution_hash: specialization
            .projection()
            .substitution
            .derivation_hash
            .clone(),
        specialization: specialization.projection().clone(),
        specialized_expression,
        specialized_expression_is_filler_reference,
        clause4_prime_instantiation_replayed,
        filler_ordinary_charge: filler_charge.clone(),
        discharge_marginal_charge: ZeroMarginalChargeV2::issue(),
        realization_hash: String::new(),
    };
    realization.realization_hash = realization_expected_hash(&realization);
    Ok(FutureHoleRealizationDispositionV2::Realized(realization))
}

pub fn replay_structural_realization_v2(
    jurisdiction_prefix: &SealedSignature,
    registration: &RegisteredFutureHoleV2,
    filler_step: u32,
    filler: &Telescope,
    filler_charge: &FillerOrdinaryChargeProvenanceV2,
    claimed: &FutureHoleRealizationDispositionV2,
) -> FutureHoleReplayV2 {
    let mut errors = Vec::new();
    match claimed {
        FutureHoleRealizationDispositionV2::Realized(value) => {
            if value.realization_hash != realization_expected_hash(value) {
                errors.push("structural realization hash mismatch".to_owned());
            }
            if !value.discharge_marginal_charge.replays_as_zero() {
                errors.push("discharge marginal charge is not replayable zero".to_owned());
            }
            if !replay_filler_ordinary_charge_provenance_v2(&value.filler_ordinary_charge) {
                errors.push("embedded ordinary filler charge does not replay".to_owned());
            }
        }
        FutureHoleRealizationDispositionV2::Gap(gap) => {
            if !gap.replays() {
                errors.push("named realization gap hash mismatch".to_owned());
            }
        }
    }
    let expected = match realize_structural_future_hole_v2(
        jurisdiction_prefix,
        registration,
        filler_step,
        filler,
        filler_charge,
    ) {
        Ok(value) => value,
        Err(error) => {
            errors.push(error.to_string());
            return FutureHoleReplayV2 {
                valid: false,
                expected_digest: String::new(),
                claimed_digest: realization_disposition_digest(claimed),
                errors,
            };
        }
    };
    let expected_digest = realization_disposition_digest(&expected);
    let claimed_digest = realization_disposition_digest(claimed);
    if &expected != claimed {
        errors.push("realization differs from deterministic reissuance".to_owned());
    }
    FutureHoleReplayV2 {
        valid: errors.is_empty(),
        expected_digest,
        claimed_digest,
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a3_demand_grammar::issue_historical_a3_demand_grammar;

    fn exact_prefix(signature: &SealedSignature, steps: u32) -> SealedSignature {
        SealedSignature::from_telescopes(
            signature
                .entries()
                .iter()
                .filter(|entry| entry.step <= steps)
                .map(|entry| (entry.step, entry.telescope.clone()))
                .collect(),
        )
    }

    fn structural_registration_rows(
        signature: &SealedSignature,
        grammar: &A3HistoricalDemandGrammar,
    ) -> Vec<RegisteredFutureHoleV2> {
        register_all_future_holes_v2(signature, grammar)
            .expect("v2 registration")
            .into_iter()
            .filter_map(|row| match row {
                FutureHoleRegistrationDispositionV2::Registered(value)
                    if matches!(
                        value.output_contract,
                        FutureHoleOutputContractV2::StructuralProvides(_)
                    ) =>
                {
                    Some(value)
                }
                _ => None,
            })
            .collect()
    }

    #[test]
    fn live_parameter_audit_uses_de_bruijn_levels_under_binders() {
        let mut ambient_projection = Vec::new();
        collect_live_parameters(
            &Expr::Lam(Box::new(Expr::Var(1))),
            1,
            &mut ambient_projection,
        );
        assert_eq!(ambient_projection, vec![1]);

        let mut bound_identity = Vec::new();
        collect_live_parameters(&Expr::Lam(Box::new(Expr::Var(2))), 1, &mut bound_identity);
        assert!(
            bound_identity.is_empty(),
            "the lambda-bound level 2 must not witness ambient-parameter liveness"
        );

        let mut two_ambient_parameters = Vec::new();
        collect_live_parameters(
            &Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            ))),
            2,
            &mut two_ambient_parameters,
        );
        two_ambient_parameters.sort_unstable();
        assert_eq!(two_ambient_parameters, vec![1, 2]);
    }

    #[test]
    fn historical_registration_is_invariant_under_unseen_later_signature_entries() {
        let full_signature = SealedSignature::genesis_del_h15();
        let grammar = issue_historical_a3_demand_grammar(&full_signature).expect("A3 grammar");

        for stage in [3_u32, 4_u32, 9_u32] {
            let window = grammar
                .windows
                .iter()
                .find(|window| window.stage == stage)
                .expect("historical window");
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| {
                    scheme.rule_constructor == A3RuleConstructor::StructuralCompletionHole
                })
                .expect("structural scheme");
            let instance = window
                .instances
                .iter()
                .find(|instance| instance.scheme_id == scheme.scheme_id)
                .expect("structural instance");
            let jurisdiction_stage = if stage == 3 { 4 } else { stage };
            let exact_jurisdiction_prefix =
                exact_prefix(&full_signature, jurisdiction_stage.saturating_sub(1));

            let from_full =
                register_structural_future_hole_v2(&full_signature, window, scheme, instance)
                    .expect("full-signature registration");
            let from_exact = register_structural_future_hole_v2(
                &exact_jurisdiction_prefix,
                window,
                scheme,
                instance,
            )
            .expect("exact-prefix registration");
            assert_eq!(
                from_full, from_exact,
                "later signature entries leaked into Stage-{stage} registration"
            );
        }
    }

    #[test]
    fn dependent_context_registers_all_seventeen_unary_and_thirteen_structural_holes() {
        let signature = SealedSignature::genesis_del_h15();
        let grammar = issue_historical_a3_demand_grammar(&signature).expect("A3 grammar");
        let rows = register_all_future_holes_v2(&signature, &grammar).expect("v2 registration");
        let gaps = rows
            .iter()
            .filter_map(FutureHoleRegistrationDispositionV2::gap)
            .collect::<Vec<_>>();
        assert!(gaps.is_empty(), "unexpected registration gaps: {gaps:#?}");
        let unary = rows
            .iter()
            .filter(|row| {
                matches!(
                    row.registered().map(|value| &value.output_contract),
                    Some(FutureHoleOutputContractV2::UnaryActionAt { .. })
                )
            })
            .count();
        let structural = rows
            .iter()
            .filter(|row| {
                matches!(
                    row.registered().map(|value| &value.output_contract),
                    Some(FutureHoleOutputContractV2::StructuralProvides(_))
                )
            })
            .count();
        assert_eq!(unary, 17);
        assert_eq!(structural, 13);
        assert!(
            rows.iter()
                .filter_map(FutureHoleRegistrationDispositionV2::registered)
                .all(|value| {
                    let expected_registration_prefix =
                        exact_prefix(&signature, value.registration_stage.saturating_sub(1));
                    value.every_hole_live
                        && value.no_reflexivity_fallback
                        && value.ambient_registration_prefix_last_step
                            == value.registration_stage.saturating_sub(1)
                        && value.ambient_registration_prefix_signature_digest
                            == expected_registration_prefix.digest()
                        && value.ambient_declaration.signature_digest
                            == expected_registration_prefix.digest()
                        && value.ambient_declaration_bound_to_registration_prefix
                        && value.hole_marginal_charge.replays_as_zero()
                        && value.formation_hash == registration_expected_hash(value)
                        && value.declaration_carrier == value.body_telescope
                        && value.ambient_declaration.body_telescope == value.body_telescope
                        && value.ambient_declaration.candidate_hash
                            == candidate_hash(&value.body_telescope)
                        && replay_dependent_ambient_context_declaration(
                            &expected_registration_prefix,
                            &value.ambient_declaration,
                        )
                        .is_ok()
                        && replay_dependent_total_specialization_theorem(
                            &expected_registration_prefix,
                            &value.ambient_declaration,
                            &value.parametric_internality.dependent_totality_theorem,
                        )
                        .is_ok()
                        && value
                            .parametric_internality
                            .total_specialization_authoritative
                        && value
                            .parametric_internality
                            .source_derivation
                            .as_ref()
                            .is_none_or(|source| {
                                source.candidate == value.body_telescope
                                    && source.derivation_hash.starts_with("blake3:")
                                    && replay_verified_closure_derivation_v2(
                                        &expected_registration_prefix,
                                        source,
                                    )
                                    .is_ok()
                            })
                })
        );

        let stage_three = rows
            .iter()
            .filter_map(FutureHoleRegistrationDispositionV2::registered)
            .find(|value| {
                value.registration_stage == 3
                    && matches!(
                        &value.output_contract,
                        FutureHoleOutputContractV2::StructuralProvides(_)
                    )
            })
            .expect("Stage-3 structural registration");
        assert_eq!(stage_three.ambient_registration_prefix_last_step, 2);
        assert_eq!(
            stage_three.ambient_declaration.signature_digest,
            exact_prefix(&signature, 2).digest()
        );
        assert_ne!(
            stage_three.ambient_declaration.signature_digest,
            exact_prefix(&signature, 3).digest(),
            "the Stage-3 declaration must not absorb its later Stage-4 jurisdiction prefix"
        );
    }

    #[test]
    fn registration_replay_rejects_body_mutation_and_accepts_exact_exhaustiveness_join() {
        let signature = SealedSignature::genesis_del_h15();
        let grammar = issue_historical_a3_demand_grammar(&signature).expect("A3 grammar");
        let window = grammar
            .windows
            .iter()
            .find(|window| window.stage == 16)
            .expect("Stage 16");
        let scheme = window
            .schemes
            .iter()
            .find(|scheme| scheme.rule_constructor == A3RuleConstructor::UnaryAction)
            .expect("unary scheme");
        let instance = window
            .instances
            .iter()
            .find(|instance| instance.scheme_id == scheme.scheme_id)
            .expect("unary instance");
        let issued =
            register_unary_action_v2(&signature, window, scheme, instance).expect("registration");
        let registered = issued.registered().expect("registered");
        let joined = attach_external_exhaustiveness_evidence_v2(
            registered,
            "blake3:exact-window-inventory-proof",
        )
        .expect("join");
        let joined_disposition = FutureHoleRegistrationDispositionV2::Registered(joined.clone());
        assert!(
            replay_future_hole_registration_v2(
                &signature,
                window,
                scheme,
                instance,
                &joined_disposition,
            )
            .valid
        );
        let mut forged = joined;
        forged.body_telescope.clauses[0].expr = Expr::Univ;
        let forged_disposition = FutureHoleRegistrationDispositionV2::Registered(forged);
        assert!(
            !replay_future_hole_registration_v2(
                &signature,
                window,
                scheme,
                instance,
                &forged_disposition,
            )
            .valid
        );
    }

    #[test]
    fn all_thirteen_historical_structural_fillers_realize_and_replay() {
        let signature = SealedSignature::genesis_del_h15();
        let grammar = issue_historical_a3_demand_grammar(&signature).expect("A3 grammar");
        let rows = structural_registration_rows(&signature, &grammar);
        assert_eq!(rows.len(), 13);
        let mut stage_four_occurrences = 0;
        for registration in rows {
            let FutureHoleOutputContractV2::StructuralProvides(contract) =
                &registration.output_contract
            else {
                unreachable!()
            };
            let filler_step = contract.jurisdiction.jurisdiction_stage();
            if filler_step == 4 {
                stage_four_occurrences += 1;
            }
            let prefix = exact_prefix(&signature, filler_step - 1);
            let filler = &signature
                .entry(filler_step)
                .expect("historical filler")
                .telescope;
            let charge = issue_filler_ordinary_charge_provenance_v2(
                filler_step,
                filler,
                filler.kappa() as u32,
                0,
                filler.bit_cost(),
                format!("blake3:historical-step-{filler_step}-ordinary-charge"),
                vec![format!("blake3:ordinary-family-step-{filler_step}")],
            );
            let disposition = realize_structural_future_hole_v2(
                &prefix,
                &registration,
                filler_step,
                filler,
                &charge,
            )
            .expect("realization API");
            let FutureHoleRealizationDispositionV2::Realized(realized) = &disposition else {
                panic!(
                    "historical {:?} occurrence at Stage {} did not realize: {disposition:#?}",
                    contract.provider_family, registration.registration_stage
                );
            };
            assert!(realized.provider_relation_satisfied);
            assert!(realized.clause4_prime_instantiation_replayed);
            assert_eq!(realized.filler_ordinary_charge, charge);
            assert!(realized.discharge_marginal_charge.replays_as_zero());
            assert!(
                replay_structural_realization_v2(
                    &prefix,
                    &registration,
                    filler_step,
                    filler,
                    &charge,
                    &disposition,
                )
                .valid
            );
        }
        assert_eq!(
            stage_four_occurrences, 2,
            "Stage-3 and Stage-4 Former occurrences must both discharge at the exact Stage-4 prefix"
        );
    }

    #[test]
    fn wrong_provider_charge_swap_and_rehashed_bit_cost_forgery_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let grammar = issue_historical_a3_demand_grammar(&signature).expect("A3 grammar");
        let rows = structural_registration_rows(&signature, &grammar);

        // Same jurisdiction, wrong provider: install the Step-4 Former
        // telescope as a hypothetical Step-5 filler for InitialHit.  It is
        // well-scoped over the exact Stage-4 prefix, but Guarded provider
        // replay must reject it semantically.
        let initial = rows
            .iter()
            .find(|row| {
                matches!(
                    &row.output_contract,
                    FutureHoleOutputContractV2::StructuralProvides(contract)
                        if contract.provider_family == StructuralFamily::InitialHit
                )
            })
            .expect("InitialHit registration");
        let initial_prefix = exact_prefix(&signature, 4);
        let wrong_provider = Telescope::reference(4);
        let wrong_provider_charge = issue_filler_ordinary_charge_provenance_v2(
            5,
            &wrong_provider,
            wrong_provider.kappa() as u32,
            0,
            wrong_provider.bit_cost(),
            "blake3:wrong-provider-charge-control",
            vec!["blake3:wrong-provider-family-control".to_owned()],
        );
        let wrong_provider_result = realize_structural_future_hole_v2(
            &initial_prefix,
            initial,
            5,
            &wrong_provider,
            &wrong_provider_charge,
        )
        .expect("wrong-provider decision");
        assert!(matches!(
            wrong_provider_result,
            FutureHoleRealizationDispositionV2::Gap(ref gap)
                if gap.id == "a3_v2_wrong_structural_provider"
                    && gap.phase == FutureHoleGapPhaseV2::Realization
                    && gap.replays()
        ));

        let former = rows
            .iter()
            .find(|row| row.registration_stage == 4)
            .expect("Stage-4 Former registration");
        let prefix = exact_prefix(&signature, 3);
        let filler = &signature.entry(4).expect("Step 4").telescope;
        let correct_charge = issue_filler_ordinary_charge_provenance_v2(
            4,
            filler,
            filler.kappa() as u32,
            0,
            filler.bit_cost(),
            "blake3:correct-step4-charge",
            vec!["blake3:correct-step4-family".to_owned()],
        );

        let mut bit_forgery = correct_charge.clone();
        bit_forgery.bit_length += 1;
        bit_forgery.provenance_hash = filler_charge_expected_hash(&bit_forgery);
        let bit_result =
            realize_structural_future_hole_v2(&prefix, former, 4, filler, &bit_forgery)
                .expect("bit forgery decision");
        assert!(matches!(
            bit_result,
            FutureHoleRealizationDispositionV2::Gap(ref gap)
                if gap.id == "a3_v2_filler_bit_cost_provenance_mismatch"
                    && gap.phase == FutureHoleGapPhaseV2::ChargeJoin
        ));

        let other_filler = &signature.entry(5).expect("Step 5").telescope;
        let swapped_charge = issue_filler_ordinary_charge_provenance_v2(
            5,
            other_filler,
            other_filler.kappa() as u32,
            0,
            other_filler.bit_cost(),
            "blake3:step5-charge-swap-control",
            vec!["blake3:step5-family-swap-control".to_owned()],
        );
        let swap_result =
            realize_structural_future_hole_v2(&prefix, former, 4, filler, &swapped_charge)
                .expect("charge swap decision");
        assert!(matches!(
            swap_result,
            FutureHoleRealizationDispositionV2::Gap(ref gap)
                if gap.id == "a3_v2_filler_charge_subject_mismatch"
                    && gap.phase == FutureHoleGapPhaseV2::ChargeJoin
        ));
    }
}
