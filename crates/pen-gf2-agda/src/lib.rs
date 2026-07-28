//! Pinned Cubical Agda backend readiness for the Genesis Fragment.
//!
//! This crate has one deliberately narrow authority boundary: it checks a
//! fixed, internally generated smoke module against one exact Agda/Cubical
//! toolchain. A successful probe returns an unforgeable safe-Rust capability
//! whose fields are private and which is not deserializable.
//!
//! The capability is **not** a theorem oracle. In particular, this crate has
//! no API that accepts caller-provided Agda source and no API that turns an
//! evidence manifest into a verified capability. The smoke check establishes
//! backend readiness only; a future restricted GF2-to-Agda translator and
//! theorem replay boundary must be reviewed separately.

#![forbid(unsafe_code)]

mod digest;
mod evidence;
mod probe;

pub use digest::{EvidenceDigest, EvidenceDigestError};
pub use evidence::{
    AGDA_EXPECTED_VERSION, AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST, AgdaDataDirectoryEvidence,
    AgdaPrimitiveSourceFileEvidence, AgdaPrimitiveSourceTreeEvidence, AgdaToolchainEvidence,
    BACKEND_CONTRACT_ID, BackendEvidenceError, BackendEvidenceManifest, CUBICAL_EXPECTED_COMMIT,
    CUBICAL_EXPECTED_FLAGS, CUBICAL_EXPECTED_INCLUDE, CUBICAL_EXPECTED_LIBRARY_NAME,
    CUBICAL_EXPECTED_SOURCE_TREE_DIGEST, CapturedOutput, CheckerInvocationEvidence,
    CubicalCheckoutEvidence, CubicalSourceFileEvidence, CubicalSourceTreeEvidence,
    EVIDENCE_SCHEMA_VERSION, FixedSmokeSourceEvidence, MAX_CHECKER_TIMEOUT_MILLIS, ProcessEvidence,
    SMOKE_FILE_NAME, SMOKE_MODULE_NAME, SMOKE_SOURCE_DIGEST,
};
pub use probe::{
    ExecutableDigestPins, MAX_CHECKER_TIMEOUT, PinnedBackendConfig, ProbeError,
    SourceTreeDigestPins, VerifiedPinnedBackend, probe_pinned_backend,
};
