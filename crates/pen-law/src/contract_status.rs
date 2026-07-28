//! Unchecked wire representations for currently unresolved Law V2 contracts.
//!
//! These values disclose what an artifact claims and which evidence remains
//! open. Constructing or deserializing them never creates a verified theorem,
//! quotient, cardinality, or bootstrap capability.

use pen_kernel::Digest;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONTRACT_STATUS_SCHEMA_VERSION: u16 = 1;

/// One unresolved comparison between two opaque candidate presentations.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedEquivalenceGoal {
    pub left_representative_digest: Digest,
    pub right_representative_digest: Digest,
    pub goal_artifact_digest: Digest,
}

/// A reference to an artifact claiming an obstruction to an equivalence.
///
/// The referenced artifact still requires an independent verifier. The word
/// "obstruction" in this DTO is descriptive, not proof authority.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedObstructionReference {
    pub left_representative_digest: Digest,
    pub right_representative_digest: Digest,
    pub obstruction_artifact_digest: Digest,
}

/// One claimed equivalence class in an unchecked complete-quotient artifact.
///
/// Neither the class digest nor membership vector is trusted without replay
/// of the containing completeness artifact.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedEquivalenceClass {
    pub class_digest: Digest,
    pub member_representative_digests: Vec<Digest>,
}

/// Adopted quotient status as untrusted wire data.
///
/// Representative count is presentation testimony and does not establish
/// the number of quotient classes.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(
    rename_all = "snake_case",
    tag = "quotient_status",
    deny_unknown_fields
)]
pub enum UncheckedQuotientStatus {
    Incomplete {
        schema_version: u16,
        representative_digests: Vec<Digest>,
        unresolved_equivalence_goals: Vec<UncheckedEquivalenceGoal>,
        claimed_proven_obstructions: Vec<UncheckedObstructionReference>,
    },
    ClaimedComplete {
        schema_version: u16,
        classes: Vec<UncheckedEquivalenceClass>,
        completeness_artifact_digest: Digest,
    },
}

impl UncheckedQuotientStatus {
    pub fn incomplete_representative_digests(&self) -> Option<&[Digest]> {
        match self {
            Self::Incomplete {
                representative_digests,
                ..
            } => Some(representative_digests),
            Self::ClaimedComplete { .. } => None,
        }
    }
}

/// Resource whose exhaustion prevented a finite-fragment decision.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FiniteFragmentResource {
    Operations,
    Depth,
    Normalization,
    ResidentBytes,
    FrontierItems,
}

/// Unchecked result claimed by a particular explicitly finite fragment.
///
/// `Proven` and `Refuted` are theorem-level claims only after a future
/// independent verifier replays their referenced evidence. The other two
/// variants are deliberately nonlogical dispositions.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(
    rename_all = "snake_case",
    tag = "fragment_outcome",
    deny_unknown_fields
)]
pub enum UncheckedFiniteFragmentOutcome {
    Proven {
        schema_version: u16,
        theorem_digest: Digest,
        fragment_digest: Digest,
        proof_artifact_digest: Digest,
    },
    Refuted {
        schema_version: u16,
        theorem_digest: Digest,
        fragment_digest: Digest,
        refutation_artifact_digest: Digest,
    },
    OutsideFragment {
        schema_version: u16,
        theorem_digest: Digest,
        fragment_digest: Digest,
        unsupported_feature_digest: Digest,
    },
    ResourceExhausted {
        schema_version: u16,
        theorem_digest: Digest,
        fragment_digest: Digest,
        resource: FiniteFragmentResource,
        resource_limit_digest: Digest,
    },
}

/// Bootstrap status supplied as unchecked wire data.
///
/// Law V2A discloses a registered founding contract. Law V2B is a stronger
/// derived-bootstrap claim and remains unchecked until a dedicated uniqueness
/// theorem verifier exists.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(
    rename_all = "snake_case",
    tag = "bootstrap_status",
    deny_unknown_fields
)]
pub enum UncheckedBootstrapStatus {
    LawV2ARegistered {
        schema_version: u16,
        bootstrap_contract_digest: Digest,
    },
    ClaimedLawV2BDerived {
        schema_version: u16,
        uniqueness_theorem_digest: Digest,
        proof_artifact_digest: Digest,
    },
}

