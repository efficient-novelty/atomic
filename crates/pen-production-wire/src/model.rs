//! Authority-free data model for the production-refinement V1 bundle.

pub const PRODUCTION_REFINEMENT_MAGIC_V1: [u8; 16] = *b"PEN-PROD-WIRE-V1";
pub const PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1: u16 = 1;
pub const PRODUCTION_REFINEMENT_SECTION_COUNT_V1: u16 = 11;
pub const V3_SEMANTIC_SCHEMA_VERSION: u16 = 3;
pub const V3_SEMANTIC_PROFILE_ID: &[u8] = b"gf2-semantic-audit-lambda-unit-v3";
pub const SYNTHESIS_PROTOCOL_ID_V2: &[u8] = b"pen-kernel-synthesis/lambda-unit/v2";
pub const SYNTHESIS_SCHEMA_VERSION_V2: u16 = 2;
pub const PUBLIC_UNIVERSE_LEVELS_V1: [u16; 2] = [0, 1];
pub const CHECKER_UNIVERSE_LEVELS_V1: [u16; 3] = [0, 1, 2];
pub const FORMATION_WITNESS_LEVELS_V1: [u16; 4] = [0, 1, 2, 3];

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WireIdV1(pub [u8; 32]);

impl WireIdV1 {
    pub const ZERO: Self = Self([0; 32]);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WireHeaderV1 {
    pub magic: [u8; 16],
    pub schema_version: u16,
    pub section_count: u16,
}

impl WireHeaderV1 {
    pub const fn canonical() -> Self {
        Self {
            magic: PRODUCTION_REFINEMENT_MAGIC_V1,
            schema_version: PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1,
            section_count: PRODUCTION_REFINEMENT_SECTION_COUNT_V1,
        }
    }
}

impl Default for WireHeaderV1 {
    fn default() -> Self {
        Self::canonical()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManifestAuthorityWireV1 {
    GenericPrototypeOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V3CorrespondenceManifestWireV1 {
    pub semantic_schema_version: u16,
    pub profile_id: Vec<u8>,
    pub semantic_manifest_digest: WireIdV1,
    pub authority: ManifestAuthorityWireV1,
    pub frozen: bool,
    pub live_profile_a_access: bool,
    pub production_inventory_bridge_digest: WireIdV1,
    pub public_universe_levels: Vec<u16>,
    pub checker_universe_levels: Vec<u16>,
    pub formation_witness_levels: Vec<u16>,
    pub maximum_context_entries: u16,
    pub synthesis_rule_inventory: Vec<SynthesisRuleWireV1>,
    pub predecessor_delta_policy_binding_digest: WireIdV1,
    pub synthesis_protocol_id: Vec<u8>,
    pub synthesis_schema_version: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeltaPolicyEntryWireV1 {
    pub global_slot: u32,
    pub global_id_bytes: WireIdV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionSignatureWireV1 {
    pub signature_digest: WireIdV1,
    pub kernel_protocol_digest: WireIdV1,
    pub global_slot_table_digest: WireIdV1,
    pub allowed_transparent_deltas: Vec<DeltaPolicyEntryWireV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WireTermV1 {
    Sort {
        level: u16,
    },
    Variable {
        index: u32,
    },
    GlobalSlot {
        slot: u32,
    },
    Pi {
        parameter: Box<WireTermV1>,
        body: Box<WireTermV1>,
    },
    Lambda {
        parameter: Box<WireTermV1>,
        body: Box<WireTermV1>,
    },
    Apply {
        function: Box<WireTermV1>,
        argument: Box<WireTermV1>,
    },
    UnitType,
    Unit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSlotEntryWireV1 {
    pub slot: u32,
    pub global_id_bytes: WireIdV1,
    pub declaration_type: WireTermV1,
    pub declaration_body: Option<WireTermV1>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GlobalSlotTableWireV1 {
    pub entries: Vec<GlobalSlotEntryWireV1>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProductionContextWireV1 {
    pub entries_oldest_first: Vec<WireTermV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConversionPathComponentWireV1 {
    PiParameter,
    PiBody,
    LambdaParameter,
    LambdaBody,
    ApplyFunction,
    ApplyArgument,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NoRedexDispositionWireV1 {
    Sort,
    Variable,
    GlobalNotEnabledByPolicy,
    Pi,
    Lambda,
    NeutralApplication,
    UnitType,
    Unit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoRedexEntryWireV1 {
    pub path: Vec<ConversionPathComponentWireV1>,
    pub term: WireTermV1,
    pub disposition: NoRedexDispositionWireV1,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NoRedexCensusWireV1 {
    pub entries: Vec<NoRedexEntryWireV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BaseQ0ReductionStepWireV1 {
    Beta {
        source: WireTermV1,
        target: WireTermV1,
    },
    TransparentDelta {
        source: WireTermV1,
        target: WireTermV1,
        global_slot: u32,
    },
    PiParameterCongruence {
        source: WireTermV1,
        target: WireTermV1,
        premise: Box<BaseQ0ReductionStepWireV1>,
    },
    PiBodyCongruence {
        source: WireTermV1,
        target: WireTermV1,
        premise: Box<BaseQ0ReductionStepWireV1>,
    },
    LambdaParameterCongruence {
        source: WireTermV1,
        target: WireTermV1,
        premise: Box<BaseQ0ReductionStepWireV1>,
    },
    LambdaBodyCongruence {
        source: WireTermV1,
        target: WireTermV1,
        premise: Box<BaseQ0ReductionStepWireV1>,
    },
    ApplyFunctionCongruence {
        source: WireTermV1,
        target: WireTermV1,
        premise: Box<BaseQ0ReductionStepWireV1>,
    },
    ApplyArgumentCongruence {
        source: WireTermV1,
        target: WireTermV1,
        premise: Box<BaseQ0ReductionStepWireV1>,
    },
}

impl BaseQ0ReductionStepWireV1 {
    pub fn source(&self) -> &WireTermV1 {
        match self {
            Self::Beta { source, .. }
            | Self::TransparentDelta { source, .. }
            | Self::PiParameterCongruence { source, .. }
            | Self::PiBodyCongruence { source, .. }
            | Self::LambdaParameterCongruence { source, .. }
            | Self::LambdaBodyCongruence { source, .. }
            | Self::ApplyFunctionCongruence { source, .. }
            | Self::ApplyArgumentCongruence { source, .. } => source,
        }
    }

    pub fn target(&self) -> &WireTermV1 {
        match self {
            Self::Beta { target, .. }
            | Self::TransparentDelta { target, .. }
            | Self::PiParameterCongruence { target, .. }
            | Self::PiBodyCongruence { target, .. }
            | Self::LambdaParameterCongruence { target, .. }
            | Self::LambdaBodyCongruence { target, .. }
            | Self::ApplyFunctionCongruence { target, .. }
            | Self::ApplyArgumentCongruence { target, .. } => target,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BaseQ0ReductionTraceWireV1 {
    pub start: WireTermV1,
    pub steps: Vec<BaseQ0ReductionStepWireV1>,
    pub end: WireTermV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EndpointJudgmentWireV1 {
    HasType { expected_type: WireTermV1 },
    TypeFormation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversionCertificateWireV1 {
    pub conversion_id: WireIdV1,
    pub context: ProductionContextWireV1,
    pub left: WireTermV1,
    pub right: WireTermV1,
    pub endpoint_judgment: EndpointJudgmentWireV1,
    pub common_normal_form: WireTermV1,
    pub left_trace: BaseQ0ReductionTraceWireV1,
    pub right_trace: BaseQ0ReductionTraceWireV1,
    pub no_redex_census: NoRedexCensusWireV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SynthesisCodeWireV1 {
    Sort {
        level: u16,
    },
    UnitType,
    Unit,
    VariableLookup {
        index: u32,
        context_ordinal: u32,
        shift_distance: u32,
    },
    GlobalLookup {
        global_slot: u32,
    },
    PiFormation {
        parameter: Box<SynthesisCodeWireV1>,
        body: Box<SynthesisCodeWireV1>,
    },
    LambdaIntroduction {
        parameter_type: Box<SynthesisCodeWireV1>,
        body: Box<SynthesisCodeWireV1>,
    },
    ApplicationElimination {
        function: Box<SynthesisCodeWireV1>,
        argument: Box<SynthesisCodeWireV1>,
        function_conversion_id: WireIdV1,
        argument_conversion_id: WireIdV1,
        dependent_result_type: WireTermV1,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SynthesisCertificateWireV1 {
    pub synthesis_id: WireIdV1,
    pub context: ProductionContextWireV1,
    pub subject: WireTermV1,
    pub inferred_type: WireTermV1,
    pub code: SynthesisCodeWireV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversionTypingSupplementWireV1 {
    pub conversion_id: WireIdV1,
    pub step_path: Vec<u32>,
    pub local_context: ProductionContextWireV1,
    pub local_endpoint_judgment: EndpointJudgmentWireV1,
    pub source_typing_code: SynthesisCertificateWireV1,
    pub target_typing_code: SynthesisCertificateWireV1,
    pub formation_level: Option<u16>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Q0RuleWireV1 {
    DeBruijn,
    SequentialSubstitution,
    Beta,
    ProvenancePreservingDelta,
    Unit,
    TelescopeFlattening,
    FreshNonrecursiveConstructorComputation,
}

pub const EXACT_Q0_INVENTORY_V1: [Q0RuleWireV1; 7] = [
    Q0RuleWireV1::DeBruijn,
    Q0RuleWireV1::SequentialSubstitution,
    Q0RuleWireV1::Beta,
    Q0RuleWireV1::ProvenancePreservingDelta,
    Q0RuleWireV1::Unit,
    Q0RuleWireV1::TelescopeFlattening,
    Q0RuleWireV1::FreshNonrecursiveConstructorComputation,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Q0InventoryWireV1 {
    pub ordered_rules: Vec<Q0RuleWireV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshRuleSchemaWireV1 {
    pub equation_id: WireIdV1,
    pub owner_slot: u32,
    pub constructor_slot: u32,
    pub parameter_context: ProductionContextWireV1,
    pub left: WireTermV1,
    pub right: WireTermV1,
    pub ty: WireTermV1,
    pub scrutinee_ordinal: u32,
    pub arity: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FamilyCodeWireV1 {
    Seed,
    GenericPublicApplication,
    GenericEquationAction,
}

pub const EXACT_FAMILY_INVENTORY_V1: [FamilyCodeWireV1; 3] = [
    FamilyCodeWireV1::Seed,
    FamilyCodeWireV1::GenericPublicApplication,
    FamilyCodeWireV1::GenericEquationAction,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FamilyInventoryWireV1 {
    pub ordered_codes: Vec<FamilyCodeWireV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FamilyJudgmentWireV1 {
    pub context: ProductionContextWireV1,
    pub subject: WireTermV1,
    pub ty: WireTermV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SeedSourceWireV1 {
    PublicHead { owner_slot: u32 },
    PublicEquation { equation_id: WireIdV1 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FamilyPayloadWireV1 {
    Seed {
        family_id: WireIdV1,
        source: SeedSourceWireV1,
        judgment: FamilyJudgmentWireV1,
    },
    GenericPublicApplication {
        family_id: WireIdV1,
        function_family_id: WireIdV1,
        argument_family_id: WireIdV1,
        judgment: FamilyJudgmentWireV1,
    },
    GenericEquationAction {
        family_id: WireIdV1,
        equation_id: WireIdV1,
        source_family_id: WireIdV1,
        judgment: FamilyJudgmentWireV1,
    },
}

impl FamilyPayloadWireV1 {
    pub fn family_id(&self) -> WireIdV1 {
        match self {
            Self::Seed { family_id, .. }
            | Self::GenericPublicApplication { family_id, .. }
            | Self::GenericEquationAction { family_id, .. } => *family_id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionRefinementBundleV1 {
    pub header: WireHeaderV1,
    pub manifest_surface: V3CorrespondenceManifestWireV1,
    pub signature: ProductionSignatureWireV1,
    pub global_slot_table: GlobalSlotTableWireV1,
    pub contexts: Vec<ProductionContextWireV1>,
    pub conversions: Vec<ConversionCertificateWireV1>,
    pub conversion_typing_supplements: Vec<ConversionTypingSupplementWireV1>,
    pub synthesis_codes: Vec<SynthesisCertificateWireV1>,
    pub q0_inventory: Q0InventoryWireV1,
    pub fresh_rule_schemas: Vec<FreshRuleSchemaWireV1>,
    pub family_inventory: FamilyInventoryWireV1,
    pub family_payloads: Vec<FamilyPayloadWireV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SynthesisRuleWireV1 {
    Sort,
    UnitType,
    Unit,
    VariableLookup,
    GlobalLookup,
    PiFormation,
    LambdaIntroduction,
    ApplicationElimination,
}

pub const EXACT_SYNTHESIS_INVENTORY_V1: [SynthesisRuleWireV1; 8] = [
    SynthesisRuleWireV1::Sort,
    SynthesisRuleWireV1::UnitType,
    SynthesisRuleWireV1::Unit,
    SynthesisRuleWireV1::VariableLookup,
    SynthesisRuleWireV1::GlobalLookup,
    SynthesisRuleWireV1::PiFormation,
    SynthesisRuleWireV1::LambdaIntroduction,
    SynthesisRuleWireV1::ApplicationElimination,
];
