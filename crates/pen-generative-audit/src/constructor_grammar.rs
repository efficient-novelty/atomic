//! JG2a: the finite constructor and public-interface grammar.
//!
//! This module closes the constructor universe that JG1 deliberately left
//! open.  Its proof token says only which seven constructor shapes may be
//! considered by a later extractor.  It does not identify an occurrence of a
//! shape in a stage and grants no authority over later audits.

use crate::{GenerativeCapabilityRoleV1, VerifiedPreExposureGenerativeCapabilityGrammarV1};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::collections::BTreeSet;

pub const GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_SCHEMA_VERSION_V1: u16 = 1;
pub const GENERATIVE_CAPABILITY_CONSTRUCTOR_COUNT_V1: usize = 7;
pub const GENERATIVE_CAPABILITY_INTERFACE_TYPE_COUNT_V1: usize = 9;

const GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2a/generative-capability-constructor-grammar/v1";
const GENERATIVE_CAPABILITY_SCOPE_GRAMMAR_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2a/generative-capability-scope-grammar/v1";
const GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_ROOT_TAG_V1: u8 = 0xb2;
const GENERATIVE_CAPABILITY_CONSTRUCTOR_ROLE_BINDING_TAG_V1: u8 = 0xc0;
const GENERATIVE_CAPABILITY_INTERFACE_SCHEMA_TAG_V1: u8 = 0xd0;

/// The exact finite constructor vocabulary. There is no caller extension
/// point and no catch-all constructor.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeCapabilityConstructorKindV1 {
    Formation,
    Abstraction,
    Aggregation,
    Transport,
    Comparison,
    DemandCompiler,
    DischargeTransformer,
}

pub const GENERATIVE_CAPABILITY_CONSTRUCTORS_V1: [GenerativeCapabilityConstructorKindV1;
    GENERATIVE_CAPABILITY_CONSTRUCTOR_COUNT_V1] = [
    GenerativeCapabilityConstructorKindV1::Formation,
    GenerativeCapabilityConstructorKindV1::Abstraction,
    GenerativeCapabilityConstructorKindV1::Aggregation,
    GenerativeCapabilityConstructorKindV1::Transport,
    GenerativeCapabilityConstructorKindV1::Comparison,
    GenerativeCapabilityConstructorKindV1::DemandCompiler,
    GenerativeCapabilityConstructorKindV1::DischargeTransformer,
];

impl GenerativeCapabilityConstructorKindV1 {
    /// The JG1 role fixed for this constructor. The map is total and cannot be
    /// supplied by a caller.
    pub const fn role(self) -> GenerativeCapabilityRoleV1 {
        match self {
            Self::Formation => GenerativeCapabilityRoleV1::Formation,
            Self::Abstraction => GenerativeCapabilityRoleV1::Abstraction,
            Self::Aggregation => GenerativeCapabilityRoleV1::Aggregation,
            Self::Transport => GenerativeCapabilityRoleV1::Transport,
            Self::Comparison => GenerativeCapabilityRoleV1::Comparison,
            Self::DemandCompiler => GenerativeCapabilityRoleV1::Compiler,
            Self::DischargeTransformer => GenerativeCapabilityRoleV1::DischargeTransformer,
        }
    }
}

impl CanonicalEncode for GenerativeCapabilityConstructorKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Formation => 0,
            Self::Abstraction => 1,
            Self::Aggregation => 2,
            Self::Transport => 3,
            Self::Comparison => 4,
            Self::DemandCompiler => 5,
            Self::DischargeTransformer => 6,
        });
    }
}

/// A proposal-level witness of the derived one-to-one constructor/role map.
/// Verification accepts only the bindings induced by
/// [`GenerativeCapabilityConstructorKindV1::role`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GenerativeCapabilityConstructorRoleBindingV1 {
    pub constructor: GenerativeCapabilityConstructorKindV1,
    pub role: GenerativeCapabilityRoleV1,
}