#[cfg(test)]
mod tests {
    use super::{
        CONTRACT_STATUS_SCHEMA_VERSION, FiniteFragmentResource, UncheckedBootstrapStatus,
        UncheckedEquivalenceClass, UncheckedEquivalenceGoal, UncheckedFiniteFragmentOutcome,
        UncheckedQuotientStatus,
    };
    use pen_kernel::Digest;

    fn digest(label: &[u8]) -> Digest {
        Digest::of_bytes(label)
    }

    #[test]
    fn presentation_count_never_completes_an_unresolved_quotient() {
        for count in 2_u8..7 {
            let representatives = (0..count)
                .map(|index| digest(&[count, index]))
                .collect::<Vec<_>>();
            let status = UncheckedQuotientStatus::Incomplete {
                schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
                representative_digests: representatives.clone(),
                unresolved_equivalence_goals: vec![UncheckedEquivalenceGoal {
                    left_representative_digest: representatives[0].clone(),
                    right_representative_digest: representatives[1].clone(),
                    goal_artifact_digest: digest(&[b'g', count]),
                }],
                claimed_proven_obstructions: Vec::new(),
            };
            assert_eq!(
                status
                    .incomplete_representative_digests()
                    .expect("incomplete representatives")
                    .len(),
                usize::from(count)
            );
            assert!(matches!(
                status,
                UncheckedQuotientStatus::Incomplete {
                    unresolved_equivalence_goals,
                    ..
                } if !unresolved_equivalence_goals.is_empty()
            ));
        }
    }

    #[test]
    fn finite_fragment_nonlogical_dispositions_remain_distinct() {
        let outside = UncheckedFiniteFragmentOutcome::OutsideFragment {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            theorem_digest: digest(b"theorem"),
            fragment_digest: digest(b"fragment"),
            unsupported_feature_digest: digest(b"feature"),
        };
        let exhausted = UncheckedFiniteFragmentOutcome::ResourceExhausted {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            theorem_digest: digest(b"theorem"),
            fragment_digest: digest(b"fragment"),
            resource: FiniteFragmentResource::Operations,
            resource_limit_digest: digest(b"budget"),
        };
        assert_ne!(
            serde_json::to_value(outside).expect("serialize outside"),
            serde_json::to_value(exhausted).expect("serialize exhaustion")
        );
    }

    #[test]
    fn deserialized_complete_and_derived_claims_are_still_named_unchecked() {
        let quotient = UncheckedQuotientStatus::ClaimedComplete {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            classes: vec![UncheckedEquivalenceClass {
                class_digest: digest(b"class"),
                member_representative_digests: vec![digest(b"presentation")],
            }],
            completeness_artifact_digest: digest(b"unchecked-completeness"),
        };
        let quotient_json = serde_json::to_string(&quotient).expect("serialize quotient");
        assert!(matches!(
            serde_json::from_str::<UncheckedQuotientStatus>(&quotient_json)
                .expect("deserialize unchecked quotient"),
            UncheckedQuotientStatus::ClaimedComplete { .. }
        ));

        let bootstrap = UncheckedBootstrapStatus::ClaimedLawV2BDerived {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            uniqueness_theorem_digest: digest(b"unchecked-uniqueness"),
            proof_artifact_digest: digest(b"unchecked-proof"),
        };
        let bootstrap_json = serde_json::to_string(&bootstrap).expect("serialize bootstrap");
        assert!(matches!(
            serde_json::from_str::<UncheckedBootstrapStatus>(&bootstrap_json)
                .expect("deserialize unchecked bootstrap"),
            UncheckedBootstrapStatus::ClaimedLawV2BDerived { .. }
        ));
    }

    #[test]
    fn equivalence_class_claims_reject_unknown_fields() {
        let class = UncheckedEquivalenceClass {
            class_digest: digest(b"class"),
            member_representative_digests: vec![digest(b"presentation")],
        };
        let mut value = serde_json::to_value(class).expect("serialize class");
        value
            .as_object_mut()
            .expect("class object")
            .insert("verified".to_owned(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<UncheckedEquivalenceClass>(value).is_err());
    }
}
