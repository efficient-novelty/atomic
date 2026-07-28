use crate::{CanonicalEncode, CanonicalEncoder, Digest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Anonymous global identity. A digest is an identifier, not evidence.
#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct GlobalId(pub Digest);

impl CanonicalEncode for GlobalId {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// Terms in the independently checked dependent-core fragment.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "form", rename_all = "snake_case", deny_unknown_fields)]
pub enum Term {
    Sort {
        level: u16,
    },
    Var {
        index: u32,
    },
    Global {
        id: GlobalId,
    },
    Pi {
        parameter: Box<Term>,
        body: Box<Term>,
    },
    Sigma {
        parameter: Box<Term>,
        body: Box<Term>,
    },
    Lambda {
        parameter_type: Box<Term>,
        body: Box<Term>,
    },
    Apply {
        function: Box<Term>,
        argument: Box<Term>,
    },
    Pair {
        sigma_type: Box<Term>,
        first: Box<Term>,
        second: Box<Term>,
    },
    First {
        pair: Box<Term>,
    },
    Second {
        pair: Box<Term>,
    },
    UnitType,
    Unit,
}

impl CanonicalEncode for Term {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Sort { level } => {
                encoder.tag(0);
                encoder.u16(*level);
            }
            Self::Var { index } => {
                encoder.tag(1);
                encoder.u32(*index);
            }
            Self::Global { id } => {
                encoder.tag(2);
                id.encode_canonical(encoder);
            }
            Self::Pi { parameter, body } => {
                encoder.tag(3);
                parameter.encode_canonical(encoder);
                body.encode_canonical(encoder);
            }
            Self::Sigma { parameter, body } => {
                encoder.tag(4);
                parameter.encode_canonical(encoder);
                body.encode_canonical(encoder);
            }
            Self::Lambda {
                parameter_type,
                body,
            } => {
                encoder.tag(5);
                parameter_type.encode_canonical(encoder);
                body.encode_canonical(encoder);
            }
            Self::Apply { function, argument } => {
                encoder.tag(6);
                function.encode_canonical(encoder);
                argument.encode_canonical(encoder);
            }
            Self::Pair {
                sigma_type,
                first,
                second,
            } => {
                encoder.tag(7);
                sigma_type.encode_canonical(encoder);
                first.encode_canonical(encoder);
                second.encode_canonical(encoder);
            }
            Self::First { pair } => {
                encoder.tag(8);
                pair.encode_canonical(encoder);
            }
            Self::Second { pair } => {
                encoder.tag(9);
                pair.encode_canonical(encoder);
            }
            Self::UnitType => encoder.tag(10),
            Self::Unit => encoder.tag(11),
        }
    }
}

/// An ordered context. Entry `i` may depend only on entries before `i`.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(transparent)]
pub struct DependentContext(pub Vec<Term>);

impl CanonicalEncode for DependentContext {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.0);
    }
}

/// A declaration may refer only to globals declared earlier in its signature.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Declaration {
    pub id: GlobalId,
    pub ty: Term,
    pub body: Option<Term>,
}

impl CanonicalEncode for Declaration {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.ty.encode_canonical(encoder);
        encoder.option(&self.body);
    }
}

/// Deserializable signature input. Verification produces a private normalized
/// handle rather than trusting this representation.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedSignature {
    pub declarations: Vec<Declaration>,
}

impl CanonicalEncode for UncheckedSignature {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.declarations);
    }
}

/// Open judgments are checked under an explicitly supplied dependent context.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "judgment", rename_all = "snake_case", deny_unknown_fields)]
pub enum OpenJudgment {
    TypeFormation {
        context: DependentContext,
        term: Term,
    },
    HasType {
        context: DependentContext,
        term: Term,
        ty: Term,
    },
    DefinitionallyEqual {
        context: DependentContext,
        left: Term,
        right: Term,
        ty: Term,
    },
}

impl OpenJudgment {
    pub fn context(&self) -> &DependentContext {
        match self {
            Self::TypeFormation { context, .. }
            | Self::HasType { context, .. }
            | Self::DefinitionallyEqual { context, .. } => context,
        }
    }
}

impl CanonicalEncode for OpenJudgment {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::TypeFormation { context, term } => {
                encoder.tag(0);
                context.encode_canonical(encoder);
                term.encode_canonical(encoder);
            }
            Self::HasType { context, term, ty } => {
                encoder.tag(1);
                context.encode_canonical(encoder);
                term.encode_canonical(encoder);
                ty.encode_canonical(encoder);
            }
            Self::DefinitionallyEqual {
                context,
                left,
                right,
                ty,
            } => {
                encoder.tag(2);
                context.encode_canonical(encoder);
                left.encode_canonical(encoder);
                right.encode_canonical(encoder);
                ty.encode_canonical(encoder);
            }
        }
    }
}