pub const GENERATIVE_CAPABILITY_CONSTRUCTOR_ROLE_BINDINGS_V1:
    [GenerativeCapabilityConstructorRoleBindingV1; GENERATIVE_CAPABILITY_CONSTRUCTOR_COUNT_V1] = [
    GenerativeCapabilityConstructorRoleBindingV1 {
        constructor: GenerativeCapabilityConstructorKindV1::Formation,
        role: GenerativeCapabilityRoleV1::Formation,
    },
    GenerativeCapabilityConstructorRoleBindingV1 {
        constructor: GenerativeCapabilityConstructorKindV1::Abstraction,
        role: GenerativeCapabilityRoleV1::Abstraction,
    },
    GenerativeCapabilityConstructorRoleBindingV1 {
        constructor: GenerativeCapabilityConstructorKindV1::Aggregation,
        role: GenerativeCapabilityRoleV1::Aggregation,
    },
    GenerativeCapabilityConstructorRoleBindingV1 {
        constructor: GenerativeCapabilityConstructorKindV1::Transport,
        role: GenerativeCapabilityRoleV1::Transport,
    },
    GenerativeCapabilityConstructorRoleBindingV1 {
        constructor: GenerativeCapabilityConstructorKindV1::Comparison,
        role: GenerativeCapabilityRoleV1::Comparison,
    },
    GenerativeCapabilityConstructorRoleBindingV1 {
        constructor: GenerativeCapabilityConstructorKindV1::DemandCompiler,
        role: GenerativeCapabilityRoleV1::Compiler,
    },
    GenerativeCapabilityConstructorRoleBindingV1 {
        constructor: GenerativeCapabilityConstructorKindV1::DischargeTransformer,
        role: GenerativeCapabilityRoleV1::DischargeTransformer,
    },
];

impl CanonicalEncode for GenerativeCapabilityConstructorRoleBindingV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_CAPABILITY_CONSTRUCTOR_ROLE_BINDING_TAG_V1);
        self.constructor.encode_canonical(encoder);
        self.role.encode_canonical(encoder);
    }
}

/// Closed abstract types for finite public constructor interfaces.
///
/// These are target-neutral operational sorts, not kernel terms and not
/// evidence that any stage realizes a constructor. In particular, transport
/// acts on a public interface along a substitution rather than on a selected
/// term; comparison returns its own witness sort; the compiler consumes a
/// sealed public grammar and a demand scheme; and a discharge transformer
/// acts on a discharge rather than merely rewriting a requirement.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeCapabilityInterfaceTypeV1 {
    PublicContext,
    PublicInterface,
    InterfaceFamily,
    Substitution,
    ComparisonWitness,
    SealedPublicGrammar,
    DemandScheme,
    LiveDemand,
    Discharge,
}

pub const GENERATIVE_CAPABILITY_INTERFACE_TYPES_V1: [GenerativeCapabilityInterfaceTypeV1;
    GENERATIVE_CAPABILITY_INTERFACE_TYPE_COUNT_V1] = [
    GenerativeCapabilityInterfaceTypeV1::PublicContext,
    GenerativeCapabilityInterfaceTypeV1::PublicInterface,
    GenerativeCapabilityInterfaceTypeV1::InterfaceFamily,
    GenerativeCapabilityInterfaceTypeV1::Substitution,
    GenerativeCapabilityInterfaceTypeV1::ComparisonWitness,
    GenerativeCapabilityInterfaceTypeV1::SealedPublicGrammar,
    GenerativeCapabilityInterfaceTypeV1::DemandScheme,
    GenerativeCapabilityInterfaceTypeV1::LiveDemand,
    GenerativeCapabilityInterfaceTypeV1::Discharge,
];

impl CanonicalEncode for GenerativeCapabilityInterfaceTypeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::PublicContext => 0,
            Self::PublicInterface => 1,
            Self::InterfaceFamily => 2,
            Self::Substitution => 3,
            Self::ComparisonWitness => 4,
            Self::SealedPublicGrammar => 5,
            Self::DemandScheme => 6,
            Self::LiveDemand => 7,
            Self::Discharge => 8,
        });
    }
}

/// One exact, finite, typed public interface. Shape identity is the ordered
/// pair `(inputs, outputs)`; the frozen seven shapes are pairwise distinct
/// even when the constructor tag is ignored.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GenerativeCapabilityInterfaceSchemaV1 {
    pub constructor: GenerativeCapabilityConstructorKindV1,
    pub inputs: Vec<GenerativeCapabilityInterfaceTypeV1>,
    pub outputs: Vec<GenerativeCapabilityInterfaceTypeV1>,
}

