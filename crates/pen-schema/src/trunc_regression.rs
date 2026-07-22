//! E-1 regression bridge for the four registered TRUNC-ER endpoint maps.
//!
//! The maps are first derived by the new dependent context judgement, then
//! compared with the already shipped TRUNC-ER projections.  The old token is
//! consumed read-only; it is not treated as the definition of the new maps.

use crate::context::{
    BinderId, Declaration, FormedSchemaContext, SchemaContextError, SubstitutionImage, TermExpr,
    TypeExpr, TypedSubstitutionToken, form_schema_context, issue_typed_substitution,
    replay_typed_substitution,
};
use pen_type::cubical::typed_boundary::{
    TruncRestrictedInstanceKind, issue_trunc_endpoint_v3_c6_bundle_token,
    replay_trunc_endpoint_v3_c6_bundle_token,
};
use pen_type::elaborate::SealedSignature;
use serde::Serialize;
use thiserror::Error;

pub const TRUNC_E1_REGRESSION_VERSION: &str = "schema2-e1-trunc-endpoint-map-regression-v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TruncEndpointMapKind {
    Identity,
    Swap,
    CollapseToX,
    CollapseToY,
}

impl TruncEndpointMapKind {
    pub const ALL: [Self; 4] = [
        Self::Identity,
        Self::Swap,
        Self::CollapseToX,
        Self::CollapseToY,
    ];

    const fn targets(self) -> (u32, u32) {
        match self {
            Self::Identity => (2, 3),
            Self::Swap => (3, 2),
            Self::CollapseToX => (2, 2),
            Self::CollapseToY => (3, 3),
        }
    }

