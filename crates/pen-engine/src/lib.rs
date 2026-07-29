//! Oracle-free orchestration boundary.
//!
//! The incomplete Law V2 lane intentionally has no `Ready`, `Advance`,
//! `Blocked`, or `Halt` constructor. It can report only `Unknown` until every
//! proof layer needed by the two laws is installed.

#![forbid(unsafe_code)]

mod h3_inductive_completion;

pub use h3_inductive_completion::*;

use pen_kernel::{Kernel, KernelError, KernelLimits};
use pen_law::{
    DEMAND_WINDOW_WIDTH, Digest, RegisteredBootstrapError, UncheckedBootstrapStatus,
    UncheckedFiniteFragmentOutcome, UncheckedQuotientStatus, load_embedded_registered_bootstrap,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceLimits {
    pub max_resident_bytes: u64,
    pub max_frontier_items: u64,
    pub worker_count: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LawfulRuntimeConfig {
    pub window_width: u8,
    pub kernel_digest: Digest,
    pub grammar_digest: Digest,
    pub scheme_calculus_digest: Digest,
    pub law_digest: Digest,
    pub resources: ResourceLimits,
}

impl LawfulRuntimeConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.window_width != DEMAND_WINDOW_WIDTH {
            return Err(ConfigError::WrongWindowWidth {
                expected: DEMAND_WINDOW_WIDTH,
                actual: self.window_width,
            });
        }
        if self.resources.max_resident_bytes == 0
            || self.resources.max_frontier_items == 0
            || self.resources.worker_count == 0
        {
            return Err(ConfigError::ZeroResourceLimit);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ConfigError {
    #[error("law-v2 requires window width {expected}, received {actual}")]
    WrongWindowWidth { expected: u8, actual: u8 },
    #[error("resource limits must be positive")]
    ZeroResourceLimit,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingComponent {
    AuthoritativeFiniteFragmentKernelBackend,
    AuthoritativeDemandSchemeCalculus,
    SchemeGenerationCompletenessTheorem,
    AuthoritativeDemandCensus,
    CompleteAdoptedQuotient,
    FiniteFragmentTheoremVerifier,
    ResponseCone,
    SemanticAudit,
    FreeSealingUniversalProperty,
}

/// Bootstrap contract selected by the current executable.
///
/// This discloses the registered Law-V2A contract; it is not evidence that
/// founding acts were derived or replayed. There is deliberately no Law-V2B
/// selection until bootstrap uniqueness has a verifier and private handle.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectedBootstrapContract {
    LawV2ARegisteredContract,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReadinessManifest {
    missing: Vec<MissingComponent>,
    selected_bootstrap_contract: SelectedBootstrapContract,
    verified_registered_bootstrap_digest: Option<Digest>,
}

impl ReadinessManifest {
    pub fn certificate_types_milestone() -> Self {
        Self::with_registered_bootstrap(None)
    }

    fn with_registered_bootstrap(verified_registered_bootstrap_digest: Option<Digest>) -> Self {
        Self {
            missing: vec![
                MissingComponent::AuthoritativeFiniteFragmentKernelBackend,
                MissingComponent::AuthoritativeDemandSchemeCalculus,
                MissingComponent::SchemeGenerationCompletenessTheorem,
                MissingComponent::AuthoritativeDemandCensus,
                MissingComponent::CompleteAdoptedQuotient,
                MissingComponent::FiniteFragmentTheoremVerifier,
                MissingComponent::ResponseCone,
                MissingComponent::SemanticAudit,
                MissingComponent::FreeSealingUniversalProperty,
            ],
            selected_bootstrap_contract: SelectedBootstrapContract::LawV2ARegisteredContract,
            verified_registered_bootstrap_digest,
        }
    }

    pub fn missing(&self) -> &[MissingComponent] {
        &self.missing
    }

    pub fn selected_bootstrap_contract(&self) -> SelectedBootstrapContract {
        self.selected_bootstrap_contract
    }

    pub fn verified_registered_bootstrap_digest(&self) -> Option<&Digest> {
        self.verified_registered_bootstrap_digest.as_ref()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum StartupOutcome {
    Unknown {
        reason: String,
        manifest: ReadinessManifest,
    },
}

pub fn certificate_types_milestone_outcome() -> StartupOutcome {
    unknown("the full Law V2 proof stack is incomplete")
}

#[derive(Debug, Error)]
pub enum StartupVerificationError {
    #[error(transparent)]
    Kernel(#[from] KernelError),
    #[error(transparent)]
    RegisteredBootstrap(#[from] RegisteredBootstrapError),
}

/// Replay the disclosed Law-V2A initial condition before reporting the
/// remaining fail-closed readiness state.
///
/// This verifies registration integrity and typing only. It does not derive
/// the bootstrap or make the later proof layers available.
pub fn registered_bootstrap_milestone_outcome() -> Result<StartupOutcome, StartupVerificationError>
{
    let kernel = Kernel::new(KernelLimits::default())?;
    let bootstrap = load_embedded_registered_bootstrap(&kernel)?;
    Ok(unknown_with_registered_bootstrap(
        "the registered Law-V2A bootstrap replayed; no adopted complete demand-scheme calculus yet determines the first guarded census",
        bootstrap.artifact_digest().clone(),
    ))
}

/// Compatibility name retained for callers introduced with the first
/// oracle-firewall slice.
pub fn firewall_milestone_outcome() -> StartupOutcome {
    certificate_types_milestone_outcome()
}

/// Inspect an unchecked adopted-quotient claim without granting it authority.
///
/// Even `ClaimedComplete` remains `Unknown`: this engine has neither the
/// ambient equivalence verifier nor a complete-quotient capability, and its
/// outcome type contains no exact-cardinality variant.
pub fn quotient_milestone_outcome(status: &UncheckedQuotientStatus) -> StartupOutcome {
    match status {
        UncheckedQuotientStatus::Incomplete { .. } => {
            unknown("the adopted quotient has unresolved equivalence goals")
        }
        UncheckedQuotientStatus::ClaimedComplete { .. } => {
            unknown("an unchecked complete quotient claim has no verifier")
        }
    }
}

/// Inspect an unchecked finite-fragment disposition without promoting it to
/// a theorem.
pub fn finite_fragment_milestone_outcome(
    outcome: &UncheckedFiniteFragmentOutcome,
) -> StartupOutcome {
    match outcome {
        UncheckedFiniteFragmentOutcome::Proven { .. }
        | UncheckedFiniteFragmentOutcome::Refuted { .. } => {
            unknown("an unchecked theorem-level finite-fragment claim has no verifier")
        }
        UncheckedFiniteFragmentOutcome::OutsideFragment { .. } => {
            unknown("the requested theorem lies outside the declared finite fragment")
        }
        UncheckedFiniteFragmentOutcome::ResourceExhausted { .. } => {
            unknown("finite-fragment verification exhausted its declared resources")
        }
    }
}

/// Inspect an unchecked bootstrap claim. Neither a registered-contract record
/// nor a claimed derived-bootstrap theorem is promoted to verified authority.
pub fn bootstrap_claim_milestone_outcome(status: &UncheckedBootstrapStatus) -> StartupOutcome {
    match status {
        UncheckedBootstrapStatus::LawV2ARegistered { .. } => {
            unknown("the Law-V2A bootstrap is a disclosed registered contract")
        }
        UncheckedBootstrapStatus::ClaimedLawV2BDerived { .. } => {
            unknown("an unchecked Law-V2B bootstrap derivation has no verifier")
        }
    }
}

fn unknown(reason: &str) -> StartupOutcome {
    StartupOutcome::Unknown {
        reason: reason.to_owned(),
        manifest: ReadinessManifest::certificate_types_milestone(),
    }
}

fn unknown_with_registered_bootstrap(reason: &str, bootstrap_digest: Digest) -> StartupOutcome {
    StartupOutcome::Unknown {
        reason: reason.to_owned(),
        manifest: ReadinessManifest::with_registered_bootstrap(Some(bootstrap_digest)),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConfigError, LawfulRuntimeConfig, ResourceLimits, SelectedBootstrapContract,
        StartupOutcome, bootstrap_claim_milestone_outcome, certificate_types_milestone_outcome,
        finite_fragment_milestone_outcome, quotient_milestone_outcome,
        registered_bootstrap_milestone_outcome,
    };
    use pen_law::{
        CONTRACT_STATUS_SCHEMA_VERSION, DEMAND_WINDOW_WIDTH, Digest, FiniteFragmentResource,
        UncheckedBootstrapStatus, UncheckedEquivalenceClass, UncheckedEquivalenceGoal,
        UncheckedFiniteFragmentOutcome, UncheckedQuotientStatus,
    };

    #[test]
    fn firewall_milestone_has_no_success_path() {
        let outcome = certificate_types_milestone_outcome();
        assert!(matches!(outcome, StartupOutcome::Unknown { .. }));
        let wire = serde_json::to_string(&outcome).expect("serialize outcome");
        assert!(wire.contains("authoritative_finite_fragment_kernel_backend"));
        assert!(!wire.contains("ambient_cubical_kernel"));
        assert!(!wire.contains("bootstrap_uniqueness_theorem"));
    }

    #[test]
    fn startup_replays_and_binds_the_registered_initial_condition() {
        let outcome =
            registered_bootstrap_milestone_outcome().expect("embedded registration must replay");
        let StartupOutcome::Unknown { reason, manifest } = outcome;
        assert!(reason.contains("registered Law-V2A bootstrap replayed"));
        assert!(reason.contains("demand-scheme calculus"));
        assert!(manifest.verified_registered_bootstrap_digest().is_some());
        let wire = serde_json::to_string(&manifest).expect("serialize manifest");
        assert!(wire.contains("verified_registered_bootstrap_digest"));
        assert!(wire.contains("authoritative_demand_scheme_calculus"));
        assert!(wire.contains("scheme_generation_completeness_theorem"));
        assert!(!wire.contains("law_v2_b"));
    }

    fn digest(label: &[u8]) -> Digest {
        Digest::of_bytes(label)
    }

    fn config() -> LawfulRuntimeConfig {
        LawfulRuntimeConfig {
            window_width: DEMAND_WINDOW_WIDTH,
            kernel_digest: Digest::of_bytes(b"kernel"),
            grammar_digest: Digest::of_bytes(b"grammar"),
            scheme_calculus_digest: Digest::of_bytes(b"schemes"),
            law_digest: Digest::of_bytes(b"law"),
            resources: ResourceLimits {
                max_resident_bytes: 1,
                max_frontier_items: 1,
                worker_count: 1,
            },
        }
    }

    #[test]
    fn lawful_config_contains_only_fixed_memory_and_resource_inputs() {
        config().validate().expect("valid contract");
        let mut wrong_width = config();
        wrong_width.window_width = DEMAND_WINDOW_WIDTH + 1;
        assert_eq!(
            wrong_width.validate(),
            Err(ConfigError::WrongWindowWidth {
                expected: DEMAND_WINDOW_WIDTH,
                actual: DEMAND_WINDOW_WIDTH + 1,
            })
        );
    }

    #[test]
    fn unknown_configuration_fields_fail_closed() {
        let mut value = serde_json::to_value(config()).expect("serialize config");
        value
            .as_object_mut()
            .expect("config is an object")
            .insert("forbidden_endpoint".to_owned(), serde_json::json!(99));
        assert!(serde_json::from_value::<LawfulRuntimeConfig>(value).is_err());
    }

    #[test]
    fn representative_count_cannot_emit_classes_or_a_cardinality() {
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
            let outcome = quotient_milestone_outcome(&status);
            assert!(matches!(outcome, StartupOutcome::Unknown { .. }));
            let wire = serde_json::to_string(&outcome).expect("serialize outcome");
            assert!(!wire.contains("cardinality"));
            assert!(!wire.contains("class_count"));
        }

        let unchecked_complete = UncheckedQuotientStatus::ClaimedComplete {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            classes: vec![UncheckedEquivalenceClass {
                class_digest: digest(b"unchecked-class"),
                member_representative_digests: vec![
                    digest(b"presentation-a"),
                    digest(b"presentation-b"),
                ],
            }],
            completeness_artifact_digest: digest(b"unchecked-completeness"),
        };
        assert!(matches!(
            quotient_milestone_outcome(&unchecked_complete),
            StartupOutcome::Unknown { .. }
        ));
    }

    #[test]
    fn outside_fragment_and_exhaustion_map_only_to_unknown() {
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
        assert!(matches!(
            finite_fragment_milestone_outcome(&outside),
            StartupOutcome::Unknown { .. }
        ));
        assert!(matches!(
            finite_fragment_milestone_outcome(&exhausted),
            StartupOutcome::Unknown { .. }
        ));
    }

    #[test]
    fn every_unchecked_theorem_and_bootstrap_claim_remains_unknown() {
        let claimed_proof = UncheckedFiniteFragmentOutcome::Proven {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            theorem_digest: digest(b"theorem"),
            fragment_digest: digest(b"fragment"),
            proof_artifact_digest: digest(b"unchecked-proof"),
        };
        let claimed_refutation = UncheckedFiniteFragmentOutcome::Refuted {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            theorem_digest: digest(b"theorem"),
            fragment_digest: digest(b"fragment"),
            refutation_artifact_digest: digest(b"unchecked-refutation"),
        };
        assert!(matches!(
            finite_fragment_milestone_outcome(&claimed_proof),
            StartupOutcome::Unknown { .. }
        ));
        assert!(matches!(
            finite_fragment_milestone_outcome(&claimed_refutation),
            StartupOutcome::Unknown { .. }
        ));

        let registered = UncheckedBootstrapStatus::LawV2ARegistered {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            bootstrap_contract_digest: digest(b"registered-contract"),
        };
        let derived = UncheckedBootstrapStatus::ClaimedLawV2BDerived {
            schema_version: CONTRACT_STATUS_SCHEMA_VERSION,
            uniqueness_theorem_digest: digest(b"unchecked-uniqueness"),
            proof_artifact_digest: digest(b"unchecked-proof"),
        };
        assert!(matches!(
            bootstrap_claim_milestone_outcome(&registered),
            StartupOutcome::Unknown { .. }
        ));
        assert!(matches!(
            bootstrap_claim_milestone_outcome(&derived),
            StartupOutcome::Unknown { .. }
        ));

        let StartupOutcome::Unknown { manifest, .. } = certificate_types_milestone_outcome();
        assert_eq!(
            manifest.selected_bootstrap_contract(),
            SelectedBootstrapContract::LawV2ARegisteredContract
        );
        let wire = serde_json::to_string(&manifest).expect("serialize manifest");
        assert!(wire.contains("law_v2_a_registered_contract"));
        assert!(!wire.contains("law_v2_b"));
    }

    #[test]
    fn mismatched_status_versions_remain_untrusted_and_unknown() {
        let mismatched = CONTRACT_STATUS_SCHEMA_VERSION + 1;
        let quotient = UncheckedQuotientStatus::Incomplete {
            schema_version: mismatched,
            representative_digests: Vec::new(),
            unresolved_equivalence_goals: Vec::new(),
            claimed_proven_obstructions: Vec::new(),
        };
        let quotient_wire = serde_json::to_string(&quotient).expect("serialize quotient");
        let quotient = serde_json::from_str::<UncheckedQuotientStatus>(&quotient_wire)
            .expect("deserialize unchecked quotient");
        assert!(matches!(
            quotient_milestone_outcome(&quotient),
            StartupOutcome::Unknown { .. }
        ));

        let fragment = UncheckedFiniteFragmentOutcome::Proven {
            schema_version: mismatched,
            theorem_digest: digest(b"theorem"),
            fragment_digest: digest(b"fragment"),
            proof_artifact_digest: digest(b"unchecked-proof"),
        };
        assert!(matches!(
            finite_fragment_milestone_outcome(&fragment),
            StartupOutcome::Unknown { .. }
        ));

        let bootstrap = UncheckedBootstrapStatus::ClaimedLawV2BDerived {
            schema_version: mismatched,
            uniqueness_theorem_digest: digest(b"unchecked-uniqueness"),
            proof_artifact_digest: digest(b"unchecked-proof"),
        };
        assert!(matches!(
            bootstrap_claim_milestone_outcome(&bootstrap),
            StartupOutcome::Unknown { .. }
        ));
    }
}
