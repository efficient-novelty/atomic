use crate::manifest::{AuditDecision, AuditUnknownReason};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::ffi::OsString;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const AGDA_EXPECTED_VERSION: &str = "2.8.0";
const AGDA_EXECUTABLE_DIGEST_PIN: &str =
    "blake3:31f99b2381750bd0387b4695c676f64ca87f370e9ec2f3e98dac76adfab2d32a";
const AGDA_PRIMITIVE_TREE_DIGEST_PIN: &str =
    "blake3:2cae2bc83c34013b7cf4221bc99509bfe74a15404be43dd04367a7733d85f52d";
const AGDA_PRIMITIVE_TREE_CONTRACT: &str = "pen-gf2-agda/agda-primitive-source-tree/v1";
const AGDA_SOURCE_RELATIVE_PATH: &str = "LawV2/SemanticAuditCoreV1.agda";
const AGDA_SOURCE_BYTES: &[u8] = include_bytes!("../agda/LawV2/SemanticAuditCoreV1.agda");
const AGDA_DATA_SNAPSHOT_DIRECTORY: &str = "agda-data";
const AGDA_PRIMITIVE_RELATIVE_ROOT: &str = "lib/prim";
const MAX_TREE_ENTRIES: u64 = 100_000;
const MAX_SOURCE_FILES: u64 = 20_000;
const MAX_SOURCE_FILE_BYTES: u64 = 64 * 1024 * 1024;
const MAX_SOURCE_TREE_BYTES: u64 = 1024 * 1024 * 1024;
const CHECKER_TIMEOUT: Duration = Duration::from_secs(30);
/// The generated production-acceptance package forces the complete wire
/// decode, structural, semantic, typing, and inventory verdicts plus the
/// transcript agreement at type-check time over the full module tree
/// with interfaces disabled, so it needs a checker budget far above the
/// 30-second single-module gate. The timeout is a run discipline, not
/// part of the digested argument protocol.
const GENERATED_PACKAGE_CHECKER_TIMEOUT: Duration = Duration::from_secs(1800);
static SCRATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Stage-specific explanation for a fail-closed pinned Agda reference check.
///
/// This diagnostic is intentionally not part of the audit decision algebra:
/// every variant still maps to `Unknown(UnsupportedVerifier)`. It exists so a
/// local operator can distinguish unavailable tooling from a pin mismatch,
/// malformed output, mutation, timeout, or cleanup failure without weakening
/// any acceptance condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgdaReferenceFailureV1 {
    ExecutableNotFound,
    ExecutableUnreadable,
    ExecutableDigestMismatch {
        observed: String,
    },
    ScratchDirectoryUnavailable,
    ScratchSetupFailed,
    VersionProbeSpawnFailed,
    VersionProbeTimedOut,
    VersionProbeFailed {
        exit_code: Option<i32>,
    },
    VersionProbeNonCanonicalOutput,
    VersionProbeStderr {
        digest: String,
    },
    VersionMismatch {
        observed_first_line: Option<String>,
    },
    DataDirectoryProbeSpawnFailed,
    DataDirectoryProbeTimedOut,
    DataDirectoryProbeFailed {
        exit_code: Option<i32>,
    },
    DataDirectoryProbeNonCanonicalOutput,
    DataDirectoryProbeStderr {
        digest: String,
    },
    DataDirectoryInvalid,
    PrimitiveTreeUnreadable,
    PrimitiveTreeDigestMismatch {
        observed: String,
    },
    PrimitiveSnapshotFailed,
    PrimitiveSnapshotMismatch,
    SourceWriteFailed,
    CheckerSpawnFailed,
    CheckerTimedOut,
    CheckerFailed {
        exit_code: Option<i32>,
        stdout_digest: String,
        stderr_digest: String,
    },
    CheckerNonCanonicalOutput,
    CheckerStdoutMismatch {
        observed_digest: String,
    },
    CheckerStderr {
        digest: String,
    },
    SourceMutated,
    ExecutableMutated,
    InstalledPrimitiveTreeMutated,
    SnapshotPrimitiveTreeMutated,
    ScratchCleanupFailed,
}