    const fn legacy(self) -> TruncRestrictedInstanceKind {
        match self {
            Self::Identity => TruncRestrictedInstanceKind::Identity,
            Self::Swap => TruncRestrictedInstanceKind::Swap,
            Self::CollapseToX => TruncRestrictedInstanceKind::CollapseToX,
            Self::CollapseToY => TruncRestrictedInstanceKind::CollapseToY,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TruncEndpointMapInstance {
    kind: TruncEndpointMapKind,
    zero_target: u32,
    one_target: u32,
    schema2_substitution: TypedSubstitutionToken,
    legacy_instance_derivation_hash: String,
}

impl TruncEndpointMapInstance {
    pub const fn kind(&self) -> TruncEndpointMapKind {
        self.kind
    }

    pub const fn endpoints(&self) -> (u32, u32) {
        (self.zero_target, self.one_target)
    }

    pub fn substitution(&self) -> &TypedSubstitutionToken {
        &self.schema2_substitution
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TruncEndpointMapRegressionToken {
    version: String,
    context: FormedSchemaContext,
    legacy_bundle_derivation_hash: String,
    instances: Vec<TruncEndpointMapInstance>,
    exact_four_map_join: bool,
    derivation_hash: String,
}

impl TruncEndpointMapRegressionToken {
    pub fn instances(&self) -> &[TruncEndpointMapInstance] {
        &self.instances
    }

    pub const fn exact_four_map_join(&self) -> bool {
        self.exact_four_map_join
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TruncRegressionError {
    #[error("schema context check failed: {0}")]
    Context(#[from] SchemaContextError),
    #[error("TRUNC-ER replay failed: {0}")]
    Legacy(String),
    #[error("new and legacy endpoint-map inventories differ")]
    InventoryMismatch,
    #[error("TRUNC endpoint regression replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(TRUNC_E1_REGRESSION_VERSION, domain, payload))
        .expect("TRUNC regression proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

pub fn trunc_schema_context() -> Result<FormedSchemaContext, SchemaContextError> {
    form_schema_context(vec![
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
        Declaration::OpaqueElement {
            binder: BinderId(3),
            name: "y".to_owned(),
            ty: TypeExpr::trunc(TypeExpr::parameter(1)),
        },
    ])
}

fn issue_map(
    context: &FormedSchemaContext,
    kind: TruncEndpointMapKind,
) -> Result<TypedSubstitutionToken, SchemaContextError> {
    let (zero_target, one_target) = kind.targets();
    issue_typed_substitution(
        context,
        context,
        vec![
            SubstitutionImage::Type {
                source: BinderId(1),
                image: TypeExpr::parameter(1),
            },
            SubstitutionImage::Term {
                source: BinderId(2),
                image: TermExpr::variable(zero_target),
            },
            SubstitutionImage::Term {
                source: BinderId(3),
                image: TermExpr::variable(one_target),
            },
        ],
    )
}

pub fn issue_trunc_endpoint_map_regression()
-> Result<TruncEndpointMapRegressionToken, TruncRegressionError> {
    let context = trunc_schema_context()?;
    let signature = SealedSignature::genesis_del_h15();
    let legacy_bundle = issue_trunc_endpoint_v3_c6_bundle_token(&signature)
        .map_err(|error| TruncRegressionError::Legacy(error.to_string()))?;
    replay_trunc_endpoint_v3_c6_bundle_token(&signature, &legacy_bundle)
        .map_err(|error| TruncRegressionError::Legacy(error.to_string()))?;
    let legacy_instances = legacy_bundle
        .restricted_instantiation()
        .variable_instances();
    let mut instances = Vec::with_capacity(4);
    for kind in TruncEndpointMapKind::ALL {
        let substitution = issue_map(&context, kind)?;
        replay_typed_substitution(&substitution)?;
        let (zero_target, one_target) = kind.targets();
        let Some(legacy) = legacy_instances
            .iter()
            .find(|instance| instance.kind() == kind.legacy())
        else {
            return Err(TruncRegressionError::InventoryMismatch);
        };
        if legacy.zero_target_variable() != zero_target
            || legacy.one_target_variable() != one_target
        {
            return Err(TruncRegressionError::InventoryMismatch);
        }
        instances.push(TruncEndpointMapInstance {
            kind,
            zero_target,
            one_target,
            schema2_substitution: substitution,
            legacy_instance_derivation_hash: legacy.derivation_hash().to_owned(),
        });
    }
    let exact_four_map_join = instances.len() == 4 && legacy_instances.len() == 4;
    if !exact_four_map_join {
        return Err(TruncRegressionError::InventoryMismatch);
    }
    let version = TRUNC_E1_REGRESSION_VERSION.to_owned();
    let legacy_bundle_derivation_hash = legacy_bundle.derivation_hash().to_owned();
    let derivation_hash = tagged_hash(
        "exact-four-trunc-endpoint-map-join",
        &(
            &version,
            &context,
            &legacy_bundle_derivation_hash,
            &instances,
            exact_four_map_join,
        ),
    );
    Ok(TruncEndpointMapRegressionToken {
        version,
        context,
        legacy_bundle_derivation_hash,
        instances,
        exact_four_map_join,
        derivation_hash,
    })
}

pub fn replay_trunc_endpoint_map_regression(
    token: &TruncEndpointMapRegressionToken,
) -> Result<(), TruncRegressionError> {
    let replay = issue_trunc_endpoint_map_regression()?;
    if replay == *token {
        Ok(())
    } else {
        Err(TruncRegressionError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_trunc_endpoint_maps_rederive_as_dependent_typed_instances() {
        let token = issue_trunc_endpoint_map_regression().expect("E-1 TRUNC regression");
        assert!(token.exact_four_map_join());
        assert_eq!(
            token
                .instances()
                .iter()
                .map(|instance| (instance.kind(), instance.endpoints()))
                .collect::<Vec<_>>(),
            vec![
                (TruncEndpointMapKind::Identity, (2, 3)),
                (TruncEndpointMapKind::Swap, (3, 2)),
                (TruncEndpointMapKind::CollapseToX, (2, 2)),
                (TruncEndpointMapKind::CollapseToY, (3, 3)),
            ]
        );
        assert!(token.instances().iter().all(|instance| {
            instance.substitution().dependent_images_checked() == 2
                && instance.substitution().genuine_expression_images() == 0
        }));
        replay_trunc_endpoint_map_regression(&token).expect("definition replay");
    }

    #[test]
    fn regression_token_mutation_is_detected() {
        let token = issue_trunc_endpoint_map_regression().unwrap();
        let mut mutated = token.clone();
        mutated.exact_four_map_join = false;
        assert_eq!(
            replay_trunc_endpoint_map_regression(&mutated),
            Err(TruncRegressionError::ReplayMismatch)
        );
    }
}