impl CanonicalEncode for GenerativeCapabilityInterfaceSchemaV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_CAPABILITY_INTERFACE_SCHEMA_TAG_V1);
        self.constructor.encode_canonical(encoder);
        encoder.sequence(&self.inputs);
        encoder.sequence(&self.outputs);
    }
}

/// The seven signatures describe kinds of reusable operation, not their
/// implementations. Formation exposes an interface in a public context;
/// abstraction binds such an interface into a family; aggregation exposes a
/// family through one interface; transport reindexes an interface;
/// comparison produces explicit comparison evidence; demand compilation
/// turns a registered scheme over sealed grammar into a live demand; and the
/// final constructor reindexes an already established discharge.
fn exact_interface_schemas_v1() -> Vec<GenerativeCapabilityInterfaceSchemaV1> {
    use GenerativeCapabilityConstructorKindV1 as Constructor;
    use GenerativeCapabilityInterfaceTypeV1 as Ty;

    vec![
        GenerativeCapabilityInterfaceSchemaV1 {
            constructor: Constructor::Formation,
            inputs: vec![Ty::PublicContext],
            outputs: vec![Ty::PublicInterface],
        },
        GenerativeCapabilityInterfaceSchemaV1 {
            constructor: Constructor::Abstraction,
            inputs: vec![Ty::PublicContext, Ty::PublicInterface],
            outputs: vec![Ty::InterfaceFamily],
        },
        GenerativeCapabilityInterfaceSchemaV1 {
            constructor: Constructor::Aggregation,
            inputs: vec![Ty::InterfaceFamily],
            outputs: vec![Ty::PublicInterface],
        },
        GenerativeCapabilityInterfaceSchemaV1 {
            constructor: Constructor::Transport,
            inputs: vec![Ty::Substitution, Ty::PublicInterface],
            outputs: vec![Ty::PublicInterface],
        },
        GenerativeCapabilityInterfaceSchemaV1 {
            constructor: Constructor::Comparison,
            inputs: vec![Ty::PublicInterface, Ty::PublicInterface],
            outputs: vec![Ty::ComparisonWitness],
        },
        GenerativeCapabilityInterfaceSchemaV1 {
            constructor: Constructor::DemandCompiler,
            inputs: vec![Ty::SealedPublicGrammar, Ty::DemandScheme],
            outputs: vec![Ty::LiveDemand],
        },
        GenerativeCapabilityInterfaceSchemaV1 {
            constructor: Constructor::DischargeTransformer,
            inputs: vec![Ty::Substitution, Ty::Discharge],
            outputs: vec![Ty::Discharge],
        },
    ]
}

/// Unverified JG2a proposal. Its JG1 digest prevents the constructor grammar
/// from being replayed under a different pre-exposure grammar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerativeCapabilityConstructorGrammarManifestV1 {
    pub schema_version: u16,
    pub jg1_manifest_digest: Digest,
    pub constructors: Vec<GenerativeCapabilityConstructorKindV1>,
    pub role_bindings: Vec<GenerativeCapabilityConstructorRoleBindingV1>,
    pub interface_types: Vec<GenerativeCapabilityInterfaceTypeV1>,
    pub interface_schemas: Vec<GenerativeCapabilityInterfaceSchemaV1>,
}

impl CanonicalEncode for GenerativeCapabilityConstructorGrammarManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_ROOT_TAG_V1);
        encoder.u16(self.schema_version);
        self.jg1_manifest_digest.encode_canonical(encoder);
        encoder.sequence(&self.constructors);
        encoder.sequence(&self.role_bindings);
        encoder.sequence(&self.interface_types);
        encoder.sequence(&self.interface_schemas);
    }
}

pub fn proposed_generative_capability_constructor_grammar_v1(
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
) -> GenerativeCapabilityConstructorGrammarManifestV1 {
    GenerativeCapabilityConstructorGrammarManifestV1 {
        schema_version: GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_SCHEMA_VERSION_V1,
        jg1_manifest_digest: jg1.manifest_digest().clone(),
        constructors: GENERATIVE_CAPABILITY_CONSTRUCTORS_V1.to_vec(),
        role_bindings: GENERATIVE_CAPABILITY_CONSTRUCTOR_ROLE_BINDINGS_V1.to_vec(),
        interface_types: GENERATIVE_CAPABILITY_INTERFACE_TYPES_V1.to_vec(),
        interface_schemas: exact_interface_schemas_v1(),
    }
}