impl std::fmt::Display for AgdaReferenceFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AgdaReferenceFailureV1::{
            CheckerFailed, CheckerStderr, CheckerStdoutMismatch, DataDirectoryProbeFailed,
            DataDirectoryProbeStderr, ExecutableDigestMismatch, PrimitiveTreeDigestMismatch,
            VersionMismatch, VersionProbeFailed, VersionProbeStderr,
        };
        match self {
            ExecutableDigestMismatch { observed } => write!(
                formatter,
                "Agda executable digest does not match the fixed pin (observed {observed})"
            ),
            VersionProbeFailed { exit_code } => {
                write!(
                    formatter,
                    "Agda version probe failed with exit code {exit_code:?}"
                )
            }
            VersionProbeStderr { digest } => write!(
                formatter,
                "Agda version probe wrote unexpected stderr (digest {digest})"
            ),
            VersionMismatch {
                observed_first_line,
            } => write!(
                formatter,
                "Agda version does not match the fixed {AGDA_EXPECTED_VERSION} pin \
                 (first line {observed_first_line:?})"
            ),
            DataDirectoryProbeFailed { exit_code } => write!(
                formatter,
                "Agda data-directory probe failed with exit code {exit_code:?}"
            ),
            DataDirectoryProbeStderr { digest } => write!(
                formatter,
                "Agda data-directory probe wrote unexpected stderr (digest {digest})"
            ),
            PrimitiveTreeDigestMismatch { observed } => write!(
                formatter,
                "Agda primitive source tree does not match the fixed pin (observed {observed})"
            ),
            CheckerFailed {
                exit_code,
                stdout_digest,
                stderr_digest,
            } => write!(
                formatter,
                "Agda checker failed with exit code {exit_code:?} \
                 (stdout {stdout_digest}, stderr {stderr_digest})"
            ),
            CheckerStdoutMismatch { observed_digest } => write!(
                formatter,
                "Agda checker stdout violates the exact one-line protocol \
                 (observed digest {observed_digest})"
            ),
            CheckerStderr { digest } => write!(
                formatter,
                "Agda checker wrote unexpected stderr (digest {digest})"
            ),
            other => formatter.write_str(match other {
                Self::ExecutableNotFound => "pinned Agda executable was not found on PATH",
                Self::ExecutableUnreadable => "Agda executable could not be read",
                Self::ScratchDirectoryUnavailable => {
                    "private Agda scratch directory could not be created"
                }
                Self::ScratchSetupFailed => "private Agda scratch layout could not be created",
                Self::VersionProbeSpawnFailed => "Agda version probe could not be started",
                Self::VersionProbeTimedOut => "Agda version probe timed out",
                Self::VersionProbeNonCanonicalOutput => {
                    "Agda version probe output was not canonical UTF-8"
                }
                Self::DataDirectoryProbeSpawnFailed => {
                    "Agda data-directory probe could not be started"
                }
                Self::DataDirectoryProbeTimedOut => "Agda data-directory probe timed out",
                Self::DataDirectoryProbeNonCanonicalOutput => {
                    "Agda data-directory probe output was not canonical UTF-8"
                }
                Self::DataDirectoryInvalid => {
                    "Agda data-directory probe did not report one absolute directory"
                }
                Self::PrimitiveTreeUnreadable => {
                    "installed Agda primitive source tree could not be inventoried"
                }
                Self::PrimitiveSnapshotFailed => {
                    "Agda primitive source tree could not be snapshotted privately"
                }
                Self::PrimitiveSnapshotMismatch => {
                    "private Agda primitive snapshot differs from the pinned installed tree"
                }
                Self::SourceWriteFailed => "fixed Agda source could not be written privately",
                Self::CheckerSpawnFailed => "Agda checker could not be started",
                Self::CheckerTimedOut => "Agda checker timed out",
                Self::CheckerNonCanonicalOutput => "Agda checker output was not canonical UTF-8",
                Self::SourceMutated => "Agda checker mutated the fixed source",
                Self::ExecutableMutated => "Agda executable changed during verification",
                Self::InstalledPrimitiveTreeMutated => {
                    "installed Agda primitive source tree changed during verification"
                }
                Self::SnapshotPrimitiveTreeMutated => {
                    "private Agda primitive snapshot changed during verification"
                }
                Self::ScratchCleanupFailed => "private Agda scratch cleanup failed",
                ExecutableDigestMismatch { .. }
                | VersionProbeFailed { .. }
                | VersionProbeStderr { .. }
                | VersionMismatch { .. }
                | DataDirectoryProbeFailed { .. }
                | DataDirectoryProbeStderr { .. }
                | PrimitiveTreeDigestMismatch { .. }
                | CheckerFailed { .. }
                | CheckerStdoutMismatch { .. }
                | CheckerStderr { .. } => unreachable!("handled above"),
            }),
        }
    }
}

/// Opaque evidence that the pinned checker accepted the fixed safe Agda
/// abstract reference model.
///
/// This does not assert Rust/Agda agreement or discharge the model's
/// explicitly excluded full-Q0 and free-completion obligations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedAgdaReferenceV1 {
    source_digest: Digest,
    executable_digest: Digest,
    primitive_tree_digest: Digest,
    primitive_source_file_count: u64,
    primitive_source_byte_length: u64,
    checker_argument_protocol_digest: Digest,
    checker_stdout_digest: Digest,
    checker_stderr_digest: Digest,
    digest: Digest,
}

/// One compile-time-fixed source in an additive safe-Agda package.
///
/// This is crate-private so untrusted callers cannot submit theorem text to
/// the checker and turn successful type checking of a different proposition
/// into authority for a built-in capability.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FixedAgdaSourceV1 {
    pub(crate) relative_path: &'static str,
    pub(crate) module_name: &'static str,
    pub(crate) bytes: &'static [u8],
}

/// One runtime-generated safe Agda source: the generated production
/// input modules (canonical bundle bytes, expected transcript bytes)
/// and the fixed acceptance template that consumes them. Everything
/// else about the pinned package discipline is identical to the
/// compile-time-fixed sources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GeneratedAgdaSourceV1 {
    pub(crate) relative_path: String,
    pub(crate) module_name: String,
    pub(crate) bytes: Vec<u8>,
}

/// One package source in exact checker order: compile-time fixed or
/// runtime generated.
#[derive(Clone, Debug)]
pub(crate) enum PackageSourceV1 {
    Fixed(FixedAgdaSourceV1),
    Generated(GeneratedAgdaSourceV1),
}

#[derive(Clone, Copy)]
struct AgdaSourceViewV1<'source> {
    relative_path: &'source str,
    module_name: &'source str,
    bytes: &'source [u8],
}

impl FixedAgdaSourceV1 {
    fn view(&self) -> AgdaSourceViewV1<'_> {
        AgdaSourceViewV1 {
            relative_path: self.relative_path,
            module_name: self.module_name,
            bytes: self.bytes,
        }
    }
}

impl PackageSourceV1 {
    fn view(&self) -> AgdaSourceViewV1<'_> {
        match self {
            Self::Fixed(source) => source.view(),
            Self::Generated(source) => AgdaSourceViewV1 {
                relative_path: &source.relative_path,
                module_name: &source.module_name,
                bytes: &source.bytes,
            },
        }
    }
}

/// Pinned checker result for one compile-time-fixed multi-source package.
///
/// The semantic meaning of the package is deliberately supplied by the
/// consuming private verifier. This handle establishes only exact source,
/// checker, primitive-tree, argument, and transcript identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VerifiedFixedAgdaPackageV1 {
    source_tree_digest: Digest,
    executable_digest: Digest,
    primitive_tree_digest: Digest,
    checker_argument_protocol_digest: Digest,
    checker_stdout_digest: Digest,
    checker_stderr_digest: Digest,
    digest: Digest,
}

impl VerifiedFixedAgdaPackageV1 {
    pub(crate) fn source_tree_digest(&self) -> &Digest {
        &self.source_tree_digest
    }

    pub(crate) fn checker_stdout_digest(&self) -> &Digest {
        &self.checker_stdout_digest
    }

    pub(crate) fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedFixedAgdaPackageV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source_tree_digest.encode_canonical(encoder);
        self.executable_digest.encode_canonical(encoder);
        self.primitive_tree_digest.encode_canonical(encoder);
        self.checker_argument_protocol_digest
            .encode_canonical(encoder);
        self.checker_stdout_digest.encode_canonical(encoder);
        self.checker_stderr_digest.encode_canonical(encoder);
    }
}

impl VerifiedAgdaReferenceV1 {
    pub fn source_digest(&self) -> &Digest {
        &self.source_digest
    }

    pub fn executable_digest(&self) -> &Digest {
        &self.executable_digest
    }

    pub fn primitive_tree_digest(&self) -> &Digest {
        &self.primitive_tree_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedAgdaReferenceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source_digest.encode_canonical(encoder);
        self.executable_digest.encode_canonical(encoder);
        self.primitive_tree_digest.encode_canonical(encoder);
        encoder.u64(self.primitive_source_file_count);
        encoder.u64(self.primitive_source_byte_length);
        self.checker_argument_protocol_digest
            .encode_canonical(encoder);
        self.checker_stdout_digest.encode_canonical(encoder);
        self.checker_stderr_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrimitiveFile {
    relative_path: String,
    byte_length: u64,
    digest: Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrimitiveTree {
    digest: Digest,
    source_file_count: u64,
    source_byte_length: u64,
    files: Vec<PrimitiveFile>,
}

pub fn verify_pinned_agda_reference_v1() -> AuditDecision<VerifiedAgdaReferenceV1> {
    match diagnose_pinned_agda_reference_v1() {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(_) => AuditDecision::Unknown(AuditUnknownReason::UnsupportedVerifier),
    }
}

/// Runs the exact same fail-closed gate as [`verify_pinned_agda_reference_v1`]
/// while retaining a stage-specific local diagnostic.
///
/// Callers must not use the error variant as semantic evidence. Only the
/// opaque success capability is evidence; every failure remains
/// `Unknown(UnsupportedVerifier)` in the audit decision algebra.
pub fn diagnose_pinned_agda_reference_v1() -> Result<VerifiedAgdaReferenceV1, AgdaReferenceFailureV1>
{
    let executable = resolve_agda_executable().ok_or(AgdaReferenceFailureV1::ExecutableNotFound)?;
    let executable_before =
        fs::read(&executable).map_err(|_| AgdaReferenceFailureV1::ExecutableUnreadable)?;
    let executable_digest = Digest::of_bytes(&executable_before);
    if executable_digest.as_str() != AGDA_EXECUTABLE_DIGEST_PIN {
        return Err(AgdaReferenceFailureV1::ExecutableDigestMismatch {
            observed: executable_digest.as_str().to_owned(),
        });
    }

    let scratch =
        create_scratch_directory().ok_or(AgdaReferenceFailureV1::ScratchDirectoryUnavailable)?;
    let result = verify_in_scratch(&executable, &executable_before, executable_digest, &scratch);
    if fs::remove_dir_all(&scratch).is_err() {
        return Err(AgdaReferenceFailureV1::ScratchCleanupFailed);
    }
    result
}

/// Check an additive, compile-time-fixed safe-Agda package under the same
/// executable and primitive-source pins as the V1 reference gate.
///
/// The source list must be in the checker's exact expected reporting order,
/// beginning with `entry_relative_path`. Imported files are checked with
/// interfaces disabled, so every source must appear exactly once.
pub(crate) fn diagnose_pinned_fixed_agda_package_v1(
    sources: &[FixedAgdaSourceV1],
    entry_relative_path: &str,
) -> Result<VerifiedFixedAgdaPackageV1, AgdaReferenceFailureV1> {
    let views = sources.iter().map(FixedAgdaSourceV1::view).collect::<Vec<_>>();
    diagnose_agda_package_views_v1(&views, entry_relative_path, CHECKER_TIMEOUT)
}

/// The pinned package gate over an exact checker-ordered mixture of
/// compile-time-fixed and runtime-generated sources: the safe-Agda
/// acceptance bridge for generated production input modules. The
/// discipline is identical to the fixed-package gate; only the source
/// of the bytes differs, and the source-tree digest covers the exact
/// generated content.
pub(crate) fn diagnose_generated_agda_package_v1(
    sources: &[PackageSourceV1],
    entry_relative_path: &str,
) -> Result<VerifiedFixedAgdaPackageV1, AgdaReferenceFailureV1> {
    let views = sources.iter().map(PackageSourceV1::view).collect::<Vec<_>>();
    diagnose_agda_package_views_v1(
        &views,
        entry_relative_path,
        GENERATED_PACKAGE_CHECKER_TIMEOUT,
    )
}

fn diagnose_agda_package_views_v1(
    sources: &[AgdaSourceViewV1<'_>],
    entry_relative_path: &str,
    checker_timeout: Duration,
) -> Result<VerifiedFixedAgdaPackageV1, AgdaReferenceFailureV1> {
    let reference = diagnose_pinned_agda_reference_v1()?;
    if sources.is_empty()
        || sources[0].relative_path != entry_relative_path
        || !valid_fixed_package_sources(sources)
    {
        return Err(AgdaReferenceFailureV1::SourceWriteFailed);
    }

    let executable = resolve_agda_executable().ok_or(AgdaReferenceFailureV1::ExecutableNotFound)?;
    let executable_before =
        fs::read(&executable).map_err(|_| AgdaReferenceFailureV1::ExecutableUnreadable)?;
    let executable_digest = Digest::of_bytes(&executable_before);
    if &executable_digest != reference.executable_digest() {
        return Err(AgdaReferenceFailureV1::ExecutableDigestMismatch {
            observed: executable_digest.as_str().to_owned(),
        });
    }

    let scratch =
        create_scratch_directory().ok_or(AgdaReferenceFailureV1::ScratchDirectoryUnavailable)?;
    let result = verify_fixed_package_in_scratch(
        &executable,
        &executable_before,
        executable_digest,
        sources,
        entry_relative_path,
        &scratch,
        checker_timeout,
    );
    if fs::remove_dir_all(&scratch).is_err() {
        return Err(AgdaReferenceFailureV1::ScratchCleanupFailed);
    }
    result
}

fn valid_fixed_package_sources(sources: &[AgdaSourceViewV1<'_>]) -> bool {
    let mut paths = std::collections::BTreeSet::new();
    let mut modules = std::collections::BTreeSet::new();
    sources.iter().all(|source| {
        let path = Path::new(source.relative_path);
        !source.relative_path.contains('\\')
            && !source.relative_path.contains('\0')
            && !source.module_name.is_empty()
            && !source
                .module_name
                .chars()
                .any(|character| matches!(character, '\r' | '\n' | '(' | ')'))
            && path.is_relative()
            && path.extension().and_then(|value| value.to_str()) == Some("agda")
            && path
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_)))
            && !source.bytes.is_empty()
            && paths.insert(source.relative_path)
            && modules.insert(source.module_name)
    })
}

fn verify_fixed_package_in_scratch(
    executable: &Path,
    executable_before: &[u8],
    executable_digest: Digest,
    sources: &[AgdaSourceViewV1<'_>],
    entry_relative_path: &str,
    scratch: &Path,
    checker_timeout: Duration,
) -> Result<VerifiedFixedAgdaPackageV1, AgdaReferenceFailureV1> {
    fs::create_dir(scratch.join("agda-app"))
        .map_err(|_| AgdaReferenceFailureV1::ScratchSetupFailed)?;

    let data_output = run_command(
        sanitized_command(executable, Some(scratch), None).arg("--print-agda-data-dir"),
        CHECKER_TIMEOUT,
    )
    .map_err(|failure| match failure {
        RunCommandFailure::Spawn => AgdaReferenceFailureV1::DataDirectoryProbeSpawnFailed,
        RunCommandFailure::Timeout => AgdaReferenceFailureV1::DataDirectoryProbeTimedOut,
        RunCommandFailure::Wait => {
            AgdaReferenceFailureV1::DataDirectoryProbeFailed { exit_code: None }
        }
    })?;
    let data_stdout = canonical_output(&data_output.stdout)
        .ok_or(AgdaReferenceFailureV1::DataDirectoryProbeNonCanonicalOutput)?;
    let data_stderr = canonical_output(&data_output.stderr)
        .ok_or(AgdaReferenceFailureV1::DataDirectoryProbeNonCanonicalOutput)?;
    if !data_output.status.success() {
        return Err(AgdaReferenceFailureV1::DataDirectoryProbeFailed {
            exit_code: data_output.status.code(),
        });
    }
    if !data_stderr.is_empty() {
        return Err(AgdaReferenceFailureV1::DataDirectoryProbeStderr {
            digest: Digest::of_bytes(data_stderr.as_bytes()).as_str().to_owned(),
        });
    }
    let data_directory =
        one_absolute_directory(&data_stdout).ok_or(AgdaReferenceFailureV1::DataDirectoryInvalid)?;
    let primitive_root = fs::canonicalize(data_directory.join(AGDA_PRIMITIVE_RELATIVE_ROOT))
        .ok()
        .filter(|path| path.is_dir())
        .ok_or(AgdaReferenceFailureV1::PrimitiveTreeUnreadable)?;
    let tree_before = collect_primitive_tree(&primitive_root)
        .ok_or(AgdaReferenceFailureV1::PrimitiveTreeUnreadable)?;
    if tree_before.digest.as_str() != AGDA_PRIMITIVE_TREE_DIGEST_PIN {
        return Err(AgdaReferenceFailureV1::PrimitiveTreeDigestMismatch {
            observed: tree_before.digest.as_str().to_owned(),
        });
    }

    let snapshot_data_directory = scratch.join(AGDA_DATA_SNAPSHOT_DIRECTORY);
    fs::create_dir(&snapshot_data_directory)
        .map_err(|_| AgdaReferenceFailureV1::PrimitiveSnapshotFailed)?;
    let snapshot_primitive_root = snapshot_data_directory.join(AGDA_PRIMITIVE_RELATIVE_ROOT);
    let materialized = materialize_primitive_tree(&primitive_root, &snapshot_primitive_root)
        .ok_or(AgdaReferenceFailureV1::PrimitiveSnapshotFailed)?;
    let snapshot_before = collect_primitive_tree(&snapshot_primitive_root)
        .ok_or(AgdaReferenceFailureV1::PrimitiveSnapshotFailed)?;
    if materialized != tree_before || snapshot_before != tree_before {
        return Err(AgdaReferenceFailureV1::PrimitiveSnapshotMismatch);
    }

    let mut source_paths = Vec::with_capacity(sources.len());
    for source in sources {
        let path = scratch.join(source.relative_path);
        let parent = path
            .parent()
            .ok_or(AgdaReferenceFailureV1::SourceWriteFailed)?;
        fs::create_dir_all(parent).map_err(|_| AgdaReferenceFailureV1::SourceWriteFailed)?;
        write_new_file(&path, source.bytes).ok_or(AgdaReferenceFailureV1::SourceWriteFailed)?;
        source_paths.push(path);
    }
    let entry_path = scratch.join(entry_relative_path);

    let mut checker = sanitized_command(
        executable,
        Some(scratch),
        Some(snapshot_data_directory.as_path()),
    );
    checker
        .args([
            "--no-libraries",
            "--ignore-interfaces",
            "--safe",
            "--without-K",
            "-i",
        ])
        .arg(scratch)
        .arg(&entry_path);
    let output = run_command(&mut checker, checker_timeout).map_err(|failure| match failure {
        RunCommandFailure::Spawn => AgdaReferenceFailureV1::CheckerSpawnFailed,
        RunCommandFailure::Timeout => AgdaReferenceFailureV1::CheckerTimedOut,
        RunCommandFailure::Wait => AgdaReferenceFailureV1::CheckerFailed {
            exit_code: None,
            stdout_digest: Digest::of_bytes(b"").as_str().to_owned(),
            stderr_digest: Digest::of_bytes(b"").as_str().to_owned(),
        },
    })?;
    let stdout = canonical_output(&output.stdout)
        .ok_or(AgdaReferenceFailureV1::CheckerNonCanonicalOutput)?;
    let stderr = canonical_output(&output.stderr)
        .ok_or(AgdaReferenceFailureV1::CheckerNonCanonicalOutput)?;
    if !output.status.success() {
        return Err(AgdaReferenceFailureV1::CheckerFailed {
            exit_code: output.status.code(),
            stdout_digest: Digest::of_bytes(stdout.as_bytes()).as_str().to_owned(),
            stderr_digest: Digest::of_bytes(stderr.as_bytes()).as_str().to_owned(),
        });
    }
    let canonical_transcript = canonical_fixed_package_transcript(&stdout, sources, &source_paths)
        .ok_or_else(|| AgdaReferenceFailureV1::CheckerStdoutMismatch {
            observed_digest: Digest::of_bytes(stdout.as_bytes()).as_str().to_owned(),
        })?;
    if !stderr.is_empty() {
        return Err(AgdaReferenceFailureV1::CheckerStderr {
            digest: Digest::of_bytes(stderr.as_bytes()).as_str().to_owned(),
        });
    }

    for (source, path) in sources.iter().zip(&source_paths) {
        if fs::read(path).ok().as_deref() != Some(source.bytes) {
            return Err(AgdaReferenceFailureV1::SourceMutated);
        }
    }
    if fs::read(executable).ok().as_deref() != Some(executable_before) {
        return Err(AgdaReferenceFailureV1::ExecutableMutated);
    }
    let installed_after = collect_primitive_tree(&primitive_root)
        .ok_or(AgdaReferenceFailureV1::InstalledPrimitiveTreeMutated)?;
    if installed_after != tree_before {
        return Err(AgdaReferenceFailureV1::InstalledPrimitiveTreeMutated);
    }
    let snapshot_after = collect_primitive_tree(&snapshot_primitive_root)
        .ok_or(AgdaReferenceFailureV1::SnapshotPrimitiveTreeMutated)?;
    if snapshot_after != snapshot_before {
        return Err(AgdaReferenceFailureV1::SnapshotPrimitiveTreeMutated);
    }

    let source_tree_digest = fixed_package_source_tree_digest(sources);
    let checker_argument_protocol_digest =
        fixed_package_checker_argument_protocol_digest(entry_relative_path);
    let checker_stdout_digest = Digest::of_bytes(canonical_transcript.as_bytes());
    let checker_stderr_digest = Digest::of_bytes(stderr.as_bytes());
    let mut verified = VerifiedFixedAgdaPackageV1 {
        source_tree_digest,
        executable_digest,
        primitive_tree_digest: tree_before.digest,
        checker_argument_protocol_digest,
        checker_stdout_digest,
        checker_stderr_digest,
        digest: Digest::of_bytes(b"pending fixed Agda package"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-fixed-agda-package/v1",
        &verified,
    );
    Ok(verified)
}

fn canonical_fixed_package_transcript(
    stdout: &str,
    sources: &[AgdaSourceViewV1<'_>],
    source_paths: &[PathBuf],
) -> Option<String> {
    let lines = stdout.lines().collect::<Vec<_>>();
    if lines.len() != sources.len() || source_paths.len() != sources.len() {
        return None;
    }
    let mut canonical = String::new();
    for ((line, source), expected_path) in lines.iter().zip(sources).zip(source_paths) {
        let line = line.trim_start();
        let prefix = format!("Checking {} (", source.module_name);
        let reported = line.strip_prefix(&prefix)?.strip_suffix(").")?;
        let reported = Path::new(reported);
        if !reported.is_absolute()
            || fs::canonicalize(reported).ok()? != fs::canonicalize(expected_path).ok()?
        {
            return None;
        }
        canonical.push_str("Checking ");
        canonical.push_str(source.module_name);
        canonical.push_str(" (<private-source>).\n");
    }
    Some(canonical)
}

fn fixed_package_source_tree_digest(sources: &[AgdaSourceViewV1<'_>]) -> Digest {
    let mut ordered = sources.to_vec();
    ordered.sort_by_key(|source| source.relative_path);
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(1);
    encoder.u64(ordered.len() as u64);
    for source in ordered {
        encoder.text(source.relative_path);
        encoder.text(source.module_name);
        encoder.bytes(source.bytes);
    }
    Digest::of_domain_bytes(
        "pen-semantic-audit/fixed-agda-source-tree/v1",
        encoder.as_bytes(),
    )
}

fn fixed_package_checker_argument_protocol_digest(entry_relative_path: &str) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(1);
    for argument in [
        "--no-libraries",
        "--ignore-interfaces",
        "--safe",
        "--without-K",
        "-i",
        "<private-scratch>",
        entry_relative_path,
    ] {
        encoder.text(argument);
    }
    encoder.text("environment=cleared");
    encoder.text("preserve-if-present=SystemRoot,WINDIR,ComSpec,PATHEXT");
    encoder.text(
        "private=HOME,USERPROFILE,XDG_CONFIG_HOME,APPDATA,LOCALAPPDATA,TMP,TEMP,TMPDIR,AGDA_DIR",
    );
    encoder.text("locale=LC_ALL:C.UTF-8,LANG:C.UTF-8");
    encoder.text("Agda_datadir=private-pinned-primitive-snapshot");
    Digest::of_domain_bytes(
        "pen-semantic-audit/fixed-agda-checker-argument-protocol/v1",
        encoder.as_bytes(),
    )
}

fn verify_in_scratch(
    executable: &Path,
    executable_before: &[u8],
    executable_digest: Digest,
    scratch: &Path,
) -> Result<VerifiedAgdaReferenceV1, AgdaReferenceFailureV1> {
    fs::create_dir(scratch.join("agda-app"))
        .map_err(|_| AgdaReferenceFailureV1::ScratchSetupFailed)?;

    let version_output = run_command(
        sanitized_command(executable, Some(scratch), None).arg("--version"),
        CHECKER_TIMEOUT,
    )
    .map_err(|failure| match failure {
        RunCommandFailure::Spawn => AgdaReferenceFailureV1::VersionProbeSpawnFailed,
        RunCommandFailure::Timeout => AgdaReferenceFailureV1::VersionProbeTimedOut,
        RunCommandFailure::Wait => AgdaReferenceFailureV1::VersionProbeFailed { exit_code: None },
    })?;
    let version_stdout = canonical_output(&version_output.stdout)
        .ok_or(AgdaReferenceFailureV1::VersionProbeNonCanonicalOutput)?;
    let version_stderr = canonical_output(&version_output.stderr)
        .ok_or(AgdaReferenceFailureV1::VersionProbeNonCanonicalOutput)?;
    if !version_output.status.success() {
        return Err(AgdaReferenceFailureV1::VersionProbeFailed {
            exit_code: version_output.status.code(),
        });
    }
    if !version_stderr.is_empty() {
        return Err(AgdaReferenceFailureV1::VersionProbeStderr {
            digest: Digest::of_bytes(version_stderr.as_bytes())
                .as_str()
                .to_owned(),
        });
    }
    let expected_version_line = format!("Agda version {AGDA_EXPECTED_VERSION}");
    if version_stdout.lines().next() != Some(expected_version_line.as_str()) {
        return Err(AgdaReferenceFailureV1::VersionMismatch {
            observed_first_line: version_stdout.lines().next().map(str::to_owned),
        });
    }

    let data_output = run_command(
        sanitized_command(executable, Some(scratch), None).arg("--print-agda-data-dir"),
        CHECKER_TIMEOUT,
    )
    .map_err(|failure| match failure {
        RunCommandFailure::Spawn => AgdaReferenceFailureV1::DataDirectoryProbeSpawnFailed,
        RunCommandFailure::Timeout => AgdaReferenceFailureV1::DataDirectoryProbeTimedOut,
        RunCommandFailure::Wait => {
            AgdaReferenceFailureV1::DataDirectoryProbeFailed { exit_code: None }
        }
    })?;
    let data_stdout = canonical_output(&data_output.stdout)
        .ok_or(AgdaReferenceFailureV1::DataDirectoryProbeNonCanonicalOutput)?;
    let data_stderr = canonical_output(&data_output.stderr)
        .ok_or(AgdaReferenceFailureV1::DataDirectoryProbeNonCanonicalOutput)?;
    if !data_output.status.success() {
        return Err(AgdaReferenceFailureV1::DataDirectoryProbeFailed {
            exit_code: data_output.status.code(),
        });
    }
    if !data_stderr.is_empty() {
        return Err(AgdaReferenceFailureV1::DataDirectoryProbeStderr {
            digest: Digest::of_bytes(data_stderr.as_bytes()).as_str().to_owned(),
        });
    }
    let data_directory =
        one_absolute_directory(&data_stdout).ok_or(AgdaReferenceFailureV1::DataDirectoryInvalid)?;
    let primitive_root = fs::canonicalize(data_directory.join(AGDA_PRIMITIVE_RELATIVE_ROOT))
        .ok()
        .filter(|path| path.is_dir())
        .ok_or(AgdaReferenceFailureV1::PrimitiveTreeUnreadable)?;
    let tree_before = collect_primitive_tree(&primitive_root)
        .ok_or(AgdaReferenceFailureV1::PrimitiveTreeUnreadable)?;
    if tree_before.digest.as_str() != AGDA_PRIMITIVE_TREE_DIGEST_PIN {
        return Err(AgdaReferenceFailureV1::PrimitiveTreeDigestMismatch {
            observed: tree_before.digest.as_str().to_owned(),
        });
    }

    let snapshot_data_directory = scratch.join(AGDA_DATA_SNAPSHOT_DIRECTORY);
    fs::create_dir(&snapshot_data_directory)
        .map_err(|_| AgdaReferenceFailureV1::PrimitiveSnapshotFailed)?;
    let snapshot_primitive_root = snapshot_data_directory.join(AGDA_PRIMITIVE_RELATIVE_ROOT);
    let materialized = materialize_primitive_tree(&primitive_root, &snapshot_primitive_root)
        .ok_or(AgdaReferenceFailureV1::PrimitiveSnapshotFailed)?;
    let snapshot_before = collect_primitive_tree(&snapshot_primitive_root)
        .ok_or(AgdaReferenceFailureV1::PrimitiveSnapshotFailed)?;
    if materialized != tree_before || snapshot_before != tree_before {
        return Err(AgdaReferenceFailureV1::PrimitiveSnapshotMismatch);
    }

    let source_path = scratch.join(AGDA_SOURCE_RELATIVE_PATH);
    let source_parent = source_path
        .parent()
        .ok_or(AgdaReferenceFailureV1::SourceWriteFailed)?;
    fs::create_dir_all(source_parent).map_err(|_| AgdaReferenceFailureV1::SourceWriteFailed)?;
    write_new_file(&source_path, AGDA_SOURCE_BYTES)
        .ok_or(AgdaReferenceFailureV1::SourceWriteFailed)?;

    let mut checker = sanitized_command(
        executable,
        Some(scratch),
        Some(snapshot_data_directory.as_path()),
    );
    checker
        .args([
            "--no-libraries",
            "--ignore-interfaces",
            "--safe",
            "--without-K",
            "-i",
        ])
        .arg(scratch)
        .arg(&source_path);
    let output = run_command(&mut checker, CHECKER_TIMEOUT).map_err(|failure| match failure {
        RunCommandFailure::Spawn => AgdaReferenceFailureV1::CheckerSpawnFailed,
        RunCommandFailure::Timeout => AgdaReferenceFailureV1::CheckerTimedOut,
        RunCommandFailure::Wait => AgdaReferenceFailureV1::CheckerFailed {
            exit_code: None,
            stdout_digest: Digest::of_bytes(b"").as_str().to_owned(),
            stderr_digest: Digest::of_bytes(b"").as_str().to_owned(),
        },
    })?;
    let stdout = canonical_output(&output.stdout)
        .ok_or(AgdaReferenceFailureV1::CheckerNonCanonicalOutput)?;
    let stderr = canonical_output(&output.stderr)
        .ok_or(AgdaReferenceFailureV1::CheckerNonCanonicalOutput)?;
    if !output.status.success() {
        return Err(AgdaReferenceFailureV1::CheckerFailed {
            exit_code: output.status.code(),
            stdout_digest: Digest::of_bytes(stdout.as_bytes()).as_str().to_owned(),
            stderr_digest: Digest::of_bytes(stderr.as_bytes()).as_str().to_owned(),
        });
    }
    if !checker_stdout_matches_exact_source(&stdout, &source_path) {
        return Err(AgdaReferenceFailureV1::CheckerStdoutMismatch {
            observed_digest: Digest::of_bytes(stdout.as_bytes()).as_str().to_owned(),
        });
    }
    if !stderr.is_empty() {
        return Err(AgdaReferenceFailureV1::CheckerStderr {
            digest: Digest::of_bytes(stderr.as_bytes()).as_str().to_owned(),
        });
    }
    if fs::read(&source_path).ok().as_deref() != Some(AGDA_SOURCE_BYTES) {
        return Err(AgdaReferenceFailureV1::SourceMutated);
    }
    if fs::read(executable).ok().as_deref() != Some(executable_before) {
        return Err(AgdaReferenceFailureV1::ExecutableMutated);
    }
    let installed_after = collect_primitive_tree(&primitive_root)
        .ok_or(AgdaReferenceFailureV1::InstalledPrimitiveTreeMutated)?;
    if installed_after != tree_before {
        return Err(AgdaReferenceFailureV1::InstalledPrimitiveTreeMutated);
    }
    let snapshot_after = collect_primitive_tree(&snapshot_primitive_root)
        .ok_or(AgdaReferenceFailureV1::SnapshotPrimitiveTreeMutated)?;
    if snapshot_after != snapshot_before {
        return Err(AgdaReferenceFailureV1::SnapshotPrimitiveTreeMutated);
    }

    let source_digest = Digest::of_bytes(AGDA_SOURCE_BYTES);
    let checker_stdout_digest =
        Digest::of_bytes(b"Checking LawV2.SemanticAuditCoreV1 (<private-source>).\n");
    let checker_stderr_digest = Digest::of_bytes(stderr.as_bytes());
    let checker_argument_protocol_digest = checker_argument_protocol_digest();
    let mut verified = VerifiedAgdaReferenceV1 {
        source_digest,
        executable_digest,
        primitive_tree_digest: tree_before.digest,
        primitive_source_file_count: tree_before.source_file_count,
        primitive_source_byte_length: tree_before.source_byte_length,
        checker_argument_protocol_digest,
        checker_stdout_digest,
        checker_stderr_digest,
        digest: Digest::of_bytes(b"pending Agda reference"),
    };
    verified.digest =
        Digest::of_canonical("pen-semantic-audit/verified-agda-reference/v1", &verified);
    Ok(verified)
}

fn sanitized_command<'a>(
    executable: &Path,
    current_directory: Option<&'a Path>,
    agda_data_directory: Option<&'a Path>,
) -> Command {
    let mut command = Command::new(executable);
    command.env_clear();
    for key in ["SystemRoot", "WINDIR", "ComSpec", "PATHEXT"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    if let Some(directory) = current_directory {
        command
            .current_dir(directory)
            .env("HOME", directory)
            .env("USERPROFILE", directory)
            .env("XDG_CONFIG_HOME", directory)
            .env("APPDATA", directory)
            .env("LOCALAPPDATA", directory)
            .env("TMP", directory)
            .env("TEMP", directory)
            .env("TMPDIR", directory)
            .env("AGDA_DIR", directory.join("agda-app"));
    }
    if let Some(data_directory) = agda_data_directory {
        command.env("Agda_datadir", data_directory);
    }
    command
        .env("LC_ALL", "C.UTF-8")
        .env("LANG", "C.UTF-8")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunCommandFailure {
    Spawn,
    Timeout,
    Wait,
}

fn run_command(command: &mut Command, timeout: Duration) -> Result<Output, RunCommandFailure> {
    let mut child = command.spawn().map_err(|_| RunCommandFailure::Spawn)?;
    let start = Instant::now();
    loop {
        if child
            .try_wait()
            .map_err(|_| RunCommandFailure::Wait)?
            .is_some()
        {
            return child
                .wait_with_output()
                .map_err(|_| RunCommandFailure::Wait);
        }
        if start.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(RunCommandFailure::Timeout);
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn resolve_agda_executable() -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for directory in std::env::split_paths(&path) {
        for extension in executable_extensions() {
            let candidate = directory.join(format!("agda{extension}"));
            let Ok(canonical) = fs::canonicalize(candidate) else {
                continue;
            };
            if canonical.is_file() {
                return Some(canonical);
            }
        }
    }
    None
}

fn executable_extensions() -> Vec<String> {
    if cfg!(windows) {
        let configured =
            std::env::var_os("PATHEXT").unwrap_or_else(|| OsString::from(".EXE;.COM;.BAT;.CMD"));
        let mut extensions = configured
            .to_string_lossy()
            .split(';')
            .filter(|value| !value.is_empty())
            .map(str::to_ascii_lowercase)
            .collect::<Vec<_>>();
        if !extensions.iter().any(|extension| extension == ".exe") {
            extensions.insert(0, ".exe".to_owned());
        }
        extensions
    } else {
        vec![String::new()]
    }
}

fn canonical_output(bytes: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(bytes).ok()?;
    let normalized = text.replace("\r\n", "\n");
    (!normalized.contains('\r')).then_some(normalized)
}

fn one_absolute_directory(output: &str) -> Option<PathBuf> {
    let lines = output.lines().collect::<Vec<_>>();
    let [line] = lines.as_slice() else {
        return None;
    };
    let path = Path::new(line.trim());
    path.is_absolute()
        .then(|| fs::canonicalize(path).ok())
        .flatten()
        .filter(|path| path.is_dir())
}

fn checker_stdout_matches_exact_source(stdout: &str, source_path: &Path) -> bool {
    let Some(line) = stdout.strip_suffix('\n') else {
        return false;
    };
    if line.contains('\n') {
        return false;
    }
    let Some(reported) = line
        .strip_prefix("Checking LawV2.SemanticAuditCoreV1 (")
        .and_then(|line| line.strip_suffix(")."))
    else {
        return false;
    };
    let reported = Path::new(reported);
    if !reported.is_absolute() {
        return false;
    }
    let (Ok(reported), Ok(expected)) = (fs::canonicalize(reported), fs::canonicalize(source_path))
    else {
        return false;
    };
    reported == expected
}

fn materialize_primitive_tree(
    installed_root: &Path,
    snapshot_root: &Path,
) -> Option<PrimitiveTree> {
    if let Some(parent) = snapshot_root.parent() {
        fs::create_dir_all(parent).ok()?;
    }
    fs::create_dir(snapshot_root).ok()?;
    let installed_root = fs::canonicalize(installed_root).ok()?;
    let mut pending = vec![installed_root.clone()];
    let mut entries = 0_u64;
    let mut total_bytes = 0_u64;
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).ok()? {
            let entry = entry.ok()?;
            entries = entries.checked_add(1)?;
            if entries > MAX_TREE_ENTRIES {
                return None;
            }
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).ok()?;
            let kind = metadata.file_type();
            if kind.is_symlink() {
                return None;
            }
            if kind.is_dir() {
                pending.push(path);
                continue;
            }
            if !kind.is_file() {
                return None;
            }
            let relative = path.strip_prefix(&installed_root).ok()?;
            let relative_path = relative.to_str()?.replace('\\', "/");
            if !is_primitive_source(&relative_path) {
                continue;
            }
            if files.len() as u64 >= MAX_SOURCE_FILES || metadata.len() > MAX_SOURCE_FILE_BYTES {
                return None;
            }
            let raw = fs::read(&path).ok()?;
            if raw.len() as u64 > MAX_SOURCE_FILE_BYTES {
                return None;
            }
            let bytes = canonical_primitive_bytes(&relative_path, &raw)?;
            total_bytes = total_bytes.checked_add(bytes.len() as u64)?;
            if total_bytes > MAX_SOURCE_TREE_BYTES {
                return None;
            }
            let snapshot_path = snapshot_root.join(relative);
            if let Some(parent) = snapshot_path.parent() {
                fs::create_dir_all(parent).ok()?;
            }
            write_new_file(&snapshot_path, &bytes)?;
            files.push(PrimitiveFile {
                relative_path,
                byte_length: bytes.len() as u64,
                digest: Digest::of_bytes(&bytes),
            });
        }
    }
    primitive_tree_from_files(files, total_bytes)
}

fn collect_primitive_tree(root: &Path) -> Option<PrimitiveTree> {
    let root = fs::canonicalize(root).ok()?;
    let mut pending = vec![root.clone()];
    let mut entries = 0_u64;
    let mut total_bytes = 0_u64;
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).ok()? {
            let entry = entry.ok()?;
            entries = entries.checked_add(1)?;
            if entries > MAX_TREE_ENTRIES {
                return None;
            }
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).ok()?;
            let kind = metadata.file_type();
            if kind.is_symlink() {
                return None;
            }
            if kind.is_dir() {
                pending.push(path);
                continue;
            }
            if !kind.is_file() {
                return None;
            }
            let relative = path.strip_prefix(&root).ok()?;
            let relative_path = relative.to_str()?.replace('\\', "/");
            if !is_primitive_source(&relative_path) {
                continue;
            }
            if files.len() as u64 >= MAX_SOURCE_FILES || metadata.len() > MAX_SOURCE_FILE_BYTES {
                return None;
            }
            let bytes = canonical_primitive_bytes(&relative_path, &fs::read(path).ok()?)?;
            total_bytes = total_bytes.checked_add(bytes.len() as u64)?;
            if total_bytes > MAX_SOURCE_TREE_BYTES {
                return None;
            }
            files.push(PrimitiveFile {
                relative_path,
                byte_length: bytes.len() as u64,
                digest: Digest::of_bytes(&bytes),
            });
        }
    }
    primitive_tree_from_files(files, total_bytes)
}

fn primitive_tree_from_files(
    mut files: Vec<PrimitiveFile>,
    total_bytes: u64,
) -> Option<PrimitiveTree> {
    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    if files.is_empty()
        || files
            .windows(2)
            .any(|pair| pair[0].relative_path >= pair[1].relative_path)
    {
        return None;
    }
    let mut encoder = CanonicalEncoder::new();
    encoder.text(AGDA_PRIMITIVE_TREE_CONTRACT);
    encoder.u64(files.len() as u64);
    for file in &files {
        encoder.text(&file.relative_path);
        encoder.u64(file.byte_length);
        encoder.text(file.digest.as_str());
    }
    Some(PrimitiveTree {
        digest: Digest::of_bytes(encoder.as_bytes()),
        source_file_count: files.len() as u64,
        source_byte_length: total_bytes,
        files,
    })
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Option<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .ok()?;
    file.write_all(bytes).ok()?;
    file.flush().ok()?;
    Some(())
}

fn is_primitive_source(path: &str) -> bool {
    path == "agda-builtins.agda-lib"
        || path.ends_with(".agda")
        || path.ends_with(".lagda")
        || path.ends_with(".lagda.md")
        || path.ends_with(".lagda.rst")
        || path.ends_with(".lagda.tex")
        || path.ends_with(".agdai")
}

fn canonical_primitive_bytes(path: &str, raw: &[u8]) -> Option<Vec<u8>> {
    if path.ends_with(".agdai") {
        return Some(raw.to_vec());
    }
    let text = std::str::from_utf8(raw).ok()?;
    let normalized = text.replace("\r\n", "\n");
    (!normalized.contains('\r')).then(|| normalized.into_bytes())
}

fn create_scratch_directory() -> Option<PathBuf> {
    let base = std::env::temp_dir();
    for _ in 0..128 {
        let ordinal = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = base.join(format!(
            "pen-semantic-audit-agda-{}-{ordinal}",
            std::process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => return Some(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return None,
        }
    }
    None
}

fn checker_argument_protocol_digest() -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(1);
    for argument in [
        "--no-libraries",
        "--ignore-interfaces",
        "--safe",
        "--without-K",
        "-i",
        "<private-scratch>",
        AGDA_SOURCE_RELATIVE_PATH,
    ] {
        encoder.text(argument);
    }
    encoder.text("environment=cleared");
    encoder.text("preserve-if-present=SystemRoot,WINDIR,ComSpec,PATHEXT");
    encoder.text(
        "private=HOME,USERPROFILE,XDG_CONFIG_HOME,APPDATA,LOCALAPPDATA,TMP,TEMP,TMPDIR,AGDA_DIR",
    );
    encoder.text("locale=LC_ALL:C.UTF-8,LANG:C.UTF-8");
    encoder.text("Agda_datadir=private-pinned-primitive-snapshot");
    Digest::of_domain_bytes(
        "pen-semantic-audit/agda-checker-argument-protocol/v1",
        encoder.as_bytes(),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        AGDA_SOURCE_BYTES, checker_stdout_matches_exact_source, verify_pinned_agda_reference_v1,
    };
    use crate::manifest::AuditDecision;
    use std::fs;
    use std::path::Path;

    #[test]
    fn fixed_source_is_safe_and_contains_no_postulate() {
        let source = std::str::from_utf8(AGDA_SOURCE_BYTES).expect("Agda source is UTF-8");
        assert!(source.contains("{-# OPTIONS --safe --without-K #-}"));
        assert!(!source.contains("postulate"));
        assert!(!source.contains("{-# TERMINATING #-}"));
        assert!(source.contains("abstract-safe-agda-reference-v1"));
        assert!(source.contains("not-rust-equivalence"));
        assert!(source.contains("cost-reference-vectors"));
        assert!(source.contains("semantic-reference-vectors"));
    }

    #[test]
    fn checker_stdout_requires_one_line_and_the_exact_canonical_source() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("agda")
            .join("LawV2")
            .join("SemanticAuditCoreV1.agda");
        let canonical = fs::canonicalize(&source).expect("canonical fixed source");
        let canonical_line = format!(
            "Checking LawV2.SemanticAuditCoreV1 ({}).\n",
            canonical.display()
        );
        assert!(checker_stdout_matches_exact_source(
            &canonical_line,
            &source
        ));

        let relative_line =
            "Checking LawV2.SemanticAuditCoreV1 (LawV2/SemanticAuditCoreV1.agda).\n";
        assert!(!checker_stdout_matches_exact_source(relative_line, &source));
        assert!(!checker_stdout_matches_exact_source(
            &format!("noise\n{canonical_line}"),
            &source
        ));
    }

    #[test]
    #[ignore = "requires the independently pinned local Agda 2.8.0 runtime"]
    fn pinned_live_agda_reference_checks() {
        assert!(matches!(
            verify_pinned_agda_reference_v1(),
            AuditDecision::Proven(_)
        ));
    }
}