/// Frozen transcript of the exact JG2a manifest. Its explicit bytes pin all
/// discriminants, arities, orders, types, and the parent JG1 identity.
pub const CANONICAL_GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_BYTES_V1: &[u8] = &[
    0xb2, 0x01, 0x00, 0x47, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x6c, 0x61, 0x6b, 0x65,
    0x33, 0x3a, 0x65, 0x65, 0x65, 0x33, 0x31, 0x31, 0x66, 0x35, 0x66, 0x38, 0x63, 0x36, 0x66, 0x37,
    0x33, 0x64, 0x34, 0x31, 0x65, 0x38, 0x62, 0x32, 0x64, 0x33, 0x64, 0x63, 0x61, 0x30, 0x62, 0x63,
    0x34, 0x62, 0x63, 0x63, 0x39, 0x63, 0x32, 0x65, 0x37, 0x33, 0x36, 0x63, 0x36, 0x33, 0x61, 0x37,
    0x33, 0x30, 0x30, 0x37, 0x33, 0x63, 0x63, 0x37, 0x35, 0x61, 0x63, 0x63, 0x31, 0x65, 0x30, 0x31,
    0x39, 0x32, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
    0x06, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0xc0, 0x01, 0x01, 0xc0,
    0x02, 0x02, 0xc0, 0x03, 0x03, 0xc0, 0x04, 0x04, 0xc0, 0x05, 0x05, 0xc0, 0x06, 0x06, 0x09, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x07,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xd0, 0x01, 0x02, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
    0xd0, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01, 0xd0, 0x03, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x01,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xd0, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xd0, 0x05,
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x07, 0xd0, 0x06, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x08, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08,
];

/// Frozen domain-separated identity of the JG2a manifest transcript.
pub const CANONICAL_GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_DIGEST_V1: &str =
    "blake3:894ea782df934accc9d2efc6b469d144ab73fe2ad2b6b4de096d961cefd7341a";

/// Frozen identity of the combined JG1 + JG2a scope grammar. This is the
/// downstream binding point for a later stage-surface authority.
pub const CANONICAL_GENERATIVE_CAPABILITY_SCOPE_GRAMMAR_DIGEST_V1: &str =
    "blake3:2fcb8a3d72c8719c25b1910a7972beb61dc906709007a320c07bb358402d49b7";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerativeCapabilityConstructorGrammarFailureV1 {
    SchemaVersionMismatch,
    Jg1GrammarDigestMismatch,
    ConstructorVocabularyMismatch,
    RoleMappingMismatch,
    InterfaceTypeVocabularyMismatch,
    InterfaceSchemasNotDisjoint,
    InterfaceSchemaMismatch,
    CanonicalTranscriptMismatch,
    CanonicalDigestMismatch,
    ScopeGrammarDigestMismatch,
}

impl std::fmt::Display for GenerativeCapabilityConstructorGrammarFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SchemaVersionMismatch => "the JG2a schema version is not the frozen version",
            Self::Jg1GrammarDigestMismatch => {
                "the JG2a proposal is not bound to the verified JG1 grammar"
            }
            Self::ConstructorVocabularyMismatch => {
                "the constructor kinds are not the exact frozen vocabulary and order"
            }
            Self::RoleMappingMismatch => {
                "the constructor-to-role map is not the exact derived one-to-one map"
            }
            Self::InterfaceTypeVocabularyMismatch => {
                "the public interface types are not the exact frozen vocabulary and order"
            }
            Self::InterfaceSchemasNotDisjoint => {
                "two public interface schemas have the same typed input/output shape"
            }
            Self::InterfaceSchemaMismatch => {
                "the public interfaces are not the exact frozen schemas and order"
            }
            Self::CanonicalTranscriptMismatch => {
                "the typed JG2a grammar does not encode to the frozen canonical transcript"
            }
            Self::CanonicalDigestMismatch => {
                "the canonical JG2a transcript does not have the frozen domain-separated digest"
            }
            Self::ScopeGrammarDigestMismatch => {
                "the combined JG1 and JG2a grammar does not have the frozen scope digest"
            }
        })
    }
}

impl std::error::Error for GenerativeCapabilityConstructorGrammarFailureV1 {}

/// Opaque proof of the exact finite JG2a constructor and interface grammar.
/// Its private fields prevent callers from manufacturing this boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedGenerativeCapabilityConstructorGrammarV1 {
    schema_version: u16,
    jg1_manifest_digest: Digest,
    constructors: Vec<GenerativeCapabilityConstructorKindV1>,
    role_bindings: Vec<GenerativeCapabilityConstructorRoleBindingV1>,
    interface_types: Vec<GenerativeCapabilityInterfaceTypeV1>,
    interface_schemas: Vec<GenerativeCapabilityInterfaceSchemaV1>,
    manifest_digest: Digest,
    scope_grammar_digest: Digest,
}

impl VerifiedGenerativeCapabilityConstructorGrammarV1 {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn jg1_manifest_digest(&self) -> &Digest {
        &self.jg1_manifest_digest
    }

    pub fn constructors(&self) -> &[GenerativeCapabilityConstructorKindV1] {
        &self.constructors
    }

    pub fn role_bindings(&self) -> &[GenerativeCapabilityConstructorRoleBindingV1] {
        &self.role_bindings
    }

    pub fn interface_types(&self) -> &[GenerativeCapabilityInterfaceTypeV1] {
        &self.interface_types
    }

    pub fn interface_schemas(&self) -> &[GenerativeCapabilityInterfaceSchemaV1] {
        &self.interface_schemas
    }

    pub fn interface_schema(
        &self,
        constructor: GenerativeCapabilityConstructorKindV1,
    ) -> &GenerativeCapabilityInterfaceSchemaV1 {
        self.interface_schemas
            .iter()
            .find(|schema| schema.constructor == constructor)
            .expect("verified JG2a grammar has exactly one schema per constructor")
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn scope_grammar_digest(&self) -> &Digest {
        &self.scope_grammar_digest
    }
}

fn interface_schemas_are_pairwise_disjoint(
    schemas: &[GenerativeCapabilityInterfaceSchemaV1],
) -> bool {
    schemas
        .iter()
        .map(|schema| (&schema.inputs, &schema.outputs))
        .collect::<BTreeSet<_>>()
        .len()
        == schemas.len()
}

pub fn verify_generative_capability_constructor_grammar_v1(
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    manifest: &GenerativeCapabilityConstructorGrammarManifestV1,
) -> Result<
    VerifiedGenerativeCapabilityConstructorGrammarV1,
    GenerativeCapabilityConstructorGrammarFailureV1,
> {
    if manifest.schema_version != GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_SCHEMA_VERSION_V1 {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::SchemaVersionMismatch);
    }
    if &manifest.jg1_manifest_digest != jg1.manifest_digest() {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::Jg1GrammarDigestMismatch);
    }
    if manifest.constructors.as_slice() != GENERATIVE_CAPABILITY_CONSTRUCTORS_V1 {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::ConstructorVocabularyMismatch);
    }
    if manifest.role_bindings.as_slice() != GENERATIVE_CAPABILITY_CONSTRUCTOR_ROLE_BINDINGS_V1 {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::RoleMappingMismatch);
    }
    if manifest.interface_types.as_slice() != GENERATIVE_CAPABILITY_INTERFACE_TYPES_V1 {
        return Err(
            GenerativeCapabilityConstructorGrammarFailureV1::InterfaceTypeVocabularyMismatch,
        );
    }
    if !interface_schemas_are_pairwise_disjoint(&manifest.interface_schemas) {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::InterfaceSchemasNotDisjoint);
    }
    if manifest.interface_schemas != exact_interface_schemas_v1() {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::InterfaceSchemaMismatch);
    }

    let mut encoder = CanonicalEncoder::new();
    manifest.encode_canonical(&mut encoder);
    if encoder.as_bytes() != CANONICAL_GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_BYTES_V1 {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::CanonicalTranscriptMismatch);
    }

    let manifest_digest = Digest::of_domain_bytes(
        GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_DIGEST_DOMAIN_V1,
        encoder.as_bytes(),
    );
    if manifest_digest.as_str() != CANONICAL_GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_DIGEST_V1 {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::CanonicalDigestMismatch);
    }

    let scope_grammar_digest = Digest::of_domain_chunks(
        GENERATIVE_CAPABILITY_SCOPE_GRAMMAR_DIGEST_DOMAIN_V1,
        &[
            jg1.manifest_digest().as_str().as_bytes(),
            manifest_digest.as_str().as_bytes(),
        ],
    );
    if scope_grammar_digest.as_str() != CANONICAL_GENERATIVE_CAPABILITY_SCOPE_GRAMMAR_DIGEST_V1 {
        return Err(GenerativeCapabilityConstructorGrammarFailureV1::ScopeGrammarDigestMismatch);
    }

    Ok(VerifiedGenerativeCapabilityConstructorGrammarV1 {
        schema_version: manifest.schema_version,
        jg1_manifest_digest: manifest.jg1_manifest_digest.clone(),
        constructors: manifest.constructors.clone(),
        role_bindings: manifest.role_bindings.clone(),
        interface_types: manifest.interface_types.clone(),
        interface_schemas: manifest.interface_schemas.clone(),
        manifest_digest,
        scope_grammar_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        proposed_pre_exposure_generative_capability_grammar_v1,
        verify_pre_exposure_generative_capability_grammar_v1,
    };

    fn verified_jg1() -> VerifiedPreExposureGenerativeCapabilityGrammarV1 {
        verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("the exact JG1 proposal must verify")
    }

    fn proposal() -> GenerativeCapabilityConstructorGrammarManifestV1 {
        proposed_generative_capability_constructor_grammar_v1(&verified_jg1())
    }

    fn verified_jg2a() -> VerifiedGenerativeCapabilityConstructorGrammarV1 {
        let jg1 = verified_jg1();
        verify_generative_capability_constructor_grammar_v1(
            &jg1,
            &proposed_generative_capability_constructor_grammar_v1(&jg1),
        )
        .expect("the exact JG2a proposal must verify")
    }

    #[test]
    fn exact_constructor_grammar_is_finite_role_locked_and_disjoint() {
        let verified = verified_jg2a();

        assert_eq!(verified.schema_version(), 1);
        assert_eq!(
            verified.constructors(),
            GENERATIVE_CAPABILITY_CONSTRUCTORS_V1
        );
        assert_eq!(
            verified.role_bindings(),
            GENERATIVE_CAPABILITY_CONSTRUCTOR_ROLE_BINDINGS_V1
        );
        assert_eq!(
            verified.interface_types(),
            GENERATIVE_CAPABILITY_INTERFACE_TYPES_V1
        );
        assert_eq!(verified.interface_schemas().len(), 7);
        assert!(interface_schemas_are_pairwise_disjoint(
            verified.interface_schemas()
        ));

        for (index, constructor) in verified.constructors().iter().copied().enumerate() {
            assert_eq!(verified.role_bindings()[index].constructor, constructor);
            assert_eq!(verified.role_bindings()[index].role, constructor.role());
            assert_eq!(verified.interface_schemas()[index].constructor, constructor);
            assert_eq!(
                verified.interface_schema(constructor),
                &verified.interface_schemas()[index]
            );
        }
    }

    #[test]
    fn signatures_name_generic_operations_without_term_level_narrowing() {
        use GenerativeCapabilityConstructorKindV1 as Constructor;
        use GenerativeCapabilityInterfaceTypeV1 as Ty;

        let verified = verified_jg2a();
        let assert_shape = |constructor, inputs: &[Ty], outputs: &[Ty]| {
            let schema = verified.interface_schema(constructor);
            assert_eq!(schema.inputs.as_slice(), inputs);
            assert_eq!(schema.outputs.as_slice(), outputs);
        };

        assert_shape(
            Constructor::Formation,
            &[Ty::PublicContext],
            &[Ty::PublicInterface],
        );
        assert_shape(
            Constructor::Abstraction,
            &[Ty::PublicContext, Ty::PublicInterface],
            &[Ty::InterfaceFamily],
        );
        assert_shape(
            Constructor::Aggregation,
            &[Ty::InterfaceFamily],
            &[Ty::PublicInterface],
        );
        assert_shape(
            Constructor::Transport,
            &[Ty::Substitution, Ty::PublicInterface],
            &[Ty::PublicInterface],
        );
        assert_shape(
            Constructor::Comparison,
            &[Ty::PublicInterface, Ty::PublicInterface],
            &[Ty::ComparisonWitness],
        );
        assert_shape(
            Constructor::DemandCompiler,
            &[Ty::SealedPublicGrammar, Ty::DemandScheme],
            &[Ty::LiveDemand],
        );
        assert_eq!(
            Constructor::DemandCompiler.role(),
            GenerativeCapabilityRoleV1::Compiler
        );
        assert_shape(
            Constructor::DischargeTransformer,
            &[Ty::Substitution, Ty::Discharge],
            &[Ty::Discharge],
        );
    }

    #[test]
    fn canonical_transcript_and_both_digests_are_deterministic() {
        let jg1 = verified_jg1();
        let proposal = proposed_generative_capability_constructor_grammar_v1(&jg1);
        let mut encoder = CanonicalEncoder::new();
        proposal.encode_canonical(&mut encoder);
        assert_eq!(
            encoder.as_bytes(),
            CANONICAL_GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_BYTES_V1
        );

        let expected_manifest_digest = Digest::of_domain_bytes(
            GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_DIGEST_DOMAIN_V1,
            CANONICAL_GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_BYTES_V1,
        );
        assert_eq!(
            expected_manifest_digest.as_str(),
            CANONICAL_GENERATIVE_CAPABILITY_CONSTRUCTOR_GRAMMAR_DIGEST_V1
        );
        let expected_scope_digest = Digest::of_domain_chunks(
            GENERATIVE_CAPABILITY_SCOPE_GRAMMAR_DIGEST_DOMAIN_V1,
            &[
                jg1.manifest_digest().as_str().as_bytes(),
                expected_manifest_digest.as_str().as_bytes(),
            ],
        );
        assert_eq!(
            expected_scope_digest.as_str(),
            CANONICAL_GENERATIVE_CAPABILITY_SCOPE_GRAMMAR_DIGEST_V1
        );

        let first = verified_jg2a();
        let second = verified_jg2a();
        assert_eq!(first, second);
        assert_eq!(first.jg1_manifest_digest(), jg1.manifest_digest());
        assert_eq!(first.manifest_digest(), &expected_manifest_digest);
        assert_eq!(first.scope_grammar_digest(), &expected_scope_digest);
    }

    #[test]
    fn schema_parent_vocabulary_order_and_mapping_mutations_fail_closed() {
        let jg1 = verified_jg1();

        let mut mutation = proposal();
        mutation.schema_version += 1;
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::SchemaVersionMismatch)
        );

        let mut mutation = proposal();
        mutation.jg1_manifest_digest = Digest::of_domain_bytes("wrong-jg1", b"wrong");
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::Jg1GrammarDigestMismatch)
        );

        let mut mutation = proposal();
        mutation.constructors.swap(0, 1);
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::ConstructorVocabularyMismatch)
        );

        let mut mutation = proposal();
        mutation.role_bindings[0].role = GenerativeCapabilityRoleV1::Compiler;
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::RoleMappingMismatch)
        );

        let mut mutation = proposal();
        mutation.interface_types.swap(0, 1);
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::InterfaceTypeVocabularyMismatch)
        );
    }

    #[test]
    fn interface_omission_duplication_order_typing_and_shape_mutations_fail_closed() {
        let jg1 = verified_jg1();

        let mut mutation = proposal();
        mutation.interface_schemas.pop();
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::InterfaceSchemaMismatch)
        );

        let mut mutation = proposal();
        mutation.interface_schemas[1] = mutation.interface_schemas[0].clone();
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::InterfaceSchemasNotDisjoint)
        );

        let mut mutation = proposal();
        mutation.interface_schemas.swap(0, 1);
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::InterfaceSchemaMismatch)
        );

        let mut mutation = proposal();
        mutation.interface_schemas[0].outputs[0] =
            GenerativeCapabilityInterfaceTypeV1::ComparisonWitness;
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::InterfaceSchemaMismatch)
        );

        let mut mutation = proposal();
        mutation.interface_schemas[1].inputs.reverse();
        assert_eq!(
            verify_generative_capability_constructor_grammar_v1(&jg1, &mutation),
            Err(GenerativeCapabilityConstructorGrammarFailureV1::InterfaceSchemaMismatch)
        );
    }
}
